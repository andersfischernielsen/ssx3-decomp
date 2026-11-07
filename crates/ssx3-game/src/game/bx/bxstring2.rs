//! `src/bx/bxstring2.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::abi::ret;
use crate::call;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::call::Unwound;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::ai::control::handplantcontrol::fr as fr_2;
pub(crate) use crate::game::ai::control::handplantcontrol::setf as setf_2;
pub(crate) use crate::game::be::bestate2::ea;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::lw as lw_3;
pub(crate) use crate::game::fe::feasyncfile::gp as gp_2;
pub(crate) use crate::game::fe::feasyncfile::gp as gp_3;
pub(crate) use crate::game::fe::feasyncfile::p_6 as p_3;
pub(crate) use crate::game::fe::feasyncfile::r;
pub(crate) use crate::game::fe::feasyncfile::sub;
pub(crate) use crate::game::fe::festateloadhint::lqsp;
pub(crate) use crate::game::fe::festateloadhint::p_3 as p_2;
pub(crate) use crate::game::fe::festateloadhint::sqsp;
pub(crate) use crate::game::fe::ovtemplatepausemenu::lw;
pub(crate) use crate::game::fe::ovtemplatepausemenu::sw;
pub(crate) use crate::game::render::particle::add32;
pub(crate) use crate::game::render::particle::copy_q;
pub(crate) use crate::game::render::particle::eq;
pub(crate) use crate::game::render::particle::fconst;
pub(crate) use crate::game::render::particle::fr;
pub(crate) use crate::game::render::particle::gp;
pub(crate) use crate::game::render::particle::ldf;
pub(crate) use crate::game::render::particle::le;
pub(crate) use crate::game::render::particle::leave;
pub(crate) use crate::game::render::particle::lt;
pub(crate) use crate::game::render::particle::p;
pub(crate) use crate::game::render::particle::setf;
pub(crate) use crate::game::render::particle::shl32;
pub(crate) use crate::game::render::particle::srl32;
pub(crate) use crate::game::render::particle::vadd;
pub(crate) use crate::game::render::particle::vscale;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::vu0::W;
use ssx3_ee_rt::vu0::X;
use ssx3_ee_rt::vu0::Y;
use ssx3_ee_rt::vu0::Z;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0026_8950, c_challenger_dtor, "cChallenger_dtor"),
    (
        0x0026_8a90,
        lobby_man_on_user_list_changed_thunk,
        "LobbyMan_OnUserListChanged_Thunk",
    ),
    (
        0x0026_8ab8,
        lobby_man_on_room_list_changed_thunk,
        "LobbyMan_OnRoomListChanged_Thunk",
    ),
    (
        0x0026_8ae0,
        lobby_man_on_rank_list_changed_thunk,
        "LobbyMan_OnRankListChanged_Thunk",
    ),
    (
        0x0026_8b08,
        lobby_man_on_recv_debug_thunk,
        "LobbyMan_OnRecvDebug_Thunk",
    ),
    (
        0x0026_8b30,
        lobby_man_on_message_thunk,
        "LobbyMan_OnMessage_Thunk",
    ),
    (
        0x0026_8b58,
        lobby_man_on_disconnect_thunk,
        "LobbyMan_OnDisconnect_Thunk",
    ),
    (
        0x0026_8b80,
        lobby_man_on_game_start_thunk,
        "LobbyMan_OnGameStart_Thunk",
    ),
    (
        0x0026_8ba8,
        lobby_man_on_connect_reply_thunk,
        "LobbyMan_OnConnectReply_Thunk",
    ),
    (
        0x0026_8bd0,
        lobby_man_on_create_account_reply_thunk,
        "LobbyMan_OnCreateAccountReply_Thunk",
    ),
    (
        0x0026_8bf8,
        lobby_man_on_edit_account_reply_thunk,
        "LobbyMan_OnEditAccountReply_Thunk",
    ),
    (
        0x0026_8c20,
        lobby_man_on_lost_by_email_reply_thunk,
        "LobbyMan_OnLostByEmailReply_Thunk",
    ),
    (
        0x0026_8c48,
        lobby_man_on_lost_by_name_reply_thunk,
        "LobbyMan_OnLostByNameReply_Thunk",
    ),
    (
        0x0026_8c70,
        lobby_man_on_login_reply_thunk,
        "LobbyMan_OnLoginReply_Thunk",
    ),
    (
        0x0026_8c98,
        lobby_man_on_select_persona_reply_thunk,
        "LobbyMan_OnSelectPersonaReply_Thunk",
    ),
    (
        0x0026_8cc0,
        lobby_man_on_own_stats_reply_thunk,
        "LobbyMan_OnOwnStatsReply_Thunk",
    ),
    (
        0x0026_8ce8,
        lobby_man_on_create_persona_reply_thunk,
        "LobbyMan_OnCreatePersonaReply_Thunk",
    ),
    (
        0x0026_8d10,
        lobby_man_on_delete_persona_reply_thunk,
        "LobbyMan_OnDeletePersonaReply_Thunk",
    ),
    (
        0x0026_8d38,
        lobby_man_on_move_to_room_reply_thunk,
        "LobbyMan_OnMoveToRoomReply_Thunk",
    ),
    (
        0x0026_8d60,
        lobby_man_on_create_room_reply_thunk,
        "LobbyMan_OnCreateRoomReply_Thunk",
    ),
    (
        0x0026_8d88,
        lobby_man_on_ping_reply_thunk,
        "LobbyMan_OnPingReply_Thunk",
    ),
    (
        0x0026_8db0,
        lobby_man_on_report_user_reply_thunk,
        "LobbyMan_OnReportUserReply_Thunk",
    ),
    (
        0x0026_8dd8,
        lobby_man_on_news_reply_thunk,
        "LobbyMan_OnNewsReply_Thunk",
    ),
    (
        0x0026_8e00,
        lobby_man_on_snapshot_reply_thunk,
        "LobbyMan_OnSnapshotReply_Thunk",
    ),
    (
        0x0026_8e28,
        lobby_man_on_lookup_user_reply_thunk,
        "LobbyMan_OnLookupUserReply_Thunk",
    ),
    (
        0x0026_8e78,
        lobby_man_on_quick_play_reply_thunk,
        "LobbyMan_OnQuickPlayReplyThunk",
    ),
    (
        0x0026_8ea0,
        lobby_man_on_chal_reply_thunk,
        "LobbyMan_OnChalReplyThunk",
    ),
    (
        0x0026_8ec8,
        lobby_man_on_mesg_reply_thunk,
        "LobbyMan_OnMesgReplyThunk",
    ),
    (
        0x0026_8ef0,
        lobby_man_on_rank_reply_thunk,
        "LobbyMan_OnRankReplyThunk",
    ),
    (
        0x0026_8f18,
        buddy_man_on_connect_event_thunk,
        "BuddyMan_OnConnectEventThunk",
    ),
    (
        0x0026_8f40,
        buddy_man_on_message_received_thunk,
        "BuddyMan_OnMessageReceivedThunk",
    ),
    (
        0x0026_8f68,
        buddy_man_on_roster_event_thunk,
        "BuddyMan_OnRosterEventThunk",
    ),
    (
        0x0026_8f90,
        buddy_man_on_roster_remove_reply_thunk,
        "BuddyMan_OnRosterRemoveReplyThunk",
    ),
    (
        0x0026_8fb8,
        buddy_man_on_roster_add_reply_thunk,
        "BuddyMan_OnRosterAddReplyThunk",
    ),
    (
        0x0026_8fe0,
        buddy_man_on_send_message_reply_thunk,
        "BuddyMan_OnSendMessageReplyThunk",
    ),
    (
        0x0026_9188,
        setup_man_client_recv_msg5,
        "cSetupMan_ClientRecvMsg5",
    ),
    (
        0x0026_93c0,
        setup_man_client_recv_msg4,
        "cSetupMan_ClientRecvMsg4",
    ),
    (
        0x0026_9538,
        csetup_man_host_recv_msg9,
        "cSetupMan_HostRecvMsg9",
    ),
    (
        0x0026_9608,
        csetupman_host_recv_msg11,
        "cSetupMan_HostRecvMsg11",
    ),
    (
        0x0026_9890,
        setup_man_client_recv_msg7,
        "cSetupMan_ClientRecvMsg7",
    ),
    (
        0x0026_9948,
        csetup_man_client_recv_msg8,
        "cSetupMan_ClientRecvMsg8",
    ),
    (
        0x0026_9a18,
        csetup_man_client_recv_msg9,
        "cSetupMan_ClientRecvMsg9",
    ),
    (
        0x0026_9ae0,
        csetup_man_client_recv_msg10,
        "cSetupMan_ClientRecvMsg10",
    ),
    (
        0x0026_9b90,
        csetup_man_client_recv_msg11,
        "cSetupMan_ClientRecvMsg11",
    ),
    (
        0x0026_9c40,
        csetup_man_client_recv_msg12,
        "cSetupMan_ClientRecvMsg12",
    ),
    (
        0x0026_9cf0,
        csetup_man_client_recv_msg13,
        "cSetupMan_ClientRecvMsg13",
    ),
    (0x0026_9dc0, net_event_fd_clone, "NetEventFD_Clone"),
    (0x0026_9e40, net_event_fe_clone, "NetEventFE_Clone"),
    (0x0026_9ed8, path_read_a_i_field, "Path_ReadAIField"),
    (
        0x0026_9f18,
        path_find_marker_boundary,
        "Path_FindMarkerBoundary",
    ),
    (0x0026_a070, path_read_track_field, "Path_ReadTrackField"),
    (
        0x0026_a090,
        path_query_events_reversed,
        "Path_QueryEventsReversed",
    ),
    (
        0x0026_a0b8,
        path_has_event_in_range_reversed,
        "Path_HasEventInRangeReversed",
    ),
    (0x0026_a338, path_read_segment, "Path_ReadSegment"),
    (0x0026_a378, path_read_event, "Path_ReadEvent"),
    (
        0x0026_a638,
        path_get_closest_point_from,
        "Path_GetClosestPointFrom",
    ),
    (
        0x0026_a8b8,
        path_get_dir_at_distance,
        "Path_GetDirAtDistance",
    ),
    (0x0026_a9b0, path_bounds_distance, "Path_BoundsDistance"),
    (0x0026_aa80, path_query_events, "Path_QueryEvents"),
    (
        0x0026_ab20,
        path_get_point_at_distance,
        "Path_GetPointAtDistance",
    ),
    (0x0026_ac88, path_has_event_in_range, "Path_HasEventInRange"),
    (0x0026_ad70, path_data_set_buffer, "PathData_SetBuffer"),
    (0x0026_adf0, path_data_parse, "PathData_Parse"),
    (0x0026_af00, path_data_free, "PathData_Free"),
    (
        0x0026_af70,
        path_data_get_track_path,
        "PathData_GetTrackPath",
    ),
    (0x0026_af88, path_data_get_ai_path, "PathData_GetAIPath"),
    (0x0026_a428, path_get_closest_point, "Path_GetClosestPoint"),
    (
        0x003d_0d70,
        path_set_track_control_value,
        "PATH_SetTrackControlValue",
    ),
    (
        0x003d_4b00,
        path_select_master_track,
        "PATH_SelectMasterTrack",
    ),
    (
        0x003d_5508,
        path_set_volume_percent,
        "PATH_SetVolumePercent",
    ),
    (
        0x0026_afb8,
        path_data_find_near_ai_paths,
        "PathData_FindNearAIPaths",
    ),
    (
        0x0026_b178,
        path_data_find_near_track_paths,
        "PathData_FindNearTrackPaths",
    ),
];

/// `Path` field: the base time the query window is shifted by.
pub(crate) mod path {

    pub const COUNT: u32 = 0x08;

    pub const START: u32 = 0x0c;

    pub const SEGMENTS: u32 = 0x18;

    pub const BASE_TIME: u32 = 0x38;
}

/// Path cursor (t0, in/out): point, distance along the path, segment (-1:
/// not on the path yet).
pub(crate) mod cursor {

    pub const DIST: u32 = 0x10;

    pub const SEGMENT: u32 = 0x14;
}

// ---------------------------------------------------------------------------
// Callees and fixed addresses.

pub(crate) mod callee {
    use super::ee;
    use super::Callee;

    pub(crate) use crate::game::be::beintnetwork::callee::PATH_QUERY_EVENTS;

    pub(crate) use crate::game::fe::feasyncfile::callee::MALLOC;

    pub(crate) use crate::game::sound::bankmonitor::callee::PATH_DATA_READ_FLOAT_2;
    pub(crate) use crate::game::sound::bankmonitor::callee::PATH_DATA_READ_INT;
    pub(crate) use crate::game::sound::bankmonitor::callee::PATH_DATA_READ_VEC3;

    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::PATH_DATA_READ_FLOAT;

    pub const FE_MODULE_RIDER_PREVIEW_REQUEST_CHAR: Callee = Callee {
        addr: 0x0019_e588,
        f: ee::s03::FEModule_RiderPreviewRequestChar,
    };

    pub const C_FE_STATE_LOBBY_MENU_FILL_PLAYER_LIST: Callee = Callee {
        addr: 0x001b_e540,
        f: ee::s03::cFEStateLobbyMenu_FillPlayerList,
    };

    pub const C_FE_STATE_LOBBY_MENU_UPDATE_PLAYER_SCROLL_ARROWS: Callee = Callee {
        addr: 0x001b_e720,
        f: ee::s03::cFEStateLobbyMenu_UpdatePlayerScrollArrows,
    };

    pub const EVENT_SELECT_ON_ACTIVATE: Callee = Callee {
        addr: 0x0020_0ac0,
        f: ee::s05::EventSelect_OnActivate,
    };

    pub const EVENT_SELECT_UPDATE: Callee = Callee {
        addr: 0x0020_2828,
        f: ee::s05::EventSelect_Update,
    };

    pub const C_OV_STATE_OVERLAY_UPDATE: Callee = Callee {
        addr: 0x0020_e900,
        f: ee::s05::cOVStateOverlay_Update,
    };

    pub const C_LUNO_TABLE_ALLOC: Callee = Callee {
        addr: 0x0022_4e50,
        f: ee::s05::cLunoTable_Alloc,
    };

    pub const CROWD_MAN2D_REMOVE_ALL: Callee = Callee {
        addr: 0x0022_9398,
        f: ee::s05::CrowdMan2d_RemoveAll,
    };

    pub const CROWD_MAN2D_RESET: Callee = Callee {
        addr: 0x0022_92e0,
        f: ee::s05::CrowdMan2d_Reset,
    };

    pub const SECTION_MAN_INIT: Callee = Callee {
        addr: 0x0022_cd40,
        f: ee::s05::SectionMan_Init,
    };

    pub const GAME_MODULE_REQUEST_STATE: Callee = Callee {
        addr: 0x0023_1250,
        f: ee::s06::GameModule_RequestState,
    };

    pub const VID_ENGINE_PLAYER_START_PLAYBACK: Callee = Callee {
        addr: 0x0025_36c8,
        f: ee::s06::VidEnginePlayer_StartPlayback,
    };

    pub const SCRIPT_LIST_PREPARE: Callee = Callee {
        addr: 0x0027_6270,
        f: ee::s07::ScriptList_Prepare,
    };

    pub const SCRIPT_LIST_SET_PAUSED_NESTED: Callee = Callee {
        addr: 0x0027_66b0,
        f: ee::s07::ScriptList_SetPausedNested,
    };

    pub const SCRIPT_LIST_GET_ENTRY_TYPE: Callee = Callee {
        addr: 0x0027_6998,
        f: ee::s07::ScriptList_GetEntryType,
    };

    pub const SCRIPT_ENGINE_STOP_MAIN_LISTS: Callee = Callee {
        addr: 0x0027_90f8,
        f: ee::s07::ScriptEngine_StopMainLists,
    };

    pub const SCRIPT_ENGINE_STOP_AUX_LISTS: Callee = Callee {
        addr: 0x0027_9488,
        f: ee::s07::ScriptEngine_StopAuxLists,
    };

    pub const SNAPSHOT_LIST_REMOVE: Callee = Callee {
        addr: 0x0026_e608,
        f: ee::s07::SnapshotList_Remove,
    };

    pub const PATH_DATA_PARSE: Callee = Callee {
        addr: 0x0026_adf0,
        f: ee::s07::PathData_Parse,
    };

    pub const WORLD_STATE_RESTORE: Callee = Callee {
        addr: 0x0026_dbd0,
        f: ee::s07::WorldState_Restore,
    };

    pub const EA_MEM_ALLOC_NAMED: Callee = Callee {
        addr: 0x0031_7d70,
        f: ee::s10::EA_MemAllocNamed,
    };

    pub const PATH_GET_LENGTH: Callee = Callee {
        addr: 0x0026_ac48,
        f: ee::s07::Path_GetLength,
    };

    pub const PATH_HAS_EVENT_IN_RANGE: Callee = Callee {
        addr: 0x0026_ac88,
        f: ee::s07::Path_HasEventInRange,
    };

    pub const SND_BANK_MAN_INIT_LOAD_REQUEST: Callee = Callee {
        addr: 0x0028_b930,
        f: ee::s07::SndBankMan_InitLoadRequest,
    };

    pub const POPUP_SET_WIDGET_ALIGN: Callee = Callee {
        addr: 0x001c_8930,
        f: ee::s03::f_001c8930,
    };

    pub const POPUP_SET_WRAPPED_TEXT: Callee = Callee {
        addr: 0x001c_97e0,
        f: ee::s03::f_001c97e0,
    };

    pub const WSCRIPT_MISSION_CTOR: Callee = Callee {
        addr: 0x0030_7128,
        f: ee::s10::f_00307128,
    };

    pub const WSCRIPT_PROCESS_LIST_PUSH_FRONT_2: Callee = Callee {
        addr: 0x0030_b2c0,
        f: ee::s10::f_0030b2c0,
    };

    pub const MULTIPARTICLE_MAN_CLEAR: Callee = Callee {
        addr: 0x0035_7b38,
        f: ee::s11::f_00357b38,
    };

    pub const MULTIPARTICLE_CTOR: Callee = Callee {
        addr: 0x0035_7e80,
        f: ee::s11::f_00357e80,
    };

    pub const OPTIONS_SET_VIBRATION: Callee = Callee {
        addr: 0x0018_d108,
        f: ee::s02::f_0018d108,
    };

    pub const SECTION_MAN_DEQUEUE_SECTION: Callee = Callee {
        addr: 0x0022_e360,
        f: ee::s05::f_0022e360,
    };

    pub const SECTION_MAN_POP_FREE_NODE: Callee = Callee {
        addr: 0x0022_e278,
        f: ee::s05::f_0022e278,
    };

    pub const NET_FE_STATE_GET_POPUP: Callee = Callee {
        addr: 0x001a_9c50,
        f: ee::s03::f_001a9c50,
    };

    pub const NET_FE_STATE_SHOW_POPUP: Callee = Callee {
        addr: 0x001a_9bc0,
        f: ee::s03::f_001a9bc0,
    };

    pub const MESSAGE_CENTER_GET_INBOX: Callee = Callee {
        addr: 0x001e_39f8,
        f: ee::s04::f_001e39f8,
    };

    pub const SSX_AUDIO_IS_DJ_SPEECH_ON: Callee = Callee {
        addr: 0x0029_f0d8,
        f: ee::s08::f_0029f0d8,
    };

    pub const SPEECH_SYSTEM_SET_EVENT_CONTEXT: Callee = Callee {
        addr: 0x002b_1458,
        f: ee::s08::f_002b1458,
    };

    pub const SSX_AUDIO_GET_LANGUAGE_MASK: Callee = Callee {
        addr: 0x0029_f198,
        f: ee::s08::f_0029f198,
    };

    pub const SSX_AUDIO_GET_SPEECH_CONTEXT_MASK: Callee = Callee {
        addr: 0x002a_1bd8,
        f: ee::s08::f_002a1bd8,
    };

    pub const INI_PARSER_SKIP_WHITESPACE: Callee = Callee {
        addr: 0x002a_78f8,
        f: ee::s08::f_002a78f8,
    };

    pub const CHUNK_RESOLVE_RELOCATE_DMA: Callee = Callee {
        addr: 0x003a_c358,
        f: ee::s13::f_003ac358,
    };

    pub const FLAG_MANAGER_GROUP_BUILD_MESH: Callee = Callee {
        addr: 0x0034_b228,
        f: ee::s11::f_0034b228,
    };

    pub const BE_GET_RIDER_CHARACTER: Callee = Callee {
        addr: 0x0014_a080,
        f: ee::s01::f_0014a080,
    };

    pub const BE_GET_RIDER_PROFILE: Callee = Callee {
        addr: 0x0014_a0e0,
        f: ee::s01::f_0014a0e0,
    };

    pub const BE_GET_CHAR_PROFILE_EVENT_RECORD: Callee = Callee {
        addr: 0x0015_2398,
        f: ee::s01::f_00152398,
    };

    pub const OV_STATE_OVERLAY_UPDATE: Callee = Callee {
        addr: 0x0020_e900,
        f: ee::s05::f_0020e900,
    };

    pub const READ_PRIVATE_MESSAGE_SET_TEXT: Callee = Callee {
        addr: 0x0021_0bd8,
        f: ee::s05::f_00210bd8,
    };

    pub const OV_SCREEN_PDA_TEMPLATE_CLOSE: Callee = Callee {
        addr: 0x0020_d190,
        f: ee::s05::f_0020d190,
    };

    pub const PATH_DATA_READ_FLOAT_3: Callee = Callee {
        addr: 0x0026_ba88,
        f: ee::s07::f_0026ba88,
    };

    pub const SUPERPIPE_COMPUTE_DESIRED_EYE: Callee = Callee {
        addr: 0x0017_85f0,
        f: ee::s02::f_001785f0,
    };

    pub const SUPERPIPE_COMPUTE_LOOK_AT: Callee = Callee {
        addr: 0x0017_8758,
        f: ee::s02::f_00178758,
    };

    pub const CHASE_CAMERA_RESET_FOR_RIDER: Callee = Callee {
        addr: 0x0016_6c60,
        f: ee::s02::f_00166c60,
    };

    pub const CHASE_CAMERA_INIT_FROM_RIDER: Callee = Callee {
        addr: 0x0016_6550,
        f: ee::s02::f_00166550,
    };
}

pub(crate) mod addr {

    /// `cBE_getBE` (cross-shard direct call the generator emitted as
    /// `call_indirect`).
    pub const CBE_GET_BE: u32 = 0x0014_dc80;

    /// `cBE_getInterface` (0x14DD58).
    pub const CBE_GET_INTERFACE: u32 = 0x0014_dd58;

    /// `cBEInterface::getNetwork`-ish lookup at 0x144BC0.
    pub const BE_INTERFACE_LOOKUP: u32 = 0x0014_4bc0;

    /// `BankMonitor_TimerKill` caller at 0x2ADDE0.
    pub const BANK_MONITOR_TIMER_RESET: u32 = 0x002a_dde0;

    /// `free` on a `SpeechBank` voice block.
    pub const SPEECH_BANK_FREE_CALLBACK: u32 = 0x003d_6ca0;

    /// `func_002523A8` (UI engine teardown).
    pub const UI_ENGINE_TEARDOWN: u32 = 0x0025_23a8;

    /// `sprintf`.
    pub const SPRINTF: u32 = 0x0041_7828;

    /// `fmodf`.
    pub const FMODF: u32 = 0x0040_d8d0;

    /// `cMemMan_alloc(size, vtable, flags, align)`.
    pub const MEM_MAN_ALLOC: u32 = 0x0031_7d70;

    /// `operator delete`.
    pub const MEM_MAN_FREE: u32 = 0x0031_7e50;

    /// `cMemMan_free`.
    pub const MEM_MAN_FREE_NOSIZE: u32 = 0x0031_7e98;

    /// The McManager query behind `cPS2MemCard_DoGetInfo`.
    pub const MC_MANAGER_GET_INFO: u32 = 0x0040_a498;
}

/// `float Path_BoundsDistance(const PathBounds *box, const Vec3 *p)`
/// (0x26a9b0): signed distance from `p` to the AABB whose X/Y/Z minima are at
/// box+0x1c/0x20/0x24 and maxima at box+0x28/0x2c/0x30. Returns the largest
/// per-axis penetration in f0 (with the scratch FPU registers the original
/// leaves behind).
pub fn path_bounds_distance(c: &mut Cpu) {
    const BOX_MIN_X: u32 = 0x1c;
    const BOX_MIN_Y: u32 = 0x20;
    const BOX_MIN_Z: u32 = 0x24;
    const BOX_MAX_X: u32 = 0x28;
    const BOX_MAX_Y: u32 = 0x2c;
    const BOX_MAX_Z: u32 = 0x30;

    let box_addr = Addr(abi::arg(c, 0)); // a0
    let p = Addr(abi::arg(c, 1)); // a1

    // X axis.
    c.cycles += 7;
    let mut f1 = p.read_fpu(c);
    let mut f2 = box_addr.off(BOX_MIN_X).read_fpu(c);
    let mut f3 = FpuF32(0); // mtc1 r0 -> f3
    let mut f4 = f3; // mov_s 4, 3
    let mut f0 = f3; // mov_s 0, 3
    if abi::c_lt_s(c, f1, f2) {
        c.cycles += 2;
        f3 = f2 - f1;
    } else {
        c.cycles += 5;
        f2 = box_addr.off(BOX_MAX_X).read_fpu(c);
        if abi::c_lt_s(c, f2, f1) {
            f3 = f1 - f2;
        }
    }

    // Y axis.
    c.cycles += 6;
    f1 = p.off(0x4).read_fpu(c);
    f2 = box_addr.off(BOX_MIN_Y).read_fpu(c);
    if abi::c_lt_s(c, f1, f2) {
        c.cycles += 2;
        f4 = f2 - f1;
    } else {
        f2 = box_addr.off(BOX_MAX_Y).read_fpu(c);
        c.cycles += 4;
        if abi::c_lt_s(c, f2, f1) {
            f4 = f1 - f2;
        }
    }

    // Z axis.
    c.cycles += 6;
    f1 = p.off(0x8).read_fpu(c);
    f2 = box_addr.off(BOX_MIN_Z).read_fpu(c);
    if abi::c_lt_s(c, f1, f2) {
        c.cycles += 2;
        f0 = f2 - f1;
    } else {
        f2 = box_addr.off(BOX_MAX_Z).read_fpu(c);
        c.cycles += 4;
        if abi::c_lt_s(c, f2, f1) {
            f0 = f1 - f2;
        }
    }

    // Combine the three penetration distances into f0.
    c.cycles += 3;
    if abi::c_lt_s(c, f4, f3) {
        c.cycles += 4;
        if !abi::c_lt_s(c, f3, f0) {
            f0 = f3;
        }
    } else {
        c.cycles += 3;
        if !abi::c_lt_s(c, f4, f0) {
            c.cycles += 2;
            f0 = f4;
            abi::set_fpr(c, 0, f0);
            abi::set_fpr(c, 1, f1);
            abi::ret(c);
            return;
        }
    }
    c.cycles += 2;
    abi::set_fpr(c, 0, f0);
    abi::set_fpr(c, 1, f1);
    abi::ret(c);
}

/// `PathTrack *PathData_GetTrackPath(PathData *self, int index)` (0x26af70):
/// `self->track_paths[index]`, records of 60 bytes after the pointer at +0x14.
pub fn path_data_get_track_path(c: &mut Cpu) {
    const TRACK_PATHS: u32 = 0x14;
    const TRACK_RECORD_SIZE: u32 = 0x3c;

    c.cycles += 5;
    let self_ = Addr(abi::arg(c, 0));
    c.g[V1][0] = TRACK_RECORD_SIZE as u64;
    let base = self_.off(TRACK_PATHS).lw(c);
    let scaled = abi::mult(c, abi::arg(c, 1), TRACK_RECORD_SIZE);
    c.g[A1][0] = sx(scaled);
    c.g[V0][0] = sx((base as u32).wrapping_add(c.g[A1][0] as u32));
    abi::ret(c);
}

