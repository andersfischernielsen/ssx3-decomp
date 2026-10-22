//! `src/be/beintmission.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::events::deliver_event_here;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::guest::Addr;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;
use crate::abi;
use crate::abi::r32;
use crate::events::event_due;
use ssx3_ee as ee;
use ssx3_ee_rt::vu0::Src;
use crate::call::sp;
use crate::call::Unwound;
use ssx3_ee_rt::vu0::W;
use ssx3_ee_rt::vu0::Y;
use ssx3_ee_rt::vu0::Z;
use ssx3_ee_rt::vu0::X;
use crate::events::back_edge_unwinds;
pub(crate) use crate::game::dirtysock::tags2::p_2 as p;
pub(crate) use crate::game::fe::feasyncfile::at;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::lw as lw_2;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0015_32a0, be_mission_int_get_instance, "BEMissionInt_GetInstance"),
    (0x0015_3350, be_mission_int_get_total_collectibles, "BEMissionInt_GetTotalCollectibles"),
    (0x0015_3390, be_mission_int_get_peak_collectible_total, "BEMissionInt_GetPeakCollectibleTotal"),
    (0x0015_3498, be_mission_int_get_collected_count, "BEMissionInt_GetCollectedCount"),
    (0x0015_3520, be_mission_int_get_track_collected_count, "BEMissionInt_GetTrackCollectedCount"),
    (0x0015_35c0, be_mission_int_count_collected_on_peak, "BEMissionInt_CountCollectedOnPeak"),
    (0x0015_3688, be_mission_int_count_collected_on_peak_for, "BEMissionInt_CountCollectedOnPeakFor"),
    (0x0015_3858, be_mission_int_get_collected_mask, "BEMissionInt_GetCollectedMask"),
    (0x0015_38e8, be_mission_int_is_collected, "BEMissionInt_IsCollected"),
    (0x0015_3c88, be_mission_int_get_mission_text1, "BEMissionInt_GetMissionText1"),
    (0x0015_3cd8, be_mission_int_get_mission_text2, "BEMissionInt_GetMissionText2"),
    (0x0015_3d28, be_mission_int_get_mission_text3, "BEMissionInt_GetMissionText3"),
    (0x0015_3d78, be_mission_int_is_mission_complete, "BEMissionInt_IsMissionComplete"),
    (0x0015_3dd0, be_mission_int_get_mission_flag1, "BEMissionInt_GetMissionFlag1"),
    (0x0015_3e28, be_mission_int_get_mission_flag0, "BEMissionInt_GetMissionFlag0"),
    (0x0015_3e80, be_mission_int_get_mission_flag2, "BEMissionInt_GetMissionFlag2"),
    (0x0015_3ed8, be_mission_int_get_mission_flag4, "BEMissionInt_GetMissionFlag4"),
    (0x0015_3f30, be_mission_int_get_mission_track, "BEMissionInt_GetMissionTrack"),
    (0x0015_3fa0, mission_get_field18, "BEMissionInt_GetMissionField18"),
    (0x0015_3fd8, mission_get_prereq, "BEMissionInt_GetMissionPrereq"),
    (0x0015_4010, be_mission_int_set_mission_flag0, "BEMissionInt_SetMissionFlag0"),
    (0x0015_4080, be_mission_int_set_mission_flag1, "BEMissionInt_SetMissionFlag1"),
    (0x0015_40f0, be_mission_int_set_mission_flag2, "BEMissionInt_SetMissionFlag2"),
    (0x0015_4160, be_mission_int_set_mission_complete, "BEMissionInt_SetMissionComplete"),
    (0x0015_41d0, be_mission_int_set_mission_flag4, "BEMissionInt_SetMissionFlag4"),
    (0x0015_4240, be_mission_int_find_mission, "BEMissionInt_FindMission"),
    (0x0015_4278, be_mission_int_count_missions_on_current_track, "BEMissionInt_CountMissionsOnCurrentTrack"),
    (0x0015_42a0, be_mission_int_count_missions_on_track, "BEMissionInt_CountMissionsOnTrack"),
    (0x0015_42e0, be_mission_int_count_completed_on_track, "BEMissionInt_CountCompletedOnTrack"),
    (0x0015_4368, be_mission_int_count_missions_on_peak, "BEMissionInt_CountMissionsOnPeak"),
    (0x0015_43e0, bemissionint_count_completed_on_peak, "BEMissionInt_CountCompletedOnPeak"),
    (0x0015_44d0, be_mission_int_count_completed_on_peak_for, "BEMissionInt_CountCompletedOnPeakFor"),
    (0x0015_4588, be_mission_int_get_nth_mission_on_current_track, "BEMissionInt_GetNthMissionOnCurrentTrack"),
    (0x0015_45b0, be_mission_int_get_nth_mission_on_track, "BEMissionInt_GetNthMissionOnTrack"),
    (0x0015_45f8, be_mission_int_get_session_type, "BEMissionInt_GetSessionType"),
    (0x0015_4630, be_mission_int_session_type_at_least, "BEMissionInt_SessionTypeAtLeast"),
    (0x0015_4678, be_mission_int_get_session_info, "BEMissionInt_GetSessionInfo"),
    (0x0015_4760, be_mission_int_get_session_name, "BEMissionInt_GetSessionName"),
    (0x0015_47a0, mission_get_field22, "BEMissionInt_GetMissionField22"),
    (0x0015_4898, bemission_int_load_state, "BEMissionInt_LoadState"),
    (0x0015_bcd0, be_mission_int_destroy_instance, "BEMissionInt_DestroyInstance"),
];

// ---------------------------------------------------------------------------
// Callees (address + the generated shard function, which dispatches through
// its override slot). Entries for calls that the generator emitted as
// `call_indirect` are kept for reference next to the `call_ptr` sites.

