//! `src/dirtysock/tags2.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::call::Unwound;
use crate::call::VSlot;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::ai::computer::sd;
pub(crate) use crate::game::animation::rideranimbase::sw as sw_4;
pub(crate) use crate::game::bx::memblockman::gp_2 as gp;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::at as at_2;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::gpa as gpa_2;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::lh_at;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::lw_at as lw_at_5;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::ofs;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::open;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::spv;
pub(crate) use crate::game::fe::feasyncfile::at_5 as at_4;
pub(crate) use crate::game::fe::feasyncfile::gp_16 as gp_13;
pub(crate) use crate::game::fe::feasyncfile::gp_17 as gp_14;
pub(crate) use crate::game::fe::feasyncfile::gp_18 as gp_15;
pub(crate) use crate::game::fe::feasyncfile::gp_3 as gp_6;
pub(crate) use crate::game::fe::feasyncfile::gp_7 as gp_11;
pub(crate) use crate::game::fe::feasyncfile::gp_8 as gp_12;
pub(crate) use crate::game::fe::feasyncfile::gpa;
pub(crate) use crate::game::fe::feasyncfile::low_2;
pub(crate) use crate::game::fe::feasyncfile::lw_3;
pub(crate) use crate::game::fe::feasyncfile::lw_5;
pub(crate) use crate::game::fe::feasyncfile::lw_6;
pub(crate) use crate::game::fe::feasyncfile::lw_7;
pub(crate) use crate::game::fe::feasyncfile::lw_8;
pub(crate) use crate::game::fe::feasyncfile::lw_at_2;
pub(crate) use crate::game::fe::feasyncfile::lw_r;
pub(crate) use crate::game::fe::feasyncfile::p_5;
pub(crate) use crate::game::fe::feasyncfile::sra32;
pub(crate) use crate::game::fe::feasyncfile::sw_4 as sw_3;
pub(crate) use crate::game::fe::feasyncfile::sw_5;
pub(crate) use crate::game::fe::feasyncfile::sw_r;
pub(crate) use crate::game::fe::fepopup::ld;
pub(crate) use crate::game::fe::fepopup::sw_4 as sw_6;
pub(crate) use crate::game::fe::fepopup3::lh;
pub(crate) use crate::game::fe::fepopup3::lw_2;
pub(crate) use crate::game::fe::fepopup3::p_2 as p_4;
pub(crate) use crate::game::fe::fepopup3::sw as sw_2;
pub(crate) use crate::game::fe::festaterewards::at_4 as at_3;
pub(crate) use crate::game::fe::festaterewards::lw_3 as lw_4;
pub(crate) use crate::game::fe::festaterewards2::r;
pub(crate) use crate::game::main::gamestate::gw;
pub(crate) use crate::game::main::gamestate::lw_at_3 as lw_at_4;
pub(crate) use crate::game::movie::movieplayer::lw_at;
pub(crate) use crate::game::scripter::ssxscriptengine::gp as gp_7;
pub(crate) use crate::game::scripter::ssxscriptengine::gp_5 as gp_10;
pub(crate) use crate::game::scripter::ssxscriptengine::low;
pub(crate) use crate::game::scripter::ssxscriptengine::lw_at_2 as lw_at_3;
pub(crate) use crate::game::scripter::ssxscriptengine::p_2 as p_3;
pub(crate) use crate::game::scripter::ssxscriptengine::sw_at;
pub(crate) use crate::game::sound::bankmonitor::gp as gp_4;
pub(crate) use crate::game::sound::bankmonitor::leave;
pub(crate) use crate::game::sound::bankmonitor::load_gp;
pub(crate) use crate::game::sound::bankmonitor::p;
pub(crate) use crate::game::sound::icepick::worldtriggermanager::forward;
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::call_indirect;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::vu0::Src;
use ssx3_ee_rt::vu0::W;
use ssx3_ee_rt::vu0::X;
use ssx3_ee_rt::vu0::Y;
use ssx3_ee_rt::vu0::Z;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x0025_9198, lobby_man_get_language, "LobbyMan_GetLanguage"),
    (
        0x0025_98c8,
        lobby_man_on_disconnect,
        "LobbyMan_OnDisconnect",
    ),
    (
        0x0025_a650,
        lobby_man_create_peer_table,
        "LobbyMan_CreatePeerTable",
    ),
    (
        0x0025_a6f8,
        lobby_man_destroy_peer_table,
        "LobbyMan_DestroyPeerTable",
    ),
    (
        0x0025_a730,
        lobby_man_lookup_peer_addr,
        "LobbyMan_LookupPeerAddr",
    ),
    (
        0x0025_a850,
        lobby_man_release_snapshot_lists,
        "LobbyMan_ReleaseSnapshotLists",
    ),
    (
        0x0025_a8b0,
        lobby_man_destroy_lists,
        "LobbyMan_DestroyLists",
    ),
    (
        0x0025_aa98,
        lobby_man_destroy_lobby_api,
        "LobbyMan_DestroyLobbyApi",
    ),
    (0x0025_aae0, lobby_man_send_chat, "LobbyMan_SendChat"),
    (0x0025_b608, lobby_man_send_request, "LobbyMan_SendRequest"),
    (0x0025_b688, lobby_man_destroy, "LobbyMan_Destroy"),
    (0x0025_b800, lobby_man_disconnect, "LobbyMan_Disconnect"),
    (
        0x0025_bc48,
        lobby_man_parse_name_suggestions,
        "LobbyMan_ParseNameSuggestions",
    ),
    (0x0025_bf18, lobbyman_edit_account, "LobbyMan_EditAccount"),
    (
        0x0025_c0c0,
        lobby_man_on_edit_account_reply,
        "LobbyMan_OnEditAccountReply",
    ),
    (
        0x0025_c2d8,
        lobby_man_on_lost_by_email_reply,
        "LobbyMan_OnLostByEmailReply",
    ),
    (
        0x0025_c4f0,
        lobby_man_on_lost_by_name_reply,
        "LobbyMan_OnLostByNameReply",
    ),
    (
        0x0025_cfe0,
        lobbyman_query_own_stats,
        "LobbyMan_QueryOwnStats",
    ),
    (0x0025_d770, lobby_man_peek_room, "LobbyMan_PeekRoom"),
    (
        0x0025_d800,
        lobby_man_move_to_room_index,
        "LobbyMan_MoveToRoomIndex",
    ),
    (
        0x0025_da90,
        lobby_man_get_kick_reason,
        "LobbyMan_GetKickReason",
    ),
    (0x0025_dac0, lobby_man_create_room, "LobbyMan_CreateRoom"),
    (0x0025_dd08, lobby_man_ping_user, "LobbyMan_PingUser"),
    (0x0025_e0c8, lobby_man_request_news, "LobbyMan_RequestNews"),
    (
        0x0025_ee20,
        lobby_man_start_quick_play,
        "LobbyMan_StartQuickPlay",
    ),
    (
        0x0025_f030,
        lobby_man_on_quick_play_reply,
        "LobbyMan_OnQuickPlayReply",
    ),
    (
        0x0025_f120,
        lobby_man_get_room_count,
        "LobbyMan_GetRoomCount",
    ),
    (0x0025_f150, lobby_man_get_room_name, "LobbyMan_GetRoomName"),
    (
        0x0025_f180,
        lobby_man_is_room_flagged,
        "LobbyMan_IsRoomFlagged",
    ),
    (
        0x0025_f1b0,
        lobby_man_get_room_user_count,
        "LobbyMan_GetRoomUserCount",
    ),
    (
        0x0025_f1e0,
        lobby_man_get_room_user_name,
        "LobbyMan_GetRoomUserName",
    ),
    (
        0x0025_f248,
        lobby_man_is_room_user_flagged,
        "LobbyMan_IsRoomUserFlagged",
    ),
    (
        0x0025_f388,
        lobby_man_get_rank_list_entry,
        "LobbyMan_GetRankListEntry",
    ),
    (0x0025_f430, lobby_man_is_in_room, "LobbyMan_IsInRoom"),
    (
        0x0025_f858,
        lobby_man_reset_chal_state,
        "LobbyMan_ResetChalState",
    ),
    (
        0x0025_fa48,
        lobby_man_chal_request_host,
        "LobbyMan_ChalRequestHost",
    ),
    (
        0x0025_fad0,
        lobby_man_chal_request_join,
        "LobbyMan_ChalRequestJoin",
    ),
    (
        0x0025_fb58,
        lobby_man_accept_challenge,
        "LobbyMan_AcceptChallenge",
    ),
    (
        0x0025_fb80,
        lobby_man_send_private_message,
        "LobbyMan_SendPrivateMessage",
    ),
    (
        0x0025_fc70,
        lobby_man_send_lock_chal,
        "LobbyMan_SendLockChal",
    ),
    (
        0x0026_0f10,
        lobby_man_reset_account_info,
        "LobbyMan_ResetAccountInfo",
    ),
    (0x0026_0f80, buddyman_create, "BuddyMan_Create"),
    (0x0026_1008, buddy_man_destroy, "BuddyMan_Destroy"),
    (
        0x0026_1070,
        buddy_man_clear_messages,
        "BuddyMan_ClearMessages",
    ),
    (0x0026_13c8, buddy_man_reconnect, "BuddyMan_Reconnect"),
    (0x0026_1408, buddy_man_disconnect, "BuddyMan_Disconnect"),
    (0x0026_1460, buddy_man_is_connected, "BuddyMan_IsConnected"),
    (0x0026_1490, buddy_man_update, "BuddyMan_Update"),
    (0x0026_14e8, buddy_man_create_api, "BuddyMan_CreateApi"),
    (0x0026_1530, buddy_man_destroy_api, "BuddyMan_DestroyApi"),
    (
        0x0026_2280,
        buddy_man_count_permanent_entries,
        "BuddyMan_CountPermanentEntries",
    ),
    (
        0x0026_22f0,
        buddy_man_count_temp_entries,
        "BuddyMan_CountTempEntries",
    ),
    (0x0026_2360, buddy_man_is_buddy, "BuddyMan_IsBuddy"),
    (0x0026_23b0, buddy_man_is_blocked, "BuddyMan_IsBlocked"),
    (0x0026_2400, buddy_man_is_temp_at, "BuddyMan_IsTempAt"),
    (0x0026_2438, buddy_man_is_temp, "BuddyMan_IsTemp"),
    (
        0x0026_2468,
        buddy_man_find_roster_entry,
        "BuddyMan_FindRosterEntry",
    ),
    (
        0x0026_2598,
        buddy_man_get_roster_count,
        "BuddyMan_GetRosterCount",
    ),
    (
        0x0026_25c8,
        buddy_man_get_roster_entry,
        "BuddyMan_GetRosterEntry",
    ),
    (
        0x0026_28c0,
        buddy_man_map_presence_state_2,
        "BuddyMan_MapPresenceState",
    ),
    (
        0x0026_2938,
        buddy_man_get_message_count,
        "BuddyMan_GetMessageCount",
    ),
    (
        0x0026_2a08,
        buddy_man_count_messages_from,
        "BuddyMan_CountMessagesFrom",
    ),
    (
        0x0026_2a38,
        buddy_man_has_message_from,
        "BuddyMan_HasMessageFrom",
    ),
    (
        0x0026_2e20,
        buddy_man_find_challenger_index,
        "BuddyMan_FindChallengerIndex",
    ),
    (
        0x0026_3128,
        buddy_man_has_challenger,
        "BuddyMan_HasChallenger",
    ),
    (
        0x0026_3308,
        buddy_man_get_challenger,
        "BuddyMan_GetChallenger",
    ),
    (0x0026_3338, buddy_man_is_online_at, "BuddyMan_IsOnlineAt"),
    (0x0026_33c8, buddy_man_is_online, "BuddyMan_IsOnline"),
    (
        0x0026_33f8,
        buddy_man_is_same_product_at,
        "BuddyMan_IsSameProductAt",
    ),
    (
        0x0026_3490,
        buddy_man_is_same_product,
        "BuddyMan_IsSameProduct",
    ),
    (
        0x0026_34c0,
        buddy_man_roster_compare,
        "BuddyMan_RosterCompare",
    ),
    (
        0x0026_35a8,
        buddy_man_delete_message_at,
        "BuddyMan_DeleteMessageAt",
    ),
    (0x0026_3630, buddy_man_get_message, "BuddyMan_GetMessage"),
    (0x0026_3678, fe_net_hook_dtor, "FENetHook_dtor"),
    (0x0026_36a8, net_start_connect, "Net_StartConnect"),
    (0x0026_39b0, fe_net_hook2_dtor, "FENetHook2_dtor"),
    (0x0026_4030, c_setup_man_create, "cSetupMan_Create"),
    (0x0026_4098, c_setup_man_destroy, "cSetupMan_Destroy"),
    (0x0026_40c0, c_setup_man_ctor, "cSetupMan_ctor"),
    (
        0x0026_4960,
        c_setup_man_alloc_packet,
        "cSetupMan_AllocPacket",
    ),
    (0x0026_4990, c_setup_man_free_packet, "cSetupMan_FreePacket"),
    (0x0026_49b0, c_setup_man_pop_packet, "cSetupMan_PopPacket"),
    (
        0x0026_4b20,
        c_setup_man_flush_send_queue,
        "cSetupMan_FlushSendQueue",
    ),
    (0x0026_4cb8, csetup_man_send_hello, "cSetupMan_SendHello"),
    (
        0x0026_4cf0,
        c_setup_man_set_local_character,
        "cSetupMan_SetLocalCharacter",
    ),
    (
        0x0026_4d80,
        csetup_man_set_remote_character,
        "cSetupMan_SetRemoteCharacter",
    ),
    (
        0x0026_5140,
        csetup_man_cancel_character,
        "cSetupMan_CancelCharacter",
    ),
    (
        0x0026_5398,
        setup_man_host_recv_cancel,
        "cSetupMan_HostRecvCancel",
    ),
    (
        0x0026_5688,
        c_setup_man_send_brag_msg10,
        "cSetupMan_SendBragMsg10",
    ),
    (
        0x0026_5700,
        c_setup_man_send_brag_msg12,
        "cSetupMan_SendBragMsg12",
    ),
    (
        0x0026_58b8,
        c_shared_login_man_destroy,
        "cSharedLoginMan_Destroy",
    ),
    (0x0026_5908, shared_login_man_dtor, "cSharedLoginMan_dtor"),
    (0x0026_5950, c_shared_login_man_init, "cSharedLoginMan_Init"),
    (
        0x0026_59c0,
        shared_login_man_shutdown,
        "cSharedLoginMan_Shutdown",
    ),
    (
        0x0026_59e8,
        cshared_login_man_request_card_info_if_idle,
        "cSharedLoginMan_RequestCardInfoIfIdle",
    ),
    (
        0x0026_5a38,
        c_shared_login_man_card_slot20,
        "cSharedLoginMan_CardSlot20",
    ),
    (
        0x0026_5cb0,
        c_shared_login_man_is_card_unformatted,
        "cSharedLoginMan_IsCardUnformatted",
    ),
    (
        0x0026_5d08,
        c_shared_login_man_card_slot47,
        "cSharedLoginMan_CardSlot47",
    ),
    (
        0x0026_5d38,
        c_shared_login_man_card_slot63,
        "cSharedLoginMan_CardSlot63",
    ),
    (
        0x0026_5d68,
        c_shared_login_man_is_card_missing,
        "cSharedLoginMan_IsCardMissing",
    ),
    (
        0x0026_5d98,
        cshared_login_man_do_load,
        "cSharedLoginMan_DoLoad",
    ),
    (
        0x0026_5e40,
        shared_login_man_do_update,
        "cSharedLoginMan_DoUpdate",
    ),
    (
        0x0026_5e80,
        shared_login_man_do_create,
        "cSharedLoginMan_DoCreate",
    ),
    (
        0x0026_5ec8,
        shared_login_do_delete,
        "cSharedLoginMan_DoDelete",
    ),
    (
        0x0026_5f00,
        c_shared_login_man_do_finish,
        "cSharedLoginMan_DoFinish",
    ),
    (
        0x0026_5f18,
        c_shared_login_man_set_account_name,
        "cSharedLoginMan_SetAccountName",
    ),
    (
        0x0026_5f68,
        c_shared_login_man_set_account_password,
        "cSharedLoginMan_SetAccountPassword",
    ),
    (
        0x0026_5fb8,
        c_shared_login_man_mc_list_dir,
        "cSharedLoginMan_McListDir",
    ),
    (
        0x0026_6010,
        shared_login_man_mc_cb_state09,
        "cSharedLoginMan_McCb_State09",
    ),
    (
        0x0026_6038,
        shared_login_man_mc_cb_state0a,
        "cSharedLoginMan_McCb_State0A",
    ),
    (
        0x0026_6060,
        c_shared_login_man_mc_get_info,
        "cSharedLoginMan_McGetInfo",
    ),
    (
        0x0026_60a0,
        shared_login_man_mc_cb_state03,
        "cSharedLoginMan_McCb_State03",
    ),
    (
        0x0026_60c8,
        c_shared_login_man_mc_open,
        "cSharedLoginMan_McOpen",
    ),
    (
        0x0026_6148,
        c_shared_login_man_mc_close,
        "cSharedLoginMan_McClose",
    ),
    (
        0x0026_6170,
        c_shared_login_man_mc_mkdir,
        "cSharedLoginMan_McMkdir",
    ),
    (
        0x0026_61d8,
        c_shared_login_man_mc_delete_file,
        "cSharedLoginMan_McDeleteFile",
    ),
    (
        0x0026_6240,
        cshared_login_man_mc_delete_path,
        "cSharedLoginMan_McDeletePath",
    ),
    (
        0x0026_6298,
        c_shared_login_man_mc_read,
        "cSharedLoginMan_McRead",
    ),
    (
        0x0026_62c0,
        c_shared_login_man_mc_write,
        "cSharedLoginMan_McWrite",
    ),
    (
        0x0026_6320,
        c_shared_login_man_mc_flush,
        "cSharedLoginMan_McFlush",
    ),
    (
        0x0026_6348,
        shared_login_man_mc_cb_state_0f,
        "cSharedLoginMan_McCb_State0F",
    ),
    (
        0x0026_6550,
        c_shared_login_man_mc_get_found_name,
        "cSharedLoginMan_McGetFoundName",
    ),
    (0x0026_6728, tib_ext_mem_alloc, "TibExtMemAlloc"),
    (0x0026_6760, tib_ext_mem_free, "TibExtMemFree"),
    (0x0026_67e8, c_usb_keyboard_ctor, "cUsbKeyboard_ctor"),
    (
        0x0026_68a8,
        usb_keyboard_on_connect,
        "cUsbKeyboard_OnConnect",
    ),
    (0x0026_6ca0, cvoip_man_create, "cVoipMan_Create"),
    (0x0026_6cd8, c_voip_man_destroy, "cVoipMan_Destroy"),
    (0x0026_6d10, voip_man_dtor, "cVoipMan_dtor"),
    (0x0026_6d58, c_voip_man_start, "cVoipMan_Start"),
    (0x0026_6df8, cvoip_man_stop, "cVoipMan_Stop"),
    (0x0026_6e30, cvoip_man_activate, "cVoipMan_Activate"),
    (0x0026_6e88, voip_man_deactivate, "cVoipMan_Deactivate"),
    (0x0026_6f58, cvoip_man_set_volume, "cVoipMan_SetVolume"),
    (0x0026_6f90, c_voip_man_get_status, "cVoipMan_GetStatus"),
    (
        0x0026_7dd8,
        dsock_channel_base_v05,
        "cDSockChannel_Base_v05",
    ),
    (
        0x0026_7e00,
        dsock_channel_is_connected,
        "cDSockChannel_IsConnected",
    ),
    (0x0026_7e18, net_event_dtor, "NetEvent_dtor"),
    (0x0026_7e48, net_event_clone, "NetEvent_Clone"),
    (0x0026_7e98, net_event102_dtor, "NetEvent102_dtor"),
    (0x0026_7ec8, net_event102_clone, "NetEvent102_Clone"),
    (0x0026_7f28, net_event102_base_dtor, "NetEvent102Base_dtor"),
    (
        0x0026_7f58,
        net_event102_base_clone,
        "NetEvent102Base_Clone",
    ),
    (0x0026_7fb8, net_error_event_dtor, "NetErrorEvent_dtor"),
    (0x0026_7fe8, net_error_event_clone, "NetErrorEvent_Clone"),
    (0x0026_8048, chat_event_dtor, "ChatEvent_dtor"),
    (0x0026_80a8, chat_event_clone, "ChatEvent_Clone"),
    (0x0026_8158, net_event_e2_dtor, "NetEventE2_dtor"),
    (0x0026_81b0, net_event_e2_clone, "NetEventE2_Clone"),
    (0x0026_8210, net_event102_tooy_dtor, "NetEvent102Tooy_dtor"),
    (
        0x0026_8240,
        net_event102_tooy_clone,
        "NetEvent102Tooy_Clone",
    ),
    (0x0026_82a8, net_event102_lock_dtor, "NetEvent102Lock_dtor"),
    (
        0x0026_8308,
        net_event102_lock_clone,
        "NetEvent102Lock_Clone",
    ),
    (0x0026_8398, net_event102_down_dtor, "NetEvent102Down_dtor"),
    (
        0x0026_83f0,
        net_event102_down_clone,
        "NetEvent102Down_Clone",
    ),
    (0x0026_8460, net_event_e3_dtor, "NetEventE3_dtor"),
    (0x0026_84c0, net_event_e3_clone, "NetEventE3_Clone"),
    (0x0026_8588, net_event_e5_dtor, "NetEventE5_dtor"),
    (0x0026_85e0, net_event_e5_clone, "NetEventE5_Clone"),
    (0x0026_8658, net_event_fb_dtor, "NetEventFB_dtor"),
    (0x0026_8688, net_event_fb_clone, "NetEventFB_Clone"),
    (0x0026_86f0, net_event_fc_dtor, "NetEventFC_dtor"),
    (0x0026_8720, net_event_fc_clone, "NetEventFC_Clone"),
    (0x0026_8790, net_event10d_dtor, "NetEvent10D_dtor"),
];

// ---------------------------------------------------------------------------
// Callees and fixed addresses.

pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    pub(crate) use crate::game::ai::rider::callee::BENEW_PLAYER_INT_GET_SLOT;
    pub(crate) use crate::game::ai::rider::callee::BUDDY_MAN_GET_ROSTER_COUNT as BUDDY_MAN_GET_ROSTER_COUNT_3;
    pub(crate) use crate::game::ai::rider::callee::BUDDY_MAN_GET_ROSTER_ENTRY as BUDDY_MAN_GET_ROSTER_ENTRY_3;
    pub(crate) use crate::game::ai::rider::callee::BUY_POPUP_INIT_BUY_BOLT;
    pub(crate) use crate::game::ai::rider::callee::CODEC_POP_FREE_FRAME;
    pub(crate) use crate::game::ai::rider::callee::CREATE_STEP1_ALL_FIELDS_FILLED;
    pub(crate) use crate::game::ai::rider::callee::CREATE_STEP1_BACKUP_ACCOUNT;
    pub(crate) use crate::game::ai::rider::callee::CREATE_STEP1_SET_ACCOUNT_NAME;
    pub(crate) use crate::game::ai::rider::callee::CREATE_STEP1_SET_CONFIRM_PASSWORD;
    pub(crate) use crate::game::ai::rider::callee::CREATE_STEP1_SET_EMAIL;
    pub(crate) use crate::game::ai::rider::callee::CREATE_STEP1_SET_INFORM;
    pub(crate) use crate::game::ai::rider::callee::CREATE_STEP1_SET_PASSWORD;
    pub(crate) use crate::game::ai::rider::callee::CREATE_STEP1_SET_SHARE_INFO;
    pub(crate) use crate::game::ai::rider::callee::CROWD_MAN_GROUP_FREE;
    pub(crate) use crate::game::ai::rider::callee::GAME_UI_OPEN_SCREEN;
    pub(crate) use crate::game::ai::rider::callee::IN_SEQ_PLAY;
    pub(crate) use crate::game::ai::rider::callee::LOBBY_MAN_GET_LANGUAGE;
    pub(crate) use crate::game::ai::rider::callee::LOBBY_MAN_SEND_REQUEST as LOBBY_MAN_SEND_REQUEST_2;
    pub(crate) use crate::game::ai::rider::callee::LOGIN_ON_ACCOUNT_MEM_POPUP;
    pub(crate) use crate::game::ai::rider::callee::LOGIN_ON_DISCONNECT_POPUP;
    pub(crate) use crate::game::ai::rider::callee::MESSAGE_CENTER_GET_LIST_INDEX;
    pub(crate) use crate::game::ai::rider::callee::MESSAGE_CENTER_OPEN_VIEW_POPUP;
    pub(crate) use crate::game::ai::rider::callee::MPEG2_GET_BITS;
    pub(crate) use crate::game::ai::rider::callee::MPEG2_MARKER_BIT;
    pub(crate) use crate::game::ai::rider::callee::NET_ACCOUNT_MEM_CTOR;
    pub(crate) use crate::game::ai::rider::callee::NET_FE_STATE_ON_CHILD_EVENT as NET_FE_STATE_ON_CHILD_EVENT_8;
    pub(crate) use crate::game::ai::rider::callee::RAIL_MAN_ENABLE_RAIL;
    pub(crate) use crate::game::ai::rider::callee::RIDER_GET_START_DIST_TO_FINISH;
    pub(crate) use crate::game::ai::rider::callee::SCRIPT_ENGINE_GET_PROCESS as SCRIPT_ENGINE_GET_PROCESS_2;
    pub(crate) use crate::game::ai::rider::callee::SCRIPT_OBJ_DEACTIVATE as SCRIPT_OBJ_DEACTIVATE_2;
    pub(crate) use crate::game::ai::rider::callee::SHOW_CHAR_STATS;
    pub(crate) use crate::game::ai::rider::callee::SHOW_CHECKING_CARD_POPUP;
    pub(crate) use crate::game::ai::rider::callee::SSX_AUDIO_GET_SURFACE_SOUND_TYPE;
    pub(crate) use crate::game::ai::rider::callee::SSX_AUDIO_PLAY_FE_SOUND;
    pub(crate) use crate::game::ai::rider::callee::SSX_AUDIO_START_BOARD_BASE_LOOP;
    pub(crate) use crate::game::ai::rider::callee::SSX_AUDIO_START_BOARD_EDGE_LOOP;
    pub(crate) use crate::game::ai::rider::callee::SSX_AUDIO_START_BOARD_SURFACE_LOOP;
    pub(crate) use crate::game::ai::rider::callee::SSX_AUDIO_STOP_TRICK_METER_SOUND;
    pub(crate) use crate::game::ai::rider::callee::SSX_AUDIO_STOP_WIPEOUT_LOOP;
    pub(crate) use crate::game::ai::rider::callee::SSX_AUDIO_STOP_WIPEOUT_LOOP2;
    pub(crate) use crate::game::ai::rider::callee::START_CARD_SCAN;
    pub(crate) use crate::game::ai::rider::callee::STREAMER_GET_SECTION_COUNT;
    pub(crate) use crate::game::ai::rider::callee::SYNC_PROCESS_DTOR;
    pub(crate) use crate::game::ai::rider::callee::TEX_MAN_EVICT_TEXTURE;
    pub(crate) use crate::game::ai::rider::callee::VRAM_HEAP_FREE_TEXTURE_BLOCKS;
    pub(crate) use crate::game::ai::rider::callee::WORLD_CACHE_LOAD;
    pub(crate) use crate::game::ai::rider::callee::WORLD_CACHE_RESET;
    pub(crate) use crate::game::ai::rider::callee::WSCRIPT_CACHE_ALLOC;
    pub(crate) use crate::game::be::beintmission::callee::AUDIO_GET_CHANNEL_VOLUME;
    pub(crate) use crate::game::be::beintmission::callee::AUDIO_GET_INSTANCE;
    pub(crate) use crate::game::be::beintmission::callee::AUDIO_IS_GAME_MUSIC_ENABLED;
    pub(crate) use crate::game::be::beintmission::callee::AUDIO_LEVEL_TO_VOLUME;
    pub(crate) use crate::game::be::beintmission::callee::AUDIO_SET_CHANNEL_VOLUME;
    pub(crate) use crate::game::be::beintmission::callee::AUDIO_START_PRERACE_MUSIC;
    pub(crate) use crate::game::be::beintmission::callee::BANK_MONITOR_STOP_ALL;
    pub(crate) use crate::game::be::beintmission::callee::BE_LIBRARY_GET_INSTANCE;
    pub(crate) use crate::game::be::beintmission::callee::BE_LIBRARY_GET_INTERFACE;
    pub(crate) use crate::game::be::beintmission::callee::BE_NEW_PLAYER_GET_INBOX;
    pub(crate) use crate::game::be::beintmission::callee::BUDDY_MAN_GET_ROSTER_ENTRY;
    pub(crate) use crate::game::be::beintmission::callee::CAMERA_TRIGGER_MAN_UPDATE_MASK1;
    pub(crate) use crate::game::be::beintmission::callee::CHASE_CAM_RESET_STATE;
    pub(crate) use crate::game::be::beintmission::callee::CHASE_CAM_SET_ALGORITHM_2 as CHASE_CAM_SET_ALGORITHM;
    pub(crate) use crate::game::be::beintmission::callee::CHASE_CAM_SET_ALGORITHM_IMMEDIATE;
    pub(crate) use crate::game::be::beintmission::callee::CROWD_AUDIO_STOP_ALL;
    pub(crate) use crate::game::be::beintmission::callee::EA_MEMSET;
    pub(crate) use crate::game::be::beintmission::callee::FIND_MISSION;
    pub(crate) use crate::game::be::beintmission::callee::GET_RIDER_PREVIEW;
    pub(crate) use crate::game::be::beintmission::callee::LOBBY_MENU_REQUEST_JOIN_ROOM;
    pub(crate) use crate::game::be::beintmission::callee::MC_MANAGER_END_OPERATION;
    pub(crate) use crate::game::be::beintmission::callee::MUSIC_PLAYER_STOP_SONG;
    pub(crate) use crate::game::be::beintmission::callee::NET_FE_STATE_CTOR;
    pub(crate) use crate::game::be::beintmission::callee::POPUP_CONFIRM_CFG_RESET;
    pub(crate) use crate::game::be::beintmission::callee::RACE_PROGRESS_RESET_ALL;
    pub(crate) use crate::game::be::beintmission::callee::REPLAY_APPLY_CAMERA;
    pub(crate) use crate::game::be::beintmission::callee::REWARD_GALLERY_CHECK_REWARDS_READY;
    pub(crate) use crate::game::be::beintmission::callee::REWARD_GALLERY_POLL_SHAPE_LOAD;
    pub(crate) use crate::game::be::beintmission::callee::RIDER_PREVIEW_SET_VISIBLE;
    pub(crate) use crate::game::be::beintmission::callee::SCRIPT_ENGINE_DEACTIVATE_BINDING;
    pub(crate) use crate::game::be::beintmission::callee::SCRIPT_ENGINE_GET_INSTANCE;
    pub(crate) use crate::game::be::beintmission::callee::SCRIPT_ENGINE_STOP_ALL_PROCESS_SEQS;
    pub(crate) use crate::game::be::beintmission::callee::SCRIPT_ENGINE_STOP_AUX_LISTS as SCRIPT_ENGINE_STOP_AUX_LISTS_2;
    pub(crate) use crate::game::be::beintmission::callee::SCRIPT_ENGINE_STOP_MAIN_LISTS as SCRIPT_ENGINE_STOP_MAIN_LISTS_2;
    pub(crate) use crate::game::be::beintmission::callee::SLOGIN_DELETE;
    pub(crate) use crate::game::be::beintmission::callee::SND_BANK_SET_MEMORY;
    pub(crate) use crate::game::be::beintmission::callee::UI_STATE_UPDATE;
    pub(crate) use crate::game::be::beintmission::callee::WSCRIPT_MAN_EVENT_QUEUE_POP;
    pub(crate) use crate::game::bx::bxstring2::callee::CROWD_MAN2D_REMOVE_ALL;
    pub(crate) use crate::game::bx::bxstring2::callee::CROWD_MAN2D_RESET;
    pub(crate) use crate::game::bx::bxstring2::callee::C_FE_STATE_LOBBY_MENU_FILL_PLAYER_LIST;
    pub(crate) use crate::game::bx::bxstring2::callee::C_FE_STATE_LOBBY_MENU_UPDATE_PLAYER_SCROLL_ARROWS;
    pub(crate) use crate::game::bx::bxstring2::callee::C_LUNO_TABLE_ALLOC;
    pub(crate) use crate::game::bx::bxstring2::callee::C_OV_STATE_OVERLAY_UPDATE;
    pub(crate) use crate::game::bx::bxstring2::callee::EA_MEM_ALLOC_NAMED;
    pub(crate) use crate::game::bx::bxstring2::callee::EVENT_SELECT_ON_ACTIVATE;
    pub(crate) use crate::game::bx::bxstring2::callee::EVENT_SELECT_UPDATE;
    pub(crate) use crate::game::bx::bxstring2::callee::FE_MODULE_RIDER_PREVIEW_REQUEST_CHAR;
    pub(crate) use crate::game::bx::bxstring2::callee::GAME_MODULE_REQUEST_STATE as GAME_MODULE_REQUEST_STATE_2;
    pub(crate) use crate::game::bx::bxstring2::callee::PATH_DATA_PARSE;
    pub(crate) use crate::game::bx::bxstring2::callee::SCRIPT_ENGINE_STOP_AUX_LISTS;
    pub(crate) use crate::game::bx::bxstring2::callee::SCRIPT_ENGINE_STOP_MAIN_LISTS;
    pub(crate) use crate::game::bx::bxstring2::callee::SCRIPT_LIST_GET_ENTRY_TYPE;
    pub(crate) use crate::game::bx::bxstring2::callee::SCRIPT_LIST_PREPARE;
    pub(crate) use crate::game::bx::bxstring2::callee::SCRIPT_LIST_SET_PAUSED_NESTED;
    pub(crate) use crate::game::bx::bxstring2::callee::SECTION_MAN_INIT;
    pub(crate) use crate::game::bx::bxstring2::callee::SNAPSHOT_LIST_REMOVE;
    pub(crate) use crate::game::bx::bxstring2::callee::VID_ENGINE_PLAYER_START_PLAYBACK;
    pub(crate) use crate::game::bx::bxstring2::callee::WORLD_STATE_RESTORE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::AVALANCHE_GET_OBJECT_MATRIX;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_GET_RIDER_PROFILE as BE_GET_RIDER_PROFILE_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_GET_TRACK_INFO;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_MISSION_SET_COLLECTED;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_SET_CHARACTER_BY_SLOT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BUDDY_GET_ROSTER_COUNT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BUDDY_GET_ROSTER_ENTRY;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BUY_POPUP_RESET;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::CLEAR_NEW_REWARDS;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::DYNAMIC_EMITTER_RESET;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::EA_MEM_ALLOC_NAMED_2 as EA_MEM_ALLOC_NAMED_4;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::EA_THREAD_YIELD;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::EA_TIMER_SERVICE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::FE_ASYNC_SET_STATE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::GAME_MODULE_REQUEST_STATE_2 as GAME_MODULE_REQUEST_STATE_3;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::GAME_PROCESS_DTOR;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::GROW_LINES;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::INIT_FROM_RIDER;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::INSTANCE_GET_NODE_MATRIX as INSTANCE_GET_NODE_MATRIX_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::LOAD_INDICATOR_RENDER;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::LOBBY_API_SEND;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::LOBBY_GET_ROOM_NAME;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::MC_END_OPERATION;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::MC_ON_OPERATION_DONE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::MEM_REPORT_STATS_LOCKED;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::MSG_PRINTF;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::OPERATOR_NEW_NAMED as OPERATOR_NEW_NAMED_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::POPUP_CONFIRM_REBUILD;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::QSORT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::RESET_FOR_RIDER;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::RIDER_ADD_BOOST_2 as RIDER_ADD_BOOST;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::RIDER_APPLY_SCRIPT_TRACK;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::RIDER_GATHER_CAM_STATE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::RIDER_GET_MAIN_STATE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::RIDER_RECORD_BEST_JUMP;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::RIDER_UPDATE_SCRIPT_SOUNDS;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SECTION_SELECT_PEAK_LIGHTING;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SERIALIZE_READ_U32;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SET_ITEM_YEAR;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SNOW_BOX_DTOR;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SSX_AUDIO_GET_INSTANCE as SSX_AUDIO_GET_INSTANCE_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SSX_AUDIO_IS_TRACK_GROUP5;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SSX_AUDIO_PLAY_MUSIC;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::SSX_AUDIO_RESUME_SFX;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::STOP_LOADING_SCREEN_LOOP;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::STREAM_FILE_QUEUE_RANGE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::STRLEN;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::STR_LEN;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::TAG_FIELD_COPY;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::TAG_FIELD_SET_RAW;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::TEX_MAN_INIT_TABLE;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_ANIM_CTOR;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_ANIM_SET_CREATE_ANIM as UI_ANIM_SET_CREATE_ANIM_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_ANIM_SET_CREATE_ANIM_2 as UI_ANIM_SET_CREATE_ANIM_2_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_LISTBOX_SET_ITEM_TEXT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_LIST_FIRST as UI_LIST_FIRST_4;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_LIST_FIRST_WRAPPER;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_LIST_NODE_SENTINEL;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_OBJECT_FIND_BY_ID;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_OBJECT_SET_ANIM as UI_OBJECT_SET_ANIM_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_SCREEN_FIND_OBJECT as UI_SCREEN_FIND_OBJECT_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_STATE_CTOR_2_2 as UI_STATE_CTOR_2_3;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_STATE_MGR_PUSH_POPUP;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_TEXT_SET_ASCII;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_TEXT_SET_EVENT_STAT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_TEXT_SET_STRING as UI_TEXT_SET_STRING_2;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::UI_TEXT_SET_TEXT_ID;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::USTR_CPY;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::USTR_LEN;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::VRAM_HEAP_INIT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::WORLD_GET_INSTANCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::ACTIVE_CAMERA_TRIGGER_LIST_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::ALGORITHM_LIST_CLEAR;
    pub(crate) use crate::game::fe::feasyncfile::callee::ALGORITHM_LIST_PUSH_2 as ALGORITHM_LIST_PUSH;
    pub(crate) use crate::game::fe::feasyncfile::callee::ANIM_CONTROLLER_IS_LAYER_ANIM_FINISHED;
    pub(crate) use crate::game::fe::feasyncfile::callee::APP_MODULE_TRY_EXIT_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::ATOI;
    pub(crate) use crate::game::fe::feasyncfile::callee::ATTACHED_EMITTER_FX_DESTROY_EMITTER;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_GET_GAME_MODULE;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_GET_MUSIC_TIER;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_GET_TRACK_TYPE;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_IS_TRACK_GROUP0_EVENT;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_LEVEL_TO_VOLUME_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_PLAY_MUSIC;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_RESET_RACE_RESULT_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_SET_CHANNEL_VOLUME_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_SET_TRIGGER;
    pub(crate) use crate::game::fe::feasyncfile::callee::AUDIO_STORE_RACE_RESULT;
    pub(crate) use crate::game::fe::feasyncfile::callee::AVALANCHE_DESTROY_ALL;
    pub(crate) use crate::game::fe::feasyncfile::callee::AVALANCHE_RESET_ONE;
    pub(crate) use crate::game::fe::feasyncfile::callee::AV_PLAYER_DECODE_FRAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::AV_PLAYER_IS_AUDIO_DONE;
    pub(crate) use crate::game::fe::feasyncfile::callee::AV_PLAYER_IS_FRAME_DUE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BANK_MONITOR_INSTANCE_RESET as BANK_MONITOR_INSTANCE_RESET_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::BANK_MONITOR_KILL_ALL_TIMERS;
    pub(crate) use crate::game::fe::feasyncfile::callee::BANK_MONITOR_KILL_TIMERS_BY_CALLBACK;
    pub(crate) use crate::game::fe::feasyncfile::callee::BENEW_PLAYER_INT_GET_SLOT_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_BACKUP_CHARACTER;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_COPY_PROFILE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_COUNT_COLLECTED_ON_PEAK;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_COUNT_COMPLETED_ON_PEAK;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_APP;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_BE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_CHAR_DISPLAY_NAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_CHAR_PROFILE_EVENT_RECORD;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_INSTANCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_INTERFACE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_RIDER_CHARACTER_2 as BE_GET_RIDER_CHARACTER_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_RIDER_PROFILE as BE_GET_RIDER_PROFILE_5;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_GET_RIDER_PROFILE_5 as BE_GET_RIDER_PROFILE_4;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_INTERFACE_GET_INDEX;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_LIBRARY_GET_INSTANCE_4 as BE_LIBRARY_GET_INSTANCE_5;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_LIBRARY_GET_INTERFACE_4 as BE_LIBRARY_GET_INTERFACE_5;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_MISSION_INT_GET_SESSION_NAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_NEW_PLAYER_INT_GET_CHARACTER_BY_SLOT;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_NEW_PLAYER_INT_GET_HUMAN_RIDER;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_NEW_PLAYER_INT_GET_SLOT_3 as BE_NEW_PLAYER_INT_GET_SLOT_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_NEW_RACE_INT_GET_SPECIAL_EVENT_VALUE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_NEW_RACE_INT_GET_TRACK;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_OPTION_INT_SWAP_PLAYER_FLAGS;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_RESET_PROFILE_CHARACTERS;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_SET_CHAR_PROFILE_FLAG;
    pub(crate) use crate::game::fe::feasyncfile::callee::BE_SET_PHASE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BOLT_FILE_ACQUIRE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BOLT_ON_FX_CLEAR_EMITTERS;
    pub(crate) use crate::game::fe::feasyncfile::callee::BOLT_ON_FX_INIT_HANDLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::BUDDY_API_DESTROY;
    pub(crate) use crate::game::fe::feasyncfile::callee::BUDDY_MAN_GET_ROSTER_COUNT_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::BUDDY_MAN_GET_ROSTER_ENTRY_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::BUILTIN_DELETE_6 as BUILTIN_DELETE_5;
    pub(crate) use crate::game::fe::feasyncfile::callee::BX_STRING_ALLOC_BUFFER;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAMERA_SET_OWNER;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAMERA_TRIGGER_LIST_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAMERA_TRIGGER_MAN_SET_IN_GAME_TRIGGERS;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAMERA_TRIGGER_STACK_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAM_CONTROLLER_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CAM_CONTROLLER_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CBE_STRING_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CBXSTRING_ASSIGN;
    pub(crate) use crate::game::fe::feasyncfile::callee::CFE_STATE_LOGIN_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CFE_STATE_NET_CONFIG_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CFS_FILL_PLAYER_INFO;
    pub(crate) use crate::game::fe::feasyncfile::callee::CFS_GET_CHARACTER_INFO;
    pub(crate) use crate::game::fe::feasyncfile::callee::CFS_SHOW_CHARACTER;
    pub(crate) use crate::game::fe::feasyncfile::callee::CFS_SHOW_OPPONENT_CHAR;
    pub(crate) use crate::game::fe::feasyncfile::callee::CHALLENGE_POPUP_SET_SETTINGS_LOCKED;
    pub(crate) use crate::game::fe::feasyncfile::callee::CHASE_CAMERA_ALGORITHM_INIT_FROM_RIDER;
    pub(crate) use crate::game::fe::feasyncfile::callee::CHASE_CAMERA_ALGORITHM_RESET_FOR_RIDER;
    pub(crate) use crate::game::fe::feasyncfile::callee::CHAT_ROOM_SELECT_PLAYER_BY_NAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::CLEAR_MUSIC_TRIGGER;
    pub(crate) use crate::game::fe::feasyncfile::callee::CROWD_MAN3D_GROUP_LOAD;
    pub(crate) use crate::game::fe::feasyncfile::callee::CROWD_MAN3D_RESET;
    pub(crate) use crate::game::fe::feasyncfile::callee::CUI_TEXT_SCROLL_CLEAR_LINES;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_BX_STRING_ASSIGN_CSTR;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_BX_STRING_CTOR_CSTR;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_COMM_SYSTEM_CHANNEL_RECV;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_COMM_SYSTEM_CHANNEL_SEND;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_CUT_SCENE_TRANSITION_MAN_SAVE_VIEWPORT;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_POPUP_CONFIRM_EMPTY_INIT_HOOK;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_CHAR_EQUIP_BUY_SELECTED_ITEM;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_CHAR_EQUIP_REFRESH_ITEM_LIST;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_CHAR_EQUIP_REMOVE_LIST_ITEM;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_CHAR_EQUIP_UPDATE_MONEY_TEXT;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_NET_CONFIG_LOAD_CONFIG_LIST;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_NET_CONFIG_SET_LIST_VISIBLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_NET_CONFIG_SHOW_CARD_ERROR_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_NET_MESSENGER_CLEAR_NAME_LIST;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_NET_QUICK_PLAY_POPUP_SET_OPTIONS_ENABLED;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_NET_QUICK_PLAY_POPUP_SET_OPTIONS_HINT_VISIBLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_NET_SSX_WORLD_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_OPTIONS_DEVICE_SELECT_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_TROPHY_ROOM_SHOW_TROPHY_IMAGE;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_STATE_TROPHY_ROOM_SHOW_TROPHY_INFO;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_FE_USE_EXISTING_ACCOUNT_CHECK_EMAIL;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_SETUP_MAN_ALLOC_PACKET;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_SETUP_MAN_FREE_PACKET;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_TRANSITION_MAN_START_CAPTURED;
    pub(crate) use crate::game::fe::feasyncfile::callee::DATA_INSTANCE_FREE_INFO;
    pub(crate) use crate::game::fe::feasyncfile::callee::DEAD_NODE_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::DEBUG_MENU_DISPATCH_ITEM_OP;
    pub(crate) use crate::game::fe::feasyncfile::callee::EAFILE_GET_PATH_SIZE_CHECKED;
    pub(crate) use crate::game::fe::feasyncfile::callee::EAFILE_LOAD_FILE_INTO;
    pub(crate) use crate::game::fe::feasyncfile::callee::EA_MEM_ALLOC_NAMED_5;
    pub(crate) use crate::game::fe::feasyncfile::callee::EA_MEM_SET;
    pub(crate) use crate::game::fe::feasyncfile::callee::EDITOR_CAM_ALGO_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::FEMODULE_RIDER_PREVIEW_UNLOAD;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_ASYNC_MANAGER_SET_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MEMCARD_FINISH;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MEMCARD_START_PROFILE_SEARCH;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MODULE_GET_PLAYER_PAD_MASK_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MODULE_GET_RIDER_PREVIEW;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MODULE_RIDER_PREVIEW_RELEASE_BOLT_FILES;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MODULE_RIDER_PREVIEW_SET_VISIBLE as FE_MODULE_RIDER_PREVIEW_SET_VISIBLE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_MODULE_RIDER_PREVIEW_UNLOAD_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_ON_ABORT_POPUP_EVENT;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_ON_EMAIL_POPUP_EVENT;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_ON_LOGIN_POPUP_EVENT;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_ON_SAVE_ACCOUNT_ANSWER;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STATE_CREATE_DUPLICATE_ON_CREATED_POPUP_EVENT;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STATE_CREATE_DUPLICATE_ON_ERROR_POPUP_EVENT;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STATE_CREATE_DUPLICATE_ON_SAVE_ACCOUNT_ANSWER;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STATE_LOBBY_MENU_REFRESH_PLAYER_LIST;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STATE_LOBBY_MENU_SHOW_ROOM_TAB;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STATE_LOBBY_MENU_UPDATE_PLAYERS_IN_ROOM;
    pub(crate) use crate::game::fe::feasyncfile::callee::FE_STRING_HASH;
    pub(crate) use crate::game::fe::feasyncfile::callee::FLAG_MANAGER_ADD_FLAG;
    pub(crate) use crate::game::fe::feasyncfile::callee::FLAG_READ_PARAMS;
    pub(crate) use crate::game::fe::feasyncfile::callee::FLEX_BRIDGE_BUILD_MESH;
    pub(crate) use crate::game::fe::feasyncfile::callee::FLEX_BRIDGE_SPRINGS_LOAD;
    pub(crate) use crate::game::fe::feasyncfile::callee::FLEX_BRIDGE_UPDATE_VERTS;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_ANIM_MAP_SET_HANDLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_LOAD_UPDATE_PLAYER_RANKING;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODE_MAN_RESET_MODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_ADVANCE_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_QUIT_TO_LOAD_SCREEN;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_REQUEST_STATE_5 as GAME_MODULE_REQUEST_STATE_4;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_RESET_RACE;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_SET_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_MODULE_START_NEXT_MODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_PROCESS_DTOR_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_RAND_NEXT;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_STATE_FINISH_CLEAR_FLAG40;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_STATE_IS_LOADING;
    pub(crate) use crate::game::fe::feasyncfile::callee::GAME_STATE_LOAD_STATE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::GET_FLAG_MANAGER;
    pub(crate) use crate::game::fe::feasyncfile::callee::GRAPHICS_MAN_FREE_ALL_TEXTURES;
    pub(crate) use crate::game::fe::feasyncfile::callee::INI_GET_CHAR;
    pub(crate) use crate::game::fe::feasyncfile::callee::INI_PARSER_NEXT_SECTION;
    pub(crate) use crate::game::fe::feasyncfile::callee::INI_SKIP_WHITESPACE;
    pub(crate) use crate::game::fe::feasyncfile::callee::INPUT_BINDING_EVAL_FLOAT;
    pub(crate) use crate::game::fe::feasyncfile::callee::INSEQ_EVENT_TRACK_FIRE_KEY;
    pub(crate) use crate::game::fe::feasyncfile::callee::INSEQ_EVENT_TRACK_NEXT_KEY;
    pub(crate) use crate::game::fe::feasyncfile::callee::INSEQ_INSTANCE_RELEASE;
    pub(crate) use crate::game::fe::feasyncfile::callee::INSEQ_STOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::INSTANCE_GET_NODE_MATRIX;
    pub(crate) use crate::game::fe::feasyncfile::callee::IN_SEQ_CHANNEL_GET_LENGTH;
    pub(crate) use crate::game::fe::feasyncfile::callee::IN_SEQ_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::IN_SEQ_GET_INSTANCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::IN_SEQ_INSTANCE_SET_SPEED;
    pub(crate) use crate::game::fe::feasyncfile::callee::IN_SEQ_INSTANCE_SET_USER_DATA_2 as IN_SEQ_INSTANCE_SET_USER_DATA;
    pub(crate) use crate::game::fe::feasyncfile::callee::IN_SEQ_START_SEQUENCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::KEYBOARD_POPUP_FIND_ENABLED_IN_COLUMN_FROM_TOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::KEYBOARD_POPUP_IS_KEY_ENABLED as KEYBOARD_POPUP_IS_KEY_ENABLED_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::LIGHT_MAN_EMPTY_HOOK_D;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOAD_AGREEMENT_TERMS;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOAD_INDICATOR_CLEAR;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_API_DESTROY_LIST;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_API_FLUSH_ROSTER;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_API_GET_RECORD;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_LIST_GET_AT_2 as LOBBY_LIST_GET_AT_4;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MAN_CONNECT;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MAN_HIDE;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MAN_RELEASE_SNAPSHOT_LISTS;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MAN_SEND_DECLINE;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MAN_SEND_REQUEST;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MAN_SHOW;
    pub(crate) use crate::game::fe::feasyncfile::callee::LUNO_TABLE_FIND;
    pub(crate) use crate::game::fe::feasyncfile::callee::LUNO_VM_CALL;
    pub(crate) use crate::game::fe::feasyncfile::callee::LUNO_VM_PASS_THROUGH;
    pub(crate) use crate::game::fe::feasyncfile::callee::LUNO_VM_REGISTER_AS_ID;
    pub(crate) use crate::game::fe::feasyncfile::callee::LUNO_VM_REGISTER_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::LUNO_VM_REGISTER_DTOR_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::MANUAL_CAM_ALGO_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::MANUAL_CAM_SELECT_ALGORITHM;
    pub(crate) use crate::game::fe::feasyncfile::callee::MC_GET_WORLD;
    pub(crate) use crate::game::fe::feasyncfile::callee::MC_MANAGER_FORMAT_SLOT_NAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::MC_MANAGER_GET as MC_MANAGER_GET_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::MC_MANAGER_GET_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::MC_MANAGER_ON_CARD_SCANNED;
    pub(crate) use crate::game::fe::feasyncfile::callee::MDF_FILE_CREATE_MODEL;
    pub(crate) use crate::game::fe::feasyncfile::callee::MEMCPY_2 as MEMCPY;
    pub(crate) use crate::game::fe::feasyncfile::callee::MEM_ALLOC;
    pub(crate) use crate::game::fe::feasyncfile::callee::MEM_ALLOC_NAMED_128;
    pub(crate) use crate::game::fe::feasyncfile::callee::MEM_CARD_STATE_BASE_SHUTDOWN;
    pub(crate) use crate::game::fe::feasyncfile::callee::MENU_SYS_SET_ACTIVE;
    pub(crate) use crate::game::fe::feasyncfile::callee::MENU_SYS_SET_ACTIVE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::MESSAGE_CENTER_EMPTY_TRIGGER;
    pub(crate) use crate::game::fe::feasyncfile::callee::MESSAGE_CENTER_IS_FLAG_SET;
    pub(crate) use crate::game::fe::feasyncfile::callee::MESSAGE_CENTER_SET_FLAG;
    pub(crate) use crate::game::fe::feasyncfile::callee::MPEG2_EXTENSION_AND_USER_DATA;
    pub(crate) use crate::game::fe::feasyncfile::callee::MPEG2_GET_BITS_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::MPEG2_MARKER_BIT_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::MUSIC_PLAYER_FADE_OUT;
    pub(crate) use crate::game::fe::feasyncfile::callee::MUSIC_PLAYER_PLAY_SONG;
    pub(crate) use crate::game::fe::feasyncfile::callee::MUSIC_PLAYER_RESUME;
    pub(crate) use crate::game::fe::feasyncfile::callee::MUSIC_PLAYER_STOP_SONG_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::NETFE_STATE_GET_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::NETFE_STATE_GOTO_NEXT_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::NETFE_STATE_ON_CHILD_EVENT_DEFAULT;
    pub(crate) use crate::game::fe::feasyncfile::callee::NETFE_STATE_OPEN_KEYBOARD_TEXT_ID;
    pub(crate) use crate::game::fe::feasyncfile::callee::NETFE_STATE_SHOW_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_CTOR_11 as NET_FE_STATE_CTOR_5;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_DTOR_5 as NET_FE_STATE_DTOR_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_GET_POPUP_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_GET_POPUP_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_GET_POPUP_4;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_GOTO_NEXT_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_INIT;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_INIT_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_ON_CHILD_EVENT_10 as NET_FE_STATE_ON_CHILD_EVENT_7;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_ON_CHILD_EVENT_12 as NET_FE_STATE_ON_CHILD_EVENT_9;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_ON_CHILD_EVENT_13 as NET_FE_STATE_ON_CHILD_EVENT_10;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_ON_CHILD_EVENT_9 as NET_FE_STATE_ON_CHILD_EVENT_6;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_ON_CHILD_EVENT_DEFAULT;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_SHOW_DISCONNECTED_POPUP_2 as NET_FE_STATE_SHOW_DISCONNECTED_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_SHOW_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_SHOW_POPUP_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_SHOW_POPUP_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_SHOW_POPUP_5 as NET_FE_STATE_SHOW_POPUP_4;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_FE_STATE_UPDATE_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_GAME_UTIL_ADVERT;
    pub(crate) use crate::game::fe::feasyncfile::callee::NET_GAME_UTIL_CREATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::OPERATOR_NEW;
    pub(crate) use crate::game::fe::feasyncfile::callee::OPERATOR_NEW_NAMED_4 as OPERATOR_NEW_NAMED_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::OVERLAY_RESET;
    pub(crate) use crate::game::fe::feasyncfile::callee::OVERLAY_UPDATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::OVERLAY_UPDATE_CALLEE;
    pub(crate) use crate::game::fe::feasyncfile::callee::OVSCREEN_MESSAGE_CENTER_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::OVSCREEN_PDA_TEMPLATE_CLOSE;
    pub(crate) use crate::game::fe::feasyncfile::callee::PATH_DATA_FIND_MARKER;
    pub(crate) use crate::game::fe::feasyncfile::callee::PATH_PLAT_RESUME;
    pub(crate) use crate::game::fe::feasyncfile::callee::PATH_PLAT_SET_RUNNING;
    pub(crate) use crate::game::fe::feasyncfile::callee::PATH_PLAT_STOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::PATH_PLAT_SUSPEND;
    pub(crate) use crate::game::fe::feasyncfile::callee::POPUP_CFG_SET_OPTION_TEXT_ID;
    pub(crate) use crate::game::fe::feasyncfile::callee::POPUP_CONFIRM_CFG_SET_OPTION_TEXT_ID;
    pub(crate) use crate::game::fe::feasyncfile::callee::POPUP_CONFIRM_CFG_SET_OPTION_TEXT_ID_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::POPUP_CONFIRM_GET_SELECTION;
    pub(crate) use crate::game::fe::feasyncfile::callee::POPUP_SET_OPTION_STRING;
    pub(crate) use crate::game::fe::feasyncfile::callee::POPUP_SET_OPTION_TEXT_ID;
    pub(crate) use crate::game::fe::feasyncfile::callee::PROCESS_LIST_FIRST;
    pub(crate) use crate::game::fe::feasyncfile::callee::PROCESS_LIST_NEXT;
    pub(crate) use crate::game::fe::feasyncfile::callee::PS2_MEM_CARD_CLEAR_DIR_CACHE;
    pub(crate) use crate::game::fe::feasyncfile::callee::PS2_PAD_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::REPLAY_COUNT_SAVE_SNAPSHOTS;
    pub(crate) use crate::game::fe::feasyncfile::callee::REPLAY_GET_WORLD;
    pub(crate) use crate::game::fe::feasyncfile::callee::REWARDS_FILE_PARSE;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_ADD_WIPEOUT_RECOVERY_BOOST;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_BUILD_WORLD_MATRIX;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_CHECK_RESET_PATH;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_GET_CHARACTER;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_GET_START_POSE_ANGLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_IS_LOADED;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_METRIX_ON_RIDER_RESET;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_METRIX_SHOW_HUD_MSG_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_PLACE;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_SCRIPT_OBJ_MATCHES_RIDER_SLOT;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_SET_INPUT_ENABLED;
    pub(crate) use crate::game::fe::feasyncfile::callee::RIDER_UPDATE_BOARD_LEAN;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ANIM_BANK_MAN_REMOVE_BANK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_BINDING_CTOR as SCRIPT_BINDING_CTOR_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_CMD_QUEUE_AT_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_CMD_QUEUE_AT_4;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_DATA_GET_BANK_LIST;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_ADD_SCRIPT_OBJ;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_CHOOSE_SCRIPT as SCRIPT_ENGINE_CHOOSE_SCRIPT_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_CLEAR_SKIP_DELAY_ON_EXIT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_INSTANCE_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_PROCESS_IN_SEQ_TOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_PROCESS_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_PROCESS_STATE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_SEQ_INSTANCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_SEQ_INSTANCE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_GET_TRANSITION_TARGET;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_IS_PLAYING_SCRIPT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_KILL_ALL_PROCESSES;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_POP_FREE_NODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_PUSH_FREE_NODE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_REMOVE_ALL_SCRIPT_OBJS_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_REMOVE_BINDING;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_RESET_PROCESS_TABLE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_SET_BINDING_FILTER;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_SET_OBJ_BINDING_FILTER;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_START_SCRIPT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_STOP_ALL_PROCESS_SEQS_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_TICK_PROCESS;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_ENGINE_UNREGISTER_SCRIPT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_LIST_IS_MOVIE_ENTRY_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_LIST_IS_MOVIE_ENTRY_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_LIST_SET_PAUSED;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_OBJ_ACTIVATE_2 as SCRIPT_OBJ_ACTIVATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_OBJ_ACTIVATE_3 as SCRIPT_OBJ_ACTIVATE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_OBJ_CAM_BUILD_MATRIX_FROM_SCRIPT;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_OBJ_CTOR as SCRIPT_OBJ_CTOR_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_RIDER_GET_PARAMS;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_SOUND_BANK_MAN_REMOVE_BANK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_VIEW_OBJ_ADD_CAM;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_VIEW_OBJ_ON_ACTIVATED;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_VIEW_OBJ_RESET;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_VIEW_OBJ_SET_TRANSITION;
    pub(crate) use crate::game::fe::feasyncfile::callee::SCRIPT_VIEW_OBJ_UPDATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SECTION_TABLE_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::SETUP_BUDDY_ACTION_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_BANK_IS_LOADED_WITH_NAME as SND_BANK_IS_LOADED_WITH_NAME_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_BANK_LOAD as SND_BANK_LOAD_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_BANK_MAN_LOAD_BANK_FROM_MEMORY;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_BANK_MAN_UNLOAD_BANK_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_LIST_FIND_BANK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_LIST_FIND_FREE_ENTRY;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_LOAD_QUEUE_CANCEL_BANK as SND_LOAD_QUEUE_CANCEL_BANK_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_LOAD_QUEUE_CANCEL_BANK_2 as SND_LOAD_QUEUE_CANCEL_BANK_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_LOAD_QUEUE_FLUSH;
    pub(crate) use crate::game::fe::feasyncfile::callee::SND_LOAD_QUEUE_PUSH;
    pub(crate) use crate::game::fe::feasyncfile::callee::SONG_INSTANCE_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::SONG_INSTANCE_PICK_RANDOM_LOOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPCH_MAKE_ID_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPEECH_INSTANCE_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPEECH_SYSTEM_IS_INSTANCE_READY;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPHERE_TREE_COLLIDE_AABB_NODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPHERE_TREE_COLLIDE_SPHERE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPHERE_TREE_COLLIDE_SPHERE_NODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPHERE_TREE_COLLIDE_TREE_NODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPHERE_TREE_COMPUTE_OFFSET;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPRAY_CONFIGURE_FOR_RIDER;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPRAY_UPDATE_EMITTER_ENABLES;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPRINTF_5 as SPRINTF_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPRINTF_6 as SPRINTF_4;
    pub(crate) use crate::game::fe::feasyncfile::callee::SPRINTF_7 as SPRINTF_5;
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_CHARACTER_TO_SPEECH_MASK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_GET_CHANNEL_GAIN;
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_GET_FADE_MASK;
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_GET_INSTANCE_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_IS_SPEECH_VOLUME_ON;
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_SET_FADE_GAIN;
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_SET_MODE;
    pub(crate) use crate::game::fe::feasyncfile::callee::SSX_AUDIO_SYSTEM_STOP_LOADING_SCREEN_LOOP;
    pub(crate) use crate::game::fe::feasyncfile::callee::STRCASECMP;
    pub(crate) use crate::game::fe::feasyncfile::callee::STRCMP;
    pub(crate) use crate::game::fe::feasyncfile::callee::STR_HASH as STR_HASH_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::STR_HASH_5 as STR_HASH_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::SYNC_PROCESS_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::TAG_FIELD_GET_BINARY;
    pub(crate) use crate::game::fe::feasyncfile::callee::TOOLTIP_MANAGER_HIDE_POPUP;
    pub(crate) use crate::game::fe::feasyncfile::callee::TOOLTIP_MANAGER_SET_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::TOOLTIP_MANAGER_SET_TEXT;
    pub(crate) use crate::game::fe::feasyncfile::callee::TRANSITION_EFFECT_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::TRANSITION_EFFECT_DRAW_RECT;
    pub(crate) use crate::game::fe::feasyncfile::callee::TUBE_MENU_BASE_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::TUBE_MENU_ITEM_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::UIENGINE_CLOSE_ALL_POPUPS;
    pub(crate) use crate::game::fe::feasyncfile::callee::UIENGINE_CREATE_SCREEN;
    pub(crate) use crate::game::fe::feasyncfile::callee::UIENGINE_GET_ACTIVE_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::UISCREEN_FIND_OBJECT;
    pub(crate) use crate::game::fe::feasyncfile::callee::UISCREEN_START_THREAD;
    pub(crate) use crate::game::fe::feasyncfile::callee::UITEXTSCROLL_CLEAR_LINES;
    pub(crate) use crate::game::fe::feasyncfile::callee::UITEXTSCROLL_SET_TEXT_ID;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_ADD_SCREEN_BY_HASH_NAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_ANIM_ADVANCE;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_ANIM_APPLY;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_ANIM_SET_CREATE_ANIM;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_ENGINE_ADD_SCREEN_BY_HASH_NAME_2 as UI_ENGINE_ADD_SCREEN_BY_HASH_NAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_ENGINE_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_DELETE_ALL_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_DELETE_ALL_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_FIRST_4 as UI_LIST_FIRST_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_FIRST_6;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_NODE_DTOR_BODY_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_NODE_DTOR_BODY_4;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_NODE_IS_SENTINEL;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_LIST_NODE_IS_SENTINEL_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_MENU_SET_SELECTION_3 as UI_MENU_SET_SELECTION_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_ADD_CHILD;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_CTOR_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_CTOR_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_DTOR_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_LOAD as UI_OBJECT_LOAD_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_SET_ANIM;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_OBJECT_UPDATE_ANIM_2 as UI_OBJECT_UPDATE_ANIM;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_FIND_OBJECT_2 as UI_SCREEN_FIND_OBJECT;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_FIND_OBJECT_4 as UI_SCREEN_FIND_OBJECT_3;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_FIND_OBJECT_5 as UI_SCREEN_FIND_OBJECT_4;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_GET_OBJECT_BY_HASH;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_LAUNCH_THREAD;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_LOOKUP_VALUE;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_PLAY_FRAME_2 as UI_SCREEN_PLAY_FRAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCREEN_SET_OBJECT_VALUE;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_SCROLL_SET_PARTS_VISIBLE;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_STATE_CTOR_2_4;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_STATE_MGR_CLOSE_ALL;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_STATE_MGR_CLOSE_ALL_POPUPS_2 as UI_STATE_MGR_CLOSE_ALL_POPUPS;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_STATE_MGR_CLOSE_TOP_STATES as UI_STATE_MGR_CLOSE_TOP_STATES_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_STATE_MGR_FIND_STATE_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_STATE_MGR_SWITCH_STATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_STRING_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_TEXTURE_SET_FIND;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_TEXT_GET_ID_STRING;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_TEXT_GET_OWNED_STRING;
    pub(crate) use crate::game::fe::feasyncfile::callee::UI_TEXT_SET_TEXT_ID_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::USTR_CPY_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::USTR_LEN_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::USTR_VSPRINTF;
    pub(crate) use crate::game::fe::feasyncfile::callee::U_STR_LEN;
    pub(crate) use crate::game::fe::feasyncfile::callee::U_STR_SPRINTF;
    pub(crate) use crate::game::fe::feasyncfile::callee::U_STR_SPRINTF_ASCII_FMT;
    pub(crate) use crate::game::fe::feasyncfile::callee::U_STR_TO_ASCII;
    pub(crate) use crate::game::fe::feasyncfile::callee::VOIP_MAN_ACTIVATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::VOIP_MAN_DEACTIVATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::VSPRINTF;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_BLOCK_POOL_INIT;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_CACHE_IS_HULL_FREE;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_CACHE_STREAMER_UPDATE_VIEW;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_MEMMAN_CREATE_SECTION_ALLOCATOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_OBJ_CTOR_2_2 as WORLD_OBJ_CTOR_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_OBJ_CTOR_3 as WORLD_OBJ_CTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_SOUND_GET_TRIGGER_IDS;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_STATE_CREATE_OBJECT;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_STATE_FINALIZE;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_TRIGGER_INSTANCE_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::callee::WORLD_TRIGGER_IS_FLAG4;
    pub(crate) use crate::game::fe::feasyncfile::callee::WSCRIPT_CACHE_NODE_DTOR;
    pub(crate) use crate::game::fe::fepopup3::callee::CFE_STATE_CREATE_STEP1_ON_EDIT_DONE_POPUP_EVENT;
    pub(crate) use crate::game::fe::fepopup3::callee::CFE_STATE_CREATE_STEP1_ON_SAVE_ACCOUNT_ANSWER;
    pub(crate) use crate::game::fe::fepopup3::callee::CFE_STATE_LOBBY_MENU_ON_CREATE_ROOM_POPUP_EVENT;
    pub(crate) use crate::game::fe::fepopup3::callee::CFE_STATE_LOBBY_MENU_PROMPT_ROOM_PASSWORD;
    pub(crate) use crate::game::fe::fepopup3::callee::CFE_STATE_NET_QUICK_MESSAGES_ON_RESET_POPUP_EVENT;
    pub(crate) use crate::game::fe::fepopup3::callee::CFE_STATE_NET_QUICK_MESSAGES_ON_SAVE_POPUP_EVENT;
    pub(crate) use crate::game::fe::fepopup3::callee::CFE_STATE_ONLINE_MAIN_MENU_ON_DISCONNECT_POPUP_EVENT;
    pub(crate) use crate::game::fe::fepopup3::callee::CFE_STATE_ONLINE_MAIN_MENU_ON_LEAVE_POPUP_EVENT;
    pub(crate) use crate::game::fe::fepopup3::callee::CFE_STATE_REWARD_GALLERY_RELEASE_TEXTURES;
    pub(crate) use crate::game::fe::fepopup3::callee::CFE_STATE_REWARD_GALLERY_V29;
    pub(crate) use crate::game::fe::fepopup3::callee::FE_ASYNC_MANAGER_GET_STATE;
    pub(crate) use crate::game::fe::fepopup3::callee::FE_LOCALE_UNLOAD_TABLE;
    pub(crate) use crate::game::fe::fepopup3::callee::FE_MODULE_RIDER_PREVIEW_CTOR;
    pub(crate) use crate::game::fe::fepopup3::callee::GAME_MODULE_CTOR;
    pub(crate) use crate::game::fe::fepopup3::callee::HUD_LAYOUT_ICON_TEXT;
    pub(crate) use crate::game::fe::fepopup3::callee::LOBBY_MAN_GET_ROOM_NAME;
    pub(crate) use crate::game::fe::fepopup3::callee::LOBBY_MAN_LOOKUP_PEER_ADDR;
    pub(crate) use crate::game::fe::fepopup3::callee::LOBBY_MAN_MOVE_TO_ROOM;
    pub(crate) use crate::game::fe::fepopup3::callee::MC_MANAGER_END_OPERATION_2 as MC_MANAGER_END_OPERATION_3;
    pub(crate) use crate::game::fe::fepopup3::callee::MC_MANAGER_END_PROGRESS;
    pub(crate) use crate::game::fe::fepopup3::callee::MC_MANAGER_REQUEST_CARD_INFO_2;
    pub(crate) use crate::game::fe::fepopup3::callee::MC_MANAGER_SET_MODE;
    pub(crate) use crate::game::fe::fepopup3::callee::MESSAGE_CENTER_GET_INBOX_3 as MESSAGE_CENTER_GET_INBOX_2;
    pub(crate) use crate::game::fe::fepopup3::callee::NET_FE_STATE_CTOR as NET_FE_STATE_CTOR_4;
    pub(crate) use crate::game::fe::fepopup3::callee::NET_FE_STATE_GET_POPUP;
    pub(crate) use crate::game::fe::fepopup3::callee::NET_FE_STATE_ON_CHILD_EVENT as NET_FE_STATE_ON_CHILD_EVENT_4;
    pub(crate) use crate::game::fe::fepopup3::callee::NET_FE_STATE_SETUP_BUDDY_ACTION_POPUP;
    pub(crate) use crate::game::fe::festaterewards2::callee::ANIM_OBJECT_V36;
    pub(crate) use crate::game::fe::festaterewards2::callee::APP_MODULE_TRY_EXIT;
    pub(crate) use crate::game::fe::festaterewards2::callee::CHAT_ROOM_UPDATE_PLAYER_QUALITY;
    pub(crate) use crate::game::fe::festaterewards2::callee::CROWD_AUDIO_INSTANCE_STOP_SOUND;
    pub(crate) use crate::game::fe::festaterewards2::callee::DL_HEAP_USABLE_SIZE;
    pub(crate) use crate::game::fe::festaterewards2::callee::DYNAMIC_EMITTER_CTOR;
    pub(crate) use crate::game::fe::festaterewards2::callee::EA_MUTEX_LOCK;
    pub(crate) use crate::game::fe::festaterewards2::callee::EA_MUTEX_UNLOCK;
    pub(crate) use crate::game::fe::festaterewards2::callee::FE_STATE_LOGIN_CTOR;
    pub(crate) use crate::game::fe::festaterewards2::callee::FE_STATE_LOGIN_ON_CHILD_EVENT;
    pub(crate) use crate::game::fe::festaterewards2::callee::FE_STATE_ON_ACTIVATE;
    pub(crate) use crate::game::fe::festaterewards2::callee::FE_STATE_STATS_REFRESH_RESULTS;
    pub(crate) use crate::game::fe::festaterewards2::callee::MEMSET;
    pub(crate) use crate::game::fe::festaterewards2::callee::MEM_GET_POOL_BLOCK_SIZE;
    pub(crate) use crate::game::fe::festaterewards2::callee::MEM_MAN_FREE;
    pub(crate) use crate::game::fe::festaterewards2::callee::MEM_REPORT_POOL_STATS;
    pub(crate) use crate::game::fe::festaterewards2::callee::MOUNTAIN_ROOM_UPDATE_EVENT_ICONS;
    pub(crate) use crate::game::fe::festaterewards2::callee::NET_FE_STATE_ON_ACTIVATE;
    pub(crate) use crate::game::fe::festaterewards2::callee::NET_FE_STATE_ON_CHILD_EVENT;
    pub(crate) use crate::game::fe::festaterewards2::callee::NET_FE_STATE_ON_DISCONNECTED_POPUP_EVENT;
    pub(crate) use crate::game::fe::festaterewards2::callee::NET_QUICK_PLAY_POPUP_ON_OPTIONS_EVENT;
    pub(crate) use crate::game::fe::festaterewards2::callee::POPUP_CONFIRM_GET_TEXT_BOX_SIZE;
    pub(crate) use crate::game::fe::festaterewards2::callee::PREVIEW_REWARD_POLL_IMAGE_LOAD;
    pub(crate) use crate::game::fe::festaterewards2::callee::PS2_PAD_CREATE;
    pub(crate) use crate::game::fe::festaterewards2::callee::REPLAY_FIND_NEXT_SNAPSHOT;
    pub(crate) use crate::game::fe::festaterewards2::callee::REPLAY_FIND_PREV_SNAPSHOT;
    pub(crate) use crate::game::fe::festaterewards2::callee::REPLAY_SET_STATE_2;
    pub(crate) use crate::game::fe::festaterewards2::callee::REPLAY_SNAP_TO_SNAPSHOT;
    pub(crate) use crate::game::fe::festaterewards2::callee::REPLAY_STREAM_CTOR;
    pub(crate) use crate::game::fe::festaterewards2::callee::REQUEST_LINE_UPDATE_SCROLL_ARROWS;
    pub(crate) use crate::game::fe::festaterewards2::callee::REQUEST_LINE_UPDATE_SONG_COUNT;
    pub(crate) use crate::game::fe::festaterewards2::callee::REQUEST_LINE_UPDATE_SONG_ICONS;
    pub(crate) use crate::game::fe::festaterewards2::callee::REQUEST_LINE_UPDATE_SONG_NAMES;
    pub(crate) use crate::game::fe::festaterewards2::callee::SCE_USBKB_SET_ARRANGEMENT;
    pub(crate) use crate::game::fe::festaterewards2::callee::SCE_USBKB_SET_READ_MODE;
    pub(crate) use crate::game::fe::festaterewards2::callee::SCRIPT_CMD_QUEUE_AT;
    pub(crate) use crate::game::fe::festaterewards2::callee::SCRIPT_ENGINE_LIST_STOP;
    pub(crate) use crate::game::fe::festaterewards2::callee::SCRIPT_ENGINE_UNLOAD_WORLD_SCRIPTS;
    pub(crate) use crate::game::fe::festaterewards2::callee::SCRIPT_LIST_ADD_MOVIE;
    pub(crate) use crate::game::fe::festaterewards2::callee::SCRIPT_LIST_POLL_LOADING;
    pub(crate) use crate::game::fe::festaterewards2::callee::SECTION_MAN_FIND_LOGICAL_SECTION;
    pub(crate) use crate::game::fe::festaterewards2::callee::SONG_INSTANCE_GET_DUCK_TO_LOOPS;
    pub(crate) use crate::game::fe::festaterewards2::callee::SPRINTF;
    pub(crate) use crate::game::fe::festaterewards2::callee::S_LOGIN_UPDATE;
    pub(crate) use crate::game::fe::festaterewards2::callee::TROPHY_ROOM_LOAD_TROPHY_TEXTURES;
    pub(crate) use crate::game::fe::festaterewards2::callee::TWSCRIPT_CACHE_RELOCATE;
    pub(crate) use crate::game::fe::festaterewards2::callee::UI_STATE_CTOR;
    pub(crate) use crate::game::fe::festaterewards2::callee::UI_STATE_UPDATE as UI_STATE_UPDATE_2;
    pub(crate) use crate::game::fe::festaterewards2::callee::UI_TEXT_SCROLL_CLEAR_LINES;
    pub(crate) use crate::game::fe::festaterewards2::callee::UI_TEXT_SCROLL_SET_WIDE_STRING;
    pub(crate) use crate::game::fe::festaterewards2::callee::UI_TEXT_SET_STRING;
    pub(crate) use crate::game::fe::festaterewards2::callee::USB_KB_DEFAULT_CONNECT_CB;
    pub(crate) use crate::game::fe::festaterewards2::callee::WORLD_STATE_STREAM_RESET;
    pub(crate) use crate::game::fe::festaterewards2::callee::WORLD_TRIGGER_MANAGER_GET_TRIGGER_INFO;
    pub(crate) use crate::game::main::gamestate::callee::ADF_DATA_SOURCE_DTOR;
    pub(crate) use crate::game::main::gamestate::callee::ANIM_DATA_BASE_FREE_CHAN_CACHES;
    pub(crate) use crate::game::main::gamestate::callee::BANK_MONITOR_ADD_TIMER;
    pub(crate) use crate::game::main::gamestate::callee::BANK_MONITOR_STOP_HANDLE as BANK_MONITOR_STOP_HANDLE_2;
    pub(crate) use crate::game::main::gamestate::callee::BE_ECON_INT_GET_TOTAL_EARNED;
    pub(crate) use crate::game::main::gamestate::callee::BE_GET_RIDER_PROFILE;
    pub(crate) use crate::game::main::gamestate::callee::BE_LIBRARY_GET_INSTANCE_3 as BE_LIBRARY_GET_INSTANCE_4;
    pub(crate) use crate::game::main::gamestate::callee::BE_LIBRARY_GET_INTERFACE_3 as BE_LIBRARY_GET_INTERFACE_4;
    pub(crate) use crate::game::main::gamestate::callee::BE_NEW_PLAYER_INT_GET_SLOT;
    pub(crate) use crate::game::main::gamestate::callee::BOLTS_FIND_BOLT_INDEX;
    pub(crate) use crate::game::main::gamestate::callee::BUILTIN_DELETE_4;
    pub(crate) use crate::game::main::gamestate::callee::C_FE_STATE_BRAGGING_RIGHTS_INIT_PLAYER_ROWS;
    pub(crate) use crate::game::main::gamestate::callee::C_FE_STATE_ONLINE_CHARACTER_SELECT_SHOW_CHAR_STATS;
    pub(crate) use crate::game::main::gamestate::callee::C_FE_STATE_ONLINE_CHARACTER_SELECT_SHOW_OPPONENT_CHAR;
    pub(crate) use crate::game::main::gamestate::callee::C_GAME_COMM_RECV_GAME_PACKET;
    pub(crate) use crate::game::main::gamestate::callee::C_KEYBOARD_POPUP_SET_KEY_ENABLED;
    pub(crate) use crate::game::main::gamestate::callee::EA_MEM_ALLOC_NAMED_3;
    pub(crate) use crate::game::main::gamestate::callee::EA_MEM_FREE_4;
    pub(crate) use crate::game::main::gamestate::callee::GAME_ANIM_MAP_UNREGISTER_BANK;
    pub(crate) use crate::game::main::gamestate::callee::GAME_STATE_LOAD_STATE;
    pub(crate) use crate::game::main::gamestate::callee::INTC_TABLE_CLEAR_SLOTS;
    pub(crate) use crate::game::main::gamestate::callee::INTC_TABLE_INSTALL_VIF_VU_HANDLERS;
    pub(crate) use crate::game::main::gamestate::callee::MEMCARD_STATE_BASE_INSTALL_CALLBACKS;
    pub(crate) use crate::game::main::gamestate::callee::MEM_FREE;
    pub(crate) use crate::game::main::gamestate::callee::MEM_MOVE;
    pub(crate) use crate::game::main::gamestate::callee::MENU_SYS_ITEM_GET_INPUT_CODE;
    pub(crate) use crate::game::main::gamestate::callee::MENU_SYS_ITEM_HANDLE_INPUT;
    pub(crate) use crate::game::main::gamestate::callee::MENU_SYS_ITEM_IS_DISABLED;
    pub(crate) use crate::game::main::gamestate::callee::MENU_SYS_PAGE_SELECT_NEXT as MENU_SYS_PAGE_SELECT_NEXT_2;
    pub(crate) use crate::game::main::gamestate::callee::MENU_SYS_PAGE_SELECT_PREV;
    pub(crate) use crate::game::main::gamestate::callee::MENU_SYS_PAGE_VALIDATE_SELECTION;
    pub(crate) use crate::game::main::gamestate::callee::MESSAGE_CENTER_COUNT_GROUP_MESSAGES;
    pub(crate) use crate::game::main::gamestate::callee::MESSAGE_CENTER_GET_MESSAGE_COUNT;
    pub(crate) use crate::game::main::gamestate::callee::MESSAGE_CENTER_REMOVE_MESSAGE;
    pub(crate) use crate::game::main::gamestate::callee::NET_FE_STATE_ON_CHILD_EVENT_3 as NET_FE_STATE_ON_CHILD_EVENT_5;
    pub(crate) use crate::game::main::gamestate::callee::NET_FE_STATE_OPEN_KEYBOARD_TEXT_ID;
    pub(crate) use crate::game::main::gamestate::callee::OV_SCREEN_MESSAGE_CENTER_REFRESH_LIST;
    pub(crate) use crate::game::main::gamestate::callee::OV_SCREEN_MESSAGE_CENTER_UPDATE_COUNT_TEXT;
    pub(crate) use crate::game::main::gamestate::callee::OV_SCREEN_MESSAGE_CENTER_UPDATE_SCROLL_ARROWS;
    pub(crate) use crate::game::main::gamestate::callee::RIDER_CLEAR_NEARBY_LIGHTS;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_BINDING_PAUSE_SEQ;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_BINDING_RESUME_SEQ;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_CMD_QUEUE_AT_2;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_ENGINE_GET_PROCESS_IN_SEQ;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_ENGINE_GET_SCRIPT_OBJ;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_ENGINE_IS_OBJ_BOUND_TO_PROCESS;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_ENGINE_LIST_START;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_ENGINE_LIST_STOP_3;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_LIST_IS_MOVIE_ENTRY;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_OBJ_CAM_HAS_BINDING;
    pub(crate) use crate::game::main::gamestate::callee::SCRIPT_OBJ_CAM_IS_BOUND_TO_PROCESS;
    pub(crate) use crate::game::main::gamestate::callee::SECTION_ALLOCATOR_CTOR;
    pub(crate) use crate::game::main::gamestate::callee::SNDSTRM_CLOSE;
    pub(crate) use crate::game::main::gamestate::callee::SPCH_MAKE_ID;
    pub(crate) use crate::game::main::gamestate::callee::SPCH_SHUTDOWN;
    pub(crate) use crate::game::main::gamestate::callee::SPEECH_INSTANCE_DTOR;
    pub(crate) use crate::game::main::gamestate::callee::SPEECH_SYSTEM_SET_EVENT_CONTEXT;
    pub(crate) use crate::game::main::gamestate::callee::SPHERE_TREE_COLLIDE_AABB;
    pub(crate) use crate::game::main::gamestate::callee::SPHERE_TREE_COLLIDE_SPHERE_TREE;
    pub(crate) use crate::game::main::gamestate::callee::SSX_AUDIO_SYSTEM_GET_CHANNEL_GAIN_PTR;
    pub(crate) use crate::game::main::gamestate::callee::SSX_AUDIO_SYSTEM_GET_INSTANCE_2;
    pub(crate) use crate::game::main::gamestate::callee::SSX_AUDIO_SYSTEM_GET_LANGUAGE_MASK;
    pub(crate) use crate::game::main::gamestate::callee::SSX_AUDIO_SYSTEM_IS_DJ_SPEECH_ON;
    pub(crate) use crate::game::main::gamestate::callee::SSX_AUDIO_SYSTEM_RESET_RACE_SOUNDS;
    pub(crate) use crate::game::main::gamestate::callee::STR_COPY;
    pub(crate) use crate::game::main::gamestate::callee::STR_HASH;
    pub(crate) use crate::game::main::gamestate::callee::STR_N_COPY;
    pub(crate) use crate::game::main::gamestate::callee::TAG_FIELD_SET_STRING;
    pub(crate) use crate::game::main::gamestate::callee::UI_LIST_FIRST_2;
    pub(crate) use crate::game::main::gamestate::callee::UI_LIST_NODE_DTOR_BODY;
    pub(crate) use crate::game::main::gamestate::callee::UI_STATE_CTOR_3 as UI_STATE_CTOR_4;
    pub(crate) use crate::game::main::gamestate::callee::UI_STATE_MGR_FIND_STATE;
    pub(crate) use crate::game::main::gamestate::callee::UI_STATE_SHUTDOWN_2;
    pub(crate) use crate::game::main::gamestate::callee::VD_TEXTURE_SINK_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::ANIM_OBJECT_SAVE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::ATTACHED_DYNAMIC_EMITTER_CREATE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::AUDIO_SYSTEM_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BANK_MONITOR_RESET;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BANK_MONITOR_RESET_INSTANCE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BELIBRARY_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BE_GET_RIDER_CHARACTER as BE_GET_RIDER_CHARACTER_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BE_GET_RIDER_PROFILE as BE_GET_RIDER_PROFILE_3;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BE_REWARD_INT_IS_PEAK_GOAL_COMPLETE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BUDDY_MAN_CLEAR_LOGIN;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BUDDY_MAN_CLEAR_MESSAGES;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BUDDY_MAN_DISCONNECT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BUILD_BUDDY_LIST;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CAMERA_ALGORITHM_DEFAULT_FOV;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CBX_STRING_CONCAT_COPY;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CBX_STRING_CTOR_COPY;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CBX_STRING_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CHANNEL_GET_TRACK;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CROWD_AUDIO_INSTANCE_REMOVE_ALL_SOURCES;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::CROWD_MAN2D_CLEAR_SLOT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::C_BX_STRING_COPY_BEFORE_WRITE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::C_FLAG_MANAGER_GROUP_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::C_TRANSITION_MAN_START;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::FE_LOCALE_UNLOAD_TABLE as FE_LOCALE_UNLOAD_TABLE_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::FIND_FREE_BANK_SLOT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::GAME_PROCESS_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::GAME_STATE_GOTO_NEXT_3;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::GET_CHANNEL_VOLUME;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::GET_RACE_MAN;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::GET_SELECTED_USER;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::GOTO_NEXT_STATE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::HANDLE_NET_EVENT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::INPUT_SYNC_PACK_SAMPLE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::INPUT_SYNC_UNPACK_SAMPLE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::INTC_TABLE_ADD_HANDLER;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::IN_SEQ_INSTANCE_GET_REMAINING_STEPS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::KEYBOARD_POPUP_MASK_TEXT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::KEYBOARD_POPUP_UPDATE_CURSOR_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::KEYBOARD_POPUP_UPDATE_SCROLL_WINDOW;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::LEVEL_TO_VOLUME;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::LINE_STRIP_ADD_VERTEX;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::LINE_STRIP_CLOSE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::LIST_GET_ENTRY_TYPE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::LOAD_BANK_ASYNC;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MATH_EVAL_CUBIC;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MC_MANAGER_GET;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MEM_CARD_BASE_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MEM_CARD_ON_CHILD_EVENT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MEM_MAN_FREE_ENTRY;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MENU_SYS_ITEM_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MENU_SYS_ITEM_DRAW_ROW_4 as MENU_SYS_ITEM_DRAW_ROW_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MOD_OBJ_CREATE_MODIFIER_BLOCK;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::MUSIC_PLAYER_GET_SELECTED_SONG_SED_VALUE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::ONLINE_CHARACTER_SELECT_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::ON_CHILD_EVENT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::OV_SCREEN_PROFILE_SAVE_PDA_REFRESH_CARD_INFO;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::OV_SCREEN_PROFILE_SAVE_PDA_RESET_SLOT_LIST;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::PATH_PLAT_FIND_BANK_SLOT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::RAIL_MODIFIER_LIST_PUSH_FRONT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::REFRESH_PLAYER_LIST;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::REPLAY_END_SAVE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::REPLAY_SAVE_EMIT as REPLAY_SAVE_EMIT_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::REPLAY_SET_CONTROLLER_INDEX;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::REPLAY_STREAM_REWIND_READ;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::REQUEST_IMAGE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::RIDER_HAS_RENDER_SLOT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::RIDER_SAY_20B9;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_BINDING_CTOR_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_BINDING_CTOR_4;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_DATA_GET_INFO;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_DATA_GET_INFO_4 as SCRIPT_DATA_GET_INFO_3;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_ENGINE_GET_INSTANCE_SCRIPT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_ENGINE_LIST_STOP_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_CAN_SKIP;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_GET_PLAY_STATUS_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_IS_PLAYING_SCRIPT_3;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_START;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_LIST_START_TRANSITION;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_OBJ_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SCRIPT_OBJ_CTOR_3;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SEND_PACKET;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SETUP_MAN_SET_LOCAL_CHARACTER;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SET_CHANNEL_VOLUME;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SET_SELECTED_USER;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SHOW_BUDDY_ACTION_POPUP;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SND_BANK_IS_LOADED_WITH_NAME;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SND_BANK_LOAD;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SND_LOAD_QUEUE_CANCEL_BANK;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SPEECH_SYSTEM_CANCEL_PENDING_INSTANCE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SSX_AUDIO_SYSTEM_DJ_SAY_20E5;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::START_RIDER_BOARD_LOOPS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::STOP_BOARD_LOOPS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::STOP_TRANSITION;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SYNC_PROCESS_SAVE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::TRANSITION_MAN_SCRIPT_SET_OWNER;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::TRANSITION_MAN_STOP_TRANSITION;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_ASCII_TO_WIDE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_LIST_CTOR_2 as UI_LIST_CTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_LIST_DELETE_ALL;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_LIST_FIRST_5;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_LIST_NODE_DTOR_BODY as UI_LIST_NODE_DTOR_BODY_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_MENU_SET_SELECTION;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_STATE_CTOR_2_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::UI_TEXT_SET_WIDE_STRING_INTERNAL;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::VEC3_NEG_DOT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::VID_ENGINE_INSTALL_MEM_HOOKS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::VID_ENGINE_PLAYER_INIT_OPEN_PARAMS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::WORLD_OBJ_REPLACE_WITH_DEAD_NODE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::WORLD_OBJ_SAVE;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::WSCRIPT_MAN_DEACTIVATE_PROCESS;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::WSCRIPT_MAN_UNLOAD_SECTION;
    pub(crate) use crate::game::sound::bankmonitor::callee::ADF_DATA_SOURCE_SET_DATA;
    pub(crate) use crate::game::sound::bankmonitor::callee::AUDIO_RAND;
    pub(crate) use crate::game::sound::bankmonitor::callee::BANK_MONITOR_TIMER_KILL;
    pub(crate) use crate::game::sound::bankmonitor::callee::BE_GET_EVENT_RECORD_INDEX;
    pub(crate) use crate::game::sound::bankmonitor::callee::BE_LIBRARY_GET_INSTANCE_3;
    pub(crate) use crate::game::sound::bankmonitor::callee::BE_LIBRARY_GET_INTERFACE as BE_LIBRARY_GET_INTERFACE_3;
    pub(crate) use crate::game::sound::bankmonitor::callee::BE_MISSION_INT_GET_COLLECTED_COUNT;
    pub(crate) use crate::game::sound::bankmonitor::callee::BOLTS_FREE_DB;
    pub(crate) use crate::game::sound::bankmonitor::callee::BUDDY_MAN_CONNECT;
    pub(crate) use crate::game::sound::bankmonitor::callee::BUILTIN_DELETE_4 as BUILTIN_DELETE_3;
    pub(crate) use crate::game::sound::bankmonitor::callee::CROWD_MAN3D_GROUP_CREATE_MESHES;
    pub(crate) use crate::game::sound::bankmonitor::callee::EA_FILE_LOAD_DECOMPRESSED;
    pub(crate) use crate::game::sound::bankmonitor::callee::GAME_ANIM_MAP_FIND_HASH;
    pub(crate) use crate::game::sound::bankmonitor::callee::GAME_STATE_GOTO_NEXT_2;
    pub(crate) use crate::game::sound::bankmonitor::callee::LOBBY_LIST_GET_AT as LOBBY_LIST_GET_AT_2;
    pub(crate) use crate::game::sound::bankmonitor::callee::MEMCARD_REQUEST_GET_INFO_NOW;
    pub(crate) use crate::game::sound::bankmonitor::callee::MEMSET_2;
    pub(crate) use crate::game::sound::bankmonitor::callee::MENU_SYS_ITEM_DRAW_ROW;
    pub(crate) use crate::game::sound::bankmonitor::callee::MUSIC_PLAYER_GET_SONG_TAG;
    pub(crate) use crate::game::sound::bankmonitor::callee::PATH_DATA_READ_FLOAT_2;
    pub(crate) use crate::game::sound::bankmonitor::callee::PATH_DATA_READ_INT;
    pub(crate) use crate::game::sound::bankmonitor::callee::PATH_DATA_READ_VEC3;
    pub(crate) use crate::game::sound::bankmonitor::callee::PS2_MEMCARD_CREATE;
    pub(crate) use crate::game::sound::bankmonitor::callee::RIDER_METRIX_CLEAR_PEAK_A4;
    pub(crate) use crate::game::sound::bankmonitor::callee::RIDER_METRIX_SHOW_HUD_NUMBER;
    pub(crate) use crate::game::sound::bankmonitor::callee::SCRIPT_BINDING_CTOR;
    pub(crate) use crate::game::sound::bankmonitor::callee::SCRIPT_ENGINE_GET_INSTANCE as SCRIPT_ENGINE_GET_INSTANCE_2;
    pub(crate) use crate::game::sound::bankmonitor::callee::SCRIPT_ENGINE_POLL_ASYNC_LOADS;
    pub(crate) use crate::game::sound::bankmonitor::callee::SCRIPT_LIST_ADD_CHOICE;
    pub(crate) use crate::game::sound::bankmonitor::callee::SECTION_ALLOCATOR_DTOR;
    pub(crate) use crate::game::sound::bankmonitor::callee::SLOGIN_MC_GET_PORT_SLOT;
    pub(crate) use crate::game::sound::bankmonitor::callee::SNDSTRM_SET_PREBUFFER;
    pub(crate) use crate::game::sound::bankmonitor::callee::SND_BANK_MAN_UNLOAD_MEMORY_BANK;
    pub(crate) use crate::game::sound::bankmonitor::callee::SONG_INSTANCE_GET_REQUEST_STATUS;
    pub(crate) use crate::game::sound::bankmonitor::callee::SPEECH_SYSTEM_STOP_INSTANCE;
    pub(crate) use crate::game::sound::bankmonitor::callee::SSX_AUDIO_SYSTEM_END_SCRIPT_MUSIC;
    pub(crate) use crate::game::sound::bankmonitor::callee::SSX_AUDIO_SYSTEM_GET_INSTANCE;
    pub(crate) use crate::game::sound::bankmonitor::callee::STREAM_FILE_RELEASE_CHUNK;
    pub(crate) use crate::game::sound::bankmonitor::callee::STR_CASE_CMP;
    pub(crate) use crate::game::sound::bankmonitor::callee::TEX_MAN_FREE_TEXTURE;
    pub(crate) use crate::game::sound::bankmonitor::callee::UI_LIST_COUNT;
    pub(crate) use crate::game::sound::bankmonitor::callee::UI_MENU_SET_NAV_BUTTONS;
    pub(crate) use crate::game::sound::bankmonitor::callee::USTR_TO_ASCII;
    pub(crate) use crate::game::sound::bankmonitor::callee::WORLD_PAINTER_SET_SCRIPT_SURFACE_MODE;
    pub(crate) use crate::game::sound::bankmonitor::callee::WSCRIPT_MAN_CALL_TASK_FUNC;
    pub(crate) use crate::game::sound::bankmonitor::callee::WSCRIPT_PROCESS_LIST_PUSH_FRONT;
    pub(crate) use crate::game::sound::bankmonitor::callee::WSCRIPT_PROCESS_LIST_REMOVE;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::BUILTIN_DELETE;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::CBXSTRING_ASSIGN_CSTR;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::C_WORLD_SOUND_UNLINK_EMITTERS;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::EA_MEM_COPY;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::EVENT_SELECT_HANDLE_INPUT;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::GRAPHICS_MAN_EMPTY_HOOK;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::IRRADIANCE_REF_GET_ENTRY;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::LOBBY_MAN_REQUEST_STATS_PAGE;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::MC_MANAGER_GET_INSTANCE;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::MENU_SYS_GET_INPUT_CODE;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::PATH_DATA_READ_FLOAT;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::PATH_SET_TRACK_CONTROL_VALUE;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::RAND_NEXT;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::RAND_NEXT_MAIN;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::RIDER_METRIX_BANK_COMBO;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::SCRIPT_DUMMY_OBJ_READ_FIELDS;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::SCRIPT_ENGINE_PUSH_FREE_NODE;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::SCRIPT_OBJ_DEACTIVATE;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::SSX_APP_BOOT_LOAD;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::STATS_PROCESS_RENDER_STUB;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::STATS_PROCESS_UPDATE_STUB;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::STREAM_BUFFER_GET_STATUS;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::STRLEN_AND_STRCPY;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::TEX_MAN_SET_ALLOC_FLAGS;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::UI_LIST_FIRST;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::UI_LIST_NODE_UNLINK;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::UI_LIST_PUSH_BACK;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::UI_STATE_MGR_RENDER_STUB;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::USTR_FROM_ASCII;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::WORLD_RESET_SECTIONS;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::WORLD_TRIGGER_GET_INFO;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::WORLD_TRIGGER_GET_NAME;

    pub const CAMERA_TRIGGER_STACK_CLEAR: Callee = Callee {
        addr: 0x0016_d200,
        f: ee::s02::CameraTriggerStack_Clear,
    };

    pub const BOLT_FILE_INDEX_MANAGER_RESET: Callee = Callee {
        addr: 0x0019_cb60,
        f: ee::s03::BoltFileIndexManager_Reset,
    };

    pub const FE_MODULE_RIDER_PREVIEW_SET_VISIBLE: Callee = Callee {
        addr: 0x0019_e538,
        f: ee::s03::FEModule_RiderPreviewSetVisible,
    };

    pub const EVENT_SELECT_ON_WIDGET_NOTIFY: Callee = Callee {
        addr: 0x0020_2738,
        f: ee::s05::EventSelect_OnWidgetNotify,
    };

    pub const GAME_STATE_GOTO_NEXT: Callee = Callee {
        addr: 0x0023_3aa0,
        f: ee::s06::GameState_GotoNext,
    };

    pub const GAME_MODULE_REQUEST_STATE: Callee = Callee {
        addr: 0x0023_1250,
        f: ee::s06::GameModule_RequestState,
    };

    pub const MEM_ALLOC_NAMED_ALIGNED: Callee = Callee {
        addr: 0x0025_2980,
        f: ee::s06::MEM_AllocNamedAligned,
    };

    pub const LOBBY_MAN_SEND_LOCK_CHAL: Callee = Callee {
        addr: 0x0025_fc70,
        f: ee::s06::LobbyMan_SendLockChal,
    };

    pub const C_SETUP_MAN_DTOR: Callee = Callee {
        addr: 0x0026_4138,
        f: ee::s07::cSetupMan_dtor,
    };

    pub const C_SHARED_LOGIN_MAN_DTOR: Callee = Callee {
        addr: 0x0026_5908,
        f: ee::s07::cSharedLoginMan_dtor,
    };

    pub const C_VOIP_MAN_DTOR: Callee = Callee {
        addr: 0x0026_6d10,
        f: ee::s07::cVoipMan_dtor,
    };

    pub const SCRIPT_LIST_GET_PLAY_STATUS: Callee = Callee {
        addr: 0x0027_6bd8,
        f: ee::s07::ScriptList_GetPlayStatus,
    };

    pub const SCRIPT_LIST_COUNT: Callee = Callee {
        addr: 0x0027_6ca8,
        f: ee::s07::ScriptList_Count,
    };

    pub const SCRIPT_ENGINE_CHOOSE_SCRIPT: Callee = Callee {
        addr: 0x0027_b0c0,
        f: ee::s07::ScriptEngine_ChooseScript,
    };

    pub const SCRIPT_ENGINE_GET_PROCESS: Callee = Callee {
        addr: 0x0028_2cb0,
        f: ee::s07::ScriptEngine_GetProcess,
    };

    pub const IN_SEQ_CHANNEL_SET_LISTENER: Callee = Callee {
        addr: 0x0027_4240,
        f: ee::s07::InSeq_ChannelSetListener,
    };

    pub const IN_SEQ_INSTANCE_PAUSE: Callee = Callee {
        addr: 0x0027_49e8,
        f: ee::s07::InSeq_InstancePause,
    };

    pub const IN_SEQ_INSTANCE_RESUME: Callee = Callee {
        addr: 0x0027_4a08,
        f: ee::s07::InSeq_InstanceResume,
    };

    pub const SCRIPT_CMD_QUEUE_CLEAR: Callee = Callee {
        addr: 0x0028_3c20,
        f: ee::s07::ScriptCmdQueue_Clear,
    };

    pub const SND_BANK_UNLOAD: Callee = Callee {
        addr: 0x0028_b528,
        f: ee::s07::SndBank_Unload,
    };

    pub const SND_BANK_FREE_MEMORY: Callee = Callee {
        addr: 0x0028_b650,
        f: ee::s07::SndBank_FreeMemory,
    };

    pub const SND_BANK_UNLOAD_MEMORY: Callee = Callee {
        addr: 0x0028_b7e0,
        f: ee::s07::SndBank_UnloadMemory,
    };

    pub const SND_BANK_RELEASE: Callee = Callee {
        addr: 0x0028_b878,
        f: ee::s07::SndBank_Release,
    };

    pub const SND_BANK_GET_BUFFER_SIZE: Callee = Callee {
        addr: 0x0028_b928,
        f: ee::s07::SndBank_GetBufferSize,
    };

    pub const BANK_MONITOR_INSTANCE_RESET: Callee = Callee {
        addr: 0x002a_bb38,
        f: ee::s08::BankMonitorInstance_Reset,
    };

    /// Cross-shard targets (`call_indirect` in the generated body).
    pub const OPERATOR_DELETE: u32 = 0x0031_7e50;

    pub const LOAD_INDICATOR_CLEAR_BUSY: u32 = 0x001f_36d8;

    pub const WSCRIPT_MAN_DELETE_ALL_PROCESSES: u32 = 0x0030_8f38;

    pub const WSCRIPT_MAN_SPAWN_MISSIONS: u32 = 0x0030_8c60;

    pub const UI_STATE_DTOR: u32 = 0x0039_e390;

    pub const MC_REQUEST_CLOSE: u32 = 0x002c_6a08;

    pub const MC_REQUEST_READ: u32 = 0x002c_6b50;

    pub const MC_REQUEST_WRITE: u32 = 0x002c_6bd0;

    pub const MC_REQUEST_FLUSH: u32 = 0x002c_6ae0;

    pub const STRCPY: u32 = 0x0041_66f4;

    pub const SECTION_MAN_GET_WORLD_SECTION_INDEX: u32 = 0x0022_e0c8;

    pub const LOBBY_LIST_COUNT: Callee = Callee {
        addr: 0x003e_e550,
        f: ee::s14::Lobby_ListGetCount,
    };

    pub const LOBBY_LIST_VISIBLE_COUNT: Callee = Callee {
        addr: 0x003e_e558,
        f: ee::s14::Lobby_ListGetVisibleCount,
    };

    pub const LOBBY_LIST_GET_AT: Callee = Callee {
        addr: 0x003e_e560,
        f: ee::s14::Lobby_ListGetAt,
    };

    pub const BUDDY_FIND_ROSTER_ENTRY: Callee = Callee {
        addr: 0x0026_2468,
        f: ee::s07::BuddyMan_FindRosterEntry,
    };

    pub const BUDDY_IS_BUDDY_AT: Callee = Callee {
        addr: 0x0026_2390,
        f: ee::s07::BuddyMan_IsBuddyAt,
    };

    pub const BUDDY_IS_BLOCKED_AT: Callee = Callee {
        addr: 0x0026_23e0,
        f: ee::s07::BuddyMan_IsBlockedAt,
    };

    pub const BUDDY_IS_TEMP_AT: Callee = Callee {
        addr: 0x0026_2400,
        f: ee::s07::BuddyMan_IsTempAt,
    };

    pub const BUDDY_IS_ONLINE_AT: Callee = Callee {
        addr: 0x0026_3338,
        f: ee::s07::BuddyMan_IsOnlineAt,
    };

    pub const BUDDY_IS_SAME_PRODUCT_AT: Callee = Callee {
        addr: 0x0026_33f8,
        f: ee::s07::BuddyMan_IsSameProductAt,
    };

    pub const BUDDY_COUNT_MESSAGES_FROM_AT: Callee = Callee {
        addr: 0x0026_2950,
        f: ee::s07::BuddyMan_CountMessagesFromAt,
    };

    pub const INSEQ_GET_INSTANCE: Callee = Callee {
        addr: 0x0027_2cc0,
        f: ee::s07::InSeq_GetInstance,
    };

    pub const INSEQ_INSTANCE_START: Callee = Callee {
        addr: 0x0027_4830,
        f: ee::s07::InSeq_InstanceStart,
    };

    pub const SCRIPT_LIST_ADD: Callee = Callee {
        addr: 0x0027_5dd8,
        f: ee::s07::ScriptList_Add,
    };

    pub const SCRIPT_LIST_SKIP: Callee = Callee {
        addr: 0x0027_66d0,
        f: ee::s07::ScriptList_Skip,
    };

    pub const SSX_AUDIO_GET_CHANNEL_VOLUME: Callee = Callee {
        addr: 0x0028_7920,
        f: ee::s07::SSXAudioSystem_GetChannelVolume,
    };

    pub const SSX_AUDIO_EMPTY_CROWD_HANDLER: Callee = Callee {
        addr: 0x0028_9680,
        f: ee::s07::SSXAudioSystem_EmptyCrowdHandler,
    };

    pub const SND_BANK_GET_BUFFER: Callee = Callee {
        addr: 0x0028_b8d8,
        f: ee::s07::SndBank_GetBuffer,
    };

    pub const SSX_AUDIO_RIDER_SAY_20B9: Callee = Callee {
        addr: 0x002a_1400,
        f: ee::s08::SSXAudioSystem_RiderSay20B9,
    };

    pub const CROWD_INSTANCE_PLAY_TRICK: Callee = Callee {
        addr: 0x002a_64a0,
        f: ee::s08::CrowdAudio_InstancePlayTrick,
    };

    pub const CROWD_INSTANCE_PLAY_FALL: Callee = Callee {
        addr: 0x002a_6648,
        f: ee::s08::CrowdAudio_InstancePlayFall,
    };

    pub const CROWD_INSTANCE_STOP_ANTICIPATE: Callee = Callee {
        addr: 0x002a_6808,
        f: ee::s08::CrowdAudio_InstanceStopAnticipate,
    };

    pub const BUILTIN_DELETE_2: Callee = Callee {
        addr: 0x0031_7e50,
        f: ee::s10::f_00317e50,
    };

    pub const EA_MEM_ALLOC: Callee = Callee {
        addr: 0x0031_7d70,
        f: ee::s10::f_00317d70,
    };

    pub const EA_MEM_FREE: Callee = Callee {
        addr: 0x0031_7e98,
        f: ee::s10::f_00317e98,
    };

    pub const OPERATOR_NEW_NAMED: Callee = Callee {
        addr: 0x0031_7e30,
        f: ee::s10::f_00317e30,
    };

    pub const UI_OBJECT_LOAD: Callee = Callee {
        addr: 0x0039_fe00,
        f: ee::s13::f_0039fe00,
    };

    pub const UI_SOUND_BASE_CTOR: Callee = Callee {
        addr: 0x0039_e288,
        f: ee::s13::f_0039e288,
    };

    pub const UI_STATE_MGR_CLOSE_TOP_STATES: Callee = Callee {
        addr: 0x0039_f190,
        f: ee::s13::f_0039f190,
    };

    pub const UI_STATE_DTOR_2: Callee = Callee {
        addr: 0x0039_e390,
        f: ee::s13::f_0039e390,
    };

    pub const SND_REMOVE_TICK_CALLBACK: Callee = Callee {
        addr: 0x003b_5948,
        f: ee::s13::f_003b5948,
    };

    pub const CW_SCRIPT_CACHE_ALLOC: Callee = Callee {
        addr: 0x003a_c800,
        f: ee::s13::f_003ac800,
    };

    pub const BE_GET_RIDER_CHARACTER: Callee = Callee {
        addr: 0x0014_a080,
        f: ee::s01::f_0014a080,
    };

    pub const BE_LIBRARY_GET_INSTANCE_2: Callee = Callee {
        addr: 0x0014_dc80,
        f: ee::s01::f_0014dc80,
    };

    pub const BE_LIBRARY_GET_INTERFACE_2: Callee = Callee {
        addr: 0x0014_dd58,
        f: ee::s01::f_0014dd58,
    };

    pub const GAME_CAMERA_SET_CONTROLLER: Callee = Callee {
        addr: 0x0015_dfd8,
        f: ee::s01::f_0015dfd8,
    };

    pub const LOBBY_MENU_ON_LOBBY_EVENT: Callee = Callee {
        addr: 0x0017_d6f0,
        f: ee::s02::f_0017d6f0,
    };

    pub const LOBBY_MAN_FORWARD_EVENT_TO_UI: Callee = Callee {
        addr: 0x0025_9230,
        f: ee::s06::f_00259230,
    };

    pub const CAMERA_TRIGGER_MAN_UPDATE_TRIGGERS: Callee = Callee {
        addr: 0x0016_d1b8,
        f: ee::s02::f_0016d1b8,
    };

    pub const VID_ENGINE_PLAYER_DTOR: Callee = Callee {
        addr: 0x0025_3418,
        f: ee::s06::f_00253418,
    };

    pub const LUNO_TABLE_DTOR: Callee = Callee {
        addr: 0x0022_4d00,
        f: ee::s05::f_00224d00,
    };

    pub const SSX_AUDIO_GET_INSTANCE: Callee = Callee {
        addr: 0x0028_b180,
        f: ee::s07::f_0028b180,
    };

    pub const SSX_AUDIO_PLAY_CHARACTER_TUNE: Callee = Callee {
        addr: 0x0028_cdf8,
        f: ee::s07::f_0028cdf8,
    };

    pub const NET_GAME_UTIL_DESTROY: Callee = Callee {
        addr: 0x003e_6e70,
        f: ee::s14::f_003e6e70,
    };

    pub const CSETUP_MAN_SEND_PACKET: Callee = Callee {
        addr: 0x0026_4be8,
        f: ee::s07::f_00264be8,
    };

    pub const CVOIP_MAN_CTOR: Callee = Callee {
        addr: 0x0026_6d00,
        f: ee::s07::f_00266d00,
    };

    pub const VOIP_RPC_STOP: Callee = Callee {
        addr: 0x003f_6af0,
        f: ee::s14::f_003f6af0,
    };

    pub const VOIP_RPC_SET_PARAMS: Callee = Callee {
        addr: 0x003f_6c10,
        f: ee::s14::f_003f6c10,
    };

    pub const REPLAY_SET_STATE: Callee = Callee {
        addr: 0x0026_f4a0,
        f: ee::s07::f_0026f4a0,
    };

    pub const REPLAY_HIGHLIGHT_SLOT_CLEAR: Callee = Callee {
        addr: 0x0026_e800,
        f: ee::s07::f_0026e800,
    };

    pub const REPLAY_SAVE_EMIT: Callee = Callee {
        addr: 0x0027_1058,
        f: ee::s07::f_00271058,
    };

    pub const MC_MANAGER_REQUEST_CARD_INFO: Callee = Callee {
        addr: 0x0023_fb20,
        f: ee::s06::f_0023fb20,
    };

    pub const SSX_AUDIO_HANDLE_MUSIC_EVENT: Callee = Callee {
        addr: 0x0028_e8c0,
        f: ee::s07::f_0028e8c0,
    };

    pub const SSX_AUDIO_STOP_LOADING_SCREEN_LOOP: Callee = Callee {
        addr: 0x0028_fa98,
        f: ee::s07::f_0028fa98,
    };

    pub const SND_BANK_MAN_UNLOAD_BANK: Callee = Callee {
        addr: 0x0028_bce8,
        f: ee::s07::f_0028bce8,
    };

    pub const SSX_AUDIO_ARCADE_SAY: Callee = Callee {
        addr: 0x002a_3de0,
        f: ee::s08::f_002a3de0,
    };

    pub const EATIMER_REMOVE_CALLBACK: Callee = Callee {
        addr: 0x003e_58c8,
        f: ee::s14::f_003e58c8,
    };

    pub const REWARDS_FILE_UNLOAD: Callee = Callee {
        addr: 0x0015_ae00,
        f: ee::s01::RewardsFile_Unload,
    };

    pub const BOUNDED_CAMERA_ALGORITHM_BASE_CTOR: Callee = Callee {
        addr: 0x0017_3208,
        f: ee::s02::BoundedCameraAlgorithmBase_ctor,
    };

    pub const NET_FE_STATE_CTOR_2: Callee = Callee {
        addr: 0x001a_8500,
        f: ee::s03::NetFEState_ctor,
    };

    pub const NET_FE_STATE_ON_CHILD_EVENT_2: Callee = Callee {
        addr: 0x001a_97b8,
        f: ee::s03::NetFEState_OnChildEvent,
    };

    pub const NET_FE_STATE_UPDATE: Callee = Callee {
        addr: 0x001a_8770,
        f: ee::s03::NetFEState_Update,
    };

    pub const SECTION_MAN_IS_EVENT_LOADED: Callee = Callee {
        addr: 0x0022_d278,
        f: ee::s05::SectionMan_IsEventLoaded,
    };

    pub const VID_ENGINE_PLAYER_CLOSE: Callee = Callee {
        addr: 0x0025_35f8,
        f: ee::s06::VidEnginePlayer_Close,
    };

    pub const LOBBY_MAN_DESTROY_LISTS: Callee = Callee {
        addr: 0x0025_a8b0,
        f: ee::s06::LobbyMan_DestroyLists,
    };

    pub const LOBBY_MAN_DESTROY_LOBBY_API: Callee = Callee {
        addr: 0x0025_aa98,
        f: ee::s06::LobbyMan_DestroyLobbyApi,
    };

    pub const LOBBY_MAN_DESTROY_PEER_TABLE: Callee = Callee {
        addr: 0x0025_a6f8,
        f: ee::s06::LobbyMan_DestroyPeerTable,
    };

    pub const SHARED_LOGIN_MAN_SHUTDOWN: Callee = Callee {
        addr: 0x0026_59c0,
        f: ee::s07::cSharedLoginMan_Shutdown,
    };

    pub const VOIP_MAN_STOP: Callee = Callee {
        addr: 0x0026_6df8,
        f: ee::s07::cVoipMan_Stop,
    };

    pub const REPLAY_CACHE_FREE: Callee = Callee {
        addr: 0x0026_d130,
        f: ee::s07::ReplayCache_Free,
    };

    pub const REPLAY_SNAPSHOT_EMPTY_HOOK_B: Callee = Callee {
        addr: 0x0026_ed90,
        f: ee::s07::Replay_SnapshotEmptyHookB,
    };

    pub const REPLAY_RESTORE_SNAPSHOT: Callee = Callee {
        addr: 0x0026_db88,
        f: ee::s07::Replay_RestoreSnapshot,
    };

    pub const REPLAY_FREE_SNAPSHOT: Callee = Callee {
        addr: 0x0027_0238,
        f: ee::s07::Replay_FreeSnapshot,
    };

    pub const REPLAY_END_PLAYBACK: Callee = Callee {
        addr: 0x0026_f980,
        f: ee::s07::Replay_EndPlayback,
    };

    pub const IN_SEQ_CHANNEL_RELEASE: Callee = Callee {
        addr: 0x0027_3a00,
        f: ee::s07::InSeq_ChannelRelease,
    };

    pub const IN_SEQ_HEAP_DELETE: Callee = Callee {
        addr: 0x0027_24e8,
        f: ee::s07::InSeq_HeapDelete,
    };

    pub const SCRIPT_ANIM_BANK_MAN_FIND_BANK: Callee = Callee {
        addr: 0x0027_5718,
        f: ee::s07::ScriptAnimBankMan_FindBank,
    };

    pub const SCRIPT_ENGINE_POLL_ASYNC_LOADS_2: Callee = Callee {
        addr: 0x0027_8918,
        f: ee::s07::ScriptEngine_PollAsyncLoads,
    };

    pub const SCRIPT_ENGINE_IS_BANK_SLOT_LOADED: Callee = Callee {
        addr: 0x0027_bf50,
        f: ee::s07::ScriptEngine_IsBankSlotLoaded,
    };

    pub const SCRIPT_LIST_IS_CURRENT_SKIPPABLE: Callee = Callee {
        addr: 0x0027_70c0,
        f: ee::s07::ScriptList_IsCurrentSkippable,
    };

    pub const SCRIPT_LIST_GET_STATE: Callee = Callee {
        addr: 0x0027_6b98,
        f: ee::s07::ScriptList_GetState,
    };

    pub const SCRIPT_LIST_GET_FIELD_B0: Callee = Callee {
        addr: 0x0027_72b8,
        f: ee::s07::ScriptList_GetFieldB0,
    };

    pub const SCRIPT_LIST_IS_PLAYING_SCRIPT: Callee = Callee {
        addr: 0x0027_71c8,
        f: ee::s07::ScriptList_IsPlayingScript,
    };

    pub const SCRIPT_ENGINE_FIND_BANK_SLOT_BY_ID: Callee = Callee {
        addr: 0x0027_b578,
        f: ee::s07::ScriptEngine_FindBankSlotById,
    };

    pub const SCRIPT_ENGINE_CANCEL_BANK_LOAD: Callee = Callee {
        addr: 0x0027_b4b8,
        f: ee::s07::ScriptEngine_CancelBankLoad,
    };

    // Called through the function table (`call_indirect` in the original).
    pub const UI_STATE_CTOR_2: u32 = 0x0039_e2a0;

    pub const UI_STATE_SHUTDOWN: u32 = 0x0039_e4a0;

    pub const UI_LIST_CLOSE_ALL_POPUPS: u32 = 0x0039_f698;

    pub const EA_MEM_FREE_2: u32 = 0x0031_7e98;

    pub const FEMODULE_SET_PREVIEW_VISIBLE: u32 = 0x001a_0570;

    pub const SSX_APP_START_FE_MODULE: u32 = 0x0022_8238;

    pub const LOC_STRING_TABLE_DTOR: u32 = 0x002c_6d70;

    pub const BENEW_PLAYER_INT_IS_MESSAGE_FLAG_SET: u32 = 0x0014_7980;

    pub const BE_MESSAGE_FLAG_SET: u32 = 0x0015_8e30;

    pub const BE_GET_INTERFACE_RIDER: u32 = 0x0014_4bc0;

    pub const SCRIPT_MAN_LOAD: u32 = 0x0030_9030;

    pub const SCRIPT_MAN_UNLOAD: u32 = 0x0030_9f18;

    pub const WORLD_VIEW_SET: u32 = 0x001f_36d8;

    pub const MENU_SYS_EVENT: u32 = 0x0020_ab50;

    pub const REPLAY_STREAM_WRITE: u32 = 0x003e_6574;

    pub const LOBBY_MAN_REQUEST_UPDATE: u32 = 0x003e_80c0;

    pub const LOBBY_MAN_REQUEST_FREE: u32 = 0x003e_81c8;

    pub const LOBBY_API_ADD_REQUEST: u32 = 0x003e_8b10;

    pub const DIRTY_SOCK_CREATE: u32 = 0x003f_7f48;

    pub const DIRTY_SOCK_DESTROY: u32 = 0x003f_6b50;

    pub const VID_ENGINE_PLAYER_REQUEST: u32 = 0x003e_7218;

    pub const APP_SOMETHING: u32 = 0x0011_73b8;

    pub const MEMITTER_FREE_DATA: Callee = Callee {
        addr: 0x0037_15b0,
        f: ee::s12::f_003715b0,
    };

    pub const DYNAMIC_EMITTER_DTOR: Callee = Callee {
        addr: 0x0037_0c08,
        f: ee::s12::f_00370c08,
    };

    pub const DYNAMIC_EMITTER_FREE_DATA: Callee = Callee {
        addr: 0x0037_0cf8,
        f: ee::s12::f_00370cf8,
    };

    pub const RCMP_DECODE_FRAME: Callee = Callee {
        addr: 0x003b_0600,
        f: ee::s13::f_003b0600,
    };

    pub const RCMP_FRAME_NUMBER: Callee = Callee {
        addr: 0x003b_0580,
        f: ee::s13::f_003b0580,
    };

    pub const CAM_CONTROLLER_SET_ALGORITHM: Callee = Callee {
        addr: 0x0015_cf28,
        f: ee::s01::f_0015cf28,
    };

    pub const FE_STATE_ON_ACTIVATE_2: Callee = Callee {
        addr: 0x0018_6518,
        f: ee::s02::f_00186518,
    };

    pub const REQUEST_LINE_REFRESH_LIST: Callee = Callee {
        addr: 0x0019_7b88,
        f: ee::s02::f_00197b88,
    };

    pub const REQUEST_LINE_UPDATE_NOW_PLAYING: Callee = Callee {
        addr: 0x0019_7e70,
        f: ee::s02::f_00197e70,
    };

    pub const REQUEST_LINE_UPDATE_HELP_TEXT: Callee = Callee {
        addr: 0x0019_81f8,
        f: ee::s02::f_001981f8,
    };

    pub const FEMODULE_GET_RIDER_PREVIEW: Callee = Callee {
        addr: 0x001a_0548,
        f: ee::s03::f_001a0548,
    };

    pub const CHAR_EQUIP_REFRESH_BUY_INFO: Callee = Callee {
        addr: 0x0019_ab78,
        f: ee::s03::f_0019ab78,
    };

    pub const CHAR_EQUIP_REFRESH_EQUIP_INFO: Callee = Callee {
        addr: 0x0019_aa08,
        f: ee::s03::f_0019aa08,
    };

    pub const CHAR_EQUIP_UPDATE_HELP_TEXT: Callee = Callee {
        addr: 0x0019_a798,
        f: ee::s03::f_0019a798,
    };

    pub const BOLT_FILE_INDEX_GET_NEXT_NODE: Callee = Callee {
        addr: 0x0019_c8f8,
        f: ee::s03::f_0019c8f8,
    };

    pub const NET_MESSENGER_SET_PLAYER_MENU_SELECTION: Callee = Callee {
        addr: 0x001a_6c40,
        f: ee::s03::f_001a6c40,
    };

    pub const NET_FE_STATE_DTOR: Callee = Callee {
        addr: 0x001a_85d0,
        f: ee::s03::f_001a85d0,
    };

    pub const NET_FE_STATE_HANDLE_NET_EVENT: Callee = Callee {
        addr: 0x001a_8e40,
        f: ee::s03::f_001a8e40,
    };

    pub const FE_STATE_STATS_REFRESH_RESULTS_2: Callee = Callee {
        addr: 0x001c_3310,
        f: ee::s03::f_001c3310,
    };

    pub const KEYBOARD_POPUP_IS_KEY_ENABLED: Callee = Callee {
        addr: 0x001c_e560,
        f: ee::s04::f_001ce560,
    };

    pub const CROWD_MAN2D_INIT_SLOT: Callee = Callee {
        addr: 0x0022_9910,
        f: ee::s05::f_00229910,
    };

    pub const GAME_STATE_PRE_RACE_MENU_RESUME: Callee = Callee {
        addr: 0x0023_6cd8,
        f: ee::s06::f_00236cd8,
    };

    pub const LOBBY_MAN_DISCONNECT: Callee = Callee {
        addr: 0x0025_b800,
        f: ee::s06::f_0025b800,
    };

    pub const LOBBY_MAN_DTOR: Callee = Callee {
        addr: 0x0025_9628,
        f: ee::s06::f_00259628,
    };

    pub const GAME_COMM_DESTROY: Callee = Callee {
        addr: 0x0025_6c50,
        f: ee::s06::f_00256c50,
    };

    pub const SETUP_MAN_SEND_CHARACTER_MSG: Callee = Callee {
        addr: 0x0026_4e50,
        f: ee::s07::f_00264e50,
    };

    pub const SCRIPT_LIST_IS_PLAYING_SCRIPT_2: Callee = Callee {
        addr: 0x0027_71c8,
        f: ee::s07::f_002771c8,
    };

    pub const SCRIPT_DATA_GET_INFO_2: Callee = Callee {
        addr: 0x0027_c070,
        f: ee::s07::f_0027c070,
    };

    pub const SCRIPT_ENGINE_DESTROY_SCRIPT_DATABASE: Callee = Callee {
        addr: 0x0028_2020,
        f: ee::s07::f_00282020,
    };

    pub const SCRIPT_ENGINE_REMOVE_ALL_SCRIPT_OBJS: Callee = Callee {
        addr: 0x0027_d050,
        f: ee::s07::f_0027d050,
    };

    pub const SCRIPT_ENGINE_GET_INSTANCE_SCRIPT_2: Callee = Callee {
        addr: 0x0027_a668,
        f: ee::s07::f_0027a668,
    };

    pub const SCRIPT_MOVIE_STOP: Callee = Callee {
        addr: 0x0028_3818,
        f: ee::s07::f_00283818,
    };

    pub const REPLAY_HIGHLIGHT_SLOT_CLEAR_2: Callee = Callee {
        addr: 0x0026_e800,
        f: ee::s07::f_0026e800,
    };

    pub const INSEQ_SPAN_TRACK_STOP: Callee = Callee {
        addr: 0x0027_2ec0,
        f: ee::s07::f_00272ec0,
    };

    pub const INSEQ_EVENT_TRACK_STOP: Callee = Callee {
        addr: 0x0027_32c8,
        f: ee::s07::f_002732c8,
    };

    pub const INSEQ_RAMP_TRACK_STOP: Callee = Callee {
        addr: 0x0027_3630,
        f: ee::s07::f_00273630,
    };

    pub const INSEQ_HEAP_DELETE: Callee = Callee {
        addr: 0x0027_24e8,
        f: ee::s07::f_002724e8,
    };

    pub const EA_MEM_FREE_3: u32 = 0x0031_7e98;

    pub const FLOAT_TO_UINT: u32 = 0x0041_39f8;

    pub const GET_HASH_VALUE32: u32 = 0x0031_7670;

    pub const UI_SCREEN_GET_OBJECT_BY_HASH_NAME: u32 = 0x0039_d860;

    pub const UI_TEXT_SET_ASCII_STRING: u32 = 0x003a_0c50;

    pub const UI_MENU_SET_SELECTED_BY_INDEX: u32 = 0x0039_b960;

    pub const UI_OBJECT_SET_VISIBLE: u32 = 0x0039_e4a0;

    pub const UI_WIDGET_SLOT_0: u32 = 0x0039_f840;

    pub const UI_WIDGET_SLOT_1: u32 = 0x0023_1320;

    pub const FE_STATE_UPDATE_BUTTONS_TEXT: u32 = 0x0019_8340;

    pub const MESSAGE_CENTER_MESSAGE: u32 = 0x002e_f378;

    pub const AUDIO_APPLY_OUTPUT_MODE: u32 = 0x002b_3a70;

    pub const SOUND_PLAYER_SET_BY_INDEX: u32 = 0x0025_34a8;

    pub const SOUND_PLAYER_START: u32 = 0x0025_3860;

    pub const BE_GET_BE: u32 = 0x0014_dc80;

    pub const BE_GET_INTERFACE: u32 = 0x0014_dd58;

    pub const BE_INTERFACE_CALL: u32 = 0x0014_7a30;

    pub const CBXSTRING_CTOR: u32 = 0x0031_8260;

    pub const KEYBOARD_POPUP_SHOW: u32 = 0x0020_ab50;

    pub const MEMORY_FREE: u32 = 0x0025_23a8;

    pub const EA_MEM_ALLOC_NAMED_2: u32 = 0x0031_7d70;

    pub const AUDIO_SET_OUTPUT: u32 = 0x002a_df80;

    // s03
    pub const ON_QUIT_POPUP_EVENT: Callee = Callee {
        addr: 0x001b_1f18,
        f: ee::s03::f_001b1f18,
    };

    pub const EMPTY_CHILD_HANDLER: Callee = Callee {
        addr: 0x001a_94d8,
        f: ee::s03::f_001a94d8,
    };

    pub const NET_FE_STATE_ON_CHILD_EVENT_3: Callee = Callee {
        addr: 0x001a_97b8,
        f: ee::s03::f_001a97b8,
    };

    pub const NET_FE_STATE_UPDATE_2: Callee = Callee {
        addr: 0x001a_8770,
        f: ee::s03::f_001a8770,
    };

    pub const NET_FE_STATE_CTOR_3: Callee = Callee {
        addr: 0x001a_8500,
        f: ee::s03::f_001a8500,
    };

    pub const NET_FE_STATE_DTOR_2: Callee = Callee {
        addr: 0x001a_85d0,
        f: ee::s03::f_001a85d0,
    };

    pub const CHAT_ROOM_UPDATE_PLAYER_QUALITY_2: Callee = Callee {
        addr: 0x001b_3bf0,
        f: ee::s03::f_001b3bf0,
    };

    pub const STATS_SELECT_CATEGORY: Callee = Callee {
        addr: 0x001c_3ee8,
        f: ee::s03::f_001c3ee8,
    };

    // s04
    pub const KEYBOARD_POPUP_UPDATE_CURSOR: Callee = Callee {
        addr: 0x001c_cc30,
        f: ee::s04::f_001ccc30,
    };

    pub const MEM_CARD_SET_USE_SAVE_TITLE: Callee = Callee {
        addr: 0x001d_9308,
        f: ee::s04::f_001d9308,
    };

    pub const TROPHY_ROOM_RELEASE_IMAGES: Callee = Callee {
        addr: 0x001d_4b20,
        f: ee::s04::f_001d4b20,
    };

    pub const MESSAGE_CENTER_GET_INBOX: Callee = Callee {
        addr: 0x001e_39f8,
        f: ee::s04::f_001e39f8,
    };

    // s05
    pub const SECTION_MAN_PUSH_FREE_NODE: Callee = Callee {
        addr: 0x0022_e288,
        f: ee::s05::f_0022e288,
    };

    // s06
    pub const MC_MANAGER_END_OPERATION_2: Callee = Callee {
        addr: 0x0023_ea30,
        f: ee::s06::f_0023ea30,
    };

    pub const BUDDY_MAN_SET_PRESENCE_ADDR: u32 = 0x0026_2768;

    pub const BUDDY_MAN_DESTROY_API: Callee = Callee {
        addr: 0x0026_1530,
        f: ee::s06::f_00261530,
    };

    // s07
    pub const BUDDY_MAN_GET_ROSTER_COUNT: Callee = Callee {
        addr: 0x0026_2598,
        f: ee::s07::f_00262598,
    };

    pub const SCRIPT_LIST_STOP: Callee = Callee {
        addr: 0x0027_6868,
        f: ee::s07::f_00276868,
    };

    pub const SCRIPT_OBJ_CAM_SET_OWNER: Callee = Callee {
        addr: 0x0027_d680,
        f: ee::s07::f_0027d680,
    };

    pub const SCRIPT_ENGINE_OBJ_TABLE_CTOR: Callee = Callee {
        addr: 0x0027_cdd0,
        f: ee::s07::f_0027cdd0,
    };

    pub const SCRIPT_ENGINE_RESET_PROCESS_TABLE: Callee = Callee {
        addr: 0x0028_28b8,
        f: ee::s07::f_002828b8,
    };

    pub const SND_BANK_UNLOAD_2: Callee = Callee {
        addr: 0x0028_b528,
        f: ee::s07::f_0028b528,
    };

    pub const SND_BANK_UNLINK: Callee = Callee {
        addr: 0x0028_b730,
        f: ee::s07::f_0028b730,
    };

    pub const SND_BANK_FREE_MEMORY_2: Callee = Callee {
        addr: 0x0028_b650,
        f: ee::s07::f_0028b650,
    };

    pub const REPLAY_RECORD_SNAPSHOT: Callee = Callee {
        addr: 0x0026_d760,
        f: ee::s07::f_0026d760,
    };

    pub const REPLAY_ADD_SNAPSHOT: Callee = Callee {
        addr: 0x0026_d740,
        f: ee::s07::f_0026d740,
    };

    // s08
    pub const BANK_MONITOR_STOP_HANDLE: Callee = Callee {
        addr: 0x002a_d5f0,
        f: ee::s08::f_002ad5f0,
    };

    pub const STREAM_BUFFER_DTOR: Callee = Callee {
        addr: 0x002a_9df0,
        f: ee::s08::f_002a9df0,
    };

    pub const BANK_MONITOR_INSTANCE_RESUME: Callee = Callee {
        addr: 0x002a_cb30,
        f: ee::s08::f_002acb30,
    };

    pub const PATH_PLAT_REFRESH_STATUS: Callee = Callee {
        addr: 0x002a_efa8,
        f: ee::s08::f_002aefa8,
    };

    pub const MEM_CARD_ASCII_CHAR_TO_SJIS: Callee = Callee {
        addr: 0x002c_2338,
        f: ee::s08::f_002c2338,
    };

    pub const C_PS2_MEM_CARD_SET_STATE: Callee = Callee {
        addr: 0x002c_48c0,
        f: ee::s08::f_002c48c0,
    };

    pub const C_PS2_MEM_CARD_GET_SAVE_NAME: Callee = Callee {
        addr: 0x002c_54d0,
        f: ee::s08::f_002c54d0,
    };

    pub const C_PS2_MEM_CARD_REQUEST_READ_SAVE: Callee = Callee {
        addr: 0x002c_4788,
        f: ee::s08::f_002c4788,
    };

    // s09
    pub const MENU_SYS_PAGE_SELECT_NEXT: Callee = Callee {
        addr: 0x002c_ab08,
        f: ee::s09::f_002cab08,
    };

    pub const MENU_SYS_ITEM_MEASURE_ROW: Callee = Callee {
        addr: 0x002c_a4c8,
        f: ee::s09::f_002ca4c8,
    };

    pub const AVALANCHE_FREE_HISTORIES: Callee = Callee {
        addr: 0x002d_9a80,
        f: ee::s09::f_002d9a80,
    };

    pub const AVALANCHE_BAKE_GROUP: Callee = Callee {
        addr: 0x002d_9660,
        f: ee::s09::f_002d9660,
    };

    pub const AVALANCHE_EXPORT_HISTORY_FILES: Callee = Callee {
        addr: 0x002d_8258,
        f: ee::s09::f_002d8258,
    };

    pub const AVALANCHE_LOAD_HISTORY_FILE: Callee = Callee {
        addr: 0x002d_87d0,
        f: ee::s09::f_002d87d0,
    };

    pub const SNOW_BOX_RESET: Callee = Callee {
        addr: 0x002e_4f10,
        f: ee::s09::f_002e4f10,
    };

    // s10
    pub const MEM_REPORT_POOL_STAT0: Callee = Callee {
        addr: 0x0032_0518,
        f: ee::s10::f_00320518,
    };

    pub const MEM_REPORT_POOL_STAT1: Callee = Callee {
        addr: 0x0032_0520,
        f: ee::s10::f_00320520,
    };

    pub const MEM_REPORT_POOL_STAT2: Callee = Callee {
        addr: 0x0032_0528,
        f: ee::s10::f_00320528,
    };

    pub const MEM_REPORT_POOL_STAT3: Callee = Callee {
        addr: 0x0032_0530,
        f: ee::s10::f_00320530,
    };

    pub const MEM_REPORT_POOL_STAT4: Callee = Callee {
        addr: 0x0032_0538,
        f: ee::s10::f_00320538,
    };

    pub const MEM_REPORT_POOL_STAT5: Callee = Callee {
        addr: 0x0032_0540,
        f: ee::s10::f_00320540,
    };

    pub const MEM_REPORT_POOL_STAT6: Callee = Callee {
        addr: 0x0032_0548,
        f: ee::s10::f_00320548,
    };

    // Fixed cross-shard `call_indirect` targets.
    pub const FE_MODULE_GET_PLAYER_PAD_MASK: u32 = 0x001a_1cd0;

    pub const UIMENU_SET_SELECTION: u32 = 0x0039_b960;

    pub const UI_STATE_ON_ACTIVATE: u32 = 0x0039_e4c0;

    pub const UI_TEXT_GET_FONT: u32 = 0x003a_04f0;

    pub const BE_REWARD_INT_IS_UNLOCKED: u32 = 0x0015_7468;

    pub const UI_STATE_CTOR_3: u32 = 0x0039_e2a0;

    pub const C_PS2_MEM_CARD_REQUEST_LIST_DIR_ALL: u32 = 0x002c_6810;

    pub const C_BX_STRING_DTOR: u32 = 0x0031_8260;

    pub const LOBBY_LIST_GET_AT_3: u32 = 0x003e_e560;

    pub const SLOGIN_MC_GET_PORT_SLOT_2: u32 = 0x003f_a1d0;

    pub const SCE_MC_FORMAT: u32 = 0x0040_a998;

    pub const SPRINTF_2: u32 = 0x0041_7828;

    pub const C_SETUP_MAN_SEND_QUIT: u32 = 0x0026_5768;

    pub const UI_OBJECT_SHOW_CLAMP_ALPHA_HALF: u32 = 0x0019_4498;

    pub const C_LUNO_TABLE_DTOR: u32 = 0x0022_4df0;

    pub const MEMSET_3: Callee = Callee {
        addr: 0x0041_6210,
        f: ee::s15::f_00416210,
    };

    pub const STR_HASH_4: Callee = Callee {
        addr: 0x0031_7670,
        f: ee::s10::f_00317670,
    };

    pub const SPRINTF_6: Callee = Callee {
        addr: 0x0041_7828,
        f: ee::s15::f_00417828,
    };

    pub const FORMAT_TAGGED: Callee = Callee {
        addr: 0x002c_2580,
        f: ee::s08::f_002c2580,
    };

    pub const BE_RESET_PROFILE_CHARACTERS_2: Callee = Callee {
        addr: 0x0015_67b8,
        f: ee::s01::BE_ResetProfileCharacters,
    };

    pub const BE_NEW_PLAYER_INT_GET_INSTANCE: Callee = Callee {
        addr: 0x0014_5970,
        f: ee::s01::BENewPlayerInt_GetInstance,
    };

    pub const BE_CHAR_INT_GET_INSTANCE: Callee = Callee {
        addr: 0x0014_7f78,
        f: ee::s01::BECharInt_GetInstance,
    };

    pub const BE_BAGT_INT_GET_INSTANCE: Callee = Callee {
        addr: 0x0014_f900,
        f: ee::s01::BEBAGTInt_GetInstance,
    };

    pub const C_MEM_MAN_ALLOC: Callee = Callee {
        addr: 0x0031_7d70,
        f: ee::s10::f_00317d70,
    };

    pub const TAG_FIELD_FIND: Callee = Callee {
        addr: 0x003e_b6d8,
        f: ee::s14::f_003eb6d8,
    };

    pub const TAG_FIELD_GET_NUMBER: Callee = Callee {
        addr: 0x003e_cf30,
        f: ee::s14::f_003ecf30,
    };

    // Front-end back-end interface accessors.
    pub const CBE_GET_BE: Callee = Callee {
        addr: 0x0014_dc80,
        f: ee::s01::f_0014dc80,
    };

    pub const CBE_GET_INTERFACE: Callee = Callee {
        addr: 0x0014_dd58,
        f: ee::s01::f_0014dd58,
    };

    pub const GET_CHAR_ID: Callee = Callee {
        addr: 0x0014_7488,
        f: ee::s01::f_00147488,
    };

    pub const GET_ALT_CHAR: Callee = Callee {
        addr: 0x0014_74c8,
        f: ee::s01::f_001474c8,
    };

    // The GM handler and the memory-card manager.
    pub const GM_PICK_OPPONENTS: Callee = Callee {
        addr: 0x0023_b0a8,
        f: ee::s06::GMHandler8_PickOpponents,
    };

    pub const MC_SET_MODE: Callee = Callee {
        addr: 0x0023_d570,
        f: ee::s06::MCManager_SetMode,
    };

    pub const MC_GET_PRODUCT_CODE: Callee = Callee {
        addr: 0x0024_1aa0,
        f: ee::s06::MCManager_GetProductCode,
    };

    pub const MC_GET_SAVE_TYPE_TAG: Callee = Callee {
        addr: 0x0024_1d40,
        f: ee::s06::MCManager_GetSaveTypeTag,
    };

    pub const F_00152BA8: Callee = Callee {
        addr: 0x0015_2ba8,
        f: ee::s01::f_00152ba8,
    };

    pub const F_00152948: Callee = Callee {
        addr: 0x0015_2948,
        f: ee::s01::f_00152948,
    };

    pub const F_002C2540: Callee = Callee {
        addr: 0x002c_2540,
        f: ee::s08::f_002c2540,
    };

    pub const F_002C26D0: Callee = Callee {
        addr: 0x002c_26d0,
        f: ee::s08::f_002c26d0,
    };

    pub const MESSAGE_CENTER_IS_FLAG_SET_2: Callee = Callee {
        addr: 0x001e_3a30,
        f: ee::s04::MessageCenter_IsFlagSet,
    };

    pub const MESSAGE_CENTER_SET_FLAG_2: Callee = Callee {
        addr: 0x001e_3a78,
        f: ee::s04::MessageCenter_SetFlag,
    };

    pub const STR_INTERN: Callee = Callee {
        addr: 0x0039_8380,
        f: ee::s13::f_00398380,
    };

    pub const F_0039E2A0: Callee = Callee {
        addr: 0x0039_e2a0,
        f: ee::s13::f_0039e2a0,
    };

    // Lobby messages and the network manager.
    pub const SNPRINTF: Callee = Callee {
        addr: 0x003e_c180,
        f: ee::s14::f_003ec180,
    };

    pub const LOBBYMAN_SEND_REQUEST: Callee = Callee {
        addr: 0x0025_b608,
        f: ee::s06::LobbyMan_SendRequest,
    };

    pub const NET_1: Callee = Callee {
        addr: 0x0026_0f80,
        f: ee::s06::f_00260f80,
    };

    pub const NET_2: Callee = Callee {
        addr: 0x0026_1460,
        f: ee::s06::f_00261460,
    };

    pub const NET_3: Callee = Callee {
        addr: 0x0026_13c8,
        f: ee::s06::f_002613c8,
    };

    pub const NET_4: Callee = Callee {
        addr: 0x0026_1408,
        f: ee::s06::f_00261408,
    };

    pub const NET_FLUSH: Callee = Callee {
        addr: 0x0025_b800,
        f: ee::s06::f_0025b800,
    };

    pub const NET_5: Callee = Callee {
        addr: 0x0025_5fa0,
        f: ee::s06::f_00255fa0,
    };

    // Audio speech helpers.
    pub const IS_PA_SPEECH_ON: Callee = Callee {
        addr: 0x0029_f128,
        f: ee::s08::f_0029f128,
    };

    pub const IS_REPLAY_PLAYING: Callee = Callee {
        addr: 0x0028_8ae0,
        f: ee::s07::f_00288ae0,
    };

    pub const RIDER_123168: Callee = Callee {
        addr: 0x0012_3168,
        f: ee::s00::f_00123168,
    };

    pub const IS_RACE_TYPE_4OR5: Callee = Callee {
        addr: 0x002a_4248,
        f: ee::s08::f_002a4248,
    };

    pub const BANK_MONITOR_ADD_TIMER_2: Callee = Callee {
        addr: 0x002a_dca0,
        f: ee::s08::f_002adca0,
    };

    pub const SPEECH_SET_EVENT_CONTEXT: Callee = Callee {
        addr: 0x002b_1458,
        f: ee::s08::f_002b1458,
    };

    pub const GET_LANGUAGE_MASK: Callee = Callee {
        addr: 0x0029_f198,
        f: ee::s08::f_0029f198,
    };

    pub const GET_RIDER_VOICE_MASK: Callee = Callee {
        addr: 0x002a_1d48,
        f: ee::s08::f_002a1d48,
    };

    pub const GET_FINISH_SPEECH_MASK: Callee = Callee {
        addr: 0x002a_1e68,
        f: ee::s08::f_002a1e68,
    };

    pub const SPEECH_QUEUE: Callee = Callee {
        addr: 0x003d_8008,
        f: ee::s14::f_003d8008,
    };

    // Network teardown.
    pub const NET_FIND: Callee = Callee {
        addr: 0x0025_5d38,
        f: ee::s06::f_00255d38,
    };

    pub const NET_REMOVE: Callee = Callee {
        addr: 0x0025_5cc8,
        f: ee::s06::f_00255cc8,
    };

    pub const VOIP_DEACTIVATE: Callee = Callee {
        addr: 0x0026_6e88,
        f: ee::s07::cVoipMan_Deactivate,
    };

    pub const VOIP_STOP: Callee = Callee {
        addr: 0x0026_6df8,
        f: ee::s07::cVoipMan_Stop,
    };

    pub const SETUPMAN_DESTROY: Callee = Callee {
        addr: 0x0026_4098,
        f: ee::s07::cSetupMan_Destroy,
    };

    pub const NET_RESET: Callee = Callee {
        addr: 0x0025_f740,
        f: ee::s06::f_0025f740,
    };

    // Text layout helpers for the popup description.
    pub const TEXT_OBJ: Callee = Callee {
        addr: 0x003a_04f0,
        f: ee::s13::f_003a04f0,
    };

    pub const MEASURE_PLAIN: Callee = Callee {
        addr: 0x0039_21f0,
        f: ee::s12::f_003921f0,
    };

    pub const MEASURE_WRAP: Callee = Callee {
        addr: 0x0039_1fb0,
        f: ee::s12::f_00391fb0,
    };

    // Rider reset transition effect.
    pub const ANIM_GET_LAYER_SEQUENCE: Callee = Callee {
        addr: 0x0031_1b20,
        f: ee::s10::AnimController_GetLayerSequence,
    };

    pub const IS_LOCAL_PLAYER: Callee = Callee {
        addr: 0x0012_f588,
        f: ee::s00::RiderResetSub_IsLocalPlayer,
    };

    pub const GET_TRANSITION_MAN: Callee = Callee {
        addr: 0x0023_0698,
        f: ee::s06::GameModule_GetTransitionMan,
    };

    pub const TRANSITION_STOP: Callee = Callee {
        addr: 0x002e_4578,
        f: ee::s09::cTransitionMan_Stop,
    };

    pub const EFFECT_ALLOC: Callee = Callee {
        addr: 0x002e_4ce8,
        f: ee::s09::TransitionEffect_AllocData,
    };

    pub const EFFECT_CTOR: Callee = Callee {
        addr: 0x002e_4d70,
        f: ee::s09::TransitionEffect_ctor,
    };

    pub const TRANSITION_START: Callee = Callee {
        addr: 0x002e_4370,
        f: ee::s09::cTransitionMan_Start,
    };
}

/// Cross-shard direct calls the generator emitted as `call_indirect`.
pub(crate) mod table {
    pub(crate) use crate::game::fe::feasyncfile::table::ASYNC_FILE_GET_RESULT;
    pub(crate) use crate::game::fe::feasyncfile::table::AUDIO_GET_RACE_MAN;
    pub(crate) use crate::game::fe::feasyncfile::table::BE_GET_INSTANCE;
    pub(crate) use crate::game::fe::feasyncfile::table::BE_GET_INTERFACE;
    pub(crate) use crate::game::fe::feasyncfile::table::BE_NEW_RACE_SET_EVENT_TYPE;
    pub(crate) use crate::game::fe::feasyncfile::table::BE_NEW_RACE_SET_GAME_MODE;
    pub(crate) use crate::game::fe::feasyncfile::table::BUILTIN_DELETE;
    pub(crate) use crate::game::fe::feasyncfile::table::BXSTRING_ASSIGN;
    pub(crate) use crate::game::fe::feasyncfile::table::BXSTRING_RELEASE;
    pub(crate) use crate::game::fe::feasyncfile::table::COLL_HULL_TRIANGLE;
    pub(crate) use crate::game::fe::feasyncfile::table::COLL_HULL_TRIANGLE2;
    pub(crate) use crate::game::fe::feasyncfile::table::COLL_SPHERE_CALC;
    pub(crate) use crate::game::fe::feasyncfile::table::COLL_SPHERE_TREE_TRIANGLE;
    pub(crate) use crate::game::fe::feasyncfile::table::EAC_GET_UNPACKED_SIZE;
    pub(crate) use crate::game::fe::feasyncfile::table::EAC_UNPACK;
    pub(crate) use crate::game::fe::feasyncfile::table::EA_MEM_ALLOC_NAMED;
    pub(crate) use crate::game::fe::feasyncfile::table::EA_MEM_FREE;
    pub(crate) use crate::game::fe::feasyncfile::table::OPERATOR_NEW_TAG;
    pub(crate) use crate::game::fe::feasyncfile::table::PATH_STOP_TRACKS;
    pub(crate) use crate::game::fe::feasyncfile::table::PS2_MEM_CARD_CREATE;
    pub(crate) use crate::game::fe::feasyncfile::table::REPLAY_BEGIN_PLAYBACK;
    pub(crate) use crate::game::fe::feasyncfile::table::REPLAY_END_SAVE;
    pub(crate) use crate::game::fe::feasyncfile::table::RIDER_MODEL_FIND;
    pub(crate) use crate::game::fe::feasyncfile::table::SCRIPT_ENGINE_GET_RESULT_VTABLE;
    pub(crate) use crate::game::fe::feasyncfile::table::SPRINTF;
    pub(crate) use crate::game::fe::feasyncfile::table::STR_CASECMP;
    pub(crate) use crate::game::fe::feasyncfile::table::STR_HASH;
    pub(crate) use crate::game::fe::feasyncfile::table::UI_LIST_FIRST;
    pub(crate) use crate::game::fe::feasyncfile::table::UI_LIST_NODE_IS_SENTINEL;
    pub(crate) use crate::game::fe::feasyncfile::table::UI_MENU_RESET_SCROLL;
    pub(crate) use crate::game::fe::feasyncfile::table::UI_SCREEN_FIND_OBJECT;
    pub(crate) use crate::game::fe::feasyncfile::table::UI_TEXT_SET_TEXT_ID;
    pub(crate) use crate::game::fe::feasyncfile::table::UI_TEXT_SET_WIDE_STRING;
    pub(crate) use crate::game::fe::feasyncfile::table::VOIP_RPC_SET_PARAMS;
    pub(crate) use crate::game::fe::feasyncfile::table::VOIP_RPC_START;
    pub(crate) use crate::game::movie::movieplayer::table::LOBBY_LIST_GET_NAME;
    pub(crate) use crate::game::movie::movieplayer::table::MC_CARD_DESTROY;
    pub(crate) use crate::game::movie::movieplayer::table::SSX_AUDIO_SYSTEM_GET_INSTANCE;
    pub(crate) use crate::game::movie::movieplayer::table::SSX_AUDIO_SYSTEM_SET_GAME_STATE;
    pub(crate) use crate::game::movie::movieplayer::table::UI_LIST_SET_FIRST_FLAGS;
    pub(crate) use crate::game::movie::movieplayer::table::UI_STATE_ON_ACTIVATE;
    pub(crate) use crate::game::movie::movieplayer::table::UI_STATE_SHUTDOWN;
    pub(crate) use crate::game::movie::movieplayer::table::VOIP_RPC_CALL;
}

/// Addresses the generator emitted as `call_indirect` (outside the shard's
/// own direct-call set, so no readable alias is needed here).
pub(crate) mod addr {
    pub(crate) use crate::game::fe::feasyncfile::addr::__BUILTIN_DELETE;
    pub(crate) use crate::game::fe::feasyncfile::addr::__FIXUNSSFSI;
    pub(crate) use crate::game::fe::feasyncfile::addr::C_BX_STRING_DTOR;
    pub(crate) use crate::game::fe::feasyncfile::addr::EA_MEM_FREE;
    pub(crate) use crate::game::fe::feasyncfile::addr::FONT_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::BANK_MONITOR_ADD;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::BE_GET_BE;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::BE_GET_INTERFACE;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::BE_GET_INTERFACE_9;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::CHASE_CAMERA_RESET_FOR_RIDER;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::CLOSE_UNPINNED_POPUPS;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::FE_NET_SEND;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::HUD_SHOW_NUMBER_DELAYED;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::MC_STATUS_TEXT;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::MEMSET;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::MEM_MAN_ALLOC;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::MENU_DTOR;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::MENU_DTOR_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::MENU_POP_PAGE;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::MENU_PUSH_PAGE;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::POSITION_MODIFIER_CTOR_3;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::RIDER_PREVIEW;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::RIDER_PREVIEW_BOLTS;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::SPATIAL_INSERT;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::SPRINTF;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::STRCASECMP;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::STRCMP;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::STRLEN;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::STR_HASH;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::UI_ENGINE_CREATE_SCREEN;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::UI_ENGINE_TEARDOWN;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::UI_OBJECT_BY_HASH;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::UI_SCREEN_START_THREAD;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::WORLD_GET;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::WORLD_PAINTER_2D;
}

/// `GetHashValue32` (0x317670) and the UI object lookups, which the generator
/// emitted as cross-shard `call_indirect`s.
pub(crate) mod table_callee {
    pub(crate) use crate::game::fe::festaterewards::table_callee::AI_SET_AI_STATE;
    pub(crate) use crate::game::fe::festaterewards::table_callee::APP_MAN_SET_NEXT_MODULE;
    pub(crate) use crate::game::fe::festaterewards::table_callee::BENEW_PLAYER_INT_GET_CHAR_ID;
    pub(crate) use crate::game::fe::festaterewards::table_callee::BE_GET;
    pub(crate) use crate::game::fe::festaterewards::table_callee::BE_GET_INTERFACE_FV;
    pub(crate) use crate::game::fe::festaterewards::table_callee::BE_GET_TEXT_FROM_ID;
    pub(crate) use crate::game::fe::festaterewards::table_callee::BE_SET_STATE;
    pub(crate) use crate::game::fe::festaterewards::table_callee::BUDDY_PRODUCT_COMPARE;
    pub(crate) use crate::game::fe::festaterewards::table_callee::BUY_POPUP_CTOR;
    pub(crate) use crate::game::fe::festaterewards::table_callee::BXSTRING_CTOR;
    pub(crate) use crate::game::fe::festaterewards::table_callee::BXSTRING_DTOR;
    pub(crate) use crate::game::fe::festaterewards::table_callee::CODEC_DECODE_FRAME;
    pub(crate) use crate::game::fe::festaterewards::table_callee::EA_MEM_ALLOC_NAMED;
    pub(crate) use crate::game::fe::festaterewards::table_callee::FE_MODULE_BIG_CTOR;
    pub(crate) use crate::game::fe::festaterewards::table_callee::FE_MODULE_SMALL_CTOR;
    pub(crate) use crate::game::fe::festaterewards::table_callee::FE_STATE_CTOR;
    pub(crate) use crate::game::fe::festaterewards::table_callee::GET_HASH_VALUE32;
    pub(crate) use crate::game::fe::festaterewards::table_callee::MUSIC_SLOT_RELEASE;
    pub(crate) use crate::game::fe::festaterewards::table_callee::OPERATOR_DELETE;
    pub(crate) use crate::game::fe::festaterewards::table_callee::OPERATOR_NEW;
    pub(crate) use crate::game::fe::festaterewards::table_callee::REWARD_GALLERY_CHEAT_CHARS;
    pub(crate) use crate::game::fe::festaterewards::table_callee::REWARD_GALLERY_CONCEPT_ARTS;
    pub(crate) use crate::game::fe::festaterewards::table_callee::REWARD_GALLERY_GET_BASE;
    pub(crate) use crate::game::fe::festaterewards::table_callee::REWARD_GALLERY_POSTERS;
    pub(crate) use crate::game::fe::festaterewards::table_callee::REWARD_GALLERY_TOYS;
    pub(crate) use crate::game::fe::festaterewards::table_callee::REWARD_GALLERY_TRADING_CARDS;
    pub(crate) use crate::game::fe::festaterewards::table_callee::REWARD_GALLERY_VIDEOS;
    pub(crate) use crate::game::fe::festaterewards::table_callee::RIDER_GET_MAIN_STATE;
    pub(crate) use crate::game::fe::festaterewards::table_callee::RIDER_REMOVE_SCRIPT_OBJECT;
    pub(crate) use crate::game::fe::festaterewards::table_callee::RIDER_SCRIPT_CLEANUP;
    pub(crate) use crate::game::fe::festaterewards::table_callee::RIDER_SET_SCRIPT_STATE;
    pub(crate) use crate::game::fe::festaterewards::table_callee::SECTION_TABLE_CTOR;
    pub(crate) use crate::game::fe::festaterewards::table_callee::SSX_AUDIO_GET_RACE_MAN;
    pub(crate) use crate::game::fe::festaterewards::table_callee::STRCPY;
    pub(crate) use crate::game::fe::festaterewards::table_callee::STRLEN;
    pub(crate) use crate::game::fe::festaterewards::table_callee::TAG_FIELD_FIND;
    pub(crate) use crate::game::fe::festaterewards::table_callee::TAG_FIELD_GET_STRING4;
    pub(crate) use crate::game::fe::festaterewards::table_callee::TAG_FIELD_SET_STRING;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_ENGINE_ADD_SCREEN_BY_HASH_NAME;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_ENGINE_GET_INTERFACE;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_MENU_SET_SELECTED_BY_INDEX;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_OBJECT_PUSH;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_OPEN_FINISH_SCREEN;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_SCREEN_ADD_OBJECT;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_SCREEN_FIND_OBJECT;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_SCREEN_PLAY_FRAME;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_STATE_DTOR;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_TEXT_SET_ASCII_STRING;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_TEXT_SET_RICH_STRING;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UI_TEXT_SET_UNICODE_STRING_BY_ID;
    pub(crate) use crate::game::fe::festaterewards::table_callee::UPLOAD_TEXTURE_PIXELS;
}

// ---------------------------------------------------------------------------
// Lobby.

/// `int LobbyMan_GetLanguage(void)` (0x259198): returns the current language
/// from the globals at 0x4a2f18.
pub fn lobby_man_get_language(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = sx(0x4a_0000u32.wrapping_add(0x2f18));
    abi::ret(c);
}

/// `bool BuddyMan_IsConnected(BuddyMan *self)` (0x261460): true when the
/// session handle (+0xc8) and connection (+0xe0) exist and the state (+0xd8)
/// is not "connecting".
pub fn buddy_man_is_connected(c: &mut Cpu) {
    c.cycles += 3;
    let this = Addr(abi::arg(c, 0));
    let mut result = 0u64;
    let session = this.off(0xc8).lw(c);
    c.g[V0][0] = session;
    if session != 0 {
        c.cycles += 4;
        let state = this.off(0xd8).lw(c) ^ 1;
        c.g[V0][0] = state;
        if state != 0 {
            c.cycles += 3;
            let connected = this.off(0xe0).lw(c);
            c.g[V0][0] = connected;
            result = (connected != 0) as u64;
        }
    }
    c.cycles += 2;
    c.g[V1][0] = result;
    c.g[V0][0] = result;
    abi::ret(c);
}

/// `int BuddyMan_MapPresenceState(BuddyMan *man, int slot)` (0x2628c0): maps a
/// slot number to a presence state (0..3) when the list at `man + 0xc8` is
/// present, `-1` for an absent list or a slot outside 0..3.
pub fn buddy_man_map_presence_state(c: &mut Cpu) {
    const PRESENCE: u32 = 0xc8;
    const NONE: u64 = u64::MAX; // -1

    c.cycles += 3;
    let man = Addr(abi::arg(c, 0));
    let slot = c.g[A1][0];
    let present = man.off(PRESENCE).lw(c) != 0;
    c.g[V0][0] = 1;
    if !present {
        c.cycles += 2;
        c.g[V0][0] = NONE;
        abi::ret(c);
        return;
    }
    c.cycles += 2;
    if slot == 1 {
        c.cycles += 2;
        c.g[V0][0] = 1;
        abi::ret(c);
        return;
    }
    c.cycles += 2;
    let below_two = (slot as i64) < 2;
    c.g[V0][0] = 2;
    if below_two {
        c.cycles += 2;
        if slot == 0 {
            c.cycles += 2;
            c.g[V0][0] = 0;
            abi::ret(c);
            return;
        }
        c.cycles += 2;
        c.g[V0][0] = NONE;
        c.cycles += 2;
        abi::ret(c);
        return;
    }
    c.cycles += 2;
    c.g[V0][0] = 3;
    if slot == 2 {
        c.cycles += 2;
        c.g[V0][0] = 2;
        abi::ret(c);
        return;
    }
    c.cycles += 2;
    c.g[V0][0] = NONE;
    if slot == 3 {
        c.cycles += 2;
        c.g[V0][0] = 3;
        abi::ret(c);
        return;
    }
    // (The original branches once more, to the shared -1 return.)
    c.cycles += 4;
    abi::ret(c);
}

/// `int BuddyMan_GetMessageCount(BuddyMan *man)` (0x262938): number of
/// messages, `(end - begin) / 4` with the two pointers at +0xac / +0xb0.
pub fn buddy_man_get_message_count(c: &mut Cpu) {
    const MESSAGES_BEGIN: u32 = 0xac;
    const MESSAGES_END: u32 = 0xb0;

    c.cycles += 5;
    let man = Addr(abi::arg(c, 0));
    let begin = man.off(MESSAGES_BEGIN).lw(c);
    c.g[V1][0] = begin;
    let end = man.off(MESSAGES_END).lw(c);
    c.g[V0][0] = sx(((end as u32).wrapping_sub(begin as u32) as i32 >> 2) as u32);
    abi::ret(c);
}

/// `BuddyMan_Message *BuddyMan_GetChallenger(BuddyMan *man, int index)`
/// (0x263308): `challengers[index]`, records being 4 bytes after the begin
/// pointer at +0xbc (end at +0xc0), or NULL when out of range.
pub fn buddy_man_get_challenger(c: &mut Cpu) {
    const CHALLENGERS_BEGIN: u32 = 0xbc;
    const CHALLENGERS_END: u32 = 0xc0;

    c.cycles += 7;
    let man = Addr(abi::arg(c, 0));
    let index = c.g[A1][0];
    let begin = man.off(CHALLENGERS_BEGIN).lw(c);
    c.g[A2][0] = begin;
    let end = man.off(CHALLENGERS_END).lw(c);
    let count = sx(((end as u32).wrapping_sub(begin as u32) as i32 >> 2) as u32);
    c.g[V0][0] = count;
    c.g[V1][0] = sx((index as u32) << 2);
    if index < count {
        c.cycles += 3;
        let slot = sx((begin as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[V1][0] = slot;
        c.g[V0][0] = Addr(slot as u32).lw(c);
    } else {
        c.cycles += 2;
        c.g[V0][0] = 0;
    }
    abi::ret(c);
}

/// `BuddyMan_Message *BuddyMan_GetMessage(BuddyMan *man, int index)`
/// (0x263630): `messages[index]`, records being 4 bytes after the begin
/// pointer at +0xac (end at +0xb0), or NULL when out of range.
pub fn buddy_man_get_message(c: &mut Cpu) {
    const MESSAGES_BEGIN: u32 = 0xac;
    const MESSAGES_END: u32 = 0xb0;

    c.cycles += 7;
    let man = Addr(abi::arg(c, 0));
    let index = c.g[A1][0];
    let begin = man.off(MESSAGES_BEGIN).lw(c);
    c.g[A2][0] = begin;
    let end = man.off(MESSAGES_END).lw(c);
    let count = sx(((end as u32).wrapping_sub(begin as u32) as i32 >> 2) as u32);
    c.g[V0][0] = count;
    c.g[V1][0] = sx((index as u32) << 2);
    if index < count {
        c.cycles += 3;
        let slot = sx((begin as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[V1][0] = slot;
        c.g[V0][0] = Addr(slot as u32).lw(c);
    } else {
        c.cycles += 2;
        c.g[V0][0] = 0;
    }
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Buddies / world state / sound banks.

/// `int BuddyMan_MapPresenceState(BuddyMan *man, int state)` (0x2628c0): map a
/// buddy state to a presence state, or -1 when the manager has no presence
/// object.
pub fn buddy_man_map_presence_state_2(c: &mut Cpu) {
    c.cycles += 3;
    let man = Addr(abi::arg(c, 0));
    let presence = man.off(0xc8).lw(c);
    c.g[V0][0] = 0x1;
    if presence == 0 {
        c.cycles += 2;
        c.g[V0][0] = u64::MAX;
        abi::ret(c);
        return;
    }
    c.cycles += 2;
    let state = c.g[A1][0];
    let is_one = state == c.g[V0][0];
    c.g[V0][0] = ((state as i64) < 2) as u64;
    if is_one {
        c.cycles += 2;
        c.g[V0][0] = 0x1;
        abi::ret(c);
        return;
    }
    'presence_low: {
        'presence_zero: {
            'presence_ge2: {
                c.cycles += 2;
                let t = c.g[V0][0] == 0;
                c.g[V0][0] = 0x2;
                if t {
                    break 'presence_ge2;
                } else {
                    c.cycles += 2;
                    let t = state == 0;
                    c.g[V0][0] = u64::MAX;
                    if t {
                        break 'presence_zero;
                    } else {
                        c.cycles += 2;
                        break 'presence_low;
                    }
                }
            }
            c.cycles += 2;
            let t = state == c.g[V0][0];
            c.g[V0][0] = 0x3;
            if t {
                c.cycles += 2;
                c.g[V0][0] = 0x2;
                abi::ret(c);
                return;
            } else {
                c.cycles += 2;
                let t = state == c.g[V0][0];
                c.g[V0][0] = u64::MAX;
                if t {
                    c.cycles += 2;
                    c.g[V0][0] = 0x3;
                    abi::ret(c);
                    return;
                } else {
                    c.cycles += 2;
                    break 'presence_low;
                }
            }
        }
        c.cycles += 2;
        c.g[V0][0] = 0;
        abi::ret(c);
        return;
    }
    c.cycles += 2;
    abi::ret(c);
}

/// `void cSharedLoginMan_DoFinish(cSharedLoginMan* m)` (0x265f00): marks the
/// login done (state 0x13 at +0), clears the pending flag at +4 and the
/// shared-login pointer `D_004A33E0`.
pub fn c_shared_login_man_do_finish(c: &mut Cpu) {
    c.cycles += 5;
    let selfp = abi::arg(c, 0);
    c.sw(selfp.wrapping_add(4), 0);
    c.g[V0][0] = 0x13;
    c.sw(gp(c, 0x2f0), 0);
    c.sw(selfp, c.g[V0][0]);
    abi::ret(c);
}

/// `void cSetupMan_FreePacket(this, void *packet)` (0x00264990):
/// `operator delete(packet)`.
pub fn c_setup_man_free_packet(c: &mut Cpu) {
    forward(
        c,
        7,
        0x10,
        |c| c.g[A0][0] = c.g[A1][0],
        callee::BUILTIN_DELETE,
        0x0026_49a0,
    )
}

/// `EA_MemFree` (0x317e98).
pub(crate) const MEM_FREE: u32 = 0x0031_7e98;

// ---------------------------------------------------------------------------
// cSharedLoginMan_Shutdown and the memory-card callbacks.

/// `void cSharedLoginMan_Shutdown(cSharedLoginMan* m)` (0x2659c0): clear the
/// state word, shut the login library down and destroy the memory-card
/// object (`gp + 0x2e8`).
pub fn shared_login_man_shutdown(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        Addr(r32(c, A0)).sw(c, 0);
        call_ptr(c, 0x003f_7f18, 0x0026_59d0)?;
        c.g[A0][0] = c.lw(r32(c, GP).wrapping_add(0x2e8));
        call_ptr(c, 0x002c_2300, 0x0026_59d8)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The shared shape of the three memory-card result callbacks: fetch the port
/// slot for `code` into a stack result, then post the new state
/// (`gp + 0x2e4`).
pub(crate) fn login_mc_callback(c: &mut Cpu, ret: u32, state: u64) -> Flow {
    c.cycles += 9;
    let frame = Frame::open(c, 0x20);
    frame.save_ra(c, 0x10);
    c.g[A1][0] = c.g[SP][0];
    call_ptr(c, 0x003f_a1d0, ret)?;
    c.g[V1][0] = state;
    frame.restore_ra(c, 0x10);
    Addr(r32(c, GP).wrapping_add(0x2e4)).sw(c, c.g[V1][0]);
    frame.close(c);
    Ok(())
}

/// `void cSharedLoginMan_McCb_State09(int code)` (0x266010): state 9.
pub fn shared_login_man_mc_cb_state09(c: &mut Cpu) {
    guest(c, |c| login_mc_callback(c, 0x0026_6020, 0x9))
}

/// `void cSharedLoginMan_McCb_State0A(int code)` (0x266038): state 0xa.
pub fn shared_login_man_mc_cb_state0a(c: &mut Cpu) {
    guest(c, |c| login_mc_callback(c, 0x0026_6048, 0xa))
}

/// `void cSharedLoginMan_McCb_State03(int code)` (0x2660a0): state 3.
pub fn shared_login_man_mc_cb_state03(c: &mut Cpu) {
    guest(c, |c| login_mc_callback(c, 0x0026_60b0, 0x3))
}

// ---------------------------------------------------------------------------
// TibExtMemFree.

/// `void TibExtMemFree(void* unused, void* p)` (0x266760): free `p` when it is
/// not null.
pub fn tib_ext_mem_free(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = c.g[A1][0];
        let none = c.g[A0][0] == 0;
        frame.save_ra(c, 0);
        if !none {
            c.cycles += 2;
            call_ptr(c, MEM_FREE, 0x0026_6778)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Small register helpers.

/// A `gp_2`-relative global.
#[inline(always)]
pub(crate) fn gp_2(c: &Cpu, off: i32) -> u32 {
    r32(c, GP).wrapping_add(off as u32)
}

/// A 32-bit guest pointer argument.
#[inline(always)]
pub(crate) fn arg(c: &Cpu, i: usize) -> u32 {
    abi::arg(c, i)
}

// ---------------------------------------------------------------------------
// Lobby.

/// `void LobbyMan_AcceptChallenge(self, a1, challenge)` (0x25fb58): store the
/// challenge id and send the lock message when the lobby is connected.
pub fn lobby_man_accept_challenge(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let this = arg(c, 0);
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = Addr(this).lw(c);
        let offline = c.g[V0][0] == 0;
        frame.save_ra(c, 0);
        if !offline {
            c.cycles += 3;
            Addr(this.wrapping_add(0xa4)).sw(c, c.g[A2][0]);
            c.g[A2][0] = 0x17;
            callee::LOBBY_MAN_SEND_LOCK_CHAL.call(c, 0x0025_fb74)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Singletons: destroy.

/// `void cSetupMan_Destroy(void)` (0x264098): destruct the setup manager (flag
/// 3) and forget it.
pub fn c_setup_man_destroy(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = Addr(gp_2(c, 0x2dc)).lw(c);
        let none = c.g[A0][0] == 0;
        frame.save_ra(c, 0);
        if !none {
            c.cycles += 3;
            c.g[A1][0] = 3;
            callee::C_SETUP_MAN_DTOR.call(c, 0x0026_40b0)?;
            Addr(gp_2(c, 0x2dc)).sw(c, 0);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cSharedLoginMan_Destroy(void)` (0x2658b8): destruct the shared login
/// manager (flag 3) and forget it.
pub fn c_shared_login_man_destroy(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = Addr(gp_2(c, 0x2e0)).lw(c);
        let none = c.g[A0][0] == 0;
        frame.save_ra(c, 0);
        if !none {
            c.cycles += 3;
            c.g[A1][0] = 3;
            callee::C_SHARED_LOGIN_MAN_DTOR.call(c, 0x0026_58d0)?;
            Addr(gp_2(c, 0x2e0)).sw(c, 0);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cVoipMan_Destroy(void)` (0x266cd8): destruct the VoIP manager and
/// forget it.
pub fn c_voip_man_destroy(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = Addr(gp_2(c, 0x304)).lw(c);
        let none = c.g[A0][0] == 0;
        frame.save_ra(c, 0);
        if !none {
            c.cycles += 3;
            c.g[A1][0] = 3;
            callee::C_VOIP_MAN_DTOR.call(c, 0x0026_6cf0)?;
            Addr(gp_2(c, 0x304)).sw(c, 0);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Shared login manager: memory-card requests (one wrapper per request code).

/// The memory-card request wrappers: call the request on the manager's card,
/// then record the request kind in the login-manager status word.
pub(crate) fn login_man_mc_request(c: &mut Cpu, target: u32, ret: u32, status: u32) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = Addr(gp_2(c, 0x2e8)).lw(c);
        frame.save_ra(c, 0);
        call_ptr(c, target, ret)?;
        c.g[V0][0] = status as u64;
        frame.restore_ra(c, 0);
        Addr(gp_2(c, 0x2e4)).sw(c, c.g[V0][0]);
        frame.close(c);
        Ok(())
    })
}

/// `void cSharedLoginMan_McClose(void)` (0x266148): request a card close.
pub fn c_shared_login_man_mc_close(c: &mut Cpu) {
    login_man_mc_request(c, callee::MC_REQUEST_CLOSE, 0x0026_615c, 8)
}

/// `void cSharedLoginMan_McRead(void)` (0x266298): request a card read.
pub fn c_shared_login_man_mc_read(c: &mut Cpu) {
    login_man_mc_request(c, callee::MC_REQUEST_READ, 0x0026_62ac, 6)
}

/// `void cSharedLoginMan_McWrite(void)` (0x2662c0): request a card write.
pub fn c_shared_login_man_mc_write(c: &mut Cpu) {
    login_man_mc_request(c, callee::MC_REQUEST_WRITE, 0x0026_62d4, 0xe)
}

/// `void cSharedLoginMan_McFlush(void)` (0x266320): request a card flush.
pub fn c_shared_login_man_mc_flush(c: &mut Cpu) {
    login_man_mc_request(c, callee::MC_REQUEST_FLUSH, 0x0026_6334, 7)
}

/// `void cSharedLoginMan_McGetFoundName(char* dst)` (0x266550): copy the
/// found save name out and mark the status 0x10.
pub fn c_shared_login_man_mc_get_found_name(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = 0x530000;
        frame.save_ra(c, 0);
        c.g[A1][0] = sx(0x0053_7190);
        call_ptr(c, callee::STRCPY, 0x0026_6564)?;
        c.g[V1][0] = 0x10;
        frame.restore_ra(c, 0);
        Addr(gp_2(c, 0x2e4)).sw(c, c.g[V1][0]);
        frame.close(c);
        Ok(())
    })
}

/// `LobbyMan_GetRoomName(net, i)` 0x25f150: the room name of the session in
/// the lobby list (`*(net + 0x5c)`), or null when the lobby is not up.
pub fn lobby_man_get_room_name(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let net = abi::arg(c, 0);
        let up = c.lw(net);
        c.g[V0][0] = up;
        if up == 0 {
            c.g[V0][0] = 0;
        } else {
            c.cycles += 3;
            c.g[A0][0] = lw_at(c, net as u64, 0x5c);
            call_ptr(c, table::LOBBY_LIST_GET_NAME, 0x0025_f16c)?;
            let name = c.g[V0][0];
            c.g[V0][0] = sx((name as u32).wrapping_add(0x1c));
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cSetupMan_AllocPacket()` 0x264960: allocate a 0xf0-byte setup packet from
/// the game heap.
pub fn c_setup_man_alloc_packet(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = 0x48_0000;
        frame.save_ra(c, 0);
        c.g[A0][0] = 0xf0;
        c.g[A1][0] = sx(0x48_05a8);
        c.g[A2][0] = 0x2000_0000;
        c.g[A3][0] = 0;
        callee::EA_MEM_ALLOC_NAMED.call(c, 0x0026_4980)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Shared login manager: card slots (virtual getters on the login object at
// `*(gp + 0x2e8)`).

/// `cSharedLoginMan_CardSlot20(this)` 0x265a38: slot +0xa0 of the login
/// object's vtable.
pub fn c_shared_login_man_card_slot20(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2e8));
        frame.save_ra(c, 0);
        let obj = c.lw((c.g[A1][0] as u32));
        c.g[V1][0] = obj;
        let m = VSlot::read(c, obj, 0xa0);
        c.g[A0][0] = m.delta;
        c.g[V0][0] = m.func;
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(m.delta as u32));
        call_ptr(c, m.func as u32, 0x0026_5a58)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cSharedLoginMan_CardSlot47(this)` 0x265d08: slot +0x178 of the login
/// object's vtable.
pub fn c_shared_login_man_card_slot47(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2e8));
        frame.save_ra(c, 0);
        let vt = c.lw((c.g[A1][0] as u32));
        c.g[V0][0] = vt;
        let m = VSlot::read(c, vt, 0x178);
        c.g[A0][0] = m.delta;
        c.g[V1][0] = m.func;
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(m.delta as u32));
        call_ptr(c, m.func as u32, 0x0026_5d28)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cSharedLoginMan_CardSlot63(this)` 0x265d38: slot +0x1f8 of the login
/// object's vtable.
pub fn c_shared_login_man_card_slot63(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2e8));
        frame.save_ra(c, 0);
        let vt = c.lw((c.g[A1][0] as u32));
        c.g[V1][0] = vt;
        let m = VSlot::read(c, vt, 0x1f8);
        c.g[A0][0] = m.delta;
        c.g[V0][0] = m.func;
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(m.delta as u32));
        call_ptr(c, m.func as u32, 0x0026_5d58)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cVoipMan_GetStatus(self, a, b)` 0x266f90: when the voip manager is up,
/// copy the RPC status into `a` and `b`.
pub fn c_voip_man_get_status(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let this = abi::arg(c, 0);
        let up = c.lw(this);
        c.g[V0][0] = up;
        // Both pointer arguments are passed through as whole registers.
        let a = abi::arg64(c, 1);
        let b = abi::arg64(c, 2);
        c.g[A0][0] = a;
        let idle = up == 0;
        c.g[A1][0] = b;
        if !idle {
            c.cycles += 2;
            call_ptr(c, table::VOIP_RPC_CALL, 0x0026_6fb0)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `operator delete(void *p)` (`__builtin_delete`), reached through the
/// function table as the generated code does (`call_indirect`).
pub(crate) const OPERATOR_DELETE: u32 = 0x0031_7e50;

// ---------------------------------------------------------------------------
// Deleting destructors.

/// The shared body of the `(flags & 1)` deleting destructors: republish
/// `vtable` at `self + offset` and delete `self` when bit 0 of `flags` is set.
pub(crate) fn deleting_dtor(c: &mut Cpu, vtable: u32, offset: u32, ret: u32) -> Flow {
    c.cycles += 7;
    let frame = Frame::open(c, 0x10);
    let vptr = sx(vtable);
    c.g[V0][0] = vptr;
    frame.save_ra(c, 0);
    c.g[A1][0] &= 1;
    Addr((c.g[A0][0] as u32).wrapping_add(offset)).sw(c, vptr);
    if c.g[A1][0] != 0 {
        c.cycles += 2;
        call_ptr(c, OPERATOR_DELETE, ret)?;
    }
    c.cycles += 3;
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `void FENetHook_dtor(FENetHook *self, int flags)` (0x263678): republishes
/// `D_004810C8` at `self+8`, deletes on bit 0.
pub fn fe_net_hook_dtor(c: &mut Cpu) {
    guest(c, |c| deleting_dtor(c, 0x0048_10c8, 8, 0x0026_369c))
}

/// `void FENetHook2_dtor(FENetHook2 *self, int flags)` (0x2639b0):
/// republishes `D_004810B0` at `self+0x14`, deletes on bit 0.
pub fn fe_net_hook2_dtor(c: &mut Cpu) {
    guest(c, |c| deleting_dtor(c, 0x0048_10b0, 0x14, 0x0026_39d4))
}

/// `void NetEvent_dtor(NetEvent *self, int flags)` (0x267e18): republishes
/// `D_004812B0` at `self+4`, deletes on bit 0.
pub fn net_event_dtor(c: &mut Cpu) {
    guest(c, |c| deleting_dtor(c, 0x0048_12b0, 4, 0x0026_7e3c))
}

/// `void NetEvent102_dtor(NetEvent102 *self, int flags)` (0x267e98): the same
/// `D_004812B0` destructor.
pub fn net_event102_dtor(c: &mut Cpu) {
    guest(c, |c| deleting_dtor(c, 0x0048_12b0, 4, 0x0026_7ebc))
}

/// `void NetEvent102Base_dtor(NetEvent102Base *self, int flags)` (0x267f28):
/// the same `D_004812B0` destructor.
pub fn net_event102_base_dtor(c: &mut Cpu) {
    guest(c, |c| deleting_dtor(c, 0x0048_12b0, 4, 0x0026_7f4c))
}

/// `void NetErrorEvent_dtor(NetErrorEvent *self, int flags)` (0x267fb8): the
/// same `D_004812B0` destructor.
pub fn net_error_event_dtor(c: &mut Cpu) {
    guest(c, |c| deleting_dtor(c, 0x0048_12b0, 4, 0x0026_7fdc))
}

/// `void NetEvent102Tooy_dtor(NetEvent102Tooy *self, int flags)` (0x268210):
/// the same `D_004812B0` destructor.
pub fn net_event102_tooy_dtor(c: &mut Cpu) {
    guest(c, |c| deleting_dtor(c, 0x0048_12b0, 4, 0x0026_8234))
}

/// `void NetEventFB_dtor(NetEventFB *self, int flags)` (0x268658): the same
/// `D_004812B0` destructor.
pub fn net_event_fb_dtor(c: &mut Cpu) {
    guest(c, |c| deleting_dtor(c, 0x0048_12b0, 4, 0x0026_867c))
}

/// `void NetEventFC_dtor(NetEventFC *self, int flags)` (0x2686f0): the same
/// `D_004812B0` destructor.
pub fn net_event_fc_dtor(c: &mut Cpu) {
    guest(c, |c| deleting_dtor(c, 0x0048_12b0, 4, 0x0026_8714))
}

/// `void NetEvent10D_dtor(NetEvent10D *self, int flags)` (0x268790): the same
/// `D_004812B0` destructor.
pub fn net_event10d_dtor(c: &mut Cpu) {
    guest(c, |c| deleting_dtor(c, 0x0048_12b0, 4, 0x0026_87b4))
}

// ---------------------------------------------------------------------------
// Lobby accessors.

/// `char *LobbyMan_GetKickReason(LobbyMan *self)` (0x25da90): the kick reason
/// string at `self+0x84`, but only when the session is live (`self+0x54`) and
/// the string is not empty.
pub fn lobby_man_get_kick_reason(c: &mut Cpu) {
    /// Non-zero while the lobby session is connected.
    const SESSION: u32 = 0x54;
    /// `char *` to the server's last kick message.
    const REASON: u32 = 0x84;

    c.cycles += 3;
    let this = abi::arg(c, 0);
    c.g[V0][0] = Addr(this).off(SESSION).lw(c);
    if c.g[V0][0] != 0 {
        c.cycles += 2;
        c.g[V0][0] = Addr(this).off(REASON).lw(c);
        if c.g[V0][0] != 0 {
            c.cycles += 3;
            c.g[V1][0] = Addr(c.g[V0][0] as u32).lb(c);
            if c.g[V1][0] == 0 {
                c.g[V0][0] = 0;
            }
        } else {
            c.g[V0][0] = 0;
        }
        c.cycles += 2;
    } else {
        c.cycles += 2;
        c.g[V0][0] = 0;
    }
    abi::ret(c);
}

/// `int LobbyMan_GetRoomCount(LobbyMan *self)` (0x25f120): the visible room
/// count of the room list at `self+0x5c`, or 0 when the lobby is not up.
pub fn lobby_man_get_room_count(c: &mut Cpu) {
    guest(c, |c| {
        /// `Lobby_List *` of rooms.
        const ROOMS: u32 = 0x5c;

        c.cycles += 5;
        let this = abi::arg(c, 0);
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(this).lw(c);
        let live = c.g[V0][0] != 0;
        c.g[V0][0] = 0;
        if live {
            c.cycles += 4;
            c.g[A0][0] = Addr(this).off(ROOMS).lw(c);
            callee::LOBBY_LIST_VISIBLE_COUNT.call(c, 0x0025_f13c)?;
        } else {
            c.cycles += 1;
        }
        frame.restore_ra(c, 0);
        c.cycles += 2;
        frame.close(c);

        Ok(())
    });
}

/// `int LobbyMan_IsRoomFlagged(LobbyMan *self, int index)` (0x25f180): bit 0
/// of the `int16` at `+0xa` of room `index` in the list at `self+0x5c`.
pub fn lobby_man_is_room_flagged(c: &mut Cpu) {
    guest(c, |c| {
        /// `Lobby_List *` of rooms.
        const ROOMS: u32 = 0x5c;

        c.cycles += 5;
        let this = abi::arg(c, 0);
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(this).lw(c);
        if c.g[V0][0] == 0 {
            c.g[V0][0] = 0;
        } else {
            c.cycles += 4;
            c.g[A0][0] = Addr(this).off(ROOMS).lw(c);
            callee::LOBBY_LIST_GET_AT.call(c, 0x0025_f19c)?;
            c.g[V0][0] = Addr(c.g[V0][0] as u32).off(0xa).lh(c);
            c.g[V0][0] &= 1;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);

        Ok(())
    });
}

/// `int LobbyMan_GetRoomUserCount(LobbyMan *self)` (0x25f1b0): the visible
/// user count of the list at `self+0x60`, or 0 when the lobby is not up.
pub fn lobby_man_get_room_user_count(c: &mut Cpu) {
    guest(c, |c| {
        /// `Lobby_List *` of the room's users.
        const USERS: u32 = 0x60;

        c.cycles += 5;
        let this = abi::arg(c, 0);
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(this).lw(c);
        let live = c.g[V0][0] != 0;
        c.g[V0][0] = 0;
        if live {
            c.cycles += 4;
            c.g[A0][0] = Addr(this).off(USERS).lw(c);
            callee::LOBBY_LIST_VISIBLE_COUNT.call(c, 0x0025_f1cc)?;
        } else {
            c.cycles += 1;
        }
        frame.restore_ra(c, 0);
        c.cycles += 2;
        frame.close(c);

        Ok(())
    });
}

// ---------------------------------------------------------------------------
// Buddy accessors.

/// `Lobby_List *` holding the buddy roster (`self + 0xcc`).
pub(crate) const BUDDY_ROSTER: u32 = 0xcc;

/// The shared body of the seven `BuddyMan_*` accessors: find the roster entry
/// for the name in `a1` and forward its index to `query(self, index)`.
pub(crate) fn roster_lookup(c: &mut Cpu, query: Callee, ret: u32) -> Flow {
    c.cycles += 12;
    let frame = Frame::open(c, 0x20);
    frame.save(c, S0, 0x10);
    frame.save_ra(c, 0);
    c.g[S0][0] = c.g[A0][0];
    callee::BUDDY_FIND_ROSTER_ENTRY.call(c, ret)?;
    c.g[A0][0] = c.g[S0][0];
    c.g[A1][0] = c.g[V0][0];
    query.call(c, ret + 0xc)?;
    frame.restore(c, S0, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `bool BuddyMan_IsBuddy(BuddyMan *self, char *name)` (0x262360).
pub fn buddy_man_is_buddy(c: &mut Cpu) {
    guest(c, |c| {
        roster_lookup(c, callee::BUDDY_IS_BUDDY_AT, 0x0026_2374)
    })
}

/// `bool BuddyMan_IsBlocked(BuddyMan *self, char *name)` (0x2623b0).
pub fn buddy_man_is_blocked(c: &mut Cpu) {
    guest(c, |c| {
        roster_lookup(c, callee::BUDDY_IS_BLOCKED_AT, 0x0026_23c4)
    })
}

/// `bool BuddyMan_IsTemp(BuddyMan *self, char *name)` (0x262438).
pub fn buddy_man_is_temp(c: &mut Cpu) {
    guest(c, |c| {
        roster_lookup(c, callee::BUDDY_IS_TEMP_AT, 0x0026_244c)
    })
}

/// `int BuddyMan_CountMessagesFrom(BuddyMan *self, char *name)` (0x262a08).
pub fn buddy_man_count_messages_from(c: &mut Cpu) {
    guest(c, |c| {
        roster_lookup(c, callee::BUDDY_COUNT_MESSAGES_FROM_AT, 0x0026_2a1c)
    })
}

/// `bool BuddyMan_IsOnline(BuddyMan *self, char *name)` (0x2633c8).
pub fn buddy_man_is_online(c: &mut Cpu) {
    guest(c, |c| {
        roster_lookup(c, callee::BUDDY_IS_ONLINE_AT, 0x0026_33dc)
    })
}

/// `bool BuddyMan_IsSameProduct(BuddyMan *self, char *name)` (0x263490).
pub fn buddy_man_is_same_product(c: &mut Cpu) {
    guest(c, |c| {
        roster_lookup(c, callee::BUDDY_IS_SAME_PRODUCT_AT, 0x0026_34a4)
    })
}

/// `int BuddyMan_GetRosterCount(BuddyMan *self)` (0x262598): the counted
/// entries of the roster list at `self+0xcc`, or 0 when there is no list.
pub fn buddy_man_get_roster_count(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let this = abi::arg(c, 0);
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr(this).off(BUDDY_ROSTER).lw(c);
        let has_list = c.g[A0][0] != 0;
        c.g[V0][0] = 0;
        if has_list {
            c.cycles += 4;
            callee::LOBBY_LIST_COUNT.call(c, 0x0026_25b4)?;
        } else {
            c.cycles += 1;
        }
        frame.restore_ra(c, 0);
        c.cycles += 2;
        frame.close(c);

        Ok(())
    });
}

// ---------------------------------------------------------------------------
// Login manager.

/// `bool cSharedLoginMan_IsCardMissing(void)` (0x265d68): negate the login
/// manager's virtual `+0x1a8/0x1ac` slot ("is the memory card present").
pub fn c_shared_login_man_is_card_missing(c: &mut Cpu) {
    /// `gp + 0x2e8`: the shared login manager singleton.
    const LOGIN_MANAGER: u32 = 0x2e8;

    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = Addr((c.g[GP][0] as u32).wrapping_add(LOGIN_MANAGER)).lw(c);
        frame.save_ra(c, 0);
        c.g[V1][0] = Addr(c.g[A1][0] as u32).lw(c);
        let slot = Addr(c.g[V1][0] as u32).off(0x1a8);
        c.g[A0][0] = slot.lh(c);
        c.g[V0][0] = slot.off(4).lw(c);
        let method = c.g[V0][0] as u32;
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, method, 0x0026_5d88)?;
        frame.restore_ra(c, 0);
        c.g[V0][0] ^= 1;
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Buddy list.

/// `bool BuddyMan_IsTempAt(BuddyMan* self, int index)` (0x262400): the
/// "temporary buddy" flag of roster entry `index` (bit 20 of +0xa8).
pub fn buddy_man_is_temp_at(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        callee::BUDDY_MAN_GET_ROSTER_ENTRY.call(c, 0x0026_2410)?;
        // The generated branches to the epilogue straight from the empty
        // roster case; the shift-and-mask (+2) belongs to the found-entry
        // path only.
        if c.g[V0][0] != 0 {
            c.g[V0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0xa8)).lw(c);
            c.cycles += 2;
            c.g[V0][0] = sx((((c.g[V0][0] as u32) as i32) >> 20) as u32);
            c.g[V0][0] &= 1;
        } else {
            c.cycles += 2;
            c.g[V0][0] = 0;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// Login manager.

/// `void cSharedLoginMan_DoDelete(cSharedLoginMan* self)` (0x265ec8): clear
/// the pending entry name and kick off the delete operation.
pub fn shared_login_do_delete(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        Addr((c.g[S0][0] as u32).wrapping_add(4)).sw(c, 0);
        Addr((c.g[GP][0] as u32).wrapping_add(0x2f0)).sw(c, 0);
        call_ptr(c, callee::SLOGIN_DELETE, 0x0026_5ee4)?;
        c.g[V1][0] = 0xf;
        frame.restore_ra(c, 0);
        Addr(c.g[S0][0] as u32).sw(c, c.g[V1][0]);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    });
}

/// `void LobbyMan_DestroyPeerTable(LobbyMan* self)` (0x25a6f8): destroy the
/// network peer table at `+0xf4` and clear the pointer.
pub fn lobby_man_destroy_peer_table(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = Addr(c.g[S0][0] as u32).off(0xf4).lw(c);
        if c.g[A0][0] == 0 {
            frame.restore(c, S0, 0x10);
        } else {
            c.cycles += 4;
            callee::NET_GAME_UTIL_DESTROY.call(c, 0x0025_a71c)?;
            let s0 = c.g[S0][0];
            Addr(s0 as u32).off(0xf4).sw(c, 0);
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Network singletons.

/// `void cSetupMan_SendHello(cSetupMan* self)` (0x264cb8): post the 0x14-byte
/// hello message (type 16, lobby state at `+0xb8`).
pub fn csetup_man_send_hello(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0xc8)).lw(c);
        // The 0x14-byte message lives at the frame base; `ra` is saved at
        // +0x20 (the argument registers must not clobber it).
        let msg = frame.at(c, 0);
        frame.save_ra(c, 0x20);
        c.g[A3][0] = 0x10;
        c.g[A1][0] = c.g[SP][0];
        c.g[A2][0] = 0x14;
        c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0xb8).lw(c);
        let a3 = c.g[A3][0];
        let v1 = c.g[V1][0];
        Addr(msg).sw(c, a3);
        Addr(msg).off(4).sw(c, v1);
        Addr(msg).off(8).sw(c, 0);
        callee::CSETUP_MAN_SEND_PACKET.call(c, 0x0026_4ce4)?;
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `void* TibExtMemAlloc(int size, int kind)` (0x266728): tagged allocation of
/// `size + 0x400` bytes (0x23000000 tag for kind 0x20).
pub fn tib_ext_mem_alloc(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x400));
        c.g[V1][0] = c.g[A2][0] ^ 0x20;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0x2000_0000;
        c.g[A2][0] = 0x2300_0000;
        c.g[A1][0] = 0x0048_0000;
        frame.save_ra(c, 0);
        if c.g[V1][0] != 0 {
            c.g[A2][0] = c.g[V0][0];
        }
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1080));
        c.g[A3][0] = 0;
        callee::OPERATOR_NEW_NAMED.call(c, 0x0026_6754)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void* cVoipMan_Create(void)` (0x266ca0): allocate the eight-byte VoIP
/// manager and construct it.
pub fn cvoip_man_create(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = 0x0048_0000;
        frame.save_ra(c, 0);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x10a0));
        c.g[A2][0] = 0x2000_0000;
        c.g[A3][0] = 0;
        c.g[A0][0] = 8;
        callee::EA_MEM_ALLOC.call(c, 0x0026_6cc0)?;
        c.g[A0][0] = c.g[V0][0];
        callee::CVOIP_MAN_CTOR.call(c, 0x0026_6cc8)?;
        frame.restore_ra(c, 0);
        let v0 = c.g[V0][0];
        Addr((c.g[GP][0] as u32).wrapping_add(0x304)).sw(c, v0);
        frame.close(c);
        Ok(())
    })
}

/// `void cVoipMan_Stop(cVoipMan* self)` (0x266df8): stop the VoIP RPC session
/// and clear the stored handle.
pub fn cvoip_man_stop(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
        if c.g[V0][0] == 0 {
            frame.restore(c, S0, 0x10);
        } else {
            c.cycles += 4;
            callee::VOIP_RPC_STOP.call(c, 0x0026_6e1c)?;
            let s0 = c.g[S0][0];
            Addr(s0 as u32).sw(c, 0);
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cVoipMan_SetVolume(cVoipMan* self, int level)` (0x266f58): push the
/// level to the RPC layer when the session is live and remember it.
pub fn cvoip_man_set_volume(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        c.g[V0][0] = Addr(abi::arg(c, 0)).lw(c);
        let silent = c.g[V0][0] == 0;
        c.g[A1][0] = c.g[A2][0];
        if !silent {
            c.cycles += 2;
            c.g[A0][0] = c.g[S0][0];
            callee::VOIP_RPC_SET_PARAMS.call(c, 0x0026_6f7c)?;
        }
        c.cycles += 5;
        let s0 = c.g[S0][0];
        Addr((c.g[GP][0] as u32).wrapping_add(0x308)).sw(c, s0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// A `gp_3`-relative global word.
#[inline(always)]
pub(crate) fn gp_3(c: &Cpu, off: i32) -> u32 {
    r(c, GP).wrapping_add(off as u32)
}

// ---------------------------------------------------------------------------
// Login / USB keyboard.

/// `void cSharedLoginMan_DoUpdate(cSharedLoginMan* self)` (0x265E40): drop the
/// pending request, clear the shared-login global and tell the SLogin service
/// to update; the manager's state becomes 7.
pub fn shared_login_man_do_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        frame.save_ra(c, 0);
        Addr((this as u32).wrapping_add(4)).sw(c, 0);
        c.g[A0][0] = 0x0053_6760;
        Addr(gp_3(c, 0x2f0)).sw(c, 0);
        callee::S_LOGIN_UPDATE.call(c, 0x0026_5e64)?;
        c.g[V1][0] = 7;
        frame.restore_ra(c, 0);
        Addr(this as u32).sw(c, 7);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void cUsbKeyboard_OnConnect(cUsbKeyboard* self)` (0x2668A8): install the
/// default connect callback and reset the keyboard's arrangement and read
/// mode to zero.
pub fn usb_keyboard_on_connect(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::USB_KB_DEFAULT_CONNECT_CB.call(c, 0x0026_68bc)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0;
        callee::SCE_USBKB_SET_ARRANGEMENT.call(c, 0x0026_68c8)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0;
        callee::SCE_USBKB_SET_READ_MODE.call(c, 0x0026_68d4)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Lobby / buddies / login.

/// `bool LobbyMan_IsRoomUserFlagged(LobbyMan* self)` (0x25f248): true when the
/// room user at `[self+0x60]` (a lobby list) exists and its `+4` bit 21 is set.
pub fn lobby_man_is_room_user_flagged(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let self_at = abi::arg(c, 0);
        c.g[V0][0] = Addr(self_at).lw(c);
        let empty = c.g[V0][0] == 0;
        c.g[V0][0] = 0;
        if !empty {
            c.cycles += 4;
            c.g[A0][0] = Addr(self_at).off(0x60).lw(c);
            callee::LOBBY_LIST_GET_AT_2.call(c, 0x0025_f264)?;
            if c.g[V0][0] != 0 {
                c.g[V0][0] = Addr(r32(c, V0)).off(4).lw(c);
                c.cycles += 2;
                c.g[V0][0] = sx(((r32(c, V0) as i32) >> 21) as u32);
                c.g[V0][0] &= 1;
            } else {
                c.cycles += 2;
                c.g[V0][0] = 0;
            }
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        leave(c, 0x10);
        Ok(())
    })
}

/// `void BuddyMan_Reconnect(BuddyMan* self)` (0x2613c8): when the reconnect
/// index `+0x40` is valid and `+0xd8` is clear, reconnect through
/// `BuddyMan_Connect` (self, index, name at +0x44, message at +0x64, `+0xec`).
pub fn buddy_man_reconnect(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let self_at = abi::arg(c, 0);
        c.g[A2][0] = Addr(self_at).off(0x40).lw(c);
        let bad_index = (c.g[A2][0] as i64) < 0;
        frame.restore_ra(c, 0);
        if !bad_index {
            c.cycles += 4;
            c.g[V0][0] = Addr(self_at).off(0xd8).lw(c);
            c.g[V0][0] ^= 1;
            let busy = c.g[V0][0] == 0;
            c.g[A1][0] = c.g[A0][0];
            if !busy {
                c.cycles += 5;
                c.g[T1][0] = Addr(self_at).off(0xec).lw(c);
                c.g[A3][0] = sx(p(self_at, 0x44));
                c.g[T0][0] = sx(p(self_at, 0x64));
                callee::BUDDY_MAN_CONNECT.call(c, 0x0026_13fc)?;
                frame.restore_ra(c, 0);
            }
        }
        c.cycles += 2;
        leave(c, 0x10);
        Ok(())
    })
}

/// `int BuddyMan_RosterCompare(void* a /*a2*/, void* b /*a3*/)`
/// (0x2634c0): a null `a` sorts after anything but null, a null `b` before
/// everything, else `strcasecmp(a, b)`.
pub fn buddy_man_roster_compare(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = c.g[A3][0];
        let mut called = false;
        if c.g[A0][0] == 0 {
            c.cycles += 2;
            c.g[V0][0] = (0u64 < c.g[A1][0]) as u64;
        } else {
            c.cycles += 2;
            let b_null = c.g[A1][0] == 0;
            c.g[V0][0] = u64::MAX;
            if !b_null {
                c.cycles += 4;
                callee::STR_CASE_CMP.call(c, 0x0026_34ec)?;
                called = true;
                frame.restore_ra(c, 0);
            }
        }
        if !called {
            c.cycles += 1;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        leave(c, 0x10);
        Ok(())
    })
}

/// `void cSharedLoginMan_McGetInfo(u32 code)` (0x266060): ask the login
/// manager for the memcard port slot, then start the memcard info request
/// with the error byte it returned.
pub fn c_shared_login_man_mc_get_info(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        frame.save_ra(c, 0x10);
        c.g[A1][0] = c.g[SP][0];
        callee::SLOGIN_MC_GET_PORT_SLOT.call(c, 0x0026_6070)?;
        c.g[V1][0] = Addr(sp(c)).lbu(c);
        c.g[V0][0] = 9;
        load_gp(c, A0, 0x2e8);
        c.sw(gp_4(c, 0x2f4), c.g[V0][0]);
        c.g[A1][0] = c.g[V1][0];
        c.sw(gp_4(c, 0x2ec), c.g[V1][0]);
        callee::MEMCARD_REQUEST_GET_INFO_NOW.call(c, 0x0026_608c)?;
        c.g[V0][0] = 1;
        frame.restore_ra(c, 0x10);
        c.sw(gp_4(c, 0x2e4), c.g[V0][0]);
        leave(c, 0x20);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Small helpers.

/// The address `reg + off`.
#[inline(always)]
pub(crate) fn at(c: &Cpu, reg: usize, off: u32) -> u32 {
    r32(c, reg).wrapping_add(off)
}

/// The address `base + off`.
#[inline(always)]
pub(crate) fn p_2(base: u32, off: u32) -> u32 {
    base.wrapping_add(off)
}

/// A `gp_5`-relative global.
#[inline(always)]
pub(crate) fn gp_5(c: &Cpu, off: i32) -> u32 {
    r32(c, GP).wrapping_add(off as u32)
}

/// `sw reg, base + off`.
#[inline(always)]
pub(crate) fn sw(c: &mut Cpu, base: u32, off: u32, v: u64) {
    c.sw(p_2(base, off), v)
}

/// The constant an original builds with `lui reg, hi; addiu reg, reg, lo`
/// (the vtables): its 32-bit value, sign-extended.
#[inline(always)]
pub(crate) fn const_reg(c: &mut Cpu, reg: usize, hi: u32, lo: i32) {
    c.g[reg][0] = sx(hi.wrapping_add(lo as u32));
}

// ---------------------------------------------------------------------------
// Lobby manager.

/// `void* LobbyMan_LookupPeerAddr(LobbyMan* self, int key /*a1*/, void* def
/// /*a2*/)` (0x25a730): the address the lobby API returns for `key`, or
/// `def` when nothing is registered.
pub fn lobby_man_lookup_peer_addr(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = c.g[A1][0];
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = c.lw(at(c, A0, 0xf4));
        let no_table = c.g[A0][0] == 0;
        c.g[S0][0] = c.g[A2][0];
        if !no_table {
            c.cycles += 5;
            c.g[A1][0] = 0x48_0000;
            c.g[A2][0] = c.g[V0][0];
            const_reg(c, A1, 0x48_0000, 0x13a0);
            call_ptr(c, callee::VID_ENGINE_PLAYER_REQUEST, 0x0025_a75c)?;
            if c.g[V0][0] != 0 {
                c.g[S0][0] = c.g[V0][0];
            }
        }
        c.cycles += 5;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        leave(c, 0x20);
        Ok(())
    });
}

/// `void LobbyMan_DestroyLobbyApi(LobbyMan* self)` (0x25aa98): destroy the
/// peer lists, then release the lobby API objects.
pub fn lobby_man_destroy_lobby_api(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::LOBBY_MAN_DESTROY_LISTS.call(c, 0x0025_aaac)?;
        c.g[A0][0] = c.lw(at(c, S0, 0x54));
        if c.g[A0][0] == 0 {
            frame.restore(c, S0, 0x10);
        } else {
            c.cycles += 6;
            call_ptr(c, callee::LOBBY_MAN_REQUEST_UPDATE, 0x0025_aac0)?;
            c.g[A0][0] = c.lw(at(c, S0, 0x54));
            call_ptr(c, callee::LOBBY_MAN_REQUEST_FREE, 0x0025_aac8)?;
            sw(c, r32(c, S0), 0x54, 0);
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void LobbyMan_SendRequest(LobbyMan* self, int tag /*a1*/, int cb /*a2*/,
/// int arg /*a3*/, int a4 /*t0*/, float f /*f12*/)` (0x25b608): fill the
/// request record and hand it to the lobby API.
pub fn lobby_man_send_request(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let this = r32(c, S0);
        c.sw(this, c.g[A1][0]);
        c.g[A1][0] = c.g[A2][0];
        c.g[A0][0] = c.lw(p_2(this, 0x54));
        c.g[A2][0] = c.g[A3][0];
        c.swc1(12, p_2(this, 0xe0));
        c.g[A3][0] = c.g[T0][0];
        c.g[T0][0] = c.g[S0][0];
        call_ptr(c, callee::LOBBY_API_ADD_REQUEST, 0x0025_b638)?;
        sw(c, r32(c, S0), 0x40, c.g[V0][0]);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        leave(c, 0x20);
        Ok(())
    });
}

/// `void LobbyMan_Disconnect(LobbyMan* self)` (0x25b800): drop the peer
/// table, then the lobby API, and clear the connection fields.
pub fn lobby_man_disconnect(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::LOBBY_MAN_DESTROY_PEER_TABLE.call(c, 0x0025_b814)?;
        c.g[V0][0] = c.lw(at(c, S0, 0x54));
        if c.g[V0][0] == 0 {
            sw(c, r32(c, S0), 0, 0);
        } else {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            callee::LOBBY_MAN_DESTROY_LOBBY_API.call(c, 0x0025_b828)?;
            sw(c, r32(c, S0), 0, 0);
        }
        c.cycles += 6;
        sw(c, r32(c, S0), 0x08, 0);
        sw(c, r32(c, S0), 0x0c, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// Login and voice managers.

/// `void cSharedLoginMan_dtor(cSharedLoginMan* self, int flags)` (0x265908):
/// shut the login manager down, then `operator delete` it when `flags & 1`.
pub fn shared_login_man_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        callee::SHARED_LOGIN_MAN_SHUTDOWN.call(c, 0x0026_5924)?;
        c.g[S0][0] = c.g[S0][0] & 1;
        let keep = c.g[S0][0] == 0;
        frame.restore(c, S0, 0x20);
        if !keep {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, callee::BUILTIN_DELETE, 0x0026_5938)?;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void cSharedLoginMan_DoCreate(cSharedLoginMan* self)` (0x265e80):
/// create the login service (the dirty-socket tag) and mark the manager
/// created.
pub fn shared_login_man_do_create(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x20);
        const_reg(c, A1, 0x53_0000, 0x6760);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        sw(c, r32(c, S0), 0x04, 0);
        const_reg(c, A0, 0x44_0000, 0x2a60);
        sw(c, gp_5(c, 0x2f0), 0, 0);
        call_ptr(c, callee::DIRTY_SOCK_CREATE, 0x0026_5eac)?;
        c.g[V1][0] = 0xb;
        frame.restore_ra(c, 0);
        sw(c, r32(c, S0), 0, c.g[V1][0]);
        frame.restore(c, S0, 0x10);
        leave(c, 0x20);
        Ok(())
    });
}

/// `void cVoipMan_dtor(cVoipMan* self, int flags)` (0x266d10): stop the voice
/// manager, then `operator delete` it when `flags & 1`.
pub fn voip_man_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        callee::VOIP_MAN_STOP.call(c, 0x0026_6d2c)?;
        c.g[S0][0] = c.g[S0][0] & 1;
        let keep = c.g[S0][0] == 0;
        frame.restore(c, S0, 0x20);
        if !keep {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, callee::BUILTIN_DELETE, 0x0026_6d40)?;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void cVoipMan_Deactivate(cVoipMan* self)` (0x266e88): tear the voice
/// session down when both the manager and its session are live.
pub fn voip_man_deactivate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = c.lw(r32(c, S0));
        if c.g[V0][0] == 0 {
            frame.restore(c, S0, 0x10);
        } else {
            c.cycles += 3;
            c.g[V0][0] = c.lw(at(c, S0, 0x04));
            if c.g[V0][0] == 0 {
                frame.restore(c, S0, 0x10);
            } else {
                c.cycles += 4;
                call_ptr(c, callee::DIRTY_SOCK_DESTROY, 0x0026_6eb8)?;
                sw(c, r32(c, S0), 0x04, 0);
                frame.restore(c, S0, 0x10);
            }
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// Lobby.

/// `bool LobbyMan_IsInRoom(self)` (0x25f430): true when the lobby manager is
/// active (`*(self)`) and its room name (`+0x54`) contains "room".
pub fn lobby_man_is_in_room(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let self_ = abi::r32(c, A0);
        c.g[V0][0] = Addr(self_).lw(c);
        if c.g[V0][0] == 0 {
            c.cycles += 2;
            c.g[V0][0] = 0;
            c.cycles += 3;
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        c.g[A0][0] = Addr(self_.wrapping_add(0x54)).lw(c);
        c.cycles += 5;
        c.g[A1][0] = 0x726f_0000;
        c.g[A1][0] |= 0x6f6d;
        call_ptr(c, 0x003e_8a70, 0x0025_f458)?;
        let missing = c.g[V0][0] == 0;
        c.g[V1][0] = 0;
        if !missing {
            c.cycles += 2;
            c.g[V0][0] = Addr(c.g[V0][0] as u32).lb(c);
            c.g[V1][0] = (0i64 < c.g[V0][0] as i64) as u64;
        }
        c.cycles += 1;
        c.g[V0][0] = c.g[V1][0];
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void LobbyMan_ResetChalState(self)` (0x25f858): tell the lobby service
/// about the challenge ("chal") with the state block at 0x4a2f40 and mark
/// the manager as waiting (`*(self) = 1`).
pub fn lobby_man_reset_chal_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x20);
        c.g[A2][0] = 0x4a_0000;
        frame.save(c, S0, 0x10);
        c.g[A1][0] = 0x6368_0000;
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A2][0] = sx(0x4a_0000u32.wrapping_add(0x2f40));
        c.g[A1][0] |= 0x616c;
        let self_ = abi::r32(c, S0);
        c.g[A0][0] = Addr(self_.wrapping_add(0x54)).lw(c);
        c.g[A3][0] = 0;
        c.g[T0][0] = 0;
        call_ptr(c, 0x003e_8b10, 0x0025_f888)?;
        c.g[V1][0] = 1;
        Addr(self_).sw(c, c.g[V1][0]);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void BuddyMan_CreateApi(self, lobby)` (0x2614e8): create the buddy API
/// object (`0x480e58` tag) for the lobby handle and register `self` as its
/// callback data with `0x268f40`.
pub fn buddy_man_create_api(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 0x48_0000;
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[A1][0];
        c.g[A1][0] = sx(0x48_0000u32.wrapping_add(0xe58));
        call_ptr(c, 0x003f_0710, 0x0026_1508)?;
        let self_ = abi::r32(c, S0);
        c.g[A1][0] = 0x27_0000;
        c.g[A2][0] = c.g[S0][0];
        Addr(self_.wrapping_add(0xc8)).sw(c, c.g[V0][0]);
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = sx(0x27_0000u32.wrapping_sub(0x70c0));
        call_ptr(c, 0x003f_17c8, 0x0026_1520)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void Net_StartConnect(self)` (0x2636a8): start the net connection on the
/// comm system (`gp_6-0x238`), then arm the retry timer: `+0x4` gets the
/// connect timeout (`gp_6-0x1e4`, or `gp_6-0x1e0` while the comm system is idle)
/// and `*(self) = 1`.
pub fn net_start_connect(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr(gp_6(c, -0x238)).lw(c);
        call_ptr(c, 0x0025_5f50, 0x0026_36c0)?;
        c.g[V0][0] = Addr(gp_6(c, -0x238)).lw(c);
        let comm = c.g[V0][0] as u32;
        c.g[V1][0] = Addr(comm.wrapping_add(0x2c)).lw(c);
        let busy = c.g[V1][0] != 0;
        c.lwc1(0, gp_6(c, -0x1e4));
        if !busy {
            c.cycles += 1;
            c.lwc1(0, gp_6(c, -0x1e0));
        }
        c.cycles += 7;
        c.g[V0][0] = 1;
        let self_ = abi::r32(c, S0);
        c.swc1(0, self_.wrapping_add(4));
        Addr(self_).sw(c, c.g[V0][0]);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void BuddyMan_Destroy(BuddyMan* this)` (0x261008): tear down the global
/// buddy manager once (`gp_7+0x238`) and clear the global slot.
pub fn buddy_man_destroy(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = Addr(gp_7(c, 0x238)).lw(c);
        let absent = c.g[A0][0] == 0;
        frame.save_ra(c, 0);
        if !absent {
            c.cycles += 9;
            callee::BUDDY_MAN_CLEAR_LOGIN.call(c, 0x0026_1020)?;
            c.g[A0][0] = Addr(gp_7(c, 0x238)).lw(c);
            callee::BUDDY_MAN_CLEAR_MESSAGES.call(c, 0x0026_1028)?;
            c.g[A0][0] = Addr(gp_7(c, 0x238)).lw(c);
            callee::BUDDY_MAN_DISCONNECT.call(c, 0x0026_1030)?;
            c.g[A0][0] = Addr(gp_7(c, 0x238)).lw(c);
            if c.g[A0][0] == 0 {
                Addr(gp_7(c, 0x238)).sw(c, 0);
            } else {
                c.cycles += 3;
                c.g[A1][0] = 3;
                call_ptr(c, 0x0026_89b8, 0x0026_1044)?;
                Addr(gp_7(c, 0x238)).sw(c, 0);
            }
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void BuddyMan_Update(BuddyMan* this)` (0x261490): run the login update
/// and drop the connection when the pending flag is set.
pub fn buddy_man_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = Addr(this).off(0xc8).lw(c);
        if c.g[A0][0] != 0 {
            c.cycles += 5;
            call_ptr(c, 0x003f_17d8, 0x0026_14b4)?;
            c.g[V0][0] = Addr(this).off(0xdc).lw(c);
            if c.g[V0][0] != 0 {
                c.cycles += 4;
                c.g[A0][0] = c.g[S0][0];
                callee::BUDDY_MAN_DISCONNECT.call(c, 0x0026_14c8)?;
                Addr(this).off(0xdc).sw(c, 0);
            }
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cSharedLoginMan_SetAccountName(cSharedLoginMan* this, const char*
/// name)` (0x265f18): clear the 17-byte account-name buffer and copy `name`
/// into it.
pub fn c_shared_login_man_set_account_name(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x30);
        c.g[A1][0] = 0;
        frame.save(c, S0, 0x20);
        c.g[A2][0] = 0x11;
        frame.save(c, S1, 0x10);
        c.g[S0][0] = 0x530000;
        c.g[S0][0] = sx(0x0053_6760);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x003e_6448, 0x0026_5f44)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x0041_66f4, 0x0026_5f50)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cSharedLoginMan_SetAccountPassword(cSharedLoginMan* this, const
/// char* password)` (0x265f68): same as the name setter, one field later.
pub fn c_shared_login_man_set_account_password(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x30);
        c.g[A1][0] = 0;
        frame.save(c, S0, 0x20);
        c.g[A2][0] = 0x11;
        frame.save(c, S1, 0x10);
        c.g[S0][0] = 0x530000;
        c.g[S0][0] = sx(0x0053_6771);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x003e_6448, 0x0026_5f94)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x0041_66f4, 0x0026_5fa0)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `D_004A3028`: the lobby instance; `D_004A2EEC`: the game comm object.
pub(crate) const LOBBY_INSTANCE_GP: i32 = -0xc8;

pub(crate) const GAME_COMM_GP: i32 = -0x204;

/// A `gp_8`-relative global address.
#[inline(always)]
pub(crate) fn gp_8(c: &Cpu, off: i32) -> u32 {
    r32(c, GP).wrapping_add(off as u32)
}

// ---------------------------------------------------------------------------
// Lobby / setup managers.

/// `void LobbyMan_Destroy()` (0x25b688): tear the lobby manager down (and, if
/// it is still alive, delete it), then the game comm object.
/// 4 (+5, +3) + 3 (+3) + 2 cycles plus the callees.
pub fn lobby_man_destroy(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        let lobby = c.lw(gp_8(c, LOBBY_INSTANCE_GP));
        c.g[A0][0] = lobby;
        let none = lobby == 0;
        frame.save_ra(c, 0);
        if !none {
            c.cycles += 5;
            callee::LOBBY_MAN_DISCONNECT.call(c, 0x0025_b6a0)?;
            let alive = c.lw(gp_8(c, LOBBY_INSTANCE_GP));
            c.g[A0][0] = alive;
            if alive == 0 {
                c.sw(gp_8(c, LOBBY_INSTANCE_GP), 0);
            } else {
                c.cycles += 3;
                c.g[A1][0] = 3;
                callee::LOBBY_MAN_DTOR.call(c, 0x0025_b6b4)?;
                c.sw(gp_8(c, LOBBY_INSTANCE_GP), 0);
            }
        }
        c.cycles += 3;
        c.g[V0][0] = c.lw(gp_8(c, GAME_COMM_GP));
        let no_comm = c.g[V0][0] == 0;
        frame.restore_ra(c, 0);
        if !no_comm {
            c.cycles += 3;
            callee::GAME_COMM_DESTROY.call(c, 0x0025_b6cc)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `void cSetupMan_CancelCharacter(cSetupMan* m)` (0x265140): send the
/// character message that cancels the pending selection: 5 (event 4 for a
/// live character, else 5; the state word at +0 moves 4 → 2) is passed with
/// the current state as the last argument. 5 (+3 (+2 (+3) or +1)) + 3 + 3
/// cycles plus the callee.
pub fn csetup_man_cancel_character(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        let self_ = r32(c, A0);
        let state = c.lw(self_);
        c.g[V1][0] = state;
        c.g[V0][0] = 0xc;
        let idle = state == c.g[V0][0];
        frame.save_ra(c, 0);
        if !idle {
            c.cycles += 3;
            let pending = c.lw(p(self_, 0x124));
            c.g[V0][0] = pending;
            let none = pending == 0;
            c.g[V0][0] = 4;
            if none {
                c.cycles += 1;
                c.g[A1][0] = 5;
            } else {
                c.cycles += 2;
                c.g[A1][0] = 4;
                let four = c.g[V0][0];
                if state == four {
                    c.cycles += 3;
                    c.g[V0][0] = 2;
                    c.sw(self_, c.g[V0][0]);
                }
            }
            c.cycles += 3;
            c.g[A3][0] = c.lw(self_);
            c.g[A2][0] = 0xffff_ffff_ffff_ffff;
            callee::SETUP_MAN_SEND_CHARACTER_MSG.call(c, 0x0026_5184)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cSharedLoginMan_RequestCardInfoIfIdle()` (0x2659e8): when the card
/// manager's slot 0x58 says it is busy, return; else request the card info
/// held at `D_004A33DC` through slot 0x28. 10 (+8) + 2 cycles plus the calls.
pub fn cshared_login_man_request_card_info_if_idle(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = c.lw(gp_8(c, 0x2e8));
        frame.save_ra(c, 0);
        let obj = r32(c, A1);
        let vt = c.lw(obj);
        c.g[V1][0] = vt;
        c.g[V0][0] = c.lw(p(vt as u32, 0x5c));
        let m = VSlot::read(c, vt, 0x58);
        c.g[A0][0] = m.this(sx(obj));
        call_ptr(c, m.func as u32, 0x0026_5a08)?;
        let busy = c.g[V0][0] != 0;
        frame.restore_ra(c, 0);
        if busy {
            c.cycles += 2;
            frame.close(c);
            return Ok(());
        }
        c.cycles += 8;
        let obj2 = c.lw(gp_8(c, 0x2e8)) as u32;
        c.g[A2][0] = c.lw(gp_8(c, 0x2e8));
        c.g[A1][0] = c.lw(gp_8(c, 0x2ec));
        let vt2 = c.lw(obj2);
        c.g[V0][0] = vt2;
        c.g[A0][0] = Addr(p(vt2 as u32, 0x28)).lh(c);
        c.g[V1][0] = c.lw(p(vt2 as u32, 0x2c));
        c.g[A0][0] = sx(obj2.wrapping_add(r32(c, A0)));
        call_ptr(c, r32(c, V1), 0x0026_5a2c)?;
        frame.restore_ra(c, 0);
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Network events.

/// `NetEvent* NetEvent_Clone(NetEvent* e)` (0x267e48): allocate 8 bytes from
/// the network heap (tagged with `D_00480488`) and copy the event's first
/// word and the class vtable at `D_004812B0`. 20 cycles plus the allocation.
pub fn net_event_clone(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = sx(0x0048_0488);
        frame.save(c, 16, 0x10);
        let self_ = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = 8;
        c.g[A2][0] = 0x2000_0000;
        c.g[A3][0] = 0;
        call_ptr(c, callee::EA_MEM_ALLOC_NAMED_2, 0x0026_7e70)?;
        c.g[A1][0] = c.lw(self_);
        c.g[V1][0] = sx(0x0048_12b0);
        c.g[A0][0] = c.g[V0][0];
        frame.restore(c, 16, 0x10);
        frame.restore_ra(c, 0);
        c.sw(p(r32(c, A0), 4), c.g[V1][0]);
        c.sw(r32(c, A0), c.g[A1][0]);
        frame.close(c);
        Ok(())
    })
}

/// A `gp_9`-relative global.
#[inline(always)]
pub(crate) fn gp_9(c: &Cpu, off: i32) -> u32 {
    r32(c, GP).wrapping_add(off as u32)
}

/// `lw` (sign-extended).
#[inline(always)]
pub(crate) fn lw(c: &mut Cpu, a: u32) -> u64 {
    c.lw(a)
}

/// `void BuddyMan_Disconnect(self)` (0x261408): clear the presence state and
/// destroy the API connections and queues.
pub fn buddy_man_disconnect(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0;
        frame.save(c, S0, 0x10);
        c.g[A2][0] = 1;
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A3][0] = 0;
        c.g[T0][0] = 0;
        c.g[RA][0] = sx(0x0026_1430);
        c.g[T1][0] = 0;
        call_ptr(c, callee::BUDDY_MAN_SET_PRESENCE_ADDR, 0x0026_1430)?;
        c.g[RA][0] = sx(0x0026_1438);
        c.g[A0][0] = c.g[S0][0];
        callee::BUDDY_MAN_DESTROY_API.call(c, 0x0026_1438)?;
        let this = r32(c, S0);
        Addr(p(this, 0xe8)).sw(c, 0);
        Addr(p(this, 0xd8)).sw(c, 0);
        Addr(p(this, 0xd0)).sw(c, 0);
        Addr(p(this, 0xdc)).sw(c, 0);
        Addr(p(this, 0xe0)).sw(c, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void* BuddyMan_GetRosterEntry(self, idx)` (0x2625c8): 0 for an invalid
/// index, otherwise the buddy at that roster position.
pub fn buddy_man_get_roster_entry(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        let negative = (c.g[S0][0] as i64) < 0;
        c.g[S1][0] = c.g[A0][0];
        if negative {
            c.cycles += 2;
            c.g[V0][0] = 0;
        } else {
            c.cycles += 5;
            c.g[RA][0] = sx(0x0026_25ec);
            callee::BUDDY_MAN_GET_ROSTER_COUNT.call(c, 0x0026_25ec)?;
            c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[S0][0] as i64)) as u64;
            if c.g[V0][0] == 0 {
                c.g[A0][0] = Addr(r32(c, S1)).off(0xcc).lw(c);
                c.cycles += 2;
                c.g[RA][0] = sx(0x0026_2608);
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, callee::LOBBY_LIST_GET_AT_3, 0x0026_2608)?;
            } else {
                c.cycles += 2;
                c.g[V0][0] = 0;
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

/// `int cSharedLoginMan_IsCardUnformatted(void)` (0x265cb0): true when the
/// global card object is formatted but not write-protected.
pub fn c_shared_login_man_is_card_unformatted(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = lw(c, gp_9(c, 0x2e8));
        frame.save_ra(c, 0);
        c.g[V1][0] = c.lw(r32(c, A1));
        c.g[A0][0] = Addr(r32(c, V1)).off(0x1a8).lh(c);
        c.g[V0][0] = Addr(r32(c, V1)).off(0x1ac).lw(c);
        let target = r32(c, V0);
        c.g[RA][0] = sx(0x0026_5cd0);
        c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
        call_ptr(c, target, 0x0026_5cd0)?;
        let formatted = c.g[V0][0] != 0;
        c.g[A1][0] = lw(c, gp_9(c, 0x2e8));
        if !formatted {
            c.cycles += 2;
            c.g[V0][0] = 0;
        } else {
            c.cycles += 6;
            c.g[V1][0] = c.lw(r32(c, A1));
            c.g[A0][0] = Addr(r32(c, V1)).off(0x1b8).lh(c);
            c.g[V0][0] = Addr(r32(c, V1)).off(0x1bc).lw(c);
            let target = r32(c, V0);
            c.g[RA][0] = sx(0x0026_5cf4);
            c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
            call_ptr(c, target, 0x0026_5cf4)?;
            c.g[V0][0] = c.g[V0][0] ^ 1;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cSharedLoginMan_McListDir(code, arg)` (0x265fb8): query the port
/// slot, clear the error buffer and start listing the directory.
pub fn c_shared_login_man_mc_list_dir(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x10);
        c.g[A0][0] = c.g[A1][0];
        c.g[RA][0] = sx(0x0026_5fd4);
        c.g[A1][0] = sx(c.g[SP][0] as u32);
        call_ptr(c, callee::SLOGIN_MC_GET_PORT_SLOT_2, 0x0026_5fd4)?;
        c.g[A0][0] = 0x53_0000;
        c.g[A2][0] = 0x40;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x7190));
        c.g[RA][0] = sx(0x0026_5fe8);
        c.g[A1][0] = 0;
        call_ptr(c, callee::EA_MEMSET, 0x0026_5fe8)?;
        c.g[A0][0] = lw(c, gp_9(c, 0x2e8));
        c.g[RA][0] = sx(0x0026_5ff4);
        c.g[A1][0] = c.g[S0][0];
        call_ptr(c, callee::C_PS2_MEM_CARD_REQUEST_LIST_DIR_ALL, 0x0026_5ff4)?;
        c.g[V0][0] = 2;
        frame.restore(c, S0, 0x20);
        c.g[RA][0] = c.ld(p(c.g[SP][0] as u32, 0x10));
        Addr(gp_9(c, 0x2e4)).sw(c, c.g[V0][0]);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx(p(c.g[SP][0] as u32, 0x30));
        c.pc = t;
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Net-event destructors, replay and script engine.

/// The body shared by the three `NetEvent*_dtor`s: destroy the string at
/// `string_off` and free `self` when bit 0 of `flags` is set.
pub(crate) fn net_event_dtor_body(
    c: &mut Cpu,
    string_off: u32,
    ret_string: u32,
    ret_free: u32,
) -> Flow {
    c.cycles += 14;
    let frame = Frame::open(c, 0x30);
    frame.save(c, S0, 0x20);
    frame.save(c, S1, 0x10);
    c.g[S0][0] = c.g[A1][0];
    c.g[S1][0] = c.g[A0][0];
    frame.save_ra(c, 0);
    c.g[A0][0] = sx(r32(c, S1).wrapping_add(string_off));
    c.g[RA][0] = sx(ret_string);
    c.g[A1][0] = 2;
    call_ptr(c, callee::C_BX_STRING_DTOR, ret_string)?;
    c.g[V0][0] = 0x48_0000;
    c.g[S0][0] = c.g[S0][0] & 1;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x12b0));
    let free = c.g[S0][0] == 0;
    Addr(p(r32(c, S1), 4)).sw(c, c.g[V0][0]);
    if !free {
        c.cycles += 2;
        c.g[RA][0] = sx(ret_free);
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, callee::BUILTIN_DELETE, ret_free)?;
    }
    c.cycles += 5;
    frame.restore(c, S0, 0x20);
    frame.restore(c, S1, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `void NetEventE2_dtor(self, flags)` (0x268158).
pub fn net_event_e2_dtor(c: &mut Cpu) {
    guest(c, |c| net_event_dtor_body(c, 8, 0x0026_817c, 0x0026_8198))
}

/// `void NetEvent102Down_dtor(self, flags)` (0x268398).
pub fn net_event102_down_dtor(c: &mut Cpu) {
    guest(c, |c| net_event_dtor_body(c, 0xc, 0x0026_83bc, 0x0026_83d8))
}

/// `void NetEventE5_dtor(self, flags)` (0x268588).
pub fn net_event_e5_dtor(c: &mut Cpu) {
    guest(c, |c| net_event_dtor_body(c, 8, 0x0026_85ac, 0x0026_85c8))
}

// ---------------------------------------------------------------------------
// Shared login / voip.

/// `void cSharedLoginMan_DoLoad(cSharedLoginMan* m)` (0x265d98): clear the
/// pending request, reset the shared buffer and mark the manager loaded.
pub fn cshared_login_man_do_load(c: &mut Cpu) {
    c.cycles += 22;
    let frame = Frame::open(c, 0x30);
    c.g[A1][0] = 0;
    frame.save(c, S0, 0x20);
    c.g[A2][0] = 0x24;
    frame.save(c, S1, 0x10);
    c.g[S0][0] = sx(0x0053_6760);
    frame.save_ra(c, 0);
    c.g[S1][0] = c.g[A0][0];
    c.sw(p_3(c, S1, 4), 0);
    c.g[A0][0] = c.g[S0][0];
    c.sw(gp_10(c, 0x2f0), 0);
    if call_ptr(c, 0x003e_6448, 0x0026_5dcc).is_err() {
        return;
    }
    c.g[A0][0] = c.g[S0][0];
    if call_ptr(c, 0x003f_8000, 0x0026_5dd4).is_err() {
        return;
    }
    c.g[V1][0] = 3;
    frame.restore(c, S0, 0x20);
    c.sw(p_3(c, S1, 0), c.g[V1][0]);
    frame.restore(c, S1, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
}

/// `void cSharedLoginMan_McDeletePath(void* name, int code)` (0x266240):
/// format the memory-card error, copy the path into a local buffer, show the
/// error and set the login state to 0xd.
pub fn cshared_login_man_mc_delete_path(c: &mut Cpu) {
    c.cycles += 22;
    let frame = Frame::open(c, 0x90);
    frame.save(c, S1, 0x70);
    c.g[S1][0] = c.g[A0][0];
    frame.save(c, S0, 0x80);
    c.g[A0][0] = c.g[A1][0];
    frame.save_ra(c, 0x60);
    c.g[A1][0] = c.g[SP][0];
    if call_ptr(c, 0x003f_a1d0, 0x0026_6260).is_err() {
        return;
    }
    c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
    c.g[A1][0] = c.g[S1][0];
    c.g[A0][0] = c.g[S0][0];
    if call_ptr(c, 0x0041_66f4, 0x0026_6270).is_err() {
        return;
    }
    c.g[A0][0] = Addr(gp_10(c, 0x2e8)).lw(c);
    c.g[A1][0] = c.g[S0][0];
    if call_ptr(c, 0x002c_6ce0, 0x0026_627c).is_err() {
        return;
    }
    c.g[V0][0] = 0xd;
    frame.restore(c, S0, 0x80);
    frame.restore(c, S1, 0x70);
    c.g[RA][0] = Addr(p_3(c, SP, 0x60)).ld(c);
    c.sw(gp_10(c, 0x2e4), c.g[V0][0]);
    let ra = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x90));
    c.pc = ra;
}

/// `void cVoipMan_Activate(cVoipMan* v)` (0x266e30): start the voice service
/// once it is configured and at least 10 units of time have passed.
pub fn cvoip_man_activate(c: &mut Cpu) {
    c.cycles += 7;
    let frame = Frame::open(c, 0x20);
    frame.save(c, S0, 0x10);
    frame.save_ra(c, 0);
    c.g[S0][0] = c.g[A0][0];
    c.g[V0][0] = Addr(p_3(c, S0, 0)).lw(c);
    let mut started = false;
    if c.g[V0][0] != 0 {
        c.cycles += 3;
        c.g[V0][0] = Addr(p_3(c, S0, 4)).lw(c);
        if c.g[V0][0] == 0 {
            c.cycles += 4;
            c.g[V0][0] = Addr(gp_10(c, 0x308)).lw(c);
            c.g[V0][0] = ((c.g[V0][0] as i64) < 10) as u64;
            if c.g[V0][0] == 0 {
                c.cycles += 5;
                if call_ptr(c, 0x003f_6b20, 0x0026_6e70).is_err() {
                    return;
                }
                c.g[V0][0] = 1;
                c.sw(p_3(c, S0, 4), c.g[V0][0]);
                started = true;
            }
        }
    }
    let _ = started;
    frame.restore(c, S0, 0x10);
    c.cycles += 3;
    frame.restore_ra(c, 0);
    frame.close(c);
}

/// `void LobbyMan_MoveToRoomIndex(LobbyMan* self, int index, int a2)`
/// (0x25d800): move to the room with that index (or the default room name
/// when the index is -1).
pub fn lobby_man_move_to_room_index(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0xffff_ffff_ffff_ffff;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        let default = c.g[A1][0] == c.g[V0][0];
        c.g[S1][0] = c.g[A2][0];
        if !default {
            c.cycles += 4;
            callee::LOBBY_MAN_GET_ROOM_NAME.call(c, 0x0025_d828)?;
            c.g[A0][0] = c.g[S0][0];
        } else {
            c.cycles += 3;
            c.g[V0][0] = 0x004a_0000;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2f40));
            c.g[A0][0] = c.g[S0][0];
        }
        c.cycles += 8;
        c.g[A2][0] = c.g[S1][0];
        c.g[A1][0] = c.g[V0][0];
        callee::LOBBY_MAN_MOVE_TO_ROOM.call(c, 0x0025_d848)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void LobbyMan_PingUser(LobbyMan* self, int key, void* def)` (0x25dd08):
/// queue a ping message for the peer address of `key`.
pub fn lobby_man_ping_user(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = lw_2(c, p_4(c.g[S0][0], 0));
        let no_net = c.g[V0][0] == 0;
        c.g[S1][0] = c.g[A1][0];
        if !no_net {
            c.cycles += 10;
            callee::LOBBY_MAN_LOOKUP_PEER_ADDR.call(c, 0x0025_dd30)?;
            c.g[T0][0] = 0x0027_0000;
            c.g[A0][0] = lw_2(c, p_4(c.g[S0][0], 0x54));
            c.g[A1][0] = c.g[S1][0];
            c.g[A2][0] = c.g[V0][0];
            c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x7278));
            c.g[T1][0] = c.g[S0][0];
            c.g[A3][0] = 0x1388;
            call_ptr(c, 0x003e_8dc0, 0x0025_dd50)?;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void BuddyMan_ClearMessages(BuddyMan* self)` (0x261070): pop every queued
/// message (`self+0xac` begin, `self+0xb0` end) and call its vtable slot 8 with
/// argument 3.
///
/// The original is a `match pc` dispatcher whose loop head (0x261088) is also
/// a resume entry after an event unwind; the loop state is in the registers
/// (`s0` = self, `v0` = end pointer, `a2` = the popped item).
pub fn buddy_man_clear_messages(c: &mut Cpu) {
    guest(c, |c| {
        let frame;
        // `true`: start at the popped-message body (the resume entry); `false`:
        // start at the loop head.
        let mut body;
        if c.pc == 0x0026_1070 {
            c.cycles += 5;
            frame = Frame::open(c, 0x20);
            frame.save(c, S0, 0x10);
            frame.save_ra(c, 0);
            c.g[S0][0] = c.g[A0][0];
            body = false;
        } else if c.pc == 0x0026_1088 {
            frame = Frame { size: 0x20 };
            body = true;
        } else {
            let pc = c.pc;
            c.jump_out(pc, 0x0026_1070, 0x0026_10cc);
            return Ok(());
        }
        loop {
            if !body {
                c.cycles += 4;
                c.g[V1][0] = lw_2(c, p_4(c.g[S0][0], 0xac));
                c.g[V0][0] = lw_2(c, p_4(c.g[S0][0], 0xb0));
                if c.g[V1][0] == c.g[V0][0] {
                    break;
                }
                c.g[A2][0] = lw_2(c, (c.g[V0][0] as u32).wrapping_sub(4));
                if back_edge_unwinds(c, 0x0026_1088) {
                    return Ok(());
                }
            }
            body = false;
            c.cycles += 3;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(4));
            let none = c.g[A2][0] == 0;
            sw_2(c, p_4(c.g[S0][0], 0xb0), c.g[V0][0]);
            if !none {
                c.cycles += 6;
                c.g[V0][0] = lw_2(c, p_4(c.g[A2][0], 0xc));
                c.g[A1][0] = 3;
                c.g[A0][0] = lh(c, p_4(c.g[V0][0], 8));
                c.g[V1][0] = lw_2(c, p_4(c.g[V0][0], 0xc));
                let target = c.g[V1][0] as u32;
                c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                call_ptr(c, target, 0x0026_10ac)?;
            }
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Net event clone constructors.

/// The three clone constructors: allocate 0xc bytes from pool `+0x488`,
/// store the event code, the class vtable and the copied sequence number.
macro_rules! event_clone {
    ($fname:ident, $doc:expr, $ret:expr, $word:expr, $vt:expr) => {
        #[doc = $doc]
        pub fn $fname(c: &mut Cpu) {
            guest(c, |c| {
                c.cycles += 23;
                let frame = Frame::open(c, 0x20);
                c.g[A1][0] = 0x0048_0000;
                frame.save(c, S0, 0x10);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x488));
                c.g[S0][0] = c.g[A0][0];
                frame.save_ra(c, 0);
                c.g[A0][0] = 0xc;
                c.g[A2][0] = 0x2000_0000;
                c.g[A3][0] = 0;
                call_ptr(c, 0x0031_7d70, $ret)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = lw_2(c, p_4(c.g[S0][0], 8));
                c.g[V0][0] = $word;
                c.g[V1][0] = $vt;
                sw_2(c, (c.g[A0][0] as u32), c.g[V0][0]);
                frame.restore(c, S0, 0x10);
                c.g[V0][0] = c.g[A0][0];
                frame.restore_ra(c, 0);
                sw_2(c, p_4(c.g[A0][0], 4), c.g[V1][0]);
                sw_2(c, p_4(c.g[A0][0], 8), c.g[A1][0]);
                frame.close(c);
                Ok(())
            })
        }
    };
}
pub(crate) use event_clone;

event_clone!(
    net_event102_clone,
    "`NetEvent102_Clone(NetEvent102* self)` (0x267ec8): copy the event code 0x102.",
    0x0026_7ef0,
    0x102,
    0x0048_1290
);

event_clone!(
    net_event102_base_clone,
    "`NetEvent102Base_Clone(NetEvent* self)` (0x267f58): copy the base event 0x102.",
    0x0026_7f80,
    0x102,
    0x0048_1290
);

event_clone!(
    net_error_event_clone,
    "`NetErrorEvent_Clone(NetErrorEvent* self)` (0x267fe8): copy the error event 0x116.",
    0x0026_8010,
    0x116,
    0x0048_1250
);

// ---------------------------------------------------------------------------
// Network and script objects.

/// `void cSharedLoginMan_McCb_State0F(void)` (0x266348): the login state
/// machine jumps to state 0xf by storing it in `D_004A33D4` (`gp+0x2e4`).
pub fn shared_login_man_mc_cb_state_0f(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = 0xf;
    let state = c.g[V0][0];
    Addr(gp(c, 0x2e4)).sw(c, state);
    abi::ret(c);
}

/// `float cDSockChannel_Base_v05(void)` (0x267dd8): the channel's base tick
/// (1/60 s) from the constant at `gp-0x4a5c`.
pub fn dsock_channel_base_v05(c: &mut Cpu) {
    c.cycles += 3;
    c.f[0] = Addr(gp(c, -0x4a5c)).read_fpu(c).0;
    abi::ret(c);
}

/// `int cDSockChannel_IsConnected(cDSockChannel* this)` (0x267e00): whether
/// the channel state at `+0xc` is 2 (connected).
pub fn dsock_channel_is_connected(c: &mut Cpu) {
    c.cycles += 4;
    let state = Addr(abi::arg(c, 0)).off(0xc).lw(c);
    c.g[V0][0] = state ^ 2;
    c.g[V0][0] = (c.g[V0][0] < 1) as u64;
    abi::ret(c);
}

// ---------------------------------------------------------------------------
// Lobby.

/// `void LobbyMan_ReleaseSnapshotLists(LobbyMan* self)` (0x25a850): destroy
/// all 29 snapshot lists hanging off `self+0x1ec`.
pub fn lobby_man_release_snapshot_lists(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S0, 0x30);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1ec));
        frame.save_ra(c, 0);
        c.g[S2][0] = 0x1c;
        loop {
            c.cycles += 5;
            c.g[V0][0] = lw_3(c, c.g[S0][0] as u32);
            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(1));
            c.g[A1][0] = 4;
            let empty = c.g[V0][0] == 0;
            c.g[A2][0] = c.g[V0][0];
            if !empty {
                c.cycles += 3;
                c.g[A0][0] = lw_3(c, p_5(c.g[S1][0], 0x54));
                call_ptr(c, 0x003e_8688, 0x0025_a88c)?;
                Addr(c.g[S0][0] as u32).sw(c, 0);
            }
            c.cycles += 2;
            let more = (c.g[S2][0] as i64) >= 0;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(4));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x0025_a870);
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

/// `void LobbyMan_SendChat(LobbyMan* self, void* value)` (0x25aae0): format the
/// chat field into a 0x200-byte buffer and send it on the lobby socket.
pub fn lobby_man_send_chat(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x220);
        frame.save(c, S0, 0x210);
        frame.save_ra(c, 0x200);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = lw_3(c, c.g[S0][0] as u32);
        let inactive = c.g[V0][0] == 0;
        c.g[A3][0] = c.g[A1][0];
        if !inactive {
            c.cycles += 13;
            c.g[A2][0] = 0x004a_0000;
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x3010));
            c.g[A0][0] = c.g[SP][0];
            c.g[A1][0] = 0x200;
            Addr(c.g[SP][0] as u32).sb(c, 0);
            call_ptr(c, 0x0025_9288, 0x0025_ab14)?;
            c.g[A1][0] = 0x6d65_0000;
            c.g[A0][0] = lw_3(c, p_5(c.g[S0][0], 0x54));
            c.g[A1][0] |= 0x7367;
            c.g[A2][0] = c.g[SP][0];
            c.g[A3][0] = 0;
            c.g[T0][0] = 0;
            call_ptr(c, 0x003e_8b10, 0x0025_ab30)?;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x210);
        frame.restore_ra(c, 0x200);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// Network events.

/// `void ChatEvent_dtor(ChatEvent* self, int flags)` (0x268048): destroy both
/// strings and delete the event when owned.
pub fn chat_event_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0xc));
        c.g[A1][0] = 2;
        call_ptr(c, 0x0031_8260, 0x0026_806c)?;
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(8));
        c.g[A1][0] = 2;
        call_ptr(c, 0x0031_8260, 0x0026_8078)?;
        c.g[V0][0] = 0x0048_0000;
        c.g[S0][0] &= 1;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x12b0));
        let owns = c.g[S0][0] == 0;
        Addr(p_5(c.g[S1][0], 4)).sw(c, c.g[V0][0]);
        if !owns {
            c.cycles += 2;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0031_7e50, 0x0026_8094)?;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void* NetEventE2_Clone(NetEventE2* self)` (0x2681b0): allocate a 0xc-byte
/// copy, set its type/vtable and copy the string.
pub fn net_event_e2_clone(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0x0048_0000;
        frame.save(c, S0, 0x10);
        c.g[A2][0] = 0x2000_0000;
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A3][0] = 0;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x488));
        c.g[A0][0] = 0xc;
        call_ptr(c, 0x0031_7d70, 0x0026_81d8)?;
        c.g[A1][0] = lw_3(c, p_5(c.g[S0][0], 8));
        c.g[V1][0] = 0x0048_0000;
        c.g[S0][0] = c.g[V0][0];
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1210));
        c.g[V0][0] = 0xe2;
        Addr(p_5(c.g[S0][0], 4)).sw(c, c.g[V1][0]);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(8));
        Addr(c.g[S0][0] as u32).sw(c, c.g[V0][0]);
        call_ptr(c, 0x0031_82d8, 0x0026_81fc)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    });
}

/// `void NetEvent102Lock_dtor(NetEvent102Lock* self, int flags)` (0x2682a8):
/// destroy both strings and delete the lock event when owned.
pub fn net_event102_lock_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x10));
        c.g[A1][0] = 2;
        call_ptr(c, 0x0031_8260, 0x0026_82cc)?;
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0xc));
        c.g[A1][0] = 2;
        call_ptr(c, 0x0031_8260, 0x0026_82d8)?;
        c.g[V0][0] = 0x0048_0000;
        c.g[S0][0] &= 1;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x12b0));
        let owns = c.g[S0][0] == 0;
        Addr(p_5(c.g[S1][0], 4)).sw(c, c.g[V0][0]);
        if !owns {
            c.cycles += 2;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0031_7e50, 0x0026_82f4)?;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `void NetEventE3_dtor(NetEventE3* self, int flags)` (0x268460): destroy
/// both strings and delete the event when owned.
pub fn net_event_e3_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A1][0];
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0xc));
        c.g[A1][0] = 2;
        call_ptr(c, 0x0031_8260, 0x0026_8484)?;
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(8));
        c.g[A1][0] = 2;
        call_ptr(c, 0x0031_8260, 0x0026_8490)?;
        c.g[V0][0] = 0x0048_0000;
        c.g[S0][0] &= 1;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x12b0));
        let owns = c.g[S0][0] == 0;
        Addr(p_5(c.g[S1][0], 4)).sw(c, c.g[V0][0]);
        if !owns {
            c.cycles += 2;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0031_7e50, 0x0026_84ac)?;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// Net / lobby / UI.

/// `void LobbyMan_QueryOwnStats(LobbyMan* self)` (0x25cfe0): format the
/// player's stats into a tag string and queue request 0x1b.
pub fn lobbyman_query_own_stats(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 25;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x220));
        c.g[A2][0] = 0x004a_0000;
        c.sq(S0, p(c.g[SP][0] as u32, 0x210));
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x3108));
        c.sd(p(c.g[SP][0] as u32, 0x200), c.g[RA][0]);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] = 0x200;
        c.g[A3][0] = Addr(p(c.g[S0][0] as u32, 0x80)).lw(c);
        c.g[RA][0] = sx(0x0025_d00c);
        c.sb(c.g[SP][0] as u32, 0);
        call_ptr(c, 0x003e_c180, 0x0025_d00c)?;
        c.g[T0][0] = 0x0027_0000;
        c.g[A2][0] = 0x6f6e_0000;
        c.g[AT][0] = 0x41a0_0000;
        c.mtc1(AT, 12);
        c.g[A0][0] = c.g[S0][0];
        c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x7340));
        c.g[A1][0] = 0x1b;
        c.g[A2][0] = c.g[A2][0] | 0x6c6e;
        c.g[A3][0] = c.g[SP][0];
        call_ptr(c, 0x0025_b608, 0x0025_d034)?;
        c.lq(S0, p(c.g[SP][0] as u32, 0x210));
        c.g[RA][0] = c.ld(p(c.g[SP][0] as u32, 0x200));
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x220));
        c.pc = ra;
        Ok(())
    })
}

/// `void* cUsbKeyboard_ctor(UsbKeyboard* self)` (0x2667e8): zero the status
/// word, register the completion callback and clear the status area.
pub fn c_usb_keyboard_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x20);
        c.g[A2][0] = 0x0053_0000;
        frame.save(c, S0, 0x10);
        c.g[A3][0] = 0x0026_0000;
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        Addr(p(c.g[S0][0] as u32, 0x460)).sw(c, 0);
        c.g[A2][0] = sx(0x0053_71d0);
        c.g[A3][0] = sx(0x0026_68a8);
        c.g[A0][0] = 0;
        c.g[A1][0] = 0;
        c.g[T0][0] = 0;
        call_ptr(c, 0x003f_61d0, 0x0026_681c)?;
        let failed = (c.g[V0][0] as i64) < 0;
        c.g[V0][0] = 1;
        if !failed {
            c.cycles += 1;
            Addr(p(c.g[S0][0] as u32, 0x460)).sw(c, c.g[V0][0]);
        }
        c.cycles += 9;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0;
        c.g[A2][0] = 0x460;
        call_ptr(c, 0x003e_6448, 0x0026_6838)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `NetEventFB* NetEventFB_Clone(NetEventFB* self)` (0x268688): allocate a
/// 16-byte copy and duplicate the two words in the source.
pub fn net_event_fb_clone(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 25;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x20));
        c.g[A1][0] = 0x0048_0000;
        c.sq(S0, p(c.g[SP][0] as u32, 0x10));
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x488));
        c.g[S0][0] = c.g[A0][0];
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[A0][0] = 0x10;
        c.g[A2][0] = 0x2000_0000;
        c.g[A3][0] = 0;
        call_ptr(c, 0x0031_7d70, 0x0026_86b0)?;
        c.g[V1][0] = c.g[V0][0];
        c.g[A2][0] = Addr(p(c.g[S0][0] as u32, 0x0c)).lw(c);
        c.g[A1][0] = Addr(p(c.g[S0][0] as u32, 0x08)).lw(c);
        c.g[V0][0] = 0xfb;
        c.g[A0][0] = 0x0048_1150;
        c.sw(c.g[V1][0] as u32, c.g[V0][0]);
        c.lq(S0, p(c.g[SP][0] as u32, 0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        c.g[V0][0] = c.g[V1][0];
        c.sw(p(c.g[V1][0] as u32, 4), c.g[A0][0]);
        c.sw(p(c.g[V1][0] as u32, 8), c.g[A1][0]);
        c.sw(p(c.g[V1][0] as u32, 0xc), c.g[A2][0]);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = ra;
        Ok(())
    })
}

/// `NetEvent102Tooy_Clone(ev)` 0x268240: allocate a 0x10-byte clone, tag it
/// and copy the event kind at `+0xc`.
pub fn net_event102_tooy_clone(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x20);
        let this64 = abi::arg64(c, 0);
        let this = this64 as u32;
        c.g[A1][0] = 0x480000;
        frame.save(c, S0, 0x10);
        c.g[A1][0] = sx(0x0048_0488);
        c.g[S0][0] = this64;
        frame.save_ra(c, 0);
        c.g[A0][0] = 0x10;
        c.g[A2][0] = 0x2000_0000;
        c.g[A3][0] = 0;
        call_ptr(c, 0x0031_7d70, 0x0026_8268)?;
        let clone = c.g[V0][0];
        c.g[V1][0] = clone;
        c.g[A2][0] = lw_at_2(c, this, 0xc);
        c.g[V0][0] = 0x102;
        c.g[A0][0] = 0x746f_0000;
        c.g[A1][0] = 0x480000;
        c.g[A0][0] |= 0x6f79;
        c.g[A1][0] = sx(0x0048_11f0);
        Addr(clone as u32).sw(c, c.g[V0][0]);
        frame.restore(c, S0, 0x10);
        c.g[V0][0] = clone;
        frame.restore_ra(c, 0);
        Addr(p(clone as u32, 8)).sw(c, c.g[A0][0]);
        Addr(p(clone as u32, 4)).sw(c, c.g[A1][0]);
        Addr(p(clone as u32, 0xc)).sw(c, c.g[A2][0]);
        frame.close(c);
        Ok(())
    })
}

/// `cSetupMan_Create(a, b, c)` 0x264030: allocate a 0x12c-byte setup manager,
/// construct it and publish it in the `gp_11 + 0x2dc` singleton.
pub fn c_setup_man_create(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x40);
        let a0 = abi::arg64(c, 0);
        let a1 = abi::arg64(c, 1);
        let a2 = abi::arg64(c, 2);
        c.g[A3][0] = 0;
        frame.save(c, S1, 0x20);
        frame.save(c, S0, 0x30);
        c.g[S1][0] = a1;
        frame.save(c, S2, 0x10);
        c.g[S0][0] = a0;
        c.g[S2][0] = a2;
        c.g[A1][0] = 0x480000;
        frame.save_ra(c, 0);
        c.g[A1][0] = sx(0x0048_0f08);
        c.g[A0][0] = 0x12c;
        c.g[A2][0] = 0x2000_0000;
        call_ptr(c, 0x0031_7d70, 0x0026_4068)?;
        c.g[A1][0] = c.g[S0][0];
        c.g[A2][0] = c.g[S1][0];
        c.g[A3][0] = c.g[S2][0];
        let mem = c.g[V0][0];
        c.g[A0][0] = mem;
        call_ptr(c, 0x0026_40c0, 0x0026_407c)?;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        Addr(gp_11(c, 0x2dc)).sw(c, c.g[V0][0]);
        frame.close(c);
        Ok(())
    })
}

/// `cSharedLoginMan_McMkdir(self, name, code)` 0x266170: fill the memory-card
/// error record, copy the name into a local buffer and ask the MC layer to
/// create the directory; record the pending request kind.
pub fn c_shared_login_man_mc_mkdir(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0xa0);
        frame.save(c, S1, 0x80);
        frame.save(c, S2, 0x70);
        let self_ = abi::arg64(c, 0);
        let name = abi::arg64(c, 1);
        c.g[S1][0] = self_;
        c.g[S2][0] = name;
        frame.save(c, S0, 0x90);
        frame.save_ra(c, 0x60);
        c.g[A1][0] = sp(c) as u64;
        c.g[A0][0] = abi::arg64(c, 2);
        call_ptr(c, 0x003f_a1d0, 0x0026_6198)?;
        let buf = p(sp(c), 0x10);
        c.g[S0][0] = sx(buf);
        c.g[A1][0] = self_;
        c.g[A0][0] = sx(buf);
        call_ptr(c, 0x0041_66f4, 0x0026_61a8)?;
        c.g[A0][0] = Addr(gp_11(c, 0x2e8)).lw(c);
        c.g[A1][0] = c.g[S0][0];
        c.g[A2][0] = c.g[S2][0];
        call_ptr(c, 0x002c_6c50, 0x0026_61b8)?;
        c.g[V0][0] = 0xb;
        frame.restore(c, S0, 0x90);
        frame.restore(c, S1, 0x80);
        frame.restore(c, S2, 0x70);
        frame.restore_ra(c, 0x60);
        Addr(gp_11(c, 0x2e4)).sw(c, c.g[V0][0]);
        frame.close(c);
        Ok(())
    })
}

/// `cSharedLoginMan_McDeleteFile(self, name, code)` 0x2661d8: fill the error
/// record, format the path and ask the MC layer to remove the file.
pub fn c_shared_login_man_mc_delete_file(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x90);
        frame.save(c, S0, 0x80);
        frame.save(c, S1, 0x70);
        let self_ = abi::arg64(c, 0);
        let name = abi::arg64(c, 1);
        c.g[S0][0] = self_;
        c.g[S1][0] = name;
        frame.save_ra(c, 0x60);
        c.g[A1][0] = sp(c) as u64;
        c.g[A0][0] = abi::arg64(c, 2);
        call_ptr(c, 0x003f_a1d0, 0x0026_61fc)?;
        c.g[A2][0] = c.g[S0][0];
        c.g[A1][0] = 0x4a0000;
        c.g[S0][0] = sx(p(sp(c), 0x10));
        c.g[A3][0] = c.g[S1][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = sx(0x004a_33e8);
        call_ptr(c, 0x0041_7828, 0x0026_6218)?;
        c.g[A0][0] = Addr(gp_11(c, 0x2e8)).lw(c);
        c.g[A1][0] = c.g[S0][0];
        call_ptr(c, 0x002c_6ce0, 0x0026_6224)?;
        c.g[V0][0] = 0xc;
        frame.restore(c, S0, 0x80);
        frame.restore(c, S1, 0x70);
        frame.restore_ra(c, 0x60);
        Addr(gp_11(c, 0x2e4)).sw(c, c.g[V0][0]);
        frame.close(c);
        Ok(())
    })
}

/// `LobbyMan_GetRoomUserName(lobby, idx, outFlag)` 0x25f1e0: the indexed room
/// user's name, or the empty string; the flag receives the "in use" bit.
pub fn lobby_man_get_room_user_name(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        let this = abi::arg(c, 0);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(this).lw(c);
        let present = c.g[V0][0] != 0;
        c.g[S0][0] = abi::arg64(c, 2);
        if present {
            c.cycles += 5;
            c.g[A0][0] = lw_at_2(c, this, 0x60);
            call_ptr(c, 0x003e_e560, 0x0025_f208)?;
            let p = c.g[V0][0];
            c.g[V1][0] = p;
            if p == 0 {
                c.g[V0][0] = 0x4a0000;
                c.cycles += 1;
                c.g[V0][0] = sx(0x004a_2ed0);
            } else {
                // The +2 is charged before the out-flag test, so it applies to
                // both non-empty paths.
                c.cycles += 2;
                if c.g[S0][0] == 0 {
                    c.g[V0][0] = sx((p as u32).wrapping_add(8));
                } else {
                    c.cycles += 6;
                    c.g[V0][0] = lw_at_2(c, p as u32, 4);
                    c.g[V0][0] = sx(((r32(c, V0) as i32) >> 21) as u32);
                    c.g[V0][0] &= 1;
                    Addr(r32(c, S0)).sw(c, c.g[V0][0]);
                    c.g[V0][0] = sx((p as u32).wrapping_add(8));
                }
            }
        } else {
            c.cycles += 2;
            c.g[V0][0] = 0;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cSetupMan_SendBragMsg12(self)` 0x265700: send a brag message (kind 12 when
/// a brag is pending, else 13), entering state 7 first when in state 9.
pub fn c_setup_man_send_brag_msg12(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x30);
        let this = abi::arg(c, 0);
        c.g[V1][0] = Addr(this).lw(c);
        c.g[V0][0] = 0xc;
        let state_12 = c.g[V1][0] == c.g[V0][0];
        frame.save_ra(c, 0x20);
        if !state_12 {
            c.cycles += 3;
            c.g[V0][0] = lw_at_2(c, this, 0x124);
            let no_pending = c.g[V0][0] == 0;
            c.g[V0][0] = 9;
            if no_pending {
                c.cycles += 1;
                c.g[A1][0] = 0xd;
            } else {
                c.cycles += 2;
                let not_nine = c.g[V1][0] != c.g[V0][0];
                c.g[A1][0] = 0xc;
                if not_nine {
                    // fall through to the send block with a1 = 12
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = 7;
                    Addr(this).sw(c, c.g[V0][0]);
                }
            }
            c.cycles += 9;
            c.g[V1][0] = Addr(this).lw(c);
            c.g[V0][0] = u64::MAX;
            c.sw(sp(c), c.g[A1][0]);
            c.sw(p(sp(c), 4), c.g[V1][0]);
            c.g[A1][0] = sp(c) as u64;
            c.sw(p(sp(c), 8), c.g[V0][0]);
            c.g[A2][0] = 0x14;
            call_ptr(c, 0x0026_4be8, 0x0026_575c)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Net and buddy manager.

/// `cSetupMan` quit-event hook helpers.

/// `LobbyMan_RequestNews(self, value)` (0x25e0c8): when the lobby manager is
/// connected, format the tag-encoded "news" request and send it.
pub fn lobby_man_request_news(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x120);
        frame.save(c, S0, 0x110);
        frame.save_ra(c, 0x100);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
        let idle = c.g[V0][0] == 0;
        c.g[A2][0] = 0x4a0000;
        if !idle {
            c.cycles += 16;
            c.g[A3][0] = c.g[A1][0];
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x2f58));
            c.g[A0][0] = c.g[SP][0];
            c.g[A1][0] = 0x100;
            Addr(c.g[SP][0] as u32).sb(c, 0);
            call_ptr(c, 0x003e_be40, 0x0025_e0fc)?;
            c.g[T0][0] = 0x270000;
            c.g[A2][0] = 0x6e650000;
            c.g[AT][0] = 0x41a00000;
            c.mtc1(AT, 12);
            c.g[A0][0] = c.g[S0][0];
            c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x7228));
            c.g[A1][0] = 0xe;
            c.g[A2][0] = c.g[A2][0] | 0x7773;
            c.g[A3][0] = c.g[SP][0];
            call_ptr(c, 0x0025_b608, 0x0025_e124)?;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x110);
        frame.restore_ra(c, 0x100);
        frame.close(c);
        Ok(())
    })
}

/// `LobbyMan_ResetAccountInfo(self)` (0x260f10): release the five account
/// strings and reset the timer/counter fields.
pub fn lobby_man_reset_account_info(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 27;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        let base = c.g[S0][0] as u32;
        for (i, off) in [0x260u32, 0x264, 0x268, 0x26c, 0x270]
            .into_iter()
            .enumerate()
        {
            c.g[A0][0] = sx(p(base, off));
            call_ptr(c, 0x0031_80b8, 0x0026_0f28 + 8 * i as u32)?;
        }
        c.g[V0][0] = 1;
        c.g[V1][0] = 0x7c1;
        Addr(p(base, 0x28c)).sw(c, c.g[V0][0]);
        Addr(p(base, 0x290)).sw(c, c.g[V1][0]);
        Addr(p(base, 0x2a0)).sw(c, 0);
        Addr(p(base, 0x298)).sw(c, c.g[V0][0]);
        Addr(p(base, 0x29c)).sw(c, c.g[V0][0]);
        Addr(p(base, 0x288)).sw(c, c.g[V0][0]);
        Addr(p(base, 0x294)).sw(c, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// The shared loop of the two buddy-roster counters: count entries whose
/// `IsTempAt` result equals `want_temp`.
pub(crate) fn buddy_man_count(
    c: &mut Cpu,
    entry: u32,
    at_head: u32,
    at_body: u32,
    at_dead: u32,
    want_temp: bool,
) -> Flow {
    let end = entry + 0x6c;
    let mut stage: u8 = match c.pc {
        pc if pc == entry => 0,
        pc if pc == at_head => 1,
        pc if pc == at_body => 2,
        pc if pc == at_dead => 3,
        pc => {
            c.jump_out(pc, entry, end);
            return Ok(());
        }
    };
    loop {
        match stage {
            0 => {
                c.cycles += 9;
                let frame = Frame::open(c, 0x40);
                frame.save(c, S0, 0x30);
                frame.save(c, S1, 0x20);
                c.g[S0][0] = 0;
                frame.save(c, S2, 0x10);
                c.g[S1][0] = 0;
                frame.save_ra(c, 0);
                c.g[S2][0] = c.g[A0][0];
                stage = 1;
            }
            1 => {
                c.cycles += 5;
                c.g[A0][0] = c.g[S2][0];
                call_ptr(c, 0x0026_2598, at_head + 8)?;
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[A1][0] = c.g[S0][0];
                if more {
                    if back_edge_unwinds(c, at_body) {
                        return Ok(());
                    }
                    stage = 2;
                } else {
                    stage = 8;
                }
            }
            2 => {
                c.cycles += 5;
                c.g[A0][0] = c.g[S2][0];
                call_ptr(c, 0x0026_2400, at_body + 8)?;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                let hit = if want_temp {
                    c.g[V0][0] != 0
                } else {
                    c.g[V0][0] == 0
                };
                if hit {
                    c.g[S1][0] = c.g[V1][0];
                }
                stage = 1;
            }
            3 => {
                c.cycles += 1;
                stage = 2;
            }
            _ => {
                c.cycles += 7;
                c.g[V0][0] = c.g[S1][0];
                let frame = Frame { size: 0x40 };
                frame.restore(c, S0, 0x30);
                frame.restore(c, S1, 0x20);
                frame.restore(c, S2, 0x10);
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
        }
    }
}

/// `int BuddyMan_CountPermanentEntries(BuddyMan* self)` (0x262280): count the
/// roster entries that are not temporary.
pub fn buddy_man_count_permanent_entries(c: &mut Cpu) {
    guest(c, |c| {
        buddy_man_count(c, 0x0026_2280, 0x0026_22bc, 0x0026_22a8, 0x0026_22a4, false)
    })
}

/// `int BuddyMan_CountTempEntries(BuddyMan* self)` (0x2622f0): count the
/// roster entries that are temporary.
pub fn buddy_man_count_temp_entries(c: &mut Cpu) {
    guest(c, |c| {
        buddy_man_count(c, 0x0026_22f0, 0x0026_232c, 0x0026_2318, 0x0026_2314, true)
    })
}

/// `void cSharedLoginMan_Init(cSharedLoginMan* self)` (0x265950): publish the
/// empty tag string, bring up the shared login state, zero the card string
/// and create the memory-card object.
pub fn c_shared_login_man_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 27;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 1;
        frame.save_ra(c, 0);
        c.g[V1][0] = 0x440000;
        Addr(c.g[A0][0] as u32).sw(c, c.g[V0][0]);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x2a18));
        Addr(gp_12(c, 0x2e4)).sw(c, 0);
        Addr(p(c.g[A0][0] as u32, 4)).sw(c, 0);
        Addr(gp_12(c, 0x2a58)).sw(c, c.g[V1][0]);
        c.g[A0][0] = sx(gp_12(c, 0x2a58));
        Addr(gp_12(c, 0x2f0)).sw(c, 0);
        Addr(gp_12(c, 0x2ec)).sw(c, 0);
        Addr(gp_12(c, 0x2f4)).sw(c, 0);
        Addr(gp_12(c, 0x2a5c)).sw(c, 0);
        call_ptr(c, 0x003f_7ee0, 0x0026_5990)?;
        c.g[A0][0] = 0x530000;
        c.g[A1][0] = 0;
        c.g[A2][0] = 0x40;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x7190));
        call_ptr(c, 0x003e_6448, 0x0026_59a4)?;
        call_ptr(c, 0x002c_22c8, 0x0026_59ac)?;
        c.g[RA][0] = Addr(c.g[SP][0] as u32).ld(c);
        Addr(gp_12(c, 0x2e8)).sw(c, c.g[V0][0]);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.pc = ra;
        Ok(())
    })
}

/// `NetEvent102Down_Clone(ev)` (0x2683f0): allocate a 0x10-byte event clone,
/// tag it and copy the kind string at `+0xc`.
pub fn net_event102_down_clone(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 27;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0x480000;
        frame.save(c, S0, 0x10);
        c.g[A2][0] = 0x2000_0000;
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = 0x10;
        c.g[A3][0] = 0;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x488));
        call_ptr(c, 0x0031_7d70, 0x0026_8418)?;
        c.g[A1][0] = Addr(p(c.g[S0][0] as u32, 0xc)).lw(c);
        c.g[A0][0] = 0x480000;
        c.g[S0][0] = c.g[V0][0];
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x11b0));
        c.g[V1][0] = 0x646f0000;
        Addr(p(c.g[S0][0] as u32, 4)).sw(c, c.g[A0][0]);
        c.g[V1][0] = c.g[V1][0] | 0x776e;
        c.g[V0][0] = 0x102;
        Addr(p(c.g[S0][0] as u32, 8)).sw(c, c.g[V1][0]);
        c.g[A0][0] = sx(p(c.g[S0][0] as u32, 0xc));
        Addr(c.g[S0][0] as u32).sw(c, c.g[V0][0]);
        call_ptr(c, 0x0031_82d8, 0x0026_8448)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `NetEventFC_Clone(ev)` (0x268720): allocate a 0x14-byte clone and copy the
/// three payload words.
pub fn net_event_fc_clone(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 27;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0x480000;
        frame.save(c, S0, 0x10);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x488));
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = 0x14;
        c.g[A2][0] = 0x2000_0000;
        c.g[A3][0] = 0;
        call_ptr(c, 0x0031_7d70, 0x0026_8748)?;
        c.g[V1][0] = c.g[V0][0];
        c.g[A3][0] = Addr(p(c.g[S0][0] as u32, 0x10)).lw(c);
        c.g[A2][0] = Addr(p(c.g[S0][0] as u32, 8)).lw(c);
        c.g[V0][0] = 0xfc;
        c.g[A1][0] = Addr(p(c.g[S0][0] as u32, 0xc)).lw(c);
        c.g[A0][0] = 0x480000;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1130));
        Addr(c.g[V1][0] as u32).sw(c, c.g[V0][0]);
        frame.restore(c, S0, 0x10);
        c.g[V0][0] = c.g[V1][0];
        frame.restore_ra(c, 0);
        Addr(p(c.g[V1][0] as u32, 4)).sw(c, c.g[A0][0]);
        Addr(p(c.g[V1][0] as u32, 8)).sw(c, c.g[A2][0]);
        Addr(p(c.g[V1][0] as u32, 0xc)).sw(c, c.g[A1][0]);
        Addr(p(c.g[V1][0] as u32, 0x10)).sw(c, c.g[A3][0]);
        frame.close(c);
        Ok(())
    })
}

/// `BuddyMan_HasMessageFrom` (0x262A38): true when any queued buddy message is
/// from `name` (compared with `strcmp`, 0x4165a8).
pub fn buddy_man_has_message_from(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x30);
        frame.save_ra(c, 0);
        c.g[S0][0] = lw_at_3(c, S1, 0xac);
        c.g[V0][0] = lw_at_3(c, S1, 0xb0);
        let empty = c.g[S0][0] == c.g[V0][0];
        c.g[S2][0] = c.g[A1][0];
        let mut found = false;
        if !empty {
            c.cycles += 2;
            c.g[V0][0] = Addr(r32(c, S0)).lw(c);
            loop {
                c.cycles += 5;
                c.g[A1][0] = c.g[S2][0];
                c.g[A0][0] = Addr(r32(c, V0)).lw(c);
                call_ptr(c, addr::STRCMP, 0x0026_2a74)?;
                let is_from = c.g[V0][0] != 0;
                if !is_from {
                    c.cycles += 2;
                    c.g[V0][0] = 1;
                    found = true;
                    break;
                }
                c.g[V0][0] = lw_at_3(c, S1, 0xb0);
                c.cycles += 3;
                c.g[S0][0] = low(c, S0).wrapping_add(0x4);
                if c.g[S0][0] == c.g[V0][0] {
                    break;
                }
                c.g[V0][0] = Addr(r32(c, S0)).lw(c);
                if event_due(c) {
                    deliver_event_here(c, 0x0026_2a68);
                }
            }
        }
        if !found {
            c.cycles += 1;
            c.g[V0][0] = 0;
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

/// `cSetupMan_ctor` (0x2640C0): the setup manager: stores its two handler
/// callbacks and id, clears its queues and initialises both name buffers.
pub fn c_setup_man_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 29;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 1;
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        sw_at(c, S0, 0x120, c.g[A1][0]);
        c.g[A0][0] = low(c, S0).wrapping_add(0x10);
        sw_at(c, S0, 0x124, c.g[A2][0]);
        c.g[A1][0] = 0;
        sw_at(c, S0, 0x4, 0);
        c.g[A2][0] = 0x80;
        for off in [0x118u32, 0x11c, 0x8, 0xc] {
            sw_at(c, S0, off, 0);
        }
        sw_at(c, S0, 0x128, c.g[A3][0]);
        sw_at(c, S0, 0, c.g[V0][0]);
        call_ptr(c, addr::MEMSET, 0x0026_4108)?;
        sw_at(c, S0, 0x90, 0);
        c.g[A0][0] = low(c, S0).wrapping_add(0x98);
        sw_at(c, S0, 0x94, 0);
        c.g[A1][0] = 0;
        c.g[A2][0] = 0x80;
        call_ptr(c, addr::MEMSET, 0x0026_4120)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `cSetupMan_SendBragMsg10` (0x265688): send the "brag" message, advancing the
/// setup manager's state (5..7 → 9, 8 → 10) and carrying the state in the
/// packet header.
pub fn c_setup_man_send_brag_msg10(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x30);
        frame.save_ra(c, 0x20);
        c.g[V0][0] = lw_at_3(c, A0, 0x124);
        let have = c.g[V0][0] != 0;
        c.g[A1][0] = 0xb;
        // The code the packet carries, and whether self[0] is reloaded last.
        let mut reload = true;
        if have {
            c.cycles += 4;
            c.g[V1][0] = Addr(r32(c, A0)).lw(c);
            c.g[V0][0] = ((c.g[V1][0] as i64) < 5) as u64;
            let below5 = c.g[V0][0] != 0;
            c.g[A1][0] = 0xa;
            if below5 {
                reload = false;
            } else {
                c.cycles += 3;
                c.g[V0][0] = ((c.g[V1][0] as i64) < 8) as u64;
                let below8 = c.g[V0][0] != 0;
                c.g[V0][0] = 9;
                if below8 {
                    c.cycles += 2;
                    c.sw(r32(c, A0), c.g[V0][0]);
                    c.g[A1][0] = 0xa;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = 8;
                    let not8 = c.g[V1][0] != c.g[V0][0];
                    c.g[V0][0] = 0xa;
                    if not8 {
                        reload = false;
                    } else {
                        c.cycles += 2;
                        c.sw(r32(c, A0), c.g[V0][0]);
                        c.g[A1][0] = 0xa;
                    }
                }
            }
        }
        if reload {
            c.cycles += 1;
            c.g[V1][0] = Addr(r32(c, A0)).lw(c);
        }
        c.cycles += 11;
        c.g[V0][0] = 0xffff_ffff_ffff_0000;
        c.sw(r32(c, SP), c.g[A1][0]);
        c.g[V0][0] |= 0xffff;
        sw_at(c, SP, 0x4, c.g[V1][0]);
        c.g[A1][0] = c.g[SP][0];
        sw_at(c, SP, 0x8, c.g[V0][0]);
        c.g[A2][0] = 0x14;
        callee::SEND_PACKET.call(c, 0x0026_56f0)?;
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `func_002685E0` (0x2685e0): `NetEventE5_Clone`. Allocates a 0x10-byte copy
/// with `operator new(0x10, tag=0x480488, 0x20000000, 0)`, sets the event id
/// word to 0xe5, vtable `0x481170` at `+4`, copies the string at `+8` and the
/// payload word at `+0xc`. Signature: `NetEventE5* clone(NetEventE5* self)`.
pub fn net_event_e5_clone(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 30;
        let frame = Frame::open(c, 0x40);
        c.g[A1][0] = 0x48_0000;
        frame.save(c, S2, 0x10);
        c.g[A2][0] = 0x2000_0000;
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[A3][0] = 0;
        frame.save_ra(c, 0);
        c.g[A1][0] = sx(low_2(c, A1).wrapping_add(0x488));
        c.g[RA][0] = sx(0x0026_8610);
        c.g[A0][0] = 0x10;
        // the original reaches `EA_MemAllocNamed` directly, not the
        // `operator new` wrapper at 0x317e30
        call_ptr(c, 0x0031_7d70, 0x0026_8610)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[V1][0] = 0x48_0000;
        c.g[S1][0] = lw_r(c, S2, 0xc);
        c.g[V1][0] = sx(low_2(c, V1).wrapping_add(0x1170));
        c.g[A1][0] = lw_r(c, S2, 0x8);
        c.g[V0][0] = 0xe5;
        sw_r(c, S0, 0x4, c.g[V1][0]);
        c.g[A0][0] = sx(low_2(c, S0).wrapping_add(0x8));
        c.g[RA][0] = sx(0x0026_8638);
        sw_r(c, S0, 0x0, c.g[V0][0]);
        call_ptr(c, 0x0031_82d8, 0x0026_8638)?;
        sw_r(c, S0, 0xc, c.g[S1][0]);
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int cSharedLoginMan_McOpen(cSharedLoginMan* m, void* a, char* buf,
/// int slot)` (0x002660c8): format the slot's name into a stack buffer, then
/// ask the memory card manager to open it; returns 4.
pub fn c_shared_login_man_mc_open(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 32;
        c.g[SP][0] = sx(sp(c).wrapping_sub(0xb0));
        c.sq(S1, sp(c).wrapping_add(0x90));
        c.sq(S2, sp(c).wrapping_add(0x80));
        c.g[S1][0] = c.g[A1][0];
        c.sq(S3, sp(c).wrapping_add(0x70));
        c.g[S2][0] = c.g[A0][0];
        c.g[A1][0] = c.g[SP][0];
        c.sq(S0, sp(c).wrapping_add(0xa0));
        c.sd(sp(c).wrapping_add(0x60), c.g[RA][0]);
        c.g[S3][0] = c.g[A3][0];
        c.g[A0][0] = c.g[A2][0];
        call_ptr(c, 0x003f_a1d0, 0x0026_60f8)?; // SLoginMc_GetPortSlot
        c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[A1][0] = sx(0x004a_33e8);
        c.g[A3][0] = c.g[S1][0];
        c.g[A2][0] = c.g[S2][0];
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x0041_7828, 0x0026_6114)?; // sprintf
        c.g[A0][0] = c.lw(gpa(c, 0x2e8));
        c.g[A1][0] = c.g[S0][0];
        c.g[A2][0] = c.g[S3][0];
        call_ptr(c, 0x002c_6968, 0x0026_6124)?; // cPS2MemCard_RequestOpen
        c.g[V0][0] = 4;
        c.lq(S0, sp(c).wrapping_add(0xa0));
        c.lq(S1, sp(c).wrapping_add(0x90));
        c.lq(S2, sp(c).wrapping_add(0x80));
        c.lq(S3, sp(c).wrapping_add(0x70));
        c.g[RA][0] = c.ld(sp(c).wrapping_add(0x60));
        c.sw(gpa(c, 0x2e4), c.g[V0][0]);
        c.g[SP][0] = sx(sp(c).wrapping_add(0xb0));
        abi::ret(c);
        Ok(())
    })
}

/// `void BuddyMan_Create()` (0x260f80): allocate the buddy manager (0xf0
/// bytes), initialise its fields and install it at `gp+0x238`.
pub fn buddyman_create(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 33;
        let _frame = Frame::open(c, 0x30);
        c.g[A1][0] = 0x480000;
        c.sd(p(sp(c), 0x20), c.g[RA][0]);
        c.g[A0][0] = 0xf0;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xe48));
        c.g[A2][0] = 0x20000000;
        c.g[A3][0] = 0;
        call_ptr(c, callee::EA_MEM_ALLOC_NAMED_3, 0x0026_0fa0)?;
        c.g[A1][0] = lw_at_4(c, gw(c, 0xda0));
        c.g[V1][0] = 0x10000;
        c.g[A0][0] = u64::MAX;
        c.g[V1][0] |= 0x5180;
        c.sw(p(c.g[V0][0] as u32, 0x40), c.g[A0][0]);
        c.sw(p(c.g[V0][0] as u32, 0xa4), c.g[V1][0]);
        c.sw(p(c.g[V0][0] as u32, 0xec), c.g[A1][0]);
        c.sw(gw(c, 0x238), c.g[V0][0]);
        c.g[RA][0] = c.ld(p(sp(c), 0x20));
        c.sw(p(c.g[V0][0] as u32, 0xac), 0);
        c.sw(p(c.g[V0][0] as u32, 0xb0), 0);
        c.sw(p(c.g[V0][0] as u32, 0xb4), 0);
        c.sw(p(c.g[V0][0] as u32, 0xbc), 0);
        c.sw(p(c.g[V0][0] as u32, 0xc0), 0);
        c.sw(p(c.g[V0][0] as u32, 0xc4), 0);
        c.sw(p(c.g[V0][0] as u32, 0xc8), 0);
        c.sw(p(c.g[V0][0] as u32, 0xcc), 0);
        c.sw(p(c.g[V0][0] as u32, 0xd0), 0);
        c.sw(p(c.g[V0][0] as u32, 0xd8), 0);
        c.sw(p(c.g[V0][0] as u32, 0xdc), 0);
        c.sw(p(c.g[V0][0] as u32, 0xe0), 0);
        c.sw(p(c.g[V0][0] as u32, 0xe4), 0);
        c.sw(p(c.g[V0][0] as u32, 0xe8), 0);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x30));
        c.pc = ra;
        Ok(())
    })
}

/// `void LobbyMan_ChalRequestJoin(LobbyMan* self, const char* name)`
/// (0x25fad0): build the challenge tag record and send a join request.
pub fn lobby_man_chal_request_join(c: &mut Cpu) {
    guest(c, |c| {
        lobby_chal_request(
            c,
            0x004a_3250,
            0x18,
            [0x0025_fafc, 0x0025_fb18, 0x0025_fb48],
        )
    });
}

/// `void LobbyMan_ChalRequestHost(LobbyMan* self, const char* name)`
/// (0x25fa48): build the challenge tag record and send a host request.
pub fn lobby_man_chal_request_host(c: &mut Cpu) {
    guest(c, |c| {
        lobby_chal_request(
            c,
            0x004a_3240,
            0x15,
            [0x0025_fa74, 0x0025_fa90, 0x0025_fac0],
        )
    });
}

/// Shared body of the two challenge requests: a 0x100-byte tag buffer with
/// the name and the "chal" field, then `LobbyMan_SendRequest(self, kind,
/// 0x6368616c, buf, 20.0f)`.
pub(crate) fn lobby_chal_request(c: &mut Cpu, tag: u32, kind: u64, rets: [u32; 3]) -> Flow {
    c.cycles += 34;
    let frame = Frame::open(c, 0x120);
    c.g[A3][0] = c.g[A1][0];
    frame.save(c, S0, 0x110);
    c.g[A2][0] = 0x4a_0000;
    c.g[S0][0] = c.g[A0][0];
    frame.save_ra(c, 0x100);
    c.g[A0][0] = c.g[SP][0];
    c.g[A2][0] = sx(0x004a_3108);
    c.sb(r32(c, SP), 0);
    c.g[RA][0] = sx(rets[0]);
    c.g[A1][0] = 0x100;
    call_ptr(c, 0x003e_c180, rets[0])?;
    c.g[A2][0] = 0x4a_0000;
    c.g[A3][0] = 0x4a_0000;
    c.g[A0][0] = c.g[SP][0];
    c.g[A2][0] = sx(0x004a_2f68);
    c.g[A3][0] = sx(tag);
    c.g[RA][0] = sx(rets[1]);
    c.g[A1][0] = 0x100;
    call_ptr(c, 0x003e_c180, rets[1])?;
    c.g[V1][0] = 0x1;
    c.g[T0][0] = 0x27_0000;
    c.g[A2][0] = 0x6368_0000;
    c.g[AT][0] = 0x41a0_0000;
    c.mtc1(1, 12);
    Addr(r32(c, S0)).sw(c, c.g[V1][0]);
    c.g[A0][0] = c.g[S0][0];
    c.g[T0][0] = sx(0x0026_8ea0);
    c.g[A1][0] = kind;
    c.g[A2][0] = c.g[A2][0] | 0x616c;
    c.g[RA][0] = sx(rets[2]);
    c.g[A3][0] = c.g[SP][0];
    callee::LOBBY_MAN_SEND_REQUEST.call(c, rets[2])?;
    frame.restore(c, S0, 0x110);
    frame.restore_ra(c, 0x100);
    frame.close(c);
    Ok(())
}

// ---------------------------------------------------------------------------
// Buddy manager.

/// `void BuddyMan_DeleteMessageAt(BuddyMan* self, int index)` (0x2635a8):
/// remove the message at `index` from the ring (moving the tail) and release
/// it.
pub fn buddy_man_delete_message_at(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x30);
        c.g[A1][0] = sx(r32(c, A1) << 2);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A2][0] = sx(p(r32(c, S0), 0xa8));
        c.g[V0][0] = Addr(p(r32(c, S0), 0xac)).lw(c);
        c.g[V1][0] = Addr(p(r32(c, S0), 0xb0)).lw(c);
        c.g[A0][0] = sx(p(r32(c, V0), r32(c, A1)));
        c.g[V0][0] = sx(p(r32(c, A0), 4));
        let last = c.g[V0][0] == c.g[V1][0];
        c.g[S1][0] = Addr(r32(c, A0)).lw(c);
        if !last {
            c.cycles += 6;
            c.g[A2][0] = Addr(p(r32(c, A2), 8)).lw(c);
            c.g[A1][0] = c.g[V0][0];
            c.g[A2][0] = sx(r32(c, A2).wrapping_sub(r32(c, V0)));
            c.g[A2][0] = sx(((r32(c, A2) as i32) >> 2) as u32);
            c.g[RA][0] = sx(0x0026_35f4);
            c.g[A2][0] = sx(r32(c, A2) << 2);
            call_ptr(c, 0x0041_610c, 0x0026_35f4)?;
        }
        c.cycles += 4;
        c.g[V0][0] = Addr(p(r32(c, S0), 0xb0)).lw(c);
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(4));
        let empty = c.g[S1][0] == 0;
        Addr(p(r32(c, S0), 0xb0)).sw(c, c.g[V0][0]);
        if !empty {
            c.cycles += 6;
            c.g[V0][0] = Addr(p(r32(c, S1), 0xc)).lw(c);
            c.g[A1][0] = 0x3;
            c.g[A0][0] = Addr(p(r32(c, V0), 8)).lh(c);
            c.g[V1][0] = Addr(p(r32(c, V0), 0xc)).lw(c);
            let target = r32(c, V1);
            c.g[RA][0] = sx(0x0026_361c);
            c.g[A0][0] = sx(p(r32(c, S1), r32(c, A0)));
            call_ptr(c, target, 0x0026_361c)?;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `lb` of `base + off`.
#[inline(always)]
pub(crate) fn lb_at(c: &mut Cpu, base: u64, off: u32) -> u64 {
    Addr(at_2(base as u32, off)).lb(c)
}

/// `void LobbyMan_PeekRoom(LobbyMan* self, int room)` (0x25d770): build the
/// room name and send the "peek" lobby request.
pub fn lobby_man_peek_room(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        open(c, 0x130);
        let sp0 = spv(c);
        c.sq(16, at_2(sp0, 0x120));
        c.sq(17, at_2(sp0, 0x110));
        c.g[S0][0] = c.g[A0][0];
        c.sd(at_2(sp0, 0x100), c.g[RA][0]);
        let this = c.g[S0][0];
        c.g[V0][0] = lw_at_5(c, this, 0);
        c.g[V1][0] = 0x4a0000;
        let nolist = c.g[V0][0] == 0;
        if !nolist {
            // The two name sources: the room name (or the empty name).
            c.cycles += 4;
            c.sb(sp0, 0);
            c.g[V0][0] = u64::MAX;
            let unnamed = c.g[A1][0] == c.g[V0][0];
            c.g[S1][0] = sx(at_2(c.g[V1][0] as u32, 0x2f58));
            if !unnamed {
                c.cycles += 4;
                callee::LOBBY_GET_ROOM_NAME.call(c, 0x0025_d7a8)?;
                c.g[A2][0] = c.g[S1][0];
            } else {
                c.cycles += 3;
                c.g[V0][0] = 0x4a0000;
                c.g[V0][0] = sx(at_2(c.g[V0][0] as u32, 0x2f40));
                c.g[A2][0] = c.g[S1][0];
            }
            c.cycles += 11;
            c.g[A3][0] = c.g[V0][0];
            c.g[A0][0] = c.g[SP][0];
            c.g[A1][0] = 0x100;
            call_ptr(c, callee::TAG_FIELD_SET_STRING, 0x0025_d7cc)?;
            c.g[A1][0] = 0x70650000;
            c.g[A0][0] = lw_at_5(c, this, 0x54);
            c.g[A1][0] = c.g[A1][0] | 0x656b;
            c.g[A2][0] = c.g[SP][0];
            c.g[A3][0] = 0;
            c.g[T0][0] = 0;
            call_ptr(c, callee::LOBBY_API_SEND, 0x0025_d7e8)?;
        }
        c.cycles += 5;
        c.lq(16, at_2(sp0, 0x120));
        c.lq(17, at_2(sp0, 0x110));
        c.g[RA][0] = c.ld(at_2(sp0, 0x100));
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx(at_2(sp0, 0x130));
        c.pc = ra;
        Ok(())
    })
}

/// `bool BuddyMan_IsOnlineAt(BuddyMan* self, int index)` (0x263338): the
/// roster entry's online flag (`+0xa7` bit 0) or its online bit (`+0xa8`).
pub fn buddy_man_is_online_at(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        open(c, 0x30);
        let sp0 = spv(c);
        c.sq(16, at_2(sp0, 0x20));
        c.sq(17, at_2(sp0, 0x10));
        c.g[S0][0] = c.g[A1][0];
        c.sd(sp0, c.g[RA][0]);
        let idx = c.g[S0][0];
        c.g[S1][0] = c.g[A0][0];
        let mut v1: u64 = 0;
        let mut to_entry = false;
        if (idx as i64) < 0 {
            c.cycles += 2;
            c.g[V0][0] = 0;
        } else {
            c.cycles += 5;
            callee::BUDDY_GET_ROSTER_COUNT.call(c, 0x0026_335c)?;
            c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[S0][0] as i64)) as u64;
            let bad = c.g[V0][0] != 0;
            c.g[V0][0] = 0;
            if bad {
                // returns 0
            } else {
                c.cycles += 6;
                c.g[A0][0] = c.g[S1][0];
                c.g[A1][0] = c.g[S0][0];
                callee::BUDDY_GET_ROSTER_ENTRY.call(c, 0x0026_3374)?;
                v1 = c.g[V0][0];
                c.g[V1][0] = v1;
                if v1 != 0 {
                    c.g[V0][0] = lb_at(c, v1, 0xa7);
                    to_entry = true;
                } else {
                    c.cycles += 2;
                    c.g[V0][0] = 0;
                }
            }
        }
        if to_entry {
            c.cycles += 3;
            c.g[V0][0] = c.g[V0][0] & 1;
            let bit0 = c.g[V0][0] != 0;
            c.g[A0][0] = 0;
            if bit0 {
                c.cycles += 2;
                c.g[A0][0] = 1;
                c.g[V0][0] = c.g[A0][0];
            } else {
                c.cycles += 5;
                c.g[V0][0] = lw_at_5(c, v1, 0xa8);
                c.g[V1][0] = 0x100000;
                c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
                let zero = c.g[V0][0] == 0;
                c.g[V0][0] = c.g[A0][0];
                if !zero {
                    c.cycles += 2;
                    c.g[A0][0] = 1;
                    c.g[V0][0] = c.g[A0][0];
                }
            }
        }
        c.cycles += 5;
        c.lq(16, at_2(sp0, 0x20));
        c.lq(17, at_2(sp0, 0x10));
        c.g[RA][0] = c.ld(sp0);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx(at_2(sp0, 0x30));
        c.pc = ra;
        Ok(())
    })
}

/// `void cSetupMan_SetLocalCharacter(cSetupMan* self, int character)`
/// (0x264cf0): set the local player's character in the race setup and its
/// profile record.
pub fn c_setup_man_set_local_character(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 35;
        open(c, 0x30);
        let sp0 = spv(c);
        c.sq(17, at_2(sp0, 0x10));
        c.sq(16, at_2(sp0, 0x20));
        c.sd(sp0, c.g[RA][0]);
        c.g[S1][0] = c.g[A1][0];
        call_ptr(c, callee::BE_LIBRARY_GET_INSTANCE, 0x0026_4d08)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 1;
        call_ptr(c, callee::BE_LIBRARY_GET_INTERFACE, 0x0026_4d14)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A2][0] = c.g[S1][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0;
        callee::BE_SET_CHARACTER_BY_SLOT.call(c, 0x0026_4d28)?;
        c.g[V0][0] = lw_at_5(c, c.g[S0][0], 0xc);
        c.g[A0][0] = lh_at(c, c.g[V0][0], 8);
        c.g[V1][0] = lw_at_5(c, c.g[V0][0], 0xc);
        let func = c.g[V1][0] as u32;
        c.g[A0][0] = sx(at_2(c.g[S0][0] as u32, c.g[A0][0] as u32));
        call_ptr(c, func, 0x0026_4d3c)?;
        let g = gpa_2(c, -0x848);
        c.g[V0][0] = lw_at_5(c, g as u64, 0);
        c.g[A1][0] = 7;
        c.g[A0][0] = lw_at_5(c, c.g[V0][0], 0x78);
        call_ptr(c, callee::BE_LIBRARY_GET_INTERFACE, 0x0026_4d4c)?;
        c.g[A0][0] = 0x530000;
        c.sw(at_2(c.g[A0][0] as u32, 0x4e58), c.g[S1][0]);
        c.g[V1][0] = lw_at_5(c, c.g[V0][0], 0xc);
        c.g[A0][0] = lh_at(c, c.g[V1][0], 8);
        c.g[A1][0] = lw_at_5(c, c.g[V1][0], 0xc);
        let func = c.g[A1][0] as u32;
        c.g[A0][0] = sx(at_2(c.g[V0][0] as u32, c.g[A0][0] as u32));
        call_ptr(c, func, 0x0026_4d68)?;
        c.lq(16, at_2(sp0, 0x20));
        c.lq(17, at_2(sp0, 0x10));
        c.g[RA][0] = c.ld(sp0);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx(at_2(sp0, 0x30));
        c.pc = ra;
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Character selection (cSetupMan).

/// `int cSetupMan_SelectCharacter(cSetupMan* self, int character)`
/// (0x264f88): called when the player picks a character in the front end.
///
/// When the setup manager is already switching to `character` (its state is
/// 12) there is nothing to do. Otherwise, if a character change is pending
/// (`self+0x124`), the chosen character becomes the local character and the
/// script is told about it with the current state; if none is pending the
/// script only gets the "player selected" message.
pub fn c_setup_man_select_character(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0xc;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V1][0] = Addr(r32(c, S0)).lw(c);
        c.g[S1][0] = c.g[A1][0];

        let this = r32(c, S0);
        let state = c.g[V1][0];
        if state == c.g[V0][0] {
            // Already on the way to this character.
            c.cycles += 1;
            frame.restore(c, S0, 0x20);
        } else {
            c.cycles += 3;
            let pending = Addr(p(this, 0x124)).lw(c);
            c.g[V0][0] = pending;
            if pending == 0 {
                c.g[V0][0] = 1;
                c.cycles += 5;
                // SendCharacterMsg(self, 1, character, 0).
                c.g[A0][0] = c.g[S0][0];
                c.g[A2][0] = c.g[S1][0];
                c.g[A1][0] = 1;
                c.g[A3][0] = 0;
                callee::SETUP_MAN_SEND_CHARACTER_MSG.call(c, 0x0026_4fe4)?;
                c.cycles += 1;
                frame.restore(c, S0, 0x20);
            } else {
                c.cycles += 2;
                if state == c.g[V0][0] {
                    // The pending character is the one asked for: mark the
                    // setup manager as "switching".
                    c.cycles += 2;
                    c.g[V0][0] = 2;
                    Addr(this).sw(c, c.g[V0][0]);
                } else {
                    c.g[A0][0] = c.g[S0][0];
                }
                c.cycles += 9;
                // SetLocalCharacter(self, character).
                c.g[A1][0] = c.g[S1][0];
                callee::SETUP_MAN_SET_LOCAL_CHARACTER.call(c, 0x0026_4fd0)?;
                // SendCharacterMsg(self, 0, character, self->state).
                c.g[A0][0] = c.g[S0][0];
                c.g[A2][0] = c.g[S1][0];
                c.g[A3][0] = Addr(r32(c, A0)).lw(c);
                c.g[A1][0] = 0;
                callee::SETUP_MAN_SEND_CHARACTER_MSG.call(c, 0x0026_4fe4)?;
                frame.restore(c, S0, 0x20);
            }
        }
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        c.cycles += 4;
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Network lock message.

/// `void* NetEvent102Lock_Clone(NetEvent102Lock* self)` (0x268308): allocates
/// a 0x14-byte copy of the lock message with its two `cBXString`s and returns
/// it.
pub fn net_event102_lock_clone(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 35;
        let frame = Frame::open(c, 0x40);
        c.g[A1][0] = 0x0048_0000;
        frame.save(c, S2, 0x10);
        c.g[A2][0] = 0x2000_0000;
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[A3][0] = 0;
        frame.save_ra(c, 0);
        c.g[A1][0] = sx(r32(c, A1).wrapping_add(0x488));
        c.g[A0][0] = 0x14;
        call_ptr(c, 0x0031_7d70, 0x0026_8338)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[V1][0] = 0x6c6f_0000;
        c.g[A2][0] = 0x0048_0000;
        c.g[S1][0] = Addr(p(r32(c, S2), 0x10)).lw(c);
        c.g[A1][0] = Addr(p(r32(c, S2), 0xc)).lw(c);
        c.g[A2][0] = sx(r32(c, A2).wrapping_add(0x11d0));
        c.g[V1][0] = c.g[V1][0] | 0x636b;
        c.g[V0][0] = 0x102;
        Addr(p(r32(c, S0), 8)).sw(c, c.g[V1][0]);
        c.g[A0][0] = sx(p(r32(c, S0), 0xc));
        Addr(p(r32(c, S0), 4)).sw(c, c.g[A2][0]);
        Addr(r32(c, S0)).sw(c, c.g[V0][0]);
        call_ptr(c, 0x0031_82d8, 0x0026_836c)?;
        c.g[A1][0] = c.g[S1][0];
        c.g[A0][0] = sx(p(r32(c, S0), 0x10));
        call_ptr(c, 0x0031_82d8, 0x0026_8378)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x20);
        frame.restore(c, S0, 0x30);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Buddy and script engine lookups.

/// `int BuddyMan_FindRosterEntry(BuddyMan* self, void* who)` (0x262468): index
/// of the roster entry whose name matches `who`, or -1.
pub fn buddy_man_find_roster_entry(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x50);
        let self_ = abi::arg(c, 0);
        let who = abi::arg64(c, 1);
        frame.save(c, S2, 0x20);
        frame.save(c, S3, 0x10);
        c.g[S2][0] = sx(self_);
        frame.save(c, S0, 0x40);
        c.g[S3][0] = who;
        frame.save(c, S1, 0x30);
        frame.save_ra(c, 0);
        c.g[S0][0] = 0;
        call_ptr(c, callee::BUDDY_MAN_GET_ROSTER_COUNT_2, 0x0026_2490)?;
        c.g[S1][0] = c.g[V0][0];
        let none = (c.g[S1][0] as i64) <= 0;
        c.g[V0][0] = 0xffff_ffff_ffff_ffff;
        if !none {
            c.cycles += 1;
            c.g[A0][0] = c.g[S2][0];
            loop {
                c.cycles += 4;
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, callee::BUDDY_MAN_GET_ROSTER_ENTRY_2, 0x0026_24a8)?;
                let missing = c.g[V0][0] == 0;
                c.g[A0][0] = c.g[V0][0];
                let mut found = false;
                if missing {
                    c.cycles += 1;
                    c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
                } else {
                    c.cycles += 4;
                    c.g[A1][0] = c.g[S3][0];
                    call_ptr(c, callee::STRCMP, 0x0026_24b8)?;
                    if c.g[V0][0] != 0 {
                        c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
                    } else {
                        c.cycles += 2;
                        c.g[V0][0] = c.g[S0][0];
                        found = true;
                    }
                }
                if found {
                    break;
                }
                c.cycles += 3;
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S1][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[A0][0] = c.g[S2][0];
                if !more {
                    c.cycles += 1;
                    c.g[V0][0] = 0xffff_ffff_ffff_ffff;
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0026_24a0);
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
    })
}

/// `void LobbyMan_MoveToRoom(self, room, name)` (0x25d860): build the
/// dirtysock tag list for a "move to room" request and send it.
pub fn lobby_man_move_to_room(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x130);
        frame.save(c, S0, 0x120);
        frame.save(c, S1, 0x110);
        frame.save_ra(c, 0x100);
        c.g[S1][0] = c.g[A0][0];
        let room = c.g[A1][0];
        let name = c.g[A2][0];
        c.g[S0][0] = name;
        if lw_4(c, S1, 0) != 0 {
            c.cycles += 9;
            c.g[A2][0] = 0x4a0000;
            c.g[A3][0] = room;
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x2f58));
            Addr(c.g[SP][0] as u32).sb(c, 0);
            c.g[A0][0] = c.g[SP][0];
            c.g[A1][0] = 0x100;
            call_ptr(c, table_callee::TAG_FIELD_SET_STRING, 0x0025_d89c)?;
            c.g[A2][0] = 0x4a0000;
            if c.g[S0][0] != 0 {
                c.cycles += 5;
                c.g[A3][0] = c.g[S0][0];
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x3040));
                c.g[A0][0] = c.g[SP][0];
                c.g[A1][0] = 0x100;
                call_ptr(c, table_callee::TAG_FIELD_SET_STRING, 0x0025_d8b8)?;
            }
            c.cycles += 10;
            c.g[T0][0] = 0x270000;
            c.g[A2][0] = 0x6d6f0000;
            c.g[AT][0] = 0x41a00000;
            c.mtc1(AT, 12);
            c.g[A0][0] = c.g[S1][0];
            c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x72c8));
            c.g[A1][0] = 0xc;
            c.g[A2][0] = c.g[A2][0] | 0x7665;
            c.g[A3][0] = c.g[SP][0];
            callee::LOBBY_MAN_SEND_REQUEST_2.call(c, 0x0025_d8e0)?;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x120);
        frame.restore(c, S1, 0x110);
        frame.restore_ra(c, 0x100);
        frame.close(c);
        Ok(())
    })
}

/// `void LobbyMan_ReportUser(self, name)` (0x25df00): tag the player's name
/// and the console language and send a "report user" request.
pub fn lobby_man_report_user(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x130);
        frame.save(c, S1, 0x110);
        frame.save(c, S0, 0x120);
        frame.save_ra(c, 0x100);
        c.g[S1][0] = c.g[A0][0];
        let name = c.g[A1][0];
        c.g[A2][0] = 0x4a0000;
        if lw_4(c, S1, 0) != 0 {
            c.cycles += 24;
            c.g[A3][0] = name;
            c.g[A0][0] = c.g[SP][0];
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x3108));
            Addr(c.g[SP][0] as u32).sb(c, 0);
            c.g[A1][0] = 0x100;
            call_ptr(c, table_callee::TAG_FIELD_SET_STRING, 0x0025_df38)?;
            c.g[S0][0] = 0x4a0000;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x2fe0));
            callee::LOBBY_MAN_GET_LANGUAGE.call(c, 0x0025_df44)?;
            c.g[A2][0] = c.g[S0][0];
            c.g[A3][0] = c.g[V0][0];
            c.g[A0][0] = c.g[SP][0];
            c.g[A1][0] = 0x100;
            call_ptr(c, table_callee::TAG_FIELD_SET_STRING, 0x0025_df58)?;
            c.g[T0][0] = 0x270000;
            c.g[A2][0] = 0x72650000;
            c.g[AT][0] = 0x41a00000;
            c.mtc1(AT, 12);
            c.g[A0][0] = c.g[S1][0];
            c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x7250));
            c.g[A1][0] = 0x10;
            c.g[A2][0] = c.g[A2][0] | 0x7074;
            c.g[A3][0] = c.g[SP][0];
            callee::LOBBY_MAN_SEND_REQUEST_2.call(c, 0x0025_df80)?;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x120);
        frame.restore(c, S1, 0x110);
        frame.restore_ra(c, 0x100);
        frame.close(c);
        Ok(())
    })
}

/// `int BuddyMan_IsSameProductAt(BuddyMan* self, int idx)` (0x2633f8): true if
/// roster entry `idx`'s product tag equals this console's product.
pub fn buddy_man_is_same_product_at(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0x10);
        let idx_negative = (c.g[S0][0] as i64) < 0;
        c.g[S1][0] = c.g[A0][0];
        let mut zero = false;
        if idx_negative {
            zero = true;
        } else {
            c.cycles += 5;
            callee::BUDDY_MAN_GET_ROSTER_COUNT_3.call(c, 0x0026_341c)?;
            c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[S0][0] as i64)) as u64;
            let out_of_range = c.g[V0][0] != 0;
            c.g[V0][0] = 0;
            if !out_of_range {
                c.cycles += 5;
                c.g[A1][0] = c.g[S0][0];
                c.g[A0][0] = c.g[S1][0];
                callee::BUDDY_MAN_GET_ROSTER_ENTRY_3.call(c, 0x0026_3434)?;
                c.g[A1][0] = 0x4a0000;
                if c.g[V0][0] == 0 {
                    zero = true;
                } else {
                    c.cycles += 14;
                    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x54));
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x31f8));
                    call_ptr(c, table_callee::TAG_FIELD_FIND, 0x0026_3448)?;
                    c.g[A3][0] = 0x4a0000;
                    c.g[A1][0] = c.g[SP][0];
                    c.g[A2][0] = 0x10;
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x2f40));
                    c.g[A0][0] = c.g[V0][0];
                    call_ptr(c, table_callee::TAG_FIELD_GET_STRING4, 0x0026_3460)?;
                    c.g[A0][0] = lw_4(c, S1, 0xec);
                    c.g[A1][0] = c.g[SP][0];
                    call_ptr(c, table_callee::BUDDY_PRODUCT_COMPARE, 0x0026_346c)?;
                    c.g[V0][0] = (c.g[V0][0] < 1) as u64;
                }
            }
        }
        if zero {
            c.cycles += 1;
            c.g[V0][0] = 0;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `LobbyMan_DestroyLists` (0x25a8b0): destroy the lobby's three API lists
/// through the LobbyApi list destructor, then release the snapshot lists and
/// any pending list object.
pub fn lobby_man_destroy_lists(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = r(c, A0);
        c.g[S0][0] = sx(this);
        c.g[A2][0] = Addr(p(this, 0x5c)).lw(c);
        if c.g[A2][0] != 0 {
            c.cycles += 5;
            c.g[A0][0] = Addr(p(this, 0x54)).lw(c);
            c.g[RA][0] = sx(0x0025_a8d8);
            c.g[A1][0] = 0;
            call_ptr(c, callee::LOBBY_API_DESTROY_LIST, 0x0025_a8d8)?;
            Addr(p(this, 0x5c)).sw(c, 0);
        }
        c.g[A2][0] = Addr(p(this, 0x60)).lw(c);
        c.cycles += 2;
        if c.g[A2][0] != 0 {
            c.cycles += 5;
            c.g[A0][0] = Addr(p(this, 0x54)).lw(c);
            c.g[RA][0] = sx(0x0025_a8f4);
            c.g[A1][0] = 1;
            call_ptr(c, callee::LOBBY_API_DESTROY_LIST, 0x0025_a8f4)?;
            Addr(p(this, 0x60)).sw(c, 0);
        }
        c.g[A2][0] = Addr(p(this, 0x64)).lw(c);
        c.cycles += 2;
        if c.g[A2][0] != 0 {
            c.cycles += 4;
            c.g[A0][0] = Addr(p(this, 0x54)).lw(c);
            c.g[RA][0] = sx(0x0025_a910);
            c.g[A1][0] = 2;
            call_ptr(c, callee::LOBBY_API_DESTROY_LIST, 0x0025_a910)?;
            Addr(p(this, 0x64)).sw(c, 0);
        }
        c.cycles += 5;
        c.g[RA][0] = sx(0x0025_a91c);
        c.g[A0][0] = c.g[S0][0];
        callee::LOBBY_MAN_RELEASE_SNAPSHOT_LISTS.call(c, 0x0025_a91c)?;
        c.g[A0][0] = Addr(p(this, 0x58)).lw(c);
        if c.g[A0][0] != 0 {
            c.cycles += 5;
            c.g[V0][0] = Addr(p(r(c, A0), 0xc)).lw(c);
            let target = r(c, V0);
            c.g[RA][0] = sx(0x0025_a934);
            call_ptr(c, target, 0x0025_a934)?;
            Addr(p(this, 0x58)).sw(c, 0);
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `BuddyMan_DestroyApi` (0x261530): send the outstanding declines, destroy
/// the API object and clear the buddy manager's fields. A dispatcher-shaped
/// loop: it resumes at every block the original can unwind at.
pub fn buddy_man_destroy_api(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc = c.pc;
        loop {
            match pc {
                0x0026_1530 => {
                    c.cycles += 8;
                    let frame = Frame::open(c, 0x30);
                    frame.save(c, S1, 0x10);
                    frame.save(c, S0, 0x20);
                    let this = r(c, A0);
                    c.g[S1][0] = sx(this);
                    frame.save_ra(c, 0);
                    c.g[V0][0] = Addr(p(this, 0xc8)).lw(c);
                    let none = c.g[V0][0] == 0;
                    // The original restores s0 here; the save/restore is a no-op.
                    if none {
                        pc = 0x0026_15b8;
                    } else {
                        pc = 0x0026_1550;
                    }
                }
                0x0026_1550 => {
                    c.cycles += 2;
                    c.g[V1][0] = Addr(p(r(c, S1), 0xbc)).lw(c);
                    pc = 0x0026_1590;
                }
                0x0026_1558 => {
                    c.cycles += 7;
                    let v0 = r(c, V0);
                    c.g[S0][0] = Addr(v0.wrapping_sub(4)).lw(c);
                    c.g[RA][0] = sx(0x0026_1564);
                    c.g[A1][0] = Addr(r(c, S0)).lw(c);
                    callee::LOBBY_MAN_SEND_DECLINE.call(c, 0x0026_1564)?;
                    c.g[V0][0] = Addr(p(r(c, S1), 0xc0)).lw(c);
                    c.g[V0][0] = sx(r(c, V0).wrapping_sub(4));
                    let no_next = c.g[S0][0] == 0;
                    Addr(p(r(c, S1), 0xc0)).sw(c, c.g[V0][0]);
                    if no_next {
                        pc = 0x0026_158c;
                    } else {
                        pc = 0x0026_1574;
                    }
                }
                0x0026_1574 => {
                    c.cycles += 6;
                    c.g[V0][0] = Addr(p(r(c, S0), 0x3c)).lw(c);
                    c.g[A1][0] = 3;
                    c.g[A0][0] = Addr(p(r(c, V0), 8)).lh(c);
                    c.g[V1][0] = Addr(p(r(c, V0), 0xc)).lw(c);
                    let target = r(c, V1);
                    c.g[RA][0] = sx(0x0026_158c);
                    c.g[A0][0] = sx(r(c, S0).wrapping_add(r(c, A0)));
                    call_ptr(c, target, 0x0026_158c)?;
                    pc = 0x0026_158c;
                }
                0x0026_158c => {
                    c.cycles += 1;
                    c.g[V1][0] = Addr(p(r(c, S1), 0xbc)).lw(c);
                    pc = 0x0026_1590;
                }
                0x0026_1590 => {
                    c.cycles += 3;
                    c.g[V0][0] = Addr(p(r(c, S1), 0xc0)).lw(c);
                    let more = c.g[V1][0] != c.g[V0][0];
                    c.g[A0][0] = Addr(gp(c, -0xc8)).lw(c);
                    if more {
                        if back_edge_unwinds(c, 0x0026_1558) {
                            return Ok(());
                        }
                        pc = 0x0026_1558;
                    } else {
                        pc = 0x0026_159c;
                    }
                }
                0x0026_159c => {
                    c.cycles += 7;
                    c.g[RA][0] = sx(0x0026_15a4);
                    c.g[A0][0] = Addr(p(r(c, S1), 0xc8)).lw(c);
                    call_ptr(c, callee::LOBBY_API_FLUSH_ROSTER, 0x0026_15a4)?;
                    c.g[RA][0] = sx(0x0026_15ac);
                    c.g[A0][0] = Addr(p(r(c, S1), 0xc8)).lw(c);
                    call_ptr(c, callee::BUDDY_API_DESTROY, 0x0026_15ac)?;
                    Addr(p(r(c, S1), 0xcc)).sw(c, 0);
                    Addr(p(r(c, S1), 0xc8)).sw(c, 0);
                    c.lq(16, p(r(c, SP), 0x20));
                    pc = 0x0026_15b8;
                }
                0x0026_15b8 => {
                    c.cycles += 4;
                    let sp = r(c, SP);
                    c.lq(17, p(sp, 0x10));
                    c.g[RA][0] = c.ld(sp);
                    let ra = r(c, RA);
                    c.g[SP][0] = sx(sp.wrapping_add(0x30));
                    c.pc = ra;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x0026_1530, 0x0026_15c8);
                    return Ok(());
                }
            }
        }
    })
}

/// `BuddyMan_GetChallengerByName` (0x263270): linear search of the buddy
/// manager's list for the challenger with the given name. Dispatcher-shaped
/// like the original.
pub fn buddy_man_get_challenger_by_name(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc = c.pc;
        loop {
            match pc {
                0x0026_3270 => {
                    c.cycles += 13;
                    let frame = Frame::open(c, 0x50);
                    frame.save(c, S2, 0x20);
                    frame.save(c, S3, 0x10);
                    let this = r(c, A0);
                    c.g[S2][0] = sx(this);
                    frame.save(c, S0, 0x40);
                    frame.save(c, S1, 0x30);
                    frame.save_ra(c, 0);
                    c.g[A0][0] = Addr(p(this, 0xc0)).lw(c);
                    c.g[V1][0] = Addr(p(this, 0xbc)).lw(c);
                    c.g[V0][0] = sx(r(c, A0).wrapping_sub(r(c, V1)));
                    c.g[V0][0] = sra32(c.g[V0][0], 2);
                    let empty = c.g[V0][0] == 0;
                    c.g[S3][0] = c.g[A1][0];
                    if empty {
                        pc = 0x0026_32a4;
                    } else {
                        pc = 0x0026_32b4;
                    }
                }
                0x0026_32a4 => {
                    c.cycles += 2;
                    c.g[V0][0] = 0;
                    pc = 0x0026_32ec;
                }
                0x0026_32ac => {
                    c.cycles += 2;
                    c.g[V0][0] = c.g[S1][0];
                    pc = 0x0026_32ec;
                }
                0x0026_32b4 => {
                    c.cycles += 3;
                    c.g[S0][0] = c.g[V1][0];
                    let end = c.g[S0][0] == c.g[A0][0];
                    c.g[V0][0] = 0;
                    if end {
                        pc = 0x0026_32ec;
                    } else {
                        pc = 0x0026_32c0;
                    }
                }
                0x0026_32c0 => {
                    c.cycles += 2;
                    c.g[S1][0] = Addr(r(c, S0)).lw(c);
                    pc = 0x0026_32c8;
                }
                0x0026_32c8 => {
                    c.cycles += 5;
                    c.g[A1][0] = c.g[S3][0];
                    c.g[RA][0] = sx(0x0026_32d4);
                    c.g[A0][0] = Addr(r(c, S1)).lw(c);
                    call_ptr(c, callee::STRCASECMP, 0x0026_32d4)?;
                    let differ = c.g[V0][0] == 0;
                    c.g[S0][0] = sx(r(c, S0).wrapping_add(4));
                    if differ {
                        if back_edge_unwinds(c, 0x0026_32ac) {
                            return Ok(());
                        }
                        pc = 0x0026_32ac;
                    } else {
                        pc = 0x0026_32dc;
                    }
                }
                0x0026_32dc => {
                    c.cycles += 3;
                    c.g[V0][0] = Addr(p(r(c, S2), 0xc0)).lw(c);
                    if c.g[S0][0] != c.g[V0][0] {
                        c.g[S1][0] = Addr(r(c, S0)).lw(c);
                        if back_edge_unwinds(c, 0x0026_32c8) {
                            return Ok(());
                        }
                        pc = 0x0026_32c8;
                    } else {
                        pc = 0x0026_32e8;
                    }
                }
                0x0026_32e8 => {
                    c.cycles += 1;
                    c.g[V0][0] = 0;
                    pc = 0x0026_32ec;
                }
                0x0026_32ec => {
                    c.cycles += 7;
                    let sp = r(c, SP);
                    c.lq(16, p(sp, 0x40));
                    c.lq(17, p(sp, 0x30));
                    c.lq(18, p(sp, 0x20));
                    c.lq(19, p(sp, 0x10));
                    c.g[RA][0] = c.ld(sp);
                    let ra = r(c, RA);
                    c.g[SP][0] = sx(sp.wrapping_add(0x50));
                    c.pc = ra;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x0026_3270, 0x0026_3308);
                    return Ok(());
                }
            }
        }
    })
}

/// `void LobbyMan_SendPrivateMessage(Lobby*, char* to, char* message)`
/// (0x25fb80): build a 0x100-byte request tag from `to` (field 0x4a3258) and
/// `message` (field 0x4a3010) with the 0x40004000 flags, then send the request
/// (`0x6d657367`) to the lobby name at `lobby+0x54`.
pub fn lobby_man_send_private_message(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 39;
        let frame = Frame::open(c, 0x130);
        c.g[A3][0] = c.g[A1][0];
        frame.save(c, S0, 0x120);
        c.g[A1][0] = 0x100;
        frame.save(c, S1, 0x110);
        c.g[S0][0] = c.g[A2][0];
        c.g[S1][0] = c.g[A0][0];
        c.g[A2][0] = 0x004a_0000;
        frame.save_ra(c, 0x100);
        c.g[A0][0] = c.g[SP][0];
        Addr(c.g[SP][0] as u32).sb(c, 0);
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x3258));
        call_ptr(c, 0x003e_c180, 0x0025_fbb4)?; // TagFieldSetString
        c.g[A2][0] = 0x004a_0000;
        c.g[A0][0] = c.g[SP][0];
        c.g[A3][0] = c.g[S0][0];
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x3010));
        c.g[A1][0] = 0x100;
        call_ptr(c, 0x003e_c180, 0x0025_fbcc)?;
        c.g[A2][0] = 0x004a_0000;
        c.g[A3][0] = 0x4000_4000;
        c.g[A0][0] = c.g[SP][0];
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x3260));
        c.g[A1][0] = 0x100;
        call_ptr(c, 0x003e_bf28, 0x0025_fbe8)?; // TagFieldSetFlags
        c.g[A0][0] = lw(c, p(c.g[S1][0] as u32, 0x54));
        c.g[A3][0] = 0x0027_0000;
        c.g[A1][0] = 0x6d65_0000;
        c.g[T0][0] = c.g[S1][0];
        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x7138));
        c.g[A1][0] |= 0x7367;
        c.g[A2][0] = c.g[SP][0];
        call_ptr(c, 0x003e_8b10, 0x0025_fc08)?; // LobbyApi_Send
        frame.restore(c, S0, 0x120);
        frame.restore(c, S1, 0x110);
        frame.restore_ra(c, 0x100);
        frame.close(c);
        Ok(())
    });
}

/// `int BuddyMan_FindRosterEntryNoCase(BuddyMan* man, char* name)`
/// (0x2624f8): case-insensitive search of the roster list at `man+0xcc`;
/// returns the entry index or -1.
pub fn buddy_man_find_roster_entry_no_case(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S2, 0x20);
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S0, 0x40);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S1, 0x30);
        let no_name = c.g[S2][0] == 0;
        frame.save_ra(c, 0);
        if no_name {
            c.cycles += 2;
            c.g[V0][0] = 0xffff_ffff_ffff_ffff;
        } else {
            c.cycles += 6;
            c.g[A0][0] = c.g[S3][0];
            c.g[S0][0] = 0;
            call_ptr(c, 0x0026_2598, 0x0026_2530)?; // BuddyMan_GetRosterCount
            c.g[S1][0] = c.g[V0][0];
            let empty = (c.g[S1][0] as i64) <= 0;
            c.g[V0][0] = 0xffff_ffff_ffff_ffff;
            if !empty {
                c.cycles += 1;
                c.g[A0][0] = c.g[S3][0];
                loop {
                    c.cycles += 4;
                    c.g[A1][0] = c.g[S0][0];
                    call_ptr(c, 0x0026_25c8, 0x0026_2548)?; // BuddyMan_GetRosterEntry
                    if c.g[V0][0] == 0 {
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                    } else {
                        c.cycles += 5;
                        c.g[A0][0] = lw(c, c.g[S2][0] as u32);
                        c.g[A1][0] = c.g[V0][0];
                        call_ptr(c, 0x0041_aa88, 0x0026_255c)?; // strcasecmp
                        let same = c.g[V0][0] == 0;
                        c.g[V0][0] = c.g[S0][0];
                        if same {
                            break;
                        }
                        c.cycles += 1;
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                    }
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S1][0] as i64)) as u64;
                    let more = c.g[V0][0] != 0;
                    c.g[A0][0] = c.g[S3][0];
                    if more {
                        if event_due(c) {
                            deliver_event_here(c, 0x0026_2540);
                        }
                        continue;
                    }
                    c.cycles += 1;
                    c.g[V0][0] = 0xffff_ffff_ffff_ffff;
                    break;
                }
            }
        }
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `int BuddyMan_FindChallengerIndex(self, name)` (0x262e20): scan the buddy
/// list for the entry whose name matches (strcasecmp), -1 if absent.
pub fn buddy_man_find_challenger_index(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = 0;
        frame.save(c, S2, 0x10);
        let this = abi::arg(c, 0);
        c.g[S1][0] = sx(this);
        frame.save_ra(c, 0);
        c.g[V0][0] = lw_5(c, S1, 0xc0);
        c.g[V1][0] = lw_5(c, S1, 0xbc);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
        c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 2) as u32);
        let empty = c.g[V0][0] == 0;
        c.g[S2][0] = c.g[A1][0];
        let mut out: i64 = -1;
        if !empty {
            c.cycles += 1;
            c.g[V0][0] = lw_5(c, S1, 0xbc);
            'scan: loop {
                c.cycles += 5;
                c.g[V1][0] = sx((c.g[S0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.g[V0][0] = Addr(c.g[V0][0] as u32).lw(c);
                if c.g[V0][0] == 0 {
                    c.g[V0][0] = lw_5(c, S1, 0xc0);
                } else {
                    c.cycles += 5;
                    c.g[A0][0] = Addr(c.g[V0][0] as u32).lw(c);
                    c.g[A1][0] = c.g[S2][0];
                    call_ptr(c, table::STR_CASECMP, 0x0026_2e78)?;
                    if c.g[V0][0] != 0 {
                        c.g[V0][0] = lw_5(c, S1, 0xc0);
                    } else {
                        c.cycles += 2;
                        out = c.g[S0][0] as i64;
                        break 'scan;
                    }
                }
                c.cycles += 7;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                c.g[V1][0] = lw_5(c, S1, 0xbc);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
                c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 2) as u32);
                c.g[V0][0] = (c.g[S0][0] < c.g[V0][0]) as u64;
                if c.g[V0][0] != 0 {
                    c.g[V0][0] = lw_5(c, S1, 0xbc);
                    if event_due(c) {
                        deliver_event_here(c, 0x0026_2e58);
                    }
                    continue 'scan;
                }
                break 'scan;
            }
            if out >= 0 {
                c.g[V0][0] = out as u64;
            }
        }
        if out < 0 {
            c.cycles += 1;
            c.g[V0][0] = u64::MAX;
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

/// `void cVoipMan_Start(self, rpc)` (0x266d58): start the voice RPC, set its
/// parameters, then either deactivate (few failures) or, unless the voip
/// capability bit is clear, activate.
pub fn c_voip_man_start(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let this = abi::arg(c, 0);
        c.g[S0][0] = sx(this);
        c.g[V0][0] = Addr(this).lw(c);
        let running = c.g[V0][0] != 0;
        c.g[A0][0] = c.g[A1][0];
        if !running {
            c.cycles += 12;
            c.g[A1][0] = 0x1770;
            c.g[A2][0] = 0x1770;
            call_ptr(c, table::VOIP_RPC_START, 0x0026_6d80)?;
            c.g[V0][0] = 1;
            c.g[A0][0] = Addr(gp(c, 0x308)).lw(c);
            Addr(this).sw(c, c.g[V0][0]);
            c.g[A1][0] = 0x55;
            call_ptr(c, table::VOIP_RPC_SET_PARAMS, 0x0026_6d94)?;
            c.g[V0][0] = Addr(gp(c, 0x308)).lw(c);
            c.g[V0][0] = ((c.g[V0][0] as i64) < 10) as u64;
            if c.g[V0][0] == 0 {
                c.cycles += 11;
                call_ptr(c, table::BE_GET_INSTANCE, 0x0026_6dbc)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 4;
                call_ptr(c, table::BE_GET_INTERFACE, 0x0026_6dc8)?;
                c.g[V1][0] = 0x0053_0000;
                c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x5610).lw(c);
                c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 18) as u32);
                c.g[V0][0] &= 1;
                if c.g[V0][0] == 0 {
                    frame.restore(c, S0, 0x10);
                } else {
                    c.cycles += 2;
                    c.g[A0][0] = c.g[S0][0];
                    callee::VOIP_MAN_ACTIVATE.call(c, 0x0026_6de8)?;
                    c.cycles += 1;
                    frame.restore(c, S0, 0x10);
                }
                c.cycles += 3;
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
            c.cycles += 4;
            c.g[A0][0] = c.g[S0][0];
            callee::VOIP_MAN_DEACTIVATE.call(c, 0x0026_6dac)?;
            frame.restore(c, S0, 0x10);
            c.cycles += 3;
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        c.cycles += 1;
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void* LobbyMan_GetRankListEntry(self, outIndex, outSlot, outField)`
/// (0x25f388): resolve the current rank-list handle to its tag structure and
/// hand back its two fields.
pub fn lobby_man_get_rank_list_entry(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S1, 0x40);
        frame.save(c, S2, 0x30);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S3, 0x20);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, S0, 0x50);
        frame.save_ra(c, 0x10);
        c.g[V0][0] = Addr(r32(c, S1)).lw(c);
        let active = c.g[V0][0] != 0;
        c.g[S3][0] = c.g[A3][0];
        let mut entry = 0u64;
        if !active {
            c.cycles += 2;
            c.g[V0][0] = 0;
        } else {
            c.cycles += 8;
            c.g[V0][0] = Addr(p(r32(c, S1), 0xf8)).lw(c);
            c.g[V0][0] = sx(r32(c, V0) << 2);
            c.g[V0][0] = sx(p(r32(c, S1), r32(c, V0)));
            c.g[A0][0] = Addr(p(r32(c, V0), 0x1ec)).lw(c);
            call_ptr(c, 0x003e_e560, 0x0025f3d0)?;
            c.g[S0][0] = c.g[V0][0];
            let missing = c.g[S0][0] == 0;
            c.g[A3][0] = sx(0x004a_0000);
            entry = c.g[S0][0];
            if !missing {
                c.cycles += 10;
                c.g[A0][0] = sx(p(r32(c, S0), 0x28));
                c.g[A3][0] = sx(0x004a_2f30);
                c.g[A1][0] = c.g[SP][0];
                c.g[A2][0] = 8;
                call_ptr(c, 0x003e_d678, 0x0025f3f0)?;
                c.g[V1][0] = Addr(p(r32(c, SP), 4)).lw(c);
                Addr(r32(c, S3)).sw(c, c.g[V1][0]);
                c.g[V0][0] = Addr(p(r32(c, S1), 0xf8)).lw(c);
                let slot = c.g[V0][0] != 0;
                c.g[V0][0] = Addr(r32(c, SP)).lw(c);
                if !slot {
                    c.cycles += 1;
                    c.g[V0][0] = Addr(r32(c, S0)).lw(c);
                }
                c.cycles += 1;
                Addr(r32(c, S2)).sw(c, c.g[V0][0]);
            }
            c.cycles += 1;
            c.g[V0][0] = entry;
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `int cSetupMan_PopPacket(self, void* dst)` (0x2649b0): receive pending
/// packets, dequeue the head packet into `dst`, free it and return its size.
pub fn c_setup_man_pop_packet(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S2, 0x10);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A1][0];
        call_ptr(c, 0x0026_4a58, 0x002649d0)?;
        c.g[A1][0] = Addr(p(r32(c, S2), 0xc)).lw(c);
        c.g[V0][0] = Addr(p(r32(c, S2), 8)).lw(c);
        let empty = c.g[A1][0] == c.g[V0][0];
        c.g[V1][0] = sx(0xffff_ffff);
        let mut len = 0u64;
        if !empty {
            c.cycles += 21;
            c.g[V0][0] = sx(r32(c, A1).wrapping_add(0x1f));
            c.g[V1][0] = ((c.g[V1][0] as i64) < (c.g[A1][0] as i64)) as u64;
            c.g[A0][0] = sx(r32(c, A1).wrapping_add(1));
            if c.g[V1][0] != 0 {
                c.g[V0][0] = c.g[A1][0];
            }
            Addr(p(r32(c, S2), 0xc)).sw(c, c.g[A0][0]);
            c.g[V0][0] = sx(((r32(c, V0) as i32) >> 5) as u32);
            c.g[A0][0] = c.g[S0][0];
            c.g[V0][0] = sx(r32(c, V0) << 5);
            c.g[V0][0] = sx(r32(c, A1).wrapping_sub(r32(c, V0)));
            c.g[V0][0] = sx(r32(c, V0) << 2);
            c.g[V0][0] = sx(p(r32(c, S2), r32(c, V0)));
            c.g[S1][0] = Addr(p(r32(c, V0), 0x10)).lw(c);
            c.g[S0][0] = Addr(p(r32(c, S1), 2)).lhu(c);
            c.g[A1][0] = sx(p(r32(c, S1), 4));
            c.g[S0][0] = sx(r32(c, S0).wrapping_sub(4));
            c.g[A2][0] = c.g[S0][0];
            call_ptr(c, 0x003e_6574, 0x00264a2c)?;
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = c.g[S1][0];
            call_ptr(c, 0x0026_4990, 0x00264a38)?;
            c.g[V0][0] = c.g[S0][0];
            len = c.g[S0][0];
        } else {
            c.cycles += 2;
            c.g[V0][0] = 0;
        }
        let _ = len;
        c.cycles += 6;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Lobby manager: create the peer table.

/// `void LobbyMan_CreatePeerTable(LobbyMan* self)` (0x25a650): if the network
/// record exists and is non-empty, create the game-util object and advertise
/// the record on it.
pub fn lobby_man_create_peer_table(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x30);
        'outer: {
            'mid: {
                c.cycles += 8;
                frame.save(c, S1, 0x10);
                frame.save(c, S0, 0x20);
                c.g[S1][0] = c.g[A0][0];
                frame.save_ra(c, 0);
                c.g[V0][0] = lw_6(c, at_4(c.g[S1][0], 0xc));
                let t = c.g[V0][0] == 0;
                frame.restore(c, S0, 0x20);
                if t {
                    break 'mid;
                }
                c.cycles += 3;
                c.g[V0][0] = lw_6(c, at_4(c.g[S1][0], 0xf4));
                if c.g[V0][0] != 0 {
                    frame.restore(c, S1, 0x10);
                    break 'outer;
                }
                'give_up: {
                    c.cycles += 6;
                    c.g[A1][0] = 0x7065_0000;
                    c.g[A0][0] = lw_6(c, at_4(c.g[S1][0], 0x54));
                    c.g[RA][0] = sx(0x0025_a68c);
                    c.g[A1][0] |= 0x7273;
                    call_ptr(c, 0x003e_8a70, 0x0025_a68c)?;
                    let t = c.g[V0][0] == 0;
                    c.g[A1][0] = 0x7065_0000;
                    if t {
                        break 'give_up;
                    }
                    c.cycles += 6;
                    c.g[A0][0] = lw_6(c, at_4(c.g[S1][0], 0x54));
                    c.g[RA][0] = sx(0x0025_a6a0);
                    c.g[A1][0] |= 0x7273;
                    call_ptr(c, 0x003e_8a70, 0x0025_a6a0)?;
                    c.g[V1][0] = Addr(c.g[V0][0] as u32).lb(c);
                    let t = c.g[V1][0] == 0;
                    frame.restore(c, S0, 0x20);
                    if t {
                        break 'mid;
                    }
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x0025_a6b4);
                    call_ptr(c, 0x003e_6e08, 0x0025_a6b4)?;
                    let t = c.g[V0][0] == 0;
                    Addr(at_4(c.g[S1][0], 0xf4)).sw(c, c.g[V0][0]);
                    if t {
                        break 'give_up;
                    }
                    c.cycles += 10;
                    c.g[A0][0] = lw_6(c, at_4(c.g[S1][0], 0x54));
                    c.g[S0][0] = sx(0x0048_0000);
                    c.g[A1][0] = 0x7065_0000;
                    c.g[S0][0] = sx(0x0048_13a0);
                    c.g[RA][0] = sx(0x0025_a6d4);
                    c.g[A1][0] |= 0x7273;
                    call_ptr(c, 0x003e_8a70, 0x0025_a6d4)?;
                    c.g[A0][0] = lw_6(c, at_4(c.g[S1][0], 0xf4));
                    c.g[A1][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x0025_a6e4);
                    c.g[A2][0] = c.g[V0][0];
                    call_ptr(c, 0x003e_7120, 0x0025_a6e4)?;
                    break 'give_up;
                }
                c.cycles += 1;
                frame.restore(c, S0, 0x20);
                break 'mid;
            }
            c.cycles += 1;
            frame.restore(c, S1, 0x10);
            break 'outer;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Lobby manager: list-changed handlers.

/// `void LobbyMan_OnRoomListChanged(self)` (0x2599b0): allocate a 0xe0-byte
/// link node with the room vtable and insert it at_4 the head of the list.
pub fn lobby_man_on_room_list_changed(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x50);
        c.cycles += 29;
        c.g[A1][0] = sx(0x0048_0000);
        frame.save(c, S0, 0x40);
        c.g[A1][0] = sx(0x0048_0488);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x30);
        c.g[A2][0] = 0x2000_0000;
        frame.save_ra(c, 0x20);
        c.g[A0][0] = 8;
        c.g[RA][0] = sx(0x0025_99dc);
        c.g[A3][0] = 0;
        call_ptr(c, 0x0031_7d70, 0x0025_99dc)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[A2][0] = lw_6(c, at_4(c.g[S0][0], 0xf0));
        c.g[V0][0] = sx(0x0048_0000);
        c.g[V1][0] = 0xe0;
        c.g[V0][0] = sx(0x0048_12b0);
        Addr(c.g[S1][0] as u32).sw(c, c.g[V1][0]);
        Addr(at_4(c.g[S1][0], 4)).sw(c, c.g[V0][0]);
        c.g[A1][0] = sx(0x0048_0000);
        Addr(frame.at(c, 0x10)).sw(c, c.g[A2][0]);
        c.g[A1][0] = sx(0x0048_04a8);
        c.g[A0][0] = 0xc;
        c.g[A2][0] = 0x2000_0000;
        c.g[RA][0] = sx(0x0025_9a14);
        c.g[A3][0] = 0;
        call_ptr(c, 0x0031_7d70, 0x0025_9a14)?;
        c.g[A1][0] = c.g[V0][0];
        c.g[V0][0] = sx(at_4(c.g[A1][0], 8));
        if c.g[V0][0] != 0 {
            Addr(at_4(c.g[A1][0], 8)).sw(c, c.g[S1][0]);
        }
        c.cycles += 14;
        c.g[V0][0] = lw_6(c, frame.at(c, 0x10));
        frame.restore(c, S0, 0x40);
        Addr(c.g[A1][0] as u32).sw(c, c.g[V0][0]);
        frame.restore(c, S1, 0x30);
        c.g[V1][0] = lw_6(c, c.g[V0][0] as u32);
        frame.restore_ra(c, 0x20);
        Addr(at_4(c.g[A1][0], 4)).sw(c, c.g[V1][0]);
        c.g[A0][0] = lw_6(c, at_4(c.g[V0][0], 4));
        Addr(c.g[A0][0] as u32).sw(c, c.g[A1][0]);
        c.g[V0][0] = lw_6(c, frame.at(c, 0x10));
        Addr(at_4(c.g[V0][0], 4)).sw(c, c.g[A1][0]);
        Addr(frame.at(c, 0)).sw(c, c.g[A1][0]);
        frame.close(c);
        Ok(())
    })
}

/// `void LobbyMan_OnRankListChanged(self)` (0x259a60): the same with the rank
/// vtable (id 0xe1).
pub fn lobby_man_on_rank_list_changed(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x50);
        c.cycles += 29;
        c.g[A1][0] = sx(0x0048_0000);
        frame.save(c, S0, 0x40);
        c.g[A1][0] = sx(0x0048_0488);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x30);
        c.g[A2][0] = 0x2000_0000;
        frame.save_ra(c, 0x20);
        c.g[A0][0] = 8;
        c.g[RA][0] = sx(0x0025_9a8c);
        c.g[A3][0] = 0;
        call_ptr(c, 0x0031_7d70, 0x0025_9a8c)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[A2][0] = lw_6(c, at_4(c.g[S0][0], 0xf0));
        c.g[V0][0] = sx(0x0048_0000);
        c.g[V1][0] = 0xe1;
        c.g[V0][0] = sx(0x0048_12b0);
        Addr(c.g[S1][0] as u32).sw(c, c.g[V1][0]);
        Addr(at_4(c.g[S1][0], 4)).sw(c, c.g[V0][0]);
        c.g[A1][0] = sx(0x0048_0000);
        Addr(frame.at(c, 0x10)).sw(c, c.g[A2][0]);
        c.g[A1][0] = sx(0x0048_04a8);
        c.g[A0][0] = 0xc;
        c.g[A2][0] = 0x2000_0000;
        c.g[RA][0] = sx(0x0025_9ac4);
        c.g[A3][0] = 0;
        call_ptr(c, 0x0031_7d70, 0x0025_9ac4)?;
        c.g[A1][0] = c.g[V0][0];
        c.g[V0][0] = sx(at_4(c.g[A1][0], 8));
        if c.g[V0][0] != 0 {
            Addr(at_4(c.g[A1][0], 8)).sw(c, c.g[S1][0]);
        }
        c.cycles += 14;
        c.g[V0][0] = lw_6(c, frame.at(c, 0x10));
        frame.restore(c, S0, 0x40);
        Addr(c.g[A1][0] as u32).sw(c, c.g[V0][0]);
        frame.restore(c, S1, 0x30);
        c.g[V1][0] = lw_6(c, c.g[V0][0] as u32);
        frame.restore_ra(c, 0x20);
        Addr(at_4(c.g[A1][0], 4)).sw(c, c.g[V1][0]);
        c.g[A0][0] = lw_6(c, at_4(c.g[V0][0], 4));
        Addr(c.g[A0][0] as u32).sw(c, c.g[A1][0]);
        c.g[V0][0] = lw_6(c, frame.at(c, 0x10));
        Addr(at_4(c.g[V0][0], 4)).sw(c, c.g[A1][0]);
        Addr(frame.at(c, 0)).sw(c, c.g[A1][0]);
        frame.close(c);
        Ok(())
    })
}

/// `void LobbyMan_OnUserListChanged(self)` (0x25d6c0): the same with the user
/// vtable (id 0xdf).
pub fn lobby_man_on_user_list_changed(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x50);
        c.cycles += 29;
        c.g[A1][0] = sx(0x0048_0000);
        frame.save(c, S0, 0x40);
        c.g[A1][0] = sx(0x0048_0488);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x30);
        c.g[A0][0] = 8;
        frame.save_ra(c, 0x20);
        c.g[A2][0] = 0x2000_0000;
        c.g[RA][0] = sx(0x0025_d6ec);
        c.g[A3][0] = 0;
        call_ptr(c, 0x0031_7d70, 0x0025_d6ec)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[A0][0] = lw_6(c, at_4(c.g[S0][0], 0xf0));
        c.g[V0][0] = sx(0x0048_0000);
        c.g[V1][0] = 0xdf;
        c.g[V0][0] = sx(0x0048_12b0);
        Addr(c.g[S1][0] as u32).sw(c, c.g[V1][0]);
        Addr(at_4(c.g[S1][0], 4)).sw(c, c.g[V0][0]);
        c.g[A1][0] = sx(0x0048_0000);
        Addr(frame.at(c, 0x10)).sw(c, c.g[A0][0]);
        c.g[A1][0] = sx(0x0048_04a8);
        c.g[A0][0] = 0xc;
        c.g[A2][0] = 0x2000_0000;
        c.g[RA][0] = sx(0x0025_d724);
        c.g[A3][0] = 0;
        call_ptr(c, 0x0031_7d70, 0x0025_d724)?;
        c.g[A1][0] = c.g[V0][0];
        c.g[V0][0] = sx(at_4(c.g[A1][0], 8));
        if c.g[V0][0] != 0 {
            Addr(at_4(c.g[A1][0], 8)).sw(c, c.g[S1][0]);
        }
        c.cycles += 14;
        c.g[V0][0] = lw_6(c, frame.at(c, 0x10));
        frame.restore(c, S0, 0x40);
        Addr(c.g[A1][0] as u32).sw(c, c.g[V0][0]);
        frame.restore(c, S1, 0x30);
        c.g[V1][0] = lw_6(c, c.g[V0][0] as u32);
        frame.restore_ra(c, 0x20);
        Addr(at_4(c.g[A1][0], 4)).sw(c, c.g[V1][0]);
        c.g[A0][0] = lw_6(c, at_4(c.g[V0][0], 4));
        Addr(c.g[A0][0] as u32).sw(c, c.g[A1][0]);
        c.g[V0][0] = lw_6(c, frame.at(c, 0x10));
        Addr(at_4(c.g[V0][0], 4)).sw(c, c.g[A1][0]);
        Addr(frame.at(c, 0)).sw(c, c.g[A1][0]);
        frame.close(c);
        Ok(())
    })
}

/// `void LobbyMan_GetRoomUserStats(self, index, out)` (0x25f288): look the
/// user up in the room list and fill in its id, level and total score.
pub fn lobby_man_get_room_user_stats(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x70);
        'end: {
            c.cycles += 7;
            frame.save(c, S0, 0x60);
            frame.save(c, S1, 0x50);
            frame.save_ra(c, 0x40);
            c.g[V0][0] = lw_6(c, c.g[A0][0] as u32);
            let t = c.g[V0][0] == 0;
            c.g[S0][0] = c.g[A2][0];
            if t {
                break 'end;
            }
            'lookup: {
                'found: {
                    c.cycles += 5;
                    c.g[RA][0] = sx(0x0025_f2ac);
                    c.g[A0][0] = lw_6(c, at_4(c.g[A0][0], 0x60));
                    call_ptr(c, 0x003e_e560, 0x0025_f2ac)?;
                    c.g[A0][0] = c.g[V0][0];
                    let t = c.g[A0][0] == 0;
                    c.g[V1][0] = sx(at_4(c.g[A0][0], 0x38));
                    if t {
                        break 'found;
                    }
                    c.cycles += 2;
                    let t = c.g[V1][0] == 0;
                    c.g[S1][0] = c.g[V1][0];
                    if t {
                        break 'found;
                    }
                    c.cycles += 6;
                    c.g[V0][0] = lw_6(c, at_4(c.g[A0][0], 0x34));
                    c.g[A0][0] = sx(at_4(c.g[A0][0], 0x28));
                    c.g[RA][0] = sx(0x0025_f2d0);
                    Addr(c.g[S0][0] as u32).sw(c, c.g[V0][0]);
                    callee::ATOI.call(c, 0x0025_f2d0)?;
                    Addr(at_4(c.g[S0][0], 0x10)).sw(c, c.g[V0][0]);
                    break 'lookup;
                }
                c.cycles += 3;
                Addr(c.g[S0][0] as u32).sw(c, 0);
                c.g[S1][0] = 0;
                Addr(at_4(c.g[S0][0], 0x10)).sw(c, 0);
                break 'lookup;
            }
            c.cycles += 15;
            c.g[A3][0] = sx(0x004a_0000);
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = sx(frame.at(c, 0));
            c.g[A3][0] = sx(0x004a_2f30);
            c.g[RA][0] = sx(0x0025_f2fc);
            c.g[A2][0] = 0x40;
            call_ptr(c, 0x003e_d678, 0x0025_f2fc)?;
            c.g[A1][0] = lw_6(c, frame.at(c, 8));
            c.g[V0][0] = lw_6(c, frame.at(c, 0xc));
            c.g[V1][0] = lw_6(c, frame.at(c, 0x10));
            c.g[A0][0] = lw_6(c, frame.at(c, 0x18));
            c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            Addr(at_4(c.g[S0][0], 4)).sw(c, c.g[A1][0]);
            Addr(at_4(c.g[S0][0], 0xc)).sw(c, c.g[A0][0]);
            Addr(at_4(c.g[S0][0], 8)).sw(c, c.g[V0][0]);
            break 'end;
        }
        c.cycles += 5;
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Buddy manager: roster replies.

/// `void BuddyMan_OnRosterAddReply(self)` (0x2618c0): allocate the 0x10a link
/// node for the buddy's roster and insert it.
pub fn buddy_man_on_roster_add_reply(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x50);
        c.cycles += 29;
        c.g[A1][0] = sx(0x0048_0000);
        frame.save(c, S0, 0x40);
        c.g[A1][0] = sx(0x0048_0488);
        frame.save(c, S1, 0x30);
        c.g[A0][0] = 8;
        frame.save_ra(c, 0x20);
        c.g[A2][0] = 0x2000_0000;
        c.g[S0][0] = lw_6(c, gp_13(c, -0xc8));
        c.g[RA][0] = sx(0x0026_18ec);
        c.g[A3][0] = 0;
        call_ptr(c, 0x0031_7d70, 0x0026_18ec)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[V1][0] = 0x10a;
        c.g[V0][0] = sx(0x0048_0000);
        Addr(c.g[S1][0] as u32).sw(c, c.g[V1][0]);
        c.g[V0][0] = sx(0x0048_12b0);
        c.g[A1][0] = sx(0x0048_0000);
        Addr(at_4(c.g[S1][0], 4)).sw(c, c.g[V0][0]);
        c.g[A1][0] = sx(0x0048_04a8);
        c.g[A0][0] = 0xc;
        c.g[A2][0] = 0x2000_0000;
        c.g[V0][0] = lw_6(c, at_4(c.g[S0][0], 0xf0));
        c.g[A3][0] = 0;
        c.g[RA][0] = sx(0x0026_1924);
        Addr(frame.at(c, 0x10)).sw(c, c.g[V0][0]);
        call_ptr(c, 0x0031_7d70, 0x0026_1924)?;
        c.g[A1][0] = c.g[V0][0];
        c.g[V0][0] = sx(at_4(c.g[A1][0], 8));
        if c.g[V0][0] != 0 {
            Addr(at_4(c.g[A1][0], 8)).sw(c, c.g[S1][0]);
        }
        c.cycles += 14;
        c.g[V0][0] = lw_6(c, frame.at(c, 0x10));
        frame.restore(c, S0, 0x40);
        Addr(c.g[A1][0] as u32).sw(c, c.g[V0][0]);
        frame.restore(c, S1, 0x30);
        c.g[V1][0] = lw_6(c, c.g[V0][0] as u32);
        frame.restore_ra(c, 0x20);
        Addr(at_4(c.g[A1][0], 4)).sw(c, c.g[V1][0]);
        c.g[A0][0] = lw_6(c, at_4(c.g[V0][0], 4));
        Addr(c.g[A0][0] as u32).sw(c, c.g[A1][0]);
        c.g[V0][0] = lw_6(c, frame.at(c, 0x10));
        Addr(at_4(c.g[V0][0], 4)).sw(c, c.g[A1][0]);
        Addr(frame.at(c, 0)).sw(c, c.g[A1][0]);
        frame.close(c);
        Ok(())
    })
}

/// `void BuddyMan_OnRosterRemoveReply(self)` (0x261970): the same with the
/// remove id 0x10b.
pub fn buddy_man_on_roster_remove_reply(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x50);
        c.cycles += 29;
        c.g[A1][0] = sx(0x0048_0000);
        frame.save(c, S0, 0x40);
        c.g[A1][0] = sx(0x0048_0488);
        frame.save(c, S1, 0x30);
        c.g[A0][0] = 8;
        frame.save_ra(c, 0x20);
        c.g[A2][0] = 0x2000_0000;
        c.g[S0][0] = lw_6(c, gp_13(c, -0xc8));
        c.g[RA][0] = sx(0x0026_199c);
        c.g[A3][0] = 0;
        call_ptr(c, 0x0031_7d70, 0x0026_199c)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[V1][0] = 0x10b;
        c.g[V0][0] = sx(0x0048_0000);
        Addr(c.g[S1][0] as u32).sw(c, c.g[V1][0]);
        c.g[V0][0] = sx(0x0048_12b0);
        c.g[A1][0] = sx(0x0048_0000);
        Addr(at_4(c.g[S1][0], 4)).sw(c, c.g[V0][0]);
        c.g[A1][0] = sx(0x0048_04a8);
        c.g[A0][0] = 0xc;
        c.g[A2][0] = 0x2000_0000;
        c.g[V0][0] = lw_6(c, at_4(c.g[S0][0], 0xf0));
        c.g[A3][0] = 0;
        c.g[RA][0] = sx(0x0026_19d4);
        Addr(frame.at(c, 0x10)).sw(c, c.g[V0][0]);
        call_ptr(c, 0x0031_7d70, 0x0026_19d4)?;
        c.g[A1][0] = c.g[V0][0];
        c.g[V0][0] = sx(at_4(c.g[A1][0], 8));
        if c.g[V0][0] != 0 {
            Addr(at_4(c.g[A1][0], 8)).sw(c, c.g[S1][0]);
        }
        c.cycles += 14;
        c.g[V0][0] = lw_6(c, frame.at(c, 0x10));
        frame.restore(c, S0, 0x40);
        Addr(c.g[A1][0] as u32).sw(c, c.g[V0][0]);
        frame.restore(c, S1, 0x30);
        c.g[V1][0] = lw_6(c, c.g[V0][0] as u32);
        frame.restore_ra(c, 0x20);
        Addr(at_4(c.g[A1][0], 4)).sw(c, c.g[V1][0]);
        c.g[A0][0] = lw_6(c, at_4(c.g[V0][0], 4));
        Addr(c.g[A0][0] as u32).sw(c, c.g[A1][0]);
        c.g[V0][0] = lw_6(c, frame.at(c, 0x10));
        Addr(at_4(c.g[V0][0], 4)).sw(c, c.g[A1][0]);
        Addr(frame.at(c, 0)).sw(c, c.g[A1][0]);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `ChatEvent_Clone` (0x2680a8): deep-copy a chat event, cloning its two
// strings and returning the new object.
pub fn chat_event_clone(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 44;
        let frame = Frame::open(c, 0x60);
        c.g[A1][0] = 0x480000;
        frame.save(c, S2, 0x30);
        c.g[A2][0] = 0x20000000;
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0x50);
        frame.save(c, S1, 0x40);
        c.g[A3][0] = 0u64;
        frame.save_ra(c, 0x20);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x498));
        c.g[A0][0] = 0x10;
        call_ptr(c, 0x00317d70, 0x0026_80d8)?;
        c.g[A0][0] = c.g[SP][0];
        c.g[S0][0] = c.g[V0][0];
        c.g[A1][0] = sx((c.g[S2][0] as u32).wrapping_add(0x8));
        call_ptr(c, 0x00317fe8, 0x0026_80e8)?;
        c.g[S1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[A1][0] = sx((c.g[S2][0] as u32).wrapping_add(0xc));
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, 0x00317fe8, 0x0026_80f8)?;
        c.g[V1][0] = 0x480000;
        c.g[V0][0] = 0xde;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1230));
        c.g[A1][0] = c.g[SP][0];
        c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[V1][0]);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x8));
        c.sw((c.g[S0][0] as u32), c.g[V0][0]);
        call_ptr(c, 0x00317fe8, 0x0026_8118)?;
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xc));
        c.g[A1][0] = c.g[S1][0];
        call_ptr(c, 0x00317fe8, 0x0026_8124)?;
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] = 0x2;
        call_ptr(c, 0x00318260, 0x0026_8130)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = 0x2;
        call_ptr(c, 0x00318260, 0x0026_813c)?;
        c.g[V0][0] = c.g[S0][0];
        c.lq(17, frame.at(c, 0x40));
        c.lq(16, frame.at(c, 0x50));
        c.lq(18, frame.at(c, 0x30));
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `void LobbyMan_StartQuickPlay(self, request, int mode)` (0x25ee20): when
/// the lobby manager is idle, build the quick-play request tags and send it.
pub fn lobby_man_start_quick_play(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x130);
        frame.save(c, S0, 0x120);
        frame.save(c, S1, 0x110);
        c.g[V1][0] = c.g[A2][0];
        let this = c.g[A0][0];
        c.g[S0][0] = this;
        frame.save_ra(c, 0x100);
        c.g[V0][0] = Addr(this as u32).lw(c);
        let idle = c.g[V0][0] == 0;
        c.g[S1][0] = c.g[A1][0];
        let mut tail = 1;
        if !idle {
            c.cycles += 3;
            c.g[V0][0] = c.g[V0][0] ^ 0x1;
            if c.g[V0][0] != 0 {
                tail = 0;
            } else {
                c.cycles += 28;
                c.sw(p(c.g[S0][0] as u32, 0xa4), c.g[V1][0]);
                c.g[A2][0] = 0x4a0000;
                c.g[A3][0] = 0x480000;
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x3200));
                c.g[A0][0] = c.g[SP][0];
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x8d0));
                c.sb(c.g[SP][0] as u32, 0);
                c.g[A1][0] = 0x100;
                call_ptr(c, 0x003e_c180, 0x0025_ee74)?;
                c.g[A2][0] = 0x4a0000;
                c.g[T1][0] = 0x4a0000;
                c.g[T0][0] = 0x14;
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x3208));
                c.g[A3][0] = c.g[S1][0];
                c.g[A0][0] = c.g[SP][0];
                c.g[A1][0] = 0x100;
                c.g[T1][0] = sx((c.g[T1][0] as u32).wrapping_add(0x2f30));
                call_ptr(c, 0x003e_c3c0, 0x0025_ee98)?;
                c.g[T0][0] = 0x270000;
                c.g[A2][0] = 0x71750000;
                c.g[AT][0] = 0x41a00000;
                c.mtc1(1, 12);
                c.g[A0][0] = c.g[S0][0];
                c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x7188));
                c.g[A1][0] = 0x12;
                c.g[A2][0] = c.g[A2][0] | 0x696b;
                c.g[A3][0] = c.g[SP][0];
                call_ptr(c, 0x0025_b608, 0x0025_eec0)?;
            }
        }
        c.cycles += tail;
        frame.restore(c, S0, 0x120);
        c.cycles += 4;
        frame.restore(c, S1, 0x110);
        frame.restore_ra(c, 0x100);
        frame.close(c);
        Ok(())
    })
}

/// `int BuddyMan_CountMessagesFromAt(BuddyMan* self, int index)` (0x262950):
/// count the roster's messages whose sender address equals the roster entry
/// at `index`.
pub fn buddy_man_count_messages_from_at(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S1, 0x30);
        frame.save(c, S3, 0x10);
        frame.save(c, S0, 0x40);
        frame.save(c, S2, 0x20);
        frame.save_ra(c, 0);
        let this = c.g[A0][0];
        c.g[S1][0] = this;
        call_ptr(c, 0x0026_25c8, 0x0026_2970)?;
        c.g[S3][0] = c.g[V0][0];
        let no_entry = c.g[S3][0] == 0;
        c.g[V0][0] = 0xffff_ffff_ffff_ffff;
        if no_entry {
            c.cycles += 1;
            c.g[V0][0] = c.g[S0][0];
            frame.restore(c, S0, 0x40);
            c.cycles += 7;
            frame.restore(c, S1, 0x30);
            frame.restore(c, S2, 0x20);
            frame.restore(c, S3, 0x10);
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
        c.cycles += 7;
        c.g[V0][0] = Addr(p(c.g[S1][0] as u32, 0xb0)).lw(c);
        c.g[S2][0] = 0;
        c.g[V1][0] = Addr(p(c.g[S1][0] as u32, 0xac)).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub((c.g[V1][0] as u32)));
        c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 2) as u32);
        let none = c.g[V0][0] == 0;
        c.g[S0][0] = 0;
        if !none {
            c.cycles += 2;
            c.g[A0][0] = c.g[S1][0];
            loop {
                c.cycles += 4;
                c.g[A1][0] = c.g[S2][0];
                call_ptr(c, 0x0026_3630, 0x0026_29a8)?;
                if c.g[V0][0] == 0 {
                    c.g[V0][0] = Addr(p(c.g[S1][0] as u32, 0xb0)).lw(c);
                } else {
                    c.cycles += 6;
                    c.g[A0][0] = Addr(c.g[V0][0] as u32).lw(c);
                    c.g[A1][0] = c.g[S3][0];
                    call_ptr(c, 0x0041_65a8, 0x0026_29bc)?;
                    c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    if c.g[V0][0] == 0 {
                        c.g[S0][0] = c.g[V1][0];
                    }
                    c.g[V0][0] = Addr(p(c.g[S1][0] as u32, 0xb0)).lw(c);
                }
                c.cycles += 7;
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                c.g[V1][0] = Addr(p(c.g[S1][0] as u32, 0xac)).lw(c);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub((c.g[V1][0] as u32)));
                c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 2) as u32);
                c.g[V0][0] = (c.g[S2][0] < c.g[V0][0]) as u64;
                let more = c.g[V0][0] != 0;
                c.g[A0][0] = c.g[S1][0];
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0026_29a0);
                }
            }
        }
        c.cycles += 1;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S0, 0x40);
        c.cycles += 7;
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Buddy manager.

/// `int BuddyMan_HasChallenger(BuddyMan* m, const char* name)` (0x263128):
/// walk the challenger list comparing names; returns 1 and bumps the entry's
/// flag on a match, 0 at the end of the list.
pub fn buddy_man_has_challenger(c: &mut Cpu) {
    // Dispatcher-shaped: an event at a back edge can unwind and the trampoline
    // resumes at the loop head, so `pc` selects the stage and the loop state
    // lives in s0-s3 exactly as the generated code keeps it.
    const ENTRY: u32 = 0x0026_3128;
    const MATCHED: u32 = 0x0026_3164;
    const NEXT: u32 = 0x0026_316c;
    const CHECK: u32 = 0x0026_3188;
    const ADVANCE: u32 = 0x0026_31a8;
    const DONE: u32 = 0x0026_31c4;
    guest(c, |c| {
        // The frame lives on the guest stack, so a resumed call finds it at
        // the same sp-relative offsets; only `sp` itself is set in ENTRY.
        let frame = Frame { size: 0x60 };
        let mut pc = c.pc;
        loop {
            match pc {
                ENTRY => {
                    c.cycles += 13;
                    c.g[SP][0] = sx(r(c, SP).wrapping_sub(0x60));
                    frame.save(c, S2, 0x30);
                    frame.save(c, S3, 0x20);
                    frame.save(c, S0, 0x50);
                    let this = r(c, A0);
                    c.g[S3][0] = sx(this);
                    frame.save(c, S1, 0x40);
                    frame.save_ra(c, 0x10);
                    c.g[V1][0] = Addr(p(this, 0xc0)).lw(c);
                    c.g[S0][0] = Addr(p(this, 0xbc)).lw(c);
                    c.g[V0][0] = sx(r(c, V1).wrapping_sub(r(c, S0)));
                    c.g[V0][0] = sx((r(c, V0) as i32 >> 2) as u32);
                    let empty = c.g[V0][0] == 0;
                    c.g[S2][0] = c.g[A1][0];
                    if empty {
                        c.cycles += 2;
                        c.g[V0][0] = 0;
                        pc = DONE;
                    } else {
                        pc = 0x0026_3174;
                    }
                    continue;
                }
                0x0026_3174 => {
                    c.cycles += 3;
                    c.g[S1][0] = c.g[S0][0];
                    let at_end = c.g[S1][0] == c.g[V1][0];
                    c.g[V0][0] = 0;
                    if at_end {
                        pc = DONE;
                    } else {
                        pc = 0x0026_3180;
                    }
                    continue;
                }
                0x0026_3180 => {
                    c.cycles += 2;
                    c.g[S0][0] = Addr(r(c, S1)).lw(c);
                    pc = CHECK;
                    continue;
                }
                MATCHED => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(NEXT);
                    c.g[A1][0] = 2;
                    call_ptr(c, 0x0031_8260, NEXT)?;
                    c.g[V0][0] = 1;
                    pc = DONE;
                    continue;
                }
                CHECK => {
                    c.cycles += 8;
                    c.g[A0][0] = sx(r(c, SP));
                    c.g[RA][0] = sx(0x0026_3194);
                    c.g[A1][0] = c.g[S2][0];
                    call_ptr(c, 0x0031_82d8, 0x0026_3194)?;
                    c.g[A0][0] = Addr(r(c, S0)).lw(c);
                    c.g[RA][0] = sx(0x0026_31a0);
                    c.g[A1][0] = c.g[S2][0];
                    call_ptr(c, 0x0041_65a8, 0x0026_31a0)?;
                    let same = c.g[V0][0] == 0;
                    c.g[A0][0] = sx(r(c, SP));
                    if same {
                        if event_due(c) {
                            if crate::events::deliver_event(c, MATCHED) {
                                return Ok(());
                            }
                        }
                        pc = MATCHED;
                        continue;
                    }
                    pc = ADVANCE;
                    continue;
                }
                ADVANCE => {
                    c.cycles += 6;
                    c.g[RA][0] = sx(0x0026_31b0);
                    c.g[A1][0] = 2;
                    call_ptr(c, 0x0031_8260, 0x0026_31b0)?;
                    c.g[S1][0] = sx(r(c, S1).wrapping_add(4));
                    c.g[V0][0] = Addr(p(r(c, S3), 0xc0)).lw(c);
                    if c.g[S1][0] != c.g[V0][0] {
                        c.g[S0][0] = Addr(r(c, S1)).lw(c);
                        pc = CHECK;
                        if event_due(c) {
                            if crate::events::deliver_event(c, CHECK) {
                                return Ok(());
                            }
                        }
                        continue;
                    }
                    pc = 0x0026_31c0;
                    continue;
                }
                0x0026_31c0 => {
                    c.cycles += 1;
                    c.g[V0][0] = 0;
                    pc = DONE;
                    continue;
                }
                DONE => {
                    c.cycles += 7;
                    frame.restore(c, 16, 0x50);
                    frame.restore(c, 17, 0x40);
                    frame.restore(c, 18, 0x30);
                    frame.restore(c, 19, 0x20);
                    frame.restore_ra(c, 0x10);
                    frame.close(c);
                    return Ok(());
                }
                other => {
                    c.jump_out(other, ENTRY, 0x0026_31e0);
                    return Ok(());
                }
            }
        }
    })
}

// ---------------------------------------------------------------------------
// `LobbyMan_CreateRoom` (0x0025dac0).
//
// `void LobbyMan_CreateRoom(LobbyMan* self, char* name, int players, int mode)`:
// format the room name, append the optional password and send the create
// request.

pub fn lobby_man_create_room(c: &mut Cpu) {
    guest(c, |c| {
        'b_25db68: {
            c.cycles += 11;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x240));
            c.g[V1][0] = c.g[A2][0];
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x220));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x210));
            c.g[S1][0] = c.g[A3][0];
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x230));
            c.g[S2][0] = c.g[A0][0];
            c.sd((c.g[SP][0] as u32).wrapping_add(0x200), c.g[RA][0]);
            c.g[V0][0] = c.lw((c.g[S2][0] as u32));
            let t = c.g[V0][0] == 0u64;
            c.g[A0][0] = c.g[A1][0];
            if t {
                break 'b_25db68;
            } else {
                'b_25db40: {
                    c.cycles += 16;
                    c.g[A2][0] = c.g[A0][0];
                    c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x100));
                    c.g[A1][0] = 0x4a0000;
                    c.g[A3][0] = c.g[V1][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3150));
                    c.g[RA][0] = sx(0x0025db08);
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x00417828, 0x0025db08)?;
                    c.g[A2][0] = 0x4a0000;
                    c.g[A3][0] = c.g[S0][0];
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x2f58));
                    c.sb((c.g[SP][0] as u32), 0u64);
                    c.g[A0][0] = c.g[SP][0];
                    c.g[RA][0] = sx(0x0025db24);
                    c.g[A1][0] = 0x100;
                    call_ptr(c, 0x003ec180, 0x0025db24)?;
                    let t = c.g[S1][0] == 0u64;
                    c.g[A2][0] = 0x4a0000;
                    if t {
                        break 'b_25db40;
                    } else {
                        c.cycles += 5;
                        c.g[A3][0] = c.g[S1][0];
                        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x3040));
                        c.g[A0][0] = c.g[SP][0];
                        c.g[RA][0] = sx(0x0025db40);
                        c.g[A1][0] = 0x100;
                        call_ptr(c, 0x003ec180, 0x0025db40)?;
                        break 'b_25db40;
                    }
                }
                c.cycles += 10;
                c.g[T0][0] = 0x270000;
                c.g[A2][0] = 0x726f0000;
                c.g[AT][0] = 0x41a00000;
                c.mtc1(1, 12);
                c.g[A0][0] = c.g[S2][0];
                c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x72a0));
                c.g[A1][0] = 0x11;
                c.g[A2][0] = c.g[A2][0] | 0x6f6d;
                c.g[RA][0] = sx(0x0025db68);
                c.g[A3][0] = c.g[SP][0];
                call_ptr(c, 0x0025b608, 0x0025db68)?;
                break 'b_25db68;
            }
        }
        c.cycles += 6;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x230));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x220));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x210));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x200));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x240));
        c.pc = t;
        return Ok(());

        Ok(())
    })
}

// ---------------------------------------------------------------------------
// `cSetupMan_HostRecvCancel` (0x00265398).
//
// `void cSetupMan_HostRecvCancel(cSetupMan* self, Packet* out)`: acknowledge a
// cancelled join and pass the queue on.

pub fn setup_man_host_recv_cancel(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x50);
        c.g[V1][0] = 0x3;
        frame.save(c, 16, 0x40);
        frame.save(c, 17, 0x30);
        frame.save_ra(c, 0x20);
        let this = c.g[A0][0] as u32;
        c.g[V0][0] = Addr(this).lw(c);
        c.g[A1][0] = 0x480000;
        if c.g[V0][0] == c.g[V1][0] {
            c.cycles += 2;
            c.g[V0][0] = 0x2;
            sw_4(c, this, 0, c.g[V0][0]);
        }
        c.cycles += 24;
        c.g[S0][0] = Addr(gp_14(c, -0xc8)).lw(c);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x488));
        c.g[A0][0] = 0x8;
        c.g[A2][0] = 0x2000_0000;
        c.g[A3][0] = 0;
        call_ptr(c, 0x0031_7d70, 0x0026_53d8)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[V1][0] = 0xf7;
        c.g[V0][0] = 0x480000;
        sw_4(c, c.g[S1][0] as u32, 0, c.g[V1][0]);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x12b0));
        c.g[A1][0] = 0x480000;
        sw_4(c, c.g[S1][0] as u32, 4, c.g[V0][0]);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4a8));
        c.g[A0][0] = 0xc;
        c.g[A2][0] = 0x2000_0000;
        let s0 = c.g[S0][0] as u32;
        c.g[V0][0] = Addr(s0.wrapping_add(0xf0)).lw(c);
        c.g[A3][0] = 0;
        sw_4(c, sp(c), 0x10, c.g[V0][0]);
        call_ptr(c, 0x0031_7d70, 0x0026_5410)?;
        c.g[A1][0] = c.g[V0][0];
        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x8));
        if c.g[V0][0] != 0 {
            sw_4(c, c.g[A1][0] as u32, 8, c.g[S1][0]);
        }
        c.cycles += 14;
        c.g[V0][0] = lw_7(c, sp(c), 0x10);
        frame.restore(c, 16, 0x40);
        sw_4(c, c.g[A1][0] as u32, 0, c.g[V0][0]);
        frame.restore(c, 17, 0x30);
        c.g[V1][0] = lw_7(c, c.g[V0][0] as u32, 4);
        frame.restore_ra(c, 0x20);
        sw_4(c, c.g[A1][0] as u32, 4, c.g[V1][0]);
        c.g[A0][0] = lw_7(c, c.g[V0][0] as u32, 4);
        sw_4(c, c.g[A0][0] as u32, 0, c.g[A1][0]);
        c.g[V0][0] = lw_7(c, sp(c), 0x10);
        sw_4(c, c.g[V0][0] as u32, 4, c.g[A1][0]);
        sw_4(c, sp(c), 0, c.g[A1][0]);
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
                c.g[A0][0] = Addr(gp_15(c, -0x238)).lw(c);
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
    c.cycles += 50;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
    c.g[A1][0] = 0x480000;
    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.g[A2][0] = 0x20000000;
    c.g[S2][0] = c.g[A0][0];
    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
    c.g[A3][0] = 0u64;
    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x488));
    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.sd((c.g[SP][0] as u32), c.g[RA][0]);
    c.g[RA][0] = sx(0x002684f8);
    c.g[A0][0] = 0x2c;
    if call_ptr(c, 0x00317d70, 0x002684f8).is_err() {
        return;
    }
    c.g[S0][0] = c.g[V0][0];
    c.g[V1][0] = 0x480000;
    c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x8));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1190));
    c.g[V0][0] = 0xe3;
    c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[V1][0]);
    c.sw((c.g[S0][0] as u32), c.g[V0][0]);
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x8));
    c.g[S1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xc));
    c.g[S3][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
    c.g[RA][0] = sx(0x00268528);
    c.g[S4][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x14));
    if call_ptr(c, 0x003182d8, 0x00268528).is_err() {
        return;
    }
    c.g[A1][0] = c.g[S1][0];
    c.g[RA][0] = sx(0x00268534);
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xc));
    if call_ptr(c, 0x003182d8, 0x00268534).is_err() {
        return;
    }
    c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[S3][0]);
    c.g[V0][0] = c.g[S0][0];
    c.sw((c.g[S0][0] as u32).wrapping_add(0x14), c.g[S4][0]);
    c.ldl(3, (c.g[S2][0] as u32).wrapping_add(0x1f));
    c.ldr(3, (c.g[S2][0] as u32).wrapping_add(0x18));
    c.ldl(4, (c.g[S2][0] as u32).wrapping_add(0x27));
    c.ldr(4, (c.g[S2][0] as u32).wrapping_add(0x20));
    c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x28));
    c.sdl(3, (c.g[S0][0] as u32).wrapping_add(0x1f));
    c.sdr(3, (c.g[S0][0] as u32).wrapping_add(0x18));
    c.sdl(4, (c.g[S0][0] as u32).wrapping_add(0x27));
    c.sdr(4, (c.g[S0][0] as u32).wrapping_add(0x20));
    c.sw((c.g[S0][0] as u32).wrapping_add(0x28), c.g[A1][0]);
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.g[RA][0] = c.ld((c.g[SP][0] as u32));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
    c.pc = t;
    return;
}

/// The two-instruction virtual-method read the originals use:
/// `a0 = lh(vt+off)`, `func = lw(vt+off+4)`. The original leaves `func` in
/// `vreg` (v0 or v1 depending on the site) and `a0` as the signed slot delta;
/// the fake-call log sees both, so they are written here too. Returns `func`
/// for the caller's `call_ptr`.
#[inline(always)]
pub(crate) fn vslot(c: &mut Cpu, vt: u64, off: u32, vreg: usize) -> u32 {
    let base = (vt as u32).wrapping_add(off);
    c.g[A0][0] = Addr(base).lh(c);
    let func = Addr(base.wrapping_add(4)).lw(c);
    c.g[vreg][0] = func;
    func as u32
}

// ---------------------------------------------------------------------------
// Setup manager / audio / song setup.

/// `void cSetupMan_SetRemoteCharacter(cSetupMan *self, void *character,
/// const u8 *blob)` (0x264d80): publish the character pointer to the BE
/// interface, store it in the interface singleton, and copy the remote
/// character's four-byte id when the blob is non-null.
pub fn csetup_man_set_remote_character(c: &mut Cpu) {
    guest(c, |c| {
        'b_264e20: {
            c.cycles += 30;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x40));
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
            c.g[S1][0] = c.g[A1][0];
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sd((c.g[SP][0] as u32), c.g[RA][0]);
            c.g[RA][0] = sx(0x00264da0);
            c.g[S2][0] = c.g[A2][0];
            call_ptr(c, 0x0014dc80, 0x00264da0)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[RA][0] = sx(0x00264dac);
            c.g[A1][0] = 0x1;
            call_ptr(c, 0x0014dd58, 0x00264dac)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[A2][0] = c.g[S1][0];
            c.g[A1][0] = 0x1;
            c.g[RA][0] = sx(0x00264dc0);
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x00147448, 0x00264dc0)?;
            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xc));
            vslot(c, c.g[V0][0], 0x8, V1);
            let t = c.g[V1][0] as u32;
            c.g[RA][0] = sx(0x00264dd4);
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add((c.g[A0][0] as u32)));
            call_ptr(c, t, 0x00264dd4)?;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            c.g[A1][0] = 0x7;
            c.g[RA][0] = sx(0x00264de4);
            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x78));
            call_ptr(c, 0x0014dd58, 0x00264de4)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[V0][0] = 0x530000;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4b30));
            let t = c.g[S2][0] == 0u64;
            c.sw((c.g[V0][0] as u32).wrapping_add(0x32c), c.g[S1][0]);
            if t {
                break 'b_264e20;
            } else {
                c.cycles += 10;
                c.lwl(3, (c.g[S2][0] as u32).wrapping_add(0x3));
                c.lwr(3, (c.g[S2][0] as u32));
                c.g[A0][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x4));
                c.g[A2][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x5));
                c.g[A3][0] = c.lb((c.g[S2][0] as u32).wrapping_add(0x6));
                c.swl(3, (c.g[V0][0] as u32).wrapping_add(0x4ab));
                c.swr(3, (c.g[V0][0] as u32).wrapping_add(0x4a8));
                c.sb((c.g[V0][0] as u32).wrapping_add(0x4ac), c.g[A0][0]);
                c.sb((c.g[V0][0] as u32).wrapping_add(0x4ad), c.g[A2][0]);
                c.sb((c.g[V0][0] as u32).wrapping_add(0x4ae), c.g[A3][0]);
                break 'b_264e20;
            }
        }
        c.cycles += 11;
        c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xc));
        vslot(c, c.g[V0][0], 0x8, V1);
        let t = c.g[V1][0] as u32;
        c.g[RA][0] = sx(0x00264e34);
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
        call_ptr(c, t, 0x00264e34)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
        return Ok(());
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// LobbyMan_SendLockChal (0x000025fc70).

/// `LobbyMan_SendLockChal`: queue a "lock challenge" message on the lobby manager and start its 20-frame hold timer.
pub fn lobby_man_send_lock_chal(c: &mut Cpu) {
    guest(c, |c| {
        'b_25fcfc: {
            c.cycles += 35;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x60));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.g[S0][0] = c.g[A2][0];
            c.g[S2][0] = c.g[A0][0];
            c.g[A2][0] = 0x480000;
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x910));
            call_ptr(c, 0x0025fb80, 0x0025fc98)?;
            c.g[S1][0] = 0x110;
            c.sw((c.g[S2][0] as u32), c.g[S0][0]);
            c.g[A1][0] = 0x480000;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x488));
            c.g[A0][0] = 0x8;
            c.g[A2][0] = 0x20000000;
            c.g[A3][0] = 0u64;
            call_ptr(c, 0x00317d70, 0x0025fcb8)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xf0));
            c.g[V0][0] = 0x480000;
            c.sw((c.g[S0][0] as u32), c.g[S1][0]);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x12b0));
            c.g[A1][0] = 0x480000;
            c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4a8));
            c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[V1][0]);
            c.g[A0][0] = 0xc;
            c.g[A2][0] = 0x20000000;
            c.g[A3][0] = 0u64;
            call_ptr(c, 0x00317d70, 0x0025fcec)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x8));
            if c.g[V0][0] != 0u64 {
                c.sw((c.g[A1][0] as u32).wrapping_add(0x8), c.g[S0][0]);
                break 'b_25fcfc;
            } else {
                break 'b_25fcfc;
            }
        }
        c.cycles += 21;
        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[AT][0] = 0x41a00000;
        c.mtc1(1, 0);
        c.sw((c.g[A1][0] as u32), c.g[A0][0]);
        c.g[AT][0] = 0x41a80000;
        c.mtc1(1, 1);
        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
        c.sw((c.g[V1][0] as u32), c.g[A1][0]);
        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
        c.sw((c.g[V0][0] as u32).wrapping_add(0x4), c.g[A1][0]);
        c.swc1(0, (c.g[S2][0] as u32).wrapping_add(0xe0));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.sw((c.g[SP][0] as u32), c.g[A1][0]);
        c.swc1(1, (c.g[GP][0] as u32).wrapping_add(0x2064));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
        c.pc = t;
        return Ok(());
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// LobbyMan_OnDisconnect (0x00002598c8).

/// `LobbyMan_OnDisconnect`: handle losing the network connection in the lobby.
pub fn lobby_man_on_disconnect(c: &mut Cpu) {
    guest(c, |c| {
        'b_2599a0: {
            'b_25999c: {
                c.cycles += 9;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x50));
                c.g[V0][0] = 0x64690000;
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[V0][0] = c.g[V0][0] | 0x7363;
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
                c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4));
                let t = c.g[V1][0] != c.g[V0][0];
                c.g[S1][0] = c.g[A0][0];
                if t {
                    break 'b_25999c;
                } else {
                    'b_25994c: {
                        c.cycles += 24;
                        c.g[A1][0] = 0x480000;
                        c.g[A2][0] = 0x20000000;
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x488));
                        c.g[A0][0] = 0x8;
                        c.g[A3][0] = 0u64;
                        call_ptr(c, 0x00317d70, 0x00259904)?;
                        c.g[S0][0] = c.g[V0][0];
                        c.g[A2][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xf0));
                        c.g[V0][0] = 0x480000;
                        c.g[V1][0] = 0x115;
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x12b0));
                        c.sw((c.g[S0][0] as u32), c.g[V1][0]);
                        c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                        c.g[A1][0] = 0x480000;
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[A2][0]);
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4a8));
                        c.g[A2][0] = 0x20000000;
                        c.g[A0][0] = 0xc;
                        c.g[A3][0] = 0u64;
                        call_ptr(c, 0x00317d70, 0x0025993c)?;
                        c.g[A2][0] = c.g[V0][0];
                        c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(0x8));
                        if c.g[V0][0] != 0u64 {
                            c.sw((c.g[A2][0] as u32).wrapping_add(0x8), c.g[S0][0]);
                            break 'b_25994c;
                        } else {
                            break 'b_25994c;
                        }
                    }
                    c.cycles += 15;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[A1][0] = 0x1;
                    c.sw((c.g[A2][0] as u32), c.g[A0][0]);
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
                    c.sw((c.g[A2][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
                    c.sw((c.g[V1][0] as u32), c.g[A2][0]);
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x4), c.g[A2][0]);
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x10), c.g[A1][0]);
                    c.sw((c.g[SP][0] as u32), c.g[A2][0]);
                    call_ptr(c, 0x002591d0, 0x0025997c)?;
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32));
                    let t = c.g[V1][0] == 0u64;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
                    if t {
                        break 'b_2599a0;
                    } else {
                        c.cycles += 5;
                        call_ptr(c, 0x002591d0, 0x00259990)?;
                        c.g[A0][0] = c.g[V0][0];
                        c.g[A1][0] = 0x2;
                        call_ptr(c, 0x00263ba8, 0x0025999c)?;
                        break 'b_25999c;
                    }
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
            break 'b_2599a0;
        }
        c.cycles += 4;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x50));
        c.pc = t;
        return Ok(());
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// LobbyMan_ParseNameSuggestions (0x000025bc48).

/// `LobbyMan_ParseNameSuggestions`: parse the name-suggestion list the server sent.
pub fn lobby_man_parse_name_suggestions(c: &mut Cpu) {
    guest(c, |c| {
        'b_25bd1c: {
            'b_25bd18: {
                c.cycles += 20;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x140));
                c.g[V0][0] = 0x4a0000;
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x130));
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x120));
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x110));
                c.g[S1][0] = c.g[A0][0];
                c.sd((c.g[SP][0] as u32).wrapping_add(0x100), c.g[RA][0]);
                c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xc));
                c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3090));
                call_ptr(c, 0x003eb6d8, 0x0025bc70)?;
                c.g[A3][0] = 0x4a0000;
                c.g[A0][0] = c.g[V0][0];
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x2f40));
                c.g[A1][0] = c.g[SP][0];
                c.g[A2][0] = 0x100;
                call_ptr(c, 0x003ed378, 0x0025bc88)?;
                c.g[V1][0] = c.lb((c.g[SP][0] as u32));
                c.g[S0][0] = c.g[SP][0];
                let t = c.g[V1][0] == 0u64;
                c.sw((c.g[S1][0] as u32).wrapping_add(0x284), 0u64);
                if t {
                    break 'b_25bd18;
                } else {
                    c.cycles += 2;
                    c.g[S2][0] = 0x2c;
                    c.g[V0][0] = c.lb((c.g[S0][0] as u32));
                    'l_25bca0: loop {
                        'b_25bce4: {
                            'b_25bce0: {
                                c.cycles += 2;
                                let t = c.g[V0][0] == 0u64;
                                c.g[A1][0] = c.g[S0][0];
                                if t {
                                    break 'b_25bce0;
                                } else {
                                    'b_25bccc: {
                                        c.cycles += 2;
                                        let t = c.g[V0][0] == c.g[S2][0];
                                        c.g[V1][0] = 0x2c;
                                        if t {
                                            break 'b_25bccc;
                                        } else {
                                            c.cycles += 2;
                                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                                            'l_25bcb8: loop {
                                                c.cycles += 3;
                                                c.g[V0][0] = c.lb((c.g[S0][0] as u32));
                                                if c.g[V0][0] == 0u64 {
                                                    c.g[V0][0] =
                                                        c.lw(
                                                            (c.g[S1][0] as u32).wrapping_add(0x284)
                                                        );
                                                    break 'b_25bce4;
                                                } else {
                                                    c.cycles += 2;
                                                    if c.g[V0][0] != c.g[V1][0] {
                                                        c.g[S0][0] =
                                                            sx((c.g[S0][0] as u32)
                                                                .wrapping_add(0x1));
                                                        if event_due(c) {
                                                            deliver_event_here(c, 0x0025bcb8);
                                                        }
                                                        continue 'l_25bcb8;
                                                    } else {
                                                        break 'b_25bccc;
                                                    }
                                                }
                                            }
                                        }
                                    }
                                    c.cycles += 3;
                                    c.g[V0][0] = c.lb((c.g[S0][0] as u32));
                                    if c.g[V0][0] == 0u64 {
                                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x284));
                                        break 'b_25bce4;
                                    } else {
                                        c.cycles += 2;
                                        c.sb((c.g[S0][0] as u32), 0u64);
                                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                                        break 'b_25bce0;
                                    }
                                }
                            }
                            c.cycles += 1;
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x284));
                            break 'b_25bce4;
                        }
                        c.cycles += 9;
                        c.g[A0][0] = sx((c.g[V0][0] as u32) << 2);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x274));
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x284), c.g[V0][0]);
                        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        call_ptr(c, 0x003184e8, 0x0025bcfc)?;
                        c.g[V1][0] = c.lb((c.g[S0][0] as u32));
                        if c.g[V1][0] == 0u64 {
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x130));
                            break 'b_25bd1c;
                        } else {
                            c.cycles += 4;
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x284));
                            c.g[V0][0] = ((c.g[V0][0] as i64) < 4i64) as u64;
                            if c.g[V0][0] != 0u64 {
                                c.g[V0][0] = c.lb((c.g[S0][0] as u32));
                                if event_due(c) {
                                    deliver_event_here(c, 0x0025bca0);
                                }
                                continue 'l_25bca0;
                            } else {
                                break 'b_25bd18;
                            }
                        }
                    }
                }
            }
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x130));
            break 'b_25bd1c;
        }
        c.cycles += 5;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x120));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x110));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x100));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x140));
        c.pc = t;
        return Ok(());
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// Lobby quick play.

/// `void LobbyMan_OnQuickPlayReply(LobbyMan* self, void* msg)` (0x25f030):
/// a null reply moves state 0x12 to 0x14; otherwise enter state 1, allocate
/// the reply message record and link it into the message list.
pub fn lobby_man_on_quick_play_reply(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x60);
        c.g[V1][0] = c.g[A1][0];
        frame.save(c, S1, 0x40);
        frame.save(c, S0, 0x50);
        frame.save(c, S2, 0x30);
        frame.save_ra(c, 0x20);
        c.g[V0][0] = Addr(r(c, V1)).off(0x8).lw(c);
        let empty = c.g[V0][0] != 0;
        c.g[S1][0] = c.g[A0][0];

        if !empty {
            c.cycles += 4;
            c.g[V1][0] = Addr(r(c, S1)).lw(c);
            c.g[V0][0] = 0x12;
            let other = c.g[V1][0] != c.g[V0][0];
            frame.restore(c, 16, 0x50);
            if other {
                frame.restore(c, 17, 0x40);
                frame.restore(c, 18, 0x30);
                c.cycles += 5;
                frame.restore_ra(c, 0x20);
                frame.close(c);
                return Ok(());
            }
            c.cycles += 3;
            c.g[V0][0] = 0x14;
            c.sw(r(c, S1), c.g[V0][0]);
            frame.restore(c, 17, 0x40);
            frame.restore(c, 18, 0x30);
            c.cycles += 5;
            frame.restore_ra(c, 0x20);
            frame.close(c);
            return Ok(());
        }

        c.cycles += 28;
        c.g[V0][0] = 1;
        c.g[A1][0] = 0x480488;
        c.sw(r(c, S1), c.g[V0][0]);
        c.g[A3][0] = 0;
        c.g[A0][0] = 0xc;
        c.g[S0][0] = Addr(r(c, V1)).off(0x8).lw(c);
        c.g[A2][0] = 0x2000_0000;
        call_ptr(c, 0x0031_7d70, 0x0025_f094)?;

        c.g[S2][0] = c.g[V0][0];
        c.g[A3][0] = Addr(r(c, S1)).off(0xf0).lw(c);
        c.g[V0][0] = 0x481290;
        c.g[V1][0] = 0x102;
        c.sw(r(c, S2), c.g[V1][0]);
        c.sw(p(r(c, S2), 0x4), c.g[V0][0]);
        c.sw(p(r(c, S2), 0x8), c.g[S0][0]);
        c.sw(p(r(c, SP), 0x10), c.g[A3][0]);
        c.g[A0][0] = 0xc;
        c.g[A1][0] = 0x4804a8;
        c.g[A2][0] = 0x2000_0000;
        c.g[A3][0] = 0;
        call_ptr(c, 0x0031_7d70, 0x0025_f0d0)?;

        c.g[A1][0] = c.g[V0][0];
        c.g[V0][0] = sx(p(r(c, A1), 0x8));
        if c.g[V0][0] != 0 {
            c.sw(Addr(r(c, A1)).off(0x8).0, c.g[S2][0]);
        }

        c.cycles += 10;
        c.g[V0][0] = Addr(p(r(c, SP), 0x10)).lw(c);
        c.sw(r(c, A1), c.g[V0][0]);
        c.g[V1][0] = Addr(r(c, V0)).off(0x4).lw(c);
        c.sw(Addr(r(c, A1)).off(0x4).0, c.g[V1][0]);
        c.g[A0][0] = Addr(r(c, V0)).off(0x4).lw(c);
        c.sw(r(c, A0), c.g[A1][0]);
        c.g[V0][0] = Addr(p(r(c, SP), 0x10)).lw(c);
        c.sw(Addr(r(c, V0)).off(0x4).0, c.g[A1][0]);
        c.sw(r(c, SP), c.g[A1][0]);
        frame.restore(c, 16, 0x50);
        frame.restore(c, 17, 0x40);
        frame.restore(c, 18, 0x30);
        c.cycles += 5;
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// LobbyMan reply handlers
// ---------------------------------------------------------------------------

/// The shared body of the `LobbyMan_On*Reply` handlers (0x25c0c0 / 0x25c2d8 /
/// 0x25c4f0).
///
/// Marks the handler consumed (`self->0 = 1`), builds a reply message — either
/// the short `{tag, D_004812B0}` one when the reply carries no text, or the
/// `{0x102, D_00481290, text}` one — and posts it through the lobby's message
/// list at `self + 0xf0`. `tag` is the message id that distinguishes the
/// handlers.
pub(crate) fn lobby_send_reply(
    c: &mut Cpu,
    tag: u64,
    ret1: u32,
    ret2: u32,
    ret3: u32,
) -> crate::call::Flow {
    let frame = Frame::open(c, 0x60);
    c.cycles += 12;
    c.g[V0][0] = 1;
    frame.save(c, 17, 0x40);
    frame.save(c, 18, 0x30);
    c.g[S1][0] = c.g[A0][0]; // self
    frame.save(c, 16, 0x50);
    c.g[S2][0] = c.g[A1][0]; // reply
    frame.save_ra(c, 0x20);
    c.sw(c.g[S1][0] as u32, c.g[V0][0]);
    c.g[V0][0] = c.lw(p(c.g[S2][0] as u32, 8));
    let has_text = c.g[V0][0] != 0;
    c.g[A1][0] = 0x0048_0000;

    'tail: {
        if !has_text {
            // Short message.
            c.cycles += 19;
            c.g[A2][0] = 0x2000_0000;
            c.g[A1][0] = sx(0x0048_0488);
            c.g[A0][0] = 8;
            c.g[RA][0] = sx(ret1);
            c.g[A3][0] = 0;
            call_ptr(c, 0x0031_7d70, ret1)?; // cMemMan_alloc
            c.g[S0][0] = c.g[V0][0];
            c.g[A2][0] = c.lw(p(c.g[S1][0] as u32, 0xf0));
            c.g[V0][0] = 0x0048_0000;
            c.g[V1][0] = tag;
            c.g[V0][0] = sx(0x0048_12b0);
            c.sw(c.g[S0][0] as u32, c.g[V1][0]);
            c.sw(p(c.g[S0][0] as u32, 4), c.g[V0][0]);
            c.g[A1][0] = 0x0048_0000;
            c.sw(p(c.g[SP][0] as u32, 0x10), c.g[A2][0]);
            c.g[A1][0] = sx(0x0048_04a8);
            c.g[A0][0] = 0xc;
            c.g[A2][0] = 0x2000_0000;
            c.g[A3][0] = 0;
            break 'tail;
        }
        // Message with its text pointer.
        c.cycles += 20;
        c.g[A0][0] = 0xc;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x488));
        c.g[A2][0] = 0x2000_0000;
        c.g[RA][0] = sx(ret2);
        c.g[A3][0] = 0;
        call_ptr(c, 0x0031_7d70, ret2)?; // cMemMan_alloc
        c.g[S0][0] = c.g[V0][0];
        c.g[A0][0] = c.lw(p(c.g[S2][0] as u32, 8));
        c.g[V0][0] = 0x0048_0000;
        c.g[A2][0] = c.lw(p(c.g[S1][0] as u32, 0xf0));
        c.g[V0][0] = sx(0x0048_1290);
        c.g[V1][0] = 0x102;
        c.sw(p(c.g[S0][0] as u32, 8), c.g[A0][0]);
        c.g[A1][0] = 0x0048_0000;
        c.sw(c.g[S0][0] as u32, c.g[V1][0]);
        c.g[A1][0] = sx(0x0048_04a8);
        c.sw(p(c.g[S0][0] as u32, 4), c.g[V0][0]);
        c.g[A0][0] = 0xc;
        c.sw(p(c.g[SP][0] as u32, 0x10), c.g[A2][0]);
        c.g[A3][0] = 0;
        c.g[A2][0] = 0x2000_0000;
    }

    // Link the message into the list at self+0xf0.
    'link: {
        c.cycles += 6;
        c.g[RA][0] = sx(ret3);
        call_ptr(c, 0x0031_7d70, ret3)?; // cMemMan_alloc
        c.g[A1][0] = c.g[V0][0];
        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(8));
        if c.g[V0][0] != 0 {
            c.sw(p(c.g[A1][0] as u32, 8), c.g[S0][0]);
            break 'link;
        }
        break 'link;
    }
    c.cycles += 15;
    c.g[V0][0] = c.lw(p(c.g[SP][0] as u32, 0x10));
    c.sw(c.g[A1][0] as u32, c.g[V0][0]);
    c.g[V1][0] = c.lw(p(c.g[V0][0] as u32, 4));
    c.sw(p(c.g[A1][0] as u32, 4), c.g[V1][0]);
    c.g[A0][0] = c.lw(p(c.g[V0][0] as u32, 4));
    c.sw(c.g[A0][0] as u32, c.g[A1][0]);
    c.g[V0][0] = c.lw(p(c.g[SP][0] as u32, 0x10));
    c.sw(p(c.g[V0][0] as u32, 4), c.g[A1][0]);
    c.sw(c.g[SP][0] as u32, c.g[A1][0]);
    frame.restore(c, 16, 0x50);
    frame.restore(c, 17, 0x40);
    frame.restore(c, 18, 0x30);
    frame.restore_ra(c, 0x20);
    frame.close(c);
    Ok(())
}

/// `void LobbyMan_OnEditAccountReply(LobbyMan* self, Reply* r)` (0x25c0c0):
/// posts the "account updated" message.
pub fn lobby_man_on_edit_account_reply(c: &mut Cpu) {
    guest(c, |c| {
        lobby_send_reply(c, 0xce, 0x0025_c104, 0x0025_c150, 0x0025_c194)
    })
}

/// `void LobbyMan_OnLostByEmailReply(LobbyMan* self, Reply* r)` (0x25c2d8):
/// posts the "account not found by e-mail" message.
pub fn lobby_man_on_lost_by_email_reply(c: &mut Cpu) {
    guest(c, |c| {
        lobby_send_reply(c, 0xcf, 0x0025_c31c, 0x0025_c368, 0x0025_c3ac)
    })
}

/// `void LobbyMan_OnLostByNameReply(LobbyMan* self, Reply* r)` (0x25c4f0):
/// posts the "account not found by name" message.
pub fn lobby_man_on_lost_by_name_reply(c: &mut Cpu) {
    guest(c, |c| {
        lobby_send_reply(c, 0xd2, 0x0025_c534, 0x0025_c580, 0x0025_c5c4)
    })
}

/// `void func_0025D538(LobbyMan* self, sMessage* msg)` (0x25d538): handle the
/// reply to "delete persona": mark the screen busy and post either the
/// "deleted" (msg+8 == 0) or the "failed" message, shifting the remaining
/// persona names down in the first case.
pub fn lobby_man_on_delete_persona_reply(c: &mut Cpu) {
    let frame = Frame::open(c, 0x80);
    'b_25d664: {
        'b_25d614: {
            c.cycles += 14;
            c.g[V0][0] = 0x1;
            frame.save(c, S3, 0x40);
            frame.save(c, S1, 0x60);
            c.g[S3][0] = c.g[A0][0];
            frame.save(c, S0, 0x70);
            c.g[S1][0] = c.g[A1][0];
            frame.save(c, S2, 0x50);
            frame.save(c, S4, 0x30);
            frame.save_ra(c, 0x20);
            sw_6(c, r32(c, S3), c.g[V0][0]);
            c.g[V0][0] = lw(c, at(c, S1, 0x8));
            let t = c.g[V0][0] != 0u64;
            c.g[A1][0] = 0x480000;
            if t {
                break 'b_25d614;
            } else {
                'b_25d5c8: {
                    c.cycles += 7;
                    c.g[V0][0] = lw(c, at(c, S3, 0x6c));
                    c.g[S4][0] = sx(at(c, S3, 0xec));
                    c.g[S2][0] = lw(c, at(c, S3, 0x50));
                    c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x1));
                    c.g[V1][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let t = c.g[V1][0] == 0u64;
                    sw_6(c, at(c, S3, 0x6c), c.g[V0][0]);
                    if t {
                        break 'b_25d5c8;
                    } else {
                        c.cycles += 5;
                        c.g[V0][0] = sx(r32(c, S2) << 2);
                        c.g[V1][0] = sx(at(c, V0, 0x70));
                        c.g[V0][0] = sx(at(c, V0, 0x74));
                        c.g[S0][0] = sx(r32(c, V1).wrapping_add(r32(c, S3)));
                        c.g[S1][0] = sx(r32(c, V0).wrapping_add(r32(c, S3)));
                        'l_25d5a0: loop {
                            c.cycles += 9;
                            c.g[A0][0] = c.g[S0][0];
                            c.g[RA][0] = sx(0x0025d5ac);
                            c.g[A1][0] = c.g[S1][0];
                            if call_ptr(c, 0x00318448, 0x0025d5ac).is_err() {
                                return;
                            }
                            c.g[S2][0] = sx(at(c, S2, 0x1));
                            c.g[V0][0] = lw(c, at(c, S3, 0x6c));
                            c.g[S1][0] = sx(at(c, S1, 0x4));
                            c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                            let t = c.g[V0][0] != 0u64;
                            c.g[S0][0] = sx(at(c, S0, 0x4));
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x0025d5a0);
                                }
                                continue 'l_25d5a0;
                            } else {
                                c.cycles += 1;
                                c.g[A1][0] = 0x480000;
                                break 'b_25d5c8;
                            }
                        }
                    }
                }
                c.cycles += 19;
                c.g[A2][0] = 0x20000000;
                c.g[A1][0] = sx(at(c, A1, 0x488));
                c.g[A0][0] = 0x8;
                c.g[RA][0] = sx(0x0025d5dc);
                c.g[A3][0] = 0u64;
                if call_ptr(c, 0x00317d70, 0x0025d5dc).is_err() {
                    return;
                }
                c.g[S0][0] = c.g[V0][0];
                c.g[A2][0] = lw(c, at(c, S4, 0x4));
                c.g[V0][0] = 0x480000;
                c.g[V1][0] = 0xd8;
                c.g[V0][0] = sx(at(c, V0, 0x12b0));
                sw_6(c, r32(c, S0), c.g[V1][0]);
                sw_6(c, at(c, S0, 0x4), c.g[V0][0]);
                c.g[A1][0] = 0x480000;
                sw_6(c, at(c, SP, 0x10), c.g[A2][0]);
                c.g[A1][0] = sx(at(c, A1, 0x4a8));
                c.g[A0][0] = 0xc;
                c.g[A2][0] = 0x20000000;
                let t = true;
                c.g[A3][0] = 0u64;
                if t {
                    break 'b_25d664;
                } else {
                    break 'b_25d614;
                }
            }
        }
        c.cycles += 20;
        c.g[A0][0] = 0xc;
        c.g[A1][0] = sx(at(c, A1, 0x488));
        c.g[A2][0] = 0x20000000;
        c.g[RA][0] = sx(0x0025d628);
        c.g[A3][0] = 0u64;
        if call_ptr(c, 0x00317d70, 0x0025d628).is_err() {
            return;
        }
        c.g[S0][0] = c.g[V0][0];
        c.g[A0][0] = lw(c, at(c, S1, 0x8));
        c.g[V0][0] = 0x480000;
        c.g[A2][0] = lw(c, at(c, S3, 0xf0));
        c.g[V0][0] = sx(at(c, V0, 0x1290));
        c.g[V1][0] = 0x102;
        sw_6(c, at(c, S0, 0x8), c.g[A0][0]);
        c.g[A1][0] = 0x480000;
        sw_6(c, r32(c, S0), c.g[V1][0]);
        c.g[A1][0] = sx(at(c, A1, 0x4a8));
        sw_6(c, at(c, S0, 0x4), c.g[V0][0]);
        c.g[A0][0] = 0xc;
        sw_6(c, at(c, SP, 0x10), c.g[A2][0]);
        c.g[A3][0] = 0u64;
        c.g[A2][0] = 0x20000000;
        break 'b_25d664;
    }
    'b_25d67c: {
        c.cycles += 6;
        c.g[RA][0] = sx(0x0025d66c);
        if call_ptr(c, 0x00317d70, 0x0025d66c).is_err() {
            return;
        }
        c.g[A1][0] = c.g[V0][0];
        c.g[V0][0] = sx(at(c, A1, 0x8));
        if c.g[V0][0] != 0u64 {
            sw_6(c, at(c, A1, 0x8), c.g[S0][0]);
            break 'b_25d67c;
        } else {
            break 'b_25d67c;
        }
    }
    c.cycles += 17;
    c.g[V0][0] = lw(c, at(c, SP, 0x10));
    sw_6(c, r32(c, A1), c.g[V0][0]);
    c.g[V1][0] = lw(c, at(c, V0, 0x4));
    sw_6(c, at(c, A1, 0x4), c.g[V1][0]);
    c.g[A0][0] = lw(c, at(c, V0, 0x4));
    sw_6(c, r32(c, A0), c.g[A1][0]);
    c.g[V0][0] = lw(c, at(c, SP, 0x10));
    sw_6(c, at(c, V0, 0x4), c.g[A1][0]);
    sw_6(c, r32(c, SP), c.g[A1][0]);
    frame.restore(c, S0, 0x70);
    frame.restore(c, S1, 0x60);
    frame.restore(c, S2, 0x50);
    frame.restore(c, S3, 0x40);
    frame.restore(c, S4, 0x30);
    c.g[RA][0] = ld(c, at(c, SP, 0x20));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(at(c, SP, 0x80));
    c.pc = t;
    return;
}

/// `void func_0025B848(LobbyMan* self, sMessage* msg)` (0x25b848): handle the
/// reply to "connect": mark the screen busy and post either the "ok"/"cancel"
/// message (msg+8 == 0) after clearing the name field, or the failure message
/// with the status, switching the screen to its "connecting" state.
pub fn lobby_man_on_connect_reply(c: &mut Cpu) {
    let frame = Frame::open(c, 0x70);
    'b_25b9a8: {
        'b_25b920: {
            c.cycles += 13;
            frame.save(c, S2, 0x40);
            frame.save(c, S3, 0x30);
            c.g[S2][0] = c.g[A0][0];
            frame.save(c, S1, 0x50);
            c.g[S3][0] = 0x1;
            frame.save(c, S0, 0x60);
            c.g[S1][0] = c.g[A1][0];
            frame.save_ra(c, 0x20);
            sw_6(c, r32(c, S2), c.g[S3][0]);
            c.g[V0][0] = lw(c, at(c, S1, 0x8));
            let t = c.g[V0][0] != 0u64;
            c.g[A1][0] = 0x480000;
            if t {
                break 'b_25b920;
            } else {
                'b_25b8fc: {
                    c.cycles += 32;
                    c.g[A0][0] = lw(c, at(c, S2, 0x54));
                    c.g[A2][0] = 0x480000;
                    c.g[A1][0] = 0x73650000;
                    c.g[T0][0] = 0u64;
                    c.g[A1][0] = c.g[A1][0] | 0x6c65;
                    c.g[A2][0] = sx(at(c, A2, 0x848));
                    c.g[RA][0] = sx(0x0025b89c);
                    c.g[A3][0] = 0u64;
                    if call_ptr(c, 0x003e8b10, 0x0025b89c).is_err() {
                        return;
                    }
                    c.g[A1][0] = 0x480000;
                    c.g[A2][0] = 0x20000000;
                    c.g[A1][0] = sx(at(c, A1, 0x488));
                    c.g[A0][0] = 0x8;
                    c.g[RA][0] = sx(0x0025b8b4);
                    c.g[A3][0] = 0u64;
                    if call_ptr(c, 0x00317d70, 0x0025b8b4).is_err() {
                        return;
                    }
                    c.g[S0][0] = c.g[V0][0];
                    c.g[A2][0] = lw(c, at(c, S2, 0xf0));
                    c.g[V0][0] = 0x480000;
                    c.g[V1][0] = 0xc9;
                    c.g[V0][0] = sx(at(c, V0, 0x12b0));
                    sw_6(c, r32(c, S0), c.g[V1][0]);
                    sw_6(c, at(c, S0, 0x4), c.g[V0][0]);
                    c.g[A1][0] = 0x480000;
                    sw_6(c, at(c, SP, 0x10), c.g[A2][0]);
                    c.g[A1][0] = sx(at(c, A1, 0x4a8));
                    c.g[A0][0] = 0xc;
                    c.g[A2][0] = 0x20000000;
                    c.g[RA][0] = sx(0x0025b8ec);
                    c.g[A3][0] = 0u64;
                    if call_ptr(c, 0x00317d70, 0x0025b8ec).is_err() {
                        return;
                    }
                    c.g[A1][0] = c.g[V0][0];
                    c.g[V0][0] = sx(at(c, A1, 0x8));
                    if c.g[V0][0] != 0u64 {
                        sw_6(c, at(c, A1, 0x8), c.g[S0][0]);
                        break 'b_25b8fc;
                    } else {
                        break 'b_25b8fc;
                    }
                }
                c.cycles += 9;
                c.g[V0][0] = lw(c, at(c, SP, 0x10));
                sw_6(c, r32(c, A1), c.g[V0][0]);
                c.g[V1][0] = lw(c, at(c, V0, 0x4));
                sw_6(c, at(c, A1, 0x4), c.g[V1][0]);
                c.g[A0][0] = lw(c, at(c, V0, 0x4));
                sw_6(c, r32(c, A0), c.g[A1][0]);
                c.g[V0][0] = lw(c, at(c, SP, 0x10));
                let t = true;
                sw_6(c, at(c, V0, 0x4), c.g[A1][0]);
                if t {
                    break 'b_25b9a8;
                } else {
                    break 'b_25b920;
                }
            }
        }
        'b_25b984: {
            c.cycles += 25;
            c.g[A0][0] = 0xc;
            c.g[A1][0] = sx(at(c, A1, 0x488));
            c.g[A2][0] = 0x20000000;
            c.g[RA][0] = sx(0x0025b934);
            c.g[A3][0] = 0u64;
            if call_ptr(c, 0x00317d70, 0x0025b934).is_err() {
                return;
            }
            c.g[S0][0] = c.g[V0][0];
            c.g[A0][0] = lw(c, at(c, S1, 0x8));
            c.g[V0][0] = 0x480000;
            c.g[A2][0] = lw(c, at(c, S2, 0xf0));
            c.g[V0][0] = sx(at(c, V0, 0x1290));
            c.g[V1][0] = 0x102;
            sw_6(c, at(c, S0, 0x8), c.g[A0][0]);
            c.g[A1][0] = 0x480000;
            sw_6(c, r32(c, S0), c.g[V1][0]);
            c.g[A1][0] = sx(at(c, A1, 0x4a8));
            sw_6(c, at(c, S0, 0x4), c.g[V0][0]);
            c.g[A0][0] = 0xc;
            sw_6(c, at(c, SP, 0x10), c.g[A2][0]);
            c.g[A3][0] = 0u64;
            c.g[RA][0] = sx(0x0025b974);
            c.g[A2][0] = 0x20000000;
            if call_ptr(c, 0x00317d70, 0x0025b974).is_err() {
                return;
            }
            c.g[A1][0] = c.g[V0][0];
            c.g[V0][0] = sx(at(c, A1, 0x8));
            if c.g[V0][0] != 0u64 {
                sw_6(c, at(c, A1, 0x8), c.g[S0][0]);
                break 'b_25b984;
            } else {
                break 'b_25b984;
            }
        }
        c.cycles += 9;
        c.g[V0][0] = lw(c, at(c, SP, 0x10));
        sw_6(c, r32(c, A1), c.g[V0][0]);
        c.g[V1][0] = lw(c, at(c, V0, 0x4));
        sw_6(c, at(c, A1, 0x4), c.g[V1][0]);
        c.g[A0][0] = lw(c, at(c, V0, 0x4));
        sw_6(c, r32(c, A0), c.g[A1][0]);
        c.g[V0][0] = lw(c, at(c, SP, 0x10));
        sw_6(c, at(c, V0, 0x4), c.g[A1][0]);
        sw_6(c, at(c, S2, 0x10), c.g[S3][0]);
        break 'b_25b9a8;
    }
    c.cycles += 8;
    sw_6(c, r32(c, SP), c.g[A1][0]);
    frame.restore(c, S0, 0x60);
    frame.restore(c, S1, 0x50);
    frame.restore(c, S2, 0x40);
    frame.restore(c, S3, 0x30);
    c.g[RA][0] = ld(c, at(c, SP, 0x20));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(at(c, SP, 0x70));
    c.pc = t;
    return;
}

/// `void func_00267128(sPtrVec* self, void** pos, void* const& x)` (0x267128):
/// insert `x` into the pointer vector before `pos`, growing the buffer to
/// twice its size (or 1) when it is full.
pub fn buddy_man_message_vector_insert(c: &mut Cpu) {
    let frame = Frame::open(c, 0x70);
    'b_2672a4: {
        'b_2671a4: {
            c.cycles += 14;
            frame.save(c, S1, 0x50);
            frame.save(c, S3, 0x30);
            c.g[S1][0] = c.g[A0][0];
            frame.save(c, S4, 0x20);
            c.g[S3][0] = c.g[A1][0];
            frame.save(c, S0, 0x60);
            frame.save(c, S2, 0x40);
            frame.save(c, S5, 0x10);
            sd(c, r32(c, SP), c.g[RA][0]);
            c.g[V1][0] = lw(c, at(c, S1, 0x8));
            c.g[V0][0] = lw(c, at(c, S1, 0xc));
            let t = c.g[V1][0] == c.g[V0][0];
            c.g[S4][0] = c.g[A2][0];
            if t {
                break 'b_2671a4;
            } else {
                'b_267174: {
                    c.cycles += 2;
                    if c.g[V1][0] == 0u64 {
                        c.g[A0][0] = lw(c, at(c, S1, 0x8));
                        break 'b_267174;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = lw(c, r32(c, V1).wrapping_sub(0x4));
                        sw_6(c, r32(c, V1), c.g[V0][0]);
                        c.g[A0][0] = lw(c, at(c, S1, 0x8));
                        break 'b_267174;
                    }
                }
                c.cycles += 12;
                c.g[A1][0] = c.g[S3][0];
                c.g[A2][0] = sx(r32(c, A0).wrapping_sub(0x4));
                c.g[V0][0] = sx(at(c, A0, 0x4));
                c.g[A2][0] = sx(r32(c, A2).wrapping_sub(r32(c, S3)));
                sw_6(c, at(c, S1, 0x8), c.g[V0][0]);
                c.g[A2][0] = sx((r32(c, A2) as i32 >> 2) as u32);
                c.g[A2][0] = sx(r32(c, A2) << 2);
                c.g[S0][0] = lw(c, r32(c, S4));
                c.g[RA][0] = sx(0x0026719c);
                c.g[A0][0] = sx(r32(c, A0).wrapping_sub(r32(c, A2)));
                if call_ptr(c, 0x0041610c, 0x0026719c).is_err() {
                    return;
                }
                let t = true;
                sw_6(c, r32(c, S3), c.g[S0][0]);
                if t {
                    break 'b_2672a4;
                } else {
                    break 'b_2671a4;
                }
            }
        }
        'b_26720c: {
            'b_2671ec: {
                c.cycles += 9;
                c.g[V0][0] = lw(c, at(c, S1, 0x4));
                c.g[S0][0] = 0x1;
                c.g[A0][0] = 0x1;
                c.g[V0][0] = sx(r32(c, V1).wrapping_sub(r32(c, V0)));
                c.g[V0][0] = sx((r32(c, V0) as i32 >> 2) as u32);
                c.g[V1][0] = sx(r32(c, V0) << 1);
                if c.g[V0][0] != 0 {
                    c.g[S0][0] = c.g[V1][0];
                }
                let t = c.g[S0][0] != c.g[A0][0];
                c.g[S0][0] = sx(r32(c, S0) << 2);
                if t {
                    break 'b_2671ec;
                } else {
                    c.cycles += 9;
                    c.g[A1][0] = 0x480000;
                    c.g[A0][0] = 0x4;
                    c.g[A1][0] = sx(at(c, A1, 0x4a8));
                    c.g[A2][0] = 0x20000000;
                    c.g[A3][0] = 0u64;
                    c.g[RA][0] = sx(0x002671e4);
                    c.g[S5][0] = 0x4;
                    if call_ptr(c, 0x00317d70, 0x002671e4).is_err() {
                        return;
                    }
                    let t = true;
                    c.g[S2][0] = c.g[V0][0];
                    if t {
                        break 'b_26720c;
                    } else {
                        break 'b_2671ec;
                    }
                }
            }
            c.cycles += 8;
            c.g[A1][0] = 0x480000;
            c.g[A1][0] = sx(at(c, A1, 0x4a8));
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = 0x20000000;
            c.g[RA][0] = sx(0x00267204);
            c.g[A3][0] = 0u64;
            if call_ptr(c, 0x00317e30, 0x00267204).is_err() {
                return;
            }
            c.g[S5][0] = c.g[S0][0];
            c.g[S2][0] = c.g[V0][0];
            break 'b_26720c;
        }
        'b_267238: {
            c.cycles += 9;
            c.g[A1][0] = lw(c, at(c, S1, 0x4));
            c.g[A0][0] = c.g[S2][0];
            c.g[V0][0] = sx(r32(c, S3).wrapping_sub(r32(c, A1)));
            c.g[A2][0] = sx((r32(c, V0) as i32 >> 2) as u32);
            c.g[S0][0] = sx(r32(c, S2).wrapping_add(r32(c, V0)));
            c.g[RA][0] = sx(0x00267228);
            c.g[A2][0] = sx(r32(c, A2) << 2);
            if call_ptr(c, 0x0041610c, 0x00267228).is_err() {
                return;
            }
            let t = c.g[S0][0] == 0u64;
            c.g[V1][0] = c.g[S0][0];
            if t {
                break 'b_267238;
            } else {
                c.cycles += 2;
                c.g[V0][0] = lw(c, r32(c, S4));
                sw_6(c, r32(c, S0), c.g[V0][0]);
                break 'b_267238;
            }
        }
        'b_267298: {
            'b_267294: {
                c.cycles += 14;
                c.g[A2][0] = lw(c, at(c, S1, 0x8));
                c.g[S0][0] = sx(at(c, V1, 0x4));
                c.g[A1][0] = c.g[S3][0];
                c.g[A0][0] = c.g[S0][0];
                c.g[A2][0] = sx(r32(c, A2).wrapping_sub(r32(c, S3)));
                c.g[S3][0] = sx(r32(c, S0).wrapping_add(r32(c, A2)));
                c.g[A2][0] = sx((r32(c, A2) as i32 >> 2) as u32);
                c.g[RA][0] = sx(0x0026725c);
                c.g[A2][0] = sx(r32(c, A2) << 2);
                if call_ptr(c, 0x0041610c, 0x0026725c).is_err() {
                    return;
                }
                c.g[A0][0] = lw(c, at(c, S1, 0x4));
                c.g[V0][0] = lw(c, at(c, S1, 0xc));
                c.g[V0][0] = sx(r32(c, V0).wrapping_sub(r32(c, A0)));
                let t = c.g[A0][0] == 0u64;
                c.g[V1][0] = sx((r32(c, V0) as i32 >> 2) as u32);
                if t {
                    break 'b_267294;
                } else {
                    'b_26728c: {
                        c.cycles += 3;
                        c.g[V0][0] = 0x1;
                        let t = c.g[V1][0] != c.g[V0][0];

                        if t {
                            break 'b_26728c;
                        } else {
                            c.cycles += 4;
                            c.g[RA][0] = sx(0x00267284);
                            if call_ptr(c, 0x00317e50, 0x00267284).is_err() {
                                return;
                            }
                            let t = true;
                            c.g[V0][0] = sx(r32(c, S2).wrapping_add(r32(c, S5)));
                            if t {
                                break 'b_267298;
                            } else {
                                break 'b_26728c;
                            }
                        }
                    }
                    c.cycles += 2;
                    c.g[RA][0] = sx(0x00267294);
                    if call_ptr(c, 0x00317e98, 0x00267294).is_err() {
                        return;
                    }
                    break 'b_267294;
                }
            }
            c.cycles += 1;
            c.g[V0][0] = sx(r32(c, S2).wrapping_add(r32(c, S5)));
            break 'b_267298;
        }
        c.cycles += 3;
        sw_6(c, at(c, S1, 0x8), c.g[S3][0]);
        sw_6(c, at(c, S1, 0xc), c.g[V0][0]);
        sw_6(c, at(c, S1, 0x4), c.g[S2][0]);
        break 'b_2672a4;
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

/// `void func_002672C8(sPtrVec* self, void** pos, void* const& x)` (0x2672c8):
/// the challenger list's insert, same shape as the message-list one.
pub fn buddy_man_challenger_vector_insert(c: &mut Cpu) {
    let frame = Frame::open(c, 0x70);
    'b_267444: {
        'b_267344: {
            c.cycles += 14;
            frame.save(c, S1, 0x50);
            frame.save(c, S3, 0x30);
            c.g[S1][0] = c.g[A0][0];
            frame.save(c, S4, 0x20);
            c.g[S3][0] = c.g[A1][0];
            frame.save(c, S0, 0x60);
            frame.save(c, S2, 0x40);
            frame.save(c, S5, 0x10);
            sd(c, r32(c, SP), c.g[RA][0]);
            c.g[V1][0] = lw(c, at(c, S1, 0x8));
            c.g[V0][0] = lw(c, at(c, S1, 0xc));
            let t = c.g[V1][0] == c.g[V0][0];
            c.g[S4][0] = c.g[A2][0];
            if t {
                break 'b_267344;
            } else {
                'b_267314: {
                    c.cycles += 2;
                    if c.g[V1][0] == 0u64 {
                        c.g[A0][0] = lw(c, at(c, S1, 0x8));
                        break 'b_267314;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = lw(c, r32(c, V1).wrapping_sub(0x4));
                        sw_6(c, r32(c, V1), c.g[V0][0]);
                        c.g[A0][0] = lw(c, at(c, S1, 0x8));
                        break 'b_267314;
                    }
                }
                c.cycles += 12;
                c.g[A1][0] = c.g[S3][0];
                c.g[A2][0] = sx(r32(c, A0).wrapping_sub(0x4));
                c.g[V0][0] = sx(at(c, A0, 0x4));
                c.g[A2][0] = sx(r32(c, A2).wrapping_sub(r32(c, S3)));
                sw_6(c, at(c, S1, 0x8), c.g[V0][0]);
                c.g[A2][0] = sx((r32(c, A2) as i32 >> 2) as u32);
                c.g[A2][0] = sx(r32(c, A2) << 2);
                c.g[S0][0] = lw(c, r32(c, S4));
                c.g[RA][0] = sx(0x0026733c);
                c.g[A0][0] = sx(r32(c, A0).wrapping_sub(r32(c, A2)));
                if call_ptr(c, 0x0041610c, 0x0026733c).is_err() {
                    return;
                }
                let t = true;
                sw_6(c, r32(c, S3), c.g[S0][0]);
                if t {
                    break 'b_267444;
                } else {
                    break 'b_267344;
                }
            }
        }
        'b_2673ac: {
            'b_26738c: {
                c.cycles += 9;
                c.g[V0][0] = lw(c, at(c, S1, 0x4));
                c.g[S0][0] = 0x1;
                c.g[A0][0] = 0x1;
                c.g[V0][0] = sx(r32(c, V1).wrapping_sub(r32(c, V0)));
                c.g[V0][0] = sx((r32(c, V0) as i32 >> 2) as u32);
                c.g[V1][0] = sx(r32(c, V0) << 1);
                if c.g[V0][0] != 0 {
                    c.g[S0][0] = c.g[V1][0];
                }
                let t = c.g[S0][0] != c.g[A0][0];
                c.g[S0][0] = sx(r32(c, S0) << 2);
                if t {
                    break 'b_26738c;
                } else {
                    c.cycles += 9;
                    c.g[A1][0] = 0x480000;
                    c.g[A0][0] = 0x4;
                    c.g[A1][0] = sx(at(c, A1, 0x4a8));
                    c.g[A2][0] = 0x20000000;
                    c.g[A3][0] = 0u64;
                    c.g[RA][0] = sx(0x00267384);
                    c.g[S5][0] = 0x4;
                    if call_ptr(c, 0x00317d70, 0x00267384).is_err() {
                        return;
                    }
                    let t = true;
                    c.g[S2][0] = c.g[V0][0];
                    if t {
                        break 'b_2673ac;
                    } else {
                        break 'b_26738c;
                    }
                }
            }
            c.cycles += 8;
            c.g[A1][0] = 0x480000;
            c.g[A1][0] = sx(at(c, A1, 0x4a8));
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = 0x20000000;
            c.g[RA][0] = sx(0x002673a4);
            c.g[A3][0] = 0u64;
            if call_ptr(c, 0x00317e30, 0x002673a4).is_err() {
                return;
            }
            c.g[S5][0] = c.g[S0][0];
            c.g[S2][0] = c.g[V0][0];
            break 'b_2673ac;
        }
        'b_2673d8: {
            c.cycles += 9;
            c.g[A1][0] = lw(c, at(c, S1, 0x4));
            c.g[A0][0] = c.g[S2][0];
            c.g[V0][0] = sx(r32(c, S3).wrapping_sub(r32(c, A1)));
            c.g[A2][0] = sx((r32(c, V0) as i32 >> 2) as u32);
            c.g[S0][0] = sx(r32(c, S2).wrapping_add(r32(c, V0)));
            c.g[RA][0] = sx(0x002673c8);
            c.g[A2][0] = sx(r32(c, A2) << 2);
            if call_ptr(c, 0x0041610c, 0x002673c8).is_err() {
                return;
            }
            let t = c.g[S0][0] == 0u64;
            c.g[V1][0] = c.g[S0][0];
            if t {
                break 'b_2673d8;
            } else {
                c.cycles += 2;
                c.g[V0][0] = lw(c, r32(c, S4));
                sw_6(c, r32(c, S0), c.g[V0][0]);
                break 'b_2673d8;
            }
        }
        'b_267438: {
            'b_267434: {
                c.cycles += 14;
                c.g[A2][0] = lw(c, at(c, S1, 0x8));
                c.g[S0][0] = sx(at(c, V1, 0x4));
                c.g[A1][0] = c.g[S3][0];
                c.g[A0][0] = c.g[S0][0];
                c.g[A2][0] = sx(r32(c, A2).wrapping_sub(r32(c, S3)));
                c.g[S3][0] = sx(r32(c, S0).wrapping_add(r32(c, A2)));
                c.g[A2][0] = sx((r32(c, A2) as i32 >> 2) as u32);
                c.g[RA][0] = sx(0x002673fc);
                c.g[A2][0] = sx(r32(c, A2) << 2);
                if call_ptr(c, 0x0041610c, 0x002673fc).is_err() {
                    return;
                }
                c.g[A0][0] = lw(c, at(c, S1, 0x4));
                c.g[V0][0] = lw(c, at(c, S1, 0xc));
                c.g[V0][0] = sx(r32(c, V0).wrapping_sub(r32(c, A0)));
                let t = c.g[A0][0] == 0u64;
                c.g[V1][0] = sx((r32(c, V0) as i32 >> 2) as u32);
                if t {
                    break 'b_267434;
                } else {
                    'b_26742c: {
                        c.cycles += 3;
                        c.g[V0][0] = 0x1;
                        let t = c.g[V1][0] != c.g[V0][0];

                        if t {
                            break 'b_26742c;
                        } else {
                            c.cycles += 4;
                            c.g[RA][0] = sx(0x00267424);
                            if call_ptr(c, 0x00317e50, 0x00267424).is_err() {
                                return;
                            }
                            let t = true;
                            c.g[V0][0] = sx(r32(c, S2).wrapping_add(r32(c, S5)));
                            if t {
                                break 'b_267438;
                            } else {
                                break 'b_26742c;
                            }
                        }
                    }
                    c.cycles += 2;
                    c.g[RA][0] = sx(0x00267434);
                    if call_ptr(c, 0x00317e98, 0x00267434).is_err() {
                        return;
                    }
                    break 'b_267434;
                }
            }
            c.cycles += 1;
            c.g[V0][0] = sx(r32(c, S2).wrapping_add(r32(c, S5)));
            break 'b_267438;
        }
        c.cycles += 3;
        sw_6(c, at(c, S1, 0x8), c.g[S3][0]);
        sw_6(c, at(c, S1, 0xc), c.g[V0][0]);
        sw_6(c, at(c, S1, 0x4), c.g[S2][0]);
        break 'b_267444;
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

/// `D_00534B30`: "player profile exists" flag.
pub(crate) const HAVE_PROFILE: u32 = 0x0053_4b30;

/// `sRiderEntry` table.
pub(crate) const RIDER_ENTRIES: u32 = 0x0053_5b20;

/// Tag names looked up in the lobby reply's tag list.
pub(crate) const TAG_A: u32 = 0x004a_3198;

pub(crate) const TAG_IDX: u32 = 0x004a_31a8;

pub(crate) const TAG_B: u32 = 0x004a_31a0;

/// `void LobbyMan_OnSnapshotReply(LobbyMan *self, Msg *msg)` (0x25e590): a
/// snapshot reply either carries the rider's score in three tag fields
/// (stored summed at `self + idx*4 + 0x104`) or a message to append to the
/// pending list at `self+0xf0`.
pub fn lobbyman_on_snapshot_reply(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x70);
        c.cycles += 10;
        frame.save(c, 16, 0x60);
        frame.save(c, 19, 0x30);
        let msg = abi::arg(c, 1);
        c.g[S0][0] = abi::arg64(c, 1);
        frame.save(c, 17, 0x50);
        frame.save_ra(c, 0x20);
        frame.save(c, 18, 0x40);
        c.g[S3][0] = abi::arg64(c, 0);
        c.g[S2][0] = c.lw(msg.wrapping_add(8));

        if c.g[S2][0] == 0 {
            c.cycles += 28;
            // a = TagFieldGetNumber(TagFieldFind(msg->tags, TAG_A), 0)
            c.g[A0][0] = c.lw(msg.wrapping_add(0xc));
            c.g[A1][0] = 0x004a_0000;
            c.g[A1][0] = sx(TAG_A);
            callee::TAG_FIELD_FIND.call(c, 0x0025_e5c8)?;
            c.g[A1][0] = 0;
            c.g[A0][0] = c.g[V0][0];
            callee::TAG_FIELD_GET_NUMBER.call(c, 0x0025_e5d4)?;
            c.g[S1][0] = c.g[V0][0];
            // idx = TagFieldGetNumber(TagFieldFind(msg->tags, TAG_IDX), 0)
            c.g[A0][0] = c.lw(msg.wrapping_add(0xc));
            c.g[A1][0] = sx(TAG_IDX);
            callee::TAG_FIELD_FIND.call(c, 0x0025_e5e8)?;
            c.g[A1][0] = 0;
            c.g[A0][0] = c.g[V0][0];
            callee::TAG_FIELD_GET_NUMBER.call(c, 0x0025_e5f4)?;
            c.g[S0][0] = c.g[V0][0];
            // b = TagFieldGetNumber(TagFieldFind(msg->tags, TAG_B), 0)
            c.g[A0][0] = c.lw(msg.wrapping_add(0xc));
            c.g[A1][0] = 0x004a_0000;
            c.g[A1][0] = sx(TAG_B);
            callee::TAG_FIELD_FIND.call(c, 0x0025_e608)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            callee::TAG_FIELD_GET_NUMBER.call(c, 0x0025_e614)?;
            c.g[S0][0] = sx((c.g[S0][0] as u32) << 2);
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[S0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[S0][0] as u32));
            c.sw((c.g[S0][0] as u32).wrapping_add(0x104), c.g[S1][0]);
            frame.restore(c, 16, 0x60);
            frame.restore(c, 17, 0x50);
            frame.restore(c, 18, 0x40);
            frame.restore(c, 19, 0x30);
            c.cycles += 7;
            frame.restore_ra(c, 0x20);
            frame.close(c);
            return Ok(());
        }

        // Non-zero: build a 0xc-byte message node and prepend it to the list.
        c.cycles += 25;
        c.g[A1][0] = 0x0048_0000;
        c.g[A3][0] = 0;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x488));
        c.g[A0][0] = 0xc;
        c.g[A2][0] = 0x2000_0000;
        callee::C_MEM_MAN_ALLOC.call(c, 0x0025_e640)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[A3][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xf0));
        c.g[V0][0] = 0x0048_0000;
        c.g[V1][0] = 0x102;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1290));
        c.sw(c.g[S1][0] as u32, c.g[V1][0]);
        c.sw((c.g[S1][0] as u32).wrapping_add(4), c.g[V0][0]);
        c.sw((c.g[S1][0] as u32).wrapping_add(8), c.g[S2][0]);
        c.g[A1][0] = 0x0048_0000;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4a8));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[A3][0]);
        c.g[A0][0] = 0xc;
        c.g[A2][0] = 0x2000_0000;
        c.g[A3][0] = 0;
        callee::C_MEM_MAN_ALLOC.call(c, 0x0025_e67c)?;
        c.g[A1][0] = c.g[V0][0];
        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(8));
        if c.g[V0][0] != 0 {
            c.sw((c.g[A1][0] as u32).wrapping_add(8), c.g[S1][0]);
        }

        // sMsgList_insert(head, node): splice the node in after the head.
        c.cycles += 9;
        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
        c.sw(c.g[A1][0] as u32, c.g[V0][0]);
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(4));
        c.sw((c.g[A1][0] as u32).wrapping_add(4), c.g[V1][0]);
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(4));
        c.sw(c.g[A0][0] as u32, c.g[A1][0]);
        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
        c.sw((c.g[V0][0] as u32).wrapping_add(4), c.g[A1][0]);
        c.sw(c.g[SP][0] as u32, c.g[A1][0]);

        frame.restore(c, 16, 0x60);
        frame.restore(c, 17, 0x50);
        frame.restore(c, 18, 0x40);
        frame.restore(c, 19, 0x30);
        c.cycles += 7;
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `void LobbyMan_EditAccount(LobbyMan *self, Msg *msg)` (0x25bf18): build the
/// account-edit screen's four message strings into a stack buffer and send
/// the "edit" request with the account id.
pub fn lobbyman_edit_account(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x270);
        c.g[A2][0] = 0x004a_0000;
        frame.save(c, 17, 0x250);
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x2f58));
        frame.save(c, 18, 0x240);
        c.g[S1][0] = abi::arg64(c, 1);
        frame.save(c, 16, 0x260);
        c.g[S2][0] = abi::arg64(c, 0);
        frame.save_ra(c, 0x230);
        let this = c.g[S2][0] as u32;
        let msg = c.g[S1][0] as u32;
        let buf = c.g[SP][0] as u32;

        c.g[A0][0] = sx(buf);
        c.g[A1][0] = 0x200;
        c.g[A3][0] = c.lw(this.wrapping_add(0x48));
        c.sb(buf, 0);
        callee::SNPRINTF.call(c, 0x0025_bf50)?;
        c.g[V1][0] = c.lw(msg.wrapping_add(0x44));
        let no_title = c.g[V1][0] == 0;
        c.g[A2][0] = 0x004a_0000;
        if !no_title {
            c.cycles += 11;
            c.g[A3][0] = c.lw(this.wrapping_add(0x4c));
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x3040));
            c.g[A0][0] = sx(buf);
            c.g[A1][0] = 0x200;
            callee::SNPRINTF.call(c, 0x0025_bf70)?;
            c.g[A2][0] = 0x004a_0000;
            c.g[A3][0] = c.lw(msg.wrapping_add(4));
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x30a0));
            c.g[A0][0] = sx(buf);
            c.g[A1][0] = 0x200;
            callee::SNPRINTF.call(c, 0x0025_bf88)?;
        }

        // Second string: the two "selected" markers.
        c.cycles += 3;
        c.g[V0][0] = c.lw(msg.wrapping_add(0x48));
        let none = c.g[V0][0] == 0;
        c.g[V0][0] = 0x59;
        if !none {
            c.cycles += 17;
            c.g[A0][0] = c.lw(msg.wrapping_add(0x38));
            c.g[V1][0] = c.lw(msg.wrapping_add(0x3c));
            c.g[A2][0] = 0x4e;
            if c.g[A0][0] != 0 {
                c.g[A2][0] = c.g[V0][0];
            }
            let line = c.g[SP][0] as u32 + 0x200;
            c.g[S0][0] = sx(line);
            c.g[A1][0] = 0x004a_0000;
            c.g[A3][0] = 0x4e;
            if c.g[V1][0] != 0 {
                c.g[A3][0] = c.g[V0][0];
            }
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3030));
            c.g[A0][0] = sx(line);
            callee::SPRINTF_6.call(c, 0x0025_bfc0)?;
            c.g[A2][0] = 0x004a_0000;
            c.g[A3][0] = c.g[S0][0];
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x3048));
            c.g[A0][0] = sx(buf);
            c.g[A1][0] = 0x200;
            callee::SNPRINTF.call(c, 0x0025_bfd8)?;
        }

        // Third string: the account id.
        c.cycles += 3;
        c.g[V0][0] = c.lw(msg.wrapping_add(0x4c));
        let no_id = c.g[V0][0] == 0;
        c.g[A0][0] = sx(buf);
        if !no_id {
            c.cycles += 5;
            c.g[A2][0] = 0x004a_0000;
            c.g[A3][0] = c.lw(msg.wrapping_add(0xc));
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x3050));
            c.g[A1][0] = 0x200;
            callee::SNPRINTF.call(c, 0x0025_bff8)?;
        }

        // Send the request and queue a 0xc-byte node on self+0xf0.
        c.cycles += 34;
        c.g[AT][0] = 0x41a0_0000;
        c.mtc1(1, 12); // f12 = 20.0
        c.g[T0][0] = 0x0027_0000;
        c.g[A2][0] = 0x6564_0000;
        c.g[A3][0] = c.g[SP][0];
        c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x7408));
        c.g[A1][0] = 5;
        c.g[A0][0] = c.g[S2][0];
        c.g[A2][0] = c.g[A2][0] | 0x6974;
        callee::LOBBYMAN_SEND_REQUEST.call(c, 0x0025_c020)?;

        c.g[S0][0] = 0xcd;
        c.g[A1][0] = 0x0048_0000;
        c.g[A0][0] = 8;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x488));
        c.g[A2][0] = 0x2000_0000;
        c.g[A3][0] = 0;
        callee::C_MEM_MAN_ALLOC.call(c, 0x0025_c03c)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[V1][0] = c.lw(this.wrapping_add(0xf0));
        c.g[V0][0] = 0x0048_0000;
        c.sw(c.g[S1][0] as u32, c.g[S0][0]);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x12b0));
        c.g[A1][0] = 0x0048_0000;
        c.sw((c.g[S1][0] as u32).wrapping_add(4), c.g[V0][0]);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4a8));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x220), c.g[V1][0]);
        c.g[A0][0] = 0xc;
        c.g[A2][0] = 0x2000_0000;
        c.g[A3][0] = 0;
        callee::C_MEM_MAN_ALLOC.call(c, 0x0025_c070)?;
        c.g[A1][0] = c.g[V0][0];
        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(8));
        if c.g[V0][0] != 0 {
            c.sw((c.g[A1][0] as u32).wrapping_add(8), c.g[S1][0]);
        }

        // Splice the node in after the list head.
        c.cycles += 15;
        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x220));
        frame.restore(c, 16, 0x260);
        c.sw(c.g[A1][0] as u32, c.g[V0][0]);
        frame.restore(c, 17, 0x250);
        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(4));
        frame.restore(c, 18, 0x240);
        c.sw((c.g[A1][0] as u32).wrapping_add(4), c.g[V1][0]);
        frame.restore_ra(c, 0x230);
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(4));
        c.sw(c.g[A0][0] as u32, c.g[A1][0]);
        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x220));
        c.sw((c.g[V0][0] as u32).wrapping_add(4), c.g[A1][0]);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x210), c.g[A1][0]);
        frame.close(c);
        Ok(())
    })
}

/// `void Net_DisconnectAll(void *self, void *arg)` (0x2636f0): tear down the
/// network: make sure the net manager exists, stop any session, clear the
/// pending-message list and post the "disconnected" notification.
pub fn net_disconnect_all(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x70);
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
        frame.save(c, 18, 0x40);
        frame.save(c, 19, 0x30);
        c.g[S2][0] = abi::arg64(c, 1);
        frame.save(c, 16, 0x60);
        c.g[S3][0] = abi::arg64(c, 0);
        frame.save(c, 17, 0x50);
        let have = c.g[V0][0] != 0;
        frame.save_ra(c, 0x20);
        if !have {
            c.cycles += 2;
            callee::NET_1.call(c, 0x0026_3720)?;
        }

        c.cycles += 4;
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
        callee::NET_2.call(c, 0x0026_3728)?;
        let running = c.g[V0][0] != 0;
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
        if !running {
            c.cycles += 3;
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
            callee::NET_3.call(c, 0x0026_3738)?;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
        }

        c.cycles += 2;
        let pending = c.g[V0][0] != 0;
        if !pending {
            c.cycles += 2;
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
            callee::NET_FLUSH.call(c, 0x0026_374c)?;
        }

        c.cycles += 9;
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
        callee::NET_4.call(c, 0x0026_3754)?;
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
        callee::NET_FLUSH.call(c, 0x0026_375c)?;
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x238));
        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x20));
        c.g[V0][0] = c.g[V0][0] ^ 1;
        let idle = c.g[V0][0] == 0;
        frame.restore(c, 16, 0x60);
        if !idle {
            c.cycles += 4;
            callee::NET_5.call(c, 0x0026_3778)?;
            let no_arg = c.g[S2][0] == 0;
            c.g[A1][0] = 0x0048_0000;
            if !no_arg {
                c.cycles += 25;
                c.g[S0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xed0));
                c.g[A0][0] = 0xc;
                c.g[A2][0] = 0x2000_0000;
                c.g[A3][0] = 0;
                callee::C_MEM_MAN_ALLOC.call(c, 0x0026_3798)?;
                c.g[S1][0] = c.g[V0][0];
                c.g[V1][0] = 0x0048_0000;
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1250));
                c.g[V0][0] = 0x116;
                c.sw(c.g[S1][0] as u32, c.g[V0][0]);
                c.g[A1][0] = 0x0048_0000;
                c.sw((c.g[S1][0] as u32).wrapping_add(4), c.g[V1][0]);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4a8));
                c.sw((c.g[S1][0] as u32).wrapping_add(8), c.g[S2][0]);
                c.g[A0][0] = 0xc;
                c.g[A2][0] = 0x2000_0000;
                c.g[A3][0] = 0;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xf0));
                c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[V0][0]);
                callee::C_MEM_MAN_ALLOC.call(c, 0x0026_37d4)?;
                c.g[A1][0] = c.g[V0][0];
                c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(8));
                if c.g[V0][0] != 0 {
                    c.sw((c.g[A1][0] as u32).wrapping_add(8), c.g[S1][0]);
                }
                c.cycles += 9;
                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                c.sw(c.g[A1][0] as u32, c.g[V0][0]);
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(4));
                c.sw((c.g[A1][0] as u32).wrapping_add(4), c.g[V1][0]);
                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(4));
                c.sw(c.g[A0][0] as u32, c.g[A1][0]);
                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                c.sw((c.g[V0][0] as u32).wrapping_add(4), c.g[A1][0]);
                c.sw(c.g[SP][0] as u32, c.g[A1][0]);
            }
            c.cycles += 2;
            c.sw(c.g[S3][0] as u32, 0);
            frame.restore(c, 16, 0x60);
        }

        c.cycles += 6;
        frame.restore(c, 17, 0x50);
        frame.restore(c, 18, 0x40);
        frame.restore(c, 19, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `void Net_AbortGameSetup(int *self, int arg)` (0x263ba8): drop the pending
/// game-setup session, stop VOIP and the setup manager, reset the net lobby
/// and post the "abort" message (0x10c for arg 1, 0x112 for arg 0).
pub fn net_abort_game_setup(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x50);
        frame.save(c, 16, 0x40);
        frame.save(c, 17, 0x30);
        c.g[S0][0] = abi::arg64(c, 0);
        frame.save_ra(c, 0x20);
        c.g[V0][0] = c.lw((c.g[S0][0] as u32));
        let no_session = c.g[V0][0] == 0;
        c.g[S1][0] = abi::arg64(c, 1);
        if !no_session {
            c.cycles += 5;
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x238));
            c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(4));
            callee::NET_FIND.call(c, 0x0026_3bd4)?;
            let missing = c.g[V0][0] == 0;
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x238));
            if !missing {
                c.cycles += 6;
                c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(4));
                callee::NET_REMOVE.call(c, 0x0026_3be4)?;
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x304));
                callee::VOIP_DEACTIVATE.call(c, 0x0026_3bec)?;
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x304));
                callee::VOIP_STOP.call(c, 0x0026_3bf4)?;
            }
        }

        c.cycles += 3;
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2dc));
        let no_setup = c.g[V0][0] == 0;
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
        if !no_setup {
            c.cycles += 3;
            callee::SETUPMAN_DESTROY.call(c, 0x0026_3c08)?;
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
        }

        c.cycles += 5;
        c.g[A1][0] = 0;
        callee::NET_RESET.call(c, 0x0026_3c14)?;
        c.g[V0][0] = 1;
        let other = c.g[S1][0] != c.g[V0][0];
        c.sw(c.g[S0][0] as u32, 0);
        if other {
            // Neither 0 nor 1: no message.
            c.cycles += 2;
            let queued = c.g[S1][0] != 0;
            frame.restore(c, 16, 0x40);
            if !queued {
                c.cycles += 9;
                c.g[A1][0] = 0x0048_0000;
                c.g[S0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x488));
                c.g[A0][0] = 8;
                c.g[A2][0] = 0x2000_0000;
                c.g[A3][0] = 0;
                callee::C_MEM_MAN_ALLOC.call(c, 0x0026_3c6c)?;
                c.g[S1][0] = c.g[V0][0];
                c.g[V1][0] = 0x112;
                post_node(c, 0x0026_3ca4)?;
            }
        } else {
            c.cycles += 10;
            c.g[A1][0] = 0x0048_0000;
            c.g[S0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x488));
            c.g[A0][0] = 8;
            c.g[A2][0] = 0x2000_0000;
            c.g[A3][0] = 0;
            callee::C_MEM_MAN_ALLOC.call(c, 0x0026_3c3c)?;
            c.g[S1][0] = c.g[V0][0];
            c.g[V1][0] = 0x10c;
            post_node(c, 0x0026_3ca4)?;
        }

        c.cycles += 4;
        frame.restore(c, 17, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// Post the 8-byte node `s1` (already holding its tag in `v1` and the class
/// pointer) onto the lobby message list at `s0+0xf0`, then restore `s0`.
pub(crate) fn post_node(c: &mut Cpu, ret: u32) -> crate::call::Flow {
    c.cycles += 16;
    c.g[V0][0] = 0x0048_0000;
    c.sw(c.g[S1][0] as u32, c.g[V1][0]);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x12b0));
    c.g[A1][0] = 0x0048_0000;
    c.sw((c.g[S1][0] as u32).wrapping_add(4), c.g[V0][0]);
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4a8));
    c.g[A0][0] = 0xc;
    c.g[A2][0] = 0x2000_0000;
    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xf0));
    c.g[A3][0] = 0;
    c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[V0][0]);
    callee::C_MEM_MAN_ALLOC.call(c, ret)?;
    c.g[A1][0] = c.g[V0][0];
    c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(8));
    if c.g[V0][0] != 0 {
        c.sw((c.g[A1][0] as u32).wrapping_add(8), c.g[S1][0]);
    }
    c.cycles += 10;
    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
    c.sw(c.g[A1][0] as u32, c.g[V0][0]);
    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(4));
    c.sw((c.g[A1][0] as u32).wrapping_add(4), c.g[V1][0]);
    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(4));
    c.sw(c.g[A0][0] as u32, c.g[A1][0]);
    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
    c.sw((c.g[V0][0] as u32).wrapping_add(4), c.g[A1][0]);
    c.sw(c.g[SP][0] as u32, c.g[A1][0]);
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x40));
    Ok(())
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    use crate::difftest::compare;
    use crate::difftest::elf_image;
    use crate::difftest::events::check_events;
    use crate::difftest::events::driven;
    use crate::difftest::events::schedule;
    use crate::difftest::events::Events;
    use crate::difftest::fakes;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::fake_call;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::ptr;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::fakes::LOG_RESULTS;
    use crate::difftest::fakes::LOG_UNWIND_AT;
    use crate::difftest::fakes::*;
    use crate::difftest::fill;
    use crate::difftest::put_vec;
    use crate::difftest::put_words;
    use crate::difftest::scramble;
    use crate::difftest::seed_count;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::RA as SENTINEL_RA;
    use crate::difftest::RA as TEST_RA;
    use crate::difftest::RA as RET;
    use crate::difftest::RA as SENTINEL;
    use crate::difftest::SP;
    use crate::difftest::SP as STACK;
    pub(crate) use crate::game::be::bedbprofile::tests::strict as strict_2;
    pub(crate) use crate::game::be::beintstat::tests::allocator;
    pub(crate) use crate::game::be::beintstat::tests::call;
    pub(crate) use crate::game::be::beintstat::tests::pool_allocator as pool_allocator_6;
    pub(crate) use crate::game::bx::bxstring::tests::heap;
    pub(crate) use crate::game::bx::bxstring::tests::pool as pool_4;
    pub(crate) use crate::game::bx::bxstring::tests::put_str_2 as put_str;
    pub(crate) use crate::game::bx::bxstring2::tests::alloc_base as alloc_base_2;
    pub(crate) use crate::game::bx::bxstring2::tests::alloc_mutex;
    pub(crate) use crate::game::bx::bxstring2::tests::base_3 as base_13;
    pub(crate) use crate::game::bx::bxstring2::tests::be_library;
    pub(crate) use crate::game::bx::bxstring2::tests::f;
    pub(crate) use crate::game::bx::bxstring2::tests::run as run_7;
    pub(crate) use crate::game::bx::bxstring2::tests::slot_2 as slot_7;
    pub(crate) use crate::game::bx::bxstring2::tests::BE_2 as BE_5;
    pub(crate) use crate::game::bx::bxstring2::tests::DB;
    pub(crate) use crate::game::bx::bxstring2::tests::MEM_MUTEX;
    pub(crate) use crate::game::bx::bxstring2::tests::RIDERS;
    pub(crate) use crate::game::bx::bxstring2::tests::RM;
    pub(crate) use crate::game::bx::bxstring2::tests::VTS;
    pub(crate) use crate::game::bx::bxstring2::tests::W_2 as W_3;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::tests::world_2 as world_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::alloc_base_3 as alloc_base;
    pub(crate) use crate::game::fe::feasyncfile::tests::alloc_base_setup;
    pub(crate) use crate::game::fe::feasyncfile::tests::b;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_11 as base_10;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_12 as base_11;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_13 as base_12;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_15 as base_14;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_16 as base_15;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_17 as base_16;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_6 as base_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_9 as base_8;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_4 as be_library_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_5 as be_library_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_7 as be_library_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_8 as be_library_7;
    pub(crate) use crate::game::fe::feasyncfile::tests::call_machine;
    pub(crate) use crate::game::fe::feasyncfile::tests::check_all;
    pub(crate) use crate::game::fe::feasyncfile::tests::diff;
    pub(crate) use crate::game::fe::feasyncfile::tests::eafile_pools;
    pub(crate) use crate::game::fe::feasyncfile::tests::env;
    pub(crate) use crate::game::fe::feasyncfile::tests::fake_small;
    pub(crate) use crate::game::fe::feasyncfile::tests::fe_base;
    pub(crate) use crate::game::fe::feasyncfile::tests::game_globals;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_10 as globals_9;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_11 as globals_10;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_12 as globals_11;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_4 as globals_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_5 as globals_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_6 as globals_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_9 as globals_8;
    pub(crate) use crate::game::fe::feasyncfile::tests::gpat;
    pub(crate) use crate::game::fe::feasyncfile::tests::h;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init as heap_init_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_10 as heap_init_11;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_12;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_5 as heap_init_7;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_8 as heap_init_9;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_9 as heap_init_10;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_10 as pool_11;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_4 as pool_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_7 as pool_8;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_8 as pool_9;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_9 as pool_10;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_allocator_10 as pool_allocator_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_allocator_5 as pool_allocator_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_allocator_6 as pool_allocator_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_allocator_9 as pool_allocator_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::put_str_6 as put_str_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::run as run_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::run_7 as run_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::run_8 as run_6;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_4 as slot_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_5 as slot_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_8 as slot_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_9 as slot_6;
    pub(crate) use crate::game::fe::feasyncfile::tests::strict;
    pub(crate) use crate::game::fe::feasyncfile::tests::string_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::string_4 as string_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::ui;
    pub(crate) use crate::game::fe::feasyncfile::tests::w32;
    pub(crate) use crate::game::fe::feasyncfile::tests::w_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::w_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::w_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::wb;
    pub(crate) use crate::game::fe::feasyncfile::tests::wh;
    pub(crate) use crate::game::fe::feasyncfile::tests::world;
    pub(crate) use crate::game::fe::feasyncfile::tests::world_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::world_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::world_5 as world_6;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_10 as BE_8;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_4 as BE_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_6 as BE_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_8 as BE_6;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE_9 as BE_7;
    pub(crate) use crate::game::fe::feasyncfile::tests::IFACE;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ2;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ2_3 as OBJ2_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ2_4 as OBJ2_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ3;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ4;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::RIDER;
    pub(crate) use crate::game::fe::feasyncfile::tests::RIDERS_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::RMAN;
    pub(crate) use crate::game::fe::feasyncfile::tests::SCRATCH as SCRATCH_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_11 as SEEDS_12;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_12 as SEEDS_13;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_13 as SEEDS_14;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_14 as SEEDS_15;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_16;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_8 as SEEDS_10;
    pub(crate) use crate::game::fe::feasyncfile::tests::WORLD_2 as WORLD;
    pub(crate) use crate::game::fe::feasyncfile::tests::WORLD_7 as WORLD_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::W_3 as W;
    pub(crate) use crate::game::fe::feasyncfile::tests::W_4 as W_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::W_7 as W_4;
    pub(crate) use crate::game::fe::fepopup3::tests::setup;
    pub(crate) use crate::game::fe::festaterewards::tests::base_4 as base_7;
    pub(crate) use crate::game::fe::festaterewards::tests::base_9 as base_17;
    pub(crate) use crate::game::fe::festaterewards::tests::globals_3 as globals_6;
    pub(crate) use crate::game::fe::festaterewards::tests::heap_init_3 as heap_init_8;
    pub(crate) use crate::game::fe::festaterewards::tests::hook;
    pub(crate) use crate::game::fe::festaterewards::tests::pool_4 as pool_6;
    pub(crate) use crate::game::fe::festaterewards::tests::slot_2 as slot_4;
    pub(crate) use crate::game::fe::festaterewards::tests::strict as strict_3;
    pub(crate) use crate::game::fe::festaterewards::tests::w16;
    pub(crate) use crate::game::fe::festaterewards::tests::BE_7 as BE_9;
    pub(crate) use crate::game::fe::festaterewards::tests::HOOKS;
    pub(crate) use crate::game::fe::festaterewards::tests::KEEP_F0;
    pub(crate) use crate::game::fe::festaterewards2::tests::belib;
    pub(crate) use crate::game::fe::festaterewards2::tests::w;
    pub(crate) use crate::game::fe::festaterewards2::tests::SEEDS as SEEDS_3;
    pub(crate) use crate::game::main::gamemode::tests::SEEDS as SEEDS_2;
    pub(crate) use crate::game::main::gamestate::tests::base_3 as base_5;
    pub(crate) use crate::game::main::gamestate::tests::SEEDS_5 as SEEDS_11;
    pub(crate) use crate::game::movie::movieplayer::tests::globals;
    pub(crate) use crate::game::movie::movieplayer::tests::BE_LIBRARY;
    pub(crate) use crate::game::object::railmodifier::tests::rng as rng_3;
    pub(crate) use crate::game::object::railmodifier::tests::SEEDS_7 as SEEDS_8;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::base as base_6;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::belib_2;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::clear;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::globals_4 as globals_5;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::gp_at;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::heap_setup;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::SEEDS_10 as SEEDS_9;
    pub(crate) use crate::game::scripter::ssxscriptengine::tests::SEEDS_4 as SEEDS_5;
    pub(crate) use crate::game::sound::bankmanager::tests::rng;
    pub(crate) use crate::game::sound::bankmanager::tests::SEEDS;
    pub(crate) use crate::game::sound::bankmonitor::tests::SEEDS_4;
    pub(crate) use crate::game::wscript::wscriptman::tests::base_6 as base_9;
    pub(crate) use crate::game::wscript::wscriptman::tests::clear_5 as clear_2;
    pub(crate) use crate::game::wscript::wscriptman::tests::globals_3 as globals_7;
    pub(crate) use crate::game::wscript::wscriptman::tests::BE_3;
    pub(crate) use crate::game::wscript::wscriptman::tests::GAME;
    pub(crate) use crate::game::wscript::wscriptman::tests::WORLD_3 as WORLD_2;
    use ssx3_ee as ee;
    use ssx3_ee_rt::funcs::GuestFn;

    #[test]
    fn lobby_man_get_language_matches() {
        check(
            ssx3_ee::s06::f_00259198,
            lobby_man_get_language,
            30,
            &|c, seed| {
                let mut r = Rng(0x2591_9800 ^ seed as u64);
                scramble(c, &mut r);
            },
        );
    }

    #[test]
    fn buddy_man_is_connected_matches() {
        check(
            ssx3_ee::s06::f_00261460,
            buddy_man_is_connected,
            300,
            &|c, seed| {
                let mut r = Rng(0x2614_6000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                // Cover: no session, "connecting" state, and connected/disconnected.
                let (session, state, conn) = match seed % 6 {
                    0 => (0, r.next_u32(), r.next_u32()),
                    1 => (1, 1, r.next_u32()),
                    2 => (1, 0, 0),
                    3 => (1, 2, r.next_u32() | 1),
                    4 => (1, 1, 0),
                    _ => (r.next_u32() | 1, r.next_u32(), r.next_u32()),
                };
                c.bus.write32(DATA + 0xc8, session);
                c.bus.write32(DATA + 0xd8, state);
                c.bus.write32(DATA + 0xe0, conn);
            },
        );
    }

    #[test]
    fn buddy_man_map_presence_state_matches() {
        check(
            ssx3_ee::s07::f_002628c0,
            buddy_man_map_presence_state,
            300,
            &|c, seed| {
                let mut r = Rng(0x0c13_0001 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = match seed % 6 {
                    0 => 0,
                    1 => 1,
                    2 => 2,
                    3 => 3,
                    4 => u64::MAX, // negative
                    _ => 5,        // out of range, high
                };
                // Cover present (non-zero) and absent.
                c.bus.write32(
                    DATA + 0xc8,
                    if seed % 7 == 0 { 0 } else { r.next_u32() | 1 },
                );
            },
        );
    }

    #[test]
    fn buddy_man_get_message_count_matches() {
        check(
            ssx3_ee::s07::f_00262938,
            buddy_man_get_message_count,
            200,
            &|c, seed| {
                let mut r = Rng(0x0c13_0002 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                let begin = DATA + 0x200;
                let end = match seed % 4 {
                    0 => begin.wrapping_sub(16), // negative count
                    1 => begin,                  // zero
                    _ => begin + 16,
                };
                c.bus.write32(DATA + 0xac, begin);
                c.bus.write32(DATA + 0xb0, end);
            },
        );
    }

    #[test]
    fn buddy_man_get_challenger_matches() {
        check(
            ssx3_ee::s07::f_00263308,
            buddy_man_get_challenger,
            300,
            &|c, seed| {
                let mut r = Rng(0x0c13_0003 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                let begin = DATA + 0x200;
                let end = if seed % 5 == 0 {
                    begin.wrapping_sub(8)
                } else {
                    begin + 16
                };
                c.bus.write32(DATA + 0xbc, begin);
                c.bus.write32(DATA + 0xc0, end);
                c.g[A1][0] = match seed % 8 {
                    0..=3 => (seed % 4) as u64, // in range when count > 0
                    4 => 4,                     // first out-of-range slot
                    5 => 0xffff_ffff,           // wraps the base by -4
                    6 => 0,
                    _ => u64::MAX,
                };
                for i in 0..8 {
                    c.bus.write32(begin.wrapping_sub(8) + i * 4, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn buddy_man_get_message_matches() {
        check(
            ssx3_ee::s07::f_00263630,
            buddy_man_get_message,
            300,
            &|c, seed| {
                let mut r = Rng(0x0c13_0004 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                let begin = DATA + 0x200;
                let end = if seed % 5 == 0 {
                    begin.wrapping_sub(8)
                } else {
                    begin + 16
                };
                c.bus.write32(DATA + 0xac, begin);
                c.bus.write32(DATA + 0xb0, end);
                c.g[A1][0] = match seed % 8 {
                    0..=3 => (seed % 4) as u64,
                    4 => 4,
                    5 => 0xffff_ffff,
                    6 => 0,
                    _ => u64::MAX,
                };
                for i in 0..8 {
                    c.bus.write32(begin.wrapping_sub(8) + i * 4, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn buddy_man_map_presence_state_matches_2() {
        check(
            ssx3_ee::s07::f_002628c0,
            buddy_man_map_presence_state_2,
            300,
            &|c, seed| {
                let mut r = Rng(0x2628 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = match seed % 7 {
                    0 => 0xffff_ffff_ffff_ffff,
                    n => (n - 1) as u64,
                };
                c.bus
                    .write32(DATA + 0xc8, if seed % 4 == 0 { 0 } else { r.next_u32() });
            },
        );
    }

    #[test]
    fn c_shared_login_man_do_finish_matches() {
        check(
            ee::s07::f_00265f00__generated,
            c_shared_login_man_do_finish,
            SEEDS,
            &|c, seed| {
                let mut r = rng(0x0026_5f00, seed);
                scramble(c, &mut r);
                c.g[GP][0] = sx(GAME_GP);
                c.g[A0][0] = sx(DATA);
                c.bus.write32(DATA, r.next_u32());
                c.bus.write32(DATA + 4, r.next_u32());
                c.bus.write32(0x004a_33e0, r.next_u32());
            },
        );
    }

    #[test]
    fn c_setup_man_free_packet_matches() {
        check(
            ee::s07::f_00264990__generated,
            c_setup_man_free_packet,
            100,
            &|c, seed| {
                machine(c, seed, 0x26_4990, 0);
                // `delete(NULL)` is the fast path through the allocator.
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = 0;
            },
        );
    }

    #[test]
    fn shared_login_man_shutdown_matches() {
        crate::difftest::check(
            ee::s07::f_002659c0__generated,
            shared_login_man_shutdown,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2659_c0, 2);
                be_library(c);
                c.g[A0][0] = sx(DATA);
                // The memory-card object (`gp + 0x2e8`): null skips the destroy.
                c.bus.write32(
                    GAME_GP + 0x2e8,
                    if seed % 2 == 0 { 0 } else { DATA + 0x800 },
                );
                c.bus.write32(DATA, r.next_u32());
            },
        );
    }

    #[test]
    fn shared_login_man_mc_cb_state09_matches() {
        crate::difftest::check(
            ee::s07::f_00266010__generated,
            shared_login_man_mc_cb_state09,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2660_10, 2);
                c.g[A0][0] = (seed % 10) as u64;
                c.bus
                    .write32(GAME_GP + 0x1aec, if seed % 2 == 0 { 0 } else { 1 });
                c.bus.write32(GAME_GP + 0x2e4, r.next_u32());
            },
        );
    }

    #[test]
    fn shared_login_man_mc_cb_state0a_matches() {
        crate::difftest::check(
            ee::s07::f_00266038__generated,
            shared_login_man_mc_cb_state0a,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2660_38, 2);
                c.g[A0][0] = (seed % 10) as u64;
                c.bus
                    .write32(GAME_GP + 0x1aec, if seed % 2 == 0 { 0 } else { 1 });
                c.bus.write32(GAME_GP + 0x2e4, r.next_u32());
            },
        );
    }

    #[test]
    fn shared_login_man_mc_cb_state03_matches() {
        crate::difftest::check(
            ee::s07::f_002660a0__generated,
            shared_login_man_mc_cb_state03,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2660_a0, 2);
                c.g[A0][0] = (seed % 10) as u64;
                c.bus
                    .write32(GAME_GP + 0x1aec, if seed % 2 == 0 { 0 } else { 1 });
                c.bus.write32(GAME_GP + 0x2e4, r.next_u32());
            },
        );
    }

    #[test]
    fn tib_ext_mem_free_matches() {
        crate::difftest::check(
            ee::s07::f_00266760__generated,
            tib_ext_mem_free,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2667_60, 2);
                be_library(c);
                c.g[A0][0] = sx(r.next_u32());
                c.g[A1][0] = if seed % 2 == 0 {
                    0
                } else {
                    sx(DATA + (r.next_u32() & 0xfff0))
                };
            },
        );
    }

    pub(crate) fn rng_2(seed: u32, salt: u64) -> Rng {
        Rng(0x9e37_79b9_7f4a_7c15 ^ (seed as u64) << 8 ^ salt)
    }

    // ---------------------------------------------------------------- lobby

    #[test]
    fn lobby_man_accept_challenge_matches() {
        check(
            ssx3_ee::s06::f_0025fb58__generated,
            lobby_man_accept_challenge,
            60,
            &|c, seed| {
                let mut r = rng_2(seed, 0x27);
                scramble(c, &mut r);
                heap_init(c);
                let lobby = DATA + 0x4000;
                let packet = DATA + 0x6000;
                // The lobby is "connected" only for some seeds.
                c.bus.write32(lobby, (seed % 2) as u32);
                // No lobby API: the private-message send returns -1 at once.
                c.bus.write32(lobby + 0x54, 0);
                c.bus.write32(lobby + 0xf0, packet);
                c.bus.write32(packet, 0);
                c.bus.write32(packet + 4, DATA + 0x7000);
                c.g[A0][0] = sx(lobby);
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = r.next_u32() as u64;
            },
        );
    }

    // ------------------------------------------------------------ singletons

    #[test]
    fn c_setup_man_destroy_matches() {
        check(
            ssx3_ee::s07::f_00264098__generated,
            c_setup_man_destroy,
            60,
            &|c, seed| {
                let mut r = rng_2(seed, 0x28);
                scramble(c, &mut r);
                heap_init(c);
                let manager = if seed % 3 == 0 {
                    0
                } else {
                    heap_block(c, 0x120)
                };
                c.bus.write32(GAME_GP + 0x2dc, manager);
            },
        );
    }

    #[test]
    fn c_shared_login_man_destroy_matches() {
        check(
            ssx3_ee::s07::f_002658b8__generated,
            c_shared_login_man_destroy,
            60,
            &|c, seed| {
                let mut r = rng_2(seed, 0x29);
                scramble(c, &mut r);
                heap_init(c);
                // No memory card and no SLogin store: shutdown takes its "not
                // initialised" early outs. The store-error mapper reads a game
                // jump table, so give it the entry for "not initialised" (2).
                const SLOGIN_STORE_ERRORS: u32 = 0x0049_7150;
                c.bus.write32(SLOGIN_STORE_ERRORS + 8, 0x003f_81dc);
                c.bus.write32(GAME_GP + 0x2e8, 0);
                c.bus.write32(GAME_GP + 0x1ad4, 0);
                c.bus.write32(GAME_GP + 0x2a30, 0);
                let manager = if seed % 3 == 0 {
                    0
                } else {
                    heap_block(c, 0x120)
                };
                c.bus.write32(GAME_GP + 0x2e0, manager);
            },
        );
    }

    /// The memory-card request wrappers: `gp_2+0x2e8` is the card. The state
    /// machine dispatches through the table at 0x4860c0; the request states
    /// (0x43, 0x45, 0x47, 0x49) all take its common "request" arm, whose only
    /// work is `DoGetInfo` when `+0x40` is clear — keep it set.
    pub(crate) fn mc_request_setup(c: &mut Cpu, seed: u32, salt: u64) {
        let mut r = machine(c, seed, salt, 0);
        const STATE_TABLE: u32 = 0x0048_60c0;
        const REQUEST_ARM: u32 = 0x002c_50ac;
        for state in [0x43u32, 0x45, 0x47, 0x49] {
            c.bus.write32(STATE_TABLE + (state - 1) * 4, REQUEST_ARM);
        }
        let card = DATA + 0x4000;
        c.bus.write32(card + 0x40, 1);
        c.bus.write32(card + 0x4, 0);
        c.bus.write32(
            card + 0x18,
            if seed % 2 == 0 {
                u32::MAX
            } else {
                r.next_u32()
            },
        );
        c.bus.write32(GAME_GP + 0x2e8, card);
    }

    #[test]
    fn c_shared_login_man_mc_close_matches() {
        check(
            ssx3_ee::s07::f_00266148__generated,
            c_shared_login_man_mc_close,
            60,
            &|c, seed| mc_request_setup(c, seed, 0x2a),
        );
    }

    #[test]
    fn c_shared_login_man_mc_read_matches() {
        check(
            ssx3_ee::s07::f_00266298__generated,
            c_shared_login_man_mc_read,
            60,
            &|c, seed| mc_request_setup(c, seed, 0x2b),
        );
    }

    #[test]
    fn c_shared_login_man_mc_write_matches() {
        check(
            ssx3_ee::s07::f_002662c0__generated,
            c_shared_login_man_mc_write,
            60,
            &|c, seed| mc_request_setup(c, seed, 0x2c),
        );
    }

    #[test]
    fn c_shared_login_man_mc_flush_matches() {
        check(
            ssx3_ee::s07::f_00266320__generated,
            c_shared_login_man_mc_flush,
            60,
            &|c, seed| mc_request_setup(c, seed, 0x2d),
        );
    }

    #[test]
    fn c_shared_login_man_mc_get_found_name_matches() {
        check(
            ssx3_ee::s07::f_00266550__generated,
            c_shared_login_man_mc_get_found_name,
            150,
            &|c, seed| {
                let mut r = rng_2(seed, 0x2e);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                let src = 0x0053_7190;
                let len = (seed % 12) as u32;
                for i in 0..len {
                    c.bus.write8(src + i, b'a' + (r.next_u32() % 26) as u8);
                }
                c.bus.write8(src + len, 0);
                c.g[A0][0] = sx(DATA + 0x5000);
            },
        );
    }

    #[test]
    fn c_voip_man_destroy_matches() {
        check(
            ssx3_ee::s07::f_00266cd8__generated,
            c_voip_man_destroy,
            60,
            &|c, seed| {
                let mut r = rng_2(seed, 0x2f);
                scramble(c, &mut r);
                heap_init(c);
                let manager = if seed % 3 == 0 {
                    0
                } else {
                    heap_block(c, 0x80)
                };
                if manager != 0 {
                    c.bus.write32(manager, 0);
                }
                c.bus.write32(GAME_GP + 0x304, manager);
            },
        );
    }

    // ------------------------------------------------------------- fixtures

    /// The main heap's dlmalloc arena over 256 KiB of test RAM, as
    /// `containers.rs` sets up for its allocator tests.
    pub(crate) const MEM_HEAP: u32 = 0x004f_f250;

    pub(crate) fn run(c: &mut Cpu, addr: u32, f: fn(&mut Cpu), args: &[u64]) -> u64 {
        for (i, a) in args.iter().enumerate() {
            c.g[[A0, A1, A2, A3, T0, T1][i]][0] = *a;
        }
        c.g[RA][0] = crate::difftest::RA as u64;
        c.pc = addr;
        f(c);
        assert_eq!(
            c.pc,
            crate::difftest::RA,
            "setup call {addr:#x} did not return"
        );
        c.g[V0][0]
    }

    pub(crate) fn heap_init(c: &mut Cpu) {
        c.g[GP][0] = GAME_GP as u64;
        run(
            c,
            0x0031_ff60,
            ssx3_ee::s10::DLHeap_InitArena,
            &[MEM_HEAP as u64, (DATA + 0x40000) as u64, 0x40000],
        );
    }

    pub(crate) fn heap_block(c: &mut Cpu, size: u32) -> u32 {
        heap_init(c);
        run(
            c,
            0x0032_0058,
            ssx3_ee::s10::MEM_HeapMemalign,
            &[MEM_HEAP as u64, 16, size as u64],
        ) as u32
    }

    #[test]
    fn lobby_man_get_room_name_matches() {
        check(
            ssx3_ee::s06::f_0025f150__generated,
            lobby_man_get_room_name,
            60,
            &|c, seed| {
                let _r = machine(c, seed, 0xc0, 1);
                globals(c);
                c.g[A0][0] = sx(DATA);
                // Lobby not up (early return) / up with a null session object (the
                // name lookup returns null and is offset).
                c.bus.write32(DATA, if seed % 2 == 0 { 0 } else { 1 });
            },
        );
    }

    #[test]
    fn c_setup_man_alloc_packet_matches() {
        check(
            ssx3_ee::s07::f_00264960__generated,
            c_setup_man_alloc_packet,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0xc1, 6);
                globals(c);
                alloc_mutex(c, &mut r);
            },
        );
    }

    /// The three shared-login card-slot getters.
    pub(crate) fn card_slot_test(generated: GuestFn, hand: GuestFn, salt: u64) {
        check(generated, hand, 40, &|c, seed| {
            let _r = machine(c, seed, salt, 1);
            globals(c);
            c.g[A0][0] = sx(DATA);
            // The login manager at gp + 0x2e8, its vtable at +0.
            c.bus.write32(
                GAME_GP + 0x2e8,
                if seed % 2 == 0 { DATA + 0x3000 } else { 0 },
            );
        });
    }

    #[test]
    fn c_shared_login_man_card_slot20_matches() {
        card_slot_test(
            ssx3_ee::s07::f_00265a38__generated,
            c_shared_login_man_card_slot20,
            0xc2,
        );
    }

    #[test]
    fn c_shared_login_man_card_slot47_matches() {
        card_slot_test(
            ssx3_ee::s07::f_00265d08__generated,
            c_shared_login_man_card_slot47,
            0xc3,
        );
    }

    #[test]
    fn c_shared_login_man_card_slot63_matches() {
        card_slot_test(
            ssx3_ee::s07::f_00265d38__generated,
            c_shared_login_man_card_slot63,
            0xc4,
        );
    }

    #[test]
    fn c_voip_man_get_status_matches() {
        check(
            ssx3_ee::s07::f_00266f90__generated,
            c_voip_man_get_status,
            60,
            &|c, seed| {
                let _r = machine(c, seed, 0xc5, 1);
                globals(c);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x100);
                c.g[A2][0] = sx(DATA + 0x200);
                // Voip not up: the RPC is skipped (its path needs the network
                // stack); the argument registers still have to be forwarded.
                c.bus.write32(DATA, 0);
            },
        );
    }

    /// Scratch area for the object fixtures (the tests own the data area).
    pub(crate) const SCRATCH: u32 = DATA + 0x1000;

    // ------------------------------------------------------------ destructors

    /// The deleting-destructor fixture: `this` is a data pointer (the
    /// allocator tolerates it, as in its own test) and `flags % 3` runs both
    /// the plain and the deleting path.
    pub(crate) fn dtor_setup(c: &mut Cpu, seed: u32, salt: u64) {
        let mut r = machine(c, seed, salt, 6);
        c.g[A0][0] = if seed % 8 == 0 {
            0
        } else {
            (DATA + 0x1000 + (r.next_u32() & 0x7_fff0)) as u64
        };
        c.g[A1][0] = (seed % 3) as u64;
    }

    #[test]
    fn fe_net_hook_dtor_matches() {
        check(
            ee::s07::f_00263678__generated,
            fe_net_hook_dtor,
            60,
            &|c, seed| dtor_setup(c, seed, 0x263678),
        );
    }

    #[test]
    fn fe_net_hook2_dtor_matches() {
        check(
            ee::s07::f_002639b0__generated,
            fe_net_hook2_dtor,
            60,
            &|c, seed| dtor_setup(c, seed, 0x2639b0),
        );
    }

    #[test]
    fn net_event_dtor_matches() {
        check(
            ee::s07::f_00267e18__generated,
            net_event_dtor,
            60,
            &|c, seed| dtor_setup(c, seed, 0x267e18),
        );
    }

    #[test]
    fn net_event102_dtor_matches() {
        check(
            ee::s07::f_00267e98__generated,
            net_event102_dtor,
            60,
            &|c, seed| dtor_setup(c, seed, 0x267e98),
        );
    }

    #[test]
    fn net_event102_base_dtor_matches() {
        check(
            ee::s07::f_00267f28__generated,
            net_event102_base_dtor,
            60,
            &|c, seed| dtor_setup(c, seed, 0x267f28),
        );
    }

    #[test]
    fn net_error_event_dtor_matches() {
        check(
            ee::s07::f_00267fb8__generated,
            net_error_event_dtor,
            60,
            &|c, seed| dtor_setup(c, seed, 0x267fb8),
        );
    }

    #[test]
    fn net_event102_tooy_dtor_matches() {
        check(
            ee::s07::f_00268210__generated,
            net_event102_tooy_dtor,
            60,
            &|c, seed| dtor_setup(c, seed, 0x268210),
        );
    }

    #[test]
    fn net_event_fb_dtor_matches() {
        check(
            ee::s07::f_00268658__generated,
            net_event_fb_dtor,
            60,
            &|c, seed| dtor_setup(c, seed, 0x268658),
        );
    }

    #[test]
    fn net_event_fc_dtor_matches() {
        check(
            ee::s07::f_002686f0__generated,
            net_event_fc_dtor,
            60,
            &|c, seed| dtor_setup(c, seed, 0x2686f0),
        );
    }

    #[test]
    fn net_event10d_dtor_matches() {
        check(
            ee::s07::f_00268790__generated,
            net_event10d_dtor,
            60,
            &|c, seed| dtor_setup(c, seed, 0x268790),
        );
    }

    // ---------------------------------------------------------------- lobby

    /// A lobby object at `SCRATCH`: the session word at `+0`, a `Lobby_List`
    /// at `+0x1000` with `count` entries whose objects carry random flags.
    pub(crate) fn lobby_object(c: &mut Cpu, seed: u32, salt: u64, list_offsets: &[u32]) {
        let mut r = machine(c, seed, salt, 1);
        c.g[A0][0] = SCRATCH as u64;
        // Half the seeds see a live session, half a dead one.
        c.bus.write32(SCRATCH, (seed % 2 == 0) as u32);
        // Room index for `Lobby_ListGetAt`.
        c.g[A1][0] = (seed % 4) as u64;
        let list = SCRATCH + 0x1000;
        for &off in list_offsets {
            c.bus.write32(SCRATCH + off, list);
        }
        c.bus.write32(list + 0x14, r.next_u32() % 32);
        c.bus.write32(list + 0x18, 4);
        c.bus.write32(list + 0x40, SCRATCH + 0x1800);
        for i in 0..4u32 {
            let room = SCRATCH + 0x2000 + 0x100 * i;
            c.bus.write32(SCRATCH + 0x1800 + 4 * i, room);
            c.bus.write16(room + 0xa, r.next_u32() as u16);
            c.bus.write32(room, r.next_u32());
        }
    }

    #[test]
    fn lobby_man_get_kick_reason_matches() {
        check(
            ee::s06::f_0025da90__generated,
            lobby_man_get_kick_reason,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x25da90, 1);
                c.g[A0][0] = SCRATCH as u64;
                c.bus.write32(SCRATCH + 0x54, (seed % 3) as u32);
                let reason = SCRATCH + 0x200;
                match seed % 4 {
                    0 => c.bus.write32(SCRATCH + 0x84, 0),
                    1 => {
                        c.bus.write32(SCRATCH + 0x84, reason);
                        c.bus.write8(reason, 0);
                    }
                    _ => {
                        c.bus.write32(SCRATCH + 0x84, reason);
                        c.bus.write8(reason, (r.next_u32() | 1) as u8);
                    }
                }
            },
        );
    }

    #[test]
    fn lobby_man_get_room_count_matches() {
        check(
            ee::s06::f_0025f120__generated,
            lobby_man_get_room_count,
            120,
            &|c, seed| {
                lobby_object(c, seed, 0x25f120, &[0x5c]);
            },
        );
    }

    #[test]
    fn lobby_man_is_room_flagged_matches() {
        check(
            ee::s06::f_0025f180__generated,
            lobby_man_is_room_flagged,
            120,
            &|c, seed| {
                lobby_object(c, seed, 0x25f180, &[0x5c]);
            },
        );
    }

    #[test]
    fn lobby_man_get_room_user_count_matches() {
        check(
            ee::s06::f_0025f1b0__generated,
            lobby_man_get_room_user_count,
            120,
            &|c, seed| {
                lobby_object(c, seed, 0x25f1b0, &[0x60]);
            },
        );
    }

    // ---------------------------------------------------------------- buddy

    /// A `BuddyMan` at `SCRATCH` with a two-entry roster at `+0xcc`; the name
    /// `bob` in `a1` matches entry 1 only for `seed % 4 == 1`. Variants: no
    /// roster (3), empty roster (0), match (1) and no match (2).
    pub(crate) fn buddy_object(c: &mut Cpu, seed: u32, salt: u64) {
        let this = SCRATCH;
        let mut r = machine(c, seed, salt, 8);
        c.g[A0][0] = this as u64;
        let name = SCRATCH + 0x4000;
        c.g[A1][0] = name as u64;
        let variant = seed % 4;
        for (i, byte) in b"bob\0".iter().enumerate() {
            c.bus.write8(name + i as u32, *byte);
        }
        for i in 0..2u32 {
            let entry = SCRATCH + 0x2000 + 0x400 * i;
            let bytes: &[u8; 4] = if variant == 1 && i == 1 {
                b"bob\0"
            } else if i == 0 {
                b"amy\0"
            } else {
                b"zoe\0"
            };
            for (j, byte) in bytes.iter().enumerate() {
                c.bus.write8(entry + j as u32, *byte);
            }
            // The product tag read by `BuddyMan_IsSameProductAt` (empty) and
            // the buddy flags at +0xa8 / the online byte at +0xa7.
            c.bus.write8(entry + 0x54, 0);
            c.bus.write32(entry + 0xa8, r.next_u32());
            c.bus.write8(entry + 0xa7, (r.next_u32() & 1) as u8);
            c.bus.write32(SCRATCH + 0x1800 + 4 * i, entry);
        }
        // The message window `[+0xac, +0xb0)` is empty and the product
        // string is a one-character name.
        c.bus.write32(this + 0xac, 0);
        c.bus.write32(this + 0xb0, 0);
        let product = SCRATCH + 0x6000;
        c.bus.write8(product, b'x');
        c.bus.write8(product + 1, 0);
        c.bus.write32(this + 0xec, product);
        if variant == 3 {
            c.bus.write32(this + BUDDY_ROSTER, 0);
        } else {
            let list = SCRATCH + 0x1000;
            c.bus.write32(this + BUDDY_ROSTER, list);
            c.bus.write32(list + 0x18, if variant == 0 { 0 } else { 2 });
            c.bus.write32(list + 0x40, SCRATCH + 0x1800);
        }
    }

    #[test]
    fn buddy_man_is_buddy_matches() {
        check(
            ee::s07::f_00262360__generated,
            buddy_man_is_buddy,
            100,
            &|c, seed| buddy_object(c, seed, 0x262360),
        );
    }

    #[test]
    fn buddy_man_is_blocked_matches() {
        check(
            ee::s07::f_002623b0__generated,
            buddy_man_is_blocked,
            100,
            &|c, seed| buddy_object(c, seed, 0x2623b0),
        );
    }

    #[test]
    fn buddy_man_is_temp_matches() {
        check(
            ee::s07::f_00262438__generated,
            buddy_man_is_temp,
            100,
            &|c, seed| buddy_object(c, seed, 0x262438),
        );
    }

    #[test]
    fn buddy_man_get_roster_count_matches() {
        check(
            ee::s07::f_00262598__generated,
            buddy_man_get_roster_count,
            100,
            &|c, seed| buddy_object(c, seed, 0x262598),
        );
    }

    #[test]
    fn buddy_man_count_messages_from_matches() {
        check(
            ee::s07::f_00262a08__generated,
            buddy_man_count_messages_from,
            100,
            &|c, seed| buddy_object(c, seed, 0x262a08),
        );
    }

    #[test]
    fn buddy_man_is_online_matches() {
        check(
            ee::s07::f_002633c8__generated,
            buddy_man_is_online,
            100,
            &|c, seed| buddy_object(c, seed, 0x2633c8),
        );
    }

    #[test]
    fn buddy_man_is_same_product_matches() {
        check(
            ee::s07::f_00263490__generated,
            buddy_man_is_same_product,
            100,
            &|c, seed| buddy_object(c, seed, 0x263490),
        );
    }

    // ---------------------------------------------------------- login manager

    #[test]
    fn c_shared_login_man_is_card_missing_matches() {
        check(
            ee::s07::f_00265d68__generated,
            c_shared_login_man_is_card_missing,
            100,
            &|c, seed| {
                let mut r = machine(c, seed, 0x265d68, 1);
                let manager = SCRATCH;
                let vt = SCRATCH + 0x200;
                c.bus.write32(crate::difftest::GAME_GP + 0x2e8, manager); // D_004A33D8
                c.bus.write32(manager, vt);
                // The `+0x1a8` slot: a signed this-delta and the method pointer,
                // which lands in the fake-call hook.
                c.bus.write16(vt + 0x1a8, (r.next_u32() & 0x7f0) as u16);
                c.bus.write32(vt + 0x1ac, fakes::FAKE);
            },
        );
    }

    // ------------------------------------------------------------------
    // Buddy list.

    #[test]
    fn buddy_man_is_temp_at_matches() {
        check(
            ssx3_ee::s07::f_00262400__generated,
            buddy_man_is_temp_at,
            SEEDS_2,
            &|c, seed| {
                let mut r = machine(c, seed, 0x262400, 0);
                let buddy = DATA + 0x100;
                let list = DATA + 0x2000;
                let node = DATA + 0x3000;
                let entry = DATA + 0x4000;
                c.bus.write32(buddy + 0xcc, list);
                c.bus
                    .write32(list + 0x18, [0u32, 1, 1, 1][seed as usize % 4]);
                c.bus.write32(list + 0x40, DATA + 0x5000);
                c.bus.write32(DATA + 0x5000, node);
                c.bus.write32(node, entry);
                c.bus.write32(entry + 0xa8, r.next_u32());
                let idx: [u64; 4] = [0, u64::MAX, 1, 0];
                c.g[A0][0] = sx(buddy);
                c.g[A1][0] = idx[seed as usize % 4];
            },
        );
    }

    #[test]
    fn shared_login_do_delete_matches() {
        check(
            ssx3_ee::s07::f_00265ec8__generated,
            shared_login_do_delete,
            SEEDS_2,
            &|c, seed| {
                let mut r = machine(c, seed, 0x265ec8, 0);
                let self_ = DATA + 0x100;
                fill(c, &mut r, self_, 0x20);
                c.g[A0][0] = sx(self_);
            },
        );
    }

    /// The engine's size-class pools (`MEM_PoolAlloc`), copied from `oc68`:
    /// allocations up to 0x200 bytes come from the per-class free lists hung
    /// off `gp + 0x2a60..`, which only work when the "pools ready" flag at
    /// `gp + 0xdd8` is set (otherwise the pool allocator recurses forever).
    pub(crate) fn pool_allocator(c: &mut Cpu) {
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
        for (i, (slot, head, block)) in POOLS.into_iter().enumerate() {
            let pool = DATA + 0x20_000 + 0x100 * i as u32;
            let base = nodes;
            for n in 0..16u32 {
                let node = base + block * n;
                c.bus.write32(node, if n == 15 { 0 } else { node + block });
            }
            c.bus.write32(pool + head, base);
            c.bus.write32(GAME_GP + slot, pool);
            nodes += block * 16;
        }
    }

    pub(crate) fn run_2(c: &mut Cpu, addr: u32, f: fn(&mut Cpu), args: &[u64]) -> u64 {
        for (i, a) in args.iter().enumerate() {
            c.g[[A0, A1, A2, A3, T0, T1][i]][0] = *a;
        }
        c.g[RA][0] = SENTINEL_RA as u64;
        c.pc = addr;
        f(c);
        assert_eq!(c.pc, SENTINEL_RA, "setup call {addr:#x} did not return");
        c.g[V0][0]
    }

    pub(crate) fn heap_init_2(c: &mut Cpu) {
        c.g[GP][0] = GAME_GP as u64;
        run_2(
            c,
            0x0031_ff60,
            ee::s10::DLHeap_InitArena,
            &[MEM_HEAP as u64, (DATA + 0x40000) as u64, 0x40000],
        );
    }

    pub(crate) fn alloc_machine(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let r = machine(c, seed, salt, 0);
        pool_allocator(c);
        heap_init_2(c);
        r
    }

    pub(crate) fn zero(c: &mut Cpu, at: u32, bytes: u32) {
        for i in (0..bytes).step_by(4) {
            c.bus.write32(at + i, 0);
        }
    }

    // ----------------------------------------------------------- networking

    #[test]
    fn lobby_man_destroy_peer_table_matches() {
        check(
            ee::s06::f_0025a6f8__generated,
            lobby_man_destroy_peer_table,
            60,
            &|c, seed| {
                let _ = machine(c, seed, 0x25a6_f8, 0);
                let self_ = DATA + 0x400;
                zero(c, self_, 0x200);
                zero(c, DATA + 0x600, 0x100);
                c.g[A0][0] = sx(self_);
                let table = if seed % 2 == 0 { 0 } else { DATA + 0x600 };
                c.bus.write32(self_ + 0xf4, table);
            },
        );
    }

    #[test]
    fn csetup_man_send_hello_matches() {
        check(
            ee::s07::f_00264cb8__generated,
            csetup_man_send_hello,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x264c_b8, 0);
                let self_ = DATA + 0x400;
                zero(c, self_, 0x200);
                zero(c, DATA + 0x600, 0x100);
                c.g[A0][0] = sx(self_);
                c.bus.write32(GAME_GP.wrapping_sub(0xc8), DATA + 0x600);
                c.bus.write32(DATA + 0x600 + 0xb8, r.next_u32());
            },
        );
    }

    #[test]
    fn tib_ext_mem_alloc_matches() {
        check(
            ee::s07::f_00266728__generated,
            tib_ext_mem_alloc,
            60,
            &|c, seed| {
                let mut r = alloc_machine(c, seed, 0x2667_28);
                c.g[A1][0] = sx(seed % 0x100);
                c.g[A2][0] = sx(if seed % 2 == 0 {
                    0x20
                } else {
                    r.next_u32() & 0xff
                });
            },
        );
    }

    #[test]
    fn cvoip_man_create_matches() {
        check(
            ee::s07::f_00266ca0__generated,
            cvoip_man_create,
            40,
            &|c, seed| {
                let _ = alloc_machine(c, seed, 0x266c_a0);
            },
        );
    }

    #[test]
    fn cvoip_man_stop_matches() {
        check(
            ee::s07::f_00266df8__generated,
            cvoip_man_stop,
            60,
            &|c, seed| {
                let _ = machine(c, seed, 0x266d_f8, 0);
                let self_ = DATA + 0x400;
                zero(c, self_, 0x100);
                c.g[A0][0] = sx(self_);
                c.bus.write32(self_, if seed % 2 == 0 { 0 } else { 1 });
            },
        );
    }

    #[test]
    fn cvoip_man_set_volume_matches() {
        check(
            ee::s07::f_00266f58__generated,
            cvoip_man_set_volume,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x266f_58, 0);
                let self_ = DATA + 0x400;
                zero(c, self_, 0x100);
                c.g[A0][0] = sx(self_);
                c.bus.write32(self_, if seed % 2 == 0 { 0 } else { 1 });
                c.g[A1][0] = sx(seed % 100);
                c.g[A2][0] = sx(r.next_u32() % 100);
            },
        );
    }

    // ---------------------------------------------------------- login / usb

    #[test]
    fn shared_login_man_do_update_matches() {
        check(
            ssx3_ee::s07::f_00265e40__generated,
            shared_login_man_do_update,
            SEEDS_3,
            &|c, seed| {
                machine(c, seed, 0x265e_40, 1);
                belib(c);
                c.g[A0][0] = DATA as u64;
            },
        );
    }

    #[test]
    fn usb_keyboard_on_connect_matches() {
        check(
            ssx3_ee::s07::f_002668a8__generated,
            usb_keyboard_on_connect,
            60,
            &|c, seed| {
                machine(c, seed, 0x2668_a8, 3);
                belib(c);
                c.g[A0][0] = DATA as u64;
            },
        );
    }

    // ------------------------------------------------------------------
    // Lobby / buddies / login.

    #[test]
    fn lobby_man_is_room_user_flagged_matches() {
        check(
            ee::s06::f_0025f248__generated,
            lobby_man_is_room_user_flagged,
            SEEDS_4,
            &|c, seed| {
                let mut r = machine(c, seed, 0x25f248, 0);
                let self_at = DATA + 0x200;
                let list = DATA + 0x300;
                let arr = DATA + 0x400;
                let item = DATA + 0x500;
                let mode = seed % 4;
                c.g[A0][0] = self_at as u64;
                c.g[A1][0] = 0; // room-user index
                c.bus.write32(self_at, (mode != 0) as u32);
                c.bus
                    .write32(self_at + 0x60, if mode == 1 { 0 } else { list });
                c.bus.write32(list + 0x18, if mode == 2 { 0 } else { 1 });
                c.bus.write32(list + 0x40, arr);
                c.bus.write32(arr, item);
                c.bus.write32(item, 1);
                c.bus.write32(item + 4, r.next_u32());
            },
        );
    }

    #[test]
    fn buddy_man_reconnect_matches() {
        check(
            ee::s06::f_002613c8__generated,
            buddy_man_reconnect,
            SEEDS_4,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2613c8, 0);
                let self_at = DATA + 0x200;
                let mode = seed % 3;
                c.g[A0][0] = self_at as u64;
                c.bus.write32(
                    self_at + 0x40,
                    if mode == 0 {
                        0xffff_ffff
                    } else {
                        (seed % 4) as u32
                    },
                );
                c.bus.write32(self_at + 0xd8, (mode <= 1) as u32);
                c.bus.write32(self_at + 0xc8, 1); // the buddy API already exists
                c.bus.write32(self_at + 0xec, r.next_u32());
            },
        );
    }

    #[test]
    fn buddy_man_roster_compare_matches() {
        check(
            ee::s07::f_002634c0__generated,
            buddy_man_roster_compare,
            SEEDS_4,
            &|c, seed| {
                let _ = machine(c, seed, 0x2634c0, 0);
                let a = DATA + 0x200;
                let b = DATA + 0x300;
                c.bus.write_bytes(a, b"alpha\0");
                let rhs: &[u8] = match seed % 5 {
                    0 => &b"alpha\0"[..],
                    1 => &b"ALPHA\0"[..],
                    2 => &b"beta\0"[..],
                    3 => &b"\0"[..],
                    _ => &b"alphan\0"[..],
                };
                c.bus.write_bytes(b, rhs);
                let (pa, pb) = match seed % 6 {
                    0 => (0u32, 0u32),
                    1 => (0, b),
                    2 => (a, 0),
                    _ => (a, b),
                };
                c.g[A2][0] = pa as u64;
                c.g[A3][0] = pb as u64;
            },
        );
    }

    #[test]
    fn c_shared_login_man_mc_get_info_matches() {
        check(
            ee::s07::f_00266060__generated,
            c_shared_login_man_mc_get_info,
            SEEDS_4,
            &|c, seed| {
                let _ = machine(c, seed, 0x266060, 0);
                c.g[A0][0] = (seed % 5) as u64;
                c.bus.write32(GAME_GP + 0x2e8, DATA + 0x400);
            },
        );
    }

    /// `BELibrary_GetInstance`'s fast path: a non-null singleton, so the game
    /// never lazily allocates it and reads the rewards file off the disc.
    pub(crate) fn be_library_2(c: &mut Cpu) {
        c.bus.write32(GAME_GP.wrapping_sub(0x1f04), DATA + 0x3_0000);
    }

    /// `DLHeap_InitArena(&MEM_HEAP, arena, size)`: the game heap over a
    /// scratch arena. The replay restore/free path allocates and frees
    /// through it, and a zeroed heap sends `DLHeap_Consolidate` into a walk
    /// over garbage.
    pub(crate) fn heap_init_3(c: &mut Cpu) {
        const MEM_HEAP: u32 = 0x004f_f250;
        const ARENA: u32 = DATA + 0x4_0000;
        c.g[GP][0] = GAME_GP as u64;
        c.g[A0][0] = MEM_HEAP as u64;
        c.g[A1][0] = ARENA as u64;
        c.g[A2][0] = 0x4_0000;
        c.g[RA][0] = crate::difftest::RA as u64;
        c.pc = 0x0031_ff60;
        ssx3_ee::s10::DLHeap_InitArena(c);
        assert_eq!(c.pc, crate::difftest::RA, "DLHeap_InitArena did not return");
    }

    #[test]
    fn lobby_man_lookup_peer_addr_matches() {
        check(
            ssx3_ee::s06::f_0025a730__generated,
            lobby_man_lookup_peer_addr,
            40,
            &|c, seed| {
                let _ = machine(c, seed, 0x25a730, 0);
                be_library_2(c);
                let this_ = DATA + 0x100;
                let tags = DATA + 0x2000;
                if seed % 2 == 0 {
                    c.bus.write32(this_ + 0xf4, 0);
                } else {
                    c.bus.write32(this_ + 0xf4, tags);
                }
                c.g[A0][0] = sx(this_);
                c.g[A1][0] = (seed % 32) as u64;
                c.g[A2][0] = sx(DATA + 0x1800);
            },
        );
    }

    #[test]
    fn lobby_man_destroy_lobby_api_matches() {
        check(
            ssx3_ee::s06::f_0025aa98__generated,
            lobby_man_destroy_lobby_api,
            60,
            &|c, seed| {
                let _ = machine(c, seed, 0x25aa98, 0);
                be_library_2(c);
                let this_ = DATA + 0x100;
                // A null lobby API takes the short path; a dummy (zeroed) one runs
                // the request teardown, which walks its empty buckets.
                c.bus
                    .write32(this_ + 0x54, if seed % 2 == 0 { 0 } else { DATA + 0x2000 });
                c.g[A0][0] = sx(this_);
            },
        );
    }

    #[test]
    fn lobby_man_send_request_matches() {
        check(
            ssx3_ee::s06::f_0025b608__generated,
            lobby_man_send_request,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x25b608, 0);
                be_library_2(c);
                let this_ = DATA + 0x100;
                // A null lobby API: the request add returns -1 at once.
                c.bus.write32(this_ + 0x54, 0);
                c.g[A0][0] = sx(this_);
                c.g[A1][0] = (seed % 8) as u64;
                c.g[A2][0] = sx(DATA + 0x1800);
                c.g[A3][0] = sx(DATA + 0x1c00);
                c.g[T0][0] = sx(DATA + 0x1c40);
                c.f[12] = r.f32(1000.0);
            },
        );
    }

    #[test]
    fn lobby_man_disconnect_matches() {
        check(
            ssx3_ee::s06::f_0025b800__generated,
            lobby_man_disconnect,
            60,
            &|c, seed| {
                let _ = machine(c, seed, 0x25b800, 0);
                be_library_2(c);
                let this_ = DATA + 0x100;
                c.bus
                    .write32(this_ + 0x54, if seed % 2 == 0 { 0 } else { DATA + 0x2000 });
                c.g[A0][0] = sx(this_);
            },
        );
    }

    #[test]
    fn shared_login_man_dtor_matches() {
        check(
            ssx3_ee::s07::f_00265908__generated,
            shared_login_man_dtor,
            60,
            &|c, seed| {
                let _ = machine(c, seed, 0x265908, 0);
                be_library_2(c);
                let this_ = DATA + 0x100;
                c.g[A0][0] = sx(this_);
                c.g[A1][0] = (seed & 1) as u64;
            },
        );
    }

    #[test]
    fn shared_login_man_do_create_matches() {
        check(
            ssx3_ee::s07::f_00265e80__generated,
            shared_login_man_do_create,
            40,
            &|c, seed| {
                let _ = machine(c, seed, 0x265e80, 0);
                be_library_2(c);
                c.g[A0][0] = sx(DATA + 0x100);
            },
        );
    }

    #[test]
    fn voip_man_dtor_matches() {
        check(
            ssx3_ee::s07::f_00266d10__generated,
            voip_man_dtor,
            60,
            &|c, seed| {
                let _ = machine(c, seed, 0x266d10, 0);
                be_library_2(c);
                c.g[A0][0] = sx(DATA + 0x100);
                c.g[A1][0] = (seed & 1) as u64;
            },
        );
    }

    #[test]
    fn voip_man_deactivate_matches() {
        check(
            ssx3_ee::s07::f_00266e88__generated,
            voip_man_deactivate,
            60,
            &|c, seed| {
                let _ = machine(c, seed, 0x266e88, 0);
                be_library_2(c);
                let this_ = DATA + 0x100;
                c.bus.write32(this_, if seed % 2 == 0 { 0 } else { 1 });
                c.bus.write32(this_ + 4, if seed % 4 < 2 { 0 } else { 1 });
                c.g[A0][0] = sx(this_);
            },
        );
    }

    pub(crate) fn put_str8(c: &mut Cpu, at: u32, s: &[u8]) {
        for (i, b) in s.iter().enumerate() {
            c.bus.write8(at + i as u32, *b);
        }
    }

    #[test]
    fn lobby_man_is_in_room_matches() {
        check(
            ee::s06::f_0025f430__generated,
            lobby_man_is_in_room,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x25f4_30, 1);
                let this = DATA + 0x100;
                c.g[A0][0] = sx(this);
                c.bus.write32(this, if seed % 4 == 0 { 0 } else { 1 });
                let room = DATA + 0x400;
                c.bus.write32(this + 0x54, room);
                // The room's name (`room + 0x1c0`) is empty or not.
                c.bus
                    .write8(room + 0x1c0, if seed % 3 == 0 { 0u8 } else { b'r' });
                c.bus.write32(room + 0x2cc, r.next_u32());
            },
        );
    }

    #[test]
    fn lobby_man_reset_chal_state_matches() {
        check(
            ee::s06::f_0025f858__generated,
            lobby_man_reset_chal_state,
            40,
            &|c, seed| {
                let mut r = call_machine(c, seed, 0x25f8_58, 1);
                let this = DATA + 0x100;
                c.g[A0][0] = sx(this);
                c.bus.write32(this, r.next_u32());
                let room = DATA + 0x400;
                c.bus.write32(this + 0x54, room);
                put_str8(c, room + 0x1c0, b"chal\0");
            },
        );
    }

    #[test]
    fn buddy_man_create_api_matches() {
        check(
            ee::s06::f_002614e8__generated,
            buddy_man_create_api,
            60,
            &|c, seed| {
                let mut r = call_machine(c, seed, 0x2614_e8, 2);
                let this = DATA + 0x100;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(DATA + 0x200);
                c.g[A2][0] = sx(r.next_u32() & 0xffff);
                c.bus.write32(DATA + 0x200, r.next_u32());
                c.bus.write32(this + 0xc8, 0);
            },
        );
    }

    #[test]
    fn net_start_connect_matches() {
        check(
            ee::s07::f_002636a8__generated,
            net_start_connect,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2636_a8, 1);
                let this = DATA + 0x100;
                c.g[A0][0] = sx(this);
                c.bus.write32(GAME_GP - 0x238, DATA + 0x1000);
                c.bus.write32(DATA + 0x1000 + 0x2c, seed as u32 % 2);
                c.bus.write32(GAME_GP - 0x1e4, r.next_u32());
                c.bus.write32(GAME_GP - 0x1e0, r.next_u32());
            },
        );
    }

    #[test]
    fn buddy_man_destroy_matches() {
        check(
            ee::s06::f_00261008__generated,
            buddy_man_destroy,
            SEEDS_5,
            &|c, seed| {
                let _ = machine(c, seed, 0x2610_08, 2);
                let buddy = DATA + 0x100;
                clear(c, buddy, 0x120);
                c.bus
                    .write32(gp_at(0x238), if seed % 2 == 0 { 0 } else { buddy });
            },
        );
    }

    #[test]
    fn buddy_man_update_matches() {
        check(
            ee::s06::f_00261490__generated,
            buddy_man_update,
            SEEDS_5,
            &|c, seed| {
                let _ = machine(c, seed, 0x2614_90, 2);
                let this = DATA + 0x100;
                clear(c, this, 0x120);
                c.g[A0][0] = sx(this);
                // +0xc8 is the login object the update drives; keep it null
                // (the login update allocates and talks to the network).
            },
        );
    }

    #[test]
    fn c_shared_login_man_set_account_name_matches() {
        check(
            ee::s07::f_00265f18__generated,
            c_shared_login_man_set_account_name,
            SEEDS_5,
            &|c, seed| {
                let _ = machine(c, seed, 0x265f_18, 0);
                clear(c, 0x0053_6740, 0x40);
                let name = DATA + 0x100;
                clear(c, name, 0x20);
                c.bus.write8(name, b'A' + (seed % 26) as u8);
                c.bus.write8(name + 1, b'a' + (seed % 26) as u8);
                c.bus.write8(name + 2, 0);
                c.g[A0][0] = sx(DATA + 0x200);
                c.g[A1][0] = sx(name);
            },
        );
    }

    #[test]
    fn c_shared_login_man_set_account_password_matches() {
        check(
            ee::s07::f_00265f68__generated,
            c_shared_login_man_set_account_password,
            SEEDS_5,
            &|c, seed| {
                let _ = machine(c, seed, 0x265f_68, 0);
                clear(c, 0x0053_6740, 0x40);
                let name = DATA + 0x100;
                clear(c, name, 0x20);
                c.bus.write8(name, b'0' + (seed % 10) as u8);
                c.bus.write8(name + 1, 0);
                c.g[A0][0] = sx(DATA + 0x200);
                c.g[A1][0] = sx(name);
            },
        );
    }

    /// Seeds per test (deep front-end callees run fewer).
    pub(crate) const SEEDS_6: u32 = 100;

    /// A real generated function that returns 0: a vtable slot for the
    /// "nothing handled" path.
    pub(crate) const ZERO_FN: u32 = 0x003c_51e0;

    /// Scrambled registers, the game's `gp_8`, the fake-call hooks and a zeroed
    /// BELibrary dummy.
    pub(crate) fn base(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let mut r = machine(c, seed, salt, 0);
        c.g[GP][0] = GAME_GP as u64;
        c.bus.write32(gp_at(-0x1f04), DATA + 0x3_e000);
        r
    }

    /// A vtable at `at` whose entries are `{delta, fake}` for each slot offset.
    pub(crate) fn put_vtable(c: &mut Cpu, r: &mut Rng, at: u32, slots: &[u32]) -> u32 {
        for (i, &off) in slots.iter().enumerate() {
            let delta = (r.next_u32() as i16 as i32 & !3) as u32;
            c.bus.write16(at + off, delta as u16);
            c.bus.write32(at + off + 4, FAKE + 0x10 * i as u32);
        }
        at
    }

    /// The main heap (0x4ff250) over 256 KiB of test RAM, so `EA_MemAllocNamed`
    /// has a real arena.
    pub(crate) fn put_arena(c: &mut Cpu) {
        c.g[GP][0] = GAME_GP as u64;
        c.g[A0][0] = 0x004f_f250;
        c.g[A1][0] = (DATA + 0x6_0000) as u64;
        c.g[A2][0] = 0x4_0000;
        c.g[RA][0] = crate::difftest::RA as u64;
        c.pc = 0x0031_ff60;
        ssx3_ee::s10::DLHeap_InitArena(c);
        assert_eq!(
            c.pc,
            crate::difftest::RA as u32,
            "arena init did not return"
        );
    }

    // ------------------------------------------------------------- lobby

    #[test]
    fn lobby_man_destroy_matches() {
        check(
            ee::s06::f_0025b688__generated,
            lobby_man_destroy,
            24,
            &|c, seed| {
                let _r = base(c, seed, 0x25b6);
                let lobby = if seed % 3 == 0 { 0 } else { DATA + 0x1000 };
                c.bus.write32(gp_at(-0xc8), lobby);
                c.bus
                    .write32(gp_at(-0x204), if seed % 2 == 0 { 0 } else { DATA + 0x3000 });
                c.g[A0][0] = sx(DATA);
            },
        );
    }

    #[test]
    fn csetup_man_cancel_character_matches() {
        check(
            ee::s07::f_00265140__generated,
            csetup_man_cancel_character,
            24,
            &|c, seed| {
                let mut r = base(c, seed, 0x2651);
                let s = DATA;
                let state = match seed % 4 {
                    0 => 0xc,
                    1 => 4,
                    _ => r.next_u32() % 16,
                };
                c.bus.write32(s, state);
                c.bus.write32(s + 0x124, if seed % 3 == 0 { 0 } else { 1 });
                c.g[A0][0] = sx(s);
                c.g[A1][0] = sx(r.next_u32() % 8);
            },
        );
    }

    #[test]
    fn cshared_login_man_request_card_info_if_idle_matches() {
        check(
            ee::s07::f_002659e8__generated,
            cshared_login_man_request_card_info_if_idle,
            80,
            &|c, seed| {
                let mut r = base(c, seed, 0x2659);
                let obj = DATA + 0x1000;
                let vt = put_vtable(c, &mut r, DATA + 0x2000, &[0x28, 0x58]);
                if seed % 2 == 0 {
                    c.bus.write16(vt + 0x58, 0);
                    c.bus.write32(vt + 0x5c, FAKE);
                } else {
                    c.bus.write16(vt + 0x58, 0);
                    c.bus.write32(vt + 0x5c, ZERO_FN);
                }
                c.bus.write32(vt + 0x28, 0);
                c.bus.write32(vt + 0x2c, FAKE + 0x20);
                c.bus.write32(obj, vt);
                c.bus.write32(gp_at(0x2e8), obj);
                c.bus.write32(gp_at(0x2ec), r.next_u32());
                fill(c, &mut r, obj + 4, 0x100);
                c.g[A0][0] = sx(DATA);
            },
        );
    }

    // -------------------------------------------------------------- network

    #[test]
    fn net_event_clone_matches() {
        check(
            ee::s07::f_00267e48__generated,
            net_event_clone,
            40,
            &|c, seed| {
                let mut r = base(c, seed, 0x267e);
                put_arena(c);
                c.bus.write32(DATA, r.next_u32());
                fill(c, &mut r, DATA + 4, 16);
                c.g[A0][0] = sx(DATA);
            },
        );
    }

    pub(crate) const SEEDS_7: u32 = 100;

    /// Call a generated helper as a setup step (arguments in a0..t1).
    pub(crate) fn load(c: &mut Cpu, addr: u32, f: GuestFn, args: &[u64]) -> u64 {
        for (i, a) in args.iter().enumerate() {
            c.g[[A0, A1, A2, A3, T0, T1][i]][0] = *a;
        }
        c.g[RA][0] = TEST_RA as u64;
        c.pc = addr;
        f(c);
        assert_eq!(c.pc, TEST_RA, "setup call {addr:#x} did not return");
        c.g[V0][0]
    }

    /// The main heap's arena over 256 KiB of test RAM, so constructors and
    /// destructors that allocate or free take their normal path.
    pub(crate) fn alloc_ready(c: &mut Cpu) {
        const MEM_HEAP: u32 = 0x004f_f250;
        c.g[GP][0] = GAME_GP as u64;
        load(
            c,
            0x0031_ff60,
            ssx3_ee::s10::f_0031ff60,
            &[MEM_HEAP as u64, (DATA + 0x4_0000) as u64, 0x4_0000],
        );
        c.bus.write32(GAME_GP + 0xdd8, 1);
    }

    /// A block from the test heap (`alloc_ready` must have run first). Used for
    /// objects whose destructor frees them (a static test pointer would send
    /// the heap into its consolidate path).
    pub(crate) fn heap_block_2(c: &mut Cpu, size: u32) -> u32 {
        const MEM_HEAP: u32 = 0x004f_f250;
        load(
            c,
            0x0032_0058,
            ssx3_ee::s10::f_00320058,
            &[MEM_HEAP as u64, 16, size as u64],
        ) as u32
    }

    /// The dummy world at `world_at`: `+0x84` the game module, its `+0xc` the
    /// race manager. Returns the module address.
    pub(crate) fn dummy_world(c: &mut Cpu, world_at: u32) -> u32 {
        let module = DATA + 0x9000;
        let race = DATA + 0x9400;
        clear(c, world_at, 0x40);
        clear(c, module, 0x100);
        clear(c, race, 0x100);
        c.bus.write32(world_at + 0x84, module);
        c.bus.write32(module + 0xc, race);
        c.bus.write32(gp_at(-0x848), world_at);
        module
    }

    /// The usual globals: a game-like `gp_9`, the BELibrary singleton dummy and
    /// a small world.
    pub(crate) fn common(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        let mut r = machine(c, seed, salt, calls);
        c.bus.write32(gp_at(-0x1f04), DATA + 0xb000);
        dummy_world(c, DATA + 0x8000);
        clear(c, DATA + 0xb000, 0x100);
        r
    }

    #[test]
    fn buddy_man_disconnect_matches() {
        check(
            ee::s06::f_00261408__generated,
            buddy_man_disconnect,
            SEEDS_7,
            &|c, seed| {
                let _ = common(c, seed, 0x261408, 0);
                alloc_ready(c);
                let this = DATA + 0x100;
                clear(c, this, 0x200);
                c.g[A0][0] = sx(this);
            },
        );
    }

    #[test]
    fn buddy_man_get_roster_entry_matches() {
        check(
            ee::s07::f_002625c8__generated,
            buddy_man_get_roster_entry,
            SEEDS_7,
            &|c, seed| {
                let _ = common(c, seed, 0x2625c8, 1);
                let this = DATA + 0x100;
                clear(c, this, 0x200);
                let list = DATA + 0x2000;
                clear(c, list, 0x100);
                c.bus.write32(this + 0xcc, list);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = match seed % 5 {
                    0 => u64::MAX,
                    1 => 0,
                    2 => 1,
                    n => n as u64,
                };
            },
        );
    }

    #[test]
    fn c_shared_login_man_is_card_unformatted_matches() {
        check(
            ee::s07::f_00265cb0__generated,
            c_shared_login_man_is_card_unformatted,
            SEEDS_7,
            &|c, seed| {
                let _ = common(c, seed, 0x265cb0, 2);
                let holder = DATA + 0x100;
                let obj = DATA + 0x2000;
                clear(c, holder, 0x10);
                clear(c, obj, 0x400);
                c.bus.write32(holder, obj);
                c.bus.write16(obj + 0x1a8, 0x20);
                c.bus.write32(obj + 0x1ac, FAKE);
                c.bus.write16(obj + 0x1b8, 0x30);
                c.bus.write32(obj + 0x1bc, FAKE + 0x10);
                c.bus.write32(gp_at(0x2e8), holder);
                let _ = seed;
            },
        );
    }

    #[test]
    fn c_shared_login_man_mc_list_dir_matches() {
        check(
            ee::s07::f_00265fb8__generated,
            c_shared_login_man_mc_list_dir,
            SEEDS_7,
            &|c, seed| {
                let _ = common(c, seed, 0x265fb8, 3);
                alloc_ready(c);
                let memcard = DATA + 0x2000;
                clear(c, memcard, 0x200);
                c.bus.write32(gp_at(0x2e8), memcard);
                c.g[A0][0] = (seed % 4) as u64;
                c.g[A1][0] = sx(DATA + 0x300);
            },
        );
    }

    #[test]
    fn net_event_e2_dtor_matches() {
        check(
            ee::s07::f_00268158__generated,
            net_event_e2_dtor,
            SEEDS_7,
            &|c, seed| {
                let _ = common(c, seed, 0x268158, 1);
                alloc_ready(c);
                let this = heap_block_2(c, 0x100);
                clear(c, this, 0x40);
                c.bus.write32(this + 8, 0x0048_dcd4); // static empty cBXString
                c.g[A0][0] = sx(this);
                c.g[A1][0] = (seed % 2) as u64;
            },
        );
    }

    #[test]
    fn net_event102_down_dtor_matches() {
        check(
            ee::s07::f_00268398__generated,
            net_event102_down_dtor,
            SEEDS_7,
            &|c, seed| {
                let _ = common(c, seed, 0x268398, 1);
                alloc_ready(c);
                let this = heap_block_2(c, 0x100);
                clear(c, this, 0x40);
                c.bus.write32(this + 0xc, 0x0048_dcd4); // static empty cBXString
                c.g[A0][0] = sx(this);
                c.g[A1][0] = (seed % 2) as u64;
            },
        );
    }

    #[test]
    fn net_event_e5_dtor_matches() {
        check(
            ee::s07::f_00268588__generated,
            net_event_e5_dtor,
            SEEDS_7,
            &|c, seed| {
                let _ = common(c, seed, 0x268588, 1);
                alloc_ready(c);
                let this = heap_block_2(c, 0x100);
                clear(c, this, 0x40);
                c.bus.write32(this + 8, 0x0048_dcd4); // static empty cBXString
                c.g[A0][0] = sx(this);
                c.g[A1][0] = (seed % 2) as u64;
            },
        );
    }

    // -------------------------------------------------------- login / voip

    #[test]
    fn cshared_login_man_do_load_matches() {
        check(
            ssx3_ee::s07::f_00265d98__generated,
            cshared_login_man_do_load,
            100,
            &|c, seed| {
                let _r = machine(c, seed, 0x265d98, 2);
                c.g[A0][0] = (DATA + 0x400) as u64;
                c.bus.write32(DATA + 0x400, seed);
            },
        );
    }

    #[test]
    fn cshared_login_man_mc_delete_path_matches() {
        check(
            ssx3_ee::s07::f_00266240__generated,
            cshared_login_man_mc_delete_path,
            100,
            &|c, seed| {
                let _r = machine(c, seed, 0x266240, 3);
                let path = DATA + 0x400;
                let text = b"/mem/0/ssx3\0";
                for (i, b) in text.iter().enumerate() {
                    c.bus.write8(path + i as u32, *b);
                }
                c.g[A0][0] = path as u64;
                c.g[A1][0] = (seed % 0x10) as u64;
            },
        );
    }

    #[test]
    fn cvoip_man_activate_matches() {
        check(
            ssx3_ee::s07::f_00266e30__generated,
            cvoip_man_activate,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x266e30, 1);
                let v = DATA + 0x400;
                c.bus.write32(v, (seed % 3 != 0) as u32);
                c.bus.write32(v + 4, ((seed / 3) % 2) as u32);
                c.bus.write32(GAME_GP + 0x308, (seed % 13) as u32);
                c.g[A0][0] = v as u64;
            },
        );
    }

    pub(crate) const ARENA: u32 = 0x0010_0000;

    pub(crate) const ARENA_SIZE: u32 = 0x0010_0000;

    /// Runs a generated function to completion as a setup step.
    pub(crate) fn run_4(c: &mut Cpu, addr: u32, f: fn(&mut Cpu), args: &[u64]) -> u64 {
        for (i, a) in args.iter().enumerate() {
            c.g[[A0, A1, A2, A3, T0, T1][i]][0] = *a;
        }
        c.g[RA][0] = crate::difftest::RA as u64;
        c.pc = addr;
        f(c);
        assert_eq!(
            c.pc,
            crate::difftest::RA,
            "setup call {addr:#x} did not return"
        );
        c.g[V0][0]
    }

    /// A fresh dlmalloc arena for the main heap.
    pub(crate) fn heap_init_5(c: &mut Cpu) {
        run_4(
            c,
            0x0031_ff60,
            ssx3_ee::s10::DLHeap_InitArena,
            &[MEM_HEAP as u64, ARENA as u64, ARENA_SIZE as u64],
        );
    }

    #[test]
    fn lobby_man_move_to_room_index_matches() {
        check(
            ssx3_ee::s06::f_0025d800__generated,
            lobby_man_move_to_room_index,
            150,
            &|c, seed| {
                let mut r = setup(c, seed, 0x25d800);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = if seed % 3 == 0 {
                    0xffff_ffff_ffff_ffff
                } else {
                    (seed % 4) as u64
                };
                c.g[A2][0] = r.next_u32() as u64;
                // No net object: `MoveToRoom` returns immediately.
                c.bus.write32(DATA, 0);
            },
        );
    }

    #[test]
    fn lobby_man_ping_user_matches() {
        check(
            ssx3_ee::s06::f_0025dd08__generated,
            lobby_man_ping_user,
            120,
            &|c, seed| {
                let mut r = setup(c, seed, 0x25dd08);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = r.next_u32() as u64;
                c.bus.write32(DATA, (seed % 2) as u32);
                c.bus.write32(DATA + 0x54, r.next_u32());
                c.bus.write32(DATA + 0xf4, 0);
                c.bus.write32(DATA + 0x8, r.next_u32());
            },
        );
    }

    #[test]
    fn buddy_man_clear_messages_matches() {
        check(
            ssx3_ee::s06::f_00261070__generated,
            buddy_man_clear_messages,
            120,
            &|c, seed| {
                let mut r = setup(c, seed, 0x261070);
                let this = DATA;
                c.pc = 0x0026_1070;
                c.g[A0][0] = this as u64;
                // A queue of 0..3 entries with some nulls.
                let n = (seed % 4) as u32;
                c.bus.write32(this + 0xac, DATA + 0x1000);
                c.bus.write32(this + 0xb0, DATA + 0x1000 + 4 * n);
                for i in 0..n {
                    if (seed >> (i + 2)) & 1 == 0 {
                        let obj = DATA + 0x1200 + 0x100 * i;
                        c.bus.write32(DATA + 0x1000 + 4 * i, obj);
                        let vt = vtable(c, &mut r, &[8]);
                        c.bus.write32(obj + 0xc, vt);
                    } else {
                        c.bus.write32(DATA + 0x1000 + 4 * i, 0);
                    }
                }
            },
        );
    }

    // ------------------------------------------------------ net event clones

    /// A cloned event: pool `+0x488` allocation, the self's sequence number.
    pub(crate) fn clone_setup(c: &mut Cpu, seed: u32, salt: u64) {
        let mut r = setup(c, seed, salt);
        heap_init_5(c);
        c.g[A0][0] = DATA as u64;
        c.bus.write32(DATA + 8, r.next_u32());
    }

    #[test]
    fn net_event102_clone_matches() {
        check(
            ssx3_ee::s07::f_00267ec8__generated,
            net_event102_clone,
            120,
            &|c, seed| clone_setup(c, seed, 0x267ec8),
        );
    }

    #[test]
    fn net_event102_base_clone_matches() {
        check(
            ssx3_ee::s07::f_00267f58__generated,
            net_event102_base_clone,
            120,
            &|c, seed| clone_setup(c, seed, 0x267f58),
        );
    }

    #[test]
    fn net_error_event_clone_matches() {
        check(
            ssx3_ee::s07::f_00267fe8__generated,
            net_error_event_clone,
            120,
            &|c, seed| clone_setup(c, seed, 0x267fe8),
        );
    }

    #[test]
    fn shared_login_man_mc_cb_state_0f_matches() {
        check(
            ee::s07::f_00266348__generated,
            shared_login_man_mc_cb_state_0f,
            SEEDS_8,
            &|c, seed| {
                let mut r = rng_3(0x0026_6348, seed);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                let v = r.next_u32();
                let a = gp(c, 0x2e4);
                c.bus.write32(a, v);
            },
        );
    }

    #[test]
    fn dsock_channel_base_v05_matches() {
        check(
            ee::s07::f_00267dd8__generated,
            dsock_channel_base_v05,
            SEEDS_8,
            &|c, seed| {
                let mut r = rng_3(0x0026_7dd8, seed);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                let bits = if seed % 2 == 0 {
                    0x3c88_8889
                } else {
                    r.edge_f32()
                };
                let a = gp(c, -0x4a5c);
                c.bus.write32(a, bits);
            },
        );
    }

    #[test]
    fn dsock_channel_is_connected_matches() {
        check(
            ee::s07::f_00267e00__generated,
            dsock_channel_is_connected,
            SEEDS_8,
            &|c, seed| {
                let mut r = rng_3(0x0026_7e00, seed);
                scramble(c, &mut r);
                c.g[A0][0] = sx(DATA);
                c.bus
                    .write32(DATA + 0xc, if seed % 3 == 0 { 2 } else { r.next_u32() });
            },
        );
    }

    /// The `cBXString` static-empty-string marker (the value the string dtor
    /// compares against before touching the heap).
    pub(crate) const EMPTY_STRING: u32 = 0x0048_dcd4;

    /// A fresh dlmalloc arena for the main heap.
    pub(crate) fn heap_init_6(c: &mut Cpu) {
        run(
            c,
            0x0031_ff60,
            ee::s10::DLHeap_InitArena,
            &[MEM_HEAP as u64, ARENA as u64, ARENA_SIZE as u64],
        );
    }

    /// One zeroed block from the dlmalloc arena (valid for `__builtin_delete`).
    pub(crate) fn heap_alloc(c: &mut Cpu, size: u32) -> u32 {
        let p_5 = run(
            c,
            0x0032_0058,
            ee::s10::MEM_HeapMemalign,
            &[MEM_HEAP as u64, 16, size as u64],
        ) as u32;
        for i in (0..size).step_by(4) {
            c.bus.write32(p_5.wrapping_add(i), 0);
        }
        p_5
    }

    #[test]
    fn lobby_man_release_snapshot_lists_matches() {
        check(
            ee::s06::f_0025a850__generated,
            lobby_man_release_snapshot_lists,
            40,
            &|c, seed| {
                let _ = base_2(c, seed, 0x25a8_50);
                // `LobbyApi_DestroyList` destroys and frees each non-null list.
                heap_init_6(c);
                let s = DATA + 0x100;
                clear(c, s, 0x400);
                let arr = s + 0x1ec;
                for i in 0..29u32 {
                    if (seed + i) % 4 == 0 {
                        let list = heap_alloc(c, 0x40);
                        w_2(c, arr + 4 * i, list);
                    } else {
                        w_2(c, arr + 4 * i, 0);
                    }
                }
                c.g[A0][0] = sx(s);
            },
        );
    }

    #[test]
    fn lobby_man_send_chat_matches() {
        check(
            ee::s06::f_0025aae0__generated,
            lobby_man_send_chat,
            32,
            &|c, seed| {
                let mut r = base_2(c, seed, 0x25aa_e0);
                let s = DATA + 0x100;
                clear(c, s, 0x80);
                w_2(c, s, if seed % 3 == 0 { 0 } else { 1 });
                w_2(c, s + 0x54, DATA + 0x300);
                clear(c, DATA + 0x300, 0x40);
                c.g[A0][0] = sx(s);
                c.g[A1][0] = r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn chat_event_dtor_matches() {
        check(
            ee::s07::f_00268048__generated,
            chat_event_dtor,
            48,
            &|c, seed| {
                let _ = base_2(c, seed, 0x2680_48);
                let s = DATA + 0x100;
                clear(c, s, 0x40);
                // Both embedded strings are the shared static empty string, so the
                // string dtors return without touching the heap; `flags = 0` keeps
                // the object owned by the caller.
                w_2(c, s, seed as u32);
                w_2(c, s + 8, EMPTY_STRING);
                w_2(c, s + 0xc, EMPTY_STRING);
                c.g[A0][0] = sx(s);
                c.g[A1][0] = 0;
            },
        );
    }

    #[test]
    fn net_event102_lock_dtor_matches() {
        check(
            ee::s07::f_002682a8__generated,
            net_event102_lock_dtor,
            48,
            &|c, seed| {
                let _ = base_2(c, seed, 0x2682_a8);
                let s = DATA + 0x100;
                clear(c, s, 0x40);
                w_2(c, s, seed as u32);
                w_2(c, s + 0xc, EMPTY_STRING);
                w_2(c, s + 0x10, EMPTY_STRING);
                c.g[A0][0] = sx(s);
                c.g[A1][0] = 0;
            },
        );
    }

    #[test]
    fn net_event_e3_dtor_matches() {
        check(
            ee::s07::f_00268460__generated,
            net_event_e3_dtor,
            48,
            &|c, seed| {
                let _ = base_2(c, seed, 0x2684_60);
                let s = DATA + 0x100;
                clear(c, s, 0x40);
                w_2(c, s, seed as u32);
                w_2(c, s + 8, EMPTY_STRING);
                w_2(c, s + 0xc, EMPTY_STRING);
                c.g[A0][0] = sx(s);
                c.g[A1][0] = 0;
            },
        );
    }

    #[test]
    fn net_event_e2_clone_matches() {
        check(
            ee::s07::f_002681b0__generated,
            net_event_e2_clone,
            40,
            &|c, seed| {
                let mut r = base_2(c, seed, 0x2681_b0);
                let s = DATA + 0x100;
                clear(c, s, 0x40);
                // A null source string: the `cBXString` ctor then skips the
                // (faked) `strlen` and the buffer allocation.
                w_2(c, s + 8, 0);
                c.g[A0][0] = sx(s);
                c.g[A1][0] = r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn lobbyman_query_own_stats_matches() {
        check(
            ee::s06::f_0025cfe0__generated,
            lobbyman_query_own_stats,
            8,
            &|c, seed| {
                machine(c, seed, 0x0025_cfe0, 2);
                be_library_3(c);
                world(c);
                pool(c);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA + 0x80, DATA + 0x1000);
                c.bus.write32(DATA + 0x1000, 0x78); // "x\0"
                c.bus.write32(DATA + 0x54, 0);
            },
        );
    }

    #[test]
    fn c_usb_keyboard_ctor_matches() {
        check(
            ee::s07::f_002667e8__generated,
            c_usb_keyboard_ctor,
            8,
            &|c, seed| {
                machine(c, seed, 0x0026_67e8, 2);
                be_library_3(c);
                world(c);
                pool(c);
                c.g[A0][0] = DATA as u64;
            },
        );
    }

    #[test]
    fn net_event_fb_clone_matches() {
        check(
            ee::s07::f_00268688__generated,
            net_event_fb_clone,
            8,
            &|c, seed| {
                let mut r = machine(c, seed, 0x0026_8688, 1);
                be_library_3(c);
                world(c);
                pool(c);
                c.g[A0][0] = DATA as u64;
                c.bus.write32(DATA + 8, r.next_u32());
                c.bus.write32(DATA + 0xc, r.next_u32());
            },
        );
    }

    #[test]
    fn net_event102_tooy_clone_matches() {
        check(
            ee::s07::f_00268240__generated,
            net_event102_tooy_clone,
            30,
            &|c, seed| {
                fakes::machine(c, seed, 0x268240, 0);
                globals_2(c);
                c.g[A0][0] = OBJ as u64;
                w_3(c, OBJ + 0xc, if seed % 3 == 0 { 0 } else { 0x102 });
            },
        );
    }

    #[test]
    fn c_setup_man_create_matches() {
        check(
            ee::s07::f_00264030__generated,
            c_setup_man_create,
            30,
            &|c, seed| {
                fakes::machine(c, seed, 0x264030, 0);
                globals_2(c);
                pool_allocator_3(c);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = if seed % 2 == 0 { 0 } else { OBJ2 as u64 };
                c.g[A2][0] = if seed % 3 == 0 { 0 } else { OBJ3 as u64 };
            },
        );
    }

    #[test]
    fn c_shared_login_man_mc_mkdir_matches() {
        check(
            ee::s07::f_00266170__generated,
            c_shared_login_man_mc_mkdir,
            20,
            &|c, seed| {
                fakes::machine(c, seed, 0x266170, 0);
                globals_2(c);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = OBJ2 as u64;
                c.g[A2][0] = (seed & 7) as u64;
                // Empty name (strcpy stops at the first byte).
                w_3(c, OBJ2, 0);
                w_3(c, GAME_GP + 0x2e8, if seed % 2 == 0 { 0 } else { OBJ3 });
            },
        );
    }

    #[test]
    fn c_shared_login_man_mc_delete_file_matches() {
        check(
            ee::s07::f_002661d8__generated,
            c_shared_login_man_mc_delete_file,
            20,
            &|c, seed| {
                fakes::machine(c, seed, 0x2661d8, 0);
                globals_2(c);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = OBJ2 as u64;
                c.g[A2][0] = (seed & 7) as u64;
                w_3(c, GAME_GP + 0x2e8, if seed % 2 == 0 { 0 } else { OBJ3 });
            },
        );
    }

    #[test]
    fn lobby_man_get_room_user_name_matches() {
        check(
            ee::s06::f_0025f1e0__generated,
            lobby_man_get_room_user_name,
            30,
            &|c, seed| {
                fakes::machine(c, seed, 0x25f1e0, 0);
                c.g[A0][0] = OBJ as u64;
                c.g[A1][0] = match seed % 4 {
                    0 => u64::MAX,
                    1 => 0,
                    2 => 1,
                    _ => 2,
                };
                c.g[A2][0] = if seed % 4 == 0 { 0 } else { OBJ2 as u64 };
                w_3(c, OBJ, if seed % 5 == 0 { 0 } else { 1 });
                w_3(c, OBJ + 0x60, OBJ3);
                w_3(c, OBJ3 + 0x18, 2);
                w_3(c, OBJ3 + 0x40, OBJ4);
                w_3(c, OBJ4, OBJ2 + 0x100);
                w_3(c, OBJ2 + 0x100, OBJ2 + 0x300);
                w_3(c, OBJ2 + 0x300 + 4, 0x0020_0000);
            },
        );
    }

    #[test]
    fn c_setup_man_send_brag_msg12_matches() {
        check(
            ee::s07::f_00265700__generated,
            c_setup_man_send_brag_msg12,
            10,
            &|c, seed| {
                fakes::machine(c, seed, 0x265700, 0);
                globals_2(c);
                pool_allocator_3(c);
                c.g[A0][0] = OBJ as u64;
                w_3(
                    c,
                    OBJ,
                    match seed % 3 {
                        0 => 12,
                        1 => 9,
                        _ => 3,
                    },
                );
                w_3(c, OBJ + 0x124, if seed % 2 == 0 { 0 } else { 1 });
                w_3(c, OBJ + 0x90, 0);
                w_3(c, OBJ + 0x120, 0);
            },
        );
    }

    // ------------------------------------------------------- net and buddies

    #[test]
    fn lobby_man_request_news_matches() {
        check(
            ee::s06::f_0025e0c8__generated,
            lobby_man_request_news,
            6,
            &|c, seed| {
                let mut r = base_3(c, seed, 0x0025_e0c8, 2);
                c.g[A0][0] = OBJ_2 as u64;
                c.g[A1][0] = r.next_u32() as u64;
                w(c, OBJ_2, if seed % 2 == 0 { 0 } else { 1 });
            },
        );
    }

    #[test]
    fn lobby_man_reset_account_info_matches() {
        check(
            ee::s06::f_00260f10__generated,
            lobby_man_reset_account_info,
            4,
            &|c, seed| {
                let _ = base_3(c, seed, 0x0026_0f10, 0);
                c.g[A0][0] = OBJ_2 as u64;
            },
        );
    }

    pub(crate) fn roster(c: &mut Cpu, count: u32, seed: u32) {
        let list = DATA + 0x3800;
        let array = DATA + 0x3900;
        let cells = DATA + 0x3a00;
        let entries = DATA + 0x4000;
        w(c, OBJ_2 + 0xcc, list);
        w(c, list + 0x18, count);
        w(c, list + 0x40, array);
        for i in 0..count {
            w(c, array + 4 * i, cells + 4 * i);
            w(c, cells + 4 * i, entries + 0x400 * i);
            let temp = if (i + seed) % 2 == 0 { 0x0010_0000 } else { 0 };
            w(c, entries + 0x400 * i + 0xa8, temp);
        }
    }

    #[test]
    fn buddy_man_count_permanent_entries_matches() {
        check(
            ee::s07::f_00262280__generated,
            buddy_man_count_permanent_entries,
            8,
            &|c, seed| {
                let _ = base_3(c, seed, 0x0026_2280, 4);
                c.g[A0][0] = OBJ_2 as u64;
                roster(c, seed % 5, seed);
            },
        );
    }

    #[test]
    fn buddy_man_count_temp_entries_matches() {
        check(
            ee::s07::f_002622f0__generated,
            buddy_man_count_temp_entries,
            8,
            &|c, seed| {
                let _ = base_3(c, seed, 0x0026_22f0, 4);
                c.g[A0][0] = OBJ_2 as u64;
                roster(c, seed % 5, seed);
            },
        );
    }

    #[test]
    fn c_shared_login_man_init_matches() {
        check(
            ee::s07::f_00265950__generated,
            c_shared_login_man_init,
            4,
            &|c, seed| {
                let _ = base_3(c, seed, 0x0026_5950, 4);
                c.g[A0][0] = OBJ_2 as u64;
            },
        );
    }

    #[test]
    fn net_event102_down_clone_matches() {
        check(
            ee::s07::f_002683f0__generated,
            net_event102_down_clone,
            6,
            &|c, seed| {
                let _ = base_3(c, seed, 0x0026_83f0, 2);
                c.g[A0][0] = OBJ_2 as u64;
                w(c, OBJ_2 + 0xc, DATA + 0x4000);
                c.bus.write8(DATA + 0x4000, b'z');
                c.bus.write8(DATA + 0x4001, 0);
            },
        );
    }

    #[test]
    fn net_event_fc_clone_matches() {
        check(
            ee::s07::f_00268720__generated,
            net_event_fc_clone,
            6,
            &|c, seed| {
                let mut r = base_3(c, seed, 0x0026_8720, 2);
                c.g[A0][0] = OBJ_2 as u64;
                for off in [8u32, 0xc, 0x10] {
                    w(c, OBJ_2 + off, r.next_u32());
                }
            },
        );
    }

    pub(crate) fn bb(c: &mut Cpu, a: u32, v: u8) {
        c.bus.write8(a, v);
    }

    /// A dummy world_2 (`gp - 0x848`) with the race-manager chain the callees
    /// walk: world_2+0x84 -> race man, race man+0x20 (painter), +0x28 (riders),
    /// +0x78 (rider count), world_2+0x7c (rider array).
    pub(crate) fn world_2(c: &mut Cpu) {
        let wld = DATA + 0x1000;
        w(c, GAME_GP.wrapping_sub(0x848), wld);
        w(c, wld + 0x84, DATA + 0x1200);
        w(c, DATA + 0x1200 + 0x20, DATA + 0x3000);
        w(c, DATA + 0x1200 + 0x28, DATA + 0x1400);
        w(c, DATA + 0x1200 + 0x78, 0);
        w(c, wld + 0x7c, DATA + 0x1500);
    }

    /// A NUL-terminated string at `a`.
    pub(crate) fn string(c: &mut Cpu, a: u32, s: &str) {
        for (i, ch) in s.bytes().enumerate() {
            bb(c, a + i as u32, ch);
        }
        bb(c, a + s.len() as u32, 0);
    }

    #[test]
    fn buddy_man_has_message_from_matches() {
        check(
            ee::s07::f_00262a38__generated,
            buddy_man_has_message_from,
            SEEDS_9,
            &|c, seed| {
                machine(c, seed, 0x26_2a38, 3);
                c.g[A0][0] = (DATA + 0x2000) as u64;
                c.g[A1][0] = (DATA + 0x2300) as u64;
                c.g[A2][0] = (DATA + 0x2300) as u64;
                w(c, DATA + 0x2000 + 0xac, DATA + 0x2100);
                w(c, DATA + 0x2000 + 0xb0, DATA + 0x2110);
                w(c, DATA + 0x2100, DATA + 0x2200);
                w(c, DATA + 0x2104, DATA + 0x2210);
                w(c, DATA + 0x2200, DATA + 0x2500);
                w(c, DATA + 0x2210, DATA + 0x2600);
                // First message from the asked-for name, the second one, or neither.
                string(c, DATA + 0x2300, "rider");
                let first = seed % 3;
                string(c, DATA + 0x2500, if first == 0 { "rider" } else { "other" });
                string(c, DATA + 0x2600, if first == 1 { "rider" } else { "other" });
                c.pc = 0x0026_2a38;
            },
        );
    }

    #[test]
    fn c_setup_man_ctor_matches() {
        check(
            ee::s07::f_002640c0__generated,
            c_setup_man_ctor,
            SEEDS_9,
            &|c, seed| {
                machine(c, seed, 0x26_40c0, 3);
                c.g[A0][0] = (DATA + 0x2000) as u64;
                c.g[A1][0] = (DATA + 0x2200) as u64;
                c.g[A2][0] = (seed % 9) as u64;
                c.g[A3][0] = (DATA + 0x2400) as u64;
                c.pc = 0x0026_40c0;
            },
        );
    }

    #[test]
    fn c_setup_man_send_brag_msg10_matches() {
        check(
            ee::s07::f_00265688__generated,
            c_setup_man_send_brag_msg10,
            SEEDS_9,
            &|c, seed| {
                machine(c, seed, 0x26_5688, 2);
                belib_2(c);
                heap_setup(c);
                world_2(c);
                c.g[A0][0] = (DATA + 0x2000) as u64;
                w(c, DATA + 0x2000 + 0x124, if seed % 2 == 0 { 0 } else { 1 });
                w(c, DATA + 0x2000, (seed % 12) as u32);
                c.pc = 0x0026_5688;
            },
        );
    }

    #[test]
    fn net_event_e5_clone_matches() {
        check(
            ee::s07::f_002685e0__generated,
            net_event_e5_clone,
            SEEDS_10,
            &|c, seed| {
                machine(c, seed, 0x2685e0, 3);
                be_library_4(c);
                world_3(c);
                heap_init_7(c);
                let src = DATA;
                c.g[A0][0] = src as u64;
                let s = DATA + 0x400;
                string_2(c, s, "event");
                w(c, src + 8, s);
                w(c, src + 0xc, seed);
            },
        );
    }

    #[test]
    fn c_shared_login_man_mc_open_matches() {
        check(
            ee::s07::f_002660c8__generated,
            c_shared_login_man_mc_open,
            50,
            &|c, seed| {
                let _ = base_4(c, seed, 0x2660, 3);
                w(
                    c,
                    GAME_GP.wrapping_add(0x2e8),
                    if seed % 3 == 0 { 0 } else { DATA + 0x400 },
                );
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x200);
                c.g[A2][0] = sx(DATA + 0x300);
                c.g[A3][0] = (seed % 4) as u64;
            },
        );
    }

    // -----------------------------------------------------------------------
    // Audio.

    #[test]
    fn buddyman_create_matches() {
        check(
            ee::s06::f_00260f80__generated,
            buddyman_create,
            SEEDS_11,
            &|c, seed| {
                base_5(c, seed, 0x60f80, 0);
                c.bus.write32(GAME_GP + 0xda0, seed);
            },
        );
    }

    // ------------------------------------------------------------------
    // Lobby / buddy.

    #[test]
    fn lobby_man_chal_request_host_matches() {
        check(
            ssx3_ee::s06::f_0025fa48__generated,
            lobby_man_chal_request_host,
            SEEDS_12,
            &|c, seed| {
                let mut r = machine(c, seed, 0x25fa48, 3);
                be_library_5(c);
                world_4(c);
                let this = DATA + 0x100;
                c.g[A0][0] = this as u64;
                let name = DATA + 0x200;
                string_3(c, name, "rider");
                c.g[A1][0] = name as u64;
                let _ = r.next_u32();
            },
        );
    }

    #[test]
    fn lobby_man_chal_request_join_matches() {
        check(
            ssx3_ee::s06::f_0025fad0__generated,
            lobby_man_chal_request_join,
            SEEDS_12,
            &|c, seed| {
                let mut r = machine(c, seed, 0x25fad0, 3);
                be_library_5(c);
                world_4(c);
                let this = DATA + 0x100;
                c.g[A0][0] = this as u64;
                let name = DATA + 0x200;
                string_3(c, name, "rider");
                c.g[A1][0] = name as u64;
                let _ = r.next_u32();
            },
        );
    }

    #[test]
    fn buddy_man_delete_message_at_matches() {
        check(
            ssx3_ee::s07::f_002635a8__generated,
            buddy_man_delete_message_at,
            SEEDS_12,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2635a8, 4);
                be_library_5(c);
                world_4(c);
                let man = DATA + 0x100;
                let ring = DATA + 0x200;
                c.g[A0][0] = man as u64;
                let count = 1 + (r.next_u32() % 4) as u32;
                c.g[A1][0] = (r.next_u32() % count) as u64;
                w(c, man + 0xa8, ring);
                w(c, man + 0xac, ring);
                w(c, man + 0xb0, ring + 4 * count);
                for i in 0..count {
                    let msg = DATA + 0x400 + 0x40 * i;
                    w(c, ring + 4 * i, msg);
                    let vt = DATA + 0x800 + 0x40 * i;
                    w(c, msg + 0xc, vt);
                    slot_3(c, vt, 8, 0, FAKE + 0x10 * i);
                    slot_3(c, vt, 0xc, 0, FAKE + 0x10 * i + 4);
                }
            },
        );
    }

    /// The lazily built BE singleton at_2 `gp-0x1f04` (a zeroed dummy), the
    /// current interface index at_2 `gp-0x1efc`, and interface 0 of the
    /// `0x5308a8` table pointing at_2 a test object whose `+8` vtable slot calls
    /// the fake. Returns the interface object.
    pub(crate) fn be_library_6(c: &mut Cpu, r: &mut Rng) -> u32 {
        c.bus
            .write32(GAME_GP.wrapping_sub(0x1f04), DATA + 0x30_0000);
        c.bus.write32(GAME_GP.wrapping_sub(0x1efc), 0);
        let obj = DATA + 0x20_0000;
        let vt = vtable(c, r, &[0x8]);
        c.bus.write32(obj + 4, vt);
        c.bus.write32(0x0053_08a8, obj);
        obj
    }

    #[test]
    fn lobby_man_peek_room_matches() {
        check(
            ee::s06::f_0025d770__generated,
            lobby_man_peek_room,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x25d770, 2);
                be_library_6(c, &mut r);
                c.bus.write32(LOG_RESULTS, 0x11);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = if seed % 3 == 0 {
                    u64::MAX
                } else {
                    (seed % 8) as u64
                };
                c.bus.write32(DATA, if seed % 4 == 0 { 0 } else { 1 });
                c.bus.write32(DATA + 0x54, DATA + 0x1000);
            },
        );
    }

    #[test]
    fn buddy_man_is_online_at_matches() {
        check(
            ee::s07::f_00263338__generated,
            buddy_man_is_online_at,
            60,
            &|c, seed| {
                let _r = machine(c, seed, 0x263338, 1);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = if seed % 4 == 0 {
                    u64::MAX
                } else {
                    (seed % 4) as u64
                };
                // An empty roster: count 0, no entries.
                c.bus
                    .write32(DATA + 0xcc, if seed % 3 == 0 { 0 } else { DATA + 0x1000 });
                c.bus.write32(DATA + 0x1000, 0);
            },
        );
    }

    #[test]
    fn c_setup_man_set_local_character_matches() {
        check(
            ee::s07::f_00264cf0__generated,
            c_setup_man_set_local_character,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x264cf0, 4);
                let iface = be_library_6(c, &mut r);
                c.bus.write32(LOG_RESULTS, DATA + 0x3000);
                c.bus.write32(LOG_RESULTS + 4, DATA + 0x4000);
                c.bus.write16(DATA + 0x3000 + 0xc, 0);
                c.bus.write32(DATA + 0x3000 + 0xc, DATA + 0x3100);
                c.bus.write16(DATA + 0x3100 + 8, 0);
                c.bus.write32(DATA + 0x3100 + 0xc, FAKE);
                c.bus.write32(DATA + 0x4000 + 0xc, DATA + 0x4100);
                c.bus.write16(DATA + 0x4100 + 8, 0);
                c.bus.write32(DATA + 0x4100 + 0xc, FAKE);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = (seed % 16) as u64;
                let _ = iface;
                // `gp-0x848` -> a world_5 whose `+0x78` is the interface slot.
                let w = DATA + 0x5000;
                world_5(c, w);
                c.bus.write32(w + 0x78, DATA + 0x5100);
            },
        );
    }

    #[test]
    fn net_event102_lock_clone_matches() {
        check(
            ee::s07::f_00268308__generated,
            net_event102_lock_clone,
            200,
            &|c, seed| {
                let mut r = base_6(c, seed, 0x26_8308);
                globals_5(c);
                pool_4(c);
                w(c, DATA, if seed % 2 == 0 { 0x102 } else { r.next_u32() });
                w(c, DATA + 0x4, [0, 0x4811d0][seed as usize % 2]);
                w(c, DATA + 0x8, 0x6c6f636b);
                w(c, DATA + 0xc, DATA + 0x2000);
                w(c, DATA + 0x10, DATA + 0x2100);
                w(c, DATA + 0x2000, 0x400);
                w(c, DATA + 0x2100, 0x200);
                c.g[A0][0] = sx(DATA);
            },
        );
    }

    pub(crate) fn w8(c: &mut Cpu, a: u32, v: u32) {
        c.bus.write8(a, v as u8);
    }

    /// A NUL-terminated string in the test data area.
    pub(crate) fn cstr(c: &mut Cpu, at: u32, s: &str) {
        for (i, b) in s.bytes().enumerate() {
            w8(c, at + i as u32, b as u32);
        }
        w8(c, at + s.len() as u32, 0);
    }

    #[test]
    fn buddy_man_find_roster_entry_matches() {
        check(
            ee::s07::f_00262468__generated,
            buddy_man_find_roster_entry,
            SEEDS_13,
            &|c, seed| {
                machine(c, seed, 0x2624, 0);
                env(c);
                let this = DATA + 0x400;
                let list = DATA + 0x1000;
                let who = DATA + 0x800;
                cstr(c, who, "BUDDY2");
                let names = [DATA + 0x900, DATA + 0xa00, DATA + 0xb00];
                cstr(c, names[0], "BUDDY1");
                cstr(c, names[1], "BUDDY2");
                cstr(c, names[2], "BUDDY3");
                let n = (seed % 3) as u32; // 0, 1 or 2 entries
                w32(c, this + 0xcc, list);
                w32(c, list + 0x18, n);
                w32(c, list + 0x40, DATA + 0x1800);
                for i in 0..n {
                    let e = DATA + 0x2000 + 0x10 * i;
                    w32(c, DATA + 0x1800 + 4 * i, e);
                    w32(c, e, names[i as usize]);
                }
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(who);
            },
        );
    }

    #[test]
    fn buddy_man_is_same_product_at_matches() {
        check(
            ee::s07::f_002633f8__generated,
            buddy_man_is_same_product_at,
            40,
            &|c, seed| {
                let mut r = base_7(c, seed, 0x2633f8, 3);
                let obj = DATA + 0x1000;
                c.g[A0][0] = obj as u64;
                c.g[A1][0] = match seed % 3 {
                    0 => (-1i64) as u64,
                    1 => 0x1000,
                    _ => (seed % 8) as u64,
                };
                w(c, obj + 0xec, DATA + 0x2000);
                w(c, DATA + 0x2000 + 0xb0, 0);
                let _ = r.next_u32();
            },
        );
    }

    #[test]
    fn lobby_man_destroy_lists_matches() {
        check(
            ee::s06::f_0025a8b0__generated,
            lobby_man_destroy_lists,
            SEEDS_14,
            &alloc_base_setup(|c, seed, _r| {
                let this = DATA;
                w(c, this + 0x54, DATA + 0x800);
                w(
                    c,
                    this + 0x5c,
                    if seed & 1 == 0 { 0 } else { DATA + 0x1000 },
                );
                w(
                    c,
                    this + 0x60,
                    if seed & 2 == 0 { 0 } else { DATA + 0x1100 },
                );
                w(
                    c,
                    this + 0x64,
                    if seed & 4 == 0 { 0 } else { DATA + 0x1200 },
                );
                if seed & 8 == 0 {
                    w(c, this + 0x58, 0);
                } else {
                    let pending = DATA + 0x1400;
                    w(c, this + 0x58, pending);
                    slot_5(c, pending, 0xc, FAKE);
                }
                c.g[A0][0] = this as u64;
            }),
        );
    }

    #[test]
    fn buddy_man_destroy_api_matches() {
        check(
            ee::s06::f_00261530__generated,
            buddy_man_destroy_api,
            SEEDS_14,
            &alloc_base_setup(|c, seed, r| {
                let this = DATA;
                let api = DATA + 0x800;
                w(c, gp(c, -0xc8), api);
                if seed & 1 == 0 {
                    w(c, this + 0xc8, 0);
                } else {
                    let list = DATA + 0x1000;
                    let n = 1 + (seed % 3);
                    w(c, this + 0xbc, list);
                    w(c, this + 0xc0, list + 4 * n);
                    for i in 0..n {
                        let e = DATA + 0x1200 + 0x40 * i;
                        w(c, e, DATA + 0x2000);
                        let vt = vtable(c, r, &[8]);
                        w(c, e + 0x3c, vt);
                        w(c, list + 4 * i, e);
                    }
                    let roster = DATA + 0x1800;
                    w(c, this + 0xc8, roster);
                }
                c.pc = 0x0026_1530;
                c.g[A0][0] = this as u64;
            }),
        );
    }

    /// The engine's size-class pool_7 allocator: `gp + 0xdd8` ready plus four
    /// free nodes per size class.
    pub(crate) fn pool_7(c: &mut Cpu) {
        const POOLS: [(u32, u32); 7] = [
            (0x2a70, 0x800),
            (0x2a60, 0x2000),
            (0x2a6c, 0x6000),
            (0x2a7c, 0x32000),
            (0x2a84, 0x25800),
            (0x2a78, 0x20000),
            (0x2a68, 0x20000),
        ];
        w(c, GAME_GP.wrapping_add(0xdd8), 1);
        let nodes = DATA + 0x80_0000;
        for (i, (slot, head)) in POOLS.into_iter().enumerate() {
            let pool_7 = DATA + 0x40_0000 + 0x4_0000 * i as u32;
            let base_9 = nodes + 0x4000 * i as u32;
            for n in 0..4u32 {
                let node = base_9 + 0x40 * n;
                w(c, node, if n == 3 { 0 } else { node + 0x40 });
            }
            w(c, pool_7 + head, base_9);
            w(c, GAME_GP + slot, pool_7);
        }
    }

    #[test]
    fn lobby_man_send_private_message_matches() {
        check(
            ssx3_ee::s06::f_0025fb80__generated,
            lobby_man_send_private_message,
            8,
            &|c, seed| {
                let _ = base_9(c, seed, 0x0025_fb80);
                elf_image(c);
                heap(c);
                pool_7(c);
                let lobby = DATA + 0x100;
                let service = DATA + 0x4000;
                clear_2(c, lobby, 0x60);
                clear_2(c, service, 0x600);
                put_str(c, DATA + 0x3000, "player_one");
                put_str(c, DATA + 0x3100, "hello there");
                w(c, lobby + 0x54, service);
                w(c, service + 0xc, 0x6964_6c65); // "idle"
                c.g[A0][0] = sx(lobby);
                c.g[A1][0] = sx(DATA + 0x3000);
                c.g[A2][0] = sx(DATA + 0x3100);
            },
        );
    }

    pub(crate) fn put_text(c: &mut Cpu, at: u32, text: &[u8]) {
        for (i, b) in text.iter().enumerate() {
            c.bus.write8(at + i as u32, *b);
        }
        c.bus.write8(at + text.len() as u32, 0);
    }

    #[test]
    fn buddy_man_find_challenger_index_matches() {
        check(
            ssx3_ee::s07::f_00262e20__generated,
            buddy_man_find_challenger_index,
            30,
            &|c, seed| {
                let _r = base_10(c, seed, 0x262e20, 2);
                let this = DATA + 0x1000;
                let name = DATA + 0x5000;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = name as u64;
                put_text(c, name, b"zed");
                let count = match seed % 4 {
                    0 => 0,
                    1 => 1,
                    2 => 2,
                    _ => 3,
                };
                let begin = DATA + 0x2000;
                for i in 0..count {
                    let ent = DATA + 0x3000 + i * 0x100;
                    let nm = DATA + 0x4000 + i * 0x100;
                    let null = seed % 5 == 4 && i == count - 1;
                    w_4(c, begin + 4 * i, if null { 0 } else { ent });
                    put_text(c, nm, if seed % 3 == 0 { b"abc" } else { b"zzz" });
                    w_4(c, ent, nm);
                }
                w_4(c, this + 0xbc, begin);
                w_4(c, this + 0xc0, begin + 4 * count);
            },
        );
    }

    #[test]
    fn c_voip_man_start_matches() {
        check(
            ssx3_ee::s07::f_00266d58__generated,
            c_voip_man_start,
            10,
            &|c, seed| {
                let _r = base_10(c, seed, 0x266d58, 2);
                let this = DATA + 0x1000;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = (DATA + 0x2000) as u64;
                w_4(c, this, if seed % 2 == 0 { 0 } else { 1 });
                w_4(
                    c,
                    GAME_GP.wrapping_add(0x308),
                    if seed % 3 == 0 { 5 } else { 20 },
                );
                w_4(
                    c,
                    GAME_GP.wrapping_add(0x5610),
                    if seed % 4 == 0 { 0 } else { 1 << 18 },
                );
            },
        );
    }

    pub(crate) const PKT: u32 = DATA + 0x3000;

    pub(crate) const DST: u32 = DATA + 0x5000;

    pub(crate) const LIST: u32 = DATA + 0x7000;

    pub(crate) const LISTARR: u32 = DATA + 0x7100;

    pub(crate) const ENTRY: u32 = DATA + 0x7200;

    #[test]
    fn lobby_man_get_rank_list_entry_matches() {
        check(
            ee::s06::f_0025f388__generated,
            lobby_man_get_rank_list_entry,
            6,
            &|c, seed| {
                let _ = base_11(c, seed, 0x25f388, 2);
                c.g[A0][0] = OBJ_3 as u64;
                c.g[A1][0] = 0;
                c.g[A2][0] = OBJ2_2 as u64;
                c.g[A3][0] = (OBJ2_2 + 4) as u64;
                w(c, OBJ_3, (seed % 4 != 0) as u32);
                w(c, OBJ_3 + 0xf8, (seed % 3) as u32);
                w(c, OBJ_3 + 0x1ec, LIST);
                w(c, LIST + 0x18, 4);
                w(c, LIST + 0x40, LISTARR);
                w(c, LISTARR, if seed % 2 == 0 { ENTRY } else { 0 });
                w(c, ENTRY, ENTRY);
                w(c, 0x004a_2f30, 0);
            },
        );
    }

    #[test]
    fn c_setup_man_pop_packet_matches() {
        check(
            ee::s07::f_002649b0__generated,
            c_setup_man_pop_packet,
            8,
            &|c, seed| {
                let _ = base_11(c, seed, 0x2649b0, 3);
                c.g[A0][0] = OBJ_3 as u64;
                c.g[A1][0] = DST as u64;
                w(c, OBJ_3 + 0xc, 5);
                w(c, OBJ_3 + 8, if seed % 3 == 0 { 5 } else { 4 });
                w(c, OBJ_3 + 0x10 + 5 * 4, PKT);
                w(c, PKT + 2, 8);
                fill(c, &mut Rng(seed as u64 ^ 0x99), PKT + 4, 4);
                w(c, OBJ_3 + 4, 0);
                w(c, OBJ_3 + 0x120, 0);
            },
        );
    }

    // ----------------------------------------------------------------- lobby

    #[test]
    fn lobby_man_create_peer_table_matches() {
        check(
            ee::s06::f_0025a650__generated,
            lobby_man_create_peer_table,
            SEEDS_15,
            &|c, seed| {
                let _ = base_12(c, seed, 0x25a6_50, 8);
                let obj = SCRATCH_2;
                c.g[A0][0] = obj as u64;
                w(c, obj + 0xc, if seed % 4 == 0 { 0 } else { 1 });
                w(c, obj + 0xf4, if seed % 3 == 0 { obj + 0x1000 } else { 0 });
                w(c, obj + 0x54, obj + 0x2000);
                // LobbyApi_GetRecord reads the "lobby" object's fields.
                w(c, obj + 0x2000, 0);
                w(c, obj + 0x2004, 0);
            },
        );
    }

    #[test]
    fn chat_event_clone_matches() {
        check(
            ee::s07::f_002680a8__generated,
            chat_event_clone,
            40,
            &|c, seed| {
                let _r = alloc_base_2(c, seed, 0x2680);
                c.g[A0][0] = sx(DATA);
                c.bus.write32(DATA + 8, 0);
                c.bus.write32(DATA + 0xc, 0);
            },
        );
    }

    // ------------------------------------------------------- lobbies and audio

    #[test]
    fn lobby_man_start_quick_play_matches() {
        check(
            ee::s06::f_0025ee20__generated,
            lobby_man_start_quick_play,
            6,
            &|c, seed| {
                let _ = base_14(c, seed, 0x0025_ee20, 3);
                c.g[A0][0] = OBJ_4 as u64;
                c.g[A1][0] = OBJ2_3 as u64;
                c.g[A2][0] = (seed % 3) as u64;
                w(c, OBJ_4, if seed % 4 == 0 { 1 } else { (seed % 3) as u32 });
                w(c, OBJ_4 + 0xa4, 0);
                w(c, OBJ_4 + 0xa0, 0);
            },
        );
    }

    // ---------------------------------------------------------- buddy list

    #[test]
    fn buddy_man_has_challenger_matches() {
        check(
            ee::s07::f_00263128__generated,
            buddy_man_has_challenger,
            60,
            &|c, seed| {
                let mut r = base_15(c, seed, 0x2631, 3);
                c.g[A0][0] = DATA as u64;
                c.pc = 0x0026_3128;
                // Two challengers; the second one's name matches.
                w(c, DATA + 0xbc, DATA + 0x200);
                w(c, DATA + 0xc0, DATA + 0x208);
                w(c, DATA + 0x200, DATA + 0x300);
                w(c, DATA + 0x204, DATA + 0x320);
                for (i, b) in b"alice\0".iter().enumerate() {
                    wb(c, DATA + 0x300 + i as u32, *b);
                }
                for (i, b) in b"bob\0".iter().enumerate() {
                    wb(c, DATA + 0x320 + i as u32, *b);
                }
                let want: &[u8] = match seed % 3 {
                    0 => b"bob\0",
                    1 => b"alice\0",
                    _ => b"carol\0",
                };
                for (i, b) in want.iter().enumerate() {
                    wb(c, DATA + 0x900 + i as u32, *b);
                }
                let _ = r.next_u32();
                c.g[A1][0] = (DATA + 0x900) as u64;
            },
        );
    }

    /// The engine's size-class pool allocator: the "pools ready" flag, seven
    /// class regions with a free-list head past their blocks, and a short
    /// free chain inside each region.
    pub(crate) fn pool8(c: &mut Cpu, _hint: u32) {
        const POOLS: [(u32, u32); 7] = [
            (0x2a70, 0x800),
            (0x2a60, 0x2000),
            (0x2a6c, 0x6000),
            (0x2a7c, 0x32000),
            (0x2a84, 0x25800),
            (0x2a78, 0x20000),
            (0x2a68, 0x20000),
        ];
        c.bus.write32(GAME_GP + 0xdd8, 1);
        for (i, (slot_8, head)) in POOLS.into_iter().enumerate() {
            let pool = DATA + 0x40_0000 + 0x4_0000 * i as u32;
            for n in 0..7u32 {
                let node = pool + 0x100 + 0x100 * n;
                c.bus.write32(node, if n == 6 { 0 } else { node + 0x100 });
            }
            c.bus.write32(pool + head, pool + 0x100);
            c.bus.write32(GAME_GP + slot_8, pool);
        }
    }

    pub(crate) fn slot_8(c: &mut Cpu, vt: u32, off: u32, f: u32) {
        c.bus.write16(vt + off, 0);
        c.bus.write32(vt + off + 4, f);
    }

    #[test]
    fn setup_man_host_recv_cancel_matches() {
        check(
            ee::s07::f_00265398__generated,
            setup_man_host_recv_cancel,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2653, 8);
                let pool = DATA + 0x30_0000;
                pool8(c, pool);
                let s0 = DATA + 0x1000;
                c.bus.write32(GAME_GP - 0xc8, s0);
                c.bus.write32(s0 + 0xf0, DATA + 0x2000);
                c.bus.write32(DATA + 0x2000 + 4, DATA + 0x3000);
                c.bus.write32(DATA, [0u32, 3][(seed % 2) as usize]);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x400);
            },
        );
    }

    #[test]
    fn lobby_man_create_room_matches() {
        strict(
            ee::s06::f_0025dac0__generated,
            lobby_man_create_room,
            30,
            &|c, seed| {
                let mut r = Rng(0x0025_dac0 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.bus.write32(DATA, 0);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x100);
                c.g[A2][0] = (r.next_u32() % 4) as u64;
                c.g[A3][0] = (r.next_u32() % 2) as u64;
            },
        );
    }

    #[test]
    fn c_setup_man_flush_send_queue_matches() {
        check(
            ee::s07::f_00264b20__generated,
            c_setup_man_flush_send_queue,
            SEEDS_16,
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
    fn net_event_e3_clone_matches() {
        check_all(
            ee::s07::f_002684c0__generated,
            net_event_e3_clone,
            SEEDS_16,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2684, 3);
                game_globals(c);
                let src = DATA + 0x2000;
                c.g[A0][0] = src as u64;
                for i in 0..0x30 {
                    c.bus.write32(src + 4 * i, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn csetup_man_set_remote_character_matches() {
        strict_2(
            ee::s07::f_00264d80__generated,
            csetup_man_set_remote_character,
            80,
            &|c, seed| {
                let _r = base_16(c, seed, 0x264d);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x1000) as u64;
                c.g[A2][0] = if seed % 2 == 0 {
                    0
                } else {
                    (DATA + 0x2000) as u64
                };
                for i in 0..8 {
                    c.bus
                        .write8(DATA + 0x2000 + i, (seed as u8).wrapping_add(i as u8));
                }
                c.bus.write32(DATA + 0x1000 + 0xc, DATA + 0x3000);
                c.bus.write16(DATA + 0x3000 + 0x8, 0);
                c.bus.write32(DATA + 0x3000 + 0xc, FAKE);
            },
        );
    }

    #[test]
    fn lobby_man_send_lock_chal_matches() {
        diff(
            ee::s06::f_0025fc70__generated,
            lobby_man_send_lock_chal,
            40,
            &|c, seed| {
                let mut r = fe_base(c, seed, 0x25_fc70);
                put_str_2(c, DATA + 0x300, b"chal");
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = if seed % 2 == 0 { sx(DATA + 0x300) } else { 1 };
                c.g[A2][0] = (seed % 4) as u64;
                w(c, DATA + 0xf0, 0);
                w(c, DATA + 0xe0, r.next_u32());
            },
        );
    }

    #[test]
    fn lobby_man_on_disconnect_matches() {
        diff(
            ee::s06::f_002598c8__generated,
            lobby_man_on_disconnect,
            20,
            &|c, seed| {
                let mut r = fe_base(c, seed, 0x25_98c8);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x2000);
                w(
                    c,
                    DATA + 0x2004,
                    if seed % 2 == 0 { 0 } else { 0x6469_7363 },
                );
                w(c, DATA + 8, r.next_u32());
                w(c, DATA + 0xf0, 0);
            },
        );
    }

    #[test]
    fn lobby_man_parse_name_suggestions_matches() {
        diff(
            ee::s06::f_0025bc48__generated,
            lobby_man_parse_name_suggestions,
            20,
            &|c, seed| {
                let mut r = fe_base(c, seed, 0x25_bc48);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x2000);
                w(c, DATA + 0x200c, DATA + 0x200);
                put_str_2(c, DATA + 0x200, b"a,b,c");
                w(c, DATA + 0x204, r.next_u32());
                for i in 0..8 {
                    w(c, DATA + 0x220 + 4 * i, r.next_u32());
                }
            },
        );
    }

    // -------------------------------------------------------- lobby reply

    #[test]
    fn lobby_man_on_quick_play_reply_matches() {
        strict_3(
            ee::s06::f_0025f030__generated,
            lobby_man_on_quick_play_reply,
            60,
            &|c, seed| {
                let _ = base_17(c, seed, 0x25f0, 6);
                allocator(c);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x800) as u64;
                w(c, DATA, if seed % 3 == 0 { 0x12 } else { 0x13 });
                w(c, DATA + 0xf0, DATA + 0x900);
                w(
                    c,
                    DATA + 0x800 + 0x8,
                    if seed % 2 == 0 { 0 } else { (seed % 5) + 1 },
                );
                w(c, DATA + 0x900, DATA + 0x980);
                w(c, DATA + 0x980 + 0x4, DATA + 0x9c0);
            },
        );
    }

    /// The engine heap, so `cMemMan_alloc` / `MEM_AllocInternal` have an arena.
    pub(crate) fn heap_init_13(c: &mut Cpu) {
        const MEM_HEAP: u32 = 0x004f_f250;
        const RET: u64 = 0x0012_3450;
        c.g[A0][0] = MEM_HEAP as u64;
        c.g[A1][0] = (DATA + 0x4_0000) as u64;
        c.g[A2][0] = 0x20_0000;
        c.g[RA][0] = RET;
        c.pc = 0x0031_ff60;
        ssx3_ee::s10::DLHeap_InitArena(c);
        assert_eq!(c.pc, RET as u32, "DLHeap_InitArena did not return");
    }

    /// The size-class pool_12 allocator's heads and free lists.
    pub(crate) fn pool_12(c: &mut Cpu) {
        const POOLS: [(u32, u32); 7] = [
            (0x2a70, 0x800),
            (0x2a60, 0x2000),
            (0x2a6c, 0x6000),
            (0x2a7c, 0x32000),
            (0x2a84, 0x25800),
            (0x2a78, 0x20000),
            (0x2a68, 0x20000),
        ];
        w(c, GAME_GP.wrapping_add(0xdd8), 1);
        for (i, (slot, size)) in POOLS.into_iter().enumerate() {
            let i = i as u32;
            let start = DATA + 0x40_0000 + 0x4_0000 * i;
            for n in 0..8u32 {
                let node = start + 0x40 * (n + 1);
                w(c, node, if n == 7 { 0 } else { node + 0x40 });
            }
            w(c, start + size, start + 0x40);
            w(c, GAME_GP + slot, start);
        }
    }

    /// The BELibrary dummy and the world -> game -> race manager chain.
    pub(crate) fn globals_12(c: &mut Cpu) {
        let be = DATA + 0x30_0000;
        let world = DATA + 0x8_0000;
        w(c, GAME_GP.wrapping_sub(0x1f04), be);
        w(c, GAME_GP.wrapping_sub(0x1efc), 0);
        w(c, 0x0053_08a8, DATA + 0x30_1000);
        w(c, GAME_GP.wrapping_sub(0x848), world);
        let game = world + 0x1000;
        w(c, world + 0x84, game);
        let race = world + 0x2000;
        w(c, game + 0xc, race);
        w(c, race + 0x7c, 0);
        w(c, race + 0x28, world + 0x3000);
    }

    pub(crate) fn base_18(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        c.g[GP][0] = GAME_GP as u64;
        heap_init_13(c);
        let r = machine(c, seed, salt, calls);
        globals_12(c);
        pool_12(c);
        r
    }

    // -- LobbyMan reply handlers ----------------------------------------

    pub(crate) fn lobby_fixture(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let r = base_18(c, seed, salt, 3);
        let self_ = DATA + 0x1000;
        let reply = DATA + 0x2000;
        let head = DATA + 0x2100;
        c.g[A0][0] = sx(self_);
        c.g[A1][0] = sx(reply);
        w(c, self_ + 0xf0, head);
        w(c, head, head);
        w(c, head + 4, head);
        w(c, reply + 8, if seed % 2 == 0 { 0 } else { DATA + 0x3000 });
        r
    }

    #[test]
    fn lobby_man_on_edit_account_reply_matches() {
        check(
            ssx3_ee::s06::f_0025c0c0__generated,
            lobby_man_on_edit_account_reply,
            60,
            &|c, seed| {
                lobby_fixture(c, seed, 0x25c0c0);
            },
        );
    }

    #[test]
    fn lobby_man_on_lost_by_email_reply_matches() {
        check(
            ssx3_ee::s06::f_0025c2d8__generated,
            lobby_man_on_lost_by_email_reply,
            60,
            &|c, seed| {
                lobby_fixture(c, seed, 0x25c2d8);
            },
        );
    }

    #[test]
    fn lobby_man_on_lost_by_name_reply_matches() {
        check(
            ssx3_ee::s06::f_0025c4f0__generated,
            lobby_man_on_lost_by_name_reply,
            60,
            &|c, seed| {
                lobby_fixture(c, seed, 0x25c4f0);
            },
        );
    }

    /// The BELibrary singleton (`gp - 0x1f04`, a zeroed dummy so nothing reads
    /// the disc).
    pub(crate) fn belib_3(c: &mut Cpu) {
        c.bus.write32(GAME_GP.wrapping_sub(0x1f04), BE);
    }

    #[test]
    fn lobbyman_edit_account_matches() {
        diff(
            ee::s06::f_0025bf18__generated,
            lobbyman_edit_account,
            60,
            &|c, seed| {
                let mut r = machine(c, seed, 0x25_bf18, 4);
                belib_3(c);
                let this = DATA + 0x1_0000;
                let msg = DATA + 0x2_0000;
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(msg);
                c.bus.write32(this + 0x48, r.next_u32());
                c.bus.write32(this + 0x4c, r.next_u32());
                let head = DATA + 0x4_0000;
                c.bus.write32(this + 0xf0, head);
                c.bus.write32(head, head);
                c.bus.write32(head + 4, head);
                c.bus
                    .write32(msg + 0x44, if seed % 3 == 0 { 0 } else { r.next_u32() | 1 });
                c.bus.write32(msg + 4, r.next_u32());
                c.bus
                    .write32(msg + 0x48, if seed % 2 == 0 { 0 } else { r.next_u32() | 1 });
                c.bus
                    .write32(msg + 0x38, if seed % 5 == 0 { 0 } else { r.next_u32() | 1 });
                c.bus
                    .write32(msg + 0x3c, if seed % 7 == 0 { 0 } else { r.next_u32() | 1 });
                c.bus
                    .write32(msg + 0x4c, if seed % 4 == 0 { 0 } else { r.next_u32() | 1 });
                c.bus.write32(msg + 0xc, r.next_u32() & 0xffff);
            },
        );
    }
}