/// `PathAI *PathData_GetAIPath(PathData *self, int index)` (0x26af88):
/// `self->ai_paths[index]`, records of 64 bytes after the pointer at +0xc.
pub fn path_data_get_ai_path(c: &mut Cpu) {
    const AI_PATHS: u32 = 0xc;

    c.cycles += 4;
    let self_ = Addr(abi::arg(c, 0));
    let base = self_.off(AI_PATHS).lw(c);
    c.g[A1][0] = sx(abi::arg(c, 1) << 6);
    c.g[V0][0] = sx((base as u32).wrapping_add(c.g[A1][0] as u32));
    abi::ret(c);
}

/// `lqc2 vf3, a; lqc2 vf4, b; vsub vf5, vf3, vf4; sqc2 vf5, out`.
#[inline(always)]
pub(crate) fn vsub(c: &mut Cpu, a: u32, b: u32, out: u32) {
    c.lqc2(3, a);
    c.lqc2(4, b);
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, out);
}

/// The inlined 4-component dot product: `lqc2 vf3, a; lqc2 vf5, b;
/// vaddw.x vf6, vf0, vf0w (1); vmul vf4, vf3, vf5; vaddy.x/vmaddz.x/
/// vmaddw.x` (x + y + z + w into vf4.x); `qmfc2 rt, vf4`. Returns the dot
/// (x lane) bits as left in `rt`.
#[inline(always)]
pub(crate) fn vdot4(c: &mut Cpu, a: u32, b: u32, rt: usize) -> FpuF32 {
    c.lqc2(3, a);
    c.lqc2(5, b);
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
    c.qmfc2(rt, 4);
    FpuF32(c.g[rt][0] as u32)
}

/// Head of the segment loop (event checks).
pub(crate) const PATH_LOOP: u32 = 0x0026_a6c8;

/// `float Path_GetClosestPointFrom(const Path* path, const Vec4* point,
/// Vec4* closest, float* dist, PathCursor* cursor /*t0*/, int flat /*t1*/)`
/// (0x26a638): walk the segments from the cursor (or from the start) and
/// keep the point closest to `point` (in XY when `flat`), updating the
/// cursor at each improvement; stop once the closest point is no longer
/// at the segment end (a cursor that was on the path) or past the search
/// window. Returns the distance along the path (f0) and stores the
/// distance to the point (`dist`, in XY for 3D searches).
///
/// State lives in the original's registers: t2 segment, f5 distance along
/// the path, f6 window end, f8 best squared distance (output), f9 best
/// squared distance (search), f10 result.
pub fn path_get_closest_point_from(c: &mut Cpu) {
    c.cycles += 8;
    let cur = abi::arg(c, 4);
    c.g[V1][0] = c.lw(p(cur, cursor::SEGMENT));
    c.g[SP][0] = sx(sp(c).wrapping_sub(0x60));
    setf(c, 10, FpuF32::ZERO);
    c.g[V0][0] = !c.g[V1][0];
    ldf(c, 9, gp(c, -0x4a54));
    c.g[T3][0] = srl32(c.g[V0][0], 31);
    setf(c, 8, FpuF32::ZERO);
    let sp = sp(c);
    let path_at = abi::arg(c, 0);
    if c.g[T3][0] == 0 {
        // Not on the path yet: start at its first point, no window.
        c.cycles += 14;
        ldf(c, 2, p(path_at, path::START));
        c.g[T2][0] = 0;
        ldf(c, 1, p(path_at, path::START + 4));
        setf(c, 5, FpuF32::ZERO);
        ldf(c, 0, p(path_at, path::START + 8));
        c.mov_s(6, 9);
        fconst(c, 3, 0x3f80_0000);
        c.swc1(2, p(sp, 0x10));
        c.swc1(1, p(sp, 0x14));
        c.swc1(0, p(sp, 0x18));
        c.swc1(3, p(sp, 0x1c));
        copy_q(c, V0, p(sp, 0x10), sp);
    } else {
        // From the cursor; the window ends a fixed distance further on.
        c.cycles += 7;
        let dist = ldf(c, 5, p(cur, cursor::DIST));
        c.g[T2][0] = c.g[V1][0];
        c.lq(V0, cur);
        let window = ldf(c, 0, gp(c, -0x4a50));
        c.sq(V0, sp);
        setf(c, 6, dist + window);
    }
    c.cycles += 4;
    c.g[V0][0] = c.lw(p(path_at, path::COUNT));
    c.g[V0][0] = ((c.g[T2][0] as i64) < (c.g[V0][0] as i64)) as u64;
    if c.g[V0][0] != 0 {
        c.cycles += 3;
        setf(c, 7, FpuF32::ZERO);
        c.g[V1][0] = c.lw(p(path_at, path::SEGMENTS));
        while path_segment(c, sp) {
            if event_due(c) {
                deliver_event_here(c, PATH_LOOP);
            }
        }
    }
    c.cycles += 5;
    let best = fr(c, 8).sqrt();
    setf(c, 1, best);
    c.mov_s(0, 10);
    c.swc1(1, abi::arg(c, 3));
    leave(c, 0x60);
}

/// One segment of [`path_get_closest_point_from`]: segment `t2` of the
/// array in v1, starting at the point at `sp+0`. True when the walk goes
/// on (t2 advanced, v1 reloaded).
pub(crate) fn path_segment(c: &mut Cpu, sp: u32) -> bool {
    let path_at = abi::arg(c, 0);
    let point = abi::arg(c, 1);
    let flat = c.g[T1][0] != 0;
    // dir = segment.xyz (w = 0), rel = point - start, t = dot(rel, dir).
    c.cycles += 17;
    c.g[V0][0] = shl32(c.g[T2][0], 4);
    c.g[V0][0] = add32(c.g[V0][0], c.g[V1][0]);
    let seg = r32(c, V0);
    ldf(c, 0, seg);
    c.swc1(0, p(sp, 0x10));
    ldf(c, 1, p(seg, 4));
    c.swc1(1, p(sp, 0x14));
    ldf(c, 0, p(seg, 8));
    c.sw(p(sp, 0x1c), 0);
    c.swc1(0, p(sp, 0x18));
    let len = ldf(c, 4, p(seg, 0xc));
    vsub(c, point, sp, p(sp, 0x30));
    copy_q(c, V0, p(sp, 0x30), p(sp, 0x20));
    let along = if !flat {
        c.cycles += 9;
        vdot4(c, p(sp, 0x20), p(sp, 0x10), V0);
        c.mtc1(V0, 2);
        fr(c, 2)
    } else {
        c.cycles += 8;
        let dy = ldf(c, 2, p(sp, 0x14));
        let rx = ldf(c, 1, p(sp, 0x20));
        let dx = ldf(c, 3, p(sp, 0x10));
        let ry = ldf(c, 0, p(sp, 0x24));
        let px = rx * dx;
        setf(c, 1, px);
        let py = ry * dy;
        setf(c, 0, py);
        let v = px + py;
        setf(c, 2, v);
        v
    };
    // t = clamp(along, 0, len)
    c.cycles += 4;
    let zero = fr(c, 7);
    let pos = le(c, zero, along);
    setf(c, 3, zero);
    if pos {
        c.cycles += 1;
        setf(c, 3, along.min(len));
    }
    let t = fr(c, 3);
    // candidate = start + dir * t (sp+0x30); rel = point - candidate.
    c.cycles += 21;
    c.mfc1(V1, 3);
    vscale(c, p(sp, 0x10), V1, p(sp, 0x50));
    copy_q(c, V0, p(sp, 0x50), p(sp, 0x40));
    vadd(c, sp, p(sp, 0x40), p(sp, 0x50));
    copy_q(c, V0, p(sp, 0x50), p(sp, 0x30));
    vsub(c, point, p(sp, 0x30), p(sp, 0x50));
    c.lq(V0, p(sp, 0x50));
    c.sq(V0, p(sp, 0x20));
    c.sq(V0, p(sp, 0x40));
    let d2 = if !flat {
        c.cycles += 9;
        vdot4(c, p(sp, 0x20), p(sp, 0x20), V0);
        c.mtc1(V0, 1);
        fr(c, 1)
    } else {
        c.cycles += 6;
        let rx = ldf(c, 1, p(sp, 0x20));
        let ry = ldf(c, 0, p(sp, 0x24));
        let ry2 = ry * ry;
        setf(c, 0, ry2);
        let v = rx * rx + ry2;
        setf(c, 1, v);
        v
    };
    c.cycles += 3;
    if lt(c, d2, fr(c, 9)) {
        // A closer point: remember it and move the cursor here.
        c.cycles += 2;
        setf(c, 9, d2);
        if !flat {
            c.cycles += 5;
            let rx = ldf(c, 1, p(sp, 0x20));
            let ry = ldf(c, 0, p(sp, 0x24));
            let rx2 = rx * rx;
            setf(c, 1, rx2);
            let ry2 = ry * ry;
            setf(c, 0, ry2);
            setf(c, 8, rx2 + ry2);
        } else {
            c.cycles += 2;
            setf(c, 8, d2);
        }
        c.cycles += 8;
        let out = abi::arg(c, 2);
        let cur = abi::arg(c, 4);
        c.lq(V1, p(sp, 0x30));
        let along_path = fr(c, 5);
        setf(c, 10, along_path + t);
        c.lq(V0, sp);
        c.sq(V1, out);
        c.sq(V0, cur);
        c.swc1(5, p(cur, cursor::DIST));
        let seg_index = c.g[T2][0];
        c.sw(p(cur, cursor::SEGMENT), seg_index);
        if c.g[T3][0] != 0 {
            c.cycles += 3;
            if eq(c, t, along) {
                return false;
            }
        }
    }
    // Past the window: stop.
    c.cycles += 3;
    if lt(c, fr(c, 6), fr(c, 5)) {
        return false;
    }
    // Next segment: start += dir * len, distance += len.
    c.cycles += 17;
    c.mfc1(V0, 4);
    vscale(c, p(sp, 0x10), V0, p(sp, 0x50));
    c.g[T2][0] = add32(c.g[T2][0], 1);
    c.lq(V0, p(sp, 0x50));
    let along_path = fr(c, 5) + len;
    setf(c, 5, along_path);
    c.sq(V0, p(sp, 0x40));
    vadd(c, sp, p(sp, 0x40), sp);
    c.g[V0][0] = c.lw(p(path_at, path::COUNT));
    c.g[V0][0] = ((c.g[T2][0] as i64) < (c.g[V0][0] as i64)) as u64;
    if c.g[V0][0] != 0 {
        c.g[V1][0] = c.lw(p(path_at, path::SEGMENTS));
        return true;
    }
    false
}

// ---------------------------------------------------------------------------
// Path_QueryEvents.

/// `int Path_QueryEvents(const PathRangeList* list, PathRange* out, int max,
/// float lo, float hi)` (0x26aa80): copies the ranges that overlap
/// `[lo, hi]` into `out` (16 bytes each), stopping at `max`, and returns how
/// many were copied. A range list is `{int count; Range* items}` and a range
/// `{u32 a, b; float min, max}`. Nothing is copied when `hi < lo`.
pub fn path_query_events(c: &mut Cpu) {
    c.cycles += 4;
    c.c_lt_s(13, 12); // hi < lo
    if c.fcc() {
        c.cycles += 2;
        c.g[V0][0] = 0;
        abi::ret(c);
        return;
    }
    let list = r32(c, A0);
    c.g[V0][0] = c.lw(list); // count
    c.cycles += 3;
    c.g[T0][0] = 0; // i
    let positive = (c.g[V0][0] as i64) > 0;
    c.g[A3][0] = 0; // copied
    if positive {
        c.cycles += 1;
        c.g[V0][0] = c.lw(list.wrapping_add(4)); // items
        'walk: loop {
            'skip: {
                c.cycles += 7;
                let item = r32(c, V0).wrapping_add(r32(c, T0) << 4);
                c.g[V1][0] = sx(item);
                c.lwc1(0, item.wrapping_add(8)); // range.min
                c.c_lt_s(13, 0); // hi < min
                if c.fcc() {
                    c.g[V0][0] = c.lw(list);
                    break 'skip;
                }
                c.cycles += 5;
                c.lwc1(0, item.wrapping_add(0xc)); // range.max
                c.c_lt_s(0, 12); // max < lo
                if c.fcc() {
                    c.g[V0][0] = c.lw(list);
                    break 'skip;
                }
                c.cycles += 11;
                // `*out++ = *item` (the original's ldl/ldr + sdl/sdr pair).
                let dst = r32(c, A1);
                c.ldl(2, item.wrapping_add(7));
                c.ldr(2, item);
                c.ldl(9, item.wrapping_add(0xf));
                c.ldr(9, item.wrapping_add(8));
                c.sdl(2, dst.wrapping_add(7));
                c.sdr(2, dst);
                c.sdl(9, dst.wrapping_add(0xf));
                c.g[A3][0] = sx(r32(c, A3).wrapping_add(1));
                c.sdr(9, dst.wrapping_add(8));
                let full = c.g[A3][0] == c.g[A2][0];
                c.g[A1][0] = sx(dst.wrapping_add(0x10));
                if full {
                    break 'walk;
                }
                c.cycles += 1;
                c.g[V0][0] = c.lw(list);
            }
            c.cycles += 4;
            c.g[T0][0] = sx(r32(c, T0).wrapping_add(1));
            let more = (c.g[T0][0] as i64) < (c.g[V0][0] as i64);
            c.g[V0][0] = more as u64;
            if !more {
                break;
            }
            c.g[V0][0] = c.lw(list.wrapping_add(4));
            if event_due(c) {
                deliver_event_here(c, 0x0026_aaa8);
            }
        }
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[A3][0];
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Path_GetPointAtDistance.

/// `Vec4 Path_GetPointAtDistance(const PathTrack* track, bool* in_range,
/// float t)` (0x26ab20): the point `t` along the track, written through the
/// hidden struct-return pointer in `a0`. The track has its start point at
/// +0xc (x, y, z), its segment count at +8 and its segments at +0x18; a
/// segment is `{dir x, dir y, dir z, len}`. `*in_range` is 1 while `t` is on
/// the track and 0 once it runs past the end (when it is not null).
pub fn path_get_point_at_distance(c: &mut Cpu) {
    c.cycles += 12;
    let track = r32(c, A1);
    let frame = Frame::open(c, 0x40);
    let sp = sp(c);
    // The start point, w = 1.
    c.lwc1(2, track.wrapping_add(0xc));
    c.g[T0][0] = c.g[A0][0];
    c.lwc1(1, track.wrapping_add(0x10));
    c.lwc1(0, track.wrapping_add(0x14));
    c.g[AT][0] = 0x3f80_0000;
    c.mtc1(1, 3);
    c.swc1(2, sp);
    c.swc1(1, sp.wrapping_add(4));
    c.swc1(0, sp.wrapping_add(8));
    let no_range = c.g[A2][0] == 0;
    c.swc1(3, sp.wrapping_add(0xc));
    if !no_range {
        c.cycles += 2;
        c.g[V0][0] = 1;
        c.sw(r32(c, A2), 1);
    }
    c.cycles += 2;
    c.g[A3][0] = 0; // segment index
    c.g[V0][0] = c.lw(track.wrapping_add(0x18)); // segments
    loop {
        // The segment, or the end of the track.
        c.cycles += 4;
        c.g[V1][0] = sx(r32(c, A3) << 4);
        c.g[A0][0] = c.lw(track.wrapping_add(8)); // segment count
        let past_end = c.g[A3][0] == c.g[A0][0];
        let seg = r32(c, V0).wrapping_add(r32(c, V1));
        c.g[V0][0] = sx(seg);
        if past_end {
            c.cycles += 2;
            if c.g[A2][0] != 0 {
                c.sw(r32(c, A2), 0);
            }
            c.cycles += 2;
            c.lq(2, sp); // the point built so far
            break;
        }
        // `t < seg.len`: the point lies on this segment.
        c.cycles += 5;
        c.lwc1(2, seg.wrapping_add(0xc));
        c.c_lt_s(12, 2);
        if !c.fcc() {
            // Advance to the segment's end: `pos += dir * len`.
            c.cycles += 22;
            c.lwc1(1, seg);
            c.sub_s(12, 12, 2);
            c.g[A3][0] = sx(r32(c, A3).wrapping_add(1));
            c.swc1(1, sp.wrapping_add(0x20));
            c.lwc1(0, seg.wrapping_add(4));
            c.swc1(0, sp.wrapping_add(0x24));
            c.lwc1(1, seg.wrapping_add(8));
            c.sw(sp.wrapping_add(0x2c), 0);
            c.swc1(1, sp.wrapping_add(0x28));
            c.mfc1(3, 2);
            c.lqc2(4, sp.wrapping_add(0x20));
            c.qmtc2(3, 3);
            c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
            c.sqc2(5, sp.wrapping_add(0x30));
            c.lq(2, sp.wrapping_add(0x30));
            c.sq(2, sp.wrapping_add(0x10));
            c.lqc2(3, sp);
            c.lqc2(4, sp.wrapping_add(0x10));
            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, sp);
            c.g[V0][0] = c.lw(track.wrapping_add(0x18));
            if event_due(c) {
                deliver_event_here(c, 0x0026_ab60);
            }
            continue;
        }
        // The point on this segment: `pos += dir * t`.
        c.cycles += 18;
        c.lwc1(0, seg);
        c.swc1(0, sp.wrapping_add(0x20));
        c.lwc1(1, seg.wrapping_add(4));
        c.swc1(1, sp.wrapping_add(0x24));
        c.lwc1(0, seg.wrapping_add(8));
        c.sw(sp.wrapping_add(0x2c), 0);
        c.swc1(0, sp.wrapping_add(0x28));
        c.mfc1(3, 12);
        c.lqc2(4, sp.wrapping_add(0x20));
        c.qmtc2(3, 3);
        c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
        c.sqc2(5, sp.wrapping_add(0x30));
        c.lq(2, sp.wrapping_add(0x30));
        c.sq(2, sp.wrapping_add(0x10));
        c.lqc2(3, sp);
        c.lqc2(4, sp.wrapping_add(0x10));
        c.vu0().add(0b1111, 5, 3, Src::Vec(4));
        c.sqc2(5, sp.wrapping_add(0x30));
        c.lq(2, sp.wrapping_add(0x30));
        break;
    }
    c.cycles += 4;
    c.sq(2, r32(c, T0));
    c.g[V0][0] = c.g[T0][0];
    frame.close(c);
}

// ---------------------------------------------------------------------------
// Path events.