#[allow(dead_code)]
pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_ON_ABORT_POPUP_EVENT;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_ON_EMAIL_POPUP_EVENT;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_ON_LOGIN_POPUP_EVENT;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_ON_SAVE_ACCOUNT_ANSWER;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_ON_CHILD_EVENT_12 as NET_FE_STATE_ON_CHILD_EVENT;
    pub(crate) use crate::game::fe::feasyncfile::callee::CHALLENGE_POPUP_SET_SETTINGS_LOCKED;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_CTOR_11 as NET_FE_STATE_CTOR_2;
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
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_GET_POPUP_4 as NET_FE_STATE_GET_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_SHOW_POPUP_3 as NET_FE_STATE_SHOW_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::POPUP_CONFIRM_CFG_SET_OPTION_TEXT_ID_2 as POPUP_CONFIRM_CFG_SET_OPTION_TEXT_ID;
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
    pub(crate) use crate::game::bx::bxstring2::callee::USTR_FROM_ASCII_2 as USTR_FROM_ASCII;
    pub(crate) use crate::game::bx::bxstring2::callee::USTR_SPRINTF;
    pub(crate) use crate::game::bx::bxstring2::callee::USTR_TO_ASCII;
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
    pub(crate) use crate::game::bx::bxstring2::callee::STR_HASH;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_SCREEN_FIND_OBJECT;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_MENU_FIND_FIRST_SELECTABLE;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_MENU_SET_SELECTION;
    pub(crate) use crate::game::bx::bxstring2::callee::STRLEN;
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
    pub(crate) use crate::game::bx::bxstring2::callee::BE_GET_RIDER_CHARACTER as BE_GET_RIDER_CHARACTER_2;
    pub(crate) use crate::game::bx::bxstring2::callee::BE_GET_RIDER_PROFILE as BE_GET_RIDER_PROFILE_2;
    pub(crate) use crate::game::bx::bxstring2::callee::BE_GET_CHAR_PROFILE_EVENT_RECORD as BE_GET_CHAR_PROFILE_EVENT_RECORD_2;
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
    pub(crate) use crate::game::bx::bxstring2::callee::MEMSET;
    pub(crate) use crate::game::bx::bxstring2::callee::QSORT;
    pub(crate) use crate::game::bx::bxstring2::callee::CROWD_MAN3D_GROUP_BUILD_MATRICES;
    pub(crate) use crate::game::bx::bxstring2::callee::CHASE_CAM_CONTROLLER_RESTORE_DEFAULT;
    pub(crate) use crate::game::bx::bxstring2::callee::SCRIPT_ENGINE_LIST_START;
    pub(crate) use crate::game::bx::bxstring2::callee::SCRIPT_ENGINE_SET_SKIP_DELAY;
    pub(crate) use crate::game::bx::bxstring2::callee::CUTSCENE_TRANSITION_MAN_FORCE_ON;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_STATE_MGR_FIND_STATE;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_STATE_SHUTDOWN;
    pub(crate) use crate::game::bx::bxstring2::callee::UI_STATE_MGR_CLOSE_ALL_POPUPS;
    pub(crate) use crate::game::bx::bxstring2::callee::GAME_COMM_RECV_GAME_PACKET;
    pub(crate) use crate::game::bx::bxstring2::callee::OPERATOR_NEW_NAMED;
    pub(crate) use crate::game::bx::bxstring2::callee::EA_MEM_FREE;
    pub(crate) use crate::game::bx::bxstring2::callee::USTR_LEN;
    pub(crate) use crate::game::bx::bxstring2::callee::USTR_CPY;
    pub(crate) use crate::game::bx::bxstring2::callee::UITEXT_SET_WIDE_STRING;


    /// `WorldObj_RunKillScript` (0x34fcc0).
    pub const WORLD_OBJ_RUN_KILL_SCRIPT: Callee =
        Callee { addr: 0x0034_fcc0, f: ee::s11::f_0034fcc0 };

    /// `UIText_SetWideStringInternal` (0x3a0eb0).
    pub const UITEXT_SET_WIDE_STRING_INTERNAL: Callee =
        Callee { addr: 0x003a_0eb0, f: ee::s13::f_003a0eb0 };

    /// `MPEG2_Error` (0x3b11e8; an empty stub).
    pub const MPEG2_ERROR: Callee = Callee { addr: 0x003b_11e8, f: ee::s13::f_003b11e8 };

    /// `BEMissionInt_CountMissionsOnTrack` (0x1542a0).
    pub const MISSION_COUNT: Callee = Callee { addr: 0x0015_42a0, f: ee::s01::f_001542a0 };

    /// `BEMissionInt_GetNthMissionOnTrack` (0x1545b0).
    pub const MISSION_NTH: Callee = Callee { addr: 0x0015_45b0, f: ee::s01::f_001545b0 };

    /// `RewardsFile_Load` (0x15ad58).
    pub const REWARDS_FILE_LOAD: Callee = Callee { addr: 0x0015_ad58, f: ee::s01::f_0015ad58 };

    /// `ChaseCamController_SetAlgorithm` (0x15d078).
    pub const CHASE_CAM_SET_ALGORITHM: Callee =
        Callee { addr: 0x0015_d078, f: ee::s01::f_0015d078 };

    /// `CameraTriggerMan_EmptyStub` (0x16cf40).
    pub const CAMERA_TRIGGER_EMPTY_STUB: Callee =
        Callee { addr: 0x0016_cf40, f: ee::s02::f_0016cf40 };

    /// `cFEStateUberTrick_RefreshTrickList` (0x183de0).
    pub const UBER_TRICK_REFRESH: Callee = Callee { addr: 0x0018_3de0, f: ee::s02::f_00183de0 };

    /// `cFEStateMemCard_ScanStep` (0x187a38).
    pub const MEMCARD_SCAN_STEP: Callee = Callee { addr: 0x0018_7a38, f: ee::s02::f_00187a38 };

    /// `NetFEState_OnWidgetAction` (0x1a8918).
    pub const NET_FE_STATE_ON_WIDGET_ACTION: Callee =
        Callee { addr: 0x001a_8918, f: ee::s03::f_001a8918 };

    /// `cOVState_ViewPopup_ShowMessage` (0x1e5428).
    pub const VIEW_POPUP_SHOW_MESSAGE: Callee =
        Callee { addr: 0x001e_5428, f: ee::s04::f_001e5428 };

    /// `cFEStateBuyAttrib_UpdateBars` (0x1f50b0).
    pub const BUY_ATTRIB_UPDATE_BARS: Callee =
        Callee { addr: 0x001f_50b0, f: ee::s04::f_001f50b0 };

    /// `cFEStateCareerStats_ShowHighlightsPage` (0x1f5da0).
    pub const CAREER_STATS_SHOW_HIGHLIGHTS: Callee =
        Callee { addr: 0x001f_5da0, f: ee::s04::f_001f5da0 };


    pub const FIND_MISSION: Callee =
        Callee { addr: 0x0015_4240, f: ee::s01::BEMissionInt_FindMission };

    pub const CHASE_CAM_RESET_STATE: Callee =
        Callee { addr: 0x0016_2138, f: ee::s01::ChaseCamController_ResetState };

    pub const CHASE_CAM_SET_ALGORITHM_IMMEDIATE: Callee =
        Callee { addr: 0x0015_d050, f: ee::s01::ChaseCamController_SetAlgorithmImmediate };

    pub const CHASE_CAM_SET_ALGORITHM_2: Callee =
        Callee { addr: 0x0015_d078, f: ee::s01::ChaseCamController_SetAlgorithm };

    pub const GET_RIDER_PREVIEW: Callee =
        Callee { addr: 0x001a_0548, f: ee::s03::FEModule_GetRiderPreview };

    pub const RIDER_PREVIEW_SET_VISIBLE: Callee =
        Callee { addr: 0x0019_e538, f: ee::s03::FEModule_RiderPreviewSetVisible };

    pub const NET_FE_STATE_CTOR: Callee =
        Callee { addr: 0x001a_8500, f: ee::s03::NetFEState_ctor };

    pub const LOBBY_MENU_REQUEST_JOIN_ROOM: Callee =
        Callee { addr: 0x001b_d540, f: ee::s03::cFEStateLobbyMenu_RequestJoinRoom };

    pub const POPUP_CONFIRM_CFG_RESET: Callee =
        Callee { addr: 0x001c_5630, f: ee::s03::cFEPopupConfirm_CfgReset };

    pub const REWARD_GALLERY_CHECK_REWARDS_READY: Callee =
        Callee { addr: 0x001d_0238, f: ee::s04::cFEStateRewardGallery_CheckRewardsReady };

    pub const REWARD_GALLERY_POLL_SHAPE_LOAD: Callee =
        Callee { addr: 0x001c_ff10, f: ee::s04::cFEStateRewardGallery_PollShapeLoad };

    pub const BUDDY_MAN_GET_ROSTER_ENTRY: Callee =
        Callee { addr: 0x0026_25c8, f: ee::s07::BuddyMan_GetRosterEntry };

    pub const REPLAY_APPLY_CAMERA: Callee =
        Callee { addr: 0x0026_edd8, f: ee::s07::Replay_ApplyCamera };

    pub const RACE_PROGRESS_RESET_ALL: Callee =
        Callee { addr: 0x0021_08f8, f: ee::s05::RaceProgress_ResetAll };

    pub const MC_MANAGER_END_OPERATION: Callee =
        Callee { addr: 0x0023_ea30, f: ee::s06::MCManager_EndOperation };

    pub const SCRIPT_ENGINE_STOP_MAIN_LISTS: Callee =
        Callee { addr: 0x0027_90f8, f: ee::s07::ScriptEngine_StopMainLists };

    pub const SCRIPT_ENGINE_STOP_AUX_LISTS: Callee =
        Callee { addr: 0x0027_9488, f: ee::s07::ScriptEngine_StopAuxLists };

    pub const SCRIPT_ENGINE_STOP_ALL_PROCESS_SEQS: Callee =
        Callee { addr: 0x0028_2390, f: ee::s07::ScriptEngine_StopAllProcessSeqs };

    pub const SCRIPT_ENGINE_GET_INSTANCE: Callee =
        Callee { addr: 0x0027_d2e8, f: ee::s07::ScriptEngine_GetInstance };

    pub const SCRIPT_ENGINE_DEACTIVATE_BINDING: Callee =
        Callee { addr: 0x0027_d4a0, f: ee::s07::ScriptEngine_DeactivateBinding };

    pub const CROWD_AUDIO_STOP_ALL: Callee =
        Callee { addr: 0x002a_7718, f: ee::s08::CrowdAudio_StopAll };

    pub const BANK_MONITOR_STOP_ALL: Callee =
        Callee { addr: 0x002a_d3c0, f: ee::s08::BankMonitor_StopAll };

    pub const AUDIO_LEVEL_TO_VOLUME: Callee =
        Callee { addr: 0x0028_73d8, f: ee::s07::SSXAudioSystem_LevelToVolume };

    pub const AUDIO_SET_CHANNEL_VOLUME: Callee =
        Callee { addr: 0x0028_7700, f: ee::s07::SSXAudioSystem_SetChannelVolume };

    pub const SND_BANK_SET_MEMORY: Callee =
        Callee { addr: 0x0028_b588, f: ee::s07::SndBank_SetMemory };

    pub const AUDIO_IS_GAME_MUSIC_ENABLED: Callee =
        Callee { addr: 0x0028_d960, f: ee::s07::SSXAudioSystem_IsGameMusicEnabled };

    pub const MUSIC_PLAYER_STOP_SONG: Callee =
        Callee { addr: 0x002b_3ac0, f: ee::s08::MusicPlayer_StopSong };


    // Indirect calls the generated code keeps as raw addresses.
    pub const UI_STATE_UPDATE: u32 = 0x0039_e510;

    pub const BE_LIBRARY_GET_INSTANCE: u32 = 0x0014_dc80;

    pub const BE_LIBRARY_GET_INTERFACE: u32 = 0x0014_dd58;

    pub const BE_NEW_PLAYER_GET_INBOX: u32 = 0x0014_7908;

    pub const WSCRIPT_MAN_EVENT_QUEUE_POP: u32 = 0x0030_c820;

    pub const AUDIO_GET_INSTANCE: u32 = 0x0028_b180;

    pub const AUDIO_START_PRERACE_MUSIC: u32 = 0x0028_72a8;

    pub const SLOGIN_DELETE: u32 = 0x003f_80c0;

    pub const CAMERA_TRIGGER_MAN_UPDATE_MASK1: u32 = 0x0016_d1d8;

    pub const EA_MEMSET: u32 = 0x003e_6448;

    pub const AUDIO_GET_CHANNEL_VOLUME: u32 = 0x0028_7920;
}


/// `lw` through the CPU load path (sign-extended 32-bit load).
#[inline(always)]
pub(crate) fn lw(c: &mut Cpu, a: u32) -> u64 {
    Addr(a).lw(c)
}


// ---------------------------------------------------------------------------
// Callees / globals.

/// `EA_MemAllocNamed` (cross-shard, called through the function table).
pub(crate) const EA_MEM_ALLOC_NAMED: u32 = 0x0031_7d70;

/// `__builtin_delete`.
pub(crate) const BUILTIN_DELETE: u32 = 0x0031_7e50;


pub(crate) const BE_GET_RIDER_CHARACTER: Callee = Callee { addr: 0x0014_a080, f: ssx3_ee::s01::f_0014a080 };

pub(crate) const BE_GET_RIDER_PROFILE: Callee = Callee { addr: 0x0014_a0e0, f: ssx3_ee::s01::f_0014a0e0 };


/// `BEMissionInt *BEMissionInt_GetInstance(void)` (0x1532a0).
pub fn be_mission_int_get_instance(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = lw(c, (c.g[GP][0] as u32).wrapping_sub(0x1eb8));
        let have = c.g[V0][0] != 0;
        frame.save_ra(c, 0);
        if !have {
            c.cycles += 11;
            c.g[A1][0] = 0x460000;
            c.g[A0][0] = 0x14;
            c.g[A1][0] = sx(0x460000u32.wrapping_sub(0x57b8));
            c.g[A2][0] = 0;
            c.g[A3][0] = 0;
            call_ptr(c, EA_MEM_ALLOC_NAMED, 0x0015_32c8)?;
            c.g[V1][0] = 0x460000;
            c.sw((c.g[V0][0] as u32).wrapping_add(0x8), 0);
            c.g[V1][0] = sx(0x460000u32.wrapping_sub(0x5208));
            c.sw((c.g[V0][0] as u32).wrapping_add(0xc), c.g[V1][0]);
            c.sw((c.g[GP][0] as u32).wrapping_sub(0x1eb8), c.g[V0][0]);
        }
        c.cycles += 4;
        c.g[V0][0] = lw(c, (c.g[GP][0] as u32).wrapping_sub(0x1eb8));
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `void BEMissionInt_DestroyInstance(void)` (0x15bcd0).
pub fn be_mission_int_destroy_instance(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = lw(c, (c.g[GP][0] as u32).wrapping_sub(0x1eb8));
        frame.save_ra(c, 0);
        call_ptr(c, BUILTIN_DELETE, 0x0015_bce4)?;
        frame.restore_ra(c, 0);
        c.sw((c.g[GP][0] as u32).wrapping_sub(0x1eb8), 0);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// Back-end mission state: 0x440000 holds two parallel record tables (mission
// records at 0x43ee10, 0x24 bytes each; peak records at 0x43d950) and the
// per-session table at 0x440770 (0x2c bytes each).

pub(crate) const MISSION_RECORDS: u32 = 0x0043_ee10;
 // 0x440000 - 0x11f0
pub(crate) const PEAK_RECORD_END: u32 = 0x0043_ee20;
 // 0x440000 - 0x11e0
pub(crate) const PEAK_RECORDS: u32 = 0x0043_d950;
 // 0x440000 - 0x26b0
pub(crate) const SESSION_TABLE: u32 = 0x0044_0770;
 // 0x440000 + 0x770
pub(crate) const MISSION_STRIDE: u32 = 0x24;

pub(crate) const SESSION_STRIDE: u32 = 0x2c;

pub(crate) const RIDER_STRIDE: u32 = 0xf88;

pub(crate) const RIDER_FLAG_TABLE: u32 = 0x004a_6ca8;
 // 0x4a0000 + 0x6ca8
pub(crate) const CHAR_STATE: u32 = 0x0053_5538;
 // 0x530000 + 0x5538

pub(crate) const FIND_MISSION: Callee = Callee { addr: 0x0015_4240, f: ssx3_ee::s01::f_00154240 };


/// The generated loop back-edge event check (`if cycles >= next_event`).
#[inline(always)]
pub(crate) fn event_check(c: &mut Cpu, head: u32) {
    if c.cycles >= c.next_event {
        c.pc = head;
        deliver_event_here(c, c.pc);
    }
}


/// `int BEMissionInt_FindMission(BEMissionInt *self, int id)` (0x154240): the
/// index of the mission record whose id equals `id`, or -1 after 88 records.
pub fn be_mission_int_find_mission(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V0][0] = 0x440000;
    c.g[V1][0] = 0;
    let mut rec = MISSION_RECORDS;
    c.g[A0][0] = sx(rec);
    loop {
        c.cycles += 3;
        c.g[V0][0] = lw(c, rec);
        if c.g[V0][0] != c.g[A1][0] {
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
            c.cycles += 3;
            c.g[V0][0] = ((c.g[V1][0] as i64) < 88) as u64;
            let t = c.g[V0][0] != 0;
            rec = rec.wrapping_add(MISSION_STRIDE);
            c.g[A0][0] = sx(rec);
            if t {
                event_check(c, 0x0015_4250);
                continue;
            } else {
                c.cycles += 2;
                c.g[V0][0] = 0xffff_ffff_ffff_ffff;
                c.pc = c.g[RA][0] as u32;
                return;
            }
        } else {
            c.cycles += 2;
            c.g[V0][0] = c.g[V1][0];
            c.pc = c.g[RA][0] as u32;
            return;
        }
    }
}


/// `int BEMissionInt_CountMissionsOnTrack(BEMissionInt *self, int track)`
/// (0x1542a0): how many of the 88 mission records have this track.
pub fn be_mission_int_count_missions_on_track(c: &mut Cpu) {
    c.cycles += 6;
    c.g[V0][0] = 0x440000;
    c.g[A0][0] = 0;
    c.g[V0][0] = sx(0x440000u32.wrapping_sub(0x11f0));
    c.g[A2][0] = 0x57;
    let mut rec = MISSION_RECORDS.wrapping_add(0x10);
    c.g[A3][0] = sx(rec);
    loop {
        c.cycles += 7;
        c.g[V0][0] = lw(c, rec);
        c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(1));
        rec = rec.wrapping_add(MISSION_STRIDE);
        c.g[A3][0] = sx(rec);
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(1));
        c.g[V0][0] = c.g[V0][0] ^ c.g[A1][0];
        let t = (c.g[A2][0] as i64) >= 0;
        if c.g[V0][0] == 0 {
            c.g[A0][0] = c.g[V1][0];
        }
        if t {
            event_check(c, 0x0015_42b8);
            continue;
        } else {
            c.cycles += 2;
            c.g[V0][0] = c.g[A0][0];
            c.pc = c.g[RA][0] as u32;
            return;
        }
    }
}


