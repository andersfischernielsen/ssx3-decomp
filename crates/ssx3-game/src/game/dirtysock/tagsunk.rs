//! `src/dirtysock/tagsunk.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi;
use crate::abi::r32;
use crate::abi::ret;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::call::VSlot;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
pub(crate) use crate::game::animation::rideranimbase::sw as sw_4;
pub(crate) use crate::game::bx::memblockman::gp_2 as gp;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::at as at_2;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::gpa as gpa_2;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::lh_at;
pub(crate) use crate::game::camera::trigger::cameratriggerfactory::lw_at as lw_at_5;
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
pub(crate) use crate::game::fe::feasyncfile::lw_at_2;
pub(crate) use crate::game::fe::feasyncfile::lw_r;
pub(crate) use crate::game::fe::feasyncfile::p_5;
pub(crate) use crate::game::fe::feasyncfile::sw_r;
pub(crate) use crate::game::fe::fepopup::ld;
pub(crate) use crate::game::fe::fepopup::sw_4 as sw_6;
pub(crate) use crate::game::fe::fepopupmisc::lh;
pub(crate) use crate::game::fe::fepopupmisc::lw_2;
pub(crate) use crate::game::fe::fepopupmisc::p_2 as p_4;
pub(crate) use crate::game::fe::fepopupmisc::sw as sw_2;
pub(crate) use crate::game::fe::festaterewards::lw_3 as lw_4;
pub(crate) use crate::game::fe::festatetrophyroom::r;
pub(crate) use crate::game::main::gamestate::gw;
pub(crate) use crate::game::main::gamestate::lw_at_3 as lw_at_4;
pub(crate) use crate::game::main::gamestate::p_2 as p;
pub(crate) use crate::game::movie::movieplayer::at;
pub(crate) use crate::game::movie::movieplayer::lw_at;
pub(crate) use crate::game::movie::movieplayer::word;
pub(crate) use crate::game::scripter::ssxscriptengine::gp as gp_7;
pub(crate) use crate::game::scripter::ssxscriptengine::gp_5 as gp_10;
pub(crate) use crate::game::scripter::ssxscriptengine::low;
pub(crate) use crate::game::scripter::ssxscriptengine::lw_at_2 as lw_at_3;
pub(crate) use crate::game::scripter::ssxscriptengine::p_2 as p_3;
pub(crate) use crate::game::scripter::ssxscriptengine::sw_at;
pub(crate) use crate::game::sound::bankmonitor::gp as gp_4;
pub(crate) use crate::game::sound::bankmonitor::leave;
pub(crate) use crate::game::sound::bankmonitor::load_gp;
pub(crate) use crate::game::sound::bankmonitor::p as p_m2;
pub(crate) use crate::game::sound::icepick::worldtriggermanager::forward;
pub(crate) use crate::game::ui::uiengine::oc391_at;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0025_9288,
        tagfieldsetstringescaped,
        "TagFieldSetStringEscaped",
    ),
    (
        0x0025_9310,
        tag_field_get_string_unescaped,
        "TagFieldGetStringUnescaped",
    ),
    (0x0026_1058, buddy_man_clear_login, "BuddyMan_ClearLogin"),
    (0x0026_6d00, c_voip_man_ctor, "cVoipMan_ctor"),
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
    (0x0026_68e8, usb_keyboard_update, "cUsbKeyboard_Update"),
    (0x0026_6ba8, c_usb_keyboard_get_key, "cUsbKeyboard_GetKey"),
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
    (
        0x0025_9398,
        lobby_man_compare_user_names,
        "LobbyMan_CompareUserNames",
    ),
    (
        0x0025_93b8,
        lobby_man_compare_room_names,
        "LobbyMan_CompareRoomNames",
    ),
    (0x0025_98a8, lobby_man_on_recv_debug, "LobbyMan_OnRecvDebug"),
    (0x0025_b650, lobby_man_create, "LobbyMan_Create"),
    (0x0025_c878, lobby_man_cancel_login, "LobbyMan_CancelLogin"),
    (
        0x0025_cd28,
        lobby_man_select_persona_index,
        "LobbyMan_SelectPersonaIndex",
    ),
    (0x0025_fc20, lobby_man_on_mesg_reply, "LobbyMan_OnMesgReply"),
    (0x0025_fc28, lobby_man_send_decline, "LobbyMan_SendDecline"),
    (
        0x0025_fc48,
        lobby_man_send_abort_chal,
        "LobbyMan_SendAbortChal",
    ),
    (
        0x0026_1a20,
        buddy_man_on_send_message_reply,
        "BuddyMan_OnSendMessageReply",
    ),
    (0x0026_2390, buddy_man_is_buddy_at, "BuddyMan_IsBuddyAt"),
    (0x0026_23e0, buddy_man_is_blocked_at, "BuddyMan_IsBlockedAt"),
    (0x0026_2620, buddy_man_get_buddy_at, "BuddyMan_GetBuddyAt"),
    (
        0x0026_2658,
        buddy_man_get_blocked_at,
        "BuddyMan_GetBlockedAt",
    ),
    (0x0026_3660, fe_net_hook_ctor, "FENetHook_ctor"),
    (0x0026_3998, fe_net_hook2_ctor, "FENetHook2_ctor"),
    (
        0x0026_5880,
        c_shared_login_man_create,
        "cSharedLoginMan_Create",
    ),
    (0x0026_58e0, c_shared_login_man_ctor, "cSharedLoginMan_ctor"),
    (
        0x0026_5df0,
        c_shared_login_man_request_load,
        "cSharedLoginMan_RequestLoad",
    ),
    (
        0x0026_5e00,
        c_shared_login_man_request_update,
        "cSharedLoginMan_RequestUpdate",
    ),
    (
        0x0026_5e10,
        c_shared_login_man_request_create,
        "cSharedLoginMan_RequestCreate",
    ),
    (
        0x0026_5e20,
        c_shared_login_man_request_delete,
        "cSharedLoginMan_RequestDelete",
    ),
    (
        0x0026_5e30,
        c_shared_login_man_request_finish,
        "cSharedLoginMan_RequestFinish",
    ),
    (
        0x0026_62e8,
        c_shared_login_man_mc_seek,
        "cSharedLoginMan_McSeek",
    ),
    (0x0026_6788, c_usb_keyboard_create, "cUsbKeyboard_Create"),
    (0x0026_67c0, c_usb_keyboard_destroy, "cUsbKeyboard_Destroy"),
    (
        0x0026_6ed0,
        c_voip_man_toggle_active,
        "cVoipMan_ToggleActive",
    ),
    (0x0026_6f08, c_voip_man_query_state, "cVoipMan_QueryState"),
    (
        0x0026_6f38,
        c_voip_man_get_shared_status,
        "cVoipMan_GetSharedStatus",
    ),
    (
        0x0026_7da8,
        c_dsock_channel_base_disconnect,
        "cDSockChannel_BaseDisconnect",
    ),
    (0x0026_3828, net_update_connect, "Net_UpdateConnect"),
    (
        0x0025_fd50,
        lobby_man_send_race_report,
        "LobbyMan_SendRaceReport",
    ),
    (
        0x0026_3018,
        buddy_man_remove_challenger,
        "BuddyMan_RemoveChallenger",
    ),
    (
        0x0026_31e0,
        buddy_man_has_challenger_str,
        "BuddyMan_HasChallengerStr",
    ),
    (
        0x0026_3270,
        buddy_man_get_challenger_by_name,
        "BuddyMan_GetChallengerByName",
    ),
    (
        0x0026_3500,
        buddy_man_delete_message,
        "BuddyMan_DeleteMessage",
    ),
    (0x0026_36f0, net_disconnect_all, "Net_DisconnectAll"),
    (0x0026_39e0, net_start_game_setup, "Net_StartGameSetup"),
    (0x0026_3ba8, net_abort_game_setup, "Net_AbortGameSetup"),
    (0x0026_3cf0, net_update_game_setup, "Net_UpdateGameSetup"),
    (0x0026_4138, c_setup_man_dtor, "cSetupMan_dtor"),
    (0x0026_42b8, c_setup_man_update, "cSetupMan_Update"),
    (
        0x0026_4a58,
        c_setup_man_receive_packets,
        "cSetupMan_ReceivePackets",
    ),
    (0x0026_4be8, c_setup_man_send_packet, "cSetupMan_SendPacket"),
    (
        0x0026_4e50,
        c_setup_man_send_character_msg,
        "cSetupMan_SendCharacterMsg",
    ),
    (
        0x0026_4f88,
        c_setup_man_select_character,
        "cSetupMan_SelectCharacter",
    ),
    (
        0x0026_5018,
        c_setup_man_confirm_character,
        "cSetupMan_ConfirmCharacter",
    ),
    (
        0x0026_5190,
        c_setup_man_host_recv_select,
        "cSetupMan_HostRecvSelect",
    ),
    (
        0x0026_5290,
        c_setup_man_host_recv_confirm,
        "cSetupMan_HostRecvConfirm",
    ),
    (
        0x0026_5458,
        c_setup_man_send_brag_msg6,
        "cSetupMan_SendBragMsg6",
    ),
    (
        0x0026_5568,
        c_setup_man_send_brag_msg8,
        "cSetupMan_SendBragMsg8",
    ),
    (0x0026_5768, c_setup_man_send_quit, "cSetupMan_SendQuit"),
    (
        0x0026_5a68,
        c_shared_login_man_update,
        "cSharedLoginMan_Update",
    ),
    (
        0x0026_6358,
        c_shared_login_man_mc_poll,
        "cSharedLoginMan_McPoll",
    ),
    (
        0x0026_6578,
        c_shared_login_man_mc_map_error,
        "cSharedLoginMan_McMapError",
    ),
    (0x0026_6850, c_usb_keyboard_dtor, "cUsbKeyboard_dtor"),
    (
        0x0026_6fc0,
        c_voip_man_debug_printf_stub,
        "cVoipMan_DebugPrintfStub",
    ),
    (0x0026_70c8, lobby_man_list_clear, "LobbyMan_ListClear"),
    (
        0x0026_7128,
        buddy_man_message_vector_insert,
        "BuddyMan_MessageVectorInsert",
    ),
    (
        0x0026_72c8,
        buddy_man_challenger_vector_insert,
        "BuddyMan_ChallengerVectorInsert",
    ),
    (0x0026_7468, lobby_man_static_init, "LobbyMan_StaticInit"),
    (0x0026_87c0, net_event10d_clone, "NetEvent10D_Clone"),
];

// ---------------------------------------------------------------------------
// Callees.

pub(crate) mod callee {

    pub(crate) use crate::game::main::gamestate::callee::STR_COPY;

    pub(crate) use crate::game::main::gamestate::callee::TAG_FIELD_SET_STRING;

    pub(crate) use crate::game::movie::movieplayer::callee::STRCPY_2 as STRCPY;

    use super::ee;
    use super::Callee;
    pub(crate) use crate::game::ai::rider::callee::BUDDY_MAN_GET_ROSTER_COUNT as BUDDY_MAN_GET_ROSTER_COUNT_3;
    pub(crate) use crate::game::ai::rider::callee::BUDDY_MAN_GET_ROSTER_ENTRY as BUDDY_MAN_GET_ROSTER_ENTRY_3;
    pub(crate) use crate::game::ai::rider::callee::LOBBY_MAN_GET_LANGUAGE;
    pub(crate) use crate::game::ai::rider::callee::LOBBY_MAN_SEND_REQUEST as LOBBY_MAN_SEND_REQUEST_2;
    pub(crate) use crate::game::be::beintmission::callee::BE_LIBRARY_GET_INSTANCE;
    pub(crate) use crate::game::be::beintmission::callee::BE_LIBRARY_GET_INTERFACE;
    pub(crate) use crate::game::be::beintmission::callee::BUDDY_MAN_GET_ROSTER_ENTRY;
    pub(crate) use crate::game::be::beintmission::callee::EA_MEMSET;
    pub(crate) use crate::game::be::beintmission::callee::SLOGIN_DELETE;
    pub(crate) use crate::game::bx::bxstringctor::callee::EA_MEM_ALLOC_NAMED;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BE_SET_CHARACTER_BY_SLOT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BUDDY_GET_ROSTER_COUNT;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::BUDDY_GET_ROSTER_ENTRY;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::LOBBY_API_SEND;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::callee::LOBBY_GET_ROOM_NAME;
    pub(crate) use crate::game::fe::feasyncfile::callee::ATOI;
    pub(crate) use crate::game::fe::feasyncfile::callee::BUDDY_API_DESTROY;
    pub(crate) use crate::game::fe::feasyncfile::callee::BUDDY_MAN_GET_ROSTER_COUNT_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::BUDDY_MAN_GET_ROSTER_ENTRY_2;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_COMM_SYSTEM_CHANNEL_SEND;
    pub(crate) use crate::game::fe::feasyncfile::callee::C_SETUP_MAN_FREE_PACKET;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_API_DESTROY_LIST;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_API_FLUSH_ROSTER;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MAN_RELEASE_SNAPSHOT_LISTS;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MAN_SEND_DECLINE;
    pub(crate) use crate::game::fe::feasyncfile::callee::LOBBY_MAN_SEND_REQUEST;
    pub(crate) use crate::game::fe::feasyncfile::callee::STRCMP;
    pub(crate) use crate::game::fe::feasyncfile::callee::VOIP_MAN_ACTIVATE;
    pub(crate) use crate::game::fe::feasyncfile::callee::VOIP_MAN_DEACTIVATE;
    pub(crate) use crate::game::fe::fepopupmisc::callee::LOBBY_MAN_GET_ROOM_NAME;
    pub(crate) use crate::game::fe::fepopupmisc::callee::LOBBY_MAN_LOOKUP_PEER_ADDR;
    pub(crate) use crate::game::fe::fepopupmisc::callee::LOBBY_MAN_MOVE_TO_ROOM;
    pub(crate) use crate::game::fe::festatetrophyroom::callee::SCE_USBKB_SET_ARRANGEMENT;
    pub(crate) use crate::game::fe::festatetrophyroom::callee::SCE_USBKB_SET_READ_MODE;
    pub(crate) use crate::game::fe::festatetrophyroom::callee::S_LOGIN_UPDATE;
    pub(crate) use crate::game::fe::festatetrophyroom::callee::USB_KB_DEFAULT_CONNECT_CB;
    pub(crate) use crate::game::main::gamestate::callee::EA_MEM_ALLOC_NAMED_3;
    pub(crate) use crate::game::movie::movieplayer::callee::TAG_FIELD_GET_STRING;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BUDDY_MAN_CLEAR_LOGIN;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BUDDY_MAN_CLEAR_MESSAGES;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::BUDDY_MAN_DISCONNECT;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SEND_PACKET;
    pub(crate) use crate::game::scripter::ssxscriptengine::callee::SETUP_MAN_SET_LOCAL_CHARACTER;
    pub(crate) use crate::game::sound::bankmonitor::callee::BUDDY_MAN_CONNECT;
    pub(crate) use crate::game::sound::bankmonitor::callee::LOBBY_LIST_GET_AT as LOBBY_LIST_GET_AT_2;
    pub(crate) use crate::game::sound::bankmonitor::callee::MEMCARD_REQUEST_GET_INFO_NOW;
    pub(crate) use crate::game::sound::bankmonitor::callee::SLOGIN_MC_GET_PORT_SLOT;
    pub(crate) use crate::game::sound::bankmonitor::callee::STR_CASE_CMP;
    pub(crate) use crate::game::sound::icepick::worldtriggermanager::callee::BUILTIN_DELETE;

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

    pub const STRCPY_2: u32 = 0x0041_66f4;

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

    pub const SETUPMAN_DESTROY: Callee = Callee {
        addr: 0x0026_4098,
        f: ee::s07::cSetupMan_Destroy,
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

    pub const BE_NEW_RACE_INT_SET_NUM_HUMAN_PLAYERS: Callee = Callee {
        addr: 0x0014_4aa8,
        f: ee::s01::f_00144aa8,
    };

    pub const BE_NEW_RACE_INT_GET_TRACK: Callee = Callee {
        addr: 0x0014_4be0,
        f: ee::s01::f_00144be0,
    };

    pub const BE_NEW_PLAYER_INT_SET_PAD_INDEX_BY_SLOT: Callee = Callee {
        addr: 0x0014_72c8,
        f: ee::s01::f_001472c8,
    };

    pub const BE_NEW_PLAYER_INT_SET_CHARACTER: Callee = Callee {
        addr: 0x0014_7338,
        f: ee::s01::f_00147338,
    };

    pub const BE_NEW_PLAYER_INT_GET_CHARACTER_BY_SLOT: u32 = 0x0014_7488;

    pub const BE_BACKUP_PLAYER_SETUP: Callee = Callee {
        addr: 0x0014_ebf8,
        f: ee::s01::f_0014ebf8,
    };

    pub const PRE_GAME_LOAD_SCREEN_CTOR: Callee = Callee {
        addr: 0x0023_2720,
        f: ee::s06::f_00232720,
    };

    pub const C_COMM_SYSTEM_HAS_CHANNEL: Callee = Callee {
        addr: 0x0025_5d38,
        f: ee::s06::f_00255d38,
    };

    pub const C_COMM_SYSTEM_CHANNEL_RECV: u32 = 0x0025_5dc0;

    pub const LOBBY_MAN_CLEAR_CHALLENGE_LOCK: u32 = 0x0025_f740;

    pub const BUDDY_MAN_CREATE: Callee = Callee {
        addr: 0x0026_0f80,
        f: ee::s06::f_00260f80,
    };

    pub const BUDDY_MAN_RECONNECT: Callee = Callee {
        addr: 0x0026_13c8,
        f: ee::s06::f_002613c8,
    };

    pub const BUDDY_MAN_IS_CONNECTED: Callee = Callee {
        addr: 0x0026_1460,
        f: ee::s06::f_00261460,
    };

    pub const BUDDY_MAN_IS_TEMP: Callee = Callee {
        addr: 0x0026_2438,
        f: ee::s07::BuddyMan_IsTemp,
    };

    pub const BUDDY_MAN_HAS_MESSAGE_FROM: Callee = Callee {
        addr: 0x0026_2a38,
        f: ee::s07::BuddyMan_HasMessageFrom,
    };

    pub const BUDDY_MAN_FIND_CHALLENGER_INDEX: Callee = Callee {
        addr: 0x0026_2e20,
        f: ee::s07::BuddyMan_FindChallengerIndex,
    };

    pub const BUDDY_MAN_REMOVE_CHALLENGER: Callee = Callee {
        addr: 0x0026_3018,
        f: ee::s07::BuddyMan_RemoveChallenger,
    };

    pub const BUDDY_MAN_HAS_CHALLENGER: Callee = Callee {
        addr: 0x0026_3128,
        f: ee::s07::BuddyMan_HasChallenger,
    };

    pub const NET_ABORT_GAME_SETUP: Callee = Callee {
        addr: 0x0026_3ba8,
        f: ee::s07::f_00263ba8,
    };

    pub const SETUP_MAN_CREATE: Callee = Callee {
        addr: 0x0026_4030,
        f: ee::s07::f_00264030,
    };

    pub const SETUP_MAN_ALLOC_PACKET: Callee = Callee {
        addr: 0x0026_4960,
        f: ee::s07::cSetupMan_AllocPacket,
    };

    pub const SETUP_MAN_POP_PACKET: Callee = Callee {
        addr: 0x0026_49b0,
        f: ee::s07::cSetupMan_PopPacket,
    };

    pub const SETUP_MAN_RECEIVE_PACKETS: Callee = Callee {
        addr: 0x0026_4a58,
        f: ee::s07::cSetupMan_ReceivePackets,
    };

    pub const SETUP_MAN_FLUSH_SEND_QUEUE: Callee = Callee {
        addr: 0x0026_4b20,
        f: ee::s07::cSetupMan_FlushSendQueue,
    };

    pub const SETUP_MAN_SEND_HELLO: Callee = Callee {
        addr: 0x0026_4cb8,
        f: ee::s07::f_00264cb8,
    };

    pub const SETUP_MAN_SET_REMOTE_CHARACTER: Callee = Callee {
        addr: 0x0026_4d80,
        f: ee::s07::cSetupMan_SetRemoteCharacter,
    };

    pub const SETUP_MAN_SELECT_CHARACTER: Callee = Callee {
        addr: 0x0026_4f88,
        f: ee::s07::cSetupMan_SelectCharacter,
    };

    pub const SETUP_MAN_HOST_RECV_SELECT: Callee = Callee {
        addr: 0x0026_5190,
        f: ee::s07::cSetupMan_HostRecvSelect,
    };

    pub const SETUP_MAN_HOST_RECV_CONFIRM: Callee = Callee {
        addr: 0x0026_5290,
        f: ee::s07::cSetupMan_HostRecvConfirm,
    };

    pub const SETUP_MAN_HOST_RECV_CANCEL: Callee = Callee {
        addr: 0x0026_5398,
        f: ee::s07::cSetupMan_HostRecvCancel,
    };

    pub const SHARED_LOGIN_MAN_DO_LOAD: Callee = Callee {
        addr: 0x0026_5d98,
        f: ee::s07::f_00265d98,
    };

    pub const SHARED_LOGIN_MAN_DO_UPDATE: Callee = Callee {
        addr: 0x0026_5e40,
        f: ee::s07::f_00265e40,
    };

    pub const SHARED_LOGIN_MAN_DO_CREATE: Callee = Callee {
        addr: 0x0026_5e80,
        f: ee::s07::f_00265e80,
    };

    pub const SHARED_LOGIN_MAN_DO_DELETE: Callee = Callee {
        addr: 0x0026_5ec8,
        f: ee::s07::f_00265ec8,
    };

    pub const SHARED_LOGIN_MAN_DO_FINISH: Callee = Callee {
        addr: 0x0026_5f00,
        f: ee::s07::f_00265f00,
    };

    pub const SHARED_LOGIN_MAN_MC_MAP_ERROR: Callee = Callee {
        addr: 0x0026_6578,
        f: ee::s07::f_00266578,
    };

    pub const VOIP_MAN_START: Callee = Callee {
        addr: 0x0026_6d58,
        f: ee::s07::f_00266d58,
    };

    pub const SETUP_MAN_CLIENT_RECV_MSG1: Callee = Callee {
        addr: 0x0026_9008,
        f: ee::s07::cSetupMan_ClientRecvMsg1,
    };

    pub const SETUP_MAN_CLIENT_RECV_MSG3: Callee = Callee {
        addr: 0x0026_90c8,
        f: ee::s07::cSetupMan_ClientRecvMsg3,
    };

    pub const SETUP_MAN_CLIENT_RECV_MSG5: Callee = Callee {
        addr: 0x0026_9188,
        f: ee::s07::cSetupMan_ClientRecvMsg5,
    };

    pub const SETUP_MAN_CLIENT_RECV_MSG0: Callee = Callee {
        addr: 0x0026_9240,
        f: ee::s07::cSetupMan_ClientRecvMsg0,
    };

    pub const SETUP_MAN_CLIENT_RECV_MSG2: Callee = Callee {
        addr: 0x0026_9300,
        f: ee::s07::cSetupMan_ClientRecvMsg2,
    };

    pub const SETUP_MAN_CLIENT_RECV_MSG4: Callee = Callee {
        addr: 0x0026_93c0,
        f: ee::s07::cSetupMan_ClientRecvMsg4,
    };

    pub const SETUP_MAN_HOST_RECV_MSG7: Callee = Callee {
        addr: 0x0026_9478,
        f: ee::s07::cSetupMan_HostRecvMsg7,
    };

    pub const SETUP_MAN_HOST_RECV_MSG9: Callee = Callee {
        addr: 0x0026_9538,
        f: ee::s07::cSetupMan_HostRecvMsg9,
    };

    pub const SETUP_MAN_HOST_RECV_MSG11: Callee = Callee {
        addr: 0x0026_9608,
        f: ee::s07::cSetupMan_HostRecvMsg11,
    };

    pub const SETUP_MAN_HOST_RECV_MSG13: Callee = Callee {
        addr: 0x0026_96f8,
        f: ee::s07::cSetupMan_HostRecvMsg13,
    };

    pub const SETUP_MAN_CLIENT_RECV_MSG6: Callee = Callee {
        addr: 0x0026_97d0,
        f: ee::s07::cSetupMan_ClientRecvMsg6,
    };

    pub const SETUP_MAN_CLIENT_RECV_MSG7: Callee = Callee {
        addr: 0x0026_9890,
        f: ee::s07::cSetupMan_ClientRecvMsg7,
    };

    pub const SETUP_MAN_CLIENT_RECV_MSG8: Callee = Callee {
        addr: 0x0026_9948,
        f: ee::s07::cSetupMan_ClientRecvMsg8,
    };

    pub const SETUP_MAN_CLIENT_RECV_MSG9: Callee = Callee {
        addr: 0x0026_9a18,
        f: ee::s07::cSetupMan_ClientRecvMsg9,
    };

    pub const SETUP_MAN_CLIENT_RECV_MSG10: Callee = Callee {
        addr: 0x0026_9ae0,
        f: ee::s07::cSetupMan_ClientRecvMsg10,
    };

    pub const SETUP_MAN_CLIENT_RECV_MSG11: Callee = Callee {
        addr: 0x0026_9b90,
        f: ee::s07::cSetupMan_ClientRecvMsg11,
    };

    pub const SETUP_MAN_CLIENT_RECV_MSG12: Callee = Callee {
        addr: 0x0026_9c40,
        f: ee::s07::cSetupMan_ClientRecvMsg12,
    };

    pub const SETUP_MAN_CLIENT_RECV_MSG13: Callee = Callee {
        addr: 0x0026_9cf0,
        f: ee::s07::cSetupMan_ClientRecvMsg13,
    };

    pub const MEMCARD_GET_PORT_ERROR: u32 = 0x002c_51d0;

    pub const MEMCARD_GET_DIR_ENTRY_NAME: u32 = 0x002c_68c8;

    pub const APP_SET_NEXT_STATE: Callee = Callee {
        addr: 0x0031_6ef8,
        f: ee::s10::f_00316ef8,
    };

    pub const EA_MEM_COPY: u32 = 0x003e_6574;

    pub const TAG_FIELD_SET_NUMBER: Callee = Callee {
        addr: 0x003e_be40,
        f: ee::s14::f_003ebe40,
    };

    pub const TAG_FIELD_SET_DATE: Callee = Callee {
        addr: 0x003e_c7e0,
        f: ee::s14::f_003ec7e0,
    };

    pub const USB_KB_SHUTDOWN: u32 = 0x003f_62f8;

    pub const SLOGIN_POLL: u32 = 0x003f_80e0;

    pub const SLOGIN_MC_IS_AVAILABLE_WRAPPER: u32 = 0x003f_a360;

    pub const MEM_MOVE: u32 = 0x0041_610c;

    pub const C_COMM_SYSTEM_CREATE_CHANNEL: u32 = 0x0025_5bf8;

    pub const C_COMM_SYSTEM_CLOSE_CHANNEL: u32 = 0x0025_5cc8;

    pub const C_COMM_SYSTEM_DISCONNECT: u32 = 0x0025_5fa0;

    pub const C_COMM_SYSTEM_IS_CHANNEL_CONNECTED: u32 = 0x0025_5d50;

    pub const C_COMM_SYSTEM_GET_CHANNEL_STATUS: u32 = 0x0025_5e40;

    pub const C_GAME_COMM_CREATE: u32 = 0x0025_6bf8;

    pub const SETUP_MAN_UPDATE: Callee = Callee {
        addr: 0x0026_42b8,
        f: ee::s07::cSetupMan_Update,
    };

    pub const C_USB_KEYBOARD_DTOR: Callee = Callee {
        addr: 0x0026_6850,
        f: ee::s07::cUsbKeyboard_dtor,
    };

    pub const BUDDY_API_DEL: u32 = 0x003f_11f8;
}

// ---------------------------------------------------------------------------
// Buddy list.

/// `void BuddyMan_ClearLogin(BuddyMan *self)` (0x261058): clear the login
/// state (a byte at +0, the handle at +0x40 to -1, a byte at +0x44 and the
/// flags byte at +0x64).
pub fn buddy_man_clear_login(c: &mut Cpu) {
    c.cycles += 6;
    let this = Addr(abi::arg(c, 0));
    c.g[V0][0] = 0xffff_ffff_ffff_ffff;
    this.off(0x64).sb(c, 0);
    this.off(0x40).sw(c, c.g[V0][0]);
    this.sb(c, 0);
    this.off(0x44).sb(c, 0);
    abi::ret(c);
}

/// `cVoipMan *cVoipMan_ctor(cVoipMan *this)` (0x266d00): zero the two words of
/// a fresh voip manager and return it unchanged in v0.
pub fn c_voip_man_ctor(c: &mut Cpu) {
    c.cycles += 4;
    let this = Addr(abi::arg(c, 0));
    c.g[V0][0] = c.g[A0][0];
    this.sw(c, 0);
    this.off(0x4).sw(c, 0);
    abi::ret(c);
}

/// `int TagFieldGetStringUnescaped(const char* tag, char* dst, int size)`
/// (0x00259310): parse a tag field into a stack buffer and copy it, dropping
/// a leading backtick.
pub fn tag_field_get_string_unescaped(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        c.g[A2][0] = sx(word(c, A2).wrapping_add(1));
        c.g[SP][0] = sx(sp(c).wrapping_sub(0x50));
        c.g[V0][0] = sx(word(c, A2) << 3);
        c.sq(30, at(c, SP, 0x30));
        c.g[V0][0] = sx(word(c, V0) >> 3);
        c.sq(16, at(c, SP, 0x40));
        c.g[V0][0] = sx(word(c, V0).wrapping_add(0xf));
        c.sd(at(c, SP, 0x20), c.g[RA][0]);
        c.g[V0][0] = sx(word(c, V0) >> 4);
        c.g[FP][0] = c.g[SP][0];
        c.g[V0][0] = sx(word(c, V0) << 4);
        c.g[S0][0] = c.g[A1][0];
        c.g[SP][0] = sx(sp(c).wrapping_sub(word(c, V0)));
        c.g[RA][0] = sx(0x0025_934c);
        c.g[A1][0] = c.g[SP][0];
        callee::TAG_FIELD_GET_STRING.call(c, 0x0025_934c)?;
        c.g[V1][0] = Addr(sp(c)).lb(c);
        c.g[V0][0] = 0x60;
        let plain = c.g[V1][0] != c.g[V0][0];
        c.g[A0][0] = c.g[S0][0];
        if plain {
            c.cycles += 2;
            c.g[A1][0] = c.g[SP][0];
            callee::STRCPY.call(c, 0x0025_9374)?;
        } else {
            c.cycles += 4;
            c.g[A1][0] = sx(word(c, SP) | 1);
            callee::STRCPY.call(c, 0x0025_9364)?;
        }
        c.cycles += 6;
        c.g[SP][0] = c.g[FP][0];
        frame_restore(c, 16, 0x40);
        frame_restore(c, 30, 0x30);
        c.g[RA][0] = c.ld(at(c, SP, 0x20));
        let ra = word(c, RA);
        c.g[SP][0] = sx(sp(c).wrapping_add(0x50));
        c.pc = ra;
        Ok(())
    })
}

/// `lq reg, off(sp)` (a bare restore, no frame object).
#[inline(always)]
pub(crate) fn frame_restore(c: &mut Cpu, reg: usize, off: u32) {
    let a = at(c, SP, off);
    c.lq(reg, a);
}

/// `void TagFieldSetStringEscaped(self, tag, value, text)` (0x259288): when
/// `text` starts with a single quote, copy the rest through a stack buffer;
/// set the tag field.
pub fn tagfieldsetstringescaped(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x140);
        c.g[V0][0] = 0x27;
        frame.save(c, 16, 0x130);
        frame.save(c, 17, 0x120);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, 18, 0x110);
        c.g[S1][0] = c.g[A1][0];
        c.sd(p(sp(c), 0x100), c.g[RA][0]);
        c.g[V1][0] = Addr(c.g[A3][0] as u32).lb(c);
        let quoted = c.g[V1][0] != c.g[V0][0];
        c.g[S2][0] = c.g[A2][0];
        if !quoted {
            c.cycles += 12;
            c.g[V0][0] = 0x60;
            c.g[A1][0] = c.g[A3][0];
            c.g[A0][0] = c.g[SP][0] | 1;
            c.sb(c.g[SP][0] as u32, c.g[V0][0]);
            call_ptr(c, callee::STR_COPY, 0x0025_92c8)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[S1][0];
            c.g[A2][0] = c.g[S2][0];
            c.g[A3][0] = c.g[SP][0];
            call_ptr(c, callee::TAG_FIELD_SET_STRING, 0x0025_92dc)?;
            frame.restore(c, 16, 0x130);
        } else {
            c.cycles += 5;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[S1][0];
            c.g[A2][0] = c.g[S2][0];
            call_ptr(c, callee::TAG_FIELD_SET_STRING, 0x0025_92f4)?;
            frame.restore(c, 16, 0x130);
        }
        c.cycles += 5;
        frame.restore(c, 17, 0x120);
        frame.restore(c, 18, 0x110);
        frame.restore_ra(c, 0x100);
        frame.close(c);
        Ok(())
    })
}

pub(crate) mod usbkb {

    /// `UsbKb_Poll()`: advance the library's async state machine.
    pub const POLL: u32 = 0x003f_63f0;

    /// `UsbKb_IsConnected(port) -> bool`.
    pub const IS_CONNECTED: u32 = 0x003f_6520;

    /// `UsbKb_GetMaxPorts() -> int`.
    pub const MAX_PORTS: u32 = 0x003f_6348;

    /// `UsbKb_GetData(port) -> KbData*` (null when nothing new).
    pub const GET_DATA: u32 = 0x003f_64e0;

    /// `UsbKb_GetKey(data, 0, i) -> u8[2]` (character, raw code).
    pub const GET_KEY: u32 = 0x003f_68f0;

    /// `UsbKb_IsKeyDown(data, 0, 0, key) -> bool`.
    pub const IS_KEY_DOWN: u32 = 0x003f_6838;
}

pub(crate) mod usbk {

    pub const SLOT_SIZE: u32 = 0x8c;

    #[cfg(test)]
    pub const SLOT_COUNT: u32 = 0x0;

    pub const SLOT_START: u32 = 0x4;

    pub const SLOT_RING: u32 = 0x8;

    /// Flags: bit 0 connected, bits 1..3 the data's "on" bit, bit 4 key
    /// 0x10 held.
    pub const SLOT_FLAGS: u32 = 0x88;

    pub const ENABLED: u32 = 0x460;

    /// `KbData`: the "on" word (+0) and the key count (+8).
    pub const DATA_COUNT: u32 = 0x8;
}

pub(crate) mod usb_kb {

    /// Per-keyboard channels (0x8c bytes): queued key count at +0, read
    /// index at +4, a 64-entry ring of key pairs (two bytes) at +8.
    pub const CHANNEL_SIZE: u64 = 0x8c;

    #[cfg(test)]
    pub const COUNT: u32 = 0x0;

    pub const READ: u32 = 0x4;

    pub const RING: u32 = 0x8;

    /// Keyboard: non-zero once opened.
    pub const ACTIVE: u32 = 0x460;

    /// `int UsbKb_GetMaxKeyboards(void)`.
    pub const GET_MAX_KEYBOARDS: u32 = 0x003f_6348;
}

pub(crate) mod table {
    pub(crate) use crate::game::fe::feasyncfile::table::BE_GET_INSTANCE;
    pub(crate) use crate::game::fe::feasyncfile::table::BE_GET_INTERFACE;
    pub(crate) use crate::game::fe::feasyncfile::table::STR_CASECMP;
    pub(crate) use crate::game::fe::feasyncfile::table::VOIP_RPC_SET_PARAMS;
    pub(crate) use crate::game::fe::feasyncfile::table::VOIP_RPC_START;
    pub(crate) use crate::game::movie::movieplayer::table::LOBBY_LIST_GET_NAME;
    pub(crate) use crate::game::movie::movieplayer::table::VOIP_RPC_CALL;
}

pub(crate) mod addr {
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::MEMSET;
    pub(crate) use crate::game::scripter::ssxscriptengine::addr::STRCMP;
}

pub(crate) mod table_callee {
    pub(crate) use crate::game::fe::festaterewards::table_callee::BUDDY_PRODUCT_COMPARE;
    pub(crate) use crate::game::fe::festaterewards::table_callee::TAG_FIELD_FIND;
    pub(crate) use crate::game::fe::festaterewards::table_callee::TAG_FIELD_GET_STRING4;
    pub(crate) use crate::game::fe::festaterewards::table_callee::TAG_FIELD_SET_STRING;
}

// ---------------------------------------------------------------------------
// Lobby.

/// `int LobbyMan_GetLanguage(void)` (0x259198): returns the current language
/// from the globals at_3 0x4a2f18.
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
/// slot number to a presence state (0..3) when the list at_3 `man + 0xc8` is
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
/// messages, `(end - begin) / 4` with the two pointers at_3 +0xac / +0xb0.
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
/// pointer at_3 +0xbc (end at_3 +0xc0), or NULL when out of range.
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
/// pointer at_3 +0xac (end at_3 +0xb0), or NULL when out of range.
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
/// login done (state 0x13 at_3 +0), clears the pending flag at_3 +4 and the
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

/// `void TibExtMemFree(void* unused, void* p_m2)` (0x266760): free `p_m2` when it is
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
        call_ptr(c, callee::STRCPY_2, 0x0026_6564)?;
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
// Shared login manager: card slots (virtual getters on the login object at_3
// `*(gp + 0x2e8)`).

/// `cSharedLoginMan_CardSlot20(this)` 0x265a38: slot +0xa0 of the login
/// object's vtable.
pub fn c_shared_login_man_card_slot20(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2e8));
        frame.save_ra(c, 0);
        let obj = c.lw(c.g[A1][0] as u32);
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
        let vt = c.lw(c.g[A1][0] as u32);
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
        let vt = c.lw(c.g[A1][0] as u32);
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

/// `operator delete(void *p_m2)` (`__builtin_delete`), reached through the
/// function table as the generated code does (`call_indirect`).
pub(crate) const OPERATOR_DELETE: u32 = 0x0031_7e50;

// ---------------------------------------------------------------------------
// Deleting destructors.

/// The shared body of the `(flags & 1)` deleting destructors: republish
/// `vtable` at_3 `self + offset` and delete `self` when bit 0 of `flags` is set.
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
/// `D_004810C8` at_3 `self+8`, deletes on bit 0.
pub fn fe_net_hook_dtor(c: &mut Cpu) {
    guest(c, |c| deleting_dtor(c, 0x0048_10c8, 8, 0x0026_369c))
}

/// `void FENetHook2_dtor(FENetHook2 *self, int flags)` (0x2639b0):
/// republishes `D_004810B0` at_3 `self+0x14`, deletes on bit 0.
pub fn fe_net_hook2_dtor(c: &mut Cpu) {
    guest(c, |c| deleting_dtor(c, 0x0048_10b0, 0x14, 0x0026_39d4))
}

/// `void NetEvent_dtor(NetEvent *self, int flags)` (0x267e18): republishes
/// `D_004812B0` at_3 `self+4`, deletes on bit 0.
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
/// string at_3 `self+0x84`, but only when the session is live (`self+0x54`) and
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
/// count of the room list at_3 `self+0x5c`, or 0 when the lobby is not up.
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
/// of the `int16` at_3 `+0xa` of room `index` in the list at_3 `self+0x5c`.
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
/// user count of the list at_3 `self+0x60`, or 0 when the lobby is not up.
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
/// entries of the roster list at_3 `self+0xcc`, or 0 when there is no list.
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
/// network peer table at_3 `+0xf4` and clear the pointer.
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
/// hello message (type 16, lobby state at_3 `+0xb8`).
pub fn csetup_man_send_hello(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0xc8)).lw(c);
        // The 0x14-byte message lives at_3 the frame base; `ra` is saved at_3
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
/// room user at_3 `[self+0x60]` (a lobby list) exists and its `+4` bit 21 is set.
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
/// `BuddyMan_Connect` (self, index, name at_3 +0x44, message at_3 +0x64, `+0xec`).
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
                c.g[A3][0] = sx(p_m2(self_at, 0x44));
                c.g[T0][0] = sx(p_m2(self_at, 0x64));
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
pub(crate) fn at_3(c: &Cpu, reg: usize, off: u32) -> u32 {
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
        c.g[A0][0] = c.lw(at_3(c, A0, 0xf4));
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
        c.g[A0][0] = c.lw(at_3(c, S0, 0x54));
        if c.g[A0][0] == 0 {
            frame.restore(c, S0, 0x10);
        } else {
            c.cycles += 6;
            call_ptr(c, callee::LOBBY_MAN_REQUEST_UPDATE, 0x0025_aac0)?;
            c.g[A0][0] = c.lw(at_3(c, S0, 0x54));
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
        c.g[V0][0] = c.lw(at_3(c, S0, 0x54));
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
            c.g[V0][0] = c.lw(at_3(c, S0, 0x04));
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
/// about the challenge ("chal") with the state block at_3 0x4a2f40 and mark
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
/// live character, else 5; the state word at_3 +0 moves 4 → 2) is passed with
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
            let pending = c.lw(p_m2(self_, 0x124));
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
/// held at_3 `D_004A33DC` through slot 0x28. 10 (+8) + 2 cycles plus the calls.
pub fn cshared_login_man_request_card_info_if_idle(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = c.lw(gp_8(c, 0x2e8));
        frame.save_ra(c, 0);
        let obj = r32(c, A1);
        let vt = c.lw(obj);
        c.g[V1][0] = vt;
        c.g[V0][0] = c.lw(p_m2(vt as u32, 0x5c));
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
        c.g[A0][0] = Addr(p_m2(vt2 as u32, 0x28)).lh(c);
        c.g[V1][0] = c.lw(p_m2(vt2 as u32, 0x2c));
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
/// word and the class vtable at_3 `D_004812B0`. 20 cycles plus the allocation.
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
        c.sw(p_m2(r32(c, A0), 4), c.g[V1][0]);
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
        Addr(p_m2(this, 0xe8)).sw(c, 0);
        Addr(p_m2(this, 0xd8)).sw(c, 0);
        Addr(p_m2(this, 0xd0)).sw(c, 0);
        Addr(p_m2(this, 0xdc)).sw(c, 0);
        Addr(p_m2(this, 0xe0)).sw(c, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void* BuddyMan_GetRosterEntry(self, idx)` (0x2625c8): 0 for an invalid
/// index, otherwise the buddy at_3 that roster position.
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
        c.g[RA][0] = c.ld(p_m2(c.g[SP][0] as u32, 0x10));
        Addr(gp_9(c, 0x2e4)).sw(c, c.g[V0][0]);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx(p_m2(c.g[SP][0] as u32, 0x30));
        c.pc = t;
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Net-event destructors, replay and script engine.

/// The body shared by the three `NetEvent*_dtor`s: destroy the string at_3
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
    Addr(p_m2(r32(c, S1), 4)).sw(c, c.g[V0][0]);
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
/// once it is configured and at_3 least 10 units of time have passed.
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
        // `true`: start at_3 the popped-message body (the resume entry); `false`:
        // start at_3 the loop head.
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
/// (1/60 s) from the constant at_3 `gp-0x4a5c`.
pub fn dsock_channel_base_v05(c: &mut Cpu) {
    c.cycles += 3;
    c.f[0] = Addr(gp(c, -0x4a5c)).read_fpu(c).0;
    abi::ret(c);
}

/// `int cDSockChannel_IsConnected(cDSockChannel* this)` (0x267e00): whether
/// the channel state at_3 `+0xc` is 2 (connected).
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
        c.sq(S0, p_m2(c.g[SP][0] as u32, 0x210));
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x3108));
        c.sd(p_m2(c.g[SP][0] as u32, 0x200), c.g[RA][0]);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] = 0x200;
        c.g[A3][0] = Addr(p_m2(c.g[S0][0] as u32, 0x80)).lw(c);
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
        c.lq(S0, p_m2(c.g[SP][0] as u32, 0x210));
        c.g[RA][0] = c.ld(p_m2(c.g[SP][0] as u32, 0x200));
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
        Addr(p_m2(c.g[S0][0] as u32, 0x460)).sw(c, 0);
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
            Addr(p_m2(c.g[S0][0] as u32, 0x460)).sw(c, c.g[V0][0]);
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
        c.sq(S0, p_m2(c.g[SP][0] as u32, 0x10));
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x488));
        c.g[S0][0] = c.g[A0][0];
        c.sd(c.g[SP][0] as u32, c.g[RA][0]);
        c.g[A0][0] = 0x10;
        c.g[A2][0] = 0x2000_0000;
        c.g[A3][0] = 0;
        call_ptr(c, 0x0031_7d70, 0x0026_86b0)?;
        c.g[V1][0] = c.g[V0][0];
        c.g[A2][0] = Addr(p_m2(c.g[S0][0] as u32, 0x0c)).lw(c);
        c.g[A1][0] = Addr(p_m2(c.g[S0][0] as u32, 0x08)).lw(c);
        c.g[V0][0] = 0xfb;
        c.g[A0][0] = 0x0048_1150;
        c.sw(c.g[V1][0] as u32, c.g[V0][0]);
        c.lq(S0, p_m2(c.g[SP][0] as u32, 0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        c.g[V0][0] = c.g[V1][0];
        c.sw(p_m2(c.g[V1][0] as u32, 4), c.g[A0][0]);
        c.sw(p_m2(c.g[V1][0] as u32, 8), c.g[A1][0]);
        c.sw(p_m2(c.g[V1][0] as u32, 0xc), c.g[A2][0]);
        let ra = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.pc = ra;
        Ok(())
    })
}

/// `NetEvent102Tooy_Clone(ev)` 0x268240: allocate a 0x10-byte clone, tag it
/// and copy the event kind at_3 `+0xc`.
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
        Addr(p_m2(clone as u32, 8)).sw(c, c.g[A0][0]);
        Addr(p_m2(clone as u32, 4)).sw(c, c.g[A1][0]);
        Addr(p_m2(clone as u32, 0xc)).sw(c, c.g[A2][0]);
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
        let buf = p_m2(sp(c), 0x10);
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
        c.g[S0][0] = sx(p_m2(sp(c), 0x10));
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
            let p_m2 = c.g[V0][0];
            c.g[V1][0] = p_m2;
            if p_m2 == 0 {
                c.g[V0][0] = 0x4a0000;
                c.cycles += 1;
                c.g[V0][0] = sx(0x004a_2ed0);
            } else {
                // The +2 is charged before the out-flag test, so it applies to
                // both non-empty paths.
                c.cycles += 2;
                if c.g[S0][0] == 0 {
                    c.g[V0][0] = sx((p_m2 as u32).wrapping_add(8));
                } else {
                    c.cycles += 6;
                    c.g[V0][0] = lw_at_2(c, p_m2 as u32, 4);
                    c.g[V0][0] = sx(((r32(c, V0) as i32) >> 21) as u32);
                    c.g[V0][0] &= 1;
                    Addr(r32(c, S0)).sw(c, c.g[V0][0]);
                    c.g[V0][0] = sx((p_m2 as u32).wrapping_add(8));
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
            c.sw(p_m2(sp(c), 4), c.g[V1][0]);
            c.g[A1][0] = sp(c) as u64;
            c.sw(p_m2(sp(c), 8), c.g[V0][0]);
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
            c.g[A0][0] = sx(p_m2(base, off));
            call_ptr(c, 0x0031_80b8, 0x0026_0f28 + 8 * i as u32)?;
        }
        c.g[V0][0] = 1;
        c.g[V1][0] = 0x7c1;
        Addr(p_m2(base, 0x28c)).sw(c, c.g[V0][0]);
        Addr(p_m2(base, 0x290)).sw(c, c.g[V1][0]);
        Addr(p_m2(base, 0x2a0)).sw(c, 0);
        Addr(p_m2(base, 0x298)).sw(c, c.g[V0][0]);
        Addr(p_m2(base, 0x29c)).sw(c, c.g[V0][0]);
        Addr(p_m2(base, 0x288)).sw(c, c.g[V0][0]);
        Addr(p_m2(base, 0x294)).sw(c, 0);
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
        Addr(p_m2(c.g[A0][0] as u32, 4)).sw(c, 0);
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
/// tag it and copy the kind string at_3 `+0xc`.
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
        c.g[A1][0] = Addr(p_m2(c.g[S0][0] as u32, 0xc)).lw(c);
        c.g[A0][0] = 0x480000;
        c.g[S0][0] = c.g[V0][0];
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x11b0));
        c.g[V1][0] = 0x646f0000;
        Addr(p_m2(c.g[S0][0] as u32, 4)).sw(c, c.g[A0][0]);
        c.g[V1][0] = c.g[V1][0] | 0x776e;
        c.g[V0][0] = 0x102;
        Addr(p_m2(c.g[S0][0] as u32, 8)).sw(c, c.g[V1][0]);
        c.g[A0][0] = sx(p_m2(c.g[S0][0] as u32, 0xc));
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
        c.g[A3][0] = Addr(p_m2(c.g[S0][0] as u32, 0x10)).lw(c);
        c.g[A2][0] = Addr(p_m2(c.g[S0][0] as u32, 8)).lw(c);
        c.g[V0][0] = 0xfc;
        c.g[A1][0] = Addr(p_m2(c.g[S0][0] as u32, 0xc)).lw(c);
        c.g[A0][0] = 0x480000;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1130));
        Addr(c.g[V1][0] as u32).sw(c, c.g[V0][0]);
        frame.restore(c, S0, 0x10);
        c.g[V0][0] = c.g[V1][0];
        frame.restore_ra(c, 0);
        Addr(p_m2(c.g[V1][0] as u32, 4)).sw(c, c.g[A0][0]);
        Addr(p_m2(c.g[V1][0] as u32, 8)).sw(c, c.g[A2][0]);
        Addr(p_m2(c.g[V1][0] as u32, 0xc)).sw(c, c.g[A1][0]);
        Addr(p_m2(c.g[V1][0] as u32, 0x10)).sw(c, c.g[A3][0]);
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
/// word to 0xe5, vtable `0x481170` at_3 `+4`, copies the string at_3 `+8` and the
/// payload word at_3 `+0xc`. Signature: `NetEventE5* clone(NetEventE5* self)`.
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
        // `operator new` wrapper at_3 0x317e30
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
/// bytes), initialise its fields and install it at_3 `gp+0x238`.
pub fn buddyman_create(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 33;
        let _frame = Frame::open(c, 0x30);
        c.g[A1][0] = 0x480000;
        c.sd(p_m2(sp(c), 0x20), c.g[RA][0]);
        c.g[A0][0] = 0xf0;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xe48));
        c.g[A2][0] = 0x20000000;
        c.g[A3][0] = 0;
        call_ptr(c, callee::EA_MEM_ALLOC_NAMED_3, 0x0026_0fa0)?;
        c.g[A1][0] = lw_at_4(c, gw(c, 0xda0));
        c.g[V1][0] = 0x10000;
        c.g[A0][0] = u64::MAX;
        c.g[V1][0] |= 0x5180;
        c.sw(p_m2(c.g[V0][0] as u32, 0x40), c.g[A0][0]);
        c.sw(p_m2(c.g[V0][0] as u32, 0xa4), c.g[V1][0]);
        c.sw(p_m2(c.g[V0][0] as u32, 0xec), c.g[A1][0]);
        c.sw(gw(c, 0x238), c.g[V0][0]);
        c.g[RA][0] = c.ld(p_m2(sp(c), 0x20));
        c.sw(p_m2(c.g[V0][0] as u32, 0xac), 0);
        c.sw(p_m2(c.g[V0][0] as u32, 0xb0), 0);
        c.sw(p_m2(c.g[V0][0] as u32, 0xb4), 0);
        c.sw(p_m2(c.g[V0][0] as u32, 0xbc), 0);
        c.sw(p_m2(c.g[V0][0] as u32, 0xc0), 0);
        c.sw(p_m2(c.g[V0][0] as u32, 0xc4), 0);
        c.sw(p_m2(c.g[V0][0] as u32, 0xc8), 0);
        c.sw(p_m2(c.g[V0][0] as u32, 0xcc), 0);
        c.sw(p_m2(c.g[V0][0] as u32, 0xd0), 0);
        c.sw(p_m2(c.g[V0][0] as u32, 0xd8), 0);
        c.sw(p_m2(c.g[V0][0] as u32, 0xdc), 0);
        c.sw(p_m2(c.g[V0][0] as u32, 0xe0), 0);
        c.sw(p_m2(c.g[V0][0] as u32, 0xe4), 0);
        c.sw(p_m2(c.g[V0][0] as u32, 0xe8), 0);
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
/// remove the message at_3 `index` from the ring (moving the tail) and release
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
        c.g[A2][0] = sx(p_m2(r32(c, S0), 0xa8));
        c.g[V0][0] = Addr(p_m2(r32(c, S0), 0xac)).lw(c);
        c.g[V1][0] = Addr(p_m2(r32(c, S0), 0xb0)).lw(c);
        c.g[A0][0] = sx(p_m2(r32(c, V0), r32(c, A1)));
        c.g[V0][0] = sx(p_m2(r32(c, A0), 4));
        let last = c.g[V0][0] == c.g[V1][0];
        c.g[S1][0] = Addr(r32(c, A0)).lw(c);
        if !last {
            c.cycles += 6;
            c.g[A2][0] = Addr(p_m2(r32(c, A2), 8)).lw(c);
            c.g[A1][0] = c.g[V0][0];
            c.g[A2][0] = sx(r32(c, A2).wrapping_sub(r32(c, V0)));
            c.g[A2][0] = sx(((r32(c, A2) as i32) >> 2) as u32);
            c.g[RA][0] = sx(0x0026_35f4);
            c.g[A2][0] = sx(r32(c, A2) << 2);
            call_ptr(c, 0x0041_610c, 0x0026_35f4)?;
        }
        c.cycles += 4;
        c.g[V0][0] = Addr(p_m2(r32(c, S0), 0xb0)).lw(c);
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(4));
        let empty = c.g[S1][0] == 0;
        Addr(p_m2(r32(c, S0), 0xb0)).sw(c, c.g[V0][0]);
        if !empty {
            c.cycles += 6;
            c.g[V0][0] = Addr(p_m2(r32(c, S1), 0xc)).lw(c);
            c.g[A1][0] = 0x3;
            c.g[A0][0] = Addr(p_m2(r32(c, V0), 8)).lh(c);
            c.g[V1][0] = Addr(p_m2(r32(c, V0), 0xc)).lw(c);
            let target = r32(c, V1);
            c.g[RA][0] = sx(0x0026_361c);
            c.g[A0][0] = sx(p_m2(r32(c, S1), r32(c, A0)));
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
        c.g[S1][0] = Addr(p_m2(r32(c, S2), 0x10)).lw(c);
        c.g[A1][0] = Addr(p_m2(r32(c, S2), 0xc)).lw(c);
        c.g[A2][0] = sx(r32(c, A2).wrapping_add(0x11d0));
        c.g[V1][0] = c.g[V1][0] | 0x636b;
        c.g[V0][0] = 0x102;
        Addr(p_m2(r32(c, S0), 8)).sw(c, c.g[V1][0]);
        c.g[A0][0] = sx(p_m2(r32(c, S0), 0xc));
        Addr(p_m2(r32(c, S0), 4)).sw(c, c.g[A2][0]);
        Addr(r32(c, S0)).sw(c, c.g[V0][0]);
        call_ptr(c, 0x0031_82d8, 0x0026_836c)?;
        c.g[A1][0] = c.g[S1][0];
        c.g[A0][0] = sx(p_m2(r32(c, S0), 0x10));
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
        c.g[A2][0] = Addr(p_m2(this, 0x5c)).lw(c);
        if c.g[A2][0] != 0 {
            c.cycles += 5;
            c.g[A0][0] = Addr(p_m2(this, 0x54)).lw(c);
            c.g[RA][0] = sx(0x0025_a8d8);
            c.g[A1][0] = 0;
            call_ptr(c, callee::LOBBY_API_DESTROY_LIST, 0x0025_a8d8)?;
            Addr(p_m2(this, 0x5c)).sw(c, 0);
        }
        c.g[A2][0] = Addr(p_m2(this, 0x60)).lw(c);
        c.cycles += 2;
        if c.g[A2][0] != 0 {
            c.cycles += 5;
            c.g[A0][0] = Addr(p_m2(this, 0x54)).lw(c);
            c.g[RA][0] = sx(0x0025_a8f4);
            c.g[A1][0] = 1;
            call_ptr(c, callee::LOBBY_API_DESTROY_LIST, 0x0025_a8f4)?;
            Addr(p_m2(this, 0x60)).sw(c, 0);
        }
        c.g[A2][0] = Addr(p_m2(this, 0x64)).lw(c);
        c.cycles += 2;
        if c.g[A2][0] != 0 {
            c.cycles += 4;
            c.g[A0][0] = Addr(p_m2(this, 0x54)).lw(c);
            c.g[RA][0] = sx(0x0025_a910);
            c.g[A1][0] = 2;
            call_ptr(c, callee::LOBBY_API_DESTROY_LIST, 0x0025_a910)?;
            Addr(p_m2(this, 0x64)).sw(c, 0);
        }
        c.cycles += 5;
        c.g[RA][0] = sx(0x0025_a91c);
        c.g[A0][0] = c.g[S0][0];
        callee::LOBBY_MAN_RELEASE_SNAPSHOT_LISTS.call(c, 0x0025_a91c)?;
        c.g[A0][0] = Addr(p_m2(this, 0x58)).lw(c);
        if c.g[A0][0] != 0 {
            c.cycles += 5;
            c.g[V0][0] = Addr(p_m2(r(c, A0), 0xc)).lw(c);
            let target = r(c, V0);
            c.g[RA][0] = sx(0x0025_a934);
            call_ptr(c, target, 0x0025_a934)?;
            Addr(p_m2(this, 0x58)).sw(c, 0);
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
/// loop: it resumes at_3 every block the original can unwind at_3.
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
                    c.g[V0][0] = Addr(p_m2(this, 0xc8)).lw(c);
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
                    c.g[V1][0] = Addr(p_m2(r(c, S1), 0xbc)).lw(c);
                    pc = 0x0026_1590;
                }
                0x0026_1558 => {
                    c.cycles += 7;
                    let v0 = r(c, V0);
                    c.g[S0][0] = Addr(v0.wrapping_sub(4)).lw(c);
                    c.g[RA][0] = sx(0x0026_1564);
                    c.g[A1][0] = Addr(r(c, S0)).lw(c);
                    callee::LOBBY_MAN_SEND_DECLINE.call(c, 0x0026_1564)?;
                    c.g[V0][0] = Addr(p_m2(r(c, S1), 0xc0)).lw(c);
                    c.g[V0][0] = sx(r(c, V0).wrapping_sub(4));
                    let no_next = c.g[S0][0] == 0;
                    Addr(p_m2(r(c, S1), 0xc0)).sw(c, c.g[V0][0]);
                    if no_next {
                        pc = 0x0026_158c;
                    } else {
                        pc = 0x0026_1574;
                    }
                }
                0x0026_1574 => {
                    c.cycles += 6;
                    c.g[V0][0] = Addr(p_m2(r(c, S0), 0x3c)).lw(c);
                    c.g[A1][0] = 3;
                    c.g[A0][0] = Addr(p_m2(r(c, V0), 8)).lh(c);
                    c.g[V1][0] = Addr(p_m2(r(c, V0), 0xc)).lw(c);
                    let target = r(c, V1);
                    c.g[RA][0] = sx(0x0026_158c);
                    c.g[A0][0] = sx(r(c, S0).wrapping_add(r(c, A0)));
                    call_ptr(c, target, 0x0026_158c)?;
                    pc = 0x0026_158c;
                }
                0x0026_158c => {
                    c.cycles += 1;
                    c.g[V1][0] = Addr(p_m2(r(c, S1), 0xbc)).lw(c);
                    pc = 0x0026_1590;
                }
                0x0026_1590 => {
                    c.cycles += 3;
                    c.g[V0][0] = Addr(p_m2(r(c, S1), 0xc0)).lw(c);
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
                    c.g[A0][0] = Addr(p_m2(r(c, S1), 0xc8)).lw(c);
                    call_ptr(c, callee::LOBBY_API_FLUSH_ROSTER, 0x0026_15a4)?;
                    c.g[RA][0] = sx(0x0026_15ac);
                    c.g[A0][0] = Addr(p_m2(r(c, S1), 0xc8)).lw(c);
                    call_ptr(c, callee::BUDDY_API_DESTROY, 0x0026_15ac)?;
                    Addr(p_m2(r(c, S1), 0xcc)).sw(c, 0);
                    Addr(p_m2(r(c, S1), 0xc8)).sw(c, 0);
                    c.lq(16, p_m2(r(c, SP), 0x20));
                    pc = 0x0026_15b8;
                }
                0x0026_15b8 => {
                    c.cycles += 4;
                    let sp = r(c, SP);
                    c.lq(17, p_m2(sp, 0x10));
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

/// `void LobbyMan_SendPrivateMessage(Lobby*, char* to, char* message)`
/// (0x25fb80): build a 0x100-byte request tag from `to` (field 0x4a3258) and
/// `message` (field 0x4a3010) with the 0x40004000 flags, then send the request
/// (`0x6d657367`) to the lobby name at_3 `lobby+0x54`.
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
        c.g[A0][0] = lw(c, p_m2(c.g[S1][0] as u32, 0x54));
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
/// (0x2624f8): case-insensitive search of the roster list at_3 `man+0xcc`;
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
        let entry: u64;
        if !active {
            c.cycles += 2;
            c.g[V0][0] = 0;
        } else {
            c.cycles += 8;
            c.g[V0][0] = Addr(p_m2(r32(c, S1), 0xf8)).lw(c);
            c.g[V0][0] = sx(r32(c, V0) << 2);
            c.g[V0][0] = sx(p_m2(r32(c, S1), r32(c, V0)));
            c.g[A0][0] = Addr(p_m2(r32(c, V0), 0x1ec)).lw(c);
            call_ptr(c, 0x003e_e560, 0x0025f3d0)?;
            c.g[S0][0] = c.g[V0][0];
            let missing = c.g[S0][0] == 0;
            c.g[A3][0] = sx(0x004a_0000);
            entry = c.g[S0][0];
            if !missing {
                c.cycles += 10;
                c.g[A0][0] = sx(p_m2(r32(c, S0), 0x28));
                c.g[A3][0] = sx(0x004a_2f30);
                c.g[A1][0] = c.g[SP][0];
                c.g[A2][0] = 8;
                call_ptr(c, 0x003e_d678, 0x0025f3f0)?;
                c.g[V1][0] = Addr(p_m2(r32(c, SP), 4)).lw(c);
                Addr(r32(c, S3)).sw(c, c.g[V1][0]);
                c.g[V0][0] = Addr(p_m2(r32(c, S1), 0xf8)).lw(c);
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
        c.g[A1][0] = Addr(p_m2(r32(c, S2), 0xc)).lw(c);
        c.g[V0][0] = Addr(p_m2(r32(c, S2), 8)).lw(c);
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
            Addr(p_m2(r32(c, S2), 0xc)).sw(c, c.g[A0][0]);
            c.g[V0][0] = sx(((r32(c, V0) as i32) >> 5) as u32);
            c.g[A0][0] = c.g[S0][0];
            c.g[V0][0] = sx(r32(c, V0) << 5);
            c.g[V0][0] = sx(r32(c, A1).wrapping_sub(r32(c, V0)));
            c.g[V0][0] = sx(r32(c, V0) << 2);
            c.g[V0][0] = sx(p_m2(r32(c, S2), r32(c, V0)));
            c.g[S1][0] = Addr(p_m2(r32(c, V0), 0x10)).lw(c);
            c.g[S0][0] = Addr(p_m2(r32(c, S1), 2)).lhu(c);
            c.g[A1][0] = sx(p_m2(r32(c, S1), 4));
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
        c.sw(c.g[S0][0] as u32, c.g[V0][0]);
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
                c.sw(p_m2(c.g[S0][0] as u32, 0xa4), c.g[V1][0]);
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
/// at_3 `index`.
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
        c.g[V0][0] = Addr(p_m2(c.g[S1][0] as u32, 0xb0)).lw(c);
        c.g[S2][0] = 0;
        c.g[V1][0] = Addr(p_m2(c.g[S1][0] as u32, 0xac)).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
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
                    c.g[V0][0] = Addr(p_m2(c.g[S1][0] as u32, 0xb0)).lw(c);
                } else {
                    c.cycles += 6;
                    c.g[A0][0] = Addr(c.g[V0][0] as u32).lw(c);
                    c.g[A1][0] = c.g[S3][0];
                    call_ptr(c, 0x0041_65a8, 0x0026_29bc)?;
                    c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    if c.g[V0][0] == 0 {
                        c.g[S0][0] = c.g[V1][0];
                    }
                    c.g[V0][0] = Addr(p_m2(c.g[S1][0] as u32, 0xb0)).lw(c);
                }
                c.cycles += 7;
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                c.g[V1][0] = Addr(p_m2(c.g[S1][0] as u32, 0xac)).lw(c);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
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
/// flag on a match, 0 at_3 the end of the list.
pub fn buddy_man_has_challenger(c: &mut Cpu) {
    // Dispatcher-shaped: an event at_3 a back edge can unwind and the trampoline
    // resumes at_3 the loop head, so `pc` selects the stage and the loop state
    // lives in s0-s3 exactly as the generated code keeps it.
    const ENTRY: u32 = 0x0026_3128;
    const MATCHED: u32 = 0x0026_3164;
    const NEXT: u32 = 0x0026_316c;
    const CHECK: u32 = 0x0026_3188;
    const ADVANCE: u32 = 0x0026_31a8;
    const DONE: u32 = 0x0026_31c4;
    guest(c, |c| {
        // The frame lives on the guest stack, so a resumed call finds it at_3
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
                    c.g[V1][0] = Addr(p_m2(this, 0xc0)).lw(c);
                    c.g[S0][0] = Addr(p_m2(this, 0xbc)).lw(c);
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
                    c.g[V0][0] = Addr(p_m2(r(c, S3), 0xc0)).lw(c);
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
            c.g[V0][0] = c.lw(c.g[S2][0] as u32);
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
                    c.sb(c.g[SP][0] as u32, 0u64);
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
    c.sd(c.g[SP][0] as u32, c.g[RA][0]);
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
    c.sw(c.g[S0][0] as u32, c.g[V0][0]);
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
    c.g[RA][0] = c.ld(c.g[SP][0] as u32);
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
            c.sd(c.g[SP][0] as u32, c.g[RA][0]);
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
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
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
                c.lwr(3, c.g[S2][0] as u32);
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
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, t, 0x00264e34)?;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld(c.g[SP][0] as u32);
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x40));
        c.pc = t;
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
            c.sw(c.g[S2][0] as u32, c.g[S0][0]);
            c.g[A1][0] = 0x480000;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x488));
            c.g[A0][0] = 0x8;
            c.g[A2][0] = 0x20000000;
            c.g[A3][0] = 0u64;
            call_ptr(c, 0x00317d70, 0x0025fcb8)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xf0));
            c.g[V0][0] = 0x480000;
            c.sw(c.g[S0][0] as u32, c.g[S1][0]);
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
        c.sw(c.g[A1][0] as u32, c.g[A0][0]);
        c.g[AT][0] = 0x41a80000;
        c.mtc1(1, 1);
        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
        c.sw(c.g[V1][0] as u32, c.g[A1][0]);
        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
        c.sw((c.g[V0][0] as u32).wrapping_add(0x4), c.g[A1][0]);
        c.swc1(0, (c.g[S2][0] as u32).wrapping_add(0xe0));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.sw(c.g[SP][0] as u32, c.g[A1][0]);
        c.swc1(1, (c.g[GP][0] as u32).wrapping_add(0x2064));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x60));
        c.pc = t;
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
                        c.sw(c.g[S0][0] as u32, c.g[V1][0]);
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
                    c.sw(c.g[A2][0] as u32, c.g[A0][0]);
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
                    c.sw((c.g[A2][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
                    c.sw(c.g[V1][0] as u32, c.g[A2][0]);
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                    c.sw((c.g[V0][0] as u32).wrapping_add(0x4), c.g[A2][0]);
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x10), c.g[A1][0]);
                    c.sw(c.g[SP][0] as u32, c.g[A2][0]);
                    call_ptr(c, 0x002591d0, 0x0025997c)?;
                    c.g[V1][0] = c.lw(c.g[V0][0] as u32);
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
                c.g[V1][0] = c.lb(c.g[SP][0] as u32);
                c.g[S0][0] = c.g[SP][0];
                let t = c.g[V1][0] == 0u64;
                c.sw((c.g[S1][0] as u32).wrapping_add(0x284), 0u64);
                if t {
                    break 'b_25bd18;
                } else {
                    c.cycles += 2;
                    c.g[S2][0] = 0x2c;
                    c.g[V0][0] = c.lb(c.g[S0][0] as u32);
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
                                                c.g[V0][0] = c.lb(c.g[S0][0] as u32);
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
                                    c.g[V0][0] = c.lb(c.g[S0][0] as u32);
                                    if c.g[V0][0] == 0u64 {
                                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x284));
                                        break 'b_25bce4;
                                    } else {
                                        c.cycles += 2;
                                        c.sb(c.g[S0][0] as u32, 0u64);
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
                        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, 0x003184e8, 0x0025bcfc)?;
                        c.g[V1][0] = c.lb(c.g[S0][0] as u32);
                        if c.g[V1][0] == 0u64 {
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x130));
                            break 'b_25bd1c;
                        } else {
                            c.cycles += 4;
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x284));
                            c.g[V0][0] = ((c.g[V0][0] as i64) < 4i64) as u64;
                            if c.g[V0][0] != 0u64 {
                                c.g[V0][0] = c.lb(c.g[S0][0] as u32);
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
        c.sw(p_m2(r(c, S2), 0x4), c.g[V0][0]);
        c.sw(p_m2(r(c, S2), 0x8), c.g[S0][0]);
        c.sw(p_m2(r(c, SP), 0x10), c.g[A3][0]);
        c.g[A0][0] = 0xc;
        c.g[A1][0] = 0x4804a8;
        c.g[A2][0] = 0x2000_0000;
        c.g[A3][0] = 0;
        call_ptr(c, 0x0031_7d70, 0x0025_f0d0)?;

        c.g[A1][0] = c.g[V0][0];
        c.g[V0][0] = sx(p_m2(r(c, A1), 0x8));
        if c.g[V0][0] != 0 {
            c.sw(Addr(r(c, A1)).off(0x8).0, c.g[S2][0]);
        }

        c.cycles += 10;
        c.g[V0][0] = Addr(p_m2(r(c, SP), 0x10)).lw(c);
        c.sw(r(c, A1), c.g[V0][0]);
        c.g[V1][0] = Addr(r(c, V0)).off(0x4).lw(c);
        c.sw(Addr(r(c, A1)).off(0x4).0, c.g[V1][0]);
        c.g[A0][0] = Addr(r(c, V0)).off(0x4).lw(c);
        c.sw(r(c, A0), c.g[A1][0]);
        c.g[V0][0] = Addr(p_m2(r(c, SP), 0x10)).lw(c);
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
/// list at_3 `self + 0xf0`. `tag` is the message id that distinguishes the
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
    c.g[V0][0] = c.lw(p_m2(c.g[S2][0] as u32, 8));
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
            c.g[A2][0] = c.lw(p_m2(c.g[S1][0] as u32, 0xf0));
            c.g[V0][0] = 0x0048_0000;
            c.g[V1][0] = tag;
            c.g[V0][0] = sx(0x0048_12b0);
            c.sw(c.g[S0][0] as u32, c.g[V1][0]);
            c.sw(p_m2(c.g[S0][0] as u32, 4), c.g[V0][0]);
            c.g[A1][0] = 0x0048_0000;
            c.sw(p_m2(c.g[SP][0] as u32, 0x10), c.g[A2][0]);
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
        c.g[A0][0] = c.lw(p_m2(c.g[S2][0] as u32, 8));
        c.g[V0][0] = 0x0048_0000;
        c.g[A2][0] = c.lw(p_m2(c.g[S1][0] as u32, 0xf0));
        c.g[V0][0] = sx(0x0048_1290);
        c.g[V1][0] = 0x102;
        c.sw(p_m2(c.g[S0][0] as u32, 8), c.g[A0][0]);
        c.g[A1][0] = 0x0048_0000;
        c.sw(c.g[S0][0] as u32, c.g[V1][0]);
        c.g[A1][0] = sx(0x0048_04a8);
        c.sw(p_m2(c.g[S0][0] as u32, 4), c.g[V0][0]);
        c.g[A0][0] = 0xc;
        c.sw(p_m2(c.g[SP][0] as u32, 0x10), c.g[A2][0]);
        c.g[A3][0] = 0;
        c.g[A2][0] = 0x2000_0000;
    }

    // Link the message into the list at_3 self+0xf0.
    'link: {
        c.cycles += 6;
        c.g[RA][0] = sx(ret3);
        call_ptr(c, 0x0031_7d70, ret3)?; // cMemMan_alloc
        c.g[A1][0] = c.g[V0][0];
        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(8));
        if c.g[V0][0] != 0 {
            c.sw(p_m2(c.g[A1][0] as u32, 8), c.g[S0][0]);
            break 'link;
        }
        break 'link;
    }
    c.cycles += 15;
    c.g[V0][0] = c.lw(p_m2(c.g[SP][0] as u32, 0x10));
    c.sw(c.g[A1][0] as u32, c.g[V0][0]);
    c.g[V1][0] = c.lw(p_m2(c.g[V0][0] as u32, 4));
    c.sw(p_m2(c.g[A1][0] as u32, 4), c.g[V1][0]);
    c.g[A0][0] = c.lw(p_m2(c.g[V0][0] as u32, 4));
    c.sw(c.g[A0][0] as u32, c.g[A1][0]);
    c.g[V0][0] = c.lw(p_m2(c.g[SP][0] as u32, 0x10));
    c.sw(p_m2(c.g[V0][0] as u32, 4), c.g[A1][0]);
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
            c.g[V0][0] = lw(c, at_3(c, S1, 0x8));
            let t = c.g[V0][0] != 0u64;
            c.g[A1][0] = 0x480000;
            if t {
                break 'b_25d614;
            } else {
                'b_25d5c8: {
                    c.cycles += 7;
                    c.g[V0][0] = lw(c, at_3(c, S3, 0x6c));
                    c.g[S4][0] = sx(at_3(c, S3, 0xec));
                    c.g[S2][0] = lw(c, at_3(c, S3, 0x50));
                    c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x1));
                    c.g[V1][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let t = c.g[V1][0] == 0u64;
                    sw_6(c, at_3(c, S3, 0x6c), c.g[V0][0]);
                    if t {
                        break 'b_25d5c8;
                    } else {
                        c.cycles += 5;
                        c.g[V0][0] = sx(r32(c, S2) << 2);
                        c.g[V1][0] = sx(at_3(c, V0, 0x70));
                        c.g[V0][0] = sx(at_3(c, V0, 0x74));
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
                            c.g[S2][0] = sx(at_3(c, S2, 0x1));
                            c.g[V0][0] = lw(c, at_3(c, S3, 0x6c));
                            c.g[S1][0] = sx(at_3(c, S1, 0x4));
                            c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                            let t = c.g[V0][0] != 0u64;
                            c.g[S0][0] = sx(at_3(c, S0, 0x4));
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
                c.g[A1][0] = sx(at_3(c, A1, 0x488));
                c.g[A0][0] = 0x8;
                c.g[RA][0] = sx(0x0025d5dc);
                c.g[A3][0] = 0u64;
                if call_ptr(c, 0x00317d70, 0x0025d5dc).is_err() {
                    return;
                }
                c.g[S0][0] = c.g[V0][0];
                c.g[A2][0] = lw(c, at_3(c, S4, 0x4));
                c.g[V0][0] = 0x480000;
                c.g[V1][0] = 0xd8;
                c.g[V0][0] = sx(at_3(c, V0, 0x12b0));
                sw_6(c, r32(c, S0), c.g[V1][0]);
                sw_6(c, at_3(c, S0, 0x4), c.g[V0][0]);
                c.g[A1][0] = 0x480000;
                sw_6(c, at_3(c, SP, 0x10), c.g[A2][0]);
                c.g[A1][0] = sx(at_3(c, A1, 0x4a8));
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
        c.g[A1][0] = sx(at_3(c, A1, 0x488));
        c.g[A2][0] = 0x20000000;
        c.g[RA][0] = sx(0x0025d628);
        c.g[A3][0] = 0u64;
        if call_ptr(c, 0x00317d70, 0x0025d628).is_err() {
            return;
        }
        c.g[S0][0] = c.g[V0][0];
        c.g[A0][0] = lw(c, at_3(c, S1, 0x8));
        c.g[V0][0] = 0x480000;
        c.g[A2][0] = lw(c, at_3(c, S3, 0xf0));
        c.g[V0][0] = sx(at_3(c, V0, 0x1290));
        c.g[V1][0] = 0x102;
        sw_6(c, at_3(c, S0, 0x8), c.g[A0][0]);
        c.g[A1][0] = 0x480000;
        sw_6(c, r32(c, S0), c.g[V1][0]);
        c.g[A1][0] = sx(at_3(c, A1, 0x4a8));
        sw_6(c, at_3(c, S0, 0x4), c.g[V0][0]);
        c.g[A0][0] = 0xc;
        sw_6(c, at_3(c, SP, 0x10), c.g[A2][0]);
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
        c.g[V0][0] = sx(at_3(c, A1, 0x8));
        if c.g[V0][0] != 0u64 {
            sw_6(c, at_3(c, A1, 0x8), c.g[S0][0]);
            break 'b_25d67c;
        } else {
            break 'b_25d67c;
        }
    }
    c.cycles += 17;
    c.g[V0][0] = lw(c, at_3(c, SP, 0x10));
    sw_6(c, r32(c, A1), c.g[V0][0]);
    c.g[V1][0] = lw(c, at_3(c, V0, 0x4));
    sw_6(c, at_3(c, A1, 0x4), c.g[V1][0]);
    c.g[A0][0] = lw(c, at_3(c, V0, 0x4));
    sw_6(c, r32(c, A0), c.g[A1][0]);
    c.g[V0][0] = lw(c, at_3(c, SP, 0x10));
    sw_6(c, at_3(c, V0, 0x4), c.g[A1][0]);
    sw_6(c, r32(c, SP), c.g[A1][0]);
    frame.restore(c, S0, 0x70);
    frame.restore(c, S1, 0x60);
    frame.restore(c, S2, 0x50);
    frame.restore(c, S3, 0x40);
    frame.restore(c, S4, 0x30);
    c.g[RA][0] = ld(c, at_3(c, SP, 0x20));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(at_3(c, SP, 0x80));
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
            c.g[V0][0] = lw(c, at_3(c, S1, 0x8));
            let t = c.g[V0][0] != 0u64;
            c.g[A1][0] = 0x480000;
            if t {
                break 'b_25b920;
            } else {
                'b_25b8fc: {
                    c.cycles += 32;
                    c.g[A0][0] = lw(c, at_3(c, S2, 0x54));
                    c.g[A2][0] = 0x480000;
                    c.g[A1][0] = 0x73650000;
                    c.g[T0][0] = 0u64;
                    c.g[A1][0] = c.g[A1][0] | 0x6c65;
                    c.g[A2][0] = sx(at_3(c, A2, 0x848));
                    c.g[RA][0] = sx(0x0025b89c);
                    c.g[A3][0] = 0u64;
                    if call_ptr(c, 0x003e8b10, 0x0025b89c).is_err() {
                        return;
                    }
                    c.g[A1][0] = 0x480000;
                    c.g[A2][0] = 0x20000000;
                    c.g[A1][0] = sx(at_3(c, A1, 0x488));
                    c.g[A0][0] = 0x8;
                    c.g[RA][0] = sx(0x0025b8b4);
                    c.g[A3][0] = 0u64;
                    if call_ptr(c, 0x00317d70, 0x0025b8b4).is_err() {
                        return;
                    }
                    c.g[S0][0] = c.g[V0][0];
                    c.g[A2][0] = lw(c, at_3(c, S2, 0xf0));
                    c.g[V0][0] = 0x480000;
                    c.g[V1][0] = 0xc9;
                    c.g[V0][0] = sx(at_3(c, V0, 0x12b0));
                    sw_6(c, r32(c, S0), c.g[V1][0]);
                    sw_6(c, at_3(c, S0, 0x4), c.g[V0][0]);
                    c.g[A1][0] = 0x480000;
                    sw_6(c, at_3(c, SP, 0x10), c.g[A2][0]);
                    c.g[A1][0] = sx(at_3(c, A1, 0x4a8));
                    c.g[A0][0] = 0xc;
                    c.g[A2][0] = 0x20000000;
                    c.g[RA][0] = sx(0x0025b8ec);
                    c.g[A3][0] = 0u64;
                    if call_ptr(c, 0x00317d70, 0x0025b8ec).is_err() {
                        return;
                    }
                    c.g[A1][0] = c.g[V0][0];
                    c.g[V0][0] = sx(at_3(c, A1, 0x8));
                    if c.g[V0][0] != 0u64 {
                        sw_6(c, at_3(c, A1, 0x8), c.g[S0][0]);
                        break 'b_25b8fc;
                    } else {
                        break 'b_25b8fc;
                    }
                }
                c.cycles += 9;
                c.g[V0][0] = lw(c, at_3(c, SP, 0x10));
                sw_6(c, r32(c, A1), c.g[V0][0]);
                c.g[V1][0] = lw(c, at_3(c, V0, 0x4));
                sw_6(c, at_3(c, A1, 0x4), c.g[V1][0]);
                c.g[A0][0] = lw(c, at_3(c, V0, 0x4));
                sw_6(c, r32(c, A0), c.g[A1][0]);
                c.g[V0][0] = lw(c, at_3(c, SP, 0x10));
                let t = true;
                sw_6(c, at_3(c, V0, 0x4), c.g[A1][0]);
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
            c.g[A1][0] = sx(at_3(c, A1, 0x488));
            c.g[A2][0] = 0x20000000;
            c.g[RA][0] = sx(0x0025b934);
            c.g[A3][0] = 0u64;
            if call_ptr(c, 0x00317d70, 0x0025b934).is_err() {
                return;
            }
            c.g[S0][0] = c.g[V0][0];
            c.g[A0][0] = lw(c, at_3(c, S1, 0x8));
            c.g[V0][0] = 0x480000;
            c.g[A2][0] = lw(c, at_3(c, S2, 0xf0));
            c.g[V0][0] = sx(at_3(c, V0, 0x1290));
            c.g[V1][0] = 0x102;
            sw_6(c, at_3(c, S0, 0x8), c.g[A0][0]);
            c.g[A1][0] = 0x480000;
            sw_6(c, r32(c, S0), c.g[V1][0]);
            c.g[A1][0] = sx(at_3(c, A1, 0x4a8));
            sw_6(c, at_3(c, S0, 0x4), c.g[V0][0]);
            c.g[A0][0] = 0xc;
            sw_6(c, at_3(c, SP, 0x10), c.g[A2][0]);
            c.g[A3][0] = 0u64;
            c.g[RA][0] = sx(0x0025b974);
            c.g[A2][0] = 0x20000000;
            if call_ptr(c, 0x00317d70, 0x0025b974).is_err() {
                return;
            }
            c.g[A1][0] = c.g[V0][0];
            c.g[V0][0] = sx(at_3(c, A1, 0x8));
            if c.g[V0][0] != 0u64 {
                sw_6(c, at_3(c, A1, 0x8), c.g[S0][0]);
                break 'b_25b984;
            } else {
                break 'b_25b984;
            }
        }
        c.cycles += 9;
        c.g[V0][0] = lw(c, at_3(c, SP, 0x10));
        sw_6(c, r32(c, A1), c.g[V0][0]);
        c.g[V1][0] = lw(c, at_3(c, V0, 0x4));
        sw_6(c, at_3(c, A1, 0x4), c.g[V1][0]);
        c.g[A0][0] = lw(c, at_3(c, V0, 0x4));
        sw_6(c, r32(c, A0), c.g[A1][0]);
        c.g[V0][0] = lw(c, at_3(c, SP, 0x10));
        sw_6(c, at_3(c, V0, 0x4), c.g[A1][0]);
        sw_6(c, at_3(c, S2, 0x10), c.g[S3][0]);
        break 'b_25b9a8;
    }
    c.cycles += 8;
    sw_6(c, r32(c, SP), c.g[A1][0]);
    frame.restore(c, S0, 0x60);
    frame.restore(c, S1, 0x50);
    frame.restore(c, S2, 0x40);
    frame.restore(c, S3, 0x30);
    c.g[RA][0] = ld(c, at_3(c, SP, 0x20));
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx(at_3(c, SP, 0x70));
    c.pc = t;
    return;
}

/// Tag names looked up in the lobby reply's tag list.
pub(crate) const TAG_A: u32 = 0x004a_3198;

pub(crate) const TAG_IDX: u32 = 0x004a_31a8;

pub(crate) const TAG_B: u32 = 0x004a_31a0;

/// `void LobbyMan_OnSnapshotReply(LobbyMan *self, Msg *msg)` (0x25e590): a
/// snapshot reply either carries the rider's score in three tag fields
/// (stored summed at_3 `self + idx*4 + 0x104`) or a message to append to the
/// pending list at_3 `self+0xf0`.
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

/// `void cUsbKeyboard_Update(cUsbKeyboard *this)` (0x2668e8): when enabled,
/// poll the USB keyboard library, refresh each port's connected bit (a
/// disconnected port's flags are cleared), then for each connected port with
/// new data append its non-empty keys to the port's 64-entry ring (dropping
/// the oldest when full), and record the held state of key 0x10 and the
/// data's "on" bit in the port flags.
pub fn usb_keyboard_update(c: &mut Cpu) {
    use usbk::*;
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0xc0);
        frame.save(c, S3, 0x80);
        frame.save(c, S0, 0xb0);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S1, 0xa0);
        frame.save(c, S2, 0x90);
        frame.save(c, S4, 0x70);
        frame.save(c, S5, 0x60);
        frame.save(c, S6, 0x50);
        frame.save(c, S7, 0x40);
        frame.save(c, FP, 0x30);
        frame.save_ra(c, 0x20);
        let this = r32(c, S3);
        c.g[V0][0] = Addr(this.wrapping_add(ENABLED)).lw(c);
        let enabled = c.g[V0][0] != 0;
        frame.restore(c, S0, 0xb0);
        if enabled {
            usb_keyboard_poll(c, frame, this)?;
            c.cycles += 1;
            frame.restore(c, S0, 0xb0);
        }
        c.cycles += 11;
        frame.restore(c, S1, 0xa0);
        frame.restore(c, S2, 0x90);
        frame.restore(c, S3, 0x80);
        frame.restore(c, S4, 0x70);
        frame.restore(c, S5, 0x60);
        frame.restore(c, S6, 0x50);
        frame.restore(c, S7, 0x40);
        frame.restore(c, FP, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// The enabled body of [`usb_keyboard_update`].
pub(crate) fn usb_keyboard_poll(c: &mut Cpu, frame: Frame, this: u32) -> crate::call::Flow {
    use usbk::*;
    c.cycles += 5;
    c.g[S1][0] = 0;
    call_ptr(c, usbkb::POLL, 0x0026_692c)?;

    // The connected bit of each of the 8 ports (s1 = port, s0 = slot).
    c.g[S2][0] = 0xffff_ffff_ffff_fffe;
    c.g[S0][0] = c.g[S3][0];
    loop {
        c.cycles += 10;
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, usbkb::IS_CONNECTED, 0x0026_6940)?;
        let flags = Addr(r32(c, S0).wrapping_add(SLOT_FLAGS));
        c.g[V1][0] = flags.lw(c);
        c.g[V0][0] = ((c.g[V0][0] ^ 1) < 1) as u64;
        c.g[V1][0] = (c.g[V1][0] & c.g[S2][0]) | c.g[V0][0];
        c.g[V0][0] = c.g[V1][0] & 1;
        flags.sw(c, c.g[V1][0]);
        if c.g[V0][0] == 0 {
            c.cycles += 1;
            flags.sw(c, 0);
        }
        c.cycles += 4;
        c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
        c.g[V0][0] = ((c.g[S1][0] as i64) < 8) as u64;
        c.g[S0][0] = sx(r32(c, S0).wrapping_add(SLOT_SIZE));
        if c.g[V0][0] == 0 {
            break;
        }
        if event_due(c) {
            deliver_event_here(c, 0x0026_6938);
        }
    }

    // Each port's new data (s0 = port, s7 = slot size, n at_3 sp+0).
    c.cycles += 4;
    c.g[S0][0] = 0;
    call_ptr(c, usbkb::MAX_PORTS, 0x0026_697c)?;
    let none = (c.g[V0][0] as i64) <= 0;
    Addr(frame.at(c, 0)).sw(c, c.g[V0][0]);
    if none {
        return Ok(());
    }
    c.cycles += 3;
    c.g[S7][0] = SLOT_SIZE as u64;
    c.mult(0, V0, S0, S7);
    loop {
        c.cycles += 7;
        c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(this));
        c.g[V0][0] = sx(r32(c, S0).wrapping_add(1));
        let next = Addr(frame.at(c, 4));
        next.sw(c, c.g[V0][0]);
        c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(SLOT_FLAGS)).lw(c) & 1;
        if c.g[V0][0] != 0 {
            c.cycles += 5;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, usbkb::GET_DATA, 0x0026_69b4)?;
            c.g[S1][0] = c.g[V0][0];
            if c.g[S1][0] != 0 {
                usb_keyboard_read_port(c, frame, this)?;
            }
        }
        c.g[S0][0] = next.lw(c);
        c.cycles += 4;
        c.g[V1][0] = Addr(frame.at(c, 0)).lw(c);
        let more = (c.g[S0][0] as i64) < (c.g[V1][0] as i64);
        c.g[V0][0] = more as u64;
        c.mult(0, V0, S0, S7);
        if !more {
            return Ok(());
        }
        if event_due(c) {
            deliver_event_here(c, 0x0026_6990);
        }
    }
}

/// One port with data (`s1`, port `s0`): append its keys to the ring and
/// update the port flags.
pub(crate) fn usb_keyboard_read_port(c: &mut Cpu, frame: Frame, this: u32) -> crate::call::Flow {
    use usbk::*;
    let data = r32(c, S1);
    c.cycles += 3;
    c.g[V0][0] = Addr(data.wrapping_add(DATA_COUNT)).lw(c);
    let empty = (c.g[V0][0] as i64) <= 0;
    c.g[S2][0] = 0;
    if !empty {
        c.cycles += 7;
        c.g[T3][0] = SLOT_SIZE as u64;
        c.g[FP][0] = u64::MAX;
        c.g[S6][0] = sx(this.wrapping_add(SLOT_START));
        c.g[S5][0] = sx(this.wrapping_add(SLOT_RING));
        c.g[S4][0] = sx(this.wrapping_add(SLOT_RING + 1));
        let spill = frame.at(c, 0x10);
        c.sq(T3, spill);
        loop {
            c.cycles += 8;
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = 0;
            c.g[A2][0] = c.g[S2][0];
            call_ptr(c, usbkb::GET_KEY, 0x0026_69f8)?;
            c.g[T1][0] = c.g[V0][0];
            let key = r32(c, T1);
            c.g[V0][0] = Addr(key).lbu(c);
            let mut store = c.g[V0][0] != 0;
            let spill = frame.at(c, 0x10);
            c.lq(T3, spill);
            if !store {
                c.cycles += 3;
                c.g[V0][0] = Addr(key.wrapping_add(1)).lbu(c);
                store = c.g[V0][0] != 0;
                if !store {
                    c.g[V0][0] = Addr(data.wrapping_add(DATA_COUNT)).lw(c);
                }
            }
            if store {
                // idx = count % 64; a full ring advances start.
                c.cycles += 14;
                c.mult(0, T0, S0, T3);
                let slot = r32(c, T0);
                c.g[T2][0] = sx(this.wrapping_add(slot));
                c.g[A3][0] = sx(r32(c, S6).wrapping_add(slot));
                let count_at = Addr(r32(c, T2));
                let start_at = Addr(r32(c, A3));
                c.g[A0][0] = count_at.lw(c);
                c.g[A2][0] = start_at.lw(c);
                c.g[V1][0] = ((c.g[FP][0] as i64) < (c.g[A0][0] as i64)) as u64;
                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x3f));
                if c.g[V1][0] != 0 {
                    c.g[V0][0] = c.g[A0][0];
                }
                c.g[A1][0] = sx((c.g[A0][0] as u32).wrapping_sub(c.g[A2][0] as u32));
                c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 6) as u32);
                c.g[A1][0] = ((c.g[A1][0] as i64) < 64) as u64;
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 6);
                let room = c.g[A1][0] != 0;
                c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                if !room {
                    c.cycles += 2;
                    c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
                    start_at.sw(c, c.g[V0][0]);
                }
                c.cycles += 12;
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 1);
                c.g[A1][0] = Addr(key).lbu(c);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(slot));
                c.g[V0][0] = sx(r32(c, S5).wrapping_add(c.g[V1][0] as u32));
                Addr(r32(c, V0)).sb(c, c.g[A1][0]);
                c.g[V1][0] = sx(r32(c, S4).wrapping_add(c.g[V1][0] as u32));
                c.g[A0][0] = Addr(key.wrapping_add(1)).lbu(c);
                Addr(r32(c, V1)).sb(c, c.g[A0][0]);
                c.g[V0][0] = sx((count_at.lw(c) as u32).wrapping_add(1));
                count_at.sw(c, c.g[V0][0]);
                c.g[V0][0] = Addr(data.wrapping_add(DATA_COUNT)).lw(c);
            }
            c.cycles += 4;
            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(1));
            c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
            if c.g[V0][0] == 0 {
                break;
            }
            let spill = frame.at(c, 0x10);
            c.sq(T3, spill);
            if event_due(c) {
                deliver_event_here(c, 0x0026_69e8);
            }
        }
    }

    // Bit 4: key 0x10 held.
    c.cycles += 7;
    c.g[A0][0] = c.g[S1][0];
    c.g[A1][0] = 0;
    c.g[A2][0] = 0;
    c.g[A3][0] = 0x10;
    call_ptr(c, usbkb::IS_KEY_DOWN, 0x0026_6aa8)?;
    let held = c.g[V0][0] != 0;
    c.mult(0, V0, S0, S7);
    c.cycles += 5;
    if held {
        c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(this));
        let flags = Addr(r32(c, V1).wrapping_add(SLOT_FLAGS));
        c.g[V0][0] = flags.lw(c) | 0x10;
        flags.sw(c, c.g[V0][0]);
    } else {
        c.g[V1][0] = 0xffff_ffff_ffff_ffef;
        c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(this));
        let flags = Addr(r32(c, A0).wrapping_add(SLOT_FLAGS));
        c.g[V0][0] = flags.lw(c) & c.g[V1][0];
        flags.sw(c, c.g[V0][0]);
    }

    // Bits 2, 3 and 1: the data's "on" bit.
    c.cycles += 25;
    c.mult(0, V0, S0, S7);
    c.g[A1][0] = 0xffff_ffff_ffff_fffb;
    c.g[A2][0] = 0xffff_ffff_ffff_fff7;
    c.g[A3][0] = 0xffff_ffff_ffff_fffd;
    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(this));
    let flags = Addr(r32(c, A0).wrapping_add(SLOT_FLAGS));
    for (mask, shift) in [(A1, 2u32), (A2, 3), (A3, 1)] {
        c.g[V0][0] = Addr(data).lw(c);
        if shift == 2 {
            c.g[V1][0] = flags.lw(c);
        }
        c.g[V0][0] = sx(((c.g[V0][0] & 1) as u32) << shift);
        c.g[V1][0] = (c.g[V1][0] & c.g[mask][0]) | c.g[V0][0];
        flags.sw(c, c.g[V1][0]);
    }
    Ok(())
}

/// `int cUsbKeyboard_GetKey(cUsbKeyboard* kb, int ch, int* key, int* mod)`
/// (0x266ba8): pop the next queued key pair of keyboard channel `ch`.
/// `*key` and `*mod` are -1 unless the popped pair's byte is non-zero.
/// Returns 1 if a pair was popped, 0 if the keyboard is closed, `ch` is out
/// of range or the queue is empty.
pub fn c_usb_keyboard_get_key(c: &mut Cpu) {
    use usb_kb::*;
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S2, 0x30);
        frame.save(c, S3, 0x20);
        c.g[S2][0] = u64::MAX;
        frame.save(c, S0, 0x50);
        c.g[S3][0] = c.g[A2][0]; // key
        frame.save(c, S1, 0x40);
        c.g[S0][0] = c.g[A0][0]; // kb
        frame.save(c, S4, 0x10);
        frame.save_ra(c, 0);
        c.g[S4][0] = c.g[A3][0]; // mod
        let (key, modifier, kb) = (Addr(r32(c, S3)), Addr(r32(c, S4)), r32(c, S0));
        key.sw(c, u64::MAX);
        modifier.sw(c, u64::MAX);
        c.g[V0][0] = Addr(kb + ACTIVE).lw(c);
        let active = c.g[V0][0] != 0;
        c.g[S1][0] = c.g[A1][0]; // ch
        let mut popped = false;
        if active {
            c.cycles += 5;
            call_ptr(c, GET_MAX_KEYBOARDS, 0x0026_6bf0)?;
            let in_range = (c.g[S1][0] as i64) < (c.g[V0][0] as i64);
            c.g[V0][0] = (in_range) as u64;
            c.g[V0][0] = CHANNEL_SIZE;
            if in_range {
                c.cycles += 9;
                c.g[V1][0] = sx(kb.wrapping_add(READ));
                c.mult(0, A1, S1, V0);
                let off = r32(c, A1);
                c.g[A3][0] = sx(r32(c, V1).wrapping_add(off));
                c.g[A0][0] = sx(kb.wrapping_add(off));
                let read_at = Addr(r32(c, A3));
                c.g[A2][0] = read_at.lw(c);
                c.g[V0][0] = Addr(r32(c, A0)).lw(c);
                let queued = (c.g[A2][0] as i64) < (c.g[V0][0] as i64);
                c.g[V0][0] = queued as u64;
                c.g[V1][0] = ((c.g[S2][0] as i64) < (c.g[A2][0] as i64)) as u64;
                if queued {
                    // The ring slot `read % 64` (C remainder), two bytes each.
                    c.cycles += 12;
                    let read = r32(c, A2);
                    c.g[V0][0] = if c.g[V1][0] != 0 {
                        c.g[A2][0]
                    } else {
                        sx(read.wrapping_add(0x3f))
                    };
                    c.g[A0][0] = sx(kb.wrapping_add(RING));
                    let rounded = ((r32(c, V0) as i32 >> 6) as u32) << 6;
                    let slot = read.wrapping_sub(rounded) << 1;
                    c.g[V0][0] = sx(slot);
                    c.g[V1][0] = sx(slot.wrapping_add(off));
                    c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, V1)));
                    c.g[V0][0] = Addr(r32(c, A0)).lbu(c);
                    if c.g[V0][0] != 0 {
                        let k = c.g[V0][0];
                        key.sw(c, k);
                    }
                    c.cycles += 5;
                    c.g[V0][0] = sx(kb.wrapping_add(RING + 1).wrapping_add(r32(c, V1)));
                    c.g[V1][0] = Addr(r32(c, V0)).lbu(c);
                    c.g[V0][0] = c.g[V1][0];
                    if c.g[V1][0] != 0 {
                        c.cycles += 1;
                        let m = c.g[V0][0];
                        modifier.sw(c, m);
                    }
                    c.cycles += 5;
                    c.g[V1][0] = read_at.lw(c);
                    c.g[V0][0] = 1;
                    c.g[V1][0] = sx(r32(c, V1).wrapping_add(1));
                    let next = c.g[V1][0];
                    read_at.sw(c, next);
                    popped = true;
                }
            }
        }
        if !popped {
            c.cycles += 1;
            c.g[V0][0] = 0;
        }
        c.cycles += 8;
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `FENetHook` vtable published at `this+8` by the constructor.
pub(crate) const FENET_HOOK_VTABLE: u32 = 0x0048_10c8;

/// `FENetHook2` vtable published at `this+0x14` by the constructor.
pub(crate) const FENET_HOOK2_VTABLE: u32 = 0x0048_10b0;

/// `void LobbyMan_OnMesgReply(void)` (0x25fc20): a lobby message reply stub
/// that returns at once, leaving every register as it found it.
pub fn lobby_man_on_mesg_reply(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `void BuddyMan_OnSendMessageReply(void)` (0x261a20): a buddy message
/// reply stub that returns at once, leaving every register as it found it.
pub fn buddy_man_on_send_message_reply(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `int cSharedLoginMan_RequestLoad(cSharedLoginMan *self)` (0x265df0):
/// record the pending login-manager request code 2 (load) in the manager
/// word and return it in v0.
pub fn c_shared_login_man_request_load(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = 0x2;
    Addr(r32(c, A0)).sw(c, c.g[V0][0]);
    ret(c);
}

/// `int cSharedLoginMan_RequestUpdate(cSharedLoginMan *self)` (0x265e00):
/// record the pending login-manager request code 6 (update) in the manager
/// word and return it in v0.
pub fn c_shared_login_man_request_update(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = 0x6;
    Addr(r32(c, A0)).sw(c, c.g[V0][0]);
    ret(c);
}

/// `int cSharedLoginMan_RequestCreate(cSharedLoginMan *self)` (0x265e10):
/// record the pending login-manager request code 0xa (create) in the
/// manager word and return it in v0.
pub fn c_shared_login_man_request_create(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = 0xa;
    Addr(r32(c, A0)).sw(c, c.g[V0][0]);
    ret(c);
}

/// `int cSharedLoginMan_RequestDelete(cSharedLoginMan *self)` (0x265e20):
/// record the pending login-manager request code 0xe (delete) in the
/// manager word and return it in v0.
pub fn c_shared_login_man_request_delete(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = 0xe;
    Addr(r32(c, A0)).sw(c, c.g[V0][0]);
    ret(c);
}

/// `int cSharedLoginMan_RequestFinish(cSharedLoginMan *self)` (0x265e30):
/// record the pending login-manager request code 0x12 (finish) in the
/// manager word and return it in v0.
pub fn c_shared_login_man_request_finish(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = 0x12;
    Addr(r32(c, A0)).sw(c, c.g[V0][0]);
    ret(c);
}

/// `FENetHook *FENetHook_ctor(FENetHook *self)` (0x263660): publish the
/// `FENetHook` vtable at `this+8`, clear `this`, and return it.
pub fn fe_net_hook_ctor(c: &mut Cpu) {
    c.cycles += 6;
    c.g[V0][0] = c.g[A0][0];
    c.g[V1][0] = FENET_HOOK_VTABLE as u64;
    Addr(r32(c, A0)).sw(c, 0);
    Addr(r32(c, A0).wrapping_add(8)).sw(c, c.g[V1][0]);
    ret(c);
}

/// `FENetHook2 *FENetHook2_ctor(FENetHook2 *self)` (0x263998): publish the
/// `FENetHook2` vtable at `this+0x14`, clear `this`, and return it.
pub fn fe_net_hook2_ctor(c: &mut Cpu) {
    c.cycles += 6;
    c.g[V0][0] = c.g[A0][0];
    c.g[V1][0] = FENET_HOOK2_VTABLE as u64;
    Addr(r32(c, A0)).sw(c, 0);
    Addr(r32(c, A0).wrapping_add(0x14)).sw(c, c.g[V1][0]);
    ret(c);
}

/// `void LobbyMan_OnRecvDebug(LobbyMan *self)` (0x2598a8): call
/// `LobbyApi_DebugPacket(self->api, packet)` on the lobby API at `self+0x54`.
pub fn lobby_man_on_recv_debug(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = Addr(r32(c, A0)).off(0x54).lw(c);
        frame.save_ra(c, 0);
        call_ptr(c, 0x003e_8fd0, 0x0025_98b8)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int cVoipMan_GetSharedStatus(cVoipMan *self)` (0x266f38): return the
/// shared voice status word behind the voice global (`gp+0x2a28`).
pub fn c_voip_man_get_shared_status(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        call_ptr(c, 0x003f_6c40, 0x0026_6f48)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cDSockChannel_BaseDisconnect(cDSockChannel *self)` (0x267da8):
/// reset the channel state, dropping any half-open connection.
pub fn c_dsock_channel_base_disconnect(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        call_ptr(c, 0x0025_5840, 0x0026_7db8)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int LobbyMan_CompareUserNames(void *a, void *b)` (0x259398): compare the
/// user names at `a2+0x1c` and `a3+0x1c` case-insensitively.
pub fn lobby_man_compare_user_names(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = sx(r32(c, A2).wrapping_add(0x1c));
        frame.save_ra(c, 0);
        c.g[A1][0] = sx(r32(c, A3).wrapping_add(0x1c));
        call_ptr(c, 0x0041_aa88, 0x0025_93ac)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int LobbyMan_CompareRoomNames(void *a, void *b)` (0x2593b8): compare the
/// room names at `a2+0x8` and `a3+0x8` case-insensitively.
pub fn lobby_man_compare_room_names(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = sx(r32(c, A2).wrapping_add(0x8));
        frame.save_ra(c, 0);
        c.g[A1][0] = sx(r32(c, A3).wrapping_add(0x8));
        call_ptr(c, 0x0041_aa88, 0x0025_93cc)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void LobbyMan_SendDecline(LobbyMan *self, int to)` (0x25fc28): send
/// the decline line for `to` through `LobbyMan_SendPrivateMessage` with
/// the text at `gp+0x180`.
pub fn lobby_man_send_decline(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A2][0] = 0x4a0000;
        frame.save_ra(c, 0);
        c.g[A2][0] = sx(r32(c, A2).wrapping_add(0x3270));
        call_ptr(c, 0x0025_fb80, 0x0025_fc3c)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `bool BuddyMan_IsBuddyAt(BuddyMan *self, int idx)` (0x262390): whether
/// the roster entry at `idx` is a buddy (flag 4 at `entry+0xa8`).
pub fn buddy_man_is_buddy_at(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        call_ptr(c, 0x0026_2620, 0x0026_23a0)?;
        frame.restore_ra(c, 0);
        c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
        frame.close(c);
        Ok(())
    })
}

/// `bool BuddyMan_IsBlockedAt(BuddyMan *self, int idx)` (0x2623e0): whether
/// the roster entry at `idx` is blocked (flag 0x200 at `entry+0xa8`).
pub fn buddy_man_is_blocked_at(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        call_ptr(c, 0x0026_2658, 0x0026_23f0)?;
        frame.restore_ra(c, 0);
        c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
        frame.close(c);
        Ok(())
    })
}

/// `void LobbyMan_SelectPersonaIndex(LobbyMan *self, int idx)` (0x25cd28):
/// select the persona named by the `idx`-th entry of the table at
/// `self+0x70` through `LobbyMan_SelectPersona`.
pub fn lobby_man_select_persona_index(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = sx(r32(c, A1) << 2);
        frame.save_ra(c, 0);
        c.g[A1][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
        c.g[A1][0] = Addr(r32(c, A1).wrapping_add(0x70)).lw(c);
        call_ptr(c, 0x0025_cd50, 0x0025_cd40)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void LobbyMan_SendAbortChal(LobbyMan *self)` (0x25fc48): send the abort
/// line for the pending challenge at `self+0x98` through
/// `LobbyMan_SendPrivateMessage` with the canned text at `0x4807f8`.
pub fn lobby_man_send_abort_chal(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        c.g[A2][0] = 0x480000;
        frame.save_ra(c, 0);
        c.g[A2][0] = sx(r32(c, A2).wrapping_add(0x7f8));
        c.g[A1][0] = Addr(r32(c, A0)).off(0x98).lw(c);
        call_ptr(c, 0x0025_fb80, 0x0025_fc60)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cSharedLoginMan *cSharedLoginMan_ctor(cSharedLoginMan *self)`
/// (0x2658e0): initialise the shared login manager in place and return it.
pub fn c_shared_login_man_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0026_5950, 0x0026_58f4)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void cUsbKeyboard_Destroy(void)` (0x2667c0): destroy the USB keyboard
/// singleton at `gp+0x300` and clear the slot when one exists.
pub fn c_usb_keyboard_destroy(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[A0][0] = Addr(r32(c, GP).wrapping_add(0x300)).lw(c);
        let absent = c.g[A0][0] == 0;
        frame.save_ra(c, 0);
        if !absent {
            c.cycles += 3;
            c.g[A1][0] = 0x3;
            call_ptr(c, callee::C_USB_KEYBOARD_DTOR.addr, 0x0026_67d8)?;
            Addr(r32(c, GP).wrapping_add(0x300)).sw(c, 0);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int cVoipMan_QueryState(cVoipMan *self)` (0x266f08): the voice
/// active flag from `VoipRpc_IsActive`, or 0 when the manager word is clear.
pub fn c_voip_man_query_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = Addr(r32(c, A0)).lw(c);
        let present = c.g[V0][0] != 0;
        frame.save_ra(c, 0);
        if !present {
            c.cycles += 2;
            c.g[V0][0] = 0;
        } else {
            c.cycles += 2;
            call_ptr(c, 0x003f_6b80, 0x0026_6f28)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void LobbyMan_CancelLogin(LobbyMan *self)` (0x25c878): cancel the
/// pending login request through `LobbyApi_CancelRequest` when the lobby is
/// in state 8.
pub fn lobby_man_cancel_login(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        c.g[V1][0] = 0x8;
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(r32(c, A0)).lw(c);
        let in_login = c.g[V0][0] == c.g[V1][0];
        frame.restore_ra(c, 0);
        if in_login {
            c.cycles += 4;
            c.g[A1][0] = Addr(r32(c, A0)).off(0x40).lw(c);
            c.g[A0][0] = Addr(r32(c, A0)).off(0x54).lw(c);
            call_ptr(c, 0x003e_8d70, 0x0025_c89c)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `void LobbyMan_Create(void)` (0x25b650): allocate the lobby manager and
/// construct it in place, publishing it at `gp-0xc8`.
pub fn lobby_man_create(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = 0x480000;
        frame.save_ra(c, 0);
        c.g[A1][0] = sx(r32(c, A1).wrapping_add(0x838));
        c.g[A2][0] = 0x2000_0000;
        c.g[A3][0] = 0;
        c.g[A0][0] = 0x300;
        call_ptr(c, 0x0031_7d70, 0x0025_b670)?;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0025_94d0, 0x0025_b678)?;
        Addr(gp_8(c, LOBBY_INSTANCE_GP)).sw(c, c.g[V0][0]);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void *BuddyMan_GetBuddyAt(BuddyMan *self, int idx)` (0x262620): the
/// roster entry at `idx` when it is a buddy (flag 4 at `entry+0xa8`), else
/// 0.
pub fn buddy_man_get_buddy_at(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        call_ptr(c, 0x0026_25c8, 0x0026_2630)?;
        if c.g[V0][0] == 0 {
            c.g[V0][0] = 0;
            // +1 here and +2 below: the generated +1s share one tail block.
            c.cycles += 1;
        } else {
            c.cycles += 4;
            c.g[V1][0] = Addr(r32(c, V0).wrapping_add(0xa8)).lw(c);
            c.g[V1][0] = c.g[V1][0] & 0x4;
            let buddy = c.g[V1][0] != 0;
            if !buddy {
                c.cycles += 2;
                c.g[V0][0] = 0;
            }
        }
        c.cycles += 2;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void *BuddyMan_GetBlockedAt(BuddyMan *self, int idx)` (0x262658): the
/// roster entry at `idx` when it is blocked (flag 0x200 at `entry+0xa8`),
/// else 0.
pub fn buddy_man_get_blocked_at(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        call_ptr(c, 0x0026_25c8, 0x0026_2668)?;
        if c.g[V0][0] == 0 {
            c.g[V0][0] = 0;
            // +1 here and +2 below: the generated +1s share one tail block.
            c.cycles += 1;
        } else {
            c.cycles += 4;
            c.g[V1][0] = Addr(r32(c, V0).wrapping_add(0xa8)).lw(c);
            c.g[V1][0] = c.g[V1][0] & 0x200;
            let blocked = c.g[V1][0] != 0;
            if !blocked {
                c.cycles += 2;
                c.g[V0][0] = 0;
            }
        }
        c.cycles += 2;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cSharedLoginMan_Create(void)` (0x265880): allocate the shared
/// login manager and construct it in place, publishing it at `gp+0x2e0`.
pub fn c_shared_login_man_create(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = 0x480000;
        frame.save_ra(c, 0);
        c.g[A1][0] = sx(r32(c, A1).wrapping_add(0xf30));
        c.g[A2][0] = 0x2000_0000;
        c.g[A3][0] = 0;
        c.g[A0][0] = 0x8;
        call_ptr(c, 0x0031_7d70, 0x0026_58a0)?;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0026_58e0, 0x0026_58a8)?;
        Addr(gp_2(c, 0x2e0)).sw(c, c.g[V0][0]);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cSharedLoginMan_McSeek(void *unused, int a1, int a2)` (0x2662e8):
/// pass the seek request (`a1`, with `a2` reduced to 0/1) to the card
/// manager at `gp+0x2e8`, then post card state 5 at `gp+0x2e4`.
pub fn c_shared_login_man_mc_seek(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0;
        let skip = c.g[A2][0] == 0;
        frame.save_ra(c, 0);
        if !skip {
            c.cycles += 2;
            c.g[V0][0] = c.g[A2][0] ^ 0x1;
            c.g[V0][0] = (c.g[V0][0] < 0x1) as u64;
        }
        c.cycles += 8;
        c.g[A0][0] = Addr(gp_2(c, 0x2e8)).lw(c);
        c.g[A2][0] = c.g[V0][0];
        call_ptr(c, 0x002c_6a60, 0x0026_630c)?;
        c.g[V0][0] = 0x5;
        frame.restore_ra(c, 0);
        Addr(gp_2(c, 0x2e4)).sw(c, c.g[V0][0]);
        frame.close(c);
        Ok(())
    })
}

/// `void cUsbKeyboard_Create(void)` (0x266788): allocate the USB keyboard
/// object and construct it in place, publishing it at `gp+0x300`.
pub fn c_usb_keyboard_create(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = 0x480000;
        frame.save_ra(c, 0);
        c.g[A1][0] = sx(r32(c, A1).wrapping_add(0x1090));
        c.g[A2][0] = 0x2000_0000;
        c.g[A3][0] = 0;
        c.g[A0][0] = 0x464;
        call_ptr(c, 0x0031_7d70, 0x0026_67a8)?;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0026_67e8, 0x0026_67b0)?;
        Addr(gp_2(c, 0x300)).sw(c, c.g[V0][0]);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cVoipMan_ToggleActive(cVoipMan *self)` (0x266ed0): deactivate the
/// voice manager when its word at `+4` is set, else activate it.
pub fn c_voip_man_toggle_active(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(r32(c, A0)).off(4).lw(c);
        let active = c.g[V0][0] != 0;
        if active {
            c.cycles += 4;
            call_ptr(c, 0x0026_6e88, 0x0026_6eec)?;
            frame.restore_ra(c, 0);
        } else {
            c.cycles += 3;
            call_ptr(c, 0x0026_6e30, 0x0026_6efc)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Net_UpdateConnect (0x263828)

pub(crate) const NET_DISCONNECT_ALL: Callee = Callee {
    addr: 0x0026_36f0,
    f: ee::s07::Net_DisconnectAll,
};

/// `gp-0x238`: the comm system (`+0x20` its state: 5 connected, 6 failed;
/// `+0x28` the connection error); `gp-0xc8`: the lobby manager (`+0xf0` its
/// message list head); `gp-0x848`: the app (`+0x14` the frame time).
pub(crate) const NET_COMM_SYSTEM_GP: u32 = 0x238;

pub(crate) const NET_LOBBY_MAN_GP: u32 = 0xc8;

/// `gp+0x238`: the buddy manager (`BuddyMan`).
pub(crate) const BUDDY_MAN_GP: i32 = 0x238;

/// The connect timer: `+0` state (1 connecting, 2 connected), `+4` the time
/// left.
pub(crate) const NET_CONNECT_STATE: u32 = 0x0;

pub(crate) const NET_CONNECT_TIMER: u32 = 0x4;

/// How [`net_update_connect`] leaves: through which of the original's three
/// epilogue entries (each restores fewer registers and costs less).
#[derive(Clone, Copy)]
pub(crate) enum NetConnectExit {
    /// `lq s0; lq s1; ld ra` (5 cycles).
    Full,
    /// `lq s1; ld ra` (4).
    S1,
    /// `ld ra` (3).
    Ra,
}

/// `void Net_UpdateConnect(DsTimer* self)` (0x263828): the connect step.
/// Connecting (state 1): on the comm system reaching state 5 become
/// connected and post message 0x105 to the lobby manager; on its failure
/// (state 6) or when the timer runs out disconnect (with the comm error or
/// 0x2d3f3f3f); else count the timer down by the frame time. Connected
/// (state 2): disconnect when the comm system failed.
pub fn net_update_connect(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x50);
        c.g[V0][0] = 1;
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        frame.save_ra(c, 0x20);
        let gp = c.g[GP][0] as u32;
        c.g[V1][0] = Addr(oc391_at(c, A0, NET_CONNECT_STATE)).lw(c);
        let connecting = c.g[V1][0] == c.g[V0][0];
        c.g[V0][0] = ((c.g[V1][0] as i64) < 2) as u64;
        let exit = if connecting {
            net_update_connect_connecting(c, gp)?
        } else {
            c.cycles += 2;
            if c.g[V0][0] != 0 {
                NetConnectExit::S1
            } else {
                c.cycles += 3;
                c.g[V0][0] = 2;
                let connected = c.g[V1][0] == c.g[V0][0];
                c.g[V1][0] = Addr(gp.wrapping_sub(NET_COMM_SYSTEM_GP)).lw(c);
                if connected {
                    c.cycles += 4;
                    c.g[V0][0] = Addr(oc391_at(c, V1, 0x20)).lw(c) ^ 6;
                    if c.g[V0][0] != 0 {
                        NetConnectExit::S1
                    } else {
                        c.cycles += 2;
                        c.g[A1][0] = Addr(oc391_at(c, V1, 0x28)).lw(c);
                        NET_DISCONNECT_ALL.call(c, 0x0026_3980)?;
                        NetConnectExit::Full
                    }
                } else {
                    c.cycles += 2;
                    NetConnectExit::Ra
                }
            }
        };
        c.cycles += match exit {
            NetConnectExit::Full => 5,
            NetConnectExit::S1 => 4,
            NetConnectExit::Ra => 3,
        };
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// State 1 of [`net_update_connect`] (a0 = self).
pub(crate) fn net_update_connect_connecting(
    c: &mut Cpu,
    gp: u32,
) -> Result<NetConnectExit, crate::call::Unwound> {
    c.cycles += 5;
    c.g[A1][0] = Addr(gp.wrapping_sub(NET_COMM_SYSTEM_GP)).lw(c);
    c.g[V1][0] = Addr(oc391_at(c, A1, 0x20)).lw(c);
    let up = c.g[V1][0] == 5;
    c.g[V0][0] = c.g[V1][0] ^ 6;
    if up {
        // Connected: state 2 and message 0x105 to the lobby manager.
        c.cycles += 27;
        c.g[V0][0] = 2;
        Addr(oc391_at(c, A0, NET_CONNECT_STATE)).sw(c, 2);
        c.g[A1][0] = 0x0048_0488; // the allocation tag
        c.g[S0][0] = Addr(gp.wrapping_sub(NET_LOBBY_MAN_GP)).lw(c);
        c.g[A0][0] = 8;
        c.g[A2][0] = 0x2000_0000;
        c.g[A3][0] = 0;
        call_ptr(c, 0x0031_7d70, 0x0026_389c)?; // EA_MemAllocNamed
        c.g[S1][0] = c.g[V0][0];
        c.g[V1][0] = 0x105;
        let msg = r32(c, S1);
        Addr(msg).sw(c, 0x105);
        c.g[V0][0] = 0x0048_12b0; // the message vtable
        Addr(msg + 4).sw(c, c.g[V0][0]);
        c.g[A1][0] = 0x0048_04a8;
        c.g[A0][0] = 0xc;
        c.g[A2][0] = 0x2000_0000;
        c.g[V0][0] = Addr(oc391_at(c, S0, 0xf0)).lw(c); // the list head
        c.g[A3][0] = 0;
        Addr(sp(c) + 0x10).sw(c, c.g[V0][0]);
        call_ptr(c, 0x0031_7d70, 0x0026_38d4)?;
        // Link the node {next, prev, msg} in before the head.
        c.g[A1][0] = c.g[V0][0];
        let node = r32(c, A1);
        c.g[V0][0] = sx(node.wrapping_add(8));
        if c.g[V0][0] != 0 {
            Addr(node + 8).sw(c, c.g[S1][0]);
        }
        c.cycles += 10;
        let sp0 = sp(c);
        c.g[V0][0] = Addr(sp0 + 0x10).lw(c);
        Addr(node).sw(c, c.g[V0][0]);
        c.g[V1][0] = Addr(oc391_at(c, V0, 4)).lw(c);
        Addr(node + 4).sw(c, c.g[V1][0]);
        c.g[A0][0] = Addr(oc391_at(c, V0, 4)).lw(c);
        Addr(r32(c, A0)).sw(c, c.g[A1][0]);
        c.g[V0][0] = Addr(sp0 + 0x10).lw(c);
        Addr(oc391_at(c, V0, 4)).sw(c, c.g[A1][0]);
        Addr(sp0).sw(c, c.g[A1][0]);
        return Ok(NetConnectExit::Full);
    }
    c.cycles += 2;
    if c.g[V0][0] == 0 {
        // The comm system failed.
        c.cycles += 4;
        c.g[A1][0] = Addr(oc391_at(c, A1, 0x28)).lw(c);
        NET_DISCONNECT_ALL.call(c, 0x0026_391c)?;
        return Ok(NetConnectExit::S1);
    }
    c.lwc1(1, oc391_at(c, A0, NET_CONNECT_TIMER));
    c.cycles += 5;
    c.mtc1(0, 0);
    c.c_lt_s(1, 0);
    c.g[V0][0] = 1;
    if !c.fcc() {
        c.cycles += 1;
        c.g[V0][0] = 0;
    }
    c.cycles += 2;
    c.g[A1][0] = 0x2d3f_0000;
    if c.g[V0][0] != 0 {
        // Timed out.
        c.cycles += 4;
        c.g[A1][0] |= 0x3f3f;
        NET_DISCONNECT_ALL.call(c, 0x0026_394c)?;
        return Ok(NetConnectExit::S1);
    }
    c.cycles += 5;
    c.g[V0][0] = Addr(gp.wrapping_sub(0x848)).lw(c);
    c.lwc1(0, oc391_at(c, V0, 0x14));
    c.sub_s(0, 1, 0);
    c.swc1(0, oc391_at(c, A0, NET_CONNECT_TIMER));
    Ok(NetConnectExit::Full)
}

pub(crate) mod tag {

    pub const WHEN: u32 = 0x004a_2f80;

    pub const REPT: u32 = 0x004a_3278;

    pub const AUTH: u32 = 0x004a_2f78;

    pub const COMMERR: u32 = 0x004a_3280;

    pub const SCOMERR: u32 = 0x004a_3288;

    pub const FINISH0: u32 = 0x004a_3290;

    pub const FINISH1: u32 = 0x004a_3298;

    pub const DNF0: u32 = 0x004a_32a0;

    pub const DNF1: u32 = 0x004a_32a8;

    pub const DESYNC: u32 = 0x004a_32b0;

    pub const NAME0: u32 = 0x004a_32b8;

    pub const QUIT0: u32 = 0x004a_32c0;

    pub const CHEAT0: u32 = 0x004a_32c8;

    pub const DISC0: u32 = 0x004a_32d0;

    pub const SCORE0: u32 = 0x004a_32d8;

    pub const POINTS0: u32 = 0x004a_32e0;

    pub const SCORE1: u32 = 0x004a_32e8;

    pub const NAME1: u32 = 0x004a_32f0;

    pub const QUIT1: u32 = 0x004a_32f8;

    pub const CHEAT1: u32 = 0x004a_3300;

    pub const DISC1: u32 = 0x004a_3308;

    pub const POINTS1: u32 = 0x004a_3310;

    pub const PINGRSLT: u32 = 0x0048_0920;

    pub const STALLCNT: u32 = 0x0048_0930;

    pub const CERRSTAT: u32 = 0x0048_0940;

    pub const CERRFLGS: u32 = 0x0048_0950;

    pub const DIDQUIT0: u32 = 0x0048_0960;

    pub const DIDQUIT1: u32 = 0x0048_0970;

    pub const RSLTSLCK: u32 = 0x0048_0980;

    pub const RACETRA0: u32 = 0x0048_0990;

    pub const RACEEVE0: u32 = 0x0048_09a0;

    pub const CHARACTER0: u32 = 0x0048_09b0;

    pub const RACETIM0: u32 = 0x0048_09c0;

    /// First per-rider stat tag for rider 0 (`SPINDEG0`); the rest follow at
    /// +0x10 steps through `PLOSTRE0`, except the `POINTS0` tag.
    pub const STATS0_BASE: u32 = 0x0048_09d0;

    pub const CHARACTER1: u32 = 0x0048_0be0;

    pub const RACETIM1: u32 = 0x0048_0bf0;

    /// First per-rider stat tag for rider 1 (`SPINDEG1`).
    pub const STATS1_BASE: u32 = 0x0048_0c00;
}

/// Size of the report record built on the stack.
pub(crate) const REPORT_BUF_SIZE: u32 = 0x800;

/// `EA_MemAllocNamed` name string for the queued lobby message.
pub(crate) const NET_EVENT_TAG: u32 = 0x0048_0488;

/// Vtable stored in the queued lobby message.
pub(crate) const NET_EVENT_VTABLE: u32 = 0x0048_12b0;

/// The `'rank'` request kind passed to `LobbyMan_SendRequest`.
pub(crate) const REQ_RANK: u32 = 0x7261_6e6b;

/// `lw reg, GAME_COMM_GP(gp)`: the report reloads the game-comm pointer
/// before each of its field reads, into the register the compiled code uses.
pub(crate) fn load_game_comm(c: &mut Cpu, reg: usize) {
    c.g[reg][0] = c.lw(gp_8(c, GAME_COMM_GP));
}

/// `TagFieldSetNumber(sp, 0x800, tag, a3)`: the caller sets `a3` itself
/// (a stats load, a stack spill, or `v0` after `__fixunssfsi`).
pub(crate) fn set_number(c: &mut Cpu, tag_addr: u32, ret_addr: u32) -> crate::call::Flow {
    c.g[A0][0] = c.g[SP][0];
    c.g[A1][0] = REPORT_BUF_SIZE as u64;
    c.g[A2][0] = sx(tag_addr);
    callee::TAG_FIELD_SET_NUMBER.call(c, ret_addr)
}

/// Return addresses of one rider's stat-block calls, in call order.
pub(crate) struct RiderRets {
    ints: [u32; 14],
    float_fix: [u32; 10],
    float_num: [u32; 10],
    tail: [u32; 10],
}

pub(crate) const RIDER_STATS_INT_OFFS: [u32; 14] = [
    0x00, 0x04, 0x08, 0x10, 0x14, 0x18, 0x24, 0x28, 0x2c, 0x30, 0x34, 0x38, 0x3c, 0x40,
];

pub(crate) const RIDER_STATS_FLOAT_OFFS: [u32; 10] =
    [0x44, 0x48, 0x4c, 0x50, 0x54, 0x58, 0x60, 0x64, 0x68, 0x6c];

pub(crate) const RIDER_STATS_TAIL_OFFS: [u32; 10] =
    [0x70, 0x74, 0x78, 0x7c, 0x80, 0x84, 0x90, 0x9c, 0xa4, 0xa8];

/// One rider's stat block: 14 int fields, 10 float fields converted with
/// `__fixunssfsi`, then 10 more int fields (the `+0x9c` field takes
/// `points_tag` instead of the sequential tag). The two generated blocks
/// have this exact shape apart from the stats register, the tag base and
/// the return addresses, which are all parameters.
pub(crate) fn report_rider_stats(
    c: &mut Cpu,
    stats_reg: usize,
    base_tag: u32,
    points_tag: u32,
    rets: &RiderRets,
) -> crate::call::Flow {
    for (i, &off) in RIDER_STATS_INT_OFFS.iter().enumerate() {
        c.g[A3][0] = c.lw((c.g[stats_reg][0] as u32).wrapping_add(off));
        set_number(c, base_tag.wrapping_add(0x10 * i as u32), rets.ints[i])?;
    }
    for (i, &off) in RIDER_STATS_FLOAT_OFFS.iter().enumerate() {
        let tag_addr = base_tag.wrapping_add(0x10 * (14 + i) as u32);
        c.lwc1(12, (c.g[stats_reg][0] as u32).wrapping_add(off));
        c.g[S0][0] = sx(tag_addr);
        call_ptr(c, callee::FLOAT_TO_UINT, rets.float_fix[i])?;
        c.g[A3][0] = c.g[V0][0];
        set_number(c, tag_addr, rets.float_num[i])?;
    }
    for (i, &off) in RIDER_STATS_TAIL_OFFS.iter().enumerate() {
        let tag_addr = if i == 7 {
            points_tag
        } else if i < 7 {
            base_tag.wrapping_add(0x10 * (24 + i) as u32)
        } else {
            base_tag.wrapping_add(0x10 * (23 + i) as u32)
        };
        c.g[A3][0] = c.lw((c.g[stats_reg][0] as u32).wrapping_add(off));
        set_number(c, tag_addr, rets.tail[i])?;
    }
    Ok(())
}

pub(crate) const RIDER0_RETS: RiderRets = RiderRets {
    ints: [
        0x0026_0378,
        0x0026_0390,
        0x0026_03a8,
        0x0026_03c0,
        0x0026_03d8,
        0x0026_03f0,
        0x0026_0408,
        0x0026_0420,
        0x0026_0438,
        0x0026_0450,
        0x0026_0468,
        0x0026_0480,
        0x0026_0498,
        0x0026_04b0,
    ],
    float_fix: [
        0x0026_04c0,
        0x0026_04e4,
        0x0026_0508,
        0x0026_052c,
        0x0026_0550,
        0x0026_0574,
        0x0026_0598,
        0x0026_05bc,
        0x0026_05e0,
        0x0026_0604,
    ],
    float_num: [
        0x0026_04d4,
        0x0026_04f8,
        0x0026_051c,
        0x0026_0540,
        0x0026_0564,
        0x0026_0588,
        0x0026_05ac,
        0x0026_05d0,
        0x0026_05f4,
        0x0026_0618,
    ],
    tail: [
        0x0026_0630,
        0x0026_0648,
        0x0026_0660,
        0x0026_0678,
        0x0026_0690,
        0x0026_06a8,
        0x0026_06c0,
        0x0026_06d8,
        0x0026_06f0,
        0x0026_0708,
    ],
};

/// `void LobbyMan_SendRaceReport(LobbyMan* self, int auth, int player0, int
/// player1, void* stats0, void* stats1)` (0x25fd50): build the post-race
/// "rank" report record (WHEN/REPT/AUTH, race flags, both riders' stat
/// tags) in a 0x800-byte stack buffer, send it with `LobbyMan_SendRequest`,
/// and queue the NetEvent message on the lobby list. Returns at once when
/// `*self == 0`.
pub fn lobby_man_send_race_report(c: &mut Cpu) {
    guest(c, |c| {
        {
            c.cycles += 19;
            let frame = Frame::open(c, 0x900);
            frame.save(c, S0, 0x8f0);
            frame.save(c, S1, 0x8e0);
            c.g[S0][0] = c.g[A2][0];
            frame.save(c, S3, 0x8c0);
            c.g[S1][0] = c.g[A3][0];
            frame.save(c, S5, 0x8a0);
            c.g[S3][0] = c.g[T1][0];
            frame.save(c, S2, 0x8d0);
            frame.save(c, S4, 0x8b0);
            frame.save(c, S6, 0x890);
            frame.save(c, S7, 0x880);
            frame.save(c, FP, 0x870);
            frame.save_ra(c, 0x860);
            c.sw(frame.at(c, 0x820), c.g[A0][0]);
            c.sw(frame.at(c, 0x824), c.g[A1][0]);
            c.g[V0][0] = c.lw(c.g[A0][0] as u32);
            let no_report = c.g[V0][0] == 0;
            c.g[S5][0] = c.g[T0][0];
            if !no_report {
                {
                    c.cycles += 18;
                    call_ptr(c, callee::BE_LIBRARY_GET_INSTANCE, 0x0025_fda4)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A1][0] = 0;
                    call_ptr(c, callee::BE_LIBRARY_GET_INTERFACE, 0x0025_fdb0)?;
                    c.sw(frame.at(c, 0x828), c.g[V0][0]);
                    c.g[A1][0] = 0x7;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x78));
                    call_ptr(c, callee::BE_LIBRARY_GET_INTERFACE, 0x0025_fdc4)?;
                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.g[T4][0] = 0;
                    load_game_comm(c, A0);
                    c.g[T3][0] = 0;
                    c.g[V1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc0));
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x18));
                    let no_flags = c.g[V0][0] == 0;
                    c.sw(frame.at(c, 0x82c), c.g[V1][0]);
                    if !no_flags {
                        c.cycles += 3;
                        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x9c));
                        if c.g[V0][0] != 0 {
                            c.g[T3][0] = 0x1;
                        } else {
                            c.cycles += 1;
                            c.g[T4][0] = 0x1;
                        }
                    }
                }
                {
                    let swapped;
                    {
                        c.cycles += 13;
                        load_game_comm(c, V0);
                        c.g[V1][0] = 0x530000;
                        c.g[A0][0] = 0x1;
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4b30));
                        c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xa0));
                        c.g[T0][0] = c.g[V0][0];
                        c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xa4));
                        c.g[T2][0] = c.g[A0][0];
                        c.g[T1][0] = c.g[A0][0];
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
                        if c.g[A1][0] != 0 {
                            c.g[T2][0] = 0;
                        }
                        swapped = c.g[V0][0] == 0;
                        if c.g[A2][0] != 0 {
                            c.g[T1][0] = 0;
                        }
                        if !swapped {
                            // Player order from the 0x534b30 table: rider 0
                            // is finish0/stats0 (report+0x90 name).
                            c.cycles += 4;
                            c.sw(frame.at(c, 0x830), c.g[S0][0]);
                            c.g[V0][0] = c.lw(frame.at(c, 0x820));
                            c.g[S2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x90));
                            if c.g[T1][0] == 0 {
                                c.cycles += 1;
                                c.sw(frame.at(c, 0x834), 0);
                            } else {
                                c.cycles += 3;
                                c.g[V1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x38));
                                let zero = c.g[V1][0] == 0;
                                c.g[A3][0] = 0;
                                if !zero {
                                    c.cycles += 2;
                                    c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x34));
                                    c.g[A3][0] =
                                        sx((c.g[V1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                                }
                                c.cycles += 2;
                                c.sw(frame.at(c, 0x834), c.g[A3][0]);
                            }
                            c.cycles += 2;
                            if c.g[T4][0] == 0 {
                                c.sw(frame.at(c, 0x838), 0);
                            } else {
                                c.cycles += 3;
                                c.g[V1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x38));
                                let zero = c.g[V1][0] == 0;
                                c.g[A3][0] = 0;
                                if !zero {
                                    c.cycles += 2;
                                    c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x34));
                                    c.g[A3][0] =
                                        sx((c.g[V1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                                }
                                c.cycles += 1;
                                c.sw(frame.at(c, 0x838), c.g[A3][0]);
                            }
                            c.cycles += 15;
                            c.g[V0][0] = 0x1;
                            c.g[V1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x10));
                            c.g[FP][0] = c.g[V0][0];
                            c.g[S7][0] = c.g[V0][0];
                            c.g[V0][0] = 0x530000;
                            c.g[A0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0xa0));
                            c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4b30));
                            if c.g[V1][0] == 0 {
                                c.g[S7][0] = 0;
                            }
                            c.g[A1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x32c));
                            if c.g[A0][0] == 0 {
                                c.g[FP][0] = 0;
                            }
                            c.g[S4][0] = c.g[S5][0];
                            c.sw(frame.at(c, 0x840), c.g[A1][0]);
                            c.g[V1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x24));
                            c.sw(frame.at(c, 0x83c), c.g[V1][0]);
                            if c.g[T2][0] == 0 {
                                c.cycles += 1;
                                c.sw(frame.at(c, 0x844), 0);
                            } else {
                                c.cycles += 3;
                                c.g[V1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x3c));
                                let zero = c.g[V1][0] == 0;
                                c.g[A3][0] = 0;
                                if !zero {
                                    c.cycles += 2;
                                    c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x34));
                                    c.g[A3][0] =
                                        sx((c.g[V1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                                }
                                c.cycles += 2;
                                c.sw(frame.at(c, 0x844), c.g[A3][0]);
                            }
                            c.cycles += 2;
                            if c.g[T3][0] == 0 {
                                c.sw(frame.at(c, 0x848), 0);
                            } else {
                                c.cycles += 3;
                                c.g[V1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x3c));
                                let zero = c.g[V1][0] == 0;
                                c.g[A3][0] = 0;
                                if !zero {
                                    c.cycles += 2;
                                    c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x34));
                                    c.g[A3][0] =
                                        sx((c.g[V1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                                }
                                c.cycles += 1;
                                c.sw(frame.at(c, 0x848), c.g[A3][0]);
                            }
                            c.cycles += 9;
                            c.g[V0][0] = 0x1;
                            load_game_comm(c, V1);
                            c.g[S6][0] = c.g[V0][0];
                            c.g[S0][0] = c.g[V0][0];
                            c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xa4));
                            c.g[S5][0] = c.g[S3][0];
                            c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x14));
                            c.g[V1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x28));
                        }
                    }
                    if swapped {
                        // Player order swapped: rider 0 is finish1/stats1
                        // (report+0x94 name).
                        c.cycles += 4;
                        c.sw(frame.at(c, 0x830), c.g[S1][0]);
                        c.g[A0][0] = c.lw(frame.at(c, 0x820));
                        c.g[S2][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x94));
                        if c.g[T2][0] == 0 {
                            c.cycles += 1;
                            c.sw(frame.at(c, 0x834), 0);
                        } else {
                            c.cycles += 3;
                            c.g[V1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x3c));
                            let zero = c.g[V1][0] == 0;
                            c.g[A3][0] = 0;
                            if !zero {
                                c.cycles += 2;
                                c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x34));
                                c.g[A3][0] =
                                    sx((c.g[V1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                            }
                            c.cycles += 2;
                            c.sw(frame.at(c, 0x834), c.g[A3][0]);
                        }
                        c.cycles += 2;
                        if c.g[T3][0] == 0 {
                            c.sw(frame.at(c, 0x838), 0);
                        } else {
                            c.cycles += 3;
                            c.g[V1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x3c));
                            let zero = c.g[V1][0] == 0;
                            c.g[A3][0] = 0;
                            if !zero {
                                c.cycles += 2;
                                c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x34));
                                c.g[A3][0] =
                                    sx((c.g[V1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                            }
                            c.cycles += 1;
                            c.sw(frame.at(c, 0x838), c.g[A3][0]);
                        }
                        c.cycles += 16;
                        c.g[V0][0] = 0x1;
                        c.g[V1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x14));
                        c.g[FP][0] = c.g[V0][0];
                        c.g[S7][0] = c.g[V0][0];
                        c.g[V0][0] = 0x530000;
                        c.g[A0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0xa4));
                        c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4b30));
                        if c.g[V1][0] == 0 {
                            c.g[S7][0] = 0;
                        }
                        c.g[A1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x328));
                        if c.g[A0][0] == 0 {
                            c.g[FP][0] = 0;
                        }
                        c.g[S4][0] = c.g[S3][0];
                        c.g[S1][0] = c.g[S0][0];
                        c.sw(frame.at(c, 0x840), c.g[A1][0]);
                        c.g[V1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x28));
                        c.sw(frame.at(c, 0x83c), c.g[V1][0]);
                        if c.g[T1][0] == 0 {
                            c.cycles += 1;
                            c.sw(frame.at(c, 0x844), 0);
                        } else {
                            c.cycles += 3;
                            c.g[V1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x38));
                            let zero = c.g[V1][0] == 0;
                            c.g[A3][0] = 0;
                            if !zero {
                                c.cycles += 2;
                                c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x34));
                                c.g[A3][0] =
                                    sx((c.g[V1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                            }
                            c.cycles += 2;
                            c.sw(frame.at(c, 0x844), c.g[A3][0]);
                        }
                        c.cycles += 2;
                        if c.g[T4][0] == 0 {
                            c.sw(frame.at(c, 0x848), 0);
                        } else {
                            c.cycles += 3;
                            c.g[V1][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x38));
                            let zero = c.g[V1][0] == 0;
                            c.g[A3][0] = 0;
                            if !zero {
                                c.cycles += 2;
                                c.g[V0][0] = c.lw((c.g[T0][0] as u32).wrapping_add(0x34));
                                c.g[A3][0] =
                                    sx((c.g[V1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                            }
                            c.cycles += 1;
                            c.sw(frame.at(c, 0x848), c.g[A3][0]);
                        }
                        c.cycles += 7;
                        c.g[V0][0] = 0x1;
                        load_game_comm(c, V1);
                        c.g[S6][0] = c.g[V0][0];
                        c.g[S0][0] = c.g[V0][0];
                        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xa0));
                        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x10));
                        c.g[V1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x24));
                    }
                }
                {
                    c.cycles += 20;
                    if c.g[A0][0] == 0 {
                        c.g[S6][0] = 0;
                    }
                    if c.g[V0][0] == 0 {
                        c.g[S0][0] = 0;
                    }
                    c.sw(frame.at(c, 0x84c), c.g[V1][0]);
                    c.g[A0][0] = c.lw(frame.at(c, 0x820));
                    c.g[A3][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xa0));
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A1][0] = REPORT_BUF_SIZE as u64;
                    c.g[A2][0] = sx(tag::WHEN);
                    c.sb(sp(c), 0);
                    callee::TAG_FIELD_SET_DATE.call(c, 0x0026_0030)?;
                    c.g[A3][0] = c.g[S2][0];
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A1][0] = REPORT_BUF_SIZE as u64;
                    c.g[A2][0] = sx(tag::REPT);
                    call_ptr(c, callee::TAG_FIELD_SET_STRING, 0x0026_0048)?;
                    c.g[V0][0] = c.lw(frame.at(c, 0x824));
                    let has_auth = c.g[V0][0] != 0;
                    c.g[V1][0] = c.lw(frame.at(c, 0x820));
                    if has_auth {
                        c.cycles += 6;
                        c.g[A3][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x9c));
                        c.g[A0][0] = c.g[SP][0];
                        c.g[A1][0] = REPORT_BUF_SIZE as u64;
                        c.g[A2][0] = sx(tag::AUTH);
                        call_ptr(c, callee::TAG_FIELD_SET_STRING, 0x0026_006c)?;
                    }
                }
                // One 115-cycle charge covers the next 15 calls.
                c.cycles += 115;
                load_game_comm(c, V0);
                c.g[A3][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x30));
                c.g[V0][0] = c.lw(frame.at(c, 0x830));
                c.g[S1][0] = sx((c.g[S1][0] as u32) << 2);
                c.g[S3][0] = 0x1;
                c.g[S2][0] = sx((c.g[V0][0] as u32) << 2);
                set_number(c, tag::COMMERR, 0x0026_0098)?;
                c.sw(frame.at(c, 0x850), c.g[S2][0]);
                load_game_comm(c, V0);
                c.g[A3][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x18));
                set_number(c, tag::SCOMERR, 0x0026_00b8)?;
                c.sw(frame.at(c, 0x854), c.g[S1][0]);
                load_game_comm(c, V0);
                c.g[A3][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x9c));
                set_number(c, tag::PINGRSLT, 0x0026_00d8)?;
                load_game_comm(c, V1);
                c.g[A3][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xb4));
                set_number(c, tag::STALLCNT, 0x0026_00f4)?;
                load_game_comm(c, V1);
                c.g[A3][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1c));
                set_number(c, tag::CERRSTAT, 0x0026_0110)?;
                load_game_comm(c, V1);
                c.g[A3][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x20));
                set_number(c, tag::CERRFLGS, 0x0026_012c)?;
                c.g[A3][0] = c.g[S7][0];
                set_number(c, tag::DIDQUIT0, 0x0026_0144)?;
                c.g[A3][0] = c.g[S0][0];
                set_number(c, tag::DIDQUIT1, 0x0026_015c)?;
                c.g[A3][0] = c.g[FP][0];
                set_number(c, tag::FINISH0, 0x0026_0174)?;
                c.g[A3][0] = c.g[S6][0];
                set_number(c, tag::FINISH1, 0x0026_018c)?;
                c.g[S0][0] = 0x530000;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x66a8));
                c.g[A3][0] = 0;
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[S0][0] as u32));
                c.g[V0][0] = c.lw(c.g[S2][0] as u32);
                if c.g[V0][0] != 0 {
                    c.g[A3][0] = c.g[S3][0];
                }
                set_number(c, tag::DNF0, 0x0026_01b8)?;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[S0][0] as u32));
                c.g[V0][0] = c.lw(c.g[S1][0] as u32);
                c.g[A3][0] = 0;
                if c.g[V0][0] != 0 {
                    c.g[A3][0] = c.g[S3][0];
                }
                set_number(c, tag::DNF1, 0x0026_01dc)?;
                c.g[A0][0] = c.lw(frame.at(c, 0x82c));
                c.g[A3][0] = 0;
                c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x9c));
                if c.g[V1][0] != 0 {
                    c.g[A3][0] = c.g[S3][0];
                }
                set_number(c, tag::RSLTSLCK, 0x0026_0200)?;
                c.g[A0][0] = c.lw(frame.at(c, 0x828));
                callee::BE_NEW_RACE_INT_GET_TRACK.call(c, 0x0026_0208)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[A3][0] = c.g[S0][0];
                set_number(c, tag::RACETRA0, 0x0026_0224)?;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0xe));
                c.g[V0][0] = 0x530000;
                c.g[S0][0] = (c.g[S0][0] < 0x3) as u64;
                c.g[S1][0] = c.lb((c.g[V0][0] as u32).wrapping_add(0x5c12));
                if c.g[S0][0] != 0 {
                    c.cycles += 3;
                    c.g[V0][0] = 0x4;
                    let not_four = c.g[S1][0] != c.g[V0][0];
                    c.g[S1][0] = 0x1;
                    if !not_four {
                        c.cycles += 1;
                        c.g[S1][0] = 0;
                    }
                }
                // Rider 0 header.
                c.cycles += 48;
                c.g[A3][0] = c.g[S1][0];
                set_number(c, tag::RACEEVE0, 0x0026_0260)?;
                load_game_comm(c, V1);
                c.g[A3][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xc));
                set_number(c, tag::DESYNC, 0x0026_027c)?;
                c.g[V0][0] = c.lw(frame.at(c, 0x820));
                c.g[A3][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x90));
                c.g[A0][0] = c.g[SP][0];
                c.g[A1][0] = REPORT_BUF_SIZE as u64;
                c.g[A2][0] = sx(tag::NAME0);
                call_ptr(c, callee::TAG_FIELD_SET_STRING, 0x0026_0298)?;
                c.g[A3][0] = c.lw(frame.at(c, 0x834));
                set_number(c, tag::QUIT0, 0x0026_02b0)?;
                c.g[A3][0] = c.lw(frame.at(c, 0x83c));
                set_number(c, tag::CHEAT0, 0x0026_02c8)?;
                c.g[A3][0] = c.lw(frame.at(c, 0x838));
                set_number(c, tag::DISC0, 0x0026_02e0)?;
                c.g[A0][0] = 0x530000;
                c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4b30));
                c.g[A3][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x328));
                set_number(c, tag::CHARACTER0, 0x0026_0300)?;
                c.g[V1][0] = 0x530000;
                if c.g[S1][0] != 0 {
                    c.cycles += 11;
                    c.g[A0][0] = c.lw(frame.at(c, 0x850));
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x6640));
                    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[A3][0] = c.lw(c.g[V0][0] as u32);
                    c.g[S2][0] = c.g[V1][0];
                    set_number(c, tag::SCORE0, 0x0026_0360)?;
                } else {
                    c.cycles += 12;
                    c.g[A0][0] = c.lw(frame.at(c, 0x850));
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x6640));
                    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[A3][0] = c.lw(c.g[V0][0] as u32);
                    c.g[S2][0] = c.g[V1][0];
                    set_number(c, tag::RACETIM0, 0x0026_0330)?;
                }
                // Rider 0 stats, then rider 1's character.
                c.cycles += 241;
                report_rider_stats(c, S4, tag::STATS0_BASE, tag::POINTS0, &RIDER0_RETS)?;
                c.g[A3][0] = c.lw(frame.at(c, 0x840));
                set_number(c, tag::CHARACTER1, 0x0026_0720)?;
                c.g[A0][0] = c.lw(frame.at(c, 0x854));
                if c.g[S1][0] == 0 {
                    c.cycles += 11;
                    c.g[V1][0] = c.lw(frame.at(c, 0x854));
                    c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x6640));
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[A3][0] = c.lw(c.g[V0][0] as u32);
                    set_number(c, tag::RACETIM1, 0x0026_074c)?;
                    c.g[V0][0] = c.lw(frame.at(c, 0x820));
                } else {
                    c.cycles += 9;
                    c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x6640));
                    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[A3][0] = c.lw(c.g[V0][0] as u32);
                    set_number(c, tag::SCORE1, 0x0026_0774)?;
                    c.g[V0][0] = c.lw(frame.at(c, 0x820));
                }
                // Rider 1 header and stats, then send and queue the message.
                {
                    c.cycles += 294;
                    c.g[A3][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x94));
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A1][0] = REPORT_BUF_SIZE as u64;
                    c.g[A2][0] = sx(tag::NAME1);
                    c.g[S1][0] = 0x100;
                    call_ptr(c, callee::TAG_FIELD_SET_STRING, 0x0026_0794)?;
                    c.g[A3][0] = c.lw(frame.at(c, 0x844));
                    set_number(c, tag::QUIT1, 0x0026_07ac)?;
                    c.g[A3][0] = c.lw(frame.at(c, 0x84c));
                    set_number(c, tag::CHEAT1, 0x0026_07c4)?;
                    c.g[A3][0] = c.lw(frame.at(c, 0x848));
                    set_number(c, tag::DISC1, 0x0026_07dc)?;
                    report_rider_stats(c, S5, tag::STATS1_BASE, tag::POINTS1, &RIDER1_RETS)?;
                    c.g[V1][0] = c.lw(frame.at(c, 0x820));
                    c.g[T0][0] = 0x270000;
                    c.g[AT][0] = 0x41a00000;
                    c.mtc1(1, 12);
                    c.g[A2][0] = REQ_RANK as u64;
                    c.g[A1][0] = c.lw(c.g[V1][0] as u32);
                    c.g[A3][0] = c.g[SP][0];
                    c.g[A0][0] = c.g[V1][0];
                    c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x7110));
                    callee::LOBBY_MAN_SEND_REQUEST.call(c, 0x0026_0bb0)?;
                    c.g[A1][0] = sx(NET_EVENT_TAG);
                    c.g[A0][0] = 0x8;
                    c.g[A2][0] = 0x20000000;
                    c.g[A3][0] = 0;
                    callee::EA_MEM_ALLOC_NAMED.call(c, 0x0026_0bc8)?;
                    c.g[S0][0] = c.g[V0][0];
                    c.g[A0][0] = c.lw(frame.at(c, 0x820));
                    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xf0));
                    c.g[V0][0] = sx(NET_EVENT_VTABLE);
                    c.sw(c.g[S0][0] as u32, c.g[S1][0]);
                    c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                    c.sw(frame.at(c, 0x810), c.g[V1][0]);
                    c.g[A1][0] = 0x480000;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4a8));
                    c.g[A0][0] = 0xc;
                    c.g[A2][0] = 0x20000000;
                    c.g[A3][0] = 0;
                    callee::EA_MEM_ALLOC_NAMED.call(c, 0x0026_0c00)?;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x8));
                    if c.g[V0][0] != 0 {
                        c.sw((c.g[A1][0] as u32).wrapping_add(0x8), c.g[S0][0]);
                    }
                }
                c.cycles += 9;
                c.g[V0][0] = c.lw(frame.at(c, 0x810));
                c.sw(c.g[A1][0] as u32, c.g[V0][0]);
                c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
                c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[V1][0]);
                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
                c.sw(c.g[A0][0] as u32, c.g[A1][0]);
                c.g[V0][0] = c.lw(frame.at(c, 0x810));
                c.sw((c.g[V0][0] as u32).wrapping_add(0x4), c.g[A1][0]);
                c.sw(frame.at(c, 0x800), c.g[A1][0]);
            }
        }
        c.cycles += 12;
        let frame = Frame { size: 0x900 };
        frame.restore(c, S0, 0x8f0);
        frame.restore(c, S1, 0x8e0);
        frame.restore(c, S2, 0x8d0);
        frame.restore(c, S3, 0x8c0);
        frame.restore(c, S4, 0x8b0);
        frame.restore(c, S5, 0x8a0);
        frame.restore(c, S6, 0x890);
        frame.restore(c, S7, 0x880);
        frame.restore(c, FP, 0x870);
        frame.restore_ra(c, 0x860);
        frame.close(c);
        Ok(())
    });
}

pub(crate) const RIDER1_RETS: RiderRets = RiderRets {
    ints: [
        0x0026_07f4,
        0x0026_080c,
        0x0026_0824,
        0x0026_083c,
        0x0026_0854,
        0x0026_086c,
        0x0026_0884,
        0x0026_089c,
        0x0026_08b4,
        0x0026_08cc,
        0x0026_08e4,
        0x0026_08fc,
        0x0026_0914,
        0x0026_092c,
    ],
    float_fix: [
        0x0026_093c,
        0x0026_0960,
        0x0026_0984,
        0x0026_09a8,
        0x0026_09cc,
        0x0026_09f0,
        0x0026_0a14,
        0x0026_0a38,
        0x0026_0a5c,
        0x0026_0a80,
    ],
    float_num: [
        0x0026_0950,
        0x0026_0974,
        0x0026_0998,
        0x0026_09bc,
        0x0026_09e0,
        0x0026_0a04,
        0x0026_0a28,
        0x0026_0a4c,
        0x0026_0a70,
        0x0026_0a94,
    ],
    tail: [
        0x0026_0aac,
        0x0026_0ac4,
        0x0026_0adc,
        0x0026_0af4,
        0x0026_0b0c,
        0x0026_0b24,
        0x0026_0b3c,
        0x0026_0b54,
        0x0026_0b6c,
        0x0026_0b84,
    ],
};

/// `void LobbyMan_StaticInit(int, int)` (0x267468): when the second
/// argument is 0xffff and the first is non-zero, placement-constructs the
/// lobby's static tables: 56 colours at 0x4d2c38..0x4d2fb8 (from the float
/// constants at gp-0x4b70..) and the handle/pair entries at gp+0x1fe8..
/// (0x4a50d8..0x4a5158, bytes and words); otherwise it returns without
/// touching them. No calls, no loops.
pub fn lobby_man_static_init(c: &mut Cpu) {
    let frame = Frame::open(c, 0x90);
    c.cycles += 12;
    c.g[V0][0] = 0xffff;
    frame.save(c, S0, 0x80);
    frame.save(c, S1, 0x70);
    frame.save(c, S2, 0x60);
    frame.save(c, S3, 0x50);
    frame.save(c, S4, 0x40);
    frame.save(c, S5, 0x30);
    frame.save(c, S6, 0x20);
    frame.save(c, S7, 0x10);
    let t = c.g[A1][0] != c.g[V0][0];
    frame.save(c, 30, 0);
    if !t {
        c.cycles += 2;
        let t = c.g[A0][0] == 0u64;
        c.g[T5][0] = 0x4d0000;
        if !t {
            c.cycles += 558;
            c.lwc1(9, (c.g[GP][0] as u32).wrapping_sub(0x4b70));
            c.lwc1(10, (c.g[GP][0] as u32).wrapping_sub(0x4b6c));
            c.g[T6][0] = 0x4d0000;
            c.lwc1(11, (c.g[GP][0] as u32).wrapping_sub(0x4b68));
            c.g[T7][0] = 0x4d0000;
            c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x4b64));
            c.g[V0][0] = sx((c.g[T7][0] as u32).wrapping_add(0x2c68));
            c.swc1(9, (c.g[T5][0] as u32).wrapping_add(0x2c48));
            c.g[S1][0] = 0x4d0000;
            c.swc1(10, (c.g[T6][0] as u32).wrapping_add(0x2c58));
            c.g[S2][0] = 0x4d0000;
            c.swc1(11, (c.g[T7][0] as u32).wrapping_add(0x2c68));
            c.g[S3][0] = 0x4d0000;
            c.lwc1(8, (c.g[GP][0] as u32).wrapping_sub(0x4b60));
            c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x2c98));
            c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x4b5c));
            c.g[S5][0] = 0x4d0000;
            c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x4b58));
            c.g[S4][0] = 0x4d0000;
            c.swc1(7, (c.g[V0][0] as u32).wrapping_add(0xc));
            c.g[T3][0] = 0x4d0000;
            c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x4b54));
            c.g[A1][0] = sx((c.g[S5][0] as u32).wrapping_add(0x2c38));
            c.g[AT][0] = 0x3f800000;
            c.mtc1(1, 0);
            c.g[A3][0] = sx((c.g[T5][0] as u32).wrapping_add(0x2c48));
            c.swc1(8, (c.g[S1][0] as u32).wrapping_add(0x2c78));
            c.g[T1][0] = sx((c.g[T6][0] as u32).wrapping_add(0x2c58));
            c.swc1(4, (c.g[S2][0] as u32).wrapping_add(0x2c88));
            c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x2c78));
            c.swc1(5, (c.g[S3][0] as u32).wrapping_add(0x2c98));
            c.g[T0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x2c88));
            c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x4b50));
            c.g[A2][0] = sx((c.g[S4][0] as u32).wrapping_add(0x2ca8));
            c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x4b4c));
            c.g[T4][0] = sx((c.g[T3][0] as u32).wrapping_add(0x2cb8));
            c.swc1(2, (c.g[A0][0] as u32).wrapping_add(0xc));
            c.g[T9][0] = 0x4d0000;
            c.g[AT][0] = 0x3f000000;
            c.mtc1(1, 6);
            c.g[T2][0] = sx((c.g[T9][0] as u32).wrapping_add(0x2cc8));
            c.swc1(3, (c.g[S4][0] as u32).wrapping_add(0x2ca8));
            c.g[S6][0] = 0x4d0000;
            c.swc1(1, (c.g[T3][0] as u32).wrapping_add(0x2cb8));
            c.g[T7][0] = sx((c.g[S6][0] as u32).wrapping_add(0x2cf8));
            c.swc1(0, (c.g[S5][0] as u32).wrapping_add(0x2c38));
            c.g[S2][0] = 0x4d0000;
            c.swc1(6, (c.g[A1][0] as u32).wrapping_add(0xc));
            c.g[S1][0] = sx((c.g[S2][0] as u32).wrapping_add(0x2d18));
            c.sw((c.g[A3][0] as u32).wrapping_add(0xc), 0u64);
            c.g[S3][0] = 0x4d0000;
            c.lwc1(17, (c.g[GP][0] as u32).wrapping_sub(0x4b48));
            c.g[S2][0] = sx((c.g[S3][0] as u32).wrapping_add(0x2d28));
            c.lwc1(18, (c.g[GP][0] as u32).wrapping_sub(0x4b44));
            c.g[T8][0] = 0x4d0000;
            c.sw((c.g[T1][0] as u32).wrapping_add(0xc), 0u64);
            c.g[FP][0] = 0x4d0000;
            c.lwc1(13, (c.g[GP][0] as u32).wrapping_sub(0x4b40));
            c.g[S3][0] = sx((c.g[FP][0] as u32).wrapping_add(0x2d38));
            c.lwc1(14, (c.g[GP][0] as u32).wrapping_sub(0x4b3c));
            c.g[T5][0] = sx((c.g[T8][0] as u32).wrapping_add(0x2cd8));
            c.swc1(17, (c.g[V0][0] as u32).wrapping_add(0x4));
            c.g[S7][0] = 0x4d0000;
            c.swc1(18, (c.g[V0][0] as u32).wrapping_add(0x8));
            c.g[FP][0] = 0x4d0000;
            c.swc1(13, (c.g[V1][0] as u32).wrapping_add(0x4));
            c.g[V0][0] = 0x4d0000;
            c.swc1(14, (c.g[V1][0] as u32).wrapping_add(0x8));
            c.g[S5][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2d48));
            c.sw((c.g[V1][0] as u32).wrapping_add(0xc), 0u64);
            c.g[T6][0] = sx((c.g[S7][0] as u32).wrapping_add(0x2ce8));
            c.lwc1(15, (c.g[GP][0] as u32).wrapping_sub(0x4b38));
            c.g[V1][0] = 0x4d0000;
            c.lwc1(16, (c.g[GP][0] as u32).wrapping_sub(0x4b34));
            c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x2d88));
            c.sw((c.g[T0][0] as u32).wrapping_add(0xc), 0u64);
            c.g[V1][0] = sx((c.g[FP][0] as u32).wrapping_add(0x2da8));
            c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(0x4b30));
            c.g[S0][0] = 0x4d0000;
            c.swc1(15, (c.g[A0][0] as u32).wrapping_add(0x4));
            c.swc1(16, (c.g[A0][0] as u32).wrapping_add(0x8));
            c.swc1(12, (c.g[A2][0] as u32).wrapping_add(0x4));
            c.g[A0][0] = 0x4d0000;
            c.swc1(0, (c.g[A2][0] as u32).wrapping_add(0xc));
            c.g[S4][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2d98));
            c.swc1(0, (c.g[A1][0] as u32).wrapping_add(0x4));
            c.g[A0][0] = 0x4d0000;
            c.swc1(0, (c.g[A1][0] as u32).wrapping_add(0x8));
            c.g[T3][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2db8));
            c.swc1(0, (c.g[A3][0] as u32).wrapping_add(0x4));
            c.g[A0][0] = 0x4d0000;
            c.swc1(0, (c.g[A3][0] as u32).wrapping_add(0x8));
            c.g[FP][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2dc8));
            c.swc1(0, (c.g[T1][0] as u32).wrapping_add(0x4));
            c.g[A1][0] = 0x4d0000;
            c.swc1(0, (c.g[T1][0] as u32).wrapping_add(0x8));
            c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2dd8));
            c.swc1(0, (c.g[T0][0] as u32).wrapping_add(0x4));
            c.g[A3][0] = 0x4d0000;
            c.swc1(0, (c.g[T0][0] as u32).wrapping_add(0x8));
            c.lwc1(19, (c.g[GP][0] as u32).wrapping_sub(0x4b2c));
            c.g[T0][0] = sx((c.g[A3][0] as u32).wrapping_add(0x2e08));
            c.swc1(0, (c.g[A2][0] as u32).wrapping_add(0x8));
            c.g[A3][0] = 0x4d0000;
            c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x4b28));
            c.g[A2][0] = 0x4d0000;
            c.swc1(19, (c.g[T4][0] as u32).wrapping_add(0x4));
            c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_add(0x2de8));
            c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x4b24));
            c.g[A2][0] = 0x4d0000;
            c.swc1(2, (c.g[T9][0] as u32).wrapping_add(0x2cc8));
            c.g[T1][0] = sx((c.g[A2][0] as u32).wrapping_add(0x2e18));
            c.lwc1(8, (c.g[GP][0] as u32).wrapping_sub(0x4b20));
            c.g[A2][0] = sx((c.g[A3][0] as u32).wrapping_add(0x2e38));
            c.swc1(1, (c.g[T2][0] as u32).wrapping_add(0xc));
            c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x4b1c));
            c.swc1(8, (c.g[T7][0] as u32).wrapping_add(0xc));
            c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x4b18));
            c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x4b14));
            c.swc1(3, (c.g[T4][0] as u32).wrapping_add(0x8));
            c.sw((c.g[T4][0] as u32).wrapping_add(0xc), 0u64);
            c.swc1(4, (c.g[T2][0] as u32).wrapping_add(0x4));
            c.swc1(5, (c.g[T2][0] as u32).wrapping_add(0x8));
            c.swc1(0, (c.g[T8][0] as u32).wrapping_add(0x2cd8));
            c.g[T2][0] = 0x4d0000;
            c.swc1(0, (c.g[T5][0] as u32).wrapping_add(0xc));
            c.g[A3][0] = sx((c.g[T2][0] as u32).wrapping_add(0x2e98));
            c.swc1(0, (c.g[S7][0] as u32).wrapping_add(0x2ce8));
            c.g[T2][0] = 0x4d0000;
            c.sw((c.g[T6][0] as u32).wrapping_add(0xc), 0u64);
            c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x4b10));
            c.swc1(0, (c.g[S6][0] as u32).wrapping_add(0x2cf8));
            c.swc1(7, (c.g[T7][0] as u32).wrapping_add(0x8));
            c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x2d08));
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x2d08));
            c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x4b0c));
            c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0xc));
            c.g[S0][0] = 0x4d0000;
            c.swc1(0, (c.g[T2][0] as u32).wrapping_add(0x2d18));
            c.g[T4][0] = sx((c.g[S0][0] as u32).wrapping_add(0x2eb8));
            c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0xc));
            c.g[S0][0] = 0x4d0000;
            c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x4b08));
            c.g[T2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x2ec8));
            c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x4b04));
            c.g[S0][0] = 0x4d0000;
            c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x4b00));
            c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x2d28));
            c.sw((c.g[S2][0] as u32).wrapping_add(0xc), 0u64);
            c.g[S0][0] = 0x4d0000;
            c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x2d38));
            c.swc1(0, (c.g[T5][0] as u32).wrapping_add(0x4));
            c.g[S0][0] = 0x4d0000;
            c.swc1(0, (c.g[T5][0] as u32).wrapping_add(0x8));
            c.sw((c.g[T6][0] as u32).wrapping_add(0x4), 0u64);
            c.g[T5][0] = 0x4d0000;
            c.sw((c.g[T6][0] as u32).wrapping_add(0x8), 0u64);
            c.g[T5][0] = sx((c.g[T5][0] as u32).wrapping_add(0x2d08));
            c.swc1(8, (c.g[T7][0] as u32).wrapping_add(0x4));
            c.g[T6][0] = 0x4d0000;
            c.swc1(0, (c.g[T5][0] as u32).wrapping_add(0x4));
            c.g[T7][0] = sx((c.g[S0][0] as u32).wrapping_add(0x2f48));
            c.swc1(0, (c.g[T5][0] as u32).wrapping_add(0x8));
            c.g[S0][0] = 0x4d0000;
            c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x4));
            c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x8));
            c.sw((c.g[S2][0] as u32).wrapping_add(0x4), 0u64);
            c.g[S1][0] = sx((c.g[T6][0] as u32).wrapping_add(0x2f28));
            c.sw((c.g[S2][0] as u32).wrapping_add(0x8), 0u64);
            c.g[T6][0] = 0xffffffffffff0000;
            c.sw((c.g[S3][0] as u32).wrapping_add(0x4), 0u64);
            c.g[S2][0] = 0x4d0000;
            c.sw((c.g[S3][0] as u32).wrapping_add(0x8), 0u64);
            c.g[T5][0] = sx((c.g[S2][0] as u32).wrapping_add(0x2f58));
            c.sw((c.g[S3][0] as u32).wrapping_add(0xc), 0u64);
            c.g[S2][0] = 0x4d0000;
            c.g[S3][0] = 0x4d0000;
            c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x4afc));
            c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_add(0x2d78));
            c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x4af8));
            c.swc1(6, (c.g[S3][0] as u32).wrapping_add(0xc));
            c.g[T6][0] = c.g[T6][0] | 0xff00;
            c.swc1(2, (c.g[V0][0] as u32).wrapping_add(0xc));
            c.g[S3][0] = 0x4d0000;
            c.swc1(1, (c.g[S0][0] as u32).wrapping_add(0x2da8));
            c.swc1(0, (c.g[S2][0] as u32).wrapping_add(0x2d48));
            c.g[S0][0] = 0x4d0000;
            c.swc1(0, (c.g[S5][0] as u32).wrapping_add(0xc));
            c.g[S2][0] = sx((c.g[S3][0] as u32).wrapping_add(0x2f68));
            c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x2d58));
            c.g[S0][0] = 0x4d0000;
            c.lwc1(8, (c.g[GP][0] as u32).wrapping_sub(0x4af4));
            c.g[S3][0] = sx((c.g[S0][0] as u32).wrapping_add(0x2f78));
            c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x4af0));
            c.g[S0][0] = 0x4d0000;
            c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x4aec));
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x2d58));
            c.lwc1(9, (c.g[GP][0] as u32).wrapping_sub(0x4ae8));
            c.swc1(6, (c.g[S0][0] as u32).wrapping_add(0xc));
            c.g[S0][0] = 0x4d0000;
            c.lwc1(10, (c.g[GP][0] as u32).wrapping_sub(0x4ae4));
            c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x2d68));
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x2d68));
            c.lwc1(11, (c.g[GP][0] as u32).wrapping_sub(0x4ae0));
            c.swc1(6, (c.g[S0][0] as u32).wrapping_add(0xc));
            c.g[S0][0] = 0x4d0000;
            c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(0x4adc));
            c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x2d78));
            c.g[S0][0] = 0x4d0000;
            c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x2d88));
            c.swc1(3, (c.g[V0][0] as u32).wrapping_add(0x4));
            c.g[S0][0] = 0x4d0000;
            c.swc1(4, (c.g[V0][0] as u32).wrapping_add(0x8));
            c.g[V0][0] = 0x4d0000;
            c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x4ad8));
            c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x2d98));
            c.swc1(5, (c.g[S4][0] as u32).wrapping_add(0x8));
            c.g[V0][0] = 0x4d0000;
            c.sw((c.g[S4][0] as u32).wrapping_add(0xc), 0u64);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2d58));
            c.swc1(7, (c.g[V1][0] as u32).wrapping_add(0x4));
            c.swc1(8, (c.g[V1][0] as u32).wrapping_add(0x8));
            c.sw((c.g[V1][0] as u32).wrapping_add(0xc), 0u64);
            c.g[V1][0] = 0x4d0000;
            c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x4ad4));
            c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x2db8));
            c.swc1(0, (c.g[T3][0] as u32).wrapping_add(0xc));
            c.g[V1][0] = 0x4d0000;
            c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x2dc8));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x2d68));
            c.swc1(0, (c.g[S5][0] as u32).wrapping_add(0x4));
            c.g[S0][0] = 0x4d0000;
            c.swc1(0, (c.g[S5][0] as u32).wrapping_add(0x8));
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x2d78));
            c.swc1(6, (c.g[V0][0] as u32).wrapping_add(0x4));
            c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x8));
            c.swc1(6, (c.g[V1][0] as u32).wrapping_add(0x4));
            c.g[V0][0] = 0x4d0000;
            c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x8));
            c.swc1(6, (c.g[S0][0] as u32).wrapping_add(0x4));
            c.g[V1][0] = 0x4d0000;
            c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x8));
            c.swc1(0, (c.g[S4][0] as u32).wrapping_add(0x4));
            c.g[S0][0] = 0x4d0000;
            c.swc1(0, (c.g[T3][0] as u32).wrapping_add(0x4));
            c.swc1(0, (c.g[T3][0] as u32).wrapping_add(0x8));
            c.swc1(0, (c.g[FP][0] as u32).wrapping_add(0x4));
            c.swc1(2, (c.g[A1][0] as u32).wrapping_add(0xc));
            c.swc1(1, (c.g[T0][0] as u32).wrapping_add(0xc));
            c.swc1(3, (c.g[FP][0] as u32).wrapping_add(0x8));
            c.sw((c.g[FP][0] as u32).wrapping_add(0xc), 0u64);
            c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x4ad0));
            c.g[FP][0] = 0x4d0000;
            c.swc1(0, (c.g[FP][0] as u32).wrapping_add(0x2dd8));
            c.swc1(5, (c.g[A0][0] as u32).wrapping_add(0x8));
            c.g[FP][0] = 0x4d0000;
            c.swc1(4, (c.g[A0][0] as u32).wrapping_add(0x4));
            c.sw((c.g[A0][0] as u32).wrapping_add(0xc), 0u64);
            c.lwc1(6, (c.g[GP][0] as u32).wrapping_sub(0x4acc));
            c.g[A0][0] = c.g[V1][0];
            c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x4ac8));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2df8));
            c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x2de8));
            c.swc1(6, (c.g[A1][0] as u32).wrapping_add(0x4));
            c.g[V0][0] = c.g[FP][0];
            c.swc1(7, (c.g[A1][0] as u32).wrapping_add(0x8));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2e28));
            c.lwc1(8, (c.g[GP][0] as u32).wrapping_sub(0x4ac4));
            c.g[A1][0] = 0x4d0000;
            c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x2df8));
            c.swc1(8, (c.g[A0][0] as u32).wrapping_add(0x8));
            c.g[V1][0] = 0x4d0000;
            c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0xc));
            c.swc1(0, (c.g[A1][0] as u32).wrapping_add(0x2e08));
            c.g[A0][0] = 0x4d0000;
            c.swc1(9, (c.g[T0][0] as u32).wrapping_add(0x4));
            c.g[A1][0] = 0x4d0000;
            c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x2e18));
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2df8));
            c.swc1(0, (c.g[T1][0] as u32).wrapping_add(0xc));
            c.g[S0][0] = 0x4d0000;
            c.swc1(0, (c.g[FP][0] as u32).wrapping_add(0x2e28));
            c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0xc));
            c.g[FP][0] = 0x4d0000;
            c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x2e38));
            c.g[FP][0] = sx((c.g[FP][0] as u32).wrapping_add(0x2ea8));
            c.swc1(10, (c.g[A2][0] as u32).wrapping_add(0x4));
            c.swc1(11, (c.g[A2][0] as u32).wrapping_add(0x8));
            c.sw((c.g[A2][0] as u32).wrapping_add(0xc), 0u64);
            c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x2e48));
            c.g[A2][0] = c.g[A0][0];
            c.swc1(12, (c.g[A1][0] as u32).wrapping_add(0x4));
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x2e48));
            c.swc1(0, (c.g[T0][0] as u32).wrapping_add(0x8));
            c.g[A0][0] = 0x4d0000;
            c.swc1(0, (c.g[T1][0] as u32).wrapping_add(0x4));
            c.g[A1][0] = c.g[A0][0];
            c.swc1(0, (c.g[T1][0] as u32).wrapping_add(0x8));
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2e68));
            c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x4));
            c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x8));
            c.swc1(0, (c.g[A2][0] as u32).wrapping_add(0x4));
            c.g[V0][0] = 0x4d0000;
            c.swc1(0, (c.g[A2][0] as u32).wrapping_add(0x8));
            c.g[V1][0] = c.g[V0][0];
            c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x4ac0));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x2e58));
            c.swc1(0, (c.g[A2][0] as u32).wrapping_add(0xc));
            c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x4abc));
            c.g[A2][0] = 0x4d0000;
            c.swc1(4, (c.g[S0][0] as u32).wrapping_add(0x2e88));
            c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x4ab8));
            c.g[S0][0] = c.g[A2][0];
            c.swc1(3, (c.g[A3][0] as u32).wrapping_add(0xc));
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x2e78));
            c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x4ab4));
            c.swc1(2, (c.g[FP][0] as u32).wrapping_add(0xc));
            c.swc1(1, (c.g[T2][0] as u32).wrapping_add(0xc));
            c.g[FP][0] = 0x4d0000;
            c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x4ab0));
            c.g[FP][0] = sx((c.g[FP][0] as u32).wrapping_add(0x2e88));
            c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x2e58));
            c.swc1(5, (c.g[V1][0] as u32).wrapping_add(0x4));
            c.g[V0][0] = 0x4d0000;
            c.sw((c.g[V1][0] as u32).wrapping_add(0xc), 0u64);
            c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x2e68));
            c.g[V1][0] = 0x4d0000;
            c.swc1(0, (c.g[A1][0] as u32).wrapping_add(0xc));
            c.g[A0][0] = c.g[V1][0];
            c.swc1(0, (c.g[A2][0] as u32).wrapping_add(0x2e78));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2ea8));
            c.sw((c.g[S0][0] as u32).wrapping_add(0xc), 0u64);
            c.g[A1][0] = 0x4d0000;
            c.sw((c.g[FP][0] as u32).wrapping_add(0xc), 0u64);
            c.g[A2][0] = 0x4d0000;
            c.lwc1(6, (c.g[GP][0] as u32).wrapping_sub(0x4aac));
            c.g[S0][0] = 0x4d0000;
            c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x4aa8));
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x2e58));
            c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x2e98));
            c.g[FP][0] = 0x4d0000;
            c.swc1(6, (c.g[A3][0] as u32).wrapping_add(0x4));
            c.g[FP][0] = sx((c.g[FP][0] as u32).wrapping_add(0x2e68));
            c.swc1(7, (c.g[A3][0] as u32).wrapping_add(0x8));
            c.g[V0][0] = 0x4d0000;
            c.lwc1(8, (c.g[GP][0] as u32).wrapping_sub(0x4aa4));
            c.g[A3][0] = 0x4d0000;
            c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x2ea8));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2e78));
            c.swc1(8, (c.g[A0][0] as u32).wrapping_add(0x4));
            c.g[V1][0] = 0x4d0000;
            c.lwc1(9, (c.g[GP][0] as u32).wrapping_sub(0x4aa0));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x2e88));
            c.swc1(0, (c.g[A1][0] as u32).wrapping_add(0x2eb8));
            c.swc1(9, (c.g[T4][0] as u32).wrapping_add(0x8));
            c.g[A1][0] = 0x4d0000;
            c.sw((c.g[T4][0] as u32).wrapping_add(0xc), 0u64);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2ee8));
            c.lwc1(10, (c.g[GP][0] as u32).wrapping_sub(0x4a9c));
            c.swc1(0, (c.g[A2][0] as u32).wrapping_add(0x2ec8));
            c.swc1(10, (c.g[T2][0] as u32).wrapping_add(0x4));
            c.g[A2][0] = 0x4d0000;
            c.swc1(0, (c.g[A3][0] as u32).wrapping_add(0x2ed8));
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x2f38));
            c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x8));
            c.swc1(0, (c.g[FP][0] as u32).wrapping_add(0x4));
            c.swc1(0, (c.g[FP][0] as u32).wrapping_add(0x8));
            c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x4));
            c.g[FP][0] = 0x4d0000;
            c.sw((c.g[V0][0] as u32).wrapping_add(0x8), 0u64);
            c.sw((c.g[V1][0] as u32).wrapping_add(0x4), 0u64);
            c.g[V0][0] = c.g[FP][0];
            c.sw((c.g[V1][0] as u32).wrapping_add(0x8), 0u64);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2f08));
            c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x8));
            c.g[V1][0] = 0x4d0000;
            c.swc1(0, (c.g[T4][0] as u32).wrapping_add(0x4));
            c.g[A0][0] = c.g[A3][0];
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2ed8));
            c.swc1(0, (c.g[T2][0] as u32).wrapping_add(0x8));
            c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x4));
            c.g[T2][0] = 0x4d0000;
            c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x4a98));
            c.g[A3][0] = 0x4d0000;
            c.swc1(12, (c.g[A1][0] as u32).wrapping_add(0xc));
            c.g[S0][0] = c.g[T2][0];
            c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x4a94));
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x2ef8));
            c.swc1(2, (c.g[S1][0] as u32).wrapping_add(0xc));
            c.g[A1][0] = 0x4d0000;
            c.swc1(1, (c.g[A2][0] as u32).wrapping_add(0xc));
            c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0xc));
            c.g[A2][0] = 0x4d0000;
            c.swc1(0, (c.g[A3][0] as u32).wrapping_add(0x2ee8));
            c.g[A0][0] = c.g[V1][0];
            c.swc1(0, (c.g[T2][0] as u32).wrapping_add(0x2ef8));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2f18));
            c.sw((c.g[S0][0] as u32).wrapping_add(0xc), 0u64);
            c.g[A3][0] = c.g[A2][0];
            c.swc1(0, (c.g[FP][0] as u32).wrapping_add(0x2f08));
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x2f38));
            c.sw((c.g[V0][0] as u32).wrapping_add(0xc), 0u64);
            c.g[T2][0] = 0x4d0000;
            c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x2f18));
            c.g[S0][0] = 0x4d0000;
            c.sw((c.g[A0][0] as u32).wrapping_add(0xc), 0u64);
            c.g[FP][0] = 0x4d0000;
            c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x4a90));
            c.g[FP][0] = sx((c.g[FP][0] as u32).wrapping_add(0x2ed8));
            c.swc1(0, (c.g[A1][0] as u32).wrapping_add(0x2f28));
            c.g[V0][0] = 0x4d0000;
            c.swc1(3, (c.g[S1][0] as u32).wrapping_add(0x4));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2ee8));
            c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x4a8c));
            c.g[V1][0] = 0x4d0000;
            c.swc1(0, (c.g[A2][0] as u32).wrapping_add(0x2f38));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x2ef8));
            c.swc1(4, (c.g[A3][0] as u32).wrapping_add(0x8));
            c.g[A0][0] = 0x4d0000;
            c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x4a88));
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2f08));
            c.swc1(0, (c.g[T2][0] as u32).wrapping_add(0x2f48));
            c.g[A1][0] = 0x4d0000;
            c.swc1(5, (c.g[T7][0] as u32).wrapping_add(0x8));
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2f18));
            c.sw((c.g[T7][0] as u32).wrapping_add(0xc), 0u64);
            c.g[A2][0] = 0xffffffffffffffff;
            c.lwc1(6, (c.g[GP][0] as u32).wrapping_sub(0x4a84));
            c.g[T2][0] = 0x2;
            c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x4a80));
            c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x2f58));
            c.swc1(6, (c.g[T5][0] as u32).wrapping_add(0x4));
            c.g[S0][0] = 0x4d0000;
            c.swc1(7, (c.g[T5][0] as u32).wrapping_add(0x8));
            c.swc1(0, (c.g[FP][0] as u32).wrapping_add(0x8));
            c.swc1(12, (c.g[V0][0] as u32).wrapping_add(0x4));
            c.swc1(0, (c.g[V0][0] as u32).wrapping_add(0x8));
            c.swc1(0, (c.g[V1][0] as u32).wrapping_add(0x4));
            c.sw((c.g[V1][0] as u32).wrapping_add(0x8), 0u64);
            c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x4));
            c.sw((c.g[A0][0] as u32).wrapping_add(0x8), 0u64);
            c.swc1(0, (c.g[A1][0] as u32).wrapping_add(0x4));
            c.sw((c.g[A1][0] as u32).wrapping_add(0x8), 0u64);
            c.swc1(0, (c.g[S1][0] as u32).wrapping_add(0x8));
            c.swc1(0, (c.g[A3][0] as u32).wrapping_add(0x4));
            c.swc1(0, (c.g[T7][0] as u32).wrapping_add(0x4));
            c.g[A3][0] = 0x1;
            c.sw((c.g[T5][0] as u32).wrapping_add(0xc), 0u64);
            c.sb((c.g[GP][0] as u32).wrapping_add(0x1fe8), c.g[A2][0]);
            c.g[T5][0] = 0x4d0000;
            c.sb((c.g[GP][0] as u32).wrapping_add(0x1ff8), 0u64);
            c.g[T5][0] = sx((c.g[T5][0] as u32).wrapping_add(0x2f88));
            c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x4a7c));
            c.g[V0][0] = c.g[A2][0] & 0xff;
            c.sb((c.g[GP][0] as u32).wrapping_add(0x2000), c.g[A3][0]);
            c.g[V0][0] = c.g[V0][0] | c.g[T6][0];
            c.sb((c.g[GP][0] as u32).wrapping_add(0x2008), c.g[T2][0]);
            c.g[T6][0] = 0x4d0000;
            c.g[A1][0] = c.lbu((c.g[GP][0] as u32).wrapping_add(0x1ff8));
            c.g[V1][0] = c.g[A3][0] & 0xff;
            c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x4a78));
            c.g[A2][0] = 0x4d0000;
            c.swc1(2, (c.g[S3][0] as u32).wrapping_add(0xc));
            c.g[A3][0] = c.g[A2][0];
            c.swc1(1, (c.g[T5][0] as u32).wrapping_add(0xc));
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x2fa8));
            c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x4a74));
            c.g[V1][0] = c.g[V1][0] | 0x100;
            c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x4a70));
            c.g[A0][0] = c.g[T2][0] & 0xff;
            c.swc1(0, (c.g[T6][0] as u32).wrapping_add(0x2f68));
            c.g[A0][0] = c.g[A0][0] | 0x200;
            c.swc1(3, (c.g[S2][0] as u32).wrapping_add(0x4));
            c.g[T2][0] = 0x3;
            c.swc1(4, (c.g[S2][0] as u32).wrapping_add(0x8));
            c.g[T6][0] = 0x5;
            c.sw((c.g[S2][0] as u32).wrapping_add(0xc), 0u64);
            c.lwc1(5, (c.g[GP][0] as u32).wrapping_sub(0x4a6c));
            c.g[S2][0] = 0x4d0000;
            c.lwc1(6, (c.g[GP][0] as u32).wrapping_sub(0x4a68));
            c.swc1(0, (c.g[S0][0] as u32).wrapping_add(0x2f78));
            c.swc1(5, (c.g[S3][0] as u32).wrapping_add(0x4));
            c.g[S0][0] = 0x6;
            c.swc1(6, (c.g[S3][0] as u32).wrapping_add(0x8));
            c.lwc1(7, (c.g[GP][0] as u32).wrapping_sub(0x4a64));
            c.g[S3][0] = 0x4d0000;
            c.lwc1(8, (c.g[GP][0] as u32).wrapping_sub(0x4a60));
            c.g[FP][0] = c.g[S3][0];
            c.swc1(0, (c.g[S2][0] as u32).wrapping_add(0x2f88));
            c.g[FP][0] = sx((c.g[FP][0] as u32).wrapping_add(0x2f98));
            c.swc1(7, (c.g[T5][0] as u32).wrapping_add(0x4));
            c.g[S2][0] = 0x7;
            c.swc1(8, (c.g[T5][0] as u32).wrapping_add(0x8));
            c.swc1(0, (c.g[S3][0] as u32).wrapping_add(0x2f98));
            c.g[T5][0] = 0x4;
            c.sw((c.g[FP][0] as u32).wrapping_add(0xc), 0u64);
            c.g[S3][0] = 0x8;
            c.swc1(0, (c.g[A2][0] as u32).wrapping_add(0x2fa8));
            c.swc1(0, (c.g[A3][0] as u32).wrapping_add(0x4));
            c.sw((c.g[A3][0] as u32).wrapping_add(0xc), 0u64);
            c.sw((c.g[GP][0] as u32).wrapping_add(0x1fe8), c.g[V0][0]);
            c.sw((c.g[GP][0] as u32).wrapping_add(0x1ff8), c.g[A1][0]);
            c.g[V0][0] = 0xa;
            c.sw((c.g[GP][0] as u32).wrapping_add(0x2000), c.g[V1][0]);
            c.g[A1][0] = 0xd;
            c.sw((c.g[FP][0] as u32).wrapping_add(0x4), 0u64);
            c.g[V1][0] = 0xb;
            c.swc1(0, (c.g[FP][0] as u32).wrapping_add(0x8));
            c.sw((c.g[A3][0] as u32).wrapping_add(0x8), 0u64);
            c.g[FP][0] = 0x9;
            c.sw((c.g[GP][0] as u32).wrapping_add(0x1ff4), 0u64);
            c.sw((c.g[GP][0] as u32).wrapping_add(0x1ff0), 0u64);
            c.sw((c.g[GP][0] as u32).wrapping_add(0x2008), c.g[A0][0]);
            c.sb((c.g[GP][0] as u32).wrapping_add(0x2010), c.g[T2][0]);
            c.g[A0][0] = 0xc;
            c.sb((c.g[GP][0] as u32).wrapping_add(0x2018), c.g[T5][0]);
            c.sb((c.g[GP][0] as u32).wrapping_add(0x2020), c.g[T6][0]);
            c.g[T1][0] = c.g[T2][0] & 0xff;
            c.sb((c.g[GP][0] as u32).wrapping_add(0x2048), c.g[V0][0]);
            c.g[T3][0] = c.g[T5][0] & 0xff;
            c.sb((c.g[GP][0] as u32).wrapping_add(0x2050), c.g[V1][0]);
            c.g[T4][0] = c.g[T6][0] & 0xff;
            c.sb((c.g[GP][0] as u32).wrapping_add(0x2058), c.g[A0][0]);
            c.g[T1][0] = c.g[T1][0] | 0x300;
            c.sb((c.g[GP][0] as u32).wrapping_add(0x2060), c.g[A1][0]);
            c.g[A0][0] = c.g[V1][0] & 0xff;
            c.g[A1][0] = c.g[V0][0] & 0xff;
            c.sb((c.g[GP][0] as u32).wrapping_add(0x2028), c.g[S0][0]);
            c.sb((c.g[GP][0] as u32).wrapping_add(0x2030), c.g[S2][0]);
            c.g[T3][0] = c.g[T3][0] | 0x400;
            c.sb((c.g[GP][0] as u32).wrapping_add(0x2038), c.g[S3][0]);
            c.g[T2][0] = c.g[S0][0] & 0xff;
            c.sb((c.g[GP][0] as u32).wrapping_add(0x2040), c.g[FP][0]);
            c.g[T0][0] = c.g[S2][0] & 0xff;
            c.g[V1][0] = c.lbu((c.g[GP][0] as u32).wrapping_add(0x2058));
            c.g[A3][0] = c.g[S3][0] & 0xff;
            c.g[V0][0] = c.lbu((c.g[GP][0] as u32).wrapping_add(0x2060));
            c.g[A2][0] = c.g[FP][0] & 0xff;
            c.g[T4][0] = c.g[T4][0] | 0x500;
            c.g[T2][0] = c.g[T2][0] | 0x600;
            c.g[T0][0] = c.g[T0][0] | 0x700;
            c.g[A3][0] = c.g[A3][0] | 0x800;
            c.g[A2][0] = c.g[A2][0] | 0x900;
            c.g[A1][0] = c.g[A1][0] | 0xa00;
            c.g[A0][0] = c.g[A0][0] | 0xb00;
            c.g[V1][0] = c.g[V1][0] | 0xc00;
            c.g[V0][0] = c.g[V0][0] | 0xd00;
            c.sw((c.g[GP][0] as u32).wrapping_add(0x2010), c.g[T1][0]);
            c.sw((c.g[GP][0] as u32).wrapping_add(0x2018), c.g[T3][0]);
            c.sw((c.g[GP][0] as u32).wrapping_add(0x2020), c.g[T4][0]);
            c.sw((c.g[GP][0] as u32).wrapping_add(0x2028), c.g[T2][0]);
            c.sw((c.g[GP][0] as u32).wrapping_add(0x2030), c.g[T0][0]);
            c.sw((c.g[GP][0] as u32).wrapping_add(0x2038), c.g[A3][0]);
            c.sw((c.g[GP][0] as u32).wrapping_add(0x2040), c.g[A2][0]);
            c.sw((c.g[GP][0] as u32).wrapping_add(0x2048), c.g[A1][0]);
            c.sw((c.g[GP][0] as u32).wrapping_add(0x2050), c.g[A0][0]);
            c.sw((c.g[GP][0] as u32).wrapping_add(0x2058), c.g[V1][0]);
            c.sw((c.g[GP][0] as u32).wrapping_add(0x2060), c.g[V0][0]);
        }
    }
    c.cycles += 11;
    frame.restore(c, S0, 0x80);
    frame.restore(c, S1, 0x70);
    frame.restore(c, S2, 0x60);
    frame.restore(c, S3, 0x50);
    frame.restore(c, S4, 0x40);
    frame.restore(c, S5, 0x30);
    frame.restore(c, S6, 0x20);
    frame.restore(c, S7, 0x10);
    frame.restore(c, 30, 0);
    frame.close(c);
    ret(c);
}

// ---------------------------------------------------------------------------
// NetEvent10D_Clone.

pub(crate) const NET_EVENT_CLONE_SIZE: u32 = 0x164;

pub(crate) const NET_EVENT_CLONE_VTABLE: u32 = 0x0048_1110;

pub(crate) const NET_EVENT_CLONE_TYPE: u32 = 0x10d;

/// `void* NetEvent10D_Clone(NetEvent* self)` (0x2687c0): allocate a 0x164-byte
/// copy of the event, stamp type 0x10d and the clone vtable, copy the three
/// strings at +0x8/+0x28/+0x48, the words at +0x148/+0x14c and the blob at
/// +0x150, and return the copy.
pub fn net_event10d_clone(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 52;
        let frame = Frame::open(c, 0x50);
        c.g[A1][0] = 0x48_0000;
        frame.save(c, S1, 0x30);
        c.g[A2][0] = 0x2000_0000;
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x40);
        frame.save(c, S2, 0x20);
        c.g[A3][0] = 0;
        frame.save(c, S3, 0x10);
        c.g[A1][0] = sx(NET_EVENT_TAG);
        frame.save_ra(c, 0);
        c.g[A0][0] = NET_EVENT_CLONE_SIZE as u64;
        call_ptr(c, callee::EA_MEM_ALLOC_NAMED.addr, 0x0026_87f4)?;
        c.g[S3][0] = sx((c.g[S1][0] as u32).wrapping_add(0x28));
        c.g[S0][0] = c.g[V0][0];
        c.g[A2][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x148)).lw(c);
        c.g[A3][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x14c)).lw(c);
        c.g[V1][0] = sx(NET_EVENT_CLONE_VTABLE);
        c.g[V0][0] = NET_EVENT_CLONE_TYPE as u64;
        Addr((c.g[S0][0] as u32).wrapping_add(0x4)).sw(c, c.g[V1][0]);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x8));
        Addr((c.g[S0][0] as u32).wrapping_add(0x148)).sw(c, c.g[A2][0]);
        c.g[A1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x8));
        Addr((c.g[S0][0] as u32).wrapping_add(0x14c)).sw(c, c.g[A3][0]);
        c.g[S2][0] = sx((c.g[S1][0] as u32).wrapping_add(0x48));
        Addr(c.g[S0][0] as u32).sw(c, c.g[V0][0]);
        call_ptr(c, callee::STRCPY.addr, 0x0026_8830)?;
        c.g[A1][0] = c.g[S3][0];
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x28));
        call_ptr(c, callee::STRCPY.addr, 0x0026_883c)?;
        c.g[A1][0] = c.g[S2][0];
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x48));
        call_ptr(c, callee::STRCPY.addr, 0x0026_8848)?;
        c.ldl(3, (c.g[S1][0] as u32).wrapping_add(0x157));
        c.ldr(3, (c.g[S1][0] as u32).wrapping_add(0x150));
        c.ldl(4, (c.g[S1][0] as u32).wrapping_add(0x15f));
        c.ldr(4, (c.g[S1][0] as u32).wrapping_add(0x158));
        c.g[A1][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x160)).lw(c);
        c.sdl(3, (c.g[S0][0] as u32).wrapping_add(0x157));
        c.sdr(3, (c.g[S0][0] as u32).wrapping_add(0x150));
        c.sdl(4, (c.g[S0][0] as u32).wrapping_add(0x15f));
        c.sdr(4, (c.g[S0][0] as u32).wrapping_add(0x158));
        Addr((c.g[S0][0] as u32).wrapping_add(0x160)).sw(c, c.g[A1][0]);
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

// ---------------------------------------------------------------------------
// LobbyMan_ListClear.

pub(crate) const LOBBY_LIST_HEAD: u32 = 0x4;

/// `void LobbyMan_ListClear(list* self)` (0x2670c8): delete every node of the
/// circular list headed at self+4, then re-link the head to itself. The loop
/// head is 0x2670f0 (back-edge event check there, as generated).
pub fn lobby_man_list_clear(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr((c.g[S1][0] as u32).wrapping_add(LOBBY_LIST_HEAD)).lw(c);
        c.g[S0][0] = Addr(c.g[V0][0] as u32).lw(c);
        if c.g[S0][0] != c.g[V0][0] {
            c.cycles += 1;
            c.g[A0][0] = c.g[S0][0];
            loop {
                c.cycles += 5;
                c.g[S0][0] = Addr(c.g[S0][0] as u32).lw(c);
                call_ptr(c, callee::BUILTIN_DELETE, 0x0026_70f8)?;
                c.g[V0][0] = Addr((c.g[S1][0] as u32).wrapping_add(LOBBY_LIST_HEAD)).lw(c);
                let more = c.g[S0][0] != c.g[V0][0];
                c.g[A0][0] = c.g[S0][0];
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x0026_70f0);
                    }
                    continue;
                }
                c.cycles += 2;
                c.g[V0][0] = Addr((c.g[S1][0] as u32).wrapping_add(LOBBY_LIST_HEAD)).lw(c);
                break;
            }
        }
        c.cycles += 7;
        Addr(c.g[V0][0] as u32).sw(c, c.g[V0][0]);
        frame.restore_ra(c, 0);
        c.g[V1][0] = Addr((c.g[S1][0] as u32).wrapping_add(LOBBY_LIST_HEAD)).lw(c);
        frame.restore(c, S1, 0x10);
        Addr((c.g[V1][0] as u32).wrapping_add(0x4)).sw(c, c.g[V1][0]);
        frame.restore(c, S0, 0x20);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// cVoipMan_DebugPrintfStub.

/// `void* cVoipMan_DebugPrintfStub(void* self, ...)` (0x266fc0): varargs stub
/// that spills the argument registers/FPRs to its frame and returns `self`
/// unchanged.
pub fn c_voip_man_debug_printf_stub(c: &mut Cpu) {
    c.cycles += 15;
    let frame = Frame::open(c, 0x80);
    let base = sp(c);
    c.g[V0][0] = c.g[A0][0];
    c.sd(base.wrapping_add(0x48), c.g[A1][0]);
    c.sd(base.wrapping_add(0x50), c.g[A2][0]);
    c.sd(base.wrapping_add(0x58), c.g[A3][0]);
    c.sd(base.wrapping_add(0x60), c.g[T0][0]);
    c.sd(base.wrapping_add(0x68), c.g[T1][0]);
    c.sd(base.wrapping_add(0x70), c.g[T2][0]);
    c.sd(base.wrapping_add(0x78), c.g[T3][0]);
    c.swc1(12, base.wrapping_add(0x38));
    c.swc1(14, base.wrapping_add(0x3c));
    c.swc1(16, base.wrapping_add(0x40));
    c.swc1(18, base.wrapping_add(0x44));
    frame.close(c);
    ret(c);
}

// ---------------------------------------------------------------------------
// cUsbKeyboard_dtor.

/// `void cUsbKeyboard_dtor(cUsbKeyboard* self, int flags)` (0x266850): shut
/// the keyboard down if it was opened (`UsbKb_Shutdown`, then clear the
/// word at self+0x460), and delete the object when bit 0 of `flags` is set.
pub fn c_usb_keyboard_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(usb_kb::ACTIVE)).lw(c);
        let empty = c.g[V0][0] == 0;
        c.g[S1][0] = c.g[A1][0];
        if !empty {
            c.cycles += 3;
            call_ptr(c, callee::USB_KB_SHUTDOWN, 0x0026_6878)?;
            Addr((c.g[S0][0] as u32).wrapping_add(usb_kb::ACTIVE)).sw(c, 0);
        }
        c.cycles += 3;
        c.g[V0][0] = c.g[S1][0] & 0x1;
        if c.g[V0][0] != 0 {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, callee::BUILTIN_DELETE, 0x0026_6890)?;
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// cSetupMan_SendQuit.

/// The `"StlBlock"` allocation name.
pub(crate) const STL_BLOCK_TAG: u32 = 0x0048_04a8;

pub(crate) const SETUP_STATE_QUIT: u32 = 0xb;

pub(crate) const SETUP_QUIT_ID: u32 = 0xf9;

/// `void cSetupMan_SendQuit(cSetupMan* self, int id)` (0x265768): unless the
/// setup state is already 10, build/send the quit packet (allocating and
/// queueing a 0x10c event when `id == 0xf9`), notify the lobby, and set the
/// state to 11.
pub fn c_setup_man_send_quit(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x70);
        c.g[V0][0] = 0xa;
        frame.save(c, S2, 0x40);
        frame.save(c, S3, 0x30);
        frame.save(c, S0, 0x60);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S1, 0x50);
        frame.save_ra(c, 0x20);
        c.g[V1][0] = Addr(c.g[S3][0] as u32).lw(c);
        let already = c.g[V1][0] == c.g[V0][0];
        c.g[S2][0] = c.g[A1][0];
        if !already {
            c.cycles += 3;
            c.g[V0][0] = SETUP_QUIT_ID as u64;
            let is_quit = c.g[S2][0] == c.g[V0][0];
            c.g[S0][0] = c.g[S2][0];
            if is_quit {
                c.cycles += 26;
                c.g[A1][0] = 0x48_0000;
                c.g[S0][0] = c.lw(gp_8(c, LOBBY_INSTANCE_GP));
                c.g[A1][0] = sx(NET_EVENT_TAG);
                c.g[A0][0] = 0x8;
                c.g[A2][0] = 0x2000_0000;
                c.g[A3][0] = 0;
                call_ptr(c, callee::EA_MEM_ALLOC_NAMED.addr, 0x0026_57bc)?;
                c.g[S1][0] = c.g[V0][0];
                c.g[V1][0] = 0x10c;
                c.g[V0][0] = 0x48_0000;
                Addr(c.g[S1][0] as u32).sw(c, c.g[V1][0]);
                c.g[V0][0] = sx(NET_EVENT_VTABLE);
                c.g[A1][0] = 0x48_0000;
                Addr((c.g[S1][0] as u32).wrapping_add(0x4)).sw(c, c.g[V0][0]);
                c.g[A1][0] = sx(STL_BLOCK_TAG);
                c.g[A0][0] = 0xc;
                c.g[A2][0] = 0x2000_0000;
                c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(SETUP_LIST_HEAD)).lw(c);
                c.g[A3][0] = 0;
                c.g[S0][0] = 0xf8;
                Addr(frame.at(c, 0x10)).sw(c, c.g[V0][0]);
                call_ptr(c, callee::EA_MEM_ALLOC_NAMED.addr, 0x0026_57f8)?;
                c.g[A1][0] = c.g[V0][0];
                c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x8));
                if c.g[V0][0] != 0 {
                    Addr((c.g[A1][0] as u32).wrapping_add(0x8)).sw(c, c.g[S1][0]);
                }
                c.cycles += 9;
                setup_insert_list_tail(c, 0x10);
                Addr(frame.at(c, 0)).sw(c, c.g[A1][0]);
            }
            c.cycles += 14;
            c.g[V0][0] = 0xffff_ffff_ffff_0000;
            c.g[A1][0] = c.g[SP][0];
            c.g[V0][0] |= 0xffff;
            Addr(sp(c)).sw(c, c.g[S0][0]);
            Addr(sp(c).wrapping_add(0x4)).sw(c, c.g[V0][0]);
            c.g[A0][0] = c.g[S3][0];
            Addr(sp(c).wrapping_add(0x8)).sw(c, 0);
            c.g[A2][0] = 0x14;
            callee::CSETUP_MAN_SEND_PACKET.call(c, 0x0026_5850)?;
            c.g[A0][0] = c.lw(gp_8(c, LOBBY_INSTANCE_GP));
            c.g[A1][0] = c.g[S2][0];
            call_ptr(c, callee::LOBBY_MAN_CLEAR_CHALLENGE_LOCK, 0x0026_585c)?;
            c.g[V0][0] = SETUP_STATE_QUIT as u64;
            Addr(c.g[S3][0] as u32).sw(c, c.g[V0][0]);
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// BuddyMan vector inserts.

pub(crate) const BUDDY_VEC_FIRST: u32 = 0x4;

pub(crate) const BUDDY_VEC_LAST: u32 = 0x8;

pub(crate) const BUDDY_VEC_CAP: u32 = 0xc;

/// The pointer-vector insert both BuddyMan vectors compile to: insert `*x`
/// at `pos`, growing (allocate, copy, free) when full. The two copies differ
/// only in their return addresses (`rets`, in call order).
fn buddy_vec_insert(c: &mut Cpu, rets: [u32; 7]) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x70);
        'outer: {
            {
                c.cycles += 14;
                frame.save(c, S1, 0x50);
                frame.save(c, S3, 0x30);
                c.g[S1][0] = c.g[A0][0];
                frame.save(c, S4, 0x20);
                c.g[S3][0] = c.g[A1][0];
                frame.save(c, S0, 0x60);
                frame.save(c, S2, 0x40);
                frame.save(c, S5, 0x10);
                frame.save_ra(c, 0);
                c.g[V1][0] = Addr((c.g[S1][0] as u32).wrapping_add(BUDDY_VEC_LAST)).lw(c);
                c.g[V0][0] = Addr((c.g[S1][0] as u32).wrapping_add(BUDDY_VEC_CAP)).lw(c);
                let full = c.g[V1][0] == c.g[V0][0];
                c.g[S4][0] = c.g[A2][0];
                if !full {
                    c.cycles += 2;
                    if c.g[V1][0] == 0 {
                        c.g[A0][0] = Addr((c.g[S1][0] as u32).wrapping_add(BUDDY_VEC_LAST)).lw(c);
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_sub(0x4)).lw(c);
                        Addr(c.g[V1][0] as u32).sw(c, c.g[V0][0]);
                        c.g[A0][0] = Addr((c.g[S1][0] as u32).wrapping_add(BUDDY_VEC_LAST)).lw(c);
                    }
                    c.cycles += 12;
                    c.g[A1][0] = c.g[S3][0];
                    c.g[A2][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x4));
                    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4));
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(c.g[S3][0] as u32));
                    Addr((c.g[S1][0] as u32).wrapping_add(BUDDY_VEC_LAST)).sw(c, c.g[V0][0]);
                    c.g[A2][0] = sx(((c.g[A2][0] as u32) as i32 >> 2) as u32);
                    c.g[A2][0] = sx((c.g[A2][0] as u32) << 2);
                    c.g[S0][0] = Addr(c.g[S4][0] as u32).lw(c);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(c.g[A2][0] as u32));
                    call_ptr(c, callee::MEM_MOVE, rets[0])?;
                    Addr(c.g[S3][0] as u32).sw(c, c.g[S0][0]);
                    break 'outer;
                }
            }
            'alloc: {
                {
                    c.cycles += 9;
                    c.g[V0][0] = Addr((c.g[S1][0] as u32).wrapping_add(BUDDY_VEC_FIRST)).lw(c);
                    c.g[S0][0] = 0x1;
                    c.g[A0][0] = 0x1;
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                    c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 2) as u32);
                    c.g[V1][0] = sx((c.g[V0][0] as u32) << 1);
                    if c.g[V0][0] != 0 {
                        c.g[S0][0] = c.g[V1][0];
                    }
                    let need = c.g[S0][0] != c.g[A0][0];
                    c.g[S0][0] = sx((c.g[S0][0] as u32) << 2);
                    if !need {
                        c.cycles += 9;
                        c.g[A1][0] = 0x48_0000;
                        c.g[A0][0] = 0x4;
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4a8));
                        c.g[A2][0] = 0x2000_0000;
                        c.g[A3][0] = 0;
                        c.g[S5][0] = 0x4;
                        call_ptr(c, callee::EA_MEM_ALLOC_NAMED.addr, rets[1])?;
                        c.g[S2][0] = c.g[V0][0];
                        break 'alloc;
                    }
                }
                c.cycles += 8;
                c.g[A1][0] = 0x48_0000;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4a8));
                c.g[A0][0] = c.g[S0][0];
                c.g[A2][0] = 0x2000_0000;
                c.g[A3][0] = 0;
                call_ptr(c, callee::OPERATOR_NEW_NAMED.addr, rets[2])?;
                c.g[S5][0] = c.g[S0][0];
                c.g[S2][0] = c.g[V0][0];
            }
            c.cycles += 9;
            c.g[A1][0] = Addr((c.g[S1][0] as u32).wrapping_add(BUDDY_VEC_FIRST)).lw(c);
            c.g[A0][0] = c.g[S2][0];
            c.g[V0][0] = sx((c.g[S3][0] as u32).wrapping_sub(c.g[A1][0] as u32));
            c.g[A2][0] = sx(((c.g[V0][0] as u32) as i32 >> 2) as u32);
            c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A2][0] = sx((c.g[A2][0] as u32) << 2);
            call_ptr(c, callee::MEM_MOVE, rets[3])?;
            let at_pos = c.g[S0][0] == 0;
            c.g[V1][0] = c.g[S0][0];
            if !at_pos {
                c.cycles += 2;
                c.g[V0][0] = Addr(c.g[S4][0] as u32).lw(c);
                Addr(c.g[S0][0] as u32).sw(c, c.g[V0][0]);
            }
            'tail: {
                {
                    c.cycles += 14;
                    c.g[A2][0] = Addr((c.g[S1][0] as u32).wrapping_add(BUDDY_VEC_LAST)).lw(c);
                    c.g[S0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4));
                    c.g[A1][0] = c.g[S3][0];
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(c.g[S3][0] as u32));
                    c.g[S3][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A2][0] as u32));
                    c.g[A2][0] = sx(((c.g[A2][0] as u32) as i32 >> 2) as u32);
                    c.g[A2][0] = sx((c.g[A2][0] as u32) << 2);
                    call_ptr(c, callee::MEM_MOVE, rets[4])?;
                    c.g[A0][0] = Addr((c.g[S1][0] as u32).wrapping_add(BUDDY_VEC_FIRST)).lw(c);
                    c.g[V0][0] = Addr((c.g[S1][0] as u32).wrapping_add(BUDDY_VEC_CAP)).lw(c);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[A0][0] as u32));
                    let no_old = c.g[A0][0] == 0;
                    c.g[V1][0] = sx(((c.g[V0][0] as u32) as i32 >> 2) as u32);
                    if !no_old {
                        c.cycles += 3;
                        c.g[V0][0] = 0x1;
                        if c.g[V1][0] == c.g[V0][0] {
                            c.cycles += 4;
                            call_ptr(c, callee::BUILTIN_DELETE, rets[5])?;
                            c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[S5][0] as u32));
                            break 'tail;
                        }
                        c.cycles += 2;
                        call_ptr(c, callee::EA_MEM_FREE.addr, rets[6])?;
                    }
                }
                c.cycles += 1;
                c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[S5][0] as u32));
            }
            c.cycles += 3;
            Addr((c.g[S1][0] as u32).wrapping_add(BUDDY_VEC_LAST)).sw(c, c.g[S3][0]);
            Addr((c.g[S1][0] as u32).wrapping_add(BUDDY_VEC_CAP)).sw(c, c.g[V0][0]);
            Addr((c.g[S1][0] as u32).wrapping_add(BUDDY_VEC_FIRST)).sw(c, c.g[S2][0]);
        }
        c.cycles += 9;
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

/// `void BuddyMan_MessageVectorInsert(vec* self, void** pos, void* const& x)`
/// (0x267128): insert into the message pointer vector.
pub fn buddy_man_message_vector_insert(c: &mut Cpu) {
    buddy_vec_insert(
        c,
        [
            0x0026_719c,
            0x0026_71e4,
            0x0026_7204,
            0x0026_7228,
            0x0026_725c,
            0x0026_7284,
            0x0026_7294,
        ],
    );
}

/// `void BuddyMan_ChallengerVectorInsert(vec* self, void** pos, void* const& x)`
/// (0x2672c8): insert into the challenger pointer vector.
pub fn buddy_man_challenger_vector_insert(c: &mut Cpu) {
    buddy_vec_insert(
        c,
        [
            0x0026_733c,
            0x0026_7384,
            0x0026_73a4,
            0x0026_73c8,
            0x0026_73fc,
            0x0026_7424,
            0x0026_7434,
        ],
    );
}

// ---------------------------------------------------------------------------
// cSharedLoginMan_McMapError: match-pc dispatcher (value jump tables at
// 0x480fe0/0x481020/0x481040/0x481060, event heads listed in the doc).

/// `int cSharedLoginMan_McMapError(int err)` (0x266578): map a memory-card
/// error code to a result code via jump tables indexed by the mode word at
/// gp+0x2e4 and by `err`. Dispatcher: resume heads (event unwind targets in
/// the generated body) are 0x2665b4, 0x2665b8, 0x2665c8, 0x2665dc, 0x26660c,
/// 0x266658 and 0x266660; every arm is re-enterable.
pub fn c_shared_login_man_mc_map_error(c: &mut Cpu) {
    let mut pc: u32 = c.pc;
    loop {
        match pc {
            0x0026_6578 => {
                c.cycles += 5;
                c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_add(0x2e4)).lw(c);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
                c.g[V0][0] = (c.g[V1][0] < 0xe) as u64;
                let out = c.g[V0][0] == 0;
                c.g[A1][0] = c.g[A0][0];
                pc = if out { 0x0026_6718 } else { 0x0026_658c };
            }
            0x0026_658c => {
                c.cycles += 7;
                c.g[V0][0] = 0x48_0000;
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xfe0));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[A0][0] = Addr(c.g[V1][0] as u32).lw(c);
                pc = c.g[A0][0] as u32;
            }
            0x0026_65a8 => {
                c.cycles += 3;
                c.g[V0][0] = 0xffff_ffff_ffff_fffc;
                pc = if c.g[A1][0] == c.g[V0][0] {
                    0x0026_65c8
                } else {
                    0x0026_65b4
                };
            }
            0x0026_65b4 => {
                c.cycles += 1;
                c.g[V0][0] = 0xffff_ffff_ffff_fffe;
                pc = 0x0026_65b8;
            }
            0x0026_65b8 => {
                c.cycles += 2;
                let ne = c.g[A1][0] != c.g[V0][0];
                c.g[V0][0] = 0x3;
                pc = if ne { 0x0026_671c } else { 0x0026_65c0 };
            }
            0x0026_65c0 => {
                c.cycles += 2;
                c.g[V0][0] = 0x9;
                ret(c);
                return;
            }
            0x0026_65c8 => {
                c.cycles += 2;
                c.g[V0][0] = 0x4;
                ret(c);
                return;
            }
            0x0026_65d0 => {
                c.cycles += 3;
                c.g[V0][0] = 0xffff_ffff_ffff_fffe;
                let ne = c.g[A1][0] != c.g[V0][0];
                c.g[V0][0] = 0x3;
                pc = if ne { 0x0026_671c } else { 0x0026_65dc };
            }
            0x0026_65dc => {
                c.cycles += 2;
                c.g[V0][0] = 0x11;
                ret(c);
                return;
            }
            0x0026_65e4 => {
                c.cycles += 4;
                c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x7));
                c.g[V0][0] = (c.g[V1][0] < 0x6) as u64;
                let out = c.g[V0][0] == 0;
                c.g[V0][0] = 0x48_0000;
                pc = if out { 0x0026_6718 } else { 0x0026_65f4 };
            }
            0x0026_65f4 => {
                c.cycles += 6;
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1020));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[A0][0] = Addr(c.g[V1][0] as u32).lw(c);
                pc = c.g[A0][0] as u32;
            }
            0x0026_660c => {
                c.cycles += 2;
                c.g[V0][0] = 0xd;
                ret(c);
                return;
            }
            0x0026_6614 => {
                c.cycles += 3;
                c.g[V0][0] = 0xffff_ffff_ffff_fffc;
                let eq = c.g[A1][0] == c.g[V0][0];
                c.g[V0][0] = 0xffff_ffff_ffff_fffe;
                if eq {
                    if back_edge_unwinds(c, 0x0026_65c8) {
                        return;
                    }
                    pc = 0x0026_65c8;
                } else {
                    pc = 0x0026_6620;
                }
            }
            0x0026_6620 => {
                c.cycles += 2;
                if c.g[A1][0] != c.g[V0][0] {
                    if back_edge_unwinds(c, 0x0026_65dc) {
                        return;
                    }
                    pc = 0x0026_65dc;
                } else {
                    pc = 0x0026_6628;
                }
            }
            0x0026_6628 => {
                c.cycles += 2;
                c.g[V0][0] = 0x9;
                ret(c);
                return;
            }
            0x0026_6630 => {
                c.cycles += 4;
                c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x8));
                c.g[V0][0] = (c.g[V1][0] < 0x7) as u64;
                let out = c.g[V0][0] == 0;
                c.g[V0][0] = 0x48_0000;
                pc = if out { 0x0026_6718 } else { 0x0026_6640 };
            }
            0x0026_6640 => {
                c.cycles += 6;
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1040));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[A0][0] = Addr(c.g[V1][0] as u32).lw(c);
                pc = c.g[A0][0] as u32;
            }
            0x0026_6658 => {
                c.cycles += 2;
                c.g[V0][0] = 0xe;
                ret(c);
                return;
            }
            0x0026_6660 => {
                c.cycles += 3;
                c.g[V0][0] = 0xffff_ffff_ffff_fffc;
                let ne = c.g[A1][0] != c.g[V0][0];
                c.g[V0][0] = 0xffff_ffff_ffff_fffe;
                if ne {
                    if back_edge_unwinds(c, 0x0026_65b8) {
                        return;
                    }
                    pc = 0x0026_65b8;
                } else {
                    pc = 0x0026_666c;
                }
            }
            0x0026_666c => {
                c.cycles += 2;
                if back_edge_unwinds(c, 0x0026_65c8) {
                    return;
                }
                pc = 0x0026_65c8;
            }
            0x0026_6674 => {
                c.cycles += 3;
                c.g[V0][0] = 0xffff_ffff_ffff_fffc;
                let eq = c.g[A1][0] == c.g[V0][0];
                c.g[V0][0] = ((c.g[A1][0] as i64) < -3) as u64;
                if eq {
                    if back_edge_unwinds(c, 0x0026_65c8) {
                        return;
                    }
                    pc = 0x0026_65c8;
                } else {
                    pc = 0x0026_6680;
                }
            }
            0x0026_6680 => {
                c.cycles += 2;
                let z = c.g[V0][0] == 0;
                c.g[V0][0] = 0xffff_ffff_ffff_fffb;
                if z {
                    if back_edge_unwinds(c, 0x0026_65b4) {
                        return;
                    }
                    pc = 0x0026_65b4;
                } else {
                    pc = 0x0026_6688;
                }
            }
            0x0026_6688 => {
                c.cycles += 2;
                let ne = c.g[A1][0] != c.g[V0][0];
                c.g[V0][0] = 0x3;
                pc = if ne { 0x0026_671c } else { 0x0026_6690 };
            }
            0x0026_6690 => {
                c.cycles += 2;
                if back_edge_unwinds(c, 0x0026_660c) {
                    return;
                }
                pc = 0x0026_660c;
            }
            0x0026_6698 => {
                c.cycles += 4;
                c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x8));
                c.g[V0][0] = (c.g[V1][0] < 0x7) as u64;
                let out = c.g[V0][0] == 0;
                c.g[V0][0] = 0x48_0000;
                pc = if out { 0x0026_6718 } else { 0x0026_66a8 };
            }
            0x0026_66a8 => {
                c.cycles += 6;
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1060));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[A0][0] = Addr(c.g[V1][0] as u32).lw(c);
                pc = c.g[A0][0] as u32;
            }
            0x0026_66c0 => {
                c.cycles += 3;
                c.g[V0][0] = 0xffff_ffff_ffff_fffd;
                let eq = c.g[A1][0] == c.g[V0][0];
                c.g[V0][0] = ((c.g[A1][0] as i64) < -2) as u64;
                if eq {
                    if back_edge_unwinds(c, 0x0026_6658) {
                        return;
                    }
                    pc = 0x0026_6658;
                } else {
                    pc = 0x0026_66cc;
                }
            }
            0x0026_66cc => {
                c.cycles += 2;
                let z = c.g[V0][0] == 0;
                c.g[V0][0] = 0xffff_ffff_ffff_fffc;
                if z {
                    if back_edge_unwinds(c, 0x0026_65b4) {
                        return;
                    }
                    pc = 0x0026_65b4;
                } else {
                    pc = 0x0026_66d4;
                }
            }
            0x0026_66d4 => {
                c.cycles += 2;
                let ne = c.g[A1][0] != c.g[V0][0];
                c.g[V0][0] = 0x3;
                pc = if ne { 0x0026_671c } else { 0x0026_66dc };
            }
            0x0026_66dc => {
                c.cycles += 2;
                c.g[V0][0] = 0x8;
                ret(c);
                return;
            }
            0x0026_66e4 => {
                c.cycles += 3;
                c.g[V0][0] = 0xffff_ffff_ffff_fffb;
                let eq = c.g[A1][0] == c.g[V0][0];
                c.g[V0][0] = ((c.g[A1][0] as i64) < -4) as u64;
                pc = if eq { 0x0026_6708 } else { 0x0026_66f0 };
            }
            0x0026_66f0 => {
                c.cycles += 2;
                let z = c.g[V0][0] == 0;
                c.g[V0][0] = 0xffff_ffff_ffff_fffa;
                if z {
                    if back_edge_unwinds(c, 0x0026_6660) {
                        return;
                    }
                    pc = 0x0026_6660;
                } else {
                    pc = 0x0026_66f8;
                }
            }
            0x0026_66f8 => {
                c.cycles += 2;
                let eq = c.g[A1][0] == c.g[V0][0];
                c.g[V0][0] = 0x3;
                pc = if eq { 0x0026_6710 } else { 0x0026_6700 };
            }
            0x0026_6700 => {
                c.cycles += 2;
                pc = 0x0026_671c;
            }
            0x0026_6708 => {
                c.cycles += 2;
                c.g[V0][0] = 0x5;
                ret(c);
                return;
            }
            0x0026_6710 => {
                c.cycles += 2;
                c.g[V0][0] = 0x6;
                ret(c);
                return;
            }
            0x0026_6718 => {
                c.cycles += 1;
                c.g[V0][0] = 0x3;
                pc = 0x0026_671c;
            }
            0x0026_671c => {
                c.cycles += 2;
                ret(c);
                return;
            }
            _ => {
                c.jump_out(pc, 0x0026_6578, 0x0026_6724);
                return;
            }
        }
    }
}

// ---------------------------------------------------------------------------
// cSharedLoginMan_McPoll.

/// `gp` slots of the shared-login memory-card stage (offsets from `gp`).
/// The operation word McPoll dispatches on.
pub(crate) const MC_MODE_GP: i32 = 0x2e4;
/// The `cPS2MemCard` object.
pub(crate) const MC_CARD_GP: i32 = 0x2e8;
/// Second argument of `cPS2MemCard_RequestGetInfoNow`.
pub(crate) const MC_INFO_ARG_GP: i32 = 0x2ec;
/// Set to 1 when an info request comes due with no retries left.
pub(crate) const MC_RETRY_FAILED_GP: i32 = 0x2f0;
/// Info-request retries left.
pub(crate) const MC_RETRIES_GP: i32 = 0x2f4;
/// The word McPoll stores as its result for table arms 0x2664e8..0x2664f8.
pub(crate) const MC_SAVED_RESULT_GP: i32 = 0x850;
/// McPoll's jump table, one word per mode below 15.
pub(crate) const MC_POLL_TABLE: u32 = 0x0048_0fa0;
/// The buffer `cPS2MemCard_GetDirEntryName` fills for the availability check.
pub(crate) const MC_DIR_NAME_BUF: u32 = 0x0053_7190;

/// How `cSharedLoginMan_McPoll` ends.
#[derive(Clone, Copy, PartialEq)]
enum McPollEnd {
    /// `*status = v1`, return 0.
    SetStatus,
    /// `*result = v1`, return 0.
    SetResult,
    /// `*result = v0`, return 0.
    SetSavedResult,
    /// Return 0.
    ReturnZero,
    /// Return `v0` as it stands.
    ReturnV0,
    /// Return what `cSharedLoginMan_McMapError` returned (`s0` already
    /// reloaded).
    MapError,
}

/// `int cSharedLoginMan_McPoll(int* status, int* result)` (0x266358): clear
/// `*result`, then by the operation word at `gp+MC_MODE_GP`: mode 10 sets
/// status 2 and result 0x800; modes 3, 15 and 16 set status 2; the others
/// ask the card object (vtable slot 0x58) and set status 1 if it answers
/// non-zero. Otherwise status becomes 2, a port error is mapped through
/// `cSharedLoginMan_McMapError` and returned, and without one modes below
/// 15 dispatch through the table at 0x480fa0 (retry an info request, scan
/// the directory entries for an available one, hand back the saved result,
/// or the card's vtable slot 0xa0 shifted left by 10).
///
/// The count loop's head 0x2664b0 checks events with the result ignored. A
/// table word equal to the function's start re-runs it from the prologue
/// (the real table never holds it). Entries other than the start go to the
/// generated `__entries` twin (override stub).
pub fn c_shared_login_man_mc_poll(c: &mut Cpu) {
    use McPollEnd::*;
    guest(c, |c| {
        let (frame, end) = 'restart: loop {
            c.cycles += 11;
            let frame = Frame::open(c, 0x40);
            c.g[V0][0] = 0xa;
            frame.save(c, S2, 0x10);
            frame.save(c, S0, 0x30);
            c.g[S2][0] = c.g[A1][0];
            frame.save(c, S1, 0x20);
            frame.save_ra(c, 0);
            Addr(c.g[S2][0] as u32).sw(c, 0);
            c.g[V1][0] = c.lw(gp_8(c, MC_MODE_GP));
            c.g[S0][0] = c.g[A0][0];
            let mode = c.g[V1][0] as i64;
            if mode == 0xa {
                c.cycles += 4;
                c.g[V0][0] = 0x2;
                c.g[V1][0] = 0x800;
                Addr(c.g[S0][0] as u32).sw(c, c.g[V0][0]);
                break (frame, SetResult);
            }
            c.cycles += 3;
            c.g[V0][0] = 0x3;
            if mode < 11 {
                c.cycles += 2;
                c.g[A1][0] = c.lw(gp_8(c, MC_CARD_GP));
                if mode == 3 {
                    c.cycles += 2;
                    c.g[V1][0] = 0x2;
                    break (frame, SetStatus);
                }
                c.cycles += 2;
            } else {
                c.cycles += 3;
                c.g[V0][0] = (mode < 15) as u64;
                if mode >= 17 {
                    c.cycles += 1;
                    c.g[A1][0] = c.lw(gp_8(c, MC_CARD_GP));
                } else {
                    c.cycles += 2;
                    c.g[A1][0] = c.lw(gp_8(c, MC_CARD_GP));
                    if mode >= 15 {
                        c.cycles += 2;
                        c.g[V1][0] = 0x2;
                        break (frame, SetStatus);
                    }
                }
                c.cycles += 1;
            }
            // Ask the card: vtable slot 0x58.
            let vt = c.lw(c.g[A1][0] as u32);
            c.g[V1][0] = vt;
            c.cycles += 6;
            let m = VSlot::read(c, vt, 0x58);
            c.g[A0][0] = m.delta;
            c.g[V0][0] = m.func;
            c.g[A0][0] = m.this(c.g[A1][0]);
            call_ptr(c, m.func as u32, 0x0026_63e4)?;
            let answered = c.g[V0][0] != 0;
            c.g[V1][0] = 0x1;
            if answered {
                break (frame, SetStatus);
            }
            // No answer: status 2, then a port error ends the poll.
            c.cycles += 7;
            c.g[V0][0] = 0x2;
            Addr(c.g[S0][0] as u32).sw(c, c.g[V0][0]);
            c.g[A0][0] = c.lw(gp_8(c, MC_CARD_GP));
            c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xc));
            call_ptr(c, callee::MEMCARD_GET_PORT_ERROR, 0x0026_640c)?;
            let error = c.g[V0][0] != 0;
            c.g[A0][0] = c.lw(gp_8(c, MC_CARD_GP));
            if error {
                c.cycles += 6;
                c.g[A1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xc));
                call_ptr(c, callee::MEMCARD_GET_PORT_ERROR, 0x0026_641c)?;
                c.g[A0][0] = c.g[V0][0];
                callee::SHARED_LOGIN_MAN_MC_MAP_ERROR.call(c, 0x0026_6424)?;
                frame.restore(c, S0, 0x30);
                break (frame, MapError);
            }
            c.cycles += 4;
            c.g[V1][0] = c.lw(gp_8(c, MC_MODE_GP));
            c.g[V0][0] = 0x48_0000;
            if c.g[V1][0] >= 0xf {
                break (frame, ReturnZero);
            }
            c.cycles += 6;
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
            c.g[V0][0] = sx(MC_POLL_TABLE);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A0][0] = c.lw(c.g[V1][0] as u32);
            let t = c.g[A0][0] as u32;
            let end = match t {
                0x0026_6358 => {
                    if event_due(c) {
                        deliver_event_here(c, 0x0026_6358);
                    }
                    continue 'restart;
                }
                0x0026_6454 => mc_poll_retry(c)?,
                0x0026_648c => mc_poll_scan(c)?,
                0x0026_64e8 | 0x0026_64f0 | 0x0026_64f8 => {
                    c.cycles += 2;
                    c.g[V0][0] = c.lw(gp_8(c, MC_SAVED_RESULT_GP));
                    SetSavedResult
                }
                0x0026_6510 => {
                    // The card's vtable slot 0xa0, shifted left by 10.
                    c.cycles += 7;
                    c.g[A1][0] = c.lw(gp_8(c, MC_CARD_GP));
                    let vt = c.lw(c.g[A1][0] as u32);
                    c.g[V1][0] = vt;
                    let m = VSlot::read(c, vt, 0xa0);
                    c.g[A0][0] = m.delta;
                    c.g[V0][0] = m.func;
                    c.g[A0][0] = m.this(c.g[A1][0]);
                    call_ptr(c, m.func as u32, 0x0026_6528)?;
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 10);
                    SetSavedResult
                }
                0x0026_6530 => ReturnZero,
                _ => {
                    c.jump_out(t, 0x0026_6358, 0x0026_654c);
                    return Ok(());
                }
            };
            break (frame, end);
        };
        match end {
            SetStatus => {
                c.cycles += 3;
                c.g[V0][0] = 0;
                Addr(c.g[S0][0] as u32).sw(c, c.g[V1][0]);
            }
            SetResult => {
                c.cycles += 3;
                c.g[V0][0] = 0;
                Addr(c.g[S2][0] as u32).sw(c, c.g[V1][0]);
            }
            SetSavedResult => {
                c.cycles += 2;
                Addr(c.g[S2][0] as u32).sw(c, c.g[V0][0]);
                c.g[V0][0] = 0;
            }
            ReturnZero => {
                c.cycles += 1;
                c.g[V0][0] = 0;
            }
            ReturnV0 | MapError => {}
        }
        if end != MapError {
            c.cycles += 1;
            frame.restore(c, S0, 0x30);
        }
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// McPoll's table arm 0x266454: with retries left, set status 1, spend one
/// and request the card info again (return `v0 = 0` as left); with none,
/// mark the retry as failed.
fn mc_poll_retry(c: &mut Cpu) -> Result<McPollEnd, crate::call::Unwound> {
    c.cycles += 3;
    c.g[V0][0] = c.lw(gp_8(c, MC_RETRIES_GP));
    let left = (c.g[V0][0] as i64) > 0;
    c.g[V0][0] = 0x1;
    if !left {
        c.cycles += 2;
        c.sw(gp_8(c, MC_RETRY_FAILED_GP), c.g[V0][0]);
        return Ok(McPollEnd::ReturnZero);
    }
    c.cycles += 9;
    Addr(c.g[S0][0] as u32).sw(c, c.g[V0][0]);
    c.g[V1][0] = c.lw(gp_8(c, MC_RETRIES_GP));
    c.g[A0][0] = c.lw(gp_8(c, MC_CARD_GP));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
    c.g[A1][0] = c.lw(gp_8(c, MC_INFO_ARG_GP));
    c.sw(gp_8(c, MC_RETRIES_GP), c.g[V1][0]);
    call_ptr(c, callee::MEMCARD_REQUEST_GET_INFO_NOW.addr, 0x0026_647c)?;
    c.g[V0][0] = 0;
    Ok(McPollEnd::ReturnV0)
}

/// McPoll's table arm 0x26648c: scan the card's directory entries (count at
/// card+0x54) for one `SLoginMc_IsAvailable_Wrapper` accepts; `*result` is
/// 1 if one is found, else 0.
fn mc_poll_scan(c: &mut Cpu) -> Result<McPollEnd, crate::call::Unwound> {
    c.cycles += 4;
    c.g[V0][0] = c.lw(gp_8(c, MC_CARD_GP));
    c.g[V0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x54));
    if (c.g[V0][0] as i64) <= 0 {
        Addr(c.g[S2][0] as u32).sw(c, 0);
        return Ok(McPollEnd::ReturnZero);
    }
    c.cycles += 5;
    c.g[S0][0] = 0;
    c.g[V0][0] = 0x53_0000;
    c.g[S1][0] = sx(MC_DIR_NAME_BUF);
    c.g[A0][0] = c.lw(gp_8(c, MC_CARD_GP));
    loop {
        c.cycles += 7;
        c.g[A1][0] = c.g[S0][0];
        c.g[A2][0] = c.g[S1][0];
        call_ptr(c, callee::MEMCARD_GET_DIR_ENTRY_NAME, 0x0026_64bc)?;
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, callee::SLOGIN_MC_IS_AVAILABLE_WRAPPER, 0x0026_64c4)?;
        let found = c.g[V0][0] != 0;
        c.g[V1][0] = c.lw(gp_8(c, MC_CARD_GP));
        if found {
            c.cycles += 1;
            c.g[V1][0] = 0x1;
            return Ok(McPollEnd::SetResult);
        }
        c.cycles += 5;
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
        c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x54));
        c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
        if c.g[V0][0] == 0 {
            c.cycles += 2;
            Addr(c.g[S2][0] as u32).sw(c, 0);
            return Ok(McPollEnd::ReturnZero);
        }
        c.g[A0][0] = c.lw(gp_8(c, MC_CARD_GP));
        if event_due(c) {
            deliver_event_here(c, 0x0026_64b0);
        }
    }
}

// ---------------------------------------------------------------------------
// cSharedLoginMan_Update.

/// `cSharedLoginMan_Update`'s busy check: ask the card object (vtable slot
/// 0x58, `this` adjusted by the slot's delta) and leave its answer in `v0`.
fn card_busy(c: &mut Cpu, ret: u32) -> crate::call::Flow {
    c.cycles += 8;
    c.g[A1][0] = c.lw(gp_8(c, MC_CARD_GP));
    c.g[V1][0] = c.lw(c.g[A1][0] as u32);
    let m = VSlot::read(c, c.g[V1][0], 0x58);
    c.g[A0][0] = m.delta;
    c.g[V0][0] = m.func;
    c.g[A0][0] = m.this(c.g[A1][0]);
    call_ptr(c, m.func as u32, ret)
}

/// `void cSharedLoginMan_Update(cSharedLoginMan* self)` (0x265a68): advance
/// the shared-login state machine: call the card object's vtable slot 0x18,
/// then for states below 0x17 dispatch through the table at 0x480f40: the
/// Do* stages (load, update, create, delete, finish) run when the card is
/// not busy (slot 0x58), the poll stage stores `SLogin_Poll`'s result at
/// `+4` and steps the state, and the last stages pick the next state from
/// the card's answer and `gp+0x850`. No loops. Entries other than the
/// start go to the generated `__entries` twin (override stub).
pub fn c_shared_login_man_update(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x30);
        c.cycles += 14;
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0x10);
        c.g[A1][0] = Addr((c.g[GP][0] as u32).wrapping_add(0x2e8)).lw(c);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = Addr(c.g[A1][0] as u32).lw(c);
        c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x18)).lh(c);
        c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x1c)).lw(c);
        let tgt = c.g[V1][0] as u32;
        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, tgt, 0x0026_5a90)?;
        c.g[V1][0] = Addr(c.g[S0][0] as u32).lw(c);
        c.g[V0][0] = (c.g[V1][0] < 0x17) as u64;
        let out_of_range = c.g[V0][0] == 0;
        Addr(frame.at(c, 0)).sw(c, 0);
        // How the chosen arm leaves: restore s0 here, store the new state
        // first, or the arm already restored s0 itself.
        #[derive(Clone, Copy, PartialEq)]
        enum After {
            RestoreExit,
            StoreExit,
            FullExit,
        }
        let mut after = After::RestoreExit;
        if !out_of_range {
            // The jump-table lookup (table at 0x480f40, indexed by state).
            c.cycles += 7;
            c.g[V0][0] = 0x48_0000;
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0xf40));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A0][0] = Addr(c.g[V1][0] as u32).lw(c);
            let t = c.g[A0][0] as u32;
            // One Do* arm: ask the manager object (slot +0x58); when it is
            // busy the arm ends, otherwise it runs its Do* stage.
            let do_arm = |c: &mut Cpu, stage: Callee, vret: u32, dret: u32| -> crate::call::Flow {
                card_busy(c, vret)?;
                if c.g[V0][0] != 0 {
                    frame.restore(c, S0, 0x20);
                    return Ok(());
                }
                c.cycles += 4;
                c.g[A0][0] = c.g[S0][0];
                stage.call(c, dret)?;
                frame.restore(c, S0, 0x20);
                Ok(())
            };
            after = match t {
                0x0026_5abc => {
                    do_arm(
                        c,
                        callee::SHARED_LOGIN_MAN_DO_LOAD,
                        0x0026_5ad4,
                        0x0026_5ae4,
                    )?;
                    After::FullExit
                }
                0x0026_5aec => {
                    do_arm(
                        c,
                        callee::SHARED_LOGIN_MAN_DO_UPDATE,
                        0x0026_5b04,
                        0x0026_5b14,
                    )?;
                    After::FullExit
                }
                0x0026_5b1c => {
                    do_arm(
                        c,
                        callee::SHARED_LOGIN_MAN_DO_CREATE,
                        0x0026_5b34,
                        0x0026_5b44,
                    )?;
                    After::FullExit
                }
                0x0026_5b4c => {
                    do_arm(
                        c,
                        callee::SHARED_LOGIN_MAN_DO_DELETE,
                        0x0026_5b64,
                        0x0026_5b74,
                    )?;
                    After::FullExit
                }
                0x0026_5b7c => {
                    do_arm(
                        c,
                        callee::SHARED_LOGIN_MAN_DO_FINISH,
                        0x0026_5b94,
                        0x0026_5ba4,
                    )?;
                    After::FullExit
                }
                0x0026_5bac => {
                    // Poll the async stage and store its result at self+4:
                    // non-zero steps the state by 2; zero with status 2 steps
                    // it by 1; zero with any other status leaves it.
                    c.cycles += 4;
                    c.g[A0][0] = c.g[SP][0];
                    call_ptr(c, callee::SLOGIN_POLL, 0x0026_5bb4)?;
                    let nz = c.g[V0][0] != 0;
                    Addr((c.g[S0][0] as u32).wrapping_add(0x4)).sw(c, c.g[V0][0]);
                    if nz {
                        c.cycles += 3;
                        c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2));
                        After::StoreExit
                    } else {
                        c.cycles += 4;
                        c.g[V1][0] = Addr(frame.at(c, 0)).lw(c);
                        c.g[V0][0] = 0x2;
                        if c.g[V1][0] != c.g[V0][0] {
                            frame.restore(c, S0, 0x20);
                            After::FullExit
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                            After::StoreExit
                        }
                    }
                }
                0x0026_5be4 => {
                    card_busy(c, 0x0026_5bfc)?;
                    let nz = c.g[V0][0] != 0;
                    frame.restore(c, S0, 0x20);
                    if nz {
                        After::FullExit
                    } else {
                        c.cycles += 9;
                        c.g[A2][0] = Addr((c.g[GP][0] as u32).wrapping_add(0x2e8)).lw(c);
                        c.g[A1][0] = 0;
                        c.g[V0][0] = Addr(c.g[A2][0] as u32).lw(c);
                        c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x28)).lh(c);
                        c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x2c)).lw(c);
                        let tgt2 = c.g[V1][0] as u32;
                        c.g[A0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, tgt2, 0x0026_5c20)?;
                        frame.restore(c, S0, 0x20);
                        After::FullExit
                    }
                }
                0x0026_5c28 => {
                    card_busy(c, 0x0026_5c40)?;
                    if c.g[V0][0] != 0 {
                        frame.restore(c, S0, 0x20);
                        After::FullExit
                    } else {
                        c.cycles += 8;
                        c.g[A1][0] = Addr((c.g[GP][0] as u32).wrapping_add(0x2e8)).lw(c);
                        c.g[V0][0] = Addr(c.g[A1][0] as u32).lw(c);
                        c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x40)).lh(c);
                        c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x44)).lw(c);
                        let tgt2 = c.g[V1][0] as u32;
                        c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                        call_ptr(c, tgt2, 0x0026_5c60)?;
                        c.g[V0][0] = 0x14;
                        After::StoreExit
                    }
                }
                0x0026_5c68 => {
                    card_busy(c, 0x0026_5c80)?;
                    if c.g[V0][0] != 0 {
                        frame.restore(c, S0, 0x20);
                        After::FullExit
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_add(0x850)).lw(c);
                        let nz = c.g[V0][0] != 0;
                        c.g[V0][0] = 0x16;
                        if !nz {
                            c.cycles += 1;
                            c.g[V0][0] = 0x15;
                        }
                        After::StoreExit
                    }
                }
                0x0026_5c9c => After::RestoreExit,
                _ => {
                    c.jump_out(t, 0x0026_5a68, 0x0026_5cac);
                    return Ok(());
                }
            };
        }
        if after == After::StoreExit {
            c.cycles += 1;
            Addr(c.g[S0][0] as u32).sw(c, c.g[V0][0]);
        }
        if after != After::FullExit {
            c.cycles += 1;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// Where a `cSetupMan_Update` stage goes next (the compiled code's shared
/// blocks; single-entry ones are inlined into their stage).
#[derive(Clone, Copy)]
enum UpdateStage {
    /// Receive and flush; states 5 and 10 run the handshake.
    Start,
    /// States 5 and 10: with the stats flag set, or in state 10, check the
    /// host flag; in state 5 with the stats flag clear (either role) go to
    /// state 7 and post a 0xf6 lobby message.
    Handshake,
    /// The host (flag set) goes to state 12 and sends packet 0xf.
    HostHandshake,
    /// `a0 = self`, then the pop.
    PopSelf,
    /// Pop one queued packet: 0xf completes the handshake (state 0xd; a
    /// client first replies from state 0xc), 0x10 copies the peer's info to
    /// the lobby manager and selects its character; other types by role.
    Pop,
    /// By the host flag and the state: the host before state 7 handles 1, 3
    /// and 5 here.
    ByRole,
    /// The host's type 3 (`cSetupMan_HostRecvConfirm`).
    HostConfirm,
    /// The host from state 7: types 7, 9, 11, 13, 0xf8 and (through
    /// `DropFa`) 0xfa.
    HostLate,
    /// A client before state 7: types 0-5.
    Client,
    /// Type 0xf8 (before state 7, either role).
    DropF8,
    /// Type 0xfa.
    DropFa,
    /// `v0 = 0xb` before the drop.
    DropState,
    /// State 0xb and the challenger drop (host or early copy).
    DropChallenger,
    /// A client from state 7: types 6, 7 and 10.
    ClientLate,
    /// A client from state 7: types 8 and 9.
    ClientLate8,
    /// A client from state 7: types 11-13, 0xf8 and 0xfa.
    ClientLate11,
    /// State 0xb and the challenger drop (the late client copy).
    ClientLateDrop,
}

/// How `cSetupMan_Update` leaves: which saved registers it still reloads.
#[derive(Clone, Copy, PartialEq)]
enum UpdateEnd {
    /// `s0`, `s1`, then the epilogue.
    RestoreS0,
    /// `s1` (s0 already reloaded), then the epilogue.
    RestoreS1,
    /// Both already reloaded.
    Epilogue,
}

/// `void cSetupMan_Update(cSetupMan* self)` (0x2642b8): the setup manager's
/// per-frame update: receive pending packets and flush the send queue, run
/// the handshake of states 5 and 10, then pop one queued packet and handle
/// it by type, role (host flag) and state (before 7 or not). The compiled
/// body is a tree of compare chains with no loops and no event checks; the
/// port walks its shared blocks as [`UpdateStage`]s.
pub fn c_setup_man_update(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x80);
        let mut stage = UpdateStage::Start;
        let end = loop {
            stage = match stage {
                UpdateStage::Start => {
                    c.cycles += 13;
                    frame.save(c, S1, 0x60);
                    frame.save(c, S0, 0x70);
                    frame.save(c, S2, 0x50);
                    frame.save_ra(c, 0x40);
                    c.g[S1][0] = c.g[A0][0];
                    callee::SETUP_MAN_RECEIVE_PACKETS.call(c, 0x0026_42d4)?;
                    c.g[A0][0] = c.g[S1][0];
                    callee::SETUP_MAN_FLUSH_SEND_QUEUE.call(c, 0x0026_42dc)?;
                    c.g[V1][0] = c.lw(c.g[S1][0] as u32);
                    c.g[V0][0] = 0x5;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = 0xa;
                    if t {
                        UpdateStage::Handshake
                    } else {
                        c.cycles += 2;
                        if c.g[V1][0] != c.g[V0][0] {
                            c.g[A0][0] = c.g[S1][0];
                            UpdateStage::Pop
                        } else {
                            UpdateStage::Handshake
                        }
                    }
                }
                UpdateStage::Handshake => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(SETUP_STATS_FLAG));
                    if c.g[V0][0] != 0 {
                        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(SETUP_HOST_FLAG));
                        UpdateStage::HostHandshake
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = 0xa;
                        let t = c.g[V1][0] != c.g[V0][0];
                        c.g[V0][0] = 0x7;
                        if t {
                            c.cycles += 26;
                            c.g[A1][0] = 0x0048_0000;
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x488));
                            c.g[S0][0] = c.lw(gp_8(c, LOBBY_INSTANCE_GP));
                            c.g[A0][0] = 0x8;
                            c.g[A2][0] = 0x2000_0000;
                            c.g[A3][0] = 0;
                            c.sw(c.g[S1][0] as u32, c.g[V0][0]);
                            call_ptr(c, callee::EA_MEM_ALLOC_NAMED.addr, 0x0026_4364)?;
                            c.g[S2][0] = c.g[V0][0];
                            c.g[V1][0] = 0xf6;
                            c.g[V0][0] = 0x0048_0000;
                            c.sw(c.g[S2][0] as u32, c.g[V1][0]);
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x12b0));
                            c.g[A1][0] = 0x0048_0000;
                            c.sw((c.g[S2][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4a8));
                            c.g[A0][0] = 0xc;
                            c.g[A2][0] = 0x2000_0000;
                            c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xf0));
                            c.g[A3][0] = 0;
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x30), c.g[V0][0]);
                            call_ptr(c, callee::EA_MEM_ALLOC_NAMED.addr, 0x0026_439c)?;
                            c.g[A1][0] = c.g[V0][0];
                            c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x8));
                            if c.g[V0][0] != 0 {
                                c.sw((c.g[A1][0] as u32).wrapping_add(0x8), c.g[S2][0]);
                            }
                            c.cycles += 9;
                            c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x30));
                            c.sw(c.g[A1][0] as u32, c.g[V0][0]);
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
                            c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[V1][0]);
                            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
                            c.sw(c.g[A0][0] as u32, c.g[A1][0]);
                            c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x30));
                            c.sw((c.g[V0][0] as u32).wrapping_add(0x4), c.g[A1][0]);
                            c.sw((c.g[SP][0] as u32).wrapping_add(0x20), c.g[A1][0]);
                            UpdateStage::PopSelf
                        } else {
                            c.cycles += 1;
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(SETUP_HOST_FLAG));
                            UpdateStage::HostHandshake
                        }
                    }
                }
                UpdateStage::HostHandshake => {
                    c.cycles += 2;
                    let t = c.g[V0][0] == 0;
                    c.g[V0][0] = 0xc;
                    if t {
                        UpdateStage::PopSelf
                    } else {
                        c.cycles += 11;
                        c.g[V1][0] = 0xf;
                        c.sw(c.g[S1][0] as u32, c.g[V0][0]);
                        c.g[A0][0] = c.g[S1][0];
                        c.sw(c.g[SP][0] as u32, c.g[V1][0]);
                        c.g[A1][0] = c.g[SP][0];
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x4), 0);
                        c.g[A2][0] = 0x14;
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x8), 0);
                        callee::CSETUP_MAN_SEND_PACKET.call(c, 0x0026_433c)?;
                        c.g[A0][0] = c.g[S1][0];
                        UpdateStage::Pop
                    }
                }
                UpdateStage::PopSelf => {
                    c.cycles += 1;
                    c.g[A0][0] = c.g[S1][0];
                    UpdateStage::Pop
                }
                UpdateStage::Pop => {
                    c.cycles += 5;
                    c.g[A1][0] = c.g[SP][0];
                    c.g[A2][0] = 0x14;
                    callee::SETUP_MAN_POP_PACKET.call(c, 0x0026_43e0)?;
                    let t = c.g[V0][0] == 0;
                    c.g[V1][0] = c.lw(c.g[SP][0] as u32);
                    if t {
                        break UpdateEnd::RestoreS0;
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = 0xf;
                        let t = c.g[V1][0] != c.g[V0][0];
                        c.g[V0][0] = 0x10;
                        if t {
                            c.cycles += 2;
                            if c.g[V1][0] != c.g[V0][0] {
                                c.g[V0][0] =
                                    c.lw((c.g[S1][0] as u32).wrapping_add(SETUP_HOST_FLAG));
                                UpdateStage::ByRole
                            } else {
                                c.cycles += 25;
                                c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                                c.g[A1][0] = 0x1;
                                c.g[V1][0] = c.lw(gp_8(c, LOBBY_INSTANCE_GP));
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x20), c.g[V0][0]);
                                c.ldl(2, (c.g[SP][0] as u32).wrapping_add(0x27));
                                c.ldr(2, (c.g[SP][0] as u32).wrapping_add(0x20));
                                c.ldl(4, (c.g[SP][0] as u32).wrapping_add(0x2f));
                                c.ldr(4, (c.g[SP][0] as u32).wrapping_add(0x28));
                                c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x30));
                                c.sdl(2, (c.g[V1][0] as u32).wrapping_add(0xd3));
                                c.sdr(2, (c.g[V1][0] as u32).wrapping_add(0xcc));
                                c.sdl(4, (c.g[V1][0] as u32).wrapping_add(0xdb));
                                c.sdr(4, (c.g[V1][0] as u32).wrapping_add(0xd4));
                                c.sw((c.g[V1][0] as u32).wrapping_add(0xdc), c.g[A2][0]);
                                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x78));
                                call_ptr(c, callee::BE_LIBRARY_GET_INTERFACE, 0x0026_447c)?;
                                c.g[A0][0] = c.g[V0][0];
                                c.g[A1][0] = 0;
                                call_ptr(
                                    c,
                                    callee::BE_NEW_PLAYER_INT_GET_CHARACTER_BY_SLOT,
                                    0x0026_4488,
                                )?;
                                c.g[A0][0] = c.g[S1][0];
                                c.g[A1][0] = c.g[V0][0];
                                callee::SETUP_MAN_SELECT_CHARACTER.call(c, 0x0026_4494)?;
                                frame.restore(c, S0, 0x70);
                                break UpdateEnd::RestoreS1;
                            }
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(SETUP_HOST_FLAG));
                            let t = c.g[V0][0] != 0;
                            c.g[V0][0] = 0xd;
                            if !t {
                                c.cycles += 10;
                                c.g[V0][0] = 0xc;
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x20), c.g[V1][0]);
                                c.sw(c.g[S1][0] as u32, c.g[V0][0]);
                                c.g[A0][0] = c.g[S1][0];
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x24), 0);
                                c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                                c.sw((c.g[SP][0] as u32).wrapping_add(0x28), 0);
                                c.g[A2][0] = 0x14;
                                callee::CSETUP_MAN_SEND_PACKET.call(c, 0x0026_4424)?;
                                c.g[V0][0] = 0xd;
                            }
                            c.cycles += 2;
                            c.sw(c.g[S1][0] as u32, c.g[V0][0]);
                            break UpdateEnd::RestoreS0;
                        }
                    }
                }
                UpdateStage::ByRole => {
                    c.cycles += 2;
                    let t = c.g[V0][0] == 0;
                    c.g[V0][0] = c.lw(c.g[S1][0] as u32);
                    if t {
                        UpdateStage::Client
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = ((c.g[V0][0] as i64) < 7i64) as u64;
                        let t = c.g[V0][0] == 0;
                        c.g[V0][0] = 0x5;
                        if t {
                            UpdateStage::HostLate
                        } else {
                            c.cycles += 2;
                            let t = c.g[V1][0] == c.g[V0][0];
                            c.g[V0][0] = (c.g[V1][0] < 0x6u64) as u64;
                            if t {
                                c.cycles += 4;
                                c.g[A0][0] = c.g[S1][0];
                                callee::SETUP_MAN_HOST_RECV_CANCEL.call(c, 0x0026_450c)?;
                                frame.restore(c, S0, 0x70);
                                break UpdateEnd::RestoreS1;
                            } else {
                                c.cycles += 2;
                                let t = c.g[V0][0] == 0;
                                c.g[V0][0] = 0x1;
                                if t {
                                    UpdateStage::DropF8
                                } else {
                                    c.cycles += 2;
                                    let t = c.g[V1][0] == c.g[V0][0];
                                    c.g[V0][0] = 0x3;
                                    if t {
                                        c.cycles += 6;
                                        c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                                        c.g[A0][0] = c.g[S1][0];
                                        c.g[A2][0] = c.g[SP][0] | 0xc;
                                        callee::SETUP_MAN_HOST_RECV_SELECT.call(c, 0x0026_44e8)?;
                                        frame.restore(c, S0, 0x70);
                                        break UpdateEnd::RestoreS1;
                                    } else {
                                        c.cycles += 2;
                                        let t = c.g[V1][0] == c.g[V0][0];
                                        frame.restore(c, S0, 0x70);
                                        if t {
                                            UpdateStage::HostConfirm
                                        } else {
                                            c.cycles += 2;
                                            frame.restore(c, S1, 0x60);
                                            break UpdateEnd::Epilogue;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                UpdateStage::HostConfirm => {
                    c.cycles += 5;
                    c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                    c.g[A0][0] = c.g[S1][0];
                    callee::SETUP_MAN_HOST_RECV_CONFIRM.call(c, 0x0026_44fc)?;
                    frame.restore(c, S0, 0x70);
                    break UpdateEnd::RestoreS1;
                }
                UpdateStage::HostLate => {
                    c.cycles += 3;
                    c.g[V0][0] = 0xb;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = (c.g[V1][0] < 0xcu64) as u64;
                    if t {
                        c.cycles += 4;
                        c.g[A0][0] = c.g[S1][0];
                        callee::SETUP_MAN_HOST_RECV_MSG11.call(c, 0x0026_4598)?;
                        frame.restore(c, S0, 0x70);
                        break UpdateEnd::RestoreS1;
                    } else {
                        c.cycles += 2;
                        let t = c.g[V0][0] == 0;
                        c.g[V0][0] = 0x7;
                        if t {
                            c.cycles += 3;
                            c.g[V0][0] = 0xf8;
                            let t = c.g[V1][0] == c.g[V0][0];
                            c.g[V0][0] = (c.g[V1][0] < 0xf9u64) as u64;
                            if t {
                                UpdateStage::DropState
                            } else {
                                c.cycles += 2;
                                let t = c.g[V0][0] == 0;
                                c.g[V0][0] = 0xd;
                                if t {
                                    UpdateStage::DropFa
                                } else {
                                    c.cycles += 2;
                                    let t = c.g[V1][0] == c.g[V0][0];
                                    frame.restore(c, S0, 0x70);
                                    if t {
                                        c.cycles += 4;
                                        c.g[A0][0] = c.g[S1][0];
                                        callee::SETUP_MAN_HOST_RECV_MSG13.call(c, 0x0026_45a8)?;
                                        frame.restore(c, S0, 0x70);
                                        break UpdateEnd::RestoreS1;
                                    } else {
                                        c.cycles += 2;
                                        frame.restore(c, S1, 0x60);
                                        break UpdateEnd::Epilogue;
                                    }
                                }
                            }
                        } else {
                            c.cycles += 2;
                            let t = c.g[V1][0] == c.g[V0][0];
                            c.g[V0][0] = 0x9;
                            if t {
                                c.cycles += 5;
                                c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                                c.g[A0][0] = c.g[S1][0];
                                callee::SETUP_MAN_HOST_RECV_MSG7.call(c, 0x0026_4570)?;
                                frame.restore(c, S0, 0x70);
                                break UpdateEnd::RestoreS1;
                            } else {
                                c.cycles += 2;
                                let t = c.g[V1][0] == c.g[V0][0];
                                frame.restore(c, S0, 0x70);
                                if t {
                                    c.cycles += 6;
                                    c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                                    c.g[A0][0] = c.g[S1][0];
                                    c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                                    callee::SETUP_MAN_HOST_RECV_MSG9.call(c, 0x0026_4588)?;
                                    frame.restore(c, S0, 0x70);
                                    break UpdateEnd::RestoreS1;
                                } else {
                                    c.cycles += 2;
                                    frame.restore(c, S1, 0x60);
                                    break UpdateEnd::Epilogue;
                                }
                            }
                        }
                    }
                }
                UpdateStage::Client => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[V0][0] as i64) < 7i64) as u64;
                    let t = c.g[V0][0] == 0;
                    c.g[V0][0] = 0x3;
                    if t {
                        UpdateStage::ClientLate
                    } else {
                        c.cycles += 2;
                        let t = c.g[V1][0] == c.g[V0][0];
                        c.g[V0][0] = (c.g[V1][0] < 0x4u64) as u64;
                        if t {
                            c.cycles += 6;
                            c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                            c.g[A0][0] = c.g[S1][0];
                            c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                            callee::SETUP_MAN_CLIENT_RECV_MSG3.call(c, 0x0026_4640)?;
                            frame.restore(c, S0, 0x70);
                            break UpdateEnd::RestoreS1;
                        } else {
                            c.cycles += 2;
                            let t = c.g[V0][0] == 0;
                            c.g[V0][0] = 0x1;
                            if t {
                                c.cycles += 3;
                                c.g[V0][0] = 0x5;
                                let t = c.g[V1][0] == c.g[V0][0];
                                c.g[V0][0] = (c.g[V1][0] < 0x5u64) as u64;
                                if t {
                                    c.cycles += 5;
                                    c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                                    c.g[A0][0] = c.g[S1][0];
                                    callee::SETUP_MAN_CLIENT_RECV_MSG5.call(c, 0x0026_4654)?;
                                    frame.restore(c, S0, 0x70);
                                    break UpdateEnd::RestoreS1;
                                } else {
                                    c.cycles += 2;
                                    let t = c.g[V0][0] != 0;
                                    c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                                    if t {
                                        c.cycles += 4;
                                        c.g[A0][0] = c.g[S1][0];
                                        callee::SETUP_MAN_CLIENT_RECV_MSG4.call(c, 0x0026_468c)?;
                                        frame.restore(c, S0, 0x70);
                                        break UpdateEnd::RestoreS1;
                                    } else {
                                        UpdateStage::DropF8
                                    }
                                }
                            } else {
                                c.cycles += 2;
                                let t = c.g[V1][0] == c.g[V0][0];
                                c.g[V0][0] = (c.g[V1][0] < 0x2u64) as u64;
                                if t {
                                    c.cycles += 6;
                                    c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                                    c.g[A0][0] = c.g[S1][0];
                                    c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                                    callee::SETUP_MAN_CLIENT_RECV_MSG1.call(c, 0x0026_4628)?;
                                    frame.restore(c, S0, 0x70);
                                    break UpdateEnd::RestoreS1;
                                } else {
                                    c.cycles += 2;
                                    let t = c.g[V0][0] == 0;
                                    c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                                    if t {
                                        c.cycles += 5;
                                        c.g[A0][0] = c.g[S1][0];
                                        c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                                        callee::SETUP_MAN_CLIENT_RECV_MSG2.call(c, 0x0026_467c)?;
                                        frame.restore(c, S0, 0x70);
                                        break UpdateEnd::RestoreS1;
                                    } else {
                                        c.cycles += 2;
                                        c.g[A0][0] = c.g[S1][0];
                                        c.cycles += 5;
                                        c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                                        c.g[A3][0] = c.g[SP][0] | 0xc;
                                        callee::SETUP_MAN_CLIENT_RECV_MSG0.call(c, 0x0026_4668)?;
                                        frame.restore(c, S0, 0x70);
                                        break UpdateEnd::RestoreS1;
                                    }
                                }
                            }
                        }
                    }
                }
                UpdateStage::DropF8 => {
                    c.cycles += 3;
                    c.g[V0][0] = 0xf8;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = 0xb;
                    if t {
                        UpdateStage::DropChallenger
                    } else {
                        UpdateStage::DropFa
                    }
                }
                UpdateStage::DropFa => {
                    c.cycles += 3;
                    c.g[V0][0] = 0xfa;
                    let t = c.g[V1][0] == c.g[V0][0];
                    frame.restore(c, S0, 0x70);
                    if t {
                        UpdateStage::DropState
                    } else {
                        c.cycles += 2;
                        frame.restore(c, S1, 0x60);
                        break UpdateEnd::Epilogue;
                    }
                }
                UpdateStage::DropState => {
                    c.cycles += 1;
                    c.g[V0][0] = 0xb;
                    UpdateStage::DropChallenger
                }
                UpdateStage::DropChallenger => {
                    c.cycles += 3;
                    setup_update_drop_challenger(
                        c,
                        [
                            0x0026_46ac,
                            0x0026_46b4,
                            0x0026_46c4,
                            0x0026_46d8,
                            0x0026_46e8,
                            0x0026_4700,
                            0x0026_4708,
                            0x0026_4718,
                            0x0026_472c,
                            0x0026_473c,
                            0x0026_4748,
                        ],
                        5,
                    )?;
                    frame.restore(c, S0, 0x70);
                    break UpdateEnd::RestoreS1;
                }
                UpdateStage::ClientLate => {
                    c.cycles += 3;
                    c.g[V0][0] = 0xa;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = (c.g[V1][0] < 0xbu64) as u64;
                    if t {
                        c.cycles += 5;
                        c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                        c.g[A0][0] = c.g[S1][0];
                        callee::SETUP_MAN_CLIENT_RECV_MSG10.call(c, 0x0026_484c)?;
                        frame.restore(c, S0, 0x70);
                        break UpdateEnd::RestoreS1;
                    } else {
                        c.cycles += 2;
                        let t = c.g[V0][0] == 0;
                        c.g[V0][0] = 0x7;
                        if t {
                            UpdateStage::ClientLate11
                        } else {
                            c.cycles += 2;
                            let t = c.g[V1][0] == c.g[V0][0];
                            c.g[V0][0] = (c.g[V1][0] < 0x8u64) as u64;
                            if t {
                                c.cycles += 5;
                                c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                                c.g[A0][0] = c.g[S1][0];
                                callee::SETUP_MAN_CLIENT_RECV_MSG7.call(c, 0x0026_4808)?;
                                frame.restore(c, S0, 0x70);
                                break UpdateEnd::RestoreS1;
                            } else {
                                c.cycles += 2;
                                let t = c.g[V0][0] == 0;
                                c.g[V0][0] = 0x6;
                                if t {
                                    UpdateStage::ClientLate8
                                } else {
                                    c.cycles += 2;
                                    let t = c.g[V1][0] == c.g[V0][0];
                                    frame.restore(c, S0, 0x70);
                                    if t {
                                        c.cycles += 5;
                                        c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                                        c.g[A0][0] = c.g[S1][0];
                                        callee::SETUP_MAN_CLIENT_RECV_MSG6.call(c, 0x0026_47f4)?;
                                        frame.restore(c, S0, 0x70);
                                        break UpdateEnd::RestoreS1;
                                    } else {
                                        c.cycles += 2;
                                        frame.restore(c, S1, 0x60);
                                        break UpdateEnd::Epilogue;
                                    }
                                }
                            }
                        }
                    }
                }
                UpdateStage::ClientLate8 => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x8;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = 0x9;
                    if t {
                        c.cycles += 6;
                        c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                        c.g[A0][0] = c.g[S1][0];
                        c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                        callee::SETUP_MAN_CLIENT_RECV_MSG8.call(c, 0x0026_4820)?;
                        frame.restore(c, S0, 0x70);
                        break UpdateEnd::RestoreS1;
                    } else {
                        c.cycles += 2;
                        let t = c.g[V1][0] == c.g[V0][0];
                        frame.restore(c, S0, 0x70);
                        if t {
                            c.cycles += 6;
                            c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                            c.g[A0][0] = c.g[S1][0];
                            c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
                            callee::SETUP_MAN_CLIENT_RECV_MSG9.call(c, 0x0026_4838)?;
                            frame.restore(c, S0, 0x70);
                            break UpdateEnd::RestoreS1;
                        } else {
                            c.cycles += 2;
                            frame.restore(c, S1, 0x60);
                            break UpdateEnd::Epilogue;
                        }
                    }
                }
                UpdateStage::ClientLate11 => {
                    c.cycles += 3;
                    c.g[V0][0] = 0xd;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = (c.g[V1][0] < 0xeu64) as u64;
                    if t {
                        c.cycles += 5;
                        c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                        c.g[A0][0] = c.g[S1][0];
                        callee::SETUP_MAN_CLIENT_RECV_MSG13.call(c, 0x0026_4888)?;
                        frame.restore(c, S0, 0x70);
                        break UpdateEnd::RestoreS1;
                    } else {
                        c.cycles += 2;
                        let t = c.g[V0][0] == 0;
                        c.g[V0][0] = 0xb;
                        if t {
                            c.cycles += 3;
                            c.g[V0][0] = 0xf8;
                            let t = c.g[V1][0] == c.g[V0][0];
                            c.g[V0][0] = 0xfa;
                            if t {
                                UpdateStage::ClientLateDrop
                            } else {
                                c.cycles += 2;
                                let t = c.g[V1][0] == c.g[V0][0];
                                frame.restore(c, S0, 0x70);
                                if t {
                                    UpdateStage::ClientLateDrop
                                } else {
                                    c.cycles += 2;
                                    frame.restore(c, S1, 0x60);
                                    break UpdateEnd::Epilogue;
                                }
                            }
                        } else {
                            c.cycles += 2;
                            let t = c.g[V1][0] == c.g[V0][0];
                            c.g[V0][0] = 0xc;
                            if t {
                                c.cycles += 5;
                                c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                                c.g[A0][0] = c.g[S1][0];
                                callee::SETUP_MAN_CLIENT_RECV_MSG11.call(c, 0x0026_4860)?;
                                frame.restore(c, S0, 0x70);
                                break UpdateEnd::RestoreS1;
                            } else {
                                c.cycles += 2;
                                let t = c.g[V1][0] == c.g[V0][0];
                                frame.restore(c, S0, 0x70);
                                if t {
                                    c.cycles += 5;
                                    c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                                    c.g[A0][0] = c.g[S1][0];
                                    callee::SETUP_MAN_CLIENT_RECV_MSG12.call(c, 0x0026_4874)?;
                                    frame.restore(c, S0, 0x70);
                                    break UpdateEnd::RestoreS1;
                                } else {
                                    c.cycles += 2;
                                    frame.restore(c, S1, 0x60);
                                    break UpdateEnd::Epilogue;
                                }
                            }
                        }
                    }
                }
                UpdateStage::ClientLateDrop => {
                    c.cycles += 4;
                    c.g[V0][0] = 0xb;
                    setup_update_drop_challenger(
                        c,
                        [
                            0x0026_48a8,
                            0x0026_48b0,
                            0x0026_48c0,
                            0x0026_48d4,
                            0x0026_48e4,
                            0x0026_48fc,
                            0x0026_4904,
                            0x0026_4914,
                            0x0026_4928,
                            0x0026_4938,
                            0x0026_4944,
                        ],
                        3,
                    )?;
                    break UpdateEnd::RestoreS0;
                }
            };
        };
        if end == UpdateEnd::RestoreS0 {
            c.cycles += 1;
            frame.restore(c, S0, 0x70);
        }
        if end != UpdateEnd::Epilogue {
            c.cycles += 1;
            frame.restore(c, S1, 0x60);
        }
        c.cycles += 4;
        frame.restore(c, S2, 0x50);
        frame.restore_ra(c, 0x40);
        frame.close(c);
        Ok(())
    })
}

/// The 0xF8/0xFA ("challenger gone") tail both roles share: make sure the
/// buddy manager exists and is connected (create it, reconnect, or else
/// disconnect the lobby), and if the challenger is still listed do the same
/// again and remove it, then clear the lobby's challenge lock for the
/// packet's sender. `rets` are the return addresses in call order;
/// `lock_cycles` is the charge before the last call (3 or 5 in the two
/// copies).
fn setup_update_drop_challenger(c: &mut Cpu, rets: [u32; 11], lock_cycles: u64) -> Flow {
    c.g[V1][0] = c.lw(gp_8(c, BUDDY_MAN_GP));
    let t = c.g[V1][0] != 0;
    c.sw(c.g[S1][0] as u32, c.g[V0][0]);
    if !t {
        c.cycles += 2;
        call_ptr(c, callee::BUDDY_MAN_CREATE.addr, rets[0])?;
    }
    c.cycles += 4;
    c.g[A0][0] = c.lw(gp_8(c, BUDDY_MAN_GP));
    call_ptr(c, callee::BUDDY_MAN_IS_CONNECTED.addr, rets[1])?;
    let t = c.g[V0][0] != 0;
    c.g[V0][0] = c.lw(gp_8(c, BUDDY_MAN_GP));
    if !t {
        c.cycles += 3;
        c.g[A0][0] = c.lw(gp_8(c, BUDDY_MAN_GP));
        call_ptr(c, callee::BUDDY_MAN_RECONNECT.addr, rets[2])?;
        c.g[V0][0] = c.lw(gp_8(c, BUDDY_MAN_GP));
    }
    c.cycles += 2;
    let t = c.g[V0][0] != 0;
    c.g[V0][0] = c.lw(gp_8(c, LOBBY_INSTANCE_GP));
    if !t {
        c.cycles += 3;
        c.g[A0][0] = c.lw(gp_8(c, LOBBY_INSTANCE_GP));
        call_ptr(c, callee::LOBBY_MAN_DISCONNECT.addr, rets[3])?;
        c.g[V0][0] = c.lw(gp_8(c, LOBBY_INSTANCE_GP));
    }
    c.cycles += 5;
    c.g[A0][0] = c.lw(gp_8(c, BUDDY_MAN_GP));
    c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x98));
    callee::BUDDY_MAN_HAS_CHALLENGER.call(c, rets[4])?;
    let t = c.g[V0][0] == 0;
    c.g[V0][0] = c.lw(gp_8(c, BUDDY_MAN_GP));
    if !t {
        c.cycles += 2;
        let t = c.g[V0][0] != 0;
        if !t {
            c.cycles += 2;
            call_ptr(c, callee::BUDDY_MAN_CREATE.addr, rets[5])?;
        }
        c.cycles += 4;
        c.g[A0][0] = c.lw(gp_8(c, BUDDY_MAN_GP));
        call_ptr(c, callee::BUDDY_MAN_IS_CONNECTED.addr, rets[6])?;
        let t = c.g[V0][0] != 0;
        c.g[V0][0] = c.lw(gp_8(c, BUDDY_MAN_GP));
        if !t {
            c.cycles += 3;
            c.g[A0][0] = c.lw(gp_8(c, BUDDY_MAN_GP));
            call_ptr(c, callee::BUDDY_MAN_RECONNECT.addr, rets[7])?;
            c.g[V0][0] = c.lw(gp_8(c, BUDDY_MAN_GP));
        }
        c.cycles += 2;
        let t = c.g[V0][0] != 0;
        c.g[V0][0] = c.lw(gp_8(c, LOBBY_INSTANCE_GP));
        if !t {
            c.cycles += 3;
            c.g[A0][0] = c.lw(gp_8(c, LOBBY_INSTANCE_GP));
            call_ptr(c, callee::LOBBY_MAN_DISCONNECT.addr, rets[8])?;
            c.g[V0][0] = c.lw(gp_8(c, LOBBY_INSTANCE_GP));
        }
        c.cycles += 3;
        c.g[A0][0] = c.lw(gp_8(c, BUDDY_MAN_GP));
        c.g[A1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x98));
        callee::BUDDY_MAN_REMOVE_CHALLENGER.call(c, rets[9])?;
    }
    c.cycles += lock_cycles;
    c.g[A0][0] = c.lw(gp_8(c, LOBBY_INSTANCE_GP));
    c.g[A1][0] = c.lw(c.g[SP][0] as u32);
    call_ptr(c, callee::LOBBY_MAN_CLEAR_CHALLENGE_LOCK, rets[10])?;
    Ok(())
}
// Field offsets of the cSetupMan object (one name per offset).
#[cfg(test)]
pub(crate) const SETUP_STATE: u32 = 0x0;

pub(crate) const SETUP_CUR_PACKET: u32 = 0x4;

pub(crate) const SETUP_RECV_TAIL: u32 = 0x8;

pub(crate) const SETUP_RECV_HEAD: u32 = 0xc;

pub(crate) const SETUP_RECV_SLOTS: u32 = 0x10;

pub(crate) const SETUP_SEND_TAIL: u32 = 0x90;

pub(crate) const SETUP_SEND_HEAD: u32 = 0x94;

pub(crate) const SETUP_SEND_SLOTS: u32 = 0x98;

pub(crate) const SETUP_CHANNEL: u32 = 0x120;

pub(crate) const SETUP_HOST_FLAG: u32 = 0x124;

pub(crate) const SETUP_STATS_FLAG: u32 = 0x128;

/// Offset of the message-list head inside the globals struct at `gp-0xc8`.
pub(crate) const SETUP_LIST_HEAD: u32 = 0xf0;

/// Link the list node in `a1` in front of the lobby message list whose head
/// the caller saved at `sp+head_off` (the insert the setup-manager senders
/// share).
pub(crate) fn setup_insert_list_tail(c: &mut Cpu, head_off: u32) {
    c.g[V0][0] = Addr(sp(c).wrapping_add(head_off)).lw(c);
    Addr(c.g[A1][0] as u32).sw(c, c.g[V0][0]);
    c.g[V1][0] = Addr(c.g[V0][0] as u32).off(4).lw(c);
    Addr(c.g[A1][0] as u32).off(4).sw(c, c.g[V1][0]);
    c.g[A0][0] = Addr(c.g[V0][0] as u32).off(4).lw(c);
    Addr(c.g[A0][0] as u32).sw(c, c.g[A1][0]);
    c.g[V0][0] = Addr(sp(c).wrapping_add(head_off)).lw(c);
    Addr(c.g[V0][0] as u32).off(4).sw(c, c.g[A1][0]);
}

/// `void cSetupMan_SendBragMsg8(cSetupMan* self, int slot, int value)`
/// (0x265568): unless the setup state is 12, post an 8-type brag message
/// (9 when the host flag at +0x124 is clear; a state in [5,7) first
/// advances to 7) through SendPacket, then queue a 0xfc net message
/// carrying slot/value on the global message list.
pub fn c_setup_man_send_brag_msg8(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x80);
        c.g[V0][0] = 0xc;
        frame.save(c, S1, 0x60);
        frame.save(c, S2, 0x50);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S0, 0x70);
        frame.save_ra(c, 0x40);
        c.g[V1][0] = Addr(c.g[A0][0] as u32).lw(c);
        let done = c.g[V1][0] == c.g[V0][0];
        c.g[S2][0] = c.g[A2][0];
        if !done {
            // Message type in v0 (8, or 9 when the host flag is clear).
            c.cycles += 3;
            let flag = Addr(c.g[A0][0] as u32).off(SETUP_HOST_FLAG).lw(c);
            c.g[V0][0] = ((c.g[V1][0] as i64) < 5) as u64;
            if flag == 0 {
                c.cycles += 1;
                c.g[V0][0] = 0x9;
            } else {
                c.cycles += 2;
                if c.g[V0][0] != 0 {
                    c.g[V0][0] = 0x8;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[V1][0] as i64) < 7) as u64;
                    let below7 = c.g[V0][0] != 0;
                    c.g[V0][0] = 0x7;
                    if below7 {
                        c.cycles += 1;
                        Addr(c.g[A0][0] as u32).sw(c, c.g[V0][0]);
                    }
                    c.cycles += 2;
                    c.g[V0][0] = 0x8;
                }
            }
            c.cycles += 34;
            c.g[A1][0] = c.g[SP][0];
            Addr(sp(c)).sw(c, c.g[V0][0]);
            Addr(sp(c).wrapping_add(4)).sw(c, c.g[S1][0]);
            c.g[A2][0] = 0x14;
            Addr(sp(c).wrapping_add(8)).sw(c, c.g[S2][0]);
            callee::CSETUP_MAN_SEND_PACKET.call(c, 0x0026_55d8)?;
            c.g[A1][0] = 0x0048_0000;
            c.g[A0][0] = 0x14;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xf18));
            c.g[A2][0] = 0x2000_0000;
            c.g[A3][0] = 0;
            call_ptr(c, callee::EA_MEM_ALLOC_NAMED.addr, 0x0026_55f0)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[V1][0] = 0x0048_0000;
            c.g[V0][0] = 0xfc;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1130));
            Addr(c.g[S0][0] as u32).sw(c, c.g[V0][0]);
            c.g[A1][0] = 0x0048_0000;
            Addr(c.g[S0][0] as u32).off(4).sw(c, c.g[V1][0]);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4a8));
            Addr(c.g[S0][0] as u32).off(0xc).sw(c, c.g[S1][0]);
            c.g[A0][0] = 0xc;
            Addr(c.g[S0][0] as u32).off(0x10).sw(c, c.g[S2][0]);
            c.g[A2][0] = 0x2000_0000;
            Addr(c.g[S0][0] as u32).off(8).sw(c, 0);
            c.g[A3][0] = 0;
            c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0xc8)).lw(c);
            c.g[V1][0] = Addr(c.g[V0][0] as u32).off(SETUP_LIST_HEAD).lw(c);
            Addr(sp(c).wrapping_add(0x30)).sw(c, c.g[V1][0]);
            call_ptr(c, callee::EA_MEM_ALLOC_NAMED.addr, 0x0026_5638)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(8));
            if c.g[V0][0] != 0 {
                Addr(c.g[A1][0] as u32).off(8).sw(c, c.g[S0][0]);
            }
            c.cycles += 9;
            setup_insert_list_tail(c, 0x30);
            Addr(sp(c).wrapping_add(0x20)).sw(c, c.g[A1][0]);
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x70);
        frame.restore(c, S1, 0x60);
        frame.restore(c, S2, 0x50);
        frame.restore_ra(c, 0x40);
        frame.close(c);
        Ok(())
    })
}

/// `void cSetupMan_SendBragMsg6(cSetupMan* self, int id)` (0x265458):
/// like SendBragMsg8, but the message type (6, or 7 when the host flag
/// is clear) is carried in v1, the packet payload is (type, id, current
/// state), and the queued net message is 0xfb with only the id payload.
pub fn c_setup_man_send_brag_msg6(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x70);
        c.g[V0][0] = 0xc;
        frame.save(c, S1, 0x50);
        frame.save(c, S0, 0x60);
        frame.save_ra(c, 0x40);
        c.g[V1][0] = Addr(c.g[A0][0] as u32).lw(c);
        let done = c.g[V1][0] == c.g[V0][0];
        c.g[S1][0] = c.g[A1][0];
        if !done {
            c.cycles += 3;
            let flag = Addr(c.g[A0][0] as u32).off(SETUP_HOST_FLAG).lw(c);
            c.g[V0][0] = ((c.g[V1][0] as i64) < 5) as u64;
            if flag == 0 {
                c.cycles += 1;
                c.g[V1][0] = 0x7;
            } else {
                c.cycles += 2;
                if c.g[V0][0] != 0 {
                    c.g[V1][0] = 0x6;
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[V1][0] as i64) < 7) as u64;
                    let below7 = c.g[V0][0] != 0;
                    c.g[V0][0] = 0x7;
                    if below7 {
                        c.cycles += 1;
                        Addr(c.g[A0][0] as u32).sw(c, c.g[V0][0]);
                    }
                    c.cycles += 2;
                    c.g[V1][0] = 0x6;
                }
            }
            c.cycles += 34;
            c.g[V0][0] = Addr(c.g[A0][0] as u32).lw(c);
            c.g[A1][0] = c.g[SP][0];
            Addr(sp(c)).sw(c, c.g[V1][0]);
            c.g[A2][0] = 0x14;
            Addr(sp(c).wrapping_add(8)).sw(c, c.g[V0][0]);
            Addr(sp(c).wrapping_add(4)).sw(c, c.g[S1][0]);
            callee::CSETUP_MAN_SEND_PACKET.call(c, 0x0026_54c4)?;
            c.g[A1][0] = 0x0048_0000;
            c.g[A0][0] = 0x10;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xf18));
            c.g[A2][0] = 0x2000_0000;
            c.g[A3][0] = 0;
            call_ptr(c, callee::EA_MEM_ALLOC_NAMED.addr, 0x0026_54dc)?;
            c.g[V1][0] = 0x0048_0000;
            c.g[S0][0] = c.g[V0][0];
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1150));
            c.g[V0][0] = 0xfb;
            Addr(c.g[S0][0] as u32).off(4).sw(c, c.g[V1][0]);
            c.g[A1][0] = 0x0048_0000;
            Addr(c.g[S0][0] as u32).sw(c, c.g[V0][0]);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4a8));
            Addr(c.g[S0][0] as u32).off(0xc).sw(c, c.g[S1][0]);
            c.g[A0][0] = 0xc;
            Addr(c.g[S0][0] as u32).off(8).sw(c, 0);
            c.g[A2][0] = 0x2000_0000;
            c.g[V1][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0xc8)).lw(c);
            c.g[A3][0] = 0;
            c.g[V0][0] = Addr(c.g[V1][0] as u32).off(SETUP_LIST_HEAD).lw(c);
            Addr(sp(c).wrapping_add(0x30)).sw(c, c.g[V0][0]);
            call_ptr(c, callee::EA_MEM_ALLOC_NAMED.addr, 0x0026_5520)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(8));
            if c.g[V0][0] != 0 {
                Addr(c.g[A1][0] as u32).off(8).sw(c, c.g[S0][0]);
            }
            c.cycles += 9;
            setup_insert_list_tail(c, 0x30);
            Addr(sp(c).wrapping_add(0x20)).sw(c, c.g[A1][0]);
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore_ra(c, 0x40);
        frame.close(c);
        Ok(())
    })
}

/// `void cSetupMan_HostRecvSelect(cSetupMan* self, int rider)`
/// (0x265190): advance state 1 -> 2 (state 2 and >= 3 pass through),
/// then set the remote character (the caller's `a2` passes through), send
/// character message 1 and queue a 0xf3 net message.
pub fn c_setup_man_host_recv_select(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x50);
        frame.save(c, S2, 0x30);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x40);
        frame.save_ra(c, 0x20);
        c.g[A0][0] = 0x2;
        c.g[V1][0] = Addr(c.g[S0][0] as u32).lw(c);
        let is2 = c.g[V1][0] == c.g[A0][0];
        c.g[S2][0] = c.g[A1][0];
        if !is2 {
            c.cycles += 3;
            let below3 = (c.g[V1][0] as i64) < 3;
            c.g[V0][0] = 0x1;
            if below3 {
                c.cycles += 2;
                if c.g[V1][0] == c.g[V0][0] {
                    Addr(c.g[S0][0] as u32).sw(c, c.g[A0][0]);
                }
            }
        }
        c.cycles += 33;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S2][0];
        callee::SETUP_MAN_SET_REMOTE_CHARACTER.call(c, 0x0026_51d8)?;
        c.g[S1][0] = 0xf3;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[S2][0];
        c.g[A3][0] = Addr(c.g[A0][0] as u32).lw(c);
        c.g[A1][0] = 0x1;
        callee::SETUP_MAN_SEND_CHARACTER_MSG.call(c, 0x0026_51f0)?;
        c.g[A1][0] = 0x0048_0000;
        c.g[S0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0xc8)).lw(c);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x488));
        c.g[A0][0] = 0x8;
        c.g[A2][0] = 0x2000_0000;
        c.g[A3][0] = 0;
        call_ptr(c, callee::EA_MEM_ALLOC_NAMED.addr, 0x0026_520c)?;
        c.g[S2][0] = c.g[V0][0];
        c.g[A1][0] = 0x0048_0000;
        c.g[V0][0] = 0x0048_0000;
        Addr(c.g[S2][0] as u32).sw(c, c.g[S1][0]);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x12b0));
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4a8));
        Addr(c.g[S2][0] as u32).off(4).sw(c, c.g[V0][0]);
        c.g[A0][0] = 0xc;
        c.g[A2][0] = 0x2000_0000;
        c.g[A3][0] = 0;
        c.g[V0][0] = Addr(c.g[S0][0] as u32).off(SETUP_LIST_HEAD).lw(c);
        Addr(sp(c).wrapping_add(0x10)).sw(c, c.g[V0][0]);
        call_ptr(c, callee::EA_MEM_ALLOC_NAMED.addr, 0x0026_5240)?;
        c.g[A1][0] = c.g[V0][0];
        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(8));
        if c.g[V0][0] != 0 {
            Addr(c.g[A1][0] as u32).off(8).sw(c, c.g[S2][0]);
        }
        c.cycles += 15;
        setup_insert_list_tail(c, 0x10);
        Addr(sp(c)).sw(c, c.g[A1][0]);
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `void cSetupMan_HostRecvConfirm(cSetupMan* self, int rider)`
/// (0x265290): advance state 1/2 -> 3 and 4 -> 5, then set the remote
/// character (with a zero third argument), send character message 3
/// and queue a 0xf5 net message.
pub fn c_setup_man_host_recv_confirm(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x50);
        frame.save(c, S2, 0x30);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x40);
        frame.save_ra(c, 0x20);
        c.g[V1][0] = Addr(c.g[S0][0] as u32).lw(c);
        let positive = (c.g[V1][0] as i64) > 0;
        c.g[S2][0] = c.g[A1][0];
        let mut skip_common = false;
        if positive {
            c.cycles += 3;
            c.g[V0][0] = ((c.g[V1][0] as i64) < 3) as u64;
            if c.g[V0][0] != 0 {
                c.g[V0][0] = 0x3;
                c.cycles += 1;
                Addr(c.g[S0][0] as u32).sw(c, c.g[V0][0]);
            } else {
                c.cycles += 3;
                c.g[V0][0] = 0x4;
                c.g[A0][0] = c.g[S0][0];
                if c.g[V1][0] == c.g[V0][0] {
                    c.cycles += 1;
                    c.g[V0][0] = 0x5;
                    c.cycles += 1;
                    Addr(c.g[S0][0] as u32).sw(c, c.g[V0][0]);
                } else {
                    skip_common = true;
                }
            }
        }
        if !skip_common {
            c.cycles += 1;
            c.g[A0][0] = c.g[S0][0];
        }
        c.cycles += 33;
        c.g[A1][0] = c.g[S2][0];
        c.g[A2][0] = 0;
        c.g[S1][0] = 0xf5;
        callee::SETUP_MAN_SET_REMOTE_CHARACTER.call(c, 0x0026_52e8)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[S2][0];
        c.g[A3][0] = Addr(c.g[A0][0] as u32).lw(c);
        c.g[A1][0] = 0x3;
        callee::SETUP_MAN_SEND_CHARACTER_MSG.call(c, 0x0026_52fc)?;
        c.g[A1][0] = 0x0048_0000;
        c.g[S0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0xc8)).lw(c);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x488));
        c.g[A0][0] = 0x8;
        c.g[A2][0] = 0x2000_0000;
        c.g[A3][0] = 0;
        call_ptr(c, callee::EA_MEM_ALLOC_NAMED.addr, 0x0026_5318)?;
        c.g[S2][0] = c.g[V0][0];
        c.g[A1][0] = 0x0048_0000;
        c.g[V0][0] = 0x0048_0000;
        Addr(c.g[S2][0] as u32).sw(c, c.g[S1][0]);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x12b0));
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4a8));
        Addr(c.g[S2][0] as u32).off(4).sw(c, c.g[V0][0]);
        c.g[A0][0] = 0xc;
        c.g[A2][0] = 0x2000_0000;
        c.g[A3][0] = 0;
        c.g[V0][0] = Addr(c.g[S0][0] as u32).off(SETUP_LIST_HEAD).lw(c);
        Addr(sp(c).wrapping_add(0x10)).sw(c, c.g[V0][0]);
        call_ptr(c, callee::EA_MEM_ALLOC_NAMED.addr, 0x0026_534c)?;
        c.g[A1][0] = c.g[V0][0];
        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(8));
        if c.g[V0][0] != 0 {
            Addr(c.g[A1][0] as u32).off(8).sw(c, c.g[S2][0]);
        }
        c.cycles += 15;
        setup_insert_list_tail(c, 0x10);
        Addr(sp(c)).sw(c, c.g[A1][0]);
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `void cSetupMan_ConfirmCharacter(cSetupMan* self, int rider)`
/// (0x265018): with the host flag set, advance state 1/2 -> 4 and
/// 3 -> 5, set the local character, send character message 2 and
/// queue a 0xf4 net message; with the flag clear, only send
/// character message 3.
pub fn c_setup_man_confirm_character(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x50);
        frame.save(c, S2, 0x30);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x40);
        frame.save_ra(c, 0x20);
        c.g[V0][0] = Addr(c.g[S0][0] as u32).off(SETUP_HOST_FLAG).lw(c);
        let host = c.g[V0][0] != 0;
        c.g[S2][0] = c.g[A1][0];
        if host {
            c.cycles += 3;
            c.g[V1][0] = Addr(c.g[S0][0] as u32).lw(c);
            let positive = (c.g[V1][0] as i64) > 0;
            c.g[V0][0] = ((c.g[V1][0] as i64) < 3) as u64;
            let mut skip_common = false;
            if positive {
                c.cycles += 2;
                if c.g[V0][0] != 0 {
                    c.g[V0][0] = 0x4;
                    c.cycles += 1;
                    Addr(c.g[S0][0] as u32).sw(c, c.g[V0][0]);
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = 0x3;
                    c.g[A0][0] = c.g[S0][0];
                    if c.g[V1][0] == c.g[V0][0] {
                        c.cycles += 1;
                        c.g[V0][0] = 0x5;
                        c.cycles += 1;
                        Addr(c.g[S0][0] as u32).sw(c, c.g[V0][0]);
                    } else {
                        skip_common = true;
                    }
                }
            }
            if !skip_common {
                c.cycles += 1;
                c.g[A0][0] = c.g[S0][0];
            }
            c.cycles += 32;
            c.g[A1][0] = c.g[S2][0];
            callee::SETUP_MAN_SET_LOCAL_CHARACTER.call(c, 0x0026_5070)?;
            c.g[S1][0] = 0xf4;
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = c.g[S2][0];
            c.g[A3][0] = Addr(c.g[A0][0] as u32).lw(c);
            c.g[A1][0] = 0x2;
            callee::SETUP_MAN_SEND_CHARACTER_MSG.call(c, 0x0026_5088)?;
            c.g[A1][0] = 0x0048_0000;
            c.g[S0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0xc8)).lw(c);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x488));
            c.g[A0][0] = 0x8;
            c.g[A2][0] = 0x2000_0000;
            c.g[A3][0] = 0;
            call_ptr(c, callee::EA_MEM_ALLOC_NAMED.addr, 0x0026_50a4)?;
            c.g[S2][0] = c.g[V0][0];
            c.g[A1][0] = 0x0048_0000;
            c.g[V0][0] = 0x0048_0000;
            Addr(c.g[S2][0] as u32).sw(c, c.g[S1][0]);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x12b0));
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4a8));
            Addr(c.g[S2][0] as u32).off(4).sw(c, c.g[V0][0]);
            c.g[A0][0] = 0xc;
            c.g[A2][0] = 0x2000_0000;
            c.g[A3][0] = 0;
            c.g[V0][0] = Addr(c.g[S0][0] as u32).off(SETUP_LIST_HEAD).lw(c);
            Addr(sp(c).wrapping_add(0x10)).sw(c, c.g[V0][0]);
            call_ptr(c, callee::EA_MEM_ALLOC_NAMED.addr, 0x0026_50d8)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(8));
            if c.g[V0][0] != 0 {
                Addr(c.g[A1][0] as u32).off(8).sw(c, c.g[S2][0]);
            }
            c.cycles += 10;
            setup_insert_list_tail(c, 0x10);
            Addr(sp(c)).sw(c, c.g[A1][0]);
        } else {
            c.cycles += 5;
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = c.g[S2][0];
            c.g[A3][0] = Addr(c.g[A0][0] as u32).lw(c);
            c.g[A1][0] = 0x3;
            callee::SETUP_MAN_SEND_CHARACTER_MSG.call(c, 0x0026_5124)?;
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

/// `void cSetupMan_SelectCharacter(cSetupMan* self, int rider)`
/// (0x264f88): unless the state is 12, with the host flag set advance
/// state 1 -> 2, set the local character and send character message
/// 0; with the flag clear send character message 1 with a zero state.
pub fn c_setup_man_select_character(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = 0xc;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V1][0] = Addr(c.g[S0][0] as u32).lw(c);
        let done = c.g[V1][0] == c.g[V0][0];
        c.g[S1][0] = c.g[A1][0];
        if !done {
            c.cycles += 3;
            let host = Addr(c.g[S0][0] as u32).off(SETUP_HOST_FLAG).lw(c) != 0;
            c.g[V0][0] = 0x1;
            if !host {
                c.cycles += 5;
                c.g[A0][0] = c.g[S0][0];
                c.g[A2][0] = c.g[S1][0];
                c.g[A1][0] = 0x1;
                c.g[A3][0] = 0;
                callee::SETUP_MAN_SEND_CHARACTER_MSG.call(c, 0x0026_5000)?;
            } else {
                c.cycles += 2;
                c.g[A0][0] = c.g[S0][0];
                if c.g[V1][0] == c.g[V0][0] {
                    c.cycles += 2;
                    c.g[V0][0] = 0x2;
                    Addr(c.g[S0][0] as u32).sw(c, c.g[V0][0]);
                }
                c.cycles += 9;
                c.g[A1][0] = c.g[S1][0];
                callee::SETUP_MAN_SET_LOCAL_CHARACTER.call(c, 0x0026_4fd0)?;
                c.g[A0][0] = c.g[S0][0];
                c.g[A2][0] = c.g[S1][0];
                c.g[A3][0] = Addr(c.g[A0][0] as u32).lw(c);
                c.g[A1][0] = 0;
                callee::SETUP_MAN_SEND_CHARACTER_MSG.call(c, 0x0026_4fe4)?;
                frame.restore(c, S0, 0x20);
                c.cycles += 4;
                frame.restore(c, S1, 0x10);
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
        }
        c.cycles += 1;
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cSetupMan_SendCharacterMsg(cSetupMan* self, int event, int rider,
/// int state)` (0x264e50): build the 0x14-byte character message
/// (event, rider, state, then seven stat bytes: all 40 when the stats
/// flag at +0x128 is set, else the seven BE character attribute
/// getters) and post it through SendPacket.
pub fn c_setup_man_send_character_msg(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S1, 0x40);
        frame.save(c, S2, 0x30);
        c.g[S1][0] = c.g[A2][0];
        frame.save(c, S0, 0x50);
        c.g[S2][0] = c.g[A0][0];
        Addr(sp(c)).sw(c, c.g[A1][0]);
        frame.save_ra(c, 0x20);
        Addr(sp(c).wrapping_add(8)).sw(c, c.g[A3][0]);
        Addr(sp(c).wrapping_add(4)).sw(c, c.g[S1][0]);
        call_ptr(c, callee::BE_LIBRARY_GET_INSTANCE, 0x0026_4e7c)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0x3;
        call_ptr(c, callee::BE_LIBRARY_GET_INTERFACE, 0x0026_4e88)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A1][0] = 0x7;
        c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
        c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x78).lw(c);
        call_ptr(c, callee::BE_LIBRARY_GET_INTERFACE, 0x0026_4e9c)?;
        c.g[V1][0] = Addr(c.g[S2][0] as u32).off(SETUP_STATS_FLAG).lw(c);
        if c.g[V1][0] != 0 {
            c.cycles += 2;
            c.g[V1][0] = 0x6;
            c.g[A0][0] = 0x28;
            c.g[V0][0] = sx(sp(c).wrapping_add(0x12));
            loop {
                c.cycles += 7;
                Addr(c.g[V0][0] as u32).sb(c, c.g[A0][0]);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(1));
                let more = (c.g[V1][0] as i64) >= 0;
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(1));
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x0026_4eb0);
                    }
                    continue;
                }
                c.cycles += 2;
                c.g[A0][0] = c.g[S2][0];
                break;
            }
        } else {
            c.cycles += 36;
            // Seven attribute getters, one stat byte each at sp+0xc..0x12.
            const GETTERS: [(u32, u32, u32); 7] = [
                (0x0014_8bc8, 0x0026_4ee4, 0xc),
                (0x0014_8c68, 0x0026_4ef8, 0xd),
                (0x0014_8b78, 0x0026_4f0c, 0xe),
                (0x0014_8cb8, 0x0026_4f20, 0xf),
                (0x0014_8d58, 0x0026_4f34, 0x10),
                (0x0014_8d08, 0x0026_4f48, 0x11),
                (0x0014_8c18, 0x0026_4f5c, 0x12),
            ];
            for (addr, ret_addr, off) in GETTERS {
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = 0;
                c.g[A2][0] = c.g[S1][0];
                call_ptr(c, addr, ret_addr)?;
                Addr(sp(c).wrapping_add(off)).sb(c, c.g[V0][0]);
            }
            c.g[A0][0] = c.g[S2][0];
        }
        c.cycles += 9;
        c.g[A1][0] = c.g[SP][0];
        c.g[A2][0] = 0x14;
        callee::CSETUP_MAN_SEND_PACKET.call(c, 0x0026_4f70)?;
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `int cSetupMan_SendPacket(cSetupMan* self, void* msg, int size)`
/// (0x264be8): flush the send queue, wrap the message in a packet
/// header (kind/flags zero, size+4), try the channel send; on failure
/// copy the packet into a freshly allocated ring slot. Always
/// returns 1 in v0.
pub fn c_setup_man_send_packet(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x130);
        frame.save(c, S2, 0x100);
        frame.save(c, S0, 0x120);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x110);
        c.g[S0][0] = c.g[A2][0];
        frame.save_ra(c, 0xf0);
        c.g[S1][0] = c.g[A1][0];
        callee::SETUP_MAN_FLUSH_SEND_QUEUE.call(c, 0x0026_4c0c)?;
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(4));
        c.g[A1][0] = c.g[S1][0];
        c.g[A2][0] = c.g[S0][0];
        c.g[A0][0] = c.g[SP][0] | 0x4;
        Addr(sp(c).wrapping_add(2)).sh(c, c.g[V0][0]);
        Addr(sp(c)).sb(c, 0);
        Addr(sp(c).wrapping_add(1)).sb(c, 0);
        call_ptr(c, callee::EA_MEM_COPY, 0x0026_4c2c)?;
        c.g[A3][0] = Addr(sp(c).wrapping_add(2)).lhu(c);
        c.g[A2][0] = c.g[SP][0];
        c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x238)).lw(c);
        c.g[A1][0] = Addr(c.g[S2][0] as u32).off(SETUP_CHANNEL).lw(c);
        call_ptr(c, callee::C_COMM_SYSTEM_CHANNEL_SEND.addr, 0x0026_4c40)?;
        if c.g[V0][0] != 0 {
            c.g[V0][0] = 0x1;
        } else {
            c.cycles += 22;
            c.g[A0][0] = c.g[S2][0];
            callee::SETUP_MAN_ALLOC_PACKET.call(c, 0x0026_4c50)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[A2][0] = Addr(sp(c).wrapping_add(2)).lhu(c);
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[SP][0];
            call_ptr(c, callee::EA_MEM_COPY, 0x0026_4c64)?;
            c.g[A2][0] = Addr(c.g[S2][0] as u32).off(SETUP_SEND_TAIL).lw(c);
            c.g[A0][0] = 0xffff_ffff_ffff_ffff;
            c.g[V0][0] = 0x1;
            c.g[A1][0] = c.g[A2][0];
            c.g[A0][0] = ((c.g[A0][0] as i64) < (c.g[A1][0] as i64)) as u64;
            c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1f));
            if c.g[A0][0] != 0 {
                c.g[V1][0] = c.g[A1][0];
            }
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
            c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 5) as u32);
            Addr(c.g[S2][0] as u32)
                .off(SETUP_SEND_TAIL)
                .sw(c, c.g[A2][0]);
            c.g[A0][0] = sx((c.g[V1][0] as u32) << 5);
            c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_sub(c.g[A0][0] as u32));
            c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
            c.g[V1][0] = sx((c.g[S2][0] as u32).wrapping_add(c.g[V1][0] as u32));
            Addr(c.g[V1][0] as u32)
                .off(SETUP_SEND_SLOTS)
                .sw(c, c.g[S0][0]);
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x120);
        frame.restore(c, S1, 0x110);
        frame.restore(c, S2, 0x100);
        frame.restore_ra(c, 0xf0);
        frame.close(c);
        Ok(())
    })
}

/// Stages of `cSetupMan_ReceivePackets` between its resume points
/// (the generated loop head at 0x264a80 is the only resume entry).
#[derive(Clone, Copy, PartialEq)]
enum RecvStage {
    /// Prologue, then the current-packet check.
    Entry,
    /// The current-packet check (0x264ac8).
    Check,
    /// Allocate and receive a packet (0x264ad4).
    Alloc,
    /// The ring store at the loop head (0x264a80).
    Head,
    /// Epilogue (0x264b08).
    Done,
}

/// `void cSetupMan_ReceivePackets(cSetupMan* self)` (0x264a58): drain
/// the receive channel into the 32-slot ring at +0x10 (indices at
/// +0x8/+0xc, current packet at +0x4) until the channel is empty or
/// the ring holds 31 packets; a packet that arrives when the ring is
/// checked is deleted unused. Dispatcher-shaped: re-enters at the
/// loop head 0x264a80 after an event unwind. The generated state
/// 0x264a7c is never targeted (entry jumps straight to the check).
pub fn c_setup_man_receive_packets(c: &mut Cpu) {
    let frame = Frame { size: 0x40 };
    let mut stage = if c.pc == 0x0026_4a80 {
        RecvStage::Head
    } else if c.pc == 0x0026_4a58 {
        RecvStage::Entry
    } else {
        c.jump_out(c.pc, 0x0026_4a58, 0x0026_4b20);
        return;
    };
    loop {
        stage = match stage {
            RecvStage::Entry => {
                c.cycles += 9;
                let frame = Frame::open(c, 0x40);
                frame.save(c, S0, 0x30);
                frame.save(c, S1, 0x20);
                c.g[S0][0] = c.g[A0][0];
                frame.save(c, S2, 0x10);
                c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(SETUP_RECV_SLOTS));
                frame.save_ra(c, 0);
                c.g[S2][0] = 0xffff_ffff_ffff_ffff;
                RecvStage::Check
            }
            RecvStage::Check => {
                c.cycles += 3;
                c.g[V0][0] = Addr(c.g[S0][0] as u32).off(SETUP_CUR_PACKET).lw(c);
                if c.g[V0][0] != 0 {
                    c.g[A1][0] = Addr(c.g[S0][0] as u32).off(SETUP_RECV_TAIL).lw(c);
                    if back_edge_unwinds(c, 0x0026_4a80) {
                        return;
                    }
                    RecvStage::Head
                } else {
                    RecvStage::Alloc
                }
            }
            RecvStage::Alloc => {
                c.cycles += 10;
                c.g[A0][0] = c.g[S0][0];
                if callee::SETUP_MAN_ALLOC_PACKET.call(c, 0x0026_4adc).is_err() {
                    return;
                }
                c.g[A0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x238)).lw(c);
                c.g[A2][0] = c.g[V0][0];
                c.g[A1][0] = Addr(c.g[S0][0] as u32).off(SETUP_CHANNEL).lw(c);
                c.g[A3][0] = 0xf0;
                Addr(c.g[S0][0] as u32)
                    .off(SETUP_CUR_PACKET)
                    .sw(c, c.g[V0][0]);
                if call_ptr(c, callee::C_COMM_SYSTEM_CHANNEL_RECV, 0x0026_4af4).is_err() {
                    return;
                }
                if c.g[V0][0] != 0 {
                    c.g[A1][0] = Addr(c.g[S0][0] as u32).off(SETUP_RECV_TAIL).lw(c);
                    if back_edge_unwinds(c, 0x0026_4a80) {
                        return;
                    }
                    RecvStage::Head
                } else {
                    c.cycles += 3;
                    c.g[A0][0] = Addr(c.g[S0][0] as u32).off(SETUP_CUR_PACKET).lw(c);
                    if call_ptr(c, callee::BUILTIN_DELETE, 0x0026_4b04).is_err() {
                        return;
                    }
                    Addr(c.g[S0][0] as u32).off(SETUP_CUR_PACKET).sw(c, 0);
                    RecvStage::Done
                }
            }
            RecvStage::Head => {
                c.cycles += 6;
                c.g[V0][0] = Addr(c.g[S0][0] as u32).off(SETUP_RECV_HEAD).lw(c);
                c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                c.g[V0][0] = ((c.g[V0][0] as i64) < 31) as u64;
                c.g[V0][0] ^= 0x1;
                let full = c.g[V0][0] != 0;
                c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[A1][0] as i64)) as u64;
                if full {
                    RecvStage::Done
                } else {
                    c.cycles += 12;
                    c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x1f));
                    if c.g[V0][0] != 0 {
                        c.g[V1][0] = c.g[A1][0];
                    }
                    c.g[A0][0] = Addr(c.g[S0][0] as u32).off(SETUP_CUR_PACKET).lw(c);
                    c.g[V1][0] = sx(((c.g[V1][0] as u32) as i32 >> 5) as u32);
                    c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 5);
                    Addr(c.g[S0][0] as u32)
                        .off(SETUP_RECV_TAIL)
                        .sw(c, c.g[V0][0]);
                    c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_sub(c.g[V1][0] as u32));
                    c.g[V0][0] = sx((c.g[V1][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    Addr(c.g[V0][0] as u32).sw(c, c.g[A0][0]);
                    Addr(c.g[S0][0] as u32).off(SETUP_CUR_PACKET).sw(c, 0);
                    RecvStage::Check
                }
            }
            RecvStage::Done => {
                c.cycles += 6;
                frame.restore(c, S0, 0x30);
                frame.restore(c, S1, 0x20);
                frame.restore(c, S2, 0x10);
                frame.restore_ra(c, 0);
                frame.close(c);
                return;
            }
        };
    }
}

/// `void cSetupMan_dtor(cSetupMan* self, int flags)` (0x264138):
/// free the current packet and every packet in both 32-slot rings
/// (receive ring at +0x10 with indices +0xc/+0x8, send ring at +0x98
/// with indices +0x94/+0x90), mark the object destroyed (+0 = 1) and,
/// when flags bit 0 is set, delete the object itself.
/// Dispatcher-shaped: re-enters at 0x264178 / 0x264208 after an event
/// unwind. The generated states 0x264174, 0x2641f0-via-loop and the
/// 0x264208 fall-through from 0x2641fc are unreachable (the generator
/// emits them for unconditional branches).
pub fn c_setup_man_dtor(c: &mut Cpu) {
    let mut pc: u32 = c.pc;
    loop {
        match pc {
            0x0026_4138 => {
                c.cycles += 11;
                let frame = Frame::open(c, 0x50);
                frame.save(c, S0, 0x40);
                frame.save(c, S1, 0x30);
                c.g[S0][0] = c.g[A0][0];
                frame.save(c, S2, 0x20);
                c.g[S1][0] = c.g[A1][0];
                frame.save(c, S3, 0x10);
                frame.save_ra(c, 0);
                c.g[A1][0] = Addr(c.g[S0][0] as u32).off(SETUP_CUR_PACKET).lw(c);
                let empty = c.g[A1][0] == 0;
                c.g[S3][0] = c.g[S1][0] & 0x1;
                pc = if empty { 0x0026_41b4 } else { 0x0026_4164 };
            }
            0x0026_4164 => {
                c.cycles += 4;
                if callee::C_SETUP_MAN_FREE_PACKET
                    .call(c, 0x0026_416c)
                    .is_err()
                {
                    return;
                }
                c.g[A0][0] = Addr(c.g[S0][0] as u32).off(SETUP_RECV_HEAD).lw(c);
                pc = 0x0026_41b8;
            }
            0x0026_4178 => {
                c.cycles += 15;
                c.g[A1][0] = 0xffff_ffff_ffff_ffff;
                c.g[A0][0] = c.g[S0][0];
                c.g[V1][0] = c.g[A2][0];
                c.g[A1][0] = ((c.g[A1][0] as i64) < (c.g[V1][0] as i64)) as u64;
                c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1f));
                if c.g[A1][0] != 0 {
                    c.g[V0][0] = c.g[V1][0];
                }
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
                c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 5) as u32);
                Addr(c.g[S0][0] as u32)
                    .off(SETUP_RECV_HEAD)
                    .sw(c, c.g[A2][0]);
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 5);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.g[A1][0] = Addr(c.g[V1][0] as u32).off(SETUP_RECV_SLOTS).lw(c);
                if callee::C_SETUP_MAN_FREE_PACKET
                    .call(c, 0x0026_41b4)
                    .is_err()
                {
                    return;
                }
                pc = 0x0026_41b4;
            }
            0x0026_41b4 => {
                c.cycles += 1;
                c.g[A0][0] = Addr(c.g[S0][0] as u32).off(SETUP_RECV_HEAD).lw(c);
                pc = 0x0026_41b8;
            }
            0x0026_41b8 => {
                c.cycles += 4;
                c.g[V0][0] = Addr(c.g[S0][0] as u32).off(SETUP_RECV_TAIL).lw(c);
                c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[V1][0] = 0xffff_ffff_ffff_ffff;
                pc = if more { 0x0026_41c8 } else { 0x0026_41f0 };
            }
            0x0026_41c8 => {
                c.cycles += 10;
                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1f));
                c.g[V1][0] = ((c.g[V1][0] as i64) < (c.g[A0][0] as i64)) as u64;
                if c.g[V1][0] != 0 {
                    c.g[V0][0] = c.g[A0][0];
                }
                c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 5) as u32);
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 5);
                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[V0][0] = Addr(c.g[V0][0] as u32).off(SETUP_RECV_SLOTS).lw(c);
                pc = 0x0026_41f4;
            }
            0x0026_41f0 => {
                c.cycles += 1;
                c.g[V0][0] = 0;
                pc = 0x0026_41f4;
            }
            0x0026_41f4 => {
                c.cycles += 2;
                if c.g[V0][0] != 0 {
                    c.g[A2][0] = Addr(c.g[S0][0] as u32).off(SETUP_RECV_HEAD).lw(c);
                    if back_edge_unwinds(c, 0x0026_4178) {
                        return;
                    }
                    pc = 0x0026_4178;
                } else {
                    pc = 0x0026_41fc;
                }
            }
            0x0026_41fc => {
                c.cycles += 3;
                c.g[S2][0] = 0xffff_ffff_ffff_ffff;
                c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(SETUP_SEND_SLOTS));
                pc = 0x0026_4240;
            }
            0x0026_4208 => {
                c.cycles += 14;
                c.g[A0][0] = c.g[S0][0];
                c.g[V1][0] = c.g[A2][0];
                c.g[A1][0] = ((c.g[S2][0] as i64) < (c.g[V1][0] as i64)) as u64;
                c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1f));
                if c.g[A1][0] != 0 {
                    c.g[V0][0] = c.g[V1][0];
                }
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
                c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 5) as u32);
                Addr(c.g[S0][0] as u32)
                    .off(SETUP_SEND_HEAD)
                    .sw(c, c.g[A2][0]);
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 5);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.g[A1][0] = Addr(c.g[V1][0] as u32).lw(c);
                if callee::C_SETUP_MAN_FREE_PACKET
                    .call(c, 0x0026_4240)
                    .is_err()
                {
                    return;
                }
                pc = 0x0026_4240;
            }
            0x0026_4240 => {
                c.cycles += 5;
                c.g[A0][0] = Addr(c.g[S0][0] as u32).off(SETUP_SEND_HEAD).lw(c);
                c.g[V0][0] = Addr(c.g[S0][0] as u32).off(SETUP_SEND_TAIL).lw(c);
                c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1f));
                pc = if more { 0x0026_4254 } else { 0x0026_4278 };
            }
            0x0026_4254 => {
                c.cycles += 9;
                c.g[V1][0] = ((c.g[S2][0] as i64) < (c.g[A0][0] as i64)) as u64;
                if c.g[V1][0] != 0 {
                    c.g[V0][0] = c.g[A0][0];
                }
                c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 5) as u32);
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 5);
                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[V0][0] = Addr(c.g[V0][0] as u32).lw(c);
                pc = 0x0026_427c;
            }
            0x0026_4278 => {
                c.cycles += 1;
                c.g[V0][0] = 0;
                pc = 0x0026_427c;
            }
            0x0026_427c => {
                c.cycles += 2;
                if c.g[V0][0] != 0 {
                    c.g[A2][0] = Addr(c.g[S0][0] as u32).off(SETUP_SEND_HEAD).lw(c);
                    if back_edge_unwinds(c, 0x0026_4208) {
                        return;
                    }
                    pc = 0x0026_4208;
                } else {
                    pc = 0x0026_4284;
                }
            }
            0x0026_4284 => {
                c.cycles += 3;
                c.g[V0][0] = 0x1;
                let keep = c.g[S3][0] == 0;
                Addr(c.g[S0][0] as u32).sw(c, c.g[V0][0]);
                pc = if keep { 0x0026_4298 } else { 0x0026_4290 };
            }
            0x0026_4290 => {
                c.cycles += 2;
                c.g[A0][0] = c.g[S0][0];
                if call_ptr(c, callee::BUILTIN_DELETE, 0x0026_4298).is_err() {
                    return;
                }
                pc = 0x0026_4298;
            }
            0x0026_4298 => {
                c.cycles += 7;
                let frame = Frame { size: 0x50 };
                frame.restore(c, S0, 0x40);
                frame.restore(c, S1, 0x30);
                frame.restore(c, S2, 0x20);
                frame.restore(c, S3, 0x10);
                frame.restore_ra(c, 0);
                frame.close(c);
                return;
            }
            _ => {
                c.jump_out(pc, 0x0026_4138, 0x0026_42b4);
                return;
            }
        }
    }
}

/// Which saved registers `Net_UpdateGameSetup` still reloads on its way
/// out (the compiled code's three epilogue entries).
#[derive(Clone, Copy, PartialEq)]
enum NetUpdateEnd {
    /// `s0`, `s1` and `s2`.
    RestoreS0,
    /// `s1` and `s2` (`s0` already reloaded).
    RestoreS1,
    /// `s2` only.
    RestoreS2,
}

/// `Net_UpdateGameSetup`'s epilogue from `end`.
fn net_update_finish(c: &mut Cpu, frame: Frame, end: NetUpdateEnd) -> crate::call::Flow {
    if end == NetUpdateEnd::RestoreS0 {
        c.cycles += 1;
        frame.restore(c, S0, 0x50);
    }
    if end != NetUpdateEnd::RestoreS2 {
        c.cycles += 1;
        frame.restore(c, S1, 0x40);
    }
    c.cycles += 4;
    frame.restore(c, S2, 0x30);
    frame.restore_ra(c, 0x20);
    frame.close(c);
    Ok(())
}

/// `void Net_UpdateGameSetup(NetGameSetup* self)` (0x263cf0): advance the
/// game-setup state machine in `self[0]`. State 1 waits for the comm channel:
/// on connect it creates the setup manager and says hello (moving to state
/// 2), on error/timeout it aborts. State 2 waits for the setup manager: when
/// it reports ready (0xd) the race is launched (presence, rider character,
/// game-comm construct and the next module), when it reports 0xb the setup
/// is aborted. Other states return at once.
pub fn net_update_game_setup(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x60);
        c.cycles += 10;
        frame.save(c, S2, 0x30);
        frame.save(c, S0, 0x50);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x40);
        frame.save_ra(c, 0x20);
        c.g[V1][0] = c.lw(r32(c, S2));
        let state1 = c.g[V1][0] == 1u64;
        c.g[V0][0] = ((c.g[V1][0] as i64) < 2i64) as u64;
        if !state1 {
            c.cycles += 2;
            let below2 = c.g[V0][0] != 0u64;
            frame.restore(c, S0, 0x50);
            if below2 {
                // State 0: nothing to do.
                return net_update_finish(c, frame, NetUpdateEnd::RestoreS1);
            }
            c.cycles += 3;
            c.g[V0][0] = 0x2;
            let state2 = c.g[V1][0] == c.g[V0][0];
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x238));
            if !state2 {
                // State 3+: nothing to do.
                c.cycles += 2;
                frame.restore(c, S1, 0x40);
                return net_update_finish(c, frame, NetUpdateEnd::RestoreS2);
            }
            // State 2: fall through to the state-2 work below.
        } else {
            // State 1: if the channel is connected, either abort when a
            // challenger is already present or create the setup and say
            // hello; otherwise check the connect timeout.
            c.cycles += 5;
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x238));
            c.g[A1][0] = c.lw((r32(c, S2)).wrapping_add(0x4));
            call_ptr(c, callee::C_COMM_SYSTEM_IS_CHANNEL_CONNECTED, 0x0026_3d40)?;
            let not_connected = c.g[V0][0] == 0u64;
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x238));
            if !not_connected {
                c.cycles += 3;
                c.g[V0][0] = c.lw((r32(c, S2)).wrapping_add(0xc));
                let no_challenger = c.g[V0][0] == 0u64;
                c.g[A0][0] = c.g[S2][0];
                if !no_challenger {
                    c.cycles += 4;
                    c.g[A1][0] = 0x1;
                    callee::NET_ABORT_GAME_SETUP.call(c, 0x0026_3d5c)?;
                    frame.restore(c, S0, 0x50);
                    return net_update_finish(c, frame, NetUpdateEnd::RestoreS1);
                }
                c.cycles += 16;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                c.g[A1][0] = 0x7;
                c.g[A0][0] = c.lw((r32(c, V0)).wrapping_add(0x78));
                call_ptr(c, callee::BE_LIBRARY_GET_INTERFACE, 0x0026_3d74)?;
                c.g[V1][0] = 0x530000;
                c.g[A0][0] = c.lw((r32(c, S2)).wrapping_add(0x4));
                c.g[A2][0] = c.lw((r32(c, V1)).wrapping_add(0x4b34));
                c.g[A1][0] = c.lw((r32(c, S2)).wrapping_add(0x8));
                callee::SETUP_MAN_CREATE.call(c, 0x0026_3d88)?;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2dc));
                c.g[A1][0] = c.lw((r32(c, V0)).wrapping_add(0x8c));
                callee::SETUP_MAN_SEND_HELLO.call(c, 0x0026_3d98)?;
                c.g[V0][0] = 0x2;
                c.sw(r32(c, S2), c.g[V0][0]);
                return net_update_finish(c, frame, NetUpdateEnd::RestoreS0);
            }
            // Not connected: the float at +0x10 is the connect timer.
            c.cycles += 4;
            c.g[A1][0] = c.lw((r32(c, S2)).wrapping_add(0x4));
            call_ptr(c, callee::C_COMM_SYSTEM_GET_CHANNEL_STATUS, 0x0026_3dac)?;
            let error = c.g[V0][0] != 0u64;
            c.g[A0][0] = c.g[S2][0];
            if !error {
                c.cycles += 6;
                c.lwc1(1, (r32(c, S2)).wrapping_add(0x10));
                c.mtc1(0, 0);
                c.c_lt_s(1, 0);
                let timed_out = c.fcc();
                c.g[V0][0] = 0x1;
                if !timed_out {
                    c.cycles += 1;
                    c.g[V0][0] = 0u64;
                }
                c.cycles += 2;
                let still_waiting = c.g[V0][0] == 0u64;
                c.g[A0][0] = c.g[S2][0];
                if still_waiting {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                    c.lwc1(0, (r32(c, V0)).wrapping_add(0x14));
                    c.sub_s(0, 1, 0);
                    c.swc1(0, (r32(c, S2)).wrapping_add(0x10));
                    return net_update_finish(c, frame, NetUpdateEnd::RestoreS0);
                }
            }
            c.cycles += 4;
            c.g[A1][0] = 0u64;
            callee::NET_ABORT_GAME_SETUP.call(c, 0x0026_3de0)?;
            frame.restore(c, S0, 0x50);
            return net_update_finish(c, frame, NetUpdateEnd::RestoreS1);
        }
        if !state1 {
            // State 2 (state 1 returned above): if the channel reports an
            // error, post the 0x113 notice unless the setup state is 0xb,
            // then abort; otherwise run the setup update and launch when
            // it reaches state 0xd.
            c.cycles += 4;
            c.g[A1][0] = c.lw((r32(c, S2)).wrapping_add(0x4));
            call_ptr(c, callee::C_COMM_SYSTEM_GET_CHANNEL_STATUS, 0x0026_3e04)?;
            let no_error = c.g[V0][0] == 0u64;
            c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2dc));
            if !no_error {
                c.cycles += 4;
                c.g[V0][0] = c.lw(r32(c, V1));
                c.g[V0][0] = c.g[V0][0] ^ 0xb;
                let is_b = c.g[V0][0] == 0u64;
                c.g[A1][0] = 0x480000;
                if !is_b {
                    c.cycles += 24;
                    c.g[S0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x488));
                    c.g[A0][0] = 0x8;
                    c.g[A2][0] = 0x20000000;
                    c.g[A3][0] = 0u64;
                    callee::EA_MEM_ALLOC_NAMED.call(c, 0x0026_3e34)?;
                    c.g[S1][0] = c.g[V0][0];
                    c.g[V1][0] = 0x113;
                    c.g[V0][0] = 0x480000;
                    c.sw(r32(c, S1), c.g[V1][0]);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x12b0));
                    c.g[A1][0] = 0x480000;
                    c.sw((r32(c, S1)).wrapping_add(0x4), c.g[V0][0]);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4a8));
                    c.g[A0][0] = 0xc;
                    c.g[A2][0] = 0x20000000;
                    c.g[V0][0] = c.lw((r32(c, S0)).wrapping_add(0xf0));
                    c.g[A3][0] = 0u64;
                    c.sw((r32(c, SP)).wrapping_add(0x10), c.g[V0][0]);
                    callee::EA_MEM_ALLOC_NAMED.call(c, 0x0026_3e6c)?;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x8));
                    if c.g[V0][0] != 0u64 {
                        c.sw((r32(c, A1)).wrapping_add(0x8), c.g[S1][0]);
                    }
                    c.cycles += 9;
                    c.g[V0][0] = c.lw((r32(c, SP)).wrapping_add(0x10));
                    c.sw(r32(c, A1), c.g[V0][0]);
                    c.g[V1][0] = c.lw((r32(c, V0)).wrapping_add(0x4));
                    c.sw((r32(c, A1)).wrapping_add(0x4), c.g[V1][0]);
                    c.g[A0][0] = c.lw((r32(c, V0)).wrapping_add(0x4));
                    c.sw(r32(c, A0), c.g[A1][0]);
                    c.g[V0][0] = c.lw((r32(c, SP)).wrapping_add(0x10));
                    c.sw((r32(c, V0)).wrapping_add(0x4), c.g[A1][0]);
                    c.sw(r32(c, SP), c.g[A1][0]);
                }
                c.cycles += 5;
                c.g[A0][0] = c.g[S2][0];
                c.g[A1][0] = 0x2;
                callee::NET_ABORT_GAME_SETUP.call(c, 0x0026_3eac)?;
                frame.restore(c, S0, 0x50);
                return net_update_finish(c, frame, NetUpdateEnd::RestoreS1);
            }
        }
        // Run the setup update, then launch on state 0xd (abort on 0xb).
        c.cycles += 8;
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2dc));
        c.g[S0][0] = 0u64;
        callee::SETUP_MAN_UPDATE.call(c, 0x0026_3ec0)?;
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2dc));
        c.g[V1][0] = 0xd;
        c.g[A0][0] = c.lw(r32(c, V0));
        let not_d = c.g[A0][0] != c.g[V1][0];
        c.g[V0][0] = 0xb;
        if !not_d {
            c.cycles += 2;
            c.g[S0][0] = 0x1;
        } else {
            c.cycles += 2;
            let not_b = c.g[A0][0] != c.g[V0][0];
            c.g[A0][0] = c.g[S2][0];
            if !not_b {
                c.cycles += 2;
                c.g[A1][0] = 0x2;
                callee::NET_ABORT_GAME_SETUP.call(c, 0x0026_3eec)?;
            }
        }
        c.cycles += 2;
        if c.g[S0][0] == 0u64 {
            frame.restore(c, S0, 0x50);
            return net_update_finish(c, frame, NetUpdateEnd::RestoreS1);
        }
        c.cycles += 8;
        callee::SETUPMAN_DESTROY.call(c, 0x0026_3efc)?;
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
        callee::LOBBY_MAN_DISCONNECT.call(c, 0x0026_3f04)?;
        c.g[V0][0] = 0x34;
        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
        let comm_present = c.g[V1][0] != 0u64;
        c.sw((c.g[GP][0] as u32).wrapping_sub(0x172c), c.g[V0][0]);
        if !comm_present {
            c.cycles += 2;
            callee::BUDDY_MAN_CREATE.call(c, 0x0026_3f1c)?;
        }
        c.cycles += 4;
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
        callee::BUDDY_MAN_IS_CONNECTED.call(c, 0x0026_3f24)?;
        let connected = c.g[V0][0] != 0u64;
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
        if !connected {
            c.cycles += 3;
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
            callee::BUDDY_MAN_RECONNECT.call(c, 0x0026_3f34)?;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
        }
        c.cycles += 2;
        let comm_again = c.g[V0][0] != 0u64;
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
        if !comm_again {
            c.cycles += 3;
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
            callee::LOBBY_MAN_DISCONNECT.call(c, 0x0026_3f48)?;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
        }
        c.cycles += 33;
        c.g[T0][0] = 0u64;
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
        c.g[T1][0] = 0u64;
        c.g[A3][0] = c.lw((r32(c, V0)).wrapping_add(0x98));
        c.g[A2][0] = 0x9;
        c.g[A1][0] = 0x1;
        call_ptr(c, callee::BUDDY_MAN_SET_PRESENCE_ADDR, 0x0026_3f68)?;
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
        c.g[A1][0] = 0x7;
        c.g[A0][0] = c.lw((r32(c, V0)).wrapping_add(0x78));
        call_ptr(c, callee::BE_LIBRARY_GET_INTERFACE, 0x0026_3f78)?;
        c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
        c.g[S1][0] = c.g[V0][0];
        c.g[A1][0] = 0x1;
        c.g[A0][0] = c.lw((r32(c, V1)).wrapping_add(0x78));
        call_ptr(c, callee::BE_LIBRARY_GET_INTERFACE, 0x0026_3f8c)?;
        c.g[V1][0] = 0x530000;
        c.g[S0][0] = c.g[V0][0];
        c.g[A2][0] = c.lw((r32(c, V1)).wrapping_add(0x4e58));
        c.g[A1][0] = 0u64;
        c.g[A0][0] = c.g[S0][0];
        callee::BE_NEW_PLAYER_INT_SET_CHARACTER.call(c, 0x0026_3fa4)?;
        c.g[V0][0] = c.lw((r32(c, S0)).wrapping_add(0xc));
        c.g[A0][0] = c.lh((r32(c, V0)).wrapping_add(0x8));
        c.g[V1][0] = c.lw((r32(c, V0)).wrapping_add(0xc));
        let target = r32(c, V1);
        c.g[A0][0] = sx((r32(c, S0)).wrapping_add(r32(c, A0)));
        call_ptr(c, target, 0x0026_3fb8)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = 0u64;
        callee::BE_BACKUP_PLAYER_SETUP.call(c, 0x0026_3fc4)?;
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x204));
        if c.g[V0][0] == 0u64 {
            c.g[A1][0] = c.lw((r32(c, S2)).wrapping_add(0x8));
        } else {
            c.cycles += 3;
            call_ptr(c, callee::GAME_COMM_DESTROY.addr, 0x0026_3fd8)?;
            c.g[A1][0] = c.lw((r32(c, S2)).wrapping_add(0x8));
        }
        c.cycles += 14;
        c.g[A0][0] = c.lw((r32(c, S2)).wrapping_add(0x4));
        call_ptr(c, callee::C_GAME_COMM_CREATE, 0x0026_3fe4)?;
        c.g[A1][0] = 0x480000;
        c.g[A2][0] = 0x100;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xef0));
        c.g[A3][0] = 0u64;
        c.g[A0][0] = 0x8;
        callee::EA_MEM_ALLOC_NAMED.call(c, 0x0026_3ffc)?;
        c.g[A0][0] = c.g[V0][0];
        callee::PRE_GAME_LOAD_SCREEN_CTOR.call(c, 0x0026_4004)?;
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
        c.g[A1][0] = c.g[V0][0];
        callee::APP_SET_NEXT_STATE.call(c, 0x0026_4010)?;
        c.sw(r32(c, S2), 0u64);
        net_update_finish(c, frame, NetUpdateEnd::RestoreS0)
    })
}

/// Post the abort notification node for `Net_AbortGameSetup`: fill the
/// message in `s1` (code already in `v1`), allocate the list node and link
/// it into the manager list at `s0 + 0xf0`, then reload `s0`. Shared by the
/// arg-1 (0x10c) and arg-0 (0x112) tails, which differ only in the code.
pub(crate) fn abort_post_node(c: &mut Cpu, frame: Frame) -> crate::call::Flow {
    c.cycles += 16;
    c.g[V0][0] = 0x0048_0000;
    c.sw(r32(c, S1), c.g[V1][0]);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x12b0));
    c.g[A1][0] = 0x0048_0000;
    c.sw((r32(c, S1)).wrapping_add(4), c.g[V0][0]);
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4a8));
    c.g[A0][0] = 0xc;
    c.g[A2][0] = 0x2000_0000;
    c.g[V0][0] = c.lw((r32(c, S0)).wrapping_add(0xf0));
    c.g[A3][0] = 0u64;
    c.sw((r32(c, SP)).wrapping_add(0x10), c.g[V0][0]);
    callee::EA_MEM_ALLOC_NAMED.call(c, 0x0026_3ca4)?;
    c.g[A1][0] = c.g[V0][0];
    c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(8));
    if c.g[V0][0] != 0u64 {
        c.sw((r32(c, A1)).wrapping_add(8), c.g[S1][0]);
    }
    c.cycles += 10;
    c.g[V0][0] = c.lw((r32(c, SP)).wrapping_add(0x10));
    c.sw(r32(c, A1), c.g[V0][0]);
    c.g[V1][0] = c.lw((r32(c, V0)).wrapping_add(4));
    c.sw((r32(c, A1)).wrapping_add(4), c.g[V1][0]);
    c.g[A0][0] = c.lw((r32(c, V0)).wrapping_add(4));
    c.sw(r32(c, A0), c.g[A1][0]);
    c.g[V0][0] = c.lw((r32(c, SP)).wrapping_add(0x10));
    c.sw((r32(c, V0)).wrapping_add(4), c.g[A1][0]);
    c.sw(r32(c, SP), c.g[A1][0]);
    frame.restore(c, S0, 0x40);
    Ok(())
}

/// `void Net_AbortGameSetup(int* self, int arg)` (0x263ba8): drop the
/// pending game-setup session: remove its comm channel and stop VoIP when
/// the channel exists, destroy the setup manager, reset the lobby state,
/// clear `self[0]`, and post the abort notice (0x10c for arg 1, 0x112 for
/// arg 0; no notice for other args).
pub fn net_abort_game_setup(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x20);
        c.g[V0][0] = c.lw(r32(c, S0));
        let no_session = c.g[V0][0] == 0u64;
        c.g[S1][0] = c.g[A1][0];
        if !no_session {
            c.cycles += 5;
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x238));
            c.g[A1][0] = c.lw((r32(c, S0)).wrapping_add(4));
            callee::C_COMM_SYSTEM_HAS_CHANNEL.call(c, 0x0026_3bd4)?;
            let missing = c.g[V0][0] == 0u64;
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x238));
            if !missing {
                c.cycles += 6;
                c.g[A1][0] = c.lw((r32(c, S0)).wrapping_add(4));
                call_ptr(c, callee::C_COMM_SYSTEM_CLOSE_CHANNEL, 0x0026_3be4)?;
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x304));
                callee::VOIP_MAN_DEACTIVATE.call(c, 0x0026_3bec)?;
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x304));
                callee::VOIP_MAN_STOP.call(c, 0x0026_3bf4)?;
            }
        }
        c.cycles += 3;
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x2dc));
        let no_setup = c.g[V0][0] == 0u64;
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
        if !no_setup {
            c.cycles += 3;
            callee::SETUPMAN_DESTROY.call(c, 0x0026_3c08)?;
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
        }
        c.cycles += 5;
        c.g[A1][0] = 0u64;
        call_ptr(c, callee::LOBBY_MAN_CLEAR_CHALLENGE_LOCK, 0x0026_3c14)?;
        c.g[V0][0] = 0x1;
        let other = c.g[S1][0] != c.g[V0][0];
        c.sw(r32(c, S0), 0u64);
        if other {
            c.cycles += 2;
            let queued = c.g[S1][0] != 0u64;
            frame.restore(c, S0, 0x40);
            if !queued {
                c.cycles += 9;
                c.g[A1][0] = 0x0048_0000;
                c.g[S0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x488));
                c.g[A0][0] = 0x8;
                c.g[A2][0] = 0x2000_0000;
                c.g[A3][0] = 0u64;
                callee::EA_MEM_ALLOC_NAMED.call(c, 0x0026_3c6c)?;
                c.g[S1][0] = c.g[V0][0];
                c.g[V1][0] = 0x112;
                abort_post_node(c, frame)?;
            }
        } else {
            c.cycles += 10;
            c.g[A1][0] = 0x0048_0000;
            c.g[S0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x488));
            c.g[A0][0] = 0x8;
            c.g[A2][0] = 0x2000_0000;
            c.g[A3][0] = 0u64;
            callee::EA_MEM_ALLOC_NAMED.call(c, 0x0026_3c3c)?;
            c.g[S1][0] = c.g[V0][0];
            c.g[V1][0] = 0x10c;
            abort_post_node(c, frame)?;
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// Call interface slot 1 (`+0xc` vtable, entry at `+8`/`+0xc`) of `iface`
/// for `Net_StartGameSetup`, leaving the dance in v0/a0/v1 as generated.
pub(crate) fn start_iface_call(c: &mut Cpu, iface: u64, ret: u32) -> crate::call::Flow {
    c.g[V0][0] = c.lw((iface as u32).wrapping_add(0xc));
    c.g[A0][0] = c.lh((r32(c, V0)).wrapping_add(0x8));
    c.g[V1][0] = c.lw((r32(c, V0)).wrapping_add(0xc));
    let target = r32(c, V1);
    c.g[A0][0] = sx((iface as u32).wrapping_add(r32(c, A0)));
    call_ptr(c, target, ret)
}

/// `void Net_StartGameSetup(self, ip, chan, a3, a4)` (0x2639e0): open the
/// game-setup session: configure the race/player interfaces for two
/// humans, publish the DSock channel record (port 0xe4a), format the peer
/// IP and open the channel, start VoIP when the global flag allows, then
/// mark the session live (`self[0] = 1`, timeout at `self + 0x10`).
pub fn net_start_game_setup(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 96;
        let frame = Frame::open(c, 0x180);
        frame.save(c, S5, 0x120);
        frame.save(c, S0, 0x170);
        c.g[S5][0] = c.g[A0][0];
        frame.save(c, S1, 0x160);
        frame.save(c, S2, 0x150);
        frame.save(c, S3, 0x140);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, S4, 0x130);
        c.g[S3][0] = c.g[A3][0];
        frame.save(c, S6, 0x110);
        c.g[S4][0] = c.g[A1][0];
        frame.save_ra(c, 0x100);
        c.g[S6][0] = 0x1;
        c.sw((r32(c, S5)).wrapping_add(0xc), c.g[T0][0]);
        call_ptr(c, callee::BE_LIBRARY_GET_INSTANCE, 0x0026_3a20)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0x1;
        call_ptr(c, callee::BE_LIBRARY_GET_INTERFACE, 0x0026_3a2c)?;
        c.g[S1][0] = c.g[V0][0];
        call_ptr(c, callee::BE_LIBRARY_GET_INSTANCE, 0x0026_3a34)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0u64;
        call_ptr(c, callee::BE_LIBRARY_GET_INTERFACE, 0x0026_3a40)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A1][0] = 0x2;
        c.g[A0][0] = c.g[S0][0];
        callee::BE_NEW_RACE_INT_SET_NUM_HUMAN_PLAYERS.call(c, 0x0026_3a50)?;
        start_iface_call(c, c.g[S0][0], 0x0026_3a64)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[A2][0] = c.g[S3][0];
        c.g[A1][0] = 0u64;
        callee::BE_NEW_PLAYER_INT_SET_PAD_INDEX_BY_SLOT.call(c, 0x0026_3a74)?;
        c.g[A2][0] = 0u64;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = 0x1;
        callee::BE_NEW_PLAYER_INT_SET_PAD_INDEX_BY_SLOT.call(c, 0x0026_3a84)?;
        start_iface_call(c, c.g[S1][0], 0x0026_3a98)?;
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
        c.g[A1][0] = 0x7;
        c.g[A0][0] = c.lw((r32(c, V0)).wrapping_add(0x78));
        call_ptr(c, callee::BE_LIBRARY_GET_INTERFACE, 0x0026_3aa8)?;
        c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x4b74));
        c.g[V1][0] = 0x530000;
        c.sw((r32(c, V1)).wrapping_add(0x4b30), c.g[S6][0]);
        c.g[S1][0] = sx((r32(c, V1)).wrapping_add(0x4b30));
        c.g[A0][0] = 0xe4a;
        c.swc1(0, (r32(c, S1)).wrapping_add(0x10));
        c.sw((r32(c, S1)).wrapping_add(0xc), c.g[A0][0]);
        c.g[S0][0] = c.g[V0][0];
        c.sw((r32(c, S1)).wrapping_add(0x8), c.g[S2][0]);
        start_iface_call(c, c.g[S0][0], 0x0026_3ae0)?;
        start_iface_call(c, c.g[S0][0], 0x0026_3af4)?;
        c.sw((r32(c, S5)).wrapping_add(0x8), c.g[S2][0]);
        c.g[A1][0] = 0x480000;
        c.g[A3][0] = sx((r32(c, S4)) >> 16);
        c.g[T0][0] = sx((r32(c, S4)) >> 8);
        c.g[A0][0] = c.g[SP][0];
        c.g[T1][0] = c.g[S4][0] & 0xff;
        c.g[T0][0] = c.g[T0][0] & 0xff;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xee0));
        c.g[A3][0] = c.g[A3][0] & 0xff;
        c.g[A2][0] = sx((r32(c, S4)) >> 24);
        call_ptr(c, callee::SPRINTF_6.addr, 0x0026_3b20)?;
        c.g[A2][0] = c.lhu((r32(c, S1)).wrapping_add(0xc));
        c.g[A1][0] = c.g[SP][0];
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x238));
        c.g[A3][0] = c.g[S2][0];
        call_ptr(c, callee::C_COMM_SYSTEM_CREATE_CHANNEL, 0x0026_3b34)?;
        c.sw((r32(c, S5)).wrapping_add(0x4), c.g[V0][0]);
        call_ptr(c, callee::BE_LIBRARY_GET_INSTANCE, 0x0026_3b3c)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0x4;
        call_ptr(c, callee::BE_LIBRARY_GET_INTERFACE, 0x0026_3b48)?;
        c.g[V1][0] = 0x530000;
        c.g[V0][0] = c.lw((r32(c, V1)).wrapping_add(0x5610));
        c.g[V0][0] = sx(((r32(c, V0)) as i32 >> 18) as u32);
        c.g[V0][0] = c.g[V0][0] & 0x1;
        let voip_off = c.g[V0][0] == 0u64;
        c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x1dc));
        if !voip_off {
            c.cycles += 6;
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x304));
            c.g[A1][0] = c.g[S4][0];
            callee::VOIP_MAN_START.call(c, 0x0026_3b6c)?;
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x304));
            callee::VOIP_MAN_ACTIVATE.call(c, 0x0026_3b74)?;
            c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x1dc));
        }
        c.cycles += 12;
        c.sw(r32(c, S5), c.g[S6][0]);
        c.swc1(0, (r32(c, S5)).wrapping_add(0x10));
        frame.restore(c, S0, 0x170);
        frame.restore(c, S1, 0x160);
        frame.restore(c, S2, 0x150);
        frame.restore(c, S3, 0x140);
        frame.restore(c, S4, 0x130);
        frame.restore(c, S5, 0x120);
        frame.restore(c, S6, 0x110);
        frame.restore_ra(c, 0x100);
        frame.close(c);
        Ok(())
    })
}

/// `void Net_DisconnectAll(void* self, void* arg)` (0x2636f0): tear down
/// the network: make sure the buddy manager exists and is connected (create
/// it, reconnect, or else `LobbyMan_Disconnect`), disconnect the buddy
/// manager and the lobby, and when the comm system's state (`+0x20`) is not
/// 1 disconnect it, post the 0x116 notice carrying `arg` unless `arg` is 0,
/// and clear `self[0]`.
pub fn net_disconnect_all(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x70);
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
        frame.save(c, S2, 0x40);
        frame.save(c, S3, 0x30);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S0, 0x60);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S1, 0x50);
        let have = c.g[V0][0] != 0u64;
        frame.save_ra(c, 0x20);
        if !have {
            c.cycles += 2;
            callee::BUDDY_MAN_CREATE.call(c, 0x0026_3720)?;
        }
        c.cycles += 4;
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
        callee::BUDDY_MAN_IS_CONNECTED.call(c, 0x0026_3728)?;
        let running = c.g[V0][0] != 0u64;
        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
        if !running {
            c.cycles += 3;
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
            callee::BUDDY_MAN_RECONNECT.call(c, 0x0026_3738)?;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
        }
        c.cycles += 2;
        let pending = c.g[V0][0] != 0u64;
        if !pending {
            c.cycles += 2;
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
            callee::LOBBY_MAN_DISCONNECT.call(c, 0x0026_374c)?;
        }
        c.cycles += 9;
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
        callee::BUDDY_MAN_DISCONNECT.call(c, 0x0026_3754)?;
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
        callee::LOBBY_MAN_DISCONNECT.call(c, 0x0026_375c)?;
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x238));
        c.g[V0][0] = c.lw((r32(c, A0)).wrapping_add(0x20));
        c.g[V0][0] = c.g[V0][0] ^ 1;
        let idle = c.g[V0][0] == 0u64;
        frame.restore(c, S0, 0x60);
        if !idle {
            c.cycles += 4;
            call_ptr(c, callee::C_COMM_SYSTEM_DISCONNECT, 0x0026_3778)?;
            let no_arg = c.g[S2][0] == 0u64;
            c.g[A1][0] = 0x0048_0000;
            if !no_arg {
                c.cycles += 25;
                c.g[S0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xed0));
                c.g[A0][0] = 0xc;
                c.g[A2][0] = 0x2000_0000;
                c.g[A3][0] = 0u64;
                callee::EA_MEM_ALLOC_NAMED.call(c, 0x0026_3798)?;
                c.g[S1][0] = c.g[V0][0];
                c.g[V1][0] = 0x0048_0000;
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1250));
                c.g[V0][0] = 0x116;
                c.sw(r32(c, S1), c.g[V0][0]);
                c.g[A1][0] = 0x0048_0000;
                c.sw((r32(c, S1)).wrapping_add(4), c.g[V1][0]);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4a8));
                c.sw((r32(c, S1)).wrapping_add(8), c.g[S2][0]);
                c.g[A0][0] = 0xc;
                c.g[A2][0] = 0x2000_0000;
                c.g[A3][0] = 0u64;
                c.g[V0][0] = c.lw((r32(c, S0)).wrapping_add(0xf0));
                c.sw((r32(c, SP)).wrapping_add(0x10), c.g[V0][0]);
                callee::EA_MEM_ALLOC_NAMED.call(c, 0x0026_37d4)?;
                c.g[A1][0] = c.g[V0][0];
                c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(8));
                if c.g[V0][0] != 0u64 {
                    c.sw((r32(c, A1)).wrapping_add(8), c.g[S1][0]);
                }
                c.cycles += 9;
                c.g[V0][0] = c.lw((r32(c, SP)).wrapping_add(0x10));
                c.sw(r32(c, A1), c.g[V0][0]);
                c.g[V1][0] = c.lw((r32(c, V0)).wrapping_add(4));
                c.sw((r32(c, A1)).wrapping_add(4), c.g[V1][0]);
                c.g[A0][0] = c.lw((r32(c, V0)).wrapping_add(4));
                c.sw(r32(c, A0), c.g[A1][0]);
                c.g[V0][0] = c.lw((r32(c, SP)).wrapping_add(0x10));
                c.sw((r32(c, V0)).wrapping_add(4), c.g[A1][0]);
                c.sw(r32(c, SP), c.g[A1][0]);
            }
            c.cycles += 2;
            c.sw(r32(c, S3), 0u64);
            frame.restore(c, S0, 0x60);
        }
        c.cycles += 6;
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `void BuddyMan_DeleteMessage(BuddyMan* self, void* msg)` (0x263500):
/// scan the message vector backwards from its finish (`+0xb0`) to its
/// begin (`+0xac`); on finding `msg`, shift the tail down over it, drop
/// the finish by one slot and release the message through its vtable.
/// The backward scan delivers events at its head (0x263528) and carries
/// on (no resume entry).
pub fn buddy_man_delete_message(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x30);
        c.cycles += 9;
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = c.lw((r32(c, S0)).wrapping_add(0xb0));
        c.g[V0][0] = c.lw((r32(c, S0)).wrapping_add(0xac));
        if c.g[A0][0] == c.g[V0][0] {
            frame.restore(c, S0, 0x20);
        } else {
            c.cycles += 1;
            c.g[S1][0] = c.lw(r32(c, A0));
            // The generated scan walks down from the finish slot; its
            // back edge checks events at 0x263528.
            'scan: loop {
                c.cycles += 2;
                if c.g[S1][0] != c.g[A1][0] {
                    c.g[V0][0] = c.lw((r32(c, S0)).wrapping_add(0xac));
                    c.cycles += 3;
                    c.g[A0][0] = sx(r32(c, A0).wrapping_sub(4));
                    if c.g[A0][0] == c.g[V0][0] {
                        c.cycles += 1;
                        frame.restore(c, S0, 0x20);
                        break 'scan;
                    }
                    c.g[S1][0] = c.lw(r32(c, A0));
                    if event_due(c) {
                        deliver_event_here(c, 0x0026_3528);
                    }
                    continue 'scan;
                }
                c.cycles += 4;
                c.g[V0][0] = c.lw((r32(c, S0)).wrapping_add(0xb0));
                c.g[V1][0] = sx((r32(c, A0)).wrapping_add(4));
                let last = c.g[V1][0] == c.g[V0][0];
                c.g[A1][0] = sx((r32(c, S0)).wrapping_add(0xa8));
                if !last {
                    c.cycles += 7;
                    c.g[A2][0] = c.lw((r32(c, A1)).wrapping_add(8));
                    c.g[A1][0] = c.g[V1][0];
                    c.g[A2][0] = sx((r32(c, A2)).wrapping_sub(r32(c, V1)));
                    c.g[A2][0] = sx(((r32(c, A2) as i32) >> 2) as u32);
                    c.g[A2][0] = sx(r32(c, A2) << 2);
                    call_ptr(c, callee::MEM_MOVE, 0x0026_3558)?;
                    c.g[V0][0] = c.lw((r32(c, S0)).wrapping_add(0xb0));
                }
                c.cycles += 3;
                c.g[V0][0] = sx(r32(c, V0).wrapping_sub(4));
                let empty = c.g[S1][0] == 0u64;
                c.sw((r32(c, S0)).wrapping_add(0xb0), c.g[V0][0]);
                if empty {
                    c.cycles += 1;
                    frame.restore(c, S0, 0x20);
                    break 'scan;
                }
                c.cycles += 8;
                c.g[V0][0] = c.lw((r32(c, S1)).wrapping_add(0xc));
                c.g[A1][0] = 0x3;
                c.g[A0][0] = c.lh((r32(c, V0)).wrapping_add(8));
                c.g[V1][0] = c.lw((r32(c, V0)).wrapping_add(0xc));
                let target = r32(c, V1);
                c.g[A0][0] = sx((r32(c, S1)).wrapping_add(r32(c, A0)));
                call_ptr(c, target, 0x0026_3580)?;
                frame.restore(c, S0, 0x20);
                break 'scan;
            }
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `Buddy* BuddyMan_GetChallengerByName(BuddyMan* self, char* name)`
/// (0x263270): linear scan of the challenger vector (`+0xbc`..`+0xc0`)
/// for the entry whose first word names `name` (`strcmp`);
/// returns the entry or 0. Dispatcher-shaped like the original: the
/// scan checks events at 0x2632ac (match) and 0x2632c8 (loop head).
pub fn buddy_man_get_challenger_by_name(c: &mut Cpu) {
    const ENTRY: u32 = 0x0026_3270;
    const FOUND: u32 = 0x0026_32ac;
    const HEAD: u32 = 0x0026_32c8;
    const DONE: u32 = 0x0026_32ec;
    let frame = Frame { size: 0x50 };
    guest(c, |c| {
        let mut pc = c.pc;
        loop {
            match pc {
                ENTRY => {
                    c.cycles += 13;
                    Frame::open(c, 0x50);
                    frame.save(c, S2, 0x20);
                    frame.save(c, S3, 0x10);
                    c.g[S2][0] = c.g[A0][0];
                    frame.save(c, S0, 0x40);
                    frame.save(c, S1, 0x30);
                    frame.save_ra(c, 0);
                    c.g[A0][0] = c.lw((r32(c, S2)).wrapping_add(0xc0));
                    c.g[V1][0] = c.lw((r32(c, S2)).wrapping_add(0xbc));
                    c.g[V0][0] = sx(r32(c, A0).wrapping_sub(r32(c, V1)));
                    c.g[V0][0] = sx(((r32(c, V0) as i32) >> 2) as u32);
                    let empty = c.g[V0][0] == 0u64;
                    c.g[S3][0] = c.g[A1][0];
                    if empty {
                        c.cycles += 2;
                        c.g[V0][0] = 0u64;
                        pc = DONE;
                    } else {
                        pc = 0x0026_32b4;
                    }
                }
                0x0026_32b4 => {
                    c.cycles += 3;
                    c.g[S0][0] = c.g[V1][0];
                    let at_end = c.g[S0][0] == c.g[A0][0];
                    c.g[V0][0] = 0u64;
                    if at_end {
                        pc = DONE;
                    } else {
                        pc = 0x0026_32c0;
                    }
                }
                0x0026_32c0 => {
                    c.cycles += 2;
                    c.g[S1][0] = c.lw(r32(c, S0));
                    pc = HEAD;
                }
                HEAD => {
                    c.cycles += 5;
                    c.g[A1][0] = c.g[S3][0];
                    c.g[A0][0] = c.lw(r32(c, S1));
                    call_ptr(c, callee::STRCMP, 0x0026_32d4)?;
                    let same = c.g[V0][0] == 0u64;
                    c.g[S0][0] = sx(r32(c, S0).wrapping_add(4));
                    if same {
                        if back_edge_unwinds(c, FOUND) {
                            return Ok(());
                        }
                        pc = FOUND;
                    } else {
                        pc = 0x0026_32dc;
                    }
                }
                FOUND => {
                    c.cycles += 2;
                    c.g[V0][0] = c.g[S1][0];
                    pc = DONE;
                }
                0x0026_32dc => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((r32(c, S2)).wrapping_add(0xc0));
                    if c.g[S0][0] != c.g[V0][0] {
                        c.g[S1][0] = c.lw(r32(c, S0));
                        if back_edge_unwinds(c, HEAD) {
                            return Ok(());
                        }
                        pc = HEAD;
                    } else {
                        pc = 0x0026_32e8;
                    }
                }
                0x0026_32e8 => {
                    c.cycles += 1;
                    c.g[V0][0] = 0u64;
                    pc = DONE;
                }
                DONE => {
                    c.cycles += 7;
                    frame.restore(c, S0, 0x40);
                    frame.restore(c, S1, 0x30);
                    frame.restore(c, S2, 0x20);
                    frame.restore(c, S3, 0x10);
                    frame.restore_ra(c, 0);
                    frame.close(c);
                    return Ok(());
                }
                other => {
                    c.jump_out(other, ENTRY, 0x0026_3308);
                    return Ok(());
                }
            }
        }
    })
}

/// `bool BuddyMan_HasChallengerStr(BuddyMan* self, char** name)`
/// (0x2631e0): like `BuddyMan_GetChallengerByName`, but the name arrives
/// indirected (`*name`) and the result is 1/0. Dispatcher-shaped: the
/// scan checks events at 0x263218 (match) and 0x263230 (loop head).
pub fn buddy_man_has_challenger_str(c: &mut Cpu) {
    const ENTRY: u32 = 0x0026_31e0;
    const FOUND: u32 = 0x0026_3218;
    const HEAD: u32 = 0x0026_3230;
    const DONE: u32 = 0x0026_3254;
    let frame = Frame { size: 0x40 };
    guest(c, |c| {
        let mut pc = c.pc;
        loop {
            match pc {
                ENTRY => {
                    c.cycles += 12;
                    Frame::open(c, 0x40);
                    frame.save(c, S1, 0x20);
                    frame.save(c, S2, 0x10);
                    c.g[S1][0] = c.g[A0][0];
                    frame.save(c, S0, 0x30);
                    frame.save_ra(c, 0);
                    c.g[A0][0] = c.lw((r32(c, S1)).wrapping_add(0xc0));
                    c.g[V1][0] = c.lw((r32(c, S1)).wrapping_add(0xbc));
                    c.g[V0][0] = sx(r32(c, A0).wrapping_sub(r32(c, V1)));
                    c.g[V0][0] = sx(((r32(c, V0) as i32) >> 2) as u32);
                    let empty = c.g[V0][0] == 0u64;
                    c.g[S2][0] = c.g[A1][0];
                    if empty {
                        c.cycles += 2;
                        c.g[V0][0] = 0u64;
                        pc = DONE;
                    } else {
                        pc = 0x0026_3220;
                    }
                }
                0x0026_3220 => {
                    c.cycles += 3;
                    c.g[S0][0] = c.g[V1][0];
                    let at_end = c.g[S0][0] == c.g[A0][0];
                    c.g[V0][0] = 0u64;
                    if at_end {
                        pc = DONE;
                    } else {
                        pc = 0x0026_322c;
                    }
                }
                0x0026_322c => {
                    c.cycles += 1;
                    c.g[V0][0] = c.lw(r32(c, S0));
                    pc = HEAD;
                }
                HEAD => {
                    c.cycles += 5;
                    c.g[A1][0] = c.lw(r32(c, S2));
                    c.g[A0][0] = c.lw(r32(c, V0));
                    call_ptr(c, callee::STRCMP, 0x0026_323c)?;
                    let same = c.g[V0][0] == 0u64;
                    c.g[S0][0] = sx(r32(c, S0).wrapping_add(4));
                    if same {
                        if back_edge_unwinds(c, FOUND) {
                            return Ok(());
                        }
                        pc = FOUND;
                    } else {
                        pc = 0x0026_3244;
                    }
                }
                FOUND => {
                    c.cycles += 2;
                    c.g[V0][0] = 0x1;
                    pc = DONE;
                }
                0x0026_3244 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((r32(c, S1)).wrapping_add(0xc0));
                    if c.g[S0][0] != c.g[V0][0] {
                        c.g[V0][0] = c.lw(r32(c, S0));
                        if back_edge_unwinds(c, HEAD) {
                            return Ok(());
                        }
                        pc = HEAD;
                    } else {
                        pc = 0x0026_3250;
                    }
                }
                0x0026_3250 => {
                    c.cycles += 1;
                    c.g[V0][0] = 0u64;
                    pc = DONE;
                }
                DONE => {
                    c.cycles += 6;
                    frame.restore(c, S0, 0x30);
                    frame.restore(c, S1, 0x20);
                    frame.restore(c, S2, 0x10);
                    frame.restore_ra(c, 0);
                    frame.close(c);
                    return Ok(());
                }
                other => {
                    c.jump_out(other, ENTRY, 0x0026_326c);
                    return Ok(());
                }
            }
        }
    })
}

/// `void BuddyMan_RemoveChallenger(BuddyMan* self, char* name)`
/// (0x263018): remove the named challenger: notify the lobby, erase the
/// entry from the challenger vector (`+0xbc`..`+0xc0`) and release it;
/// then, for a temporary buddy with no pending message, delete its roster
/// entry through `BuddyApi_Del` on the buddy API at `self + 0xc8` (reply
/// callback `BuddyMan_OnRosterRemoveReplyThunk`, timeout 600).
pub fn buddy_man_remove_challenger(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x40);
        c.cycles += 11;
        frame.save(c, S0, 0x30);
        frame.save(c, S2, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x20);
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A1][0];
        callee::BUDDY_MAN_FIND_CHALLENGER_INDEX.call(c, 0x0026_3038)?;
        c.g[S1][0] = c.g[V0][0];
        let missing = (c.g[S1][0] as i64) < 0;
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
        // Set on the has-message path, which restores s0 early and skips
        // the shared +1 charge at the epilogue.
        let mut s0_restored = false;
        if !missing {
            c.cycles += 10;
            c.g[A1][0] = c.g[S2][0];
            call_ptr(c, callee::LOBBY_MAN_SEND_DECLINE.addr, 0x0026_304c)?;
            c.g[V0][0] = c.lw((r32(c, S0)).wrapping_add(0xbc));
            c.g[V1][0] = sx(r32(c, S1) << 2);
            c.g[A0][0] = c.lw((r32(c, S0)).wrapping_add(0xc0));
            c.g[A1][0] = sx((r32(c, S0)).wrapping_add(0xb8));
            c.g[V0][0] = sx((r32(c, V0)).wrapping_add(r32(c, V1)));
            c.g[V1][0] = sx((r32(c, V0)).wrapping_add(4));
            let last = c.g[V1][0] == c.g[A0][0];
            c.g[S1][0] = c.lw(r32(c, V0));
            if !last {
                c.cycles += 7;
                c.g[A2][0] = c.lw((r32(c, A1)).wrapping_add(8));
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = c.g[V1][0];
                c.g[A2][0] = sx((r32(c, A2)).wrapping_sub(r32(c, V1)));
                c.g[A2][0] = sx(((r32(c, A2) as i32) >> 2) as u32);
                c.g[A2][0] = sx(r32(c, A2) << 2);
                call_ptr(c, callee::MEM_MOVE, 0x0026_3088)?;
            }
            c.cycles += 4;
            c.g[V0][0] = c.lw((r32(c, S0)).wrapping_add(0xc0));
            c.g[V0][0] = sx(r32(c, V0).wrapping_sub(4));
            let empty = c.g[S1][0] == 0u64;
            c.sw((r32(c, S0)).wrapping_add(0xc0), c.g[V0][0]);
            if !empty {
                c.cycles += 6;
                c.g[V0][0] = c.lw((r32(c, S1)).wrapping_add(0x3c));
                c.g[A1][0] = 0x3;
                c.g[A0][0] = c.lh((r32(c, V0)).wrapping_add(8));
                c.g[V1][0] = c.lw((r32(c, V0)).wrapping_add(0xc));
                let target = r32(c, V1);
                c.g[A0][0] = sx((r32(c, S1)).wrapping_add(r32(c, A0)));
                call_ptr(c, target, 0x0026_30b0)?;
            }
            c.cycles += 5;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[S2][0];
            callee::BUDDY_MAN_IS_TEMP.call(c, 0x0026_30bc)?;
            let not_temp = c.g[V0][0] == 0u64;
            c.g[A0][0] = c.g[S0][0];
            if !not_temp {
                c.cycles += 4;
                c.g[A1][0] = c.g[S2][0];
                callee::BUDDY_MAN_HAS_MESSAGE_FROM.call(c, 0x0026_30cc)?;
                if c.g[V0][0] != 0u64 {
                    frame.restore(c, S0, 0x30);
                    s0_restored = true;
                } else {
                    c.cycles += 8;
                    c.g[A1][0] = c.g[S2][0];
                    c.g[A0][0] = c.g[S0][0];
                    callee::BUDDY_FIND_ROSTER_ENTRY.call(c, 0x0026_30e0)?;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[A0][0] = c.g[S0][0];
                    callee::BUDDY_MAN_GET_ROSTER_ENTRY.call(c, 0x0026_30ec)?;
                    let no_entry = c.g[V0][0] == 0u64;
                    c.g[A2][0] = 0x270000;
                    if !no_entry {
                        c.cycles += 6;
                        c.g[A0][0] = c.lw((r32(c, S0)).wrapping_add(0xc8));
                        c.g[A1][0] = c.g[V0][0];
                        c.g[A2][0] = sx((r32(c, A2)).wrapping_sub(0x7070));
                        c.g[A3][0] = c.g[S0][0];
                        c.g[T0][0] = 0x258;
                        call_ptr(c, callee::BUDDY_API_DEL, 0x0026_310c)?;
                    }
                }
            }
        }
        if !s0_restored {
            c.cycles += 1;
            frame.restore(c, S0, 0x30);
        }
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;

    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::DATA;

    pub(crate) use crate::game::be::beintplayer::tests::r_u32;
    pub(crate) use crate::game::dirtysock::tags::tests::put_str;
    pub(crate) use crate::game::main::gamestate::tests::base_3 as base_2;
    pub(crate) use crate::game::main::gamestate::tests::SEEDS_5 as SEEDS;
    pub(crate) use crate::game::movie::movieplayer::tests::base_3 as base;

    #[test]
    fn buddy_man_clear_login_matches() {
        check(
            ssx3_ee::s06::f_00261058,
            buddy_man_clear_login,
            100,
            &|c, seed| {
                let mut r = Rng(0x2610_5800 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = DATA as u64;
                for off in [0u32, 0x40, 0x44, 0x64] {
                    c.bus.write32(DATA + off, r.next_u32());
                }
            },
        );
    }

    #[test]
    fn c_voip_man_ctor_matches() {
        check(ssx3_ee::s07::f_00266d00, c_voip_man_ctor, 50, &|c, seed| {
            let mut r = Rng(0x0c13_0005 ^ seed as u64);
            scramble(c, &mut r);
            c.g[A0][0] = DATA as u64;
            c.bus.write32(DATA, r.next_u32());
            c.bus.write32(DATA + 4, r.next_u32());
        });
    }

    #[test]
    fn tag_field_get_string_unescaped_matches() {
        check(
            ee::s06::f_00259310__generated,
            tag_field_get_string_unescaped,
            24,
            &|c, seed| {
                let _ = base(c, seed, 0x2593_10, 2);
                let tag = DATA + 0x1000;
                let dst = DATA + 0x2000;
                let text: &[u8] = match seed % 4 {
                    0 => b"`abcdef",
                    1 => b"abcdef",
                    2 => b"",
                    _ => b"`",
                };
                put_str(c, tag, text);
                c.g[A0][0] = tag as u64;
                c.g[A1][0] = dst as u64;
                c.g[A2][0] = (4 + seed % 13) as u64;
                c.g[A3][0] = 0;
                c.g[T0][0] = 0;
            },
        );
    }

    #[test]
    fn tagfieldsetstringescaped_matches() {
        check(
            ee::s06::f_00259288__generated,
            tagfieldsetstringescaped,
            SEEDS,
            &|c, seed| {
                base_2(c, seed, 0x59288, 0);
                let s = DATA + 0x100;
                let text = DATA + 0x800;
                c.g[A0][0] = sx(s);
                c.g[A1][0] = (seed % 4) as u64;
                c.g[A2][0] = r_u32(seed, 3) as u64;
                c.g[A3][0] = sx(text);
                c.bus.write8(text, if seed % 2 == 0 { 0x27 } else { b'x' });
                for i in 1..16u32 {
                    c.bus
                        .write8(text + i, b'a' + (seed.wrapping_add(i) % 26) as u8);
                }
                c.bus.write8(text + 16, 0);
                c.bus.write32(0x004a_50d0, 0);
            },
        );
    }

    use crate::difftest::elf_image;
    use crate::difftest::fakes;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::vtable;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::fakes::LOG_RESULTS;
    use crate::difftest::fill;
    use crate::difftest::GAME_GP;
    use crate::difftest::RA as SENTINEL_RA;
    use crate::difftest::RA as TEST_RA;
    pub(crate) use crate::game::be::bedbprofile::tests::strict as strict_2;
    pub(crate) use crate::game::be::beintstat::tests::allocator;
    pub(crate) use crate::game::bx::bxstring::tests::heap;
    pub(crate) use crate::game::bx::bxstring::tests::pool as pool_4;
    pub(crate) use crate::game::bx::bxstring::tests::put_str_2 as put_str_m2;
    pub(crate) use crate::game::bx::bxstringctor::tests::alloc_base as alloc_base_2;
    pub(crate) use crate::game::bx::bxstringctor::tests::alloc_mutex;
    pub(crate) use crate::game::bx::bxstringctor::tests::be_library;
    pub(crate) use crate::game::camera::trigger::cameratriggerfactory::tests::world_2 as world_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::alloc_base_setup;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_11 as base_10;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_12 as base_11;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_13 as base_12;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_15 as base_14;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_16 as base_15;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_17 as base_16;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_2 as base_2_m2;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::base_6 as base_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_4 as be_library_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_5 as be_library_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::be_library_7 as be_library_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::call_machine;
    pub(crate) use crate::game::fe::feasyncfile::tests::check_all;
    pub(crate) use crate::game::fe::feasyncfile::tests::diff;
    pub(crate) use crate::game::fe::feasyncfile::tests::env;
    pub(crate) use crate::game::fe::feasyncfile::tests::fake_small;
    pub(crate) use crate::game::fe::feasyncfile::tests::fe_base;
    pub(crate) use crate::game::fe::feasyncfile::tests::game_globals;
    pub(crate) use crate::game::fe::feasyncfile::tests::globals_4 as globals_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::heap_init_5 as heap_init_7;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool;
    pub(crate) use crate::game::fe::feasyncfile::tests::pool_allocator_6 as pool_allocator_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::put_str_6 as put_str_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_5 as slot_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::slot_8 as slot_5;
    pub(crate) use crate::game::fe::feasyncfile::tests::strict;
    pub(crate) use crate::game::fe::feasyncfile::tests::string_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::string_4 as string_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::w32;
    pub(crate) use crate::game::fe::feasyncfile::tests::w_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::w_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::w_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::wb;
    pub(crate) use crate::game::fe::feasyncfile::tests::world;
    pub(crate) use crate::game::fe::feasyncfile::tests::world_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::world_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::BE;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ2;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ2_3 as OBJ2_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ2_4 as OBJ2_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ3;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ4;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ_3;
    pub(crate) use crate::game::fe::feasyncfile::tests::OBJ_4;
    pub(crate) use crate::game::fe::feasyncfile::tests::SCRATCH as SCRATCH_2;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_11 as SEEDS_12;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_12 as SEEDS_13;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_13 as SEEDS_14;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_14 as SEEDS_15;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_16;
    pub(crate) use crate::game::fe::feasyncfile::tests::SEEDS_8 as SEEDS_10;
    pub(crate) use crate::game::fe::fepopupmisc::tests::setup;
    pub(crate) use crate::game::fe::festaterewards::tests::base_4 as base_7;
    pub(crate) use crate::game::fe::festaterewards::tests::base_9 as base_17;
    pub(crate) use crate::game::fe::festaterewards::tests::strict as strict_3;
    pub(crate) use crate::game::fe::festatetrophyroom::tests::belib;
    pub(crate) use crate::game::fe::festatetrophyroom::tests::w;
    pub(crate) use crate::game::fe::festatetrophyroom::tests::SEEDS as SEEDS_3;
    pub(crate) use crate::game::main::gamemode::tests::SEEDS as SEEDS_2;
    pub(crate) use crate::game::main::gamestate::tests::base_3 as base_5;
    pub(crate) use crate::game::main::gamestate::tests::SEEDS_5 as SEEDS_11;
    pub(crate) use crate::game::movie::movieplayer::tests::globals;
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
    pub(crate) use crate::game::sound::bankmanager::tests::SEEDS as SEEDS_m2;
    pub(crate) use crate::game::sound::bankmonitor::tests::SEEDS_4;
    pub(crate) use crate::game::wscript::wscriptman::tests::base_6 as base_9;
    pub(crate) use crate::game::wscript::wscriptman::tests::clear_5 as clear_2;
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
                    5 => 0xffff_ffff,           // wraps the base_8 by -4
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
            SEEDS_m2,
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
                // No lobby API: the private-message send returns -1 at_3 once.
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
    /// machine dispatches through the table at_3 0x4860c0; the request states
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
            // The login manager at_3 gp + 0x2e8, its vtable at_3 +0.
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

    /// A lobby object at_3 `SCRATCH`: the session word at_3 `+0`, a `Lobby_List`
    /// at_3 `+0x1000` with `count` entries whose objects carry random flags.
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

    /// A `BuddyMan` at_3 `SCRATCH` with a two-entry roster at_3 `+0xcc`; the name
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
            // the buddy flags at_3 +0xa8 / the online byte at_3 +0xa7.
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
    /// off `gp + 0x2a60..`, which only work when the "pools ready" flag at_3
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
            let base_8 = nodes;
            for n in 0..16u32 {
                let node = base_8 + block * n;
                c.bus.write32(node, if n == 15 { 0 } else { node + block });
            }
            c.bus.write32(pool + head, base_8);
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

    pub(crate) fn zero(c: &mut Cpu, at_3: u32, bytes: u32) {
        for i in (0..bytes).step_by(4) {
            c.bus.write32(at_3 + i, 0);
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
                // A null lobby API: the request add returns -1 at_3 once.
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

    pub(crate) fn put_str8(c: &mut Cpu, at_3: u32, s: &[u8]) {
        for (i, b) in s.iter().enumerate() {
            c.bus.write8(at_3 + i as u32, *b);
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

    /// A real generated function that returns 0: a vtable slot for the
    /// "nothing handled" path.
    pub(crate) const ZERO_FN: u32 = 0x003c_51e0;

    /// Scrambled registers, the game's `gp_8`, the fake-call hooks and a zeroed
    /// BELibrary dummy.
    pub(crate) fn base_8(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let r = machine(c, seed, salt, 0);
        c.g[GP][0] = GAME_GP as u64;
        c.bus.write32(gp_at(-0x1f04), DATA + 0x3_e000);
        r
    }

    /// A vtable at_3 `at_3` whose entries are `{delta, fake}` for each slot offset.
    pub(crate) fn put_vtable(c: &mut Cpu, r: &mut Rng, at_3: u32, slots: &[u32]) -> u32 {
        for (i, &off) in slots.iter().enumerate() {
            let delta = (r.next_u32() as i16 as i32 & !3) as u32;
            c.bus.write16(at_3 + off, delta as u16);
            c.bus.write32(at_3 + off + 4, FAKE + 0x10 * i as u32);
        }
        at_3
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
                let _r = base_8(c, seed, 0x25b6);
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
                let mut r = base_8(c, seed, 0x2651);
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
                let mut r = base_8(c, seed, 0x2659);
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
                let mut r = base_8(c, seed, 0x267e);
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

    /// The dummy world at_3 `world_at`: `+0x84` the game module, its `+0xc` the
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
        let r = machine(c, seed, salt, calls);
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
                let _ = base_2_m2(c, seed, 0x25a8_50);
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
                let mut r = base_2_m2(c, seed, 0x25aa_e0);
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
                let _ = base_2_m2(c, seed, 0x2680_48);
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
                let _ = base_2_m2(c, seed, 0x2682_a8);
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
                let _ = base_2_m2(c, seed, 0x2684_60);
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
                let mut r = base_2_m2(c, seed, 0x2681_b0);
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
                // Empty name (strcpy stops at_3 the first byte).
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

    /// A NUL-terminated string at_3 `a`.
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
    pub(crate) fn cstr(c: &mut Cpu, at_3: u32, s: &str) {
        for (i, b) in s.bytes().enumerate() {
            w8(c, at_3 + i as u32, b as u32);
        }
        w8(c, at_3 + s.len() as u32, 0);
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
                put_str_m2(c, DATA + 0x3000, "player_one");
                put_str_m2(c, DATA + 0x3100, "hello there");
                w(c, lobby + 0x54, service);
                w(c, service + 0xc, 0x6964_6c65); // "idle"
                c.g[A0][0] = sx(lobby);
                c.g[A1][0] = sx(DATA + 0x3000);
                c.g[A2][0] = sx(DATA + 0x3100);
            },
        );
    }

    pub(crate) fn put_text(c: &mut Cpu, at_3: u32, text: &[u8]) {
        for (i, b) in text.iter().enumerate() {
            c.bus.write8(at_3 + i as u32, *b);
        }
        c.bus.write8(at_3 + text.len() as u32, 0);
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
                let _r = machine(c, seed, 0x2653, 8);
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
                let _r = machine(c, seed, 0x264b, 3);
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

    /// cUsbKeyboard_Update: disabled or enabled; ports connected or not;
    /// 0..8 ports (and a non-positive count); data present or not; 0..5
    /// keys each, empty or not; rings not full / full; ring counts negative
    /// too (the signed `% 64`). The libusbkb callees run for real on the
    /// game's tables (ELF image) with the library state at_3 0x525f80 (the
    /// poll state machine idle).
    #[test]
    fn usb_keyboard_update_matches() {
        check(
            ee::s07::f_002668e8__generated,
            usb_keyboard_update,
            300,
            &|c, seed| {
                let mut r = Rng(0x2668e8 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                elf_image(c);
                c.g[GP][0] = GAME_GP as u64;
                c.pc = 0x0026_68e8;
                let this = DATA;
                let lib = 0x0052_5f80u32;
                let conns = DATA + 0x2000;
                c.g[A0][0] = sx(this);
                fill(c, &mut r, this, 0x460 / 4);
                for j in 0..8 {
                    let slot = this + j * usbk::SLOT_SIZE;
                    let count = (r.next_u32() % 300) as i32 - 80;
                    let start = count - (r.next_u32() % 70) as i32;
                    c.bus.write32(slot + usbk::SLOT_COUNT, count as u32);
                    c.bus.write32(slot + usbk::SLOT_START, start as u32);
                }
                c.bus.write32(this + usbk::ENABLED, (seed % 8 != 0) as u32);
                c.bus.write32(lib + 0x9c, 4);
                for i in 0..8 {
                    c.bus.write8(lib + 0x10 + i, (r.next_u32() % 3 != 0) as u8);
                }
                let ports = match seed % 7 {
                    0 => 0,
                    1 => (-(r.next_u32() as i32 % 3)) as u32,
                    _ => 1 + r.next_u32() % 8,
                };
                c.bus.write32(0x0052_6010, ports);
                c.bus.write32(lib + 0xa8, conns);
                for j in 0..8 {
                    let d = conns + j * 0x8c;
                    fill(c, &mut r, d, 0x23);
                    c.bus.write8(d + 0x88, (r.next_u32() % 4 != 0) as u8);
                    c.bus.write32(d + usbk::DATA_COUNT, r.next_u32() % 6);
                    for k in 0..6 {
                        let code = match r.next_u32() % 4 {
                            0 => 0,
                            1 => 0x10,
                            _ => r.next_u32() % 0x100,
                        };
                        c.bus.write16(d + 0xc + 2 * k, code as u16);
                    }
                }
            },
        );
    }

    /// `cUsbKeyboard_GetKey`: open and closed keyboards, channels in and
    /// out of range (negative too) of the real `UsbKb_GetMaxKeyboards`,
    /// empty and non-empty queues with negative and large read indices,
    /// zero and non-zero key / modifier bytes.
    #[test]
    fn c_usb_keyboard_get_key_matches() {
        use super::usb_kb::*;
        crate::difftest::check_lohi(
            ee::s07::f_00266ba8__generated,
            c_usb_keyboard_get_key,
            300,
            &|c, seed| {
                let mut r = Rng(0x0026_6ba8 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.pc = 0x0026_6ba8;
                let (kb, key, modifier) = (DATA, DATA + 0x800, DATA + 0x810);
                let max = 1 + r.next_u32() % 4;
                c.bus.write32(0x0052_6010, max);
                c.bus.write32(
                    kb + ACTIVE,
                    if seed % 6 == 0 { 0 } else { r.next_u32() | 1 },
                );
                let ch = match seed % 7 {
                    0 => max + r.next_u32() % 3,
                    1 => (r.next_u32() % 3 + 1).wrapping_neg(),
                    _ => r.next_u32() % max,
                };
                c.g[A0][0] = sx(kb);
                c.g[A1][0] = sx(ch);
                c.g[A2][0] = sx(key);
                c.g[A3][0] = sx(modifier);
                for i in 0..4u32 {
                    let chan = kb + i * CHANNEL_SIZE as u32;
                    let read = match r.next_u32() % 4 {
                        0 => (r.next_u32() % 200).wrapping_neg(),
                        1 => r.next_u32() % 1000,
                        _ => r.next_u32() % 64,
                    };
                    let count = if r.next_u32() % 4 == 0 {
                        read
                    } else {
                        read.wrapping_add(1 + r.next_u32() % 5)
                    };
                    c.bus.write32(chan + COUNT, count);
                    c.bus.write32(chan + READ, read);
                    for b in 0..0x80u32 {
                        let v = if r.next_u32() % 3 == 0 {
                            0
                        } else {
                            r.next_u32() as u8
                        };
                        c.bus.write8(chan + RING + b, v);
                    }
                }
            },
        );
    }

    #[test]
    fn lobby_man_on_mesg_reply_matches() {
        check(
            ee::s06::f_0025fc20__generated,
            lobby_man_on_mesg_reply,
            8,
            &|c, _seed| {
                let mut r = Rng(0x25fc_2000);
                scramble(c, &mut r);
            },
        );
    }

    #[test]
    fn buddy_man_on_send_message_reply_matches() {
        check(
            ee::s06::f_00261a20__generated,
            buddy_man_on_send_message_reply,
            8,
            &|c, _seed| {
                let mut r = Rng(0x261a_2000);
                scramble(c, &mut r);
            },
        );
    }

    #[test]
    fn c_shared_login_man_request_load_matches() {
        check(
            ee::s07::f_00265df0__generated,
            c_shared_login_man_request_load,
            24,
            &|c, seed| {
                let mut r = Rng(0x265d_f000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = match seed % 3 {
                    0 => DATA as u64,
                    1 => (DATA + 0x40) as u64,
                    _ => 0,
                };
            },
        );
    }

    #[test]
    fn c_shared_login_man_request_update_matches() {
        check(
            ee::s07::f_00265e00__generated,
            c_shared_login_man_request_update,
            24,
            &|c, seed| {
                let mut r = Rng(0x265e_0000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = match seed % 3 {
                    0 => DATA as u64,
                    1 => (DATA + 0x40) as u64,
                    _ => 0,
                };
            },
        );
    }

    #[test]
    fn c_shared_login_man_request_create_matches() {
        check(
            ee::s07::f_00265e10__generated,
            c_shared_login_man_request_create,
            24,
            &|c, seed| {
                let mut r = Rng(0x265e_1000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = match seed % 3 {
                    0 => DATA as u64,
                    1 => (DATA + 0x40) as u64,
                    _ => 0,
                };
            },
        );
    }

    #[test]
    fn c_shared_login_man_request_delete_matches() {
        check(
            ee::s07::f_00265e20__generated,
            c_shared_login_man_request_delete,
            24,
            &|c, seed| {
                let mut r = Rng(0x265e_2000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = match seed % 3 {
                    0 => DATA as u64,
                    1 => (DATA + 0x40) as u64,
                    _ => 0,
                };
            },
        );
    }

    #[test]
    fn c_shared_login_man_request_finish_matches() {
        check(
            ee::s07::f_00265e30__generated,
            c_shared_login_man_request_finish,
            24,
            &|c, seed| {
                let mut r = Rng(0x265e_3000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = match seed % 3 {
                    0 => DATA as u64,
                    1 => (DATA + 0x40) as u64,
                    _ => 0,
                };
            },
        );
    }

    #[test]
    fn fe_net_hook_ctor_matches() {
        check(
            ee::s07::f_00263660__generated,
            fe_net_hook_ctor,
            24,
            &|c, seed| {
                let mut r = Rng(0x2636_6000 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = match seed % 3 {
                    0 => DATA as u64,
                    1 => (DATA + 0x100) as u64,
                    _ => 0,
                };
            },
        );
    }

    #[test]
    fn fe_net_hook2_ctor_matches() {
        check(
            ee::s07::f_00263998__generated,
            fe_net_hook2_ctor,
            24,
            &|c, seed| {
                let mut r = Rng(0x2639_9800 ^ seed as u64);
                scramble(c, &mut r);
                c.g[A0][0] = match seed % 3 {
                    0 => DATA as u64,
                    1 => (DATA + 0x100) as u64,
                    _ => 0,
                };
            },
        );
    }

    #[test]
    fn lobby_man_on_recv_debug_matches() {
        check(
            ee::s06::f_002598a8__generated,
            lobby_man_on_recv_debug,
            24,
            &|c, seed| {
                let _r = machine(c, seed, 0x2598_a800, 0);
                let lobby = DATA + 0x1000;
                let packet = DATA + 0x2000;
                let text = DATA + 0x3000;
                c.bus
                    .write32(lobby + 0x54, if seed % 3 == 0 { 0 } else { packet });
                c.bus
                    .write32(packet + 4, if seed % 2 == 0 { 0 } else { 0xffff_ffff });
                c.bus.write32(packet + 8, 0);
                c.bus.write32(packet + 0xc, text);
                match seed % 3 {
                    0 => c.bus.write8(text, 0),
                    1 => {
                        c.bus.write8(text, b'A');
                        c.bus.write8(text + 1, 0);
                    }
                    _ => {
                        c.bus.write8(text, 200);
                        c.bus.write8(text + 1, 0);
                    }
                }
                c.g[A0][0] = lobby as u64;
                c.g[A1][0] = packet as u64;
            },
        );
    }

    #[test]
    fn c_voip_man_get_shared_status_matches() {
        check(
            ee::s07::f_00266f38__generated,
            c_voip_man_get_shared_status,
            24,
            &|c, seed| {
                let _r = machine(c, seed, 0x266f_3800, 0);
                let status = DATA + 0x1000;
                c.bus
                    .write32(GAME_GP + 0x2a28, if seed % 3 == 0 { 0 } else { status });
                c.bus.write32(status, 0x4000 + seed);
                c.g[A0][0] = DATA as u64;
            },
        );
    }

    #[test]
    fn c_dsock_channel_base_disconnect_matches() {
        check(
            ee::s07::f_00267da8__generated,
            c_dsock_channel_base_disconnect,
            32,
            &|c, seed| {
                let mut r = machine(c, seed, 0x267d_a800, 0);
                c.g[A0][0] = DATA as u64;
                c.bus
                    .write32(DATA, if seed % 2 == 0 { 0 } else { r.next_u32() });
                c.bus.write32(DATA + 4, r.next_u32());
            },
        );
    }

    #[test]
    fn lobby_man_compare_user_names_matches() {
        check(
            ee::s06::f_00259398__generated,
            lobby_man_compare_user_names,
            32,
            &|c, seed| {
                let mut r = Rng(0x2593_9800 ^ seed as u64);
                scramble(c, &mut r);
                let put = crate::game::dirtysock::tags::tests::put_str;
                let (a, b) = match seed % 5 {
                    0 => (&b"alice"[..], &b"alice"[..]),
                    1 => (&b"alice"[..], &b"bob"[..]),
                    2 => (&b""[..], &b"x"[..]),
                    3 => (&b"Alice"[..], &b"alice"[..]),
                    _ => (&b"bob"[..], &b"bobby"[..]),
                };
                put(c, DATA + 0x100, a);
                put(c, DATA + 0x200, b);
                c.g[A2][0] = (DATA + 0x100 - 0x1c) as u64;
                c.g[A3][0] = (DATA + 0x200 - 0x1c) as u64;
            },
        );
    }

    #[test]
    fn lobby_man_compare_room_names_matches() {
        check(
            ee::s06::f_002593b8__generated,
            lobby_man_compare_room_names,
            32,
            &|c, seed| {
                let mut r = Rng(0x2593_b800 ^ seed as u64);
                scramble(c, &mut r);
                let put = crate::game::dirtysock::tags::tests::put_str;
                let (a, b) = match seed % 5 {
                    0 => (&b"room1"[..], &b"room1"[..]),
                    1 => (&b"room1"[..], &b"room2"[..]),
                    2 => (&b""[..], &b"z"[..]),
                    3 => (&b"Room1"[..], &b"room1"[..]),
                    _ => (&b"lobby"[..], &b"lobb"[..]),
                };
                put(c, DATA + 0x100, a);
                put(c, DATA + 0x200, b);
                c.g[A2][0] = (DATA + 0x100 - 0x8) as u64;
                c.g[A3][0] = (DATA + 0x200 - 0x8) as u64;
            },
        );
    }

    #[test]
    fn lobby_man_send_decline_matches() {
        check(
            ee::s06::f_0025fc28__generated,
            lobby_man_send_decline,
            24,
            &|c, seed| {
                let _r = machine(c, seed, 0x25fc_2800, 0);
                let put = crate::game::dirtysock::tags::tests::put_str;
                let lobby = DATA + 0x4000;
                let packet = DATA + 0x6000;
                c.bus.write32(lobby, (seed % 2) as u32);
                c.bus.write32(lobby + 0x54, 0);
                c.bus.write32(lobby + 0xf0, packet);
                c.bus.write32(packet, 0);
                c.bus.write32(packet + 4, DATA + 0x7000);
                match seed % 3 {
                    0 => put(c, DATA + 0x5000, b"bob"),
                    1 => put(c, DATA + 0x5000, b""),
                    _ => put(c, DATA + 0x5000, b"x"),
                }
                c.g[A0][0] = lobby as u64;
                c.g[A1][0] = (DATA + 0x5000) as u64;
            },
        );
    }

    pub(crate) fn roster_setup(c: &mut Cpu, seed: u32, bit: u32) {
        let man = DATA + 0x1000;
        let list = DATA + 0x2000;
        let array = DATA + 0x2100;
        let entry = DATA + 0x2200;
        match seed % 4 {
            0 => {
                c.g[A1][0] = 0xffff_ffff_ffff_ffff;
                c.bus.write32(man + 0xcc, 0);
            }
            1 => {
                c.g[A1][0] = 0;
                c.bus.write32(man + 0xcc, 0);
            }
            _ => {
                c.g[A1][0] = 0;
                c.bus.write32(man + 0xcc, list);
                c.bus.write32(list + 0x18, 2);
                c.bus.write32(list + 0x40, array);
                c.bus.write32(array, entry);
                c.bus
                    .write32(entry + 0xa8, if seed % 4 == 2 { 0 } else { bit });
            }
        }
        c.g[A0][0] = man as u64;
    }

    #[test]
    fn buddy_man_is_buddy_at_matches() {
        check(
            ee::s07::f_00262390__generated,
            buddy_man_is_buddy_at,
            32,
            &|c, seed| {
                let _r = machine(c, seed, 0x2623_9000, 0);
                roster_setup(c, seed, 0x4);
            },
        );
    }

    #[test]
    fn buddy_man_is_blocked_at_matches() {
        check(
            ee::s07::f_002623e0__generated,
            buddy_man_is_blocked_at,
            32,
            &|c, seed| {
                let _r = machine(c, seed, 0x2623_e000, 0);
                roster_setup(c, seed, 0x200);
            },
        );
    }

    #[test]
    fn buddy_man_get_buddy_at_matches() {
        check(
            ee::s07::f_00262620__generated,
            buddy_man_get_buddy_at,
            32,
            &|c, seed| {
                let _r = machine(c, seed, 0x2626_2000, 0);
                roster_setup(c, seed, 0x4);
            },
        );
    }

    #[test]
    fn buddy_man_get_blocked_at_matches() {
        check(
            ee::s07::f_00262658__generated,
            buddy_man_get_blocked_at,
            32,
            &|c, seed| {
                let _r = machine(c, seed, 0x2626_5800, 0);
                roster_setup(c, seed, 0x200);
            },
        );
    }

    #[test]
    fn lobby_man_select_persona_index_matches() {
        check(
            ee::s06::f_0025cd28__generated,
            lobby_man_select_persona_index,
            24,
            &|c, seed| {
                let _r = machine(c, seed, 0x25cd_2800, 0);
                let put = crate::game::dirtysock::tags::tests::put_str;
                let this = DATA + 0x1000;
                c.bus.write32(this + 0x54, 0);
                c.bus
                    .write32(this + 0x18, if seed % 2 == 0 { 0 } else { 1 });
                c.bus.write32(this + 0xf0, 0);
                put(c, DATA + 0x2000, b"a");
                put(c, DATA + 0x2100, b"bb");
                put(c, DATA + 0x2200, b"ccc");
                c.bus.write32(this + 0x70, DATA + 0x2000);
                c.bus.write32(this + 0x74, DATA + 0x2100);
                c.bus.write32(this + 0x78, DATA + 0x2200);
                c.g[A0][0] = this as u64;
                c.g[A1][0] = (seed % 3) as u64;
            },
        );
    }

    #[test]
    fn lobby_man_send_abort_chal_matches() {
        check(
            ee::s06::f_0025fc48__generated,
            lobby_man_send_abort_chal,
            24,
            &|c, seed| {
                let _r = machine(c, seed, 0x25fc_4800, 0);
                let put = crate::game::dirtysock::tags::tests::put_str;
                let lobby = DATA + 0x4000;
                let packet = DATA + 0x6000;
                c.bus.write32(lobby, (seed % 2) as u32);
                c.bus.write32(lobby + 0x54, 0);
                c.bus.write32(lobby + 0xf0, packet);
                c.bus.write32(packet, 0);
                c.bus.write32(packet + 4, DATA + 0x7000);
                match seed % 3 {
                    0 => {
                        put(c, DATA + 0x5000, b"challenger");
                        c.bus.write32(lobby + 0x98, DATA + 0x5000);
                    }
                    1 => {
                        put(c, DATA + 0x5000, b"");
                        c.bus.write32(lobby + 0x98, DATA + 0x5000);
                    }
                    _ => c.bus.write32(lobby + 0x98, 0),
                }
                c.g[A0][0] = lobby as u64;
            },
        );
    }

    #[test]
    fn c_shared_login_man_ctor_matches() {
        check(
            ee::s07::f_002658e0__generated,
            c_shared_login_man_ctor,
            16,
            &|c, seed| {
                let _r = machine(c, seed, 0x2658_e000, 0);
                c.g[A0][0] = (DATA + 0x1000 + (seed % 2) * 0x100) as u64;
            },
        );
    }

    #[test]
    fn c_usb_keyboard_destroy_matches() {
        check(
            ee::s07::f_002667c0__generated,
            c_usb_keyboard_destroy,
            16,
            &|c, seed| {
                let _r = machine(c, seed, 0x2667_c000, 0);
                if seed % 2 == 0 {
                    c.bus.write32(GAME_GP + 0x300, 0);
                } else {
                    let kb = crate::game::dirtysock::tagsunk::tests::heap_block(c, 0x468);
                    c.bus.write32(kb + 0x460, 0);
                    c.bus.write32(GAME_GP + 0x300, kb);
                }
            },
        );
    }

    #[test]
    fn c_voip_man_query_state_matches() {
        check(
            ee::s07::f_00266f08__generated,
            c_voip_man_query_state,
            16,
            &|c, seed| {
                let _r = machine(c, seed, 0x266f_0800, 0);
                let status = DATA + 0x1000;
                c.bus
                    .write32(GAME_GP + 0x2a28, if seed % 4 == 3 { 0 } else { status });
                c.bus.write32(status, 0x5000 + seed);
                let v = DATA + 0x2000;
                c.bus.write32(v, if seed % 2 == 0 { 0 } else { 1 });
                c.g[A0][0] = v as u64;
            },
        );
    }

    #[test]
    fn lobby_man_cancel_login_matches() {
        check(
            ee::s06::f_0025c878__generated,
            lobby_man_cancel_login,
            24,
            &|c, seed| {
                let _r = machine(c, seed, 0x25c8_7800, 0);
                let lobby = DATA + 0x1000;
                c.bus.write32(lobby, if seed % 3 == 0 { 8 } else { 7 });
                c.bus.write32(lobby + 0x40, 0x1234);
                c.bus.write32(lobby + 0x54, 0);
                c.g[A0][0] = lobby as u64;
            },
        );
    }

    #[test]
    fn lobby_man_create_matches() {
        check(
            ee::s06::f_0025b650__generated,
            lobby_man_create,
            8,
            &|c, seed| {
                let _r = machine(c, seed, 0x25b6_5000, 0);
                crate::game::dirtysock::tagsunk::tests::heap_init(c);
            },
        );
    }

    #[test]
    fn c_shared_login_man_create_matches() {
        check(
            ee::s07::f_00265880__generated,
            c_shared_login_man_create,
            8,
            &|c, seed| {
                let _r = machine(c, seed, 0x2658_8000, 0);
                crate::game::dirtysock::tagsunk::tests::heap_init(c);
            },
        );
    }

    #[test]
    fn c_shared_login_man_mc_seek_matches() {
        check(
            ee::s07::f_002662e8__generated,
            c_shared_login_man_mc_seek,
            24,
            &|c, seed| {
                let _r = machine(c, seed, 0x2662_e800, 0);
                let card = DATA + 0x4000;
                c.bus
                    .write32(GAME_GP + 0x2e8, if seed % 2 == 0 { 0 } else { card });
                c.g[A1][0] = seed as u64;
                c.g[A2][0] = (seed % 3) as u64;
            },
        );
    }

    #[test]
    fn c_usb_keyboard_create_matches() {
        check(
            ee::s07::f_00266788__generated,
            c_usb_keyboard_create,
            8,
            &|c, seed| {
                let _r = machine(c, seed, 0x2667_8800, 0);
                crate::game::dirtysock::tagsunk::tests::heap_init(c);
            },
        );
    }

    #[test]
    fn c_voip_man_toggle_active_matches() {
        check(
            ee::s07::f_00266ed0__generated,
            c_voip_man_toggle_active,
            16,
            &|c, seed| {
                let _r = machine(c, seed, 0x266e_d000, 0);
                let v = DATA + 0x1000;
                c.bus.write32(v, 0);
                c.bus.write32(v + 4, (seed % 2) as u32);
                c.g[A0][0] = v as u64;
            },
        );
    }

    /// Gate and answers of [`net_fake_mem_alloc`].
    pub(crate) const NET_CTRL: u32 = DATA + 0xd_8000;

    pub(crate) const NET_MAGIC: u32 = 0x0026_3828;

    /// `EA_MemAllocNamed` (0x317d70) for the net tests: logs the size, tag
    /// and `ra`, returns the n-th block from `NET_CTRL + 0x10`.
    pub(crate) fn net_fake_mem_alloc(c: &mut Cpu) {
        use crate::difftest::fakes::{fall_back, prev_cell};
        if c.bus.read32(NET_CTRL) != NET_MAGIC {
            return fall_back(
                c,
                prev_cell(net_fake_mem_alloc),
                ee::s10::f_00317d70__generated,
            );
        }
        let n = c.bus.read32(NET_CTRL + 4) & 3;
        c.bus.write32(NET_CTRL + 4, n + 1);
        let log = NET_CTRL + 0x40 + 0x10 * n;
        c.bus.write32(log, c.g[A0][0] as u32);
        c.bus.write32(log + 4, c.g[A1][0] as u32);
        c.bus.write32(log + 8, c.g[A2][0] as u32);
        c.bus.write32(log + 0xc, c.g[RA][0] as u32);
        c.g[V0][0] = c.bus.read32(NET_CTRL + 0x10 + 4 * n) as u64;
        c.cycles += 11;
        ret(c);
    }

    /// Net_UpdateConnect: idle, connecting (the comm system up: the message
    /// post through a fake allocator; or still waiting: the timer counts
    /// down), connected (no failure) and unknown states. Not reached: the
    /// three disconnect paths (comm failure, timeout): the real
    /// Net_DisconnectAll runs the whole lobby shutdown.
    #[test]
    fn net_update_connect_matches() {
        use crate::difftest::fakes::{hook, prev_cell};
        hook(
            std::ptr::addr_of_mut!(ee::s10::OVERRIDE_00317D70),
            net_fake_mem_alloc,
            prev_cell(net_fake_mem_alloc),
        );
        check(
            ee::s07::f_00263828__generated,
            net_update_connect,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x263828, 0);
                c.pc = 0x0026_3828;
                let this = DATA;
                let comm = DATA + 0x100;
                let lobby = DATA + 0x200;
                let head = DATA + 0x300;
                let prev = DATA + 0x400;
                let app = DATA + 0x700;
                c.bus.write32(NET_CTRL, NET_MAGIC);
                c.bus.write32(NET_CTRL + 0x10, DATA + 0x500);
                c.bus.write32(NET_CTRL + 0x14, DATA + 0x600);
                c.g[A0][0] = sx(this);
                let state = [0u32, 1, 1, 2, 3, 0xffff_ffff][(seed % 6) as usize];
                c.bus.write32(this + NET_CONNECT_STATE, state);
                c.bus.write32(
                    this + NET_CONNECT_TIMER,
                    (0.01 + (r.next_u32() % 100) as f32).to_bits(),
                );
                c.bus.write32(GAME_GP - NET_COMM_SYSTEM_GP, comm);
                let comm_state = if seed % 3 == 0 {
                    5
                } else {
                    [0u32, 2, 4, 7][(seed / 3 % 4) as usize]
                };
                c.bus.write32(comm + 0x20, comm_state);
                c.bus.write32(comm + 0x28, r.next_u32());
                c.bus.write32(GAME_GP - NET_LOBBY_MAN_GP, lobby);
                c.bus.write32(lobby + 0xf0, head);
                c.bus.write32(head, r.next_u32());
                c.bus.write32(head + 4, prev);
                c.bus.write32(GAME_GP - 0x848, app);
                c.bus.write32(app + 0x14, r.f32(0.1));
            },
        );
    }
    use crate::difftest::check_paths;
    use crate::difftest::events::check_events;
    use crate::difftest::events::driven;
    use crate::difftest::events::schedule;
    use crate::difftest::events::Events;
    use crate::difftest::fakes::fall_back;
    use crate::difftest::fakes::hook;
    use crate::difftest::fakes::prev_cell;
    use crate::difftest::fakes::LOG_UNWIND_AT;
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Write `s` and a terminating NUL at `at`.
    pub(crate) fn put_cstr(c: &mut Cpu, at: u32, s: &[u8]) {
        for (i, b) in s.iter().enumerate() {
            c.bus.write8(at + i as u32, *b);
        }
        c.bus.write8(at + s.len() as u32, 0);
    }

    /// The engine heap, so `EA_MemAllocNamed`/`MEM_AllocInternal` have an arena.
    pub(crate) fn race_report_heap_init(c: &mut Cpu) {
        const MEM_HEAP: u32 = 0x004f_f250;
        const RET: u64 = 0x0012_3450;
        c.g[A0][0] = MEM_HEAP as u64;
        c.g[A1][0] = (DATA + 0x4_0000) as u64;
        c.g[A2][0] = 0x4_0000;
        c.g[RA][0] = RET;
        c.pc = 0x0031_ff60;
        ee::s10::DLHeap_InitArena(c);
        assert_eq!(c.pc, RET as u32, "DLHeap_InitArena did not return");
    }

    /// The size-class pool allocator's heads and free lists.
    pub(crate) fn race_report_pool(c: &mut Cpu) {
        const POOLS: [(u32, u32); 7] = [
            (0x2a70, 0x800),
            (0x2a60, 0x2000),
            (0x2a6c, 0x6000),
            (0x2a7c, 0x32000),
            (0x2a84, 0x25800),
            (0x2a78, 0x20000),
            (0x2a68, 0x20000),
        ];
        c.bus.write32(GAME_GP.wrapping_add(0xdd8), 1);
        for (i, (slot, head)) in POOLS.into_iter().enumerate() {
            let i = i as u32;
            let pl = DATA + 0x40_0000 + 0x4_0000 * i;
            let base = DATA + 0x80_0000 + 0x4000 * i;
            for n in 0..8u32 {
                let node = base + 0x400 * n;
                c.bus.write32(node, if n == 7 { 0 } else { node + 0x400 });
            }
            c.bus.write32(pl + head, base);
            c.bus.write32(GAME_GP.wrapping_add(slot), pl);
        }
    }

    // Branch steering is all seed arithmetic: `seed % 7` the *report==0 exit,
    // `b = seed/7` residues for auth, the race+0x18/+0x9c flags, the table
    // player order, the +0xa0 player and the zero timers; the +0xa4 player
    // and the +0x3c timer from `seed / 17` and `seed / 19`; finishes seed % 3.
    #[test]
    fn lobby_man_send_race_report_matches() {
        check(
            ee::s06::f_0025fd50__generated,
            lobby_man_send_race_report,
            210,
            &|c, seed| {
                let mut r = machine(c, seed, 0x25_fd50, 0);
                elf_image(c);
                race_report_heap_init(c);
                race_report_pool(c);
                let b = seed / 7;
                // BELibrary dummy, interface table and world.
                c.bus.write32(GAME_GP.wrapping_sub(0x1f04), DATA + 0x3_0000);
                c.bus.write32(GAME_GP.wrapping_sub(0x1efc), 0);
                c.bus.write32(0x0053_08a8, DATA + 0x8000);
                c.bus.write32(DATA + 0x8000 + 4, DATA + 0x8100);
                c.bus.write16(DATA + 0x8100 + 8, 0);
                c.bus.write32(DATA + 0x8100 + 0xc, FAKE);
                let world = DATA + 0x9000;
                c.bus.write32(GAME_GP.wrapping_sub(0x848), world);
                c.bus.write32(world + 0x78, DATA + 0x9100);
                c.bus.write32(world + 0xc0, DATA + 0x9200);
                c.bus
                    .write32(DATA + 0x9200 + 0x9c, if (b / 5) % 2 == 0 { 0 } else { 1 });
                // Race state and the static player-order table.
                let race = DATA + 0x2000;
                c.bus.write32(GAME_GP.wrapping_sub(0x204), race);
                let f18 = (b / 2) % 3;
                c.bus.write32(race + 0x18, if f18 == 0 { 0 } else { 1 });
                c.bus.write32(race + 0x9c, if f18 == 2 { 1 } else { 0 });
                c.bus.write32(
                    race + 0xa0,
                    if (b / 12) % 2 == 0 { 0 } else { DATA + 0x4000 },
                );
                c.bus.write32(
                    race + 0xa4,
                    if (seed / 17) % 2 == 0 {
                        0
                    } else {
                        DATA + 0x4100
                    },
                );
                c.bus
                    .write32(race + 0x34, if (b / 3) % 2 == 0 { 0 } else { 100 });
                c.bus
                    .write32(race + 0x38, if (b / 5) % 2 == 0 { 0 } else { 250 });
                c.bus
                    .write32(race + 0x3c, if (seed / 19) % 2 == 0 { 0 } else { 400 });
                c.bus.write32(race + 0x10, (seed / 11) % 2);
                c.bus.write32(race + 0x14, (seed / 13) % 2);
                for off in [0xcu32, 0x1c, 0x20, 0x24, 0x28, 0x30, 0xb4] {
                    c.bus.write32(race + off, r.next_u32());
                }
                c.bus
                    .write32(0x0053_4b38, if (b / 6) % 2 == 0 { 0 } else { 1 });
                c.bus.write32(0x0053_4b30 + 0x328, r.next_u32());
                c.bus.write32(0x0053_4b30 + 0x32c, r.next_u32());
                // Track id / desync byte and the finish-indexed tables.
                let track = [0u32, 14, 15, 16][((b / 4) % 4) as usize];
                c.bus.write32(0x0053_5c08, track);
                c.bus
                    .write8(0x0053_5c12, if (b / 8) % 2 == 0 { 4 } else { 7 });
                for i in 0..4u32 {
                    c.bus
                        .write32(0x0053_66a8 + 4 * i, if (seed + i) % 2 == 0 { 0 } else { 1 });
                    c.bus.write32(0x0053_6640 + 4 * i, r.next_u32());
                }
                // Report object, names and the lobby list head.
                let report = DATA + 0x1000;
                c.bus
                    .write32(report, if seed % 7 == 0 { 0 } else { DATA + 0x6000 });
                c.bus.write32(report + 0x54, 0);
                c.bus.write32(report + 0xf0, DATA + 0x5000);
                c.bus.write32(DATA + 0x5000 + 4, DATA + 0x5100);
                c.bus.write32(report + 0x90, DATA + 0x3000);
                c.bus.write32(report + 0x94, DATA + 0x3100);
                c.bus.write32(report + 0x9c, DATA + 0x3200);
                c.bus.write32(report + 0xa0, r.next_u32());
                put_cstr(c, DATA + 0x3000, b"rider zero");
                put_cstr(c, DATA + 0x3100, b"rider one");
                put_cstr(c, DATA + 0x3200, b"auth-str");
                // Two rider stat blocks: payload ints, seed-derived edge floats.
                const EDGE: [u32; 10] = [
                    0,
                    0x8000_0000,
                    0x3f80_0000,
                    0xbf80_0000,
                    0x7f80_0000,
                    0x7fc0_0000,
                    0x4b00_0000,
                    0xcf00_0000,
                    0x42c8_0000,
                    0x0000_0001,
                ];
                for (si, stats) in [DATA + 0x7000, DATA + 0x7200].into_iter().enumerate() {
                    for off in RIDER_STATS_INT_OFFS
                        .iter()
                        .chain(RIDER_STATS_TAIL_OFFS.iter())
                    {
                        c.bus.write32(stats + off, r.next_u32());
                    }
                    for (i, &off) in RIDER_STATS_FLOAT_OFFS.iter().enumerate() {
                        c.bus
                            .write32(stats + off, EDGE[(seed as usize + si * 3 + i) % 10]);
                    }
                }
                c.g[A0][0] = sx(report);
                c.g[A1][0] = if b % 2 == 0 { 0 } else { sx(DATA + 0x3300) };
                c.g[A2][0] = (seed % 3) as u64;
                c.g[A3][0] = ((seed / 3) % 3) as u64;
                c.g[T0][0] = sx(DATA + 0x7000);
                c.g[T1][0] = sx(DATA + 0x7200);
                c.pc = 0x0025_fd50;
            },
        );
    }

    #[test]
    fn lobby_man_static_init_matches() {
        check_paths(
            ee::s07::f_00267468__generated,
            lobby_man_static_init,
            300,
            &[23, 25, 583],
            &|c, seed| {
                let mut r = Rng(0x2674_6800 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0026_7468;
                c.g[GP][0] = sx(GAME_GP);
                // Steering from seed arithmetic only: seed % 3 picks the guard outcome.
                match seed % 3 {
                    0 => {
                        c.g[A1][0] = 0x1234 + seed as u64; // != 0xffff: skip
                        c.g[A0][0] = 1;
                    }
                    1 => {
                        c.g[A1][0] = 0xffff;
                        c.g[A0][0] = 0; // zero first arg: skip
                    }
                    _ => {
                        c.g[A1][0] = 0xffff;
                        c.g[A0][0] = 1; // run the full init
                    }
                }
                // Junk in the float-constant pool below gp and in the
                // destination tables, so every load/store is observable.
                fill(c, &mut r, GAME_GP - 0x4b70, 0x100);
                fill(c, &mut r, 0x004d_2c00, 0x100);
                fill(c, &mut r, GAME_GP + 0x1fe8, 0x24);
            },
        );
    }

    pub(crate) const MEM_FAKES_CTRL: u32 = DATA + 0xe_0000;

    pub(crate) const MEM_FAKES_MAGIC: u32 = 0x0c40_2001;

    pub(crate) fn mem_fake_alloc(c: &mut Cpu) {
        if c.bus.read32(MEM_FAKES_CTRL) != MEM_FAKES_MAGIC {
            return fall_back(c, prev_cell(mem_fake_alloc), ee::s10::f_00317d70__generated);
        }
        let n = c.bus.read32(MEM_FAKES_CTRL + 4);
        c.bus.write32(MEM_FAKES_CTRL + 4, n + 1);
        c.g[V0][0] = c.bus.read32(MEM_FAKES_CTRL + 0x10 + 4 * (n & 7)) as u64;
        c.cycles += 11;
        ret(c);
    }

    pub(crate) fn mem_fake_strcpy(c: &mut Cpu) {
        if c.bus.read32(MEM_FAKES_CTRL) != MEM_FAKES_MAGIC {
            return fall_back(
                c,
                prev_cell(mem_fake_strcpy),
                ee::s15::f_004166f4__generated,
            );
        }
        let dst = c.g[A0][0] as u32;
        let src = c.g[A1][0] as u32;
        for i in 0..0x1000u32 {
            let b = c.bus.read8(src.wrapping_add(i));
            c.bus.write8(dst.wrapping_add(i), b);
            if b == 0 {
                break;
            }
        }
        c.g[V0][0] = c.g[A0][0];
        c.cycles += 5;
        ret(c);
    }

    pub(crate) fn mem_fake_delete(c: &mut Cpu) {
        if c.bus.read32(MEM_FAKES_CTRL) != MEM_FAKES_MAGIC {
            return fall_back(
                c,
                prev_cell(mem_fake_delete),
                ee::s10::f_00317e50__generated,
            );
        }
        c.cycles += 3;
        ret(c);
    }

    pub(crate) fn mem_fakes_hook() {
        hook(
            std::ptr::addr_of_mut!(ee::s10::OVERRIDE_00317D70),
            mem_fake_alloc,
            prev_cell(mem_fake_alloc),
        );
        hook(
            std::ptr::addr_of_mut!(ee::s15::OVERRIDE_004166F4),
            mem_fake_strcpy,
            prev_cell(mem_fake_strcpy),
        );
        hook(
            std::ptr::addr_of_mut!(ee::s10::OVERRIDE_00317E50),
            mem_fake_delete,
            prev_cell(mem_fake_delete),
        );
    }

    #[test]
    fn net_event10d_clone_matches() {
        mem_fakes_hook();
        check(
            ee::s07::f_002687c0__generated,
            net_event10d_clone,
            60,
            &|c, seed| {
                let _r = machine(c, seed, 0x2687c0, 0);
                c.pc = 0x0026_87c0;
                c.bus.write32(MEM_FAKES_CTRL, MEM_FAKES_MAGIC);
                c.bus.write32(MEM_FAKES_CTRL + 4, 0);
                c.bus.write32(MEM_FAKES_CTRL + 0x10, DATA + 0x4000);
                let self_ = DATA;
                c.g[A0][0] = self_ as u64;
                for (off, base) in [(0x8u32, b'a'), (0x28, b'm'), (0x48, b'z')] {
                    for i in 0..8u32 {
                        c.bus
                            .write8(self_ + off + i, base + ((seed + i) % 20) as u8);
                    }
                    c.bus.write8(self_ + off + 8, 0);
                }
                c.bus.write32(self_ + 0x148, 0x1111 * seed);
                c.bus.write32(self_ + 0x14c, 0x2222 * seed);
                for i in 0..0x14u32 {
                    c.bus.write8(self_ + 0x150 + i, (seed + i) as u8);
                }
            },
        );
    }

    #[test]
    fn lobby_man_list_clear_matches() {
        mem_fakes_hook();
        check(
            ee::s07::f_002670c8__generated,
            lobby_man_list_clear,
            60,
            &|c, seed| {
                let _r = machine(c, seed, 0x2670c8, 0);
                c.pc = 0x0026_70c8;
                c.bus.write32(MEM_FAKES_CTRL, MEM_FAKES_MAGIC);
                let self_ = DATA;
                let head = DATA + 0x100;
                let n1 = DATA + 0x200;
                let n2 = DATA + 0x300;
                c.g[A0][0] = self_ as u64;
                c.bus.write32(self_ + LOBBY_LIST_HEAD, head);
                match seed % 3 {
                    0 => {
                        c.bus.write32(head, head);
                        c.bus.write32(head + 4, head);
                    }
                    1 => {
                        c.bus.write32(head, n1);
                        c.bus.write32(head + 4, n1);
                        c.bus.write32(n1, head);
                        c.bus.write32(n1 + 4, head);
                    }
                    _ => {
                        c.bus.write32(head, n1);
                        c.bus.write32(head + 4, n2);
                        c.bus.write32(n1, n2);
                        c.bus.write32(n1 + 4, head);
                        c.bus.write32(n2, head);
                        c.bus.write32(n2 + 4, n1);
                    }
                }
            },
        );
    }

    /// Four nodes, so the loop head 0x2670f0 meets the due event.
    #[test]
    fn lobby_man_list_clear_events() {
        mem_fakes_hook();
        driven!(gen, ee::s07::f_002670c8__generated);
        driven!(hand, lobby_man_list_clear);
        check_events(gen, hand, 60, &|c, seed| {
            let mut r = machine(c, seed, 0x2670c9, 0);
            c.bus.write32(MEM_FAKES_CTRL, MEM_FAKES_MAGIC);
            let self_ = DATA;
            let head = DATA + 0x100;
            c.g[A0][0] = self_ as u64;
            c.bus.write32(self_ + LOBBY_LIST_HEAD, head);
            let nodes = [head, DATA + 0x200, DATA + 0x300, DATA + 0x400, DATA + 0x500];
            for (i, &n) in nodes.iter().enumerate() {
                c.bus.write32(n, nodes[(i + 1) % 5]);
                c.bus.write32(n + 4, nodes[(i + 4) % 5]);
            }
            schedule(c, &mut r, seed, 0x0026_70c8)
        });
    }

    #[test]
    fn c_voip_man_debug_printf_stub_matches() {
        check(
            ee::s07::f_00266fc0__generated,
            c_voip_man_debug_printf_stub,
            30,
            &|c, seed| {
                let mut r = Rng(0x266fc0 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x0026_6fc0;
                c.g[A0][0] = (DATA + 0x100 * (seed % 8)) as u64;
            },
        );
    }

    #[test]
    fn c_usb_keyboard_dtor_matches() {
        mem_fakes_hook();
        check(
            ee::s07::f_00266850__generated,
            c_usb_keyboard_dtor,
            80,
            &|c, seed| {
                let _r = machine(c, seed, 0x266850, 0);
                c.pc = 0x0026_6850;
                c.bus.write32(MEM_FAKES_CTRL, MEM_FAKES_MAGIC);
                let self_ = DATA + 0x1000;
                c.g[A0][0] = self_ as u64;
                c.g[A1][0] = (seed % 4) as u64;
                // `seed / 4 % 2`: opened, so `UsbKb_Shutdown` runs; its two
                // module ids are -1 (nothing loaded to unload).
                let opened = seed / 4 % 2 == 1;
                c.bus.write32(self_ + usb_kb::ACTIVE, opened as u32);
                c.bus.write32(0x0052_5f80, u32::MAX);
                c.bus.write32(0x0052_5f84, u32::MAX);
            },
        );
    }

    pub(crate) fn mem_fake_new(c: &mut Cpu) {
        if c.bus.read32(MEM_FAKES_CTRL) != MEM_FAKES_MAGIC {
            return fall_back(c, prev_cell(mem_fake_new), ee::s10::f_00317e30__generated);
        }
        c.g[V0][0] = c.bus.read32(MEM_FAKES_CTRL + 0x18) as u64;
        c.cycles += 7;
        ret(c);
    }

    pub(crate) fn mem_fake_free(c: &mut Cpu) {
        if c.bus.read32(MEM_FAKES_CTRL) != MEM_FAKES_MAGIC {
            return fall_back(c, prev_cell(mem_fake_free), ee::s10::f_00317e98__generated);
        }
        c.cycles += 3;
        ret(c);
    }

    pub(crate) fn mem_fakes_hook_with_new() {
        mem_fakes_hook();
        hook(
            std::ptr::addr_of_mut!(ee::s10::OVERRIDE_00317E30),
            mem_fake_new,
            prev_cell(mem_fake_new),
        );
        hook(
            std::ptr::addr_of_mut!(ee::s10::OVERRIDE_00317E98),
            mem_fake_free,
            prev_cell(mem_fake_free),
        );
    }

    pub(crate) fn buddy_vec_fixture(c: &mut Cpu, seed: u32, entry: u32) {
        let _r = machine(c, seed, entry as u64, 0);
        c.pc = entry;
        c.bus.write32(MEM_FAKES_CTRL, MEM_FAKES_MAGIC);
        c.bus.write32(MEM_FAKES_CTRL + 4, 0);
        c.bus.write32(MEM_FAKES_CTRL + 0x10, DATA + 0x4000);
        c.bus.write32(MEM_FAKES_CTRL + 0x18, DATA + 0x5000);
        let self_ = DATA;
        let buf = DATA + 0x1000;
        let xref = DATA + 0x2000;
        c.bus.write32(xref, DATA + 0x3000);
        c.g[A0][0] = self_ as u64;
        c.g[A2][0] = xref as u64;
        for i in 0..8u32 {
            c.bus.write32(buf + 4 * i, 0x1000 + i);
        }
        match seed % 6 {
            0 => {
                // not full, insert mid (3 elements; tail copy is 4 bytes)
                c.bus.write32(self_ + BUDDY_VEC_FIRST, buf);
                c.bus.write32(self_ + BUDDY_VEC_LAST, buf + 12);
                c.bus.write32(self_ + BUDDY_VEC_CAP, buf + 16);
                c.g[A1][0] = (buf + 4) as u64;
            }
            1 => {
                // not full, insert at the beginning (1 element; tail copy 0)
                c.bus.write32(self_ + BUDDY_VEC_FIRST, buf);
                c.bus.write32(self_ + BUDDY_VEC_LAST, buf + 4);
                c.bus.write32(self_ + BUDDY_VEC_CAP, buf + 16);
                c.g[A1][0] = buf as u64;
            }
            2 => {
                // full, count 1 -> operator-new grow, delete-old
                c.bus.write32(self_ + BUDDY_VEC_FIRST, buf);
                c.bus.write32(self_ + BUDDY_VEC_LAST, buf + 4);
                c.bus.write32(self_ + BUDDY_VEC_CAP, buf + 4);
                c.g[A1][0] = buf as u64;
            }
            3 => {
                // full, count 2 -> operator-new grow, free-old
                c.bus.write32(self_ + BUDDY_VEC_FIRST, buf);
                c.bus.write32(self_ + BUDDY_VEC_LAST, buf + 8);
                c.bus.write32(self_ + BUDDY_VEC_CAP, buf + 8);
                c.g[A1][0] = (buf + 4) as u64;
            }
            4 => {
                // full, count 0 (non-null) -> named-alloc grow, free-old
                c.bus.write32(self_ + BUDDY_VEC_FIRST, buf);
                c.bus.write32(self_ + BUDDY_VEC_LAST, buf);
                c.bus.write32(self_ + BUDDY_VEC_CAP, buf);
                c.g[A1][0] = buf as u64;
            }
            _ => {
                // full, empty null vector -> named-alloc grow, no free
                c.bus.write32(self_ + BUDDY_VEC_FIRST, 0);
                c.bus.write32(self_ + BUDDY_VEC_LAST, 0);
                c.bus.write32(self_ + BUDDY_VEC_CAP, 0);
                c.g[A1][0] = 0;
            }
        }
    }

    #[test]
    fn buddy_man_message_vector_insert_matches() {
        mem_fakes_hook_with_new();
        check(
            ee::s07::f_00267128__generated,
            buddy_man_message_vector_insert,
            60,
            &|c, seed| {
                buddy_vec_fixture(c, seed, 0x267128);
            },
        );
    }

    #[test]
    fn buddy_man_challenger_vector_insert_matches() {
        mem_fakes_hook_with_new();
        check(
            ee::s07::f_002672c8__generated,
            buddy_man_challenger_vector_insert,
            60,
            &|c, seed| {
                buddy_vec_fixture(c, seed, 0x2672c8);
            },
        );
    }

    /// McMapError's operation word (`gp+MC_MODE_GP`) and error code (`a0`);
    /// its four jump tables (0x480fe0, 0x481020, 0x481040, 0x481060) come from
    /// the game image.
    pub(crate) fn mc_map_fixture(c: &mut Cpu, seed: u32, mode: u32, err: i32) {
        let _r = machine(c, seed, 0x266578, 0);
        elf_image(c);
        c.pc = 0x0026_6578;
        c.bus.write32(GAME_GP.wrapping_add(MC_MODE_GP as u32), mode);
        c.g[A0][0] = err as i64 as u64;
    }

    /// Mode 0-15 (0 and 15 leave the table's range) crossed with errors -8..2:
    /// every arm of the four tables, every return and every back edge.
    #[test]
    fn c_shared_login_man_mc_map_error_matches() {
        // Every cycle total the cross produces (the arms share some).
        const PATHS: &[u64] = &[8, 15, 17, 19, 20, 21, 22, 23, 24, 25];
        check_paths(
            ee::s07::f_00266578__generated,
            c_shared_login_man_mc_map_error,
            176,
            PATHS,
            &|c, seed| {
                mc_map_fixture(c, seed, seed % 16, (seed / 16 % 11) as i32 - 8);
            },
        );
    }

    /// One `(mode, err)` pair per resume head, so every event mode meets
    /// every head.
    #[test]
    fn c_shared_login_man_mc_map_error_events() {
        const HEADS: [(u32, i32); 7] = [
            (8, -4),  // 0x2665c8
            (8, 0),   // 0x2665dc
            (7, 0),   // 0x2665b8
            (6, 0),   // 0x2665b4
            (6, -5),  // 0x26660c
            (11, -3), // 0x266658
            (12, 0),  // 0x266660
        ];
        driven!(gen, ee::s07::f_00266578__generated);
        driven!(hand, c_shared_login_man_mc_map_error);
        check_events(gen, hand, 420, &|c, seed| {
            let (mode, err) = HEADS[(seed / 3 % 7) as usize];
            mc_map_fixture(c, seed, mode, err);
            let mut r = Rng(0x266578 ^ seed as u64);
            schedule(c, &mut r, seed, 0x0026_6578)
        });
    }

    /// `RiderIface_HasVisited` (0x1406c0): returns 0 in 2 cycles.
    pub(crate) const RIDER_IFACE_HAS_VISITED: u32 = 0x0014_06c0;
    /// `SLoginMc_IsAvailable` (0x3fb278): returns 1 in 2 cycles.
    pub(crate) const SLOGIN_MC_IS_AVAILABLE: u32 = 0x003f_b278;

    /// A shared-login card object at `gp+MC_CARD_GP` (port 0) with its
    /// vtable; `slots` gives `(slot, function)` pairs (delta 0). Returns the
    /// card. The game image supplies the jump tables.
    pub(crate) fn shared_login_card(c: &mut Cpu, slots: &[(u32, u32)]) -> u32 {
        elf_image(c);
        let card = DATA + 0x1000;
        let vt = DATA + 0x2000;
        c.bus.write32(GAME_GP.wrapping_add(MC_CARD_GP as u32), card);
        c.bus.write32(card, vt);
        c.bus.write32(card + 0xc, 0);
        for &(slot, f) in slots {
            c.bus.write16(vt + slot, 0);
            c.bus.write32(vt + slot + 4, f);
        }
        card
    }

    /// McPoll's operation words: 10, 3, 15 and 16 end before the card is
    /// asked; 20 is past the table; the rest index the table at 0x480fa0
    /// (1 retry, 2 directory scan, 5/6/14 saved result, 9 vtable slot 0xa0,
    /// 0/4/11 nothing).
    pub(crate) const MC_POLL_MODES: [u32; 14] = [10, 3, 15, 16, 20, 0, 1, 2, 4, 5, 6, 9, 11, 14];

    /// `cSharedLoginMan_McPoll`: mode `MC_POLL_MODES[seed % 14]`; by
    /// `seed / 14 % 5`: 0 the card answers (slot 0x58 is a fake, non-zero),
    /// 1 port error -3, 2 port error word 1 (reported as -10), 3 no error with
    /// no retries and no directory entries, 4 no error with two retries and
    /// three entries.
    pub(crate) fn mc_poll_fixture(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x266358, 0);
        c.pc = 0x0026_6358;
        let class = seed / 14 % 5;
        let answer = if class == 0 {
            FAKE
        } else {
            RIDER_IFACE_HAS_VISITED
        };
        let card = shared_login_card(c, &[(0x58, answer), (0xa0, FAKE + 0x10)]);
        let error: i32 = match class {
            1 => -3,
            2 => 1,
            _ => 0,
        };
        c.bus.write32(card + 0x184, error as u32);
        let more = class == 4;
        c.bus.write32(card + 0x54, if more { 3 } else { 0 });
        c.bus.write32(
            GAME_GP.wrapping_add(MC_RETRIES_GP as u32),
            if more { 2 } else { 0 },
        );
        c.bus.write32(
            GAME_GP.wrapping_add(MC_SAVED_RESULT_GP as u32),
            r.next_u32(),
        );
        c.bus.write32(
            GAME_GP.wrapping_add(MC_MODE_GP as u32),
            MC_POLL_MODES[(seed % 14) as usize],
        );
        c.g[A0][0] = DATA as u64;
        c.g[A1][0] = (DATA + 0x800) as u64;
    }

    #[test]
    fn c_shared_login_man_mc_poll_matches() {
        check(
            ee::s07::f_00266358__generated,
            c_shared_login_man_mc_poll,
            70,
            &|c, seed| mc_poll_fixture(c, seed),
        );
    }

    /// `cSharedLoginMan_Update`: state `seed % 24` through the game's table at
    /// 0x480f40 (23 is out of range); by `seed / 24 % 2` the card is idle
    /// (slot 0x58 returns 0, so the stage runs) or busy (returns 1); by
    /// `seed / 48 % 2` the SLogin object has finished (`+0xd0 == 4`, so
    /// `SLogin_Poll` returns 0 with status 0) or runs its callback (a fake,
    /// non-zero), and `gp+0x850` is 0 or 1. The card's other slots (0x18,
    /// 0x28, 0x40) return 0.
    pub(crate) fn shared_login_update_fixture(c: &mut Cpu, seed: u32) {
        let _r = machine(c, seed, 0x265a68, 0);
        c.pc = 0x0026_5a68;
        let busy = if seed / 24 % 2 == 1 {
            SLOGIN_MC_IS_AVAILABLE
        } else {
            RIDER_IFACE_HAS_VISITED
        };
        shared_login_card(
            c,
            &[
                (0x18, RIDER_IFACE_HAS_VISITED),
                (0x28, RIDER_IFACE_HAS_VISITED),
                (0x40, RIDER_IFACE_HAS_VISITED),
                (0x58, busy),
            ],
        );
        let finished = seed / 48 % 2 == 0;
        let slogin = DATA + 0x3000;
        c.bus.write32(GAME_GP.wrapping_add(0x1ad4), slogin);
        c.bus.write32(slogin + 0xd0, if finished { 4 } else { 0 });
        c.bus.write32(slogin + 0xd4, FAKE + 0x20);
        c.bus.write32(
            GAME_GP.wrapping_add(MC_SAVED_RESULT_GP as u32),
            seed / 48 % 2,
        );
        let self_ = DATA;
        c.g[A0][0] = self_ as u64;
        c.bus.write32(self_, seed % 24);
        c.bus.write32(self_ + 4, 0);
    }

    #[test]
    fn c_shared_login_man_update_matches() {
        // Every cycle total the fixture produces: each table target with both card answers.
        const PATHS: &[u64] = &[
            20, 27, 36, 41, 42, 45, 47, 48, 54, 74, 128, 133, 147, 156, 187, 201,
        ];
        check_paths(
            ee::s07::f_00265a68__generated,
            c_shared_login_man_update,
            96,
            PATHS,
            &|c, seed| shared_login_update_fixture(c, seed),
        );
    }

    #[test]
    fn c_setup_man_send_quit_matches() {
        mem_fakes_hook();
        check(
            ee::s07::f_00265768__generated,
            c_setup_man_send_quit,
            60,
            &|c, seed| {
                let _r = machine(c, seed, 0x265768, 0);
                c.pc = 0x0026_5768;
                c.bus.write32(MEM_FAKES_CTRL, MEM_FAKES_MAGIC);
                c.bus.write32(MEM_FAKES_CTRL + 4, 0);
                c.bus.write32(MEM_FAKES_CTRL + 0x10, DATA + 0x800);
                c.bus.write32(MEM_FAKES_CTRL + 0x14, DATA + 0x900);
                c.bus.write32(GAME_GP - 0x1f04, DATA + 0x9000);
                c.bus.write32(GAME_GP - 0x848, DATA + 0x9100);
                let self_ = DATA;
                c.g[A0][0] = self_ as u64;
                let lobby = DATA + 0x500;
                c.bus.write32(GAME_GP - 0xc8, lobby);
                c.bus.write32(lobby, 0);
                let head = DATA + 0x600;
                c.bus.write32(lobby + 0xf0, head);
                c.bus.write32(head + 4, DATA + 0x700);
                c.bus.write32(self_ + 0x90, 0);
                c.bus.write32(self_ + 0x94, 0);
                c.bus.write32(self_ + 0x120, 0);
                let comm = DATA + 0xa00;
                c.bus.write32(GAME_GP - 0x238, comm);
                c.bus.write32(comm, DATA + 0xa100);
                c.bus.write32(DATA + 0xa100 + 8, DATA + 0xa200);
                c.bus.write16(DATA + 0xa200 + 0x40, 0);
                c.bus.write32(DATA + 0xa200 + 0x44, FAKE);
                match seed % 3 {
                    0 => {
                        c.bus.write32(self_, 10);
                        c.g[A1][0] = 0x42;
                    }
                    1 => {
                        c.bus.write32(self_, 5);
                        c.g[A1][0] = 0x42;
                    }
                    _ => {
                        c.bus.write32(self_, 5);
                        c.g[A1][0] = SETUP_QUIT_ID as u64;
                    }
                }
            },
        );
    }

    /// Gate and steering words of the `cSetupMan_Update` fakes.
    pub(crate) const SETUP_MAN_UPDATE_CTRL: u32 = DATA + 0xf_0000;

    pub(crate) const SETUP_MAN_UPDATE_MAGIC: u32 = 0x0c40_32b8;

    pub(crate) const SETUP_MAN_UPDATE_POP_PRESENT: u32 = SETUP_MAN_UPDATE_CTRL + 0x04;

    pub(crate) const SETUP_MAN_UPDATE_MSG: u32 = SETUP_MAN_UPDATE_CTRL + 0x08;

    pub(crate) const SETUP_MAN_UPDATE_HAS: u32 = SETUP_MAN_UPDATE_CTRL + 0x18;

    /// `BuddyMan_IsConnected` answers, one bit per call.
    pub(crate) const SETUP_MAN_UPDATE_CONNECTED: u32 = SETUP_MAN_UPDATE_CTRL + 0x1c;

    pub(crate) const SETUP_MAN_UPDATE_CONNECTED_CALLS: u32 = SETUP_MAN_UPDATE_CTRL + 0x20;

    /// What `BuddyMan_Create` stores at `gp+BUDDY_MAN_GP` (0: it fails).
    pub(crate) const SETUP_MAN_UPDATE_CREATED: u32 = SETUP_MAN_UPDATE_CTRL + 0x30;

    pub(crate) const SETUP_MAN_UPDATE_UNWIND_AT: u32 = SETUP_MAN_UPDATE_CTRL + 0x24;

    pub(crate) const SETUP_MAN_UPDATE_FAKE_COUNT: u32 = SETUP_MAN_UPDATE_CTRL + 0x28;

    pub(crate) const SETUP_MAN_UPDATE_ALLOC_COUNT: u32 = SETUP_MAN_UPDATE_CTRL + 0x2c;

    /// Count one fake call; true (with `pc` parked) when this call unwinds.
    pub(crate) fn setup_man_update_fake_step(c: &mut Cpu) -> bool {
        let n = c.bus.read32(SETUP_MAN_UPDATE_FAKE_COUNT);
        c.bus.write32(SETUP_MAN_UPDATE_FAKE_COUNT, n + 1);
        if n == c.bus.read32(SETUP_MAN_UPDATE_UNWIND_AT) {
            c.pc = 0xdead_0000;
            return true;
        }
        false
    }

    /// The next fake allocation block (distinct, writable, non-zero).
    pub(crate) fn setup_man_update_fake_block(c: &mut Cpu) -> u64 {
        let n = c.bus.read32(SETUP_MAN_UPDATE_ALLOC_COUNT);
        c.bus.write32(SETUP_MAN_UPDATE_ALLOC_COUNT, n + 1);
        (DATA + 0x4_0000 + (n & 7) * 0x2000) as u64
    }

    macro_rules! setup_man_update_plain_fake {
        ($name:ident, $gen:path) => {
            fn $name(c: &mut Cpu) {
                if c.bus.read32(SETUP_MAN_UPDATE_CTRL) != SETUP_MAN_UPDATE_MAGIC {
                    return fall_back(c, prev_cell($name), $gen);
                }
                if setup_man_update_fake_step(c) {
                    return;
                }
                c.cycles += 7;
                ret(c);
            }
        };
    }

    setup_man_update_plain_fake!(setup_man_update_fake_flush, ee::s07::f_00264b20__generated);

    setup_man_update_plain_fake!(setup_man_update_fake_delete, ee::s10::f_00317e50__generated);

    pub(crate) fn setup_man_update_fake_pop(c: &mut Cpu) {
        if c.bus.read32(SETUP_MAN_UPDATE_CTRL) != SETUP_MAN_UPDATE_MAGIC {
            return fall_back(
                c,
                prev_cell(setup_man_update_fake_pop),
                ee::s07::f_002649b0__generated,
            );
        }
        if setup_man_update_fake_step(c) {
            return;
        }
        c.cycles += 7;
        if c.bus.read32(SETUP_MAN_UPDATE_POP_PRESENT) == 0 {
            c.g[V0][0] = 0;
        } else {
            let dst = c.g[A1][0] as u32;
            for i in 0..5u32 {
                let w = c.bus.read32(SETUP_MAN_UPDATE_MSG + 4 * i);
                c.bus.write32(dst + 4 * i, w);
            }
            c.g[V0][0] = 0x14;
        }
        ret(c);
    }

    pub(crate) fn setup_man_update_fake_alloc_packet(c: &mut Cpu) {
        if c.bus.read32(SETUP_MAN_UPDATE_CTRL) != SETUP_MAN_UPDATE_MAGIC {
            return fall_back(
                c,
                prev_cell(setup_man_update_fake_alloc_packet),
                ee::s07::f_00264960__generated,
            );
        }
        if setup_man_update_fake_step(c) {
            return;
        }
        c.cycles += 7;
        c.g[V0][0] = setup_man_update_fake_block(c);
        ret(c);
    }

    pub(crate) fn setup_man_update_fake_mem_alloc(c: &mut Cpu) {
        if c.bus.read32(SETUP_MAN_UPDATE_CTRL) != SETUP_MAN_UPDATE_MAGIC {
            return fall_back(
                c,
                prev_cell(setup_man_update_fake_mem_alloc),
                ee::s10::f_00317d70__generated,
            );
        }
        if setup_man_update_fake_step(c) {
            return;
        }
        c.cycles += 7;
        c.g[V0][0] = setup_man_update_fake_block(c);
        ret(c);
    }

    pub(crate) fn setup_man_update_fake_has_challenger(c: &mut Cpu) {
        if c.bus.read32(SETUP_MAN_UPDATE_CTRL) != SETUP_MAN_UPDATE_MAGIC {
            return fall_back(
                c,
                prev_cell(setup_man_update_fake_has_challenger),
                ee::s07::f_00263128__generated,
            );
        }
        if setup_man_update_fake_step(c) {
            return;
        }
        c.cycles += 7;
        c.g[V0][0] = c.bus.read32(SETUP_MAN_UPDATE_HAS) as u64;
        ret(c);
    }

    pub(crate) fn setup_man_update_fake_buddy_connected(c: &mut Cpu) {
        if c.bus.read32(SETUP_MAN_UPDATE_CTRL) != SETUP_MAN_UPDATE_MAGIC {
            return fall_back(
                c,
                prev_cell(setup_man_update_fake_buddy_connected),
                ee::s06::f_00261460__generated,
            );
        }
        if setup_man_update_fake_step(c) {
            return;
        }
        c.cycles += 7;
        // One answer bit per call, in call order.
        let k = c.bus.read32(SETUP_MAN_UPDATE_CONNECTED_CALLS);
        c.bus.write32(SETUP_MAN_UPDATE_CONNECTED_CALLS, k + 1);
        c.g[V0][0] = (c.bus.read32(SETUP_MAN_UPDATE_CONNECTED) >> k & 1) as u64;
        ret(c);
    }

    /// `BuddyMan_Create`: stores the buddy manager at `gp+BUDDY_MAN_GP` when
    /// the steering word says it succeeds.
    pub(crate) fn setup_man_update_fake_buddy_create(c: &mut Cpu) {
        if c.bus.read32(SETUP_MAN_UPDATE_CTRL) != SETUP_MAN_UPDATE_MAGIC {
            return fall_back(
                c,
                prev_cell(setup_man_update_fake_buddy_create),
                ee::s06::f_00260f80__generated,
            );
        }
        if setup_man_update_fake_step(c) {
            return;
        }
        c.cycles += 7;
        let buddy = c.bus.read32(SETUP_MAN_UPDATE_CREATED);
        c.bus
            .write32(GAME_GP.wrapping_add(BUDDY_MAN_GP as u32), buddy);
        ret(c);
    }

    setup_man_update_plain_fake!(
        setup_man_update_fake_buddy_reconnect,
        ee::s06::f_002613c8__generated
    );

    setup_man_update_plain_fake!(
        setup_man_update_fake_lobby_disconnect,
        ee::s06::f_0025b800__generated
    );

    /// Hooks the `cSetupMan_Update` fakes (gated by `SETUP_MAN_UPDATE_CTRL`).
    pub(crate) fn setup_man_update_hooks() {
        hook(
            std::ptr::addr_of_mut!(ee::s07::OVERRIDE_002649B0),
            setup_man_update_fake_pop,
            prev_cell(setup_man_update_fake_pop),
        );
        hook(
            std::ptr::addr_of_mut!(ee::s07::OVERRIDE_00264B20),
            setup_man_update_fake_flush,
            prev_cell(setup_man_update_fake_flush),
        );
        hook(
            std::ptr::addr_of_mut!(ee::s07::OVERRIDE_00264960),
            setup_man_update_fake_alloc_packet,
            prev_cell(setup_man_update_fake_alloc_packet),
        );
        hook(
            std::ptr::addr_of_mut!(ee::s10::OVERRIDE_00317D70),
            setup_man_update_fake_mem_alloc,
            prev_cell(setup_man_update_fake_mem_alloc),
        );
        hook(
            std::ptr::addr_of_mut!(ee::s10::OVERRIDE_00317E50),
            setup_man_update_fake_delete,
            prev_cell(setup_man_update_fake_delete),
        );
        hook(
            std::ptr::addr_of_mut!(ee::s07::OVERRIDE_00263128),
            setup_man_update_fake_has_challenger,
            prev_cell(setup_man_update_fake_has_challenger),
        );
        hook(
            std::ptr::addr_of_mut!(ee::s06::OVERRIDE_00261460),
            setup_man_update_fake_buddy_connected,
            prev_cell(setup_man_update_fake_buddy_connected),
        );
        hook(
            std::ptr::addr_of_mut!(ee::s06::OVERRIDE_00260F80),
            setup_man_update_fake_buddy_create,
            prev_cell(setup_man_update_fake_buddy_create),
        );
        hook(
            std::ptr::addr_of_mut!(ee::s06::OVERRIDE_002613C8),
            setup_man_update_fake_buddy_reconnect,
            prev_cell(setup_man_update_fake_buddy_reconnect),
        );
        hook(
            std::ptr::addr_of_mut!(ee::s06::OVERRIDE_0025B800),
            setup_man_update_fake_lobby_disconnect,
            prev_cell(setup_man_update_fake_lobby_disconnect),
        );
    }

    /// Packet types `cSetupMan_Update` dispatches (and 0x77, which no table
    /// lists).
    pub(crate) const SETUP_MAN_UPDATE_TYPES: [u32; 19] = [
        0xf, 0x10, 1, 3, 5, 0xf8, 0xfa, 7, 9, 0xb, 0xd, 0, 2, 4, 6, 8, 0xa, 0xc, 0x77,
    ];

    /// The setup manager at `DATA` in `state` with the host and stats flags,
    /// a queued packet of `kind` (`None`: the queue is empty), the fakes'
    /// answers (`has`: HasChallenger, `connected`: IsConnected bits per
    /// call, `buddy`: the buddy manager at `gp+BUDDY_MAN_GP`, `created`: what
    /// Create stores there) and the fake call that unwinds (`u32::MAX`:
    /// none).
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn setup_man_update_fixture(
        c: &mut Cpu,
        seed: u32,
        state: u32,
        host: bool,
        stats: bool,
        kind: Option<u32>,
        has: u32,
        connected: u32,
        buddy: bool,
        unwind_at: u32,
    ) {
        // Real callees on the packet paths allocate: give both sides the
        // pool allocator and a DLHeap arena.
        let _r = crate::game::dirtysock::tagsunk::tests::alloc_machine(c, seed, 0x2642_b8);
        c.pc = 0x0026_42b8;
        let this = DATA;
        c.g[A0][0] = sx(this);
        c.bus.write32(this, state);
        c.bus.write32(this + SETUP_HOST_FLAG, host as u32);
        c.bus.write32(this + SETUP_STATS_FLAG, stats as u32);
        c.bus.write32(this + SETUP_SEND_TAIL, 0);
        c.bus.write32(this + SETUP_SEND_HEAD, 0);
        c.bus.write32(this + SETUP_CHANNEL, 0);
        c.bus.write32(SETUP_MAN_UPDATE_CTRL, SETUP_MAN_UPDATE_MAGIC);
        c.bus
            .write32(SETUP_MAN_UPDATE_POP_PRESENT, kind.is_some() as u32);
        c.bus.write32(SETUP_MAN_UPDATE_MSG, kind.unwrap_or(0));
        c.bus.write32(SETUP_MAN_UPDATE_MSG + 4, seed % 9);
        c.bus.write32(SETUP_MAN_UPDATE_MSG + 8, seed % 5);
        c.bus.write32(SETUP_MAN_UPDATE_MSG + 0xc, seed % 7);
        c.bus.write32(SETUP_MAN_UPDATE_MSG + 0x10, seed % 3);
        c.bus.write32(SETUP_MAN_UPDATE_HAS, has);
        c.bus.write32(SETUP_MAN_UPDATE_CONNECTED, connected);
        c.bus.write32(SETUP_MAN_UPDATE_CONNECTED_CALLS, 0);
        c.bus.write32(SETUP_MAN_UPDATE_UNWIND_AT, unwind_at);
        c.bus.write32(SETUP_MAN_UPDATE_FAKE_COUNT, 0);
        c.bus.write32(SETUP_MAN_UPDATE_ALLOC_COUNT, 0);
        // Globals: comm system (gp-0x238), buddy manager (gp+0x238), lobby
        // (gp-0xc8), app (gp-0x848).
        let comm = DATA + 0x1000;
        let lobby = DATA + 0x2000;
        let app = DATA + 0x3000;
        let buddy_man = DATA + 0x1800;
        c.bus.write32(SETUP_MAN_UPDATE_CREATED, buddy_man);
        c.bus.write32(
            GAME_GP.wrapping_add(BUDDY_MAN_GP as u32),
            if buddy { buddy_man } else { 0 },
        );
        c.bus.write32(GAME_GP - NET_COMM_SYSTEM_GP, comm);
        c.bus
            .write32(GAME_GP.wrapping_add(LOBBY_INSTANCE_GP as u32), lobby);
        c.bus.write32(GAME_GP - 0x848, app);
        c.bus.write32(comm, DATA + 0x1100);
        c.bus.write32(DATA + 0x1100 + 8, DATA + 0x1200);
        c.bus.write16(DATA + 0x1200 + 0x38, 0);
        c.bus.write32(DATA + 0x1200 + 0x3c, RIDER_IFACE_HAS_VISITED);
        c.bus.write16(DATA + 0x1200 + 0x40, 0);
        c.bus.write32(DATA + 0x1200 + 0x44, RIDER_IFACE_HAS_VISITED);
        c.bus.write32(lobby, 0);
        c.bus.write32(lobby + 0x98, DATA + 0x900);
        c.bus.write32(lobby + 0xbc, DATA + 0x200);
        c.bus.write32(lobby + 0xc0, DATA + 0x200);
        c.bus.write32(lobby + SETUP_LIST_HEAD, DATA + 0x2800);
        c.bus.write32(app + 0x78, DATA + 0x3100);
        // BELibrary dummy (GetInstance reads gp-0x1f04).
        c.bus.write32(GAME_GP - 0x1f04, DATA + 0x3200);
        c.bus.write32(DATA + 0x3200 + 4, DATA + 0x3300);
        c.bus.write16(DATA + 0x3300 + 8, 0);
        c.bus.write32(DATA + 0x3300 + 0xc, RIDER_IFACE_HAS_VISITED);
    }

    /// `cSetupMan_Update`, every packet type (and an empty queue) crossed
    /// with the role, the state (0, 8, and the handshake states 5 and 10) and
    /// the stats flag: `seed % 20` the type (19: no packet), `seed / 20 % 2`
    /// the host flag, `seed / 40 % 4` the state, `seed / 160 % 2` the stats
    /// flag. Seeds 320-639 repeat the cross with one fake call unwinding
    /// (index `seed / 7 % 8`). Real callees: the packet handlers (they
    /// cannot unwind here) and the channel vtable leaf; faked: PopPacket,
    /// FlushSendQueue, AllocPacket, MemAlloc, operator delete and the
    /// buddy/lobby calls of the challenger drop.
    #[test]
    fn c_setup_man_update_matches() {
        setup_man_update_hooks();
        // Every cycle total of the unwind-free half (seeds 0-319): the handlers of all four chains.
        const PATHS: &[u64] = &[
            102, 106, 108, 110, 112, 120, 121, 122, 123, 124, 127, 129, 131, 133, 137, 156, 164,
            175, 177, 178, 181, 183, 184, 185, 187, 188, 189, 191, 193, 194, 195, 198, 200, 201,
            204, 205, 221, 223, 225, 226, 227, 228, 229, 230, 231, 233, 234, 235, 237, 238, 239,
            242, 245, 248, 249, 277, 281, 282, 283, 296, 299, 304, 306, 308, 319, 327, 436, 438,
            440, 442, 444, 446, 450, 455, 457, 458, 460, 461, 464, 494, 507, 515, 518, 520, 524,
            525, 527, 531, 557, 559, 562, 563, 564, 568, 640, 648, 650, 658, 693, 701, 703, 711,
            749, 752, 759, 760, 773, 796, 812, 826, 849, 866, 981, 992, 1045,
        ];
        check_paths(
            ee::s07::f_002642b8__generated,
            c_setup_man_update,
            640,
            PATHS,
            &|c, seed| {
                let t = seed % 20;
                let kind = (t < 19).then(|| SETUP_MAN_UPDATE_TYPES[t as usize]);
                let host = seed / 20 % 2 == 1;
                let state = [0, 8, 5, 10][(seed / 40 % 4) as usize];
                let stats = seed / 160 % 2 == 1;
                let unwind_at = if seed >= 320 { seed / 7 % 8 } else { u32::MAX };
                setup_man_update_fixture(c, seed, state, host, stats, kind, 1, 3, true, unwind_at);
            },
        );
    }

    /// The challenger drop (types 0xf8 and 0xfa) in both roles before and
    /// from state 7, crossed with the fakes' answers: `seed % 2` the type,
    /// `seed / 2 % 2` the host flag, `seed / 4 % 2` the state (0 or 8),
    /// `seed / 8 % 2` HasChallenger, `seed / 16 % 4` the two IsConnected
    /// answers, `seed / 64 % 2` whether the buddy manager exists (Create
    /// then stores it).
    #[test]
    fn c_setup_man_update_drop_matches() {
        setup_man_update_hooks();
        // Every cycle total of the drop cross.
        const PATHS: &[u64] = &[
            166, 168, 170, 171, 172, 173, 174, 175, 176, 177, 178, 179, 180, 181, 182, 183, 184,
            185, 187, 189, 190, 191, 192, 193, 194, 221, 223, 225, 226, 227, 228, 229, 230, 231,
            232, 233, 234, 235, 236, 237, 238, 239, 240, 241, 242, 243, 244, 245, 246, 247, 248,
            249, 250, 252, 254, 255, 256, 257, 258, 259,
        ];
        check_paths(
            ee::s07::f_002642b8__generated,
            c_setup_man_update,
            128,
            PATHS,
            &|c, seed| {
                let kind = Some([0xf8, 0xfa][(seed % 2) as usize]);
                let host = seed / 2 % 2 == 1;
                let state = [0, 8][(seed / 4 % 2) as usize];
                let has = seed / 8 % 2;
                let connected = seed / 16 % 4;
                let buddy = seed / 64 % 2 == 1;
                setup_man_update_fixture(
                    c,
                    seed,
                    state,
                    host,
                    false,
                    kind,
                    has,
                    connected,
                    buddy,
                    u32::MAX,
                );
            },
        );
    }

    pub(crate) const SETUP_FAKES_CTRL: u32 = DATA + 0xe_0000;

    pub(crate) const SETUP_FAKES_MAGIC: u32 = 0x0c40_4001;

    pub(crate) const SETUP_SELF: u32 = DATA + 0x400;

    pub(crate) fn zero_words(c: &mut Cpu, addr: u32, bytes: u32) {
        for i in (0..bytes).step_by(4) {
            c.bus.write32(addr + i, 0);
        }
    }

    pub(crate) fn setup_fake_alloc(c: &mut Cpu) {
        if c.bus.read32(SETUP_FAKES_CTRL) != SETUP_FAKES_MAGIC {
            return fall_back(
                c,
                prev_cell(setup_fake_alloc),
                ee::s10::f_00317d70__generated,
            );
        }
        let n = c.bus.read32(SETUP_FAKES_CTRL + 4);
        c.bus.write32(SETUP_FAKES_CTRL + 4, n + 1);
        c.g[V0][0] = sx(DATA + 0x8_0000 + (n & 7) * 0x40);
        c.cycles += 11;
        ret(c);
    }

    pub(crate) fn setup_fake_delete(c: &mut Cpu) {
        if c.bus.read32(SETUP_FAKES_CTRL) != SETUP_FAKES_MAGIC {
            return fall_back(
                c,
                prev_cell(setup_fake_delete),
                ee::s10::f_00317e50__generated,
            );
        }
        c.cycles += 6;
        ret(c);
    }

    pub(crate) fn setup_fake_get_be(c: &mut Cpu) {
        if c.bus.read32(SETUP_FAKES_CTRL) != SETUP_FAKES_MAGIC {
            return fall_back(
                c,
                prev_cell(setup_fake_get_be),
                ee::s01::f_0014dc80__generated,
            );
        }
        c.g[V0][0] = sx(DATA + 0x3_0000);
        c.cycles += 5;
        ret(c);
    }

    pub(crate) fn setup_fake_get_interface(c: &mut Cpu) {
        if c.bus.read32(SETUP_FAKES_CTRL) != SETUP_FAKES_MAGIC {
            return fall_back(
                c,
                prev_cell(setup_fake_get_interface),
                ee::s01::f_0014dd58__generated,
            );
        }
        c.g[V0][0] = sx(DATA + 0x3_b000);
        c.cycles += 5;
        ret(c);
    }

    pub(crate) fn setup_fake_set_local(c: &mut Cpu) {
        if c.bus.read32(SETUP_FAKES_CTRL) != SETUP_FAKES_MAGIC {
            return fall_back(
                c,
                prev_cell(setup_fake_set_local),
                ee::s07::f_00264cf0__generated,
            );
        }
        c.g[V0][0] = sx(DATA + 0x3_9000);
        c.cycles += 7;
        ret(c);
    }

    pub(crate) fn setup_fake_set_remote(c: &mut Cpu) {
        if c.bus.read32(SETUP_FAKES_CTRL) != SETUP_FAKES_MAGIC {
            return fall_back(
                c,
                prev_cell(setup_fake_set_remote),
                ee::s07::f_00264d80__generated,
            );
        }
        c.g[V0][0] = sx(DATA + 0x3_9000);
        c.cycles += 7;
        ret(c);
    }

    pub(crate) fn setup_fake_flush(c: &mut Cpu) {
        if c.bus.read32(SETUP_FAKES_CTRL) != SETUP_FAKES_MAGIC {
            return fall_back(
                c,
                prev_cell(setup_fake_flush),
                ee::s07::f_00264b20__generated,
            );
        }
        c.cycles += 5;
        ret(c);
    }

    pub(crate) fn setup_fake_alloc_packet(c: &mut Cpu) {
        if c.bus.read32(SETUP_FAKES_CTRL) != SETUP_FAKES_MAGIC {
            return fall_back(
                c,
                prev_cell(setup_fake_alloc_packet),
                ee::s07::f_00264960__generated,
            );
        }
        let n = c.bus.read32(SETUP_FAKES_CTRL + 8);
        c.bus.write32(SETUP_FAKES_CTRL + 8, n + 1);
        c.g[V0][0] = sx(DATA + 0xa_0000 + (n & 3) * 0x100);
        c.cycles += 6;
        ret(c);
    }

    pub(crate) fn setup_fake_free_packet(c: &mut Cpu) {
        if c.bus.read32(SETUP_FAKES_CTRL) != SETUP_FAKES_MAGIC {
            return fall_back(
                c,
                prev_cell(setup_fake_free_packet),
                ee::s07::f_00264990__generated,
            );
        }
        c.cycles += 6;
        ret(c);
    }

    pub(crate) fn setup_fakes_install() {
        hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317D70),
            setup_fake_alloc,
            prev_cell(setup_fake_alloc),
        );
        hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E50),
            setup_fake_delete,
            prev_cell(setup_fake_delete),
        );
        hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014DC80),
            setup_fake_get_be,
            prev_cell(setup_fake_get_be),
        );
        hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014DD58),
            setup_fake_get_interface,
            prev_cell(setup_fake_get_interface),
        );
        hook(
            addr_of_mut!(ee::s07::OVERRIDE_00264CF0),
            setup_fake_set_local,
            prev_cell(setup_fake_set_local),
        );
        hook(
            addr_of_mut!(ee::s07::OVERRIDE_00264D80),
            setup_fake_set_remote,
            prev_cell(setup_fake_set_remote),
        );
        hook(
            addr_of_mut!(ee::s07::OVERRIDE_00264B20),
            setup_fake_flush,
            prev_cell(setup_fake_flush),
        );
        hook(
            addr_of_mut!(ee::s07::OVERRIDE_00264960),
            setup_fake_alloc_packet,
            prev_cell(setup_fake_alloc_packet),
        );
        hook(
            addr_of_mut!(ee::s07::OVERRIDE_00264990),
            setup_fake_free_packet,
            prev_cell(setup_fake_free_packet),
        );
    }

    /// Scrambled machine, gated fakes, and the globals every port here
    /// touches: BELibrary dummy, world, comm channel 0 vtable, and the
    /// global message-list head chain.
    pub(crate) fn setup_machine(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        setup_fakes_install();
        let r = crate::difftest::fakes::machine(c, seed, salt, 0);
        c.bus.write32(SETUP_FAKES_CTRL, SETUP_FAKES_MAGIC);
        c.bus.write32(SETUP_FAKES_CTRL + 4, 0);
        c.bus.write32(SETUP_FAKES_CTRL + 8, 0);
        zero_words(c, DATA + 0x3_0000, 0x100);
        zero_words(c, DATA + 0x3_1000, 0x100);
        zero_words(c, DATA + 0x3_2000, 0x300);
        zero_words(c, DATA + 0x3_3000, 0x400);
        c.bus.write32(GAME_GP.wrapping_sub(0xc8), DATA + 0x3_3000);
        c.bus.write32(GAME_GP.wrapping_sub(0x1f04), DATA + 0x3_0000);
        c.bus.write32(GAME_GP.wrapping_sub(0x848), DATA + 0x3_1000);
        c.bus.write32(GAME_GP.wrapping_sub(0x238), DATA + 0x3_2000);
        c.bus
            .write32(DATA + 0x3_3000 + SETUP_LIST_HEAD, DATA + 0x3_3200);
        c.bus.write32(DATA + 0x3_3200 + 4, DATA + 0x3_3300);
        // Comm channel 0 -> object -> vtable (send slot +0x44, recv +0x3c).
        c.bus.write32(DATA + 0x3_2000, DATA + 0x3_2100);
        c.bus.write32(DATA + 0x3_2100 + 8, DATA + 0x3_2200);
        c.bus.write16(DATA + 0x3_2200 + 0x38, 0);
        c.bus.write16(DATA + 0x3_2200 + 0x40, 0);
        c.bus.write32(DATA + 0x3_2200 + 0x3c, FAKE);
        c.bus.write32(DATA + 0x3_2200 + 0x44, FAKE);
        zero_words(c, SETUP_SELF, 0x200);
        r
    }

    pub(crate) fn setup_self_with_state(c: &mut Cpu, state: u32, flag124: u32) {
        c.bus.write32(SETUP_SELF + SETUP_STATE, state);
        c.bus.write32(SETUP_SELF + SETUP_HOST_FLAG, flag124);
        c.g[A0][0] = sx(SETUP_SELF);
    }

    #[test]
    fn c_setup_man_send_brag_msg8_matches() {
        check(
            ee::s07::f_00265568__generated,
            c_setup_man_send_brag_msg8,
            210,
            &|c, seed| {
                let _r = setup_machine(c, seed, 0x265568);
                c.pc = 0x0026_5568;
                let state = [12u32, 0, 3, 5, 6, 7, 9][(seed % 7) as usize];
                setup_self_with_state(c, state, seed % 2);
                c.g[A1][0] = sx(seed % 16);
                c.g[A2][0] = sx((seed / 7 % 16) as u32);
            },
        );
    }

    #[test]
    fn c_setup_man_send_brag_msg6_matches() {
        check(
            ee::s07::f_00265458__generated,
            c_setup_man_send_brag_msg6,
            210,
            &|c, seed| {
                let _r = setup_machine(c, seed, 0x265458);
                c.pc = 0x0026_5458;
                let state = [12u32, 0, 3, 5, 6, 7, 9][(seed % 7) as usize];
                setup_self_with_state(c, state, seed % 2);
                c.g[A1][0] = sx(seed % 16);
            },
        );
    }

    #[test]
    fn c_setup_man_host_recv_select_matches() {
        check(
            ee::s07::f_00265190__generated,
            c_setup_man_host_recv_select,
            200,
            &|c, seed| {
                let _r = setup_machine(c, seed, 0x265190);
                c.pc = 0x0026_5190;
                let state = [2u32, 1, 0, 3, 0xffff_ffff][(seed % 5) as usize];
                setup_self_with_state(c, state, 0);
                c.bus.write32(SETUP_SELF + SETUP_STATS_FLAG, 1);
                c.g[A1][0] = sx(seed % 8);
                c.g[A2][0] = sx(DATA + 0x5000);
            },
        );
    }

    #[test]
    fn c_setup_man_host_recv_confirm_matches() {
        check(
            ee::s07::f_00265290__generated,
            c_setup_man_host_recv_confirm,
            210,
            &|c, seed| {
                let _r = setup_machine(c, seed, 0x265290);
                c.pc = 0x0026_5290;
                let state = [0u32, 0xffff_ffff, 1, 2, 3, 4, 5][(seed % 7) as usize];
                setup_self_with_state(c, state, 0);
                c.bus.write32(SETUP_SELF + SETUP_STATS_FLAG, 1);
                c.g[A1][0] = sx(seed % 8);
            },
        );
    }

    #[test]
    fn c_setup_man_confirm_character_matches() {
        check(
            ee::s07::f_00265018__generated,
            c_setup_man_confirm_character,
            240,
            &|c, seed| {
                let _r = setup_machine(c, seed, 0x265018);
                c.pc = 0x0026_5018;
                let state = [0u32, 1, 2, 3, 4, 0xffff_ffff][(seed % 6) as usize];
                setup_self_with_state(c, state, (seed / 6 % 2) as u32);
                c.bus.write32(SETUP_SELF + SETUP_STATS_FLAG, 1);
                c.g[A1][0] = sx(seed % 8);
            },
        );
    }

    #[test]
    fn c_setup_man_select_character_matches() {
        check(
            ee::s07::f_00264f88__generated,
            c_setup_man_select_character,
            240,
            &|c, seed| {
                let _r = setup_machine(c, seed, 0x264f88);
                c.pc = 0x0026_4f88;
                let state = [12u32, 1, 0, 5][(seed % 4) as usize];
                setup_self_with_state(c, state, (seed / 4 % 2) as u32);
                c.bus.write32(SETUP_SELF + SETUP_STATS_FLAG, 1);
                c.g[A1][0] = sx(seed % 8);
            },
        );
    }

    #[test]
    fn c_setup_man_send_character_msg_matches() {
        check(
            ee::s07::f_00264e50__generated,
            c_setup_man_send_character_msg,
            120,
            &|c, seed| {
                let _r = setup_machine(c, seed, 0x264e50);
                c.pc = 0x0026_4e50;
                setup_self_with_state(c, 0, 0);
                c.bus.write32(SETUP_SELF + SETUP_STATS_FLAG, seed % 2);
                c.g[A1][0] = sx(seed % 4);
                c.g[A2][0] = sx((seed / 2 % 8) as u32);
                c.g[A3][0] = sx((seed / 4 % 16) as u32);
            },
        );
        driven!(gen, ee::s07::f_00264e50__generated);
        driven!(hand, c_setup_man_send_character_msg);
        check_events(gen, hand, 120, &|c, seed| {
            let mut r = setup_machine(c, seed, 0x264e51);
            setup_self_with_state(c, 0, 0);
            c.bus.write32(SETUP_SELF + SETUP_STATS_FLAG, 1);
            c.g[A1][0] = sx(seed % 4);
            c.g[A2][0] = sx((seed / 2 % 8) as u32);
            c.g[A3][0] = sx((seed / 4 % 16) as u32);
            schedule(c, &mut r, seed, 0x0026_4e50)
        });
    }

    #[test]
    fn c_setup_man_send_packet_matches() {
        check(
            ee::s07::f_00264be8__generated,
            c_setup_man_send_packet,
            200,
            &|c, seed| {
                let _r = setup_machine(c, seed, 0x264be8);
                c.pc = 0x0026_4be8;
                setup_self_with_state(c, 0, 0);
                c.bus.write32(SETUP_SELF + SETUP_SEND_TAIL, seed / 8 % 32);
                let msg = DATA + 0x2000;
                for i in 0..16u32 {
                    c.bus.write32(msg + i * 4, 0x1000 + i);
                }
                if seed % 2 == 0 {
                    c.bus
                        .write32(DATA + 0x3_2200 + 0x44, RIDER_IFACE_HAS_VISITED);
                }
                c.g[A1][0] = sx(msg);
                c.g[A2][0] = sx([0u32, 4, 0x14, 0x40][(seed / 2 % 4) as usize]);
            },
        );
    }

    #[test]
    fn c_setup_man_receive_packets_matches() {
        check(
            ee::s07::f_00264a58__generated,
            c_setup_man_receive_packets,
            200,
            &|c, seed| {
                let _r = setup_machine(c, seed, 0x264a58);
                c.pc = 0x0026_4a58;
                setup_self_with_state(c, 0, 0);
                if seed % 3 != 0 {
                    c.bus.write32(SETUP_SELF + SETUP_CUR_PACKET, DATA + 0x6000);
                }
                c.bus.write32(
                    SETUP_SELF + SETUP_RECV_TAIL,
                    [0u32, 5, 31, 40][(seed % 4) as usize],
                );
                if seed % 2 == 0 {
                    c.bus
                        .write32(DATA + 0x3_2200 + 0x3c, RIDER_IFACE_HAS_VISITED);
                }
            },
        );
        driven!(gen, ee::s07::f_00264a58__generated);
        driven!(hand, c_setup_man_receive_packets);
        check_events(gen, hand, 120, &|c, seed| {
            let mut r = setup_machine(c, seed, 0x264a59);
            setup_self_with_state(c, 0, 0);
            c.bus.write32(SETUP_SELF + SETUP_CUR_PACKET, DATA + 0x6000);
            schedule(c, &mut r, seed, 0x0026_4a58)
        });
    }

    #[test]
    fn c_setup_man_dtor_matches() {
        check(
            ee::s07::f_00264138__generated,
            c_setup_man_dtor,
            240,
            &|c, seed| {
                let _r = setup_machine(c, seed, 0x264138);
                c.pc = 0x0026_4138;
                setup_self_with_state(c, 0, 0);
                if seed % 2 == 0 {
                    c.bus.write32(SETUP_SELF + SETUP_CUR_PACKET, DATA + 0x6000);
                }
                // Receive ring: head +0xc, tail +0x8, slots at +0x10.
                let (head, tail) = [(0u32, 0u32), (0, 2), (2, 2), (31, 33)][(seed % 4) as usize];
                c.bus.write32(SETUP_SELF + SETUP_RECV_HEAD, head);
                c.bus.write32(SETUP_SELF + SETUP_RECV_TAIL, tail);
                for i in 0..4u32 {
                    c.bus.write32(
                        SETUP_SELF + SETUP_RECV_SLOTS + i * 4,
                        DATA + 0x6000 + i * 0x10,
                    );
                }
                let (shead, stail) = [(0u32, 0u32), (0, 1), (3, 3)][(seed / 4 % 3) as usize];
                c.bus.write32(SETUP_SELF + SETUP_SEND_HEAD, shead);
                c.bus.write32(SETUP_SELF + SETUP_SEND_TAIL, stail);
                for i in 0..4u32 {
                    c.bus.write32(
                        SETUP_SELF + SETUP_SEND_SLOTS + i * 4,
                        DATA + 0x7000 + i * 0x10,
                    );
                }
                c.g[A1][0] = sx(seed % 2);
            },
        );
        driven!(gen, ee::s07::f_00264138__generated);
        driven!(hand, c_setup_man_dtor);
        check_events(gen, hand, 120, &|c, seed| {
            let mut r = setup_machine(c, seed, 0x264139);
            setup_self_with_state(c, 0, 0);
            // `seed / 3 % 3`: packets in the receive ring (loop head
            // 0x264178), the send ring (0x264208) or both.
            let rings = seed / 3 % 3;
            let recv = if rings != 1 { 3 } else { 0 };
            let send = if rings != 0 { 3 } else { 0 };
            c.bus.write32(SETUP_SELF + SETUP_RECV_HEAD, 0);
            c.bus.write32(SETUP_SELF + SETUP_RECV_TAIL, recv);
            c.bus.write32(SETUP_SELF + SETUP_SEND_HEAD, 0);
            c.bus.write32(SETUP_SELF + SETUP_SEND_TAIL, send);
            for i in 0..4u32 {
                c.bus.write32(
                    SETUP_SELF + SETUP_RECV_SLOTS + i * 4,
                    DATA + 0x6000 + i * 0x10,
                );
                c.bus.write32(
                    SETUP_SELF + SETUP_SEND_SLOTS + i * 4,
                    DATA + 0x7000 + i * 0x10,
                );
            }
            c.g[A1][0] = 0;
            schedule(c, &mut r, seed, 0x0026_4138)
        });
    }

    /// Gate for this module's callee fakes: a word in the machine's RAM.
    /// With it off, a hooked slot runs the body it held before.
    pub(crate) const NET_FAKES_CTRL: u32 = DATA + 0xf_ff00;

    pub(crate) const NET_FAKES_MAGIC: u32 = 0x6f63_3405;

    pub(crate) const NET_FAKES_COUNT: u32 = DATA + 0xf_ff04;

    pub(crate) const NET_FAKES_UNWIND: u32 = DATA + 0xf_ff08;

    /// Per-callee forced `v0` result cells.
    pub(crate) const NET_FAKE_BE: u32 = DATA + 0xf_ff10;

    pub(crate) const NET_FAKE_IFACE: u32 = DATA + 0xf_ff14;

    pub(crate) const NET_FAKE_ALLOC: u32 = DATA + 0xf_ff18;

    pub(crate) const NET_FAKE_ISCONN: u32 = DATA + 0xf_ff1c;

    pub(crate) const NET_FAKE_FIND: u32 = DATA + 0xf_ff20;

    pub(crate) const NET_FAKE_IDX: u32 = DATA + 0xf_ff24;

    pub(crate) const NET_FAKE_ISTEMP: u32 = DATA + 0xf_ff28;

    pub(crate) const NET_FAKE_HASMSG: u32 = DATA + 0xf_ff2c;

    pub(crate) const NET_FAKE_GROSTER: u32 = DATA + 0xf_ff30;

    macro_rules! net_gated_fake {
        ($name:ident, $prev:ident, $cell:expr, $gen:path) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $name(c: &mut Cpu) {
                if c.bus.read32(NET_FAKES_CTRL) == NET_FAKES_MAGIC {
                    let idx = c.bus.read32(NET_FAKES_COUNT);
                    c.bus.write32(NET_FAKES_COUNT, idx + 1);
                    if idx == c.bus.read32(NET_FAKES_UNWIND) {
                        c.pc = 0xdead_0000 | idx << 4;
                        return;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = sx(c.bus.read32($cell));
                    c.pc = r32(c, RA);
                    return;
                }
                fakes::fall_back(c, &$prev, $gen)
            }
        };
    }

    net_gated_fake!(
        net_fake_get_be,
        NET_PREV_GET_BE,
        NET_FAKE_BE,
        ee::s01::f_0014dc80__generated
    );

    net_gated_fake!(
        net_fake_get_iface,
        NET_PREV_GET_IFACE,
        NET_FAKE_IFACE,
        ee::s01::f_0014dd58__generated
    );

    net_gated_fake!(
        net_fake_set_humans,
        NET_PREV_SET_HUMANS,
        NET_FAKE_BE,
        ee::s01::f_00144aa8__generated
    );

    net_gated_fake!(
        net_fake_set_rider,
        NET_PREV_SET_RIDER,
        NET_FAKE_BE,
        ee::s01::f_001472c8__generated
    );

    net_gated_fake!(
        net_fake_set_char,
        NET_PREV_SET_CHAR,
        NET_FAKE_BE,
        ee::s01::f_00147338__generated
    );

    net_gated_fake!(
        net_fake_iface_ebf8,
        NET_PREV_IFACE_EBF8,
        NET_FAKE_BE,
        ee::s01::f_0014ebf8__generated
    );

    net_gated_fake!(
        net_fake_alloc,
        NET_PREV_ALLOC,
        NET_FAKE_ALLOC,
        ee::s10::f_00317d70__generated
    );

    net_gated_fake!(
        net_fake_setup_create,
        NET_PREV_SETUP_CREATE,
        NET_FAKE_ALLOC,
        ee::s07::f_00264030__generated
    );

    net_gated_fake!(
        net_fake_setup_hello,
        NET_PREV_SETUP_HELLO,
        NET_FAKE_BE,
        ee::s07::f_00264cb8__generated
    );

    net_gated_fake!(
        net_fake_setup_destroy,
        NET_PREV_SETUP_DESTROY,
        NET_FAKE_BE,
        ee::s07::f_00264098__generated
    );

    net_gated_fake!(
        net_fake_flush,
        NET_PREV_FLUSH,
        NET_FAKE_BE,
        ee::s06::f_0025b800__generated
    );

    net_gated_fake!(
        net_fake_net_create,
        NET_PREV_NET_CREATE,
        NET_FAKE_BE,
        ee::s06::f_00260f80__generated
    );

    net_gated_fake!(
        net_fake_is_conn,
        NET_PREV_IS_CONN,
        NET_FAKE_ISCONN,
        ee::s06::f_00261460__generated
    );

    net_gated_fake!(
        net_fake_reconnect,
        NET_PREV_RECONNECT,
        NET_FAKE_BE,
        ee::s06::f_002613c8__generated
    );

    net_gated_fake!(
        net_fake_net_disc,
        NET_PREV_NET_DISC,
        NET_FAKE_BE,
        ee::s06::f_00261408__generated
    );

    net_gated_fake!(
        net_fake_find_chan,
        NET_PREV_FIND_CHAN,
        NET_FAKE_FIND,
        ee::s06::f_00255d38__generated
    );

    net_gated_fake!(
        net_fake_voip_start,
        NET_PREV_VOIP_START,
        NET_FAKE_BE,
        ee::s07::f_00266d58__generated
    );

    net_gated_fake!(
        net_fake_voip_activate,
        NET_PREV_VOIP_ACTIVATE,
        NET_FAKE_BE,
        ee::s07::f_00266e30__generated
    );

    net_gated_fake!(
        net_fake_voip_deactivate,
        NET_PREV_VOIP_DEACTIVATE,
        NET_FAKE_BE,
        ee::s07::f_00266e88__generated
    );

    net_gated_fake!(
        net_fake_voip_stop,
        NET_PREV_VOIP_STOP,
        NET_FAKE_BE,
        ee::s07::f_00266df8__generated
    );

    net_gated_fake!(
        net_fake_module_ctor,
        NET_PREV_MODULE_CTOR,
        NET_FAKE_ALLOC,
        ee::s06::f_00232720__generated
    );

    net_gated_fake!(
        net_fake_set_next,
        NET_PREV_SET_NEXT,
        NET_FAKE_BE,
        ee::s10::f_00316ef8__generated
    );

    net_gated_fake!(
        net_fake_find_chal,
        NET_PREV_FIND_CHAL,
        NET_FAKE_IDX,
        ee::s07::f_00262e20__generated
    );

    net_gated_fake!(
        net_fake_is_temp,
        NET_PREV_IS_TEMP,
        NET_FAKE_ISTEMP,
        ee::s07::f_00262438__generated
    );

    net_gated_fake!(
        net_fake_has_msg,
        NET_PREV_HAS_MSG,
        NET_FAKE_HASMSG,
        ee::s07::f_00262a38__generated
    );

    net_gated_fake!(
        net_fake_find_roster,
        NET_PREV_FIND_ROSTER,
        NET_FAKE_GROSTER,
        ee::s07::f_00262468__generated
    );

    net_gated_fake!(
        net_fake_get_roster,
        NET_PREV_GET_ROSTER,
        NET_FAKE_GROSTER,
        ee::s07::f_002625c8__generated
    );

    /// Install every gated fake used below (once per process each).
    pub(crate) fn net_fakes_install() {
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014DC80),
            net_fake_get_be,
            &NET_PREV_GET_BE,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014DD58),
            net_fake_get_iface,
            &NET_PREV_GET_IFACE,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_00144AA8),
            net_fake_set_humans,
            &NET_PREV_SET_HUMANS,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_001472C8),
            net_fake_set_rider,
            &NET_PREV_SET_RIDER,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_00147338),
            net_fake_set_char,
            &NET_PREV_SET_CHAR,
        );
        fakes::hook(
            addr_of_mut!(ee::s01::OVERRIDE_0014EBF8),
            net_fake_iface_ebf8,
            &NET_PREV_IFACE_EBF8,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317D70),
            net_fake_alloc,
            &NET_PREV_ALLOC,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00264030),
            net_fake_setup_create,
            &NET_PREV_SETUP_CREATE,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00264CB8),
            net_fake_setup_hello,
            &NET_PREV_SETUP_HELLO,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00264098),
            net_fake_setup_destroy,
            &NET_PREV_SETUP_DESTROY,
        );
        fakes::hook(
            addr_of_mut!(ee::s06::OVERRIDE_0025B800),
            net_fake_flush,
            &NET_PREV_FLUSH,
        );
        fakes::hook(
            addr_of_mut!(ee::s06::OVERRIDE_00260F80),
            net_fake_net_create,
            &NET_PREV_NET_CREATE,
        );
        fakes::hook(
            addr_of_mut!(ee::s06::OVERRIDE_00261460),
            net_fake_is_conn,
            &NET_PREV_IS_CONN,
        );
        fakes::hook(
            addr_of_mut!(ee::s06::OVERRIDE_002613C8),
            net_fake_reconnect,
            &NET_PREV_RECONNECT,
        );
        fakes::hook(
            addr_of_mut!(ee::s06::OVERRIDE_00261408),
            net_fake_net_disc,
            &NET_PREV_NET_DISC,
        );
        fakes::hook(
            addr_of_mut!(ee::s06::OVERRIDE_00255D38),
            net_fake_find_chan,
            &NET_PREV_FIND_CHAN,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00266D58),
            net_fake_voip_start,
            &NET_PREV_VOIP_START,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00266E30),
            net_fake_voip_activate,
            &NET_PREV_VOIP_ACTIVATE,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00266E88),
            net_fake_voip_deactivate,
            &NET_PREV_VOIP_DEACTIVATE,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00266DF8),
            net_fake_voip_stop,
            &NET_PREV_VOIP_STOP,
        );
        fakes::hook(
            addr_of_mut!(ee::s06::OVERRIDE_00232720),
            net_fake_module_ctor,
            &NET_PREV_MODULE_CTOR,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00316EF8),
            net_fake_set_next,
            &NET_PREV_SET_NEXT,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00262E20),
            net_fake_find_chal,
            &NET_PREV_FIND_CHAL,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00262438),
            net_fake_is_temp,
            &NET_PREV_IS_TEMP,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00262A38),
            net_fake_has_msg,
            &NET_PREV_HAS_MSG,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_00262468),
            net_fake_find_roster,
            &NET_PREV_FIND_ROSTER,
        );
        fakes::hook(
            addr_of_mut!(ee::s07::OVERRIDE_002625C8),
            net_fake_get_roster,
            &NET_PREV_GET_ROSTER,
        );
    }

    pub(crate) fn net_fakes_on(c: &mut Cpu) {
        net_fakes_install();
        c.bus.write32(NET_FAKES_CTRL, NET_FAKES_MAGIC);
        c.bus.write32(NET_FAKES_COUNT, 0);
        c.bus.write32(NET_FAKES_UNWIND, u32::MAX);
        c.bus.write32(LOG_UNWIND_AT, u32::MAX);
    }

    /// A BuddyMan at DATA with three challengers named alice/bob/carol.
    /// Returns the wanted name text for `class` (see the tests).
    pub(crate) fn buddy_challengers_fixture(c: &mut Cpu, class: u32) -> &'static [u8] {
        let begin = DATA + 0x200;
        c.bus.write32(DATA + 0xbc, begin);
        let n: u32 = if class == 0 { 0 } else { 3 };
        c.bus.write32(DATA + 0xc0, begin + 4 * n);
        let names: [&[u8]; 3] = [b"alice", b"bob", b"carol"];
        for i in 0..3u32 {
            let entry = DATA + 0x400 + 0x40 * i;
            c.bus.write32(begin + 4 * i, entry);
            c.bus.write32(entry, DATA + 0x600 + 0x20 * i);
            put_cstr(c, DATA + 0x600 + 0x20 * i, names[i as usize]);
        }
        match class {
            1 => b"alice",
            2 => b"carol",
            3 => b"dave",
            _ => b"bob",
        }
    }

    /// Gate of [`strcmp_fake`]: a word in the machine's RAM.
    pub(crate) const STRCMP_FAKE_CTRL: u32 = DATA + 0xf_fe00;

    pub(crate) const STRCMP_FAKE_MAGIC: u32 = 0x0041_65a8;

    /// `strcmp` without event checks, for the name scans' event tests: an
    /// event delivered inside the real one resumes there, and `drive` only
    /// re-enters the function under test.
    pub(crate) fn strcmp_fake(c: &mut Cpu) {
        if c.bus.read32(STRCMP_FAKE_CTRL) != STRCMP_FAKE_MAGIC {
            return fall_back(c, prev_cell(strcmp_fake), ee::s15::f_004165a8__generated);
        }
        let (a, b) = (c.g[A0][0] as u32, c.g[A1][0] as u32);
        let mut i = 0;
        let d = loop {
            let (x, y) = (c.bus.read8(a + i), c.bus.read8(b + i));
            if x != y || x == 0 {
                break x as i32 - y as i32;
            }
            i += 1;
        };
        c.g[V0][0] = d as i64 as u64;
        c.cycles += 10;
        ret(c);
    }

    /// Hooks [`strcmp_fake`] and turns it on for this machine.
    pub(crate) fn strcmp_fake_on(c: &mut Cpu) {
        hook(
            std::ptr::addr_of_mut!(ee::s15::OVERRIDE_004165A8),
            strcmp_fake,
            prev_cell(strcmp_fake),
        );
        c.bus.write32(STRCMP_FAKE_CTRL, STRCMP_FAKE_MAGIC);
    }

    #[test]
    fn buddy_man_has_challenger_str_matches() {
        check(
            ee::s07::f_002631e0__generated,
            buddy_man_has_challenger_str,
            100,
            &|c, seed| {
                let mut r = Rng(0x0026_31e0 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                let want = buddy_challengers_fixture(c, seed % 5);
                put_cstr(c, DATA + 0x800, want);
                c.bus.write32(DATA + 0x880, DATA + 0x800);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x880) as u64;
                c.pc = 0x0026_31e0;
            },
        );
    }

    #[test]
    fn buddy_man_has_challenger_str_events() {
        let setup = |c: &mut Cpu, seed: u32| -> Events {
            let mut r = Rng(0x0026_31e0 ^ (seed as u64) << 8);
            scramble(c, &mut r);
            let want = buddy_challengers_fixture(c, seed % 5);
            put_cstr(c, DATA + 0x800, want);
            c.bus.write32(DATA + 0x880, DATA + 0x800);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA + 0x880) as u64;
            strcmp_fake_on(c);
            schedule(c, &mut r, seed, 0x0026_31e0)
        };
        driven!(gen, ee::s07::f_002631e0__generated);
        driven!(hand, buddy_man_has_challenger_str);
        check_events(gen, hand, 60, &setup);
    }

    #[test]
    fn buddy_man_get_challenger_by_name_matches() {
        check(
            ee::s07::f_00263270__generated,
            buddy_man_get_challenger_by_name,
            100,
            &|c, seed| {
                let mut r = Rng(0x0026_3270 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                let want = buddy_challengers_fixture(c, seed % 5);
                put_cstr(c, DATA + 0x800, want);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x800) as u64;
                c.pc = 0x0026_3270;
            },
        );
    }

    #[test]
    fn buddy_man_get_challenger_by_name_events() {
        let setup = |c: &mut Cpu, seed: u32| -> Events {
            let mut r = Rng(0x0026_3270 ^ (seed as u64) << 8);
            scramble(c, &mut r);
            let want = buddy_challengers_fixture(c, seed % 5);
            put_cstr(c, DATA + 0x800, want);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA + 0x800) as u64;
            strcmp_fake_on(c);
            schedule(c, &mut r, seed, 0x0026_3270)
        };
        driven!(gen, ee::s07::f_00263270__generated);
        driven!(hand, buddy_man_get_challenger_by_name);
        check_events(gen, hand, 60, &setup);
    }

    /// Message vector fixture for `BuddyMan_DeleteMessage`: entries in the
    /// slots at DATA+0x200.., finish pointing at the last slot.
    pub(crate) fn buddy_messages_fixture(c: &mut Cpu, seed: u32) -> u32 {
        let buf = DATA + 0x200;
        for i in 0..3u32 {
            let obj = DATA + 0x400 + 0x40 * i;
            let vt = DATA + 0x800 + 0x20 * i;
            c.bus.write32(buf + 4 * i, obj);
            c.bus.write32(obj + 0xc, vt);
            c.bus.write16(vt + 8, 0);
            c.bus.write32(vt + 0xc, FAKE);
        }
        // The scan starts at the finish slot `*(self+0xb0)` and walks down
        // to `*(self+0xac)`; the finish slot holds a dummy that never
        // matches, the three messages sit below it. A match one slot below
        // the finish needs no tail shift; lower matches shift the slots
        // above them down by one (memmove size from the finish value).
        c.bus.write32(buf + 12, DATA + 0x6000);
        let class = seed % 5;
        if class == 0 {
            // Empty: begin == finish.
            c.bus.write32(DATA + 0xac, buf + 12);
            c.bus.write32(DATA + 0xb0, buf + 12);
        } else {
            c.bus.write32(DATA + 0xac, buf - 4);
            c.bus.write32(DATA + 0xb0, buf + 12);
        }
        match class {
            // Target one slot below the finish: no tail shift.
            1 => DATA + 0x400 + 0x80,
            // Target two slots below: tail shifts by one.
            2 => DATA + 0x400 + 0x40,
            // Null message two slots below: drop only.
            4 => {
                c.bus.write32(buf + 4, 0);
                0
            }
            // Absent (also the empty-vector class).
            _ => DATA + 0x5000,
        }
    }

    #[test]
    fn buddy_man_delete_message_matches() {
        check(
            ee::s07::f_00263500__generated,
            buddy_man_delete_message,
            100,
            &|c, seed| {
                let _r = machine(c, seed, 0x263500, 4);
                c.bus.write32(LOG_UNWIND_AT, u32::MAX);
                let target = buddy_messages_fixture(c, seed);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = target as u64;
                c.pc = 0x0026_3500;
            },
        );
    }

    #[test]
    fn buddy_man_delete_message_events() {
        let setup = |c: &mut Cpu, seed: u32| -> Events {
            let mut r = machine(c, seed, 0x263500, 4);
            c.bus.write32(LOG_UNWIND_AT, u32::MAX);
            let target = buddy_messages_fixture(c, seed);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = target as u64;
            schedule(c, &mut r, seed, 0x0026_3500)
        };
        driven!(gen, ee::s07::f_00263500__generated);
        driven!(hand, buddy_man_delete_message);
        check_events(gen, hand, 60, &setup);
    }

    #[test]
    fn buddy_man_remove_challenger_matches() {
        check(
            ee::s07::f_00263018__generated,
            buddy_man_remove_challenger,
            160,
            &|c, seed| {
                let _r = machine(c, seed, 0x263018, 6);
                net_fakes_on(c);
                // Challenger vector (finish at the last slot) and vtables.
                let buf = DATA + 0x200;
                for i in 0..3u32 {
                    let obj = DATA + 0x400 + 0x40 * i;
                    let vt = DATA + 0x800 + 0x20 * i;
                    c.bus.write32(buf + 4 * i, obj);
                    c.bus.write32(obj + 0x3c, vt);
                    c.bus.write16(vt + 8, 0);
                    c.bus.write32(vt + 0xc, FAKE);
                }
                c.bus.write32(DATA + 0xbc, buf);
                c.bus.write32(DATA + 0xc0, buf + 12);
                c.bus.write32(DATA + 0xc8, DATA + 0x2000);
                // Lobby manager for the notify callee: no API, one list node.
                let mgr = DATA + 0x1000;
                c.bus.write32(GAME_GP - 0xc8, mgr);
                c.bus.write32(mgr + 0xf0, DATA + 0x1800);
                c.bus.write32(DATA + 0x1800 + 4, DATA + 0x1900);
                // Branch steering from the seed residue only.
                let class = seed % 8;
                let (idx, temp, msg, roster) = match class {
                    0 => (u32::MAX, 0, 0, 0),
                    1 => (2, 0, 0, 0),
                    2 => (1, 0, 0, 0),
                    3 => (0, 1, 1, 0),
                    4 => (1, 1, 0, 0),
                    5 => (1, 1, 0, DATA + 0x2100),
                    6 => (1, 0, 0, 0),
                    _ => (2, 1, 0, DATA + 0x2100),
                };
                if class == 6 {
                    c.bus.write32(buf + 4, 0);
                }
                c.bus.write32(NET_FAKE_IDX, idx);
                c.bus.write32(NET_FAKE_ISTEMP, temp);
                c.bus.write32(NET_FAKE_HASMSG, msg);
                c.bus.write32(NET_FAKE_GROSTER, roster);
                if class == 7 {
                    c.bus.write32(NET_FAKES_UNWIND, (seed / 8) % 3);
                }
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x900) as u64;
                c.pc = 0x0026_3018;
            },
        );
    }

    /// Globals shared by the Net fixtures: comm system, lobby manager,
    /// setup manager, world and the BE-library dummy.
    pub(crate) fn net_globals_fixture(c: &mut Cpu) {
        c.bus.write32(GAME_GP - 0x1f04, DATA + 0x9000);
        let world = DATA + 0x4000;
        c.bus.write32(GAME_GP - 0x848, world);
        c.bus.write32(world + 0x78, DATA + 0x2100);
        c.bus.write32(world + 0x14, 0x3f00_0000);
        let mgr = DATA + 0x1000;
        c.bus.write32(GAME_GP - 0xc8, mgr);
        c.bus.write32(mgr + 0xf0, DATA + 0x1800);
        c.bus.write32(DATA + 0x1800 + 4, DATA + 0x1900);
        c.bus.write32(mgr + 0x8c, 7);
        c.bus.write32(mgr + 0x98, 9);
        let comm = DATA + 0x3000;
        c.bus.write32(GAME_GP - 0x238, comm);
        c.bus.write32(comm, DATA + 0x3100);
        let chan = DATA + 0x3100;
        let vt = DATA + 0x3200;
        c.bus.write32(chan + 8, vt);
        c.bus.write16(vt + 8, 0);
        c.bus.write32(vt + 0xc, FAKE);
        c.bus.write16(vt + 0x10, 0);
        c.bus.write32(vt + 0x14, FAKE);
        c.bus.write16(vt + 0x20, 0);
        c.bus.write32(vt + 0x24, FAKE);
        let iface = DATA + 0x2100;
        c.bus.write32(iface + 0xc, DATA + 0x2200);
        c.bus.write16(DATA + 0x2200 + 8, 0);
        c.bus.write32(DATA + 0x2200 + 0xc, FAKE);
    }

    #[test]
    fn net_disconnect_all_matches() {
        check(
            ee::s07::f_002636f0__generated,
            net_disconnect_all,
            160,
            &|c, seed| {
                let _r = machine(c, seed, 0x2636f0, 6);
                net_fakes_on(c);
                net_globals_fixture(c);
                c.bus.write32(NET_FAKE_ALLOC, DATA + 0x6000);
                c.bus.write32(NET_FAKE_ISCONN, (seed / 2) & 1);
                // Net manager pointer and the comm busy flag, from seed bits.
                if seed & 1 == 0 {
                    c.bus.write32(GAME_GP + 0x238, DATA + 0x3800);
                }
                let comm = DATA + 0x3000;
                c.bus.write32(comm + 0x1c, u32::MAX);
                c.bus.write32(comm + 0x20, (seed / 4) & 1);
                if seed % 16 == 15 {
                    c.bus.write32(NET_FAKES_UNWIND, (seed / 16) % 4);
                }
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = if seed & 8 == 0 {
                    0
                } else {
                    (DATA + 0x500) as u64
                };
                c.pc = 0x0026_36f0;
            },
        );
    }

    #[test]
    fn net_abort_game_setup_matches() {
        check(
            ee::s07::f_00263ba8__generated,
            net_abort_game_setup,
            240,
            &|c, seed| {
                let _r = machine(c, seed, 0x263ba8, 6);
                net_fakes_on(c);
                net_globals_fixture(c);
                c.bus.write32(NET_FAKE_ALLOC, DATA + 0x6000);
                c.bus.write32(NET_FAKE_FIND, (seed / 2) & 1);
                if (seed / 4) & 1 == 1 {
                    c.bus.write32(GAME_GP + 0x2dc, DATA + 0x2000);
                }
                c.bus.write32(GAME_GP + 0x304, DATA + 0x5000);
                c.bus.write32(DATA, seed & 1);
                c.bus.write32(DATA + 4, 0);
                if seed % 24 == 23 {
                    c.bus.write32(NET_FAKES_UNWIND, (seed / 24) % 3);
                }
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 3) as u64;
                c.pc = 0x0026_3ba8;
            },
        );
    }

    #[test]
    fn net_start_game_setup_matches() {
        check(
            ee::s07::f_002639e0__generated,
            net_start_game_setup,
            60,
            &|c, seed| {
                let _r = machine(c, seed, 0x2639e0, 8);
                net_fakes_on(c);
                net_globals_fixture(c);
                c.bus.write32(NET_FAKE_BE, DATA + 0x2000);
                c.bus.write32(NET_FAKE_IFACE, DATA + 0x2100);
                c.bus.write32(NET_FAKE_ALLOC, DATA + 0x6000);
                c.bus.write32(GAME_GP - 0x4b74, 0x3c88_8889);
                c.bus.write32(GAME_GP - 0x1dc, 0x3f80_0000);
                c.bus.write32(GAME_GP + 0x304, DATA + 0x5000);
                c.bus
                    .write32(0x0053_5610, if seed & 1 == 1 { 1 << 18 } else { 0 });
                if seed % 12 == 11 {
                    c.bus.write32(NET_FAKES_UNWIND, (seed / 12) % 4);
                }
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (0xc0a8_0101 + (seed % 4)) as u64;
                c.g[A2][0] = 0;
                c.g[A3][0] = (seed % 3) as u64;
                c.g[T0][0] = (seed % 5) as u64;
                c.pc = 0x0026_39e0;
            },
        );
    }

    #[test]
    fn net_update_game_setup_matches() {
        check(
            ee::s07::f_00263cf0__generated,
            net_update_game_setup,
            240,
            &|c, seed| {
                let _r = machine(c, seed, 0x263cf0, 10);
                net_fakes_on(c);
                net_globals_fixture(c);
                c.bus.write32(NET_FAKE_IFACE, DATA + 0x2100);
                c.bus.write32(NET_FAKE_ALLOC, DATA + 0x6000);
                // The launch's buddy-manager checks: `seed / 12 % 2` whether it
                // exists (else Create, a fake, leaves it missing and the
                // lobby is disconnected), `seed / 24 % 2` IsConnected.
                c.bus.write32(NET_FAKE_ISCONN, seed / 24 % 2);
                c.bus.write32(NET_FAKE_FIND, 1);
                c.bus.write32(
                    GAME_GP.wrapping_add(BUDDY_MAN_GP as u32),
                    if seed / 12 % 2 == 0 { DATA + 0x3800 } else { 0 },
                );
                c.bus.write32(GAME_GP + 0x304, DATA + 0x5000);
                let setup = DATA + 0x2000;
                c.bus.write32(GAME_GP + 0x2dc, setup);
                let comm = DATA + 0x3000;
                let chan = DATA + 0x3100;
                // The state classes, `seed % 12`.
                let class = seed % 12;
                let (state, setup_state, comm28, chan_fn, timer) = match class {
                    0 => (0, 0, 0, FAKE, 0),
                    1 => (3, 0, 0, FAKE, 0),
                    2 => (1, 0, 0, FAKE, 0),
                    3 => (1, 0, 0, FAKE, 0),
                    4 => (1, 0, 1, 0x0014_06c0, 0),
                    5 => (1, 0, 0, 0x0014_06c0, 0x3fc0_0000),
                    6 => (1, 0, 0, 0x0014_06c0, 0xbfc0_0000),
                    7 => (2, 5, 1, FAKE, 0),
                    8 => (2, 0xd, 0, FAKE, 0),
                    9 => (2, 0xb, 0, FAKE, 0),
                    10 => (2, 5, 0, FAKE, 0),
                    _ => (2, 0xb, 1, FAKE, 0),
                };
                c.bus.write32(DATA, state);
                c.bus.write32(DATA + 4, 0);
                c.bus.write32(DATA + 8, DATA + 0x1111);
                c.bus.write32(DATA + 0xc, if class == 2 { 1 } else { 0 });
                c.bus.write32(DATA + 0x10, timer);
                c.bus.write32(setup, setup_state);
                c.bus.write32(comm + 0x28, comm28);
                c.bus.write32(chan + 4, 0);
                c.bus.write32(chan + 0x20, 0);
                c.bus.write32(DATA + 0x3200 + 0x24, chan_fn);
                if class == 9 {
                    c.bus.write32(NET_FAKES_UNWIND, 1);
                }
                c.g[A0][0] = DATA as u64;
                c.pc = 0x0026_3cf0;
            },
        );
    }
}