/// `int Path_QueryEventsReversed(Path *path, void *out, int maxOut,
/// float a, float b)` (0x26a090): query the events whose time lies in
/// `[base - a, base - b]`.
pub fn path_query_events_reversed(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let path = Addr(abi::arg(c, 0));
        c.lwc1(0, path.off(path::BASE_TIME).0);
        let base = fr_2(c, 0);
        // The window is measured backwards from the path's base time.
        let upper = fr_2(c, 13);
        setf_2(c, 13, base - upper);
        let lower = fr_2(c, 12);
        setf_2(c, 12, base - lower);
        callee::PATH_QUERY_EVENTS.call(c, 0x0026_a0a8)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Copies of a field, conditional on the arguments.

/// `void Path_ReadTrackField(Path *self, void *src, int already_read)`
/// (0x0026a070): read a float into `self + 0x38` unless a value was supplied.
pub fn path_read_track_field(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 3;
        let frame = Frame::open(c, 0x10);
        let supplied = c.g[A2][0] != 0;
        frame.save_ra(c, 0);
        if !supplied {
            c.cycles += 2;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x38));
            call_ptr(c, callee::PATH_DATA_READ_FLOAT, 0x0026_a084)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// LobbyMan_On*_Thunk.

/// Base of the lobby callback table (`li v0, 0x4a0000` in every thunk): the
/// per-thunk `slot` is the offset the original adds (`0x2fb0`, `0x2fb8`, ...),
/// so the entry lives at `0x4a0000 + slot` (`D_004A2FB0`-ish).
pub(crate) const LOBBY_CALLBACKS: u32 = 0x004a_0000;

/// The shared shape of the seventeen lobby callbacks: add the (16-bit) lobby
/// object offset published in the callback table to the handler's third
/// argument (`a2`, the lobby object) and tail into the matching `On*` handler.
pub(crate) fn lobby_thunk(c: &mut Cpu, ret: u32, slot: u32, target: u32) -> Flow {
    c.cycles += 9;
    c.g[V0][0] = 0x004a_0000;
    let frame = Frame::open(c, 0x10);
    c.g[A0][0] = Addr(LOBBY_CALLBACKS.wrapping_add(slot)).lh(c);
    frame.save_ra(c, 0);
    c.g[A0][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
    call_ptr(c, target, ret)?;
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `void LobbyMan_OnUserListChanged_Thunk(void* m, void* h, void* obj)`
/// (0x268a90).
pub fn lobby_man_on_user_list_changed_thunk(c: &mut Cpu) {
    guest(c, |c| lobby_thunk(c, 0x0026_8aa8, 0x2fb0, 0x0025_d6c0))
}

/// `void LobbyMan_OnRoomListChanged_Thunk(void* m, void* h, void* obj)`
/// (0x268ab8).
pub fn lobby_man_on_room_list_changed_thunk(c: &mut Cpu) {
    guest(c, |c| lobby_thunk(c, 0x0026_8ad0, 0x2fb8, 0x0025_99b0))
}

/// `void LobbyMan_OnRankListChanged_Thunk(void* m, void* h, void* obj)`
/// (0x268ae0).
pub fn lobby_man_on_rank_list_changed_thunk(c: &mut Cpu) {
    guest(c, |c| lobby_thunk(c, 0x0026_8af8, 0x2fc0, 0x0025_9a60))
}

/// `void LobbyMan_OnRecvDebug_Thunk(void* m, void* h, void* obj)` (0x268b08).
pub fn lobby_man_on_recv_debug_thunk(c: &mut Cpu) {
    guest(c, |c| lobby_thunk(c, 0x0026_8b20, 0x2ff0, 0x0025_98a8))
}

/// `void LobbyMan_OnMessage_Thunk(void* m, void* h, void* obj)` (0x268b30).
pub fn lobby_man_on_message_thunk(c: &mut Cpu) {
    guest(c, |c| lobby_thunk(c, 0x0026_8b48, 0x2ff8, 0x0025_9b10))
}

/// `void LobbyMan_OnDisconnect_Thunk(void* m, void* h, void* obj)` (0x268b58).
pub fn lobby_man_on_disconnect_thunk(c: &mut Cpu) {
    guest(c, |c| lobby_thunk(c, 0x0026_8b70, 0x3000, 0x0025_98c8))
}

/// `void LobbyMan_OnGameStart_Thunk(void* m, void* h, void* obj)` (0x268b80).
pub fn lobby_man_on_game_start_thunk(c: &mut Cpu) {
    guest(c, |c| lobby_thunk(c, 0x0026_8b98, 0x3008, 0x0025_a1d0))
}

/// `void LobbyMan_OnConnectReply_Thunk(void* m, void* h, void* obj)`
/// (0x268ba8).
pub fn lobby_man_on_connect_reply_thunk(c: &mut Cpu) {
    guest(c, |c| lobby_thunk(c, 0x0026_8bc0, 0x3020, 0x0025_b848))
}

/// `void LobbyMan_OnCreateAccountReply_Thunk(void* m, void* h, void* obj)`
/// (0x268bd0).
pub fn lobby_man_on_create_account_reply_thunk(c: &mut Cpu) {
    guest(c, |c| lobby_thunk(c, 0x0026_8be8, 0x3088, 0x0025_bd30))
}

/// `void LobbyMan_OnEditAccountReply_Thunk(void* m, void* h, void* obj)`
/// (0x268bf8).
pub fn lobby_man_on_edit_account_reply_thunk(c: &mut Cpu) {
    guest(c, |c| lobby_thunk(c, 0x0026_8c10, 0x30a8, 0x0025_c0c0))
}

/// `void LobbyMan_OnLostByEmailReply_Thunk(void* m, void* h, void* obj)`
/// (0x268c20).
pub fn lobby_man_on_lost_by_email_reply_thunk(c: &mut Cpu) {
    guest(c, |c| lobby_thunk(c, 0x0026_8c38, 0x30b0, 0x0025_c2d8))
}

/// `void LobbyMan_OnLostByNameReply_Thunk(void* m, void* h, void* obj)`
/// (0x268c48).
pub fn lobby_man_on_lost_by_name_reply_thunk(c: &mut Cpu) {
    guest(c, |c| lobby_thunk(c, 0x0026_8c60, 0x30b8, 0x0025_c4f0))
}

/// `void LobbyMan_OnLoginReply_Thunk(void* m, void* h, void* obj)` (0x268c70).
pub fn lobby_man_on_login_reply_thunk(c: &mut Cpu) {
    guest(c, |c| lobby_thunk(c, 0x0026_8c88, 0x30e8, 0x0025_c8a8))
}

/// `void LobbyMan_OnSelectPersonaReply_Thunk(void* m, void* h, void* obj)`
/// (0x268c98).
pub fn lobby_man_on_select_persona_reply_thunk(c: &mut Cpu) {
    guest(c, |c| lobby_thunk(c, 0x0026_8cb0, 0x3118, 0x0025_ceb0))
}

/// `void LobbyMan_OnOwnStatsReply_Thunk(void* m, void* h, void* obj)`
/// (0x268cc0).
pub fn lobby_man_on_own_stats_reply_thunk(c: &mut Cpu) {
    guest(c, |c| lobby_thunk(c, 0x0026_8cd8, 0x3120, 0x0025_d048))
}

/// `void LobbyMan_OnCreatePersonaReply_Thunk(void* m, void* h, void* obj)`
/// (0x268ce8).
pub fn lobby_man_on_create_persona_reply_thunk(c: &mut Cpu) {
    guest(c, |c| lobby_thunk(c, 0x0026_8d00, 0x3138, 0x0025_d2d8))
}

/// `void LobbyMan_OnDeletePersonaReply_Thunk(void* m, void* h, void* obj)`
/// (0x268d10).
pub fn lobby_man_on_delete_persona_reply_thunk(c: &mut Cpu) {
    guest(c, |c| lobby_thunk(c, 0x0026_8d28, 0x3140, 0x0025_d538))
}

// ---------------------------------------------------------------------------
// Message-map thunks.

/// Base of the message maps: the thunk's `lui v0, 0x4a` and the `int16`
/// member offsets live at `0x4a3148..` (`D_004A3148` and friends).
pub(crate) const MESSAGE_MAP: u32 = 0x004a_0000;

/// The shared body of the reply-callback thunks:
/// `handler(a2 + message_map[entry], a1, ...)`.
///
/// `entry` is the absolute address of the message's `int16` member offset,
/// `target` the handler and `ret` the return address the original materialises
/// in `ra` before the tail call (it keeps the `0x10` frame so the handler's own
/// frame starts where the original's does).
pub(crate) fn message_thunk(c: &mut Cpu, entry: u32, target: u32, ret: u32) -> Flow {
    c.cycles += 9;
    let frame = Frame::open(c, 0x10);
    // The original materialises the map base in v0, then loads the offset.
    c.g[V0][0] = MESSAGE_MAP as u64;
    let this = (c.g[A2][0] as u32).wrapping_add(Addr(entry).lh(c) as u32);
    frame.save_ra(c, 0);
    c.g[A0][0] = sx(this);
    call_ptr(c, target, ret)?;
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `void LobbyMan_OnMoveToRoomReply_Thunk(LobbyMan *self, void *msg, void *obj)`
/// (0x268d38): `LobbyMan_OnMoveToRoomReply(obj + D_004A3148, msg)`.
pub fn lobby_man_on_move_to_room_reply_thunk(c: &mut Cpu) {
    guest(c, |c| {
        message_thunk(c, 0x004a_3148, 0x0025_d8f8, 0x0026_8d50)
    })
}

/// `void LobbyMan_OnCreateRoomReply_Thunk(...)` (0x268d60):
/// `LobbyMan_OnCreateRoomReply(obj + D_004A3158, msg)`.
pub fn lobby_man_on_create_room_reply_thunk(c: &mut Cpu) {
    guest(c, |c| {
        message_thunk(c, 0x004a_3158, 0x0025_db80, 0x0026_8d78)
    })
}

/// `void LobbyMan_OnPingReply_Thunk(...)` (0x268d88):
/// `LobbyMan_OnPingReply(obj + D_004A3160, msg)`.
pub fn lobby_man_on_ping_reply_thunk(c: &mut Cpu) {
    guest(c, |c| {
        message_thunk(c, 0x004a_3160, 0x0025_dde0, 0x0026_8da0)
    })
}

/// `void LobbyMan_OnReportUserReply_Thunk(...)` (0x268db0):
/// `LobbyMan_OnReportUserReply(obj + D_004A3170, msg)`.
pub fn lobby_man_on_report_user_reply_thunk(c: &mut Cpu) {
    guest(c, |c| {
        message_thunk(c, 0x004a_3170, 0x0025_df98, 0x0026_8dc8)
    })
}

/// `void LobbyMan_OnNewsReply_Thunk(...)` (0x268dd8):
/// `LobbyMan_OnNewsReply(obj + D_004A3178, msg)`.
pub fn lobby_man_on_news_reply_thunk(c: &mut Cpu) {
    guest(c, |c| {
        message_thunk(c, 0x004a_3178, 0x0025_e138, 0x0026_8df0)
    })
}

/// `void LobbyMan_OnSnapshotReply_Thunk(...)` (0x268e00):
/// `LobbyMan_OnSnapshotReply(obj + D_004A31B0, msg)`.
pub fn lobby_man_on_snapshot_reply_thunk(c: &mut Cpu) {
    guest(c, |c| {
        message_thunk(c, 0x004a_31b0, 0x0025_e590, 0x0026_8e18)
    })
}

/// `void LobbyMan_OnLookupUserReply_Thunk(...)` (0x268e28):
/// `LobbyMan_OnLookupUserReply(obj + D_004A31B8, msg)`.
pub fn lobby_man_on_lookup_user_reply_thunk(c: &mut Cpu) {
    guest(c, |c| {
        message_thunk(c, 0x004a_31b8, 0x0025_e7f0, 0x0026_8e40)
    })
}

/// `void LobbyMan_OnQuickPlayReplyThunk(...)` (0x268e78):
/// `LobbyMan_OnQuickPlayReply(obj + D_004A3210, msg)`.
pub fn lobby_man_on_quick_play_reply_thunk(c: &mut Cpu) {
    guest(c, |c| {
        message_thunk(c, 0x004a_3210, 0x0025_f030, 0x0026_8e90)
    })
}

/// `void LobbyMan_OnChalReplyThunk(...)` (0x268ea0):
/// `LobbyMan_OnChalReply(obj + D_004A3248, msg)`.
pub fn lobby_man_on_chal_reply_thunk(c: &mut Cpu) {
    guest(c, |c| {
        message_thunk(c, 0x004a_3248, 0x0025_f8a0, 0x0026_8eb8)
    })
}

/// `void LobbyMan_OnMesgReplyThunk(...)` (0x268ec8):
/// `LobbyMan_OnMesgReply(obj + D_004A3268, msg)`.
pub fn lobby_man_on_mesg_reply_thunk(c: &mut Cpu) {
    guest(c, |c| {
        message_thunk(c, 0x004a_3268, 0x0025_fc20, 0x0026_8ee0)
    })
}

/// `void LobbyMan_OnRankReplyThunk(...)` (0x268ef0):
/// `LobbyMan_OnRankReply(obj + D_004A3318, msg)`.
pub fn lobby_man_on_rank_reply_thunk(c: &mut Cpu) {
    guest(c, |c| {
        message_thunk(c, 0x004a_3318, 0x0026_0c68, 0x0026_8f08)
    })
}

/// `void BuddyMan_OnConnectEventThunk(...)` (0x268f18):
/// `BuddyMan_OnConnectEvent(obj + D_004A3340, msg)`.
pub fn buddy_man_on_connect_event_thunk(c: &mut Cpu) {
    guest(c, |c| {
        message_thunk(c, 0x004a_3340, 0x0026_15c8, 0x0026_8f30)
    })
}

/// `void BuddyMan_OnMessageReceivedThunk(...)` (0x268f40):
/// `BuddyMan_OnMessageReceived(obj + D_004A3348, msg)`.
pub fn buddy_man_on_message_received_thunk(c: &mut Cpu) {
    guest(c, |c| {
        message_thunk(c, 0x004a_3348, 0x0026_1a28, 0x0026_8f58)
    })
}

/// `void BuddyMan_OnRosterEventThunk(...)` (0x268f68):
/// `BuddyMan_OnRosterEvent(obj + D_004A3350, msg)`.
pub fn buddy_man_on_roster_event_thunk(c: &mut Cpu) {
    guest(c, |c| {
        message_thunk(c, 0x004a_3350, 0x0026_1770, 0x0026_8f80)
    })
}

/// `void BuddyMan_OnRosterRemoveReplyThunk(...)` (0x268f90):
/// `BuddyMan_OnRosterRemoveReply(obj + D_004A3380, msg)`.
pub fn buddy_man_on_roster_remove_reply_thunk(c: &mut Cpu) {
    guest(c, |c| {
        message_thunk(c, 0x004a_3380, 0x0026_1970, 0x0026_8fa8)
    })
}

/// `void BuddyMan_OnRosterAddReplyThunk(...)` (0x268fb8):
/// `BuddyMan_OnRosterAddReply(obj + D_004A3388, msg)`.
pub fn buddy_man_on_roster_add_reply_thunk(c: &mut Cpu) {
    guest(c, |c| {
        message_thunk(c, 0x004a_3388, 0x0026_18c0, 0x0026_8fd0)
    })
}

/// `void BuddyMan_OnSendMessageReplyThunk(...)` (0x268fe0):
/// `BuddyMan_OnSendMessageReply(obj + D_004A33A0, msg)`.
pub fn buddy_man_on_send_message_reply_thunk(c: &mut Cpu) {
    guest(c, |c| {
        message_thunk(c, 0x004a_33a0, 0x0026_1a20, 0x0026_8ff8)
    })
}

// ---------------------------------------------------------------------------
// Path data, playback and the replay snapshot list.

/// `PathData_SetBuffer(self, v)` 0x26ad70: remember the path blob and re-parse
/// it; always reports success.
pub fn path_data_set_buffer(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x10);
        let this64 = abi::arg64(c, 0);
        c.g[V0][0] = this64;
        let this = this64 as u32;
        frame.save_ra(c, 0);
        c.g[V1][0] = 1;
        let blob = c.g[A1][0];
        c.sw(this.wrapping_add(0x48), blob);
        c.sw(this.wrapping_add(0x4c), 1);
        callee::PATH_DATA_PARSE.call(c, 0x0026_ad8c)?;
        frame.restore_ra(c, 0);
        c.g[V0][0] = 1;
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Path readers.

/// `void Path_ReadAIField(PathData* dst, u32 unused, int id /*a2*/)`
/// (0x269ed8): read the int at `+0x38` for id 0x64 and at `+0x3c` for id 0x65.
pub fn path_read_a_i_field(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let dst = abi::arg(c, 0);
        let id = abi::arg64(c, 2);
        if id == 0x64 {
            c.g[V0][0] = 0x64;
            c.cycles += 4;
            c.g[A0][0] = sx(p(dst, 0x38));
            callee::PATH_DATA_READ_INT.call(c, 0x0026_9ef0)?;
            frame.restore_ra(c, 0);
        } else {
            c.cycles += 3;
            c.g[V0][0] = 0x65;
            let other = id != 0x65;
            frame.restore_ra(c, 0);
            if !other {
                c.cycles += 3;
                c.g[A0][0] = sx(p(dst, 0x3c));
                callee::PATH_DATA_READ_INT.call(c, 0x0026_9f0c)?;
                frame.restore_ra(c, 0);
            }
        }
        c.cycles += 2;
        leave(c, 0x10);
        Ok(())
    })
}

/// `void Path_ReadSegment(u32 unused, const void* src /*a1*/, void* dst
/// /*a2*/)` (0x26a338): copy a segment's three floats and its length from
/// `src` to `dst`.
pub fn path_read_segment(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A2][0];
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S0][0];
        callee::PATH_DATA_READ_VEC3.call(c, 0x0026_a358)?;
        c.g[A0][0] = sx(p(r32(c, S0), 0xc));
        c.g[A1][0] = sx(p(r32(c, S1), 0xc));
        callee::PATH_DATA_READ_FLOAT_2.call(c, 0x0026_a364)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Path events.

/// `int Path_HasEventInRangeReversed(Path* path, float a, float b)` (0x26a0b8):
/// the reversed range query — `lo = a < 0 ? 0 : path->range_end - a`,
/// `hi = b < 0 ? Path_GetLength(path) : path->range_end - b`, then
/// [`path_has_event_in_range`] on `[lo, hi]`.
pub fn path_has_event_in_range_reversed(c: &mut Cpu) {
    guest(c, |c| {
        const RANGE_END: u32 = 0x38;
        c.cycles += 8;
        c.mtc1(0, 0); // f0 = 0.0
        let sp = r32(c, SP).wrapping_sub(0x30);
        c.g[SP][0] = sx(sp);
        c.sq(16, sp.wrapping_add(0x10));
        c.c_le_s(0, 12); // 0.0 <= a
        Addr(sp).sd(c, c.g[RA][0]);
        c.swc1(20, sp.wrapping_add(0x20));
        let upper = !c.fcc();
        c.g[S0][0] = c.g[A0][0];
        if upper {
            // a < 0: the low end clamps to zero.
            c.cycles += 1;
            c.mov_s(20, 0);
        } else {
            c.cycles += 3;
            c.lwc1(0, r32(c, S0).wrapping_add(RANGE_END));
            c.sub_s(20, 0, 12); // f20 = range_end - a
        }
        // Higher end.
        c.cycles += 5;
        c.mtc1(0, 0); // f0 = 0.0
        c.c_lt_s(13, 0);
        if !c.fcc() {
            // b >= 0: the high end is range_end - b.
            c.lwc1(0, r32(c, S0).wrapping_add(RANGE_END));
            c.cycles += 2;
            c.sub_s(0, 0, 13);
            c.g[A0][0] = c.g[S0][0];
        } else {
            // b < 0: the high end is the path length.
            c.cycles += 4;
            c.g[A0][0] = c.g[S0][0];
            callee::PATH_GET_LENGTH.call(c, 0x0026_a104)?;
            c.g[A0][0] = c.g[S0][0];
        }
        c.cycles += 8;
        c.mov_s(12, 20); // low
        c.mov_s(13, 0); // high
        callee::PATH_HAS_EVENT_IN_RANGE.call(c, 0x0026_a120)?;
        c.lq(16, sp.wrapping_add(0x10));
        c.g[RA][0] = Addr(sp).ld(c);
        c.lwc1(20, sp.wrapping_add(0x20));
        c.g[SP][0] = sx(sp.wrapping_add(0x30));
        abi::ret(c);
        Ok(())
    })
}

/// `int Path_HasEventInRange(Path* path, float lo, float hi)` (0x26ac88):
/// 1 if any segment of the path overlaps `[lo, hi]` and is a scripted event
/// (type 0x12 start / 0x14 end), else 0. `path->count` at `+0x0`, the item
/// array at `+0x4` (0x10 bytes each: type at `+0x0`, start at `+0x8`, end at
/// `+0xc`).
pub fn path_has_event_in_range(c: &mut Cpu) {
    const EVENT_END: u32 = 0x14;
    const EVENT_START: u32 = 0x12;
    c.cycles += 3;
    c.g[V0][0] = Addr(r32(c, A0)).lw(c);
    c.g[V1][0] = 0;
    if (c.g[V0][0] as i64) <= 0 {
        c.cycles += 2;
        c.g[V0][0] = 0;
        abi::ret(c);
        return;
    }
    c.cycles += 5;
    c.g[A0][0] = Addr(r32(c, A0).wrapping_add(0x4)).lw(c);
    c.g[A1][0] = c.g[V0][0];
    c.g[A3][0] = EVENT_END as u64;
    c.g[A2][0] = EVENT_START as u64;
    loop {
        let item = r32(c, A0);
        'counted: {
            c.cycles += 5;
            c.lwc1(0, item.wrapping_add(0x8));
            c.c_lt_s(13, 0); // hi < start
            if c.fcc() {
                c.g[V1][0] = sx(r32(c, V1).wrapping_add(1));
                break 'counted;
            }
            c.cycles += 5;
            c.lwc1(0, item.wrapping_add(0xc));
            c.c_lt_s(0, 12); // end < lo
            if c.fcc() {
                c.g[V1][0] = sx(r32(c, V1).wrapping_add(1));
                break 'counted;
            }
            c.cycles += 3;
            c.g[V0][0] = Addr(item).lw(c);
            let event = c.g[V0][0] == c.g[A3][0];
            if !event {
                c.cycles += 2;
                let not_start = c.g[V0][0] != c.g[A2][0];
                c.g[V1][0] = sx(r32(c, V1).wrapping_add(1));
                if not_start {
                    break 'counted;
                }
            }
            // A start or end marker inside the range.
            c.cycles += 2;
            c.g[V0][0] = 0x1;
            abi::ret(c);
            return;
        }
        c.cycles += 3;
        c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A1][0] as i64)) as u64;
        let more = c.g[V0][0] != 0;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x10));
        if !more {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0026_aca8);
        }
    }
    c.cycles += 2;
    c.g[V0][0] = 0;
    abi::ret(c);
}

/// `sNode* NetEventFD_Clone(sNode* self)` (0x269DC0): allocate a 12-byte
/// `0xFD` node carrying `self->value`.
pub fn net_event_fd_clone(c: &mut Cpu) {
    net_event_clone(c, 0xfd, 0x13e0, 0x0026_9de8);
}

/// `sNode* NetEventFE_Clone(sNode* self)` (0x269E40): the `0xFE` variant.
pub fn net_event_fe_clone(c: &mut Cpu) {
    net_event_clone(c, 0xfe, 0x13c0, 0x0026_9e68);
}

/// The shared body of the two `NetEvent*_Clone` nodes: `vtable_off` is the
/// node vtable's offset from 0x480000.
pub(crate) fn net_event_clone(c: &mut Cpu, kind: u64, vtable_off: u32, ret: u32) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0x480000;
        frame.save(c, S0, 0x10);
        c.g[A1][0] = sx(ea(c.g[A1][0], 0x488)); // the allocation's name
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = 0xc;
        c.g[A2][0] = 0x2000_0000;
        c.g[A3][0] = 0;
        call_ptr(c, addr::MEM_MAN_ALLOC, ret)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = c.lw(ea(c.g[S0][0], 8));
        c.g[V0][0] = kind;
        c.g[V1][0] = 0x480000;
        c.g[V1][0] = sx(ea(c.g[V1][0], vtable_off));
        c.sw(c.g[A0][0] as u32, c.g[V0][0]);
        frame.restore(c, S0, 0x10);
        c.g[V0][0] = c.g[A0][0];
        frame.restore_ra(c, 0);
        c.sw(ea(c.g[A0][0], 4), c.g[V1][0]);
        c.sw(ea(c.g[A0][0], 8), c.g[A1][0]);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// cBXString, cChallenger.

/// `cChallenger_dtor(cChallenger* self, int flags)` (0x268950): install the
/// challenger vtable, destroy the two embedded strings, then delete the
/// object when the low flag bit is set.
pub fn c_challenger_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x30);
        let this = c.g[A0][0];
        let flags = c.g[A1][0];
        c.g[V0][0] = 0x0048_0000;
        frame.save(c, S0, 0x20);
        c.g[V0][0] = 0x0048_10e0;
        frame.save(c, S1, 0x10);
        c.g[S0][0] = flags;
        frame.save_ra(c, 0);
        c.g[S1][0] = this;
        sw(c, p(this as u32, 0x3c), 0x0048_10e0);
        c.g[A0][0] = sx((this as u32).wrapping_add(4));
        c.g[A1][0] = 2;
        call_ptr(c, 0x0031_8260, 0x0026_8980)?;
        c.g[A0][0] = this;
        c.g[A1][0] = 2;
        call_ptr(c, 0x0031_8260, 0x0026_898c)?;
        c.g[S0][0] = c.g[S0][0] & 1;
        let keep = c.g[S0][0] == 0;
        frame.restore(c, S0, 0x20);
        if !keep {
            c.cycles += 3;
            c.g[A0][0] = this;
            call_ptr(c, 0x0031_7e50, 0x0026_89a0)?;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `sVec4_12F398 Path_GetDirAtDistance(Path* self, float t)` (0x26a8b8):
/// walk the path segments (16 bytes each, `fC` at +0xc) subtracting each
/// segment's length until `t` falls inside one; return that segment's
/// direction (`f0`, `f4`, `f8`) with `fC` zeroed.
pub fn path_get_dir_at_distance(c: &mut Cpu) {
    c.cycles += 2;
    let out = c.g[A0][0];
    let path = c.g[A1][0];
    c.g[A2][0] = out;
    c.g[A3][0] = 0;
    let mut i: u32 = 0;
    loop {
        c.cycles += 6;
        let segs = lw(c, p(path as u32, 0x18)) as u32;
        c.g[V1][0] = sx(i << 4);
        c.g[V0][0] = sx((lw(c, p(path as u32, 8)) as u32).wrapping_sub(1));
        let last = c.g[A3][0] == c.g[V0][0];
        c.g[A0][0] = sx(segs.wrapping_add(i << 4));
        let e = c.g[A0][0] as u32;
        let mut selected = last;
        if !last {
            c.cycles += 5;
            c.lwc1(0, p(e, 0xc));
            c.c_le_s(12, 0);
            if !c.fcc() {
                c.sub_s(12, 12, 0);
            } else {
                selected = true;
            }
        }
        if selected {
            c.cycles += 8;
            c.lwc1(0, e);
            c.swc1(0, out as u32);
            c.lwc1(1, p(e, 4));
            c.swc1(1, p(out as u32, 4));
            c.lwc1(0, p(e, 8));
            sw(c, p(out as u32, 0xc), 0);
            c.swc1(0, p(out as u32, 8));
            break;
        }
        c.cycles += 2;
        i = i.wrapping_add(1);
        c.g[A3][0] = sx(i);
        if event_due(c) {
            deliver_event_here(c, 0x0026_a8c0);
        }
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[A2][0];
    abi::ret(c);
}

/// `void PathData_Free(PathData* d)` (0x26af00): free the three owned
/// buffers and clear their pointers.
pub fn path_data_free(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        sqsp(c, 16, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = Addr(p_2(r32(c, S0), 0xc)).lw(c);
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            call::call_ptr(c, 0x0031_7e98, 0x0026_af24)?;
            c.g[A0][0] = Addr(p_2(r32(c, S0), 0x14)).lw(c);
        } else {
            c.g[A0][0] = Addr(p_2(r32(c, S0), 0x14)).lw(c);
        }
        c.cycles += 2;
        let clear = c.g[A0][0] == 0;
        Addr(p_2(r32(c, S0), 0xc)).sw(c, 0);
        if !clear {
            c.cycles += 2;
            call::call_ptr(c, 0x0031_7e98, 0x0026_af38)?;
        }
        c.cycles += 3;
        c.g[A0][0] = Addr(p_2(r32(c, S0), 0x48)).lw(c);
        let head_empty = c.g[A0][0] == 0;
        Addr(p_2(r32(c, S0), 0x14)).sw(c, 0);
        let mut tail_clear = false;
        if !head_empty {
            c.cycles += 3;
            c.g[V0][0] = Addr(p_2(r32(c, S0), 0x4c)).lw(c);
            if c.g[V0][0] == 0 {
                Addr(p_2(r32(c, S0), 0x4c)).sw(c, 0);
            } else {
                c.cycles += 2;
                call::call_ptr(c, 0x0031_7e98, 0x0026_af58)?;
                tail_clear = true;
            }
        } else {
            tail_clear = true;
        }
        if tail_clear {
            c.cycles += 1;
            Addr(p_2(r32(c, S0), 0x4c)).sw(c, 0);
        }
        c.cycles += 5;
        Addr(p_2(r32(c, S0), 0x48)).sw(c, 0);
        lqsp(c, 16, 0x10);
        frame.restore_ra(c, 0);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = ra;
        Ok(())
    });
}

/// `void PathData_Parse(PathData* p)` (0x0026adf0): read the path header and
/// each of the four sections in turn, advancing the cursor by each section's
/// returned size.
pub fn path_data_parse(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 32;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x40));
        c.sq(S1, sp(c).wrapping_add(0x20));
        c.sd(sp(c).wrapping_add(0x10), c.g[RA][0]);
        c.g[S1][0] = c.g[A0][0];
        c.sq(S0, sp(c).wrapping_add(0x30));
        c.g[A0][0] = c.g[SP][0];
        c.g[S0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x48));
        c.sw((c.g[S1][0] as u32).wrapping_add(8), 0);
        c.sw((c.g[S1][0] as u32).wrapping_add(0x10), 0);
        c.g[A1][0] = c.g[S0][0];
        c.sw(c.g[S1][0] as u32, 0);
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(4));
        call_ptr(c, 0x0026_ba68, 0x0026_ae24)?; // PathData_ReadInt
        c.g[A1][0] = c.g[S0][0];
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0026_b410, 0x0026_ae30)?; // PathData_ReadAIPaths
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[S0][0];
        call_ptr(c, 0x0026_b310, 0x0026_ae40)?; // PathData_ReadTrackPaths
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[S0][0];
        call_ptr(c, 0x0026_b7d8, 0x0026_ae50)?; // PathData_ReadFloatIntPairs
        c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x0026_b880, 0x0026_ae5c)?; // PathData_ReadMarkers
        c.lq(S0, sp(c).wrapping_add(0x30));
        c.lq(S1, sp(c).wrapping_add(0x20));
        c.g[RA][0] = c.ld(sp(c).wrapping_add(0x10));
        c.g[SP][0] = sx(sp(c).wrapping_add(0x40));
        abi::ret(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `cSetupMan_ClientRecvMsg11` (0x269b90): store the message id in the caller's
// word, allocate a 12-byte message node, fill kind/vtable/extra, allocate and
// link a list node, and splice it in after the head stored at `gp-0xc8`+0xf0.
// The 0xFD/0xFE constructors differ only in those constants and the extra
// word (`cSetupMan_ClientRecvMsg10/12/13` are the same shape).
pub(crate) fn recv_msg_node(
    c: &mut Cpu,
    kind: u64,
    vt: u32,
    extra: u64,
    ret1: u32,
    ret2: u32,
) -> crate::call::Flow {
    c.sw(c.g[A0][0] as u32, c.g[A1][0]);
    c.g[A1][0] = 0x480000;
    c.g[A0][0] = 0xc;
    c.g[A2][0] = 0x20000000;
    c.g[A3][0] = 0u64;
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xf18));
    call_ptr(c, 0x0031_7d70, ret1)?;
    c.g[S0][0] = c.g[V0][0];
    c.g[V1][0] = kind;
    c.sw(c.g[S0][0] as u32, c.g[V1][0]);
    c.g[V0][0] = 0x480000;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(vt));
    c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
    c.sw((c.g[S0][0] as u32).wrapping_add(0x8), extra);
    c.g[A1][0] = 0x480000;
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4a8));
    c.g[A0][0] = 0xc;
    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
    c.g[A2][0] = 0x20000000;
    c.g[A3][0] = 0u64;
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xf0));
    c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[V0][0]);
    call_ptr(c, 0x0031_7d70, ret2)?;
    c.g[A1][0] = c.g[V0][0];
    c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x8));
    if c.g[V0][0] != 0 {
        c.sw((c.g[A1][0] as u32).wrapping_add(0x8), c.g[S0][0]);
    }
    // Splice the node after the head.
    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
    c.sw(c.g[A1][0] as u32, c.g[V0][0]);
    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
    c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[V1][0]);
    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
    c.sw(c.g[A0][0] as u32, c.g[A1][0]);
    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
    c.sw((c.g[V0][0] as u32).wrapping_add(0x4), c.g[A1][0]);
    c.sw(c.g[SP][0] as u32, c.g[A1][0]);
    Ok(())
}

/// `cSetupMan_ClientRecvMsg11` (0x269b90): queue message kind 0xFD.
pub fn csetup_man_client_recv_msg11(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 30;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save_ra(c, 0x20);
        recv_msg_node(c, 0xfd, 0x13e0, 0, 0x0026_9bb8, 0x0026_9bf8)?;
        c.cycles += 13;
        c.lq(16, frame.at(c, 0x30));
        frame.close(c);
        Ok(())
    })
}

/// `cSetupMan_ClientRecvMsg13` (0x269cf0): queue message kind 0xFE.
pub fn csetup_man_client_recv_msg13(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 30;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save_ra(c, 0x20);
        recv_msg_node(c, 0xfe, 0x13c0, 0, 0x0026_9d18, 0x0026_9d58)?;
        c.cycles += 13;
        c.lq(16, frame.at(c, 0x30));
        frame.close(c);
        Ok(())
    })
}

/// `cSetupMan_ClientRecvMsg10` (0x269ae0): queue message kind 0xFD with the
/// extra word 1.
pub fn csetup_man_client_recv_msg10(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 31;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save_ra(c, 0x20);
        recv_msg_node(c, 0xfd, 0x13e0, 1, 0x0026_9b08, 0x0026_9b4c)?;
        c.cycles += 13;
        c.lq(16, frame.at(c, 0x30));
        frame.close(c);
        Ok(())
    })
}

/// `cSetupMan_ClientRecvMsg12` (0x269c40): queue message kind 0xFE with the
/// extra word 1.
pub fn csetup_man_client_recv_msg12(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 31;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save_ra(c, 0x20);
        recv_msg_node(c, 0xfe, 0x13c0, 1, 0x0026_9c68, 0x0026_9cac)?;
        c.cycles += 13;
        c.lq(16, frame.at(c, 0x30));
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `Path_ReadEvent` (0x26a378): read an event key, look it up in the fixed
// 21-entry key table into `dst`, then read the event's delta and two floats.
pub fn path_read_event(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        c.g[A0][0] = c.g[SP][0];
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A1][0];
        c.g[S1][0] = c.g[A2][0];
        frame.save_ra(c, 0x10);
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
        callee::PATH_DATA_READ_INT.call(c, 0x0026_a39c)?;
        c.sw(c.g[S1][0] as u32, 0u64);
        c.g[V0][0] = 0x440000;
        c.g[A2][0] = c.g[V0][0];
        c.g[A0][0] = c.lw(c.g[SP][0] as u32);
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x53d0));
        let first_differs = c.g[V1][0] != c.g[A0][0];
        c.g[A1][0] = 0u64;
        if !first_differs {
            c.cycles += 2;
        } else {
            c.cycles += 1;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
            loop {
                c.cycles += 3;
                c.g[V0][0] = ((c.g[A1][0] as i64) < 21i64) as u64;
                let out_of_range = c.g[V0][0] == 0;
                c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_add(0x53d0));
                if out_of_range {
                    break;
                }
                c.cycles += 6;
                c.g[V0][0] = sx((c.g[A1][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.g[A0][0] = c.lw(c.g[SP][0] as u32);
                c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                if c.g[V1][0] != c.g[A0][0] {
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1));
                    if event_due(c) {
                        deliver_event_here(c, 0x0026_a3c4);
                    }
                    continue;
                }
                c.cycles += 1;
                c.sw(c.g[S1][0] as u32, c.g[A1][0]);
                break;
            }
            c.cycles += 1;
        }
        c.g[A1][0] = c.g[S0][0];
        c.cycles += 14;
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
        callee::PATH_DATA_READ_INT.call(c, 0x0026_a3f8)?;
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x8));
        c.g[A1][0] = c.g[S0][0];
        callee::PATH_DATA_READ_FLOAT_3.call(c, 0x0026_a408)?;
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0xc));
        c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4));
        callee::PATH_DATA_READ_FLOAT_3.call(c, 0x0026_a414)?;
        c.lq(16, frame.at(c, 0x30));
        c.lq(17, frame.at(c, 0x20));
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// The `addiu sp; jr ra` tail of a frame whose `ra` was reloaded from
/// `off(sp)` by the body.
#[inline(always)]
pub(crate) fn frame_less_return(c: &mut Cpu, size: u32) {
    let ra = r(c, RA);
    c.g[SP][0] = sx(r(c, SP).wrapping_add(size));
    c.pc = ra;
}