/// `int BEMissionInt_CountMissionsOnPeak(BEMissionInt *self, int peak)`
/// (0x154368): count mission records on `peak` across the mission/peak tables.
pub fn be_mission_int_count_missions_on_peak(c: &mut Cpu) {
    c.cycles += 8;
    c.g[V1][0] = 0x440000;
    c.g[V0][0] = 0x440000;
    c.g[T4][0] = sx(0x440000u32.wrapping_sub(0x11e0));
    c.g[T3][0] = sx(0x440000u32.wrapping_sub(0x26b0));
    c.g[A3][0] = 0;
    c.g[V1][0] = 0;
    c.g[T2][0] = 0x24;
    c.mult(0, V0, V1, T2);
    loop {
        c.cycles += 4;
        c.g[A2][0] = 0;
        c.g[T1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
        c.g[A0][0] = sx((c.g[T3][0] as u32).wrapping_add(0x54));
        c.g[T0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T4][0] as u32));
        loop {
            c.cycles += 3;
            c.g[V0][0] = lw(c, c.g[A0][0] as u32);
            if c.g[V0][0] != c.g[A1][0] {
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
            } else {
                c.cycles += 5;
                c.g[V0][0] = lw(c, c.g[T0][0] as u32);
                c.g[V1][0] = sx((c.g[A3][0] as u32).wrapping_add(1));
                c.g[V0][0] = c.g[V0][0] ^ c.g[A2][0];
                if c.g[V0][0] == 0 {
                    c.g[A3][0] = c.g[V1][0];
                }
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[A2][0] as i64) < 22) as u64;
            let t = c.g[V0][0] != 0;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x64));
            if t {
                event_check(c, 0x0015_4398);
                continue;
            }
            c.cycles += 4;
            c.g[V1][0] = c.g[T1][0];
            c.g[V0][0] = ((c.g[V1][0] as i64) < 88) as u64;
            let t = c.g[V0][0] != 0;
            c.mult(0, V0, V1, T2);
            if t {
                event_check(c, 0x0015_4388);
                break;
            } else {
                c.cycles += 2;
                c.g[V0][0] = c.g[A3][0];
                c.pc = c.g[RA][0] as u32;
                return;
            }
        }
    }
}


/// `int BEMissionInt_CountCollectedOnPeakFor(BEMissionInt *self, int track,
/// int peak, int mission)` (0x153688): sum the "collected" flags of the mission
/// matching `track`, `peak`, `mission`.
pub fn be_mission_int_count_collected_on_peak_for(c: &mut Cpu) {
    c.cycles += 12;
    c.g[V0][0] = 0xf88;
    c.g[V1][0] = 0x9b50;
    c.mult(0, T4, A2, V0);
    c.mult(0, A1, A1, V1);
    c.g[V0][0] = 0x440000;
    c.g[V1][0] = 0x4a0000;
    c.g[T5][0] = sx(0x440000u32.wrapping_sub(0x26b0));
    c.g[V1][0] = sx(0x4a0000u32.wrapping_add(0x6ca8));
    c.g[T1][0] = 0;
    c.g[A0][0] = 0;
    c.g[T3][0] = 0xc;
    c.mult(0, A2, A0, T3);
    loop {
        c.cycles += 6;
        c.g[T2][0] = sx((c.g[A0][0] as u32).wrapping_add(1));
        c.g[T0][0] = 0x15;
        c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[T4][0] as u32));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
        c.g[A2][0] = sx((c.g[T5][0] as u32).wrapping_add(0x54));
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        loop {
            c.cycles += 3;
            c.g[V0][0] = lw(c, c.g[A2][0] as u32);
            if c.g[V0][0] != c.g[A3][0] {
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xc));
            } else {
                c.cycles += 3;
                c.g[V0][0] = c.lb((c.g[A0][0] as u32).wrapping_add(4));
                c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xc));
            }
            c.cycles += 3;
            c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_sub(1));
            let t = (c.g[T0][0] as i64) >= 0;
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x64));
            if t {
                event_check(c, 0x0015_36d0);
                continue;
            }
            c.cycles += 3;
            c.g[A0][0] = c.g[T2][0];
            let t = (c.g[A0][0] as i64) <= 0;
            c.mult(0, A2, A0, T3);
            if t {
                event_check(c, 0x0015_36b8);
                break;
            } else {
                c.cycles += 2;
                c.g[V0][0] = c.g[T1][0];
                c.pc = c.g[RA][0] as u32;
                return;
            }
        }
    }
}


/// `int BEMissionInt_GetMissionTrack(BEMissionInt *self, int id)` (0x153f30):
/// the track of the mission record with this id.
pub fn be_mission_int_get_mission_track(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        FIND_MISSION.call(c, 0x0015_3f40)?;
        c.g[A0][0] = MISSION_STRIDE as u64;
        c.g[V1][0] = 0x440000;
        c.mult(0, V0, V0, A0);
        c.g[V1][0] = sx(0x440000u32.wrapping_sub(0x11f0));
        frame.restore_ra(c, 0);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[V0][0] = lw(c, (c.g[V1][0] as u32).wrapping_add(0x10));
        frame.close(c);
        Ok(())
    })
}


/// Common tail of the per-flag mission getters: after `FindMission` the flag
/// word is `*((char * 0xf88) + 0x4a6ca8 + index*4 + 0x118)`, shifted right by
/// `shift` and masked to its low bit.
pub(crate) fn mission_flag(c: &mut Cpu, entry: u64, find_ret: u32, char_ret: u32, shift: u32) -> Flow {
    let frame = Frame::open(c, 0x20);
    frame.save(c, S0, 0x10);
    frame.save_ra(c, 0);
    c.cycles += entry as u64;
    FIND_MISSION.call(c, find_ret)?;
    c.g[S0][0] = c.g[V0][0];
    c.g[A0][0] = 0;
    BE_GET_RIDER_CHARACTER.call(c, char_ret)?;
    c.g[A0][0] = RIDER_STRIDE as u64;
    c.g[S0][0] = sx((c.g[S0][0] as u32) << 2);
    c.mult(0, V0, V0, A0);
    c.g[V1][0] = RIDER_FLAG_TABLE as u64;
    frame.restore_ra(c, 0);
    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.g[V0][0] = lw(c, (c.g[S0][0] as u32).wrapping_add(0x118));
    frame.restore(c, S0, 0x10);
    c.g[V0][0] = sx((c.g[V0][0] as u32) >> shift);
    c.g[V0][0] = c.g[V0][0] & 0x1;
    frame.close(c);
    Ok(())
}


/// `int BEMissionInt_IsMissionComplete(BEMissionInt *self, int id)` (0x153d78).
pub fn be_mission_int_is_mission_complete(c: &mut Cpu) {
    guest(c, |c| mission_flag(c, 22, 0x0015_3d8c, 0x0015_3d98, 3))
}


/// `int BEMissionInt_GetMissionFlag1(BEMissionInt *self, int id)` (0x153dd0).
pub fn be_mission_int_get_mission_flag1(c: &mut Cpu) {
    guest(c, |c| mission_flag(c, 22, 0x0015_3de4, 0x0015_3df0, 1))
}


/// `int BEMissionInt_GetMissionFlag0(BEMissionInt *self, int id)` (0x153e28).
pub fn be_mission_int_get_mission_flag0(c: &mut Cpu) {
    guest(c, |c| mission_flag(c, 21, 0x0015_3e3c, 0x0015_3e48, 0))
}


/// `int BEMissionInt_GetMissionFlag2(BEMissionInt *self, int id)` (0x153e80).
pub fn be_mission_int_get_mission_flag2(c: &mut Cpu) {
    guest(c, |c| mission_flag(c, 22, 0x0015_3e94, 0x0015_3ea0, 2))
}


/// `int BEMissionInt_GetMissionFlag4(BEMissionInt *self, int id)` (0x153ed8).
pub fn be_mission_int_get_mission_flag4(c: &mut Cpu) {
    guest(c, |c| mission_flag(c, 22, 0x0015_3eec, 0x0015_3ef8, 4))
}


/// Common tail of the per-flag mission setters: after `FindMission`, replace
/// bit `shift` of the flag word with the low bit of `a2`.
pub(crate) fn set_mission_flag(c: &mut Cpu, entry: u64, find_ret: u32, char_ret: u32, shift: u32, mask: u64) -> Flow {
    let frame = Frame::open(c, 0x30);
    frame.save(c, S1, 0x10);
    frame.save(c, S0, 0x20);
    frame.save_ra(c, 0);
    c.cycles += entry as u64;
    c.g[S1][0] = c.g[A2][0];
    FIND_MISSION.call(c, find_ret)?;
    c.g[S0][0] = c.g[V0][0];
    c.g[A0][0] = 0;
    BE_GET_RIDER_CHARACTER.call(c, char_ret)?;
    c.g[A0][0] = RIDER_STRIDE as u64;
    c.g[S0][0] = sx((c.g[S0][0] as u32) << 2);
    c.mult(0, V0, V0, A0);
    c.g[V1][0] = RIDER_FLAG_TABLE as u64;
    c.g[A0][0] = mask;
    c.g[S1][0] = c.g[S1][0] & 1;
    frame.restore_ra(c, 0);
    c.g[S1][0] = sx((c.g[S1][0] as u32) << shift);
    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.g[V0][0] = lw(c, (c.g[S0][0] as u32).wrapping_add(0x118));
    c.g[V0][0] = c.g[V0][0] & c.g[A0][0];
    c.g[V0][0] = c.g[V0][0] | c.g[S1][0];
    c.sw((c.g[S0][0] as u32).wrapping_add(0x118), c.g[V0][0]);
    frame.restore(c, S0, 0x20);
    frame.restore(c, S1, 0x10);
    frame.close(c);
    Ok(())
}


/// `void BEMissionInt_SetMissionFlag0(BEMissionInt *self, int id, int v)`
/// (0x154010).
pub fn be_mission_int_set_mission_flag0(c: &mut Cpu) {
    guest(c, |c| set_mission_flag(c, 27, 0x0015_4028, 0x0015_4034, 0, 0xffff_ffff_ffff_fffe))
}


