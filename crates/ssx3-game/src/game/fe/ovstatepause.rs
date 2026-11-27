//! `src/fe/ovstatepause.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::ai::computer::lh_2 as lh;
pub(crate) use crate::game::ai::computer::sd;
pub(crate) use crate::game::dirtysock::tagsunk::gp_5 as gp;
pub(crate) use crate::game::dirtysock::tagsunk::p_2;
pub(crate) use crate::game::dirtysock::tagsunk::sw;
pub(crate) use crate::game::fe::all_in_one_fe::OV_STATE_VTBL_BASE;
pub(crate) use crate::game::fe::all_in_one_fe::OV_STR_SP_B_HI;
pub(crate) use crate::game::fe::feasyncfile::gp as gp_2;
pub(crate) use crate::game::fe::feasyncfile::gp as gp_3;
pub(crate) use crate::game::fe::feasyncfile::lw_5 as lw_4;
pub(crate) use crate::game::fe::feasyncfile::r;
pub(crate) use crate::game::fe::feasyncfile::sw_4 as sw_2;
pub(crate) use crate::game::fe::fepopup::at as at_3;
pub(crate) use crate::game::fe::fepopup::ld;
pub(crate) use crate::game::fe::fepopup::lw_9 as lw_6;
pub(crate) use crate::game::fe::fepopup::sw_4 as sw_3;
pub(crate) use crate::game::fe::festateoptions::gpa;
pub(crate) use crate::game::fe::festaterewards::gp_4;
pub(crate) use crate::game::fe::festaterewards::lw_3 as lw_2;
pub(crate) use crate::game::fe::ovstateprofile::ov_screen_set_two_visible;
pub(crate) use crate::game::fe::ovstateprofile::ov_screen_set_widget_visibility;
pub(crate) use crate::game::fe::ovstatetrophy::oc1015_ctor2;
pub(crate) use crate::game::fe::ovstatetrophy::oc1015_dtor;
pub(crate) use crate::game::fe::ovstatetrophy::OC1015_DATA;
pub(crate) use crate::game::fe::ovstatetrophy::OC1015_UI_DTOR;
pub(crate) use crate::game::fe::ovstatetrophy::OC1015_VT_BASE;
pub(crate) use crate::game::main::gamemode::low;
pub(crate) use crate::game::main::gamemode::lw_r;
pub(crate) use crate::game::main::gamemode::sp_2;
pub(crate) use crate::game::scripter::ssxscriptengine::lw_at_2 as lw_at;
pub(crate) use crate::game::scripter::ssxscriptengine::sw_at;
pub(crate) use crate::game::wscript::wscriptman::lw_4 as lw_3;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x001f_aa78,
        ov_screen_be_options_on_widget_notify,
        "OVScreenBEOptions_OnWidgetNotify",
    ),
    (
        0x001f_aaf0,
        ov_screen_be_options_v22,
        "OVScreenBEOptions_v22",
    ),
    (
        0x001f_ab08,
        ov_screen_be_options_v21,
        "OVScreenBEOptions_v21",
    ),
    (
        0x001f_af10,
        ovscreen_in_game_error_update,
        "OVScreenInGameError_Update",
    ),
    (
        0x001f_aff8,
        ov_screen_in_game_error_show_cancel_game,
        "OVScreenInGameError_ShowCancelGame",
    ),
    (
        0x001f_b180,
        ov_screen_in_game_error_update_server_timeout_text,
        "OVScreenInGameError_UpdateServerTimeoutText",
    ),
    (
        0x001f_b218,
        ov_screen_in_game_error_update_opponent_disconnected_text,
        "OVScreenInGameError_UpdateOpponentDisconnectedText",
    ),
    (
        0x001f_b378,
        ovscreeningameerror_show_continue_button,
        "OVScreenInGameError_ShowContinueButton",
    ),
    (
        0x001f_b458,
        in_game_error_highlight_continue,
        "OVScreenInGameError_HighlightContinue",
    ),
    (
        0x001f_b588,
        in_game_error_highlight_cancel,
        "OVScreenInGameError_HighlightCancel",
    ),
    (
        0x001f_bcc8,
        ov_screen_race_pre_on_deactivate,
        "OVScreenRacePre_OnDeactivate",
    ),
    (
        0x001f_bce0,
        ov_screen_race_pre_update,
        "OVScreenRacePre_Update",
    ),
    (
        0x001f_d0d8,
        ov_screen_multi_freestyle_pre_on_activate,
        "OVScreenMultiFreestylePre_OnActivate",
    ),
    (
        0x001f_d118,
        ov_multi_freestyle_pre_on_deactivate,
        "OVScreenMultiFreestylePre_OnDeactivate",
    ),
    (
        0x001f_d150,
        ov_screen_multi_freestyle_pre_update,
        "OVScreenMultiFreestylePre_Update",
    ),
    (
        0x001f_d268,
        ov_screen_rival_pre_init,
        "OVScreenRivalPre_Init",
    ),
    (
        0x001f_db78,
        ov_screen_rival_pre_on_widget_notify,
        "OVScreenRivalPre_OnWidgetNotify",
    ),
    (
        0x001f_dbf0,
        ov_screen_save_replay_ctor,
        "OVScreenSaveReplay_ctor",
    ),
    (
        0x001f_df00,
        ov_screen_save_replay_v33,
        "OVScreenSaveReplay_v33",
    ),
    (
        0x001f_e5b0,
        ov_screen_save_replay_v38,
        "OVScreenSaveReplay_v38",
    ),
    (
        0x001f_e648,
        ov_screen_save_replay_v39,
        "OVScreenSaveReplay_v39",
    ),
    (
        0x001f_e8f0,
        ov_screen_save_replay_on_child_event,
        "OVScreenSaveReplay_OnChildEvent",
    ),
    (
        0x001f_e968,
        ov_screen_save_records_ctor,
        "OVScreenSaveRecords_ctor",
    ),
    (
        0x001f_ec60,
        ov_screen_save_records_v33,
        "OVScreenSaveRecords_v33",
    ),
    (
        0x001f_f278,
        ov_screen_save_records_v38,
        "OVScreenSaveRecords_v38",
    ),
    (
        0x001f_f3b0,
        ov_screen_save_records_on_widget_action,
        "OVScreenSaveRecords_OnWidgetAction",
    ),
    (
        0x001f_f6d0,
        ov_screen_reward_list_on_activate,
        "OVScreenRewardList_OnActivate",
    ),
    (
        0x001f_f700,
        ov_screen_reward_list_on_widget_action,
        "OVScreenRewardList_OnWidgetAction",
    ),
    (0x0023_d570, mc_manager_set_mode, "MCManager_SetMode"),
    (0x001f_b6b8, ov_screen_race_pre_init, "OVScreenRacePre_Init"),
    (
        0x001f_bbd8,
        ov_screen_race_pre_on_activate,
        "OVScreenRacePre_OnActivate",
    ),
    (0x0022_1df0, ov_screen_race_pre_dtor, "OVScreenRacePre_dtor"),
    (0x001f_d190, fe_get_event_name_text, "FE_GetEventNameText"),
    (
        0x001f_9188,
        ov_screen_be_options_init,
        "OVScreenBEOptions_Init",
    ),
    (
        0x001f_a9e0,
        ov_screen_be_options_update_2,
        "OVScreenBEOptions_Update",
    ),
    (0x001f_ab18, ingame_error_init, "OVScreenInGameError_Init"),
    (
        0x001f_b2b8,
        ov_screen_in_game_error_handle_select,
        "OVScreenInGameError_HandleSelect",
    ),
    (
        0x001f_bd20,
        ov_screen_freestyle_pre_init,
        "OVScreenFreestylePre_Init",
    ),
    (
        0x001f_c878,
        ov_screen_multi_freestyle_pre_init,
        "OVScreenMultiFreestylePre_Init",
    ),
    (
        0x001f_cec0,
        mfpre_transition_in,
        "OVScreenMultiFreestylePre_TransitionIn",
    ),
    (
        0x001f_d320,
        ov_screen_rival_pre_transition_in,
        "OVScreenRivalPre_TransitionIn",
    ),
    (
        0x001f_db88,
        ov_screen_rival_pre_on_widget_action,
        "OVScreenRivalPre_OnWidgetAction",
    ),
    (0x001f_dc30, savereplay_init, "OVScreenSaveReplay_Init"),
    (
        0x001f_df40,
        ov_screen_save_replay_v26,
        "OVScreenSaveReplay_v26",
    ),
    (
        0x001f_e4a8,
        ov_screen_save_replay_v37,
        "OVScreenSaveReplay_v37",
    ),
    (
        0x001f_e6e8,
        savereplay_widget_action,
        "OVScreenSaveReplay_OnWidgetAction",
    ),
    (
        0x001f_e9a8,
        ov_screen_save_records_init,
        "OVScreenSaveRecords_Init",
    ),
    (
        0x001f_ec40,
        ov_screen_save_records_shutdown,
        "OVScreenSaveRecords_Shutdown",
    ),
    (
        0x001f_eca0,
        ov_screen_save_records_v26,
        "OVScreenSaveRecords_v26",
    ),
    (
        0x001f_f170,
        ov_screen_save_records_v37,
        "OVScreenSaveRecords_v37",
    ),
    (
        0x001f_f310,
        ov_screen_save_records_v39,
        "OVScreenSaveRecords_v39",
    ),
    (
        0x001f_f590,
        ov_screen_reward_list_init,
        "OVScreenRewardList_Init",
    ),
    (
        0x001f_f7b8,
        ov_screen_reward_list_get_entry_text,
        "OVScreenRewardList_GetEntryText",
    ),
    (
        0x001f_fd08,
        ov_screen_reward_list_show_page,
        "OVScreenRewardList_ShowPage",
    ),
    (
        0x001f_ff78,
        ov_screen_reward_list_transition_in,
        "OVScreenRewardList_TransitionIn",
    ),
    (
        0x0022_1bf0,
        ov_screen_be_options_dtor,
        "OVScreenBEOptions_dtor",
    ),
    (
        0x0022_1c18,
        ov_screen_be_options_ctor,
        "OVScreenBEOptions_ctor",
    ),
    (
        0x0022_1c80,
        ov_screen_in_game_error_ctor,
        "OVScreenInGameError_ctor",
    ),
    (
        0x0022_1ce0,
        ov_screen_in_game_error_ctor_2,
        "OVScreenInGameError_ctor_2",
    ),
    (
        0x0022_1d58,
        ov_screen_in_game_error_dtor,
        "OVScreenInGameError_dtor",
    ),
    (0x0022_1e18, ov_screen_race_pre_ctor, "OVScreenRacePre_ctor"),
    (
        0x0022_1e80,
        ov_screen_freestyle_pre_dtor,
        "OVScreenFreestylePre_dtor",
    ),
    (
        0x0022_1ea8,
        ov_screen_freestyle_pre_ctor,
        "OVScreenFreestylePre_ctor",
    ),
    (
        0x0022_1f10,
        ov_screen_multi_freestyle_pre_dtor,
        "OVScreenMultiFreestylePre_dtor",
    ),
    (
        0x0022_1f38,
        ov_screen_multi_freestyle_pre_ctor,
        "OVScreenMultiFreestylePre_ctor",
    ),
    (
        0x0022_1fa0,
        ov_screen_rival_pre_dtor,
        "OVScreenRivalPre_dtor",
    ),
    (
        0x0022_1fc8,
        ov_screen_rival_pre_ctor,
        "OVScreenRivalPre_ctor",
    ),
    (
        0x0022_2030,
        ov_screen_save_replay_dtor,
        "OVScreenSaveReplay_dtor",
    ),
    (
        0x0022_2058,
        ov_screen_save_replay_ctor_2,
        "OVScreenSaveReplay_ctor_2",
    ),
    (
        0x0022_2098,
        ov_screen_save_records_dtor,
        "OVScreenSaveRecords_dtor",
    ),
    (
        0x0022_20c0,
        ov_screen_save_records_ctor_2,
        "OVScreenSaveRecords_ctor_2",
    ),
    (
        0x0022_21f0,
        ov_screen_reward_list_dtor,
        "OVScreenRewardList_dtor",
    ),
    (
        0x0022_2218,
        ov_screen_reward_list_ctor,
        "OVScreenRewardList_ctor",
    ),
];

// ---------------------------------------------------------------------------
// Callees (direct calls, same shard or cross-shard through the table).

pub(crate) mod callee {
    use super::ee;
    use super::Callee;

    pub(crate) use crate::game::be::beintmission::callee::AUDIO_START_PRERACE_MUSIC;

    pub(crate) use crate::game::be::beintmission::callee::BE_LIBRARY_GET_INSTANCE;
    pub(crate) use crate::game::be::beintmission::callee::BE_LIBRARY_GET_INTERFACE;

    pub(crate) use crate::game::dirtysock::tagsunk::callee::BE_MESSAGE_FLAG_SET;

    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STRING_HASH;

    pub(crate) use crate::game::fe::feasyncfile::callee::OVERLAY_UPDATE;

    pub(crate) use crate::game::fe::feasyncfile::callee::OV_STATE_OVERLAY_INIT;

    pub(crate) use crate::game::fe::feasyncfile::callee::UI_MENU_SET_SELECTION_3 as UI_MENU_SET_SELECTION_2;

    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_FIND_OBJECT_5 as UI_SCREEN_FIND_OBJECT;

    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_LAUNCH_THREAD;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_LOOKUP_VALUE;

    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_SET_OBJECT_VALUE;

    pub(crate) use crate::game::fe::fepopup::callee::C00227F80;

    pub(crate) use crate::game::fe::festaterewards::callee::GAME_ERROR_OPPONENT_DISCONNECTED;
    pub(crate) use crate::game::fe::festaterewards::callee::GAME_ERROR_SERVER_TIMEOUT;
    pub(crate) use crate::game::fe::festaterewards::callee::GAME_ERROR_SHOW_CANCEL;

    pub(crate) use crate::game::main::gamemode::callee::STR_HASH as STR_HASH_2;

    pub(crate) use crate::game::main::ssxappdtor::callee::AUDIO_GET_INSTANCE;

    pub(crate) use crate::game::main::ssxappdtor::callee::REWARD_LIST_SHOW_PAGE;

    pub(crate) use crate::game::main::ssxappdtor::callee::UI_STATE_ON_ACTIVATE;

    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MEM_CARD_ON_CHILD_EVENT;

    pub(crate) use crate::game::scripter::ssxscriptengine::callee::REPLAY_END_SAVE;

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
pub(crate) mod addr {}

/// `GetHashValue32` (0x317670) and the UI object lookups, which the generator
/// emitted as cross-shard `call_indirect`s.
pub(crate) mod table_callee {

    pub(crate) use crate::game::fe::festaterewards::table_callee::GET_HASH_VALUE32;

    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_SCREEN_FIND_OBJECT;

    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_TEXT_SET_UNICODE_STRING_BY_ID;
}

/// Cross-shard direct calls the generator emitted as `call_indirect`.
pub(crate) mod table {

    pub(crate) use crate::game::fe::feasyncfile::table::REPLAY_BEGIN_PLAYBACK;
    pub(crate) use crate::game::fe::feasyncfile::table::REPLAY_END_SAVE;
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
                c.sd(c.g[SP][0] as u32, c.g[RA][0]);
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
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        Ok(())
    })
}

/// `OVScreenInGameError_ShowCancelGame(this, on)` (0x1faff8): show or hide
/// the "cancel game" prompt (and its second, menu-driven variant).
pub fn ov_screen_in_game_error_show_cancel_game(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 28;
        let this = abi::arg(c, 0);
        let _on = abi::arg(c, 1);
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
            0x001f_b484,
            0x001f_b490,
            0x001f_b4a0,
            0x001f_b4ac,
            0x001f_b4bc,
            0x001f_b4c8,
            0x001f_b500,
            0x001f_b538,
            0x001f_b550,
            0x001f_b568,
        ],
    );
}

/// `void OVScreenInGameError_HighlightCancel(OVScreen*, int on)` (0x1fb588).
pub fn in_game_error_highlight_cancel(c: &mut Cpu) {
    in_game_error_highlight(
        c,
        [0x0046_fa90, 0x0046_fac8, 0x0046_fae8],
        [
            0x001f_b5b4,
            0x001f_b5c0,
            0x001f_b5d0,
            0x001f_b5dc,
            0x001f_b5ec,
            0x001f_b5f8,
            0x001f_b630,
            0x001f_b668,
            0x001f_b680,
            0x001f_b698,
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
                                    if callee::C00227F80.call(c, 0x001ff428).is_err() {
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
                        if callee::C00227F80.call(c, 0x001ff474).is_err() {
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
                        if callee::C00227F80.call(c, 0x001ff4e4).is_err() {
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

/// `void OVScreenRacePre_Init(OVStateRacePre* state)` (0x1fb6b8, HEAT pre-event
/// screen `onCreateScreen`; `a0` is kept in `s5`): creates and starts the two
/// UI screens (the first one is stored at `state+0x40`), runs
/// `OVScreenOverlayMenu_Init`, fetches the BE interfaces (new-player, char
/// display, race, score), writes the track name / event-name line, shows the
/// rider-count objects (0x46fbd8/e8/f8 by `*(player_cfg)`, hiding 0x4a2148 or
/// setting its text for the game mode at `0x535c11`), sets the caption by
/// event kind, fills the six rider rows (names, with the player name for the
/// human riders in mode 2), the best-time text, plays FE sound 0xe and the
/// race-intro announcer line.
///
/// The body keeps the generated block structure, registers and cycle
/// charges (register leftovers are visible to the fake callees' call log);
/// every call goes through the function table with its return address.
pub fn ov_screen_race_pre_init(c: &mut Cpu) {
    guest(c, |c| {
        'b_1fb724: {
            c.cycles += 23;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x2f0));
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x290));
            c.g[S5][0] = c.g[A0][0];
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x2d0));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x2c0));
            c.g[A0][0] = 0x470000;
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x2b0));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x438));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x2a0));
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x280));
            c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x270));
            c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x260));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x250), c.g[RA][0]);
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x2e0));
            c.g[S0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x10));
            call_ptr(c, 0x00317670, 0x001fb6f8)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S5][0];
            c.g[A3][0] = 0u64;
            call_ptr(c, 0x00397cc0, 0x001fb70c)?;
            let t = c.g[V0][0] == 0u64;
            c.sw((c.g[S5][0] as u32).wrapping_add(0x40), c.g[V0][0]);
            if t {
                break 'b_1fb724;
            } else {
                c.cycles += 4;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0u64;
                c.g[A2][0] = 0u64;
                call_ptr(c, 0x0039c870, 0x001fb724)?;
                break 'b_1fb724;
            }
        }
        // Second screen (0x46e378 = the overlay-less status screen): create and start its thread.
        'b_1fb760: {
            c.cycles += 11;
            c.g[A0][0] = 0x470000;
            c.g[S0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x10));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1c88));
            call_ptr(c, 0x00317670, 0x001fb734)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S5][0];
            c.g[A3][0] = 0u64;
            call_ptr(c, 0x00397cc0, 0x001fb748)?;
            let t = c.g[V0][0] == 0u64;

            if t {
                break 'b_1fb760;
            } else {
                c.cycles += 4;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0u64;
                c.g[A2][0] = 0u64;
                call_ptr(c, 0x0039c870, 0x001fb760)?;
                break 'b_1fb760;
            }
        }
        // OVScreenOverlayMenu_Init, the BE interfaces, the track-name text, the
        // objective-mode object visibility and caption.
        'b_1fb990: {
            'b_1fb930: {
                c.cycles += 89;
                c.g[A0][0] = c.g[S5][0];
                call_ptr(c, 0x0020d1d8, 0x001fb768)?;
                call_ptr(c, 0x0014dc80, 0x001fb770)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0x1;
                call_ptr(c, 0x0014dd58, 0x001fb77c)?;
                c.sw((c.g[SP][0] as u32).wrapping_add(0x230), c.g[V0][0]);
                call_ptr(c, 0x0014dc80, 0x001fb784)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0x2;
                call_ptr(c, 0x0014dd58, 0x001fb790)?;
                c.g[FP][0] = c.g[V0][0];
                call_ptr(c, 0x0014dc80, 0x001fb798)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0u64;
                call_ptr(c, 0x0014dd58, 0x001fb7a4)?;
                c.g[S0][0] = c.g[V0][0];
                call_ptr(c, 0x0014dc80, 0x001fb7ac)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0x8;
                call_ptr(c, 0x0014dd58, 0x001fb7b8)?;
                c.sw((c.g[SP][0] as u32).wrapping_add(0x234), c.g[V0][0]);
                c.g[A1][0] = 0u64;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x230));
                c.g[S6][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc0));
                c.g[S4][0] = c.lw(c.g[S6][0] as u32);
                call_ptr(c, 0x00146e98, 0x001fb7d4)?;
                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x230));
                c.g[A1][0] = 0x1;
                c.sw((c.g[SP][0] as u32).wrapping_add(0x238), c.g[V0][0]);
                call_ptr(c, 0x00146e98, 0x001fb7e4)?;
                c.g[A0][0] = 0x470000;
                c.sw((c.g[SP][0] as u32).wrapping_add(0x23c), c.g[V0][0]);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1c78));
                call_ptr(c, 0x00317670, 0x001fb7f4)?;
                c.g[A0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x40));
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039d860, 0x001fb800)?;
                c.g[S2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[S3][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x00144bf0, 0x001fb810)?;
                c.g[A0][0] = c.g[S2][0];
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x002c2540, 0x001fb81c)?;
                c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                c.g[A0][0] = 0x470000;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1ab8));
                c.g[S0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8c));
                c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                call_ptr(c, 0x00317670, 0x001fb83c)?;
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                c.g[A0][0] = c.g[S0][0];
                let t = c.g[V1][0] as u32;
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, t, 0x001fb84c)?;
                c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
                c.g[A1][0] = 0x4a0000;
                c.g[A2][0] = c.g[S2][0];
                c.g[A3][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2140));
                call_ptr(c, 0x002c27c0, 0x001fb868)?;
                c.g[A0][0] = c.g[S3][0];
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x003a0e90, 0x001fb874)?;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.g[S5][0];
                c.g[S3][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x428));
                c.g[A1][0] = c.g[S3][0];
                call_ptr(c, 0x0039e8b8, 0x001fb888)?;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.g[S5][0];
                c.g[S2][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x418));
                c.g[A1][0] = c.g[S2][0];
                call_ptr(c, 0x0039e8b8, 0x001fb89c)?;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.g[S5][0];
                c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x408));
                c.g[A1][0] = c.g[S1][0];
                call_ptr(c, 0x0039e8b8, 0x001fb8b0)?;
                c.g[V1][0] = 0x530000;
                c.g[S0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x5bc8));
                c.g[V0][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x49));
                let t = c.g[V0][0] != 0u64;
                c.g[A0][0] = 0x4a0000;
                if t {
                    break 'b_1fb930;
                } else {
                    'b_1fb920: {
                        'b_1fb91c: {
                            'b_1fb918: {
                                'b_1fb90c: {
                                    'b_1fb8fc: {
                                        'b_1fb8f8: {
                                            c.cycles += 3;
                                            c.g[V0][0] = 0x2;
                                            let t = c.g[S4][0] == c.g[V0][0];
                                            c.g[V0][0] = ((c.g[S4][0] as i64) < 3i64) as u64;
                                            if t {
                                                break 'b_1fb8f8;
                                            } else {
                                                'b_1fb8e8: {
                                                    c.cycles += 2;
                                                    let t = c.g[V0][0] != 0u64;
                                                    c.g[V0][0] = 0x3;
                                                    if t {
                                                        break 'b_1fb8e8;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = c.g[S4][0] == c.g[V0][0];
                                                        c.g[A1][0] = 0x4a0000;
                                                        if t {
                                                            break 'b_1fb90c;
                                                        } else {
                                                            c.cycles += 2;
                                                            let t = true;
                                                            c.g[A0][0] = c.g[S5][0];
                                                            if t {
                                                                break 'b_1fb920;
                                                            } else {
                                                                break 'b_1fb8e8;
                                                            }
                                                        }
                                                    }
                                                }
                                                c.cycles += 2;
                                                let t = (c.g[S4][0] as i64) < 0;
                                                c.g[A1][0] = c.g[S3][0];
                                                if t {
                                                    break 'b_1fb918;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = true;

                                                    if t {
                                                        break 'b_1fb8fc;
                                                    } else {
                                                        break 'b_1fb8f8;
                                                    }
                                                }
                                            }
                                        }
                                        c.cycles += 1;
                                        c.g[A1][0] = c.g[S2][0];
                                        break 'b_1fb8fc;
                                    }
                                    c.cycles += 4;
                                    c.g[A0][0] = c.g[S5][0];
                                    call_ptr(c, 0x0039e948, 0x001fb904)?;
                                    let t = true;
                                    c.g[A1][0] = 0x4a0000;
                                    if t {
                                        break 'b_1fb91c;
                                    } else {
                                        break 'b_1fb90c;
                                    }
                                }
                                c.cycles += 3;
                                c.g[A1][0] = c.g[S1][0];
                                c.g[A0][0] = c.g[S5][0];
                                call_ptr(c, 0x0039e948, 0x001fb918)?;
                                break 'b_1fb918;
                            }
                            c.cycles += 1;
                            c.g[A1][0] = 0x4a0000;
                            break 'b_1fb91c;
                        }
                        c.cycles += 1;
                        c.g[A0][0] = c.g[S5][0];
                        break 'b_1fb920;
                    }
                    c.cycles += 4;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2148));
                    call_ptr(c, 0x0039e8b8, 0x001fb928)?;
                    let t = true;
                    c.g[A0][0] = 0x470000;
                    if t {
                        break 'b_1fb990;
                    } else {
                        break 'b_1fb930;
                    }
                }
            }
            'b_1fb978: {
                c.cycles += 10;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2148));
                call_ptr(c, 0x00317670, 0x001fb938)?;
                c.g[A0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x40));
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039d860, 0x001fb944)?;
                c.g[S3][0] = c.g[V0][0];
                c.g[V1][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x49));
                c.g[V0][0] = 0x2;
                if c.g[V1][0] != c.g[V0][0] {
                    c.g[A0][0] = 0x470000;
                    break 'b_1fb978;
                } else {
                    c.cycles += 8;
                    c.g[A0][0] = 0x470000;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x3f8));
                    call_ptr(c, 0x00317670, 0x001fb964)?;
                    c.g[A0][0] = c.g[S3][0];
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x003a1000, 0x001fb970)?;
                    let t = true;
                    c.g[A0][0] = 0x470000;
                    if t {
                        break 'b_1fb990;
                    } else {
                        break 'b_1fb978;
                    }
                }
            }
            c.cycles += 6;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x3e0));
            call_ptr(c, 0x00317670, 0x001fb980)?;
            c.g[A0][0] = c.g[S3][0];
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x003a1000, 0x001fb98c)?;
            c.g[A0][0] = 0x470000;
            break 'b_1fb990;
        }
        // Caption text id by event kind (s4 = 1 or 2: special strings).
        'b_1fba04: {
            'b_1fb9ec: {
                'b_1fb9d0: {
                    'b_1fb9c0: {
                        c.cycles += 9;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x3c8));
                        call_ptr(c, 0x00317670, 0x001fb998)?;
                        c.g[A0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x40));
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x0039d860, 0x001fb9a4)?;
                        c.g[S3][0] = c.g[V0][0];
                        c.g[V0][0] = 0x1;
                        let t = c.g[S4][0] != c.g[V0][0];
                        c.g[V0][0] = 0x2;
                        if t {
                            break 'b_1fb9c0;
                        } else {
                            c.cycles += 3;
                            c.g[A0][0] = 0x470000;
                            let t = true;
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x3b8));
                            if t {
                                break 'b_1fb9d0;
                            } else {
                                break 'b_1fb9c0;
                            }
                        }
                    }
                    c.cycles += 2;
                    if c.g[S4][0] != c.g[V0][0] {
                        c.g[A0][0] = 0x470000;
                        break 'b_1fb9ec;
                    } else {
                        c.cycles += 2;
                        c.g[A0][0] = 0x470000;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x3a0));
                        break 'b_1fb9d0;
                    }
                }
                c.cycles += 7;
                call_ptr(c, 0x00317670, 0x001fb9d8)?;
                c.g[A0][0] = c.g[S3][0];
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x003a1000, 0x001fb9e4)?;
                let t = true;
                c.g[V1][0] = sx((c.g[S6][0] as u32).wrapping_add(0x40));
                if t {
                    break 'b_1fba04;
                } else {
                    break 'b_1fb9ec;
                }
            }
            c.cycles += 6;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x388));
            call_ptr(c, 0x00317670, 0x001fb9f4)?;
            c.g[A0][0] = c.g[S3][0];
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x003a1000, 0x001fba00)?;
            c.g[V1][0] = sx((c.g[S6][0] as u32).wrapping_add(0x40));
            break 'b_1fba04;
        }
        c.cycles += 13;
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
        c.g[S6][0] = sx((c.g[S6][0] as u32).wrapping_add(0x18));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x248), c.g[V1][0]);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x244), c.g[S6][0]);
        c.g[S2][0] = 0u64;
        c.g[S4][0] = sx((c.g[SP][0] as u32).wrapping_add(0x160));
        c.g[S7][0] = 0x4a0000;
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
        c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x78));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x240), c.g[V0][0]);
        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x248));
        // Six rider rows (s2 = row): character display names, with the player name
        // for the human riders in multi-player mode (+0x49 of 0x535bc8 == 2).
        'l_1fba38: loop {
            'b_1fbb3c: {
                'b_1fbb24: {
                    c.cycles += 22;
                    c.g[V0][0] = sx((c.g[S2][0] as u32) << 2);
                    c.g[A2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                    c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[S6][0] = c.g[A2][0];
                    c.g[S1][0] = c.lw(c.g[V1][0] as u32);
                    c.g[A0][0] = c.g[SP][0];
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x244));
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[S0][0] = c.lw(c.g[V0][0] as u32);
                    c.g[V0][0] = 0x4a0000;
                    c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2170));
                    call_ptr(c, 0x00417828, 0x001fba68)?;
                    c.g[A0][0] = c.g[SP][0];
                    call_ptr(c, 0x00317670, 0x001fba70)?;
                    c.g[A0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x40));
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x001fba7c)?;
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x240));
                    c.g[S3][0] = c.g[V0][0];
                    c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V1][0] as i64)) as u64;
                    if c.g[V0][0] == 0u64 {
                        c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
                        break 'b_1fbb24;
                    } else {
                        'b_1fbb04: {
                            c.cycles += 6;
                            c.g[A0][0] = 0x530000;
                            c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5bc8));
                            c.g[V1][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x49));
                            c.g[A0][0] = 0x2;
                            if c.g[V1][0] != c.g[A0][0] {
                                c.g[A1][0] = c.g[S0][0];
                                break 'b_1fbb04;
                            } else {
                                'b_1fbabc: {
                                    c.cycles += 3;
                                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x238));
                                    let t = c.g[S2][0] == c.g[V0][0];
                                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x23c));
                                    if t {
                                        break 'b_1fbabc;
                                    } else {
                                        c.cycles += 2;
                                        let t = c.g[S2][0] != c.g[V1][0];
                                        c.g[A1][0] = c.g[S0][0];
                                        if t {
                                            break 'b_1fbb04;
                                        } else {
                                            break 'b_1fbabc;
                                        }
                                    }
                                }
                                c.cycles += 18;
                                c.g[A2][0] = c.g[S1][0];
                                c.g[A1][0] = c.g[S0][0];
                                c.g[A0][0] = c.g[FP][0];
                                call_ptr(c, 0x0014eec8, 0x001fbacc)?;
                                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x230));
                                c.g[S0][0] = c.g[V0][0];
                                c.g[A1][0] = c.g[S2][0];
                                call_ptr(c, 0x00146f88, 0x001fbadc)?;
                                c.g[A0][0] = c.g[S4][0];
                                c.g[A1][0] = sx((c.g[S7][0] as u32).wrapping_add(0x24f0));
                                c.g[A2][0] = c.g[S0][0];
                                c.g[A3][0] = c.g[V0][0];
                                call_ptr(c, 0x00417828, 0x001fbaf0)?;
                                c.g[A0][0] = c.g[S3][0];
                                c.g[A1][0] = c.g[S4][0];
                                call_ptr(c, 0x003a0c50, 0x001fbafc)?;
                                let t = true;
                                c.g[S2][0] = c.g[S6][0];
                                if t {
                                    break 'b_1fbb3c;
                                } else {
                                    break 'b_1fbb04;
                                }
                            }
                        }
                        c.cycles += 8;
                        c.g[A0][0] = c.g[FP][0];
                        c.g[A2][0] = c.g[S1][0];
                        call_ptr(c, 0x0014eec8, 0x001fbb10)?;
                        c.g[A0][0] = c.g[S3][0];
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x003a0c50, 0x001fbb1c)?;
                        let t = true;
                        c.g[S2][0] = c.g[S6][0];
                        if t {
                            break 'b_1fbb3c;
                        } else {
                            break 'b_1fbb24;
                        }
                    }
                }
                c.cycles += 6;
                c.g[A1][0] = 0u64;
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                let t = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x001fbb38)?;
                c.g[S2][0] = c.g[S6][0];
                break 'b_1fbb3c;
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S2][0] as i64) < 6i64) as u64;
            if c.g[V0][0] != 0u64 {
                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x248));
                if event_due(c) {
                    deliver_event_here(c, 0x001fba38);
                }
                continue 'l_1fba38;
            } else {
                // High score text, FE sound 0xe and the race-intro announcer line.
                c.cycles += 36;
                c.g[A0][0] = 0x4a0000;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x24f8));
                call_ptr(c, 0x00317670, 0x001fbb54)?;
                c.g[A0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x40));
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039d860, 0x001fbb60)?;
                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x234));
                c.g[S3][0] = c.g[V0][0];
                c.g[A1][0] = 0u64;
                call_ptr(c, 0x00155130, 0x001fbb70)?;
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x0020a1a8, 0x001fbb78)?;
                c.g[A0][0] = c.g[S3][0];
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x003a0c50, 0x001fbb84)?;
                call_ptr(c, 0x0028b180, 0x001fbb8c)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0xe;
                call_ptr(c, 0x00294f78, 0x001fbb98)?;
                call_ptr(c, 0x0028b180, 0x001fbba0)?;
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x002a31c0, 0x001fbba8)?;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x2e0));
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x2d0));
                c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x2c0));
                c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x2b0));
                c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x2a0));
                c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x290));
                c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x280));
                c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x270));
                c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x260));
                c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x250));
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x2f0));
                c.pc = t;
                return Ok(());
            }
        }
    })
}

/// `void cOVState_HEAT_PREEVENT_onGainFocus(cOVState* self)` (0x1fbbd8, `OVScreenRacePre_OnActivate`).
///
/// The pre-race overlay gains focus: runs the base `cUIState::onGainFocus`,
/// then shows the "RIDERPCT_D" rider name text (`sprintf` + `triggerTextFade`)
/// for human rider 1, and for rider 2 as well when the game path
/// (`newRace.cGamePath`, byte 0x535c11) is 2 (split screen). Finally sets the
/// game-comm overlay timeout (`+0x64`) to 1200 when a `cGameComm` exists.
pub fn ov_screen_race_pre_on_activate(c: &mut Cpu) {
    guest(c, |c| {
        'b_1fbca0: {
            c.cycles += 38;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
            c.g[S0][0] = c.g[A0][0];
            call_ptr(c, 0x0039e4c0, 0x001fbbf8)?;
            call_ptr(c, 0x0014dc80, 0x001fbc00)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0x1;
            call_ptr(c, 0x0014dd58, 0x001fbc0c)?;
            c.g[S1][0] = c.g[V0][0];
            call_ptr(c, 0x0014dc80, 0x001fbc14)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0u64;
            call_ptr(c, 0x0014dd58, 0x001fbc20)?;
            c.g[V1][0] = 0x4a0000;
            c.g[A0][0] = c.g[S1][0];
            c.g[S3][0] = sx((c.g[V1][0] as u32).wrapping_add(0x2170));
            c.g[A1][0] = 0u64;
            call_ptr(c, 0x00146e98, 0x001fbc34)?;
            c.g[A0][0] = c.g[SP][0];
            c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
            c.g[A1][0] = c.g[S3][0];
            call_ptr(c, 0x00417828, 0x001fbc44)?;
            c.g[V1][0] = 0x470000;
            c.g[A0][0] = c.g[S0][0];
            c.g[S2][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1b18));
            c.g[A1][0] = c.g[SP][0];
            c.g[A2][0] = c.g[S2][0];
            call_ptr(c, 0x0020a6f0, 0x001fbc5c)?;
            c.g[V1][0] = 0x530000;
            c.g[A0][0] = 0x2;
            c.g[V0][0] = c.lb((c.g[V1][0] as u32).wrapping_add(0x5c11));
            let t = c.g[V0][0] != c.g[A0][0];
            c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x204));
            if t {
                break 'b_1fbca0;
            } else {
                c.cycles += 12;
                c.g[A0][0] = c.g[S1][0];
                c.g[A1][0] = 0x1;
                call_ptr(c, 0x00146e98, 0x001fbc7c)?;
                c.g[A1][0] = c.g[S3][0];
                c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                c.g[A0][0] = c.g[SP][0];
                call_ptr(c, 0x00417828, 0x001fbc8c)?;
                c.g[A0][0] = c.g[S0][0];
                c.g[A2][0] = c.g[S2][0];
                c.g[A1][0] = c.g[SP][0];
                call_ptr(c, 0x0020a6f0, 0x001fbc9c)?;
                c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x204));
                break 'b_1fbca0;
            }
        }
        'b_1fbcac: {
            c.cycles += 2;
            let t = c.g[V1][0] == 0u64;
            c.g[V0][0] = 0x4b0;
            if t {
                break 'b_1fbcac;
            } else {
                c.cycles += 1;
                c.sw((c.g[V1][0] as u32).wrapping_add(0x64), c.g[V0][0]);
                break 'b_1fbcac;
            }
        }
        c.cycles += 7;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
        c.pc = t;
        return Ok(());
    });
}

/// `OVScreenRacePre_dtor(this)` (0x221df0): sets the vtable word at +8 to the
/// UIState vtable (0x474e08), then calls `UIState_dtor(this, a1)`.
pub fn ov_screen_race_pre_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0x470000;
        frame.save_ra(c, 0);
        c.g[V1][0] = c.g[A0][0];
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4e08));
        c.sw((c.g[V1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
        call_ptr(c, 0x0039_e390, 0x0022_1e0c)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `FE_GetEventNameText(kind)` (0x001fd190): the pause-screen text of an event kind 6..=11
/// (jump table): hashes a name through the world's string table. The normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn fe_get_event_name_text(c: &mut Cpu) {
    if c.pc != 0x001fd190 {
        return ee::s04::f_001fd190__generated(c);
    }
    guest(c, |c| {
        'b_1fd258: {
            'b_1fd250: {
                c.cycles += 7;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
                c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x6));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[V0][0] = (c.g[V1][0] < 0x6u64) as u64;
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
                let t = c.g[V0][0] == 0u64;
                c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                if t {
                    break 'b_1fd250;
                } else {
                    'b_1fd224: {
                        'b_1fd218: {
                            'b_1fd208: {
                                'b_1fd1f8: {
                                    'b_1fd1e8: {
                                        'b_1fd1d8: {
                                            c.cycles += 7;
                                            c.g[V0][0] = 0x470000;
                                            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xc0));
                                            c.g[V1][0] =
                                                sx((c.g[V1][0] as u32)
                                                    .wrapping_add(c.g[V0][0] as u32));
                                            c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                                            let t = c.g[A0][0] as u32;
                                            match t {
                                                0x001fd1c8 => {
                                                    c.cycles += 4;
                                                    c.g[V0][0] =
                                                        c.lw(
                                                            (c.g[GP][0] as u32).wrapping_sub(0x848)
                                                        );
                                                    c.g[A0][0] = 0x470000;
                                                    let t = true;
                                                    c.g[A0][0] =
                                                        sx((c.g[A0][0] as u32).wrapping_sub(0x130));
                                                    if t {
                                                        break 'b_1fd224;
                                                    } else {
                                                        break 'b_1fd1d8;
                                                    }
                                                }
                                                0x001fd1d8 => {
                                                    break 'b_1fd1d8;
                                                }
                                                0x001fd1e8 => {
                                                    break 'b_1fd1e8;
                                                }
                                                0x001fd1f8 => {
                                                    break 'b_1fd1f8;
                                                }
                                                0x001fd208 => {
                                                    break 'b_1fd208;
                                                }
                                                0x001fd218 => {
                                                    break 'b_1fd218;
                                                }
                                                _ => {
                                                    c.jump_out(t, 0x001fd190, 0x001fd268);
                                                    return Ok(());
                                                }
                                            }
                                        }
                                        c.cycles += 4;
                                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                        c.g[A0][0] = 0x470000;
                                        let t = true;
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x120));
                                        if t {
                                            break 'b_1fd224;
                                        } else {
                                            break 'b_1fd1e8;
                                        }
                                    }
                                    c.cycles += 4;
                                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                    c.g[A0][0] = 0x470000;
                                    let t = true;
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x110));
                                    if t {
                                        break 'b_1fd224;
                                    } else {
                                        break 'b_1fd1f8;
                                    }
                                }
                                c.cycles += 4;
                                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                c.g[A0][0] = 0x470000;
                                let t = true;
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0xf8));
                                if t {
                                    break 'b_1fd224;
                                } else {
                                    break 'b_1fd208;
                                }
                            }
                            c.cycles += 4;
                            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                            c.g[A0][0] = 0x470000;
                            let t = true;
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0xe8));
                            if t {
                                break 'b_1fd224;
                            } else {
                                break 'b_1fd218;
                            }
                        }
                        c.cycles += 3;
                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                        c.g[A0][0] = 0x470000;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0xd8));
                        break 'b_1fd224;
                    }
                    c.cycles += 11;
                    c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                    c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                    c.g[RA][0] = sx(0x001fd238);
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    call_ptr(c, 0x00317670, 0x001fd238)?;
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                    c.g[A0][0] = c.g[S0][0];
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x001fd248);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, t, 0x001fd248)?;
                    let t = true;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                    if t {
                        break 'b_1fd258;
                    } else {
                        break 'b_1fd250;
                    }
                }
            }
            c.cycles += 2;
            c.g[V0][0] = 0u64;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            break 'b_1fd258;
        }
        c.cycles += 4;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        return Ok(());
    });
}

/// The static options block the screen copies (`cBEOptionInterface` options,
/// 0x288 bytes) every time it reads a field: `0x535610`, 8-byte aligned, so
/// the generated body always takes its doubleword copy loop (its `ldl`/`ldr`
/// loop for a misaligned source is dead code and is not ported).
#[allow(dead_code)]
pub(crate) const OVBO_OPTIONS_STATIC: u32 = 0x0053_5610;

/// The doubleword copy loop of one inlined 0x288-byte options copy: 20
/// iterations of 32 bytes through the four temporaries `tmp` (their last
/// values stay in the registers), `src`/`dst` advance, V0 = end of the
/// loop part of the source. `head` is the loop head the event check reports.
pub(crate) fn ovbo_copy_loop(c: &mut Cpu, src: usize, dst: usize, tmp: [usize; 4], head: u32) {
    c.g[V0][0] = sx((c.g[src][0] as u32).wrapping_add(0x280));
    loop {
        c.cycles += 11;
        for (i, &t) in tmp.iter().enumerate() {
            let a = (c.g[src][0] as u32).wrapping_add(8 * i as u32);
            c.g[t][0] = c.ld(a);
        }
        for (i, &t) in tmp.iter().enumerate() {
            let a = (c.g[dst][0] as u32).wrapping_add(8 * i as u32);
            let v = c.g[t][0];
            c.sd(a, v);
        }
        c.g[src][0] = sx((c.g[src][0] as u32).wrapping_add(0x20));
        let again = c.g[src][0] != c.g[V0][0];
        c.g[dst][0] = sx((c.g[dst][0] as u32).wrapping_add(0x20));
        if !again {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, head);
        }
    }
}

/// The last 8 bytes of the options copy (`ldl`/`ldr` + `sdl`/`sdr` through
/// register `reg`), source and destination in the registers `src` and `dst`.
pub(crate) fn ovbo_copy_tail(c: &mut Cpu, reg: usize, src: usize, dst: usize) {
    let s = c.g[src][0] as u32;
    let d = c.g[dst][0] as u32;
    c.ldl(reg, s.wrapping_add(7));
    c.ldr(reg, s);
    c.sdl(reg, d.wrapping_add(7));
    c.sdr(reg, d);
}

/// `A0 = 0x470000 - sub` (a string address in the screen's string table).
pub(crate) fn ovbo_str_a0(c: &mut Cpu, sub: u32) {
    c.g[A0][0] = sx(0x0047_0000u32.wrapping_sub(sub));
}

/// Hash the string in A0, then look the widget up in the screen
/// (`UIScreen_FindObject(self->mpScreen, hash)`).
pub(crate) fn ovbo_find(c: &mut Cpu, ret_hash: u32, ret_find: u32) -> Flow {
    call_ptr(c, 0x0031_7670, ret_hash)?;
    let screen = (c.g[S4][0] as u32).wrapping_add(0x40);
    c.g[A0][0] = c.lw(screen);
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, 0x0039_d860, ret_find)
}

/// `UIListBox_AddItemStrId(list = S0, hash(string at 0x470000 - sub), id)`.
pub(crate) fn ovbo_add_entry(c: &mut Cpu, sub: u32, ret_hash: u32, id: u32, ret_add: u32) -> Flow {
    ovbo_str_a0(c, sub);
    call_ptr(c, 0x0031_7670, ret_hash)?;
    c.g[A1][0] = c.g[V0][0];
    c.g[A2][0] = id as u64;
    c.g[A0][0] = c.g[S0][0];
    call_ptr(c, 0x0039_a568, ret_add)
}

/// `if UIListBox_FindItemByValue(list = S0, value in A1, &sel = A2) {
/// UIListBox_SetSelection(list, sel) }` after the caller loaded A1/A2; the
/// found byte is at `sel_off(sp)`.
pub(crate) fn ovbo_select(c: &mut Cpu, sel_off: u32, ret_find: u32, ret_set: u32) -> Flow {
    call_ptr(c, 0x0039_a768, ret_find)?;
    let found = c.g[V0][0] != 0;
    let a = sp(c).wrapping_add(sel_off);
    c.g[A1][0] = c.lbu(a);
    if found {
        c.cycles += 2;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0039_a7a8, ret_set)?;
    }
    Ok(())
}

/// `cOVStatePauseOptions::onCreateScreen` (the pause menu's "options" page, SSX 3
/// `OVScreenBEOptions_Init(self)`, 0x1f9188): creates the "37beoptions"
/// screen, then fills its widgets from the saved options (a 0x288-byte
/// options block at 0x535610 that the compiled code copies to the stack for
/// every field it reads): HUD list box, the two camera list boxes (chase
/// camera algorithm per player), the music/SFX/speech volume sliders (and
/// the audio system's volumes), the DJ-speech and arcade-SFX list boxes, and
/// the voice-chat widgets (visible only in a network game).
///
/// Registers and cycles follow the generated body exactly.
pub fn ov_screen_be_options_init(c: &mut Cpu) {
    guest(c, |c| {
        // Prologue: 0x350-byte frame, s0-s7/fp/ra saved.
        c.cycles += 23;
        let f = Frame::open(c, 0x350);
        f.save(c, S4, 0x300);
        c.g[S4][0] = c.g[A0][0];
        f.save(c, S1, 0x330);
        f.save(c, S2, 0x320);
        ovbo_str_a0(c, 0x728);
        f.save(c, S3, 0x310);
        f.save(c, S5, 0x2f0);
        f.save(c, S6, 0x2e0);
        f.save(c, S7, 0x2d0);
        f.save(c, FP, 0x2c0);
        f.save_ra(c, 0x2b0);
        f.save(c, S0, 0x340);
        let engine = (c.g[S4][0] as u32).wrapping_add(0x10);
        c.g[S0][0] = c.lw(engine);

        // self->mpScreen = engine->addScreenByHashName(self, hash("37beoptions")).
        call_ptr(c, 0x0031_7670, 0x001f_91c8)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S4][0];
        c.g[A3][0] = 0;
        call_ptr(c, 0x0039_7cc0, 0x001f_91dc)?;
        let none = c.g[V0][0] == 0;
        let at = (c.g[S4][0] as u32).wrapping_add(0x40);
        c.sw(at, c.g[V0][0]);
        if !none {
            // screen->playFrame(0, 0)
            c.cycles += 4;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            call_ptr(c, 0x0039_c870, 0x001f_91f4)?;
        }

        // cOVState::onCreateScreen(self); cBE_getBE()->getInterface(4).
        c.cycles += 21;
        c.g[A0][0] = c.g[S4][0];
        c.g[S1][0] = 0x004a_0000;
        call_ptr(c, 0x0020_a380, 0x001f_9200)?;
        c.g[S3][0] = 0x004a_0000;
        call_ptr(c, 0x0014_dc80, 0x001f_9208)?;
        c.g[S5][0] = 0x004a_0000;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 4;
        call_ptr(c, 0x0014_dd58, 0x001f_9218)?;
        c.g[S6][0] = 0x004a_0000;
        c.g[A0][0] = 0x004a_0000;
        c.g[S7][0] = 0x004a_0000;
        c.g[A0][0] = sx(0x004a_0000u32.wrapping_add(0x2498));
        c.g[FP][0] = 0x004a_0000;
        ovbo_find(c, 0x001f_9230, 0x001f_923c)?;

        // HUD list box ("0lb"): Full / Minimal / None, current = options.f30.
        c.g[S0][0] = c.g[V0][0];
        c.g[A0][0] = 0x0047_0000;
        if c.g[S0][0] != 0 {
            c.cycles += 25;
            ovbo_add_entry(c, 0x718, 0x001f_9250, 0, 0x001f_9260)?;
            ovbo_add_entry(c, 0x708, 0x001f_926c, 1, 0x001f_927c)?;
            ovbo_add_entry(c, 0x6f0, 0x001f_9288, 2, 0x001f_9298)?;
            c.g[V1][0] = 0x0053_0000;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x5610));
            c.g[V0][0] = c.g[V1][0] & 7;
            c.g[A0][0] = sx(sp(c).wrapping_add(0x10));
            c.cycles += 1;
            ovbo_copy_loop(c, V1, A0, [T1, A1, A2, A3], 0x001f_9308);
            c.cycles += 11;
            ovbo_copy_tail(c, T0, V1, A0);
            c.g[A2][0] = c.g[SP][0];
            let opt = sp(c).wrapping_add(0x10);
            c.g[A1][0] = c.lw(opt);
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = sx((c.g[A1][0] as u32) >> 30);
            ovbo_select(c, 0, 0x001f_9358, 0x001f_9368)?;
        }

        // Camera list box of player 1 ("1lb") and its "camera" label.
        c.cycles += 8;
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x24a0));
        ovbo_find(c, 0x001f_9370, 0x001f_937c)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A0][0] = 0x0047_0000;
        if c.g[S0][0] != 0 {
            c.cycles += 31;
            ovbo_add_entry(c, 0x6e0, 0x001f_9390, 0x3c, 0x001f_93a0)?;
            ovbo_add_entry(c, 0x6d0, 0x001f_93ac, 0x3d, 0x001f_93bc)?;
            ovbo_add_entry(c, 0x6c0, 0x001f_93c8, 0x3e, 0x001f_93d8)?;
            // camera = world->views->PlayerCamera[0]; algo = controller->+0x18
            let world = (c.g[GP][0] as u32).wrapping_sub(0x848);
            c.g[A1][0] = c.lw(world);
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = c.g[SP][0];
            let a = (c.g[A1][0] as u32).wrapping_add(0x84);
            c.g[V0][0] = c.lw(a);
            let a = (c.g[V0][0] as u32).wrapping_add(0x84);
            c.g[V1][0] = c.lw(a);
            let a = (c.g[V1][0] as u32).wrapping_add(4);
            c.g[A1][0] = c.lw(a);
            let a = (c.g[A1][0] as u32).wrapping_add(0xa8);
            c.g[V0][0] = c.lw(a);
            let a = (c.g[V0][0] as u32).wrapping_add(0x18);
            c.g[A1][0] = c.lw(a);
            ovbo_select(c, 0, 0x001f_93fc, 0x001f_940c)?;

            c.cycles += 9;
            ovbo_str_a0(c, 0x6b0);
            ovbo_find(c, 0x001f_9418, 0x001f_9424)?;
            c.g[S2][0] = c.g[V0][0];
            let world = (c.g[GP][0] as u32).wrapping_sub(0x848);
            c.g[V0][0] = c.lw(world);
            if c.g[S2][0] != 0 {
                ovbo_camera_label(c, 0x001f_944c, 0x001f_945c, 0x001f_9474, 0x001f_9480, 1)?;
            }
        }

        // Camera list box of player 2 ("2lb") and its label.
        c.cycles += 8;
        c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x24a8));
        ovbo_find(c, 0x001f_9488, 0x001f_9494)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A0][0] = 0x0047_0000;
        if c.g[S0][0] != 0 {
            c.cycles += 27;
            ovbo_add_entry(c, 0x6e0, 0x001f_94a8, 0x3c, 0x001f_94b8)?;
            ovbo_add_entry(c, 0x6d0, 0x001f_94c4, 0x3d, 0x001f_94d4)?;
            ovbo_add_entry(c, 0x6c0, 0x001f_94e0, 0x3e, 0x001f_94f0)?;
            let world = (c.g[GP][0] as u32).wrapping_sub(0x848);
            c.g[V1][0] = c.lw(world);
            let a = (c.g[V1][0] as u32).wrapping_add(0x84);
            c.g[A0][0] = c.lw(a);
            let a = (c.g[A0][0] as u32).wrapping_add(0x84);
            c.g[V1][0] = c.lw(a);
            let a = (c.g[V1][0] as u32).wrapping_add(0x10);
            c.g[V0][0] = c.lw(a);
            c.g[V0][0] = (c.g[V0][0] < 2) as u64;
            c.g[A0][0] = 0x0047_0000;
            if c.g[V0][0] == 0 {
                // two or more views: select by PlayerCamera[1]'s algorithm
                c.cycles += 8;
                let a = (c.g[V1][0] as u32).wrapping_add(8);
                c.g[V1][0] = c.lw(a);
                c.g[A0][0] = c.g[S0][0];
                c.g[A2][0] = c.g[SP][0];
                let a = (c.g[V1][0] as u32).wrapping_add(0xa8);
                c.g[V0][0] = c.lw(a);
                let a = (c.g[V0][0] as u32).wrapping_add(0x18);
                c.g[A1][0] = c.lw(a);
                ovbo_select(c, 0, 0x001f_9524, 0x001f_9534)?;
                c.cycles += 1;
                c.g[A0][0] = 0x0047_0000;
            }
            c.cycles += 8;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x690));
            ovbo_find(c, 0x001f_9540, 0x001f_954c)?;
            c.g[S2][0] = c.g[V0][0];
            let world = (c.g[GP][0] as u32).wrapping_sub(0x848);
            c.g[V0][0] = c.lw(world);
            if c.g[S2][0] != 0 {
                ovbo_camera_label(c, 0x001f_9574, 0x001f_9584, 0x001f_959c, 0x001f_95a8, 2)?;
            }
        }

        // Music volume slider ("4sl"): 12 steps, value options.f0; applies it.
        c.cycles += 8;
        c.g[A0][0] = sx((c.g[S5][0] as u32).wrapping_add(0x24b0));
        ovbo_find(c, 0x001f_95b0, 0x001f_95bc)?;
        c.g[A2][0] = c.g[V0][0];
        c.g[V1][0] = 0x0053_0000;
        if c.g[A2][0] != 0 {
            ovbo_slider(
                c,
                &OvboSlider {
                    heads: [0x001f_9644, 0x001f_9700],
                    tmps: [[A3, T0, T1, V1], [A2, A3, T0, T1]],
                    tails: [A3, V0],
                    cycles: [13, 7],
                    field: OvboField::Low4,
                    ret_audio: 0x001f_9690,
                    ret_set: 0x001f_9748,
                    set: 0x0028_7410,
                },
            )?;
        }
        // SFX volume slider ("5sl"): value options.f8 (bits 8..11).
        c.cycles += 8;
        c.g[A0][0] = sx((c.g[S6][0] as u32).wrapping_add(0x24b8));
        ovbo_find(c, 0x001f_9750, 0x001f_975c)?;
        c.g[A2][0] = c.g[V0][0];
        c.g[V1][0] = 0x0053_0000;
        if c.g[A2][0] != 0 {
            ovbo_slider(
                c,
                &OvboSlider {
                    heads: [0x001f_97e4, 0x001f_98a4],
                    tmps: [[T1, V1, A3, T0], [T0, T1, A2, A3]],
                    tails: [T1, T0],
                    cycles: [14, 8],
                    field: OvboField::Bits8,
                    ret_audio: 0x001f_9834,
                    ret_set: 0x001f_98f0,
                    set: 0x0028_7488,
                },
            )?;
        }
        // Speech volume slider ("6sl"): value options.f4 (bits 4..7).
        c.cycles += 8;
        c.g[A0][0] = sx((c.g[S7][0] as u32).wrapping_add(0x24c0));
        ovbo_find(c, 0x001f_98f8, 0x001f_9904)?;
        c.g[A2][0] = c.g[V0][0];
        c.g[V1][0] = 0x0053_0000;
        if c.g[A2][0] != 0 {
            ovbo_slider(
                c,
                &OvboSlider {
                    heads: [0x001f_998c, 0x001f_9a48],
                    tmps: [[A3, T0, T1, V1], [A2, A3, T0, T1]],
                    tails: [A3, V0],
                    cycles: [13, 7],
                    field: OvboField::Bits4,
                    ret_audio: 0x001f_99d8,
                    ret_set: 0x001f_9a90,
                    set: 0x0028_7520,
                },
            )?;
        }

        // DJ speech on/off list box ("7lb").
        c.cycles += 8;
        c.g[A0][0] = sx((c.g[FP][0] as u32).wrapping_add(0x24c8));
        ovbo_find(c, 0x001f_9a98, 0x001f_9aa4)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A0][0] = 0x0047_0000;
        if c.g[S0][0] != 0 {
            c.cycles += 20;
            ovbo_add_entry(c, 0x680, 0x001f_9ab8, 0, 0x001f_9ac8)?;
            ovbo_add_entry(c, 0x670, 0x001f_9ad4, 1, 0x001f_9ae4)?;
            c.g[V1][0] = 0x0053_0000;
            c.g[A0][0] = sx(sp(c).wrapping_add(0x10));
            c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x5610));
            c.g[S2][0] = c.g[V1][0];
            c.g[V0][0] = c.g[A1][0] & 7;
            c.g[S1][0] = c.g[A0][0];
            c.cycles += 2;
            c.g[A2][0] = sx(sp(c).wrapping_add(0x2a0));
            ovbo_copy_loop(c, A1, A0, [T1, V1, A3, T0], 0x001f_9b64);
            c.cycles += 11;
            ovbo_copy_tail(c, T1, A1, A0);
            let opt = sp(c).wrapping_add(0x10);
            c.g[A1][0] = c.lw(opt);
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = sx((c.g[A1][0] as u32) >> 17);
            c.g[A1][0] &= 1;
            ovbo_select(c, 0x2a0, 0x001f_9bb4, 0x001f_9bc4)?;
            // Cfg_PO() copy again, then SetDJSpeech(audio, cfg.iUseDJSpeech != 0).
            c.cycles += 7;
            call_ptr(c, 0x0028_b180, 0x001f_9bcc)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[V1][0] = sx((c.g[S2][0] as u32).wrapping_add(0x5610));
            c.g[V0][0] = c.g[V1][0] & 7;
            c.g[A1][0] = c.g[S1][0];
            c.cycles += 1;
            ovbo_copy_loop(c, V1, A1, [T0, T1, A2, A3], 0x001f_9c3c);
            c.cycles += 8;
            ovbo_copy_tail(c, T0, V1, A1);
            let opt = sp(c).wrapping_add(0x10);
            c.g[A1][0] = c.lw(opt);
            c.g[A1][0] = sx(((c.g[A1][0] as u32) as i32 >> 17) as u32);
            c.g[A1][0] &= 1;
            call_ptr(c, 0x0028_7558, 0x001f_9c88)?;
        }

        // Arcade SFX on/off list box ("8lb").
        c.cycles += 9;
        c.g[A3][0] = 0x004a_0000;
        c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(0x24d0));
        ovbo_find(c, 0x001f_9c94, 0x001f_9ca0)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A0][0] = 0x0047_0000;
        if c.g[S0][0] != 0 {
            c.cycles += 20;
            ovbo_add_entry(c, 0x680, 0x001f_9cb4, 0, 0x001f_9cc4)?;
            ovbo_add_entry(c, 0x670, 0x001f_9cd0, 1, 0x001f_9ce0)?;
            c.g[V1][0] = 0x0053_0000;
            c.g[A0][0] = sx(sp(c).wrapping_add(0x10));
            c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x5610));
            c.g[S2][0] = c.g[V1][0];
            c.g[V0][0] = c.g[A1][0] & 7;
            c.g[S1][0] = c.g[A0][0];
            c.cycles += 2;
            c.g[A2][0] = sx(sp(c).wrapping_add(0x2a1));
            ovbo_copy_loop(c, A1, A0, [T0, T1, V1, A3], 0x001f_9d60);
            c.cycles += 10;
            ovbo_copy_tail(c, T0, A1, A0);
            let opt = sp(c).wrapping_add(0x12);
            c.g[A1][0] = c.lhu(opt);
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] &= 1;
            ovbo_select(c, 0x2a1, 0x001f_9dac, 0x001f_9dbc)?;
            c.cycles += 7;
            call_ptr(c, 0x0028_b180, 0x001f_9dc4)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[V1][0] = sx((c.g[S2][0] as u32).wrapping_add(0x5610));
            c.g[V0][0] = c.g[V1][0] & 7;
            c.g[A1][0] = c.g[S1][0];
            c.cycles += 1;
            ovbo_copy_loop(c, V1, A1, [A2, A3, T0, T1], 0x001f_9e34);
            c.cycles += 7;
            ovbo_copy_tail(c, V0, V1, A1);
            let opt = sp(c).wrapping_add(0x12);
            c.g[A1][0] = c.lh(opt);
            c.g[A1][0] &= 1;
            call_ptr(c, 0x0028_75d0, 0x001f_9e7c)?;
        }

        // "EATalkPair": hidden and static (shown again for network games below).
        c.cycles += 10;
        c.g[T1][0] = 0x0047_0000;
        c.g[S1][0] = sx((c.g[T1][0] as u32).wrapping_sub(0x660));
        c.g[A0][0] = c.g[S1][0];
        ovbo_find(c, 0x001f_9e8c, 0x001f_9e98)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        if c.g[S0][0] != 0 {
            c.cycles += 11;
            ovbo_vcall(c, S0, V0, V1, 0x48, 0x4c, 0x001f_9eb8)?;
            c.g[A1][0] = 1;
            ovbo_vcall(c, S0, V0, V1, 0x40, 0x44, 0x001f_9ed0)?;
        }

        // Network game: voice-chat widgets; otherwise grey out the label.
        // `end0` paths charge one more cycle before the epilogue.
        'end: {
            'end0: {
                c.cycles += 8;
                let world = (c.g[GP][0] as u32).wrapping_sub(0x848);
                c.g[V0][0] = c.lw(world);
                c.g[A1][0] = 7;
                let a = (c.g[V0][0] as u32).wrapping_add(0x78);
                c.g[A0][0] = c.lw(a);
                call_ptr(c, 0x0014_dd58, 0x001f_9ee0)?;
                c.g[V0][0] = 0x0053_0000;
                let a = (c.g[V0][0] as u32).wrapping_add(0x4b30);
                c.g[V1][0] = c.lw(a);
                c.g[V0][0] = 0x004a_0000;
                if c.g[V1][0] == 0 {
                    // Not a network game: the talk label is static and hidden.
                    c.cycles += 8;
                    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x24d8));
                    ovbo_find(c, 0x001f_a04c, 0x001f_a058)?;
                    c.g[S0][0] = c.g[V0][0];
                    c.g[A1][0] = 1;
                    if c.g[S0][0] == 0 {
                        break 'end0;
                    }
                    c.cycles += 27;
                    ovbo_vcall(c, S0, V0, V1, 0xc8, 0xcc, 0x001f_a078)?;
                    // staticColour = *(0x4c66c8) (16 bytes at +0x88).
                    c.g[A0][0] = 0x004c_0000;
                    c.g[A1][0] = 1;
                    c.g[T0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x66c8));
                    let t0 = c.g[T0][0] as u32;
                    let s0 = c.g[S0][0] as u32;
                    c.ldl(V1, t0.wrapping_add(7));
                    c.ldr(V1, t0);
                    c.ldl(A2, t0.wrapping_add(0xf));
                    c.ldr(A2, t0.wrapping_add(8));
                    c.sdl(V1, s0.wrapping_add(0x8f));
                    c.sdr(V1, s0.wrapping_add(0x88));
                    c.sdl(A2, s0.wrapping_add(0x97));
                    c.sdr(A2, s0.wrapping_add(0x90));
                    ovbo_vcall(c, S0, V0, V1, 0x40, 0x44, 0x001f_a0b8)?;
                    c.g[A1][0] = 0;
                    ovbo_vcall(c, S0, V0, V1, 0x48, 0x4c, 0x001f_a0d0)?;
                    break 'end0;
                }
                // Network game.
                c.cycles += 21;
                c.g[A0][0] = c.g[S1][0];
                ovbo_find(c, 0x001f_9ef8, 0x001f_9f04)?;
                c.g[V1][0] = 0x004a_0000;
                c.g[S2][0] = c.g[V0][0];
                c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x24d8));
                ovbo_find(c, 0x001f_9f14, 0x001f_9f20)?;
                c.g[A0][0] = 0x0047_0000;
                c.g[S3][0] = c.g[V0][0];
                ovbo_str_a0(c, 0x650);
                ovbo_find(c, 0x001f_9f30, 0x001f_9f3c)?;
                c.g[S1][0] = c.g[V0][0];
                if c.g[S2][0] == 0 {
                    break 'end0;
                }
                c.cycles += 2;
                f.restore(c, S0, 0x340);
                if c.g[S3][0] == 0 {
                    break 'end;
                }
                c.cycles += 2;
                c.g[A1][0] = 1;
                if c.g[S1][0] == 0 {
                    break 'end;
                }
                c.cycles += 45;
                c.g[S0][0] = 0xa;
                // talkPair->setVisible(1) / setStatic(0), then talk item and slider.
                ovbo_vcall(c, S2, V1, V0, 0x48, 0x4c, 0x001f_9f6c)?;
                c.g[A1][0] = 0;
                ovbo_vcall(c, S2, V1, V0, 0x40, 0x44, 0x001f_9f84)?;
                c.g[A1][0] = 1;
                ovbo_vcall(c, S3, V1, V0, 0x48, 0x4c, 0x001f_9f9c)?;
                c.g[A1][0] = 0;
                ovbo_vcall(c, S3, V1, V0, 0x40, 0x44, 0x001f_9fb4)?;
                c.g[A1][0] = 1;
                ovbo_vcall(c, S1, V1, V0, 0x48, 0x4c, 0x001f_9fcc)?;
                c.g[A1][0] = 0;
                ovbo_vcall(c, S1, V1, V0, 0x40, 0x44, 0x001f_9fe4)?;
                // slider: 10 values, value = level / 10; VoipMan level query.
                let a = (c.g[S1][0] as u32).wrapping_add(0x78);
                c.sw(a, c.g[S0][0]);
                c.g[A1][0] = sx(sp(c).wrapping_add(0x2a4));
                let voip = (c.g[GP][0] as u32).wrapping_add(0x304);
                c.g[A0][0] = c.lw(voip);
                c.g[A2][0] = sx(sp(c).wrapping_add(0x2a8));
                call_ptr(c, 0x0026_6f90, 0x001f_9ff8)?;
                let a = sp(c).wrapping_add(0x2a4);
                c.g[V1][0] = c.lw(a);
                c.g[V0][0] = 0xa;
                c.cycles += 7;
                c.div(0, V1, S0);
                c.g[V1][0] = c.lo[0];
                let a = (c.g[S1][0] as u32).wrapping_add(0x74);
                c.sw(a, c.g[V1][0]);
                let a = sp(c).wrapping_add(0x2a4);
                c.g[V0][0] = c.lw(a);
                c.g[V0][0] = ((c.g[V0][0] as i64) < 10) as u64;
                c.cycles += 4;
                let voip = (c.g[GP][0] as u32).wrapping_add(0x304);
                c.g[A0][0] = c.lw(voip);
                if c.g[V0][0] == 0 {
                    // level >= 10: VoipMan::Activate
                    call_ptr(c, 0x0026_6e30, 0x001f_a03c)?;
                } else {
                    // level < 10: VoipMan::Deactivate
                    call_ptr(c, 0x0026_6e88, 0x001f_a02c)?;
                }
                f.restore(c, S0, 0x340);
                break 'end;
            }
            c.cycles += 1;
            f.restore(c, S0, 0x340);
        }

        // Epilogue.
        c.cycles += 11;
        f.restore(c, S1, 0x330);
        f.restore(c, S2, 0x320);
        f.restore(c, S3, 0x310);
        f.restore(c, S4, 0x300);
        f.restore(c, S5, 0x2f0);
        f.restore(c, S6, 0x2e0);
        f.restore(c, S7, 0x2d0);
        f.restore(c, FP, 0x2c0);
        f.restore_ra(c, 0x2b0);
        f.close(c);
        Ok(())
    });
}

/// `lw off(reg)` as the generated body reads it (sign-extended).
pub(crate) fn ovbo_lw(c: &mut Cpu, reg: usize, off: u32) -> u64 {
    let a = (c.g[reg][0] as u32).wrapping_add(off);
    c.lw(a)
}

/// Virtual call of a gcc 2.9x vtable slot: `vt = *(obj + 8)` into register
/// `vt`, `A0 = obj + (lh vt+delta_off)`, `fn = lw vt+fn_off` into register
/// `func`, then `jalr func` returning to `ret` (A1 = the argument, set by the
/// caller).
pub(crate) fn ovbo_vcall(
    c: &mut Cpu,
    obj: usize,
    vt: usize,
    func: usize,
    delta_off: u32,
    fn_off: u32,
    ret: u32,
) -> Flow {
    c.g[vt][0] = ovbo_lw(c, obj, 8);
    let a = (c.g[vt][0] as u32).wrapping_add(delta_off);
    c.g[A0][0] = c.lh(a);
    let a = (c.g[vt][0] as u32).wrapping_add(fn_off);
    c.g[func][0] = c.lw(a);
    let target = c.g[func][0] as u32;
    c.g[A0][0] = sx((c.g[obj][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, target, ret)
}

/// How a slider's value is cut out of the copied options word at `sp+0x10`.
#[derive(Clone, Copy)]
pub(crate) enum OvboField {
    /// `lw & 0xf`
    Low4,
    /// `(lw >> 8) & 0xf`
    Bits8,
    /// `lbu >> 4`
    Bits4,
}

/// Read the slider value from the stack copy of the options.
pub(crate) fn ovbo_field(c: &mut Cpu, field: OvboField) -> u64 {
    let a = sp(c).wrapping_add(0x10);
    match field {
        OvboField::Low4 => c.lw(a) & 0xf,
        OvboField::Bits8 => sx((c.lw(a) as u32) >> 8) & 0xf,
        OvboField::Bits4 => c.lbu(a) >> 4,
    }
}

/// One volume slider of the options screen (A2 = widget, V1 = 0x530000):
/// 12 values, current value from a copy of the options block (widget +0x74),
/// then the audio system is told the volume from a second copy
/// (`SSXAudioSystem_Set*Volume(GetInstance(), field)`).
pub(crate) struct OvboSlider {
    /// Loop heads and temporaries of the first and second copy.
    heads: [u32; 2],
    tmps: [[usize; 4]; 2],
    /// Tail registers of the two copies.
    tails: [usize; 2],
    /// Cycles of the blocks after the first and second copy.
    cycles: [u64; 2],
    field: OvboField,
    /// Return of `SSXAudioSystem_GetInstance` and of the volume call.
    ret_audio: u32,
    ret_set: u32,
    set: u32,
}

pub(crate) fn ovbo_slider(c: &mut Cpu, p: &OvboSlider) -> Flow {
    c.cycles += 8;
    c.g[V0][0] = 0xc;
    c.g[A0][0] = sx(sp(c).wrapping_add(0x10));
    c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x5610));
    let a = (c.g[A2][0] as u32).wrapping_add(0x78);
    c.sw(a, c.g[V0][0]);
    c.g[S2][0] = c.g[V1][0];
    c.g[V0][0] = c.g[A1][0] & 7;
    c.g[S1][0] = c.g[A0][0];
    c.cycles += 1;
    ovbo_copy_loop(c, A1, A0, p.tmps[0], p.heads[0]);
    c.cycles += p.cycles[0];
    ovbo_copy_tail(c, p.tails[0], A1, A0);
    c.g[V0][0] = ovbo_field(c, p.field);
    let a = (c.g[A2][0] as u32).wrapping_add(0x74);
    c.sw(a, c.g[V0][0]);
    call_ptr(c, 0x0028_b180, p.ret_audio)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[V1][0] = sx((c.g[S2][0] as u32).wrapping_add(0x5610));
    c.g[V0][0] = c.g[V1][0] & 7;
    c.g[A1][0] = c.g[S1][0];
    c.cycles += 1;
    ovbo_copy_loop(c, V1, A1, p.tmps[1], p.heads[1]);
    c.cycles += p.cycles[1];
    ovbo_copy_tail(c, p.tails[1], V1, A1);
    c.g[A1][0] = ovbo_field(c, p.field);
    call_ptr(c, p.set, p.ret_set)
}

/// The "camera N" label under a camera list box (`setFormatted("kT_CMNCameraVar",
/// N)` through the widget's vtable), with the world in V0. The two copies of
/// this code differ in where S0/A1 are loaded (`player` 1: S0 before the
/// test, A1 after).
pub(crate) fn ovbo_camera_label(
    c: &mut Cpu,
    ret_hash: u32,
    ret_call: u32,
    ret_fmt: u32,
    ret_set: u32,
    player: u32,
) -> Flow {
    c.cycles += 13;
    ovbo_str_a0(c, 0x6a0);
    c.g[S0][0] = ovbo_lw(c, V0, 0x8c);
    c.g[S1][0] = ovbo_lw(c, S0, 4);
    c.g[V0][0] = {
        let a = (c.g[S1][0] as u32).wrapping_add(0x20);
        c.lh(a)
    };
    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
    call_ptr(c, 0x0031_7670, ret_hash)?;
    c.g[V1][0] = ovbo_lw(c, S1, 0x24);
    c.g[A0][0] = c.g[S0][0];
    let target = c.g[V1][0] as u32;
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, target, ret_call)?;
    let none = c.g[V0][0] == 0;
    if player == 1 {
        c.g[S0][0] = sx(sp(c).wrapping_add(0x10));
    } else {
        c.g[A1][0] = c.g[V0][0];
    }
    if !none {
        c.cycles += 7;
        if player == 1 {
            c.g[A1][0] = c.g[V0][0];
        } else {
            c.g[S0][0] = sx(sp(c).wrapping_add(0x10));
        }
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = player as u64;
        call_ptr(c, 0x002c_26d0, ret_fmt)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = c.g[S0][0];
        call_ptr(c, 0x003a_0e90, ret_set)?;
    }
    Ok(())
}

/// `void OVScreenFreestylePre_Init(OVScreenFreestylePre* self)` (0x1fbd20): the
/// freestyle pre-event overlay. Creates the "freestyle_pre" and "OV_darkblue"
/// screens (starting their threads), runs the overlay template init, then
/// fills the screen's text objects: the event caption (location name plus the
/// slopestyle / big air / half-pipe title), the heat objectives and tabs (path
/// 0: heat 1 / 2 / final; other paths: quick play or multiplayer headings),
/// the three rival rows (name, run 1, run 2 and total scores, skipping the
/// human rider's own rank), the player rows and the record score.
///
/// Kept as the generated block structure (every register, cycle and call in
/// order) with the calls through `call_ptr` and unwinds through `?`.
#[allow(unreachable_code, unused_labels, unused_assignments, unused_variables)]
pub fn ov_screen_freestyle_pre_init(c: &mut Cpu) {
    guest(c, |c| {
        'b_1fbd8c: {
            c.cycles += 23;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x840));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x800));
            c.g[S3][0] = c.g[A0][0];
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x820));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x810));
            c.g[A0][0] = 0x470000;
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x7f0));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x370));
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x7e0));
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x7d0));
            c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x7c0));
            c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x7b0));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x7a0), c.g[RA][0]);
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x830));
            c.g[RA][0] = sx(0x001fbd60);
            c.g[S0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x10));
            call_ptr(c, 0x00317670, 0x001fbd60)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S3][0];
            c.g[RA][0] = sx(0x001fbd74);
            c.g[A3][0] = 0u64;
            call_ptr(c, 0x00397cc0, 0x001fbd74)?;
            let t = c.g[V0][0] == 0u64;
            c.sw((c.g[S3][0] as u32).wrapping_add(0x40), c.g[V0][0]);
            if t {
                break 'b_1fbd8c;
            } else {
                c.cycles += 4;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0u64;
                c.g[RA][0] = sx(0x001fbd8c);
                c.g[A2][0] = 0u64;
                call_ptr(c, 0x0039c870, 0x001fbd8c)?;
                break 'b_1fbd8c;
            }
        }
        'b_1fbdc4: {
            c.cycles += 11;
            c.g[A0][0] = 0x470000;
            c.g[S0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x10));
            c.g[RA][0] = sx(0x001fbd9c);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1c88));
            call_ptr(c, 0x00317670, 0x001fbd9c)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S3][0];
            c.g[RA][0] = sx(0x001fbdb0);
            c.g[A3][0] = 0u64;
            call_ptr(c, 0x00397cc0, 0x001fbdb0)?;
            let t = c.g[V0][0] == 0u64;
            c.g[A1][0] = 0u64;
            if t {
                break 'b_1fbdc4;
            } else {
                c.cycles += 3;
                c.g[A0][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x001fbdc4);
                c.g[A2][0] = 0u64;
                call_ptr(c, 0x0039c870, 0x001fbdc4)?;
                break 'b_1fbdc4;
            }
        }
        'b_1fbef8: {
            'b_1fbecc: {
                'b_1fbec4: {
                    'b_1fbeb4: {
                        c.cycles += 45;
                        c.g[RA][0] = sx(0x001fbdcc);
                        c.g[A0][0] = c.g[S3][0];
                        call_ptr(c, 0x0020d1d8, 0x001fbdcc)?;
                        c.g[RA][0] = sx(0x001fbdd4);
                        call_ptr(c, 0x0014dc80, 0x001fbdd4)?;
                        c.g[A0][0] = c.g[V0][0];
                        c.g[RA][0] = sx(0x001fbde0);
                        c.g[A1][0] = 0u64;
                        call_ptr(c, 0x0014dd58, 0x001fbde0)?;
                        c.g[RA][0] = sx(0x001fbde8);
                        c.g[S0][0] = c.g[V0][0];
                        call_ptr(c, 0x0014dc80, 0x001fbde8)?;
                        c.g[A0][0] = c.g[V0][0];
                        c.g[RA][0] = sx(0x001fbdf4);
                        c.g[A1][0] = 0x1;
                        call_ptr(c, 0x0014dd58, 0x001fbdf4)?;
                        c.g[RA][0] = sx(0x001fbdfc);
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x780), c.g[V0][0]);
                        call_ptr(c, 0x0014dc80, 0x001fbdfc)?;
                        c.g[A0][0] = c.g[V0][0];
                        c.g[RA][0] = sx(0x001fbe08);
                        c.g[A1][0] = 0x2;
                        call_ptr(c, 0x0014dd58, 0x001fbe08)?;
                        c.g[RA][0] = sx(0x001fbe10);
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x784), c.g[V0][0]);
                        call_ptr(c, 0x0014dc80, 0x001fbe10)?;
                        c.g[A0][0] = c.g[V0][0];
                        c.g[RA][0] = sx(0x001fbe1c);
                        c.g[A1][0] = 0x8;
                        call_ptr(c, 0x0014dd58, 0x001fbe1c)?;
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x788), c.g[V0][0]);
                        c.g[A0][0] = 0x470000;
                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1c78));
                        c.g[FP][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc0));
                        c.g[V0][0] = c.lw(c.g[FP][0] as u32);
                        c.g[RA][0] = sx(0x001fbe3c);
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x78c), c.g[V0][0]);
                        call_ptr(c, 0x00317670, 0x001fbe3c)?;
                        c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                        c.g[RA][0] = sx(0x001fbe48);
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x0039d860, 0x001fbe48)?;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[RA][0] = sx(0x001fbe54);
                        c.g[S2][0] = c.g[V0][0];
                        call_ptr(c, 0x00144bf0, 0x001fbe54)?;
                        c.g[A1][0] = c.g[V0][0];
                        c.g[RA][0] = sx(0x001fbe60);
                        c.g[A0][0] = c.g[SP][0];
                        call_ptr(c, 0x002c2540, 0x001fbe60)?;
                        c.g[V1][0] = 0x530000;
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x5bc8));
                        c.g[V0][0] = 0x2;
                        c.g[V1][0] = c.lb((c.g[V1][0] as u32).wrapping_add(0x48));
                        let t = c.g[V1][0] == c.g[V0][0];
                        c.g[A3][0] = 0u64;
                        if t {
                            break 'b_1fbeb4;
                        } else {
                            'b_1fbea8: {
                                'b_1fbe94: {
                                    c.cycles += 3;
                                    c.g[V0][0] = ((c.g[V1][0] as i64) < 3i64) as u64;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[V0][0] = 0x1;
                                    if t {
                                        break 'b_1fbe94;
                                    } else {
                                        c.cycles += 2;
                                        let t = c.g[V1][0] == c.g[V0][0];
                                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                        if t {
                                            break 'b_1fbea8;
                                        } else {
                                            c.cycles += 2;
                                            let t = true;
                                            c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
                                            if t {
                                                break 'b_1fbef8;
                                            } else {
                                                break 'b_1fbe94;
                                            }
                                        }
                                    }
                                }
                                c.cycles += 3;
                                c.g[V0][0] = 0x3;
                                let t = c.g[V1][0] == c.g[V0][0];
                                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                if t {
                                    break 'b_1fbec4;
                                } else {
                                    c.cycles += 2;
                                    let t = true;
                                    c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
                                    if t {
                                        break 'b_1fbef8;
                                    } else {
                                        break 'b_1fbea8;
                                    }
                                }
                            }
                            c.cycles += 3;
                            c.g[A0][0] = 0x470000;
                            let t = true;
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1c68));
                            if t {
                                break 'b_1fbecc;
                            } else {
                                break 'b_1fbeb4;
                            }
                        }
                    }
                    c.cycles += 4;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.g[A0][0] = 0x470000;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1c58));
                    if t {
                        break 'b_1fbecc;
                    } else {
                        break 'b_1fbec4;
                    }
                }
                c.cycles += 2;
                c.g[A0][0] = 0x470000;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1c40));
                break 'b_1fbecc;
            }
            c.cycles += 11;
            c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
            c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
            c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
            c.g[RA][0] = sx(0x001fbee0);
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
            call_ptr(c, 0x00317670, 0x001fbee0)?;
            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
            c.g[A0][0] = c.g[S0][0];
            let t = c.g[V1][0] as u32;
            c.g[RA][0] = sx(0x001fbef0);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, t, 0x001fbef0)?;
            c.g[A3][0] = c.g[V0][0];
            c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
            break 'b_1fbef8;
        }
        'b_1fc13c: {
            'b_1fc138: {
                'b_1fc120: {
                    'b_1fc0c0: {
                        c.cycles += 27;
                        c.g[A1][0] = 0x4a0000;
                        c.g[A2][0] = c.g[SP][0];
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2140));
                        c.g[RA][0] = sx(0x001fbf0c);
                        c.g[A0][0] = c.g[S0][0];
                        call_ptr(c, 0x002c27c0, 0x001fbf0c)?;
                        c.g[V1][0] = 0x530000;
                        c.g[A0][0] = c.g[S2][0];
                        c.g[S1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x5bc8));
                        c.g[RA][0] = sx(0x001fbf20);
                        c.g[A1][0] = c.g[S0][0];
                        call_ptr(c, 0x003a0e90, 0x001fbf20)?;
                        c.g[A0][0] = 0x470000;
                        c.g[RA][0] = sx(0x001fbf2c);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x360));
                        call_ptr(c, 0x00317670, 0x001fbf2c)?;
                        c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                        c.g[RA][0] = sx(0x001fbf38);
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x0039d860, 0x001fbf38)?;
                        c.g[A0][0] = 0x470000;
                        c.g[S6][0] = c.g[V0][0];
                        c.g[RA][0] = sx(0x001fbf48);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x350));
                        call_ptr(c, 0x00317670, 0x001fbf48)?;
                        c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                        c.g[RA][0] = sx(0x001fbf54);
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x0039d860, 0x001fbf54)?;
                        c.g[S2][0] = c.g[V0][0];
                        c.g[V0][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0x49));
                        let t = c.g[V0][0] != 0u64;
                        c.g[A0][0] = 0x470000;
                        if t {
                            break 'b_1fc0c0;
                        } else {
                            'b_1fbf98: {
                                c.cycles += 4;
                                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x78c));
                                c.g[V0][0] = 0x1;
                                let t = c.g[A0][0] != c.g[V0][0];
                                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x78c));
                                if t {
                                    break 'b_1fbf98;
                                } else {
                                    c.cycles += 9;
                                    c.g[A0][0] = 0x470000;
                                    c.g[RA][0] = sx(0x001fbf80);
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x340));
                                    call_ptr(c, 0x00317670, 0x001fbf80)?;
                                    c.g[A1][0] = c.g[V0][0];
                                    c.g[RA][0] = sx(0x001fbf8c);
                                    c.g[A0][0] = c.g[S6][0];
                                    call_ptr(c, 0x003a1000, 0x001fbf8c)?;
                                    c.g[A0][0] = 0x470000;
                                    let t = true;
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x328));
                                    if t {
                                        break 'b_1fc120;
                                    } else {
                                        break 'b_1fbf98;
                                    }
                                }
                            }
                            'b_1fc0a0: {
                                c.cycles += 3;
                                c.g[V0][0] = 0x2;
                                if c.g[V1][0] != c.g[V0][0] {
                                    c.g[A0][0] = 0x470000;
                                    break 'b_1fc0a0;
                                } else {
                                    'b_1fc074: {
                                        'b_1fc054: {
                                            c.cycles += 22;
                                            c.g[S0][0] = 0x530000;
                                            c.g[A0][0] = c.g[FP][0];
                                            c.g[S0][0] =
                                                sx((c.g[S0][0] as u32).wrapping_add(0x6708));
                                            c.g[A2][0] = 0u64;
                                            c.g[RA][0] = sx(0x001fbfbc);
                                            c.g[A1][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                            call_ptr(c, 0x00238590, 0x001fbfbc)?;
                                            c.g[A1][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                            c.g[A2][0] = 0x1;
                                            c.g[S1][0] = c.g[V0][0];
                                            c.g[RA][0] = sx(0x001fbfd0);
                                            c.g[A0][0] = c.g[FP][0];
                                            call_ptr(c, 0x00238590, 0x001fbfd0)?;
                                            c.g[A0][0] =
                                                c.lw((c.g[SP][0] as u32).wrapping_add(0x780));
                                            c.g[S1][0] =
                                                sx((c.g[S1][0] as u32)
                                                    .wrapping_add(c.g[V0][0] as u32));
                                            c.g[RA][0] = sx(0x001fbfe0);
                                            c.g[A1][0] = 0u64;
                                            call_ptr(c, 0x00146e98, 0x001fbfe0)?;
                                            c.g[A1][0] = c.g[V0][0];
                                            c.g[A0][0] = c.g[FP][0];
                                            c.g[RA][0] = sx(0x001fbff0);
                                            c.g[A2][0] = 0u64;
                                            call_ptr(c, 0x00238590, 0x001fbff0)?;
                                            c.g[S4][0] =
                                                sx((c.g[S1][0] as u32)
                                                    .wrapping_sub(c.g[V0][0] as u32));
                                            let t = (c.g[S4][0] as i64) <= 0;
                                            c.g[V0][0] =
                                                c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                            if t {
                                                break 'b_1fc054;
                                            } else {
                                                c.cycles += 22;
                                                c.g[A0][0] = 0x470000;
                                                c.g[A0][0] =
                                                    sx((c.g[A0][0] as u32).wrapping_sub(0x310));
                                                c.g[S0][0] =
                                                    c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                                                c.g[S1][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                                                c.g[V0][0] =
                                                    c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                                                c.g[RA][0] = sx(0x001fc018);
                                                c.g[S0][0] = sx((c.g[S0][0] as u32)
                                                    .wrapping_add(c.g[V0][0] as u32));
                                                call_ptr(c, 0x00317670, 0x001fc018)?;
                                                c.g[V1][0] =
                                                    c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                                                c.g[A0][0] = c.g[S0][0];
                                                let t = c.g[V1][0] as u32;
                                                c.g[RA][0] = sx(0x001fc028);
                                                c.g[A1][0] = c.g[V0][0];
                                                call_ptr(c, t, 0x001fc028)?;
                                                c.g[S0][0] =
                                                    sx((c.g[SP][0] as u32).wrapping_add(0x140));
                                                c.g[A1][0] = c.g[V0][0];
                                                c.g[A0][0] = c.g[S0][0];
                                                c.g[RA][0] = sx(0x001fc03c);
                                                c.g[A2][0] = c.g[S4][0];
                                                call_ptr(c, 0x002c26d0, 0x001fc03c)?;
                                                c.g[S5][0] = c.g[S0][0];
                                                c.g[A0][0] = c.g[S6][0];
                                                c.g[RA][0] = sx(0x001fc04c);
                                                c.g[A1][0] = c.g[S0][0];
                                                call_ptr(c, 0x003a0e90, 0x001fc04c)?;
                                                let t = true;
                                                c.g[V0][0] =
                                                    c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                                                if t {
                                                    break 'b_1fc074;
                                                } else {
                                                    break 'b_1fc054;
                                                }
                                            }
                                        }
                                        c.cycles += 8;
                                        c.g[A0][0] = 0x470000;
                                        c.g[RA][0] = sx(0x001fc060);
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1b98));
                                        call_ptr(c, 0x00317670, 0x001fc060)?;
                                        c.g[A0][0] = c.g[S6][0];
                                        c.g[RA][0] = sx(0x001fc06c);
                                        c.g[A1][0] = c.g[V0][0];
                                        call_ptr(c, 0x003a1000, 0x001fc06c)?;
                                        c.g[S5][0] = sx((c.g[SP][0] as u32).wrapping_add(0x140));
                                        c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                                        break 'b_1fc074;
                                    }
                                    c.cycles += 11;
                                    c.g[A1][0] = 0u64;
                                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                    let t = c.g[V1][0] as u32;
                                    c.g[RA][0] = sx(0x001fc088);
                                    c.g[A0][0] =
                                        sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    call_ptr(c, t, 0x001fc088)?;
                                    c.g[A1][0] = 0x4a0000;
                                    c.g[A0][0] = c.g[S3][0];
                                    c.g[RA][0] = sx(0x001fc098);
                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2500));
                                    call_ptr(c, 0x0039e8b8, 0x001fc098)?;
                                    let t = true;
                                    c.g[V0][0] = 0x470000;
                                    if t {
                                        break 'b_1fc13c;
                                    } else {
                                        break 'b_1fc0a0;
                                    }
                                }
                            }
                            c.cycles += 8;
                            c.g[RA][0] = sx(0x001fc0a8);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2f8));
                            call_ptr(c, 0x00317670, 0x001fc0a8)?;
                            c.g[A1][0] = c.g[V0][0];
                            c.g[RA][0] = sx(0x001fc0b4);
                            c.g[A0][0] = c.g[S6][0];
                            call_ptr(c, 0x003a1000, 0x001fc0b4)?;
                            c.g[A0][0] = 0x470000;
                            let t = true;
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2e0));
                            if t {
                                break 'b_1fc120;
                            } else {
                                break 'b_1fc0c0;
                            }
                        }
                    }
                    'b_1fc104: {
                        c.cycles += 9;
                        c.g[RA][0] = sx(0x001fc0c8);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2e0));
                        call_ptr(c, 0x00317670, 0x001fc0c8)?;
                        c.g[A1][0] = c.g[V0][0];
                        c.g[RA][0] = sx(0x001fc0d4);
                        c.g[A0][0] = c.g[S6][0];
                        call_ptr(c, 0x003a1000, 0x001fc0d4)?;
                        c.g[V1][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0x49));
                        c.g[V0][0] = 0x2;
                        if c.g[V1][0] != c.g[V0][0] {
                            c.g[V1][0] = c.lb((c.g[S1][0] as u32).wrapping_add(0x48));
                            break 'b_1fc104;
                        } else {
                            c.cycles += 8;
                            c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                            c.g[A1][0] = 0u64;
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                            let t = c.g[V1][0] as u32;
                            c.g[RA][0] = sx(0x001fc0fc);
                            c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x001fc0fc)?;
                            let t = true;
                            c.g[S5][0] = sx((c.g[SP][0] as u32).wrapping_add(0x140));
                            if t {
                                break 'b_1fc138;
                            } else {
                                break 'b_1fc104;
                            }
                        }
                    }
                    'b_1fc11c: {
                        c.cycles += 3;
                        c.g[V0][0] = 0x1;
                        if c.g[V1][0] != c.g[V0][0] {
                            c.g[A0][0] = 0x470000;
                            break 'b_1fc11c;
                        } else {
                            c.cycles += 3;
                            c.g[A0][0] = 0x470000;
                            let t = true;
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2c8));
                            if t {
                                break 'b_1fc120;
                            } else {
                                break 'b_1fc11c;
                            }
                        }
                    }
                    c.cycles += 1;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2a8));
                    break 'b_1fc120;
                }
                c.cycles += 6;
                c.g[RA][0] = sx(0x001fc128);
                call_ptr(c, 0x00317670, 0x001fc128)?;
                c.g[A0][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x001fc134);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x003a1000, 0x001fc134)?;
                c.g[S5][0] = sx((c.g[SP][0] as u32).wrapping_add(0x140));
                break 'b_1fc138;
            }
            c.cycles += 1;
            c.g[V0][0] = 0x470000;
            break 'b_1fc13c;
        }
        'b_1fc25c: {
            'b_1fc1fc: {
                c.cycles += 19;
                c.g[A0][0] = c.g[S3][0];
                c.g[S2][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x290));
                c.g[V0][0] = 0x530000;
                c.g[A1][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x001fc154);
                c.g[S4][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5bc8));
                call_ptr(c, 0x0039e8b8, 0x001fc154)?;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.g[S3][0];
                c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x280));
                c.g[RA][0] = sx(0x001fc168);
                c.g[A1][0] = c.g[S1][0];
                call_ptr(c, 0x0039e8b8, 0x001fc168)?;
                c.g[V0][0] = 0x470000;
                c.g[A0][0] = c.g[S3][0];
                c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x270));
                c.g[RA][0] = sx(0x001fc17c);
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x0039e8b8, 0x001fc17c)?;
                c.g[V0][0] = c.lb((c.g[S4][0] as u32).wrapping_add(0x49));
                let t = c.g[V0][0] != 0u64;
                c.g[A0][0] = 0x4a0000;
                if t {
                    break 'b_1fc1fc;
                } else {
                    'b_1fc1ec: {
                        'b_1fc1e8: {
                            'b_1fc1e4: {
                                'b_1fc1d8: {
                                    'b_1fc1c8: {
                                        'b_1fc1c4: {
                                            c.cycles += 4;
                                            c.g[V1][0] =
                                                c.lw((c.g[SP][0] as u32).wrapping_add(0x78c));
                                            c.g[V0][0] = 0x2;
                                            let t = c.g[V1][0] == c.g[V0][0];
                                            c.g[V0][0] = ((c.g[V1][0] as i64) < 3i64) as u64;
                                            if t {
                                                break 'b_1fc1c4;
                                            } else {
                                                'b_1fc1b4: {
                                                    c.cycles += 2;
                                                    let t = c.g[V0][0] != 0u64;
                                                    c.g[A0][0] =
                                                        c.lw(
                                                            (c.g[SP][0] as u32).wrapping_add(0x78c)
                                                        );
                                                    if t {
                                                        break 'b_1fc1b4;
                                                    } else {
                                                        c.cycles += 3;
                                                        c.g[V0][0] = 0x3;
                                                        let t = c.g[V1][0] == c.g[V0][0];
                                                        c.g[A1][0] = 0x4a0000;
                                                        if t {
                                                            break 'b_1fc1d8;
                                                        } else {
                                                            c.cycles += 2;
                                                            let t = true;
                                                            c.g[A0][0] = c.g[S3][0];
                                                            if t {
                                                                break 'b_1fc1ec;
                                                            } else {
                                                                break 'b_1fc1b4;
                                                            }
                                                        }
                                                    }
                                                }
                                                c.cycles += 2;
                                                let t = (c.g[A0][0] as i64) < 0;
                                                c.g[A1][0] = c.g[S2][0];
                                                if t {
                                                    break 'b_1fc1e4;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = true;

                                                    if t {
                                                        break 'b_1fc1c8;
                                                    } else {
                                                        break 'b_1fc1c4;
                                                    }
                                                }
                                            }
                                        }
                                        c.cycles += 1;
                                        c.g[A1][0] = c.g[S1][0];
                                        break 'b_1fc1c8;
                                    }
                                    c.cycles += 4;
                                    c.g[RA][0] = sx(0x001fc1d0);
                                    c.g[A0][0] = c.g[S3][0];
                                    call_ptr(c, 0x0039e948, 0x001fc1d0)?;
                                    let t = true;
                                    c.g[A1][0] = 0x4a0000;
                                    if t {
                                        break 'b_1fc1e8;
                                    } else {
                                        break 'b_1fc1d8;
                                    }
                                }
                                c.cycles += 3;
                                c.g[A1][0] = c.g[S0][0];
                                c.g[RA][0] = sx(0x001fc1e4);
                                c.g[A0][0] = c.g[S3][0];
                                call_ptr(c, 0x0039e948, 0x001fc1e4)?;
                                break 'b_1fc1e4;
                            }
                            c.cycles += 1;
                            c.g[A1][0] = 0x4a0000;
                            break 'b_1fc1e8;
                        }
                        c.cycles += 1;
                        c.g[A0][0] = c.g[S3][0];
                        break 'b_1fc1ec;
                    }
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x001fc1f4);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2148));
                    call_ptr(c, 0x0039e8b8, 0x001fc1f4)?;
                    let t = true;
                    c.g[A0][0] = 0x4a0000;
                    if t {
                        break 'b_1fc25c;
                    } else {
                        break 'b_1fc1fc;
                    }
                }
            }
            'b_1fc244: {
                c.cycles += 10;
                c.g[RA][0] = sx(0x001fc204);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2148));
                call_ptr(c, 0x00317670, 0x001fc204)?;
                c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                c.g[RA][0] = sx(0x001fc210);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039d860, 0x001fc210)?;
                c.g[S2][0] = c.g[V0][0];
                c.g[V1][0] = c.lb((c.g[S4][0] as u32).wrapping_add(0x49));
                c.g[V0][0] = 0x2;
                if c.g[V1][0] != c.g[V0][0] {
                    c.g[A0][0] = 0x470000;
                    break 'b_1fc244;
                } else {
                    c.cycles += 8;
                    c.g[A0][0] = 0x470000;
                    c.g[RA][0] = sx(0x001fc230);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x3f8));
                    call_ptr(c, 0x00317670, 0x001fc230)?;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x001fc23c);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x003a1000, 0x001fc23c)?;
                    let t = true;
                    c.g[A0][0] = 0x4a0000;
                    if t {
                        break 'b_1fc25c;
                    } else {
                        break 'b_1fc244;
                    }
                }
            }
            c.cycles += 6;
            c.g[RA][0] = sx(0x001fc24c);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x3e0));
            call_ptr(c, 0x00317670, 0x001fc24c)?;
            c.g[A0][0] = c.g[S2][0];
            c.g[RA][0] = sx(0x001fc258);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x003a1000, 0x001fc258)?;
            c.g[A0][0] = 0x4a0000;
            break 'b_1fc25c;
        }
        'b_1fc344: {
            'b_1fc30c: {
                c.cycles += 11;
                c.g[RA][0] = sx(0x001fc264);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2158));
                call_ptr(c, 0x00317670, 0x001fc264)?;
                c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                c.g[RA][0] = sx(0x001fc270);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039d860, 0x001fc270)?;
                c.g[A0][0] = 0x530000;
                c.g[S2][0] = c.g[V0][0];
                c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5bc8));
                c.g[V0][0] = c.lb((c.g[V1][0] as u32).wrapping_add(0x49));
                if c.g[V0][0] != 0u64 {
                    c.g[A0][0] = 0x470000;
                    break 'b_1fc30c;
                } else {
                    'b_1fc2d0: {
                        'b_1fc2b8: {
                            c.cycles += 4;
                            c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x78c));
                            c.g[V0][0] = ((c.g[V1][0] as i64) < 3i64) as u64;
                            if c.g[V0][0] != 0u64 {
                                c.g[A0][0] = 0x470000;
                                break 'b_1fc2b8;
                            } else {
                                c.cycles += 8;
                                c.g[A0][0] = 0x470000;
                                c.g[RA][0] = sx(0x001fc2a4);
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1c00));
                                call_ptr(c, 0x00317670, 0x001fc2a4)?;
                                c.g[A0][0] = c.g[S2][0];
                                c.g[RA][0] = sx(0x001fc2b0);
                                c.g[A1][0] = c.g[V0][0];
                                call_ptr(c, 0x003a1000, 0x001fc2b0)?;
                                let t = true;
                                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x78c));
                                if t {
                                    break 'b_1fc2d0;
                                } else {
                                    break 'b_1fc2b8;
                                }
                            }
                        }
                        c.cycles += 6;
                        c.g[RA][0] = sx(0x001fc2c0);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x260));
                        call_ptr(c, 0x00317670, 0x001fc2c0)?;
                        c.g[A0][0] = c.g[S2][0];
                        c.g[RA][0] = sx(0x001fc2cc);
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x003a1000, 0x001fc2cc)?;
                        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x78c));
                        break 'b_1fc2d0;
                    }
                    'b_1fc2e4: {
                        c.cycles += 3;
                        c.g[V0][0] = 0x1;
                        let t = c.g[A0][0] == c.g[V0][0];
                        c.g[V0][0] = 0x3;
                        if t {
                            break 'b_1fc2e4;
                        } else {
                            c.cycles += 2;
                            let t = c.g[A0][0] != c.g[V0][0];
                            c.g[V0][0] = 0x530000;
                            if t {
                                break 'b_1fc344;
                            } else {
                                break 'b_1fc2e4;
                            }
                        }
                    }
                    c.cycles += 10;
                    c.g[A1][0] = 0x470000;
                    c.g[A0][0] = c.g[S3][0];
                    c.g[RA][0] = sx(0x001fc2f4);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x250));
                    call_ptr(c, 0x0039e8b8, 0x001fc2f4)?;
                    c.g[A1][0] = 0x470000;
                    c.g[A0][0] = c.g[S3][0];
                    c.g[RA][0] = sx(0x001fc304);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x238));
                    call_ptr(c, 0x0039e8b8, 0x001fc304)?;
                    let t = true;
                    c.g[V0][0] = 0x530000;
                    if t {
                        break 'b_1fc344;
                    } else {
                        break 'b_1fc30c;
                    }
                }
            }
            c.cycles += 14;
            c.g[RA][0] = sx(0x001fc314);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1c00));
            call_ptr(c, 0x00317670, 0x001fc314)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x001fc320);
            c.g[A0][0] = c.g[S2][0];
            call_ptr(c, 0x003a1000, 0x001fc320)?;
            c.g[A1][0] = 0x470000;
            c.g[A0][0] = c.g[S3][0];
            c.g[RA][0] = sx(0x001fc330);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x250));
            call_ptr(c, 0x0039e8b8, 0x001fc330)?;
            c.g[A1][0] = 0x470000;
            c.g[A0][0] = c.g[S3][0];
            c.g[RA][0] = sx(0x001fc340);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x238));
            call_ptr(c, 0x0039e8b8, 0x001fc340)?;
            c.g[V0][0] = 0x530000;
            break 'b_1fc344;
        }
        c.cycles += 9;
        c.g[V1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x190));
        c.g[S7][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6708));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x790), c.g[S5][0]);
        c.g[V0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x170));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x798), c.g[V1][0]);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x794), c.g[V0][0]);
        c.g[S6][0] = 0x1;
        c.g[S4][0] = sx((c.g[SP][0] as u32).wrapping_add(0x160));
        'l_1fc368: loop {
            'b_1fc388: {
                c.cycles += 6;
                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x780));
                c.g[A1][0] = 0u64;
                c.g[RA][0] = sx(0x001fc378);
                c.g[S2][0] = c.lw(c.g[S7][0] as u32);
                call_ptr(c, 0x00146e98, 0x001fc378)?;
                let t = c.g[S2][0] != c.g[V0][0];
                c.g[A0][0] = 0x470000;
                if t {
                    break 'b_1fc388;
                } else {
                    c.cycles += 2;
                    c.g[S7][0] = sx((c.g[S7][0] as u32).wrapping_add(0x4));
                    c.g[S2][0] = c.lw(c.g[S7][0] as u32);
                    break 'b_1fc388;
                }
            }
            'b_1fc3dc: {
                c.cycles += 12;
                c.g[A2][0] = c.g[S6][0];
                c.g[A1][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1be8));
                c.g[RA][0] = sx(0x001fc398);
                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x790));
                call_ptr(c, 0x00417828, 0x001fc398)?;
                c.g[RA][0] = sx(0x001fc3a0);
                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x790));
                call_ptr(c, 0x00317670, 0x001fc3a0)?;
                c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                c.g[RA][0] = sx(0x001fc3ac);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039d860, 0x001fc3ac)?;
                c.g[S1][0] = c.g[V0][0];
                let t = c.g[S1][0] == 0u64;
                c.g[V0][0] = sx((c.g[S2][0] as u32) << 2);
                if t {
                    break 'b_1fc3dc;
                } else {
                    c.cycles += 9;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x784));
                    c.g[V1][0] = sx((c.g[FP][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[V0][0] = c.g[V1][0];
                    c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x40));
                    c.g[RA][0] = sx(0x001fc3d0);
                    c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x18));
                    call_ptr(c, 0x0014eec8, 0x001fc3d0)?;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x001fc3dc);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x003a0c50, 0x001fc3dc)?;
                    break 'b_1fc3dc;
                }
            }
            'b_1fc448: {
                c.cycles += 13;
                c.g[V0][0] = 0x470000;
                c.g[A2][0] = c.g[S6][0];
                c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1bd8));
                c.g[RA][0] = sx(0x001fc3f0);
                c.g[A0][0] = c.g[S5][0];
                call_ptr(c, 0x00417828, 0x001fc3f0)?;
                c.g[RA][0] = sx(0x001fc3f8);
                c.g[A0][0] = c.g[S5][0];
                call_ptr(c, 0x00317670, 0x001fc3f8)?;
                c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                c.g[RA][0] = sx(0x001fc404);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039d860, 0x001fc404)?;
                c.g[S1][0] = c.g[V0][0];
                let t = c.g[S1][0] == 0u64;
                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x78c));
                if t {
                    break 'b_1fc448;
                } else {
                    c.cycles += 14;
                    c.g[A2][0] = 0x2;
                    c.g[A0][0] = c.g[FP][0];
                    c.g[A1][0] = c.g[S2][0];
                    c.g[V0][0] = c.g[V1][0] ^ 0x3;
                    c.g[RA][0] = sx(0x001fc428);
                    if c.g[V0][0] != 0 {
                        c.g[A2][0] = 0u64;
                    }
                    call_ptr(c, 0x00238590, 0x001fc428)?;
                    c.g[A2][0] = c.g[V0][0];
                    c.g[A0][0] = c.g[S4][0];
                    c.g[V0][0] = 0x4a0000;
                    c.g[RA][0] = sx(0x001fc43c);
                    c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2138));
                    call_ptr(c, 0x00417828, 0x001fc43c)?;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x001fc448);
                    c.g[A1][0] = c.g[S4][0];
                    call_ptr(c, 0x003a0c50, 0x001fc448)?;
                    break 'b_1fc448;
                }
            }
            'b_1fc548: {
                'b_1fc538: {
                    c.cycles += 16;
                    c.g[V1][0] = 0x470000;
                    c.g[A2][0] = c.g[S6][0];
                    c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1bc8));
                    c.g[RA][0] = sx(0x001fc45c);
                    c.g[A0][0] = c.g[S5][0];
                    call_ptr(c, 0x00417828, 0x001fc45c)?;
                    c.g[RA][0] = sx(0x001fc464);
                    c.g[A0][0] = c.g[S5][0];
                    call_ptr(c, 0x00317670, 0x001fc464)?;
                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                    c.g[RA][0] = sx(0x001fc470);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x001fc470)?;
                    c.g[A0][0] = 0x530000;
                    c.g[S1][0] = c.g[V0][0];
                    c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5bc8));
                    c.g[V0][0] = c.lb((c.g[V1][0] as u32).wrapping_add(0x49));
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x4a0000;
                    if t {
                        break 'b_1fc538;
                    } else {
                        c.cycles += 4;
                        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x78c));
                        c.g[V1][0] = 0x2;
                        let t = c.g[V0][0] != c.g[V1][0];
                        c.g[V0][0] = 0x4a0000;
                        if t {
                            break 'b_1fc538;
                        } else {
                            c.cycles += 40;
                            c.g[A0][0] = c.g[FP][0];
                            c.g[A1][0] = c.g[S2][0];
                            c.g[RA][0] = sx(0x001fc4a8);
                            c.g[A2][0] = 0x1;
                            call_ptr(c, 0x00238590, 0x001fc4a8)?;
                            c.g[A2][0] = c.g[V0][0];
                            c.g[A1][0] = sx((c.g[GP][0] as u32).wrapping_sub(0xfb8));
                            c.g[RA][0] = sx(0x001fc4b8);
                            c.g[A0][0] = c.g[S4][0];
                            call_ptr(c, 0x00417828, 0x001fc4b8)?;
                            c.g[A0][0] = c.g[S1][0];
                            c.g[RA][0] = sx(0x001fc4c4);
                            c.g[A1][0] = c.g[S4][0];
                            call_ptr(c, 0x003a0c50, 0x001fc4c4)?;
                            c.g[A0][0] = 0x470000;
                            c.g[A2][0] = c.g[S6][0];
                            c.g[A1][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1bb8));
                            c.g[RA][0] = sx(0x001fc4d8);
                            c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x794));
                            call_ptr(c, 0x00417828, 0x001fc4d8)?;
                            c.g[RA][0] = sx(0x001fc4e0);
                            c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x794));
                            call_ptr(c, 0x00317670, 0x001fc4e0)?;
                            c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                            c.g[RA][0] = sx(0x001fc4ec);
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x0039d860, 0x001fc4ec)?;
                            c.g[S1][0] = c.g[V0][0];
                            c.g[A0][0] = c.g[FP][0];
                            c.g[A1][0] = c.g[S2][0];
                            c.g[RA][0] = sx(0x001fc500);
                            c.g[A2][0] = 0u64;
                            call_ptr(c, 0x00238590, 0x001fc500)?;
                            c.g[S0][0] = c.g[V0][0];
                            c.g[A1][0] = c.g[S2][0];
                            c.g[A0][0] = c.g[FP][0];
                            c.g[RA][0] = sx(0x001fc514);
                            c.g[A2][0] = 0x1;
                            call_ptr(c, 0x00238590, 0x001fc514)?;
                            c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x798));
                            c.g[A1][0] = sx((c.g[GP][0] as u32).wrapping_sub(0xfb8));
                            c.g[RA][0] = sx(0x001fc524);
                            c.g[A2][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                            call_ptr(c, 0x00417828, 0x001fc524)?;
                            c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x798));
                            c.g[RA][0] = sx(0x001fc530);
                            c.g[A0][0] = c.g[S1][0];
                            call_ptr(c, 0x003a0c50, 0x001fc530)?;
                            let t = true;
                            c.g[S6][0] = sx((c.g[S6][0] as u32).wrapping_add(0x1));
                            if t {
                                break 'b_1fc548;
                            } else {
                                break 'b_1fc538;
                            }
                        }
                    }
                }
                c.cycles += 4;
                c.g[A0][0] = c.g[S1][0];
                c.g[RA][0] = sx(0x001fc544);
                c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2168));
                call_ptr(c, 0x003a0c50, 0x001fc544)?;
                c.g[S6][0] = sx((c.g[S6][0] as u32).wrapping_add(0x1));
                break 'b_1fc548;
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S6][0] as i64) < 4i64) as u64;
            let t = c.g[V0][0] != 0u64;
            c.g[S7][0] = sx((c.g[S7][0] as u32).wrapping_add(0x4));
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x001fc368);
                }
                continue 'l_1fc368;
            } else {
                'b_1fc64c: {
                    'b_1fc640: {
                        'b_1fc63c: {
                            c.cycles += 37;
                            c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x780));
                            c.g[RA][0] = sx(0x001fc560);
                            c.g[A1][0] = 0u64;
                            call_ptr(c, 0x00146e98, 0x001fc560)?;
                            c.g[A0][0] = 0x4a0000;
                            c.g[S1][0] = c.g[V0][0];
                            c.g[RA][0] = sx(0x001fc570);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2508));
                            call_ptr(c, 0x00317670, 0x001fc570)?;
                            c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                            c.g[RA][0] = sx(0x001fc57c);
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x0039d860, 0x001fc57c)?;
                            c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x780));
                            c.g[S2][0] = c.g[V0][0];
                            c.g[RA][0] = sx(0x001fc58c);
                            c.g[A1][0] = c.g[S1][0];
                            call_ptr(c, 0x00147398, 0x001fc58c)?;
                            c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x780));
                            c.g[S0][0] = c.g[V0][0];
                            c.g[RA][0] = sx(0x001fc59c);
                            c.g[A1][0] = c.g[S1][0];
                            call_ptr(c, 0x00147410, 0x001fc59c)?;
                            c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x784));
                            c.g[A2][0] = c.g[V0][0];
                            c.g[RA][0] = sx(0x001fc5ac);
                            c.g[A1][0] = c.g[S0][0];
                            call_ptr(c, 0x0014eec8, 0x001fc5ac)?;
                            c.g[A0][0] = c.g[S2][0];
                            c.g[RA][0] = sx(0x001fc5b8);
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x003a0c50, 0x001fc5b8)?;
                            c.g[A0][0] = 0x4a0000;
                            c.g[RA][0] = sx(0x001fc5c4);
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2510));
                            call_ptr(c, 0x00317670, 0x001fc5c4)?;
                            c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                            c.g[RA][0] = sx(0x001fc5d0);
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x0039d860, 0x001fc5d0)?;
                            c.g[A0][0] = 0x530000;
                            c.g[S2][0] = c.g[V0][0];
                            c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5bc8));
                            c.g[V0][0] = c.lb((c.g[V1][0] as u32).wrapping_add(0x49));
                            let t = c.g[V0][0] != 0u64;
                            c.g[A1][0] = 0x4a0000;
                            if t {
                                break 'b_1fc63c;
                            } else {
                                c.cycles += 4;
                                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x78c));
                                c.g[V0][0] = 0x2;
                                let t = c.g[V1][0] != c.g[V0][0];
                                c.g[A0][0] = c.g[S2][0];
                                if t {
                                    break 'b_1fc640;
                                } else {
                                    c.cycles += 17;
                                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x780));
                                    c.g[RA][0] = sx(0x001fc604);
                                    c.g[A1][0] = 0u64;
                                    call_ptr(c, 0x00146e98, 0x001fc604)?;
                                    c.g[A0][0] = c.g[FP][0];
                                    c.g[A1][0] = c.g[V0][0];
                                    c.g[RA][0] = sx(0x001fc614);
                                    c.g[A2][0] = 0u64;
                                    call_ptr(c, 0x00238590, 0x001fc614)?;
                                    c.g[A2][0] = c.g[V0][0];
                                    c.g[A0][0] = c.g[S5][0];
                                    c.g[V0][0] = 0x4a0000;
                                    c.g[RA][0] = sx(0x001fc628);
                                    c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2138));
                                    call_ptr(c, 0x00417828, 0x001fc628)?;
                                    c.g[A0][0] = c.g[S2][0];
                                    c.g[RA][0] = sx(0x001fc634);
                                    c.g[A1][0] = c.g[S5][0];
                                    call_ptr(c, 0x003a0c50, 0x001fc634)?;
                                    let t = true;
                                    c.g[A0][0] = 0x4a0000;
                                    if t {
                                        break 'b_1fc64c;
                                    } else {
                                        break 'b_1fc63c;
                                    }
                                }
                            }
                        }
                        c.cycles += 1;
                        c.g[A0][0] = c.g[S2][0];
                        break 'b_1fc640;
                    }
                    c.cycles += 3;
                    c.g[RA][0] = sx(0x001fc648);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2168));
                    call_ptr(c, 0x003a0c50, 0x001fc648)?;
                    c.g[A0][0] = 0x4a0000;
                    break 'b_1fc64c;
                }
                'b_1fc6f4: {
                    'b_1fc6cc: {
                        c.cycles += 12;
                        c.g[RA][0] = sx(0x001fc654);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2518));
                        call_ptr(c, 0x00317670, 0x001fc654)?;
                        c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                        c.g[RA][0] = sx(0x001fc660);
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x0039d860, 0x001fc660)?;
                        c.g[A0][0] = 0x530000;
                        c.g[S2][0] = c.g[V0][0];
                        c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5bc8));
                        c.g[V0][0] = 0x2;
                        c.g[A0][0] = c.lb((c.g[V1][0] as u32).wrapping_add(0x49));
                        if c.g[A0][0] != c.g[V0][0] {
                            c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
                            break 'b_1fc6cc;
                        } else {
                            c.cycles += 20;
                            c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x780));
                            c.g[RA][0] = sx(0x001fc688);
                            c.g[A1][0] = 0x1;
                            call_ptr(c, 0x00146e98, 0x001fc688)?;
                            c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x780));
                            c.g[S1][0] = c.g[V0][0];
                            c.g[RA][0] = sx(0x001fc698);
                            c.g[A1][0] = c.g[S1][0];
                            call_ptr(c, 0x00147398, 0x001fc698)?;
                            c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x780));
                            c.g[S0][0] = c.g[V0][0];
                            c.g[RA][0] = sx(0x001fc6a8);
                            c.g[A1][0] = c.g[S1][0];
                            call_ptr(c, 0x00147410, 0x001fc6a8)?;
                            c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x784));
                            c.g[A1][0] = c.g[S0][0];
                            c.g[RA][0] = sx(0x001fc6b8);
                            c.g[A2][0] = c.g[V0][0];
                            call_ptr(c, 0x0014eec8, 0x001fc6b8)?;
                            c.g[A0][0] = c.g[S2][0];
                            c.g[RA][0] = sx(0x001fc6c4);
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x003a0c50, 0x001fc6c4)?;
                            let t = true;
                            c.g[A0][0] = 0x470000;
                            if t {
                                break 'b_1fc6f4;
                            } else {
                                break 'b_1fc6cc;
                            }
                        }
                    }
                    c.cycles += 10;
                    c.g[A1][0] = 0u64;
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x001fc6e0);
                    c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x001fc6e0)?;
                    c.g[A1][0] = 0x470000;
                    c.g[A0][0] = c.g[S3][0];
                    c.g[RA][0] = sx(0x001fc6f0);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x228));
                    call_ptr(c, 0x0039e8b8, 0x001fc6f0)?;
                    c.g[A0][0] = 0x470000;
                    break 'b_1fc6f4;
                }
                c.cycles += 29;
                c.g[RA][0] = sx(0x001fc6fc);
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x218));
                call_ptr(c, 0x00317670, 0x001fc6fc)?;
                c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                c.g[RA][0] = sx(0x001fc708);
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039d860, 0x001fc708)?;
                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x788));
                c.g[S2][0] = c.g[V0][0];
                c.g[RA][0] = sx(0x001fc718);
                c.g[A1][0] = 0u64;
                call_ptr(c, 0x00155130, 0x001fc718)?;
                c.g[V1][0] = 0x4a0000;
                c.g[A0][0] = c.g[S5][0];
                c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x2138));
                c.g[RA][0] = sx(0x001fc72c);
                c.g[A2][0] = c.g[V0][0];
                call_ptr(c, 0x00417828, 0x001fc72c)?;
                c.g[A0][0] = c.g[S2][0];
                c.g[RA][0] = sx(0x001fc738);
                c.g[A1][0] = c.g[S5][0];
                call_ptr(c, 0x003a0c50, 0x001fc738)?;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x830));
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x820));
                c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x810));
                c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x800));
                c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x7f0));
                c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x7e0));
                c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x7d0));
                c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x7c0));
                c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x7b0));
                c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x7a0));
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x840));
                c.pc = t;
                return Ok(());
            }
        }
    });
}

/// `void OVScreenMultiFreestylePre_Init(OVStateMultiFreestylePre* self)`
/// (0x1fc878, ovstatepause.cpp): builds the multiplayer freestyle pre-race
/// screen. Adds the "50multifs_pre" and "OV_darkblue" UI screens (play frame 0
/// each), then fills the labels from the new-race settings (`0x535bc8`: byte
/// +0x48 game mode, +0x49 game path, +0x4a event kind): the location and mode
/// caption, the two rider names (multiplayer path: the player-slot names), the
/// objective texts per mode and the record time or score line, showing or hiding
/// the mode-specific objects. Every call goes through the function table in the
/// original order, so the body follows the generated block structure.
pub fn ov_screen_multi_freestyle_pre_init(c: &mut Cpu) {
    guest(c, |c| {
        'b_1fc8e4: {
            c.cycles += 23;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x490));
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x430));
            c.g[S5][0] = c.g[A0][0];
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x470));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x460));
            c.g[A0][0] = 0x470000;
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x450));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1f8));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x440));
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x420));
            c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x410));
            c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x400));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x3f0), c.g[RA][0]);
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x480));
            c.g[S0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x10));
            call_ptr(c, 0x0031_7670, 0x001f_c8b8)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S5][0];
            c.g[A3][0] = 0u64;
            call_ptr(c, 0x0039_7cc0, 0x001f_c8cc)?;
            let t = c.g[V0][0] == 0u64;
            c.sw((c.g[S5][0] as u32).wrapping_add(0x40), c.g[V0][0]);
            if t {
                break 'b_1fc8e4;
            } else {
                c.cycles += 4;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0u64;
                c.g[A2][0] = 0u64;
                call_ptr(c, 0x0039_c870, 0x001f_c8e4)?;
                break 'b_1fc8e4;
            }
        }
        'b_1fc924: {
            c.cycles += 11;
            c.g[A0][0] = 0x470000;
            c.g[S0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x10));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1c88));
            call_ptr(c, 0x0031_7670, 0x001f_c8f4)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S5][0];
            c.g[A3][0] = 0u64;
            call_ptr(c, 0x0039_7cc0, 0x001f_c908)?;
            if c.g[V0][0] == 0u64 {
                c.g[A0][0] = c.g[S5][0];
                break 'b_1fc924;
            } else {
                c.cycles += 5;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0u64;
                c.g[A2][0] = 0u64;
                call_ptr(c, 0x0039_c870, 0x001f_c920)?;
                c.g[A0][0] = c.g[S5][0];
                break 'b_1fc924;
            }
        }
        'b_1fca98: {
            'b_1fca94: {
                'b_1fca68: {
                    'b_1fca5c: {
                        'b_1fca4c: {
                            c.cycles += 54;
                            c.g[S0][0] = 0x530000;
                            call_ptr(c, 0x0020_a380, 0x001f_c92c)?;
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x5bc8));
                            call_ptr(c, 0x0014_dc80, 0x001f_c934)?;
                            c.g[S1][0] = 0x4a0000;
                            c.g[A0][0] = c.g[V0][0];
                            c.g[A1][0] = 0x1;
                            call_ptr(c, 0x0014_dd58, 0x001f_c944)?;
                            c.g[S2][0] = 0x470000;
                            c.g[S7][0] = c.g[V0][0];
                            call_ptr(c, 0x0014_dc80, 0x001f_c950)?;
                            c.g[A0][0] = c.g[V0][0];
                            c.g[A1][0] = 0u64;
                            call_ptr(c, 0x0014_dd58, 0x001f_c95c)?;
                            c.g[S3][0] = c.g[V0][0];
                            call_ptr(c, 0x0014_dc80, 0x001f_c964)?;
                            c.g[A0][0] = c.g[V0][0];
                            c.g[A1][0] = 0x2;
                            call_ptr(c, 0x0014_dd58, 0x001f_c970)?;
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x3e0), c.g[V0][0]);
                            call_ptr(c, 0x0014_dc80, 0x001f_c978)?;
                            c.g[A1][0] = 0x8;
                            c.g[A0][0] = c.g[V0][0];
                            call_ptr(c, 0x0014_dd58, 0x001f_c984)?;
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x3e4), c.g[V0][0]);
                            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x21b0));
                            c.g[FP][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x4a));
                            c.g[S6][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x48));
                            call_ptr(c, 0x0031_7670, 0x001f_c998)?;
                            c.g[A0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x40));
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x0039_d860, 0x001f_c9a4)?;
                            c.g[S4][0] = c.g[V0][0];
                            c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x3f8));
                            call_ptr(c, 0x0031_7670, 0x001f_c9b0)?;
                            c.g[A0][0] = c.g[S4][0];
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x003a_1000, 0x001f_c9bc)?;
                            c.g[A0][0] = 0x4a0000;
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x21b8));
                            call_ptr(c, 0x0031_7670, 0x001f_c9c8)?;
                            c.g[A0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x40));
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x0039_d860, 0x001f_c9d4)?;
                            c.g[A0][0] = c.g[S3][0];
                            c.g[S4][0] = c.g[V0][0];
                            call_ptr(c, 0x0014_4bf0, 0x001f_c9e0)?;
                            c.g[A1][0] = c.g[V0][0];
                            c.g[A0][0] = c.g[SP][0];
                            call_ptr(c, 0x002c_2540, 0x001f_c9ec)?;
                            c.g[V1][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x48));
                            c.g[V0][0] = 0x2;
                            let t = c.g[V1][0] == c.g[V0][0];
                            c.g[A3][0] = 0u64;
                            if t {
                                break 'b_1fca4c;
                            } else {
                                'b_1fca3c: {
                                    'b_1fca18: {
                                        c.cycles += 3;
                                        c.g[V0][0] = ((c.g[V1][0] as i64) < 3i64) as u64;
                                        let t = c.g[V0][0] == 0u64;
                                        c.g[V0][0] = 0x1;
                                        if t {
                                            break 'b_1fca18;
                                        } else {
                                            c.cycles += 2;
                                            let t = c.g[V1][0] == c.g[V0][0];
                                            c.g[V0][0] = 0x4a0000;
                                            if t {
                                                break 'b_1fca3c;
                                            } else {
                                                c.cycles += 2;
                                                let t = true;
                                                c.g[V1][0] = 0x6;
                                                if t {
                                                    break 'b_1fca98;
                                                } else {
                                                    break 'b_1fca18;
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 3;
                                    c.g[V0][0] = 0x3;
                                    let t = c.g[V1][0] == c.g[V0][0];
                                    c.g[V0][0] = 0x5;
                                    if t {
                                        break 'b_1fca5c;
                                    } else {
                                        c.cycles += 2;
                                        let t = c.g[V1][0] != c.g[V0][0];
                                        c.g[V0][0] = 0x4a0000;
                                        if t {
                                            break 'b_1fca94;
                                        } else {
                                            c.cycles += 4;
                                            c.g[V0][0] =
                                                c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                            c.g[A0][0] = 0x470000;
                                            let t = true;
                                            c.g[A0][0] =
                                                sx((c.g[A0][0] as u32).wrapping_sub(0x1ab8));
                                            if t {
                                                break 'b_1fca68;
                                            } else {
                                                break 'b_1fca3c;
                                            }
                                        }
                                    }
                                }
                                c.cycles += 4;
                                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                c.g[A0][0] = 0x470000;
                                let t = true;
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1c68));
                                if t {
                                    break 'b_1fca68;
                                } else {
                                    break 'b_1fca4c;
                                }
                            }
                        }
                        c.cycles += 4;
                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                        c.g[A0][0] = 0x470000;
                        let t = true;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1c58));
                        if t {
                            break 'b_1fca68;
                        } else {
                            break 'b_1fca5c;
                        }
                    }
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.g[A0][0] = 0x470000;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1c40));
                    break 'b_1fca68;
                }
                c.cycles += 11;
                c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                call_ptr(c, 0x0031_7670, 0x001f_ca7c)?;
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                c.g[A0][0] = c.g[S0][0];
                let t = c.g[V1][0] as u32;
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, t, 0x001f_ca8c)?;
                c.g[A3][0] = c.g[V0][0];
                c.g[V0][0] = 0x4a0000;
                break 'b_1fca94;
            }
            c.cycles += 1;
            c.g[V1][0] = 0x6;
            break 'b_1fca98;
        }
        'b_1fcad4: {
            'b_1fcac0: {
                c.cycles += 2;
                let t = c.g[S6][0] != c.g[V1][0];
                c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2140));
                if t {
                    break 'b_1fcac0;
                } else {
                    c.cycles += 8;
                    c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
                    c.g[A1][0] = 0x470000;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1aa8));
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A2][0] = c.g[SP][0];
                    call_ptr(c, 0x002c_27c0, 0x001f_cab8)?;
                    let t = true;
                    c.g[A1][0] = c.g[S0][0];
                    if t {
                        break 'b_1fcad4;
                    } else {
                        break 'b_1fcac0;
                    }
                }
            }
            c.cycles += 5;
            c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
            c.g[A2][0] = c.g[SP][0];
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x002c_27c0, 0x001f_cad0)?;
            c.g[A1][0] = c.g[S0][0];
            break 'b_1fcad4;
        }
        'b_1fcb50: {
            'b_1fcb2c: {
                c.cycles += 15;
                c.g[A0][0] = c.g[S4][0];
                call_ptr(c, 0x003a_0e90, 0x001f_cadc)?;
                c.g[A0][0] = 0x4a0000;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2520));
                call_ptr(c, 0x0031_7670, 0x001f_cae8)?;
                c.g[A0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x40));
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039_d860, 0x001f_caf4)?;
                c.g[A0][0] = 0x530000;
                c.g[S4][0] = c.g[V0][0];
                c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5bc8));
                c.g[V0][0] = 0x2;
                c.g[A0][0] = c.lb((c.g[V1][0] as u32).wrapping_add(0x49));
                let t = c.g[A0][0] != c.g[V0][0];
                c.g[A0][0] = c.g[S7][0];
                if t {
                    break 'b_1fcb2c;
                } else {
                    c.cycles += 7;
                    c.g[A1][0] = 0u64;
                    call_ptr(c, 0x0014_6f88, 0x001f_cb18)?;
                    c.g[A0][0] = c.g[S4][0];
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x003a_0c50, 0x001f_cb24)?;
                    let t = true;
                    c.g[A0][0] = 0x4a0000;
                    if t {
                        break 'b_1fcb50;
                    } else {
                        break 'b_1fcb2c;
                    }
                }
            }
            c.cycles += 9;
            c.g[A1][0] = 0u64;
            call_ptr(c, 0x0014_6e98, 0x001f_cb34)?;
            c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x3e0));
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0014_ee58, 0x001f_cb40)?;
            c.g[A0][0] = c.g[S4][0];
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x003a_0c50, 0x001f_cb4c)?;
            c.g[A0][0] = 0x4a0000;
            break 'b_1fcb50;
        }
        'b_1fcc0c: {
            'b_1fcbe8: {
                c.cycles += 10;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2528));
                call_ptr(c, 0x0031_7670, 0x001f_cb58)?;
                c.g[A0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x40));
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039_d860, 0x001f_cb64)?;
                c.g[S3][0] = c.g[V0][0];
                c.g[V0][0] = sx((c.g[S6][0] as u32).wrapping_sub(0x1));
                c.g[V0][0] = (c.g[V0][0] < 0x3u64) as u64;
                let t = c.g[V0][0] == 0u64;
                c.g[S0][0] = 0x4a0000;
                if t {
                    break 'b_1fcbe8;
                } else {
                    'b_1fcbcc: {
                        'b_1fcbc8: {
                            c.cycles += 17;
                            c.g[A0][0] = c.g[S5][0];
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x2530));
                            c.g[A1][0] = c.g[S0][0];
                            call_ptr(c, 0x0039_e948, 0x001f_cb88)?;
                            c.g[A1][0] = 0x4a0000;
                            c.g[A0][0] = c.g[S5][0];
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2538));
                            call_ptr(c, 0x0039_e948, 0x001f_cb98)?;
                            c.g[A0][0] = c.g[S0][0];
                            call_ptr(c, 0x0031_7670, 0x001f_cba0)?;
                            c.g[A0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x40));
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x0039_d860, 0x001f_cbac)?;
                            c.g[S4][0] = c.g[V0][0];
                            c.g[V0][0] = 0x1;
                            if c.g[S6][0] != c.g[V0][0] {
                                c.g[A0][0] = 0x470000;
                                break 'b_1fcbc8;
                            } else {
                                c.cycles += 3;
                                c.g[A0][0] = 0x470000;
                                let t = true;
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2c8));
                                if t {
                                    break 'b_1fcbcc;
                                } else {
                                    break 'b_1fcbc8;
                                }
                            }
                        }
                        c.cycles += 1;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2a8));
                        break 'b_1fcbcc;
                    }
                    c.cycles += 7;
                    call_ptr(c, 0x0031_7670, 0x001f_cbd4)?;
                    c.g[A0][0] = c.g[S4][0];
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x003a_1000, 0x001f_cbe0)?;
                    let t = true;
                    c.g[V0][0] = 0x4a0000;
                    if t {
                        break 'b_1fcc0c;
                    } else {
                        break 'b_1fcbe8;
                    }
                }
            }
            c.cycles += 9;
            c.g[A1][0] = 0x4a0000;
            c.g[A0][0] = c.g[S5][0];
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2530));
            call_ptr(c, 0x0039_e8b8, 0x001f_cbf8)?;
            c.g[A1][0] = 0x4a0000;
            c.g[A0][0] = c.g[S5][0];
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2538));
            call_ptr(c, 0x0039_e8b8, 0x001f_cc08)?;
            c.g[V0][0] = 0x4a0000;
            break 'b_1fcc0c;
        }
        'b_1fcca4: {
            'b_1fcc78: {
                c.cycles += 18;
                c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x21b0));
                call_ptr(c, 0x0031_7670, 0x001f_cc14)?;
                c.g[A0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x40));
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039_d860, 0x001f_cc20)?;
                c.g[V1][0] = 0x470000;
                c.g[S4][0] = c.g[V0][0];
                c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x3f8));
                call_ptr(c, 0x0031_7670, 0x001f_cc30)?;
                c.g[A0][0] = c.g[S4][0];
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x003a_1000, 0x001f_cc3c)?;
                c.g[A0][0] = 0x530000;
                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5bc8));
                c.g[V1][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x49));
                c.g[A0][0] = 0x2;
                let t = c.g[V1][0] != c.g[A0][0];
                c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x140));
                if t {
                    break 'b_1fcc78;
                } else {
                    c.cycles += 9;
                    c.g[A0][0] = c.g[S7][0];
                    c.g[A1][0] = 0x1;
                    c.g[S2][0] = c.g[S0][0];
                    call_ptr(c, 0x0014_6f88, 0x001f_cc64)?;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x0041_66f4, 0x001f_cc70)?;
                    let t = true;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    if t {
                        break 'b_1fcca4;
                    } else {
                        break 'b_1fcc78;
                    }
                }
            }
            c.cycles += 11;
            c.g[A0][0] = c.g[S7][0];
            c.g[A1][0] = 0x1;
            call_ptr(c, 0x0014_6e98, 0x001f_cc84)?;
            c.g[S2][0] = c.g[S0][0];
            c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x3e0));
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0014_ee58, 0x001f_cc94)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x0041_66f4, 0x001f_cca0)?;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            break 'b_1fcca4;
        }
        'b_1fcd5c: {
            'b_1fcd2c: {
                c.cycles += 29;
                c.g[A0][0] = 0x470000;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1e8));
                c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                call_ptr(c, 0x0031_7670, 0x001f_ccc0)?;
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                c.g[A0][0] = c.g[S0][0];
                let t = c.g[V1][0] as u32;
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, t, 0x001f_ccd0)?;
                c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x160));
                c.g[A2][0] = c.g[S2][0];
                c.g[A1][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x002c_26d0, 0x001f_cce4)?;
                c.g[A0][0] = 0x4a0000;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2540));
                call_ptr(c, 0x0031_7670, 0x001f_ccf0)?;
                c.g[A0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x40));
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039_d860, 0x001f_ccfc)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x003a_0e90, 0x001f_cd08)?;
                c.g[V0][0] = sx((c.g[S6][0] as u32).wrapping_sub(0x2));
                c.g[V0][0] = (c.g[V0][0] < 0x2u64) as u64;
                let t = c.g[V0][0] == 0u64;
                c.g[A1][0] = 0x470000;
                if t {
                    break 'b_1fcd2c;
                } else {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S5][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1d8));
                    call_ptr(c, 0x0039_e8b8, 0x001f_cd24)?;
                    let t = true;
                    c.g[V0][0] = 0x4;
                    if t {
                        break 'b_1fcd5c;
                    } else {
                        break 'b_1fcd2c;
                    }
                }
            }
            'b_1fcd58: {
                c.cycles += 4;
                c.g[V0][0] = sx((c.g[S6][0] as u32).wrapping_sub(0x5));
                c.g[V0][0] = (c.g[V0][0] < 0x2u64) as u64;
                let t = c.g[V0][0] == 0u64;
                c.g[A1][0] = 0x470000;
                if t {
                    break 'b_1fcd58;
                } else {
                    c.cycles += 7;
                    c.g[A0][0] = c.g[S5][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1778));
                    call_ptr(c, 0x0039_e8b8, 0x001f_cd48)?;
                    c.g[A1][0] = 0x470000;
                    c.g[A0][0] = c.g[S5][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1d8));
                    call_ptr(c, 0x0039_e8b8, 0x001f_cd58)?;
                    break 'b_1fcd58;
                }
            }
            c.cycles += 1;
            c.g[V0][0] = 0x4;
            break 'b_1fcd5c;
        }
        'b_1fcd9c: {
            'b_1fcd84: {
                c.cycles += 2;
                if c.g[FP][0] != c.g[V0][0] {
                    c.g[A0][0] = 0x470000;
                    break 'b_1fcd84;
                } else {
                    c.cycles += 8;
                    c.g[A0][0] = 0x470000;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1c8));
                    call_ptr(c, 0x0031_7670, 0x001f_cd70)?;
                    c.g[A0][0] = c.g[S3][0];
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x003a_1000, 0x001f_cd7c)?;
                    let t = true;
                    c.g[A0][0] = 0x470000;
                    if t {
                        break 'b_1fcd9c;
                    } else {
                        break 'b_1fcd84;
                    }
                }
            }
            c.cycles += 6;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1b0));
            call_ptr(c, 0x0031_7670, 0x001f_cd8c)?;
            c.g[A0][0] = c.g[S3][0];
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x003a_1000, 0x001f_cd98)?;
            c.g[A0][0] = 0x470000;
            break 'b_1fcd9c;
        }
        'b_1fce90: {
            'b_1fce28: {
                'b_1fcdcc: {
                    c.cycles += 9;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x190));
                    call_ptr(c, 0x0031_7670, 0x001f_cda4)?;
                    c.g[A0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x40));
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039_d860, 0x001f_cdb0)?;
                    c.g[S4][0] = c.g[V0][0];
                    c.g[V0][0] = 0x4;
                    let t = c.g[FP][0] == c.g[V0][0];
                    c.g[A0][0] = 0x470000;
                    if t {
                        break 'b_1fcdcc;
                    } else {
                        c.cycles += 2;
                        let t = c.g[FP][0] != 0u64;
                        c.g[A0][0] = 0x470000;
                        if t {
                            break 'b_1fce28;
                        } else {
                            c.cycles += 1;
                            c.g[A0][0] = 0x470000;
                            break 'b_1fcdcc;
                        }
                    }
                }
                c.cycles += 23;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x180));
                call_ptr(c, 0x0031_7670, 0x001f_cdd4)?;
                c.g[A0][0] = c.g[S4][0];
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x003a_1000, 0x001f_cde0)?;
                c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x3e4));
                c.g[A1][0] = 0u64;
                call_ptr(c, 0x0015_5130, 0x001f_cdec)?;
                c.g[A0][0] = 0x470000;
                c.g[S0][0] = c.g[V0][0];
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x218));
                call_ptr(c, 0x0031_7670, 0x001f_cdfc)?;
                c.g[A0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x40));
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039_d860, 0x001f_ce08)?;
                c.g[S4][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x0020_a1a8, 0x001f_ce14)?;
                c.g[A0][0] = c.g[S4][0];
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x003a_0c50, 0x001f_ce20)?;
                let t = true;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x480));
                if t {
                    break 'b_1fce90;
                } else {
                    break 'b_1fce28;
                }
            }
            c.cycles += 26;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x168));
            call_ptr(c, 0x0031_7670, 0x001f_ce30)?;
            c.g[A0][0] = c.g[S4][0];
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x003a_1000, 0x001f_ce3c)?;
            c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x3e4));
            c.g[A1][0] = 0u64;
            call_ptr(c, 0x0015_5130, 0x001f_ce48)?;
            c.g[A0][0] = 0x470000;
            c.g[S1][0] = c.g[V0][0];
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x218));
            call_ptr(c, 0x0031_7670, 0x001f_ce58)?;
            c.g[A0][0] = c.lw((c.g[S5][0] as u32).wrapping_add(0x40));
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039_d860, 0x001f_ce64)?;
            c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x3c0));
            c.g[A1][0] = 0x4a0000;
            c.g[S4][0] = c.g[V0][0];
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2138));
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = c.g[S1][0];
            call_ptr(c, 0x0041_7828, 0x001f_ce80)?;
            c.g[A0][0] = c.g[S4][0];
            c.g[A1][0] = c.g[S0][0];
            call_ptr(c, 0x003a_0c50, 0x001f_ce8c)?;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x480));
            break 'b_1fce90;
        }
        c.cycles += 11;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x470));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x460));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x450));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x440));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x430));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x420));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x410));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x400));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x3f0));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x490));
        c.pc = t;
        return Ok(());
    })
}

// Stack slots of `OVScreenRewardList_GetEntryText` (frame 0x370; the first
// 0x28 bytes are the per-reward-type counters).
/// Saved `this` (a0): the reward screen.
pub(crate) const RWL_THIS: u32 = 0x2a0;

/// Saved a1: the output text buffer.
pub(crate) const RWL_OUT: u32 = 0x2a4;

/// Saved a2: the entry index asked for (-1: only count the lines).
pub(crate) const RWL_INDEX: u32 = 0x2a8;

/// The rewards file (`BERewardInt_GetRewardsFile`).
pub(crate) const RWL_FILE: u32 = 0x2ac;

/// The running line counter.
pub(crate) const RWL_LINES: u32 = 0x2b0;

/// The reward category (0..32) being listed.
pub(crate) const RWL_CAT: u32 = 0x2b4;

/// `sp + 0x30`: the format string scratch buffer.
pub(crate) const RWL_FMT: u32 = 0x2b8;

/// `sp + 0x100`: the second scratch buffer.
pub(crate) const RWL_TMP1: u32 = 0x2bc;

/// `sp + 0x1d0`: the third scratch buffer.
pub(crate) const RWL_TMP2: u32 = 0x2c0;

/// `reward type * 4`: the offset into the trophy name table.
pub(crate) const RWL_TYPE4: u32 = 0x2c4;

/// `(base + off)` sign-extended as a 32-bit sum.
#[inline(always)]
pub(crate) fn rwl_add(base: u64, off: u32) -> u64 {
    sx((base as u32).wrapping_add(off))
}

/// `lw off(reg)`.
#[inline(always)]
pub(crate) fn rwl_lw(c: &mut Cpu, reg: usize, off: u32) -> u64 {
    let a = (c.g[reg][0] as u32).wrapping_add(off);
    c.lw(a)
}

/// `lw off(sp)`.
#[inline(always)]
pub(crate) fn rwl_slw(c: &mut Cpu, off: u32) -> u64 {
    let a = sp(c).wrapping_add(off);
    c.lw(a)
}

/// `sw v, off(sp)`.
#[inline(always)]
pub(crate) fn rwl_ssw(c: &mut Cpu, off: u32, v: u64) {
    let a = sp(c).wrapping_add(off);
    c.sw(a, v);
}

/// The world pointer at `gp - 0x848`.
#[inline(always)]
pub(crate) fn rwl_world(c: &mut Cpu) -> u64 {
    c.lw((c.g[GP][0] as u32).wrapping_sub(0x848))
}

/// Second half of a string-table lookup: call the table's lookup method
/// (`vtable + 0x24`) with `this = s0`, `a1 = v0` (the string hash).
pub(crate) fn rwl_vcall(c: &mut Cpu, ret: u32) -> Flow {
    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
    c.g[A0][0] = c.g[S0][0];
    let target = c.g[V1][0] as u32;
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, target, ret)
}

/// Look up a localised string: `a0` holds the name, `s0` the string table
/// object and `s1` its vtable. `StrHash(a0)`, then the table's lookup
/// method; the text pointer is left in `v0`.
pub(crate) fn rwl_string(c: &mut Cpu, ret_hash: u32, ret_lookup: u32) -> Flow {
    c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
    c.g[S0][0] = rwl_add(c.g[S0][0], c.g[V0][0] as u32);
    call_ptr(c, 0x0031_7670, ret_hash)?;
    rwl_vcall(c, ret_lookup)
}

/// Epilogue part shared by the exits that restore `s0` first.
pub(crate) fn rwl_restore_s0(c: &mut Cpu) {
    let a = sp(c).wrapping_add(0x360);
    c.lq(16, a);
}

/// `UStr_Sprintf(out, format = a1, text = v0)` and leave.
pub(crate) fn rwl_sprintf_out(c: &mut Cpu) -> Flow {
    c.cycles += 4;
    c.g[A2][0] = c.g[V0][0];
    call_ptr(c, 0x002c_26d0, 0x001f_fc80)?;
    rwl_restore_s0(c);
    Ok(())
}

/// `UStr_SprintfAsciiFmt_2(out, format = a1, text = v0)` and leave.
pub(crate) fn rwl_sprintf2_out(c: &mut Cpu) -> Flow {
    c.cycles += 4;
    c.g[A2][0] = c.g[V0][0];
    call_ptr(c, 0x002c_27c0, 0x001f_fcc4)?;
    rwl_restore_s0(c);
    Ok(())
}

/// The end of the function when the list has been walked: with index -1
/// the line count goes into `this + 0x9c`, otherwise the "no entry" text
/// is printed.
pub(crate) fn rwl_final(c: &mut Cpu) -> Flow {
    c.cycles += 3;
    c.g[V0][0] = 0xffff_ffff_ffff_ffff;
    let not_count_only = c.g[A0][0] != c.g[V0][0];
    c.g[A1][0] = 0x4a0000;
    if not_count_only {
        c.cycles += 3;
        c.g[A0][0] = rwl_slw(c, RWL_OUT);
        c.g[A1][0] = rwl_add(c.g[A1][0], 0x2560);
        call_ptr(c, 0x002c_27c0, 0x001f_fcd8)?;
    } else {
        c.cycles += 4;
        c.g[V1][0] = rwl_slw(c, RWL_LINES);
        c.g[V0][0] = rwl_slw(c, RWL_THIS);
        let a = (c.g[V0][0] as u32).wrapping_add(0x9c);
        c.sw(a, c.g[V1][0]);
    }
    c.cycles += 1;
    rwl_restore_s0(c);
    Ok(())
}

/// `cOVState_REWARDS::checkRewardList(this, text, index)`: the rewards
/// screen's list of earned rewards. It walks the 32 reward categories and
/// numbers the lines (a category header line, then one line per new reward;
/// money first when there is any) and builds the text of line `index`
/// into `text`; with `index == -1` it only stores the line count at
/// `this + 0x9c`. Frame 0x370, saves s0-s7 and fp.
pub fn ov_screen_reward_list_get_entry_text(c: &mut Cpu) {
    guest(c, |c| {
        rwl_body(c)?;
        // Epilogue ('b_1ffcdc): s0 was restored by the exit paths.
        let frame = Frame { size: 0x370 };
        c.cycles += 11;
        frame.restore(c, 17, 0x350);
        frame.restore(c, 18, 0x340);
        frame.restore(c, 19, 0x330);
        frame.restore(c, 20, 0x320);
        frame.restore(c, 21, 0x310);
        frame.restore(c, 22, 0x300);
        frame.restore(c, 23, 0x2f0);
        frame.restore(c, 30, 0x2e0);
        frame.restore_ra(c, 0x2d0);
        frame.close(c);
        Ok(())
    })
}

pub(crate) fn rwl_body(c: &mut Cpu) -> Flow {
    c.cycles += 31;
    let frame = Frame::open(c, 0x370);
    let a2 = c.g[A2][0];
    rwl_ssw(c, RWL_INDEX, a2);
    frame.save(c, 23, 0x2f0);
    frame.save(c, 16, 0x360);
    c.g[S7][0] = rwl_add(a2, 1);
    frame.save(c, 19, 0x330);
    frame.save(c, 22, 0x300);
    let a0 = c.g[A0][0];
    rwl_ssw(c, RWL_THIS, a0);
    let a1 = c.g[A1][0];
    rwl_ssw(c, RWL_OUT, a1);
    frame.save(c, 17, 0x350);
    frame.save(c, 18, 0x340);
    frame.save(c, 20, 0x320);
    frame.save(c, 21, 0x310);
    frame.save(c, 30, 0x2e0);
    frame.save_ra(c, 0x2d0);
    c.g[S0][0] = 0;
    // The reward interface: BELibrary -> interface 13.
    call_ptr(c, 0x0014_dc80, 0x001f_f7fc)?;
    rwl_ssw(c, RWL_LINES, 0);
    c.g[A1][0] = 0xd;
    c.g[A0][0] = c.g[V0][0];
    call_ptr(c, 0x0014_dd58, 0x001f_f80c)?;
    c.g[S3][0] = c.g[V0][0];
    c.g[A0][0] = c.g[S3][0];
    call_ptr(c, 0x0015_8f20, 0x001f_f818)?; // GetNewRewards
    c.g[S6][0] = c.g[V0][0];
    call_ptr(c, 0x0015_a478, 0x001f_f820)?; // GetRewardsFile
    let file = c.g[V0][0];
    rwl_ssw(c, RWL_FILE, file);
    c.g[A0][0] = c.g[S6][0];
    call_ptr(c, 0x0015_a6f0, 0x001f_f82c)?; // CountNewRewards
    let any_new = c.g[V0][0] != 0;
    c.g[A0][0] = c.g[SP][0];
    if !any_new {
        c.cycles += 3;
        c.g[V0][0] = rwl_lw(c, S3, 0x10);
        if c.g[V0][0] == 0 {
            // Nothing at all to list.
            c.g[A0][0] = rwl_slw(c, RWL_INDEX);
            return rwl_final(c);
        }
    }

    // Zero the per-type counters.
    c.cycles += 6;
    c.g[A1][0] = 0;
    c.g[A2][0] = 0x28;
    call_ptr(c, 0x0041_6210, 0x001f_f84c)?;
    c.g[V1][0] = rwl_lw(c, S3, 0x10);
    c.g[V0][0] = rwl_add(c.g[SP][0], 0x30);
    if c.g[V1][0] != 0 {
        // The money line comes first.
        c.cycles += 3;
        c.g[S0][0] = 1;
        rwl_ssw(c, RWL_LINES, 1);
        if c.g[S7][0] == c.g[S0][0] {
            c.cycles += 21;
            c.g[V0][0] = rwl_world(c);
            c.g[A0][0] = 0x470000;
            c.g[S2][0] = rwl_add(c.g[SP][0], 0x30);
            c.g[A0][0] = rwl_add(c.g[A0][0], 0x400);
            c.g[S0][0] = rwl_lw(c, V0, 0x8c);
            c.g[S1][0] = rwl_lw(c, S0, 4);
            rwl_string(c, 0x001f_f888, 0x001f_f898)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S2][0];
            call_ptr(c, 0x002c_2508, 0x001f_f8a4)?; // UStr_Copy
            c.g[A0][0] = rwl_lw(c, S3, 0x10);
            call_ptr(c, 0x0019_8af0, 0x001f_f8ac)?; // FE_FormatMoney
            c.g[A0][0] = rwl_slw(c, RWL_OUT);
            c.g[A1][0] = c.g[S2][0];
            return rwl_sprintf_out(c);
        }
    }

    c.cycles += 6;
    c.g[V1][0] = rwl_add(c.g[SP][0], 0x100);
    c.g[A0][0] = rwl_add(c.g[SP][0], 0x1d0);
    rwl_ssw(c, RWL_CAT, 0);
    let fmt = c.g[V0][0];
    rwl_ssw(c, RWL_FMT, fmt);
    let tmp1 = c.g[V1][0];
    rwl_ssw(c, RWL_TMP1, tmp1);
    let tmp2 = c.g[A0][0];
    rwl_ssw(c, RWL_TMP2, tmp2);

    // One pass per category (header line plus its reward lines).
    'outer: loop {
        c.cycles += 6;
        c.g[V0][0] = rwl_slw(c, RWL_LINES);
        c.g[S0][0] = rwl_add(c.g[S0][0], 1);
        c.g[S4][0] = 0;
        c.g[S5][0] = c.g[SP][0];
        c.g[V0][0] = rwl_add(c.g[V0][0], 1);
        let lines = c.g[V0][0];
        rwl_ssw(c, RWL_LINES, lines);
        // One pass per reward type s4 (0..10) of the category.
        'inner: loop {
            c.cycles += 8;
            c.g[A1][0] = rwl_slw(c, RWL_CAT);
            c.g[A0][0] = c.g[S6][0];
            c.g[A2][0] = c.g[S4][0];
            call_ptr(c, 0x0015_a750, 0x001f_f8f8)?; // GetNewRewardCount
            c.g[S2][0] = c.g[V0][0];
            c.g[V0][0] = 8;
            let not_type8 = c.g[S4][0] != c.g[V0][0];
            c.g[V0][0] = 9;
            // The loop condition is ready in v0 only on the cheat-char skip.
            let mut cond_ready = false;
            if not_type8 {
                c.cycles += 2;
                if c.g[S4][0] == c.g[V0][0] {
                    // Type 9: the accessory / board graphic.
                    c.cycles += 2;
                    if (c.g[S2][0] as i64) <= 0 {
                        c.g[S4][0] = rwl_add(c.g[S4][0], 1);
                    } else {
                        c.cycles += 3;
                        c.g[S0][0] = rwl_add(c.g[S0][0], 1);
                        if c.g[S7][0] != c.g[S0][0] {
                            c.g[S4][0] = rwl_add(c.g[S4][0], 1);
                        } else {
                            return rwl_accessory_line(c);
                        }
                    }
                } else {
                    // Types 0..7: bit-tested rewards.
                    c.cycles += 2;
                    let none = (c.g[S2][0] as i64) <= 0;
                    c.g[A0][0] = c.g[S6][0];
                    if !none {
                        c.cycles += 7;
                        c.g[A1][0] = c.g[S4][0];
                        call_ptr(c, 0x0015_a768, 0x001f_fa9c)?; // GetNewRewardBits
                        c.g[S3][0] = c.g[S5][0];
                        c.g[A0][0] = sx((c.g[S4][0] as u32) << 2);
                        c.g[S1][0] = c.g[V0][0];
                        let type4 = c.g[A0][0];
                        rwl_ssw(c, RWL_TYPE4, type4);
                        c.g[FP][0] = 2;
                        loop {
                            c.cycles += 6;
                            c.g[A1][0] = rwl_lw(c, S3, 0);
                            c.g[A0][0] = c.g[S6][0];
                            c.g[A2][0] = c.g[S1][0];
                            call_ptr(c, 0x0015_a5f0, 0x001f_fac0)?; // NewRewardsTestBit
                            if c.g[V0][0] == 0 {
                                c.g[V0][0] = rwl_lw(c, S3, 0);
                            } else {
                                c.cycles += 3;
                                c.g[S0][0] = rwl_add(c.g[S0][0], 1);
                                let not_target = c.g[S7][0] != c.g[S0][0];
                                c.g[S2][0] = rwl_add(c.g[S2][0], 0xffff_ffff);
                                if not_target {
                                    c.cycles += 1;
                                    c.g[V0][0] = rwl_lw(c, S3, 0);
                                } else {
                                    c.cycles += 2;
                                    let not_type2 = c.g[S4][0] != c.g[FP][0];
                                    c.g[V0][0] = rwl_world(c);
                                    if not_type2 {
                                        return rwl_trophy_line(c);
                                    }
                                    return rwl_type2_line(c);
                                }
                            }
                            // Count this bit as seen.
                            c.cycles += 3;
                            c.g[V0][0] = rwl_add(c.g[V0][0], 1);
                            let again = (c.g[S2][0] as i64) > 0;
                            let a = c.g[S3][0] as u32;
                            c.sw(a, c.g[V0][0]);
                            if again {
                                if event_due(c) {
                                    deliver_event_here(c, 0x001f_fab0);
                                }
                                continue;
                            }
                            break;
                        }
                    }
                    c.cycles += 1;
                    c.g[S4][0] = rwl_add(c.g[S4][0], 1);
                }
            } else {
                // Type 8: the cheat character.
                c.cycles += 3;
                c.g[V0][0] = ((c.g[S2][0] as i64) < 10i64) as u64;
                let few = c.g[V0][0] != 0;
                c.g[S4][0] = rwl_add(c.g[S4][0], 1);
                if !few {
                    c.cycles += 3;
                    c.g[S0][0] = rwl_add(c.g[S0][0], 1);
                    let not_target = c.g[S7][0] != c.g[S0][0];
                    c.g[V0][0] = ((c.g[S4][0] as i64) < 10i64) as u64;
                    if not_target {
                        cond_ready = true;
                    } else {
                        return rwl_cheat_line(c);
                    }
                }
            }
            if !cond_ready {
                c.cycles += 1;
                c.g[V0][0] = ((c.g[S4][0] as i64) < 10i64) as u64;
            }
            c.cycles += 2;
            let more = c.g[V0][0] != 0;
            c.g[S5][0] = rwl_add(c.g[S5][0], 4);
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x001f_f8e8);
                }
                continue 'inner;
            }
            break 'inner;
        }

        // End of the category.
        c.cycles += 3;
        c.g[A0][0] = rwl_slw(c, RWL_LINES);
        let header_empty = c.g[A0][0] == c.g[S0][0];
        c.g[V0][0] = rwl_slw(c, RWL_LINES);
        if header_empty {
            // No reward lines: drop the header line again.
            c.cycles += 3;
            c.g[V0][0] = rwl_add(c.g[V0][0], 0xffff_ffff);
            let lines = c.g[V0][0];
            rwl_ssw(c, RWL_LINES, lines);
            c.g[S0][0] = c.g[V0][0];
        } else {
            c.cycles += 2;
            if c.g[A0][0] != c.g[S7][0] {
                let s0 = c.g[S0][0];
                rwl_ssw(c, RWL_LINES, s0);
            } else {
                return rwl_header_line(c);
            }
        }
        c.cycles += 5;
        c.g[V1][0] = rwl_slw(c, RWL_CAT);
        c.g[V1][0] = rwl_add(c.g[V1][0], 1);
        c.g[V0][0] = ((c.g[V1][0] as i64) < 32i64) as u64;
        let more = c.g[V0][0] != 0;
        let cat = c.g[V1][0];
        rwl_ssw(c, RWL_CAT, cat);
        if more {
            if event_due(c) {
                deliver_event_here(c, 0x001f_f8d0);
            }
            continue 'outer;
        }
        c.cycles += 1;
        c.g[A0][0] = rwl_slw(c, RWL_INDEX);
        return rwl_final(c);
    }
}

/// Line of type 9 (accessory / board graphic reward): "<kind>: <name>".
pub(crate) fn rwl_accessory_line(c: &mut Cpu) -> Flow {
    c.cycles += 15;
    c.g[A0][0] = 0;
    call_ptr(c, 0x0014_a130, 0x001f_f9c0)?; // BE_FindRiderByProfile
    c.g[A0][0] = c.g[V0][0];
    call_ptr(c, 0x0014_a080, 0x001f_f9c8)?; // BE_GetRiderCharacter
    c.g[S0][0] = c.g[V0][0];
    c.g[S1][0] = c.lh((c.g[S6][0] as u32).wrapping_add(4));
    call_ptr(c, 0x0014_bdb8, 0x001f_f9d4)?; // Bolts_GetDB
    c.g[S2][0] = c.g[V0][0];
    c.g[A1][0] = c.g[S0][0];
    c.g[A2][0] = c.g[S1][0];
    c.g[A0][0] = c.g[S2][0];
    call_ptr(c, 0x0014_d448, 0x001f_f9e8)?; // Bolts_FindBoltIndex
    c.g[A1][0] = c.g[V0][0];
    let found = (c.g[A1][0] as i64) >= 0;
    c.g[V0][0] = 0x38;
    if found {
        c.cycles += 3;
        c.g[V1][0] = rwl_lw(c, S2, 4);
        c.mult(0, 2, 5, 2);
        c.g[S2][0] = rwl_add(c.g[V1][0], c.g[V0][0] as u32);
    } else {
        c.cycles += 2;
        c.g[S2][0] = 0;
    }
    c.cycles += 4;
    c.g[V0][0] = rwl_lw(c, S2, 0x34);
    c.g[V0][0] &= 0x800;
    let accessory = c.g[V0][0] == 0;
    c.g[V0][0] = rwl_world(c);
    if accessory {
        c.cycles += 2;
        c.g[A0][0] = 0x470000;
        c.g[A0][0] = rwl_add(c.g[A0][0], 0x440);
    } else {
        c.cycles += 3;
        c.g[A0][0] = 0x470000;
        c.g[A0][0] = rwl_add(c.g[A0][0], 0x428);
    }
    c.cycles += 24;
    c.g[S0][0] = rwl_lw(c, V0, 0x8c);
    c.g[S1][0] = rwl_lw(c, S0, 4);
    rwl_string(c, 0x001f_fa40, 0x001f_fa50)?;
    c.g[A0][0] = rwl_slw(c, RWL_TMP2);
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, 0x002c_2508, 0x001f_fa5c)?; // UStr_Copy
    c.g[A0][0] = rwl_slw(c, RWL_TMP2);
    c.g[A2][0] = c.g[A0][0];
    c.g[V1][0] = 0x470000;
    c.g[A0][0] = rwl_slw(c, RWL_FMT);
    c.g[A1][0] = rwl_add(c.g[V1][0], 0x418);
    call_ptr(c, 0x002c_2718, 0x001f_fa74)?; // UStr_SprintfAsciiFmt
    c.g[A2][0] = rwl_lw(c, S2, 0x14);
    c.g[A0][0] = rwl_slw(c, RWL_OUT);
    c.g[A1][0] = rwl_slw(c, RWL_FMT);
    call_ptr(c, 0x002c_26d0, 0x001f_fa84)?; // UStr_Sprintf
    rwl_restore_s0(c);
    Ok(())
}

/// Line of type 8 (the cheat character): "<text> <name>".
pub(crate) fn rwl_cheat_line(c: &mut Cpu) -> Flow {
    c.cycles += 31;
    c.g[V0][0] = rwl_world(c);
    c.g[V1][0] = 0x440000;
    c.g[V1][0] = rwl_add(c.g[V1][0], 0x1ce8);
    c.g[S0][0] = rwl_lw(c, V0, 0x8c);
    c.g[A0][0] = rwl_lw(c, V1, 0x20);
    c.g[S1][0] = rwl_lw(c, S0, 4);
    rwl_string(c, 0x001f_f944, 0x001f_f954)?;
    c.g[A0][0] = rwl_slw(c, RWL_FMT);
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, 0x002c_2508, 0x001f_f960)?; // UStr_Copy
    c.g[A0][0] = rwl_slw(c, RWL_FILE);
    c.g[A2][0] = rwl_add(c.g[S2][0], 0xffff_fff6);
    c.g[A1][0] = 8;
    call_ptr(c, 0x0015_af40, 0x001f_f970)?; // GetRewardTypeText
    c.g[A1][0] = rwl_slw(c, RWL_FMT);
    c.g[A2][0] = c.g[V0][0];
    c.g[A0][0] = rwl_slw(c, RWL_TMP1);
    call_ptr(c, 0x002c_26d0, 0x001f_f980)?; // UStr_Sprintf
    c.g[V0][0] = 0x470000;
    c.g[A0][0] = rwl_slw(c, RWL_OUT);
    c.g[A2][0] = rwl_slw(c, RWL_TMP1);
    c.g[A1][0] = rwl_add(c.g[V0][0], 0x418);
    call_ptr(c, 0x002c_27c0, 0x001f_f994)?; // UStr_SprintfAsciiFmt_2
    rwl_restore_s0(c);
    Ok(())
}

/// Line of a bit-tested reward type other than 2: "<type trophy text>
/// <reward name>".
pub(crate) fn rwl_trophy_line(c: &mut Cpu) -> Flow {
    c.cycles += 31;
    c.g[V1][0] = rwl_world(c);
    c.g[V0][0] = 0x440000;
    c.g[A0][0] = rwl_slw(c, RWL_TYPE4);
    c.g[V0][0] = rwl_add(c.g[V0][0], 0x1ce8);
    c.g[S0][0] = rwl_lw(c, V1, 0x8c);
    c.g[S2][0] = rwl_add(c.g[SP][0], 0x100);
    c.g[V0][0] = rwl_add(c.g[A0][0], c.g[V0][0] as u32);
    c.g[S1][0] = rwl_lw(c, S0, 4);
    c.g[A0][0] = rwl_lw(c, V0, 0);
    rwl_string(c, 0x001f_fc2c, 0x001f_fc3c)?;
    c.g[A1][0] = c.g[V0][0];
    c.g[A0][0] = c.g[S2][0];
    call_ptr(c, 0x002c_2508, 0x001f_fc48)?; // UStr_Copy
    c.g[S0][0] = rwl_add(c.g[SP][0], 0x30);
    c.g[A1][0] = 0x470000;
    c.g[A1][0] = rwl_add(c.g[A1][0], 0x418);
    c.g[A2][0] = c.g[S2][0];
    c.g[A0][0] = c.g[S0][0];
    call_ptr(c, 0x002c_2718, 0x001f_fc60)?; // UStr_SprintfAsciiFmt
    c.g[A2][0] = rwl_lw(c, S3, 0);
    c.g[A1][0] = c.g[S4][0];
    c.g[A0][0] = rwl_slw(c, RWL_FILE);
    call_ptr(c, 0x0015_af40, 0x001f_fc70)?; // GetRewardTypeText
    c.g[A0][0] = rwl_slw(c, RWL_OUT);
    c.g[A1][0] = c.g[S0][0];
    rwl_sprintf_out(c)
}

/// Line of reward type 2: its text is the reward type text itself.
pub(crate) fn rwl_type2_line(c: &mut Cpu) -> Flow {
    c.cycles += 18;
    c.g[A1][0] = 2;
    c.g[A0][0] = rwl_slw(c, RWL_FILE);
    c.g[S0][0] = rwl_lw(c, V0, 0x8c);
    c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(8));
    c.g[S1][0] = rwl_lw(c, S0, 4);
    c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
    c.g[S0][0] = rwl_add(c.g[S0][0], c.g[V0][0] as u32);
    call_ptr(c, 0x0015_af40, 0x001f_fafc)?; // GetRewardTypeText
    c.g[A0][0] = c.g[V0][0];
    call_ptr(c, 0x0031_7670, 0x001f_fb04)?; // StrHash
    rwl_vcall(c, 0x001f_fb14)?;
    c.g[V1][0] = 0x470000;
    c.g[A0][0] = rwl_slw(c, RWL_OUT);
    c.g[A1][0] = rwl_add(c.g[V1][0], 0x418);
    rwl_sprintf2_out(c)
}

/// Header line of a category: the category's title, or "platinum earned"
/// for the last three categories when the platinum flag is clear.
pub(crate) fn rwl_header_line(c: &mut Cpu) -> Flow {
    c.cycles += 9;
    c.g[V0][0] = 0x4a0000;
    c.g[S0][0] = 0xffff_ffff_ffff_ffff;
    c.g[S2][0] = rwl_add(c.g[V0][0], 0x22a0);
    call_ptr(c, 0x0014_dc80, 0x001f_fb68)?; // BELibrary_GetInstance
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = 0xd;
    call_ptr(c, 0x0014_dd58, 0x001f_fb74)?; // BELibrary_GetInterface
    if c.g[V0][0] != 0 {
        c.g[S0][0] = rwl_lw(c, V0, 0x14);
    }
    c.cycles += 5;
    c.g[V1][0] = rwl_slw(c, RWL_CAT);
    c.g[V0][0] = rwl_add(c.g[V1][0], 0xffff_ffef);
    c.g[V0][0] = (c.g[V0][0] < 3u64) as u64;
    let plain = c.g[V0][0] == 0;
    c.g[A0][0] = rwl_slw(c, RWL_CAT);
    let mut platinum_earned = false;
    if !plain {
        c.cycles += 2;
        platinum_earned = c.g[S0][0] == 0;
        c.g[V0][0] = rwl_world(c);
    }
    if platinum_earned {
        c.cycles += 4;
        c.g[A0][0] = 0x470000;
        c.g[A0][0] = rwl_add(c.g[A0][0], 0x450);
        c.g[S0][0] = rwl_lw(c, V0, 0x8c);
        c.g[S1][0] = rwl_lw(c, S0, 4);
    } else {
        c.cycles += 9;
        c.g[V1][0] = rwl_world(c);
        c.g[V0][0] = sx((c.g[A0][0] as u32) << 2);
        c.g[S0][0] = rwl_lw(c, V1, 0x8c);
        c.g[A0][0] = 0x440000;
        c.g[A0][0] = rwl_add(c.g[A0][0], 0x1c68);
        c.g[S1][0] = rwl_lw(c, S0, 4);
        c.g[V0][0] = rwl_add(c.g[V0][0], c.g[A0][0] as u32);
        c.g[A0][0] = rwl_lw(c, V0, 0);
    }
    c.cycles += 9;
    rwl_string(c, 0x001f_fca4, 0x001f_fcb4)?;
    c.g[A0][0] = rwl_slw(c, RWL_OUT);
    c.g[A1][0] = c.g[S2][0];
    rwl_sprintf2_out(c)
}

/// `void OVScreenRivalPre_TransitionIn(cOVScreenRivalPre* self, int msg)`
/// (0x001fd320): fills the rival pre-race overlay when it transitions in
/// (`a1` non-zero). Sets the rider name and the title/status texts from the
/// event type at `0x535bc8+0x4a` (rival time, rival points, the six peak
/// events: track name, challenger name, "beat the challenge" lines, goal time
/// or score formatted through `FE_FormatMinSec` / `sprintf`, locked-state
/// handling by the event path and location). Every register, cycle and call is
/// kept as in the generated body (the event jump table as `match`).
pub fn ov_screen_rival_pre_transition_in(c: &mut Cpu) {
    if c.pc != 0x001fd320 {
        return ee::s05::f_001fd320__generated(c);
    }
    guest(c, |c| 'l_1fd320: loop {
        'b_1fdb44: {
            'b_1fdb40: {
                c.cycles += 13;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x980));
                c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x910));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x970));
                c.g[S6][0] = c.g[A0][0];
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x960));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x950));
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x940));
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x930));
                c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x920));
                c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x900));
                c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x8f0));
                let t = c.g[A1][0] == 0u64;
                c.sd((c.g[SP][0] as u32).wrapping_add(0x8e0), c.g[RA][0]);
                if t {
                    break 'b_1fdb40;
                } else {
                    'b_1fd52c: {
                        'b_1fd514: {
                            'b_1fd504: {
                                'b_1fd4c4: {
                                    c.cycles += 52;
                                    c.g[RA][0] = sx(0x001fd35c);
                                    c.g[S0][0] = 0x530000;
                                    call_ptr(c, 0x0014dc80, 0x001fd35c)?;
                                    c.g[A0][0] = c.g[V0][0];
                                    c.g[RA][0] = sx(0x001fd368);
                                    c.g[A1][0] = 0x1;
                                    call_ptr(c, 0x0014dd58, 0x001fd368)?;
                                    c.g[S1][0] = 0x4a0000;
                                    c.g[RA][0] = sx(0x001fd374);
                                    c.g[FP][0] = c.g[V0][0];
                                    call_ptr(c, 0x0014dc80, 0x001fd374)?;
                                    c.g[A0][0] = c.g[V0][0];
                                    c.g[RA][0] = sx(0x001fd380);
                                    c.g[A1][0] = 0u64;
                                    call_ptr(c, 0x0014dd58, 0x001fd380)?;
                                    c.g[RA][0] = sx(0x001fd388);
                                    c.g[S7][0] = c.g[V0][0];
                                    call_ptr(c, 0x0014dc80, 0x001fd388)?;
                                    c.g[A0][0] = c.g[V0][0];
                                    c.g[RA][0] = sx(0x001fd394);
                                    c.g[A1][0] = 0x2;
                                    call_ptr(c, 0x0014dd58, 0x001fd394)?;
                                    c.g[RA][0] = sx(0x001fd39c);
                                    c.sw((c.g[SP][0] as u32).wrapping_add(0x8c0), c.g[V0][0]);
                                    call_ptr(c, 0x0014dc80, 0x001fd39c)?;
                                    c.g[A1][0] = 0xd;
                                    c.g[RA][0] = sx(0x001fd3a8);
                                    c.g[A0][0] = c.g[V0][0];
                                    call_ptr(c, 0x0014dd58, 0x001fd3a8)?;
                                    c.sw((c.g[SP][0] as u32).wrapping_add(0x8c4), c.g[V0][0]);
                                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x5bc8));
                                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x2520));
                                    c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc0));
                                    c.sw((c.g[SP][0] as u32).wrapping_add(0x8c8), c.g[V0][0]);
                                    c.g[RA][0] = sx(0x001fd3c8);
                                    c.g[S5][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x4a));
                                    call_ptr(c, 0x00317670, 0x001fd3c8)?;
                                    c.g[A0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x40));
                                    c.g[RA][0] = sx(0x001fd3d4);
                                    c.g[A1][0] = c.g[V0][0];
                                    call_ptr(c, 0x0039d860, 0x001fd3d4)?;
                                    c.g[S4][0] = c.g[V0][0];
                                    c.g[A0][0] = c.g[FP][0];
                                    c.g[RA][0] = sx(0x001fd3e4);
                                    c.g[A1][0] = 0u64;
                                    call_ptr(c, 0x00146e98, 0x001fd3e4)?;
                                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8c0));
                                    c.g[RA][0] = sx(0x001fd3f0);
                                    c.g[A1][0] = c.g[V0][0];
                                    call_ptr(c, 0x0014ee58, 0x001fd3f0)?;
                                    c.g[A0][0] = c.g[S4][0];
                                    c.g[RA][0] = sx(0x001fd3fc);
                                    c.g[A1][0] = c.g[V0][0];
                                    call_ptr(c, 0x003a0c50, 0x001fd3fc)?;
                                    c.g[A0][0] = 0x4a0000;
                                    c.g[RA][0] = sx(0x001fd408);
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x21b8));
                                    call_ptr(c, 0x00317670, 0x001fd408)?;
                                    c.g[A0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x40));
                                    c.g[RA][0] = sx(0x001fd414);
                                    c.g[A1][0] = c.g[V0][0];
                                    call_ptr(c, 0x0039d860, 0x001fd414)?;
                                    c.g[S4][0] = c.g[V0][0];
                                    c.g[V0][0] = 0x5;
                                    let t = c.g[S5][0] == c.g[V0][0];
                                    c.g[V0][0] = ((c.g[S5][0] as i64) < 6i64) as u64;
                                    if t {
                                        break 'b_1fd4c4;
                                    } else {
                                        'b_1fd450: {
                                            'b_1fd43c: {
                                                c.cycles += 2;
                                                let t = c.g[V0][0] == 0u64;
                                                c.g[V0][0] = 0x4;
                                                if t {
                                                    break 'b_1fd43c;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = c.g[S5][0] == c.g[V0][0];
                                                    c.g[S3][0] =
                                                        sx((c.g[SP][0] as u32).wrapping_add(0x290));
                                                    if t {
                                                        break 'b_1fd450;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = true;
                                                        c.g[A0][0] = 0x4a0000;
                                                        if t {
                                                            break 'b_1fd52c;
                                                        } else {
                                                            break 'b_1fd43c;
                                                        }
                                                    }
                                                }
                                            }
                                            c.cycles += 3;
                                            c.g[V0][0] = ((c.g[S5][0] as i64) < 12i64) as u64;
                                            if c.g[V0][0] == 0u64 {
                                                c.g[A0][0] = 0x4a0000;
                                                break 'b_1fd52c;
                                            } else {
                                                c.cycles += 2;
                                                let t = true;

                                                if t {
                                                    break 'b_1fd514;
                                                } else {
                                                    break 'b_1fd450;
                                                }
                                            }
                                        }
                                        c.cycles += 29;
                                        c.g[RA][0] = sx(0x001fd458);
                                        c.g[A0][0] = c.g[S7][0];
                                        call_ptr(c, 0x00144bf0, 0x001fd458)?;
                                        c.g[A1][0] = c.g[V0][0];
                                        c.g[RA][0] = sx(0x001fd464);
                                        c.g[A0][0] = c.g[S3][0];
                                        call_ptr(c, 0x002c2540, 0x001fd464)?;
                                        c.g[S2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x260));
                                        c.g[A1][0] = 0x4a0000;
                                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2140));
                                        c.g[RA][0] = sx(0x001fd478);
                                        c.g[A0][0] = c.g[S2][0];
                                        call_ptr(c, 0x002c2540, 0x001fd478)?;
                                        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                        c.g[A0][0] = 0x470000;
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1ab8));
                                        c.g[S0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8c));
                                        c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                                        c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                                        c.g[RA][0] = sx(0x001fd498);
                                        c.g[S0][0] =
                                            sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                        call_ptr(c, 0x00317670, 0x001fd498)?;
                                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                                        c.g[A0][0] = c.g[S0][0];
                                        let t = c.g[V1][0] as u32;
                                        c.g[RA][0] = sx(0x001fd4a8);
                                        c.g[A1][0] = c.g[V0][0];
                                        call_ptr(c, t, 0x001fd4a8)?;
                                        c.g[A1][0] = c.g[S2][0];
                                        c.g[A0][0] = c.g[SP][0];
                                        c.g[A2][0] = c.g[S3][0];
                                        c.g[RA][0] = sx(0x001fd4bc);
                                        c.g[A3][0] = c.g[V0][0];
                                        call_ptr(c, 0x002c26d0, 0x001fd4bc)?;
                                        let t = true;
                                        c.g[A0][0] = c.g[S4][0];
                                        if t {
                                            break 'b_1fd504;
                                        } else {
                                            break 'b_1fd4c4;
                                        }
                                    }
                                }
                                c.cycles += 16;
                                c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x290));
                                c.g[RA][0] = sx(0x001fd4d0);
                                c.g[A0][0] = c.g[S7][0];
                                call_ptr(c, 0x00144bf0, 0x001fd4d0)?;
                                c.g[A1][0] = c.g[V0][0];
                                c.g[RA][0] = sx(0x001fd4dc);
                                c.g[A0][0] = c.g[S1][0];
                                call_ptr(c, 0x002c2540, 0x001fd4dc)?;
                                c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x260));
                                c.g[A1][0] = 0x470000;
                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1aa8));
                                c.g[RA][0] = sx(0x001fd4f0);
                                c.g[A0][0] = c.g[S0][0];
                                call_ptr(c, 0x002c2540, 0x001fd4f0)?;
                                c.g[A1][0] = c.g[S0][0];
                                c.g[A0][0] = c.g[SP][0];
                                c.g[RA][0] = sx(0x001fd500);
                                c.g[A2][0] = c.g[S1][0];
                                call_ptr(c, 0x002c26d0, 0x001fd500)?;
                                c.g[A0][0] = c.g[S4][0];
                                break 'b_1fd504;
                            }
                            c.cycles += 4;
                            c.g[RA][0] = sx(0x001fd50c);
                            c.g[A1][0] = c.g[SP][0];
                            call_ptr(c, 0x003a0e90, 0x001fd50c)?;
                            let t = true;
                            c.g[A0][0] = 0x4a0000;
                            if t {
                                break 'b_1fd52c;
                            } else {
                                break 'b_1fd514;
                            }
                        }
                        c.cycles += 6;
                        c.g[RA][0] = sx(0x001fd51c);
                        c.g[A0][0] = c.g[S5][0];
                        call_ptr(c, 0x001fd190, 0x001fd51c)?;
                        c.g[A0][0] = c.g[S4][0];
                        c.g[RA][0] = sx(0x001fd528);
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x003a0e90, 0x001fd528)?;
                        c.g[A0][0] = 0x4a0000;
                        break 'b_1fd52c;
                    }
                    'b_1fd654: {
                        'b_1fd624: {
                            'b_1fd5f8: {
                                'b_1fd5e0: {
                                    c.cycles += 42;
                                    c.g[S0][0] = 0x4a0000;
                                    c.g[RA][0] = sx(0x001fd538);
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2528));
                                    call_ptr(c, 0x00317670, 0x001fd538)?;
                                    c.g[A0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x40));
                                    c.g[RA][0] = sx(0x001fd544);
                                    c.g[A1][0] = c.g[V0][0];
                                    call_ptr(c, 0x0039d860, 0x001fd544)?;
                                    c.sw((c.g[SP][0] as u32).wrapping_add(0x8cc), c.g[V0][0]);
                                    c.g[RA][0] = sx(0x001fd550);
                                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x2530));
                                    call_ptr(c, 0x00317670, 0x001fd550)?;
                                    c.g[A0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x40));
                                    c.g[RA][0] = sx(0x001fd55c);
                                    c.g[A1][0] = c.g[V0][0];
                                    call_ptr(c, 0x0039d860, 0x001fd55c)?;
                                    c.g[A0][0] = 0x4a0000;
                                    c.sw((c.g[SP][0] as u32).wrapping_add(0x8d0), c.g[V0][0]);
                                    c.g[RA][0] = sx(0x001fd56c);
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x21b0));
                                    call_ptr(c, 0x00317670, 0x001fd56c)?;
                                    c.g[A0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x40));
                                    c.g[RA][0] = sx(0x001fd578);
                                    c.g[A1][0] = c.g[V0][0];
                                    call_ptr(c, 0x0039d860, 0x001fd578)?;
                                    c.g[A0][0] = 0x470000;
                                    c.g[S4][0] = c.g[V0][0];
                                    c.g[RA][0] = sx(0x001fd588);
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1a10));
                                    call_ptr(c, 0x00317670, 0x001fd588)?;
                                    c.g[A1][0] = c.g[V0][0];
                                    c.g[RA][0] = sx(0x001fd594);
                                    c.g[A0][0] = c.g[S4][0];
                                    call_ptr(c, 0x003a1000, 0x001fd594)?;
                                    c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x2c0));
                                    c.g[RA][0] = sx(0x001fd5a0);
                                    c.g[A0][0] = c.g[S7][0];
                                    call_ptr(c, 0x00145750, 0x001fd5a0)?;
                                    c.g[S3][0] = c.g[S0][0];
                                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8c0));
                                    c.g[A1][0] = c.g[V0][0];
                                    c.g[RA][0] = sx(0x001fd5b4);
                                    c.g[A2][0] = 0u64;
                                    call_ptr(c, 0x0014eec8, 0x001fd5b4)?;
                                    c.g[A1][0] = c.g[V0][0];
                                    c.g[RA][0] = sx(0x001fd5c0);
                                    c.g[A0][0] = c.g[S0][0];
                                    call_ptr(c, 0x002c2540, 0x001fd5c0)?;
                                    c.g[V1][0] = sx((c.g[S5][0] as u32).wrapping_sub(0x4));
                                    c.g[S2][0] = c.g[V1][0];
                                    c.g[V1][0] = (c.g[V1][0] < 0x2u64) as u64;
                                    let t = c.g[V1][0] == 0u64;
                                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                    if t {
                                        break 'b_1fd5e0;
                                    } else {
                                        c.cycles += 3;
                                        c.g[A0][0] = 0x470000;
                                        let t = true;
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x98));
                                        if t {
                                            break 'b_1fd5f8;
                                        } else {
                                            break 'b_1fd5e0;
                                        }
                                    }
                                }
                                c.cycles += 4;
                                c.g[V0][0] = sx((c.g[S5][0] as u32).wrapping_sub(0x6));
                                c.g[V0][0] = (c.g[V0][0] < 0x3u64) as u64;
                                let t = c.g[V0][0] == 0u64;
                                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                if t {
                                    break 'b_1fd624;
                                } else {
                                    c.cycles += 2;
                                    c.g[A0][0] = 0x470000;
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x80));
                                    break 'b_1fd5f8;
                                }
                            }
                            c.cycles += 11;
                            c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                            c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                            c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                            c.g[RA][0] = sx(0x001fd60c);
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                            call_ptr(c, 0x00317670, 0x001fd60c)?;
                            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                            c.g[A0][0] = c.g[S0][0];
                            let t = c.g[V1][0] as u32;
                            c.g[RA][0] = sx(0x001fd61c);
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, t, 0x001fd61c)?;
                            let t = true;
                            c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x300));
                            if t {
                                break 'b_1fd654;
                            } else {
                                break 'b_1fd624;
                            }
                        }
                        c.cycles += 12;
                        c.g[A0][0] = 0x470000;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x68));
                        c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                        c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                        c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                        c.g[RA][0] = sx(0x001fd640);
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                        call_ptr(c, 0x00317670, 0x001fd640)?;
                        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                        c.g[A0][0] = c.g[S0][0];
                        let t = c.g[V1][0] as u32;
                        c.g[RA][0] = sx(0x001fd650);
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, t, 0x001fd650)?;
                        c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x300));
                        break 'b_1fd654;
                    }
                    c.cycles += 16;
                    c.g[A2][0] = c.g[S3][0];
                    c.g[A1][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x001fd664);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x002c26d0, 0x001fd664)?;
                    c.g[V0][0] = 0x4a0000;
                    c.g[RA][0] = sx(0x001fd670);
                    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2520));
                    call_ptr(c, 0x00317670, 0x001fd670)?;
                    c.g[A0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x40));
                    c.g[RA][0] = sx(0x001fd67c);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x001fd67c)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x001fd688);
                    c.g[A1][0] = c.g[S0][0];
                    call_ptr(c, 0x003a0d00, 0x001fd688)?;
                    c.g[V0][0] = (c.g[S2][0] < 0x8u64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = 0x470000;
                    if t {
                        break 'b_1fdb40;
                    } else {
                        'b_1fdb2c: {
                            'b_1fdb10: {
                                'b_1fda18: {
                                    'b_1fd8c0: {
                                        'b_1fd8ac: {
                                            'b_1fd88c: {
                                                'b_1fd878: {
                                                    'b_1fd794: {
                                                        c.cycles += 6;
                                                        c.g[V1][0] = sx((c.g[S2][0] as u32) << 2);
                                                        c.g[V0][0] =
                                                            sx((c.g[V0][0] as u32)
                                                                .wrapping_add(0xc0));
                                                        c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                            .wrapping_add(c.g[V0][0] as u32));
                                                        c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                                                        let t = c.g[A0][0] as u32;
                                                        match t {
                                                            0x001fd320 => {
                                                                if event_due(c) {
                                                                    deliver_event_here(
                                                                        c, 0x001fd320,
                                                                    );
                                                                }
                                                                continue 'l_1fd320;
                                                            }
                                                            0x001fd6ac => {
                                                                c.cycles += 18;
                                                                c.g[A0][0] = 0x470000;
                                                                c.g[RA][0] = sx(0x001fd6b8);
                                                                c.g[A0][0] = sx((c.g[A0][0]
                                                                    as u32)
                                                                    .wrapping_sub(0x1c8));
                                                                call_ptr(
                                                                    c, 0x00317670, 0x001fd6b8,
                                                                )?;
                                                                c.g[A0][0] = c
                                                                    .lw((c.g[SP][0] as u32)
                                                                        .wrapping_add(0x8cc));
                                                                c.g[RA][0] = sx(0x001fd6c4);
                                                                c.g[A1][0] = c.g[V0][0];
                                                                call_ptr(
                                                                    c, 0x003a1000, 0x001fd6c4,
                                                                )?;
                                                                c.g[A0][0] = 0x470000;
                                                                c.g[RA][0] = sx(0x001fd6d0);
                                                                c.g[A0][0] = sx((c.g[A0][0]
                                                                    as u32)
                                                                    .wrapping_sub(0x50));
                                                                call_ptr(
                                                                    c, 0x00317670, 0x001fd6d0,
                                                                )?;
                                                                c.g[A0][0] = c
                                                                    .lw((c.g[SP][0] as u32)
                                                                        .wrapping_add(0x8d0));
                                                                c.g[RA][0] = sx(0x001fd6dc);
                                                                c.g[A1][0] = c.g[V0][0];
                                                                call_ptr(
                                                                    c, 0x003a1000, 0x001fd6dc,
                                                                )?;
                                                                c.g[V0][0] = 0x530000;
                                                                c.g[A0][0] = 0x1;
                                                                c.g[V1][0] = sx((c.g[V0][0]
                                                                    as u32)
                                                                    .wrapping_add(0x5bc8));
                                                                c.g[V0][0] = c
                                                                    .lb((c.g[V1][0] as u32)
                                                                        .wrapping_add(0x49));
                                                                let t = c.g[V0][0] == c.g[A0][0];
                                                                c.g[V1][0] = 0x4a0000;
                                                                if t {
                                                                    break 'b_1fd88c;
                                                                } else {
                                                                    'b_1fd72c: {
                                                                        c.cycles += 5;
                                                                        c.g[RA][0] = sx(0x001fd6fc);
                                                                        c.g[A0][0] = c.g[S7][0];
                                                                        call_ptr(
                                                                            c, 0x00144be0,
                                                                            0x001fd6fc,
                                                                        )?;
                                                                        c.g[V1][0] = 0xe;
                                                                        let t = c.g[V0][0]
                                                                            != c.g[V1][0];

                                                                        if t {
                                                                            break 'b_1fd72c;
                                                                        } else {
                                                                            c.cycles += 9;
                                                                            c.g[RA][0] =
                                                                                sx(0x001fd710);
                                                                            c.g[A0][0] = 0u64;
                                                                            call_ptr(
                                                                                c, 0x0014a080,
                                                                                0x001fd710,
                                                                            )?;
                                                                            c.g[A2][0] = c.g[V0][0];
                                                                            c.g[A0][0] = c.g[FP][0];
                                                                            c.g[A1][0] = 0u64;
                                                                            c.g[RA][0] =
                                                                                sx(0x001fd724);
                                                                            c.g[A3][0] = 0x6;
                                                                            call_ptr(
                                                                                c, 0x00146a70,
                                                                                0x001fd724,
                                                                            )?;
                                                                            let t =
                                                                                c.g[V0][0] == 0u64;
                                                                            c.g[V1][0] = 0x4a0000;
                                                                            if t {
                                                                                break 'b_1fd88c;
                                                                            } else {
                                                                                break 'b_1fd72c;
                                                                            }
                                                                        }
                                                                    }
                                                                    'b_1fd764: {
                                                                        c.cycles += 5;
                                                                        c.g[RA][0] = sx(0x001fd734);
                                                                        c.g[A0][0] = c.g[S7][0];
                                                                        call_ptr(
                                                                            c, 0x00144be0,
                                                                            0x001fd734,
                                                                        )?;
                                                                        c.g[V1][0] = 0xf;
                                                                        let t = c.g[V0][0]
                                                                            != c.g[V1][0];

                                                                        if t {
                                                                            break 'b_1fd764;
                                                                        } else {
                                                                            c.cycles += 9;
                                                                            c.g[RA][0] =
                                                                                sx(0x001fd748);
                                                                            c.g[A0][0] = 0u64;
                                                                            call_ptr(
                                                                                c, 0x0014a080,
                                                                                0x001fd748,
                                                                            )?;
                                                                            c.g[A2][0] = c.g[V0][0];
                                                                            c.g[A0][0] = c.g[FP][0];
                                                                            c.g[A1][0] = 0u64;
                                                                            c.g[RA][0] =
                                                                                sx(0x001fd75c);
                                                                            c.g[A3][0] = 0x7;
                                                                            call_ptr(
                                                                                c, 0x00146a70,
                                                                                0x001fd75c,
                                                                            )?;
                                                                            let t =
                                                                                c.g[V0][0] == 0u64;
                                                                            c.g[V1][0] = 0x4a0000;
                                                                            if t {
                                                                                break 'b_1fd88c;
                                                                            } else {
                                                                                break 'b_1fd764;
                                                                            }
                                                                        }
                                                                    }
                                                                    c.cycles += 5;
                                                                    c.g[RA][0] = sx(0x001fd76c);
                                                                    c.g[A0][0] = c.g[S7][0];
                                                                    call_ptr(
                                                                        c, 0x00144be0, 0x001fd76c,
                                                                    )?;
                                                                    c.g[V1][0] = 0x10;
                                                                    let t =
                                                                        c.g[V0][0] != c.g[V1][0];
                                                                    c.g[A1][0] = 0x470000;
                                                                    if t {
                                                                        break 'b_1fd8ac;
                                                                    } else {
                                                                        c.cycles += 7;
                                                                        c.g[RA][0] = sx(0x001fd780);
                                                                        c.g[A0][0] = 0u64;
                                                                        call_ptr(
                                                                            c, 0x0014a080,
                                                                            0x001fd780,
                                                                        )?;
                                                                        c.g[A0][0] = c.g[FP][0];
                                                                        c.g[A2][0] = c.g[V0][0];
                                                                        c.g[A1][0] = 0u64;
                                                                        let t = true;
                                                                        c.g[A3][0] = 0x8;
                                                                        if t {
                                                                            break 'b_1fd878;
                                                                        } else {
                                                                            break 'b_1fd794;
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                            0x001fd794 => {
                                                                break 'b_1fd794;
                                                            }
                                                            0x001fd8c0 => {
                                                                break 'b_1fd8c0;
                                                            }
                                                            0x001fda18 => {
                                                                break 'b_1fda18;
                                                            }
                                                            _ => {
                                                                c.jump_out(
                                                                    t, 0x001fd320, 0x001fdb74,
                                                                );
                                                                return Ok(());
                                                            }
                                                        }
                                                    }
                                                    c.cycles += 18;
                                                    c.g[A0][0] = 0x470000;
                                                    c.g[RA][0] = sx(0x001fd7a0);
                                                    c.g[A0][0] =
                                                        sx((c.g[A0][0] as u32).wrapping_sub(0x20));
                                                    call_ptr(c, 0x00317670, 0x001fd7a0)?;
                                                    c.g[A0][0] =
                                                        c.lw(
                                                            (c.g[SP][0] as u32).wrapping_add(0x8cc)
                                                        );
                                                    c.g[RA][0] = sx(0x001fd7ac);
                                                    c.g[A1][0] = c.g[V0][0];
                                                    call_ptr(c, 0x003a1000, 0x001fd7ac)?;
                                                    c.g[A0][0] = 0x470000;
                                                    c.g[RA][0] = sx(0x001fd7b8);
                                                    c.g[A0][0] =
                                                        sx((c.g[A0][0] as u32).wrapping_add(0x0));
                                                    call_ptr(c, 0x00317670, 0x001fd7b8)?;
                                                    c.g[A0][0] =
                                                        c.lw(
                                                            (c.g[SP][0] as u32).wrapping_add(0x8d0)
                                                        );
                                                    c.g[RA][0] = sx(0x001fd7c4);
                                                    c.g[A1][0] = c.g[V0][0];
                                                    call_ptr(c, 0x003a1000, 0x001fd7c4)?;
                                                    c.g[V0][0] = 0x530000;
                                                    c.g[A0][0] = 0x1;
                                                    c.g[V1][0] =
                                                        sx((c.g[V0][0] as u32)
                                                            .wrapping_add(0x5bc8));
                                                    c.g[V0][0] = c
                                                        .lb((c.g[V1][0] as u32).wrapping_add(0x49));
                                                    let t = c.g[V0][0] == c.g[A0][0];
                                                    c.g[V1][0] = 0x4a0000;
                                                    if t {
                                                        break 'b_1fd88c;
                                                    } else {
                                                        'b_1fd814: {
                                                            c.cycles += 5;
                                                            c.g[RA][0] = sx(0x001fd7e4);
                                                            c.g[A0][0] = c.g[S7][0];
                                                            call_ptr(c, 0x00144be0, 0x001fd7e4)?;
                                                            c.g[V1][0] = 0xe;
                                                            let t = c.g[V0][0] != c.g[V1][0];

                                                            if t {
                                                                break 'b_1fd814;
                                                            } else {
                                                                c.cycles += 9;
                                                                c.g[RA][0] = sx(0x001fd7f8);
                                                                c.g[A0][0] = 0u64;
                                                                call_ptr(
                                                                    c, 0x0014a080, 0x001fd7f8,
                                                                )?;
                                                                c.g[A2][0] = c.g[V0][0];
                                                                c.g[A0][0] = c.g[FP][0];
                                                                c.g[A1][0] = 0u64;
                                                                c.g[RA][0] = sx(0x001fd80c);
                                                                c.g[A3][0] = 0x9;
                                                                call_ptr(
                                                                    c, 0x00146a70, 0x001fd80c,
                                                                )?;
                                                                let t = c.g[V0][0] == 0u64;
                                                                c.g[V1][0] = 0x4a0000;
                                                                if t {
                                                                    break 'b_1fd88c;
                                                                } else {
                                                                    break 'b_1fd814;
                                                                }
                                                            }
                                                        }
                                                        'b_1fd84c: {
                                                            c.cycles += 5;
                                                            c.g[RA][0] = sx(0x001fd81c);
                                                            c.g[A0][0] = c.g[S7][0];
                                                            call_ptr(c, 0x00144be0, 0x001fd81c)?;
                                                            c.g[V1][0] = 0xf;
                                                            let t = c.g[V0][0] != c.g[V1][0];

                                                            if t {
                                                                break 'b_1fd84c;
                                                            } else {
                                                                c.cycles += 9;
                                                                c.g[RA][0] = sx(0x001fd830);
                                                                c.g[A0][0] = 0u64;
                                                                call_ptr(
                                                                    c, 0x0014a080, 0x001fd830,
                                                                )?;
                                                                c.g[A2][0] = c.g[V0][0];
                                                                c.g[A0][0] = c.g[FP][0];
                                                                c.g[A1][0] = 0u64;
                                                                c.g[RA][0] = sx(0x001fd844);
                                                                c.g[A3][0] = 0xa;
                                                                call_ptr(
                                                                    c, 0x00146a70, 0x001fd844,
                                                                )?;
                                                                let t = c.g[V0][0] == 0u64;
                                                                c.g[V1][0] = 0x4a0000;
                                                                if t {
                                                                    break 'b_1fd88c;
                                                                } else {
                                                                    break 'b_1fd84c;
                                                                }
                                                            }
                                                        }
                                                        c.cycles += 5;
                                                        c.g[RA][0] = sx(0x001fd854);
                                                        c.g[A0][0] = c.g[S7][0];
                                                        call_ptr(c, 0x00144be0, 0x001fd854)?;
                                                        c.g[V1][0] = 0x10;
                                                        let t = c.g[V0][0] != c.g[V1][0];
                                                        c.g[A1][0] = 0x470000;
                                                        if t {
                                                            break 'b_1fd8ac;
                                                        } else {
                                                            c.cycles += 6;
                                                            c.g[RA][0] = sx(0x001fd868);
                                                            c.g[A0][0] = 0u64;
                                                            call_ptr(c, 0x0014a080, 0x001fd868)?;
                                                            c.g[A0][0] = c.g[FP][0];
                                                            c.g[A2][0] = c.g[V0][0];
                                                            c.g[A1][0] = 0u64;
                                                            c.g[A3][0] = 0xb;
                                                            break 'b_1fd878;
                                                        }
                                                    }
                                                }
                                                c.cycles += 4;
                                                c.g[RA][0] = sx(0x001fd880);
                                                call_ptr(c, 0x00146a70, 0x001fd880)?;
                                                if c.g[V0][0] != 0u64 {
                                                    c.g[A1][0] = 0x470000;
                                                    break 'b_1fd8ac;
                                                } else {
                                                    c.cycles += 1;
                                                    c.g[V1][0] = 0x4a0000;
                                                    break 'b_1fd88c;
                                                }
                                            }
                                            c.cycles += 8;
                                            c.g[A0][0] = c.g[S6][0];
                                            c.g[RA][0] = sx(0x001fd898);
                                            c.g[A1][0] =
                                                sx((c.g[V1][0] as u32).wrapping_add(0x2530));
                                            call_ptr(c, 0x0039e8b8, 0x001fd898)?;
                                            c.g[A1][0] = 0x4a0000;
                                            c.g[A0][0] = c.g[S6][0];
                                            c.g[RA][0] = sx(0x001fd8a8);
                                            c.g[A1][0] =
                                                sx((c.g[A1][0] as u32).wrapping_add(0x2538));
                                            call_ptr(c, 0x0039e8b8, 0x001fd8a8)?;
                                            c.g[A1][0] = 0x470000;
                                            break 'b_1fd8ac;
                                        }
                                        c.cycles += 5;
                                        c.g[A0][0] = c.g[S6][0];
                                        c.g[RA][0] = sx(0x001fd8b8);
                                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x30));
                                        call_ptr(c, 0x0039e8b8, 0x001fd8b8)?;
                                        let t = true;
                                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x970));
                                        if t {
                                            break 'b_1fdb44;
                                        } else {
                                            break 'b_1fd8c0;
                                        }
                                    }
                                    'b_1fd910: {
                                        'b_1fd8f8: {
                                            c.cycles += 10;
                                            c.g[A0][0] = 0x470000;
                                            c.g[RA][0] = sx(0x001fd8cc);
                                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x30));
                                            call_ptr(c, 0x00317670, 0x001fd8cc)?;
                                            c.g[A0][0] =
                                                c.lw((c.g[S6][0] as u32).wrapping_add(0x40));
                                            c.g[RA][0] = sx(0x001fd8d8);
                                            c.g[A1][0] = c.g[V0][0];
                                            call_ptr(c, 0x0039d860, 0x001fd8d8)?;
                                            c.g[V1][0] =
                                                c.lw((c.g[SP][0] as u32).wrapping_add(0x8c8));
                                            c.g[A0][0] =
                                                c.lw((c.g[V1][0] as u32).wrapping_add(0x78));
                                            let t = (c.g[A0][0] as i64) < 0;
                                            c.g[S4][0] = c.g[V0][0];
                                            if t {
                                                break 'b_1fd8f8;
                                            } else {
                                                c.cycles += 4;
                                                c.mtc1(4, 0);
                                                c.cvt_s_w(0, 0);
                                                let t = true;

                                                if t {
                                                    break 'b_1fd910;
                                                } else {
                                                    break 'b_1fd8f8;
                                                }
                                            }
                                        }
                                        c.cycles += 6;
                                        c.g[V1][0] = sx((c.g[A0][0] as u32) >> 1);
                                        c.g[V0][0] = c.g[A0][0] & 0x1;
                                        c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
                                        c.mtc1(2, 0);
                                        c.cvt_s_w(0, 0);
                                        c.add_s(0, 0, 0);
                                        break 'b_1fd910;
                                    }
                                    'b_1fda10: {
                                        'b_1fda04: {
                                            c.cycles += 45;
                                            c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(0x5540));
                                            c.g[RA][0] = sx(0x001fd91c);
                                            c.mul_s(12, 0, 12);
                                            call_ptr(c, 0x004139f8, 0x001fd91c)?;
                                            c.g[S3][0] =
                                                sx((c.g[SP][0] as u32).wrapping_add(0x620));
                                            c.g[RA][0] = sx(0x001fd928);
                                            c.g[A0][0] = c.g[V0][0];
                                            Callee {
                                                addr: 0x0020a1a8,
                                                f: ee::s05::FE_FormatMinSec,
                                            }
                                            .call(c, 0x001fd928)?;
                                            c.g[A1][0] = c.g[V0][0];
                                            c.g[RA][0] = sx(0x001fd934);
                                            c.g[A0][0] = c.g[S3][0];
                                            call_ptr(c, 0x002c2540, 0x001fd934)?;
                                            c.g[V1][0] =
                                                c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                            c.g[A0][0] = 0x470000;
                                            c.g[A0][0] =
                                                sx((c.g[A0][0] as u32).wrapping_sub(0x16e8));
                                            c.g[S0][0] =
                                                c.lw((c.g[V1][0] as u32).wrapping_add(0x8c));
                                            c.g[S1][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                                            c.g[V0][0] =
                                                c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                                            c.g[RA][0] = sx(0x001fd954);
                                            c.g[S0][0] =
                                                sx((c.g[S0][0] as u32)
                                                    .wrapping_add(c.g[V0][0] as u32));
                                            call_ptr(c, 0x00317670, 0x001fd954)?;
                                            c.g[V1][0] =
                                                c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                                            c.g[A0][0] = c.g[S0][0];
                                            let t = c.g[V1][0] as u32;
                                            c.g[RA][0] = sx(0x001fd964);
                                            c.g[A1][0] = c.g[V0][0];
                                            call_ptr(c, t, 0x001fd964)?;
                                            c.g[S2][0] =
                                                sx((c.g[SP][0] as u32).wrapping_add(0x290));
                                            c.g[A1][0] = 0x4a0000;
                                            c.g[S1][0] = c.g[V0][0];
                                            c.g[A1][0] =
                                                sx((c.g[A1][0] as u32).wrapping_add(0x2550));
                                            c.g[RA][0] = sx(0x001fd97c);
                                            c.g[A0][0] = c.g[S2][0];
                                            call_ptr(c, 0x002c2540, 0x001fd97c)?;
                                            c.g[S0][0] =
                                                sx((c.g[SP][0] as u32).wrapping_add(0x660));
                                            c.g[A2][0] = c.g[S1][0];
                                            c.g[A3][0] = c.g[S3][0];
                                            c.g[A1][0] = c.g[S2][0];
                                            c.g[RA][0] = sx(0x001fd994);
                                            c.g[A0][0] = c.g[S0][0];
                                            call_ptr(c, 0x002c26d0, 0x001fd994)?;
                                            c.g[A1][0] = c.g[S0][0];
                                            c.g[RA][0] = sx(0x001fd9a0);
                                            c.g[A0][0] = c.g[S4][0];
                                            call_ptr(c, 0x003a0d00, 0x001fd9a0)?;
                                            c.g[A0][0] = 0x470000;
                                            c.g[RA][0] = sx(0x001fd9ac);
                                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                                            call_ptr(c, 0x00317670, 0x001fd9ac)?;
                                            c.g[A0][0] =
                                                c.lw((c.g[SP][0] as u32).wrapping_add(0x8cc));
                                            c.g[RA][0] = sx(0x001fd9b8);
                                            c.g[A1][0] = c.g[V0][0];
                                            call_ptr(c, 0x003a1000, 0x001fd9b8)?;
                                            c.g[V0][0] = 0x8;
                                            let t = c.g[S5][0] == c.g[V0][0];
                                            c.g[V0][0] = 0x7;
                                            if t {
                                                break 'b_1fda04;
                                            } else {
                                                'b_1fd9e4: {
                                                    c.cycles += 2;
                                                    let t = c.g[S5][0] != c.g[V0][0];
                                                    c.g[V0][0] = 0x6;
                                                    if t {
                                                        break 'b_1fd9e4;
                                                    } else {
                                                        c.cycles += 6;
                                                        c.g[A0][0] = c
                                                            .lw((c.g[SP][0] as u32)
                                                                .wrapping_add(0x8c4));
                                                        c.g[A1][0] = 0u64;
                                                        c.g[RA][0] = sx(0x001fd9dc);
                                                        c.g[A2][0] = 0x2;
                                                        call_ptr(c, 0x00157b08, 0x001fd9dc)?;
                                                        let t = c.g[V0][0] == 0u64;
                                                        c.g[V0][0] = 0x6;
                                                        if t {
                                                            break 'b_1fda04;
                                                        } else {
                                                            break 'b_1fd9e4;
                                                        }
                                                    }
                                                }
                                                c.cycles += 2;
                                                let t = c.g[S5][0] != c.g[V0][0];
                                                c.g[A0][0] = 0x470000;
                                                if t {
                                                    break 'b_1fda10;
                                                } else {
                                                    c.cycles += 6;
                                                    c.g[A0][0] =
                                                        c.lw(
                                                            (c.g[SP][0] as u32).wrapping_add(0x8c4)
                                                        );
                                                    c.g[A1][0] = 0u64;
                                                    c.g[RA][0] = sx(0x001fd9fc);
                                                    c.g[A2][0] = 0x1;
                                                    call_ptr(c, 0x00157b08, 0x001fd9fc)?;
                                                    if c.g[V0][0] != 0u64 {
                                                        c.g[A0][0] = 0x470000;
                                                        break 'b_1fda10;
                                                    } else {
                                                        break 'b_1fda04;
                                                    }
                                                }
                                            }
                                        }
                                        c.cycles += 3;
                                        c.g[A0][0] = 0x470000;
                                        let t = true;
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x30));
                                        if t {
                                            break 'b_1fdb10;
                                        } else {
                                            break 'b_1fda10;
                                        }
                                    }
                                    c.cycles += 2;
                                    let t = true;
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x48));
                                    if t {
                                        break 'b_1fdb10;
                                    } else {
                                        break 'b_1fda18;
                                    }
                                }
                                'b_1fdb08: {
                                    c.cycles += 44;
                                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                    c.g[A0][0] = 0x470000;
                                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8c8));
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x16b0));
                                    c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                                    c.g[S3][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x7c));
                                    c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                                    c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                                    c.g[RA][0] = sx(0x001fda40);
                                    c.g[S0][0] =
                                        sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                    call_ptr(c, 0x00317670, 0x001fda40)?;
                                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                                    c.g[A0][0] = c.g[S0][0];
                                    let t = c.g[V1][0] as u32;
                                    c.g[RA][0] = sx(0x001fda50);
                                    c.g[A1][0] = c.g[V0][0];
                                    call_ptr(c, t, 0x001fda50)?;
                                    c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x290));
                                    c.g[A1][0] = 0x4a0000;
                                    c.g[S0][0] = c.g[V0][0];
                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2558));
                                    c.g[RA][0] = sx(0x001fda68);
                                    c.g[A0][0] = c.g[S1][0];
                                    call_ptr(c, 0x002c2540, 0x001fda68)?;
                                    c.g[S2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x620));
                                    c.g[A2][0] = c.g[S0][0];
                                    c.g[A3][0] = c.g[S3][0];
                                    c.g[A1][0] = c.g[S1][0];
                                    c.g[RA][0] = sx(0x001fda80);
                                    c.g[A0][0] = c.g[S2][0];
                                    call_ptr(c, 0x002c26d0, 0x001fda80)?;
                                    c.g[A0][0] = 0x470000;
                                    c.g[RA][0] = sx(0x001fda8c);
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x30));
                                    call_ptr(c, 0x00317670, 0x001fda8c)?;
                                    c.g[A0][0] = c.lw((c.g[S6][0] as u32).wrapping_add(0x40));
                                    c.g[RA][0] = sx(0x001fda98);
                                    c.g[A1][0] = c.g[V0][0];
                                    call_ptr(c, 0x0039d860, 0x001fda98)?;
                                    c.g[A0][0] = c.g[V0][0];
                                    c.g[RA][0] = sx(0x001fdaa4);
                                    c.g[A1][0] = c.g[S2][0];
                                    call_ptr(c, 0x003a0d00, 0x001fdaa4)?;
                                    c.g[A0][0] = 0x470000;
                                    c.g[RA][0] = sx(0x001fdab0);
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x68));
                                    call_ptr(c, 0x00317670, 0x001fdab0)?;
                                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8cc));
                                    c.g[RA][0] = sx(0x001fdabc);
                                    c.g[A1][0] = c.g[V0][0];
                                    call_ptr(c, 0x003a1000, 0x001fdabc)?;
                                    c.g[V0][0] = 0xb;
                                    let t = c.g[S5][0] == c.g[V0][0];
                                    c.g[V0][0] = 0xa;
                                    if t {
                                        break 'b_1fdb08;
                                    } else {
                                        'b_1fdae8: {
                                            c.cycles += 2;
                                            let t = c.g[S5][0] != c.g[V0][0];
                                            c.g[V0][0] = 0x9;
                                            if t {
                                                break 'b_1fdae8;
                                            } else {
                                                c.cycles += 6;
                                                c.g[A0][0] =
                                                    c.lw((c.g[SP][0] as u32).wrapping_add(0x8c4));
                                                c.g[A1][0] = 0u64;
                                                c.g[RA][0] = sx(0x001fdae0);
                                                c.g[A2][0] = 0x2;
                                                call_ptr(c, 0x00157b08, 0x001fdae0)?;
                                                let t = c.g[V0][0] == 0u64;
                                                c.g[V0][0] = 0x9;
                                                if t {
                                                    break 'b_1fdb08;
                                                } else {
                                                    break 'b_1fdae8;
                                                }
                                            }
                                        }
                                        c.cycles += 2;
                                        let t = c.g[S5][0] != c.g[V0][0];
                                        c.g[A0][0] = 0x470000;
                                        if t {
                                            break 'b_1fdb2c;
                                        } else {
                                            c.cycles += 6;
                                            c.g[A0][0] =
                                                c.lw((c.g[SP][0] as u32).wrapping_add(0x8c4));
                                            c.g[A1][0] = 0u64;
                                            c.g[RA][0] = sx(0x001fdb00);
                                            c.g[A2][0] = 0x1;
                                            call_ptr(c, 0x00157b08, 0x001fdb00)?;
                                            let t = c.g[V0][0] != 0u64;
                                            c.g[A0][0] = 0x470000;
                                            if t {
                                                break 'b_1fdb2c;
                                            } else {
                                                break 'b_1fdb08;
                                            }
                                        }
                                    }
                                }
                                c.cycles += 2;
                                c.g[A0][0] = 0x470000;
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x80));
                                break 'b_1fdb10;
                            }
                            c.cycles += 7;
                            c.g[RA][0] = sx(0x001fdb18);
                            call_ptr(c, 0x00317670, 0x001fdb18)?;
                            c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8d0));
                            c.g[RA][0] = sx(0x001fdb24);
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x003a1000, 0x001fdb24)?;
                            let t = true;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x970));
                            if t {
                                break 'b_1fdb44;
                            } else {
                                break 'b_1fdb2c;
                            }
                        }
                        c.cycles += 5;
                        c.g[RA][0] = sx(0x001fdb34);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x98));
                        call_ptr(c, 0x00317670, 0x001fdb34)?;
                        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8d0));
                        c.g[RA][0] = sx(0x001fdb40);
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x003a1000, 0x001fdb40)?;
                        break 'b_1fdb40;
                    }
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x970));
            break 'b_1fdb44;
        }
        c.cycles += 12;
        c.g[V0][0] = 0u64;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x960));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x950));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x940));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x930));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x920));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x910));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x900));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x8f0));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x8e0));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x980));
        c.pc = t;
        return Ok(());
    });
}

/// `OVScreenSaveReplay_v26` (0x001fdf40): the normal entry, converted from the generated body
/// (a `match pc` / labelled-block body: every register, cycle and call kept).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn ov_screen_save_replay_v26(c: &mut Cpu) {
    if c.pc != 0x001fdf40 {
        return ee::s05::f_001fdf40__generated(c);
    }
    guest(c, |c| {
        'b_1fdff0: {
            c.cycles += 23;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x720));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x6e0));
            c.g[S3][0] = c.g[A0][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x6f0));
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x710));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x700));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x6d0));
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x6c0));
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x6b0));
            c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x6a0));
            c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x690));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x680), c.g[RA][0]);
            c.g[RA][0] = sx(0x001fdf7c);
            Callee {
                addr: 0x00227f80,
                f: ee::s05::MCManager_Get,
            }
            .call(c, 0x001fdf7c)?;
            c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
            c.g[S2][0] = c.g[V0][0];
            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x150));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x154));
            let t = c.g[V0][0] as u32;
            c.g[RA][0] = sx(0x001fdf94);
            c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, t, 0x001fdf94)?;
            if c.g[V0][0] == 0u64 {
                c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
                break 'b_1fdff0;
            } else {
                'b_1fdfcc: {
                    c.cycles += 8;
                    c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x434));
                    c.g[V1][0] = c.lw(c.g[A1][0] as u32);
                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x1a8));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1ac));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x001fdfb4);
                    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x001fdfb4)?;
                    if c.g[V0][0] != 0u64 {
                        c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x214));
                        break 'b_1fdfcc;
                    } else {
                        c.cycles += 4;
                        c.g[RA][0] = sx(0x001fdfc4);
                        call_ptr(c, 0x001d8240, 0x001fdfc4)?;
                        let t = true;
                        c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
                        if t {
                            break 'b_1fdff0;
                        } else {
                            break 'b_1fdfcc;
                        }
                    }
                }
                'b_1fdfec: {
                    c.cycles += 3;
                    c.g[V0][0] = 0x3;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[A1][0] = 0x3;
                    if t {
                        break 'b_1fdfec;
                    } else {
                        c.cycles += 5;
                        c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x118));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x11c));
                        let t = c.g[V1][0] as u32;
                        c.g[RA][0] = sx(0x001fdfec);
                        c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, t, 0x001fdfec)?;
                        break 'b_1fdfec;
                    }
                }
                c.cycles += 1;
                c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
                break 'b_1fdff0;
            }
        }
        'b_1fe3f8: {
            'b_1fe3f4: {
                c.cycles += 6;
                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x150));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x154));
                let t = c.g[V0][0] as u32;
                c.g[RA][0] = sx(0x001fe000);
                c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x001fe000)?;
                let t = c.g[V0][0] == 0u64;
                c.g[S7][0] = 0u64;
                if t {
                    break 'b_1fe3f4;
                } else {
                    c.cycles += 60;
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
                    c.g[S5][0] = 0x1;
                    c.g[FP][0] = 0x470000;
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x158));
                    c.g[S6][0] = sx((c.g[S3][0] as u32).wrapping_add(0x230));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x15c));
                    c.g[S4][0] = 0u64;
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x001fe02c);
                    c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x001fe02c)?;
                    c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x428));
                    c.g[A2][0] = c.g[SP][0];
                    c.g[A3][0] = 0x320;
                    c.g[RA][0] = sx(0x001fe040);
                    c.g[A0][0] = c.g[S2][0];
                    call_ptr(c, 0x00241710, 0x001fe040)?;
                    c.g[A0][0] = 0x470000;
                    c.g[RA][0] = sx(0x001fe04c);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x130));
                    call_ptr(c, 0x00317670, 0x001fe04c)?;
                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                    c.g[RA][0] = sx(0x001fe058);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x001fe058)?;
                    c.g[A1][0] = c.g[SP][0];
                    c.g[RA][0] = sx(0x001fe064);
                    c.g[A0][0] = c.g[V0][0];
                    call_ptr(c, 0x003a0d00, 0x001fe064)?;
                    c.g[A0][0] = 0x470000;
                    c.g[RA][0] = sx(0x001fe070);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xf0));
                    call_ptr(c, 0x00317670, 0x001fe070)?;
                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                    c.g[RA][0] = sx(0x001fe07c);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x001fe07c)?;
                    c.g[A2][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x434));
                    c.g[S0][0] = c.g[V0][0];
                    c.g[A1][0] = c.g[SP][0];
                    c.g[V0][0] = c.lw(c.g[A2][0] as u32);
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x200));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x204));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x001fe09c);
                    c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x001fe09c)?;
                    c.g[A1][0] = c.g[SP][0];
                    c.g[RA][0] = sx(0x001fe0a8);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x003a0e90, 0x001fe0a8)?;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                    c.g[A1][0] = 0u64;
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x001fe0c0);
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x001fe0c0)?;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.g[A0][0] = 0x470000;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x158));
                    c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                    c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                    c.g[RA][0] = sx(0x001fe0e0);
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    call_ptr(c, 0x00317670, 0x001fe0e0)?;
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                    c.g[A0][0] = c.g[S0][0];
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x001fe0f0);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, t, 0x001fe0f0)?;
                    c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x640));
                    'l_1fe0f8: loop {
                        'b_1fe2e8: {
                            'b_1fe2b4: {
                                'b_1fe2a4: {
                                    'b_1fe21c: {
                                        c.cycles += 30;
                                        c.g[A2][0] = c.g[S5][0];
                                        c.g[A1][0] = sx((c.g[FP][0] as u32).wrapping_add(0x148));
                                        c.g[RA][0] = sx(0x001fe108);
                                        c.g[A0][0] = c.g[S1][0];
                                        call_ptr(c, 0x00417828, 0x001fe108)?;
                                        c.g[RA][0] = sx(0x001fe110);
                                        c.g[A0][0] = c.g[S1][0];
                                        call_ptr(c, 0x00317670, 0x001fe110)?;
                                        c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                                        c.g[RA][0] = sx(0x001fe11c);
                                        c.g[A1][0] = c.g[V0][0];
                                        call_ptr(c, 0x0039d860, 0x001fe11c)?;
                                        c.g[A3][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x434));
                                        c.g[S0][0] = c.g[V0][0];
                                        c.g[A2][0] = c.g[S6][0];
                                        c.g[A1][0] = c.g[S4][0];
                                        c.g[V0][0] = c.lw(c.g[A3][0] as u32);
                                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x70));
                                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x74));
                                        let t = c.g[V1][0] as u32;
                                        c.g[RA][0] = sx(0x001fe140);
                                        c.g[A0][0] =
                                            sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                        call_ptr(c, t, 0x001fe140)?;
                                        c.g[A0][0] = c.g[S0][0];
                                        c.g[RA][0] = sx(0x001fe14c);
                                        c.g[A1][0] = c.g[S6][0];
                                        call_ptr(c, 0x003a0e90, 0x001fe14c)?;
                                        c.g[A2][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x434));
                                        c.g[A1][0] = c.g[S4][0];
                                        c.g[V1][0] = c.lw(c.g[A2][0] as u32);
                                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x218));
                                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x21c));
                                        let t = c.g[V0][0] as u32;
                                        c.g[RA][0] = sx(0x001fe168);
                                        c.g[A0][0] =
                                            sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                        call_ptr(c, t, 0x001fe168)?;
                                        let t = c.g[V0][0] != 0u64;
                                        c.g[A2][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x434));
                                        if t {
                                            break 'b_1fe21c;
                                        } else {
                                            c.cycles += 8;
                                            c.g[A1][0] = c.g[S4][0];
                                            c.g[V1][0] = c.lw(c.g[A2][0] as u32);
                                            c.g[A0][0] =
                                                c.lh((c.g[V1][0] as u32).wrapping_add(0x210));
                                            c.g[V0][0] =
                                                c.lw((c.g[V1][0] as u32).wrapping_add(0x214));
                                            let t = c.g[V0][0] as u32;
                                            c.g[RA][0] = sx(0x001fe188);
                                            c.g[A0][0] =
                                                sx((c.g[A2][0] as u32)
                                                    .wrapping_add(c.g[A0][0] as u32));
                                            call_ptr(c, t, 0x001fe188)?;
                                            if c.g[V0][0] != 0u64 {
                                                c.g[A2][0] =
                                                    c.lw((c.g[S2][0] as u32).wrapping_add(0x434));
                                                break 'b_1fe21c;
                                            } else {
                                                c.cycles += 8;
                                                c.g[A1][0] =
                                                    c.lw((c.g[S2][0] as u32).wrapping_add(0x434));
                                                c.g[V1][0] = c.lw(c.g[A1][0] as u32);
                                                c.g[A0][0] =
                                                    c.lh((c.g[V1][0] as u32).wrapping_add(0x1b8));
                                                c.g[V0][0] =
                                                    c.lw((c.g[V1][0] as u32).wrapping_add(0x1bc));
                                                let t = c.g[V0][0] as u32;
                                                c.g[RA][0] = sx(0x001fe1a8);
                                                c.g[A0][0] = sx((c.g[A1][0] as u32)
                                                    .wrapping_add(c.g[A0][0] as u32));
                                                call_ptr(c, t, 0x001fe1a8)?;
                                                if c.g[V0][0] == 0u64 {
                                                    c.g[A2][0] =
                                                        c.lw(
                                                            (c.g[S2][0] as u32).wrapping_add(0x434)
                                                        );
                                                    break 'b_1fe21c;
                                                } else {
                                                    'b_1fe1f4: {
                                                        c.cycles += 2;
                                                        if c.g[S7][0] != 0u64 {
                                                            c.g[V0][0] = c.lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x8));
                                                            break 'b_1fe1f4;
                                                        } else {
                                                            c.cycles += 15;
                                                            c.g[V1][0] = c.lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x8));
                                                            c.g[A1][0] = 0u64;
                                                            c.g[S7][0] = 0x1;
                                                            c.g[A0][0] = c.lh((c.g[V1][0] as u32)
                                                                .wrapping_add(0xc8));
                                                            c.g[V0][0] = c.lw((c.g[V1][0] as u32)
                                                                .wrapping_add(0xcc));
                                                            let t = c.g[V0][0] as u32;
                                                            c.g[RA][0] = sx(0x001fe1d4);
                                                            c.g[A0][0] = sx((c.g[S0][0] as u32)
                                                                .wrapping_add(c.g[A0][0] as u32));
                                                            call_ptr(c, t, 0x001fe1d4)?;
                                                            c.g[V1][0] = c.lw((c.g[S0][0] as u32)
                                                                .wrapping_add(0x8));
                                                            c.g[A1][0] = 0u64;
                                                            c.g[A0][0] = c.lh((c.g[V1][0] as u32)
                                                                .wrapping_add(0x40));
                                                            c.g[V0][0] = c.lw((c.g[V1][0] as u32)
                                                                .wrapping_add(0x44));
                                                            let t = c.g[V0][0] as u32;
                                                            c.g[RA][0] = sx(0x001fe1ec);
                                                            c.g[A0][0] = sx((c.g[S0][0] as u32)
                                                                .wrapping_add(c.g[A0][0] as u32));
                                                            call_ptr(c, t, 0x001fe1ec)?;
                                                            let t = true;
                                                            c.g[S5][0] = sx((c.g[S5][0] as u32)
                                                                .wrapping_add(0x1));
                                                            if t {
                                                                break 'b_1fe2e8;
                                                            } else {
                                                                break 'b_1fe1f4;
                                                            }
                                                        }
                                                    }
                                                    c.cycles += 10;
                                                    c.g[A1][0] = 0x1;
                                                    c.g[A0][0] = c
                                                        .lh((c.g[V0][0] as u32).wrapping_add(0xc8));
                                                    c.g[V1][0] = c
                                                        .lw((c.g[V0][0] as u32).wrapping_add(0xcc));
                                                    let t = c.g[V1][0] as u32;
                                                    c.g[RA][0] = sx(0x001fe208);
                                                    c.g[A0][0] = sx((c.g[S0][0] as u32)
                                                        .wrapping_add(c.g[A0][0] as u32));
                                                    call_ptr(c, t, 0x001fe208)?;
                                                    c.g[V0][0] =
                                                        c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                                    c.g[A1][0] = 0x1;
                                                    c.g[A0][0] = c
                                                        .lh((c.g[V0][0] as u32).wrapping_add(0x40));
                                                    let t = true;
                                                    c.g[V1][0] = c
                                                        .lw((c.g[V0][0] as u32).wrapping_add(0x44));
                                                    if t {
                                                        break 'b_1fe2a4;
                                                    } else {
                                                        break 'b_1fe21c;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 8;
                                    c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x428));
                                    c.g[V1][0] = c.lw(c.g[A2][0] as u32);
                                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x1b0));
                                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1b4));
                                    let t = c.g[V0][0] as u32;
                                    c.g[RA][0] = sx(0x001fe234);
                                    c.g[A0][0] =
                                        sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    call_ptr(c, t, 0x001fe234)?;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[A1][0] = c.g[S4][0];
                                    if t {
                                        break 'b_1fe2b4;
                                    } else {
                                        'b_1fe29c: {
                                            'b_1fe27c: {
                                                c.cycles += 8;
                                                c.g[A2][0] =
                                                    c.lw((c.g[S2][0] as u32).wrapping_add(0x434));
                                                c.g[V1][0] = c.lw(c.g[A2][0] as u32);
                                                c.g[A0][0] =
                                                    c.lh((c.g[V1][0] as u32).wrapping_add(0x210));
                                                c.g[V0][0] =
                                                    c.lw((c.g[V1][0] as u32).wrapping_add(0x214));
                                                let t = c.g[V0][0] as u32;
                                                c.g[RA][0] = sx(0x001fe254);
                                                c.g[A0][0] = sx((c.g[A2][0] as u32)
                                                    .wrapping_add(c.g[A0][0] as u32));
                                                call_ptr(c, t, 0x001fe254)?;
                                                let t = c.g[V0][0] == 0u64;
                                                c.g[A1][0] = 0x1;
                                                if t {
                                                    break 'b_1fe27c;
                                                } else {
                                                    c.cycles += 8;
                                                    c.g[V0][0] =
                                                        c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                                    c.g[A0][0] = c
                                                        .lh((c.g[V0][0] as u32).wrapping_add(0x40));
                                                    c.g[V1][0] = c
                                                        .lw((c.g[V0][0] as u32).wrapping_add(0x44));
                                                    let t = c.g[V1][0] as u32;
                                                    c.g[RA][0] = sx(0x001fe270);
                                                    c.g[A0][0] = sx((c.g[S0][0] as u32)
                                                        .wrapping_add(c.g[A0][0] as u32));
                                                    call_ptr(c, t, 0x001fe270)?;
                                                    c.g[V0][0] =
                                                        c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                                    let t = true;
                                                    c.g[A1][0] = 0x1;
                                                    if t {
                                                        break 'b_1fe29c;
                                                    } else {
                                                        break 'b_1fe27c;
                                                    }
                                                }
                                            }
                                            c.cycles += 8;
                                            c.g[V0][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                            c.g[A1][0] = 0u64;
                                            c.g[A0][0] =
                                                c.lh((c.g[V0][0] as u32).wrapping_add(0x40));
                                            c.g[V1][0] =
                                                c.lw((c.g[V0][0] as u32).wrapping_add(0x44));
                                            let t = c.g[V1][0] as u32;
                                            c.g[RA][0] = sx(0x001fe294);
                                            c.g[A0][0] =
                                                sx((c.g[S0][0] as u32)
                                                    .wrapping_add(c.g[A0][0] as u32));
                                            call_ptr(c, t, 0x001fe294)?;
                                            c.g[V0][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                            c.g[A1][0] = 0u64;
                                            break 'b_1fe29c;
                                        }
                                        c.cycles += 2;
                                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0xc8));
                                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xcc));
                                        break 'b_1fe2a4;
                                    }
                                }
                                c.cycles += 4;
                                let t = c.g[V1][0] as u32;
                                c.g[RA][0] = sx(0x001fe2ac);
                                c.g[A0][0] =
                                    sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                call_ptr(c, t, 0x001fe2ac)?;
                                let t = true;
                                c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_add(0x1));
                                if t {
                                    break 'b_1fe2e8;
                                } else {
                                    break 'b_1fe2b4;
                                }
                            }
                            c.cycles += 13;
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                            c.g[A1][0] = 0x1;
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0xc8));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xcc));
                            let t = c.g[V1][0] as u32;
                            c.g[RA][0] = sx(0x001fe2cc);
                            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x001fe2cc)?;
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                            c.g[A1][0] = 0x1;
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x40));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x44));
                            let t = c.g[V1][0] as u32;
                            c.g[RA][0] = sx(0x001fe2e4);
                            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x001fe2e4)?;
                            c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_add(0x1));
                            break 'b_1fe2e8;
                        }
                        c.cycles += 3;
                        c.g[V0][0] = ((c.g[S5][0] as i64) < 5i64) as u64;
                        let t = c.g[V0][0] != 0u64;
                        c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_add(0x1));
                        if t {
                            if event_due(c) {
                                deliver_event_here(c, 0x001fe0f8);
                            }
                            continue 'l_1fe0f8;
                        } else {
                            'b_1fe378: {
                                'b_1fe348: {
                                    c.cycles += 8;
                                    c.g[A2][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x428));
                                    c.g[A0][0] = c.g[S3][0];
                                    c.g[A1][0] = c.g[S2][0];
                                    c.g[RA][0] = sx(0x001fe308);
                                    c.g[A3][0] = 0x1;
                                    call_ptr(c, 0x001d97e0, 0x001fe308)?;
                                    c.g[V1][0] = 0xffffffffffffffff;
                                    let t = c.g[V0][0] != c.g[V1][0];
                                    c.g[V0][0] = 0x470000;
                                    if t {
                                        break 'b_1fe348;
                                    } else {
                                        c.cycles += 13;
                                        c.g[RA][0] = sx(0x001fe31c);
                                        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x168));
                                        call_ptr(c, 0x00317670, 0x001fe31c)?;
                                        c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                                        c.g[RA][0] = sx(0x001fe328);
                                        c.g[A1][0] = c.g[V0][0];
                                        call_ptr(c, 0x0039d860, 0x001fe328)?;
                                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
                                        c.g[A1][0] = 0u64;
                                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x48));
                                        c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4c));
                                        let t = c.g[A2][0] as u32;
                                        c.g[RA][0] = sx(0x001fe340);
                                        c.g[A0][0] =
                                            sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                        call_ptr(c, t, 0x001fe340)?;
                                        let t = true;
                                        c.g[A1][0] = c.g[S2][0];
                                        if t {
                                            break 'b_1fe378;
                                        } else {
                                            break 'b_1fe348;
                                        }
                                    }
                                }
                                c.cycles += 12;
                                c.g[RA][0] = sx(0x001fe350);
                                c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x168));
                                call_ptr(c, 0x00317670, 0x001fe350)?;
                                c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                                c.g[RA][0] = sx(0x001fe35c);
                                c.g[A1][0] = c.g[V0][0];
                                call_ptr(c, 0x0039d860, 0x001fe35c)?;
                                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
                                c.g[A1][0] = 0x1;
                                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x48));
                                c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4c));
                                let t = c.g[A2][0] as u32;
                                c.g[RA][0] = sx(0x001fe374);
                                c.g[A0][0] =
                                    sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                call_ptr(c, t, 0x001fe374)?;
                                c.g[A1][0] = c.g[S2][0];
                                break 'b_1fe378;
                            }
                            'b_1fe3c8: {
                                c.cycles += 7;
                                c.g[A0][0] = c.g[S3][0];
                                c.g[A2][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x428));
                                c.g[RA][0] = sx(0x001fe388);
                                c.g[A3][0] = 0u64;
                                call_ptr(c, 0x001d97e0, 0x001fe388)?;
                                c.g[V1][0] = 0xffffffffffffffff;
                                let t = c.g[V0][0] != c.g[V1][0];
                                c.g[V0][0] = 0x470000;
                                if t {
                                    break 'b_1fe3c8;
                                } else {
                                    c.cycles += 13;
                                    c.g[RA][0] = sx(0x001fe39c);
                                    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x178));
                                    call_ptr(c, 0x00317670, 0x001fe39c)?;
                                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                                    c.g[RA][0] = sx(0x001fe3a8);
                                    c.g[A1][0] = c.g[V0][0];
                                    call_ptr(c, 0x0039d860, 0x001fe3a8)?;
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
                                    c.g[A1][0] = 0u64;
                                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x48));
                                    c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4c));
                                    let t = c.g[A2][0] as u32;
                                    c.g[RA][0] = sx(0x001fe3c0);
                                    c.g[A0][0] =
                                        sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    call_ptr(c, t, 0x001fe3c0)?;
                                    let t = true;
                                    c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x214));
                                    if t {
                                        break 'b_1fe3f8;
                                    } else {
                                        break 'b_1fe3c8;
                                    }
                                }
                            }
                            c.cycles += 11;
                            c.g[RA][0] = sx(0x001fe3d0);
                            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x178));
                            call_ptr(c, 0x00317670, 0x001fe3d0)?;
                            c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                            c.g[RA][0] = sx(0x001fe3dc);
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x0039d860, 0x001fe3dc)?;
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
                            c.g[A1][0] = 0x1;
                            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x48));
                            c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4c));
                            let t = c.g[A2][0] as u32;
                            c.g[RA][0] = sx(0x001fe3f4);
                            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x001fe3f4)?;
                            break 'b_1fe3f4;
                        }
                    }
                }
            }
            c.cycles += 1;
            c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x214));
            break 'b_1fe3f8;
        }
        'b_1fe474: {
            c.cycles += 3;
            c.g[V0][0] = 0x3;
            let t = c.g[V1][0] == c.g[V0][0];
            c.g[S1][0] = 0x1;
            if t {
                break 'b_1fe474;
            } else {
                c.cycles += 3;
                c.g[FP][0] = 0x470000;
                c.g[S2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x640));
                c.g[A2][0] = c.g[S1][0];
                'l_1fe410: loop {
                    c.cycles += 25;
                    c.g[A1][0] = sx((c.g[FP][0] as u32).wrapping_add(0x148));
                    c.g[A0][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x001fe420);
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                    call_ptr(c, 0x00417828, 0x001fe420)?;
                    c.g[RA][0] = sx(0x001fe428);
                    c.g[A0][0] = c.g[S2][0];
                    call_ptr(c, 0x00317670, 0x001fe428)?;
                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                    c.g[RA][0] = sx(0x001fe434);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x001fe434)?;
                    c.g[S0][0] = c.g[V0][0];
                    c.g[A1][0] = 0u64;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0xc8));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xcc));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x001fe450);
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x001fe450)?;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                    c.g[A1][0] = 0x1;
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x40));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x44));
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x001fe468);
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x001fe468)?;
                    c.g[V0][0] = ((c.g[S1][0] as i64) < 5i64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[A2][0] = c.g[S1][0];
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x001fe410);
                        }
                        continue 'l_1fe410;
                    } else {
                        break 'b_1fe474;
                    }
                }
            }
        }
        c.cycles += 12;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x710));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x700));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x6f0));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x6e0));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x6d0));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x6c0));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x6b0));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x6a0));
        c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x690));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x680));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x720));
        c.pc = t;
        return Ok(());
    });
}

/// `void OVScreenInGameError_Init(OVScreen* this)` (0x001fab18): builds the
/// in-game error pop-up: adds the screen, hides the cancel/continue widgets,
/// picks the error text by the cheat state of `cGameComm` (`gp-0x204`),
/// and, with a game comm, pings the server and sets the cancel button.
pub fn ingame_error_init(c: &mut Cpu) {
    guest(c, |c| {
        'b_1fab68: {
            c.cycles += 16;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x30));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[S1][0] = c.g[A0][0];
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.g[A0][0] = 0x470000;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x630));
            c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
            call_ptr(c, 0x00317670, 0x001fab3c)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S1][0];
            c.g[A3][0] = 0u64;
            call_ptr(c, 0x00397cc0, 0x001fab50)?;
            let t = c.g[V0][0] == 0u64;
            c.sw((c.g[S1][0] as u32).wrapping_add(0x40), c.g[V0][0]);
            if t {
                break 'b_1fab68;
            } else {
                c.cycles += 4;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0u64;
                c.g[A2][0] = 0u64;
                call_ptr(c, 0x0039c870, 0x001fab68)?;
                break 'b_1fab68;
            }
        }
        'b_1fabb0: {
            c.cycles += 11;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0020a380, 0x001fab70)?;
            c.g[A0][0] = 0x470000;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x620));
            call_ptr(c, 0x00317670, 0x001fab7c)?;
            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039d860, 0x001fab88)?;
            c.g[S0][0] = c.g[V0][0];
            if c.g[S0][0] == 0u64 {
                c.g[A0][0] = 0x470000;
                break 'b_1fabb0;
            } else {
                c.cycles += 7;
                c.g[A0][0] = 0x470000;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x610));
                call_ptr(c, 0x00317670, 0x001faba0)?;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x003a1000, 0x001fabac)?;
                c.g[A0][0] = 0x470000;
                break 'b_1fabb0;
            }
        }
        'b_1fae78: {
            c.cycles += 8;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x5f8));
            call_ptr(c, 0x00317670, 0x001fabb8)?;
            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039d860, 0x001fabc4)?;
            c.g[S0][0] = c.g[V0][0];
            if c.g[S0][0] == 0u64 {
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x204));
                break 'b_1fae78;
            } else {
                'b_1fad0c: {
                    'b_1fad08: {
                        'b_1facf0: {
                            'b_1facd4: {
                                'b_1fac48: {
                                    c.cycles += 13;
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                    c.g[A1][0] = 0x1;
                                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                    let t = c.g[V1][0] as u32;
                                    c.g[A0][0] =
                                        sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    call_ptr(c, t, 0x001fabe8)?;
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x14));
                                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x204));
                                    c.g[V0][0] = c.g[V0][0] | 0x80;
                                    c.sw((c.g[S0][0] as u32).wrapping_add(0x14), c.g[V0][0]);
                                    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x24));
                                    let t = c.g[V1][0] == 0u64;
                                    c.g[A1][0] = 0x1;
                                    if t {
                                        break 'b_1fac48;
                                    } else {
                                        'b_1fac18: {
                                            c.cycles += 2;
                                            let t = c.g[V1][0] != c.g[A1][0];
                                            c.g[V0][0] = 0x2;
                                            if t {
                                                break 'b_1fac18;
                                            } else {
                                                c.cycles += 3;
                                                c.g[A0][0] = 0x470000;
                                                let t = true;
                                                c.g[A0][0] =
                                                    sx((c.g[A0][0] as u32).wrapping_sub(0x5e8));
                                                if t {
                                                    break 'b_1facd4;
                                                } else {
                                                    break 'b_1fac18;
                                                }
                                            }
                                        }
                                        'b_1fac2c: {
                                            c.cycles += 2;
                                            let t = c.g[V1][0] != c.g[V0][0];
                                            c.g[V0][0] = 0x3;
                                            if t {
                                                break 'b_1fac2c;
                                            } else {
                                                c.cycles += 3;
                                                c.g[A0][0] = 0x470000;
                                                let t = true;
                                                c.g[A0][0] =
                                                    sx((c.g[A0][0] as u32).wrapping_sub(0x5e8));
                                                if t {
                                                    break 'b_1facd4;
                                                } else {
                                                    break 'b_1fac2c;
                                                }
                                            }
                                        }
                                        'b_1fac40: {
                                            c.cycles += 2;
                                            if c.g[V1][0] != c.g[V0][0] {
                                                c.g[A0][0] = 0x470000;
                                                break 'b_1fac40;
                                            } else {
                                                c.cycles += 3;
                                                c.g[A0][0] = 0x470000;
                                                let t = true;
                                                c.g[A0][0] =
                                                    sx((c.g[A0][0] as u32).wrapping_sub(0x5e8));
                                                if t {
                                                    break 'b_1facd4;
                                                } else {
                                                    break 'b_1fac40;
                                                }
                                            }
                                        }
                                        c.cycles += 2;
                                        let t = true;
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x5c8));
                                        if t {
                                            break 'b_1facd4;
                                        } else {
                                            break 'b_1fac48;
                                        }
                                    }
                                }
                                'b_1fac9c: {
                                    c.cycles += 3;
                                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x28));
                                    if c.g[V0][0] == 0u64 {
                                        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8c));
                                        break 'b_1fac9c;
                                    } else {
                                        'b_1fac70: {
                                            c.cycles += 4;
                                            c.sw((c.g[S1][0] as u32).wrapping_add(0xa0), 0u64);
                                            c.g[V1][0] =
                                                c.lw((c.g[A0][0] as u32).wrapping_add(0x28));
                                            let t = c.g[V1][0] != c.g[A1][0];
                                            c.g[V0][0] = 0x2;
                                            if t {
                                                break 'b_1fac70;
                                            } else {
                                                c.cycles += 3;
                                                c.g[A0][0] = 0x470000;
                                                let t = true;
                                                c.g[A0][0] =
                                                    sx((c.g[A0][0] as u32).wrapping_sub(0x5a8));
                                                if t {
                                                    break 'b_1facd4;
                                                } else {
                                                    break 'b_1fac70;
                                                }
                                            }
                                        }
                                        'b_1fac84: {
                                            c.cycles += 2;
                                            let t = c.g[V1][0] != c.g[V0][0];
                                            c.g[V0][0] = 0x3;
                                            if t {
                                                break 'b_1fac84;
                                            } else {
                                                c.cycles += 3;
                                                c.g[A0][0] = 0x470000;
                                                let t = true;
                                                c.g[A0][0] =
                                                    sx((c.g[A0][0] as u32).wrapping_sub(0x5a8));
                                                if t {
                                                    break 'b_1facd4;
                                                } else {
                                                    break 'b_1fac84;
                                                }
                                            }
                                        }
                                        'b_1fac94: {
                                            c.cycles += 2;
                                            let t = c.g[V1][0] != c.g[V0][0];
                                            c.g[A0][0] = 0x470000;
                                            if t {
                                                break 'b_1fac94;
                                            } else {
                                                c.cycles += 2;
                                                let t = true;
                                                c.g[A0][0] =
                                                    sx((c.g[A0][0] as u32).wrapping_sub(0x5a8));
                                                if t {
                                                    break 'b_1facd4;
                                                } else {
                                                    break 'b_1fac94;
                                                }
                                            }
                                        }
                                        c.cycles += 2;
                                        let t = true;
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x5a8));
                                        if t {
                                            break 'b_1facd4;
                                        } else {
                                            break 'b_1fac9c;
                                        }
                                    }
                                }
                                'b_1facc4: {
                                    c.cycles += 2;
                                    if c.g[V0][0] == 0u64 {
                                        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x14));
                                        break 'b_1facc4;
                                    } else {
                                        c.cycles += 8;
                                        c.g[A0][0] = 0x470000;
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x588));
                                        call_ptr(c, 0x00317670, 0x001facb0)?;
                                        c.g[A0][0] = c.g[S0][0];
                                        c.g[A1][0] = c.g[V0][0];
                                        call_ptr(c, 0x003a1000, 0x001facbc)?;
                                        let t = true;
                                        c.sw((c.g[S1][0] as u32).wrapping_add(0xa0), 0u64);
                                        if t {
                                            break 'b_1fad08;
                                        } else {
                                            break 'b_1facc4;
                                        }
                                    }
                                }
                                c.cycles += 2;
                                let t = c.g[V0][0] == 0u64;
                                c.sw((c.g[S1][0] as u32).wrapping_add(0xa0), 0u64);
                                if t {
                                    break 'b_1facf0;
                                } else {
                                    c.cycles += 2;
                                    c.g[A0][0] = 0x470000;
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x588));
                                    break 'b_1facd4;
                                }
                            }
                            c.cycles += 7;
                            call_ptr(c, 0x00317670, 0x001facdc)?;
                            c.g[A0][0] = c.g[S0][0];
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x003a1000, 0x001face8)?;
                            let t = true;
                            c.g[A0][0] = 0x470000;
                            if t {
                                break 'b_1fad0c;
                            } else {
                                break 'b_1facf0;
                            }
                        }
                        c.cycles += 6;
                        c.g[A0][0] = 0x470000;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x5a8));
                        call_ptr(c, 0x00317670, 0x001facfc)?;
                        c.g[A0][0] = c.g[S0][0];
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x003a1000, 0x001fad08)?;
                        break 'b_1fad08;
                    }
                    c.cycles += 1;
                    c.g[A0][0] = 0x470000;
                    break 'b_1fad0c;
                }
                'b_1fad88: {
                    c.cycles += 8;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x570));
                    call_ptr(c, 0x00317670, 0x001fad14)?;
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x001fad20)?;
                    c.g[S0][0] = c.g[V0][0];
                    let t = c.g[S0][0] == 0u64;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x204));
                    if t {
                        break 'b_1fad88;
                    } else {
                        'b_1fad70: {
                            'b_1fad58: {
                                c.cycles += 3;
                                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xa0));
                                if c.g[V1][0] == 0u64 {
                                    c.g[A0][0] = 0x470000;
                                    break 'b_1fad58;
                                } else {
                                    c.cycles += 8;
                                    c.g[A0][0] = 0x470000;
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x560));
                                    call_ptr(c, 0x00317670, 0x001fad44)?;
                                    c.g[A1][0] = c.g[V0][0];
                                    c.g[A0][0] = c.g[S0][0];
                                    call_ptr(c, 0x003a1000, 0x001fad50)?;
                                    let t = true;
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                    if t {
                                        break 'b_1fad70;
                                    } else {
                                        break 'b_1fad58;
                                    }
                                }
                            }
                            c.cycles += 6;
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x550));
                            call_ptr(c, 0x00317670, 0x001fad60)?;
                            c.g[A1][0] = c.g[V0][0];
                            c.g[A0][0] = c.g[S0][0];
                            call_ptr(c, 0x003a1000, 0x001fad6c)?;
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                            break 'b_1fad70;
                        }
                        c.cycles += 6;
                        c.g[A1][0] = 0u64;
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                        let t = c.g[V1][0] as u32;
                        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, t, 0x001fad84)?;
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x18), 0u64);
                        break 'b_1fad88;
                    }
                }
                'b_1fadc0: {
                    c.cycles += 9;
                    c.g[A0][0] = 0x470000;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x538));
                    call_ptr(c, 0x00317670, 0x001fad94)?;
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x001fada0)?;
                    c.g[S0][0] = c.g[V0][0];
                    let t = c.g[S0][0] == 0u64;
                    c.g[A1][0] = 0u64;
                    if t {
                        break 'b_1fadc0;
                    } else {
                        c.cycles += 5;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                        let t = c.g[V1][0] as u32;
                        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, t, 0x001fadc0)?;
                        break 'b_1fadc0;
                    }
                }
                'b_1fadf8: {
                    c.cycles += 9;
                    c.g[A0][0] = 0x470000;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x528));
                    call_ptr(c, 0x00317670, 0x001fadcc)?;
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x001fadd8)?;
                    c.g[S0][0] = c.g[V0][0];
                    let t = c.g[S0][0] == 0u64;
                    c.g[A1][0] = 0u64;
                    if t {
                        break 'b_1fadf8;
                    } else {
                        c.cycles += 5;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                        let t = c.g[V1][0] as u32;
                        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, t, 0x001fadf8)?;
                        break 'b_1fadf8;
                    }
                }
                'b_1fae30: {
                    c.cycles += 9;
                    c.g[A0][0] = 0x470000;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x518));
                    call_ptr(c, 0x00317670, 0x001fae04)?;
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x001fae10)?;
                    c.g[S0][0] = c.g[V0][0];
                    let t = c.g[S0][0] == 0u64;
                    c.g[A1][0] = 0u64;
                    if t {
                        break 'b_1fae30;
                    } else {
                        c.cycles += 5;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                        let t = c.g[V1][0] as u32;
                        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, t, 0x001fae30)?;
                        break 'b_1fae30;
                    }
                }
                'b_1fae68: {
                    c.cycles += 9;
                    c.g[A0][0] = 0x470000;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x500));
                    call_ptr(c, 0x00317670, 0x001fae3c)?;
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x001fae48)?;
                    c.g[S0][0] = c.g[V0][0];
                    let t = c.g[S0][0] == 0u64;
                    c.g[A1][0] = 0u64;
                    if t {
                        break 'b_1fae68;
                    } else {
                        c.cycles += 5;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                        let t = c.g[V1][0] as u32;
                        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, t, 0x001fae68)?;
                        break 'b_1fae68;
                    }
                }
                c.cycles += 4;
                c.g[A0][0] = c.g[S1][0];
                c.g[A1][0] = 0u64;
                Callee {
                    addr: 0x001fb378,
                    f: ee::s04::OVScreenInGameError_ShowContinueButton,
                }
                .call(c, 0x001fae74)?;
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x204));
                break 'b_1fae78;
            }
        }
        'b_1faef4: {
            'b_1faef0: {
                'b_1faee4: {
                    c.cycles += 2;
                    let t = c.g[A0][0] == 0u64;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                    if t {
                        break 'b_1faee4;
                    } else {
                        c.cycles += 18;
                        c.g[A2][0] = 0x3;
                        c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
                        call_ptr(c, 0x002583a8, 0x001fae8c)?;
                        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2a74));
                        c.g[A1][0] = 0u64;
                        c.g[A0][0] = c.g[S1][0];
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x18));
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x9c), c.g[V0][0]);
                        Callee {
                            addr: 0x001faff8,
                            f: ee::s04::OVScreenInGameError_ShowCancelGame,
                        }
                        .call(c, 0x001faea4)?;
                        c.g[A0][0] = 0x470000;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x4e8));
                        call_ptr(c, 0x00317670, 0x001faeb0)?;
                        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x0039d860, 0x001faebc)?;
                        c.g[S0][0] = c.g[V0][0];
                        let t = c.g[S0][0] == 0u64;
                        c.g[A0][0] = 0x470000;
                        if t {
                            break 'b_1faef0;
                        } else {
                            c.cycles += 7;
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x4d8));
                            call_ptr(c, 0x00317670, 0x001faed0)?;
                            c.g[A0][0] = c.g[S0][0];
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x003a1000, 0x001faedc)?;
                            let t = true;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
                            if t {
                                break 'b_1faef4;
                            } else {
                                break 'b_1faee4;
                            }
                        }
                    }
                }
                c.cycles += 3;
                c.g[A0][0] = c.g[S1][0];
                c.g[A1][0] = 0x1;
                Callee {
                    addr: 0x001faff8,
                    f: ee::s04::OVScreenInGameError_ShowCancelGame,
                }
                .call(c, 0x001faef0)?;
                break 'b_1faef0;
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x20));
            break 'b_1faef4;
        }
        c.cycles += 4;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = t;
        return Ok(());
    });
}

/// `int OVScreenMultiFreestylePre_TransitionIn(OVScreen* this, void* msg)`
/// (0x001fcec0): with a message, shifts the layout groups by game mode
/// (modes 2/3: `screendata`-style group by +130 x; modes 5/6: three groups
/// moved in y). Returns 0.
pub fn mfpre_transition_in(c: &mut Cpu) {
    guest(c, |c| {
        'b_1fd0c4: {
            'b_1fd0c0: {
                c.cycles += 5;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x30), c.g[RA][0]);
                let t = c.g[A1][0] == 0u64;
                c.g[S0][0] = c.g[A0][0];
                if t {
                    break 'b_1fd0c0;
                } else {
                    'b_1fcf70: {
                        c.cycles += 11;
                        call_ptr(c, 0x0014dc80, 0x001fcedc)?;
                        c.g[A0][0] = c.g[V0][0];
                        c.g[A1][0] = 0u64;
                        call_ptr(c, 0x0014dd58, 0x001fcee8)?;
                        c.g[V1][0] = 0x530000;
                        c.g[V1][0] = c.lb((c.g[V1][0] as u32).wrapping_add(0x5c10));
                        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x2));
                        c.g[V0][0] = (c.g[V0][0] < 0x2u64) as u64;
                        let t = c.g[V0][0] == 0u64;
                        c.g[A0][0] = 0x470000;
                        if t {
                            break 'b_1fcf70;
                        } else {
                            c.cycles += 28;
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1778));
                            call_ptr(c, 0x00317670, 0x001fcf08)?;
                            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x0039d860, 0x001fcf14)?;
                            c.ldl(3, (c.g[V0][0] as u32).wrapping_add(0x4b));
                            c.ldr(3, (c.g[V0][0] as u32).wrapping_add(0x44));
                            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                            c.sdl(3, (c.g[SP][0] as u32).wrapping_add(0x7));
                            c.sdr(3, c.g[SP][0] as u32);
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[A0][0]);
                            c.g[AT][0] = 0x43020000;
                            c.mtc1(1, 3);
                            c.lwc1(0, c.g[SP][0] as u32);
                            c.lwc1(2, (c.g[SP][0] as u32).wrapping_add(0x4));
                            c.add_s(0, 0, 3);
                            c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x8));
                            c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x14));
                            c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x18));
                            c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x10));
                            c.ldl(7, (c.g[SP][0] as u32).wrapping_add(0x17));
                            c.ldr(7, (c.g[SP][0] as u32).wrapping_add(0x10));
                            c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x18));
                            c.sdl(7, (c.g[V0][0] as u32).wrapping_add(0x4b));
                            c.sdr(7, (c.g[V0][0] as u32).wrapping_add(0x44));
                            c.sw((c.g[V0][0] as u32).wrapping_add(0x4c), c.g[V1][0]);
                            let t = true;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                            if t {
                                break 'b_1fd0c4;
                            } else {
                                break 'b_1fcf70;
                            }
                        }
                    }
                    c.cycles += 4;
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x5));
                    c.g[V0][0] = (c.g[V0][0] < 0x2u64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A0][0] = 0x4a0000;
                    if t {
                        break 'b_1fd0c0;
                    } else {
                        c.cycles += 80;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2548));
                        call_ptr(c, 0x00317670, 0x001fcf88)?;
                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x0039d860, 0x001fcf94)?;
                        c.ldl(6, (c.g[V0][0] as u32).wrapping_add(0x4b));
                        c.ldr(6, (c.g[V0][0] as u32).wrapping_add(0x44));
                        c.g[A3][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                        c.sdl(6, (c.g[SP][0] as u32).wrapping_add(0x7));
                        c.sdr(6, c.g[SP][0] as u32);
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[A3][0]);
                        c.g[AT][0] = 0x42200000;
                        c.mtc1(1, 3);
                        c.g[A0][0] = 0x470000;
                        c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
                        c.lwc1(2, c.g[SP][0] as u32);
                        c.add_s(0, 0, 3);
                        c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x8));
                        c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x20));
                        c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x28));
                        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x24));
                        c.ldl(5, (c.g[SP][0] as u32).wrapping_add(0x27));
                        c.ldr(5, (c.g[SP][0] as u32).wrapping_add(0x20));
                        c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x28));
                        c.sdl(5, (c.g[V0][0] as u32).wrapping_add(0x4b));
                        c.sdr(5, (c.g[V0][0] as u32).wrapping_add(0x44));
                        c.sw((c.g[V0][0] as u32).wrapping_add(0x4c), c.g[A2][0]);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x150));
                        call_ptr(c, 0x00317670, 0x001fcff4)?;
                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x0039d860, 0x001fd000)?;
                        c.ldl(4, (c.g[V0][0] as u32).wrapping_add(0x4b));
                        c.ldr(4, (c.g[V0][0] as u32).wrapping_add(0x44));
                        c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                        c.sdl(4, (c.g[SP][0] as u32).wrapping_add(0x7));
                        c.sdr(4, c.g[SP][0] as u32);
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[A1][0]);
                        c.g[AT][0] = 0x42860000;
                        c.mtc1(1, 3);
                        c.g[A0][0] = 0x470000;
                        c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
                        c.lwc1(2, c.g[SP][0] as u32);
                        c.add_s(0, 0, 3);
                        c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x8));
                        c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x20));
                        c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x28));
                        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x24));
                        c.ldl(3, (c.g[SP][0] as u32).wrapping_add(0x27));
                        c.ldr(3, (c.g[SP][0] as u32).wrapping_add(0x20));
                        c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x28));
                        c.sdl(3, (c.g[V0][0] as u32).wrapping_add(0x4b));
                        c.sdr(3, (c.g[V0][0] as u32).wrapping_add(0x44));
                        c.sw((c.g[V0][0] as u32).wrapping_add(0x4c), c.g[A1][0]);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x140));
                        call_ptr(c, 0x00317670, 0x001fd060)?;
                        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x0039d860, 0x001fd06c)?;
                        c.ldl(3, (c.g[V0][0] as u32).wrapping_add(0x4b));
                        c.ldr(3, (c.g[V0][0] as u32).wrapping_add(0x44));
                        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                        c.sdl(3, (c.g[SP][0] as u32).wrapping_add(0x7));
                        c.sdr(3, c.g[SP][0] as u32);
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[A0][0]);
                        c.g[AT][0] = 0x42400000;
                        c.mtc1(1, 3);
                        c.lwc1(0, (c.g[SP][0] as u32).wrapping_add(0x4));
                        c.lwc1(2, c.g[SP][0] as u32);
                        c.sub_s(0, 0, 3);
                        c.lwc1(1, (c.g[SP][0] as u32).wrapping_add(0x8));
                        c.swc1(2, (c.g[SP][0] as u32).wrapping_add(0x20));
                        c.swc1(1, (c.g[SP][0] as u32).wrapping_add(0x28));
                        c.swc1(0, (c.g[SP][0] as u32).wrapping_add(0x24));
                        c.ldl(7, (c.g[SP][0] as u32).wrapping_add(0x27));
                        c.ldr(7, (c.g[SP][0] as u32).wrapping_add(0x20));
                        c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x28));
                        c.sdl(7, (c.g[V0][0] as u32).wrapping_add(0x4b));
                        c.sdr(7, (c.g[V0][0] as u32).wrapping_add(0x44));
                        c.sw((c.g[V0][0] as u32).wrapping_add(0x4c), c.g[V1][0]);
                        break 'b_1fd0c0;
                    }
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
            break 'b_1fd0c4;
        }
        c.cycles += 4;
        c.g[V0][0] = 0u64;
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x30));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.pc = t;
        return Ok(());
    });
}

/// `void OVScreenSaveReplay_Init(OVScreen* this)` (0x001fdc30): builds the
/// save-replay screen: sets its texts, labels the four memory-card slots
/// (`sprintf`, slot `i-1` stored at widget+0x18), adds the dark-blue overlay
/// screen, pushes pause state 2 and runs `cOVStateOverlay_Init`.
pub fn savereplay_init(c: &mut Cpu) {
    guest(c, |c| {
        'b_1fddd0: {
            c.cycles += 24;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xa0));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.g[S1][0] = c.g[A0][0];
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.g[A0][0] = 0x470000;
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xe0));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.g[S3][0] = 0x470000;
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.g[S6][0] = 0x530000;
            c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
            c.g[S4][0] = 0x440000;
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
            c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
            call_ptr(c, 0x00317670, 0x001fdc74)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S1][0];
            c.g[A3][0] = 0u64;
            call_ptr(c, 0x00397cc0, 0x001fdc88)?;
            let t = c.g[V0][0] == 0u64;
            c.sw((c.g[S1][0] as u32).wrapping_add(0x40), c.g[V0][0]);
            if t {
                break 'b_1fddd0;
            } else {
                c.cycles += 66;
                c.g[A2][0] = 0u64;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0u64;
                c.g[S2][0] = 0x1;
                call_ptr(c, 0x0039c870, 0x001fdca4)?;
                c.g[S5][0] = 0x470000;
                c.g[A0][0] = 0x470000;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xf0));
                call_ptr(c, 0x00317670, 0x001fdcb4)?;
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039d860, 0x001fdcc0)?;
                c.g[S0][0] = 0x4a0000;
                c.g[A0][0] = c.g[V0][0];
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x2560));
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x003a0c50, 0x001fdcd4)?;
                c.g[A0][0] = 0x470000;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x100));
                call_ptr(c, 0x00317670, 0x001fdce0)?;
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039d860, 0x001fdcec)?;
                c.g[A2][0] = c.g[V0][0];
                c.g[A1][0] = 0x1;
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                let t = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x001fdd08)?;
                c.g[A0][0] = 0x470000;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x118));
                call_ptr(c, 0x00317670, 0x001fdd14)?;
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039d860, 0x001fdd20)?;
                c.g[A2][0] = c.g[V0][0];
                c.g[A1][0] = 0u64;
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                let t = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x001fdd3c)?;
                c.g[A0][0] = 0x470000;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1608));
                call_ptr(c, 0x00317670, 0x001fdd48)?;
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039d860, 0x001fdd54)?;
                c.g[A2][0] = c.g[V0][0];
                c.g[A1][0] = 0u64;
                c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                let t = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x001fdd70)?;
                c.g[A0][0] = 0x470000;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x130));
                call_ptr(c, 0x00317670, 0x001fdd7c)?;
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039d860, 0x001fdd88)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x003a0c50, 0x001fdd94)?;
                c.g[A0][0] = c.g[SP][0];
                'l_1fdd98: loop {
                    c.cycles += 14;
                    c.g[A2][0] = c.g[S2][0];
                    c.g[A1][0] = sx((c.g[S5][0] as u32).wrapping_add(0x148));
                    call_ptr(c, 0x00417828, 0x001fdda4)?;
                    c.g[A0][0] = c.g[SP][0];
                    call_ptr(c, 0x00317670, 0x001fddac)?;
                    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x001fddb8)?;
                    c.g[V1][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x18), c.g[V1][0]);
                    c.g[V0][0] = ((c.g[S2][0] as i64) < 5i64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[A0][0] = c.g[SP][0];
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x001fdd98);
                        }
                        continue 'l_1fdd98;
                    } else {
                        break 'b_1fddd0;
                    }
                }
            }
        }
        'b_1fde04: {
            c.cycles += 10;
            c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_sub(0x1c88));
            c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
            call_ptr(c, 0x00317670, 0x001fdddc)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S1][0];
            c.g[A3][0] = 0u64;
            call_ptr(c, 0x00397cc0, 0x001fddf0)?;
            let t = c.g[V0][0] == 0u64;
            c.g[A0][0] = c.g[V0][0];
            if t {
                break 'b_1fde04;
            } else {
                c.cycles += 3;
                c.g[A1][0] = 0u64;
                c.g[A2][0] = 0u64;
                call_ptr(c, 0x0039c870, 0x001fde04)?;
                break 'b_1fde04;
            }
        }
        c.cycles += 23;
        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x69c));
        c.g[V0][0] = sx((c.g[S4][0] as u32).wrapping_add(0x28f0));
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
        c.g[A2][0] = sx((c.g[S6][0] as u32).wrapping_add(0x66e8));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
        c.g[A1][0] = 0x2;
        c.g[V0][0] = sx((c.g[V1][0] as u32) << 2);
        c.sw((c.g[GP][0] as u32).wrapping_sub(0x6a0), c.g[A0][0]);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
        c.g[A0][0] = c.g[S1][0];
        c.sw(c.g[V0][0] as u32, c.g[A1][0]);
        c.sw((c.g[GP][0] as u32).wrapping_sub(0x69c), c.g[V1][0]);
        Callee {
            addr: 0x0020a380,
            f: ee::s05::cOVStateOverlay_Init,
        }
        .call(c, 0x001fde38)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xa0));
        c.pc = t;
        return Ok(());
    });
}

/// `void OVScreenSaveReplay_OnWidgetAction(OVScreen* this, void* msg, int type)`
/// (0x001fe6e8): widget events of the save-replay screen (types 5, 6, 7:
/// confirm card slot, stop/skip, save), driving the memory-card manager
/// (`MCManager_Get`) through its virtual methods and the state at `this+0x1c0`.
pub fn savereplay_widget_action(c: &mut Cpu) {
    guest(c, |c| {
        'b_1fe8d4: {
            'b_1fe8d0: {
                c.cycles += 10;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[S0][0] = c.g[A0][0];
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.g[S2][0] = c.g[A1][0];
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
                let t = c.g[S2][0] == 0u64;
                c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                if t {
                    break 'b_1fe8d0;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1a8));
                    if c.g[V0][0] != 0u64 {
                        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                        break 'b_1fe8d4;
                    } else {
                        'b_1fe830: {
                            'b_1fe7c8: {
                                c.cycles += 3;
                                c.g[S1][0] = 0x6;
                                let t = c.g[A2][0] == c.g[S1][0];
                                c.g[V0][0] = (c.g[A2][0] < 0x7u64) as u64;
                                if t {
                                    break 'b_1fe7c8;
                                } else {
                                    'b_1fe754: {
                                        'b_1fe740: {
                                            c.cycles += 2;
                                            let t = c.g[V0][0] == 0u64;
                                            c.g[V0][0] = 0x5;
                                            if t {
                                                break 'b_1fe740;
                                            } else {
                                                c.cycles += 2;
                                                if c.g[A2][0] == c.g[V0][0] {
                                                    c.g[V0][0] =
                                                        c.lw(
                                                            (c.g[S0][0] as u32).wrapping_add(0x1c0)
                                                        );
                                                    break 'b_1fe754;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = true;
                                                    c.lq(
                                                        16,
                                                        (c.g[SP][0] as u32).wrapping_add(0x50),
                                                    );
                                                    if t {
                                                        break 'b_1fe8d4;
                                                    } else {
                                                        break 'b_1fe740;
                                                    }
                                                }
                                            }
                                        }
                                        c.cycles += 3;
                                        c.g[V0][0] = 0x7;
                                        if c.g[A2][0] == c.g[V0][0] {
                                            c.g[V0][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x1c0));
                                            break 'b_1fe830;
                                        } else {
                                            c.cycles += 2;
                                            let t = true;
                                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                                            if t {
                                                break 'b_1fe8d4;
                                            } else {
                                                break 'b_1fe754;
                                            }
                                        }
                                    }
                                    c.cycles += 2;
                                    let t = c.g[V0][0] == c.g[S1][0];
                                    c.g[S3][0] = 0x1;
                                    if t {
                                        break 'b_1fe8d0;
                                    } else {
                                        c.cycles += 7;
                                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                        Callee {
                                            addr: 0x00227f80,
                                            f: ee::s05::MCManager_Get,
                                        }
                                        .call(c, 0x001fe764)?;
                                        c.g[S1][0] = c.g[V0][0];
                                        c.g[S4][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x214));
                                        c.g[V0][0] = 0x3;
                                        if c.g[S4][0] != c.g[V0][0] {
                                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                                            break 'b_1fe8d4;
                                        } else {
                                            'b_1fe7ac: {
                                                c.cycles += 7;
                                                c.g[V0][0] =
                                                    c.lw((c.g[S2][0] as u32).wrapping_add(0x18));
                                                c.sw(
                                                    (c.g[S0][0] as u32).wrapping_add(0x1c4),
                                                    c.g[V0][0],
                                                );
                                                c.sw(
                                                    (c.g[S1][0] as u32).wrapping_add(0xf8),
                                                    c.g[V0][0],
                                                );
                                                c.sw(
                                                    (c.g[S0][0] as u32).wrapping_add(0x1a8),
                                                    c.g[S3][0],
                                                );
                                                c.g[V0][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x1c8));
                                                let t = c.g[V0][0] != c.g[S3][0];
                                                c.g[A0][0] = c.g[S1][0];
                                                if t {
                                                    break 'b_1fe7ac;
                                                } else {
                                                    c.cycles += 6;
                                                    c.g[V0][0] =
                                                        c.lw(
                                                            (c.g[GP][0] as u32).wrapping_sub(0x848)
                                                        );
                                                    c.g[V1][0] = c
                                                        .lw((c.g[V0][0] as u32).wrapping_add(0x84));
                                                    c.g[A0][0] = c
                                                        .lw((c.g[V1][0] as u32).wrapping_add(0x28));
                                                    call_ptr(c, 0x00270ed8, 0x001fe7a4)?;
                                                    c.sw(
                                                        (c.g[S0][0] as u32).wrapping_add(0x1c8),
                                                        0u64,
                                                    );
                                                    c.g[A0][0] = c.g[S1][0];
                                                    break 'b_1fe7ac;
                                                }
                                            }
                                            c.cycles += 7;
                                            c.g[A1][0] = 0u64;
                                            c.g[A2][0] = 0xffffffffffffffff;
                                            call_ptr(c, 0x0023cae8, 0x001fe7b8)?;
                                            c.sw(
                                                (c.g[S0][0] as u32).wrapping_add(0x1c0),
                                                c.g[S4][0],
                                            );
                                            c.sw(
                                                (c.g[S0][0] as u32).wrapping_add(0x1a8),
                                                c.g[S3][0],
                                            );
                                            let t = true;
                                            c.sw(
                                                (c.g[S0][0] as u32).wrapping_add(0x1dc),
                                                c.g[S3][0],
                                            );
                                            if t {
                                                break 'b_1fe8d0;
                                            } else {
                                                break 'b_1fe7c8;
                                            }
                                        }
                                    }
                                }
                            }
                            c.cycles += 7;
                            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                            Callee {
                                addr: 0x00227f80,
                                f: ee::s05::MCManager_Get,
                            }
                            .call(c, 0x001fe7d0)?;
                            c.g[A2][0] = c.g[V0][0];
                            c.g[A0][0] = 0x1;
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1a8));
                            if c.g[V0][0] == c.g[A0][0] {
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                                break 'b_1fe8d4;
                            } else {
                                c.cycles += 3;
                                c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c0));
                                if c.g[V1][0] == c.g[S1][0] {
                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                                    break 'b_1fe8d4;
                                } else {
                                    'b_1fe828: {
                                        'b_1fe810: {
                                            'b_1fe80c: {
                                                c.cycles += 2;
                                                let t = c.g[V1][0] == 0u64;
                                                c.sw(
                                                    (c.g[S0][0] as u32).wrapping_add(0x1e4),
                                                    c.g[A0][0],
                                                );
                                                if t {
                                                    break 'b_1fe80c;
                                                } else {
                                                    c.cycles += 3;
                                                    c.g[V0][0] = 0x2;
                                                    if c.g[V1][0] == c.g[V0][0] {
                                                        c.g[V0][0] = c
                                                            .lw((c.g[A2][0] as u32)
                                                                .wrapping_add(0x748));
                                                        break 'b_1fe810;
                                                    } else {
                                                        c.cycles += 2;
                                                        let t = c.g[V1][0] != c.g[A0][0];
                                                        c.g[V0][0] = 0x6;
                                                        if t {
                                                            break 'b_1fe828;
                                                        } else {
                                                            break 'b_1fe80c;
                                                        }
                                                    }
                                                }
                                            }
                                            c.cycles += 1;
                                            c.g[V0][0] =
                                                c.lw((c.g[A2][0] as u32).wrapping_add(0x748));
                                            break 'b_1fe810;
                                        }
                                        c.cycles += 6;
                                        c.g[A1][0] = 0u64;
                                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
                                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                                        let t = c.g[V1][0] as u32;
                                        c.g[A0][0] =
                                            sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                        call_ptr(c, t, 0x001fe824)?;
                                        c.g[V0][0] = 0x6;
                                        break 'b_1fe828;
                                    }
                                    c.cycles += 2;
                                    let t = true;
                                    c.sw((c.g[S0][0] as u32).wrapping_add(0x1c0), c.g[V0][0]);
                                    if t {
                                        break 'b_1fe8d0;
                                    } else {
                                        break 'b_1fe830;
                                    }
                                }
                            }
                        }
                        c.cycles += 2;
                        let t = c.g[V0][0] == c.g[S1][0];
                        c.g[S3][0] = 0x1;
                        if t {
                            break 'b_1fe8d0;
                        } else {
                            c.cycles += 12;
                            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                            Callee {
                                addr: 0x00227f80,
                                f: ee::s05::MCManager_Get,
                            }
                            .call(c, 0x001fe840)?;
                            c.g[S1][0] = c.g[V0][0];
                            c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x434));
                            c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x428));
                            c.g[V1][0] = c.lw(c.g[A2][0] as u32);
                            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x1b0));
                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1b4));
                            let t = c.g[V0][0] as u32;
                            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x001fe860)?;
                            if c.g[V0][0] == 0u64 {
                                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                                break 'b_1fe8d4;
                            } else {
                                c.cycles += 6;
                                c.g[V0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x18));
                                c.sw((c.g[S0][0] as u32).wrapping_add(0x1c4), c.g[V0][0]);
                                c.sw((c.g[S1][0] as u32).wrapping_add(0xf8), c.g[V0][0]);
                                c.g[S2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c0));
                                if c.g[S2][0] != c.g[S3][0] {
                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
                                    break 'b_1fe8d4;
                                } else {
                                    c.cycles += 9;
                                    c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x434));
                                    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c4));
                                    c.g[V1][0] = c.lw(c.g[A2][0] as u32);
                                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x218));
                                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x21c));
                                    let t = c.g[V0][0] as u32;
                                    c.g[A0][0] =
                                        sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    call_ptr(c, t, 0x001fe89c)?;
                                    let t = c.g[V0][0] == 0u64;
                                    c.g[V0][0] = 0x4;
                                    if t {
                                        break 'b_1fe8d0;
                                    } else {
                                        c.cycles += 11;
                                        c.sw((c.g[S0][0] as u32).wrapping_add(0x1a8), c.g[S2][0]);
                                        c.g[A0][0] = c.g[S1][0];
                                        c.sw((c.g[S0][0] as u32).wrapping_add(0x1c0), c.g[V0][0]);
                                        c.g[A1][0] = 0u64;
                                        call_ptr(c, 0x0023d570, 0x001fe8b8)?;
                                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x748));
                                        c.g[A1][0] = 0x18;
                                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
                                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                                        let t = c.g[V1][0] as u32;
                                        c.g[A0][0] =
                                            sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                        call_ptr(c, t, 0x001fe8d0)?;
                                        break 'b_1fe8d0;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
            break 'b_1fe8d4;
        }
        c.cycles += 7;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
        c.pc = t;
        return Ok(());
    });
}

/// `OVScreenSaveRecords_Init(self)` (0x001fe9a8): builds the Save Replay / Save Records screen (`self+0x40`): creates it by hash name, hides the replay title, shows the records title and help text, clears the text fields and hides memory-card slots 2..4; then a dark-blue backdrop screen and `cOVState_onCreateScreen`.
pub fn ov_screen_save_records_init(c: &mut Cpu) {
    guest(c, |c| {
        'b_1febd4: {
            c.cycles += 23;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0xb0));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
            c.g[S2][0] = c.g[A0][0];
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
            c.g[A0][0] = 0x470000;
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xe0));
            c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.g[S4][0] = 0x470000;
            c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
            c.g[S0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
            call_ptr(c, 0x00317670, 0x001fe9e8)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S2][0];
            c.g[A3][0] = 0u64;
            call_ptr(c, 0x00397cc0, 0x001fe9fc)?;
            let t = c.g[V0][0] == 0u64;
            c.sw((c.g[S2][0] as u32).wrapping_add(0x40), c.g[V0][0]);
            if t {
                break 'b_1febd4;
            } else {
                c.cycles += 69;
                c.g[A2][0] = 0u64;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0u64;
                c.g[S3][0] = 0x1;
                call_ptr(c, 0x0039c870, 0x001fea18)?;
                c.g[S7][0] = 0x470000;
                c.g[A0][0] = 0x470000;
                c.g[S6][0] = 0x4a0000;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xf0));
                c.g[S5][0] = 0x1;
                call_ptr(c, 0x00317670, 0x001fea30)?;
                c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039d860, 0x001fea3c)?;
                c.g[S0][0] = 0x4a0000;
                c.g[A0][0] = c.g[V0][0];
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x2560));
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x003a0c50, 0x001fea50)?;
                c.g[A0][0] = 0x470000;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x100));
                call_ptr(c, 0x00317670, 0x001fea5c)?;
                c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039d860, 0x001fea68)?;
                c.g[S1][0] = c.g[V0][0];
                c.g[A1][0] = 0u64;
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                let t = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x001fea84)?;
                c.g[A0][0] = 0x470000;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x118));
                call_ptr(c, 0x00317670, 0x001fea90)?;
                c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039d860, 0x001fea9c)?;
                c.g[S1][0] = c.g[V0][0];
                c.g[A1][0] = 0x1;
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                let t = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x001feab8)?;
                c.g[A0][0] = 0x470000;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1608));
                call_ptr(c, 0x00317670, 0x001feac4)?;
                c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039d860, 0x001fead0)?;
                c.g[S1][0] = c.g[V0][0];
                c.g[A1][0] = 0x1;
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                let t = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x001feaec)?;
                c.g[A0][0] = 0x470000;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x130));
                call_ptr(c, 0x00317670, 0x001feaf8)?;
                c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039d860, 0x001feb04)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, 0x003a0c50, 0x001feb10)?;
                c.g[A0][0] = c.g[SP][0];
                'l_1feb18: loop {
                    'b_1feb78: {
                        'b_1feb48: {
                            c.cycles += 10;
                            c.g[A1][0] = sx((c.g[S7][0] as u32).wrapping_add(0x148));
                            c.g[A2][0] = c.g[S3][0];
                            call_ptr(c, 0x00417828, 0x001feb24)?;
                            c.g[A0][0] = c.g[SP][0];
                            call_ptr(c, 0x00317670, 0x001feb2c)?;
                            c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x0039d860, 0x001feb38)?;
                            let t = c.g[S3][0] != c.g[S5][0];
                            c.g[S1][0] = c.g[V0][0];
                            if t {
                                break 'b_1feb48;
                            } else {
                                c.cycles += 2;
                                let t = true;
                                c.sw((c.g[S1][0] as u32).wrapping_add(0x18), 0u64);
                                if t {
                                    break 'b_1feb78;
                                } else {
                                    break 'b_1feb48;
                                }
                            }
                        }
                        c.cycles += 12;
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                        c.g[A1][0] = 0x1;
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x40));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x44));
                        let t = c.g[V1][0] as u32;
                        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, t, 0x001feb60)?;
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                        c.g[A1][0] = 0u64;
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                        let t = c.g[V1][0] as u32;
                        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, t, 0x001feb78)?;
                        break 'b_1feb78;
                    }
                    'b_1febc8: {
                        c.cycles += 13;
                        c.g[A0][0] = c.g[SP][0];
                        c.g[A1][0] = sx((c.g[S6][0] as u32).wrapping_add(0x2138));
                        c.g[A2][0] = c.g[S3][0];
                        call_ptr(c, 0x00417828, 0x001feb88)?;
                        c.g[A0][0] = c.g[SP][0];
                        call_ptr(c, 0x00317670, 0x001feb90)?;
                        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x0039d860, 0x001feb9c)?;
                        c.g[S1][0] = c.g[V0][0];
                        c.g[V0][0] = ((c.g[S3][0] as i64) < 2i64) as u64;
                        if c.g[V0][0] != 0u64 {
                            c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                            break 'b_1febc8;
                        } else {
                            c.cycles += 7;
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
                            c.g[A1][0] = 0u64;
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                            let t = c.g[V1][0] as u32;
                            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x001febc4)?;
                            c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1));
                            break 'b_1febc8;
                        }
                    }
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[S3][0] as i64) < 5i64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[A0][0] = c.g[SP][0];
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x001feb18);
                        }
                        continue 'l_1feb18;
                    } else {
                        break 'b_1febd4;
                    }
                }
            }
        }
        'b_1fec08: {
            c.cycles += 10;
            c.g[A0][0] = sx((c.g[S4][0] as u32).wrapping_sub(0x1c88));
            c.g[S0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
            call_ptr(c, 0x00317670, 0x001febe0)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S2][0];
            c.g[A3][0] = 0u64;
            call_ptr(c, 0x00397cc0, 0x001febf4)?;
            let t = c.g[V0][0] == 0u64;
            c.g[A0][0] = c.g[V0][0];
            if t {
                break 'b_1fec08;
            } else {
                c.cycles += 3;
                c.g[A1][0] = 0u64;
                c.g[A2][0] = 0u64;
                call_ptr(c, 0x0039c870, 0x001fec08)?;
                break 'b_1fec08;
            }
        }
        c.cycles += 13;
        c.g[A0][0] = c.g[S2][0];
        call_ptr(c, 0x0020a380, 0x001fec10)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0xa0));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x90));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x80));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x70));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0xb0));
        c.pc = t;
        return Ok(());
    })
}

/// `OVScreenSaveRecords_v26(self)` (0x001feca0, displayStatus): refreshes the memory-card status text of the Save Records screen from the memory-card manager (device string, slot name, static/enabled state of the slot widget, visibility of the two save-type widgets).
pub fn ov_screen_save_records_v26(c: &mut Cpu) {
    guest(c, |c| {
        'b_1fed40: {
            c.cycles += 19;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x6e0));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x6a0));
            c.g[S3][0] = c.g[A0][0];
            c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x690));
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x6d0));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x6c0));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x6b0));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x680), c.g[RA][0]);
            call_ptr(c, 0x00227f80, 0x001feccc)?;
            c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
            c.g[S4][0] = c.g[V0][0];
            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x150));
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x154));
            let t = c.g[V0][0] as u32;
            c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, t, 0x001fece4)?;
            if c.g[V0][0] == 0u64 {
                c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
                break 'b_1fed40;
            } else {
                'b_1fed1c: {
                    c.cycles += 8;
                    c.g[A1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x434));
                    c.g[V1][0] = c.lw(c.g[A1][0] as u32);
                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x1a8));
                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1ac));
                    let t = c.g[V0][0] as u32;
                    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x001fed04)?;
                    if c.g[V0][0] != 0u64 {
                        c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x214));
                        break 'b_1fed1c;
                    } else {
                        c.cycles += 4;
                        call_ptr(c, 0x001d8240, 0x001fed14)?;
                        let t = true;
                        c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
                        if t {
                            break 'b_1fed40;
                        } else {
                            break 'b_1fed1c;
                        }
                    }
                }
                'b_1fed3c: {
                    c.cycles += 3;
                    c.g[V0][0] = 0x3;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[A1][0] = 0x3;
                    if t {
                        break 'b_1fed3c;
                    } else {
                        c.cycles += 5;
                        c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x118));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x11c));
                        let t = c.g[V1][0] as u32;
                        c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, t, 0x001fed3c)?;
                        break 'b_1fed3c;
                    }
                }
                c.cycles += 1;
                c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
                break 'b_1fed40;
            }
        }
        'b_1ff0e8: {
            'b_1ff0e4: {
                c.cycles += 6;
                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x150));
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x154));
                let t = c.g[V0][0] as u32;
                c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x001fed50)?;
                let t = c.g[V0][0] == 0u64;
                c.g[S2][0] = sx((c.g[S3][0] as u32).wrapping_add(0x230));
                if t {
                    break 'b_1ff0e4;
                } else {
                    'b_1fefe8: {
                        'b_1fefb4: {
                            'b_1fefa4: {
                                'b_1fef1c: {
                                    c.cycles += 86;
                                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
                                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x158));
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x15c));
                                    let t = c.g[V1][0] as u32;
                                    c.g[A0][0] =
                                        sx((c.g[S3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    call_ptr(c, t, 0x001fed6c)?;
                                    c.g[A1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x428));
                                    c.g[A2][0] = c.g[SP][0];
                                    c.g[A3][0] = 0x320;
                                    c.g[A0][0] = c.g[S4][0];
                                    call_ptr(c, 0x00241710, 0x001fed80)?;
                                    c.g[A0][0] = 0x470000;
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x130));
                                    call_ptr(c, 0x00317670, 0x001fed8c)?;
                                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                                    c.g[A1][0] = c.g[V0][0];
                                    call_ptr(c, 0x0039d860, 0x001fed98)?;
                                    c.g[A1][0] = c.g[SP][0];
                                    c.g[A0][0] = c.g[V0][0];
                                    call_ptr(c, 0x003a0d00, 0x001feda4)?;
                                    c.g[A0][0] = 0x470000;
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xf0));
                                    call_ptr(c, 0x00317670, 0x001fedb0)?;
                                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                                    c.g[A1][0] = c.g[V0][0];
                                    call_ptr(c, 0x0039d860, 0x001fedbc)?;
                                    c.g[A2][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x434));
                                    c.g[S0][0] = c.g[V0][0];
                                    c.g[A1][0] = c.g[SP][0];
                                    c.g[V0][0] = c.lw(c.g[A2][0] as u32);
                                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x200));
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x204));
                                    let t = c.g[V1][0] as u32;
                                    c.g[A0][0] =
                                        sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    call_ptr(c, t, 0x001feddc)?;
                                    c.g[A1][0] = c.g[SP][0];
                                    c.g[A0][0] = c.g[S0][0];
                                    call_ptr(c, 0x003a0e90, 0x001fede8)?;
                                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                    c.g[A1][0] = 0u64;
                                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                    let t = c.g[V1][0] as u32;
                                    c.g[A0][0] =
                                        sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    call_ptr(c, t, 0x001fee00)?;
                                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                    c.g[A0][0] = 0x470000;
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x158));
                                    c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                                    c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                                    c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                                    c.g[S0][0] =
                                        sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                                    call_ptr(c, 0x00317670, 0x001fee20)?;
                                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                                    c.g[A0][0] = c.g[S0][0];
                                    let t = c.g[V1][0] as u32;
                                    c.g[A1][0] = c.g[V0][0];
                                    call_ptr(c, t, 0x001fee30)?;
                                    c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x640));
                                    c.g[A1][0] = 0x470000;
                                    c.g[A2][0] = 0x1;
                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x148));
                                    c.g[A0][0] = c.g[S0][0];
                                    call_ptr(c, 0x00417828, 0x001fee48)?;
                                    c.g[A0][0] = c.g[S0][0];
                                    call_ptr(c, 0x00317670, 0x001fee50)?;
                                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                                    c.g[A1][0] = c.g[V0][0];
                                    call_ptr(c, 0x0039d860, 0x001fee5c)?;
                                    c.g[A3][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x434));
                                    c.g[S0][0] = c.g[V0][0];
                                    c.g[A2][0] = c.g[S2][0];
                                    c.g[A1][0] = 0u64;
                                    c.g[V0][0] = c.lw(c.g[A3][0] as u32);
                                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x70));
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x74));
                                    let t = c.g[V1][0] as u32;
                                    c.g[A0][0] =
                                        sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    call_ptr(c, t, 0x001fee80)?;
                                    c.g[A0][0] = c.g[S0][0];
                                    c.g[A1][0] = c.g[S2][0];
                                    call_ptr(c, 0x003a0e90, 0x001fee8c)?;
                                    c.g[A2][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x434));
                                    c.g[A1][0] = 0u64;
                                    c.g[V1][0] = c.lw(c.g[A2][0] as u32);
                                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x218));
                                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x21c));
                                    let t = c.g[V0][0] as u32;
                                    c.g[A0][0] =
                                        sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    call_ptr(c, t, 0x001feea8)?;
                                    let t = c.g[V0][0] != 0u64;
                                    c.g[A2][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x434));
                                    if t {
                                        break 'b_1fef1c;
                                    } else {
                                        c.cycles += 8;
                                        c.g[A1][0] = 0u64;
                                        c.g[V1][0] = c.lw(c.g[A2][0] as u32);
                                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x210));
                                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x214));
                                        let t = c.g[V0][0] as u32;
                                        c.g[A0][0] =
                                            sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                        call_ptr(c, t, 0x001feec8)?;
                                        if c.g[V0][0] != 0u64 {
                                            c.g[A2][0] =
                                                c.lw((c.g[S4][0] as u32).wrapping_add(0x434));
                                            break 'b_1fef1c;
                                        } else {
                                            'b_1fef18: {
                                                c.cycles += 8;
                                                c.g[A1][0] =
                                                    c.lw((c.g[S4][0] as u32).wrapping_add(0x434));
                                                c.g[V1][0] = c.lw(c.g[A1][0] as u32);
                                                c.g[A0][0] =
                                                    c.lh((c.g[V1][0] as u32).wrapping_add(0x1b8));
                                                c.g[V0][0] =
                                                    c.lw((c.g[V1][0] as u32).wrapping_add(0x1bc));
                                                let t = c.g[V0][0] as u32;
                                                c.g[A0][0] = sx((c.g[A1][0] as u32)
                                                    .wrapping_add(c.g[A0][0] as u32));
                                                call_ptr(c, t, 0x001feee8)?;
                                                let t = c.g[V0][0] == 0u64;
                                                c.g[A1][0] = 0u64;
                                                if t {
                                                    break 'b_1fef18;
                                                } else {
                                                    c.cycles += 10;
                                                    c.g[V0][0] =
                                                        c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                                    c.g[A0][0] = c
                                                        .lh((c.g[V0][0] as u32).wrapping_add(0xc8));
                                                    c.g[V1][0] = c
                                                        .lw((c.g[V0][0] as u32).wrapping_add(0xcc));
                                                    let t = c.g[V1][0] as u32;
                                                    c.g[A0][0] = sx((c.g[S0][0] as u32)
                                                        .wrapping_add(c.g[A0][0] as u32));
                                                    call_ptr(c, t, 0x001fef04)?;
                                                    c.g[V0][0] =
                                                        c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                                    c.g[A1][0] = 0u64;
                                                    c.g[A0][0] = c
                                                        .lh((c.g[V0][0] as u32).wrapping_add(0x40));
                                                    let t = true;
                                                    c.g[V1][0] = c
                                                        .lw((c.g[V0][0] as u32).wrapping_add(0x44));
                                                    if t {
                                                        break 'b_1fefa4;
                                                    } else {
                                                        break 'b_1fef18;
                                                    }
                                                }
                                            }
                                            c.cycles += 1;
                                            c.g[A2][0] =
                                                c.lw((c.g[S4][0] as u32).wrapping_add(0x434));
                                            break 'b_1fef1c;
                                        }
                                    }
                                }
                                c.cycles += 8;
                                c.g[A1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x428));
                                c.g[V1][0] = c.lw(c.g[A2][0] as u32);
                                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x1b0));
                                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1b4));
                                let t = c.g[V0][0] as u32;
                                c.g[A0][0] =
                                    sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                call_ptr(c, t, 0x001fef34)?;
                                let t = c.g[V0][0] == 0u64;
                                c.g[A1][0] = 0u64;
                                if t {
                                    break 'b_1fefb4;
                                } else {
                                    'b_1fef9c: {
                                        'b_1fef7c: {
                                            c.cycles += 8;
                                            c.g[A2][0] =
                                                c.lw((c.g[S4][0] as u32).wrapping_add(0x434));
                                            c.g[V1][0] = c.lw(c.g[A2][0] as u32);
                                            c.g[A0][0] =
                                                c.lh((c.g[V1][0] as u32).wrapping_add(0x210));
                                            c.g[V0][0] =
                                                c.lw((c.g[V1][0] as u32).wrapping_add(0x214));
                                            let t = c.g[V0][0] as u32;
                                            c.g[A0][0] =
                                                sx((c.g[A2][0] as u32)
                                                    .wrapping_add(c.g[A0][0] as u32));
                                            call_ptr(c, t, 0x001fef54)?;
                                            let t = c.g[V0][0] == 0u64;
                                            c.g[A1][0] = 0x1;
                                            if t {
                                                break 'b_1fef7c;
                                            } else {
                                                c.cycles += 8;
                                                c.g[V0][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                                c.g[A0][0] =
                                                    c.lh((c.g[V0][0] as u32).wrapping_add(0x40));
                                                c.g[V1][0] =
                                                    c.lw((c.g[V0][0] as u32).wrapping_add(0x44));
                                                let t = c.g[V1][0] as u32;
                                                c.g[A0][0] = sx((c.g[S0][0] as u32)
                                                    .wrapping_add(c.g[A0][0] as u32));
                                                call_ptr(c, t, 0x001fef70)?;
                                                c.g[V0][0] =
                                                    c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                                let t = true;
                                                c.g[A1][0] = 0x1;
                                                if t {
                                                    break 'b_1fef9c;
                                                } else {
                                                    break 'b_1fef7c;
                                                }
                                            }
                                        }
                                        c.cycles += 8;
                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                        c.g[A1][0] = 0u64;
                                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x40));
                                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x44));
                                        let t = c.g[V1][0] as u32;
                                        c.g[A0][0] =
                                            sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                        call_ptr(c, t, 0x001fef94)?;
                                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                                        c.g[A1][0] = 0u64;
                                        break 'b_1fef9c;
                                    }
                                    c.cycles += 2;
                                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0xc8));
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xcc));
                                    break 'b_1fefa4;
                                }
                            }
                            c.cycles += 4;
                            let t = c.g[V1][0] as u32;
                            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x001fefac)?;
                            let t = true;
                            c.g[A2][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x428));
                            if t {
                                break 'b_1fefe8;
                            } else {
                                break 'b_1fefb4;
                            }
                        }
                        c.cycles += 13;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                        c.g[A1][0] = 0x1;
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0xc8));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xcc));
                        let t = c.g[V1][0] as u32;
                        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, t, 0x001fefcc)?;
                        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                        c.g[A1][0] = 0x1;
                        c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x40));
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x44));
                        let t = c.g[V1][0] as u32;
                        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, t, 0x001fefe4)?;
                        c.g[A2][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x428));
                        break 'b_1fefe8;
                    }
                    'b_1ff068: {
                        'b_1ff038: {
                            c.cycles += 7;
                            c.g[A0][0] = c.g[S3][0];
                            c.g[A1][0] = c.g[S4][0];
                            c.g[A3][0] = 0x1;
                            call_ptr(c, 0x001d97e0, 0x001feff8)?;
                            c.g[V1][0] = 0xffffffffffffffff;
                            let t = c.g[V0][0] != c.g[V1][0];
                            c.g[A0][0] = 0x470000;
                            if t {
                                break 'b_1ff038;
                            } else {
                                c.cycles += 13;
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x168));
                                call_ptr(c, 0x00317670, 0x001ff00c)?;
                                c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                                c.g[A1][0] = c.g[V0][0];
                                call_ptr(c, 0x0039d860, 0x001ff018)?;
                                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
                                c.g[A1][0] = 0u64;
                                c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x48));
                                c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4c));
                                let t = c.g[A2][0] as u32;
                                c.g[A0][0] =
                                    sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                call_ptr(c, t, 0x001ff030)?;
                                let t = true;
                                c.g[A1][0] = c.g[S4][0];
                                if t {
                                    break 'b_1ff068;
                                } else {
                                    break 'b_1ff038;
                                }
                            }
                        }
                        c.cycles += 12;
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x168));
                        call_ptr(c, 0x00317670, 0x001ff040)?;
                        c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x0039d860, 0x001ff04c)?;
                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
                        c.g[A1][0] = 0x1;
                        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x48));
                        c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4c));
                        let t = c.g[A2][0] as u32;
                        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, t, 0x001ff064)?;
                        c.g[A1][0] = c.g[S4][0];
                        break 'b_1ff068;
                    }
                    'b_1ff0b8: {
                        c.cycles += 7;
                        c.g[A0][0] = c.g[S3][0];
                        c.g[A2][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x428));
                        c.g[A3][0] = 0u64;
                        call_ptr(c, 0x001d97e0, 0x001ff078)?;
                        c.g[V1][0] = 0xffffffffffffffff;
                        let t = c.g[V0][0] != c.g[V1][0];
                        c.g[A0][0] = 0x470000;
                        if t {
                            break 'b_1ff0b8;
                        } else {
                            c.cycles += 13;
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x178));
                            call_ptr(c, 0x00317670, 0x001ff08c)?;
                            c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                            c.g[A1][0] = c.g[V0][0];
                            call_ptr(c, 0x0039d860, 0x001ff098)?;
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
                            c.g[A1][0] = 0u64;
                            c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x48));
                            c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4c));
                            let t = c.g[A2][0] as u32;
                            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x001ff0b0)?;
                            let t = true;
                            c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x214));
                            if t {
                                break 'b_1ff0e8;
                            } else {
                                break 'b_1ff0b8;
                            }
                        }
                    }
                    c.cycles += 11;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x178));
                    call_ptr(c, 0x00317670, 0x001ff0c0)?;
                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x001ff0cc)?;
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8));
                    c.g[A1][0] = 0x1;
                    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x48));
                    c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4c));
                    let t = c.g[A2][0] as u32;
                    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x001ff0e4)?;
                    break 'b_1ff0e4;
                }
            }
            c.cycles += 1;
            c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x214));
            break 'b_1ff0e8;
        }
        'b_1ff150: {
            c.cycles += 3;
            c.g[V0][0] = 0x3;
            let t = c.g[V1][0] == c.g[V0][0];
            c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x640));
            if t {
                break 'b_1ff150;
            } else {
                c.cycles += 23;
                c.g[A1][0] = 0x470000;
                c.g[A2][0] = 0x1;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x148));
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x00417828, 0x001ff108)?;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x00317670, 0x001ff110)?;
                c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x40));
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039d860, 0x001ff11c)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A1][0] = 0u64;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0xc8));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xcc));
                let t = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x001ff138)?;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                c.g[A1][0] = 0x1;
                c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x40));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x44));
                let t = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, t, 0x001ff150)?;
                break 'b_1ff150;
            }
        }
        c.cycles += 8;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x6d0));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x6c0));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x6b0));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x6a0));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x690));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x680));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x6e0));
        c.pc = t;
        return Ok(());
    })
}

/// `OVScreenRewardList_ShowPage(self, which)` (0x001ffd08, updateMenu): fills the 9 text rows of the reward list from entry `which`, greys rows past the count (`self+0x9c`) and shows or hides the up/down arrows.
pub fn ov_screen_reward_list_show_page(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x250));
        c.g[V0][0] = 0x4a0000;
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x210));
        c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x1e0));
        c.g[S1][0] = c.g[A0][0];
        c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x200));
        c.g[S4][0] = c.g[A1][0];
        c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x1f0));
        c.g[S2][0] = 0u64;
        c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x1d0));
        c.g[S3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x1c0));
        c.g[S5][0] = 0x4a0000;
        c.swc1(23, (c.g[SP][0] as u32).wrapping_add(0x248));
        c.g[S6][0] = 0x4a0000;
        c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0x240));
        c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x238));
        c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x230));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x220));
        c.sd((c.g[SP][0] as u32).wrapping_add(0x1b0), c.g[RA][0]);
        c.g[AT][0] = 0x3f800000;
        c.mtc1(1, 23);
        c.lwc1(22, (c.g[GP][0] as u32).wrapping_sub(0x553c));
        c.lwc1(21, (c.g[GP][0] as u32).wrapping_sub(0x5538));
        c.lwc1(20, (c.g[GP][0] as u32).wrapping_sub(0x5534));
        c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2580));
        c.g[V1][0] = c.lb(c.g[A2][0] as u32);
        c.g[A0][0] = c.lb((c.g[A2][0] as u32).wrapping_add(0x1));
        c.sb(c.g[SP][0] as u32, c.g[V1][0]);
        c.sb((c.g[SP][0] as u32).wrapping_add(0x1), c.g[A0][0]);
        c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x30));
        'l_1ffd88: loop {
            'b_1ffdc4: {
                c.cycles += 13;
                c.g[A1][0] = c.g[S3][0];
                c.sb(c.g[SP][0] as u32, c.g[V0][0]);
                c.g[A2][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[S4][0] as u32));
                c.g[A0][0] = c.g[S1][0];
                call_ptr(c, 0x001ff7b8, 0x001ffd9c)?;
                c.g[A0][0] = c.g[SP][0];
                call_ptr(c, 0x00317670, 0x001ffda4)?;
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039d860, 0x001ffdb0)?;
                c.g[S0][0] = c.g[V0][0];
                let t = c.g[S0][0] == 0u64;
                c.g[A0][0] = c.g[S0][0];
                if t {
                    break 'b_1ffdc4;
                } else {
                    c.cycles += 2;
                    c.g[A1][0] = c.g[S3][0];
                    call_ptr(c, 0x003a0e90, 0x001ffdc4)?;
                    break 'b_1ffdc4;
                }
            }
            'b_1ffe30: {
                c.cycles += 4;
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x9c));
                c.g[V0][0] = ((c.g[V0][0] as i64) < 10i64) as u64;
                let t = c.g[V0][0] == 0u64;
                c.g[A1][0] = 0x1;
                if t {
                    break 'b_1ffe30;
                } else {
                    c.cycles += 23;
                    c.swc1(23, (c.g[SP][0] as u32).wrapping_add(0x1a0));
                    c.swc1(22, (c.g[SP][0] as u32).wrapping_add(0x1a4));
                    c.swc1(21, (c.g[SP][0] as u32).wrapping_add(0x1a8));
                    c.swc1(20, (c.g[SP][0] as u32).wrapping_add(0x1ac));
                    c.ldl(2, (c.g[SP][0] as u32).wrapping_add(0x1a7));
                    c.ldr(2, (c.g[SP][0] as u32).wrapping_add(0x1a0));
                    c.ldl(3, (c.g[SP][0] as u32).wrapping_add(0x1af));
                    c.ldr(3, (c.g[SP][0] as u32).wrapping_add(0x1a8));
                    c.sdl(2, (c.g[S0][0] as u32).wrapping_add(0x8f));
                    c.sdr(2, (c.g[S0][0] as u32).wrapping_add(0x88));
                    c.sdl(3, (c.g[S0][0] as u32).wrapping_add(0x97));
                    c.sdr(3, (c.g[S0][0] as u32).wrapping_add(0x90));
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0xc8));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xcc));
                    let t = c.g[V1][0] as u32;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x001ffe18)?;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
                    c.g[A1][0] = 0x1;
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x40));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x44));
                    let t = c.g[V1][0] as u32;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, t, 0x001ffe30)?;
                    break 'b_1ffe30;
                }
            }
            c.cycles += 4;
            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
            c.g[V0][0] = ((c.g[S2][0] as i64) < 9i64) as u64;
            let t = c.g[V0][0] != 0u64;
            c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x30));
            if t {
                if event_due(c) {
                    deliver_event_here(c, 0x001ffd88);
                }
                continue 'l_1ffd88;
            } else {
                'b_1ffebc: {
                    'b_1ffeb8: {
                        'b_1ffe84: {
                            c.cycles += 2;
                            let t = c.g[S4][0] != 0u64;

                            if t {
                                break 'b_1ffe84;
                            } else {
                                c.cycles += 8;
                                c.g[A0][0] = sx((c.g[S5][0] as u32).wrapping_add(0x2570));
                                call_ptr(c, 0x00317670, 0x001ffe50)?;
                                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                                c.g[A1][0] = c.g[V0][0];
                                call_ptr(c, 0x0039d860, 0x001ffe5c)?;
                                c.g[A2][0] = c.g[V0][0];
                                let t = c.g[A2][0] == 0u64;
                                c.g[A1][0] = 0u64;
                                if t {
                                    break 'b_1ffeb8;
                                } else {
                                    c.cycles += 7;
                                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                    let t = c.g[V1][0] as u32;
                                    c.g[A0][0] =
                                        sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    call_ptr(c, t, 0x001ffe7c)?;
                                    let t = true;
                                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x9c));
                                    if t {
                                        break 'b_1ffebc;
                                    } else {
                                        break 'b_1ffe84;
                                    }
                                }
                            }
                        }
                        c.cycles += 8;
                        c.g[A0][0] = sx((c.g[S5][0] as u32).wrapping_add(0x2570));
                        call_ptr(c, 0x00317670, 0x001ffe8c)?;
                        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x0039d860, 0x001ffe98)?;
                        c.g[A2][0] = c.g[V0][0];
                        let t = c.g[A2][0] == 0u64;
                        c.g[A1][0] = 0x1;
                        if t {
                            break 'b_1ffeb8;
                        } else {
                            c.cycles += 5;
                            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                            let t = c.g[V1][0] as u32;
                            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x001ffeb8)?;
                            break 'b_1ffeb8;
                        }
                    }
                    c.cycles += 1;
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x9c));
                    break 'b_1ffebc;
                }
                'b_1fff40: {
                    'b_1fff3c: {
                        'b_1fff08: {
                            c.cycles += 4;
                            c.g[V0][0] = sx((c.g[S4][0] as u32).wrapping_add(0x9));
                            c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                            let t = c.g[V0][0] != 0u64;

                            if t {
                                break 'b_1fff08;
                            } else {
                                c.cycles += 8;
                                c.g[A0][0] = sx((c.g[S6][0] as u32).wrapping_add(0x2578));
                                call_ptr(c, 0x00317670, 0x001ffed4)?;
                                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                                c.g[A1][0] = c.g[V0][0];
                                call_ptr(c, 0x0039d860, 0x001ffee0)?;
                                c.g[A2][0] = c.g[V0][0];
                                let t = c.g[A2][0] == 0u64;
                                c.g[A1][0] = 0u64;
                                if t {
                                    break 'b_1fff3c;
                                } else {
                                    c.cycles += 7;
                                    c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                                    let t = c.g[V1][0] as u32;
                                    c.g[A0][0] =
                                        sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                                    call_ptr(c, t, 0x001fff00)?;
                                    let t = true;
                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x220));
                                    if t {
                                        break 'b_1fff40;
                                    } else {
                                        break 'b_1fff08;
                                    }
                                }
                            }
                        }
                        c.cycles += 8;
                        c.g[A0][0] = sx((c.g[S6][0] as u32).wrapping_add(0x2578));
                        call_ptr(c, 0x00317670, 0x001fff10)?;
                        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x0039d860, 0x001fff1c)?;
                        c.g[A2][0] = c.g[V0][0];
                        let t = c.g[A2][0] == 0u64;
                        c.g[A1][0] = 0x1;
                        if t {
                            break 'b_1fff3c;
                        } else {
                            c.cycles += 5;
                            c.g[V0][0] = c.lw((c.g[A2][0] as u32).wrapping_add(0x8));
                            c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x48));
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4c));
                            let t = c.g[V1][0] as u32;
                            c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                            call_ptr(c, t, 0x001fff3c)?;
                            break 'b_1fff3c;
                        }
                    }
                    c.cycles += 1;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x220));
                    break 'b_1fff40;
                }
                c.cycles += 13;
                c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x210));
                c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x200));
                c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x1f0));
                c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x1e0));
                c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x1d0));
                c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x1c0));
                c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x1b0));
                c.lwc1(23, (c.g[SP][0] as u32).wrapping_add(0x248));
                c.lwc1(22, (c.g[SP][0] as u32).wrapping_add(0x240));
                c.lwc1(21, (c.g[SP][0] as u32).wrapping_add(0x238));
                c.lwc1(20, (c.g[SP][0] as u32).wrapping_add(0x230));
                let t = c.g[RA][0] as u32;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x250));
                c.pc = t;
                return Ok(());
            }
        }
    })
}

/// `OVScreenRewardList_TransitionIn(self)` (0x001fff78, onGainTransition): once per screen, writes the reward
/// list title (event name or `<location> <mode>`) and the congratulation line with the human rider's name
/// into the screen's text objects, then marks the transition done (`self+0x48`); returns 0.
/// The body is a `match pc` dispatcher (the mode jump table is in the game image).
/// Any other `pc` (a resume after an unwound callee or event) runs the
/// generated body (at install: its `__generated` alias).
pub fn ov_screen_reward_list_transition_in(c: &mut Cpu) {
    if c.pc != 0x001fff78 {
        return ee::s05::f_001fff78__generated(c);
    }
    guest(c, |c| {
        let mut pc: u32 = 0x001fff78;
        loop {
            match pc {
                0x001fff78 => {
                    c.cycles += 12;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x270));
                    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x220));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x260));
                    c.g[S4][0] = c.g[A0][0];
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x250));
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x240));
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x230));
                    c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x210));
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x200), c.g[RA][0]);
                    c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x48));
                    let t = c.g[V0][0] != 0u64;
                    c.g[V1][0] = 0x1;
                    if t {
                        pc = 0x002001ec;
                        continue;
                    }
                    pc = 0x001fffa8;
                    continue;
                }
                0x001fffa8 => {
                    c.cycles += 22;
                    c.g[RA][0] = sx(0x001fffb0);
                    call_ptr(c, 0x0014dc80, 0x001fffb0)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x001fffbc);
                    c.g[A1][0] = 0x2;
                    call_ptr(c, 0x0014dd58, 0x001fffbc)?;
                    c.g[RA][0] = sx(0x001fffc4);
                    c.g[S5][0] = c.g[V0][0];
                    call_ptr(c, 0x0014dc80, 0x001fffc4)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x001fffd0);
                    c.g[A1][0] = 0x1;
                    call_ptr(c, 0x0014dd58, 0x001fffd0)?;
                    c.g[RA][0] = sx(0x001fffd8);
                    c.g[S3][0] = c.g[V0][0];
                    call_ptr(c, 0x0014dc80, 0x001fffd8)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x001fffe4);
                    c.g[A1][0] = 0u64;
                    call_ptr(c, 0x0014dd58, 0x001fffe4)?;
                    c.g[S2][0] = c.g[V0][0];
                    c.g[V1][0] = 0x5;
                    c.g[V0][0] = 0x530000;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5bc8));
                    c.g[A0][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x4a));
                    let t = c.g[A0][0] == c.g[V1][0];
                    c.g[A1][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x48));
                    if t {
                        pc = 0x00200034;
                        continue;
                    }
                    pc = 0x00200000;
                    continue;
                }
                0x00200000 => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[A0][0] as i64) < 5i64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = (c.g[A1][0] < 0x6u64) as u64;
                    if t {
                        pc = 0x00200068;
                        continue;
                    }
                    pc = 0x0020000c;
                    continue;
                }
                0x0020000c => {
                    c.cycles += 1;
                    c.g[V0][0] = ((c.g[A0][0] as i64) < 12i64) as u64;
                    pc = 0x00200010;
                    continue;
                }
                0x00200010 => {
                    c.cycles += 2;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = (c.g[A1][0] < 0x6u64) as u64;
                    if t {
                        pc = 0x00200068;
                        continue;
                    }
                    pc = 0x00200018;
                    continue;
                }
                0x00200018 => {
                    c.cycles += 2;
                    c.g[RA][0] = sx(0x00200020);
                    call_ptr(c, 0x001fd190, 0x00200020)?;
                    pc = 0x00200020;
                    continue;
                }
                0x00200020 => {
                    c.cycles += 5;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x0020002c);
                    c.g[A0][0] = c.g[SP][0];
                    call_ptr(c, 0x002c2508, 0x0020002c)?;
                    let t = true;
                    c.g[A0][0] = 0x470000;
                    if t {
                        pc = 0x00200124;
                        continue;
                    }
                    pc = 0x00200034;
                    continue;
                }
                0x00200034 => {
                    c.cycles += 13;
                    c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0xd0));
                    c.g[A1][0] = 0x470000;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0xae0));
                    c.g[RA][0] = sx(0x00200048);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x002c2540, 0x00200048)?;
                    c.g[RA][0] = sx(0x00200050);
                    c.g[A0][0] = c.g[S2][0];
                    call_ptr(c, 0x00144bf0, 0x00200050)?;
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A2][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x00200060);
                    c.g[A0][0] = c.g[SP][0];
                    call_ptr(c, 0x002c26d0, 0x00200060)?;
                    let t = true;
                    c.g[A0][0] = 0x470000;
                    if t {
                        pc = 0x00200124;
                        continue;
                    }
                    pc = 0x00200068;
                    continue;
                }
                0x00200068 => {
                    c.cycles += 2;
                    let t = c.g[V0][0] == 0u64;
                    c.g[S1][0] = 0u64;
                    if t {
                        pc = 0x002000f0;
                        continue;
                    }
                    pc = 0x00200070;
                    continue;
                }
                0x00200070 => {
                    c.cycles += 7;
                    c.g[V0][0] = 0x470000;
                    c.g[V1][0] = sx((c.g[A1][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4a0));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                    let t = c.g[A0][0] as u32;

                    pc = t;
                    continue;
                }
                0x0020008c => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.g[A0][0] = 0x470000;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1ab8));
                    if t {
                        pc = 0x002000c8;
                        continue;
                    }
                    pc = 0x0020009c;
                    continue;
                }
                0x0020009c => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.g[A0][0] = 0x470000;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1c68));
                    if t {
                        pc = 0x002000c8;
                        continue;
                    }
                    pc = 0x002000ac;
                    continue;
                }
                0x002000ac => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.g[A0][0] = 0x470000;
                    let t = true;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1c58));
                    if t {
                        pc = 0x002000c8;
                        continue;
                    }
                    pc = 0x002000bc;
                    continue;
                }
                0x002000bc => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.g[A0][0] = 0x470000;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1c40));
                    pc = 0x002000c8;
                    continue;
                }
                0x002000c8 => {
                    c.cycles += 10;
                    c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                    c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                    c.g[RA][0] = sx(0x002000dc);
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    call_ptr(c, 0x00317670, 0x002000dc)?;
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                    c.g[A0][0] = c.g[S0][0];
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x002000ec);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, t, 0x002000ec)?;
                    c.g[S1][0] = c.g[V0][0];
                    pc = 0x002000f0;
                    continue;
                }
                0x002000f0 => {
                    c.cycles += 13;
                    c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0xd0));
                    c.g[A1][0] = 0x4a0000;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2400));
                    c.g[RA][0] = sx(0x00200104);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x002c2540, 0x00200104)?;
                    c.g[RA][0] = sx(0x0020010c);
                    c.g[A0][0] = c.g[S2][0];
                    call_ptr(c, 0x00144bf0, 0x0020010c)?;
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A2][0] = c.g[V0][0];
                    c.g[A3][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x00200120);
                    c.g[A0][0] = c.g[SP][0];
                    call_ptr(c, 0x002c26d0, 0x00200120)?;
                    c.g[A0][0] = 0x470000;
                    pc = 0x00200124;
                    continue;
                }
                0x00200124 => {
                    c.cycles += 50;
                    c.g[RA][0] = sx(0x0020012c);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x468));
                    call_ptr(c, 0x00317670, 0x0020012c)?;
                    c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x40));
                    c.g[RA][0] = sx(0x00200138);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x00200138)?;
                    c.g[A1][0] = c.g[SP][0];
                    c.g[RA][0] = sx(0x00200144);
                    c.g[A0][0] = c.g[V0][0];
                    call_ptr(c, 0x003a0e90, 0x00200144)?;
                    c.g[A0][0] = c.g[S3][0];
                    c.g[RA][0] = sx(0x00200150);
                    c.g[A1][0] = 0u64;
                    call_ptr(c, 0x00146e98, 0x00200150)?;
                    c.g[A0][0] = 0x4a0000;
                    c.g[S1][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x00200160);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2588));
                    call_ptr(c, 0x00317670, 0x00200160)?;
                    c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x40));
                    c.g[RA][0] = sx(0x0020016c);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039d860, 0x0020016c)?;
                    c.g[S0][0] = 0x4a0000;
                    c.g[S2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x1c0));
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x2590));
                    c.g[A1][0] = c.g[S1][0];
                    c.g[S3][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x00200188);
                    c.g[A0][0] = c.g[S5][0];
                    call_ptr(c, 0x0014ee58, 0x00200188)?;
                    c.g[A2][0] = c.g[V0][0];
                    c.g[A1][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x00200198);
                    c.g[A0][0] = c.g[S2][0];
                    call_ptr(c, 0x002c27c0, 0x00200198)?;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.g[A0][0] = 0x470000;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x480));
                    c.g[S0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                    c.g[S1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                    c.g[RA][0] = sx(0x002001b8);
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    call_ptr(c, 0x00317670, 0x002001b8)?;
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                    c.g[A0][0] = c.g[S0][0];
                    let t = c.g[V1][0] as u32;
                    c.g[RA][0] = sx(0x002001c8);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, t, 0x002001c8)?;
                    c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0xf0));
                    c.g[A1][0] = c.g[V0][0];
                    c.g[A0][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x002001dc);
                    c.g[A2][0] = c.g[S2][0];
                    call_ptr(c, 0x002c26d0, 0x002001dc)?;
                    c.g[A0][0] = c.g[S3][0];
                    c.g[RA][0] = sx(0x002001e8);
                    c.g[A1][0] = c.g[S0][0];
                    call_ptr(c, 0x003a0e90, 0x002001e8)?;
                    c.g[V1][0] = 0x1;
                    pc = 0x002001ec;
                    continue;
                }
                0x002001ec => {
                    c.cycles += 11;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x260));
                    c.sw((c.g[S4][0] as u32).wrapping_add(0x48), c.g[V1][0]);
                    c.g[V0][0] = 0u64;
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x250));
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x240));
                    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x230));
                    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x220));
                    c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x210));
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x200));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x270));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x001fff78, 0x00200218);
                    return Ok(());
                }
            }
        }
    });
}

/// `void OVScreenRivalPre_OnWidgetAction(OVScreenRivalPre* self, widget, event)`
/// (0x1fdb88): on the "next" event (5) with a widget, set the
/// overlay status to 3, mark the game state's overlay done, start the
/// no-start-gate audio event and pop the UI state stack once.
pub fn ov_screen_rival_pre_on_widget_action(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let no_widget = c.g[A1][0] == 0;
        c.g[S0][0] = c.g[A0][0];
        let mut run_tail = false;
        if !no_widget {
            c.cycles += 3;
            c.g[V0][0] = 0x5;
            if c.g[A2][0] == c.g[V0][0] {
                c.cycles += 14;
                c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                c.g[V0][0] = 0x3;
                c.sw((c.g[GP][0] as u32).wrapping_sub(0x9f4), c.g[V0][0]);
                c.g[A0][0] = 0x1;
                c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x84));
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x200));
                c.sw((c.g[V1][0] as u32).wrapping_add(0x8), c.g[A0][0]);
                call_ptr(c, 0x0028_b180, 0x001f_dbc8)?;
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x0028_72a8, 0x001f_dbd0)?;
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                c.g[A1][0] = 0x1;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                call_ptr(c, 0x0039_f190, 0x001f_dbe0)?;
                run_tail = true;
            }
        } else {
            run_tail = true;
        }
        if run_tail {
            c.cycles += 1;
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void OVScreenSaveReplay_Shutdown(OVScreenSaveReplay* self)` (0x1fde60):
/// when the screen's +0x1c8 flag is set, clear it through `0x270ed8`; then
/// drop the replay-state table entry (`gp-0x69c`), clear the replay slot at
/// `gp-0x6a0`, and run the two base shutdowns (`0x1d58b8`, `0x26f8a0`).
#[allow(dead_code)]
pub fn ov_screen_save_replay_shutdown_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        c.g[V1][0] = 0x1;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c8));
        let not_set = c.g[V0][0] != c.g[V1][0];
        c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x69c));
        if !not_set {
            c.cycles += 6;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
            c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x28));
            call_ptr(c, 0x0027_0ed8, 0x001f_de90)?;
            c.sw((c.g[S0][0] as u32).wrapping_add(0x1c8), 0u64);
            c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x69c));
        }
        c.cycles += 26;
        c.g[V0][0] = 0x530000;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x66e8));
        c.g[A2][0] = 0x440000;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x1));
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x28f0));
        c.g[V1][0] = sx((c.g[A1][0] as u32) << 2);
        c.g[A0][0] = c.g[S0][0];
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.sw((c.g[GP][0] as u32).wrapping_sub(0x69c), c.g[A1][0]);
        c.g[V0][0] = c.lw(c.g[V1][0] as u32);
        c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A2][0] as u32));
        c.g[V1][0] = c.lw(c.g[V0][0] as u32);
        c.sw((c.g[GP][0] as u32).wrapping_sub(0x6a0), c.g[V1][0]);
        call_ptr(c, 0x001d_58b8, 0x001f_ded4)?;
        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
        c.g[A1][0] = 0u64;
        c.g[A2][0] = 0u64;
        c.g[A3][0] = 0u64;
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x84));
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x28));
        call_ptr(c, 0x0026_f8a0, 0x001f_def0)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The `cGame_exit(game, 0)` tail of `OVScreenInGameError_HandleSelect`
/// (0x1fb360 return): `a1 = 0`, `a0 = world->game`, call 0x230488.
pub(crate) fn in_game_error_exit_call(c: &mut Cpu) -> Flow {
    c.cycles += 3;
    c.g[A1][0] = 0u64;
    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
    call_ptr(c, 0x0023_0488, 0x001f_b360)
}

/// `void OVScreenInGameError_HandleSelect(OVScreenInGameError* self, item)`
/// (0x1fb2b8): hide the error screen's options (`+0x1c` keeps its low bits
/// and sets `0x780`), pop the UI state stack, then act on the item kind
/// (`item+0x18`): kind 4 calls 0x12b340 and pops the table entry at
/// `gp-0x69c`, other kinds exit the game (`cGame_exit`).
pub fn ov_screen_in_game_error_handle_select(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        c.g[A1][0] = 0x1;
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
        call_ptr(c, 0x0039_f190, 0x001f_b2e0)?;
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x1c));
        c.g[V0][0] = 0xffff_ffff_ffff_c0ff;
        c.g[V1][0] = c.g[V1][0] & c.g[V0][0];
        c.g[V1][0] = c.g[V1][0] | 0x780;
        c.sw((c.g[S0][0] as u32).wrapping_add(0x1c), c.g[V1][0]);
        c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x18));
        let kind_zero = c.g[V1][0] == 0u64;
        c.g[V0][0] = 0x4;
        if kind_zero {
            c.cycles += 1;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            in_game_error_exit_call(c)?;
        } else {
            c.cycles += 2;
            let kind_not_four = c.g[V1][0] != c.g[V0][0];
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            if kind_not_four {
                in_game_error_exit_call(c)?;
            } else {
                c.cycles += 18;
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x84));
                c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                call_ptr(c, 0x0012_b340, 0x001f_b314)?;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x69c));
                c.g[A0][0] = 0x530000;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x66e8));
                c.g[A1][0] = 0x440000;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x28f0));
                c.g[V1][0] = sx((c.g[V0][0] as u32) << 2);
                c.sw((c.g[GP][0] as u32).wrapping_sub(0x69c), c.g[V0][0]);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
                c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                c.sw((c.g[GP][0] as u32).wrapping_sub(0x6a0), c.g[V1][0]);
            }
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void OVScreenBEOptions_Update(OVScreenBEOptions* self)` (0x1fa9e0): run the
/// overlay update, then, while the current option table (`gp-0x204`) has
/// its two flags clear, play the screen's frame named by the hash of
/// `D_0046E050` (`cUIScreen_playFrame`) and pop the UI state stack.
pub fn ov_screen_be_options_update_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0020_e900, 0x001f_a9f4)?;
        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x204));
        if c.g[V1][0] != 0 {
            c.cycles += 3;
            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x68));
            let zero_68 = c.g[V0][0] == 0u64;
            c.g[A0][0] = 0u64;
            let flags_clear = if zero_68 {
                true
            } else {
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x64));
                c.g[V0][0] == 0u64
            };
            if flags_clear {
                c.cycles += 1;
                c.g[A0][0] = 0x1;
            }
            c.cycles += 2;
            if c.g[A0][0] != 0u64 {
                c.cycles += 10;
                c.g[A0][0] = 0x470000;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1fb0));
                call_ptr(c, 0x0031_7670, 0x001f_aa30)?;
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x0039_c778, 0x001f_aa3c)?;
                c.g[A1][0] = c.g[V0][0];
                c.g[V0][0] = 0xffff;
                let frame_found = c.g[A1][0] == c.g[V0][0];
                c.g[A1][0] = c.g[A1][0] & 0xffff;
                if !frame_found {
                    c.cycles += 3;
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x40));
                    c.g[A2][0] = 0x1;
                    call_ptr(c, 0x0039_c870, 0x001f_aa58)?;
                }
                c.cycles += 5;
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x10));
                c.g[A1][0] = 0x1;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                call_ptr(c, 0x0039_f190, 0x001f_aa68)?;
            }
            frame.restore(c, S0, 0x10);
        } else {
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

pub(crate) const REWARD_STATE_OVERLAY_INIT: Callee = Callee {
    addr: 0x0020_a380,
    f: ee::s05::f_0020a380,
};

pub(crate) const REWARD_LIST_GET_ENTRY_TEXT: Callee = Callee {
    addr: 0x001f_f7b8,
    f: ee::s05::f_001ff7b8,
};

/// Return addresses of the twelve calls of the save-replay body at 0x1fe4a8
/// (taken from its `c.g[RA][0] = sx(...)` lines, in order).
pub(crate) const SAVE_REPLAY_RETS_1FE4A8: [u32; 12] = [
    0x001f_e4dc,
    0x001f_e4e8,
    0x001f_e500,
    0x001f_e50c,
    0x001f_e518,
    0x001f_e530,
    0x001f_e53c,
    0x001f_e548,
    0x001f_e560,
    0x001f_e56c,
    0x001f_e578,
    0x001f_e590,
];

/// The same twelve return addresses of the identical body at 0x1ff170.
pub(crate) const SAVE_REPLAY_RETS_1FF170: [u32; 12] = [
    0x001f_f1a4,
    0x001f_f1b0,
    0x001f_f1c8,
    0x001f_f1d4,
    0x001f_f1e0,
    0x001f_f1f8,
    0x001f_f204,
    0x001f_f210,
    0x001f_f228,
    0x001f_f234,
    0x001f_f240,
    0x001f_f258,
];

/// `cOVState_SAVEREPLAY_hideActionButtons(this, flag)` (0x1fe4a8), the same
/// body as the record screen's at 0x1ff170: when `this->+0x22c` differs from
/// `flag`, four text lookups each followed by a find and a dispatch through
/// the found object's class; then `this->+0x22c = flag`. `rets` are the
/// return addresses of the twelve calls, in order.
pub(crate) fn save_replay_body(c: &mut Cpu, rets: &[u32; 12]) -> Flow {
    c.cycles += 10;
    let frame = Frame::open(c, 0x40);
    frame.save(c, S1, 0x20);
    frame.save(c, S2, 0x10);
    c.g[S1][0] = c.g[A0][0];
    frame.save(c, S0, 0x30);
    c.g[S2][0] = c.g[A1][0];
    frame.save_ra(c, 0);
    c.g[V0][0] = Addr(c.g[S1][0] as u32).off(0x22c).lw(c);
    let unchanged = c.g[V0][0] == c.g[S2][0];
    c.g[S0][0] = c.g[S2][0] ^ 0x1;
    if !unchanged {
        c.cycles += 49;
        c.g[A0][0] = 0x004a_0000;
        c.g[A0][0] = sx(abi::r32(c, A0).wrapping_add(0x2090));
        call_ptr(c, 0x0031_7670, rets[0])?;
        save_replay_find_dispatch(c, rets[1], rets[2])?;
        c.g[A0][0] = 0x004a_0000;
        c.g[A0][0] = sx(abi::r32(c, A0).wrapping_add(0x2568));
        call_ptr(c, 0x0031_7670, rets[3])?;
        save_replay_find_dispatch(c, rets[4], rets[5])?;
        c.g[A0][0] = 0x0047_0000;
        c.g[A0][0] = sx(abi::r32(c, A0).wrapping_add(0x188));
        call_ptr(c, 0x0031_7670, rets[6])?;
        save_replay_find_dispatch(c, rets[7], rets[8])?;
        c.g[A0][0] = 0x004a_0000;
        c.g[A0][0] = sx(abi::r32(c, A0).wrapping_add(0x20a0));
        call_ptr(c, 0x0031_7670, rets[9])?;
        save_replay_find_dispatch(c, rets[10], rets[11])?;
        let flag = c.g[S2][0];
        Addr(c.g[S1][0] as u32).off(0x22c).sw(c, flag);
    }
    c.cycles += 6;
    frame.restore(c, S0, 0x30);
    frame.restore(c, S1, 0x20);
    frame.restore(c, S2, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `obj = cUIScreen_getObjectByHashName(this->+0x40)`, then the object's
/// class (`obj->+8`) gives a delta (`+0x48`) and a function (`+0x4c`), called
/// with `obj + delta` and `s0` as its second argument.
pub(crate) fn save_replay_find_dispatch(c: &mut Cpu, find_ret: u32, call_ret: u32) -> Flow {
    c.g[A0][0] = Addr(c.g[S1][0] as u32).off(0x40).lw(c);
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, 0x0039_d860, find_ret)?;
    c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x8).lw(c);
    c.g[A1][0] = c.g[S0][0];
    c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x48).lh(c);
    c.g[A2][0] = Addr(c.g[V1][0] as u32).off(0x4c).lw(c);
    let t = c.g[A2][0] as u32;
    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, t, call_ret)?;
    Ok(())
}

/// `cOVState_SAVEREPLAY_hideActionButtons` (0x1fe4a8).
pub fn ov_screen_save_replay_v37(c: &mut Cpu) {
    guest(c, |c| save_replay_body(c, &SAVE_REPLAY_RETS_1FE4A8));
}

/// `OVScreenSaveRecords_v37` (0x1ff170): the save-replay body with its own
/// return addresses.
pub fn ov_screen_save_records_v37(c: &mut Cpu) {
    guest(c, |c| save_replay_body(c, &SAVE_REPLAY_RETS_1FF170));
}

/// `OVScreenSaveRecords_Shutdown` (0x1fec40): `MemCardStateBase_Shutdown()`.
pub fn ov_screen_save_records_shutdown(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        call_ptr(c, 0x001d_58b8, 0x001f_ec50)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `OVScreenSaveRecords_v39` (0x1ff310): sets `this->+0x224` to `s0 = !a2`
/// after a lookup and two dispatches through found objects.
pub fn ov_screen_save_records_v39(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 40;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x30);
        c.g[A0][0] = 0x004a_0000;
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A2][0];
        c.g[A0][0] = sx(abi::r32(c, A0).wrapping_add(0x2090));
        c.g[S0][0] = c.g[S0][0] ^ 0x1;
        call_ptr(c, 0x0031_7670, 0x001f_f33c)?;
        c.g[A0][0] = Addr(c.g[S1][0] as u32).off(0x40).lw(c);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_d860, 0x001f_f348)?;
        c.g[A0][0] = 0x004a_0000;
        c.g[S2][0] = c.g[V0][0];
        c.g[A0][0] = sx(abi::r32(c, A0).wrapping_add(0x2568));
        call_ptr(c, 0x0031_7670, 0x001f_f358)?;
        c.g[A0][0] = Addr(c.g[S1][0] as u32).off(0x40).lw(c);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_d860, 0x001f_f364)?;
        c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x8).lw(c);
        c.g[A1][0] = c.g[S0][0];
        c.g[A0][0] = Addr(c.g[V1][0] as u32).off(0x48).lh(c);
        c.g[A2][0] = Addr(c.g[V1][0] as u32).off(0x4c).lw(c);
        let t = c.g[A2][0] as u32;
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x001f_f37c)?;
        let flag = c.g[S0][0];
        Addr(c.g[S1][0] as u32).off(0x224).sw(c, flag);
        c.g[A1][0] = c.g[S0][0];
        c.g[V0][0] = Addr(c.g[S2][0] as u32).off(0x8).lw(c);
        c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x48).lh(c);
        c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0x4c).lw(c);
        let t = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x001f_f398)?;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `OVScreenRewardList_Init(this)` (0x1ff590): the reward list's text and
/// model lookups, the overlay init, the entry text, and the two list
/// handles at `+0x9c` / `+0x40`. Returns through its three exits as the
/// generated body does (the one-cycle tail on the early exits).
pub fn ov_screen_reward_list_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        frame.save(c, S0, 0x20);
        c.g[A0][0] = 0x0047_0000;
        c.g[A0][0] = sx(abi::r32(c, A0).wrapping_add(0x3c8));
        c.g[S0][0] = Addr(c.g[S1][0] as u32).off(0x10).lw(c);
        call_ptr(c, 0x0031_7670, 0x001f_f5b4)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A3][0] = 0;
        call_ptr(c, 0x0039_7cc0, 0x001f_f5c8)?;
        let absent = c.g[V0][0] == 0;
        let found = c.g[V0][0];
        Addr(c.g[S1][0] as u32).off(0x40).sw(c, found);
        if !absent {
            c.cycles += 4;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            call_ptr(c, 0x0039_c870, 0x001f_f5e0)?;
        }

        c.cycles += 11;
        c.g[A0][0] = 0x0047_0000;
        c.g[S0][0] = Addr(c.g[S1][0] as u32).off(0x10).lw(c);
        c.g[A0][0] = sx(abi::r32(c, A0).wrapping_sub(0x1c88));
        call_ptr(c, 0x0031_7670, 0x001f_f5f0)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A3][0] = 0;
        call_ptr(c, 0x0039_7cc0, 0x001f_f604)?;
        let absent = c.g[V0][0] == 0;
        if !absent {
            c.cycles += 4;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            call_ptr(c, 0x0039_c870, 0x001f_f61c)?;
        }

        // The text and the entry block: a block that ends in the common tail.
        c.cycles += 16;
        c.g[A0][0] = c.g[S1][0];
        REWARD_STATE_OVERLAY_INIT.call(c, 0x001f_f624)?;
        let list = c.g[S1][0];
        Addr(list as u32).off(0x9c).sw(c, 0);
        c.g[A2][0] = 0xffff_ffff_ffff_ffff;
        c.g[A1][0] = 0;
        c.g[A0][0] = c.g[S1][0];
        REWARD_LIST_GET_ENTRY_TEXT.call(c, 0x001f_f638)?;
        c.g[A0][0] = 0x0047_0000;
        c.g[A0][0] = sx(abi::r32(c, A0).wrapping_sub(0x17e8));
        call_ptr(c, 0x0031_7670, 0x001f_f644)?;
        c.g[A0][0] = Addr(c.g[S1][0] as u32).off(0x40).lw(c);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_d860, 0x001f_f650)?;
        c.g[A2][0] = c.g[V0][0];
        let no_object = c.g[A2][0] == 0;
        c.g[V1][0] = 0xffff_ffff_ffff_ff7f;
        // Every path but the direct call below ends the block with one cycle
        // and `a1 = 0x4a0000`.
        let mut through_block_end = true;
        if !no_object {
            c.cycles += 8;
            c.g[V0][0] = Addr(c.g[A2][0] as u32).off(0x14).lw(c);
            c.g[A0][0] = Addr(c.g[A2][0] as u32).off(0x96).lbu(c);
            c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
            let flags = c.g[V0][0];
            Addr(c.g[A2][0] as u32).off(0x14).sw(c, flags);
            c.g[V1][0] = Addr(c.g[S1][0] as u32).off(0x9c).lw(c);
            c.g[A0][0] = ((c.g[A0][0] as i64) < (c.g[V1][0] as i64)) as u64;
            if c.g[A0][0] == 0 {
                c.g[V0][0] = Addr(c.g[A2][0] as u32).off(0x90).lw(c);
                c.cycles += 2;
                c.g[V0][0] |= 0x8;
                let mode = c.g[V0][0];
                Addr(c.g[A2][0] as u32).off(0x90).sw(c, mode);
            } else {
                through_block_end = false;
                c.cycles += 5;
                c.g[A1][0] = Addr(c.g[S1][0] as u32).off(0x9c).lbu(c);
                c.g[A0][0] = c.g[A2][0];
                call_ptr(c, 0x0039_b760, 0x001f_f688)?;
                c.g[A1][0] = 0x004a_0000;
            }
        }
        if through_block_end {
            c.cycles += 1;
            c.g[A1][0] = 0x004a_0000;
        }

        c.cycles += 12;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = sx(abi::r32(c, A1).wrapping_add(0x2570));
        call_ptr(c, 0x0039_e8b8, 0x001f_f6a8)?;
        c.g[A1][0] = 0x004a_0000;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = sx(abi::r32(c, A1).wrapping_add(0x2578));
        call_ptr(c, 0x0039_e8b8, 0x001f_f6b8)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

pub(crate) const OV_PAUSE_VTBL_OPTIONS: u32 = 0x0047_2b88;

pub(crate) const OV_PAUSE_VTBL_ONLINE_ERROR: u32 = 0x0047_2ab0;

/// `void OVScreenBEOptions_dtor(OVScreenBEOptions *self)` (0x221bf0): sets
/// the base vtable at `+8` and runs the UIState destructor.
pub fn ov_screen_be_options_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V1][0] = c.g[A0][0];
        c.g[V0][0] = sx(OV_STATE_VTBL_BASE);
        c.sw((c.g[V1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
        call_ptr(c, 0x0039_e390, 0x0022_1c0c)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `OVScreenBEOptions *OVScreenBEOptions_ctor(OVScreenBEOptions *self, int a1,
/// int overlayId)` (0x221c18): runs the UIState constructor with the caller's
/// `a1`, stores the overlay id at `+0x98`, sets the options vtable and the
/// chat buffer's initial word. Returns `self` in `v0`.
pub fn ov_screen_be_options_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x30);
        frame.save(c, 16, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, 17, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A2][0];
        call_ptr(c, 0x0039_e2a0, 0x0022_1c34)?;
        let this = c.g[S0][0] as u32;
        c.sw(this.wrapping_add(0x98), c.g[S1][0]);
        c.sw(this.wrapping_add(0x48), 0);
        c.sw(this.wrapping_add(0x8), sx(OV_STATE_VTBL_BASE));
        c.g[A1][0] = OV_STR_SP_B_HI as u64;
        c.sw(this.wrapping_add(0x4c), 0);
        c.sw(this.wrapping_add(0x54), 0);
        c.g[V1][0] = sx(OV_PAUSE_VTBL_OPTIONS);
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, 17, 0x10);
        c.g[A0][0] = c.lhu((c.g[A1][0] as u32).wrapping_add(0x2088));
        c.sw(this.wrapping_add(0x8), c.g[V1][0]);
        c.sh(this.wrapping_add(0x58), c.g[A0][0]);
        frame.restore(c, 16, 0x20);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `OVScreenInGameError *OVScreenInGameError_ctor(OVScreenInGameError *self)`
/// (0x221c80): runs the UIState constructor with a null second argument,
/// clears the state fields, sets the online-error vtable and the chat
/// buffer's initial word. Returns `self` in `v0`.
pub fn ov_screen_in_game_error_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0;
        frame.save(c, 16, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0039_e2a0, 0x0022_1c98)?;
        let this = c.g[S0][0] as u32;
        c.sw(this.wrapping_add(0x98), 0);
        c.sw(this.wrapping_add(0x48), 0);
        c.sw(this.wrapping_add(0x8), sx(OV_STATE_VTBL_BASE));
        c.g[A1][0] = OV_STR_SP_B_HI as u64;
        c.sw(this.wrapping_add(0x4c), 0);
        c.sw(this.wrapping_add(0x54), 0);
        c.g[V1][0] = sx(OV_PAUSE_VTBL_ONLINE_ERROR);
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        c.g[A0][0] = c.lhu((c.g[A1][0] as u32).wrapping_add(0x2088));
        c.sw(this.wrapping_add(0x8), c.g[V1][0]);
        c.sh(this.wrapping_add(0x58), c.g[A0][0]);
        frame.restore(c, 16, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `OVScreenInGameError *OVScreenInGameError_ctor_2(OVScreenInGameError *self,
/// int a1, int overlayId)` (0x221ce0): as `OVScreenInGameError_ctor`, but the
/// UIState constructor gets the caller's `a1`, the overlay id is stored at
/// `+0x98`, and the aux info flag (`+0xa0`) is set to 1 with the ping start
/// tick (`+0x9c`) cleared. Returns `self` in `v0`.
pub fn ov_screen_in_game_error_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 29;
        let frame = Frame::open(c, 0x30);
        frame.save(c, 16, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, 17, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A2][0];
        call_ptr(c, 0x0039_e2a0, 0x0022_1cfc)?;
        let this = c.g[S0][0] as u32;
        c.sw(this.wrapping_add(0x98), c.g[S1][0]);
        c.sw(this.wrapping_add(0x48), 0);
        c.sw(this.wrapping_add(0x8), sx(OV_STATE_VTBL_BASE));
        c.g[A2][0] = OV_STR_SP_B_HI as u64;
        c.sw(this.wrapping_add(0x4c), 0);
        c.sw(this.wrapping_add(0x54), 0);
        c.g[V1][0] = sx(OV_PAUSE_VTBL_ONLINE_ERROR);
        c.g[A1][0] = 0x1;
        c.g[V0][0] = c.g[S0][0];
        c.g[A0][0] = c.lhu((c.g[A2][0] as u32).wrapping_add(0x2088));
        c.sw(this.wrapping_add(0x8), c.g[V1][0]);
        c.sh(this.wrapping_add(0x58), c.g[A0][0]);
        c.sw(this.wrapping_add(0xa0), c.g[A1][0]);
        c.sw(this.wrapping_add(0x9c), 0);
        frame.restore(c, 16, 0x20);
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The destructor shape shared by the `OVScreen*` classes: a 0x10 frame, the
/// vtable word `0x470000 + vt_off` stored at `this + 8`, then a tail call of
/// the base destructor `base_dtor` returning to `ret`. The frame closes after it.
pub(crate) fn oc1013_screen_dtor(c: &mut Cpu, vt_off: u32, base_dtor: u32, ret: u32) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0x0047_0000;
        frame.save_ra(c, 0);
        c.g[V1][0] = c.g[A0][0];
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(vt_off));
        let vt = c.g[V0][0];
        Addr((c.g[V1][0] as u32).wrapping_add(0x8)).sw(c, vt);
        call_ptr(c, base_dtor, ret)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The constructor shape of the three `OVScreen*Pre` classes: a 0x30 frame
/// keeping `this` in s0 and the third argument in s1, the `UIState` base
/// constructor (0x39e2a0) returning to `call_ret`, then the vtable words
/// (`0x470000 + 0x4e08` at +8, `0x470000 + vt_off` after the reload) and the
/// zeroed fields. `this + 0x58` gets the halfword at `0x4a0000 + 0x2088`.
pub(crate) fn oc1013_screen_ctor(c: &mut Cpu, call_ret: u32, vt_off: u32) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A2][0];
        call_ptr(c, 0x0039_e2a0, call_ret)?;
        let this = c.g[S0][0] as u32;
        c.g[V1][0] = 0x0047_0000;
        let s1 = c.g[S1][0];
        Addr(this.wrapping_add(0x98)).sw(c, s1);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4e08));
        Addr(this.wrapping_add(0x48)).sw(c, 0);
        let vt = c.g[V1][0];
        Addr(this.wrapping_add(0x8)).sw(c, vt);
        c.g[A1][0] = 0x004a_0000;
        Addr(this.wrapping_add(0x4c)).sw(c, 0);
        c.g[V1][0] = 0x0047_0000;
        Addr(this.wrapping_add(0x54)).sw(c, 0);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(vt_off));
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x10);
        c.g[A0][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x2088)).lhu(c);
        let vt2 = c.g[V1][0];
        Addr(this.wrapping_add(0x8)).sw(c, vt2);
        let a0 = c.g[A0][0];
        Addr(this.wrapping_add(0x58)).sh(c, a0);
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `OVScreenInGameError_dtor` (0x221d58): vtable `0x474e08`, then the
/// `UIState` base destructor (0x39e390).
pub fn ov_screen_in_game_error_dtor(c: &mut Cpu) {
    oc1013_screen_dtor(c, 0x4e08, 0x0039_e390, 0x0022_1d74)
}

/// `OVScreenRacePre_ctor` (0x221e18): the `UIState` constructor, then the
/// race-pre screen fields and vtable `0x472850`.
pub fn ov_screen_race_pre_ctor(c: &mut Cpu) {
    oc1013_screen_ctor(c, 0x0022_1e34, 0x2850)
}

/// `OVScreenFreestylePre_dtor` (0x221e80): vtable `0x474e08`, then the
/// `UIState` base destructor.
pub fn ov_screen_freestyle_pre_dtor(c: &mut Cpu) {
    oc1013_screen_dtor(c, 0x4e08, 0x0039_e390, 0x0022_1e9c)
}

/// `OVScreenFreestylePre_ctor` (0x221ea8): as the race-pre constructor, with
/// vtable `0x472778`.
pub fn ov_screen_freestyle_pre_ctor(c: &mut Cpu) {
    oc1013_screen_ctor(c, 0x0022_1ec4, 0x2778)
}

/// `OVScreenMultiFreestylePre_dtor` (0x221f10): vtable `0x474e08`, then the
/// `UIState` base destructor.
pub fn ov_screen_multi_freestyle_pre_dtor(c: &mut Cpu) {
    oc1013_screen_dtor(c, 0x4e08, 0x0039_e390, 0x0022_1f2c)
}

/// `OVScreenMultiFreestylePre_ctor` (0x221f38): as the race-pre constructor,
/// with vtable `0x4726a0`.
pub fn ov_screen_multi_freestyle_pre_ctor(c: &mut Cpu) {
    oc1013_screen_ctor(c, 0x0022_1f54, 0x26a0)
}

/// `UIState_ctor` (0x39e2a0): the base constructor every `OVState` runs first.
pub(crate) const OV_UI_STATE_CTOR: u32 = 0x0039_e2a0;

/// `UIState_dtor` (0x39e390): the base destructor the `OVState` destructors end in.
pub(crate) const OV_UI_STATE_DTOR: u32 = 0x0039_e390;

/// `MemCardStateBase_ctor` (0x1d53b0), the base of the save-screen overlay states.
pub(crate) const OV_MEMCARD_CTOR: u32 = 0x001d_53b0;

/// `MemCardStateBase_dtor` (0x1d5428).
pub(crate) const OV_MEMCARD_DTOR: u32 = 0x001d_5428;

/// The game-image data page (`0x4a0000`) holding the chat buffer's first
/// halfword at +0x2088.
pub(crate) const OV_DATA_PAGE: u32 = 0x004a_0000;

/// The stores every constructor of this group makes after its base constructor
/// (`this` is `s0`): the overlay id at +0x98, the transition, pause-menu and
/// PDA flags (+0x48, +0x4c, +0x54) cleared, the base vtable at +0x8 and then
/// `vptr` over it, and the chat buffer's first halfword at +0x58. Leaves `v0` =
/// this, `v1` = `vptr`, `a1` = the data page and `a0` = the halfword, as the
/// generated body does.
pub(crate) fn ov_base_fill_fields(c: &mut Cpu, ov_id: u64, vptr: u32) {
    let obj = c.g[S0][0] as u32;
    c.sw(obj.wrapping_add(0x98), ov_id);
    c.sw(obj.wrapping_add(0x48), 0);
    c.sw(obj.wrapping_add(0x8), 0x0047_4e08);
    c.sw(obj.wrapping_add(0x4c), 0);
    c.sw(obj.wrapping_add(0x54), 0);
    c.g[V1][0] = vptr as u64;
    c.sw(obj.wrapping_add(0x8), c.g[V1][0]);
    c.g[V0][0] = c.g[S0][0];
    c.g[A1][0] = OV_DATA_PAGE as u64;
    c.g[A0][0] = c.lhu(OV_DATA_PAGE.wrapping_add(0x2088));
    c.sh(obj.wrapping_add(0x58), c.g[A0][0]);
}

/// A one-call destructor wrapper: the vtable word `vtbl` at `this+0x8`, then
/// `callee(this)` returning to `ret` (the callee's unwind returns at once, as
/// the generated code does). `this` (a0) is passed through unchanged.
pub(crate) fn ov_wrapper_dtor_body(
    c: &mut Cpu,
    vtbl: u32,
    callee: u32,
    ret: u32,
) -> crate::call::Flow {
    c.cycles += 10;
    let f = Frame::open(c, 0x10);
    f.save_ra(c, 0);
    c.g[V1][0] = c.g[A0][0];
    c.g[V0][0] = vtbl as u64;
    let this = c.g[V1][0] as u32;
    c.sw(this.wrapping_add(0x8), c.g[V0][0]);
    call_ptr(c, callee, ret)?;
    f.restore_ra(c, 0);
    f.close(c);
    Ok(())
}

/// The save-screen overlay constructor that takes `(this, 0, 0)` into the
/// `MemCardStateBase` constructor and then stores its vtable word at +0x8.
pub(crate) fn ov_memcard_ctor_2_body(c: &mut Cpu, vtbl: u32, ret: u32) -> crate::call::Flow {
    c.cycles += 15;
    let f = Frame::open(c, 0x20);
    c.g[A1][0] = 0;
    f.save(c, 16, 0x10);
    c.g[A2][0] = 0;
    f.save_ra(c, 0);
    c.g[S0][0] = c.g[A0][0];
    call_ptr(c, OV_MEMCARD_CTOR, ret)?;
    c.g[V1][0] = vtbl as u64;
    c.g[V0][0] = c.g[S0][0];
    f.restore_ra(c, 0);
    c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V1][0]);
    f.restore(c, 16, 0x10);
    f.close(c);
    Ok(())
}

/// `OVScreenRivalPre_dtor` (0x221fa0): the rival-preview overlay destructor.
/// Stores the base vtable and runs `UIState_dtor`.
pub fn ov_screen_rival_pre_dtor(c: &mut Cpu) {
    guest(c, |c| {
        ov_wrapper_dtor_body(c, 0x0047_4e08, OV_UI_STATE_DTOR, 0x0022_1fbc)
    });
}

/// `OVScreenRivalPre_ctor` (0x221fc8): constructs the rival-preview overlay
/// state in `a0`: `UIState_ctor`, then the stores with overlay id `a2` and the
/// rival-preview vtable.
pub fn ov_screen_rival_pre_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let f = Frame::open(c, 0x30);
        f.save(c, 16, 0x20);
        c.g[S0][0] = c.g[A0][0];
        f.save(c, 17, 0x10);
        f.save_ra(c, 0);
        c.g[S1][0] = c.g[A2][0];
        call_ptr(c, OV_UI_STATE_CTOR, 0x0022_1fe4)?;
        let ov_id = c.g[S1][0];
        ov_base_fill_fields(c, ov_id, 0x0047_25c8);
        f.restore(c, 17, 0x10);
        f.restore(c, 16, 0x20);
        f.restore_ra(c, 0);
        f.close(c);
        Ok(())
    })
}

/// `OVScreenSaveReplay_dtor` (0x222030): the save-replay overlay destructor.
/// Stores the save-screen vtable and runs `MemCardStateBase_dtor`.
pub fn ov_screen_save_replay_dtor(c: &mut Cpu) {
    guest(c, |c| {
        ov_wrapper_dtor_body(c, 0x0047_2440, OV_MEMCARD_DTOR, 0x0022_204c)
    });
}

/// `OVScreenSaveReplay_ctor_2` (0x222058): the save-replay overlay constructor
/// (`MemCardStateBase_ctor(this, 0, 0)`, then the vtable word).
pub fn ov_screen_save_replay_ctor_2(c: &mut Cpu) {
    guest(c, |c| ov_memcard_ctor_2_body(c, 0x0047_2440, 0x0022_2074));
}

/// `OVScreenSaveRecords_dtor` (0x222098): the save-records overlay destructor.
/// Stores its vtable and runs `MemCardStateBase_dtor`.
pub fn ov_screen_save_records_dtor(c: &mut Cpu) {
    guest(c, |c| {
        ov_wrapper_dtor_body(c, 0x0047_22b8, OV_MEMCARD_DTOR, 0x0022_20b4)
    });
}

/// `OVScreenSaveRecords_ctor_2` (0x2220c0): the save-records overlay
/// constructor (`MemCardStateBase_ctor(this, 0, 0)`, then its vtable word).
pub fn ov_screen_save_records_ctor_2(c: &mut Cpu) {
    guest(c, |c| ov_memcard_ctor_2_body(c, 0x0047_22b8, 0x0022_20dc));
}

pub(crate) const OC1015_VT_REWARDS: u32 = OC1015_DATA + 0x2108;

/// `OVScreenRewardList_dtor` (0x2221f0): the reward list's destructor, the
/// same as the popup's.
pub fn ov_screen_reward_list_dtor(c: &mut Cpu) {
    guest(c, |c| {
        oc1015_dtor(c, OC1015_VT_BASE, OC1015_UI_DTOR, 0x0022_220c)
    });
}

/// `OVScreenRewardList_ctor` (0x222218): constructs the reward list overlay
/// state in `a0` with the state word from `a2`, vtable `cOVState_REWARDS`.
pub fn ov_screen_reward_list_ctor(c: &mut Cpu) {
    guest(c, |c| oc1015_ctor2(c, 0x0022_2234, OC1015_VT_REWARDS));
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    pub(crate) use crate::game::fe::uitemplatemap::tests::oc797_install_fakes;

    use crate::difftest::fakes::machine;

    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::FAKE;

    use crate::difftest::scramble;

    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;

    pub(crate) use crate::game::ai::control::handplantcontrol::tests::put;
    pub(crate) use crate::game::ai::rider::tests::clear_2 as clear_3;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::tests::gpf;
    pub(crate) use crate::game::dirtysock::tagsunk::tests::be_library_2;
    pub(crate) use crate::game::dirtysock::tagsunk::tests::world_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::alloc_base_4 as alloc_base;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_11 as base_3;

    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_2 as be_library;
    pub(crate) use crate::game::fe::feasyncfile::tests::clear;

    pub(crate) use crate::game::fe::feasyncfile::tests::fake_small;
    pub(crate) use crate::game::fe::feasyncfile::tests::game_globals;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_2 as globals;

    pub(crate) use crate::game::fe::feasyncfile::tests::w_4 as w_2;

    pub(crate) use crate::game::fe::fepopup::tests::base_14 as base_8;
    pub(crate) use crate::game::fe::fepopup::tests::check_all_3 as check_all;

    pub(crate) use crate::game::fe::festatebonusmaterial::tests::str_hash;
    pub(crate) use crate::game::fe::festatecharequipdetail::tests::world;

    pub(crate) use crate::game::fe::festateoptions::tests::setup;
    pub(crate) use crate::game::fe::festateoptions::tests::AUX;
    pub(crate) use crate::game::fe::festaterewards::tests::base_4 as base;
    pub(crate) use crate::game::fe::festaterewards::tests::base_9 as base_6;

    pub(crate) use crate::game::fe::festaterewards::tests::slot_2 as slot;
    pub(crate) use crate::game::fe::festaterewards::tests::strict;

    pub(crate) use crate::game::fe::ovstatemission::tests::str_hash as str_hash_2;
    pub(crate) use crate::game::fe::ovstateprofile::tests::hash_object;
    pub(crate) use crate::game::main::gamemode::tests::be_library as be_library_3;
    pub(crate) use crate::game::main::gamemode::tests::SEEDS;
    pub(crate) use crate::game::main::gamemode::tests::SEEDS_2 as SEEDS_3;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::base_14 as base_7;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::check3;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::w;

    pub(crate) use crate::game::scripter::ssxscriptengine::tests::LEAF;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::SEEDS_10 as SEEDS_2;

    pub(crate) use crate::game::sound::soundsys::tests::base_9 as base_5;

    pub(crate) use crate::game::wscript::wscriptman::tests::base_6 as base_2;
    pub(crate) use crate::game::wscript::wscriptman::tests::clear_5 as clear_2;

    use ssx3_ee as ee;
    use ssx3_ee_rt::funcs::GuestFn;

    #[test]
    fn mc_manager_set_mode_matches() {
        check(
            ssx3_ee::s06::f_0023d570,
            mc_manager_set_mode,
            300,
            &|c, seed| {
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
            },
        );
    }

    #[test]
    fn ov_screen_reward_list_on_activate_matches() {
        check(
            ee::s05::f_001ff6d0__generated,
            ov_screen_reward_list_on_activate,
            20,
            &|c, seed| {
                machine(c, seed, 0x1ff6d0, 6);
                globals(c);
                clear(c, DATA, 0x400);
                c.g[A0][0] = DATA as u64;
                let _ = seed;
            },
        );
    }

    #[test]
    fn ov_multi_freestyle_pre_on_deactivate_matches() {
        check(
            ssx3_ee::s04::f_001fd118__generated,
            ov_multi_freestyle_pre_on_deactivate,
            SEEDS,
            &|c, seed| {
                let _r = machine(c, seed, 0xfd118, 1);
                be_library(c);
                world(c);
                c.bus.write32(GAME_GP + 0x410, DATA + 0x2000);
                c.bus.write32(
                    GAME_GP.wrapping_sub(0x204),
                    if seed % 2 == 0 { DATA + 0x2400 } else { 0 },
                );
                c.g[A0][0] = sx(DATA + 0x300);
            },
        );
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
        ov_pre_update_test(
            ssx3_ee::s04::f_001fbce0__generated,
            ov_screen_race_pre_update,
            0x28,
        );
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
        save_screen_ctor_test(
            ssx3_ee::s05::f_001fdbf0__generated,
            ov_screen_save_replay_ctor,
            0x2b,
        );
    }

    #[test]
    fn ov_screen_save_records_ctor_matches() {
        save_screen_ctor_test(
            ssx3_ee::s05::f_001fe968__generated,
            ov_screen_save_records_ctor,
            0x2c,
        );
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
        close_top_state_once_test(
            ssx3_ee::s05::f_001fdf00__generated,
            ov_screen_save_replay_v33,
            0x2d,
        );
    }

    #[test]
    fn ov_screen_save_records_v33_matches() {
        close_top_state_once_test(
            ssx3_ee::s05::f_001fec60__generated,
            ov_screen_save_records_v33,
            0x2e,
        );
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
        check(
            ee::s04::f_001faaf0__generated,
            ov_screen_be_options_v22,
            200,
            &|c, seed| {
                setup(c, seed, 33);
                c.g[A2][0] = [4u64, 0, 9][(seed % 3) as usize];
            },
        );
    }

    #[test]
    fn ov_screen_be_options_v21_matches() {
        check(
            ee::s04::f_001fab08__generated,
            ov_screen_be_options_v21,
            200,
            &|c, seed| {
                setup(c, seed, 34);
                c.g[A2][0] = if seed % 2 == 0 { 0 } else { seed as u64 };
            },
        );
    }

    #[test]
    fn ov_screen_race_pre_on_deactivate_matches() {
        check(
            ee::s04::f_001fbcc8__generated,
            ov_screen_race_pre_on_deactivate,
            200,
            &|c, seed| {
                let mut r = setup(c, seed, 35);
                if seed % 3 == 0 {
                    gpf(c, -0x204, 0);
                } else {
                    gpf(c, -0x204, AUX);
                    put(c, AUX + 0x64, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn ov_screen_rival_pre_on_widget_notify_matches() {
        check(
            ee::s05::f_001fdb78__generated,
            ov_screen_rival_pre_on_widget_notify,
            200,
            &|c, seed| {
                setup(c, seed, 36);
                c.g[A2][0] = [6u64, 0, 9][(seed % 3) as usize];
            },
        );
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
        check(
            ee::s04::f_001faa78__generated,
            ov_screen_be_options_on_widget_notify,
            SEEDS_3,
            &|c, seed| {
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
            },
        );
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
        check(
            ssx3_ee::s05::f_001fe648__generated,
            ov_screen_save_replay_v39,
            20,
            &|c, seed| {
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
            },
        );
    }

    #[test]
    fn ov_screen_rival_pre_init_matches() {
        check(
            ee::s05::f_001fd268__generated,
            ov_screen_rival_pre_init,
            40,
            &|c, seed| {
                alloc_base(c, seed, 0x1fd2, 5);
                c.g[A0][0] = DATA as u64;
                w(c, DATA + 0x10, DATA + 0x400);
                w(c, DATA + 0x400, DATA + 0x500);
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
        check3(
            ee::s04::f_001fb458__generated,
            in_game_error_highlight_continue,
            40,
            &|c, seed| {
                let _ = base_7(c, seed, 0x1fb458);
                highlight_screen(c, [0x0046_fb48, 0x0046_fad8, 0x0046_fb00]);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = (seed % 2) as u64;
            },
        );
    }

    #[test]
    fn in_game_error_highlight_cancel_matches() {
        check3(
            ee::s04::f_001fb588__generated,
            in_game_error_highlight_cancel,
            40,
            &|c, seed| {
                let _ = base_7(c, seed, 0x1fb588);
                highlight_screen(c, [0x0046_fa90, 0x0046_fac8, 0x0046_fae8]);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = (seed % 2) as u64;
            },
        );
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

    use crate::difftest::fakes;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::ptr;
    use crate::difftest::fill;
    use crate::difftest::RA as DT_RA;
    pub(crate) use crate::game::fe::fepopupmisc::tests::oc797_arm;

    use std::sync::atomic::AtomicUsize;

    // Fixture objects, all in DATA+0x200000 and up: clear of the fake log
    // (DATA+0x100000) and of the pointers the fakes return (DATA..DATA+0x40000).
    pub(crate) const OC766_BASE: u32 = DATA + 0x20_0000;

    pub(crate) const OC766_CTRL: u32 = OC766_BASE;

    pub(crate) const OC766_MAGIC: u32 = 0x4f43_3736;

    /// Results of the successive `UIEngine_CreateScreen` calls (0 = null).
    pub(crate) const OC766_CREATE: u32 = OC766_BASE + 0x100;

    /// `BENewPlayerInt_GetHumanRider(0)` and `(1)`.
    pub(crate) const OC766_RIDERS: u32 = OC766_BASE + 0x140;

    pub(crate) const OC766_STATE: u32 = OC766_BASE + 0x2000;

    pub(crate) const OC766_WORLD: u32 = OC766_BASE + 0x3000;

    pub(crate) const OC766_CFG: u32 = OC766_BASE + 0x3400;

    pub(crate) const OC766_W84: u32 = OC766_BASE + 0x3800;

    pub(crate) const OC766_W84P: u32 = OC766_BASE + 0x3900;

    pub(crate) const OC766_OVOBJ: u32 = OC766_BASE + 0x3a00;

    pub(crate) const OC766_IFACE: u32 = OC766_BASE + 0x3c00;

    pub(crate) const OC766_IFVT: u32 = OC766_BASE + 0x3d00;

    /// The game-mode byte (`0x535c11`).
    pub(crate) const OC766_MODE: u32 = 0x0053_5c11;

    /// The mailbox word of the fake for `addr`.
    pub(crate) fn oc766_mail(addr: u32) -> u32 {
        OC766_BASE + 0x10_0000 + (addr & 0xf_fffc)
    }

    /// Make the fake for `addr` return `v` in `v0` (stored +3).
    pub(crate) fn oc766_force(c: &mut Cpu, addr: u32, v: u32) {
        c.bus.write32(oc766_mail(addr), v.wrapping_add(3));
    }

    /// Make the fake for `addr` return 0.
    pub(crate) fn oc766_force_zero(c: &mut Cpu, addr: u32) {
        c.bus.write32(oc766_mail(addr), 1);
    }

    /// The fake callee: logs the call like `fakes::fake_call`, then returns
    /// the forced result of its mailbox (0 = the fake's pointer), the next
    /// word of the `CreateScreen` list or the human rider asked for.
    pub(crate) fn oc766_fake_call(c: &mut Cpu, addr: u32) {
        let a1 = c.g[A1][0] as u32;
        let ra = c.g[RA][0] as u32;
        let mail = c.bus.read32(oc766_mail(addr));
        let create = if addr == 0x0039_7cc0 {
            let k = c.bus.read32(OC766_CREATE + 0x20);
            c.bus.write32(OC766_CREATE + 0x20, k + 1);
            Some(c.bus.read32(OC766_CREATE + 4 * (k & 3)))
        } else {
            None
        };
        let rider = if addr == 0x0014_6e98 {
            Some(c.bus.read32(OC766_RIDERS + 4 * (a1 & 1)))
        } else {
            None
        };
        fakes::fake_call(c, addr);
        if c.pc != ra {
            return;
        }
        if let Some(v) = rider {
            c.g[V0][0] = sx(v);
        } else if let Some(v) = create {
            if v == 0 {
                c.g[V0][0] = 0;
            }
        } else if mail != 0 {
            c.g[V0][0] = match mail {
                1 => 0,
                m => (m - 3) as u64,
            };
        }
    }

    macro_rules! oc766_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC766_CTRL) != OC766_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc766_fake_call(c, $addr);
                }
            )*
            fn oc766_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc766_fakes!(
        (
            oc766_fake_317670,
            OC766_PREV_317670,
            ee::s10::OVERRIDE_00317670,
            ee::s10::f_00317670__generated,
            0x0031_7670
        ),
        (
            oc766_fake_397cc0,
            OC766_PREV_397CC0,
            ee::s13::OVERRIDE_00397CC0,
            ee::s13::f_00397cc0__generated,
            0x0039_7cc0
        ),
        (
            oc766_fake_39c870,
            OC766_PREV_39C870,
            ee::s13::OVERRIDE_0039C870,
            ee::s13::f_0039c870__generated,
            0x0039_c870
        ),
        (
            oc766_fake_14dc80,
            OC766_PREV_14DC80,
            ee::s01::OVERRIDE_0014DC80,
            ee::s01::f_0014dc80__generated,
            0x0014_dc80
        ),
        (
            oc766_fake_14dd58,
            OC766_PREV_14DD58,
            ee::s01::OVERRIDE_0014DD58,
            ee::s01::f_0014dd58__generated,
            0x0014_dd58
        ),
        (
            oc766_fake_146e98,
            OC766_PREV_146E98,
            ee::s01::OVERRIDE_00146E98,
            ee::s01::f_00146e98__generated,
            0x0014_6e98
        ),
        (
            oc766_fake_39d860,
            OC766_PREV_39D860,
            ee::s13::OVERRIDE_0039D860,
            ee::s13::f_0039d860__generated,
            0x0039_d860
        ),
        (
            oc766_fake_144bf0,
            OC766_PREV_144BF0,
            ee::s01::OVERRIDE_00144BF0,
            ee::s01::f_00144bf0__generated,
            0x0014_4bf0
        ),
        (
            oc766_fake_2c2540,
            OC766_PREV_2C2540,
            ee::s08::OVERRIDE_002C2540,
            ee::s08::f_002c2540__generated,
            0x002c_2540
        ),
        (
            oc766_fake_2c27c0,
            OC766_PREV_2C27C0,
            ee::s08::OVERRIDE_002C27C0,
            ee::s08::f_002c27c0__generated,
            0x002c_27c0
        ),
        (
            oc766_fake_3a0e90,
            OC766_PREV_3A0E90,
            ee::s13::OVERRIDE_003A0E90,
            ee::s13::f_003a0e90__generated,
            0x003a_0e90
        ),
        (
            oc766_fake_39e8b8,
            OC766_PREV_39E8B8,
            ee::s13::OVERRIDE_0039E8B8,
            ee::s13::f_0039e8b8__generated,
            0x0039_e8b8
        ),
        (
            oc766_fake_39e948,
            OC766_PREV_39E948,
            ee::s13::OVERRIDE_0039E948,
            ee::s13::f_0039e948__generated,
            0x0039_e948
        ),
        (
            oc766_fake_3a1000,
            OC766_PREV_3A1000,
            ee::s13::OVERRIDE_003A1000,
            ee::s13::f_003a1000__generated,
            0x003a_1000
        ),
        (
            oc766_fake_417828,
            OC766_PREV_417828,
            ee::s15::OVERRIDE_00417828,
            ee::s15::f_00417828__generated,
            0x0041_7828
        ),
        (
            oc766_fake_14eec8,
            OC766_PREV_14EEC8,
            ee::s01::OVERRIDE_0014EEC8,
            ee::s01::f_0014eec8__generated,
            0x0014_eec8
        ),
        (
            oc766_fake_146f88,
            OC766_PREV_146F88,
            ee::s01::OVERRIDE_00146F88,
            ee::s01::f_00146f88__generated,
            0x0014_6f88
        ),
        (
            oc766_fake_155130,
            OC766_PREV_155130,
            ee::s01::OVERRIDE_00155130,
            ee::s01::f_00155130__generated,
            0x0015_5130
        ),
        (
            oc766_fake_28b180,
            OC766_PREV_28B180,
            ee::s07::OVERRIDE_0028B180,
            ee::s07::f_0028b180__generated,
            0x0028_b180
        ),
        (
            oc766_fake_3a0c70,
            OC766_PREV_3A0C70,
            ee::s13::OVERRIDE_003A0C70,
            ee::s13::f_003a0c70__generated,
            0x003a_0c70
        ),
        (
            oc766_fake_2ea820,
            OC766_PREV_2EA820,
            ee::s09::OVERRIDE_002EA820,
            ee::s09::f_002ea820__generated,
            0x002e_a820
        ),
        (
            oc766_fake_2ea860,
            OC766_PREV_2EA860,
            ee::s09::OVERRIDE_002EA860,
            ee::s09::f_002ea860__generated,
            0x002e_a860
        ),
        (
            oc766_fake_39f9d8,
            OC766_PREV_39F9D8,
            ee::s13::OVERRIDE_0039F9D8,
            ee::s13::f_0039f9d8__generated,
            0x0039_f9d8
        ),
        (
            oc766_fake_413af8,
            OC766_PREV_413AF8,
            ee::s15::OVERRIDE_00413AF8,
            ee::s15::f_00413af8__generated,
            0x0041_3af8
        ),
        (
            oc766_fake_287920,
            OC766_PREV_287920,
            ee::s07::OVERRIDE_00287920,
            ee::s07::f_00287920__generated,
            0x0028_7920
        ),
        (
            oc766_fake_29f128,
            OC766_PREV_29F128,
            ee::s08::OVERRIDE_0029F128,
            ee::s08::f_0029f128__generated,
            0x0029_f128
        ),
        (
            oc766_fake_2a4040,
            OC766_PREV_2A4040,
            ee::s08::OVERRIDE_002A4040,
            ee::s08::f_002a4040__generated,
            0x002a_4040
        ),
        (
            oc766_fake_295028,
            OC766_PREV_295028,
            ee::s08::OVERRIDE_00295028,
            ee::s08::f_00295028__generated,
            0x0029_5028
        ),
        (
            oc766_fake_2b1458,
            OC766_PREV_2B1458,
            ee::s08::OVERRIDE_002B1458,
            ee::s08::f_002b1458__generated,
            0x002b_1458
        ),
        (
            oc766_fake_3d8008,
            OC766_PREV_3D8008,
            ee::s14::OVERRIDE_003D8008,
            ee::s14::f_003d8008__generated,
            0x003d_8008
        ),
    );

    /// `OVScreenRacePre_Init`: the two screens created (or null), the heat /
    /// event kind (-1 to 5) picking the shown rider-count object and the caption,
    /// the game mode byte (0, 1, 2: the +0x4a2148 text), 0 to 7 riders in the six
    /// rows (human riders get "name (player)" in mode 2), and the announcer
    /// line's gates; the interface vtable call, the real `OVScreenOverlayMenu_Init`
    /// (`cOVStateOverlay_Init`, 5 rows), `UIText_SetString`, `FE_FormatMinSec`,
    /// `PlayFESound` and `PASayRaceIntro` run on faked callees.
    #[test]
    fn ov_screen_race_pre_init_matches() {
        oc766_install_fakes();
        covered(
            ee::s04::f_001fb6b8__generated,
            ov_screen_race_pre_init,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1fb6b8, 40);
                c.pc = 0x001f_b6b8;
                c.bus.write32(OC766_CTRL, OC766_MAGIC);
                c.g[A0][0] = sx(OC766_STATE);
                c.bus.write32(OC766_STATE + 0x10, r.next_u32());
                // The world.
                c.bus.write32(crate::difftest::GAME_GP - 0x848, OC766_WORLD);
                c.bus.write32(OC766_WORLD + 0xc0, OC766_CFG);
                c.bus.write32(OC766_WORLD + 0x84, OC766_W84);
                c.bus.write32(OC766_WORLD + 0x8c, OC766_IFACE);
                // Heat / event kind and the rider rows.
                let kind = [-1i32, 0, 1, 2, 3, 4][(seed % 6) as usize];
                c.bus.write32(OC766_CFG, kind as u32);
                fill(c, &mut r, OC766_CFG + 4, 0x20);
                // Game mode byte.
                c.bus.write8(OC766_MODE, ((seed / 6) % 3) as u8);
                // Rider count.
                c.bus.write32(OC766_W84 + 0xc, OC766_W84P);
                c.bus.write32(OC766_W84P + 0x78, (seed / 18) % 8);
                // cOVStateOverlay_Init: the cut-scene transition state.
                c.bus.write32(OC766_W84 + 0x64, OC766_OVOBJ);
                c.bus
                    .write32(OC766_OVOBJ + 0x10, [0, 1, 3, 2][((seed / 5) % 4) as usize]);
                // Interface object: vtable slot (+0x20 delta, +0x24 target).
                c.bus.write32(OC766_IFACE + 4, OC766_IFVT);
                c.bus
                    .write16(OC766_IFVT + 0x20, (r.next_u32() & 0xfc) as u16);
                c.bus.write32(OC766_IFVT + 0x24, fakes::FAKE + 0x40);
                // The two screens: null or not.
                for k in 0..4 {
                    let v = if r.next_u32() % 4 == 0 {
                        0
                    } else {
                        0x8000_0000 + 0x40 * k
                    };
                    c.bus.write32(OC766_CREATE + 4 * k, v);
                }
                // Human riders.
                for k in 0..2 {
                    let v = if r.next_u32() % 5 == 0 {
                        0xffff_ffff
                    } else {
                        r.next_u32() % 7
                    };
                    c.bus.write32(OC766_RIDERS + 4 * k, v);
                }
                // Announcer gates: speech on, not event mode 4.
                if (seed / 3) % 3 == 0 {
                    oc766_force_zero(c, 0x0029_f128);
                } else {
                    oc766_force(c, 0x0029_f128, 1);
                }
                if (seed / 9) % 3 == 0 {
                    oc766_force(c, 0x002a_4040, 1);
                } else {
                    oc766_force_zero(c, 0x002a_4040);
                }
                if (seed / 27) % 2 == 0 {
                    oc766_force_zero(c, 0x0029_5028);
                }
                if (seed / 54) % 2 == 0 {
                    oc766_force_zero(c, 0x002b_1458);
                }
            },
        );
    }

    /// The word that arms oc778's callee fakes in a machine's RAM.
    pub(crate) const OC778_CTRL: u32 = DATA + 0x30_0000;

    pub(crate) const OC778_MAGIC: u32 = 0x0778_f4ce;

    /// The mailbox word of the fake for `addr` (0: the fake's own pointer).
    pub(crate) fn oc778_mail(addr: u32) -> u32 {
        DATA + 0x30_1000 + (addr & 0xffc)
    }

    /// Arm the gated fakes in this machine.
    pub(crate) fn oc778_arm(c: &mut Cpu) {
        c.bus.write32(OC778_CTRL, OC778_MAGIC);
    }

    /// The fake callee: logs the call like `fakes::fake_call`, then returns the
    /// forced result of its mailbox unless it unwound.
    pub(crate) fn oc778_fake_call(c: &mut Cpu, addr: u32) {
        let a0 = c.g[A0][0];
        let ra = c.g[RA][0] as u32;
        let mail = c.bus.read32(oc778_mail(addr));
        fakes::fake_call(c, addr);
        if c.pc == ra && mail != 0 {
            c.g[V0][0] = match mail {
                1 => 0,
                2 => a0,
                m => (m - 3) as u64,
            };
        }
    }

    macro_rules! oc778_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC778_CTRL) != OC778_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc778_fake_call(c, $addr);
                }
            )*
            pub(crate) fn oc778_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc778_fakes!(
        (
            oc778_fk_146e98,
            OC778_PREV_00146E98,
            ee::s01::OVERRIDE_00146E98,
            ee::s01::f_00146e98__generated,
            0x0014_6e98
        ),
        (
            oc778_fk_14dc80,
            OC778_PREV_0014DC80,
            ee::s01::OVERRIDE_0014DC80,
            ee::s01::f_0014dc80__generated,
            0x0014_dc80
        ),
        (
            oc778_fk_14dd58,
            OC778_PREV_0014DD58,
            ee::s01::OVERRIDE_0014DD58,
            ee::s01::f_0014dd58__generated,
            0x0014_dd58
        ),
        (
            oc778_fk_14de28,
            OC778_PREV_0014DE28,
            ee::s01::OVERRIDE_0014DE28,
            ee::s01::f_0014de28__generated,
            0x0014_de28
        ),
        (
            oc778_fk_14de68,
            OC778_PREV_0014DE68,
            ee::s01::OVERRIDE_0014DE68,
            ee::s01::f_0014de68__generated,
            0x0014_de68
        ),
        (
            oc778_fk_14eca0,
            OC778_PREV_0014ECA0,
            ee::s01::OVERRIDE_0014ECA0,
            ee::s01::f_0014eca0__generated,
            0x0014_eca0
        ),
        (
            oc778_fk_155a50,
            OC778_PREV_00155A50,
            ee::s01::OVERRIDE_00155A50,
            ee::s01::f_00155a50__generated,
            0x0015_5a50
        ),
        (
            oc778_fk_155e58,
            OC778_PREV_00155E58,
            ee::s01::OVERRIDE_00155E58,
            ee::s01::f_00155e58__generated,
            0x0015_5e58
        ),
        (
            oc778_fk_113b10,
            OC778_PREV_00113B10,
            ee::s00::OVERRIDE_00113B10,
            ee::s00::f_00113b10__generated,
            0x0011_3b10
        ),
        (
            oc778_fk_128958,
            OC778_PREV_00128958,
            ee::s00::OVERRIDE_00128958,
            ee::s00::f_00128958__generated,
            0x0012_8958
        ),
        (
            oc778_fk_1289f0,
            OC778_PREV_001289F0,
            ee::s00::OVERRIDE_001289F0,
            ee::s00::f_001289f0__generated,
            0x0012_89f0
        ),
        (
            oc778_fk_128a10,
            OC778_PREV_00128A10,
            ee::s00::OVERRIDE_00128A10,
            ee::s00::f_00128a10__generated,
            0x0012_8a10
        ),
        (
            oc778_fk_128a48,
            OC778_PREV_00128A48,
            ee::s00::OVERRIDE_00128A48,
            ee::s00::f_00128a48__generated,
            0x0012_8a48
        ),
        (
            oc778_fk_129160,
            OC778_PREV_00129160,
            ee::s00::OVERRIDE_00129160,
            ee::s00::f_00129160__generated,
            0x0012_9160
        ),
        (
            oc778_fk_194738,
            OC778_PREV_00194738,
            ee::s02::OVERRIDE_00194738,
            ee::s02::f_00194738__generated,
            0x0019_4738
        ),
        (
            oc778_fk_1d5038,
            OC778_PREV_001D5038,
            ee::s04::OVERRIDE_001D5038,
            ee::s04::f_001d5038__generated,
            0x001d_5038
        ),
        (
            oc778_fk_1f3700,
            OC778_PREV_001F3700,
            ee::s04::OVERRIDE_001F3700,
            ee::s04::f_001f3700__generated,
            0x001f_3700
        ),
        (
            oc778_fk_20a6f0,
            OC778_PREV_0020A6F0,
            ee::s05::OVERRIDE_0020A6F0,
            ee::s05::f_0020a6f0__generated,
            0x0020_a6f0
        ),
        (
            oc778_fk_25b6d8,
            OC778_PREV_0025B6D8,
            ee::s06::OVERRIDE_0025B6D8,
            ee::s06::f_0025b6d8__generated,
            0x0025_b6d8
        ),
        (
            oc778_fk_266df8,
            OC778_PREV_00266DF8,
            ee::s07::OVERRIDE_00266DF8,
            ee::s07::f_00266df8__generated,
            0x0026_6df8
        ),
        (
            oc778_fk_266e88,
            OC778_PREV_00266E88,
            ee::s07::OVERRIDE_00266E88,
            ee::s07::f_00266e88__generated,
            0x0026_6e88
        ),
        (
            oc778_fk_26f7b8,
            OC778_PREV_0026F7B8,
            ee::s07::OVERRIDE_0026F7B8,
            ee::s07::f_0026f7b8__generated,
            0x0026_f7b8
        ),
        (
            oc778_fk_2790a0,
            OC778_PREV_002790A0,
            ee::s07::OVERRIDE_002790A0,
            ee::s07::f_002790a0__generated,
            0x0027_90a0
        ),
        (
            oc778_fk_2a4cf8,
            OC778_PREV_002A4CF8,
            ee::s08::OVERRIDE_002A4CF8,
            ee::s08::f_002a4cf8__generated,
            0x002a_4cf8
        ),
        (
            oc778_fk_2a77f8,
            OC778_PREV_002A77F8,
            ee::s08::OVERRIDE_002A77F8,
            ee::s08::f_002a77f8__generated,
            0x002a_77f8
        ),
        (
            oc778_fk_2a7890,
            OC778_PREV_002A7890,
            ee::s08::OVERRIDE_002A7890,
            ee::s08::f_002a7890__generated,
            0x002a_7890
        ),
        (
            oc778_fk_2a7bf8,
            OC778_PREV_002A7BF8,
            ee::s08::OVERRIDE_002A7BF8,
            ee::s08::f_002a7bf8__generated,
            0x002a_7bf8
        ),
        (
            oc778_fk_308c60,
            OC778_PREV_00308C60,
            ee::s10::OVERRIDE_00308C60,
            ee::s10::f_00308c60__generated,
            0x0030_8c60
        ),
        (
            oc778_fk_308f38,
            OC778_PREV_00308F38,
            ee::s10::OVERRIDE_00308F38,
            ee::s10::f_00308f38__generated,
            0x0030_8f38
        ),
        (
            oc778_fk_317d70,
            OC778_PREV_00317D70,
            ee::s10::OVERRIDE_00317D70,
            ee::s10::f_00317d70__generated,
            0x0031_7d70
        ),
        (
            oc778_fk_317e30,
            OC778_PREV_00317E30,
            ee::s10::OVERRIDE_00317E30,
            ee::s10::f_00317e30__generated,
            0x0031_7e30
        ),
        (
            oc778_fk_38d638,
            OC778_PREV_0038D638,
            ee::s12::OVERRIDE_0038D638,
            ee::s12::f_0038d638__generated,
            0x0038_d638
        ),
        (
            oc778_fk_39e2a0,
            OC778_PREV_0039E2A0,
            ee::s13::OVERRIDE_0039E2A0,
            ee::s13::f_0039e2a0__generated,
            0x0039_e2a0
        ),
        (
            oc778_fk_39e4c0,
            OC778_PREV_0039E4C0,
            ee::s13::OVERRIDE_0039E4C0,
            ee::s13::f_0039e4c0__generated,
            0x0039_e4c0
        ),
        (
            oc778_fk_39f238,
            OC778_PREV_0039F238,
            ee::s13::OVERRIDE_0039F238,
            ee::s13::f_0039f238__generated,
            0x0039_f238
        ),
        (
            oc778_fk_39f400,
            OC778_PREV_0039F400,
            ee::s13::OVERRIDE_0039F400,
            ee::s13::f_0039f400__generated,
            0x0039_f400
        ),
        (
            oc778_fk_3e6448,
            OC778_PREV_003E6448,
            ee::s14::OVERRIDE_003E6448,
            ee::s14::f_003e6448__generated,
            0x003e_6448
        ),
        (
            oc778_fk_417828,
            OC778_PREV_00417828,
            ee::s15::OVERRIDE_00417828,
            ee::s15::f_00417828__generated,
            0x0041_7828
        ),
    );

    /// `OVScreenRacePre_OnActivate`: the game path byte (2 = second rider
    /// text), the game-comm pointer (null or set) and the unwinds of its calls.
    #[test]
    fn ov_screen_race_pre_on_activate_matches() {
        oc778_install_fakes();
        fakes::covered(
            ee::s04::f_001fbbd8__generated,
            ov_screen_race_pre_on_activate,
            240,
            &|c, seed| {
                machine(c, seed, 0x778a, 10);
                oc778_arm(c);
                c.g[A0][0] = sx(DATA + 0x1_0000);
                c.bus.write8(0x0053_5c11, [2u8, 0, 1][(seed % 3) as usize]);
                let comm = if (seed / 3) % 2 == 0 {
                    0
                } else {
                    DATA + 0x2_0000
                };
                c.bus.write32(GAME_GP - 0x204, comm);
            },
        );
    }

    /// `OVScreenRacePre_dtor`: vtable store and the `UIState_dtor` call.
    #[test]
    fn ov_screen_race_pre_dtor_matches() {
        oc797_install_fakes();
        fakes::covered(
            ee::s05::f_00221df0__generated,
            ov_screen_race_pre_dtor,
            80,
            &|c, seed| {
                let mut r = machine(c, seed, 0x797d, 1);
                oc797_arm(c);
                c.g[A0][0] = ptr(&mut r) as u64;
                c.g[RA][0] = DT_RA as u64;
            },
        );
    }

    pub(crate) use crate::game::fe::fereal::tests::oc802_install_fakes;
    pub(crate) use crate::game::fe::fereal::tests::OC802_CTRL;

    pub(crate) use crate::game::fe::fereal::tests::OC802_MAGIC;

    /// `FE_GetEventNameText`: kinds 5..=12 (6..=11 through the table), the
    /// string-table lookup through the world's object.
    #[test]
    fn fe_get_event_name_text_matches() {
        oc802_install_fakes();
        fakes::covered(
            ee::s04::f_001fd190__generated,
            fe_get_event_name_text,
            80,
            &|c, seed| {
                let _ = fakes::machine(c, seed, 0x802b, 4);
                c.bus.write32(OC802_CTRL, OC802_MAGIC);
                let table = [
                    0x001f_d1c8u32,
                    0x001f_d1d8,
                    0x001f_d1e8,
                    0x001f_d1f8,
                    0x001f_d208,
                    0x001f_d218,
                ];
                for (i, t) in table.into_iter().enumerate() {
                    c.bus.write32(0x0046_ff40 + 4 * i as u32, t);
                }
                if seed % 11 == 10 {
                    c.bus
                        .write32(0x0046_ff40 + 4 * (seed / 11 % 6), 0x0012_3450);
                }
                let world = DATA + 0x25_0000;
                let obj = DATA + 0x26_0000;
                let inner = DATA + 0x27_0000;
                c.bus.write32(GAME_GP - 0x848, world);
                c.bus.write32(world + 0x8c, obj);
                c.bus.write32(obj + 4, inner);
                c.bus.write16(inner + 0x20, (seed * 8) as u16);
                c.bus.write32(inner + 0x24, fakes::FAKE);
                c.g[A0][0] = (5 + seed % 8) as u64;
                c.pc = 0x001f_d190;
            },
        );
    }

    use crate::difftest::elf_image;
    use crate::difftest::events::check_events;
    use crate::difftest::events::schedule;
    use crate::difftest::fakes::check_reaching;

    use crate::game::bx::bxstringctor::tests::base_3 as base_3_m2;

    pub(crate) use crate::game::fe::all_in_one_fe::tests::oc1012_install_fakes;
    pub(crate) use crate::game::fe::all_in_one_fe::tests::oc1012_object;

    pub(crate) use crate::game::fe::all_in_one_fe::tests::SEEDS as SEEDS_4;

    pub(crate) use crate::game::fe::ovstateenterlodge::tests::OC982_STATE;
    pub(crate) use crate::game::fe::ovstatetrophy::tests::oc1015_object_setup;

    pub(crate) use crate::game::fe::ovtemplatedialog::tests::oc983_fixture;

    pub(crate) use crate::game::fe::ovtemplatedialog::tests::oc983_paths;

    pub(crate) use crate::game::fe::ovtemplatedialog::tests::OC983_SELF;

    pub(crate) use crate::game::fe::ovtemplatepausemenu::tests::oc982_machine;
    pub(crate) use crate::game::fe::ovtemplatepausemenu::tests::oc982_path_count;
    pub(crate) use crate::game::fe::ovtemplatepausemenu::tests::oc982_world;

    pub(crate) use crate::game::fe::ovtemplatepausemenu::tests::OC982_MODEMAN;
    pub(crate) use crate::game::fe::ovtemplatepausemenu::tests::OC982_SELF;

    use std::ptr::addr_of_mut;

    // Fixture objects live in DATA+0x200000 and up: clear of the fake log
    // (DATA+0x100000) and of the pointers the fakes return (DATA..DATA+0x40000).

    /// Gate word of this module's fakes, and its magic value.
    pub(crate) const OC975_CTRL: u32 = DATA + 0x20_0000;

    pub(crate) const OC975_MAGIC: u32 = 0x4f43_3937;

    /// The state object (`self`) and the engine it points to.
    pub(crate) const OC975_SELF: u32 = DATA + 0x21_0000;

    /// World (gp-0x848) and the camera/label object graph below it.
    pub(crate) const OC975_WORLD: u32 = DATA + 0x22_0000;

    pub(crate) const OC975_OBJ: u32 = DATA + 0x23_0000;

    /// Per-callee mailbox: bit n of the word makes the n-th call of that
    /// callee return 0 (lookups that find nothing, items not found, ...).
    pub(crate) fn oc975_mail(addr: u32) -> u32 {
        OC975_CTRL + 0x100 + (addr & 0xffc)
    }

    /// Per-callee call counter.
    pub(crate) fn oc975_count(addr: u32) -> u32 {
        OC975_CTRL + 0x1100 + (addr & 0xffc)
    }

    /// The fake callee: logs the call like `fakes::fake_call`, then returns 0
    /// if its mailbox asks for it on this call.
    pub(crate) fn oc975_fake_call(c: &mut Cpu, addr: u32) {
        let mail = c.bus.read32(oc975_mail(addr));
        let n = c.bus.read32(oc975_count(addr));
        c.bus.write32(oc975_count(addr), n + 1);
        let ra = c.g[RA][0] as u32;
        fakes::fake_call(c, addr);
        if c.pc == ra && (mail >> (n & 31)) & 1 == 1 {
            c.g[V0][0] = 0;
        }
    }

    macro_rules! oc975_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC975_CTRL) != OC975_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc975_fake_call(c, $addr);
                }
            )*
            fn oc975_install_fakes() {
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc975_fakes!(
        (
            oc975_fake_317670,
            OC975_PREV_317670,
            ee::s10::OVERRIDE_00317670,
            ee::s10::f_00317670__generated,
            0x00317670
        ),
        (
            oc975_fake_397cc0,
            OC975_PREV_397CC0,
            ee::s13::OVERRIDE_00397CC0,
            ee::s13::f_00397cc0__generated,
            0x00397cc0
        ),
        (
            oc975_fake_39c870,
            OC975_PREV_39C870,
            ee::s13::OVERRIDE_0039C870,
            ee::s13::f_0039c870__generated,
            0x0039c870
        ),
        (
            oc975_fake_20a380,
            OC975_PREV_20A380,
            ee::s05::OVERRIDE_0020A380,
            ee::s05::f_0020a380__generated,
            0x0020a380
        ),
        (
            oc975_fake_14dc80,
            OC975_PREV_14DC80,
            ee::s01::OVERRIDE_0014DC80,
            ee::s01::f_0014dc80__generated,
            0x0014dc80
        ),
        (
            oc975_fake_14dd58,
            OC975_PREV_14DD58,
            ee::s01::OVERRIDE_0014DD58,
            ee::s01::f_0014dd58__generated,
            0x0014dd58
        ),
        (
            oc975_fake_39d860,
            OC975_PREV_39D860,
            ee::s13::OVERRIDE_0039D860,
            ee::s13::f_0039d860__generated,
            0x0039d860
        ),
        (
            oc975_fake_39a568,
            OC975_PREV_39A568,
            ee::s13::OVERRIDE_0039A568,
            ee::s13::f_0039a568__generated,
            0x0039a568
        ),
        (
            oc975_fake_39a768,
            OC975_PREV_39A768,
            ee::s13::OVERRIDE_0039A768,
            ee::s13::f_0039a768__generated,
            0x0039a768
        ),
        (
            oc975_fake_39a7a8,
            OC975_PREV_39A7A8,
            ee::s13::OVERRIDE_0039A7A8,
            ee::s13::f_0039a7a8__generated,
            0x0039a7a8
        ),
        (
            oc975_fake_2c26d0,
            OC975_PREV_2C26D0,
            ee::s08::OVERRIDE_002C26D0,
            ee::s08::f_002c26d0__generated,
            0x002c26d0
        ),
        (
            oc975_fake_3a0e90,
            OC975_PREV_3A0E90,
            ee::s13::OVERRIDE_003A0E90,
            ee::s13::f_003a0e90__generated,
            0x003a0e90
        ),
        (
            oc975_fake_28b180,
            OC975_PREV_28B180,
            ee::s07::OVERRIDE_0028B180,
            ee::s07::f_0028b180__generated,
            0x0028b180
        ),
        (
            oc975_fake_287410,
            OC975_PREV_287410,
            ee::s07::OVERRIDE_00287410,
            ee::s07::f_00287410__generated,
            0x00287410
        ),
        (
            oc975_fake_287488,
            OC975_PREV_287488,
            ee::s07::OVERRIDE_00287488,
            ee::s07::f_00287488__generated,
            0x00287488
        ),
        (
            oc975_fake_287520,
            OC975_PREV_287520,
            ee::s07::OVERRIDE_00287520,
            ee::s07::f_00287520__generated,
            0x00287520
        ),
        (
            oc975_fake_287558,
            OC975_PREV_287558,
            ee::s07::OVERRIDE_00287558,
            ee::s07::f_00287558__generated,
            0x00287558
        ),
        (
            oc975_fake_2875d0,
            OC975_PREV_2875D0,
            ee::s07::OVERRIDE_002875D0,
            ee::s07::f_002875d0__generated,
            0x002875d0
        ),
        (
            oc975_fake_266f90,
            OC975_PREV_266F90,
            ee::s07::OVERRIDE_00266F90,
            ee::s07::f_00266f90__generated,
            0x00266f90
        ),
        (
            oc975_fake_266e88,
            OC975_PREV_266E88,
            ee::s07::OVERRIDE_00266E88,
            ee::s07::f_00266e88__generated,
            0x00266e88
        ),
        (
            oc975_fake_266e30,
            OC975_PREV_266E30,
            ee::s07::OVERRIDE_00266E30,
            ee::s07::f_00266e30__generated,
            0x00266e30
        ),
    );

    pub(crate) fn oc975_w(c: &mut Cpu, a: u32, v: u32) {
        c.bus.write32(a, v);
    }

    /// One seed's machine: fakes gated on, the options block, the world with
    /// its camera graph, the label object, the network flag and mailboxes.
    pub(crate) fn oc975_setup(c: &mut Cpu, seed: u32, calls: u32) -> Rng {
        oc975_install_fakes();
        let mut r = machine(c, seed, 0x1f_9188, calls);
        oc975_w(c, OC975_CTRL, OC975_MAGIC);
        // Lookups that find nothing (all found / none found / random).
        let missing = match seed % 5 {
            0 => 0,
            1 => u32::MAX,
            _ => r.next_u32() & r.next_u32(),
        };
        oc975_w(c, oc975_mail(0x0039_d860), missing);
        // List box value lookups that find nothing.
        let not_found = match seed % 3 {
            0 => 0,
            1 => u32::MAX,
            _ => r.next_u32(),
        };
        oc975_w(c, oc975_mail(0x0039_a768), not_found);
        // The screen could not be created.
        oc975_w(c, oc975_mail(0x0039_7cc0), (seed % 7 == 0) as u32);
        // self, engine.
        oc975_w(c, OC975_SELF + 0x10, OC975_OBJ + 0x8000);
        // The world and the camera graph: world+0x84 -> views -> +0x84 ->
        // man; man+4/+8 = cameras (+0xa8 -> controller, +0x18 = algorithm),
        // man+0x10 = number of views.
        let (man, cam0, cam1, cc0, cc1) = (
            OC975_OBJ + 0x100,
            OC975_OBJ + 0x200,
            OC975_OBJ + 0x300,
            OC975_OBJ + 0x400,
            OC975_OBJ + 0x500,
        );
        oc975_w(c, GAME_GP.wrapping_sub(0x848), OC975_WORLD);
        oc975_w(c, OC975_WORLD + 0x78, OC975_OBJ);
        oc975_w(c, OC975_WORLD + 0x84, OC975_OBJ + 0x1000);
        oc975_w(c, OC975_OBJ + 0x1000 + 0x84, man);
        oc975_w(c, man + 4, cam0);
        oc975_w(c, man + 8, cam1);
        oc975_w(c, man + 0x10, seed % 4);
        oc975_w(c, cam0 + 0xa8, cc0);
        oc975_w(c, cam1 + 0xa8, cc1);
        oc975_w(c, cc0 + 0x18, 0x3c + r.next_u32() % 4);
        oc975_w(c, cc1 + 0x18, 0x3c + r.next_u32() % 4);
        // The camera label object: world+0x8c -> label; label+4 -> vtable
        // (delta at +0x20, function at +0x24: a real leaf returning 0, or a
        // fake that returns a pointer).
        let (label, vt) = (OC975_OBJ + 0x600, OC975_OBJ + 0x700);
        oc975_w(c, OC975_WORLD + 0x8c, label);
        oc975_w(c, label + 4, vt);
        c.bus.write16(vt + 0x20, (r.next_u32() % 8 * 16) as u16);
        oc975_w(
            c,
            vt + 0x24,
            [0x0014_06c0, fakes::FAKE][((seed / 4) % 2) as usize],
        );
        oc975_w(c, GAME_GP.wrapping_add(0x304), OC975_OBJ + 0x900);
        // The options block, the network flag and the static colour.
        fill(c, &mut r, OVBO_OPTIONS_STATIC, 0x288 / 4);
        oc975_w(c, 0x0053_4b30, (seed / 2) % 2);
        fill(c, &mut r, 0x004c_66c8, 4);
        c.g[A0][0] = sx(OC975_SELF);
        r
    }

    /// Every branch: screen creation failing or not, each widget found or
    /// missing, list box entries found or not, one or two views, the label
    /// callback returning 0 or not, network game or not, both voice-chat
    /// level paths; with unwinding fake callees (host-thread unwind after any
    /// call).
    #[test]
    fn ov_screen_be_options_init_matches() {
        covered(
            ee::s04::f_001f9188__generated,
            ov_screen_be_options_init,
            400,
            &|c, seed| {
                oc975_setup(c, seed, 60);
            },
        );
    }

    /// The call arms the seeds reach (return addresses of logged calls).
    #[test]
    fn ov_screen_be_options_init_reaches_every_call() {
        fakes::check_reaching(
            ee::s04::f_001f9188__generated,
            ov_screen_be_options_init,
            300,
            &[
                0x001f_91f4,
                0x001f_9358,
                0x001f_9368,
                0x001f_93fc,
                0x001f_940c,
                0x001f_9474,
                0x001f_9480,
                0x001f_9524,
                0x001f_9534,
                0x001f_959c,
                0x001f_95a8,
                0x001f_9690,
                0x001f_9748,
                0x001f_9834,
                0x001f_98f0,
                0x001f_99d8,
                0x001f_9a90,
                0x001f_9bb4,
                0x001f_9bc4,
                0x001f_9bcc,
                0x001f_9c88,
                0x001f_9dac,
                0x001f_9dbc,
                0x001f_9dc4,
                0x001f_9e7c,
                0x001f_9eb8,
                0x001f_9ed0,
                0x001f_9f6c,
                0x001f_9fe4,
                0x001f_9ff8,
                0x001f_a02c,
                0x001f_a03c,
                0x001f_a078,
                0x001f_a0b8,
                0x001f_a0d0,
            ],
            &|c, seed| {
                oc975_setup(c, seed, 0);
            },
        );
    }

    /// The seeds make many different paths (distinct cycle totals).
    #[test]
    fn ov_screen_be_options_init_path_count() {
        let mut seen = std::collections::BTreeSet::new();
        for seed in 0..300 {
            let mut c = crate::difftest::fresh();
            oc975_setup(&mut c, seed, 0);
            let before = c.cycles;
            crate::difftest::run_generated(&mut c, ee::s04::f_001f9188__generated);
            seen.insert(c.cycles - before);
        }
        assert!(seen.len() >= 40, "only {} distinct paths", seen.len());
    }

    /// Events delivered inside the copy loops (cycle count, event schedule).
    #[test]
    fn ov_screen_be_options_init_events() {
        check_events(
            ee::s04::f_001f9188__generated,
            ov_screen_be_options_init,
            120,
            &|c, seed| {
                let mut r = oc975_setup(c, seed, 0);
                schedule(c, &mut r, seed, 0x001f_9188)
            },
        );
    }

    // Fixture objects, all in DATA+0x200000 and up: clear of the fake log
    // (DATA+0x100000) and of the pointers the fakes return (DATA..DATA+0x40000).
    pub(crate) const OC977_BASE: u32 = DATA + 0x20_0000;

    /// The word that arms this module's callee fakes in a machine's RAM.
    pub(crate) const OC977_CTRL: u32 = OC977_BASE;

    pub(crate) const OC977_MAGIC: u32 = 0x0c97_7f5e;

    pub(crate) const OC977_STATE: u32 = OC977_BASE + 0x2000;

    pub(crate) const OC977_WORLD: u32 = OC977_BASE + 0x3000;

    pub(crate) const OC977_CFG: u32 = OC977_BASE + 0x3400;

    pub(crate) const OC977_IFACE: u32 = OC977_BASE + 0x3c00;

    pub(crate) const OC977_IFVT: u32 = OC977_BASE + 0x3d00;

    /// `GameModeMan` byte (the event kind, `0x535c10`) and game-path byte (`0x535c11`).
    pub(crate) const OC977_KIND: u32 = 0x0053_5c10;

    pub(crate) const OC977_PATH: u32 = 0x0053_5c11;

    /// `cGameModeMan_iCharRank` (`0x536708`).
    pub(crate) const OC977_RANK: u32 = 0x0053_6708;

    /// The mailbox word of the fake for `addr` (0: the fake's own pointer).
    pub(crate) fn oc977_mail(addr: u32) -> u32 {
        OC977_BASE + 0x10_0000 + (addr & 0xf_fffc)
    }

    /// Make the fake for `addr` return `v` in `v0`.
    pub(crate) fn oc977_force(c: &mut Cpu, addr: u32, v: u32) {
        c.bus.write32(oc977_mail(addr), v.wrapping_add(3));
    }

    /// The fake callee: logs the call like `fakes::fake_call`, then returns the
    /// forced result of its mailbox unless it unwound.
    pub(crate) fn oc977_fake_call(c: &mut Cpu, addr: u32) {
        let ra = c.g[RA][0] as u32;
        let mail = c.bus.read32(oc977_mail(addr));
        fakes::fake_call(c, addr);
        if c.pc == ra && mail != 0 {
            c.g[V0][0] = match mail {
                1 => 0,
                m => sx(m - 3),
            };
        }
    }

    macro_rules! oc977_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC977_CTRL) != OC977_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc977_fake_call(c, $addr);
                }
            )*
            fn oc977_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc977_fakes!(
        (
            oc977_fake_144bf0,
            OC977_PREV_00144BF0,
            ee::s01::OVERRIDE_00144BF0,
            ee::s01::f_00144bf0__generated,
            0x00144bf0
        ),
        (
            oc977_fake_146e98,
            OC977_PREV_00146E98,
            ee::s01::OVERRIDE_00146E98,
            ee::s01::f_00146e98__generated,
            0x00146e98
        ),
        (
            oc977_fake_147398,
            OC977_PREV_00147398,
            ee::s01::OVERRIDE_00147398,
            ee::s01::f_00147398__generated,
            0x00147398
        ),
        (
            oc977_fake_147410,
            OC977_PREV_00147410,
            ee::s01::OVERRIDE_00147410,
            ee::s01::f_00147410__generated,
            0x00147410
        ),
        (
            oc977_fake_14dc80,
            OC977_PREV_0014DC80,
            ee::s01::OVERRIDE_0014DC80,
            ee::s01::f_0014dc80__generated,
            0x0014dc80
        ),
        (
            oc977_fake_14dd58,
            OC977_PREV_0014DD58,
            ee::s01::OVERRIDE_0014DD58,
            ee::s01::f_0014dd58__generated,
            0x0014dd58
        ),
        (
            oc977_fake_14eec8,
            OC977_PREV_0014EEC8,
            ee::s01::OVERRIDE_0014EEC8,
            ee::s01::f_0014eec8__generated,
            0x0014eec8
        ),
        (
            oc977_fake_155130,
            OC977_PREV_00155130,
            ee::s01::OVERRIDE_00155130,
            ee::s01::f_00155130__generated,
            0x00155130
        ),
        (
            oc977_fake_20d1d8,
            OC977_PREV_0020D1D8,
            ee::s05::OVERRIDE_0020D1D8,
            ee::s05::f_0020d1d8__generated,
            0x0020d1d8
        ),
        (
            oc977_fake_238590,
            OC977_PREV_00238590,
            ee::s06::OVERRIDE_00238590,
            ee::s06::f_00238590__generated,
            0x00238590
        ),
        (
            oc977_fake_2c2540,
            OC977_PREV_002C2540,
            ee::s08::OVERRIDE_002C2540,
            ee::s08::f_002c2540__generated,
            0x002c2540
        ),
        (
            oc977_fake_2c26d0,
            OC977_PREV_002C26D0,
            ee::s08::OVERRIDE_002C26D0,
            ee::s08::f_002c26d0__generated,
            0x002c26d0
        ),
        (
            oc977_fake_2c27c0,
            OC977_PREV_002C27C0,
            ee::s08::OVERRIDE_002C27C0,
            ee::s08::f_002c27c0__generated,
            0x002c27c0
        ),
        (
            oc977_fake_317670,
            OC977_PREV_00317670,
            ee::s10::OVERRIDE_00317670,
            ee::s10::f_00317670__generated,
            0x00317670
        ),
        (
            oc977_fake_397cc0,
            OC977_PREV_00397CC0,
            ee::s13::OVERRIDE_00397CC0,
            ee::s13::f_00397cc0__generated,
            0x00397cc0
        ),
        (
            oc977_fake_39c870,
            OC977_PREV_0039C870,
            ee::s13::OVERRIDE_0039C870,
            ee::s13::f_0039c870__generated,
            0x0039c870
        ),
        (
            oc977_fake_39d860,
            OC977_PREV_0039D860,
            ee::s13::OVERRIDE_0039D860,
            ee::s13::f_0039d860__generated,
            0x0039d860
        ),
        (
            oc977_fake_39e8b8,
            OC977_PREV_0039E8B8,
            ee::s13::OVERRIDE_0039E8B8,
            ee::s13::f_0039e8b8__generated,
            0x0039e8b8
        ),
        (
            oc977_fake_39e948,
            OC977_PREV_0039E948,
            ee::s13::OVERRIDE_0039E948,
            ee::s13::f_0039e948__generated,
            0x0039e948
        ),
        (
            oc977_fake_3a0c50,
            OC977_PREV_003A0C50,
            ee::s13::OVERRIDE_003A0C50,
            ee::s13::f_003a0c50__generated,
            0x003a0c50
        ),
        (
            oc977_fake_3a0e90,
            OC977_PREV_003A0E90,
            ee::s13::OVERRIDE_003A0E90,
            ee::s13::f_003a0e90__generated,
            0x003a0e90
        ),
        (
            oc977_fake_3a1000,
            OC977_PREV_003A1000,
            ee::s13::OVERRIDE_003A1000,
            ee::s13::f_003a1000__generated,
            0x003a1000
        ),
        (
            oc977_fake_417828,
            OC977_PREV_00417828,
            ee::s15::OVERRIDE_00417828,
            ee::s15::f_00417828__generated,
            0x00417828
        ),
    );

    /// `OVScreenFreestylePre_Init`: both screens created or null (the thread
    /// starts follow), the event kind byte (0..4: caption pick), the game path
    /// (0 career, 1 quick play, 2 multiplayer), the heat number (-1..5: objective
    /// texts, tabs, score columns), the human rider equal to the first ranked
    /// rider or not (the row skip), the heat-score result (0 or positive: the
    /// "earn x points" branch), and null text objects. Every callee runs on a
    /// logged fake; `covered` also unwinds at random call indices.
    #[test]
    fn ov_screen_freestyle_pre_init_matches() {
        oc977_install_fakes();
        covered(
            ee::s04::f_001fbd20__generated,
            ov_screen_freestyle_pre_init,
            420,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1fbd20, 100);
                c.pc = 0x001f_bd20;
                c.bus.write32(OC977_CTRL, OC977_MAGIC);
                c.g[A0][0] = sx(OC977_STATE);
                c.bus.write32(OC977_STATE + 0x10, r.next_u32());
                // The world, the heat config (heat number first) and the
                // interface object the event kind query goes through.
                c.bus.write32(GAME_GP - 0x848, OC977_WORLD);
                c.bus.write32(OC977_WORLD + 0xc0, OC977_CFG);
                c.bus.write32(OC977_WORLD + 0x8c, OC977_IFACE);
                fill(c, &mut r, OC977_CFG, 0x100);
                let heat = [-1i32, 0, 1, 2, 3, 4, 5][((seed / 15) % 7) as usize];
                c.bus.write32(OC977_CFG, heat as u32);
                c.bus.write32(OC977_IFACE + 4, OC977_IFVT);
                c.bus
                    .write16(OC977_IFVT + 0x20, (r.next_u32() & 0xfc) as u16);
                c.bus.write32(OC977_IFVT + 0x24, fakes::FAKE + 0x40);
                // Event kind and game path.
                c.bus.write8(OC977_KIND, (seed % 5) as u8);
                c.bus.write8(OC977_PATH, ((seed / 5) % 3) as u8);
                // Rank table with small character ids.
                for k in 0..8 {
                    c.bus.write32(OC977_RANK + 4 * k, r.next_u32() % 8);
                }
                // The human rider: equal to the first ranked one, or a pointer.
                if r.next_u32() % 2 == 0 {
                    let first = c.bus.read32(OC977_RANK);
                    oc977_force(c, 0x0014_6e98, first);
                }
                // Heat scores: zero, small positive, or the fake's pointer.
                match r.next_u32() % 3 {
                    0 => oc977_force(c, 0x0023_8590, 0),
                    1 => oc977_force(c, 0x0023_8590, 5),
                    _ => {}
                }
                // Null screens / text objects.
                if r.next_u32() % 4 == 0 {
                    oc977_force(c, 0x0039_7cc0, 0);
                }
                if r.next_u32() % 5 == 0 {
                    oc977_force(c, 0x0039_d860, 0);
                }
            },
        );
    }

    pub(crate) const OC978_BASE: u32 = DATA + 0x20_0000;

    /// Gate word: the fakes only answer while this holds the magic.
    pub(crate) const OC978_CTRL: u32 = OC978_BASE;

    pub(crate) const OC978_MAGIC: u32 = 0x4f43_3938;

    /// Results of the two `UIEngine_AddScreenByHashName` calls (0 = null),
    /// and the call counter at +0x20.
    pub(crate) const OC978_CREATE: u32 = OC978_BASE + 0x100;

    pub(crate) const OC978_STATE: u32 = OC978_BASE + 0x2000;

    pub(crate) const OC978_WORLD: u32 = OC978_BASE + 0x3000;

    pub(crate) const OC978_IFACE: u32 = OC978_BASE + 0x3400;

    pub(crate) const OC978_IFVT: u32 = OC978_BASE + 0x3800;

    /// The new-race settings (`cBENewRaceInterface_newRace`).
    pub(crate) const OC978_NEWRACE: u32 = 0x0053_5bc8;

    /// The fake callee: logs like `fakes::fake_call`; the screen creator
    /// answers from the `OC978_CREATE` list (0 or a pointer).
    pub(crate) fn oc978_fake_call(c: &mut Cpu, addr: u32) {
        let ra = c.g[RA][0] as u32;
        let create = if addr == 0x0039_7cc0 {
            let k = c.bus.read32(OC978_CREATE + 0x20);
            c.bus.write32(OC978_CREATE + 0x20, k + 1);
            Some(c.bus.read32(OC978_CREATE + 4 * (k & 3)))
        } else {
            None
        };
        fakes::fake_call(c, addr);
        if c.pc != ra {
            return;
        }
        if create == Some(0) {
            c.g[V0][0] = 0;
        }
    }

    macro_rules! oc978_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC978_CTRL) != OC978_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc978_fake_call(c, $addr);
                }
            )*
            fn oc978_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc978_fakes!(
        (
            oc978_fake_317670,
            OC978_PREV_317670,
            ee::s10::OVERRIDE_00317670,
            ee::s10::f_00317670__generated,
            0x0031_7670
        ),
        (
            oc978_fake_397cc0,
            OC978_PREV_397CC0,
            ee::s13::OVERRIDE_00397CC0,
            ee::s13::f_00397cc0__generated,
            0x0039_7cc0
        ),
        (
            oc978_fake_39c870,
            OC978_PREV_39C870,
            ee::s13::OVERRIDE_0039C870,
            ee::s13::f_0039c870__generated,
            0x0039_c870
        ),
        (
            oc978_fake_20a380,
            OC978_PREV_20A380,
            ee::s05::OVERRIDE_0020A380,
            ee::s05::f_0020a380__generated,
            0x0020_a380
        ),
        (
            oc978_fake_14dc80,
            OC978_PREV_14DC80,
            ee::s01::OVERRIDE_0014DC80,
            ee::s01::f_0014dc80__generated,
            0x0014_dc80
        ),
        (
            oc978_fake_14dd58,
            OC978_PREV_14DD58,
            ee::s01::OVERRIDE_0014DD58,
            ee::s01::f_0014dd58__generated,
            0x0014_dd58
        ),
        (
            oc978_fake_39d860,
            OC978_PREV_39D860,
            ee::s13::OVERRIDE_0039D860,
            ee::s13::f_0039d860__generated,
            0x0039_d860
        ),
        (
            oc978_fake_3a1000,
            OC978_PREV_3A1000,
            ee::s13::OVERRIDE_003A1000,
            ee::s13::f_003a1000__generated,
            0x003a_1000
        ),
        (
            oc978_fake_144bf0,
            OC978_PREV_144BF0,
            ee::s01::OVERRIDE_00144BF0,
            ee::s01::f_00144bf0__generated,
            0x0014_4bf0
        ),
        (
            oc978_fake_2c2540,
            OC978_PREV_2C2540,
            ee::s08::OVERRIDE_002C2540,
            ee::s08::f_002c2540__generated,
            0x002c_2540
        ),
        (
            oc978_fake_2c27c0,
            OC978_PREV_2C27C0,
            ee::s08::OVERRIDE_002C27C0,
            ee::s08::f_002c27c0__generated,
            0x002c_27c0
        ),
        (
            oc978_fake_3a0e90,
            OC978_PREV_3A0E90,
            ee::s13::OVERRIDE_003A0E90,
            ee::s13::f_003a0e90__generated,
            0x003a_0e90
        ),
        (
            oc978_fake_146f88,
            OC978_PREV_146F88,
            ee::s01::OVERRIDE_00146F88,
            ee::s01::f_00146f88__generated,
            0x0014_6f88
        ),
        (
            oc978_fake_146e98,
            OC978_PREV_146E98,
            ee::s01::OVERRIDE_00146E98,
            ee::s01::f_00146e98__generated,
            0x0014_6e98
        ),
        (
            oc978_fake_14ee58,
            OC978_PREV_14EE58,
            ee::s01::OVERRIDE_0014EE58,
            ee::s01::f_0014ee58__generated,
            0x0014_ee58
        ),
        (
            oc978_fake_3a0c50,
            OC978_PREV_3A0C50,
            ee::s13::OVERRIDE_003A0C50,
            ee::s13::f_003a0c50__generated,
            0x003a_0c50
        ),
        (
            oc978_fake_39e948,
            OC978_PREV_39E948,
            ee::s13::OVERRIDE_0039E948,
            ee::s13::f_0039e948__generated,
            0x0039_e948
        ),
        (
            oc978_fake_39e8b8,
            OC978_PREV_39E8B8,
            ee::s13::OVERRIDE_0039E8B8,
            ee::s13::f_0039e8b8__generated,
            0x0039_e8b8
        ),
        (
            oc978_fake_4166f4,
            OC978_PREV_4166F4,
            ee::s15::OVERRIDE_004166F4,
            ee::s15::f_004166f4__generated,
            0x0041_66f4
        ),
        (
            oc978_fake_2c26d0,
            OC978_PREV_2C26D0,
            ee::s08::OVERRIDE_002C26D0,
            ee::s08::f_002c26d0__generated,
            0x002c_26d0
        ),
        (
            oc978_fake_155130,
            OC978_PREV_155130,
            ee::s01::OVERRIDE_00155130,
            ee::s01::f_00155130__generated,
            0x0015_5130
        ),
        (
            oc978_fake_20a1a8,
            OC978_PREV_20A1A8,
            ee::s05::OVERRIDE_0020A1A8,
            ee::s05::f_0020a1a8__generated,
            0x0020_a1a8
        ),
        (
            oc978_fake_417828,
            OC978_PREV_417828,
            ee::s15::OVERRIDE_00417828,
            ee::s15::f_00417828__generated,
            0x0041_7828
        ),
    );

    /// `OVScreenMultiFreestylePre_Init`: both screens created or null, game
    /// mode 0..7 (the per-mode objective lines and hidden objects), game path
    /// (multiplayer or not), event kind 0..5, the interface vtable call of the
    /// location lookup, and unwinds at any call.
    #[test]
    fn ov_screen_multi_freestyle_pre_init_matches() {
        oc978_install_fakes();
        covered(
            ee::s04::f_001fc878__generated,
            ov_screen_multi_freestyle_pre_init,
            576,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1fc878, 45);
                c.pc = 0x001f_c878;
                c.bus.write32(OC978_CTRL, OC978_MAGIC);
                c.g[A0][0] = sx(OC978_STATE);
                c.bus.write32(OC978_STATE + 0x10, r.next_u32());
                c.bus.write32(GAME_GP - 0x848, OC978_WORLD);
                c.bus.write32(OC978_WORLD + 0x8c, OC978_IFACE);
                c.bus.write32(OC978_IFACE + 4, OC978_IFVT);
                c.bus
                    .write16(OC978_IFVT + 0x20, (r.next_u32() & 0xfc) as u16);
                c.bus.write32(OC978_IFVT + 0x24, fakes::FAKE + 0x40);
                c.bus.write8(OC978_NEWRACE + 0x48, (seed % 8) as u8);
                c.bus
                    .write8(OC978_NEWRACE + 0x49, [2u8, 0, 1][((seed / 8) % 3) as usize]);
                c.bus.write8(OC978_NEWRACE + 0x4a, ((seed / 24) % 6) as u8);
                for k in 0..4u32 {
                    let v = if (seed / 144) & 1 != 0 && k % 2 == 0 {
                        0
                    } else if (seed / 288) & 1 != 0 && k % 2 == 1 {
                        0
                    } else {
                        0x8000_0000 + 0x40 * k
                    };
                    c.bus.write32(OC978_CREATE + 4 * k, v);
                }
                c.bus.write32(OC978_CREATE + 0x20, 0);
            },
        );
    }

    pub(crate) const OC979_BASE: u32 = DATA + 0x20_0000;

    /// Gate word: the fakes only answer while this holds the magic.
    pub(crate) const OC979_CTRL: u32 = OC979_BASE;

    pub(crate) const OC979_MAGIC: u32 = 0x4f43_3739;

    /// Per forced fake: its call counter (4 bytes each).
    pub(crate) const OC979_CNT: u32 = OC979_BASE + 0x100;

    /// Per forced fake: its answers, 0x800 bytes each (call n answers entry
    /// `n & 0x1ff`).
    pub(crate) const OC979_RES: u32 = OC979_BASE + 0x1000;

    pub(crate) const OC979_THIS: u32 = OC979_BASE + 0x9000;

    pub(crate) const OC979_OUT: u32 = OC979_BASE + 0x9400;

    pub(crate) const OC979_WORLD: u32 = OC979_BASE + 0xa000;

    pub(crate) const OC979_STRTAB: u32 = OC979_BASE + 0xa400;

    pub(crate) const OC979_STRVT: u32 = OC979_BASE + 0xa800;

    /// The reward interface (+0x10: money) and the second, platinum one
    /// (+0x14: platinum flag).
    pub(crate) const OC979_IFACE0: u32 = OC979_BASE + 0xb000;

    pub(crate) const OC979_IFACE1: u32 = OC979_BASE + 0xb100;

    /// The bolts database: header (+4: entries) and four 0x38 byte entries.
    pub(crate) const OC979_DB: u32 = OC979_BASE + 0xc000;

    pub(crate) const OC979_DBROWS: u32 = OC979_BASE + 0xc100;

    // Forced results (index into the answer tables).
    pub(crate) const F_IFACE: u32 = 0;
    // BELibrary_GetInterface
    pub(crate) const F_COUNT: u32 = 1;
    // CountNewRewards
    pub(crate) const F_RCOUNT: u32 = 2;
    // GetNewRewardCount
    pub(crate) const F_TESTBIT: u32 = 3;
    // NewRewardsTestBit
    pub(crate) const F_BOLTDB: u32 = 4;
    // Bolts_GetDB
    pub(crate) const F_BOLTIDX: u32 = 5;
    // Bolts_FindBoltIndex
    pub(crate) const F_NONE: u32 = 99;

    /// The fake callee: logs like `fakes::fake_call`; the forced ones answer
    /// from their table in RAM.
    pub(crate) fn oc979_fake_call(c: &mut Cpu, addr: u32, which: u32) {
        let ra = c.g[RA][0] as u32;
        let call = if which < 8 {
            let n = c.bus.read32(OC979_CNT + 4 * which);
            c.bus.write32(OC979_CNT + 4 * which, n + 1);
            Some(n)
        } else {
            None
        };
        fakes::fake_call(c, addr);
        if c.pc != ra {
            return;
        }
        if let Some(n) = call {
            let v = c.bus.read32(OC979_RES + which * 0x800 + 4 * (n & 0x1ff));
            c.g[V0][0] = sx(v);
        }
    }

    macro_rules! oc979_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr, $which:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC979_CTRL) != OC979_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc979_fake_call(c, $addr, $which);
                }
            )*
            fn oc979_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc979_fakes!(
        (
            oc979_fake_14dc80,
            OC979_PREV_14DC80,
            ee::s01::OVERRIDE_0014DC80,
            ee::s01::f_0014dc80__generated,
            0x0014_dc80,
            F_NONE
        ),
        (
            oc979_fake_14dd58,
            OC979_PREV_14DD58,
            ee::s01::OVERRIDE_0014DD58,
            ee::s01::f_0014dd58__generated,
            0x0014_dd58,
            F_IFACE
        ),
        (
            oc979_fake_158f20,
            OC979_PREV_158F20,
            ee::s01::OVERRIDE_00158F20,
            ee::s01::f_00158f20__generated,
            0x0015_8f20,
            F_NONE
        ),
        (
            oc979_fake_15a478,
            OC979_PREV_15A478,
            ee::s01::OVERRIDE_0015A478,
            ee::s01::f_0015a478__generated,
            0x0015_a478,
            F_NONE
        ),
        (
            oc979_fake_15a6f0,
            OC979_PREV_15A6F0,
            ee::s01::OVERRIDE_0015A6F0,
            ee::s01::f_0015a6f0__generated,
            0x0015_a6f0,
            F_COUNT
        ),
        (
            oc979_fake_15a750,
            OC979_PREV_15A750,
            ee::s01::OVERRIDE_0015A750,
            ee::s01::f_0015a750__generated,
            0x0015_a750,
            F_RCOUNT
        ),
        (
            oc979_fake_15a768,
            OC979_PREV_15A768,
            ee::s01::OVERRIDE_0015A768,
            ee::s01::f_0015a768__generated,
            0x0015_a768,
            F_NONE
        ),
        (
            oc979_fake_15a5f0,
            OC979_PREV_15A5F0,
            ee::s01::OVERRIDE_0015A5F0,
            ee::s01::f_0015a5f0__generated,
            0x0015_a5f0,
            F_TESTBIT
        ),
        (
            oc979_fake_15af40,
            OC979_PREV_15AF40,
            ee::s01::OVERRIDE_0015AF40,
            ee::s01::f_0015af40__generated,
            0x0015_af40,
            F_NONE
        ),
        (
            oc979_fake_317670,
            OC979_PREV_317670,
            ee::s10::OVERRIDE_00317670,
            ee::s10::f_00317670__generated,
            0x0031_7670,
            F_NONE
        ),
        (
            oc979_fake_2c2508,
            OC979_PREV_2C2508,
            ee::s08::OVERRIDE_002C2508,
            ee::s08::f_002c2508__generated,
            0x002c_2508,
            F_NONE
        ),
        (
            oc979_fake_198af0,
            OC979_PREV_198AF0,
            ee::s03::OVERRIDE_00198AF0,
            ee::s03::f_00198af0__generated,
            0x0019_8af0,
            F_NONE
        ),
        (
            oc979_fake_2c26d0,
            OC979_PREV_2C26D0,
            ee::s08::OVERRIDE_002C26D0,
            ee::s08::f_002c26d0__generated,
            0x002c_26d0,
            F_NONE
        ),
        (
            oc979_fake_2c2718,
            OC979_PREV_2C2718,
            ee::s08::OVERRIDE_002C2718,
            ee::s08::f_002c2718__generated,
            0x002c_2718,
            F_NONE
        ),
        (
            oc979_fake_2c27c0,
            OC979_PREV_2C27C0,
            ee::s08::OVERRIDE_002C27C0,
            ee::s08::f_002c27c0__generated,
            0x002c_27c0,
            F_NONE
        ),
        (
            oc979_fake_14a130,
            OC979_PREV_14A130,
            ee::s01::OVERRIDE_0014A130,
            ee::s01::f_0014a130__generated,
            0x0014_a130,
            F_NONE
        ),
        (
            oc979_fake_14a080,
            OC979_PREV_14A080,
            ee::s01::OVERRIDE_0014A080,
            ee::s01::f_0014a080__generated,
            0x0014_a080,
            F_NONE
        ),
        (
            oc979_fake_14bdb8,
            OC979_PREV_14BDB8,
            ee::s01::OVERRIDE_0014BDB8,
            ee::s01::f_0014bdb8__generated,
            0x0014_bdb8,
            F_BOLTDB
        ),
        (
            oc979_fake_14d448,
            OC979_PREV_14D448,
            ee::s01::OVERRIDE_0014D448,
            ee::s01::f_0014d448__generated,
            0x0014_d448,
            F_BOLTIDX
        ),
    );

    /// Answer `n` of forced fake `which`.
    pub(crate) fn oc979_put(c: &mut Cpu, which: u32, n: u32, v: u32) {
        c.bus.write32(OC979_RES + which * 0x800 + 4 * n, v);
    }

    /// A reward list fixture. The seed picks a scenario (`seed % 10`) that
    /// forces one line kind: 0 money, 1 cheat character, 2 accessory / board
    /// graphic, 3 reward type 2, 4 other bit-tested type, 5 category header,
    /// 6 header of a platinum category, 7 "count only" (-1) or an index past
    /// the end, 8 and 9 random lists. The reward counts of category k, type j
    /// are answer `k * 10 + j` of `GetNewRewardCount`.
    pub(crate) fn oc979_setup(c: &mut Cpu, seed: u32, calls: u32) {
        let mut r = machine(c, seed, 0x1f_f7b8, calls);
        c.pc = 0x001f_f7b8;
        c.bus.write32(OC979_CTRL, OC979_MAGIC);
        let scen = seed % 10;
        let mut money = 0u32;
        let mut count = 1 + r.next_u32() % 3;
        let mut index = 1u32;
        let mut cnt = [0u32; 512];
        let mut bit = [0u32; 512];
        let random_lists = matches!(scen, 0 | 7 | 8 | 9);
        for k in 0..512usize {
            let j = k % 10;
            let x = r.next_u32();
            let small = 1 + x / 16 % 3;
            if random_lists {
                cnt[k] = if j == 8 {
                    if x % 10 < 3 {
                        10 + x / 16 % 3
                    } else {
                        x / 16 % 3
                    }
                } else if j == 9 {
                    if x % 10 < 5 {
                        small
                    } else {
                        0
                    }
                } else if x % 10 < 3 {
                    small
                } else {
                    0
                };
            }
            bit[k] = (r.next_u32() % 10 < 7) as u32;
        }
        match scen {
            0 => {
                money = 1 + r.next_u32() % 0xff;
                index = 0;
            }
            1 => {
                cnt[8] = 10 + r.next_u32() % 3;
            }
            2 => {
                cnt[9] = 1 + r.next_u32() % 3;
            }
            3 => {
                cnt[2] = 1;
                bit[0] = 1;
            }
            4 => {
                cnt[1] = 1 + r.next_u32() % 2;
                bit[0] = 1;
            }
            5 => {
                cnt[3] = 1;
                bit[0] = 1;
                index = 0;
            }
            6 => {
                let cat = 17 + r.next_u32() % 3;
                cnt[(cat * 10 + 3) as usize] = 1;
                bit[0] = 1;
                index = 0;
            }
            7 => {
                money = if r.next_u32() % 3 == 0 {
                    1 + r.next_u32() % 0xff
                } else {
                    0
                };
                index = if (seed / 10) % 2 == 0 { u32::MAX } else { 1000 };
            }
            _ => {
                if r.next_u32() % 2 == 0 {
                    money = 1 + r.next_u32() % 0xff;
                }
                index = if r.next_u32() % 8 == 0 {
                    u32::MAX
                } else {
                    r.next_u32() % 14
                };
                if r.next_u32() % 5 == 0 {
                    count = 0;
                }
            }
        }
        if scen == 0 || scen == 7 {
            count = 1 + r.next_u32() % 3;
        }
        for k in 0..512u32 {
            oc979_put(c, F_RCOUNT, k, cnt[k as usize]);
            oc979_put(c, F_TESTBIT, k, bit[k as usize]);
        }
        oc979_put(c, F_COUNT, 0, count);
        // Interfaces: the first has the money, the second the platinum flag.
        oc979_put(c, F_IFACE, 0, OC979_IFACE0);
        let second = if r.next_u32() % 3 == 0 {
            0
        } else {
            OC979_IFACE1
        };
        oc979_put(c, F_IFACE, 1, second);
        c.bus.write32(OC979_IFACE0 + 0x10, money);
        c.bus.write32(
            OC979_IFACE1 + 0x14,
            if r.next_u32() % 2 == 0 { 0 } else { 5 },
        );
        // Bolts database.
        oc979_put(c, F_BOLTDB, 0, OC979_DB);
        c.bus.write32(OC979_DB + 4, OC979_DBROWS);
        for row in 0..4u32 {
            let flags = r.next_u32();
            let flags = if r.next_u32() % 2 == 0 {
                flags | 0x800
            } else {
                flags & !0x800
            };
            c.bus.write32(OC979_DBROWS + 0x38 * row + 0x34, flags);
            c.bus
                .write32(OC979_DBROWS + 0x38 * row + 0x14, r.next_u32());
        }
        let found = if r.next_u32() % 3 == 0 {
            u32::MAX
        } else {
            r.next_u32() % 4
        };
        oc979_put(c, F_BOLTIDX, 0, found);
        // The string table the text lookups go through.
        c.bus.write32(GAME_GP - 0x848, OC979_WORLD);
        c.bus.write32(OC979_WORLD + 0x8c, OC979_STRTAB);
        c.bus.write32(OC979_STRTAB + 4, OC979_STRVT);
        c.bus
            .write16(OC979_STRVT + 0x20, (r.next_u32() & 0xfc) as u16);
        c.bus.write32(OC979_STRVT + 0x24, fakes::FAKE + 0x40);
        c.g[A0][0] = sx(OC979_THIS);
        c.g[A1][0] = sx(OC979_OUT);
        c.g[A2][0] = sx(index);
    }

    /// `OVScreenRewardList_GetEntryText`: every kind of line (money, the
    /// cheat character, accessory / board graphic, reward types 2 and the
    /// others, category and platinum headers, the count-only and the
    /// no-entry endings) against the generated body.
    #[test]
    fn ov_screen_reward_list_get_entry_text_matches() {
        oc979_install_fakes();
        check_reaching(
            ee::s05::f_001ff7b8__generated,
            ov_screen_reward_list_get_entry_text,
            400,
            &[
                0x001f_f888, // money line
                0x001f_f970, // cheat character
                0x001f_f9c0, // accessory
                0x001f_fa74, // accessory sprintf
                0x001f_fafc, // reward type 2
                0x001f_fc70, // other reward types
                0x001f_fb68, // category header
                0x001f_fca4, // header text lookup
                0x001f_fcd8, // no entry text
            ],
            &|c, seed| oc979_setup(c, seed, 0),
        );
    }

    /// The same lists with a callee unwinding at a random call.
    #[test]
    fn ov_screen_reward_list_get_entry_text_unwinds() {
        oc979_install_fakes();
        covered(
            ee::s05::f_001ff7b8__generated,
            ov_screen_reward_list_get_entry_text,
            300,
            &|c, seed| oc979_setup(c, seed, 40),
        );
    }

    // -- 0x001fd320 OVScreenRivalPre_TransitionIn ----------------------------

    pub(crate) fn rival_pre_setup(c: &mut Cpu, seed: u32) {
        let mut r = oc982_machine(c, seed, 0x1fd320, 90);
        oc982_world(c, &mut r);
        // Event type 4..11 (rival time/points, the peak events) and others.
        let ev = [4u8, 5, 6, 7, 8, 9, 10, 11, 0, 12][seed as usize % 10];
        c.bus.write8(OC982_STATE + 0x4a, ev);
        c.bus.write8(OC982_STATE + 0x49, ((seed / 10) % 3) as u8);
        c.bus.write8(OC982_STATE + 0x48, ((seed / 3) % 4) as u8);
        c.bus.write32(OC982_SELF + 0x40, DATA + 0x5_a000);
        // The game-mode manager: times and scores read from it.
        let value = match seed % 4 {
            0 => 0,
            1 => r.next_u32() % 100_000,
            2 => 0x8000_0000 | r.next_u32(),
            _ => r.next_u32() % 100,
        };
        c.bus.write32(OC982_MODEMAN + 0x78, value);
        c.bus.write32(OC982_MODEMAN + 0x7c, r.next_u32() % 100_000);
        c.g[A0][0] = OC982_SELF as u64;
        c.g[A1][0] = if seed % 7 == 0 { 0 } else { 1 };
        c.pc = 0x001f_d320;
    }

    /// OVScreenRivalPre_TransitionIn over the event types, the state flags and
    /// time or score values (including a negative one for the unsigned float
    /// conversion), with and without the message argument; callees faked.
    #[test]
    fn ov_screen_rival_pre_transition_in_matches() {
        check(
            ee::s05::f_001fd320__generated,
            ov_screen_rival_pre_transition_in,
            140,
            &rival_pre_setup,
        );
        assert!(
            oc982_path_count(ee::s05::f_001fd320__generated, 140, &rival_pre_setup) >= 6,
            "fixture reaches too few paths"
        );
    }

    pub(crate) const OC983_VT: u32 = DATA + 0x3800;

    /// `OVScreenSaveReplay_v26(self)`: the save-replay screen's status display
    /// (memory card present / formatted / full texts through `sprintf`), reading
    /// the memory-card manager and the screen's vtable calls.
    pub(crate) fn oc983_save_replay_setup(c: &mut Cpu, seed: u32) {
        let _r = oc983_fixture(c, seed, 0x1fdf40, 30);
        c.pc = 0x001f_df40;
        c.g[A0][0] = sx(OC983_SELF);
        c.bus.write32(OC983_SELF + 8, OC983_VT);
    }

    #[test]
    fn ov_screen_save_replay_v26_matches() {
        check(
            ee::s05::f_001fdf40__generated,
            ov_screen_save_replay_v26,
            300,
            &oc983_save_replay_setup,
        );
        let n = oc983_paths(
            ee::s05::f_001fdf40__generated,
            300,
            &oc983_save_replay_setup,
        );
        assert!(n >= 6, "only {n} distinct paths");
    }

    // The callees are replaced by gated fakes (live only on a machine whose
    // RAM holds `OC986_MAGIC` at `OC986_CTRL`). A fake logs its arguments
    // (RAM compare), returns the next scripted value for its target and may
    // unwind at a chosen call index.
    pub(crate) const OC986_CTRL: u32 = DATA + 0xa_0000;

    pub(crate) const OC986_MAGIC: u32 = 0x0c98_6a11;

    pub(crate) const OC986_COUNT: u32 = OC986_CTRL + 4;

    pub(crate) const OC986_UNWIND: u32 = OC986_CTRL + 8;

    pub(crate) const OC986_PER: u32 = OC986_CTRL + 0x100;

    pub(crate) const OC986_RES: u32 = OC986_CTRL + 0x200;

    pub(crate) const OC986_LOG: u32 = OC986_CTRL + 0x2000;

    /// Shared vtable-ish record of every fixture widget.
    pub(crate) const OC986_VT: u32 = DATA + 0x3000;

    pub(crate) fn oc986_fake_body(c: &mut Cpu, target: u32) {
        let n = c.bus.read32(OC986_COUNT);
        c.bus.write32(OC986_COUNT, n.wrapping_add(1));
        let rec = OC986_LOG + 0x40 * (n & 63);
        let regs = [A0, A1, A2, A3, T0, V1, S0, S1, S2, RA];
        c.bus.write32(rec, target);
        for (i, r) in regs.into_iter().enumerate() {
            let v = c.g[r][0] as u32;
            c.bus.write32(rec + 4 + 4 * i as u32, v);
        }
        let k = c.bus.read32(OC986_PER + 4 * target);
        c.bus.write32(OC986_PER + 4 * target, k.wrapping_add(1));
        let res = c.bus.read32(OC986_RES + 0x80 * target + 4 * (k & 31));
        c.g[V0][0] = sx(res);
        c.cycles += 5;
        let ra = c.g[RA][0] as u32;
        c.pc = if n == c.bus.read32(OC986_UNWIND) {
            0xdead_0000 | n << 4
        } else {
            ra
        };
    }

    macro_rules! oc986_fakes {
        ($(($fake:ident, $prev:ident, $idx:expr, $slot:expr, $generated:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC986_CTRL) != OC986_MAGIC {
                        return fakes::fall_back(c, &$prev, $generated);
                    }
                    oc986_fake_body(c, $idx);
                }
            )*
            pub(crate) fn oc986_install() {
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    pub(crate) const OC986_T_145870: u32 = 0;

    pub(crate) const OC986_T_146E98: u32 = 1;

    pub(crate) const OC986_T_147398: u32 = 2;

    pub(crate) const OC986_T_14DC80: u32 = 3;

    pub(crate) const OC986_T_14DD58: u32 = 4;

    pub(crate) const OC986_T_157B08: u32 = 5;

    pub(crate) const OC986_T_158910: u32 = 6;

    pub(crate) const OC986_T_317670: u32 = 7;

    pub(crate) const OC986_T_397CC0: u32 = 8;

    pub(crate) const OC986_T_39C870: u32 = 9;

    pub(crate) const OC986_T_39D860: u32 = 10;

    pub(crate) const OC986_T_3A0C50: u32 = 11;

    pub(crate) const OC986_T_3A1000: u32 = 12;

    pub(crate) const OC986_T_20A380: u32 = 13;

    pub(crate) const OC986_T_2583A8: u32 = 14;

    pub(crate) const OC986_T_23CAE8: u32 = 15;

    pub(crate) const OC986_T_23D570: u32 = 16;

    pub(crate) const OC986_T_270ED8: u32 = 17;

    pub(crate) const OC986_T_417828: u32 = 18;

    pub(crate) const OC986_T_227F80: u32 = 19;

    pub(crate) const OC986_T_1FB378: u32 = 20;

    pub(crate) const OC986_T_1FAFF8: u32 = 21;

    oc986_fakes! {
        (oc986_fake_145870, OC986_PREV_145870, OC986_T_145870, ee::s01::OVERRIDE_00145870, ee::s01::f_00145870__generated),
        (oc986_fake_146e98, OC986_PREV_146E98, OC986_T_146E98, ee::s01::OVERRIDE_00146E98, ee::s01::f_00146e98__generated),
        (oc986_fake_147398, OC986_PREV_147398, OC986_T_147398, ee::s01::OVERRIDE_00147398, ee::s01::f_00147398__generated),
        (oc986_fake_14dc80, OC986_PREV_14DC80, OC986_T_14DC80, ee::s01::OVERRIDE_0014DC80, ee::s01::f_0014dc80__generated),
        (oc986_fake_14dd58, OC986_PREV_14DD58, OC986_T_14DD58, ee::s01::OVERRIDE_0014DD58, ee::s01::f_0014dd58__generated),
        (oc986_fake_157b08, OC986_PREV_157B08, OC986_T_157B08, ee::s01::OVERRIDE_00157B08, ee::s01::f_00157b08__generated),
        (oc986_fake_158910, OC986_PREV_158910, OC986_T_158910, ee::s01::OVERRIDE_00158910, ee::s01::f_00158910__generated),
        (oc986_fake_317670, OC986_PREV_317670, OC986_T_317670, ee::s10::OVERRIDE_00317670, ee::s10::f_00317670__generated),
        (oc986_fake_397cc0, OC986_PREV_397CC0, OC986_T_397CC0, ee::s13::OVERRIDE_00397CC0, ee::s13::f_00397cc0__generated),
        (oc986_fake_39c870, OC986_PREV_39C870, OC986_T_39C870, ee::s13::OVERRIDE_0039C870, ee::s13::f_0039c870__generated),
        (oc986_fake_39d860, OC986_PREV_39D860, OC986_T_39D860, ee::s13::OVERRIDE_0039D860, ee::s13::f_0039d860__generated),
        (oc986_fake_3a0c50, OC986_PREV_3A0C50, OC986_T_3A0C50, ee::s13::OVERRIDE_003A0C50, ee::s13::f_003a0c50__generated),
        (oc986_fake_3a1000, OC986_PREV_3A1000, OC986_T_3A1000, ee::s13::OVERRIDE_003A1000, ee::s13::f_003a1000__generated),
        (oc986_fake_20a380, OC986_PREV_20A380, OC986_T_20A380, ee::s05::OVERRIDE_0020A380, ee::s05::f_0020a380__generated),
        (oc986_fake_2583a8, OC986_PREV_2583A8, OC986_T_2583A8, ee::s06::OVERRIDE_002583A8, ee::s06::f_002583a8__generated),
        (oc986_fake_23cae8, OC986_PREV_23CAE8, OC986_T_23CAE8, ee::s06::OVERRIDE_0023CAE8, ee::s06::f_0023cae8__generated),
        (oc986_fake_23d570, OC986_PREV_23D570, OC986_T_23D570, ee::s06::OVERRIDE_0023D570, ee::s06::f_0023d570__generated),
        (oc986_fake_270ed8, OC986_PREV_270ED8, OC986_T_270ED8, ee::s07::OVERRIDE_00270ED8, ee::s07::f_00270ed8__generated),
        (oc986_fake_417828, OC986_PREV_417828, OC986_T_417828, ee::s15::OVERRIDE_00417828, ee::s15::f_00417828__generated),
        (oc986_fake_227f80, OC986_PREV_227F80, OC986_T_227F80, ee::s05::OVERRIDE_00227F80, ee::s05::f_00227f80__generated),
        (oc986_fake_1fb378, OC986_PREV_1FB378, OC986_T_1FB378, ee::s04::OVERRIDE_001FB378, ee::s04::f_001fb378__generated),
        (oc986_fake_1faff8, OC986_PREV_1FAFF8, OC986_T_1FAFF8, ee::s04::OVERRIDE_001FAFF8, ee::s04::f_001faff8__generated),
    }

    /// Fill the 32 scripted results of one callee (repeating).
    pub(crate) fn oc986_results(c: &mut Cpu, target: u32, f: &mut dyn FnMut(u32) -> u32) {
        for k in 0..32 {
            let v = f(k);
            c.bus.write32(OC986_RES + 0x80 * target + 4 * k, v);
        }
    }

    /// Scrambled machine with the fakes armed; one call in four unwinds at
    /// a call index below `fake_calls` (the machine's own miss hook, for
    /// vtable calls, unwinds among the first `miss_calls`).
    pub(crate) fn oc986_machine(
        c: &mut Cpu,
        seed: u32,
        salt: u64,
        miss_calls: u32,
        fake_calls: u32,
    ) -> Rng {
        let mut r = machine(c, seed, salt, miss_calls);
        elf_image(c);
        c.bus.write32(OC986_CTRL, OC986_MAGIC);
        let at = if seed % 4 == 1 {
            r.next_u32() % fake_calls
        } else {
            u32::MAX
        };
        c.bus.write32(OC986_UNWIND, at);
        c.bus.write16(OC986_VT + 0x48, ((seed % 3) * 0x10) as u16);
        c.bus.write32(OC986_VT + 0x4c, FAKE);
        r
    }

    /// A fixture widget: `+8` points at the shared record, flags random.
    pub(crate) fn oc986_widget(c: &mut Cpu, w: u32, r: &mut Rng) {
        c.bus.write32(w + 8, OC986_VT);
        c.bus.write32(w + 0x14, r.next_u32());
        c.bus.write32(w + 0x18, r.next_u32());
    }

    /// 0 or a widget, per call of `UIScreen_FindObject`: `zero_mod` is the
    /// period at which a result is null (0: never).
    pub(crate) fn oc986_find_results(c: &mut Cpu, r: &mut Rng, zero_mod: u32) {
        let mut ws = Vec::new();
        for k in 0..32u32 {
            let w = DATA + 0x1_0000 + 0x100 * k;
            oc986_widget(c, w, r);
            let null = zero_mod != 0 && r.next_u32() % zero_mod == 0;
            ws.push(if null { 0 } else { w });
        }
        oc986_results(c, OC986_T_39D860, &mut |k| ws[k as usize]);
    }

    /// `InGameError_Init`: the in-game error pop-up (cheat/lost-connection text).
    #[test]
    fn oc986_ingame_error_init_matches() {
        oc986_install();
        check(
            ee::s04::f_001fab18__generated,
            ingame_error_init,
            200,
            &|c, seed| {
                let mut r = oc986_machine(c, seed, 0x1fab18, 12, 70);
                let this = DATA + 0x1000;
                c.bus.write32(this + 0x10, DATA + 0x1800);
                let comm = DATA + 0x5000;
                let cheat0 = [0u32, 1, 2, 3, 7][(seed % 5) as usize];
                let cheat1 = [0u32, 1, 2, 3, 9][((seed / 5) % 5) as usize];
                c.bus.write32(comm + 0x24, cheat0);
                c.bus.write32(comm + 0x28, cheat1);
                c.bus.write32(comm + 0x8c, (seed / 25) & 1);
                c.bus.write32(comm + 0x14, (seed / 50) & 1);
                c.bus.write32(comm + 0xa0, (seed / 100) & 1);
                c.bus.write32(
                    GAME_GP.wrapping_sub(0x204),
                    if seed % 17 == 0 { 0 } else { comm },
                );
                c.bus.write32(GAME_GP.wrapping_sub(0xc8), DATA + 0x5400);
                c.bus.write32(DATA + 0x5404, r.next_u32());
                c.bus.write32(GAME_GP.wrapping_add(0x2a74), DATA + 0x5600);
                c.bus.write32(DATA + 0x5618, r.next_u32());
                oc986_results(c, OC986_T_317670, &mut |_| r.next_u32() | 1);
                let screen = if seed % 13 == 0 { 0 } else { DATA + 0x2000 };
                oc986_results(c, OC986_T_397CC0, &mut |_| screen);
                oc986_find_results(c, &mut r, 6);
                c.g[A0][0] = sx(this);
            },
        );
    }

    /// `MultiFreestylePre_TransitionIn`: layout shift by game mode.
    #[test]
    fn oc986_mfpre_transition_in_matches() {
        oc986_install();
        check(
            ee::s04::f_001fcec0__generated,
            mfpre_transition_in,
            80,
            &|c, seed| {
                let mut r = oc986_machine(c, seed, 0x1fcec0, 0, 10);
                let this = DATA + 0x1000;
                c.bus.write32(this + 0x40, DATA + 0x2000);
                c.bus.write8(0x0053_5c10, (seed % 8) as u8);
                oc986_results(c, OC986_T_14DC80, &mut |k| DATA + 0x2400 + 0x10 * k);
                oc986_results(c, OC986_T_14DD58, &mut |k| DATA + 0x2800 + 0x10 * k);
                oc986_results(c, OC986_T_317670, &mut |_| r.next_u32() | 1);
                let mut ws = Vec::new();
                for k in 0..32u32 {
                    let w = DATA + 0x1_0000 + 0x100 * k;
                    for off in [0x44u32, 0x48, 0x4c] {
                        let v = r.edge_f32();
                        c.bus.write32(w + off, v);
                    }
                    ws.push(w);
                }
                oc986_results(c, OC986_T_39D860, &mut |k| ws[k as usize]);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = if seed % 9 == 0 { 0 } else { sx(DATA + 0x1c00) };
            },
        );
    }

    /// `SaveReplay_Init`: the save-replay screen and its card-slot labels.
    #[test]
    fn oc986_savereplay_init_matches() {
        oc986_install();
        check(
            ee::s05::f_001fdc30__generated,
            savereplay_init,
            120,
            &|c, seed| {
                let mut r = oc986_machine(c, seed, 0x1fdc30, 12, 60);
                let this = DATA + 0x1000;
                c.bus.write32(this + 0x10, DATA + 0x1800);
                c.bus.write32(GAME_GP.wrapping_sub(0x69c), (seed * 7) % 20);
                oc986_results(c, OC986_T_317670, &mut |_| r.next_u32() | 1);
                // First screen null every fifth seed, the overlay every third.
                oc986_results(c, OC986_T_397CC0, &mut |k| {
                    let null = if k % 2 == 0 {
                        seed % 5 == 0
                    } else {
                        seed % 3 == 0
                    };
                    if null {
                        0
                    } else {
                        DATA + 0x2000 + 0x100 * k
                    }
                });
                oc986_find_results(c, &mut r, 0);
                c.g[A0][0] = sx(this);
            },
        );
    }

    /// `SaveReplay_OnWidgetAction`: confirm / stop / save events (5, 6, 7).
    #[test]
    fn oc986_savereplay_widget_action_matches() {
        oc986_install();
        check(
            ee::s05::f_001fe6e8__generated,
            savereplay_widget_action,
            220,
            &|c, seed| {
                let mut r = oc986_machine(c, seed, 0x1fe6e8, 6, 6);
                let this = DATA + 0x1000;
                let msg = DATA + 0x1c00;
                let world = DATA + 0x5000;
                c.bus.write32(GAME_GP.wrapping_sub(0x848), world);
                c.bus.write32(world + 0x84, DATA + 0x5100);
                c.bus.write32(DATA + 0x5128, r.next_u32());
                let ty = seed % 9;
                c.bus.write32(this + 0x1a8, (r.next_u32() % 8 == 0) as u32);
                let state = match ty {
                    7 if r.next_u32() % 4 != 0 => 1,
                    _ => r.next_u32() % 8,
                };
                c.bus.write32(this + 0x1c0, state);
                c.bus.write32(this + 0x1c8, r.next_u32() % 3);
                c.bus
                    .write32(this + 0x214, if r.next_u32() % 4 != 0 { 3 } else { 5 });
                c.bus.write32(msg + 0x18, r.next_u32());
                // The memory-card manager and its virtual methods: the call
                // targets are `FAKE` (answers non-zero) or a real leaf that
                // returns 0 (0x1406c0).
                let mp = DATA + 0x6000;
                let card = DATA + 0x6400;
                let card_vt = DATA + 0x6800;
                let obj = DATA + 0x6c00;
                c.bus.write32(mp + 0xf8, r.next_u32());
                c.bus.write32(mp + 0x428, r.next_u32());
                c.bus.write32(mp + 0x434, card);
                c.bus.write32(mp + 0x748, obj);
                c.bus.write32(card, card_vt);
                c.bus.write16(card_vt + 0x1b0, 0x10);
                c.bus.write32(
                    card_vt + 0x1b4,
                    if r.next_u32() % 4 == 0 {
                        0x0014_06c0
                    } else {
                        FAKE
                    },
                );
                c.bus.write16(card_vt + 0x218, 0x20);
                c.bus.write32(
                    card_vt + 0x21c,
                    if r.next_u32() % 4 == 0 {
                        0x0014_06c0
                    } else {
                        FAKE
                    },
                );
                c.bus.write16(obj + 8, 0x30);
                c.bus.write32(obj + 0xc, FAKE);
                oc986_results(c, OC986_T_227F80, &mut |_| mp);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = if seed % 11 == 0 { 0 } else { sx(msg) };
                c.g[A2][0] = ty as u64;
            },
        );
    }

    /// Gate word, magic and forced-result mailbox of this module's callee fakes
    /// (machine RAM only: both sides of a diff get their own).
    pub(crate) const OC987_GATE: u32 = DATA + 0x20_0000;

    pub(crate) const OC987_MAGIC: u32 = 0x0987_0987;

    pub(crate) const OC987_MBOX: u32 = DATA + 0x20_0100;

    /// The screen object every test passes in a0.
    pub(crate) const OC987_OBJ: u32 = DATA + 0x1000;

    /// `m = (value << 1) | 1`: the value call `n` returns instead of the fake's pointer.
    pub(crate) fn oc987_enc(v: i32) -> u32 {
        ((v as u32) << 1) | 1
    }

    /// A logged fake call (the shared `fakes::fake`), then the mailbox override.
    pub(crate) fn oc987_fake_call(c: &mut Cpu, target: u32) {
        let n = c.bus.read32(fakes::LOG_COUNT);
        let ret = c.g[RA][0] as u32;
        fakes::fake(c, target, ret);
        if c.pc == ret && n < 128 {
            let m = c.bus.read32(OC987_MBOX + 4 * n);
            if m & 1 != 0 {
                c.g[V0][0] = sx(((m as i32) >> 1) as u32);
            }
        }
    }

    macro_rules! oc987_fake {
        ($name:ident, $prev:ident, $addr:expr, $generated:path) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $name(c: &mut Cpu) {
                if c.bus.read32(OC987_GATE) != OC987_MAGIC {
                    return fakes::fall_back(c, &$prev, $generated);
                }
                oc987_fake_call(c, $addr);
            }
        };
    }

    oc987_fake!(
        oc987_fake_00144bc0,
        OC987_PREV_00144BC0,
        0x00144bc0,
        ee::s01::f_00144bc0__generated
    );

    oc987_fake!(
        oc987_fake_00144bf0,
        OC987_PREV_00144BF0,
        0x00144bf0,
        ee::s01::f_00144bf0__generated
    );

    oc987_fake!(
        oc987_fake_00146e98,
        OC987_PREV_00146E98,
        0x00146e98,
        ee::s01::f_00146e98__generated
    );

    oc987_fake!(
        oc987_fake_0014dc80,
        OC987_PREV_0014DC80,
        0x0014dc80,
        ee::s01::f_0014dc80__generated
    );

    oc987_fake!(
        oc987_fake_0014dd58,
        OC987_PREV_0014DD58,
        0x0014dd58,
        ee::s01::f_0014dd58__generated
    );

    oc987_fake!(
        oc987_fake_0014ee58,
        OC987_PREV_0014EE58,
        0x0014ee58,
        ee::s01::f_0014ee58__generated
    );

    oc987_fake!(
        oc987_fake_001545f8,
        OC987_PREV_001545F8,
        0x001545f8,
        ee::s01::f_001545f8__generated
    );

    oc987_fake!(
        oc987_fake_001d8240,
        OC987_PREV_001D8240,
        0x001d8240,
        ee::s04::f_001d8240__generated
    );

    oc987_fake!(
        oc987_fake_001d97e0,
        OC987_PREV_001D97E0,
        0x001d97e0,
        ee::s04::f_001d97e0__generated
    );

    oc987_fake!(
        oc987_fake_001fd190,
        OC987_PREV_001FD190,
        0x001fd190,
        ee::s04::f_001fd190__generated
    );

    oc987_fake!(
        oc987_fake_001ff7b8,
        OC987_PREV_001FF7B8,
        0x001ff7b8,
        ee::s05::f_001ff7b8__generated
    );

    oc987_fake!(
        oc987_fake_00209e78,
        OC987_PREV_00209E78,
        0x00209e78,
        ee::s05::f_00209e78__generated
    );

    oc987_fake!(
        oc987_fake_0020a380,
        OC987_PREV_0020A380,
        0x0020a380,
        ee::s05::f_0020a380__generated
    );

    oc987_fake!(
        oc987_fake_00227f80,
        OC987_PREV_00227F80,
        0x00227f80,
        ee::s05::f_00227f80__generated
    );

    oc987_fake!(
        oc987_fake_00241710,
        OC987_PREV_00241710,
        0x00241710,
        ee::s06::f_00241710__generated
    );

    oc987_fake!(
        oc987_fake_0026b680,
        OC987_PREV_0026B680,
        0x0026b680,
        ee::s07::f_0026b680__generated
    );

    oc987_fake!(
        oc987_fake_002c2508,
        OC987_PREV_002C2508,
        0x002c2508,
        ee::s08::f_002c2508__generated
    );

    oc987_fake!(
        oc987_fake_002c2540,
        OC987_PREV_002C2540,
        0x002c2540,
        ee::s08::f_002c2540__generated
    );

    oc987_fake!(
        oc987_fake_002c26d0,
        OC987_PREV_002C26D0,
        0x002c26d0,
        ee::s08::f_002c26d0__generated
    );

    oc987_fake!(
        oc987_fake_002c27c0,
        OC987_PREV_002C27C0,
        0x002c27c0,
        ee::s08::f_002c27c0__generated
    );

    oc987_fake!(
        oc987_fake_00317670,
        OC987_PREV_00317670,
        0x00317670,
        ee::s10::f_00317670__generated
    );

    oc987_fake!(
        oc987_fake_00397cc0,
        OC987_PREV_00397CC0,
        0x00397cc0,
        ee::s13::f_00397cc0__generated
    );

    oc987_fake!(
        oc987_fake_0039b960,
        OC987_PREV_0039B960,
        0x0039b960,
        ee::s13::f_0039b960__generated
    );

    oc987_fake!(
        oc987_fake_0039c870,
        OC987_PREV_0039C870,
        0x0039c870,
        ee::s13::f_0039c870__generated
    );

    oc987_fake!(
        oc987_fake_0039d860,
        OC987_PREV_0039D860,
        0x0039d860,
        ee::s13::f_0039d860__generated
    );

    oc987_fake!(
        oc987_fake_003a0c50,
        OC987_PREV_003A0C50,
        0x003a0c50,
        ee::s13::f_003a0c50__generated
    );

    oc987_fake!(
        oc987_fake_003a0d00,
        OC987_PREV_003A0D00,
        0x003a0d00,
        ee::s13::f_003a0d00__generated
    );

    oc987_fake!(
        oc987_fake_003a0e90,
        OC987_PREV_003A0E90,
        0x003a0e90,
        ee::s13::f_003a0e90__generated
    );

    oc987_fake!(
        oc987_fake_003a1000,
        OC987_PREV_003A1000,
        0x003a1000,
        ee::s13::f_003a1000__generated
    );

    oc987_fake!(
        oc987_fake_00417828,
        OC987_PREV_00417828,
        0x00417828,
        ee::s15::f_00417828__generated
    );

    pub(crate) fn oc987_install() {
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_00144BC0),
            oc987_fake_00144bc0,
            &OC987_PREV_00144BC0,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_00144BF0),
            oc987_fake_00144bf0,
            &OC987_PREV_00144BF0,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_00146E98),
            oc987_fake_00146e98,
            &OC987_PREV_00146E98,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014DC80),
            oc987_fake_0014dc80,
            &OC987_PREV_0014DC80,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014DD58),
            oc987_fake_0014dd58,
            &OC987_PREV_0014DD58,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014EE58),
            oc987_fake_0014ee58,
            &OC987_PREV_0014EE58,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_001545F8),
            oc987_fake_001545f8,
            &OC987_PREV_001545F8,
        );
        fakes::hook(
            addr_of_mut!(ee::s04::OVERRIDE_001D8240),
            oc987_fake_001d8240,
            &OC987_PREV_001D8240,
        );
        fakes::hook(
            addr_of_mut!(ee::s04::OVERRIDE_001D97E0),
            oc987_fake_001d97e0,
            &OC987_PREV_001D97E0,
        );
        fakes::hook(
            addr_of_mut!(ee::s04::OVERRIDE_001FD190),
            oc987_fake_001fd190,
            &OC987_PREV_001FD190,
        );
        fakes::hook(
            addr_of_mut!(ee::s05::OVERRIDE_001FF7B8),
            oc987_fake_001ff7b8,
            &OC987_PREV_001FF7B8,
        );
        fakes::hook(
            addr_of_mut!(ee::s05::OVERRIDE_00209E78),
            oc987_fake_00209e78,
            &OC987_PREV_00209E78,
        );
        fakes::hook(
            addr_of_mut!(ee::s05::OVERRIDE_0020A380),
            oc987_fake_0020a380,
            &OC987_PREV_0020A380,
        );
        fakes::hook(
            addr_of_mut!(ee::s05::OVERRIDE_00227F80),
            oc987_fake_00227f80,
            &OC987_PREV_00227F80,
        );
        fakes::hook(
            addr_of_mut!(ee::s06::OVERRIDE_00241710),
            oc987_fake_00241710,
            &OC987_PREV_00241710,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_0026B680),
            oc987_fake_0026b680,
            &OC987_PREV_0026B680,
        );
        fakes::hook(
            addr_of_mut!(ee::s08::OVERRIDE_002C2508),
            oc987_fake_002c2508,
            &OC987_PREV_002C2508,
        );
        fakes::hook(
            addr_of_mut!(ee::s08::OVERRIDE_002C2540),
            oc987_fake_002c2540,
            &OC987_PREV_002C2540,
        );
        fakes::hook(
            addr_of_mut!(ee::s08::OVERRIDE_002C26D0),
            oc987_fake_002c26d0,
            &OC987_PREV_002C26D0,
        );
        fakes::hook(
            addr_of_mut!(ee::s08::OVERRIDE_002C27C0),
            oc987_fake_002c27c0,
            &OC987_PREV_002C27C0,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317670),
            oc987_fake_00317670,
            &OC987_PREV_00317670,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_00397CC0),
            oc987_fake_00397cc0,
            &OC987_PREV_00397CC0,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_0039B960),
            oc987_fake_0039b960,
            &OC987_PREV_0039B960,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_0039C870),
            oc987_fake_0039c870,
            &OC987_PREV_0039C870,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_0039D860),
            oc987_fake_0039d860,
            &OC987_PREV_0039D860,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003A0C50),
            oc987_fake_003a0c50,
            &OC987_PREV_003A0C50,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003A0D00),
            oc987_fake_003a0d00,
            &OC987_PREV_003A0D00,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003A0E90),
            oc987_fake_003a0e90,
            &OC987_PREV_003A0E90,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003A1000),
            oc987_fake_003a1000,
            &OC987_PREV_003A1000,
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00417828),
            oc987_fake_00417828,
            &OC987_PREV_00417828,
        );
    }

    /// Machine with the gate set and a random mailbox: about one call in six
    /// returns one of `forced` instead of a pointer.
    pub(crate) fn oc987_machine(
        c: &mut Cpu,
        seed: u32,
        salt: u64,
        calls: u32,
        forced: &[i32],
    ) -> Rng {
        oc987_install();
        let mut r = fakes::machine(c, seed, salt, calls);
        c.bus.write32(OC987_GATE, OC987_MAGIC);
        for n in 0..128u32 {
            if r.next_u32() % 6 == 3 {
                let v = forced[r.next_u32() as usize % forced.len()];
                c.bus.write32(OC987_MBOX + 4 * n, oc987_enc(v));
            }
        }
        c.g[A0][0] = sx(OC987_OBJ);
        r
    }

    /// Save Records screen build: null and non-null screens from the engine.
    #[test]
    fn ov_screen_save_records_init_matches() {
        covered(
            ee::s05::f_001fe9a8__generated,
            ov_screen_save_records_init,
            64,
            &|c, seed| {
                oc987_machine(c, seed, 0x1fe9a8, 60, &[0]);
            },
        );
    }

    /// Memory-card status refresh: `self+0x214 == 3` or not, card-manager
    /// answers forced to 0 / -1 (the `== -1` tests of `func_001D97E0`).
    #[test]
    fn ov_screen_save_records_v26_matches() {
        covered(
            ee::s05::f_001feca0__generated,
            ov_screen_save_records_v26,
            64,
            &|c, seed| {
                oc987_machine(c, seed, 0x1feca0, 40, &[0, -1, 1]);
                c.bus
                    .write32(OC987_OBJ + 0x214, if seed % 2 == 0 { 3 } else { 5 });
            },
        );
    }

    /// Reward list page: `which` 0 / not 0, row count below / above the page.
    #[test]
    fn ov_screen_reward_list_show_page_matches() {
        covered(
            ee::s05::f_001ffd08__generated,
            ov_screen_reward_list_show_page,
            64,
            &|c, seed| {
                oc987_machine(c, seed, 0x1ffd08, 40, &[0]);
                c.g[A1][0] = [0u64, 1, 2, 7][seed as usize % 4];
                c.bus.write32(
                    OC987_OBJ + 0x9c,
                    [3u32, 12, 20, 10][(seed as usize / 2) % 4],
                );
            },
        );
    }

    /// Reward list title: every event kind and mode of the jump table (the
    /// game image holds the table), done flag set or clear.
    #[test]
    fn ov_screen_reward_list_transition_in_matches() {
        covered(
            ee::s05::f_001fff78__generated,
            ov_screen_reward_list_transition_in,
            64,
            &|c, seed| {
                oc987_machine(c, seed, 0x1fff78, 30, &[0]);
                elf_image(c);
                c.pc = 0x001f_ff78;
                c.bus.write8(0x0053_5bc8 + 0x4a, (seed % 15) as u8);
                c.bus.write8(0x0053_5bc8 + 0x48, ((seed / 3) % 8) as u8);
                c.bus.write32(OC987_OBJ + 0x48, (seed % 5 == 4) as u32);
            },
        );
    }

    #[test]
    fn ov_screen_rival_pre_on_widget_action_matches() {
        check(
            ee::s05::f_001fdb88__generated,
            ov_screen_rival_pre_on_widget_action,
            40,
            &|c, seed| {
                let mut r = base_3_m2(c, seed, 0x1fdb);
                let world = c.bus.read32(GAME_GP.wrapping_sub(0x848));
                let game = c.bus.read32(world + 0x84);
                c.bus.write32(game + 0x200, DATA + 0x5000);
                c.bus.write32(DATA + 0x5008, 0);
                c.bus.write32(DATA + 0x10, DATA + 0x3000);
                fill(c, &mut r, DATA + 0x3000, 0x40);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = if seed % 3 == 0 { 0 } else { 1 + seed as u64 };
                c.g[A2][0] = if seed % 2 == 0 { 5 } else { (seed % 7) as u64 };
                fill(c, &mut r, DATA + 0x100, 0x40);
            },
        );
    }

    #[test]
    fn ov_screen_in_game_error_handle_select_matches() {
        check(
            ee::s04::f_001fb2b8__generated,
            ov_screen_in_game_error_handle_select,
            40,
            &|c, seed| {
                let mut r = fakes::machine_rt(c, seed, 0x1fb2, 0);
                let world = DATA + 0x1000;
                let game = DATA + 0x1200;
                c.bus.write32(GAME_GP.wrapping_sub(0x848), world);
                c.bus.write32(world + 0x84, game);
                c.bus.write32(game + 0x200, DATA + 0x5000);
                c.bus.write32(game + 0xc, DATA + 0x6000);
                c.bus.write32(GAME_GP.wrapping_sub(0x69c), 1 + seed % 3);
                let item = DATA + 0x800;
                c.bus
                    .write32(item + 0x18, [0, 1, 4, 2][(seed % 4) as usize]);
                c.bus.write32(DATA + 0x10, DATA + 0x3000);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(item);
                fill(c, &mut r, DATA + 0x100, 0x40);
            },
        );
    }

    #[test]
    fn ov_screen_be_options_update_matches() {
        check(
            ee::s04::f_001fa9e0__generated,
            ov_screen_be_options_update_2,
            40,
            &|c, seed| {
                let mut r = base_3_m2(c, seed, 0x1fa9);
                let table = DATA + 0x4000;
                c.bus.write32(
                    GAME_GP.wrapping_sub(0x204),
                    if seed % 5 == 0 { 0 } else { table },
                );
                c.bus
                    .write32(table + 0x68, if seed % 3 == 0 { 0 } else { 1 });
                c.bus
                    .write32(table + 0x64, if seed % 2 == 0 { 0 } else { 1 });
                c.bus.write32(DATA + 0x40, DATA + 0x5000);
                c.bus.write32(DATA + 0x10, DATA + 0x3000);
                c.g[A0][0] = sx(DATA);
                fill(c, &mut r, DATA + 0x100, 0x40);
            },
        );
    }

    /// Gate of this module's fakes: a word in the test machine's RAM. Only the
    /// machines of these tests carry it, so other tests run the real bodies.
    pub(crate) const OC991_GATE: u32 = DATA + 0x21_0000;

    pub(crate) const OC991_MAGIC: u32 = 0x0991_c0de;

    /// Mailbox: the result of the gated fake with slot `k` is the word at `OC991_MB + 4k`.
    pub(crate) const OC991_MB: u32 = DATA + 0x21_0100;

    /// Result of every indirect call the `miss` hook answers (table targets outside the image).
    pub(crate) const OC991_MISS: u32 = DATA + 0x21_0200;

    /// The `this` object of the overlay functions.
    pub(crate) const OC991_THIS: u32 = DATA + 0x4000;

    macro_rules! oc991_fake {
        ($name:ident, $target:expr, $gen:path) => {
            fn $name(c: &mut Cpu) {
                if c.bus.read32(OC991_GATE) != OC991_MAGIC {
                    return fakes::fall_back(c, fakes::prev_cell($name), $gen);
                }
                fakes::fake_call(c, $target);
            }
        };
        ($name:ident, $target:expr, $gen:path, $slot:expr) => {
            fn $name(c: &mut Cpu) {
                if c.bus.read32(OC991_GATE) != OC991_MAGIC {
                    return fakes::fall_back(c, fakes::prev_cell($name), $gen);
                }
                fakes::fake_call(c, $target);
                c.g[V0][0] = c.bus.read32(OC991_MB + 4 * $slot) as u64;
            }
        };
    }

    oc991_fake!(
        oc991_fake_str_hash,
        0x0031_7670,
        ee::s10::f_00317670__generated
    );

    oc991_fake!(
        oc991_fake_find_object,
        0x0039_d860,
        ee::s13::f_0039d860__generated,
        0
    );

    oc991_fake!(
        oc991_fake_397cc0,
        0x0039_7cc0,
        ee::s13::f_00397cc0__generated,
        1
    );

    oc991_fake!(
        oc991_fake_c870,
        0x0039_c870,
        ee::s13::f_0039c870__generated,
        2
    );

    oc991_fake!(
        oc991_fake_b760,
        0x0039_b760,
        ee::s13::f_0039b760__generated,
        3
    );

    oc991_fake!(
        oc991_fake_e8b8,
        0x0039_e8b8,
        ee::s13::f_0039e8b8__generated,
        4
    );

    oc991_fake!(
        oc991_fake_f400,
        0x0039_f400,
        ee::s13::f_0039f400__generated,
        5
    );

    oc991_fake!(
        oc991_fake_overlay_init,
        0x0020_a380,
        ee::s05::f_0020a380__generated,
        6
    );

    oc991_fake!(
        oc991_fake_event_widget,
        0x0020_09d0,
        ee::s05::f_002009d0__generated
    );

    oc991_fake!(
        oc991_fake_event_transition,
        0x0020_0a70,
        ee::s05::f_00200a70__generated
    );

    oc991_fake!(
        oc991_fake_memcard,
        0x001d_58b8,
        ee::s04::f_001d58b8__generated
    );

    /// The entry text of the reward list: a fake that also stores its
    /// mailbox word at `this + 0x9c` (the list's flag, which Init tests next).
    pub(crate) fn oc991_fake_entry_text(c: &mut Cpu) {
        if c.bus.read32(OC991_GATE) != OC991_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc991_fake_entry_text),
                ee::s05::f_001ff7b8__generated,
            );
        }
        let this = c.g[A0][0] as u32;
        fakes::fake_call(c, 0x001f_f7b8);
        let flag = c.bus.read32(OC991_MB + 4 * 7);
        c.bus.write32(this + 0x9c, flag);
    }

    /// The `miss` hook: an indirect call to a table target answers with the
    /// `OC991_MISS` word.
    pub(crate) fn oc991_miss(c: &mut Cpu, target: u32) {
        fakes::fake_call(c, target);
        c.g[V0][0] = c.bus.read32(OC991_MISS) as u64;
    }

    /// Puts every fake in its override slot (once per process).
    pub(crate) fn oc991_install() {
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317670),
            oc991_fake_str_hash,
            fakes::prev_cell(oc991_fake_str_hash),
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_0039D860),
            oc991_fake_find_object,
            fakes::prev_cell(oc991_fake_find_object),
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_00397CC0),
            oc991_fake_397cc0,
            fakes::prev_cell(oc991_fake_397cc0),
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_0039C870),
            oc991_fake_c870,
            fakes::prev_cell(oc991_fake_c870),
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_0039B760),
            oc991_fake_b760,
            fakes::prev_cell(oc991_fake_b760),
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_0039E8B8),
            oc991_fake_e8b8,
            fakes::prev_cell(oc991_fake_e8b8),
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_0039F400),
            oc991_fake_f400,
            fakes::prev_cell(oc991_fake_f400),
        );
        fakes::hook(
            addr_of_mut!(ee::s05::OVERRIDE_0020A380),
            oc991_fake_overlay_init,
            fakes::prev_cell(oc991_fake_overlay_init),
        );
        fakes::hook(
            addr_of_mut!(ee::s05::OVERRIDE_001FF7B8),
            oc991_fake_entry_text,
            fakes::prev_cell(oc991_fake_entry_text),
        );
        fakes::hook(
            addr_of_mut!(ee::s05::OVERRIDE_002009D0),
            oc991_fake_event_widget,
            fakes::prev_cell(oc991_fake_event_widget),
        );
        fakes::hook(
            addr_of_mut!(ee::s05::OVERRIDE_00200A70),
            oc991_fake_event_transition,
            fakes::prev_cell(oc991_fake_event_transition),
        );
        fakes::hook(
            addr_of_mut!(ee::s04::OVERRIDE_001D58B8),
            oc991_fake_memcard,
            fakes::prev_cell(oc991_fake_memcard),
        );
    }

    /// Scrambled registers and the gated fakes; the machine's miss hook
    /// answers table calls.
    pub(crate) fn oc991_machine(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        oc991_install();
        let r = machine(c, seed, salt, calls);
        c.hooks.miss = oc991_miss;
        c.bus.write32(OC991_GATE, OC991_MAGIC);
        r
    }

    pub(crate) fn oc991_set(c: &mut Cpu, slot: u32, value: u32) {
        c.bus.write32(OC991_MB + 4 * slot, value);
    }

    /// A class table at `vt` with a random delta and a table target at each
    /// `slots` offset (the delta is the halfword before the target word).
    pub(crate) fn oc991_vtable(c: &mut Cpu, r: &mut Rng, vt: u32, slots: &[u32]) {
        for &off in slots {
            let delta = (r.next_u32() as i16 as i32 & !3) as u32;
            c.bus.write16(vt + off, delta as u16);
            c.bus.write32(vt + off + 4, fakes::FAKE + off);
        }
    }

    /// The found object (mailbox slot 0) with its class at `obj + 8`.
    pub(crate) fn oc991_objects(c: &mut Cpu, r: &mut Rng) -> u32 {
        let vt = DATA + 0x2100;
        let obj = DATA + 0x2000;
        oc991_vtable(c, r, vt, &[0x48]);
        c.bus.write32(obj + 8, vt);
        oc991_set(c, 0, obj);
        obj
    }

    pub(crate) fn oc991_setup_replay(c: &mut Cpu, seed: u32, salt: u64) {
        let mut r = oc991_machine(c, seed, salt, 12);
        oc991_objects(c, &mut r);
        let flag = seed % 2;
        let stored = if seed % 3 == 0 { flag } else { flag ^ 1 };
        c.bus.write32(OC991_THIS + 0x22c, stored);
        c.g[A0][0] = sx(OC991_THIS);
        c.g[A1][0] = flag as u64;
    }

    #[test]
    fn ov_screen_save_replay_v37_matches() {
        check(
            ee::s05::f_001fe4a8__generated,
            ov_screen_save_replay_v37,
            200,
            &|c, seed| oc991_setup_replay(c, seed, 0x1fe4),
        );
    }

    #[test]
    fn ov_screen_save_records_v37_matches() {
        check(
            ee::s05::f_001ff170__generated,
            ov_screen_save_records_v37,
            200,
            &|c, seed| oc991_setup_replay(c, seed, 0x1ff1),
        );
    }

    #[test]
    fn ov_screen_save_records_shutdown_matches() {
        check(
            ee::s05::f_001fec40__generated,
            ov_screen_save_records_shutdown,
            40,
            &|c, seed| {
                let _r = oc991_machine(c, seed, 0x1fec, 1);
            },
        );
    }

    #[test]
    fn ov_screen_save_records_v39_matches() {
        check(
            ee::s05::f_001ff310__generated,
            ov_screen_save_records_v39,
            200,
            &|c, seed| {
                let mut r = oc991_machine(c, seed, 0x1ff3, 6);
                oc991_objects(c, &mut r);
                c.g[A0][0] = sx(OC991_THIS);
                c.g[A2][0] = (seed % 2) as u64;
            },
        );
    }

    pub(crate) fn oc991_setup_init(c: &mut Cpu, seed: u32) {
        let mut r = oc991_machine(c, seed, 0x1ff5, 13);
        let obj = oc991_objects(c, &mut r);
        c.bus.write32(obj + 0x14, r.next_u32());
        c.bus.write8(obj + 0x96, (seed % 5) as u8);
        if seed % 3 == 0 {
            oc991_set(c, 0, 0);
        }
        // Both 0x397cc0 calls answer with this word (null sometimes).
        oc991_set(c, 1, if seed % 4 == 0 { 0 } else { DATA + 0x3100 });
        // The entry text writes the list flag; odd seeds make the byte test true.
        oc991_set(c, 7, if seed % 2 == 0 { 0 } else { 0x100 });
        c.bus.write32(OC991_THIS + 0x10, DATA + 0x3000);
        c.g[A0][0] = sx(OC991_THIS);
    }

    #[test]
    fn ov_screen_reward_list_init_matches() {
        check(
            ee::s05::f_001ff590__generated,
            ov_screen_reward_list_init,
            200,
            &|c, seed| oc991_setup_init(c, seed),
        );
    }

    #[test]
    fn ov_screen_be_options_dtor_matches() {
        oc1012_install_fakes();
        covered(
            ee::s05::f_00221bf0__generated,
            ov_screen_be_options_dtor,
            SEEDS_4,
            &|c, seed| {
                oc1012_object(c, seed, 0x221bf0, 1);
                c.g[A0][0] = sx(DATA + 0x100 + (seed % 4) * 0x10);
            },
        );
    }

    #[test]
    fn ov_screen_be_options_ctor_matches() {
        oc1012_install_fakes();
        covered(
            ee::s05::f_00221c18__generated,
            ov_screen_be_options_ctor,
            SEEDS_4,
            &|c, seed| {
                oc1012_object(c, seed, 0x221c18, 1);
                c.g[A0][0] = sx(DATA + 0x100 + (seed % 4) * 0x10);
                c.g[A1][0] = sx(seed.wrapping_mul(0x85eb_ca6b));
                c.g[A2][0] = sx(0x40 + seed % 5);
            },
        );
    }

    #[test]
    fn ov_screen_in_game_error_ctor_matches() {
        oc1012_install_fakes();
        covered(
            ee::s05::f_00221c80__generated,
            ov_screen_in_game_error_ctor,
            SEEDS_4,
            &|c, seed| {
                oc1012_object(c, seed, 0x221c80, 1);
                c.g[A0][0] = sx(DATA + 0x100 + (seed % 4) * 0x10);
            },
        );
    }

    #[test]
    fn ov_screen_in_game_error_ctor_2_matches() {
        oc1012_install_fakes();
        covered(
            ee::s05::f_00221ce0__generated,
            ov_screen_in_game_error_ctor_2,
            SEEDS_4,
            &|c, seed| {
                oc1012_object(c, seed, 0x221ce0, 1);
                c.g[A0][0] = sx(DATA + 0x100 + (seed % 4) * 0x10);
                c.g[A1][0] = sx(seed.wrapping_mul(0xc2b2_ae35));
                c.g[A2][0] = sx(0x50 + seed % 6);
            },
        );
    }

    /// Gate of the callee fakes: they answer only while this word holds
    /// `OC1013_MAGIC` (otherwise the body they replaced runs).
    pub(crate) const OC1013_CTRL: u32 = DATA + 0x0040_0000;

    pub(crate) const OC1013_MAGIC: u32 = 0x1013_c0de;

    /// The fixture screen object (`this`).
    pub(crate) const OC1013_O: u32 = DATA + 0x0050_0000;

    pub(crate) static OC1013_PREV_39E390: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static OC1013_PREV_39E2A0: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static OC1013_PREV_1D5428: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static OC1013_PREV_1D53B0: AtomicUsize = AtomicUsize::new(0);

    pub(crate) fn oc1013_fake_39e390(c: &mut Cpu) {
        if c.bus.read32(OC1013_CTRL) != OC1013_MAGIC {
            return fakes::fall_back(c, &OC1013_PREV_39E390, ee::s13::f_0039e390__generated);
        }
        fakes::fake_call(c, 0x0039_e390);
    }

    pub(crate) fn oc1013_fake_39e2a0(c: &mut Cpu) {
        if c.bus.read32(OC1013_CTRL) != OC1013_MAGIC {
            return fakes::fall_back(c, &OC1013_PREV_39E2A0, ee::s13::f_0039e2a0__generated);
        }
        fakes::fake_call(c, 0x0039_e2a0);
    }

    pub(crate) fn oc1013_fake_1d5428(c: &mut Cpu) {
        if c.bus.read32(OC1013_CTRL) != OC1013_MAGIC {
            return fakes::fall_back(c, &OC1013_PREV_1D5428, ee::s04::f_001d5428__generated);
        }
        fakes::fake_call(c, 0x001d_5428);
    }

    pub(crate) fn oc1013_fake_1d53b0(c: &mut Cpu) {
        if c.bus.read32(OC1013_CTRL) != OC1013_MAGIC {
            return fakes::fall_back(c, &OC1013_PREV_1D53B0, ee::s04::f_001d53b0__generated);
        }
        fakes::fake_call(c, 0x001d_53b0);
    }

    /// Hook the four base-class bodies once per process.
    pub(crate) fn oc1013_install_fakes() {
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_0039E390),
            oc1013_fake_39e390,
            &OC1013_PREV_39E390,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_0039E2A0),
            oc1013_fake_39e2a0,
            &OC1013_PREV_39E2A0,
        );
        fakes::hook(
            addr_of_mut!(ee::s04::OVERRIDE_001D5428),
            oc1013_fake_1d5428,
            &OC1013_PREV_1D5428,
        );
        fakes::hook(
            addr_of_mut!(ee::s04::OVERRIDE_001D53B0),
            oc1013_fake_1d53b0,
            &OC1013_PREV_1D53B0,
        );
    }

    /// Scrambled machine with one logged callee call (it may unwind) and the
    /// gate on.
    pub(crate) fn oc1013_base(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let r = fakes::machine(c, seed, salt, 1);
        c.bus.write32(OC1013_CTRL, OC1013_MAGIC);
        r
    }

    #[test]
    fn ov_screen_in_game_error_dtor_matches() {
        oc1013_install_fakes();
        fakes::covered(
            ee::s05::f_00221d58__generated,
            ov_screen_in_game_error_dtor,
            200,
            &|c, seed| {
                oc1013_base(c, seed, 0x1013_0001);
                c.g[A0][0] = sx(OC1013_O);
            },
        );
    }

    #[test]
    fn ov_screen_race_pre_ctor_matches() {
        oc1013_install_fakes();
        fakes::covered(
            ee::s05::f_00221e18__generated,
            ov_screen_race_pre_ctor,
            200,
            &|c, seed| {
                let mut r = oc1013_base(c, seed, 0x1013_0004);
                c.g[A0][0] = sx(OC1013_O);
                c.g[A2][0] = sx(r.next_u32());
            },
        );
    }

    #[test]
    fn ov_screen_freestyle_pre_dtor_matches() {
        oc1013_install_fakes();
        fakes::covered(
            ee::s05::f_00221e80__generated,
            ov_screen_freestyle_pre_dtor,
            200,
            &|c, seed| {
                oc1013_base(c, seed, 0x1013_0005);
                c.g[A0][0] = sx(OC1013_O);
            },
        );
    }

    #[test]
    fn ov_screen_freestyle_pre_ctor_matches() {
        oc1013_install_fakes();
        fakes::covered(
            ee::s05::f_00221ea8__generated,
            ov_screen_freestyle_pre_ctor,
            200,
            &|c, seed| {
                let mut r = oc1013_base(c, seed, 0x1013_0006);
                c.g[A0][0] = sx(OC1013_O);
                c.g[A2][0] = sx(r.next_u32());
            },
        );
    }

    #[test]
    fn ov_screen_multi_freestyle_pre_dtor_matches() {
        oc1013_install_fakes();
        fakes::covered(
            ee::s05::f_00221f10__generated,
            ov_screen_multi_freestyle_pre_dtor,
            200,
            &|c, seed| {
                oc1013_base(c, seed, 0x1013_0007);
                c.g[A0][0] = sx(OC1013_O);
            },
        );
    }

    #[test]
    fn ov_screen_multi_freestyle_pre_ctor_matches() {
        oc1013_install_fakes();
        fakes::covered(
            ee::s05::f_00221f38__generated,
            ov_screen_multi_freestyle_pre_ctor,
            200,
            &|c, seed| {
                let mut r = oc1013_base(c, seed, 0x1013_0008);
                c.g[A0][0] = sx(OC1013_O);
                c.g[A2][0] = sx(r.next_u32());
            },
        );
    }

    /// Gate word of this module's fakes (its own address and magic in the
    /// machine's RAM, so other tests' fakes stay off).
    pub(crate) const OC1014_CTRL: u32 = DATA + 0x0020_a140;

    pub(crate) const OC1014_MAGIC: u32 = 0x1014_0c0d;

    pub(crate) static OC1014_PREV_UI_CTOR: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static OC1014_PREV_UI_DTOR: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static OC1014_PREV_MEMCARD_CTOR: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static OC1014_PREV_MEMCARD_DTOR: AtomicUsize = AtomicUsize::new(0);

    /// `UIState_ctor` with its gate on: logs the call and returns to the
    /// caller, so only the wrappers' own work is compared.
    pub(crate) fn oc1014_fake_ui_ctor(c: &mut Cpu) {
        if c.bus.read32(OC1014_CTRL) != OC1014_MAGIC {
            return fakes::fall_back(c, &OC1014_PREV_UI_CTOR, ee::s13::f_0039e2a0__generated);
        }
        fakes::fake_call(c, 0x0039_e2a0);
    }

    /// `UIState_dtor` with its gate on (see `oc1014_fake_ui_ctor`).
    pub(crate) fn oc1014_fake_ui_dtor(c: &mut Cpu) {
        if c.bus.read32(OC1014_CTRL) != OC1014_MAGIC {
            return fakes::fall_back(c, &OC1014_PREV_UI_DTOR, ee::s13::f_0039e390__generated);
        }
        fakes::fake_call(c, 0x0039_e390);
    }

    /// `MemCardStateBase_ctor` with its gate on (see `oc1014_fake_ui_ctor`).
    pub(crate) fn oc1014_fake_memcard_ctor(c: &mut Cpu) {
        if c.bus.read32(OC1014_CTRL) != OC1014_MAGIC {
            return fakes::fall_back(c, &OC1014_PREV_MEMCARD_CTOR, ee::s04::f_001d53b0__generated);
        }
        fakes::fake_call(c, 0x001d_53b0);
    }

    /// `MemCardStateBase_dtor` with its gate on (see `oc1014_fake_ui_ctor`).
    pub(crate) fn oc1014_fake_memcard_dtor(c: &mut Cpu) {
        if c.bus.read32(OC1014_CTRL) != OC1014_MAGIC {
            return fakes::fall_back(c, &OC1014_PREV_MEMCARD_DTOR, ee::s04::f_001d5428__generated);
        }
        fakes::fake_call(c, 0x001d_5428);
    }

    /// Puts this module's four gated fakes in their override slots (once per
    /// process; with the gate off they run the body they replaced).
    pub(crate) fn oc1014_hook() {
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_0039E2A0),
            oc1014_fake_ui_ctor,
            &OC1014_PREV_UI_CTOR,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_0039E390),
            oc1014_fake_ui_dtor,
            &OC1014_PREV_UI_DTOR,
        );
        fakes::hook(
            addr_of_mut!(ee::s04::OVERRIDE_001D53B0),
            oc1014_fake_memcard_ctor,
            &OC1014_PREV_MEMCARD_CTOR,
        );
        fakes::hook(
            addr_of_mut!(ee::s04::OVERRIDE_001D5428),
            oc1014_fake_memcard_dtor,
            &OC1014_PREV_MEMCARD_DTOR,
        );
    }

    /// Scrambled registers, the gate on, and the object (junk in its first
    /// 0x100 bytes, so every stored field shows) at `DATA + 0x2000` (varied by
    /// seed) as `this`. Each function makes one call, so one unwind index.
    pub(crate) fn oc1014_setup(c: &mut Cpu, seed: u32, salt: u64) {
        let mut r = machine(c, seed, salt, 1);
        c.bus.write32(OC1014_CTRL, OC1014_MAGIC);
        let obj = DATA + 0x2000 + (seed % 4) * 0x100;
        crate::difftest::fill(c, &mut r, obj, 0x40);
        c.g[A0][0] = obj as u64;
    }

    #[test]
    fn ov_screen_rival_pre_dtor_matches() {
        oc1014_hook();
        check(
            ee::s05::f_00221fa0__generated,
            ov_screen_rival_pre_dtor,
            200,
            &|c, seed| oc1014_setup(c, seed, 0x0022_1fa0),
        );
    }

    #[test]
    fn ov_screen_rival_pre_ctor_matches() {
        oc1014_hook();
        check(
            ee::s05::f_00221fc8__generated,
            ov_screen_rival_pre_ctor,
            200,
            &|c, seed| oc1014_setup(c, seed, 0x0022_1fc8),
        );
    }

    #[test]
    fn ov_screen_save_replay_dtor_matches() {
        oc1014_hook();
        check(
            ee::s05::f_00222030__generated,
            ov_screen_save_replay_dtor,
            200,
            &|c, seed| oc1014_setup(c, seed, 0x0022_2030),
        );
    }

    #[test]
    fn ov_screen_save_replay_ctor_2_matches() {
        oc1014_hook();
        check(
            ee::s05::f_00222058__generated,
            ov_screen_save_replay_ctor_2,
            200,
            &|c, seed| oc1014_setup(c, seed, 0x0022_2058),
        );
    }

    #[test]
    fn ov_screen_save_records_dtor_matches() {
        oc1014_hook();
        check(
            ee::s05::f_00222098__generated,
            ov_screen_save_records_dtor,
            200,
            &|c, seed| oc1014_setup(c, seed, 0x0022_2098),
        );
    }

    #[test]
    fn ov_screen_save_records_ctor_2_matches() {
        oc1014_hook();
        check(
            ee::s05::f_002220c0__generated,
            ov_screen_save_records_ctor_2,
            200,
            &|c, seed| oc1014_setup(c, seed, 0x0022_20c0),
        );
    }

    #[test]
    fn ov_screen_reward_list_dtor_matches() {
        check(
            ee::s05::f_002221f0__generated,
            ov_screen_reward_list_dtor,
            200,
            &|c, seed| oc1015_object_setup(c, seed),
        );
    }

    #[test]
    fn ov_screen_reward_list_ctor_matches() {
        check(
            ee::s05::f_00222218__generated,
            ov_screen_reward_list_ctor,
            200,
            &|c, seed| oc1015_object_setup(c, seed),
        );
    }
}