// ---------------------------------------------------------------------------
// cSetupMan message handlers.

/// The shared body of `cSetupMan_ClientRecvMsg4`/`Msg5` (0x2693c0/0x269188):
/// allocate a small network message holding the received payload word, chain
/// it onto the client's queue and store the queue head.
pub(crate) fn setup_man_client_recv_msg_alloc(c: &mut Cpu, ret1: u32, ret2: u32) -> Flow {
    c.cycles += 31;
    let frame = Frame::open(c, 0x60);
    let a0 = r(c, A0);
    let a1 = r(c, A1);
    c.g[A2][0] = 0x2000_0000;
    frame.save(c, S0, 0x50);
    frame.save(c, S1, 0x40);
    c.g[S0][0] = 0xf7;
    frame.save(c, S2, 0x30);
    c.g[A3][0] = 0;
    frame.save_ra(c, 0x20);
    c.sw(a0, a1 as u64);
    c.g[A1][0] = sx(0x0048_0488);
    c.g[S1][0] = sx(Addr(gp_2(c, -0xc8)).lw(c) as u32);
    c.g[RA][0] = sx(ret1);
    c.g[A0][0] = 0x8;
    call_ptr(c, callee::MALLOC, ret1)?;
    c.g[S2][0] = c.g[V0][0];
    c.g[A1][0] = 0x0048_0000;
    c.g[V0][0] = 0x0048_0000;
    c.sw(r(c, S2), c.g[S0][0]);
    c.g[V0][0] = sx(0x0048_12b0);
    c.g[A1][0] = sx(0x0048_04a8);
    c.sw(p(r(c, S2), 4), c.g[V0][0]);
    c.g[A0][0] = 0xc;
    c.g[A2][0] = 0x2000_0000;
    c.g[A3][0] = 0;
    c.g[V0][0] = Addr(p(r(c, S1), 0xf0)).lw(c);
    c.g[RA][0] = sx(ret2);
    c.sw(p(r(c, SP), 0x10), c.g[V0][0]);
    call_ptr(c, callee::MALLOC, ret2)?;
    c.g[A1][0] = c.g[V0][0];
    c.g[V0][0] = sx(r(c, A1).wrapping_add(8));
    if c.g[V0][0] != 0 {
        c.sw(p(r(c, A1), 8), c.g[S2][0]);
    }
    c.cycles += 15;
    c.g[V0][0] = Addr(p(r(c, SP), 0x10)).lw(c);
    frame.restore(c, 16, 0x50);
    c.sw(r(c, A1), c.g[V0][0]);
    frame.restore(c, 17, 0x40);
    c.g[V1][0] = Addr(p(r(c, V0), 4)).lw(c);
    frame.restore(c, 18, 0x30);
    c.sw(p(r(c, A1), 4), c.g[V1][0]);
    c.g[RA][0] = c.ld(p(r(c, SP), 0x20));
    c.g[A0][0] = Addr(p(r(c, V0), 4)).lw(c);
    c.sw(r(c, A0), c.g[A1][0]);
    c.g[V0][0] = Addr(p(r(c, SP), 0x10)).lw(c);
    c.sw(p(r(c, V0), 4), c.g[A1][0]);
    c.sw(r(c, SP), c.g[A1][0]);
    frame_less_return(c, 0x60);
    Ok(())
}

/// `void cSetupMan_ClientRecvMsg5(void* q, int payload)` (0x269188).
pub fn setup_man_client_recv_msg5(c: &mut Cpu) {
    guest(c, |c| {
        setup_man_client_recv_msg_alloc(c, 0x0026_91c0, 0x0026_91f4)
    })
}

/// `void cSetupMan_ClientRecvMsg4(void* q, int payload)` (0x2693c0).
pub fn setup_man_client_recv_msg4(c: &mut Cpu) {
    guest(c, |c| {
        setup_man_client_recv_msg_alloc(c, 0x0026_93f8, 0x0026_942c)
    })
}

/// `void cSetupMan_ClientRecvMsg7(void* q, int payload)` (0x269890): build a
/// 0x10-byte host message with the payload at `+0xc` and queue it.
pub fn setup_man_client_recv_msg7(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 32;
        let frame = Frame::open(c, 0x50);
        c.g[A0][0] = 0x10;
        frame.save(c, S0, 0x40);
        c.g[A2][0] = 0x2000_0000;
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S1, 0x30);
        c.g[A1][0] = sx(0x0048_0f18);
        frame.save_ra(c, 0x20);
        c.g[A3][0] = 0;
        call_ptr(c, callee::MALLOC, 0x0026_98bc)?;
        c.g[V1][0] = 0x0048_0000;
        c.g[S1][0] = c.g[V0][0];
        c.g[V1][0] = sx(0x0048_1150);
        c.g[V0][0] = 0xfb;
        c.sw(p(r(c, S1), 4), c.g[V1][0]);
        c.g[A1][0] = sx(0x0048_04a8);
        c.sw(r(c, S1), c.g[V0][0]);
        c.sw(p(r(c, S1), 0xc), c.g[S0][0]);
        c.g[A0][0] = 0xc;
        c.sw(p(r(c, S1), 8), 0);
        c.g[A2][0] = 0x2000_0000;
        c.g[V1][0] = Addr(gp_2(c, -0xc8)).lw(c);
        c.g[A3][0] = 0;
        c.g[V0][0] = Addr(p(r(c, V1), 0xf0)).lw(c);
        c.g[RA][0] = sx(0x0026_9900);
        c.sw(p(r(c, SP), 0x10), c.g[V0][0]);
        call_ptr(c, callee::MALLOC, 0x0026_9900)?;
        c.g[A1][0] = c.g[V0][0];
        c.g[V0][0] = sx(r(c, A1).wrapping_add(8));
        if c.g[V0][0] != 0 {
            c.sw(p(r(c, A1), 8), c.g[S1][0]);
        }
        c.cycles += 14;
        c.g[V0][0] = Addr(p(r(c, SP), 0x10)).lw(c);
        frame.restore(c, 16, 0x40);
        c.sw(r(c, A1), c.g[V0][0]);
        frame.restore(c, 17, 0x30);
        c.g[V1][0] = Addr(p(r(c, V0), 4)).lw(c);
        c.g[RA][0] = c.ld(p(r(c, SP), 0x20));
        c.sw(p(r(c, A1), 4), c.g[V1][0]);
        c.g[A0][0] = Addr(p(r(c, V0), 4)).lw(c);
        c.sw(r(c, A0), c.g[A1][0]);
        c.g[V0][0] = Addr(p(r(c, SP), 0x10)).lw(c);
        c.sw(p(r(c, V0), 4), c.g[A1][0]);
        c.sw(r(c, SP), c.g[A1][0]);
        frame_less_return(c, 0x50);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Setup-manager message helpers.

/// `void cSetupMan_ClientRecvMsg9(cSetupMan *self, u32 a1, u32 a2)`
/// (0x269a18): allocate a 0x14-byte message node (kind 0xFC, vtable
/// `0x481130`, the two arguments at +0xC/+0x10), then a 0xC-byte list node
/// and splice it after the head stored at `*(gp_3-0xC8)+0xF0`.
pub fn csetup_man_client_recv_msg9(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 35;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x50);
        c.g[A0][0] = 0x14;
        frame.save(c, S1, 0x40);
        c.g[S0][0] = c.g[A1][0];
        c.g[S1][0] = c.g[A2][0];
        c.g[A1][0] = 0x0048_0f18;
        frame.save(c, S2, 0x30);
        c.g[A2][0] = 0x2000_0000;
        frame.save_ra(c, 0x20);
        c.g[A3][0] = 0;
        call_ptr(c, 0x0031_7d70, 0x0026_9a4c)?;
        c.g[S2][0] = c.g[V0][0];
        c.g[V1][0] = 0x0048_1130;
        c.g[V0][0] = 0xfc;
        Addr(p_3(c.g[S2][0], 0)).sw(c, c.g[V0][0]);
        c.g[A1][0] = 0x0048_04a8;
        Addr(p_3(c.g[S2][0], 0x4)).sw(c, c.g[V1][0]);
        Addr(p_3(c.g[S2][0], 0xc)).sw(c, c.g[S0][0]);
        c.g[A0][0] = 0xc;
        Addr(p_3(c.g[S2][0], 0x10)).sw(c, c.g[S1][0]);
        c.g[A2][0] = 0x2000_0000;
        Addr(p_3(c.g[S2][0], 0x8)).sw(c, 0);
        c.g[A3][0] = 0;
        c.g[V0][0] = Addr(gp_3(c, -0xc8)).lw(c);
        c.g[V1][0] = Addr(p_3(c.g[V0][0], 0xf0)).lw(c);
        Addr(frame.at(c, 0x10)).sw(c, c.g[V1][0]);
        call_ptr(c, 0x0031_7d70, 0x0026_9a94)?;
        c.g[A1][0] = c.g[V0][0];
        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x8));
        if c.g[V0][0] != 0 {
            Addr(p_3(c.g[A1][0], 0x8)).sw(c, c.g[S2][0]);
        }
        // Splice the list node after the head.
        c.cycles += 15;
        c.g[V0][0] = Addr(frame.at(c, 0x10)).lw(c);
        frame.restore(c, S0, 0x50);
        Addr(c.g[A1][0] as u32).sw(c, c.g[V0][0]);
        frame.restore(c, S1, 0x40);
        c.g[V1][0] = Addr(p_3(c.g[V0][0], 0x4)).lw(c);
        frame.restore(c, S2, 0x30);
        Addr(p_3(c.g[A1][0], 0x4)).sw(c, c.g[V1][0]);
        frame.restore_ra(c, 0x20);
        c.g[A0][0] = Addr(p_3(c.g[V0][0], 0x4)).lw(c);
        Addr(c.g[A0][0] as u32).sw(c, c.g[A1][0]);
        c.g[V0][0] = Addr(frame.at(c, 0x10)).lw(c);
        Addr(p_3(c.g[V0][0], 0x4)).sw(c, c.g[A1][0]);
        Addr(frame.at(c, 0)).sw(c, c.g[A1][0]);
        frame.close(c);
        Ok(())
    })
}

/// `void cSetupMan_HostRecvMsg9(cSetupMan *self, u32 a1, u32 a2)` (0x269538):
/// same 0x14-byte message layout as the client variant, with `+8 = 1` and
/// the node spliced after `*(gp_3-0xC8)+0xF0`.
pub fn csetup_man_host_recv_msg9(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 36;
        let frame = Frame::open(c, 0x60);
        c.g[A0][0] = 0x14;
        frame.save(c, S0, 0x50);
        c.g[A3][0] = 0;
        frame.save(c, S1, 0x40);
        c.g[S0][0] = c.g[A1][0];
        c.g[S1][0] = c.g[A2][0];
        c.g[A1][0] = 0x0048_0f18;
        frame.save(c, S2, 0x30);
        frame.save_ra(c, 0x20);
        c.g[A2][0] = 0x2000_0000;
        call_ptr(c, 0x0031_7d70, 0x0026_956c)?;
        c.g[S2][0] = c.g[V0][0];
        c.g[V1][0] = 0xfc;
        c.g[V0][0] = 0x0048_1130;
        Addr(c.g[S2][0] as u32).sw(c, c.g[V1][0]);
        c.g[A0][0] = 0x1;
        Addr(p_3(c.g[S2][0], 0x8)).sw(c, c.g[A0][0]);
        c.g[A1][0] = 0x0048_04a8;
        Addr(p_3(c.g[S2][0], 0x4)).sw(c, c.g[V0][0]);
        Addr(p_3(c.g[S2][0], 0xc)).sw(c, c.g[S0][0]);
        c.g[A0][0] = 0xc;
        Addr(p_3(c.g[S2][0], 0x10)).sw(c, c.g[S1][0]);
        c.g[A2][0] = 0x2000_0000;
        c.g[V1][0] = Addr(gp_3(c, -0xc8)).lw(c);
        c.g[A3][0] = 0;
        c.g[V0][0] = Addr(p_3(c.g[V1][0], 0xf0)).lw(c);
        Addr(frame.at(c, 0x10)).sw(c, c.g[V0][0]);
        call_ptr(c, 0x0031_7d70, 0x0026_95b8)?;
        c.g[A1][0] = c.g[V0][0];
        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x8));
        if c.g[V0][0] != 0 {
            Addr(p_3(c.g[A1][0], 0x8)).sw(c, c.g[S2][0]);
        }
        c.cycles += 15;
        c.g[V0][0] = Addr(frame.at(c, 0x10)).lw(c);
        frame.restore(c, S0, 0x50);
        Addr(c.g[A1][0] as u32).sw(c, c.g[V0][0]);
        frame.restore(c, S1, 0x40);
        c.g[V1][0] = Addr(p_3(c.g[V0][0], 0x4)).lw(c);
        frame.restore(c, S2, 0x30);
        Addr(p_3(c.g[A1][0], 0x4)).sw(c, c.g[V1][0]);
        frame.restore_ra(c, 0x20);
        c.g[A0][0] = Addr(p_3(c.g[V0][0], 0x4)).lw(c);
        Addr(c.g[A0][0] as u32).sw(c, c.g[A1][0]);
        c.g[V0][0] = Addr(frame.at(c, 0x10)).lw(c);
        Addr(p_3(c.g[V0][0], 0x4)).sw(c, c.g[A1][0]);
        Addr(frame.at(c, 0)).sw(c, c.g[A1][0]);
        frame.close(c);
        Ok(())
    })
}

/// `void cSetupMan_ClientRecvMsg8(cSetupMan *self, u32 a1, u32 a2)`
/// (0x269948): queue message kind 0xFC with the extra word 1.
pub fn csetup_man_client_recv_msg8(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 36;
        let frame = Frame::open(c, 0x60);
        c.g[A0][0] = 0x14;
        frame.save(c, S0, 0x50);
        c.g[A3][0] = 0;
        frame.save(c, S1, 0x40);
        c.g[S0][0] = c.g[A1][0];
        c.g[S1][0] = c.g[A2][0];
        c.g[A1][0] = 0x0048_0f18;
        frame.save(c, S2, 0x30);
        frame.save_ra(c, 0x20);
        c.g[A2][0] = 0x2000_0000;
        call_ptr(c, 0x0031_7d70, 0x0026_997c)?;
        c.g[S2][0] = c.g[V0][0];
        c.g[V1][0] = 0xfc;
        c.g[V0][0] = 0x0048_1130;
        Addr(c.g[S2][0] as u32).sw(c, c.g[V1][0]);
        c.g[A0][0] = 1;
        Addr(p_3(c.g[S2][0], 0x8)).sw(c, c.g[A0][0]);
        c.g[A1][0] = 0x0048_04a8;
        Addr(p_3(c.g[S2][0], 0x4)).sw(c, c.g[V0][0]);
        Addr(p_3(c.g[S2][0], 0xc)).sw(c, c.g[S0][0]);
        c.g[A0][0] = 0xc;
        Addr(p_3(c.g[S2][0], 0x10)).sw(c, c.g[S1][0]);
        c.g[A2][0] = 0x2000_0000;
        c.g[V1][0] = Addr(gp_3(c, -0xc8)).lw(c);
        c.g[A3][0] = 0;
        c.g[V0][0] = Addr(p_3(c.g[V1][0], 0xf0)).lw(c);
        Addr(frame.at(c, 0x10)).sw(c, c.g[V0][0]);
        call_ptr(c, 0x0031_7d70, 0x0026_99c8)?;
        c.g[A1][0] = c.g[V0][0];
        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x8));
        if c.g[V0][0] != 0 {
            Addr(p_3(c.g[A1][0], 0x8)).sw(c, c.g[S2][0]);
        }
        c.cycles += 15;
        c.g[V0][0] = Addr(frame.at(c, 0x10)).lw(c);
        frame.restore(c, S0, 0x50);
        Addr(c.g[A1][0] as u32).sw(c, c.g[V0][0]);
        frame.restore(c, S1, 0x40);
        c.g[V1][0] = Addr(p_3(c.g[V0][0], 0x4)).lw(c);
        frame.restore(c, S2, 0x30);
        Addr(p_3(c.g[A1][0], 0x4)).sw(c, c.g[V1][0]);
        frame.restore_ra(c, 0x20);
        c.g[A0][0] = Addr(p_3(c.g[V0][0], 0x4)).lw(c);
        Addr(c.g[A0][0] as u32).sw(c, c.g[A1][0]);
        c.g[V0][0] = Addr(frame.at(c, 0x10)).lw(c);
        Addr(p_3(c.g[V0][0], 0x4)).sw(c, c.g[A1][0]);
        Addr(frame.at(c, 0)).sw(c, c.g[A1][0]);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Path_FindMarkerBoundary (0x00269f18).

/// `Path_FindMarkerBoundary(path, rangeList, float* t)`:
/// scan the range list for the entry containing `*t` and clamp `*t` to that
/// entry's boundary (or `*t + 50` when nothing matches), then evaluate the
/// path at the clamped distance. The list is `{int count; Entry* entries}`
/// with 0x10-byte entries (`{int type; int pad; float lo; float hi;}`), and
/// the same object doubles as the path track (`+8` segment count, `+0x18`
/// segments).
pub fn path_find_marker_boundary(c: &mut Cpu) {
    guest(c, |c| {
        'b_269fc0: {
            c.cycles += 11;
            c.g[SP][0] = sub(c.g[SP][0], 0x20);
            c.g[AT][0] = 0x4248_0000;
            c.mtc1(1, 0);
            c.sq(16, p(c.g[SP][0] as u32, 0x10));
            c.sd(p(c.g[SP][0] as u32, 0), c.g[RA][0]);
            c.g[S0][0] = c.g[A0][0];
            c.g[V0][0] = lw(c, c.g[A1][0] as u32);
            c.lwc1(1, c.g[A2][0] as u32);
            c.add_s(3, 1, 0);
            let none = (c.g[V0][0] as i64) <= 0;
            c.mov_s(2, 3);
            if none {
                break 'b_269fc0;
            }
            c.cycles += 7;
            c.g[V1][0] = lw(c, p(c.g[A1][0] as u32, 4));
            c.mov_s(4, 1);
            c.g[A0][0] = c.g[V0][0];
            c.g[T1][0] = 0x10;
            c.g[T0][0] = 0xc;
            c.g[A3][0] = 0xe;
            loop {
                'b_269fb8: {
                    c.cycles += 5;
                    c.lwc1(1, p(c.g[V1][0] as u32, 8));
                    c.c_lt_s(4, 1);
                    if c.fcc() {
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(1));
                        break 'b_269fb8;
                    }
                    c.cycles += 5;
                    c.lwc1(0, p(c.g[V1][0] as u32, 0xc));
                    c.c_lt_s(0, 4);
                    if c.fcc() {
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(1));
                        break 'b_269fb8;
                    }
                    'b_269fb4: {
                        'b_269f9c: {
                            c.cycles += 3;
                            c.g[V0][0] = lw(c, c.g[V1][0] as u32);
                            if c.g[V0][0] != c.g[T1][0] {
                                break 'b_269f9c;
                            }
                            c.cycles += 2;
                            c.max_s(3, 0, 3);
                            break 'b_269fb4;
                        }
                        'b_269fac: {
                            c.cycles += 2;
                            if c.g[V0][0] != c.g[T0][0] {
                                break 'b_269fac;
                            }
                            c.cycles += 2;
                            c.min_s(2, 1, 2);
                            break 'b_269fb4;
                        }
                        c.cycles += 2;
                        if c.g[V0][0] == c.g[A3][0] {
                            c.max_s(2, 0, 2);
                        }
                        break 'b_269fb4;
                    }
                    c.cycles += 1;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(1));
                    break 'b_269fb8;
                }
                c.cycles += 2;
                let more = c.g[A0][0] != 0;
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
                if !more {
                    break 'b_269fc0;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0026_9f60);
                }
            }
        }
        'b_269fd8: {
            c.cycles += 5;
            c.lwc1(0, c.g[A2][0] as u32);
            c.c_eq_s(2, 0);
            if c.fcc() {
                c.swc1(3, c.g[A2][0] as u32);
                break 'b_269fd8;
            }
            c.cycles += 1;
            c.swc1(2, c.g[A2][0] as u32);
            break 'b_269fd8;
        }
        c.cycles += 9;
        c.lwc1(12, c.g[A2][0] as u32);
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0;
        call_ptr(c, 0x0026_ab20, 0x0026_9fe8)?;
        c.g[V0][0] = c.g[S0][0];
        c.g[RA][0] = c.ld(p(c.g[SP][0] as u32, 0));
        c.lq(16, p(c.g[SP][0] as u32, 0x10));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = t;
        Ok(())
    });
}

/// A `gp_4`-relative global.
#[inline(always)]
pub(crate) fn gp_4(c: &Cpu, off: i32) -> u32 {
    (c.g[GP][0] as u32).wrapping_add(off as u32)
}

/// `cSetupMan_HostRecvMsg11(self, msg)` (0x269608): handle message type 11
/// from the host: normalise the message type (6..7 -> 8, 9 -> 10), forward
/// it to the character setup as message 11, then build a two-node packet
/// entry (type 0xfd, the default handler) and splice it into the setup
/// manager's list.
pub fn csetupman_host_recv_msg11(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x40);
        let msg = abi::arg(c, 0);
        frame.save(c, 16, 0x30);
        c.sd(p(sp(c), 0x20), c.g[RA][0]);
        c.g[V1][0] = lw_3(c, msg);
        c.g[V0][0] = ((c.g[V1][0] as i64) < 6) as u64;
        if c.g[V0][0] != 0 {
            c.g[A2][0] = lw_3(c, msg);
        } else if (c.g[V1][0] as i64) < 8 {
            c.cycles += 3;
            c.g[V0][0] = 8;
            c.cycles += 2;
            c.sw(msg, c.g[V0][0]);
            c.g[A2][0] = lw_3(c, msg);
        } else {
            c.cycles += 3;
            c.g[V0][0] = 9;
            if c.g[V1][0] == c.g[V0][0] {
                c.g[V0][0] = 0xa;
                c.cycles += 2;
                c.sw(msg, c.g[V0][0]);
                c.g[A2][0] = lw_3(c, msg);
            } else {
                c.cycles += 2;
                c.g[A2][0] = lw_3(c, msg);
            }
        }
        c.cycles += 30;
        c.g[A1][0] = 0xb;
        c.g[RA][0] = sx(0x0026_9658);
        c.g[A3][0] = 0;
        c.pc = 0x0026_4e50;
        ee::s07::cSetupMan_SendCharacterMsg(c);
        if c.pc != 0x0026_9658 {
            return Err(Unwound);
        }
        c.g[A1][0] = 0x48_0000;
        c.g[A0][0] = 0xc;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xf18));
        c.g[A2][0] = 0x2000_0000;
        c.g[RA][0] = sx(0x0026_9670);
        c.g[A3][0] = 0;
        call_ptr(c, 0x0031_7d70, 0x0026_9670)?; // operator new(0xc, ...)
        c.g[S0][0] = c.g[V0][0];
        c.g[V1][0] = 0xfd;
        c.g[V0][0] = 0x48_0000;
        c.sw(c.g[S0][0] as u32, c.g[V1][0]);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x13e0));
        c.g[A0][0] = 1;
        c.sw(p(c.g[S0][0] as u32, 8), c.g[A0][0]);
        c.g[A1][0] = 0x48_0000;
        c.sw(p(c.g[S0][0] as u32, 4), c.g[V0][0]);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4a8));
        c.g[V1][0] = lw_3(c, gp_4(c, -0xc8));
        c.g[A0][0] = 0xc;
        c.g[A2][0] = 0x2000_0000;
        c.g[A3][0] = 0;
        c.g[V0][0] = lw_3(c, p(c.g[V1][0] as u32, 0xf0));
        c.g[RA][0] = sx(0x0026_96b4);
        c.sw(p(sp(c), 0x10), c.g[V0][0]);
        call_ptr(c, 0x0031_7d70, 0x0026_96b4)?; // operator new(0xc, ...)
        c.g[A1][0] = c.g[V0][0];
        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(8));
        if c.g[V0][0] != 0 {
            c.sw(p(c.g[A1][0] as u32, 8), c.g[S0][0]);
        }
        c.cycles += 13;
        c.g[V0][0] = lw_3(c, p(sp(c), 0x10));
        frame.restore(c, 16, 0x30);
        c.sw(c.g[A1][0] as u32, c.g[V0][0]);
        c.g[RA][0] = c.ld(p(sp(c), 0x20));
        c.g[V1][0] = lw_3(c, p(c.g[V0][0] as u32, 4));
        c.sw(p(c.g[A1][0] as u32, 4), c.g[V1][0]);
        c.g[A0][0] = lw_3(c, p(c.g[V0][0] as u32, 4));
        c.sw(c.g[A0][0] as u32, c.g[A1][0]);
        c.g[V0][0] = lw_3(c, p(sp(c), 0x10));
        c.sw(p(c.g[V0][0] as u32, 4), c.g[A1][0]);
        c.sw(sp(c), c.g[A1][0]);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        Ok(())
    })
}

pub(crate) mod path_music {

    /// Initialised flag (byte), the master volume (byte), a map selection
    /// in progress (word: the selected map's flags), the selected map.
    pub const INITED: u32 = 0x0044_f42c;

    pub const MASTER_VOLUME: u32 = 0x0044_f42e;

    pub const SELECTED: u32 = 0x0044_f430;

    pub const CURRENT_MAP: u32 = 0x0044_f420;

    /// The four maps (0x928 bytes each): flags (`+0x30`, high nibbles
    /// matched against the mask), loaded (`+0x34`); a map's 24 tracks at
    /// `+0x58`.
    pub const MAPS: u32 = 0x0051_7610;

    pub const MAP_SIZE: u64 = 0x928;

    pub const MAP_FLAGS: u32 = 0x30;

    pub const MAP_LOADED: u32 = 0x34;

    pub const MAP_TRACKS: u32 = 0x58;

    pub const TRACKS: u32 = 24;
}

pub(crate) mod path_callee {
    use super::ee;
    use super::Callee;

    pub const SELECT_MAP: Callee = Callee {
        addr: 0x003d_5450,
        f: ee::s14::PATH_SelectMap,
    };

    pub const TRACK_SET_VOLUME: Callee = Callee {
        addr: 0x003d_62c8,
        f: ee::s14::PATH_TrackSetVolume,
    };
}

pub(crate) mod path_ctl {

    /// `u8`: the PATH player is initialised.
    pub const ENABLED: u32 = 0x0044_f42c;

    /// `PathMap*` of the selected map (its 24 track slots at +0x58).
    pub const CURRENT_MAP: u32 = 0x0044_f420;

    /// `int` id of the selected map (0 = none selected).
    pub const CURRENT_ID: u32 = 0x0044_f430;

    /// `&maps[0].loaded` (+0x34) and `&maps[0].id` (+0x30): the loop walks
    /// both with a stride of one map.
    pub const MAP0_LOADED: u32 = 0x0051_7644;

    pub const MAP0_ID: u32 = 0x0051_7640;

    pub const MAP_SIZE: u64 = 0x928;

    pub const SLOT0: u32 = 0x58;

    /// Track bytes: the control value and its previous value.
    pub const VALUE: u32 = 0x1;

    pub const PREV_VALUE: u32 = 0x2;
}