/// `void BEMissionInt_SetMissionFlag1(BEMissionInt *self, int id, int v)`
/// (0x154080).
pub fn be_mission_int_set_mission_flag1(c: &mut Cpu) {
    guest(c, |c| set_mission_flag(c, 28, 0x0015_4098, 0x0015_40a4, 1, 0xffff_ffff_ffff_fffd))
}


/// `void BEMissionInt_SetMissionFlag2(BEMissionInt *self, int id, int v)`
/// (0x1540f0).
pub fn be_mission_int_set_mission_flag2(c: &mut Cpu) {
    guest(c, |c| set_mission_flag(c, 28, 0x0015_4108, 0x0015_4114, 2, 0xffff_ffff_ffff_fffb))
}


/// `void BEMissionInt_SetMissionComplete(BEMissionInt *self, int id, int v)`
/// (0x154160).
pub fn be_mission_int_set_mission_complete(c: &mut Cpu) {
    guest(c, |c| set_mission_flag(c, 28, 0x0015_4178, 0x0015_4184, 3, 0xffff_ffff_ffff_fff7))
}


/// `void BEMissionInt_SetMissionFlag4(BEMissionInt *self, int id, int v)`
/// (0x1541d0).
pub fn be_mission_int_set_mission_flag4(c: &mut Cpu) {
    guest(c, |c| set_mission_flag(c, 28, 0x0015_41e8, 0x0015_41f4, 4, 0xffff_ffff_ffff_ffef))
}


/// `void *BEMissionInt_GetNthMissionOnTrack(BEMissionInt *self, int n,
/// int track)` (0x1545b0): the `n`-th record on `track`, or 0.
pub fn be_mission_int_get_nth_mission_on_track(c: &mut Cpu) {
    c.cycles += 6;
    c.g[V0][0] = 0x440000;
    c.g[A0][0] = 0;
    c.g[V1][0] = sx(0x440000u32.wrapping_sub(0x11f0));
    c.g[A3][0] = sx(MISSION_RECORDS.wrapping_add(0xc60));
    c.g[V0][0] = lw(c, (c.g[V1][0] as u32).wrapping_add(0x10));
    loop {
        c.cycles += 2;
        if c.g[V0][0] != c.g[A2][0] {
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(MISSION_STRIDE));
        } else {
            c.cycles += 2;
            let t = c.g[A0][0] != c.g[A1][0];
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(1));
            if t {
                c.cycles += 1;
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(MISSION_STRIDE));
            } else {
                c.cycles += 2;
                c.g[V0][0] = lw(c, c.g[V1][0] as u32);
                c.pc = c.g[RA][0] as u32;
                return;
            }
        }
        c.cycles += 3;
        c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A3][0] as i64)) as u64;
        if c.g[V0][0] != 0 {
            c.g[V0][0] = lw(c, (c.g[V1][0] as u32).wrapping_add(0x10));
            event_check(c, 0x0015_45c8);
            continue;
        } else {
            c.cycles += 2;
            c.g[V0][0] = 0;
            c.pc = c.g[RA][0] as u32;
            return;
        }
    }
}


/// `int BEMissionInt_GetSessionType(BEMissionInt *self, int id)` (0x1545f8):
/// the session type at +0x18 of the matching session record, or 0.
pub fn be_mission_int_get_session_type(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V0][0] = 0x440000;
    c.g[V1][0] = sx(SESSION_TABLE);
    c.g[A0][0] = sx(SESSION_TABLE.wrapping_add(0x3c8));
    c.g[V0][0] = lw(c, SESSION_TABLE);
    loop {
        c.cycles += 2;
        if c.g[V0][0] != c.g[A1][0] {
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(SESSION_STRIDE));
            c.cycles += 3;
            c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A0][0] as i64)) as u64;
            if c.g[V0][0] != 0 {
                c.g[V0][0] = lw(c, c.g[V1][0] as u32);
                event_check(c, 0x0015_4608);
                continue;
            } else {
                c.cycles += 2;
                c.g[V0][0] = 0;
                c.pc = c.g[RA][0] as u32;
                return;
            }
        } else {
            c.cycles += 2;
            c.g[V0][0] = lw(c, (c.g[V1][0] as u32).wrapping_add(0x18));
            c.pc = c.g[RA][0] as u32;
            return;
        }
    }
}


/// `const char *BEMissionInt_GetSessionName(BEMissionInt *self, int id)`
/// (0x154760): the name field (+4) of the matching session record, or 0.
pub fn be_mission_int_get_session_name(c: &mut Cpu) {
    c.cycles += 6;
    c.g[V0][0] = sx(SESSION_TABLE);
    c.g[V1][0] = c.g[V0][0];
    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(4));
    c.g[A2][0] = sx(SESSION_TABLE.wrapping_add(0x3c8));
    loop {
        c.cycles += 3;
        c.g[V0][0] = lw(c, c.g[V1][0] as u32);
        let t = c.g[V0][0] != c.g[A1][0];
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(SESSION_STRIDE));
        if t {
            c.cycles += 3;
            c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A2][0] as i64)) as u64;
            let t = c.g[V0][0] != 0;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(SESSION_STRIDE));
            if t {
                event_check(c, 0x0015_4778);
                continue;
            } else {
                c.cycles += 2;
                c.g[V0][0] = 0;
                c.pc = c.g[RA][0] as u32;
                return;
            }
        } else {
            c.cycles += 2;
            c.g[V0][0] = c.g[A0][0];
            c.pc = c.g[RA][0] as u32;
            return;
        }
    }
}


/// `void *BEMissionInt_GetSessionInfo(BEMissionInt *self, int id, void *out)`
/// (0x154678): copy the matching 0x2c-byte session record to `out`, or the
/// first record when there is no match.
pub fn be_mission_int_get_session_info(c: &mut Cpu) {
    c.cycles += 4;
    c.g[A3][0] = 0x440000;
    c.g[V1][0] = sx(SESSION_TABLE);
    c.g[A1][0] = sx(SESSION_TABLE.wrapping_add(0x3c8));
    c.g[V0][0] = lw(c, SESSION_TABLE);
    loop {
        c.cycles += 2;
        if c.g[V0][0] != c.g[A2][0] {
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(SESSION_STRIDE));
        } else {
            c.cycles += 23;
            copy_session(c, c.g[V1][0] as u32, c.g[A0][0] as u32);
            break;
        }
        c.cycles += 3;
        c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A1][0] as i64)) as u64;
        if c.g[V0][0] != 0 {
            c.g[V0][0] = lw(c, c.g[V1][0] as u32);
            event_check(c, 0x0015_4688);
            continue;
        } else {
            c.cycles += 23;
            let first = SESSION_TABLE;
            // This copy uses r2/r3/r5, so it leaves v1 holding the last
            // doubleword of the record (r3 is reloaded for it).
            let q4 = copy_session(c, first, c.g[A0][0] as u32);
            c.g[V1][0] = q4;
            c.g[A2][0] = sx(first);
            break;
        }
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[A0][0];
    c.pc = c.g[RA][0] as u32;
}


/// Copy the 0x2c-byte session record at `src` to `dst` (two unaligned 8-byte
/// chunks, an 8-byte chunk, a 4-byte word, and the trailing word).
pub(crate) fn copy_session(c: &mut Cpu, src: u32, dst: u32) -> u64 {
    // The original reads four doublewords before writing any of them.
    let q0 = Addr(src).read_u64_unaligned(c);
    let q1 = Addr(src + 8).read_u64_unaligned(c);
    let q2 = Addr(src + 0x10).read_u64_unaligned(c);
    let q3 = Addr(src + 0x18).read_u64_unaligned(c);
    Addr(dst).write_u64_unaligned(c, q0);
    Addr(dst + 8).write_u64_unaligned(c, q1);
    Addr(dst + 0x10).write_u64_unaligned(c, q2);
    Addr(dst + 0x18).write_u64_unaligned(c, q3);
    let q4 = Addr(src + 0x20).read_u64_unaligned(c);
    let tail = lw(c, src.wrapping_add(0x28));
    Addr(dst + 0x20).write_u64_unaligned(c, q4);
    c.g[A1][0] = tail;
    c.sw(dst.wrapping_add(0x28), tail);
    q4
}


// ---------------------------------------------------------------------------
// Mission text getters: a session-string accessor through a virtual method.

/// Common body of the `BEMissionInt_GetMissionTextN` getters (0x153c88/0x153cd8/
/// 0x153d28): look the mission up, then call the per-rider table entry's
/// `+off` string getter.
pub(crate) fn get_mission_text(c: &mut Cpu, off: u32, find_ret: u32, call_ret: u32) -> Flow {
    let frame = Frame::open(c, 0x10);
    frame.save_ra(c, 0);
    c.cycles += 20;
    FIND_MISSION.call(c, find_ret)?;
    c.g[A0][0] = lw(c, (c.g[GP][0] as u32).wrapping_sub(0x848));
    c.g[V1][0] = MISSION_STRIDE as u64;
    c.mult(0, V0, V0, V1);
    c.g[A2][0] = 0x440000;
    c.g[A3][0] = lw(c, (c.g[A0][0] as u32).wrapping_add(0x8c));
    c.g[A2][0] = sx(0x440000u32.wrapping_sub(0x11f0));
    c.g[V1][0] = lw(c, (c.g[A3][0] as u32).wrapping_add(4));
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
    c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x20));
    c.g[V0][0] = lw(c, (c.g[V1][0] as u32).wrapping_add(0x24));
    c.g[A1][0] = lw(c, (c.g[A2][0] as u32).wrapping_add(off));
    let t = c.g[V0][0] as u32;
    call_ptr(c, t, call_ret)?;
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}


/// `const char *BEMissionInt_GetMissionText1(BEMissionInt *self, int id)`
/// (0x153c88).
pub fn be_mission_int_get_mission_text1(c: &mut Cpu) {
    guest(c, |c| get_mission_text(c, 4, 0x0015_3c98, 0x0015_3ccc))
}


/// `const char *BEMissionInt_GetMissionText2(BEMissionInt *self, int id)`
/// (0x153cd8).
pub fn be_mission_int_get_mission_text2(c: &mut Cpu) {
    guest(c, |c| get_mission_text(c, 8, 0x0015_3ce8, 0x0015_3d1c))
}


/// `const char *BEMissionInt_GetMissionText3(BEMissionInt *self, int id)`
/// (0x153d28).
pub fn be_mission_int_get_mission_text3(c: &mut Cpu) {
    guest(c, |c| get_mission_text(c, 0xc, 0x0015_3d38, 0x0015_3d6c))
}


/// `char BEMissionInt_GetCollectedCount(BEMissionInt *self, int rider, int idx)`
/// (0x153498): the per-rider collected count byte.
pub fn be_mission_int_get_collected_count(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 33;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S0, 0x30);
        frame.save(c, S2, 0x10);
        c.g[S0][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S1][0];
        BE_GET_RIDER_PROFILE.call(c, 0x0015_34bc)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[S2][0] = c.g[V0][0];
        BE_GET_RIDER_CHARACTER.call(c, 0x0015_34c8)?;
        c.g[A0][0] = 0x530000;
        c.g[A2][0] = 0xc;
        c.g[A1][0] = lw(c, (c.g[A0][0] as u32).wrapping_add(0x5f0));
        c.g[V1][0] = RIDER_STRIDE as u64;
        c.mult(0, V0, V0, V1);
        c.g[A0][0] = 0x9b50;
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A1][0] as u32));
        c.mult(0, S2, S2, A0);
        c.mult(0, S0, S0, A2);
        c.g[V1][0] = 0x4a0000;
        c.g[V1][0] = sx(0x4a0000u32.wrapping_add(0x6ca8));
        frame.restore(c, S1, 0x20);
        frame.restore_ra(c, 0);
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[S2][0] as u32));
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[V0][0] = c.lb((c.g[S0][0] as u32).wrapping_add(4));
        frame.restore(c, S2, 0x10);
        frame.restore(c, S0, 0x30);
        frame.close(c);
        Ok(())
    })
}