/// `void PATH_SetVolumePercent(u32 mask, s8 percent)` (0x3d5508): clamp
/// the volume to 0..100 (mask -1 also sets the master volume) and give it
/// to every track whose bit is set in `mask`, of every loaded map whose
/// flag nibbles match the mask. Does nothing before init or while a map is
/// selected.
pub fn path_set_volume_percent(c: &mut Cpu) {
    use path_callee::*;
    use path_music::*;
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0xa0);
        c.g[V0][0] = 0x0045_0000;
        frame.save(c, S1, 0x80);
        c.g[A1][0] = sx((c.g[A1][0] as u32) << 24);
        frame.save(c, S2, 0x70);
        c.g[S1][0] = sx(((c.g[A1][0] as u32) as i32 >> 24) as u32);
        c.g[V1][0] = Addr(INITED).lbu(c);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0x90);
        frame.save(c, S3, 0x60);
        frame.save(c, S4, 0x50);
        frame.save(c, S5, 0x40);
        frame.save(c, S6, 0x30);
        frame.save(c, S7, 0x20);
        frame.save(c, FP, 0x10);
        frame.save_ra(c, 0);
        let mask = c.g[S2][0];
        // Whether the exit passes the block at 0x3d5668 (+1 cycle).
        let mut tail = true;
        'body: {
            if c.g[V1][0] == 0 {
                break 'body;
            }
            c.cycles += 4;
            c.g[V1][0] = 0x0045_0000;
            c.g[V0][0] = Addr(SELECTED).lw(c);
            if c.g[V0][0] != 0 {
                tail = false;
                break 'body;
            }
            c.cycles += 4;
            c.g[A0][0] = u64::MAX;
            SELECT_MAP.call(c, 0x003d_5564)?;
            c.g[V1][0] = 100;
            if c.g[V0][0] == 0 {
                break 'body;
            }
            // Clamp to 0..100.
            c.cycles += 8;
            c.g[A0][0] = ((c.g[S1][0] as i64) < 101) as u64;
            if c.g[A0][0] == 0 {
                c.g[S1][0] = 100;
            }
            c.g[V0][0] = (-1 < c.g[S1][0] as i64) as u64;
            c.g[V1][0] = u64::MAX;
            if c.g[V0][0] == 0 {
                c.g[S1][0] = 0;
            }
            if mask == u64::MAX {
                c.cycles += 2;
                c.g[V0][0] = 0x0045_0000;
                let vol = c.g[S1][0];
                Addr(MASTER_VOLUME).sb(c, vol);
            }
            c.cycles += 9;
            c.g[V0][0] = 0x0051_0000;
            c.g[A0][0] = 0;
            c.g[S5][0] = sx(MAPS + MAP_LOADED);
            c.g[S4][0] = 0x0045_0000;
            c.g[FP][0] = MAP_SIZE;
            c.g[S7][0] = 0xffff_ffff_f000_0000;
            c.g[S6][0] = 0x0f00_0000;
            c.mult(0, A1, A0, FP);
            loop {
                // Map i = a0 (a1 = i * 0x928).
                let map = MAPS.wrapping_add(c.g[A1][0] as u32);
                c.cycles += 4;
                c.g[V0][0] = sx(map.wrapping_add(MAP_LOADED));
                c.g[V1][0] = Addr(map.wrapping_add(MAP_LOADED)).lw(c);
                c.g[S3][0] = sx((c.g[A0][0] as u32).wrapping_add(1));
                let mut skip_tail = true;
                'map: {
                    if c.g[V1][0] == 0 {
                        break 'map;
                    }
                    c.cycles += 8;
                    c.g[V0][0] = sx(MAPS + MAP_FLAGS);
                    c.g[V1][0] = sx(map.wrapping_add(MAP_FLAGS));
                    c.g[V1][0] = Addr(map.wrapping_add(MAP_FLAGS)).lw(c);
                    let flags = c.g[V1][0];
                    let high = flags & c.g[S7][0] & mask;
                    c.g[V0][0] = flags & c.g[S6][0];
                    if high == 0 {
                        break 'map;
                    }
                    c.cycles += 3;
                    c.g[V0][0] &= mask;
                    if c.g[V0][0] == 0 {
                        c.g[A0][0] = c.g[S3][0];
                        skip_tail = false;
                        break 'map;
                    }
                    c.cycles += 4;
                    SELECT_MAP.call(c, 0x003d_55fc)?;
                    let selected = c.g[V0][0] != 0;
                    c.g[A0][0] = c.g[S3][0];
                    if !selected {
                        skip_tail = false;
                        break 'map;
                    }
                    // The selected map's tracks in the mask.
                    c.cycles += 3;
                    c.g[S0][0] = 0;
                    c.g[V0][0] = Addr(CURRENT_MAP).lw(c);
                    loop {
                        let j = c.g[S0][0] as u32;
                        c.cycles += 5;
                        c.g[V1][0] = sx(j << 2);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(j << 2));
                        c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(MAP_TRACKS)).lw(c);
                        let track = c.g[A0][0];
                        c.g[V0][0] = sx((mask as u32) >> (j & 31));
                        let mut step_tail = true;
                        if track != 0 {
                            c.cycles += 4;
                            c.g[V0][0] = (c.g[V0][0] ^ 1) & 1;
                            if c.g[V0][0] != 0 {
                                step_tail = false;
                            } else {
                                c.cycles += 2;
                                c.g[A1][0] = c.g[S1][0];
                                TRACK_SET_VOLUME.call(c, 0x003d_563c)?;
                            }
                        }
                        if step_tail {
                            c.cycles += 1;
                        }
                        c.g[S0][0] = sx(j.wrapping_add(1));
                        c.cycles += 3;
                        c.g[V0][0] = ((c.g[S0][0] as i64) < TRACKS as i64) as u64;
                        let more = c.g[V0][0] != 0;
                        c.g[V0][0] = Addr(CURRENT_MAP).lw(c);
                        if !more {
                            break;
                        }
                        if event_due(c) {
                            deliver_event_here(c, 0x003d_5610);
                        }
                    }
                }
                if skip_tail {
                    c.cycles += 1;
                    c.g[A0][0] = c.g[S3][0];
                }
                c.cycles += 3;
                c.g[V0][0] = ((c.g[A0][0] as i64) < 4) as u64;
                if c.g[V0][0] == 0 {
                    break;
                }
                c.mult(0, A1, A0, FP);
                if event_due(c) {
                    deliver_event_here(c, 0x003d_55b8);
                }
            }
            // Done: forget the selection.
            c.cycles += 3;
            c.g[V0][0] = 0x0045_0000;
            Addr(CURRENT_MAP).sw(c, 0);
            Addr(SELECTED).sw(c, 0);
        }
        if tail {
            c.cycles += 1;
        }
        frame.restore(c, S0, 0x90);
        c.cycles += 11;
        frame.restore(c, S1, 0x80);
        frame.restore(c, S2, 0x70);
        frame.restore(c, S3, 0x60);
        frame.restore(c, S4, 0x50);
        frame.restore(c, S5, 0x40);
        frame.restore(c, S6, 0x30);
        frame.restore(c, S7, 0x20);
        frame.restore(c, FP, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

pub(crate) fn gp_lit(c: &mut Cpu, neg: u32) -> FpuF32 {
    let gp = c.g[GP][0] as u32;
    Addr(gp.wrapping_sub(neg)).read_fpu(c)
}

pub(crate) fn frame_f(c: &mut Cpu, off: u32) -> FpuF32 {
    Addr(sp(c).wrapping_add(off)).read_fpu(c)
}

pub(crate) fn set_frame_f(c: &mut Cpu, off: u32, v: FpuF32) {
    Addr(sp(c).wrapping_add(off)).write_fpu(c, v)
}

// ---------------------------------------------------------------------------
// Path_GetClosestPoint (0x26a428)

/// Path: segment count (+0x8), start point (+0xc..0x14), segments (+0x18:
/// 0x10 bytes each, unit direction xyz and length).
pub(crate) const PATH_SEG_COUNT: u32 = 0x8;

pub(crate) const PATH_START: u32 = 0xc;

pub(crate) const PATH_SEGMENTS: u32 = 0x18;

#[cfg(test)]
pub(crate) const SEG_SIZE: u32 = 0x10;

/// `.lit4` at `gp-0x4a58`: the initial best distance (a large value).
pub(crate) const LIT_PATH_FAR: u32 = 0x4a58;

/// Frame slots (0x60 bytes): the current segment start point, the segment
/// direction, the point-to-start delta, the candidate point and VU0
/// spill slots.
pub(crate) const CP_START: u32 = 0x00;

pub(crate) const CP_DIR: u32 = 0x10;

pub(crate) const CP_DELTA: u32 = 0x20;

pub(crate) const CP_POINT: u32 = 0x30;

pub(crate) const CP_STEP: u32 = 0x40;

pub(crate) const CP_SPILL: u32 = 0x50;

/// `vf5 = a - b` (lane-wise, `vsub`), spilled through `spill` and v0 to `dst`.
pub(crate) fn vu_sub_spill(c: &mut Cpu, a: u32, b: u32, spill: u32, dst: u32) {
    c.lqc2(3, a);
    c.lqc2(4, b);
    c.vu0().sub(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, spill);
    c.lq(V0, spill);
    c.sq(V0, dst);
}

/// `vf5 = a + b`, spilled through `spill` and v0 to `dst`.
pub(crate) fn vu_add_spill(c: &mut Cpu, a: u32, b: u32, spill: u32, dst: u32) {
    c.lqc2(3, a);
    c.lqc2(4, b);
    c.vu0().add(0b1111, 5, 3, Src::Vec(4));
    c.sqc2(5, spill);
    c.lq(V0, spill);
    c.sq(V0, dst);
}

/// `vf5 = dir * s` with `s` moved in through GPR `gpr` (`mfc1`, `qmtc2` to
/// vf3), spilled to the spill slot and loaded into v0.
pub(crate) fn vu_scale_dir(c: &mut Cpu, gpr: usize, s: FpuF32) {
    let base = sp(c);
    c.g[gpr][0] = sx(s.0);
    c.lqc2(4, base + CP_DIR);
    c.qmtc2(gpr, 3);
    c.vu0().mul(0b1111, 5, 4, Src::Bc(3, X));
    c.sqc2(5, base + CP_SPILL);
}

/// The 4-lane dot product `a . b` on VU0 (vf4.x, through v0), returned as
/// the float moved to f1.
pub(crate) fn vu_dot4_f1(c: &mut Cpu, a: u32, b: u32) -> FpuF32 {
    c.lqc2(3, a);
    c.lqc2(5, b);
    c.vu0().add(0b1000, 6, 0, Src::Bc(0, W));
    c.vu0().mul(0b1111, 4, 3, Src::Vec(5));
    c.vu0().adda(0b1000, 4, Src::Bc(4, Y));
    c.vu0().madda(0b1000, 6, Src::Bc(4, Z));
    c.vu0().madd(0b1000, 4, 6, Src::Bc(4, W));
    c.qmfc2(V0, 4);
    FpuF32(c.g[V0][0] as u32)
}

/// The squared length of the frame vector at `off` in the xy plane.
pub(crate) fn xy_len2(c: &mut Cpu, off: u32) -> (FpuF32, FpuF32) {
    let x = frame_f(c, off);
    let y = frame_f(c, off + 4);
    (x * x, y * y)
}

/// `float Path_GetClosestPoint(Path* path, Vec4* p, Vec4* closest,
/// float* dist, int flat)` (0x26a428): walk the path's segments; project
/// `p` onto each (in the xy plane when `flat`, else with the 4-lane VU0
/// dot product), clamp to the segment, and keep the nearest point in
/// `*closest`. Returns the distance along the path of that point in f0
/// and stores the distance to it in `*dist`. The segment loop keeps its
/// state in the frame and the original's registers (t1 index, f5 length
/// so far, f6 best length, f7/f9 best distances, f8 zero) across its
/// back-edge event check.
pub fn path_get_closest_point(c: &mut Cpu) {
    c.cycles += 17;
    let path = Addr(c.g[A0][0] as u32);
    let start_x = path.off(PATH_START).read_fpu(c);
    c.g[SP][0] = sx(sp(c).wrapping_sub(0x60));
    let start_y = path.off(PATH_START + 4).read_fpu(c);
    c.g[T1][0] = 0;
    let start_z = path.off(PATH_START + 8).read_fpu(c);
    c.g[AT][0] = 0x3f80_0000;
    c.g[V0][0] = path.off(PATH_SEG_COUNT).lw(c);
    let base = sp(c);
    set_frame_f(c, CP_START, start_x);
    set_frame_f(c, CP_START + 4, start_y);
    set_frame_f(c, CP_START + 8, start_z);
    let far = gp_lit(c, LIT_PATH_FAR);
    set_frame_f(c, CP_START + 0xc, FpuF32::ONE);
    c.f[0] = start_x.0;
    c.f[1] = start_y.0;
    c.f[2] = start_z.0;
    c.f[3] = FpuF32::ONE.0;
    c.f[5] = 0;
    c.f[6] = 0;
    c.f[7] = 0;
    c.f[9] = far.0;
    if (c.g[V0][0] as i64) > 0 {
        c.cycles += 3;
        c.f[8] = c.f[6];
        c.g[V1][0] = path.off(PATH_SEGMENTS).lw(c);
        loop {
            // The segment: direction into the frame, delta = p - start.
            c.cycles += 17;
            let flat = c.g[T0][0] != 0;
            let p = c.g[A1][0] as u32;
            let seg = Addr(((c.g[T1][0] as u32) << 4).wrapping_add(c.g[V1][0] as u32));
            c.g[V0][0] = sx(seg.0);
            let dx = seg.read_fpu(c);
            set_frame_f(c, CP_DIR, dx);
            let dy = seg.off(4).read_fpu(c);
            set_frame_f(c, CP_DIR + 4, dy);
            let dz = seg.off(8).read_fpu(c);
            Addr(base + CP_DIR + 0xc).sw(c, 0);
            set_frame_f(c, CP_DIR + 8, dz);
            let len = seg.off(0xc).read_fpu(c);
            c.f[0] = dz.0;
            c.f[1] = dy.0;
            c.f[4] = len.0;
            vu_sub_spill(c, p, base + CP_START, base + CP_POINT, base + CP_DELTA);
            // Projection of the delta on the direction.
            let along = if flat {
                c.cycles += 8;
                let ex = frame_f(c, CP_DELTA);
                let gx = frame_f(c, CP_DIR);
                let ey = frame_f(c, CP_DELTA + 4);
                let gy = frame_f(c, CP_DIR + 4);
                let a = ex * gx;
                let b = ey * gy;
                c.f[0] = b.0;
                c.f[2] = gy.0;
                c.f[3] = gx.0;
                a + b
            } else {
                c.cycles += 9;
                vu_dot4_f1(c, base + CP_DELTA, base + CP_DIR)
            };
            c.f[1] = along.0;
            // Clamp to [0, len].
            c.cycles += 4;
            let zero = FpuF32(c.f[8]);
            let ge = zero.le(along);
            abi::set_fcc(c, ge);
            let t = if ge {
                c.cycles += 1;
                along.min(len)
            } else {
                zero
            };
            c.f[2] = t.0;
            // Candidate point = start + dir * t; delta = p - candidate.
            c.cycles += 21;
            vu_scale_dir(c, V1, t);
            c.lq(V0, base + CP_SPILL);
            c.sq(V0, base + CP_STEP);
            vu_add_spill(
                c,
                base + CP_START,
                base + CP_STEP,
                base + CP_SPILL,
                base + CP_POINT,
            );
            vu_sub_spill(c, p, base + CP_POINT, base + CP_SPILL, base + CP_DELTA);
            c.sq(V0, base + CP_STEP);
            let dist2 = if flat {
                c.cycles += 6;
                let (x2, y2) = xy_len2(c, CP_DELTA);
                c.f[0] = y2.0;
                x2 + y2
            } else {
                c.cycles += 9;
                vu_dot4_f1(c, base + CP_DELTA, base + CP_DELTA)
            };
            c.f[1] = dist2.0;
            // Nearer than the best so far?
            c.cycles += 3;
            let best = FpuF32(c.f[9]);
            let nearer = dist2.lt(best);
            abi::set_fcc(c, nearer);
            if nearer {
                c.cycles += 2;
                c.f[9] = dist2.0;
                if flat {
                    c.cycles += 2;
                    c.f[7] = dist2.0;
                } else {
                    c.cycles += 5;
                    let (x2, y2) = xy_len2(c, CP_DELTA);
                    c.f[1] = x2.0;
                    c.f[0] = y2.0;
                    c.f[7] = (x2 + y2).0;
                }
                c.cycles += 3;
                c.lq(V0, base + CP_POINT);
                c.f[6] = (FpuF32(c.f[5]) + t).0;
                let out = c.g[A2][0] as u32;
                c.sq(V0, out);
            }
            // Next segment start = start + dir * len.
            c.cycles += 17;
            vu_scale_dir(c, V0, len);
            c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(1));
            c.lq(V0, base + CP_SPILL);
            c.f[5] = (FpuF32(c.f[5]) + len).0;
            c.sq(V0, base + CP_STEP);
            c.lqc2(3, base + CP_START);
            c.lqc2(4, base + CP_STEP);
            c.vu0().add(0b1111, 5, 3, Src::Vec(4));
            c.sqc2(5, base + CP_START);
            c.g[V0][0] = path.off(PATH_SEG_COUNT).lw(c);
            c.g[V0][0] = ((c.g[T1][0] as i64) < (c.g[V0][0] as i64)) as u64;
            if c.g[V0][0] == 0 {
                break;
            }
            c.g[V1][0] = path.off(PATH_SEGMENTS).lw(c);
            if event_due(c) {
                deliver_event_here(c, 0x0026_a478);
            }
        }
    }
    c.cycles += 5;
    let dist = FpuF32(c.f[7]).sqrt();
    c.f[1] = dist.0;
    c.f[0] = c.f[6];
    Addr(c.g[A3][0] as u32).write_fpu(c, dist);
    c.g[SP][0] = sx(sp(c).wrapping_add(0x60));
    ret(c);
}

// ---------------------------------------------------------------------------
// PATH_SelectMasterTrack (0x3d4b00)

/// The PATH module's state pointer (`0x44f420`): the master track selection
/// (+0 index, +4 length, +8 kind) and 24 track pointers from +0x58.
pub(crate) const PATH_STATE: u32 = 0x0044_f420;

pub(crate) const PATH_MASTER_INDEX: u32 = 0x0;

pub(crate) const PATH_MASTER_LENGTH: u32 = 0x4;

pub(crate) const PATH_MASTER_KIND: u32 = 0x8;

pub(crate) const PATH_TRACKS: u32 = 0x58;

pub(crate) const PATH_TRACK_COUNT: i64 = 24;

/// Track: disabled byte (+3), length (+0x24), header pointer (+0x30, its
/// first word holds the kind in bits 21-26), signed halfword (+0x36,
/// negative: not a candidate).
pub(crate) const TRACK_DISABLED: u32 = 0x3;

pub(crate) const TRACK_LENGTH: u32 = 0x24;

pub(crate) const TRACK_HEADER: u32 = 0x30;

pub(crate) const TRACK_FLAGS: u32 = 0x36;

/// `void PATH_SelectMasterTrack(void)` (0x3d4b00): picks the longest of the
/// 24 PATH tracks that is present, enabled (`+3 == 0`), has a header and a
/// non-negative `+0x36`: index, length (unsigned compare, strictly longer
/// wins) and kind (`header[0] >> 21 & 0x3f`) into the state's master
/// selection (reset to -1/0/0 first).
///
/// Registers as the original: a1 = state, a2 = track slot cursor, a3 =
/// index, t0 = 1, a0/v0/v1 scratch.
pub fn path_select_master_track(c: &mut Cpu) {
    c.cycles += 12;
    c.g[V0][0] = 0x0045_0000;
    c.g[A0][0] = u64::MAX;
    c.g[V1][0] = Addr(PATH_STATE).lw(c);
    c.g[A3][0] = 0;
    c.g[T0][0] = 1;
    c.g[A1][0] = c.g[V1][0];
    let state = Addr(r32(c, A1));
    state.off(PATH_MASTER_INDEX).sw(c, u64::MAX);
    state.off(PATH_MASTER_LENGTH).sw(c, 0);
    c.g[A2][0] = sx(state.0.wrapping_add(PATH_TRACKS));
    state.off(PATH_MASTER_KIND).sw(c, 0);
    c.g[A0][0] = Addr(r32(c, A2)).lw(c);
    loop {
        c.cycles += 2;
        let track = r32(c, A0);
        c.g[A2][0] = sx(r32(c, A2).wrapping_add(4));
        // Through the shared `a3++` with its extra cycle, or straight to the
        // loop test (the original's two increments).
        let mut tail_cycle = true;
        if c.g[A0][0] != 0 {
            c.cycles += 3;
            c.g[V0][0] = Addr(track).off(TRACK_DISABLED).lbu(c);
            c.g[V1][0] = 0;
            if c.g[V0][0] == 0 {
                c.cycles += 3;
                c.g[V0][0] = Addr(track).off(TRACK_FLAGS).lh(c);
                if (c.g[V0][0] as i64) >= 0 {
                    c.cycles += 3;
                    c.g[V0][0] = Addr(track).off(TRACK_HEADER).lw(c);
                    c.g[V1][0] = if c.g[V0][0] == 0 { 0 } else { c.g[T0][0] };
                }
                c.cycles += 2;
                if c.g[V1][0] == 0 {
                    tail_cycle = false;
                } else {
                    c.cycles += 5;
                    c.g[V1][0] = Addr(track).off(TRACK_LENGTH).lw(c);
                    c.g[V0][0] = state.off(PATH_MASTER_LENGTH).lw(c);
                    c.g[V0][0] = (c.g[V0][0] < c.g[V1][0]) as u64;
                    if c.g[V0][0] == 0 {
                        tail_cycle = false;
                    } else {
                        // The longest so far.
                        c.cycles += 7;
                        state.off(PATH_MASTER_LENGTH).sw(c, c.g[V1][0]);
                        state.off(PATH_MASTER_INDEX).sw(c, c.g[A3][0]);
                        c.g[V1][0] = Addr(track).off(TRACK_HEADER).lw(c);
                        c.g[V0][0] = Addr(r32(c, V1)).lw(c);
                        c.g[V0][0] = sx(r32(c, V0) >> 21) & 0x3f;
                        state.off(PATH_MASTER_KIND).sw(c, c.g[V0][0]);
                    }
                }
            }
        }
        if tail_cycle {
            c.cycles += 1;
        }
        c.g[A3][0] = sx(r32(c, A3).wrapping_add(1));
        c.cycles += 3;
        c.g[V0][0] = ((c.g[A3][0] as i64) < PATH_TRACK_COUNT) as u64;
        if c.g[V0][0] == 0 {
            break;
        }
        c.g[A0][0] = Addr(r32(c, A2)).lw(c);
        if event_due(c) {
            deliver_event_here(c, 0x003d_4b30);
        }
    }
    c.cycles += 2;
    ret(c);
}

pub(crate) const PATH_SELECT_MAP: Callee = Callee {
    addr: 0x003d_5450,
    f: ee::s14::PATH_SelectMap,
};

/// `int PATH_SetTrackControlValue(uint mask, uint value)` (0x3d0d70): set
/// the control byte of every track of every loaded path map whose id
/// matches `mask` in both its top and second nibbles (the old value moves
/// to the previous-value byte). Fails with -18 unless the player is on, no
/// map is selected and the `PATH_SelectMap(-1)` query succeeds; values of
/// 0x80 and above change nothing. Clears the selection and returns 0.
pub fn path_set_track_control_value(c: &mut Cpu) {
    use path_ctl::*;
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0xa0);
        c.g[V0][0] = 0x45_0000;
        frame.save(c, S1, 0x80);
        frame.save(c, S2, 0x70);
        c.g[S1][0] = c.g[A1][0];
        c.g[V1][0] = Addr(ENABLED).lbu(c);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0x90);
        frame.save(c, S3, 0x60);
        frame.save(c, S4, 0x50);
        frame.save(c, S5, 0x40);
        frame.save(c, S6, 0x30);
        frame.save(c, S7, 0x20);
        frame.save(c, FP, 0x10);
        frame.save_ra(c, 0);

        let mut ready = false;
        let mut busy = false;
        if c.g[V1][0] != 0 {
            c.cycles += 4;
            c.g[V1][0] = 0x45_0000;
            busy = Addr(CURRENT_ID).lw(c) != 0;
            c.g[V0][0] = sx(-18i32 as u32);
            if !busy {
                c.cycles += 4;
                c.g[A0][0] = u64::MAX;
                PATH_SELECT_MAP.call(c, 0x003d_0dc8)?;
                ready = c.g[V0][0] != 0;
                c.g[V0][0] = (c.g[S1][0] < 0x80) as u64;
            }
        }
        if !busy && !ready {
            c.cycles += 2;
            c.g[V0][0] = sx(-18i32 as u32);
        } else if ready {
            c.cycles += 2;
            c.g[S5][0] = 0x45_0000;
            if c.g[V0][0] != 0 {
                set_values(c)?;
            }
            c.cycles += 4;
            c.g[V0][0] = 0x45_0000;
            Addr(CURRENT_MAP).sw(c, 0);
            Addr(CURRENT_ID).sw(c, 0);
            c.g[V0][0] = 0;
        }

        c.cycles += 12;
        frame.restore(c, S0, 0x90);
        frame.restore(c, S1, 0x80);
        frame.restore(c, S2, 0x70);
        frame.restore(c, S3, 0x60);
        frame.restore(c, S4, 0x50);
        frame.restore(c, S5, 0x40);
        frame.restore(c, S6, 0x30);
        frame.restore(c, S7, 0x20);
        frame.restore(c, FP, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The map loop of `PATH_SetTrackControlValue`: a0 = map index, a1 = its
/// byte offset (`mult`), s1 = value, s2 = mask.
pub(crate) fn set_values(c: &mut Cpu) -> crate::call::Flow {
    use path_ctl::*;
    c.cycles += 8;
    c.g[V0][0] = 0x51_0000;
    c.g[A0][0] = 0;
    c.g[S3][0] = sx(MAP0_LOADED);
    c.g[S7][0] = MAP_SIZE;
    c.g[FP][0] = sx(MAP0_ID);
    c.g[S6][0] = 0xffff_ffff_f000_0000;
    c.g[S4][0] = 0x0f00_0000;
    c.mult(0, A1, A0, S7);
    loop {
        c.cycles += 4;
        c.g[V0][0] = sx(r32(c, A1).wrapping_add(r32(c, S3)));
        c.g[V1][0] = Addr(r32(c, V0)).lw(c);
        c.g[S0][0] = sx(r32(c, A0).wrapping_add(1));
        // `direct`: the second-nibble test failed; skip the 1-cycle tail.
        let mut direct = false;
        if c.g[V1][0] != 0 {
            c.cycles += 6;
            c.g[V1][0] = Addr(r32(c, A1).wrapping_add(r32(c, FP))).lw(c);
            let top = c.g[V1][0] & c.g[S6][0] & c.g[S2][0];
            c.g[V0][0] = c.g[V1][0] & c.g[S4][0];
            if top != 0 {
                c.cycles += 3;
                c.g[V0][0] &= c.g[S2][0];
                if c.g[V0][0] == 0 {
                    c.g[A0][0] = c.g[S0][0];
                    direct = true;
                } else {
                    c.cycles += 4;
                    c.g[A0][0] &= 0xff;
                    PATH_SELECT_MAP.call(c, 0x003d_0e3c)?;
                    let selected = c.g[V0][0] != 0;
                    c.g[A0][0] = 0;
                    if selected {
                        set_map_tracks(c);
                    }
                }
            }
        }
        if !direct {
            c.cycles += 1;
            c.g[A0][0] = c.g[S0][0];
        }
        c.cycles += 3;
        c.g[V0][0] = ((c.g[A0][0] as i64) < 4) as u64;
        if c.g[V0][0] == 0 {
            return Ok(());
        }
        c.mult(0, A1, A0, S7);
        if event_due(c) {
            deliver_event_here(c, 0x003d_0e00);
        }
    }
}

/// Write the value (s1) into the 24 track slots of the selected map.
pub(crate) fn set_map_tracks(c: &mut Cpu) {
    use path_ctl::*;
    c.cycles += 3;
    c.g[A1][0] = 0x45_0000;
    c.g[V0][0] = Addr(CURRENT_MAP).lw(c);
    loop {
        c.cycles += 5;
        c.g[V1][0] = sx(r32(c, A0) << 2);
        c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
        c.g[V1][0] = Addr(r32(c, V0).wrapping_add(SLOT0)).lw(c);
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(1));
        if c.g[V1][0] != 0 {
            c.cycles += 3;
            let track = Addr(r32(c, V1));
            let (value, prev) = (c.g[S1][0], track.off(VALUE).lbu(c));
            c.g[V0][0] = prev;
            track.off(VALUE).sb(c, value);
            track.off(PREV_VALUE).sb(c, prev);
        }
        c.cycles += 3;
        let more = (c.g[A0][0] as i64) < 24;
        c.g[V0][0] = Addr(CURRENT_MAP).lw(c);
        if !more {
            return;
        }
        if event_due(c) {
            deliver_event_here(c, 0x003d_0e50);
        }
    }
}