/// `int BEMissionInt_CountCollectedOnPeak(BEMissionInt *self, int rider, int
/// peak)` (0x1535c0): sum the collected bits of missions on this peak.
pub fn be_mission_int_count_collected_on_peak(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S2, 0x10);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, S1, 0x20);
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S0][0];
        BE_GET_RIDER_PROFILE.call(c, 0x0015_35e4)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S0][0];
        BE_GET_RIDER_CHARACTER.call(c, 0x0015_35f0)?;
        c.g[V1][0] = RIDER_STRIDE as u64;
        c.g[T3][0] = 0xc;
        c.mult(0, A1, V0, V1);
        c.g[A3][0] = 0;
        c.g[V0][0] = 0x9b50;
        c.g[V1][0] = 0x440000;
        c.mult(0, S1, S1, V0);
        c.g[V1][0] = sx(0x440000u32.wrapping_sub(0x26b0));
        c.g[V0][0] = 0x4a0000;
        c.g[A0][0] = 0;
        c.g[T2][0] = sx(0x4a0000u32.wrapping_add(0x6ca8));
        c.mult(0, A1, A0, T3);
        loop {
            c.cycles += 6;
            c.g[T0][0] = sx((c.g[A0][0] as u32).wrapping_add(1));
            c.g[A2][0] = 0x15;
            c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[T1][0] as u32));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S1][0] as u32));
            c.g[A1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x54));
            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[T2][0] as u32));
            loop {
                c.cycles += 3;
                c.g[V0][0] = lw(c, c.g[A1][0] as u32);
                if c.g[V0][0] != c.g[S2][0] {
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xc));
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = c.lb((c.g[A0][0] as u32).wrapping_add(4));
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xc));
                }
                c.cycles += 3;
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(1));
                let t = (c.g[A2][0] as i64) >= 0;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x64));
                if t {
                    event_check(c, 0x0015_3638);
                    continue;
                }
                c.cycles += 3;
                c.g[A0][0] = c.g[T0][0];
                if (c.g[A0][0] as i64) <= 0 {
                    c.mult(0, A1, A0, T3);
                    event_check(c, 0x0015_3620);
                    break;
                } else {
                    c.cycles += 7;
                    frame.restore(c, S0, 0x30);
                    c.g[V0][0] = c.g[A3][0];
                    frame.restore(c, S1, 0x20);
                    frame.restore(c, S2, 0x10);
                    frame.restore_ra(c, 0);
                    frame.close(c);
                    return Ok(());
                }
            }
        }
    })
}


/// `int BEMissionInt_GetCollectedMask(BEMissionInt *self, int rider, int peak)`
/// (0x153858): pointer into the per-rider collected bit table for this peak.
pub fn be_mission_int_get_collected_mask(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 35;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S2, 0x10);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, S0, 0x30);
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S1][0];
        BE_GET_RIDER_PROFILE.call(c, 0x0015_387c)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[S0][0] = c.g[V0][0];
        BE_GET_RIDER_CHARACTER.call(c, 0x0015_3888)?;
        c.g[V1][0] = 0x530000;
        c.g[A0][0] = RIDER_STRIDE as u64;
        c.g[A1][0] = lw(c, (c.g[V1][0] as u32).wrapping_add(0x5f0));
        c.mult(0, V0, V0, A0);
        c.g[A2][0] = 0xc;
        c.g[V1][0] = 0x9b50;
        c.mult(0, S0, S0, V1);
        c.mult(0, A1, A1, A2);
        c.g[V1][0] = 0x4a0000;
        frame.restore(c, S1, 0x20);
        c.mult(0, S2, S2, A2);
        c.g[V1][0] = sx(0x4a0000u32.wrapping_add(0x6ca8));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        frame.restore_ra(c, 0);
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(4));
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A1][0] as u32));
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[S2][0] as u32));
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(4));
        frame.restore(c, S2, 0x10);
        frame.restore(c, S0, 0x30);
        frame.close(c);
        Ok(())
    })
}


/// `bool BEMissionInt_IsCollected(BEMissionInt *self, int rider, int peak,
/// int idx)` (0x1538e8): whether collected bit `idx` is set for this rider/peak.
pub fn be_mission_int_is_collected(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 48;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S2, 0x20);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S1, 0x30);
        frame.save(c, S3, 0x10);
        c.g[S1][0] = c.g[A3][0];
        c.g[S3][0] = c.g[A2][0];
        frame.save(c, S0, 0x40);
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S2][0];
        BE_GET_RIDER_PROFILE.call(c, 0x0015_3914)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[S0][0] = c.g[V0][0];
        BE_GET_RIDER_CHARACTER.call(c, 0x0015_3920)?;
        c.g[A0][0] = 0x530000;
        c.g[V1][0] = RIDER_STRIDE as u64;
        c.mult(0, V0, V0, V1);
        c.g[A1][0] = lw(c, (c.g[A0][0] as u32).wrapping_add(0x5f0));
        c.g[A2][0] = 0x9b50;
        c.g[V1][0] = 0xc;
        c.mult(0, S0, S0, A2);
        c.mult(0, A1, A1, V1);
        c.g[A0][0] = 0xffff_ffff_ffff_ffff;
        frame.restore(c, S2, 0x20);
        c.mult(0, S1, S1, V1);
        c.g[A0][0] = ((c.g[A0][0] as i64) < (c.g[S3][0] as i64)) as u64;
        c.g[V1][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1f));
        frame.restore_ra(c, 0);
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        if c.g[A0][0] != 0 {
            c.g[V1][0] = c.g[S3][0];
        }
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A1][0] as u32));
        c.g[V0][0] = 0x4a0000;
        c.g[V0][0] = sx(0x4a0000u32.wrapping_add(0x6cac));
        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[S0][0] as u32));
        c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 5) as u32);
        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
        c.g[A0][0] = 0x1;
        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[A0][0] = sx((c.g[A0][0] as u32) << ((c.g[S3][0] as u32) & 31));
        c.g[V0][0] = lw(c, (c.g[S1][0] as u32).wrapping_add(4));
        frame.restore(c, S0, 0x40);
        c.g[V0][0] = c.g[V0][0] & c.g[A0][0];
        frame.restore(c, S1, 0x30);
        frame.restore(c, S3, 0x10);
        c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
        frame.close(c);
        Ok(())
    })
}

pub(crate) const COUNT_COMPLETED_ON_PEAK: Callee = Callee { addr: 0x0015_43e0, f: ssx3_ee::s01::f_001543e0 };


/// `int BEMissionInt_CountCompletedOnTrack(BEMissionInt *self, int rider,
/// int track)` (0x1542e0): number of completed missions on this track.
pub fn be_mission_int_count_completed_on_track(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x60);
        c.g[V0][0] = 0x440000;
        frame.save(c, S0, 0x50);
        frame.save(c, S1, 0x40);
        c.g[S0][0] = sx(0x440000u32.wrapping_sub(0x11f0));
        frame.save(c, S2, 0x30);
        c.g[S1][0] = 0;
        frame.save(c, S3, 0x20);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S4, 0x10);
        c.g[S3][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S4][0] = sx(MISSION_RECORDS.wrapping_add(0xc60));
        c.g[V0][0] = lw(c, (c.g[S0][0] as u32).wrapping_add(0x10));
        loop {
            c.cycles += 2;
            if c.g[V0][0] != c.g[S2][0] {
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(MISSION_STRIDE));
            } else {
                c.cycles += 6;
                c.g[A1][0] = lw(c, c.g[S0][0] as u32);
                c.g[A0][0] = c.g[S3][0];
                Callee { addr: 0x0015_3d78, f: ssx3_ee::s01::f_00153d78 }.call(c, 0x0015_432c)?;
                c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                if c.g[V0][0] != 0 {
                    c.g[S1][0] = c.g[V1][0];
                }
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(MISSION_STRIDE));
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S4][0] as i64)) as u64;
            if c.g[V0][0] != 0 {
                c.g[V0][0] = lw(c, (c.g[S0][0] as u32).wrapping_add(0x10));
                event_check(c, 0x0015_4318);
                continue;
            } else {
                c.cycles += 9;
                c.g[V0][0] = c.g[S1][0];
                frame.restore(c, S0, 0x50);
                frame.restore(c, S1, 0x40);
                frame.restore(c, S2, 0x30);
                frame.restore(c, S3, 0x20);
                frame.restore(c, S4, 0x10);
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
        }
    })
}


/// `int BEMissionInt_CountCompletedOnPeakFor(BEMissionInt *self, int rider,
/// int peak, int track)` (0x1544d0): count completed missions of `rider` on
/// `peak` and `track`.
pub fn be_mission_int_count_completed_on_peak_for(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        c.g[V1][0] = lw(c, (c.g[GP][0] as u32).wrapping_sub(0x848));
        frame.save_ra(c, 0);
        c.g[V0][0] = lw(c, (c.g[V1][0] as u32).wrapping_add(0x84));
        let t = c.g[V0][0] == 0;
        c.g[T1][0] = c.g[A2][0];
        if !t {
            c.cycles += 4;
            c.g[A1][0] = c.g[A3][0];
            COUNT_COMPLETED_ON_PEAK.call(c, 0x0015_44f0)?;
            c.cycles += 2;
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        c.cycles += 18;
        c.g[V1][0] = 0x9b50;
        c.g[A0][0] = RIDER_STRIDE as u64;
        c.mult(0, V1, A1, V1);
        c.mult(0, A0, T1, A0);
        c.g[A1][0] = 0x4a0000;
        c.g[A2][0] = 0x440000;
        c.g[A1][0] = sx(0x4a0000u32.wrapping_add(0x6ca8));
        c.g[V0][0] = 0x440000;
        c.g[V0][0] = sx(0x440000u32.wrapping_sub(0x11f0));
        c.g[T2][0] = sx(0x440000u32.wrapping_sub(0x265c));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x118));
        c.g[T0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10));
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[A2][0] = 0;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
        c.g[T1][0] = 0x64;
        c.g[A1][0] = 0x57;
        loop {
            c.cycles += 6;
            c.g[V0][0] = lw(c, c.g[T0][0] as u32);
            c.mult(0, V1, V0, T1);
            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[T2][0] as u32));
            c.g[V1][0] = lw(c, c.g[V0][0] as u32);
            if c.g[V1][0] != c.g[A3][0] {
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(4));
            } else {
                c.cycles += 5;
                c.g[V0][0] = lw(c, c.g[A0][0] as u32);
                c.g[V0][0] = sx((c.g[V0][0] as u32) >> 3);
                c.g[V0][0] = c.g[V0][0] & 1;
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(4));
            }
            c.cycles += 3;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_sub(1));
            let t = (c.g[A1][0] as i64) >= 0;
            c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add(MISSION_STRIDE));
            if t {
                event_check(c, 0x0015_4540);
                continue;
            } else {
                c.cycles += 2;
                c.g[V0][0] = c.g[A2][0];
                c.cycles += 2;
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
        }
    })
}


// ---------------------------------------------------------------------------
// BE mission / reward lookups.

/// `int BEMissionInt_CountMissionsOnCurrentTrack(BEMissionInt* m)`
/// (0x154278): count the missions whose track id equals the current one
/// (`D_005305F0`).
pub fn be_mission_int_count_missions_on_current_track(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        c.g[V0][0] = 0x0053_0000;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = c.lw(r32(c, V0).wrapping_add(0x5f0));
        frame.save_ra(c, 0);
        callee::MISSION_COUNT.call(c, 0x0015_4290)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `int BEMissionInt_GetNthMissionOnCurrentTrack(BEMissionInt* m, int n)`
/// (0x154588): the `n`-th mission on the current track (`D_005305F0`), or 0.
pub fn be_mission_int_get_nth_mission_on_current_track(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        c.g[V0][0] = 0x0053_0000;
        let frame = Frame::open(c, 0x10);
        c.g[A2][0] = c.lw(r32(c, V0).wrapping_add(0x5f0));
        frame.save_ra(c, 0);
        callee::MISSION_NTH.call(c, 0x0015_45a0)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// BE mission table: 88 × 0x24-byte `BEMissionInt` records.
pub(crate) const MISSION_TABLE: u32 = 0x0044_0000u32.wrapping_sub(0x11f0);


/// `int16 BEMissionInt_GetMissionField18(BEMissionInt* missions, int id)`
/// (0x153fa0): the signed halfword at +0x18 of mission `id`.
pub fn mission_get_field18(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        callee::FIND_MISSION.call(c, 0x0015_3fb0)?;
        c.g[A0][0] = 0x24;
        c.g[V1][0] = 0x0044_0000;
        c.mult(0, V0, V0, A0);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x11f0));
        frame.restore_ra(c, 0);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[V0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x18)).lh(c);
        frame.close(c);
        Ok(())
    });
}


/// `uint32 BEMissionInt_GetMissionPrereq(BEMissionInt* missions, int id)`
/// (0x153fd8): the word at +0x1c of mission `id` (the prerequisite id).
pub fn mission_get_prereq(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        callee::FIND_MISSION.call(c, 0x0015_3fe8)?;
        c.g[A0][0] = 0x24;
        c.g[V1][0] = 0x0044_0000;
        c.mult(0, V0, V0, A0);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x11f0));
        frame.restore_ra(c, 0);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0x1c)).lw(c);
        frame.close(c);
        Ok(())
    });
}


/// `int16 BEMissionInt_GetMissionField22(BEMissionInt* missions, int id)`
/// (0x1547a0): linear search over the 88 mission records, halfword at +0x22.
pub fn mission_get_field22(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V0][0] = 0x0044_0000;
    c.g[V1][0] = sx(MISSION_TABLE);
    c.g[A0][0] = sx(MISSION_TABLE.wrapping_add(0xc60));
    c.g[V0][0] = Addr(MISSION_TABLE).lw(c);
    let mut cur = MISSION_TABLE;
    loop {
        c.cycles += 2;
        if c.g[V0][0] != c.g[A1][0] {
            cur = cur.wrapping_add(0x24);
            c.g[V1][0] = sx(cur);
            c.cycles += 3;
            let in_table = (sx(cur) as i64) < (c.g[A0][0] as i64);
            c.g[V0][0] = in_table as u64;
            if !in_table {
                c.cycles += 2;
                c.g[V0][0] = 0;
                crate::abi::ret(c);
                return;
            }
            c.g[V0][0] = Addr(cur).lw(c);
            if event_due(c) {
                deliver_event_here(c, 0x0015_47b0);
            }
        } else {
            c.cycles += 2;
            c.g[V0][0] = Addr(cur.wrapping_add(0x22)).lh(c);
            crate::abi::ret(c);
            return;
        }
    }
}


/// `int BEMissionInt_GetTotalCollectibles(this, int start)` (0x153350): the
/// signed sum of the collectible-count bytes from `&D_0043FA70 + start`
/// (the reference's loop runs once: the bound is a constant 0).
pub fn be_mission_int_get_total_collectibles(c: &mut Cpu) {
    c.cycles += 6;
    c.g[V0][0] = 0x0044_0000;
    c.g[A0][0] = 0;
    c.g[V0][0] = sx(0x0043_fa70);
    c.g[V1][0] = c.g[A0][0];
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
    loop {
        c.cycles += 7;
        c.g[V0][0] = Addr(c.g[A1][0] as u32).lb(c);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
        let more = (c.g[V1][0] as i64) >= 0;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        if more {
            if event_due(c) {
                deliver_event_here(c, 0x0015_3368);
            }
            continue;
        }
        c.cycles += 2;
        c.g[V0][0] = c.g[A0][0];
        abi::ret(c);
        return;
    }
}


// ---------------------------------------------------------------------------
// BE mission accessor.

/// The 22 mission-session records `BEMissionInt_SessionTypeAtLeast` scans:
/// `{ int id; int pad[5]; int value; }` (0x2c bytes each).
pub(crate) const MISSION_SESSIONS: u32 = 0x0044_0770;

pub(crate) const MISSION_SESSION_COUNT: u32 = 22;

pub(crate) const MISSION_SESSION_SIZE: u32 = 0x2c;

pub(crate) const SESSION_ID: u32 = 0x00;

pub(crate) const SESSION_VALUE: u32 = 0x18;


/// `bool BEMissionInt_SessionTypeAtLeast(int value /*a1*/, int session_type
/// /*a2*/)` (0x154630): true when the session type's record is present and
/// its value is at least `value`.
pub fn be_mission_int_session_type_at_least(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V0][0] = (MISSION_SESSIONS & 0xffff_0000) as u64;
    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(MISSION_SESSIONS & 0xffff));
    c.g[A0][0] = sx((c.g[V1][0] as u32).wrapping_add(MISSION_SESSION_SIZE * MISSION_SESSION_COUNT));
    c.g[V0][0] = c.lw(r32(c, V1));
    loop {
        // `id == session_type`: is the record's value at least `value`?
        c.cycles += 2;
        if c.g[V0][0] != c.g[A2][0] {
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(MISSION_SESSION_SIZE));
        } else {
            c.cycles += 4;
            c.g[V0][0] = c.lw(p(r32(c, V1), SESSION_ID + SESSION_VALUE));
            c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[A1][0] as i64)) as u64;
            let too_small = c.g[V0][0] != 0;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(MISSION_SESSION_SIZE));
            if !too_small {
                c.cycles += 2;
                c.g[V0][0] = 1;
                abi::ret(c);
                return;
            }
        }
        c.cycles += 3;
        c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A0][0] as i64)) as u64;
        if c.g[V0][0] != 0 {
            c.g[V0][0] = c.lw(r32(c, V1));
            if event_due(c) {
                deliver_event_here(c, 0x0015_4640);
            }
            continue;
        }
        break;
    }
    c.cycles += 2;
    c.g[V0][0] = 0;
    abi::ret(c);
}


// ---------------------------------------------------------------------------
// Data scans.

/// `int BEMissionInt_GetPeakCollectibleTotal(iface, int owner)` (0x153390):
/// for each of the 22 mission records whose `owner` field (at `+0x54`) is
/// `owner`, add the matching byte of the per-mission table.
pub fn be_mission_int_get_peak_collectible_total(c: &mut Cpu) {
    c.cycles += 8;
    c.g[V0][0] = 0x440000;
    c.g[V1][0] = 0x440000;
    // 0x43d950: 22 mission records (0x64 bytes each), owner at +0x54.
    c.g[T2][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x26b0));
    // 0x43fa70: per-record contribution bytes.
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x590));
    c.g[T0][0] = 0u64;
    c.g[V0][0] = 0u64;
    c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
    loop {
        c.cycles += 4;
        c.g[T1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
        c.g[A0][0] = sx((c.g[T2][0] as u32).wrapping_add(0x54));
        c.g[A3][0] = 0x15;
        loop {
            c.cycles += 3;
            c.g[V0][0] = c.lw((c.g[A0][0] as u32));
            if c.g[V0][0] != c.g[A1][0] {
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
            } else {
                c.cycles += 3;
                c.g[V0][0] = c.lb((c.g[A2][0] as u32));
                c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_add((c.g[V0][0] as u32)));
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
            }
            c.cycles += 3;
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x1));
            let more = (c.g[A3][0] as i64) >= 0;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x64));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0015_33c0);
            }
        }
        c.cycles += 3;
        c.g[V0][0] = c.g[T1][0];
        let done = (c.g[V0][0] as i64) <= 0;
        c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
        if !done {
            c.cycles += 2;
            c.g[V0][0] = c.g[T0][0];
            abi::ret(c);
            return;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0015_33b0);
        }
    }
}


/// `int BEMissionInt_GetTrackCollectedCount(rider, a, idx)` (0x153520): sum the
/// per-slot collected bytes of the character's profile record.
pub fn be_mission_int_get_track_collected_count(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S0, 0x30);
        c.g[S0][0] = c.g[A2][0];
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S1][0];
        callee::BE_GET_RIDER_PROFILE.call(c, 0x0015_3544)?;
        c.g[S2][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S1][0];
        callee::BE_GET_RIDER_CHARACTER.call(c, 0x0015_3550)?;
        c.g[V1][0] = 0xc;
        c.g[A0][0] = 0xf88;
        c.mult(0, S0, S0, V1);
        c.mult(0, V0, V0, A0);
        c.g[A1][0] = 0x9b50;
        c.g[V1][0] = 0x004a_6ca8;
        c.mult(0, S2, S2, A1);
        c.g[A1][0] = 0;
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[A0][0] = c.g[A1][0];
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[S2][0] as u32));
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.cycles += 7;
        c.g[V0][0] = Addr(c.g[S0][0] as u32).off(4).lb(c);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(1));
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xc));
        let more = (c.g[A0][0] as i64) >= 0;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        if more {
            if event_due(c) {
                deliver_event_here(c, 0x0015_3588);
            }
            // The counter starts at 0, so the second iteration never runs; the
            // generated loop is reproduced for shape.
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x30);
        c.g[V0][0] = c.g[A1][0];
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


// ---------------------------------------------------------------------------
// `BEMissionInt_LoadState` (0x154898): stream-load the selected character's
// two record blocks and the mission's own state into `self`.
pub fn bemission_int_load_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 43;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S3, 0x10);
        frame.save(c, S1, 0x30);
        c.g[S3][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S0, 0x40);
        frame.save(c, S2, 0x20);
        frame.save_ra(c, 0);
        c.g[A0][0] = 0u64;
        callee::BE_GET_RIDER_CHARACTER_2.call(c, 0x0015_48c0)?;
        c.g[V1][0] = 0xf88;
        c.g[A1][0] = c.lw((c.g[S1][0] as u32));
        c.mult(0, 18, 2, 3);
        c.g[S0][0] = 0x4a0000;
        c.g[A0][0] = c.lh((c.g[A1][0] as u32).wrapping_add(0x10));
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x6cac));
        c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x14));
        c.g[A2][0] = 0x114;
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        let target = c.g[V0][0] as u32;
        c.g[A1][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[S0][0] as u32)));
        call_ptr(c, target, 0x0015_48ec)?;
        c.g[V1][0] = c.lw((c.g[S1][0] as u32));
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x114));
        c.g[A1][0] = sx((c.g[S2][0] as u32).wrapping_add((c.g[S0][0] as u32)));
        c.g[A2][0] = 0x160;
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x10));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        call_ptr(c, target, 0x0015_490c)?;
        c.g[V1][0] = c.lw((c.g[S1][0] as u32));
        c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_add(0x8));
        c.g[A2][0] = 0x4;
        c.g[A0][0] = c.lh((c.g[V1][0] as u32).wrapping_add(0x10));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
        let target = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        call_ptr(c, target, 0x0015_4928)?;
        c.lq(16, frame.at(c, 0x40));
        c.lq(17, frame.at(c, 0x30));
        c.lq(18, frame.at(c, 0x20));
        c.lq(19, frame.at(c, 0x10));
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}