// ---------------------------------------------------------------------------
// PathData_FindNearTrackPaths (0x0026b178)

/// Path data: track path count and the path array (0x3c bytes per path,
/// bounds at +0x1c).
pub(crate) const NEAR_PATHS_COUNT: u32 = 0x10;

pub(crate) const NEAR_PATHS_ITEMS: u32 = 0x14;

pub(crate) const NEAR_PATHS_STRIDE: u32 = 0x3c;

/// `.lit4` (gp-relative): the "already taken" distance (1e37), loaded twice.
pub(crate) const NEAR_PATHS_TAKEN_LIT: u32 = 0x4a44;

pub(crate) const NEAR_PATHS_TAKEN_LIT2: u32 = 0x4a40;

/// `addu` of two registers' low words.
#[inline]
pub(crate) fn near_paths_addu(a: u64, b: u64) -> u64 {
    sx((a as u32).wrapping_add(b as u32))
}

pub(crate) const PATH_BOUNDS_DISTANCE: Callee = Callee {
    addr: 0x0026_a9b0,
    f: ee::s07::Path_BoundsDistance,
};

/// `int PathData_FindNearTrackPaths(PathData* self, const Vec3* pos,
/// TrackPath** out, int max)` (0x0026b178): collect up to `min(count, max)`
/// track paths near `pos`: first every path whose bounds contain `pos`
/// (distance 0) in order, then the remaining slots by increasing bounds
/// distance (selection by repeated minimum search over a distance array on
/// the stack; a taken path's distance becomes 1e37). Returns the number
/// written.
pub fn path_data_find_near_track_paths(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x180);
        frame.save(c, S0, 0x160);
        frame.save(c, S2, 0x140);
        frame.save(c, S5, 0x110);
        frame.save(c, S6, 0x100);
        frame.save(c, S7, 0xf0);
        frame.save(c, FP, 0xe0);
        frame.save(c, S1, 0x150);
        frame.save(c, S3, 0x130);
        frame.save(c, S4, 0x120);
        frame.save_ra(c, 0xd0);
        frame.save_fpr(c, 21, 0x178);
        frame.save_fpr(c, 20, 0x170);
        c.g[S0][0] = 0; // found
        c.g[S2][0] = 0; // i
        c.g[S5][0] = c.g[A0][0]; // self
        c.g[S7][0] = c.g[A2][0]; // out
        c.g[FP][0] = c.g[A1][0]; // pos
        let count = Addr(r32(c, S5)).off(NEAR_PATHS_COUNT).lw(c);
        c.g[V0][0] = count;
        let max = abi::arg64(c, 3);
        c.g[V1][0] = ((count as i64) < (max as i64)) as u64;
        // want = min(count, max) (s6)
        c.g[S6][0] = if (count as i64) < (max as i64) {
            count
        } else {
            max
        };

        // Pass 1: distances; paths containing pos are taken in order.
        if (count as i64) > 0 {
            c.cycles += 5;
            c.f[21] = 0;
            c.g[S1][0] = c.g[SP][0]; // &dist[i]
            let taken = Addr(r32(c, GP).wrapping_sub(NEAR_PATHS_TAKEN_LIT)).read_fpu(c);
            c.f[20] = taken.0;
            c.g[S4][0] = 0; // i * stride
            c.g[S3][0] = c.g[S7][0]; // &out[found]
            loop {
                c.cycles += 8;
                let items = Addr(r32(c, S5)).off(NEAR_PATHS_ITEMS).lw(c);
                c.g[A1][0] = c.g[FP][0];
                c.g[A0][0] = near_paths_addu(items, c.g[S4][0]);
                PATH_BOUNDS_DISTANCE.call(c, 0x0026_b1f8)?;
                let dist = FpuF32(c.f[0]);
                let inside = dist.eq_s(FpuF32(c.f[21]));
                abi::set_fcc(c, inside);
                Addr(r32(c, S1)).write_fpu(c, dist);
                if inside {
                    c.cycles += 7;
                    let items = Addr(r32(c, S5)).off(NEAR_PATHS_ITEMS).lw(c);
                    c.g[S0][0] = near_paths_addu(c.g[S0][0], 1);
                    let taken = FpuF32(c.f[20]);
                    Addr(r32(c, S1)).write_fpu(c, taken);
                    let path = near_paths_addu(items, c.g[S4][0]);
                    c.g[V0][0] = path;
                    Addr(r32(c, S3)).sw(c, path);
                    let full = c.g[S0][0] == c.g[S6][0];
                    c.g[S3][0] = near_paths_addu(c.g[S3][0], 4);
                    if full {
                        break;
                    }
                }
                c.cycles += 6;
                let count = Addr(r32(c, S5)).off(NEAR_PATHS_COUNT).lw(c);
                c.g[S2][0] = near_paths_addu(c.g[S2][0], 1);
                c.g[S1][0] = near_paths_addu(c.g[S1][0], 4);
                let more = (c.g[S2][0] as i64) < (count as i64);
                c.g[V0][0] = more as u64;
                c.g[S4][0] = near_paths_addu(c.g[S4][0], NEAR_PATHS_STRIDE as u64);
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0026_b1e8);
                }
            }
        }

        // Pass 2: fill the remaining slots with the nearest untaken paths.
        c.cycles += 3;
        let short = (c.g[S0][0] as i64) < (c.g[S6][0] as i64);
        c.g[V0][0] = c.g[S0][0];
        if short {
            c.cycles += 2;
            let taken = Addr(r32(c, GP).wrapping_sub(NEAR_PATHS_TAKEN_LIT2)).read_fpu(c);
            c.f[2] = taken.0;
            c.g[T1][0] = NEAR_PATHS_STRIDE as u64;
            loop {
                c.cycles += 8;
                let count = Addr(r32(c, S5)).off(NEAR_PATHS_COUNT).lw(c);
                c.g[V1][0] = count;
                c.g[S2][0] = 1;
                c.g[T0][0] = near_paths_addu(c.g[S0][0], 1);
                c.f[1] = Addr(sp(c)).read_fpu(c).0; // best = dist[0]
                c.g[A3][0] = 0; // best index
                c.g[V0][0] = (1 < count as i64) as u64;
                c.g[A1][0] = sx((c.g[S0][0] as u32) << 2);
                if 1 < count as i64 {
                    c.cycles += 2;
                    c.g[A0][0] = count;
                    c.g[V1][0] = near_paths_addu(c.g[SP][0], 4);
                    loop {
                        c.cycles += 5;
                        let d = Addr(r32(c, V1)).read_fpu(c);
                        c.f[0] = d.0;
                        let nearer = d.lt(FpuF32(c.f[1]));
                        abi::set_fcc(c, nearer);
                        if nearer {
                            c.cycles += 3;
                            c.f[1] = d.0;
                            c.g[A3][0] = c.g[S2][0];
                        }
                        c.g[S2][0] = near_paths_addu(c.g[S2][0], 1);
                        c.cycles += 3;
                        let more = (c.g[S2][0] as i64) < (c.g[A0][0] as i64);
                        c.g[V0][0] = more as u64;
                        c.g[V1][0] = near_paths_addu(c.g[V1][0], 4);
                        if !more {
                            break;
                        }
                        if event_due(c) {
                            deliver_event_here(c, 0x0026_b278);
                        }
                    }
                }
                // out[found] = &items[best]; dist[best] = taken.
                c.cycles += 11;
                let best = r32(c, A3);
                let offset = abi::mult(c, best, NEAR_PATHS_STRIDE);
                c.g[A2][0] = sx(offset);
                let items = Addr(r32(c, S5)).off(NEAR_PATHS_ITEMS).lw(c);
                c.g[V0][0] = sx(best << 2);
                c.g[A1][0] = near_paths_addu(c.g[A1][0], c.g[S7][0]);
                c.g[V1][0] = near_paths_addu(c.g[SP][0], c.g[V0][0]);
                c.g[S0][0] = c.g[T0][0];
                let taken = FpuF32(c.f[2]);
                Addr(r32(c, V1)).write_fpu(c, taken);
                let more = (c.g[S0][0] as i64) < (c.g[S6][0] as i64);
                c.g[V0][0] = more as u64;
                let path = near_paths_addu(items, c.g[A2][0]);
                c.g[A0][0] = path;
                Addr(r32(c, A1)).sw(c, path);
                if !more {
                    c.cycles += 1;
                    c.g[V0][0] = c.g[S0][0];
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0026_b250);
                }
            }
        }

        c.cycles += 14;
        frame.restore(c, S1, 0x150);
        frame.restore(c, S0, 0x160);
        frame.restore(c, S2, 0x140);
        frame.restore(c, S3, 0x130);
        frame.restore(c, S4, 0x120);
        frame.restore(c, S5, 0x110);
        frame.restore(c, S6, 0x100);
        frame.restore(c, S7, 0xf0);
        frame.restore(c, FP, 0xe0);
        frame.restore_ra(c, 0xd0);
        frame.restore_fpr(c, 21, 0x178);
        frame.restore_fpr(c, 20, 0x170);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// PathData_FindNearAIPaths.

/// `PathData::aiPathCount`.
pub(crate) const PATH_DATA_AI_COUNT: u32 = 0x8;

/// `PathData::aiPaths` (0x40-byte `AIPath` records).
pub(crate) const PATH_DATA_AI_PATHS: u32 = 0xc;

/// `AIPath::enabled` (only checked when the caller asks for enabled paths).
pub(crate) const AI_PATH_ENABLED: u32 = 0x3c;

/// The caller's point, spilled at `sp+0x320`.
pub(crate) const FIND_NEAR_AI_POINT_SLOT: u32 = 0x320;

/// `s1 = s1 + n` etc.: 32-bit add, sign-extended.
#[inline(always)]
pub(crate) fn find_near_ai_add(v: u64, n: u32) -> u64 {
    sx((v as u32).wrapping_add(n))
}