/// `BEMissionInt_CountCompletedOnPeak(self, a1)` (0x1543e0): count, over the
/// 88 missions of the current peak and their 22 sub-goals, how many are
/// completed: a matching id pair queries `BEMissionInt_IsMissionComplete`.
pub fn bemissionint_count_completed_on_peak(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0xa0);
        let this = abi::arg(c, 0);
        let a1 = abi::arg(c, 1);
        c.g[V0][0] = 0x44_0000;
        frame.save(c, 20, 0x50);
        c.g[V1][0] = 0;
        frame.save(c, 22, 0x30);
        c.g[S4][0] = 0;
        frame.save(c, 23, 0x20);
        c.g[S6][0] = c.g[A1][0];
        frame.save(c, 30, 0x10);
        c.g[S7][0] = c.g[A0][0];
        frame.save(c, 16, 0x90);
        c.g[FP][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x11f0));
        frame.save(c, 17, 0x80);
        frame.save(c, 18, 0x70);
        frame.save(c, 19, 0x60);
        frame.save(c, 21, 0x40);
        frame.save_ra(c, 0);
        c.g[A0][0] = 0x24;
        loop {
            c.cycles += 10;
            c.g[S5][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
            c.mult(0, V0, V1, A0); // v0 = i * 0x24
            c.g[S1][0] = 0;
            c.g[V1][0] = 0x44_0000;
            c.g[A0][0] = 0x44_0000;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x26b0));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x11e0));
            c.g[S0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x54));
            c.g[S3][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[FP][0] as u32)));
            c.g[S2][0] = sx((c.g[V0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
            loop {
                c.cycles += 3;
                c.g[V0][0] = lw_2(c, c.g[S2][0] as u32);
                let mut next = false;
                if c.g[V0][0] != c.g[S1][0] {
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                    next = true;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = lw_2(c, c.g[S0][0] as u32);
                    if c.g[V0][0] != c.g[S6][0] {
                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                        next = true;
                    } else {
                        c.cycles += 6;
                        c.g[A1][0] = lw_2(c, c.g[S3][0] as u32);
                        c.g[RA][0] = sx(0x0015_4474);
                        c.g[A0][0] = c.g[S7][0];
                        c.pc = 0x0015_3d78;
                        ee::s01::BEMissionInt_IsMissionComplete(c);
                        if c.pc != 0x0015_4474 {
                            return Err(Unwound);
                        }
                        c.g[V1][0] = sx((c.g[S4][0] as u32).wrapping_add(1));
                        if c.g[V0][0] != 0 {
                            c.g[S4][0] = c.g[V1][0];
                        }
                        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                        next = true;
                    }
                }
                let _ = next;
                c.cycles += 3;
                c.g[V0][0] = ((c.g[S1][0] as i64) < 22) as u64;
                let t = c.g[V0][0] != 0;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x64));
                if !t {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0015_4450);
                }
            }
            c.cycles += 4;
            c.g[V1][0] = c.g[S5][0];
            c.g[V0][0] = ((c.g[V1][0] as i64) < 88) as u64;
            if c.g[V0][0] == 0 {
                break;
            }
            c.g[A0][0] = 0x24;
            if event_due(c) {
                deliver_event_here(c, 0x0015_4428);
            }
        }
        c.cycles += 13;
        c.g[V0][0] = c.g[S4][0];
        frame.restore(c, 16, 0x90);
        frame.restore(c, 17, 0x80);
        frame.restore(c, 18, 0x70);
        frame.restore(c, 19, 0x60);
        frame.restore(c, 20, 0x50);
        frame.restore(c, 21, 0x40);
        frame.restore(c, 22, 0x30);
        frame.restore(c, 23, 0x20);
        frame.restore(c, 30, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::fakes::machine;
    use crate::difftest::check;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::fill;
    use crate::difftest::fakes::LOG_UNWIND_AT;
    use crate::difftest::scramble;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::events::check_events;
    use crate::difftest::events::driven;
    use crate::difftest::events::schedule;
    use crate::difftest::fakes::ptr;
    use crate::difftest::events::Events;
    use crate::difftest::fakes::vtable;
    use crate::difftest::RA as RET;
    use ssx3_ee as ee;
    use crate::difftest::SP as STACK;
    use crate::difftest::check_lohi;
    use crate::difftest::fakes;
    use crate::difftest::put_vec;
    use crate::difftest::put_words;
    use crate::difftest::RA as SENTINEL;
    use crate::difftest::compare;
    pub(crate) use crate::game::main::gamemode::tests::SEEDS as SEEDS_2;
    pub(crate) use crate::game::dirtysock::tags2::tests::be_library_2 as be_library;
    pub(crate) use crate::game::fe::fereal::tests::base;
    pub(crate) use crate::game::fe::feasyncfile::tests::w_4 as w;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_9 as globals;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_7 as pool;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_9 as heap_init;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_11 as base_2;
    pub(crate) use crate::game::bx::bxstring2::tests::W_2 as W;
    pub(crate) use crate::game::bx::bxstring2::tests::RM;
    pub(crate) use crate::game::bx::bxstring2::tests::RIDERS;
    pub(crate) use crate::game::bx::bxstring2::tests::BE_2 as BE;
    pub(crate) use crate::game::bx::bxstring2::tests::DB;
    pub(crate) use crate::game::bx::bxstring2::tests::VTS;
    pub(crate) use crate::game::bx::bxstring2::tests::gp_at;
    pub(crate) use crate::game::bx::bxstring2::tests::base_3;
    pub(crate) use crate::game::ai::ai::tests::SEEDS as SEEDS_3;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::tests::w as w_2;
    pub(crate) use crate::game::ai::ai::tests::strict;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::tests::globals_3 as globals_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::tests::pool_4 as pool_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::tests::base_6 as base_4;


    /// Common setup for the singleton accessors: scrambled machine, gp set,
    /// the gp-relative singleton slot mostly empty (so the allocation path is
    /// taken) or pre-filled.
    pub(crate) fn instance_setup(c: &mut Cpu, seed: u32, slot: u32, salt: u64) -> Rng {
        let mut r = machine(c, seed, salt, 0);
        c.bus.write32(GAME_GP.wrapping_sub(slot), if seed % 3 == 0 { 0 } else { DATA + 0x100 });
        r
    }


    #[test]
    fn be_mission_int_get_instance_matches() {
        check(ssx3_ee::s01::f_001532a0, be_mission_int_get_instance, 90, &|c, seed| {
            instance_setup(c, seed, 0x1eb8, 0x315);
        });
    }


    /// Fill the mission/peak record tables with a mix of ids and tracks.
    pub(crate) fn fill_missions(c: &mut Cpu, r: &mut Rng, tracks: &[u32]) {
        for i in 0..88u32 {
            c.bus.write32(MISSION_RECORDS + i * 0x24, 0x1000 + i);
            c.bus.write32(MISSION_RECORDS + i * 0x24 + 0x10, tracks[(i as usize) % tracks.len()]);
        }
        fill(c, r, 0x43d000, 0xc00);
    }


    #[test]
    fn be_mission_int_find_mission_matches() {
        check(ssx3_ee::s01::f_00154240, be_mission_int_find_mission, 60, &|c, seed| {
            let mut r = machine(c, seed, 0xf00d, 0);
            fill_missions(c, &mut r, &[1, 2, 3]);
            let idx = seed as u32 % 89;
            c.g[A1][0] = if idx < 88 { (0x1000 + idx) as u64 } else { 0x9999 };
        });
    }


    #[test]
    fn be_mission_int_count_missions_on_track_matches() {
        check(ssx3_ee::s01::f_001542a0, be_mission_int_count_missions_on_track, 40, &|c, seed| {
            let mut r = machine(c, seed, 0x42a0, 0);
            fill_missions(c, &mut r, &[0, 1, 2]);
            c.g[A1][0] = (seed % 3) as u64;
        });
    }


    #[test]
    fn be_mission_int_count_missions_on_peak_matches() {
        check(ssx3_ee::s01::f_00154368, be_mission_int_count_missions_on_peak, 40, &|c, seed| {
            let mut r = machine(c, seed, 0x4368, 0);
            fill_missions(c, &mut r, &[0, 1, 2]);
            // Make the first peak-table entry equal a1 on even seeds (match
            // branch), otherwise use a value that never appears.
            c.g[A1][0] = if seed % 2 == 0 {
                c.bus.read32(0x43d9a4) as u64
            } else {
                0xdead_beef
            };
        });
    }


    #[test]
    fn be_mission_int_count_collected_on_peak_for_matches() {
        check(ssx3_ee::s01::f_00153688, be_mission_int_count_collected_on_peak_for, 40, &|c, seed| {
            let mut r = machine(c, seed, 0x3688, 0);
            fill_missions(c, &mut r, &[0, 1, 2]);
            c.g[A1][0] = (seed % 3) as u64; // track
            c.g[A2][0] = if seed % 2 == 0 {
                c.bus.read32(0x43d9a4) as u64
            } else {
                0xdead_beef
            };
            c.g[A3][0] = (seed % 5) as u64; // mission
        });
    }


    /// Shared setup for the mission-flag getters/setters: a found mission id
    /// (or not) in `a1`, plus the flag word region.
    pub(crate) fn flag_setup(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let mut r = machine(c, seed, salt, 0);
        fill_missions(c, &mut r, &[0, 1, 2]);
        let idx = seed as u32 % 90;
        c.g[A1][0] = if idx < 88 { (0x1000 + idx) as u64 } else { 0x9999 };
        c.g[A2][0] = (r.next_u32() | 1) as u64;
        fill(c, &mut r, RIDER_FLAG_TABLE, 0x200);
        r
    }


    #[test]
    fn be_mission_int_flag_getters_match() {
        for (gen, hand, salt) in [
            (ssx3_ee::s01::f_00153d78 as fn(&mut Cpu), be_mission_int_is_mission_complete as fn(&mut Cpu), 0x3d78),
            (ssx3_ee::s01::f_00153dd0, be_mission_int_get_mission_flag1, 0x3dd0),
            (ssx3_ee::s01::f_00153e28, be_mission_int_get_mission_flag0, 0x3e28),
            (ssx3_ee::s01::f_00153e80, be_mission_int_get_mission_flag2, 0x3e80),
            (ssx3_ee::s01::f_00153ed8, be_mission_int_get_mission_flag4, 0x3ed8),
        ] {
            check(gen, hand, 40, &|c, seed| {
                flag_setup(c, seed, salt);
            });
        }
    }


    #[test]
    fn be_mission_int_flag_setters_match() {
        for (gen, hand, salt) in [
            (ssx3_ee::s01::f_00154010 as fn(&mut Cpu), be_mission_int_set_mission_flag0 as fn(&mut Cpu), 0x4010),
            (ssx3_ee::s01::f_00154080, be_mission_int_set_mission_flag1, 0x4080),
            (ssx3_ee::s01::f_001540f0, be_mission_int_set_mission_flag2, 0x40f0),
            (ssx3_ee::s01::f_00154160, be_mission_int_set_mission_complete, 0x4160),
            (ssx3_ee::s01::f_001541d0, be_mission_int_set_mission_flag4, 0x41d0),
        ] {
            check(gen, hand, 40, &|c, seed| {
                flag_setup(c, seed, salt);
            });
        }
    }


    #[test]
    fn be_mission_int_get_mission_track_matches() {
        check(ssx3_ee::s01::f_00153f30, be_mission_int_get_mission_track, 40, &|c, seed| {
            let mut r = machine(c, seed, 0x3f30, 0);
            fill_missions(c, &mut r, &[0, 1, 2]);
            let idx = seed as u32 % 90;
            c.g[A1][0] = if idx < 88 { (0x1000 + idx) as u64 } else { 0x9999 };
        });
    }


    #[test]
    fn be_mission_int_get_nth_mission_on_track_matches() {
        check(ssx3_ee::s01::f_001545b0, be_mission_int_get_nth_mission_on_track, 40, &|c, seed| {
            let mut r = machine(c, seed, 0x45b0, 0);
            fill_missions(c, &mut r, &[0, 1, 2]);
            c.g[A1][0] = (seed % 5) as u64; // n-th on track
            c.g[A2][0] = (seed % 3) as u64; // track
        });
    }


    #[test]
    fn be_mission_int_session_getters_match() {
        for (gen, hand, salt) in [
            (ssx3_ee::s01::f_001545f8 as fn(&mut Cpu), be_mission_int_get_session_type as fn(&mut Cpu), 0x45f8),
            (ssx3_ee::s01::f_00154760, be_mission_int_get_session_name, 0x4760),
        ] {
            check(gen, hand, 40, &|c, seed| {
                let mut r = machine(c, seed, salt, 0);
                for i in 0..22u32 {
                    c.bus.write32(SESSION_TABLE + i * SESSION_STRIDE, 0x2000 + i);
                    fill(c, &mut r, SESSION_TABLE + i * SESSION_STRIDE + 4, 0xa);
                }
                let idx = seed as u32 % 23;
                c.g[A1][0] = if idx < 22 { (0x2000 + idx) as u64 } else { 0x9999 };
            });
        }
    }


    #[test]
    fn be_mission_int_get_session_info_matches() {
        check(ssx3_ee::s01::f_00154678, be_mission_int_get_session_info, 40, &|c, seed| {
            let mut r = machine(c, seed, 0x4678, 0);
            for i in 0..22u32 {
                c.bus.write32(SESSION_TABLE + i * SESSION_STRIDE, 0x2000 + i);
                fill(c, &mut r, SESSION_TABLE + i * SESSION_STRIDE + 4, 0xa);
            }
            let idx = seed as u32 % 23;
            c.g[A2][0] = if idx < 22 { (0x2000 + idx) as u64 } else { 0x9999 };
            c.g[A0][0] = (DATA + 0x1000) as u64;
        });
    }


    #[test]
    fn be_mission_int_get_mission_text_matches() {
        for (gen, hand, salt) in [
            (ssx3_ee::s01::f_00153c88 as fn(&mut Cpu), be_mission_int_get_mission_text1 as fn(&mut Cpu), 0x3c88),
            (ssx3_ee::s01::f_00153cd8, be_mission_int_get_mission_text2, 0x3cd8),
            (ssx3_ee::s01::f_00153d28, be_mission_int_get_mission_text3, 0x3d28),
        ] {
            check(gen, hand, 40, &|c, seed| {
                let mut r = machine(c, seed, salt, 1);
                fill_missions(c, &mut r, &[0, 1, 2]);
                let idx = seed as u32 % 90;
                c.g[A1][0] = if idx < 88 { (0x1000 + idx) as u64 } else { 0x9999 };
            });
        }
    }


    #[test]
    fn be_mission_int_collected_matches() {
        for (gen, hand, salt) in [
            (ssx3_ee::s01::f_00153498 as fn(&mut Cpu), be_mission_int_get_collected_count as fn(&mut Cpu), 0x3498),
            (ssx3_ee::s01::f_001535c0, be_mission_int_count_collected_on_peak, 0x35c0),
            (ssx3_ee::s01::f_00153858, be_mission_int_get_collected_mask, 0x3858),
            (ssx3_ee::s01::f_001538e8, be_mission_int_is_collected, 0x38e8),
        ] {
            check(gen, hand, 40, &|c, seed| {
                let mut r = machine(c, seed, salt, 0);
                c.g[A1][0] = (seed % 8) as u64;
                c.g[A2][0] = (seed % 8) as u64;
                c.g[A3][0] = (seed % 20) as u64;
                c.g[T1][0] = 0;
                fill(c, &mut r, 0x530000, 0x1000);
                fill(c, &mut r, 0x43d000, 0x1000);
            });
        }
    }


    #[test]
    fn be_mission_int_count_completed_matches() {
        for (gen, hand, salt) in [
            (ssx3_ee::s01::f_001542e0 as fn(&mut Cpu), be_mission_int_count_completed_on_track as fn(&mut Cpu), 0x42e0),
            (ssx3_ee::s01::f_001544d0, be_mission_int_count_completed_on_peak_for, 0x44d0),
        ] {
            check(gen, hand, 40, &|c, seed| {
                let mut r = machine(c, seed, salt, 0);
                c.g[A0][0] = (DATA + 0x100) as u64;
                c.g[A1][0] = (seed % 8) as u64;
                c.g[A2][0] = (seed % 8) as u64;
                c.g[A3][0] = (seed % 4) as u64;
                // World chain so `world->+0x84` is sometimes present.
                let world = if seed % 3 == 0 { 0 } else { DATA + 0x400 };
                c.bus.write32(GAME_GP.wrapping_sub(0x848), world);
                fill_missions(c, &mut r, &[0, 1, 2, 3]);
                fill(c, &mut r, 0x530000, 0x1000);
            });
        }
    }


    pub(crate) const SEEDS: u32 = 60;


    #[test]
    fn be_mission_int_count_missions_on_current_track_matches() {
        crate::difftest::check(
            ee::s01::f_00154278__generated,
            be_mission_int_count_missions_on_current_track,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1542_78, 2);
                // The current track id (`D_005305F0`); 0 makes every table
                // entry match, other values only some.
                c.bus.write32(0x0053_05f0, if seed % 3 == 0 { 0 } else { r.next_u32() });
                c.g[A0][0] = sx(DATA);
            },
        );
    }


    #[test]
    fn be_mission_int_get_nth_mission_on_current_track_matches() {
        crate::difftest::check(
            ee::s01::f_00154588__generated,
            be_mission_int_get_nth_mission_on_current_track,
            SEEDS,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1545_88, 2);
                c.bus.write32(0x0053_05f0, if seed % 3 == 0 { 0 } else { r.next_u32() });
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = (seed % 4) as u64;
            },
        );
    }


    pub(crate) fn rng(seed: u32, salt: u64) -> Rng {
        Rng(0x9e37_79b9_7f4a_7c15 ^ (seed as u64) << 8 ^ salt)
    }


    /// The 88 mission records the BE mission accessors search.
    pub(crate) fn mission_table(c: &mut Cpu, r: &mut Rng) {
        for k in 0..88u32 {
            let e = 0x0042_ee10 + 0x24 * k;
            c.bus.write32(e, k * 7);
            c.bus.write32(e + 0x1c, r.next_u32());
            c.bus.write16(e + 0x18, r.next_u32() as u16);
            c.bus.write16(e + 0x22, r.next_u32() as u16);
        }
    }


    #[test]
    fn mission_get_field18_matches() {
        check(ssx3_ee::s01::f_00153fa0__generated, mission_get_field18, SEEDS_2, &|c, seed| {
            let mut r = rng(seed, 0x3fa0);
            mission_table(c, &mut r);
            let ids: [u64; 6] = [0, 7, 609, 616, u64::MAX, 14];
            c.g[A1][0] = ids[seed as usize % ids.len()];
        });
    }


    #[test]
    fn mission_get_prereq_matches() {
        check(ssx3_ee::s01::f_00153fd8__generated, mission_get_prereq, SEEDS_2, &|c, seed| {
            let mut r = rng(seed, 0x3fd8);
            mission_table(c, &mut r);
            let ids: [u64; 6] = [0, 7, 609, 616, u64::MAX, 21];
            c.g[A1][0] = ids[seed as usize % ids.len()];
        });
    }


    #[test]
    fn mission_get_field22_matches() {
        check(ssx3_ee::s01::f_001547a0__generated, mission_get_field22, SEEDS_2, &|c, seed| {
            let mut r = rng(seed, 0x47a0);
            mission_table(c, &mut r);
            let ids: [u64; 6] = [0, 35, 609, 616, u64::MAX, 0xffff_ffff];
            c.g[A1][0] = ids[seed as usize % ids.len()];
        });
    }


    #[test]
    fn be_mission_int_get_total_collectibles_matches() {
        check(ssx3_ee::s01::f_00153350__generated, be_mission_int_get_total_collectibles, 64, &|c, seed| {
            let mut r = machine(c, seed, 0x1533_50, 0);
            let table = 0x0043_fa70u32;
            let start = r.next_u32() % 8;
            c.g[A0][0] = r.next_u32() as u64;
            c.g[A1][0] = start as u64;
            for k in 0..10u32 {
                c.bus.write8(table + k, r.next_u32() as u8);
            }
        });
    }


    #[test]
    fn be_mission_int_session_type_at_least_matches() {
        let setup = |c: &mut Cpu, seed: u32| -> Events {
            let mut r = machine(c, seed, 0x1546, 0);
            be_library(c);
            for i in 0..22u32 {
                let e = 0x0044_0770 + 0x2c * i;
                c.bus.write32(e, i);
                c.bus.write32(e + 0x18, 10 * i);
            }
            // (session type, value): no match, matching with a high enough
            // value, matching with too small a value, the last record, and a
            // signed value.
            let (id, value): (u64, u64) = match seed % 5 {
                0 => (999, 0),
                1 => (5, 30),
                2 => (5, 100),
                3 => (21, 210),
                _ => (0, u64::MAX),
            };
            c.g[A1][0] = value;
            c.g[A2][0] = id;
            let _ = r.next_u32();
            schedule(c, &mut r, seed, 0x0015_4630)
        };
        check_events(
            ssx3_ee::s01::f_00154630__generated,
            be_mission_int_session_type_at_least,
            80,
            &setup,
        );
    }


    

    #[test]
    fn be_mission_int_get_peak_collectible_total_matches() {
        check(ee::s01::f_00153390__generated, be_mission_int_get_peak_collectible_total, 40, &|c, seed| {
            let mut r = base(c, seed, 0x15_3390);
            let owner = if seed % 3 == 0 { 0x1234 } else { seed % 7 };
            c.g[A1][0] = owner as u64;
            for j in 0..22u32 {
                let rec = 0x0043_d950 + 0x64 * j;
                let this_owner = if (seed + j) % 4 == 0 { owner } else { r.next_u32() & 0x7fff_ffff };
                c.bus.write32(rec + 0x54, this_owner);
                c.bus.write8(0x0043_fa70 + j, (r.next_u32() % 200) as u8);
            }
        });
    }


    #[test]
    fn be_mission_int_get_track_collected_count_matches() {
        check_lohi(ssx3_ee::s01::f_00153520__generated, be_mission_int_get_track_collected_count, 20, &|c, seed| {
            let _r = base_2(c, seed, 0x153520, 3);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (seed % 8) as u64;
            c.g[A2][0] = (seed % 4) as u64;
            for i in 0..0x20000u32 {
                c.bus.write8(0x004a_6ca8 + i, seed.wrapping_add(i) as u8);
            }
        });
    }


    #[test]
    fn bemission_int_load_state_matches() {
        check(
            ee::s01::f_00154898__generated,
            bemission_int_load_state,
            60,
            &|c, seed| {
                let mut r = base_3(c, seed, 0x1548);
                let stream = DATA + 0x400;
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(stream);
                c.bus.write32(stream, DATA + 0x500);
                c.bus.write16(DATA + 0x500 + 0x10, 0);
                c.bus.write32(DATA + 0x500 + 0x14, FAKE);
                fill(c, &mut r, DATA + 0x8, 0x10);
            },
        );
    }


    #[test]
    fn bemissionint_count_completed_on_peak_matches() {
        strict(ee::s01::f_001543e0__generated, bemissionint_count_completed_on_peak, SEEDS_3, &|c, seed| {
            let _r = base_4(c, seed, 0x43e0, 0);
            c.g[A0][0] = sx(DATA + 0x100);
            // a non-zero sub-goal id: no mission lookup is taken
            c.g[A1][0] = sx(0x1234 + (seed as u32 & 1));
        });
    }
}