/// `int PathData_FindNearAIPaths(PathData* d, const Vec3* p, AIPath** out,
/// int max, bool enabledOnly)` (0x26afb8): collect up to `max` AI paths
/// near `p` into `out`. First pass: the bounds distance of every path
/// (`Path_BoundsDistance`, kept in a stack array; disabled paths count as
/// "far" (`gp-0x4a4c`) when `enabledOnly`); paths containing `p` (distance
/// 0) are taken at once. If fewer than `max` were found, the rest are the
/// nearest remaining paths in order (selection by smallest distance, each
/// pick marked used with `gp-0x4a48`). Returns the number written.
pub fn path_data_find_near_ai_paths(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x3e0);
        frame.save(c, S0, 0x3c0);
        frame.save(c, S2, 0x3a0);
        c.g[S0][0] = 0; // found
        frame.save(c, S4, 0x380);
        c.g[S2][0] = 0; // path index
        frame.save(c, S6, 0x360);
        c.g[S4][0] = c.g[A0][0]; // data
        frame.save(c, S7, 0x350);
        frame.save(c, FP, 0x340);
        c.g[S7][0] = c.g[A2][0]; // out
        frame.save(c, S1, 0x3b0);
        c.g[FP][0] = c.g[T0][0]; // enabledOnly
        frame.save(c, S3, 0x390);
        frame.save(c, S5, 0x370);
        frame.save_ra(c, 0x330);
        frame.save_fpr(c, 21, 0x3d8);
        frame.save_fpr(c, 20, 0x3d0);
        let point_slot = frame.at(c, FIND_NEAR_AI_POINT_SLOT);
        c.sw(point_slot, c.g[A1][0]);
        c.g[V0][0] = Addr((c.g[S4][0] as u32).wrapping_add(PATH_DATA_AI_COUNT)).lw(c);
        c.g[V1][0] = ((c.g[V0][0] as i64) < (c.g[A3][0] as i64)) as u64;
        // limit = min(count, max)
        c.g[S6][0] = if c.g[V1][0] == 0 {
            c.g[A3][0]
        } else {
            c.g[V0][0]
        };

        if (c.g[V0][0] as i64) > 0 {
            c.cycles += 6;
            c.lwc1(20, (c.g[GP][0] as u32).wrapping_sub(0x4a4c)); // "far"
            c.g[S1][0] = c.g[SP][0]; // distance slot
            c.mtc1(0, 21); // 0.0
            c.g[S3][0] = 0; // path offset
            c.g[S5][0] = c.g[S7][0]; // out cursor
            loop {
                c.cycles += 6;
                c.g[A0][0] = Addr((c.g[S4][0] as u32).wrapping_add(PATH_DATA_AI_PATHS)).lw(c);
                c.g[A1][0] = Addr(sp(c).wrapping_add(FIND_NEAR_AI_POINT_SLOT)).lw(c);
                c.g[A0][0] = find_near_ai_add(c.g[A0][0], c.g[S3][0] as u32);
                Callee {
                    addr: 0x0026_a9b0,
                    f: ee::s07::f_0026a9b0,
                }
                .call(c, 0x0026_b040)?;
                c.swc1(0, c.g[S1][0] as u32);
                if c.g[FP][0] != 0 {
                    c.cycles += 5;
                    c.g[V0][0] = Addr((c.g[S4][0] as u32).wrapping_add(PATH_DATA_AI_PATHS)).lw(c);
                    c.g[V0][0] = find_near_ai_add(c.g[S3][0], c.g[V0][0] as u32);
                    c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(AI_PATH_ENABLED)).lw(c);
                    if c.g[V1][0] == 0 {
                        c.swc1(20, c.g[S1][0] as u32);
                    }
                }
                c.cycles += 5;
                c.lwc1(0, c.g[S1][0] as u32);
                c.c_eq_s(0, 21);
                if c.fcc() {
                    // Inside the bounds: take it now.
                    c.cycles += 7;
                    c.g[V0][0] = Addr((c.g[S4][0] as u32).wrapping_add(PATH_DATA_AI_PATHS)).lw(c);
                    c.g[S0][0] = find_near_ai_add(c.g[S0][0], 1);
                    c.swc1(20, c.g[S1][0] as u32);
                    c.g[V0][0] = find_near_ai_add(c.g[V0][0], c.g[S3][0] as u32);
                    c.sw(c.g[S5][0] as u32, c.g[V0][0]);
                    let full = c.g[S0][0] == c.g[S6][0];
                    c.g[S5][0] = find_near_ai_add(c.g[S5][0], 4);
                    if full {
                        break;
                    }
                    c.cycles += 1;
                }
                c.g[V0][0] = Addr((c.g[S4][0] as u32).wrapping_add(PATH_DATA_AI_COUNT)).lw(c);
                c.cycles += 5;
                c.g[S2][0] = find_near_ai_add(c.g[S2][0], 1);
                c.g[S1][0] = find_near_ai_add(c.g[S1][0], 4);
                c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[S3][0] = find_near_ai_add(c.g[S3][0], 0x40);
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0026_b030);
                }
            }
        }

        // Fill the rest with the nearest remaining paths.
        c.cycles += 3;
        c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S6][0] as i64)) as u64;
        let short = c.g[V0][0] != 0;
        c.g[V0][0] = c.g[S0][0];
        if short {
            c.cycles += 2;
            c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x4a48)); // "used"
            loop {
                c.cycles += 8;
                c.g[V1][0] = Addr((c.g[S4][0] as u32).wrapping_add(PATH_DATA_AI_COUNT)).lw(c);
                c.g[S2][0] = 1;
                c.g[A3][0] = find_near_ai_add(c.g[S0][0], 1);
                c.lwc1(1, sp(c)); // best = dist[0]
                c.g[A0][0] = 0; // best index
                c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V1][0] as i64)) as u64;
                c.g[A2][0] = sx((c.g[S0][0] as u32) << 2);
                if c.g[V0][0] != 0 {
                    c.cycles += 2;
                    c.g[A1][0] = c.g[V1][0];
                    c.g[V1][0] = sx(sp(c).wrapping_add(4));
                    loop {
                        c.cycles += 5;
                        c.lwc1(0, c.g[V1][0] as u32);
                        c.c_lt_s(0, 1);
                        if c.fcc() {
                            c.cycles += 3;
                            c.mov_s(1, 0);
                            c.g[A0][0] = c.g[S2][0];
                        }
                        c.g[S2][0] = find_near_ai_add(c.g[S2][0], 1);
                        c.cycles += 3;
                        c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[A1][0] as i64)) as u64;
                        c.g[V1][0] = find_near_ai_add(c.g[V1][0], 4);
                        if c.g[V0][0] == 0 {
                            break;
                        }
                        if event_due(c) {
                            deliver_event_here(c, 0x0026_b0e0);
                        }
                    }
                }
                c.cycles += 11;
                c.g[V0][0] = Addr((c.g[S4][0] as u32).wrapping_add(PATH_DATA_AI_PATHS)).lw(c);
                c.g[V1][0] = sx((c.g[A0][0] as u32) << 6);
                c.g[A0][0] = sx((c.g[A0][0] as u32) << 2);
                c.g[A2][0] = find_near_ai_add(c.g[A2][0], c.g[S7][0] as u32);
                c.g[V0][0] = find_near_ai_add(c.g[V0][0], c.g[V1][0] as u32);
                c.g[A1][0] = find_near_ai_add(c.g[SP][0], c.g[A0][0] as u32);
                c.sw(c.g[A2][0] as u32, c.g[V0][0]);
                c.g[S0][0] = c.g[A3][0];
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S6][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.swc1(2, c.g[A1][0] as u32);
                if !more {
                    c.cycles += 1;
                    c.g[V0][0] = c.g[S0][0];
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0026_b0b8);
                }
            }
        }

        c.cycles += 14;
        frame.restore(c, S1, 0x3b0);
        frame.restore(c, S0, 0x3c0);
        frame.restore(c, S2, 0x3a0);
        frame.restore(c, S3, 0x390);
        frame.restore(c, S4, 0x380);
        frame.restore(c, S5, 0x370);
        frame.restore(c, S6, 0x360);
        frame.restore(c, S7, 0x350);
        frame.restore(c, FP, 0x340);
        frame.restore_ra(c, 0x330);
        frame.restore_fpr(c, 21, 0x3d8);
        frame.restore_fpr(c, 20, 0x3d0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;

    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::ptr;

    use crate::difftest::fakes::FAKE;
    use crate::difftest::fakes::LOG_RESULTS;

    use crate::difftest::fill;
    use crate::difftest::put_vec;
    use crate::difftest::put_words;
    use crate::difftest::scramble;

    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::RA as SENTINEL;

    pub(crate) use crate::game::ai::ai::tests::strict;
    pub(crate) use crate::game::ai::ai::tests::uf as uf_3;
    pub(crate) use crate::game::ai::ai::tests::SEEDS as SEEDS_6;
    pub(crate) use crate::game::ai::control::handplantcontrol::tests::put;
    pub(crate) use crate::game::ai::control::handplantcontrol::tests::SEEDS_2;
    pub(crate) use crate::game::be::bestate2::tests::belib;
    pub(crate) use crate::game::be::bestate2::tests::w;
    pub(crate) use crate::game::be::bestate2::tests::SEEDS as SEEDS_4;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::tests::base_6;

    pub(crate) use crate::game::fe::feasyncfile::tests::alloc_base_4 as alloc_base_2;

    pub(crate) use crate::game::fe::feasyncfile::tests::base_6 as base_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::diff;

    pub(crate) use crate::game::fe::feasyncfile::tests::globals_12 as globals_3;

    pub(crate) use crate::game::fe::festateloadhint::tests::w32;
    pub(crate) use crate::game::fe::festateloadhint::tests::SEEDS_2 as SEEDS_5;

    pub(crate) use crate::game::fe::festateoptions::tests::setup;
    pub(crate) use crate::game::fe::festateoptions::tests::AUX;
    pub(crate) use crate::game::fe::ovtemplatepausemenu::tests::setup as setup_2;
    pub(crate) use crate::game::fe::ovtemplatepausemenu::tests::EMPTY;

    pub(crate) use crate::game::movie::movieplayer::tests::globals;

    pub(crate) use crate::game::render::particle::tests::base;
    pub(crate) use crate::game::render::particle::tests::f;
    pub(crate) use crate::game::render::particle::tests::gp_at;
    pub(crate) use crate::game::render::particle::tests::put_vec3;
    pub(crate) use crate::game::render::particle::tests::uf;
    pub(crate) use crate::game::render::particle::tests::SEEDS;
    pub(crate) use crate::game::sound::bankmonitor::tests::SEEDS_4 as SEEDS_3;
    use ssx3_ee as ee;

    #[test]
    fn path_bounds_distance_matches() {
        check(
            ssx3_ee::s07::f_0026a9b0,
            path_bounds_distance,
            300,
            &|c, seed| {
                let mut r = Rng(0x0c13_0006 ^ seed as u64);
                scramble(c, &mut r);
                let point = DATA;
                let bounds = DATA + 0x100;
                c.g[A0][0] = bounds as u64; // box
                c.g[A1][0] = point as u64; // point
                if seed % 3 == 0 {
                    // Degenerate box at 0 so the per-axis distance equals |d|;
                    // the table picks each of the four combine branches.
                    let d: [f32; 3] = match (seed / 3) % 8 {
                        0 => [2.0, 1.0, 3.0],
                        1 => [2.0, 1.0, 1.0],
                        2 => [1.0, 2.0, 5.0],
                        3 => [1.0, 3.0, 2.0],
                        4 => [-2.0, -1.0, -3.0],
                        5 => [-2.0, -1.0, -1.0],
                        6 => [1.0, -2.0, 5.0],
                        _ => [0.0, 3.0, 0.0],
                    };
                    for i in 0..3 {
                        c.bus.write32(point + i * 4, d[i as usize].to_bits());
                    }
                    for off in [0x1c, 0x20, 0x24, 0x28, 0x2c, 0x30] {
                        c.bus.write32(bounds + off, 0);
                    }
                } else {
                    for off in [0x1c, 0x20, 0x24, 0x28, 0x2c, 0x30] {
                        c.bus.write32(bounds + off, r.edge_f32());
                    }
                    for i in 0..3 {
                        c.bus.write32(point + i * 4, r.edge_f32());
                    }
                }
            },
        );
    }

    #[test]
    fn path_data_get_track_path_matches() {
        check_lohi(
            ssx3_ee::s07::f_0026af70,
            path_data_get_track_path,
            200,
            &|c, seed| {
                let mut r = Rng(0x0c13_0007 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
                c.bus.write32(DATA + 0x14, r.next_u32());
            },
        );
    }

    #[test]
    fn path_data_get_ai_path_matches() {
        check(
            ssx3_ee::s07::f_0026af88,
            path_data_get_ai_path,
            200,
            &|c, seed| {
                let mut r = Rng(0x0c13_0008 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = ((r.next_u32() as u64) << 32) | r.next_u32() as u64;
                c.bus.write32(DATA + 0xc, r.next_u32());
            },
        );
    }

    #[test]
    fn path_get_closest_point_from_matches() {
        check(
            ee::s07::f_0026a638__generated,
            path_get_closest_point_from,
            SEEDS,
            &|c, seed| {
                let mut r = base(c, seed, 0x26a6);
                let (pth, point, out, dist, cur, segs) = (
                    DATA,
                    DATA + 0x80,
                    DATA + 0x90,
                    DATA + 0xa0,
                    DATA + 0xc0,
                    DATA + 0x100,
                );
                let n = 1 + r.next_u32() % 5;
                c.bus.write32(pth + 8, n);
                put_vec3(c, &mut r, pth + 0xc, -10.0, 10.0);
                c.bus.write32(pth + 0x18, segs);
                for i in 0..n {
                    let s = segs + 0x10 * i;
                    put_vec3(c, &mut r, s, -1.0, 1.0);
                    c.bus.write32(s + 0xc, f(uf(&mut r, 0.5, 10.0)));
                }
                put_vec3(c, &mut r, point, -20.0, 20.0);
                c.bus.write32(point + 0xc, f(uf(&mut r, -1.0, 1.0)));
                if seed % 3 == 0 {
                    c.bus.write32(cur + 0x14, u32::MAX);
                } else {
                    put_vec3(c, &mut r, cur, -10.0, 10.0);
                    c.bus.write32(cur + 0xc, f(1.0));
                    c.bus.write32(cur + 0x10, f(uf(&mut r, 0.0, 20.0)));
                    c.bus.write32(cur + 0x14, r.next_u32() % n);
                }
                c.bus.write32(gp_at(-0x4a54), f(1.0e30));
                c.bus.write32(gp_at(-0x4a50), f(uf(&mut r, 0.0, 15.0)));
                c.g[A0][0] = sx(pth);
                c.g[A1][0] = sx(point);
                c.g[A2][0] = sx(out);
                c.g[A3][0] = sx(dist);
                c.g[T0][0] = sx(cur);
                c.g[T1][0] = (seed / 3 % 2) as u64;
            },
        );
    }

    pub(crate) fn uf_2(r: &mut Rng, lo: f32, hi: f32) -> u32 {
        f(lo + (r.next_u32() as f32 / u32::MAX as f32) * (hi - lo))
    }

    // Path_QueryEvents -----------------------------------------------------

    pub(crate) const RANGE_LIST: u32 = DATA;

    pub(crate) const RANGES: u32 = DATA + 0x400;

    pub(crate) const RANGE_OUT: u32 = DATA + 0x800;

    #[test]
    fn path_query_events_matches() {
        check(
            ssx3_ee::s07::f_0026aa80__generated,
            path_query_events,
            200,
            &|c, seed| {
                let mut r = Rng(0x0026_aa80 ^ seed as u64);
                scramble(c, &mut r);
                let count = (seed % 5) as u32; // 0..4
                c.g[A0][0] = RANGE_LIST as u64;
                c.g[A1][0] = RANGE_OUT as u64;
                c.g[A2][0] = (seed % 3) as u64; // max
                c.bus.write32(RANGE_LIST, count);
                c.bus.write32(RANGE_LIST + 4, RANGES);
                for i in 0..5 {
                    let lo = f32::from_bits(uf_2(&mut r, -20.0, 20.0));
                    let hi = lo + if seed % 4 == 0 { -5.0 } else { 3.0 };
                    c.bus.write32(RANGES + 16 * i, r.next_u32());
                    c.bus.write32(RANGES + 16 * i + 4, r.next_u32());
                    c.bus.write32(RANGES + 16 * i + 8, f(lo));
                    c.bus.write32(RANGES + 16 * i + 0xc, f(hi));
                }
                c.f[12] = uf_2(&mut r, -10.0, 10.0);
                c.f[13] = if seed % 7 == 0 {
                    f(-200.0)
                } else {
                    uf_2(&mut r, -10.0, 10.0)
                };
            },
        );
    }

    // Path_GetPointAtDistance ---------------------------------------------

    pub(crate) const TRACK: u32 = DATA;

    pub(crate) const TRACK_SEGS: u32 = DATA + 0x400;

    pub(crate) const POINT: u32 = DATA + 0x800;

    pub(crate) const IN_RANGE: u32 = DATA + 0x900;

    pub(crate) fn put_track(c: &mut Cpu, r: &mut Rng, count: u32) {
        c.bus.write32(TRACK + 8, count);
        c.bus.write32(TRACK + 0xc, r.f32(100.0));
        c.bus.write32(TRACK + 0x10, r.f32(100.0));
        c.bus.write32(TRACK + 0x14, r.f32(100.0));
        c.bus.write32(TRACK + 0x18, TRACK_SEGS);
        for i in 0..4 {
            put_vec(
                c,
                TRACK_SEGS + 16 * i,
                [r.f32(2.0), r.f32(2.0), r.f32(2.0), uf_2(r, 10.0, 60.0)],
            );
        }
    }

    #[test]
    fn path_get_point_at_distance_matches() {
        check(
            ssx3_ee::s07::f_0026ab20__generated,
            path_get_point_at_distance,
            200,
            &|c, seed| {
                let mut r = Rng(0x0026_ab20 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = POINT as u64;
                c.g[A1][0] = TRACK as u64;
                c.g[A2][0] = if seed % 5 == 0 { 0 } else { IN_RANGE as u64 };
                put_track(c, &mut r, (seed % 4) as u32);
                // Inside the track half the time, past its end otherwise.
                c.f[12] = if seed % 2 == 0 {
                    uf_2(&mut r, 0.0, 120.0)
                } else {
                    uf_2(&mut r, 200.0, 400.0)
                };
            },
        );
    }

    // ------------------------------------------------------------------- paths

    #[test]
    fn path_query_events_reversed_matches() {
        check(
            ee::s07::f_0026a090__generated,
            path_query_events_reversed,
            SEEDS_2,
            &|c, seed| {
                let mut r = Rng(0x0026_a090 ^ seed as u64);
                scramble(c, &mut r);
                let path = DATA + 0x1000;
                c.g[A0][0] = path as u64;
                c.g[A1][0] = (DATA + 0x3000) as u64; // out
                c.g[A2][0] = (seed % 4) as u64; // maxOut
                let count = seed % 4;
                c.bus.write32(path + 0x0, count);
                c.bus.write32(path + 0x4, DATA + 0x2000);
                // base 100, window [base-100, base-99] = [0, 1].
                c.bus.write32(path + 0x38, 100.0f32.to_bits());
                c.f[12] = 100.0f32.to_bits();
                c.f[13] = 99.0f32.to_bits();
                for i in 0..count {
                    let e = DATA + 0x2000 + 16 * i;
                    c.bus.write32(e + 8, 0.5f32.to_bits());
                    c.bus.write32(e + 0xc, 0.75f32.to_bits());
                }
                let _ = r.next_u32();
            },
        );
    }

    #[test]
    fn path_read_track_field_matches() {
        check(
            ee::s07::f_0026a070__generated,
            path_read_track_field,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x26_a070, 0);
                c.g[A0][0] = DATA as u64;
                // Half the seeds use the very same object, skipping the copy.
                c.g[A1][0] = if seed % 2 == 0 {
                    (DATA + 0x38) as u64
                } else {
                    (DATA + 0x800) as u64
                };
                c.g[A2][0] = if seed % 3 == 0 { 1 } else { 0 };
                c.bus.write32(DATA + 0x800, r.next_u32());
            },
        );
    }

    /// The lazily built BELibrary singleton (`gp - 0x1f04`): a zeroed dummy so
    /// the file/allocator paths fail fast instead of waiting on the disc.
    pub(crate) fn be_library(c: &mut Cpu) {
        c.bus
            .write32(GAME_GP.wrapping_sub(0x1f04), DATA + 0x30_0000);
    }

    /// One lobby-thunk test: the handler's `a2` points at a zeroed object, the
    /// callback table slot holds a small offset (so the `lh` + add is
    /// exercised) and `a1` a zeroed packet.
    pub(crate) fn lobby_setup(c: &mut Cpu, seed: u32, salt: u64, slot: u32) {
        let mut r = machine(c, seed, salt, 2);
        be_library(c);
        c.g[A1][0] = sx(DATA + 0x400);
        c.g[A2][0] = sx(DATA + 0x10);
        c.bus.write16(
            LOBBY_CALLBACKS.wrapping_add(slot),
            if seed % 3 == 0 {
                0
            } else {
                (4 * (r.next_u32() % 8)) as u16
            },
        );
        // The packet `LobbyApi_DebugPacket`/`TagField*` read: zeroed fields.
        for k in 0..8u32 {
            c.bus.write32(DATA + 0x400 + 4 * k, 0);
        }
    }

    /// The engine's size-class pool allocator (`MEM_PoolAlloc`): blocks up to
    /// 0x400 bytes are popped from per-size free lists hanging off
    /// `gp + 0x2a60..0x2a84`, but only when the "pools ready" flag at
    /// `gp + 0xdd8` is set. On a fresh test machine that flag is clear, so an
    /// allocation grows the `DLHeap` from a zeroed header and the reply
    /// handlers — which allocate a message node on every call — then walk a
    /// corrupt bucket chain and never terminate. A short free list per size
    /// class makes each allocation pop a node instead, as it does in the game.
    pub(crate) fn pool_allocator(c: &mut Cpu) {
        c.bus.write32(GAME_GP + 0xdd8, 1); // pools ready
                                           // (pool pointer global, free-list head offset within the pool object)
        const POOLS: [(u32, u32); 7] = [
            (0x2a70, 0x800),   // 8-byte blocks
            (0x2a60, 0x2000),  // 16
            (0x2a6c, 0x32000), // 32
            (0x2a7c, 0x25800), // 64
            (0x2a84, 0x25800), // 128
            (0x2a78, 0x20000), // 256
            (0x2a68, 0x20000), // 512
        ];
        let nodes = DATA + 0x28_000;
        for (i, (slot, head)) in POOLS.into_iter().enumerate() {
            let pool = DATA + 0x20_000 + 0x100 * i as u32;
            let base = nodes + 0x100 * i as u32;
            // Four free nodes per size, spaced wider than the largest block of
            // that size class (the head is updated on pop, so only the next
            // pointer of an unpopped node matters).
            for n in 0..4u32 {
                let node = base + 0x40 * n;
                c.bus.write32(node, if n == 3 { 0 } else { node + 0x40 });
            }
            c.bus.write32(pool + head, base);
            c.bus.write32(GAME_GP + slot, pool);
        }
    }

    /// [`lobby_setup`] plus the pool allocator: for the reply handlers that
    /// allocate a message node on every call.
    pub(crate) fn lobby_alloc_setup(c: &mut Cpu, seed: u32, salt: u64, slot: u32) {
        lobby_setup(c, seed, salt, slot);
        pool_allocator(c);
    }

    #[test]
    fn lobby_man_on_user_list_changed_thunk_matches() {
        crate::difftest::check(
            ee::s07::f_00268a90__generated,
            lobby_man_on_user_list_changed_thunk,
            30,
            &|c, seed| {
                lobby_alloc_setup(c, seed, 0x268a_90, 0x2fb0);
            },
        );
    }

    #[test]
    fn lobby_man_on_room_list_changed_thunk_matches() {
        crate::difftest::check(
            ee::s07::f_00268ab8__generated,
            lobby_man_on_room_list_changed_thunk,
            30,
            &|c, seed| {
                lobby_setup(c, seed, 0x268a_b8, 0x2fb8);
            },
        );
    }

    #[test]
    fn lobby_man_on_rank_list_changed_thunk_matches() {
        crate::difftest::check(
            ee::s07::f_00268ae0__generated,
            lobby_man_on_rank_list_changed_thunk,
            30,
            &|c, seed| {
                lobby_setup(c, seed, 0x268a_e0, 0x2fc0);
            },
        );
    }

    #[test]
    fn lobby_man_on_recv_debug_thunk_matches() {
        crate::difftest::check(
            ee::s07::f_00268b08__generated,
            lobby_man_on_recv_debug_thunk,
            30,
            &|c, seed| {
                lobby_setup(c, seed, 0x268b_08, 0x2ff0);
            },
        );
    }

    #[test]
    fn lobby_man_on_message_thunk_matches() {
        crate::difftest::check(
            ee::s07::f_00268b30__generated,
            lobby_man_on_message_thunk,
            24,
            &|c, seed| {
                lobby_setup(c, seed, 0x268b_30, 0x2ff8);
            },
        );
    }

    #[test]
    fn lobby_man_on_disconnect_thunk_matches() {
        crate::difftest::check(
            ee::s07::f_00268b58__generated,
            lobby_man_on_disconnect_thunk,
            30,
            &|c, seed| {
                lobby_setup(c, seed, 0x268b_58, 0x3000);
            },
        );
    }

    #[test]
    fn lobby_man_on_game_start_thunk_matches() {
        crate::difftest::check(
            ee::s07::f_00268b80__generated,
            lobby_man_on_game_start_thunk,
            24,
            &|c, seed| {
                lobby_alloc_setup(c, seed, 0x268b_80, 0x3008);
            },
        );
    }

    #[test]
    fn lobby_man_on_connect_reply_thunk_matches() {
        crate::difftest::check(
            ee::s07::f_00268ba8__generated,
            lobby_man_on_connect_reply_thunk,
            30,
            &|c, seed| {
                lobby_setup(c, seed, 0x268b_a8, 0x3020);
            },
        );
    }

    #[test]
    fn lobby_man_on_create_account_reply_thunk_matches() {
        crate::difftest::check(
            ee::s07::f_00268bd0__generated,
            lobby_man_on_create_account_reply_thunk,
            30,
            &|c, seed| {
                lobby_alloc_setup(c, seed, 0x268b_d0, 0x3088);
            },
        );
    }

    #[test]
    fn lobby_man_on_edit_account_reply_thunk_matches() {
        crate::difftest::check(
            ee::s07::f_00268bf8__generated,
            lobby_man_on_edit_account_reply_thunk,
            30,
            &|c, seed| {
                lobby_alloc_setup(c, seed, 0x268b_f8, 0x30a8);
            },
        );
    }

    #[test]
    fn lobby_man_on_lost_by_email_reply_thunk_matches() {
        crate::difftest::check(
            ee::s07::f_00268c20__generated,
            lobby_man_on_lost_by_email_reply_thunk,
            30,
            &|c, seed| {
                lobby_alloc_setup(c, seed, 0x268c_20, 0x30b0);
            },
        );
    }

    #[test]
    fn lobby_man_on_lost_by_name_reply_thunk_matches() {
        crate::difftest::check(
            ee::s07::f_00268c48__generated,
            lobby_man_on_lost_by_name_reply_thunk,
            30,
            &|c, seed| {
                lobby_alloc_setup(c, seed, 0x268c_48, 0x30b8);
            },
        );
    }

    #[test]
    fn lobby_man_on_login_reply_thunk_matches() {
        crate::difftest::check(
            ee::s07::f_00268c70__generated,
            lobby_man_on_login_reply_thunk,
            24,
            &|c, seed| {
                lobby_setup(c, seed, 0x268c_70, 0x30e8);
            },
        );
    }

    #[test]
    fn lobby_man_on_select_persona_reply_thunk_matches() {
        crate::difftest::check(
            ee::s07::f_00268c98__generated,
            lobby_man_on_select_persona_reply_thunk,
            30,
            &|c, seed| {
                lobby_alloc_setup(c, seed, 0x268c_98, 0x3118);
            },
        );
    }

    #[test]
    fn lobby_man_on_own_stats_reply_thunk_matches() {
        crate::difftest::check(
            ee::s07::f_00268cc0__generated,
            lobby_man_on_own_stats_reply_thunk,
            30,
            &|c, seed| {
                lobby_setup(c, seed, 0x268c_c0, 0x3120);
            },
        );
    }

    #[test]
    fn lobby_man_on_create_persona_reply_thunk_matches() {
        crate::difftest::check(
            ee::s07::f_00268ce8__generated,
            lobby_man_on_create_persona_reply_thunk,
            30,
            &|c, seed| {
                lobby_alloc_setup(c, seed, 0x268c_e8, 0x3138);
            },
        );
    }

    #[test]
    fn lobby_man_on_delete_persona_reply_thunk_matches() {
        crate::difftest::check(
            ee::s07::f_00268d10__generated,
            lobby_man_on_delete_persona_reply_thunk,
            30,
            &|c, seed| {
                lobby_setup(c, seed, 0x268d_10, 0x3140);
            },
        );
    }

    /// `this`/`self` scratch object, message object and other fixtures all
    /// live in the (unused) data area the differential tests own.
    pub(crate) const SCRATCH: u32 = DATA + 0x1000;

    /// The game singletons a real callee may reach: a zeroed world at
    /// `gp - 0x848` (`+0x84` module, `+0xc` race manager, no riders at `+0x7c`)
    /// and a zeroed BE library at `gp - 0x1f04`, so nothing tries to build one
    /// through the file system or walk a rider loop.
    pub(crate) fn game_globals(c: &mut Cpu) {
        let gp = crate::difftest::GAME_GP;
        let world = SCRATCH + 0x9000;
        let module = SCRATCH + 0x9100;
        let race = SCRATCH + 0x9200;
        c.bus.write32(gp - 0x848, world);
        c.bus.write32(world + 0x84, module);
        c.bus.write32(module + 0xc, race);
        c.bus.write32(race + 0x7c, 0);
        c.bus.write32(gp - 0x1f04, SCRATCH + 0x9300);
        c.bus.write32(gp - 0x1efc, 0);
    }

    /// [`machine`] plus the game singletons (most callees read them).
    pub(crate) fn game(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        let r = machine(c, seed, salt, calls);
        game_globals(c);
        r
    }

    /// A fixture for the message-map thunks: a fake-call machine (so the
    /// handler's own indirect calls and any unwind are compared), a reply
    /// message in `a1`, the object base in `a2`, and a small member offset in
    /// the message map.
    pub(crate) fn thunk_setup(c: &mut Cpu, seed: u32, salt: u64, entry: u32) {
        let mut r = game(c, seed, salt, 1);
        let message = SCRATCH + 0x7000;
        // Half the seeds hand the handler a null message: the reply handlers
        // take their "nothing arrived" path then.
        c.g[A1][0] = if seed % 2 == 0 { 0 } else { message as u64 };
        c.g[A2][0] = SCRATCH as u64;
        c.g[A0][0] = ptr(&mut r) as u64;
        c.bus
            .write32(message + 8, if seed % 4 == 0 { r.next_u32() } else { 0 });
        c.bus.write32(
            message + 0xc,
            if seed % 4 == 1 { r.next_u32() % 8 } else { 0 },
        );
        // The member offset the thunk adds to the object base (positive, so
        // the adjusted `this` stays inside the scratch area).
        c.bus.write16(entry, (r.next_u32() & 0xff8) as u16);
    }

    /// The engine's size-class pool allocator (`MEM_PoolAlloc`): blocks up to
    /// 0x400 bytes are popped from per-size free lists hanging off the seven
    /// pool globals `gp + 0x2a60..0x2a84`, but only when the "pools ready" flag
    /// at `gp + 0xdd8` is set. On a fresh test machine that flag is clear, so an
    /// allocation falls back to growing the `DLHeap` from a zeroed header; the
    /// reply handlers that allocate a list node then walk a corrupt bucket
    /// chain in `DLHeap_Consolidate` and never terminate. Give every pool a
    /// short free list of zeroed nodes instead: the handlers' allocations then
    /// just pop one, exactly as they do in the game (each `MEM_Pool*Alloc` is
    /// two loads and two stores off `pool + head`).
    pub(crate) fn pool_allocator_2(c: &mut Cpu) {
        let gp = crate::difftest::GAME_GP;
        c.bus.write32(gp + 0xdd8, 1); // pools ready
                                      // (pool pointer global, free-list head offset in the pool object) for
                                      // the 8, 16, 32, 64, 128, 256 and 512-byte size classes.
        const POOLS: [(u32, u32); 7] = [
            (0x2a70, 0x800),
            (0x2a60, 0x2000),
            (0x2a6c, 0x6000),
            (0x2a7c, 0x32000),
            (0x2a84, 0x25800),
            (0x2a78, 0x20000),
            (0x2a68, 0x20000),
        ];
        for (i, (slot, head)) in POOLS.into_iter().enumerate() {
            let i = i as u32;
            let pool = 0x008c_0000 + 0x1000 * i;
            let base = 0x008b_0000 + 0x100 * i;
            // Four free nodes per class, spaced wider than the largest block
            // of that class (the head is updated on pop, so only the next
            // pointer of a node that is still queued is ever read).
            for n in 0..4u32 {
                let node = base + 0x40 * n;
                c.bus.write32(node, if n == 3 { 0 } else { node + 0x40 });
            }
            c.bus.write32(pool + head, base);
            c.bus.write32(gp + slot, pool);
        }
    }

    /// [`thunk_setup`] plus the pool allocator: for the reply handlers that
    /// allocate on every call.
    pub(crate) fn thunk_setup_alloc(c: &mut Cpu, seed: u32, salt: u64, entry: u32) {
        thunk_setup(c, seed, salt, entry);
        pool_allocator_2(c);
    }

    #[test]
    fn lobby_man_on_move_to_room_reply_thunk_matches() {
        check(
            ee::s07::f_00268d38__generated,
            lobby_man_on_move_to_room_reply_thunk,
            48,
            &|c, seed| {
                thunk_setup(c, seed, 0x268d38, 0x004a_3148);
            },
        );
    }

    #[test]
    fn lobby_man_on_create_room_reply_thunk_matches() {
        check(
            ee::s07::f_00268d60__generated,
            lobby_man_on_create_room_reply_thunk,
            48,
            &|c, seed| {
                thunk_setup(c, seed, 0x268d60, 0x004a_3158);
            },
        );
    }

    #[test]
    fn lobby_man_on_ping_reply_thunk_matches() {
        check(
            ee::s07::f_00268d88__generated,
            lobby_man_on_ping_reply_thunk,
            48,
            &|c, seed| {
                thunk_setup(c, seed, 0x268d88, 0x004a_3160);
            },
        );
    }

    #[test]
    fn lobby_man_on_report_user_reply_thunk_matches() {
        check(
            ee::s07::f_00268db0__generated,
            lobby_man_on_report_user_reply_thunk,
            48,
            &|c, seed| {
                thunk_setup(c, seed, 0x268db0, 0x004a_3170);
            },
        );
    }

    #[test]
    fn lobby_man_on_news_reply_thunk_matches() {
        check(
            ee::s07::f_00268dd8__generated,
            lobby_man_on_news_reply_thunk,
            48,
            &|c, seed| {
                thunk_setup(c, seed, 0x268dd8, 0x004a_3178);
            },
        );
    }

    #[test]
    fn lobby_man_on_snapshot_reply_thunk_matches() {
        check(
            ee::s07::f_00268e00__generated,
            lobby_man_on_snapshot_reply_thunk,
            48,
            &|c, seed| {
                thunk_setup(c, seed, 0x268e00, 0x004a_31b0);
            },
        );
    }

    #[test]
    fn lobby_man_on_lookup_user_reply_thunk_matches() {
        check(
            ee::s07::f_00268e28__generated,
            lobby_man_on_lookup_user_reply_thunk,
            48,
            &|c, seed| {
                thunk_setup_alloc(c, seed, 0x268e28, 0x004a_31b8);
            },
        );
    }

    #[test]
    fn lobby_man_on_quick_play_reply_thunk_matches() {
        check(
            ee::s07::f_00268e78__generated,
            lobby_man_on_quick_play_reply_thunk,
            48,
            &|c, seed| {
                thunk_setup(c, seed, 0x268e78, 0x004a_3210);
            },
        );
    }

    #[test]
    fn lobby_man_on_chal_reply_thunk_matches() {
        check(
            ee::s07::f_00268ea0__generated,
            lobby_man_on_chal_reply_thunk,
            48,
            &|c, seed| {
                thunk_setup(c, seed, 0x268ea0, 0x004a_3248);
            },
        );
    }

    #[test]
    fn lobby_man_on_mesg_reply_thunk_matches() {
        check(
            ee::s07::f_00268ec8__generated,
            lobby_man_on_mesg_reply_thunk,
            48,
            &|c, seed| {
                thunk_setup(c, seed, 0x268ec8, 0x004a_3268);
            },
        );
    }

    #[test]
    fn lobby_man_on_rank_reply_thunk_matches() {
        check(
            ee::s07::f_00268ef0__generated,
            lobby_man_on_rank_reply_thunk,
            48,
            &|c, seed| {
                thunk_setup(c, seed, 0x268ef0, 0x004a_3318);
            },
        );
    }

    #[test]
    fn buddy_man_on_connect_event_thunk_matches() {
        check(
            ee::s07::f_00268f18__generated,
            buddy_man_on_connect_event_thunk,
            48,
            &|c, seed| {
                thunk_setup(c, seed, 0x268f18, 0x004a_3340);
            },
        );
    }

    #[test]
    fn buddy_man_on_message_received_thunk_matches() {
        check(
            ee::s07::f_00268f40__generated,
            buddy_man_on_message_received_thunk,
            48,
            &|c, seed| {
                thunk_setup(c, seed, 0x268f40, 0x004a_3348);
            },
        );
    }

    #[test]
    fn buddy_man_on_roster_event_thunk_matches() {
        check(
            ee::s07::f_00268f68__generated,
            buddy_man_on_roster_event_thunk,
            48,
            &|c, seed| {
                thunk_setup_alloc(c, seed, 0x268f68, 0x004a_3350);
            },
        );
    }

    #[test]
    fn buddy_man_on_roster_remove_reply_thunk_matches() {
        check(
            ee::s07::f_00268f90__generated,
            buddy_man_on_roster_remove_reply_thunk,
            48,
            &|c, seed| {
                thunk_setup_alloc(c, seed, 0x268f90, 0x004a_3380);
            },
        );
    }

    #[test]
    fn buddy_man_on_roster_add_reply_thunk_matches() {
        check(
            ee::s07::f_00268fb8__generated,
            buddy_man_on_roster_add_reply_thunk,
            48,
            &|c, seed| {
                thunk_setup_alloc(c, seed, 0x268fb8, 0x004a_3388);
            },
        );
    }

    #[test]
    fn buddy_man_on_send_message_reply_thunk_matches() {
        check(
            ee::s07::f_00268fe0__generated,
            buddy_man_on_send_message_reply_thunk,
            48,
            &|c, seed| {
                thunk_setup(c, seed, 0x268fe0, 0x004a_33a0);
            },
        );
    }

    /// The memory-card/setup heap mutex the allocator waits on.
    pub(crate) const MEM_MUTEX: u32 = 0x0053_8b00;

    /// The allocator's mutex looks held by this thread, so `EA_MemAllocNamed`
    /// takes its fast path (the `dispatch.rs` tests' recipe).
    pub(crate) fn alloc_mutex(c: &mut Cpu, r: &mut Rng) {
        c.bus.write32(MEM_MUTEX + 4, 0x42);
        c.bus.write32(LOG_RESULTS, 0x42);
        c.bus.write32(MEM_MUTEX + 8, r.next_u32() % 4);
    }

    #[test]
    fn path_data_set_buffer_matches() {
        check(
            ssx3_ee::s07::f_0026ad70__generated,
            path_data_set_buffer,
            20,
            &|c, seed| {
                let mut r = machine(c, seed, 0xc6, 6);
                globals(c);
                alloc_mutex(c, &mut r);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x4000);
                // A zeroed blob parses as empty sections (no allocation loops).
                c.bus.write32(DATA + 0x48, DATA + 0x4000);
                c.bus.write32(DATA + 0x4000, 0);
            },
        );
    }

    // ------------------------------------------------------------------
    // Path readers.

    #[test]
    fn path_read_a_i_field_matches() {
        check(
            ee::s07::f_00269ed8__generated,
            path_read_a_i_field,
            SEEDS_3,
            &|c, seed| {
                let mut r = machine(c, seed, 0x269ed8, 0);
                let dst = DATA + 0x200;
                c.g[A0][0] = dst as u64;
                let id = match seed % 3 {
                    0 => 0x64u64,
                    1 => 0x65,
                    _ => r.next_u32() as u64 | 0x100,
                };
                c.g[A2][0] = id;
                c.g[A1][0] = if seed % 5 == 0 {
                    (DATA + 0x400) as u64
                } else {
                    (dst + 0x38) as u64
                };
            },
        );
    }

    #[test]
    fn path_read_segment_matches() {
        check(
            ee::s07::f_0026a338__generated,
            path_read_segment,
            SEEDS_3,
            &|c, seed| {
                let mut r = machine(c, seed, 0x26a338, 0);
                let src = DATA + 0x300;
                let dst = if seed % 3 == 0 { src } else { DATA + 0x400 };
                c.g[A1][0] = src as u64;
                c.g[A2][0] = dst as u64;
                for k in 0..4u32 {
                    let v = r.next_u32();
                    c.bus.write32(src + 4 * k, v);
                }
            },
        );
    }

    /// The object under test.
    pub(crate) const W: u32 = DATA;

    /// Segment table for `Path_GetLength`.
    pub(crate) const SEGMENTS: u32 = AUX + 0x600;

    pub(crate) fn fbits(x: f32) -> u32 {
        x.to_bits()
    }

    // ------------------------------------------------------------------
    // Path events.

    #[test]
    fn path_has_event_in_range_reversed_matches() {
        check(
            ee::s07::f_0026a0b8__generated,
            path_has_event_in_range_reversed,
            200,
            &|c, seed| {
                let mut r = setup(c, seed, 7);
                c.g[A0][0] = W as u64;
                put(c, W + 0x38, r.f32(10.0));
                let neg = |x: u32| x | 0x8000_0000;
                c.f[12] = if seed % 3 == 0 {
                    neg(r.f32(2.0))
                } else {
                    r.f32(2.0)
                };
                c.f[13] = if seed % 4 == 0 {
                    neg(r.f32(2.0))
                } else {
                    r.f32(2.0)
                };
                // `Path_HasEventInRange` reads the count at +0 and 0x10-byte
                // items from +4; `Path_GetLength` the count at +8 and 0x10-byte
                // segments (length at +0xc) from the pointer at +0x18.
                let count = seed % 3;
                for i in 0..count {
                    let item = W + 4 + 0x10 * i;
                    put(
                        c,
                        item,
                        match (seed + i) % 3 {
                            0 => 0x14,
                            1 => 0x12,
                            _ => 0,
                        },
                    );
                    put(c, item + 8, r.f32(2.0));
                    put(c, item + 0xc, r.f32(2.0));
                    put(c, SEGMENTS + 0x10 * i + 0xc, r.f32(2.0));
                }
                // The same object is both the segment list (`Path_GetLength`
                // reads the count at +8 and the segments at +0x18) and the item
                // list, so set those two words last (`count` is tiny).
                put(c, W, count);
                put(c, W + 8, count);
                put(c, W + 0x18, SEGMENTS);
            },
        );
    }

    #[test]
    fn path_has_event_in_range_matches() {
        check(
            ee::s07::f_0026ac88__generated,
            path_has_event_in_range,
            250,
            &|c, seed| {
                let mut r = setup(c, seed, 8);
                c.g[A0][0] = W as u64;
                c.f[12] = r.f32(3.0);
                c.f[13] = r.f32(3.0);
                let count = seed % 4;
                put(c, W, count);
                for i in 0..count {
                    let item = W + 4 + 0x10 * i;
                    put(
                        c,
                        item,
                        match (seed + i) % 4 {
                            0 => 0x14,
                            1 => 0x12,
                            _ => 0,
                        },
                    );
                    put(
                        c,
                        item + 8,
                        if (seed + i) % 5 == 0 {
                            fbits(10.0)
                        } else {
                            r.f32(2.0)
                        },
                    );
                    put(
                        c,
                        item + 0xc,
                        if (seed + i) % 7 == 0 {
                            fbits(-10.0)
                        } else {
                            r.f32(2.0)
                        },
                    );
                }
            },
        );
    }

    #[test]
    fn net_event_fd_clone_matches() {
        check(
            ee::s07::f_00269dc0__generated,
            net_event_fd_clone,
            SEEDS_4,
            &|c, seed| {
                machine(c, seed, 0x0026_9dc0, 1);
                belib(c);
                c.g[A0][0] = DATA as u64;
                w(c, DATA + 8, seed.wrapping_mul(0x1234_5678));
                pool_allocator_3(c);
            },
        );
    }

    #[test]
    fn net_event_fe_clone_matches() {
        check(
            ee::s07::f_00269e40__generated,
            net_event_fe_clone,
            SEEDS_4,
            &|c, seed| {
                machine(c, seed, 0x0026_9e40, 1);
                belib(c);
                c.g[A0][0] = DATA as u64;
                w(c, DATA + 8, seed.wrapping_mul(0x1234_5678));
                pool_allocator_3(c);
            },
        );
    }

    /// The engine's size-class pool allocator: blocks up to 0x400 bytes come
    /// from per-size free lists hanging off `gp + 0x2a60..0x2a84` once the
    /// "pools ready" flag at `gp + 0xdd8` is set. Without it an allocation
    /// grows the DLHeap from a zeroed header and can walk a corrupt bucket
    /// chain.
    pub(crate) fn pool_allocator_3(c: &mut Cpu) {
        c.bus.write32(GAME_GP + 0xdd8, 1);
        const POOLS: [(u32, u32); 7] = [
            (0x2a70, 0x800),   // 8-byte blocks
            (0x2a60, 0x2000),  // 16
            (0x2a6c, 0x32000), // 32
            (0x2a7c, 0x25800), // 64
            (0x2a84, 0x25800), // 128
            (0x2a78, 0x20000), // 256
            (0x2a68, 0x20000), // 512
        ];
        let nodes = DATA + 0x28_000;
        for (i, (slot, head)) in POOLS.into_iter().enumerate() {
            let pool = DATA + 0x20_000 + 0x100 * i as u32;
            let base = nodes + 0x100 * i as u32;
            for n in 0..4u32 {
                let node = base + 0x40 * n;
                w(c, node, if n == 3 { 0 } else { node + 0x40 });
            }
            w(c, pool + head, base);
            w(c, GAME_GP + slot, pool);
        }
    }

    // ------------------------------------------------------- challenger/string

    #[test]
    fn c_challenger_dtor_matches() {
        check(
            ssx3_ee::s07::f_00268950__generated,
            c_challenger_dtor,
            90,
            &|c, seed| {
                let _r = setup_2(c, seed, 0x268950);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = 0;
                c.bus.write32(DATA, EMPTY);
                if seed % 2 == 0 {
                    c.bus.write32(DATA + 4, EMPTY);
                } else {
                    // A shared (refcount 2) buffer: the dtor only drops the count.
                    c.bus.write32(DATA + 4, DATA + 0x40);
                    c.bus.write32(DATA + 0x34, 2);
                    c.bus.write32(DATA + 0x38, 0);
                }
            },
        );
    }

    // ------------------------------------------------------------ path/search

    #[test]
    fn path_get_dir_at_distance_matches() {
        check(
            ssx3_ee::s07::f_0026a8b8__generated,
            path_get_dir_at_distance,
            150,
            &|c, seed| {
                let mut r = setup_2(c, seed, 0x26a8b8);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x100) as u64;
                let n = 1 + seed % 3;
                c.bus.write32(DATA + 0x108, n);
                c.bus.write32(DATA + 0x118, DATA + 0x400);
                let lens = [1.0f32, 2.5, 6.0];
                for i in 0..n {
                    let e = DATA + 0x400 + i * 16;
                    c.bus.write32(e, r.next_u32());
                    c.bus.write32(e + 4, r.next_u32());
                    c.bus.write32(e + 8, r.next_u32());
                    c.bus.write32(e + 0xc, lens[(i % 3) as usize].to_bits());
                }
                c.f[12] = if seed % 4 == 0 {
                    20.0f32.to_bits()
                } else {
                    r.f32(6.0)
                };
            },
        );
    }

    #[test]
    fn path_data_free_matches() {
        check(
            ssx3_ee::s07::f_0026af00__generated,
            path_data_free,
            SEEDS_5,
            &|c, seed| {
                let _ = machine(c, seed, 0x26af00, 0);
                let d = DATA + 0x1000;
                c.g[A0][0] = d as u64;
                // No owned buffers: no frees (a free of an arbitrary pointer walks
                // the heap headers and can run away).
                w32(c, d + 0xc, 0);
                w32(c, d + 0x14, 0);
                w32(c, d + 0x48, if seed % 2 == 0 { 0 } else { DATA + 0x3000 });
                w32(c, d + 0x4c, 0);
                w32(c, DATA + 0x3000, 0);
            },
        );
    }

    #[test]
    fn path_data_parse_matches() {
        check(
            ee::s07::f_0026adf0__generated,
            path_data_parse,
            60,
            &|c, seed| {
                let _ = base_2(c, seed, 0x26ad, 3);
                w(c, DATA + 0x48, DATA + 0x1000);
                // The section buffer is zeroed, so every reader stops at once
                // (a random count walks billions of entries). Half the seeds give
                // the AI-path section one entry to run its loop once.
                w(c, DATA + 0x1004, (seed % 2) as u32);
                c.g[A0][0] = sx(DATA);
            },
        );
    }

    pub(crate) const W_2: u32 = DATA + 0x5_0000;

    pub(crate) const RM: u32 = DATA + 0x5_1000;

    pub(crate) const RIDERS: u32 = DATA + 0x5_2000;

    pub(crate) const BE_2: u32 = DATA + 0x5_3000;

    pub(crate) const DB: u32 = DATA + 0x5_4000;

    pub(crate) const VTS: u32 = DATA + 0x5_5000;

    pub(crate) const MEM_HEAP: u32 = 0x004f_f250;

    pub(crate) fn run(c: &mut Cpu, addr: u32, f: GuestFn, args: &[u64]) -> u64 {
        for (i, a) in args.iter().enumerate() {
            c.g[[A0, A1, A2, A3, T0, T1][i]][0] = *a;
        }
        c.g[RA][0] = SENTINEL as u64;
        c.pc = addr;
        f(c);
        assert_eq!(c.pc, SENTINEL, "setup call {addr:#x} did not return");
        c.g[V0][0]
    }

    /// A dummy world (`gp-0x848`), its player/bolt manager, the script DB and
    /// the BELibrary singleton (`gp-0x1f04`).
    pub(crate) fn base_3(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let r = machine(c, seed, salt, 0);
        c.bus.write32(gp_at(-0x1f04), BE_2);
        c.bus.write32(gp_at(-0x848), W_2);
        c.bus.write32(W_2 + 0x7c, RM);
        c.bus.write32(W_2 + 0x84, RM);
        c.bus.write32(RM + 0x84, RIDERS);
        c.bus.write32(RIDERS + 0x10, 0);
        c.bus.write32(W_2 + 0x8c, DB);
        c.bus.write32(DB + 4, VTS);
        r
    }

    /// [`base_3`] plus the size-class pool allocator and a dlmalloc arena.
    pub(crate) fn alloc_base(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let r = base_3(c, seed, salt);
        c.bus.write32(GAME_GP + 0xdd8, 1);
        const POOLS: [(u32, u32, u32); 7] = [
            (0x2a70, 0x800, 8),
            (0x2a60, 0x2000, 0x10),
            (0x2a6c, 0x32000, 0x20),
            (0x2a7c, 0x25800, 0x40),
            (0x2a84, 0x25800, 0x80),
            (0x2a78, 0x20000, 0x100),
            (0x2a68, 0x20000, 0x200),
        ];
        let mut nodes = DATA + 0x24_000;
        for (i, (slot_2, head, block)) in POOLS.into_iter().enumerate() {
            let pool = DATA + 0x20_000 + 0x100 * i as u32;
            let block_base = nodes;
            for n in 0..16u32 {
                let node = block_base + block * n;
                c.bus.write32(node, if n == 15 { 0 } else { node + block });
            }
            c.bus.write32(pool + head, block_base);
            c.bus.write32(GAME_GP + slot_2, pool);
            nodes += block * 16;
        }
        run(
            c,
            0x0031_ff60,
            ee::s10::DLHeap_InitArena,
            &[MEM_HEAP as u64, (DATA + 0x4_0000) as u64, 0x4_0000],
        );
        r
    }

    /// A `{delta = 0, fn = fake}` vtable pair at `off` in `vt`.
    pub(crate) fn slot_2(c: &mut Cpu, vt: u32, off: u32, index: u32) {
        c.bus.write16(vt + off, 0);
        c.bus.write32(vt + off + 4, FAKE + 0x10 * index);
    }

    #[test]
    fn csetup_man_recv_msg11_matches() {
        check(
            ee::s07::f_00269b90__generated,
            csetup_man_client_recv_msg11,
            40,
            &|c, seed| {
                let _r = alloc_base(c, seed, 0x2691);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = (seed + 1) as u64;
                c.bus.write32(gp_at(-0xc8), DATA + 0x800);
                c.bus.write32(DATA + 0x8f0, DATA + 0x900);
                c.bus.write32(DATA + 0x900, 0);
            },
        );
    }

    #[test]
    fn csetup_man_recv_msg13_matches() {
        check(
            ee::s07::f_00269cf0__generated,
            csetup_man_client_recv_msg13,
            40,
            &|c, seed| {
                let _r = alloc_base(c, seed, 0x2692);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = (seed + 1) as u64;
                c.bus.write32(gp_at(-0xc8), DATA + 0x800);
                c.bus.write32(DATA + 0x8f0, DATA + 0x900);
            },
        );
    }

    #[test]
    fn csetup_man_recv_msg10_matches() {
        check(
            ee::s07::f_00269ae0__generated,
            csetup_man_client_recv_msg10,
            40,
            &|c, seed| {
                let _r = alloc_base(c, seed, 0x2693);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = (seed + 1) as u64;
                c.bus.write32(gp_at(-0xc8), DATA + 0x800);
                c.bus.write32(DATA + 0x8f0, DATA + 0x900);
            },
        );
    }

    #[test]
    fn csetup_man_recv_msg12_matches() {
        check(
            ee::s07::f_00269c40__generated,
            csetup_man_client_recv_msg12,
            40,
            &|c, seed| {
                let _r = alloc_base(c, seed, 0x2694);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = (seed + 1) as u64;
                c.bus.write32(gp_at(-0xc8), DATA + 0x800);
                c.bus.write32(DATA + 0x8f0, DATA + 0x900);
            },
        );
    }

    #[test]
    fn path_read_event_matches() {
        check(
            ee::s07::f_0026a378__generated,
            path_read_event,
            80,
            &|c, seed| {
                let mut r = base_3(c, seed, 0x26a3);
                let src = DATA + 0x1000;
                let dst = DATA + 0x2000;
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(src);
                c.g[A2][0] = sx(dst);
                c.bus.write32(src + 4, (seed % 25) as u32);
                for i in 0..21u32 {
                    c.bus.write32(0x0044_53d0 + 4 * i, i * 3);
                }
                fill(c, &mut r, dst, 0x20);
                fill(c, &mut r, src + 8, 0x10);
            },
        );
    }

    // ------------------------------------------------------ cSetupMan queue

    #[test]
    fn setup_man_client_recv_msg5_matches() {
        check(
            ee::s07::f_00269188__generated,
            setup_man_client_recv_msg5,
            40,
            &|c, seed| {
                let mut r = alloc_base_2(c, seed, 0x2691, 3);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = r.next_u32() as u64;
                w(
                    c,
                    GAME_GP.wrapping_sub(0xc8),
                    if seed % 2 == 0 { 0 } else { DATA + 0x800 },
                );
            },
        );
    }

    #[test]
    fn setup_man_client_recv_msg4_matches() {
        check(
            ee::s07::f_002693c0__generated,
            setup_man_client_recv_msg4,
            40,
            &|c, seed| {
                let mut r = alloc_base_2(c, seed, 0x2693, 3);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = r.next_u32() as u64;
                w(
                    c,
                    GAME_GP.wrapping_sub(0xc8),
                    if seed % 2 == 0 { 0 } else { DATA + 0x800 },
                );
            },
        );
    }

    #[test]
    fn setup_man_client_recv_msg7_matches() {
        check(
            ee::s07::f_00269890__generated,
            setup_man_client_recv_msg7,
            40,
            &|c, seed| {
                let mut r = alloc_base_2(c, seed, 0x2698, 3);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = r.next_u32() as u64;
                w(
                    c,
                    GAME_GP.wrapping_sub(0xc8),
                    if seed % 2 == 0 { 0 } else { DATA + 0x800 },
                );
            },
        );
    }

    #[test]
    fn csetup_man_host_recv_msg9_matches() {
        check(
            ee::s07::f_00269538__generated,
            csetup_man_host_recv_msg9,
            80,
            &|c, seed| {
                let _r = alloc_base(c, seed, 0x2695);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed + 1) as u64;
                c.g[A2][0] = (seed as u64) << 32 | 0x77;
                c.bus.write32(gp_at(-0xc8), DATA + 0x800);
                c.bus.write32(DATA + 0x8f0, DATA + 0x900);
                c.bus.write32(DATA + 0x900, 0);
            },
        );
    }

    #[test]
    fn csetup_man_recv_msg8_matches() {
        check(
            ee::s07::f_00269948__generated,
            csetup_man_client_recv_msg8,
            60,
            &|c, seed| {
                let _r = alloc_base(c, seed, 0x2699);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed + 1) as u64;
                c.g[A2][0] = (seed as u64) << 32 | 0x99;
                c.bus.write32(gp_at(-0xc8), DATA + 0x800);
                c.bus.write32(DATA + 0x8f0, DATA + 0x900);
                c.bus.write32(DATA + 0x900, 0);
            },
        );
    }

    #[test]
    fn csetup_man_recv_msg9_matches() {
        check(
            ee::s07::f_00269a18__generated,
            csetup_man_client_recv_msg9,
            100,
            &|c, seed| {
                let _r = alloc_base(c, seed, 0x269a);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed + 1) as u64;
                c.g[A2][0] = (seed as u64) << 32 | 0x55;
                c.bus.write32(gp_at(-0xc8), DATA + 0x800);
                c.bus.write32(DATA + 0x8f0, DATA + 0x900);
                c.bus.write32(DATA + 0x900, 0);
            },
        );
    }

    #[test]
    fn path_find_marker_boundary_matches() {
        diff(
            ee::s07::f_00269f18__generated,
            path_find_marker_boundary,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x26_9f18, 0);
                globals_3(c);
                let list = DATA + 0x1000;
                let entries = DATA + 0x2000;
                let count = (seed % 4) as u32;
                w(c, list, count);
                w(c, list + 4, entries);
                w(c, list + 8, (seed / 4 % 4) as u32);
                for i in 0..4 {
                    w(c, list + 0xc + 4 * i, uf_3(&mut r, 20.0));
                }
                let segs = DATA + 0x2500;
                w(c, list + 0x18, segs);
                for i in 0..4 {
                    for j in 0..3 {
                        w(c, segs + 0x10 * i + 4 * j, uf_3(&mut r, 2.0));
                    }
                    w(
                        c,
                        segs + 0x10 * i + 0xc,
                        10.0f32.to_bits() + (r.next_u32() % 0x40),
                    );
                }
                for i in 0..count {
                    let e = entries + 0x10 * i;
                    let kind = [12u32, 16, 14, 7][(seed as usize + i as usize) % 4];
                    w(c, e, kind);
                    w(c, e + 8, uf_3(&mut r, 50.0));
                    w(c, e + 0xc, uf_3(&mut r, 50.0));
                }
                w(c, DATA + 0x3000, uf_3(&mut r, 60.0));
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(list);
                c.g[A2][0] = sx(DATA + 0x3000);
            },
        );
    }

    #[test]
    fn csetupman_host_recv_msg11_matches() {
        strict(
            ee::s07::f_00269608__generated,
            csetupman_host_recv_msg11,
            SEEDS_6,
            &|c, seed| {
                let r = base_6(c, seed, 0x9608, 0);
                let _ = r;
                let msg = DATA + 0x100;
                w(c, msg, (seed % 12) as u32);
                // the setup manager's packet list behind gp_4 - 0xc8
                let list = DATA + 0x2000;
                let node = DATA + 0x2100;
                let prev = DATA + 0x2200;
                w(c, GAME_GP.wrapping_sub(0xc8), list);
                w(c, list + 0xf0, node);
                w(c, node + 4, prev);
                w(c, prev, 0);
                c.g[A0][0] = sx(DATA + 0x3000);
                c.g[A1][0] = sx(msg);
            },
        );
    }

    /// PATH_SetVolumePercent: not initialised, a selection in progress, or
    /// the four maps (loaded or not, flag nibbles matching the mask or not)
    /// with 24 tracks (present or not); masks -1 or random, volumes in and
    /// out of 0..100.
    #[test]
    fn path_set_volume_percent_matches() {
        use crate::difftest::fakes::machine;
        use crate::difftest::{check, DATA};
        let w = |c: &mut Cpu, a: u32, v: u32| c.bus.write32(a, v);
        check(
            ee::s14::f_003d5508__generated,
            path_set_volume_percent,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3d_5508, 0);
                c.pc = 0x003d_5508;
                c.bus.write8(0x0044_f42c, (seed % 11 != 0) as u8);
                c.bus.write8(0x0044_f42e, r.next_u32() as u8);
                w(
                    c,
                    0x0044_f430,
                    if seed % 13 == 1 { r.next_u32() | 1 } else { 0 },
                );
                w(c, 0x0044_f420, r.next_u32());
                let mask = if seed % 4 == 0 {
                    u32::MAX
                } else {
                    r.next_u32()
                };
                for i in 0..4u32 {
                    let map = 0x0051_7610 + 0x928 * i;
                    w(c, map + 0x34, (r.next_u32() % 4 != 0) as u32);
                    let flags = match r.next_u32() % 3 {
                        0 => r.next_u32(),
                        1 => mask & 0xff00_0000 | 1,
                        _ => mask & 0xf000_0000,
                    };
                    w(c, map + 0x30, flags);
                    for j in 0..24u32 {
                        let t = if r.next_u32() % 3 == 0 {
                            0
                        } else {
                            DATA + 0x40 * (j + 24 * i)
                        };
                        w(c, map + 0x58 + 4 * j, t);
                    }
                }
                c.g[A0][0] = sx(mask);
                c.g[A1][0] = match seed % 5 {
                    0 => 200,
                    1 => sx(0xffff_ff80),
                    _ => r.next_u32() as u64 | (r.next_u32() as u64) << 32,
                };
            },
        );
    }

    /// Path_GetClosestPoint: paths of 0..5 segments (and a negative
    /// count), the point anywhere around them, flat (xy) and full (VU0)
    /// distances, so the projection clamps at both ends and the best point
    /// changes or not.
    #[test]
    fn path_get_closest_point_matches() {
        check(
            ee::s07::f_0026a428__generated,
            path_get_closest_point,
            240,
            &|c, seed| {
                let mut r = machine(c, seed, 0x26_a428, 0);
                c.pc = 0x0026_a428;
                let path = DATA;
                let segs = DATA + 0x1000;
                let p = DATA + 0x2000;
                let out = DATA + 0x3000;
                let dist = DATA + 0x3100;
                let n = match seed % 11 {
                    0 => 0,
                    1 => r.next_u32() | 0x8000_0000,
                    _ => r.next_u32() % 5 + 1,
                };
                c.bus.write32(path + PATH_SEG_COUNT, n);
                for i in 0..3 {
                    c.bus.write32(path + PATH_START + 4 * i, r.f32(50.0));
                }
                c.bus.write32(path + PATH_SEGMENTS, segs);
                for s in 0..5 {
                    let a = (r.next_u32() % 628) as f32 / 100.0;
                    let z = (r.next_u32() % 100) as f32 / 400.0;
                    let seg = segs + SEG_SIZE * s;
                    c.bus.write32(seg, a.cos().to_bits());
                    c.bus.write32(seg + 4, a.sin().to_bits());
                    c.bus.write32(seg + 8, z.to_bits());
                    c.bus
                        .write32(seg + 0xc, ((r.next_u32() % 4000) as f32 / 100.0).to_bits());
                }
                for i in 0..4 {
                    c.bus.write32(p + 4 * i, r.f32(80.0));
                }
                fill(c, &mut r, out, 4);
                let far = if seed % 5 == 0 { 100.0f32 } else { 1.0e30 };
                c.bus.write32(GAME_GP - LIT_PATH_FAR, far.to_bits());
                c.g[A0][0] = path as u64;
                c.g[A1][0] = p as u64;
                c.g[A2][0] = out as u64;
                c.g[A3][0] = dist as u64;
                c.g[T0][0] = if seed % 2 == 0 {
                    0
                } else {
                    r.next_u32() as u64 | 1
                };
            },
        );
    }

    /// PATH_SelectMasterTrack: the state at DATA (pointer at 0x44f420) with
    /// 24 track slots, each empty, disabled, with a negative `+0x36`, no
    /// header, or a candidate with a random length (sometimes equal to an
    /// earlier one, sometimes with the top bit set) and header word.
    #[test]
    fn path_select_master_track_matches() {
        check(
            ee::s14::f_003d4b00__generated,
            path_select_master_track,
            300,
            &|c, seed| {
                let mut r = Rng(0x003d_4b00 ^ (seed as u64) << 9);
                scramble(c, &mut r);
                let state = DATA;
                c.bus.write32(PATH_STATE, state);
                fill(c, &mut r, state, 4);
                let lengths = [r.next_u32() % 100, r.next_u32()];
                for i in 0..24u32 {
                    let track = DATA + 0x1000 + 0x100 * i;
                    let header = DATA + 0x4000 + 0x10 * i;
                    let empty = r.next_u32() % 6 == 0;
                    c.bus
                        .write32(state + PATH_TRACKS + 4 * i, if empty { 0 } else { track });
                    fill(c, &mut r, track, 0x10);
                    c.bus
                        .write8(track + TRACK_DISABLED, (r.next_u32() % 5 == 0) as u8);
                    let flags = if r.next_u32() % 5 == 0 {
                        0x8000 | r.next_u32()
                    } else {
                        r.next_u32() & 0x7fff
                    };
                    c.bus.write16(track + TRACK_FLAGS, flags as u16);
                    c.bus.write32(
                        track + TRACK_HEADER,
                        if r.next_u32() % 5 == 0 { 0 } else { header },
                    );
                    c.bus.write32(header, r.next_u32());
                    let len = match r.next_u32() % 4 {
                        0 => lengths[0],
                        1 => lengths[1],
                        _ => r.next_u32() % (100 + 10 * seed),
                    };
                    c.bus.write32(track + TRACK_LENGTH, len);
                }
                c.pc = 0x003d_4b00;
            },
        );
    }

    /// PATH_SetTrackControlValue: player on/off, a map already selected or
    /// not, values below and above 0x80, random masks (sometimes all ones)
    /// against four maps with random ids and loaded flags, each with 24
    /// slots that are empty or point at a random track; the real
    /// `PATH_SelectMap`.
    #[test]
    fn path_set_track_control_value_matches() {
        use super::path_ctl::*;
        check_lohi(
            ee::s14::f_003d0d70__generated,
            path_set_track_control_value,
            300,
            &|c, seed| {
                let mut r = Rng(0x003d_0d70 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.pc = 0x003d_0d70;
                let enabled = if seed % 8 == 0 {
                    0
                } else {
                    1 + r.next_u32() % 255
                };
                c.bus.write8(ENABLED, enabled as u8);
                let id = if seed % 8 == 1 { r.next_u32() | 1 } else { 0 };
                c.bus.write32(CURRENT_ID, id);
                c.bus.write32(CURRENT_MAP, r.next_u32());
                let mask = if seed % 3 == 0 {
                    u64::MAX
                } else {
                    (r.next_u32() as u64) << 32 | r.next_u32() as u64
                };
                c.g[A0][0] = mask;
                c.g[A1][0] = if seed % 5 == 0 {
                    (r.next_u32() as u64) << 32 | r.next_u32() as u64
                } else {
                    (r.next_u32() % 0x80) as u64
                };
                for m in 0..4u32 {
                    let map = MAP0_ID - 0x30 + m * MAP_SIZE as u32;
                    c.bus.write32(map + 0x30, r.next_u32());
                    c.bus
                        .write32(map + 0x34, if r.next_u32() % 3 == 0 { 0 } else { 1 });
                    for j in 0..24u32 {
                        let track = DATA + (m * 24 + j) * 0x10;
                        fill(c, &mut r, track, 4);
                        let p = if r.next_u32() % 4 == 0 { 0 } else { track };
                        c.bus.write32(map + SLOT0 + 4 * j, p);
                    }
                }
            },
        );
    }

    // be_library is defined in this module

    /// PathData_FindNearTrackPaths: 0..9 track paths whose bounds contain
    /// the position (distance 0, taken first) or lie around it, `max` below,
    /// at or above the count (sometimes with garbage in the upper half of
    /// a3), so both passes, the early "full" exit and the minimum search run.
    #[test]
    fn path_data_find_near_track_paths_matches() {
        check(
            ee::s07::f_0026b178__generated,
            path_data_find_near_track_paths,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x26b178, 0);
                let this = DATA;
                let items = DATA + 0x1000;
                let pos = DATA + 0x800;
                let out = DATA + 0x900;
                let taken = 1e37f32.to_bits();
                put_words(c, GAME_GP - NEAR_PATHS_TAKEN_LIT, &[taken]);
                put_words(c, GAME_GP - NEAR_PATHS_TAKEN_LIT2, &[taken]);
                let count = r.next_u32() % 10;
                put_words(c, this + NEAR_PATHS_COUNT, &[count, items]);
                let p = [r.f32(50.0), r.f32(50.0), r.f32(50.0)];
                put_words(c, pos, &p);
                for i in 0..count {
                    let box_at = items + NEAR_PATHS_STRIDE * i;
                    let inside = r.next_u32() % 3 == 0;
                    for k in 0..3u32 {
                        let centre = f32::from_bits(p[k as usize]);
                        let (lo, hi) = if inside {
                            (centre - 1.0 - (r.next_u32() % 8) as f32, centre + 1.0)
                        } else {
                            let a = centre + (r.next_u32() % 64) as f32 - 32.0;
                            (a, a + (r.next_u32() % 16) as f32)
                        };
                        c.bus.write32(box_at + 0x1c + 4 * k, lo.to_bits());
                        c.bus.write32(box_at + 0x28 + 4 * k, hi.to_bits());
                    }
                }
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(pos);
                c.g[A2][0] = sx(out);
                let max = r.next_u32() % 12;
                c.g[A3][0] = if seed % 7 == 0 {
                    (r.next_u32() as u64) << 32 | max as u64
                } else {
                    max as u64
                };
                c.pc = 0x0026_b178;
            },
        );
    }

    /// PathData_FindNearAIPaths: 0..12 AI paths whose boxes contain the
    /// point (taken in the first pass, possibly filling `max`) or lie at
    /// random distances (equal ones too), some disabled; `max` from -1 to 8,
    /// `enabledOnly` on half the seeds; the real Path_BoundsDistance.
    #[test]
    fn path_data_find_near_ai_paths_matches() {
        const PATHDATA: u32 = DATA;
        const POINT: u32 = DATA + 0x80;
        const PATHS: u32 = DATA + 0x100;
        const OUT: u32 = DATA + 0x1000;
        check(
            ee::s07::f_0026afb8__generated,
            path_data_find_near_ai_paths,
            300,
            &|c, seed| {
                let mut r = Rng(0x0026_afb8 ^ (seed as u64) << 4);
                scramble(c, &mut r);
                c.pc = 0x0026_afb8;
                c.g[GP][0] = sx(GAME_GP);
                c.bus.write32(GAME_GP - 0x4a4c, 1.0e30f32.to_bits());
                c.bus.write32(GAME_GP - 0x4a48, 2.0e30f32.to_bits());
                let n = seed % 13;
                c.bus.write32(PATHDATA + PATH_DATA_AI_COUNT, n);
                c.bus.write32(PATHDATA + PATH_DATA_AI_PATHS, PATHS);
                for k in 0..3 {
                    c.bus.write32(POINT + 4 * k, r.f32(4.0));
                }
                let point = [0, 1, 2].map(|k| f32::from_bits(c.bus.read32(POINT + 4 * k)));
                for i in 0..n {
                    let path = PATHS + 0x40 * i;
                    let mode = r.next_u32() % 4;
                    for k in 0..3u32 {
                        let p = point[k as usize];
                        let (lo, hi) = match mode {
                            0 => (p - 1.0, p + 1.0),                  // contains the point
                            1 => (p + 1.0 + (i % 3) as f32, p + 5.0), // same distance per i%3
                            _ => (p + 0.5 + r.next_u32() as f32 / 1.0e9, p + 9.0),
                        };
                        c.bus.write32(path + 0x1c + 4 * k, lo.to_bits());
                        c.bus.write32(path + 0x28 + 4 * k, hi.to_bits());
                    }
                    c.bus
                        .write32(path + AI_PATH_ENABLED, (r.next_u32() % 3 != 0) as u32);
                }
                c.g[A0][0] = sx(PATHDATA);
                c.g[A1][0] = sx(POINT);
                c.g[A2][0] = sx(OUT);
                c.g[A3][0] = sx(((seed / 13 % 10) as i32 - 1) as u32);
                c.g[T0][0] = (seed % 2) as u64;
            },
        );
    }
}
