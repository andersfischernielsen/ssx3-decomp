//! `src/dirtysock/buddyapi.cpp`: hand-written replacements (moved from the
//! batch modules by `tools/reorg_modules.py`).

use crate::abi::addu;
use crate::abi::r32;
use crate::abi::ret;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::sp;
use crate::call::tail_wrapper;
use crate::call::Callee;
use crate::call::Flow;
use crate::call::Frame;
use crate::call::Target;
use crate::call::Unwound;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
use crate::events::Resume;
use crate::guest::Addr;
use callee::*;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x003e_fe20,
        buddy_api_lookup_product,
        "BuddyApi_LookupProduct",
    ),
    (0x003e_fed0, buddy_api_debug_printf, "BuddyApi_DebugPrintf"),
    (0x003e_ff40, buddy_api_debug_packet, "BuddyApi_DebugPacket"),
    (0x003f_00d8, buddy_api_send_packet, "BuddyApi_SendPacket"),
    (
        0x003f_0150,
        buddy_api_strip_user_name,
        "BuddyApi_StripUserName",
    ),
    (
        0x003f_0198,
        buddy_api_dispatch_message,
        "BuddyApi_DispatchMessage",
    ),
    (
        0x003f_02d0,
        buddy_api_parse_presence,
        "BuddyApi_ParsePresence",
    ),
    (
        0x003f_0568,
        buddy_api_send_presence,
        "BuddyApi_SendPresence",
    ),
    (0x003f_0610, buddy_api_set_state, "BuddyApi_SetState"),
    (
        0x003f_0658,
        buddy_api_set_pending_op,
        "BuddyApi_SetPendingOp",
    ),
    (0x003f_0710, buddy_api_create, "BuddyApi_Create"),
    (0x003f_07a0, buddy_api_destroy, "BuddyApi_Destroy"),
    (0x003f_0818, buddy_api_config, "BuddyApi_Config"),
    (0x003f_08e8, buddy_api_connect, "BuddyApi_Connect"),
    (0x003f_0b50, buddy_api_disconnect, "BuddyApi_Disconnect"),
    (0x003f_0be0, buddy_api_flush_roster, "BuddyApi_FlushRoster"),
    (0x003f_0c90, buddy_api_get_roster, "BuddyApi_GetRoster"),
    (0x003f_0e38, buddy_api_add, "BuddyApi_Add"),
    (0x003f_11f8, buddy_api_del, "BuddyApi_Del"),
    (0x003f_14d8, buddy_api_find, "BuddyApi_Find"),
    (
        0x003f_1538,
        buddy_api_clear_presence,
        "BuddyApi_ClearPresence",
    ),
    (
        0x003f_1548,
        buddy_api_set_presence_text,
        "BuddyApi_SetPresenceText",
    ),
    (
        0x003f_1578,
        buddy_api_set_presence_field,
        "BuddyApi_SetPresenceField",
    ),
    (0x003f_15c8, buddy_api_set_presence, "BuddyApi_SetPresence"),
    (0x003f_16d8, buddy_api_send, "BuddyApi_Send"),
    (0x003f_17c8, buddy_api_set_callback, "BuddyApi_SetCallback"),
    (0x003f_17d8, buddy_api_process, "BuddyApi_Process"),
    (
        0x003f_4448,
        dirty_rpc_alarm_handler,
        "DirtyRpc_AlarmHandler",
    ),
    (0x003f_44f8, dirty_rpc_thread_main, "DirtyRpc_ThreadMain"),
    (0x003f_4528, dirty_rpc_send, "DirtyRpc_Send"),
    (
        0x003f_4a78,
        dirty_rpc_process_replies,
        "DirtyRpc_ProcessReplies",
    ),
    (0x003f_4c68, dirty_rpc_call, "DirtyRpc_Call"),
    (0x003f_4c98, dirty_rpc_init_request, "DirtyRpc_InitRequest"),
    (0x003f_4d68, dirty_rpc_init_iop, "DirtyRpc_InitIop"),
    (0x003f_4e80, dirty_rpc_sync_dcache, "DirtyRpc_SyncDCache"),
    (
        0x003f_4ea0,
        net_game_link_check_recv_buf,
        "NetGameLink_CheckRecvBuf",
    ),
    (0x003f_4f40, dirty_rpc_wait_iop, "DirtyRpc_WaitIop"),
    (0x003f_5000, dirty_rpc_startup, "DirtyRpc_Startup"),
    (0x003f_53f8, dirty_rpc_shutdown, "DirtyRpc_Shutdown"),
    (0x003f_5470, dirty_rpc_is_started, "DirtyRpc_IsStarted"),
    (0x003f_5478, proto_aries_create, "ProtoAriesCreate"),
    (0x003f_55b8, proto_aries_destroy, "ProtoAriesDestroy"),
    (0x003f_5638, proto_aries_connect, "ProtoAriesConnect"),
    (0x003f_5738, proto_aries_unconnect, "ProtoAriesUnconnect"),
    (0x003f_57c8, proto_aries_status, "ProtoAriesStatus"),
    (0x003f_5908, proto_aries_nop, "ProtoAries_Nop"),
    (0x003f_5910, proto_aries_send, "ProtoAriesSend"),
    (0x003f_5a70, proto_aries_peek, "ProtoAriesPeek"),
    (0x003f_5c38, proto_aries_recv, "ProtoAriesRecv"),
    (0x003f_5d88, proto_aries_tick, "ProtoAriesTick"),
    (0x003f_5da8, usb_kb_load_modules, "UsbKb_LoadModules"),
    (0x003f_5e90, usb_kb_unload_module, "UsbKb_UnloadModule"),
    (0x003f_5f08, usb_kb_request_info, "UsbKb_RequestInfo"),
    (0x003f_5f88, usb_kb_sync_info, "UsbKb_SyncInfo"),
    (0x003f_6040, usb_kb_find_next_ready, "UsbKb_FindNextReady"),
    (0x003f_60b8, usb_kb_request_read, "UsbKb_RequestRead"),
    (0x003f_6138, usb_kb_sync_read, "UsbKb_SyncRead"),
    (0x003f_61d0, usb_kb_init, "UsbKb_Init"),
    (0x003f_62f8, usb_kb_shutdown, "UsbKb_Shutdown"),
    (
        0x003f_6348,
        usb_kb_get_max_keyboards,
        "UsbKb_GetMaxKeyboards",
    ),
    (
        0x003f_6358,
        usb_kb_default_connect_cb,
        "UsbKb_DefaultConnectCb",
    ),
    (0x003f_63f0, usb_kb_update, "UsbKb_Update"),
    (0x003f_64e0, usb_kb_get_data, "UsbKb_GetData"),
    (0x003f_6520, usb_kb_is_connected, "UsbKb_IsConnected"),
    (0x003f_6538, usb_kb_any_connected, "UsbKb_AnyConnected"),
    (0x003f_65a0, usb_kb_get_keypad_entry, "UsbKb_GetKeypadEntry"),
    (0x003f_65f8, usb_kb_lookup_key_char, "UsbKb_LookupKeyChar"),
    (0x003f_6670, usb_kb_convert_raw_code, "UsbKb_ConvertRawCode"),
    (
        0x003f_6700,
        usb_kb_check_modifier_key,
        "UsbKb_CheckModifierKey",
    ),
    (0x003f_67e0, usb_kb_check_led_key, "UsbKb_CheckLedKey"),
    (0x003f_6838, usb_kb_is_key_down, "UsbKb_IsKeyDown"),
    (0x003f_68f0, usb_kb_translate_key, "UsbKb_TranslateKey"),
    (0x003f_6960, voip_rpc_call, "VoipRpc_Call"),
    (0x003f_6a40, voip_rpc_init, "VoipRpc_Init"),
    (0x003f_6ab0, voip_rpc_disable, "VoipRpc_Disable"),
    (0x003f_6ab8, voip_rpc_start, "VoipRpc_Start"),
    (0x003f_6af0, voip_rpc_stop, "VoipRpc_Stop"),
    (0x003f_6b20, voip_rpc_enable_voice, "VoipRpc_EnableVoice"),
    (0x003f_6b50, voip_rpc_disable_voice, "VoipRpc_DisableVoice"),
    (0x003f_6b80, voip_rpc_is_active, "VoipRpc_IsActive"),
    (0x003f_6ba0, voip_rpc_get_status, "VoipRpc_GetStatus"),
    (0x003f_6c10, voip_rpc_set_params, "VoipRpc_SetParams"),
    (
        0x003f_6c40,
        voip_rpc_get_shared_status,
        "VoipRpc_GetSharedStatus",
    ),
    (0x003f_4410, dirty_rpc_intc_handler, "DirtyRpc_IntcHandler"),
    (
        0x003f_5a50,
        proto_aries_rpc_reply_callback,
        "ProtoAries_RpcReplyCallback",
    ),
];

// ---------------------------------------------------------------------------
// Callees.

pub(crate) mod callee {
    use super::ee;
    use super::Callee;
    pub(crate) use crate::game::dirtysock::tagsunk::callee::LOBBY_LIST_GET_AT;
    pub(crate) use crate::game::dirtysock::tagsunk::callee::TAG_FIELD_FIND;
    pub(crate) use crate::game::dirtysock::tagsunk::callee::TAG_FIELD_GET_NUMBER;
    pub(crate) use crate::game::fe::festatetrophyroom::callee::USB_KB_DEFAULT_CONNECT_CB;
    pub(crate) use crate::game::wscript::wscriptman::callee::REMOVE_INTC_HANDLER;

    pub const TAG_FIELD_DELETE: Callee = Callee {
        addr: 0x003e_b918,
        f: ee::s14::TagFieldDelete,
    };

    pub const TAG_FIELD_STRNZCPY: Callee = Callee {
        addr: 0x003e_bb60,
        f: ee::s14::TagFieldStrnzcpy,
    };

    pub const TAG_FIELD_SET_TOKEN: Callee = Callee {
        addr: 0x003e_c0f8,
        f: ee::s14::TagFieldSetToken,
    };

    pub const TAG_FIELD_SET_STRING: Callee = Callee {
        addr: 0x003e_c180,
        f: ee::s14::TagFieldSetString,
    };

    pub const TAG_FIELD_GET_TOKEN: Callee = Callee {
        addr: 0x003e_d070,
        f: ee::s14::TagFieldGetToken,
    };

    pub const TAG_FIELD_GET_STRING: Callee = Callee {
        addr: 0x003e_d378,
        f: ee::s14::TagFieldGetString,
    };

    pub const BUDDY_API_LOOKUP_PRODUCT: Callee = Callee {
        addr: 0x003e_fe20,
        f: ee::s14::BuddyApi_LookupProduct,
    };

    pub const BUDDY_API_DEBUG_PRINTF: Callee = Callee {
        addr: 0x003e_fed0,
        f: ee::s14::BuddyApi_DebugPrintf,
    };

    pub const BUDDY_API_DEBUG_PACKET: Callee = Callee {
        addr: 0x003e_ff40,
        f: ee::s14::BuddyApi_DebugPacket,
    };

    pub const BUDDY_API_SEND_PACKET: Callee = Callee {
        addr: 0x003f_00d8,
        f: ee::s14::BuddyApi_SendPacket,
    };

    pub const BUDDY_API_SET_STATE: Callee = Callee {
        addr: 0x003f_0610,
        f: ee::s14::BuddyApi_SetState,
    };

    pub const BUDDY_API_DISCONNECT: Callee = Callee {
        addr: 0x003f_0b50,
        f: ee::s14::BuddyApi_Disconnect,
    };

    pub const BUDDY_API_FLUSH_ROSTER: Callee = Callee {
        addr: 0x003f_0be0,
        f: ee::s14::BuddyApi_FlushRoster,
    };

    pub const SOCKET_LOOKUP: Callee = Callee {
        addr: 0x003f_3a98,
        f: ee::s14::SocketLookup,
    };

    pub const PROTO_ARIES_CREATE: Callee = Callee {
        addr: 0x003f_5478,
        f: ee::s14::ProtoAriesCreate,
    };

    pub const PROTO_ARIES_DESTROY: Callee = Callee {
        addr: 0x003f_55b8,
        f: ee::s14::ProtoAriesDestroy,
    };

    pub const PROTO_ARIES_SEND: Callee = Callee {
        addr: 0x003f_5910,
        f: ee::s14::ProtoAriesSend,
    };

    pub const PROTO_ARIES_TICK: Callee = Callee {
        addr: 0x003f_5d88,
        f: ee::s14::ProtoAriesTick,
    };

    pub const LOBBY_MEM_ALLOC: Callee = Callee {
        addr: 0x0025_91e8,
        f: ee::s06::Lobby_MemAlloc,
    };

    pub const LOBBY_MEM_FREE: Callee = Callee {
        addr: 0x0025_9210,
        f: ee::s06::Lobby_MemFree,
    };

    pub const MEMSET: Callee = Callee {
        addr: 0x0041_6210,
        f: ee::s15::memset,
    };

    pub const STRCAT: Callee = Callee {
        addr: 0x0041_62d0,
        f: ee::s15::strcat,
    };

    pub const STRCHR: Callee = Callee {
        addr: 0x0041_6404,
        f: ee::s15::strchr,
    };

    pub const STRCMP: Callee = Callee {
        addr: 0x0041_65a8,
        f: ee::s15::strcmp,
    };

    pub const STRCPY: Callee = Callee {
        addr: 0x0041_66f4,
        f: ee::s15::strcpy,
    };

    pub const STRLEN: Callee = Callee {
        addr: 0x0041_6810,
        f: ee::s15::strlen,
    };

    pub const STRNCPY: Callee = Callee {
        addr: 0x0041_6ce0,
        f: ee::s15::strncpy,
    };

    pub const VSPRINTF: Callee = Callee {
        addr: 0x0041_86c8,
        f: ee::s15::vsprintf,
    };

    pub const TAG_FIELD_SET_NUMBER: Callee = Callee {
        addr: 0x003e_be40,
        f: ee::s14::TagFieldSetNumber,
    };

    pub const LOBBY_LIST_CREATE: Callee = Callee {
        addr: 0x003e_e1e8,
        f: ee::s14::Lobby_ListCreate,
    };

    pub const LOBBY_LIST_DESTROY: Callee = Callee {
        addr: 0x003e_e260,
        f: ee::s14::Lobby_ListDestroy,
    };

    pub const LOBBY_LIST_ADD: Callee = Callee {
        addr: 0x003e_e320,
        f: ee::s14::Lobby_ListAdd,
    };

    pub const LOBBY_LIST_GET_DATA: Callee = Callee {
        addr: 0x003e_e468,
        f: ee::s14::Lobby_ListGetData,
    };

    pub const LOBBY_LIST_REMOVE: Callee = Callee {
        addr: 0x003e_e480,
        f: ee::s14::Lobby_ListRemove,
    };

    pub const LOBBY_LIST_MARK_DIRTY: Callee = Callee {
        addr: 0x003e_e600,
        f: ee::s14::Lobby_ListMarkDirty,
    };

    pub const LOBBY_HASH_CREATE: Callee = Callee {
        addr: 0x003e_e778,
        f: ee::s14::Lobby_HashCreate,
    };

    pub const LOBBY_HASH_DESTROY: Callee = Callee {
        addr: 0x003e_e820,
        f: ee::s14::Lobby_HashDestroy,
    };

    pub const LOBBY_HASH_POP_ANY: Callee = Callee {
        addr: 0x003e_e9a0,
        f: ee::s14::Lobby_HashPopAny,
    };

    pub const LOBBY_HASH_ADD_STR: Callee = Callee {
        addr: 0x003e_eb30,
        f: ee::s14::Lobby_HashAddStr,
    };

    pub const LOBBY_HASH_FIND_STR: Callee = Callee {
        addr: 0x003e_ec88,
        f: ee::s14::Lobby_HashFindStr,
    };

    pub const LOBBY_HASH_DEL_STR: Callee = Callee {
        addr: 0x003e_ed08,
        f: ee::s14::Lobby_HashDelStr,
    };

    pub const BUDDY_API_SEND_PRESENCE: Callee = Callee {
        addr: 0x003f_0568,
        f: ee::s14::BuddyApi_SendPresence,
    };

    pub const BUDDY_API_SET_PENDING_OP: Callee = Callee {
        addr: 0x003f_0658,
        f: ee::s14::BuddyApi_SetPendingOp,
    };

    pub const PROTO_ARIES_UNCONNECT: Callee = Callee {
        addr: 0x003f_5738,
        f: ee::s14::ProtoAriesUnconnect,
    };

    pub const PROTO_ARIES_NOP: Callee = Callee {
        addr: 0x003f_5908,
        f: ee::s14::ProtoAries_Nop,
    };

    pub const PROTO_ARIES_CONNECT: Callee = Callee {
        addr: 0x003f_5638,
        f: ee::s14::ProtoAriesConnect,
    };

    pub const PROTO_ARIES_PEEK: Callee = Callee {
        addr: 0x003f_5a70,
        f: ee::s14::ProtoAriesPeek,
    };

    pub const PROTO_ARIES_RECV: Callee = Callee {
        addr: 0x003f_5c38,
        f: ee::s14::ProtoAriesRecv,
    };

    pub const BUDDY_API_STRIP_USER_NAME: Callee = Callee {
        addr: 0x003f_0150,
        f: ee::s14::BuddyApi_StripUserName,
    };

    pub const BUDDY_API_DISPATCH_MESSAGE: Callee = Callee {
        addr: 0x003f_0198,
        f: ee::s14::BuddyApi_DispatchMessage,
    };

    pub const BUDDY_API_PARSE_PRESENCE: Callee = Callee {
        addr: 0x003f_02d0,
        f: ee::s14::BuddyApi_ParsePresence,
    };

    pub const LOBBY_LIST_SORT: Callee = Callee {
        addr: 0x003e_e620,
        f: ee::s14::Lobby_ListSort,
    };

    pub const MEMCMP: Callee = Callee {
        addr: 0x0041_5fc8,
        f: ee::s15::memcmp,
    };

    pub const DIRTY_RPC_SEND: Callee = Callee {
        addr: 0x003f_4528,
        f: ee::s14::DirtyRpc_Send,
    };

    pub const DIRTY_RPC_PROCESS_REPLIES: Callee = Callee {
        addr: 0x003f_4a78,
        f: ee::s14::DirtyRpc_ProcessReplies,
    };

    pub const DIRTY_RPC_CALL: Callee = Callee {
        addr: 0x003f_4c68,
        f: ee::s14::DirtyRpc_Call,
    };

    pub const DIRTY_RPC_WAIT_IOP: Callee = Callee {
        addr: 0x003f_4f40,
        f: ee::s14::DirtyRpc_WaitIop,
    };

    pub const ADD_INTC_HANDLER: Callee = Callee {
        addr: 0x0042_3a90,
        f: ee::s15::f_00423a90,
    };

    pub const RELEASE_ALARM: Callee = Callee {
        addr: 0x0042_3b30,
        f: ee::s15::ReleaseAlarm,
    };

    pub const I_SET_ALARM: Callee = Callee {
        addr: 0x0042_3b80,
        f: ee::s15::iSetAlarm,
    };

    pub const CREATE_THREAD: Callee = Callee {
        addr: 0x0042_3ba0,
        f: ee::s15::CreateThread,
    };

    pub const DELETE_THREAD: Callee = Callee {
        addr: 0x0042_3bb0,
        f: ee::s15::DeleteThread,
    };

    pub const START_THREAD: Callee = Callee {
        addr: 0x0042_3bc0,
        f: ee::s15::_StartThread,
    };

    pub const TERMINATE_THREAD: Callee = Callee {
        addr: 0x0042_3bf0,
        f: ee::s15::TerminateThread,
    };

    pub const SLEEP_THREAD: Callee = Callee {
        addr: 0x0042_3cc0,
        f: ee::s15::SleepThread,
    };

    pub const WAKEUP_THREAD: Callee = Callee {
        addr: 0x0042_3cd0,
        f: ee::s15::WakeupThread,
    };

    pub const CREATE_SEMA: Callee = Callee {
        addr: 0x0042_3da0,
        f: ee::s15::CreateSema,
    };

    pub const DELETE_SEMA: Callee = Callee {
        addr: 0x0042_3db0,
        f: ee::s15::DeleteSema,
    };

    pub const FLUSH_CACHE: Callee = Callee {
        addr: 0x0042_4020,
        f: ee::s15::FlushCache,
    };

    pub const SCE_SIF_DMA_STAT: Callee = Callee {
        addr: 0x0042_4140,
        f: ee::s15::sceSifDmaStat,
    };

    pub const SCE_SIF_SET_DMA: Callee = Callee {
        addr: 0x0042_4160,
        f: ee::s15::sceSifSetDma,
    };

    pub const ENABLE_INTC: Callee = Callee {
        addr: 0x0042_48e8,
        f: ee::s15::EnableIntc,
    };

    pub const I_WAKEUP_THREAD: Callee = Callee {
        addr: 0x0042_4c50,
        f: ee::s15::iWakeupThread,
    };

    pub const SCE_SIF_INIT_CMD: Callee = Callee {
        addr: 0x0042_5d38,
        f: ee::s15::sceSifInitCmd,
    };

    pub const SCE_SIF_SEND_CMD: Callee = Callee {
        addr: 0x0042_61b0,
        f: ee::s15::sceSifSendCmd,
    };

    pub const SCE_SIF_ALLOC_IOP_HEAP: Callee = Callee {
        addr: 0x0042_aba0,
        f: ee::s15::sceSifAllocIopHeap,
    };

    pub const WAIT_SEMA: Callee = Callee {
        addr: 0x0042_3de0,
        f: ee::s15::WaitSema,
    };

    pub const SIGNAL_SEMA: Callee = Callee {
        addr: 0x0042_3dc0,
        f: ee::s15::SignalSema,
    };

    pub const GET_THREAD_ID: Callee = Callee {
        addr: 0x0042_3c90,
        f: ee::s15::GetThreadId,
    };

    pub const MEMCPY: Callee = Callee {
        addr: 0x0041_605c,
        f: ee::s15::memcpy,
    };

    pub const SYNC_DCACHE: Callee = Callee {
        addr: 0x0042_4698,
        f: ee::s15::SyncDCache,
    };

    pub const SET_ALARM: Callee = Callee {
        addr: 0x0042_3b20,
        f: ee::s15::SetAlarm,
    };

    pub const NET_TICK: Callee = Callee {
        addr: 0x003f_4de8,
        f: ee::s14::NetTick,
    };

    pub const NET_RPC_ALLOC: Callee = Callee {
        addr: 0x0025_6bb0,
        f: ee::s06::Net_RpcAlloc,
    };

    pub const NET_RPC_FREE: Callee = Callee {
        addr: 0x0025_6bd8,
        f: ee::s06::Net_RpcFree,
    };

    pub const DIRTY_RPC_INIT_REQUEST: Callee = Callee {
        addr: 0x003f_4c98,
        f: ee::s14::DirtyRpc_InitRequest,
    };

    pub const INVALID_DCACHE: Callee = Callee {
        addr: 0x0042_47d8,
        f: ee::s15::InvalidDCache,
    };

    pub const SCE_SIF_LOAD_MODULE: Callee = Callee {
        addr: 0x0042_ba68,
        f: ee::s15::sceSifLoadModule,
    };

    pub const SCE_SIF_STOP_MODULE: Callee = Callee {
        addr: 0x0042_b438,
        f: ee::s15::sceSifStopModule,
    };

    pub const SCE_SIF_UNLOAD_MODULE: Callee = Callee {
        addr: 0x0042_b640,
        f: ee::s15::sceSifUnloadModule,
    };

    pub const SCE_USB_KB_INIT: Callee = Callee {
        addr: 0x0040_c028,
        f: ee::s15::sceUsbKbInit,
    };

    pub const SCE_USB_KB_END: Callee = Callee {
        addr: 0x0040_c220,
        f: ee::s15::sceUsbKbEnd,
    };

    pub const SCE_USB_KB_GET_INFO: Callee = Callee {
        addr: 0x0040_c2e8,
        f: ee::s15::sceUsbKbGetInfo,
    };

    pub const SCE_USB_KB_READ: Callee = Callee {
        addr: 0x0040_c390,
        f: ee::s15::sceUsbKbRead,
    };

    pub const SCE_USB_KB_SET_LED_STATUS: Callee = Callee {
        addr: 0x0040_c550,
        f: ee::s15::sceUsbKbSetLEDStatus,
    };

    pub const SCE_USB_KB_SET_LED_MODE: Callee = Callee {
        addr: 0x0040_c628,
        f: ee::s15::sceUsbKbSetLEDMode,
    };

    pub const SCE_USB_KB_SET_REPEAT: Callee = Callee {
        addr: 0x0040_c728,
        f: ee::s15::sceUsbKbSetRepeat,
    };

    pub const SCE_USB_KB_SET_CODE_TYPE: Callee = Callee {
        addr: 0x0040_c770,
        f: ee::s15::sceUsbKbSetCodeType,
    };

    pub const SCE_USB_KB_SYNC: Callee = Callee {
        addr: 0x0040_c810,
        f: ee::s15::sceUsbKbSync,
    };

    pub const SCE_USB_KB_CNV_RAW_CODE: Callee = Callee {
        addr: 0x0040_d268,
        f: ee::s15::sceUsbKbCnvRawCode,
    };

    pub const SCE_USB_KB_CLEAR_RBUF: Callee = Callee {
        addr: 0x0040_d4a0,
        f: ee::s15::sceUsbKbClearRbuf,
    };

    pub const USB_KB_LOAD_MODULES: Callee = Callee {
        addr: 0x003f_5da8,
        f: ee::s14::UsbKb_LoadModules,
    };

    pub const USB_KB_UNLOAD_MODULE: Callee = Callee {
        addr: 0x003f_5e90,
        f: ee::s14::UsbKb_UnloadModule,
    };

    pub const USB_KB_REQUEST_INFO: Callee = Callee {
        addr: 0x003f_5f08,
        f: ee::s14::UsbKb_RequestInfo,
    };

    pub const USB_KB_SYNC_INFO: Callee = Callee {
        addr: 0x003f_5f88,
        f: ee::s14::UsbKb_SyncInfo,
    };

    pub const USB_KB_FIND_NEXT_READY: Callee = Callee {
        addr: 0x003f_6040,
        f: ee::s14::UsbKb_FindNextReady,
    };

    pub const USB_KB_REQUEST_READ: Callee = Callee {
        addr: 0x003f_60b8,
        f: ee::s14::UsbKb_RequestRead,
    };

    pub const USB_KB_SYNC_READ: Callee = Callee {
        addr: 0x003f_6138,
        f: ee::s14::UsbKb_SyncRead,
    };

    pub const USB_KB_GET_MAX_KEYBOARDS: Callee = Callee {
        addr: 0x003f_6348,
        f: ee::s14::UsbKb_GetMaxKeyboards,
    };

    pub const USB_KB_UPDATE: Callee = Callee {
        addr: 0x003f_63f0,
        f: ee::s14::UsbKb_Update,
    };

    pub const USB_KB_IS_CONNECTED: Callee = Callee {
        addr: 0x003f_6520,
        f: ee::s14::UsbKb_IsConnected,
    };

    pub const USB_KB_GET_KEYPAD_ENTRY: Callee = Callee {
        addr: 0x003f_65a0,
        f: ee::s14::UsbKb_GetKeypadEntry,
    };

    pub const USB_KB_LOOKUP_KEY_CHAR: Callee = Callee {
        addr: 0x003f_65f8,
        f: ee::s14::UsbKb_LookupKeyChar,
    };

    pub const USB_KB_CONVERT_RAW_CODE: Callee = Callee {
        addr: 0x003f_6670,
        f: ee::s14::UsbKb_ConvertRawCode,
    };

    pub const USB_KB_CHECK_MODIFIER_KEY: Callee = Callee {
        addr: 0x003f_6700,
        f: ee::s14::UsbKb_CheckModifierKey,
    };

    pub const USB_KB_CHECK_LED_KEY: Callee = Callee {
        addr: 0x003f_67e0,
        f: ee::s14::UsbKb_CheckLedKey,
    };

    pub const VOIP_RPC_CALL: Callee = Callee {
        addr: 0x003f_6960,
        f: ee::s14::VoipRpc_Call,
    };

    pub const VOIP_RPC_GET_SHARED_STATUS: Callee = Callee {
        addr: 0x003f_6c40,
        f: ee::s14::VoipRpc_GetSharedStatus,
    };

    pub const SCE_SIF_CHECK_STAT_RPC: Callee = Callee {
        addr: 0x0042_6f08,
        f: ee::s15::sceSifCheckStatRpc,
    };

    pub const SCE_SIF_CALL_RPC: Callee = Callee {
        addr: 0x0042_6d18,
        f: ee::s15::sceSifCallRpc,
    };

    pub const SCE_SIF_BIND_RPC: Callee = Callee {
        addr: 0x0042_6b48,
        f: ee::s15::sceSifBindRpc,
    };
}

// ---------------------------------------------------------------------------
// The BuddyApi object (`BuddyApiT`, 0x50c bytes, `BuddyApi_Create`).

/// The pending host lookup `Connect` starts (`SocketLookup`'s result), 0 when
/// none.
pub(crate) const BUDDY_LOOKUP: u32 = 0x0;

/// In the lookup object: the server address, the poll function and the
/// function that releases it.
pub(crate) const LOOKUP_ADDRESS: u32 = 0x4;

pub(crate) const LOOKUP_POLL: u32 = 0x8;

pub(crate) const LOOKUP_RELEASE: u32 = 0xc;

/// The `ProtoAries` connection (`Create`).
pub(crate) const BUDDY_ARIES: u32 = 0x4;

/// The connection state (`SetState`).
pub(crate) const BUDDY_STATE: u32 = 0x8;

/// The login deadline: the tick `Connect` started at plus its timeout in ms
/// (0: none); an `AUTH` reply clears it.
pub(crate) const BUDDY_LOGIN_DEADLINE: u32 = 0xc;

/// The server port (`Connect`'s third argument).
pub(crate) const BUDDY_PORT: u32 = 0x10;

/// The op counter: `Create` starts it at 100, `SetPendingOp` bumps it.
pub(crate) const BUDDY_OP_COUNTER: u32 = 0x14;

/// `Config`'s last argument: non-zero accepts buddy names containing a `.`.
pub(crate) const BUDDY_ACCEPT_DOTTED: u32 = 0x18;

/// The product id (`Config`); presence packets carry it as `P`.
pub(crate) const BUDDY_PRODUCT: u32 = 0x1c;

/// The presence text (0x100 bytes), sent as `PROD` in presence packets.
pub(crate) const BUDDY_PRESENCE_TEXT: u32 = 0x40;

/// The status record (a 0x100-byte tag record), sent as `STAT`; it carries
/// `P`.
pub(crate) const BUDDY_STATUS: u32 = 0x140;

/// The presence record (a tag record: `RSRC` and `SHOW`) `SendPresence`
/// starts its packet from.
pub(crate) const BUDDY_PRESENCE: u32 = 0x240;

/// The presence record's size (it ends at `BUDDY_PRESENCE_DUE`).
pub(crate) const BUDDY_PRESENCE_SIZE: u64 = 0x80;

/// The tick at which the presence goes out (0: none scheduled); cleared once
/// `SendPresence` has sent.
pub(crate) const BUDDY_PRESENCE_DUE: u32 = 0x2c0;

/// The last presence show value (`Connect` sets it to -1).
pub(crate) const BUDDY_PRESENCE_SHOW: u32 = 0x2c4;

/// `Config`'s first string (0x40 bytes), sent as `PRES` at connect;
/// `SetPresenceText` only sets the text while it is not empty.
pub(crate) const BUDDY_PRES: u32 = 0x2c8;

/// How many two-letter codes `Config` stored (at most 4).
pub(crate) const BUDDY_CODE_COUNT: u32 = 0x308;

/// The two-letter codes, one halfword each, first letter in the high byte.
pub(crate) const BUDDY_CODES: u32 = 0x30c;

/// `Create`'s first string (0x40 bytes), sent as `PROD` at connect.
pub(crate) const BUDDY_PRODUCT_NAME: u32 = 0x314;

/// `Create`'s second string (0x40 bytes), sent as `VERS` at connect.
pub(crate) const BUDDY_VERSION: u32 = 0x354;

/// The login record `Connect` builds (0x100 bytes), sent in the `AUTH`
/// packet.
pub(crate) const BUDDY_LOGIN: u32 = 0x394;

/// The user name `Connect` keeps (0x10 bytes): no leading `/`, nothing
/// from the next `/` on. Roster requests send it as `LRSC`.
pub(crate) const BUDDY_USER: u32 = 0x494;

/// The roster: a name hash (name -> list node) and the list of entries.
pub(crate) const BUDDY_ROSTER_HASH: u32 = 0x4a4;

pub(crate) const BUDDY_ROSTER_LIST: u32 = 0x4a8;

/// Message callback context and function (`SetCallback`): `RECV` (through
/// `DispatchMessage`) and `ADMN` messages.
pub(crate) const BUDDY_MESSAGE_ARG: u32 = 0x4ac;

pub(crate) const BUDDY_MESSAGE_FN: u32 = 0x4b0;

/// Roster callback context and function (`GetRoster`): the `rost` and
/// `endr` reports.
pub(crate) const BUDDY_ROSTER_ARG: u32 = 0x4b4;

pub(crate) const BUDDY_ROSTER_FN: u32 = 0x4b8;

/// Roster entries still expected (from an `RGET` reply's `SIZE`).
pub(crate) const BUDDY_ROSTER_LEFT: u32 = 0x4bc;

/// State callback context and function (`Connect`'s last two arguments;
/// `SetState` calls it); also gates the lookup poll.
pub(crate) const BUDDY_STATE_ARG: u32 = 0x4c0;

pub(crate) const BUDDY_STATE_FN: u32 = 0x4c4;

/// The three pending-op slots `SetPendingOp` fills (add, delete, send).
pub(crate) const BUDDY_OPS: u32 = 0x4c8;

pub(crate) const BUDDY_OP_SIZE: u32 = 0x14;

pub(crate) const BUDDY_OP_COUNT: i64 = 3;

/// In a pending-op slot: the op's four-char code, its id (the op counter's
/// value), its deadline (0 for none), the callback's argument and the
/// callback.
pub(crate) const BUDDY_OP_TAG: u32 = 0x0;

pub(crate) const BUDDY_OP_ID: u32 = 0x4;

pub(crate) const BUDDY_OP_DEADLINE: u32 = 0x8;

pub(crate) const BUDDY_OP_ARG: u32 = 0xc;

pub(crate) const BUDDY_OP_FN: u32 = 0x10;

/// Pending-op kinds (`SetPendingOp`'s second argument).
pub(crate) const BUDDY_OP_KIND_ADD: u64 = 0;

pub(crate) const BUDDY_OP_KIND_DEL: u64 = 1;

pub(crate) const BUDDY_OP_KIND_SEND: u64 = 2;

/// The op slots answered by `RADD`, `RDEL` and `SEND` replies.
pub(crate) const BUDDY_OP_SLOT_ADD: u32 = BUDDY_OPS;

pub(crate) const BUDDY_OP_SLOT_DEL: u32 = BUDDY_OPS + BUDDY_OP_SIZE;

pub(crate) const BUDDY_OP_SLOT_SEND: u32 = BUDDY_OPS + 2 * BUDDY_OP_SIZE;

/// Debug print hook: its context and function (0: no debug output).
pub(crate) const BUDDY_DEBUG_ARG: u32 = 0x504;

pub(crate) const BUDDY_DEBUG_FN: u32 = 0x508;

/// The object's size.
pub(crate) const BUDDY_SIZE: u32 = 0x50c;

/// A roster entry (0xb8 bytes, copied from the caller's buddy by `Add`): the
/// user name at 0 (at most 0x1f characters), the group string at 0x20, then
/// the presence `ParsePresence` fills: the product name (0x14 bytes), the
/// presence text (0x50 bytes), the language code's two letters and a NUL,
/// the show index, and the flags.
pub(crate) const ROSTER_ENTRY_SIZE: u64 = 0xb8;

pub(crate) const ROSTER_NAME_SIZE: u64 = 0x20;

pub(crate) const ROSTER_ENTRY_GROUP: u32 = 0x20;

pub(crate) const ROSTER_ENTRY_PRODUCT: u32 = 0x40;

pub(crate) const ROSTER_ENTRY_PRODUCT_SIZE: u32 = 0x14;

pub(crate) const ROSTER_ENTRY_PRESENCE: u32 = 0x54;

pub(crate) const ROSTER_ENTRY_PRESENCE_SIZE: u32 = 0x50;

pub(crate) const ROSTER_ENTRY_LANG: u32 = 0xa4;

pub(crate) const ROSTER_ENTRY_LANG_LO: u32 = 0xa5;

pub(crate) const ROSTER_ENTRY_LANG_END: u32 = 0xa6;

pub(crate) const ROSTER_ENTRY_SHOW: u32 = 0xa7;

pub(crate) const ROSTER_ENTRY_FLAGS: u32 = 0xa8;

/// Where `Process`'s 8-byte entry copy loop stops (the last 0x18 bytes
/// follow).
pub(crate) const ROSTER_ENTRY_COPY_END: u32 = 0xa0;

/// The id of the op that last changed the entry (`Add`, `Del`); an `RADD`
/// reply finds its entry by it.
pub(crate) const ROSTER_ENTRY_OP_ID: u32 = 0xac;

/// Entry flags: on the server lists `B` (set by a `ROST` entry of list id
/// 1), `I` (list id 2) and `P`; the presence packet had a `PROD` field; an
/// add was sent and waits for its reply.
pub(crate) const ROSTER_FLAG_LIST_B: u64 = 0x4;

pub(crate) const ROSTER_FLAG_LIST_I: u64 = 0x200;

pub(crate) const ROSTER_FLAG_LIST_P: u64 = 0x80_0000;

pub(crate) const ROSTER_FLAG_HAS_TEXT: u64 = 0x2000;

pub(crate) const ROSTER_FLAG_ADDING: u64 = 0x1_0000;

/// Entry flags that keep an entry in the roster after a delete.
pub(crate) const ROSTER_FLAG_KEEP: u64 = 0x10_0204;

/// The mask a `ROST` entry's flag update applies (it clears 0x10_0000).
pub(crate) const ROSTER_FLAG_ROST_CLEARS: u64 = 0xffff_ffff_ffef_ffff;

/// Sizes of the strings `Create`/`Config` copy, and of the login record and
/// user name `Connect` builds.
pub(crate) const BUDDY_STRING_SIZE: u32 = 0x40;

pub(crate) const BUDDY_LOGIN_SIZE: u32 = 0x100;

pub(crate) const BUDDY_USER_SIZE: u32 = 0x10;

/// The receive-buffer size `Create` asks `ProtoAriesCreate` for.
pub(crate) const BUDDY_ARIES_BUFFER: u32 = 0x8000;

/// The op counter's first value.
pub(crate) const BUDDY_FIRST_OP: u32 = 100;

/// The state `Connect` sets once the lookup is under way.
pub(crate) const BUDDY_STATE_CONNECT_STARTED: u32 = 1;

/// At most this many two-letter codes.
pub(crate) const BUDDY_MAX_CODES: i64 = 4;

/// Codes are lowercase letters: a byte below `'a'` ends them.
pub(crate) const BUDDY_CODE_FIRST_LETTER: i64 = b'a' as i64;

pub(crate) const BUDDY_MS_PER_SECOND: u64 = 1000;

/// A packet, message or event record: its four-char type, kind, code and
/// data pointer.
pub(crate) const BUDDY_PACKET_TYPE: u32 = 0x0;

pub(crate) const BUDDY_PACKET_KIND: u32 = 0x4;

pub(crate) const BUDDY_PACKET_CODE: u32 = 0x8;

pub(crate) const BUDDY_PACKET_DATA: u32 = 0xc;

// Four-char codes, as the words the code compares and stores.
/// "chat"
pub(crate) const BUDDY_TYPE_CHAT: u32 = 0x6368_6174;

/// "conn"
pub(crate) const BUDDY_EVENT_CONN: u32 = 0x636f_6e6e;

/// "disc"
pub(crate) const BUDDY_CODE_DISC: u32 = 0x6469_7363;

/// "PSET"
pub(crate) const BUDDY_KIND_PSET: u32 = 0x5053_4554;

/// "----"
pub(crate) const BUDDY_CODE_NONE: u32 = 0x2d2d_2d2d;

/// "****"
pub(crate) const BUDDY_KIND_EVENT: u32 = 0x2a2a_2a2a;

/// The code of a link-status packet (kind -1) when the link went down;
/// `DebugPacket` prints it as "disc".
pub(crate) const BUDDY_CODE_DISCONNECTED: u32 = 0xfefe_fefe;

// Tag names (game strings).
/// "PASS"
pub(crate) const BUDDY_TAG_PASS: u32 = 0x004a_4aa0;

/// "USER"
pub(crate) const BUDDY_TAG_USER: u32 = 0x004a_4ae0;

/// ""
pub(crate) const BUDDY_STR_EMPTY: u32 = 0x004a_4ae8;

/// "DOMN"
pub(crate) const BUDDY_TAG_DOMN: u32 = 0x004a_4af0;

/// "RSRC"
pub(crate) const BUDDY_TAG_RSRC: u32 = 0x004a_4af8;

/// "SHOW"
pub(crate) const BUDDY_TAG_SHOW: u32 = 0x004a_4b00;

/// "PROD"
pub(crate) const BUDDY_TAG_PROD: u32 = 0x004a_4b08;

/// "STAT"
pub(crate) const BUDDY_TAG_STAT: u32 = 0x004a_4b10;

/// "P"
pub(crate) const BUDDY_TAG_P: u32 = 0x004a_4b18;

/// "VERS"
pub(crate) const BUDDY_TAG_VERS: u32 = 0x004a_4b20;

/// "PRES"
pub(crate) const BUDDY_TAG_PRES: u32 = 0x004a_4b28;

/// "LKEY"
pub(crate) const BUDDY_TAG_LKEY: u32 = 0x004a_4b30;

// Debug formats (game strings).
/// "%s%c%c%c%c/%c%c%c%c\n"
pub(crate) const BUDDY_FMT_PACKET_HEADER: u32 = 0x0049_6b58;

/// " [null data]\n"
pub(crate) const BUDDY_FMT_NULL_DATA: u32 = 0x0049_6b70;

/// " [binary data]\n"
pub(crate) const BUDDY_FMT_BINARY_DATA: u32 = 0x0049_6b80;

/// " [long data]\n"
pub(crate) const BUDDY_FMT_LONG_DATA: u32 = 0x0049_6b90;

/// "%s\n"
pub(crate) const BUDDY_FMT_LINE: u32 = 0x004a_4ad0;

/// "send: "
pub(crate) const BUDDY_FMT_SEND: u32 = 0x004a_4ad8;

/// Game tables: the pending-op codes (`addu`, `delu`, `send`), the six
/// presence show names (`DISC`, `CHAT`, `AWAY`, `XA`, `DND`, `PASS`;
/// `SetPresence` takes index 0 for a show out of range) and the product
/// table `LookupProduct` searches.
pub(crate) const BUDDY_OP_CODES: u32 = 0x0045_12b0;

pub(crate) const BUDDY_SHOW_NAMES: u32 = 0x0045_12c0;

pub(crate) const BUDDY_SHOW_NAME_COUNT: i64 = 6;

pub(crate) const BUDDY_PRODUCT_TABLE: u32 = 0x0045_12d8;

/// A product-table entry: the product id, a code (halfword) and the name
/// pointer; the table ends at an id of 0.
pub(crate) const BUDDY_PRODUCT_ENTRY_SIZE: u32 = 0xc;

pub(crate) const PRODUCT_ENTRY_CODE: u32 = 0x4;

pub(crate) const PRODUCT_ENTRY_NAME: u32 = 0x8;

/// `lui` of an address or four-char code: the high half the register holds
/// before the `addiu`/`ori` (every low half here is below 0x8000).
pub(crate) fn buddy_lui(word: u32) -> u64 {
    sx(word & 0xffff_0000)
}

/// `addiu rd, base, %lo(addr)` from a register holding `addr`'s high half.
pub(crate) fn buddy_lo(base: u64, addr: u32) -> u64 {
    addu(base, (addr & 0xffff) as u64)
}

/// The field at `off` of the object whose address is in register `reg`.
pub(crate) fn buddy_field(c: &Cpu, reg: usize, off: u32) -> Addr {
    Addr(r32(c, reg).wrapping_add(off))
}

// ---------------------------------------------------------------------------
// BuddyApi_LookupProduct (0x3efe20)

pub(crate) const BUDDY_LOOKUP_ENTRY_HEAD: u32 = 0x003e_fe58;

pub(crate) const BUDDY_LOOKUP_CODE_HEAD: u32 = 0x003e_fe78;

/// `int BuddyApi_LookupProduct(BuddyApi* self, u32 product, const s16* codes,
/// int count, char* name, int size)` (0x3efe20): find the product table
/// entry for `product` whose code is one of the `count` codes, copy its name
/// into `name` and return the code; 0 when there is none. (`self` is not
/// read; the last two arguments are in t0/t1.)
pub fn buddy_api_lookup_product(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        c.g[V1][0] = buddy_lui(BUDDY_PRODUCT_TABLE);
        let frame = Frame::open(c, 0x20);
        c.g[T8][0] = c.g[A1][0];
        c.g[V0][0] = Addr(BUDDY_PRODUCT_TABLE).lw(c);
        frame.save(c, S0, 0x10);
        c.g[T7][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[T0][0];
        let empty = c.g[V0][0] == 0;
        c.g[A1][0] = c.g[T1][0];
        'found: {
            if !empty {
                // t4: the entry, t5 the entry for the copy, t6 its name slot.
                c.cycles += 4;
                c.g[T4][0] = sx(BUDDY_PRODUCT_TABLE);
                c.g[T6][0] = sx(BUDDY_PRODUCT_TABLE + PRODUCT_ENTRY_NAME);
                c.g[T5][0] = c.g[T4][0];
                c.g[V0][0] = Addr(r32(c, T4)).lw(c);
                loop {
                    c.cycles += 2;
                    if c.g[V0][0] != c.g[T8][0] {
                        c.g[T4][0] = addu(c.g[T4][0], BUDDY_PRODUCT_ENTRY_SIZE as u64);
                    } else {
                        c.cycles += 2;
                        let no_codes = (c.g[A3][0] as i64) <= 0;
                        c.g[T3][0] = 0;
                        if !no_codes {
                            c.cycles += 4;
                            c.g[S0][0] = c.g[T5][0];
                            c.g[A2][0] = c.g[T6][0];
                            c.g[T2][0] = c.g[T7][0];
                            loop {
                                c.cycles += 4;
                                c.g[V1][0] = Addr(r32(c, T2)).lh(c);
                                c.g[V0][0] = buddy_field(c, S0, PRODUCT_ENTRY_CODE).lh(c);
                                let differ = c.g[V1][0] != c.g[V0][0];
                                c.g[T3][0] = addu(c.g[T3][0], 1);
                                if !differ {
                                    c.cycles += 4;
                                    c.g[A2][0] = Addr(r32(c, A2)).lw(c);
                                    callee::TAG_FIELD_STRNZCPY.call(c, 0x003e_fe90)?;
                                    c.g[V0][0] = buddy_field(c, S0, PRODUCT_ENTRY_CODE).lh(c);
                                    break 'found;
                                }
                                c.cycles += 3;
                                c.g[V0][0] = ((c.g[T3][0] as i64) < (c.g[A3][0] as i64)) as u64;
                                let more = c.g[V0][0] != 0;
                                c.g[T2][0] = addu(c.g[T2][0], 2);
                                if !more {
                                    break;
                                }
                                if event_due(c) {
                                    deliver_event_here(c, BUDDY_LOOKUP_CODE_HEAD);
                                }
                            }
                        }
                        c.cycles += 1;
                        c.g[T4][0] = addu(c.g[T4][0], BUDDY_PRODUCT_ENTRY_SIZE as u64);
                    }
                    c.cycles += 4;
                    c.g[T6][0] = addu(c.g[T6][0], BUDDY_PRODUCT_ENTRY_SIZE as u64);
                    c.g[V0][0] = Addr(r32(c, T4)).lw(c);
                    let more = c.g[V0][0] != 0;
                    c.g[T5][0] = addu(c.g[T5][0], BUDDY_PRODUCT_ENTRY_SIZE as u64);
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, BUDDY_LOOKUP_ENTRY_HEAD);
                    }
                }
            }
            c.cycles += 1;
            c.g[V0][0] = 0;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// BuddyApi_DebugPrintf (0x3efed0)

/// Where `DebugPrintf` spills its variable arguments (a2..t3, then the
/// caller's stack words follow) and the float arguments f12/f14/f16/f18.
pub(crate) const BUDDY_PRINTF_VARARGS: u32 = 0x1060;

pub(crate) const BUDDY_PRINTF_FLOATS: u32 = 0x1050;

/// `void BuddyApi_DebugPrintf(BuddyApi* self, const char* fmt, ...)`
/// (0x3efed0): format into a 4 KiB stack buffer with `vsprintf`, then pass
/// the text to the debug hook at +0x508 (with its context from +0x504) when
/// one is set.
pub fn buddy_api_debug_printf(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x1090);
        frame.save(c, S0, 0x1010);
        c.g[S0][0] = c.g[A0][0];
        c.sd(frame.at(c, BUDDY_PRINTF_VARARGS), c.g[A2][0]);
        frame.save_ra(c, 0x1000);
        c.g[A0][0] = c.g[SP][0];
        c.sd(frame.at(c, BUDDY_PRINTF_VARARGS + 8), c.g[A3][0]);
        c.g[A2][0] = sx(frame.at(c, BUDDY_PRINTF_VARARGS));
        for (i, reg) in [T0, T1, T2, T3].into_iter().enumerate() {
            let slot = frame.at(c, BUDDY_PRINTF_VARARGS + 0x10 + 8 * i as u32);
            c.sd(slot, c.g[reg][0]);
        }
        for (i, f) in [12, 14, 16, 18].into_iter().enumerate() {
            frame.save_fpr(c, f, BUDDY_PRINTF_FLOATS + 4 * i as u32);
        }
        callee::VSPRINTF.call(c, 0x003e_ff14)?;
        c.g[V0][0] = buddy_field(c, S0, BUDDY_DEBUG_FN).lw(c);
        if c.g[V0][0] != 0 {
            c.cycles += 4;
            c.g[A0][0] = buddy_field(c, S0, BUDDY_DEBUG_ARG).lw(c);
            let hook = r32(c, V0);
            c.g[A1][0] = c.g[SP][0];
            call_ptr(c, hook, 0x003e_ff2c)?;
        }
        frame.restore(c, S0, 0x1010);
        c.cycles += 3;
        frame.restore_ra(c, 0x1000);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// BuddyApi_DebugPacket (0x3eff40)

pub(crate) const BUDDY_DEBUG_PACKET_SCAN_HEAD: u32 = 0x003f_0010;

/// Data longer than this prints as "[long data]".
pub(crate) const BUDDY_DEBUG_MAX_TEXT: i64 = 513;

/// The first byte that ends printable data.
pub(crate) const BUDDY_DEBUG_DEL: i64 = 127;

/// `void BuddyApi_DebugPacket(BuddyApi* self, const char* prefix,
/// const Packet* packet)` (0x3eff40): print the packet's kind and code as
/// four characters each (an event, kind -1, prints as `****` with
/// `conn`/`disc`/`----`; a code of 0 as `----`), then its data. Empty data
/// or data starting with DEL (0x7f) prints "[null data]". Otherwise the bytes
/// up to the end or the first DEL are checked: a control character other than
/// tab, LF or CR (or a byte of 0x80 and above) prints "[binary data]", more
/// than 512 of them "[long data]"; else the whole string prints as text,
/// including anything after a DEL, which is not checked.
pub fn buddy_api_debug_packet(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x20);
        c.g[T4][0] = buddy_field(c, A2, BUDDY_PACKET_CODE).lw(c);
        c.g[S1][0] = buddy_field(c, A2, BUDDY_PACKET_DATA).lw(c);
        c.g[T1][0] = buddy_field(c, A2, BUDDY_PACKET_KIND).lw(c);
        if c.g[T4][0] == 0 {
            c.g[T4][0] = sx(BUDDY_CODE_NONE);
        }
        c.g[V0][0] = BUDDY_KIND_LINK;
        let event = c.g[T1][0] == c.g[V0][0];
        c.g[A2][0] = c.g[A1][0];
        if event {
            c.cycles += 3;
            c.g[T1][0] = sx(BUDDY_KIND_EVENT);
            if c.g[T4][0] == c.g[V0][0] {
                c.cycles += 3;
                c.g[T4][0] = sx(BUDDY_EVENT_CONN);
            } else {
                c.cycles += 4;
                c.g[V0][0] = sx(BUDDY_CODE_DISCONNECTED);
                if c.g[T4][0] != c.g[V0][0] {
                    c.cycles += 1;
                    c.g[T4][0] = sx(BUDDY_CODE_NONE);
                } else {
                    c.cycles += 3;
                    c.g[T4][0] = sx(BUDDY_CODE_DISC);
                }
            }
        }
        // The header: prefix, then kind and code one byte each (the code's
        // last three bytes go on the stack).
        c.cycles += 18;
        let kind = r32(c, T1) as i32;
        let code = r32(c, T4) as i32;
        c.g[V0][0] = sx((code >> 16) as u32);
        c.g[T0][0] = sx((kind >> 16) as u32);
        c.g[V1][0] = sx((code >> 8) as u32);
        c.g[T2][0] = c.g[T1][0];
        c.g[A3][0] = sx((kind >> 24) as u32);
        Addr(frame.at(c, 0)).sw(c, c.g[V0][0]);
        c.g[A1][0] = sx(BUDDY_FMT_PACKET_HEADER);
        Addr(frame.at(c, 8)).sw(c, c.g[V1][0]);
        c.g[T3][0] = sx((code >> 24) as u32);
        Addr(frame.at(c, 0x10)).sw(c, c.g[T4][0]);
        c.g[A0][0] = c.g[S0][0];
        c.g[T1][0] = sx((kind >> 8) as u32);
        callee::BUDDY_API_DEBUG_PRINTF.call(c, 0x003e_ffe8)?;
        // Classify the data: a2 counts the printable run, t0 = 1 for a
        // control character other than tab, LF or CR.
        c.g[V0][0] = Addr(r32(c, S1)).lb(c);
        c.g[A2][0] = 0;
        let empty = c.g[V0][0] == 0;
        c.g[T0][0] = 0;
        'classified: {
            if empty {
                break 'classified;
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[V0][0] as i64) < BUDDY_DEBUG_DEL) as u64;
            if c.g[V0][0] == 0 {
                break 'classified;
            }
            c.cycles += 3;
            c.g[A3][0] = b'\r' as u64;
            c.g[V1][0] = c.g[S1][0];
            c.g[A1][0] = Addr(r32(c, S1)).lb(c);
            loop {
                'next: {
                    'advance: {
                        c.cycles += 3;
                        c.g[V0][0] = ((c.g[A1][0] as i64) < b' ' as i64) as u64;
                        let printable = c.g[V0][0] == 0;
                        c.g[A0][0] = Addr(r32(c, V1)).lbu(c);
                        if printable {
                            break 'advance;
                        }
                        c.cycles += 4;
                        c.g[V0][0] = sx(r32(c, A0).wrapping_sub(b'\t' as u32));
                        c.g[V0][0] = (c.g[V0][0] < 2) as u64;
                        if c.g[V0][0] != 0 {
                            // Tab or LF.
                            c.g[V1][0] = addu(c.g[V1][0], 1);
                            break 'next;
                        }
                        c.cycles += 2;
                        if c.g[A1][0] != c.g[A3][0] {
                            c.g[T0][0] = 1;
                            break 'classified;
                        }
                    }
                    c.cycles += 1;
                    c.g[V1][0] = addu(c.g[V1][0], 1);
                }
                c.cycles += 3;
                c.g[V0][0] = Addr(r32(c, V1)).lb(c);
                let end = c.g[V0][0] == 0;
                c.g[A2][0] = addu(c.g[A2][0], 1);
                if end {
                    break 'classified;
                }
                c.cycles += 3;
                c.g[V0][0] = ((c.g[V0][0] as i64) < BUDDY_DEBUG_DEL) as u64;
                if c.g[V0][0] == 0 {
                    break 'classified;
                }
                c.g[A1][0] = Addr(r32(c, V1)).lb(c);
                if event_due(c) {
                    deliver_event_here(c, BUDDY_DEBUG_PACKET_SCAN_HEAD);
                }
            }
        }
        'printed: {
            c.cycles += 2;
            let none = c.g[A2][0] == 0;
            c.g[V0][0] = 1;
            if none {
                c.cycles += 6;
                c.g[A1][0] = sx(BUDDY_FMT_NULL_DATA);
                c.g[A0][0] = c.g[S0][0];
                callee::BUDDY_API_DEBUG_PRINTF.call(c, 0x003f_0068)?;
                frame.restore(c, S0, 0x40);
                break 'printed;
            }
            c.cycles += 2;
            let binary = c.g[T0][0] == c.g[V0][0];
            c.g[V0][0] = ((c.g[A2][0] as i64) < BUDDY_DEBUG_MAX_TEXT) as u64;
            if binary {
                c.cycles += 6;
                c.g[A1][0] = sx(BUDDY_FMT_BINARY_DATA);
                c.g[A0][0] = c.g[S0][0];
                callee::BUDDY_API_DEBUG_PRINTF.call(c, 0x003f_0088)?;
                frame.restore(c, S0, 0x40);
                break 'printed;
            }
            c.cycles += 2;
            let short = c.g[V0][0] != 0;
            c.g[A1][0] = buddy_lui(BUDDY_FMT_LINE);
            if !short {
                c.cycles += 6;
                c.g[A1][0] = sx(BUDDY_FMT_LONG_DATA);
                c.g[A0][0] = c.g[S0][0];
                callee::BUDDY_API_DEBUG_PRINTF.call(c, 0x003f_00a8)?;
                frame.restore(c, S0, 0x40);
                break 'printed;
            }
            c.cycles += 5;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = sx(BUDDY_FMT_LINE);
            c.g[A2][0] = c.g[S1][0];
            callee::BUDDY_API_DEBUG_PRINTF.call(c, 0x003f_00c0)?;
            frame.restore(c, S0, 0x40);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// BuddyApi_SendPacket (0x3f00d8)

/// `void BuddyApi_SendPacket(BuddyApi* self, u32 kind, const char* data)`
/// (0x3f00d8): send `data` (NUL-terminated) as a `kind` packet on the Aries
/// connection, then print it through `DebugPacket` ("send: ") when the
/// debug hook is set.
pub fn buddy_api_send_packet(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x50);
        c.g[T0][0] = u64::MAX;
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0x10);
        c.g[S2][0] = c.g[A2][0];
        c.g[A2][0] = 0;
        c.g[A0][0] = buddy_field(c, S0, BUDDY_ARIES).lw(c);
        c.g[A3][0] = c.g[S2][0];
        callee::PROTO_ARIES_SEND.call(c, 0x003f_010c)?;
        c.g[V1][0] = buddy_field(c, S0, BUDDY_DEBUG_FN).lw(c);
        let quiet = c.g[V1][0] == 0;
        c.g[A1][0] = buddy_lui(BUDDY_FMT_SEND);
        if !quiet {
            // The packet record on the stack: {0, kind, 0, data}.
            c.cycles += 8;
            Addr(frame.at(c, BUDDY_PACKET_KIND)).sw(c, c.g[S1][0]);
            Addr(frame.at(c, BUDDY_PACKET_DATA)).sw(c, c.g[S2][0]);
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = sx(BUDDY_FMT_SEND);
            Addr(frame.at(c, BUDDY_PACKET_CODE)).sw(c, 0);
            Addr(frame.at(c, BUDDY_PACKET_TYPE)).sw(c, 0);
            c.g[A2][0] = c.g[SP][0];
            callee::BUDDY_API_DEBUG_PACKET.call(c, 0x003f_0138)?;
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// BuddyApi_StripUserName (0x3f0150)

/// `void BuddyApi_StripUserName(char* name)` (0x3f0150): cut the name at its
/// first `@`, then at its first `/`, in place.
pub fn buddy_api_strip_user_name(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = b'@' as u64;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        callee::STRCHR.call(c, 0x003f_0168)?;
        if c.g[V0][0] != 0 {
            Addr(r32(c, V0)).sb(c, 0);
        }
        c.cycles += 5;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = b'/' as u64;
        callee::STRCHR.call(c, 0x003f_017c)?;
        if c.g[V0][0] != 0 {
            Addr(r32(c, V0)).sb(c, 0);
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// BuddyApi_DispatchMessage (0x3f0198)

/// The stack copy of the message's tags and the user name buffer.
pub(crate) const BUDDY_MSG_TAGS_SIZE: u32 = 0x800;

pub(crate) const BUDDY_MSG_USER: u32 = 0x800;

pub(crate) const BUDDY_MSG_USER_SIZE: u32 = 0x100;

/// `void BuddyApi_DispatchMessage(BuddyApi* self, Message* msg)` (0x3f0198):
/// copy the message's tags to the stack, split its `USER` value
/// `name@domain/resource` into `USER`, `DOMN` and `RSRC` fields, point the
/// message at the copy, mark it `chat` and pass it to the message callback
/// (`cb(self, msg, ctx)`).
pub fn buddy_api_dispatch_message(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 35;
        let frame = Frame::open(c, 0x960);
        frame.save(c, S0, 0x950);
        frame.save(c, S1, 0x940);
        frame.save(c, S2, 0x930);
        frame.save(c, S3, 0x920);
        frame.save(c, S4, 0x910);
        c.g[S3][0] = c.g[A1][0];
        frame.save_ra(c, 0x900);
        c.g[S4][0] = c.g[A0][0];
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] = BUDDY_MSG_TAGS_SIZE as u64;
        c.g[A2][0] = buddy_field(c, S3, BUDDY_PACKET_DATA).lw(c);
        callee::TAG_FIELD_STRNZCPY.call(c, 0x003f_01cc)?;
        c.g[V1][0] = buddy_lui(BUDDY_TAG_USER);
        c.g[A0][0] = c.g[SP][0];
        c.g[S2][0] = sx(BUDDY_TAG_USER);
        c.g[A1][0] = c.g[S2][0];
        callee::TAG_FIELD_FIND.call(c, 0x003f_01e0)?;
        c.g[S0][0] = sx(frame.at(c, BUDDY_MSG_USER));
        c.g[A3][0] = sx(BUDDY_STR_EMPTY);
        c.g[A2][0] = BUDDY_MSG_USER_SIZE as u64;
        c.g[A1][0] = c.g[S0][0];
        c.g[A0][0] = c.g[V0][0];
        callee::TAG_FIELD_GET_STRING.call(c, 0x003f_01fc)?;
        c.g[S1][0] = c.g[S0][0];
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] = c.g[S2][0];
        callee::TAG_FIELD_DELETE.call(c, 0x003f_020c)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = b'@' as u64;
        callee::STRCHR.call(c, 0x003f_0218)?;
        c.g[S0][0] = c.g[V0][0];
        if c.g[S0][0] != 0 {
            Addr(r32(c, S0)).sb(c, 0);
        }
        c.cycles += 7;
        c.g[A2][0] = c.g[S2][0];
        c.g[A3][0] = c.g[S1][0];
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] = BUDDY_MSG_TAGS_SIZE as u64;
        callee::TAG_FIELD_SET_STRING.call(c, 0x003f_0238)?;
        let no_domain = c.g[S0][0] == 0;
        c.g[S1][0] = addu(c.g[S0][0], 1);
        if !no_domain {
            c.cycles += 6;
            c.g[A1][0] = b'/' as u64;
            c.g[A0][0] = c.g[S1][0];
            callee::STRCHR.call(c, 0x003f_024c)?;
            c.g[S0][0] = c.g[V0][0];
            if c.g[S0][0] != 0 {
                Addr(r32(c, S0)).sb(c, 0);
            }
            c.cycles += 8;
            c.g[A2][0] = sx(BUDDY_TAG_DOMN);
            c.g[A3][0] = c.g[S1][0];
            c.g[A0][0] = c.g[SP][0];
            c.g[A1][0] = BUDDY_MSG_TAGS_SIZE as u64;
            callee::TAG_FIELD_SET_STRING.call(c, 0x003f_0270)?;
            let no_resource = c.g[S0][0] == 0;
            c.g[A2][0] = buddy_lui(BUDDY_TAG_RSRC);
            if !no_resource {
                c.cycles += 5;
                c.g[A3][0] = addu(c.g[S0][0], 1);
                c.g[A2][0] = sx(BUDDY_TAG_RSRC);
                c.g[A0][0] = c.g[SP][0];
                c.g[A1][0] = BUDDY_MSG_TAGS_SIZE as u64;
                callee::TAG_FIELD_SET_STRING.call(c, 0x003f_028c)?;
            }
        }
        c.cycles += 17;
        buddy_field(c, S3, BUDDY_PACKET_DATA).sw(c, c.g[SP][0]);
        c.g[V0][0] = BUDDY_TYPE_CHAT as u64;
        c.g[A0][0] = c.g[S4][0];
        buddy_field(c, S3, BUDDY_PACKET_TYPE).sw(c, c.g[V0][0]);
        c.g[A1][0] = c.g[S3][0];
        c.g[V0][0] = buddy_field(c, S4, BUDDY_MESSAGE_FN).lw(c);
        let callback = r32(c, V0);
        c.g[A2][0] = buddy_field(c, A0, BUDDY_MESSAGE_ARG).lw(c);
        call_ptr(c, callback, 0x003f_02b0)?;
        frame.restore(c, S0, 0x950);
        frame.restore(c, S1, 0x940);
        frame.restore(c, S2, 0x930);
        frame.restore(c, S3, 0x920);
        frame.restore(c, S4, 0x910);
        frame.restore_ra(c, 0x900);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// BuddyApi_ParsePresence (0x3f02d0)

/// `ParsePresence`'s entry (its dispatch's first arm).
pub(crate) const BUDDY_PARSE_ENTRY: u32 = callee::BUDDY_API_PARSE_PRESENCE.addr;

/// Back-edge heads (the function is re-entered there after an unwind): the
/// state-name scan, a code found in the status record, and the code scan.
pub(crate) const BUDDY_PARSE_STATE_HEAD: u32 = 0x003f_0350;

pub(crate) const BUDDY_PARSE_CODE_FOUND: u32 = 0x003f_03f8;

pub(crate) const BUDDY_PARSE_CODE_HEAD: u32 = 0x003f_04b0;

pub(crate) const BUDDY_PARSE_END: u32 = 0x003f_0564;

pub(crate) const BUDDY_PARSE_FRAME: u32 = 0x1b0;

/// Stack buffers: the code being looked up (two letters and a NUL), the
/// `SHOW`/`STAT` text (0x100 bytes) and the found code as a halfword.
pub(crate) const BUDDY_PARSE_CODE_NAME: u32 = 0x0;

pub(crate) const BUDDY_PARSE_TEXT: u32 = 0x10;

pub(crate) const BUDDY_PARSE_TEXT_SIZE: u32 = 0x100;

pub(crate) const BUDDY_PARSE_CODE: u32 = 0x110;

/// `void BuddyApi_ParsePresence(BuddyApi* self, Message* msg, Presence* out)`
/// (0x3f02d0): decode a presence packet: the `SHOW` state's index among the
/// six state names, the `PROD` text and, when there is no text, the product
/// `P` from the `STAT` record and the first of our codes the record has a
/// field for (its value replaces the text); then the product name and code
/// via `LookupProduct`.
///
/// Dispatcher-shaped: an event at a back edge may unwind, and the function
/// is re-entered at that head with its state in s0-s7.
pub fn buddy_api_parse_presence(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame {
            size: BUDDY_PARSE_FRAME,
        };
        let mut stage = c.pc;
        loop {
            stage = match stage {
                BUDDY_PARSE_ENTRY => {
                    c.cycles += 32;
                    let _ = Frame::open(c, BUDDY_PARSE_FRAME);
                    frame.save(c, S2, 0x180);
                    frame.save(c, S0, 0x1a0);
                    c.g[S2][0] = c.g[A2][0];
                    frame.save(c, S1, 0x190);
                    frame.save(c, S3, 0x170);
                    c.g[S1][0] = buddy_lui(BUDDY_STR_EMPTY);
                    frame.save(c, S4, 0x160);
                    c.g[S3][0] = 0;
                    frame.save(c, S6, 0x140);
                    c.g[S4][0] = c.g[A0][0];
                    frame.save(c, S5, 0x150);
                    c.g[S6][0] = c.g[A1][0];
                    frame.save(c, S7, 0x130);
                    frame.save_ra(c, 0x120);
                    c.g[A1][0] = sx(BUDDY_TAG_SHOW);
                    for off in [
                        ROSTER_ENTRY_SHOW,
                        ROSTER_ENTRY_LANG,
                        ROSTER_ENTRY_PRESENCE,
                        ROSTER_ENTRY_PRODUCT,
                    ] {
                        buddy_field(c, S2, off).sb(c, 0);
                    }
                    c.g[A0][0] = buddy_field(c, S6, BUDDY_PACKET_DATA).lw(c);
                    callee::TAG_FIELD_FIND.call(c, 0x003f_032c)?;
                    c.g[S0][0] = sx(frame.at(c, BUDDY_PARSE_TEXT));
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A2][0] = BUDDY_PARSE_TEXT_SIZE as u64;
                    c.g[A3][0] = buddy_lo(c.g[S1][0], BUDDY_STR_EMPTY);
                    callee::TAG_FIELD_GET_STRING.call(c, 0x003f_0344)?;
                    c.g[S5][0] = c.g[S0][0];
                    c.g[S0][0] = buddy_lui(BUDDY_TAG_PROD);
                    BUDDY_PARSE_STATE_HEAD
                }
                BUDDY_PARSE_STATE_HEAD => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[S3][0] as i64) < BUDDY_SHOW_NAME_COUNT) as u64;
                    let past_end = c.g[V0][0] == 0;
                    c.g[V0][0] = buddy_lui(BUDDY_SHOW_NAMES);
                    if !past_end {
                        c.cycles += 8;
                        c.g[V1][0] = sx(r32(c, S3) << 2);
                        c.g[V0][0] = sx(BUDDY_SHOW_NAMES);
                        c.g[A1][0] = c.g[S5][0];
                        c.g[V1][0] = addu(c.g[V1][0], c.g[V0][0]);
                        c.g[A0][0] = Addr(r32(c, V1)).lw(c);
                        callee::STRCMP.call(c, 0x003f_0374)?;
                        if c.g[V0][0] != 0 {
                            c.g[S3][0] = addu(c.g[S3][0], 1);
                            if back_edge_unwinds(c, BUDDY_PARSE_STATE_HEAD) {
                                return Ok(());
                            }
                            continue;
                        }
                        c.cycles += 1;
                        buddy_field(c, S2, ROSTER_ENTRY_SHOW).sb(c, c.g[S3][0]);
                    }
                    // The presence text.
                    c.cycles += 6;
                    c.g[A0][0] = buddy_field(c, S6, BUDDY_PACKET_DATA).lw(c);
                    c.g[A1][0] = buddy_lo(c.g[S0][0], BUDDY_TAG_PROD);
                    callee::TAG_FIELD_FIND.call(c, 0x003f_038c)?;
                    c.g[A0][0] = c.g[V0][0];
                    let no_text = c.g[A0][0] == 0;
                    c.g[A1][0] = addu(c.g[S2][0], ROSTER_ENTRY_PRESENCE as u64);
                    if !no_text {
                        c.cycles += 3;
                        c.g[V0][0] = buddy_field(c, S2, ROSTER_ENTRY_FLAGS).lw(c);
                        c.g[V0][0] |= ROSTER_FLAG_HAS_TEXT;
                        buddy_field(c, S2, ROSTER_ENTRY_FLAGS).sw(c, c.g[V0][0]);
                    }
                    c.cycles += 8;
                    c.g[S0][0] = buddy_lo(c.g[S1][0], BUDDY_STR_EMPTY);
                    c.g[S7][0] = c.g[A1][0];
                    c.g[A2][0] = ROSTER_ENTRY_PRESENCE_SIZE as u64;
                    c.g[A3][0] = c.g[S0][0];
                    callee::TAG_FIELD_GET_STRING.call(c, 0x003f_03b8)?;
                    c.g[V1][0] = buddy_field(c, S2, ROSTER_ENTRY_PRESENCE).lb(c);
                    let empty = c.g[V1][0] == 0;
                    c.g[A2][0] = addu(c.g[S4][0], BUDDY_CODES as u64);
                    if !empty {
                        // Text: our product, with our first code.
                        c.cycles += 13;
                        c.g[A1][0] = buddy_field(c, S4, BUDDY_PRODUCT).lw(c);
                        c.g[A0][0] = c.g[S4][0];
                        c.g[A3][0] = 1;
                        c.g[T0][0] = addu(c.g[S2][0], ROSTER_ENTRY_PRODUCT as u64);
                        c.g[T1][0] = ROSTER_ENTRY_PRODUCT_SIZE as u64;
                        callee::BUDDY_API_LOOKUP_PRODUCT.call(c, 0x003f_03dc)?;
                        c.g[V1][0] = buddy_field(c, S4, BUDDY_CODES).lhu(c);
                        c.g[V1][0] = sx(r32(c, V1) >> 8);
                        buddy_field(c, S2, ROSTER_ENTRY_LANG).sb(c, c.g[V1][0]);
                        c.g[V0][0] = buddy_field(c, S4, BUDDY_CODES).lh(c);
                        buddy_field(c, S2, ROSTER_ENTRY_LANG_END).sb(c, 0);
                        buddy_field(c, S2, ROSTER_ENTRY_LANG_LO).sb(c, c.g[V0][0]);
                        buddy_parse_presence_leave(c, frame);
                        return Ok(());
                    }
                    // No text: the product from the status record, then our
                    // codes against its fields.
                    c.cycles += 21;
                    c.g[A0][0] = buddy_field(c, S6, BUDDY_PACKET_DATA).lw(c);
                    c.g[A1][0] = sx(BUDDY_TAG_STAT);
                    c.g[S3][0] = 0;
                    callee::TAG_FIELD_FIND.call(c, 0x003f_046c)?;
                    c.g[A3][0] = c.g[S0][0];
                    c.g[A2][0] = BUDDY_PARSE_TEXT_SIZE as u64;
                    c.g[A1][0] = c.g[S5][0];
                    c.g[A0][0] = c.g[V0][0];
                    callee::TAG_FIELD_GET_STRING.call(c, 0x003f_0480)?;
                    c.g[A1][0] = sx(BUDDY_TAG_P);
                    c.g[A0][0] = c.g[S5][0];
                    callee::TAG_FIELD_FIND.call(c, 0x003f_0490)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A1][0] = 0;
                    callee::TAG_FIELD_GET_TOKEN.call(c, 0x003f_049c)?;
                    c.g[S6][0] = c.g[V0][0];
                    c.g[V0][0] = buddy_field(c, S4, BUDDY_CODE_COUNT).lw(c);
                    let no_codes = (c.g[V0][0] as i64) <= 0;
                    c.g[S0][0] = addu(c.g[S4][0], BUDDY_CODES as u64);
                    if no_codes {
                        buddy_parse_presence_no_code(c, frame)?;
                        return Ok(());
                    }
                    c.cycles += 1;
                    BUDDY_PARSE_CODE_HEAD
                }
                BUDDY_PARSE_CODE_HEAD => {
                    // Look the code at s0 up as a field name.
                    c.cycles += 13;
                    c.g[V0][0] = Addr(r32(c, S0)).lhu(c);
                    c.g[A0][0] = c.g[S5][0];
                    Addr(frame.at(c, BUDDY_PARSE_CODE_NAME + 2)).sb(c, 0);
                    c.g[A1][0] = c.g[SP][0];
                    c.g[V0][0] = sx(r32(c, V0) << 16);
                    c.g[V1][0] = sx((r32(c, V0) as i32 >> 24) as u32);
                    c.g[V0][0] = sx((r32(c, V0) as i32 >> 16) as u32);
                    Addr(frame.at(c, BUDDY_PARSE_CODE_NAME)).sb(c, c.g[V1][0]);
                    Addr(frame.at(c, BUDDY_PARSE_CODE_NAME + 1)).sb(c, c.g[V0][0]);
                    callee::TAG_FIELD_FIND.call(c, 0x003f_04d8)?;
                    c.g[A0][0] = c.g[V0][0];
                    let found = c.g[A0][0] != 0;
                    c.g[S3][0] = addu(c.g[S3][0], 1);
                    if found {
                        if back_edge_unwinds(c, BUDDY_PARSE_CODE_FOUND) {
                            return Ok(());
                        }
                        BUDDY_PARSE_CODE_FOUND
                    } else {
                        c.cycles += 4;
                        c.g[V0][0] = buddy_field(c, S4, BUDDY_CODE_COUNT).lw(c);
                        c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[V0][0] as i64)) as u64;
                        let more = c.g[V0][0] != 0;
                        c.g[S0][0] = addu(c.g[S0][0], 2);
                        if !more {
                            buddy_parse_presence_no_code(c, frame)?;
                            return Ok(());
                        }
                        if back_edge_unwinds(c, BUDDY_PARSE_CODE_HEAD) {
                            return Ok(());
                        }
                        BUDDY_PARSE_CODE_HEAD
                    }
                }
                BUDDY_PARSE_CODE_FOUND => {
                    // The field's value (a0) becomes the text; the code is the
                    // record's code.
                    c.cycles += 24;
                    c.g[A3][0] = buddy_lo(c.g[S1][0], BUDDY_STR_EMPTY);
                    c.g[A2][0] = ROSTER_ENTRY_PRESENCE_SIZE as u64;
                    c.g[A1][0] = c.g[S7][0];
                    callee::TAG_FIELD_GET_STRING.call(c, 0x003f_0408)?;
                    c.g[A3][0] = Addr(frame.at(c, BUDDY_PARSE_CODE_NAME + 1)).lbu(c);
                    c.g[A0][0] = c.g[S4][0];
                    c.g[A2][0] = Addr(frame.at(c, BUDDY_PARSE_CODE_NAME)).lbu(c);
                    c.g[A1][0] = c.g[S6][0];
                    c.g[V0][0] = sx(r32(c, A3) << 24);
                    buddy_field(c, S2, ROSTER_ENTRY_LANG_LO).sb(c, c.g[A3][0]);
                    c.g[V1][0] = sx(r32(c, A2) << 24);
                    c.g[V0][0] = sx((r32(c, V0) as i32 >> 8) as u32);
                    c.g[V0][0] |= c.g[V1][0];
                    buddy_field(c, S2, ROSTER_ENTRY_LANG).sb(c, c.g[A2][0]);
                    buddy_field(c, S2, ROSTER_ENTRY_LANG_END).sb(c, 0);
                    c.g[V0][0] = sx((r32(c, V0) as i32 >> 16) as u32);
                    Addr(frame.at(c, BUDDY_PARSE_CODE)).sh(c, c.g[V0][0]);
                    c.g[A2][0] = sx(frame.at(c, BUDDY_PARSE_CODE));
                    c.g[A3][0] = 1;
                    c.g[T0][0] = addu(c.g[S2][0], ROSTER_ENTRY_PRODUCT as u64);
                    c.g[T1][0] = ROSTER_ENTRY_PRODUCT_SIZE as u64;
                    callee::BUDDY_API_LOOKUP_PRODUCT.call(c, 0x003f_0450)?;
                    c.g[V0][0] = buddy_field(c, S2, ROSTER_ENTRY_LANG).lb(c);
                    buddy_parse_presence_finish(c, frame)?;
                    return Ok(());
                }
                pc => {
                    c.jump_out(pc, BUDDY_PARSE_ENTRY, BUDDY_PARSE_END);
                    return Ok(());
                }
            };
        }
    })
}

/// `ParsePresence` without a code found (0x3f04f4): reload the code's first
/// letter and finish.
pub(crate) fn buddy_parse_presence_no_code(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 1;
    c.g[V0][0] = buddy_field(c, S2, ROSTER_ENTRY_LANG).lb(c);
    buddy_parse_presence_finish(c, frame)
}

/// `ParsePresence`'s finish (0x3f04f8, v0 = the code's first letter): with
/// no code yet, the product `P` (s6) with any of our codes.
pub(crate) fn buddy_parse_presence_finish(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 2;
    let have_code = c.g[V0][0] != 0;
    frame.restore(c, S0, 0x1a0);
    if have_code {
        buddy_parse_presence_epilogue(c, frame);
        return Ok(());
    }
    c.cycles += 14;
    c.g[A0][0] = c.g[S4][0];
    c.g[A1][0] = c.g[S6][0];
    c.g[A3][0] = buddy_field(c, A0, BUDDY_CODE_COUNT).lw(c);
    c.g[A2][0] = addu(c.g[A0][0], BUDDY_CODES as u64);
    c.g[T0][0] = addu(c.g[S2][0], ROSTER_ENTRY_PRODUCT as u64);
    c.g[T1][0] = ROSTER_ENTRY_PRODUCT_SIZE as u64;
    callee::BUDDY_API_LOOKUP_PRODUCT.call(c, 0x003f_051c)?;
    buddy_field(c, S2, ROSTER_ENTRY_LANG_END).sb(c, 0);
    c.g[V1][0] = sx(r32(c, V0) << 16);
    Addr(frame.at(c, BUDDY_PARSE_CODE)).sh(c, c.g[V0][0]);
    c.g[V0][0] = sx((r32(c, V1) as i32 >> 24) as u32);
    c.g[V1][0] = sx((r32(c, V1) as i32 >> 16) as u32);
    buddy_field(c, S2, ROSTER_ENTRY_LANG).sb(c, c.g[V0][0]);
    buddy_field(c, S2, ROSTER_ENTRY_LANG_LO).sb(c, c.g[V1][0]);
    buddy_parse_presence_leave(c, frame);
    Ok(())
}

/// `ParsePresence`'s common exit (0x3f0538).
pub(crate) fn buddy_parse_presence_leave(c: &mut Cpu, frame: Frame) {
    c.cycles += 1;
    frame.restore(c, S0, 0x1a0);
    buddy_parse_presence_epilogue(c, frame);
}

/// `ParsePresence`'s epilogue (0x3f053c; s0 is already restored).
pub(crate) fn buddy_parse_presence_epilogue(c: &mut Cpu, frame: Frame) {
    c.cycles += 10;
    frame.restore(c, S1, 0x190);
    frame.restore(c, S2, 0x180);
    frame.restore(c, S3, 0x170);
    frame.restore(c, S4, 0x160);
    frame.restore(c, S5, 0x150);
    frame.restore(c, S6, 0x140);
    frame.restore(c, S7, 0x130);
    frame.restore_ra(c, 0x120);
    frame.close(c);
}

// ---------------------------------------------------------------------------
// BuddyApi_SendPresence (0x3f0568)

/// The record `SendPresence` builds on the stack.
pub(crate) const BUDDY_PRESENCE_PACKET_SIZE: u32 = 0x200;

pub(crate) const BUDDY_STATUS_SIZE: u32 = 0x100;

/// `void BuddyApi_SendPresence(BuddyApi* self)` (0x3f0568): put the product
/// into the status record as `P` (when there is one), build a `PSET` packet
/// from the presence record plus `STAT` (the status record) and `PROD` (the
/// presence text) when they are not empty, send it and clear +0x2c0.
pub fn buddy_api_send_presence(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x220);
        frame.save(c, S0, 0x210);
        frame.save_ra(c, 0x200);
        c.g[S0][0] = c.g[A0][0];
        c.g[A3][0] = buddy_field(c, S0, BUDDY_PRODUCT).lw(c);
        let no_product = c.g[A3][0] == 0;
        c.g[A2][0] = buddy_lui(BUDDY_TAG_P);
        if !no_product {
            c.cycles += 4;
            c.g[A0][0] = addu(c.g[S0][0], BUDDY_STATUS as u64);
            c.g[A2][0] = sx(BUDDY_TAG_P);
            c.g[A1][0] = BUDDY_STATUS_SIZE as u64;
            callee::TAG_FIELD_SET_TOKEN.call(c, 0x003f_0594)?;
        }
        c.cycles += 7;
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] = BUDDY_PRESENCE_PACKET_SIZE as u64;
        c.g[A2][0] = addu(c.g[S0][0], BUDDY_PRESENCE as u64);
        callee::TAG_FIELD_STRNZCPY.call(c, 0x003f_05a4)?;
        c.g[V1][0] = buddy_field(c, S0, BUDDY_STATUS).lb(c);
        let no_status = c.g[V1][0] == 0;
        c.g[A2][0] = buddy_lui(BUDDY_TAG_STAT);
        if !no_status {
            c.cycles += 5;
            c.g[A0][0] = c.g[SP][0];
            c.g[A2][0] = sx(BUDDY_TAG_STAT);
            c.g[A1][0] = BUDDY_PRESENCE_PACKET_SIZE as u64;
            c.g[A3][0] = addu(c.g[S0][0], BUDDY_STATUS as u64);
            callee::TAG_FIELD_SET_STRING.call(c, 0x003f_05c4)?;
        }
        c.cycles += 3;
        c.g[V0][0] = buddy_field(c, S0, BUDDY_PRESENCE_TEXT).lb(c);
        let no_text = c.g[V0][0] == 0;
        c.g[A2][0] = buddy_lui(BUDDY_TAG_PROD);
        if !no_text {
            c.cycles += 5;
            c.g[A0][0] = c.g[SP][0];
            c.g[A2][0] = sx(BUDDY_TAG_PROD);
            c.g[A1][0] = BUDDY_PRESENCE_PACKET_SIZE as u64;
            c.g[A3][0] = addu(c.g[S0][0], BUDDY_PRESENCE_TEXT as u64);
            callee::TAG_FIELD_SET_STRING.call(c, 0x003f_05e4)?;
        }
        c.cycles += 10;
        c.g[A1][0] = BUDDY_KIND_PSET as u64;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[SP][0];
        callee::BUDDY_API_SEND_PACKET.call(c, 0x003f_05f8)?;
        buddy_field(c, S0, BUDDY_PRESENCE_DUE).sw(c, 0);
        frame.restore(c, S0, 0x210);
        frame.restore_ra(c, 0x200);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// BuddyApi_SetState (0x3f0610)

/// `void BuddyApi_SetState(BuddyApi* self, int state)` (0x3f0610): store the
/// state and, when a state callback is set, call it with a `conn` event
/// record `{"conn", state, 0, 0}`: `cb(self, &event, ctx)`.
pub fn buddy_api_set_state(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        frame.save_ra(c, 0x10);
        buddy_field(c, A0, BUDDY_STATE).sw(c, c.g[A1][0]);
        c.g[V1][0] = buddy_field(c, A0, BUDDY_STATE_FN).lw(c);
        let no_callback = c.g[V1][0] == 0;
        c.g[V0][0] = buddy_lui(BUDDY_EVENT_CONN);
        if !no_callback {
            c.cycles += 8;
            Addr(frame.at(c, BUDDY_PACKET_KIND)).sw(c, c.g[A1][0]);
            c.g[V0][0] = BUDDY_EVENT_CONN as u64;
            Addr(frame.at(c, BUDDY_PACKET_CODE)).sw(c, 0);
            Addr(frame.at(c, BUDDY_PACKET_TYPE)).sw(c, c.g[V0][0]);
            c.g[A1][0] = c.g[SP][0];
            Addr(frame.at(c, BUDDY_PACKET_DATA)).sw(c, 0);
            let callback = r32(c, V1);
            c.g[A2][0] = buddy_field(c, A0, BUDDY_STATE_ARG).lw(c);
            call_ptr(c, callback, 0x003f_0648)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// BuddyApi_SetPendingOp (0x3f0658)

/// `void BuddyApi_SetPendingOp(BuddyApi* self, int op, void* callback,
/// void* context, int timeout)` (0x3f0658): fill op `op`'s pending slot with
/// its four-char code, the next op number, its deadline (the tick plus 1000 x
/// `timeout`, or 0 when `timeout` is not positive), the callback (+0x10) and
/// its context (+0xc).
pub fn buddy_api_set_pending_op(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S1, 0x40);
        frame.save(c, S0, 0x50);
        c.g[S1][0] = c.g[T0][0];
        frame.save(c, S2, 0x30);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S3, 0x20);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S4, 0x10);
        c.g[S3][0] = c.g[A3][0];
        frame.save_ra(c, 0);
        c.g[S4][0] = c.g[A2][0];
        let no_timeout = (c.g[S1][0] as i64) <= 0;
        c.g[T0][0] = 0;
        if !no_timeout {
            c.cycles += 5;
            c.g[A0][0] = buddy_field(c, S0, BUDDY_ARIES).lw(c);
            callee::PROTO_ARIES_TICK.call(c, 0x003f_0698)?;
            c.g[V1][0] = BUDDY_MS_PER_SECOND;
            c.mult(0, V1, S1, V1);
            c.g[T0][0] = addu(c.g[V0][0], c.g[V1][0]);
        }
        c.cycles += 27;
        c.g[A0][0] = BUDDY_OP_SIZE as u64;
        c.g[V1][0] = buddy_lui(BUDDY_OP_CODES);
        c.mult(0, A0, S2, A0);
        c.g[V1][0] = sx(BUDDY_OP_CODES);
        c.g[V0][0] = sx(r32(c, S2) << 2);
        frame.restore(c, S1, 0x40);
        c.g[V0][0] = addu(c.g[V0][0], c.g[V1][0]);
        frame.restore(c, S2, 0x30);
        c.g[A1][0] = Addr(r32(c, V0)).lw(c);
        c.g[V1][0] = addu(c.g[S0][0], c.g[A0][0]);
        frame.restore_ra(c, 0);
        let slot = r32(c, V1).wrapping_add(BUDDY_OPS);
        Addr(slot + BUDDY_OP_TAG).sw(c, c.g[A1][0]);
        c.g[A3][0] = c.g[V1][0];
        c.g[A2][0] = c.g[V1][0];
        c.g[A0][0] = c.g[V1][0];
        c.g[V0][0] = buddy_field(c, S0, BUDDY_OP_COUNTER).lw(c);
        c.g[V0][0] = addu(c.g[V0][0], 1);
        buddy_field(c, S0, BUDDY_OP_COUNTER).sw(c, c.g[V0][0]);
        Addr(slot.wrapping_add(BUDDY_OP_ID)).sw(c, c.g[V0][0]);
        Addr(slot.wrapping_add(BUDDY_OP_ARG)).sw(c, c.g[S3][0]);
        Addr(slot.wrapping_add(BUDDY_OP_FN)).sw(c, c.g[S4][0]);
        frame.restore(c, S0, 0x50);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        Addr(slot.wrapping_add(BUDDY_OP_DEADLINE)).sw(c, c.g[T0][0]);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// BuddyApi_Create (0x3f0710)

/// `BuddyApi* BuddyApi_Create(const char* product, const char* version)`
/// (0x3f0710): allocate and clear the object, keep the two strings, open
/// the Aries connection and start the op counter at 100; 0 when the
/// allocation fails.
pub fn buddy_api_create(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x30);
        c.g[S2][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = BUDDY_SIZE as u64;
        callee::LOBBY_MEM_ALLOC.call(c, 0x003f_0734)?;
        c.g[S0][0] = c.g[V0][0];
        let failed = c.g[S0][0] == 0;
        c.g[A1][0] = 0;
        if !failed {
            c.cycles += 16;
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = BUDDY_SIZE as u64;
            callee::MEMSET.call(c, 0x003f_074c)?;
            c.g[A2][0] = c.g[S1][0];
            c.g[A0][0] = addu(c.g[S0][0], BUDDY_PRODUCT_NAME as u64);
            c.g[A1][0] = BUDDY_STRING_SIZE as u64;
            callee::TAG_FIELD_STRNZCPY.call(c, 0x003f_075c)?;
            c.g[A2][0] = c.g[S2][0];
            c.g[A0][0] = addu(c.g[S0][0], BUDDY_VERSION as u64);
            c.g[A1][0] = BUDDY_STRING_SIZE as u64;
            callee::TAG_FIELD_STRNZCPY.call(c, 0x003f_076c)?;
            c.g[A0][0] = BUDDY_ARIES_BUFFER as u64;
            callee::PROTO_ARIES_CREATE.call(c, 0x003f_0774)?;
            c.g[V1][0] = BUDDY_FIRST_OP as u64;
            buddy_field(c, S0, BUDDY_ARIES).sw(c, c.g[V0][0]);
            buddy_field(c, S0, BUDDY_OP_COUNTER).sw(c, c.g[V1][0]);
        }
        c.cycles += 7;
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
// BuddyApi_Destroy (0x3f07a0)

/// `void BuddyApi_Destroy(BuddyApi* self)` (0x3f07a0): release a pending
/// host lookup, disconnect, flush the roster, destroy the Aries connection
/// and free the object.
pub fn buddy_api_destroy(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = buddy_field(c, S0, BUDDY_LOOKUP).lw(c);
        if c.g[A0][0] != 0 {
            c.cycles += 4;
            c.g[V0][0] = buddy_field(c, A0, LOOKUP_RELEASE).lw(c);
            let release = r32(c, V0);
            call_ptr(c, release, 0x003f_07c8)?;
            buddy_field(c, S0, BUDDY_LOOKUP).sw(c, 0);
        }
        c.cycles += 12;
        c.g[A0][0] = c.g[S0][0];
        callee::BUDDY_API_DISCONNECT.call(c, 0x003f_07d4)?;
        c.g[A0][0] = c.g[S0][0];
        callee::BUDDY_API_FLUSH_ROSTER.call(c, 0x003f_07dc)?;
        c.g[A0][0] = buddy_field(c, S0, BUDDY_ARIES).lw(c);
        callee::PROTO_ARIES_DESTROY.call(c, 0x003f_07e4)?;
        c.g[A0][0] = c.g[S0][0];
        callee::LOBBY_MEM_FREE.call(c, 0x003f_07ec)?;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// BuddyApi_Config (0x3f0818)

pub(crate) const BUDDY_CONFIG_CODE_HEAD: u32 = 0x003f_0860;

/// `int BuddyApi_Config(BuddyApi* self, const char* pres, u32 product,
/// const char* codes, int arg)` (0x3f0818): keep `pres`, the product and
/// `arg`, and up to four two-letter lowercase codes from `codes` (as
/// halfwords, first letter high); returns 0.
pub fn buddy_api_config(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S2, 0x20);
        c.g[S0][0] = c.g[A2][0];
        c.g[S2][0] = c.g[A0][0];
        c.g[A2][0] = c.g[A1][0];
        frame.save(c, S1, 0x30);
        c.g[A1][0] = BUDDY_STRING_SIZE as u64;
        frame.save(c, S3, 0x10);
        c.g[S1][0] = c.g[A3][0];
        frame.save_ra(c, 0);
        c.g[S3][0] = c.g[T0][0];
        c.g[A0][0] = addu(c.g[S2][0], BUDDY_PRES as u64);
        callee::TAG_FIELD_STRNZCPY.call(c, 0x003f_0850)?;
        buddy_field(c, S2, BUDDY_PRODUCT).sw(c, c.g[S0][0]);
        c.g[A2][0] = 0;
        c.g[A1][0] = addu(c.g[S2][0], BUDDY_CODES as u64);
        loop {
            c.cycles += 4;
            c.g[V0][0] = Addr(r32(c, S1)).lb(c);
            c.g[V0][0] = ((c.g[V0][0] as i64) < BUDDY_CODE_FIRST_LETTER) as u64;
            if c.g[V0][0] != 0 {
                break;
            }
            c.cycles += 4;
            c.g[V0][0] = buddy_field(c, S1, 1).lb(c);
            c.g[V0][0] = ((c.g[V0][0] as i64) < BUDDY_CODE_FIRST_LETTER) as u64;
            if c.g[V0][0] != 0 {
                break;
            }
            c.cycles += 15;
            let slot = Addr(r32(c, A1));
            slot.sh(c, 0);
            c.g[A2][0] = addu(c.g[A2][0], 1);
            c.g[A0][0] = ((c.g[A2][0] as i64) < BUDDY_MAX_CODES) as u64;
            c.g[V1][0] = Addr(r32(c, S1)).lbu(c);
            c.g[V1][0] = sx(r32(c, V1) << 24);
            c.g[V1][0] = sx((r32(c, V1) as i32 >> 16) as u32);
            slot.sh(c, c.g[V1][0]);
            c.g[V0][0] = buddy_field(c, S1, 1).lbu(c);
            c.g[S1][0] = addu(c.g[S1][0], 2);
            c.g[V0][0] = sx(r32(c, V0) << 24);
            c.g[V0][0] = sx((r32(c, V0) as i32 >> 24) as u32);
            c.g[V1][0] |= c.g[V0][0];
            slot.sh(c, c.g[V1][0]);
            let more = c.g[A0][0] != 0;
            c.g[A1][0] = addu(c.g[A1][0], 2);
            if !more {
                c.cycles += 1;
                break;
            }
            if event_due(c) {
                deliver_event_here(c, BUDDY_CONFIG_CODE_HEAD);
            }
        }
        buddy_field(c, S2, BUDDY_ACCEPT_DOTTED).sw(c, c.g[S3][0]);
        c.cycles += 9;
        c.g[V0][0] = 0;
        buddy_field(c, S2, BUDDY_CODE_COUNT).sw(c, c.g[A2][0]);
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
// BuddyApi_Connect (0x3f08e8)

/// `Connect`'s stack buffers: the user part of `user:secret` (at sp, 0x100
/// bytes) and the copy of the login string it is split from (0x100 bytes).
pub(crate) const BUDDY_CONNECT_SPLIT: u32 = 0x100;

pub(crate) const BUDDY_CONNECT_SPLIT_SIZE: u32 = 0x100;

/// `TagFieldSetString(self->login, 0x100, tag, value)` with the registers
/// `Connect` leaves at those calls (a0 = s4, the login record).
pub(crate) fn buddy_connect_set_login(c: &mut Cpu, tag: u32, value: u64, ret: u32) -> Flow {
    c.g[A0][0] = c.g[S4][0];
    c.g[A1][0] = BUDDY_LOGIN_SIZE as u64;
    c.g[A2][0] = sx(tag);
    c.g[A3][0] = value;
    callee::TAG_FIELD_SET_STRING.call(c, ret)
}

/// `int BuddyApi_Connect(BuddyApi* self, const char* host, int arg,
/// int timeout, const char* login, const char* user, void* state_cb,
/// void* state_ctx)` (0x3f08e8): disconnect, keep the state callback, build
/// the login record (`PROD`, `VERS`, `PRES`, then `USER` and `LKEY` from a
/// login without `:`, or from `name:secret` the `USER` `name` + `user` and
/// `LKEY` for a `$`-prefixed secret, `PASS` otherwise), keep the user name
/// (no leading `/`, cut at the next `/`), start the host lookup with
/// `timeout` seconds, enter state 1 and set the deadline; returns 0.
pub fn buddy_api_connect(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 48;
        let frame = Frame::open(c, 0x2a0);
        frame.save(c, S3, 0x260);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S0, 0x290);
        frame.save(c, S1, 0x280);
        c.g[S0][0] = c.g[T3][0];
        frame.save(c, S2, 0x270);
        c.g[S1][0] = c.g[A2][0];
        frame.save(c, S5, 0x240);
        c.g[S2][0] = c.g[T2][0];
        frame.save(c, S6, 0x230);
        c.g[S5][0] = c.g[T0][0];
        frame.save(c, S7, 0x220);
        c.g[S6][0] = c.g[T1][0];
        frame.save(c, FP, 0x210);
        c.g[S7][0] = c.g[A1][0];
        frame.save(c, S4, 0x250);
        frame.save_ra(c, 0x200);
        c.g[FP][0] = c.g[A3][0];
        callee::BUDDY_API_DISCONNECT.call(c, 0x003f_0938)?;
        c.g[S4][0] = addu(c.g[S3][0], BUDDY_LOGIN as u64);
        buddy_field(c, S3, BUDDY_STATE_ARG).sw(c, c.g[S0][0]);
        buddy_field(c, S3, BUDDY_STATE_FN).sw(c, c.g[S2][0]);
        buddy_field(c, S3, BUDDY_PORT).sw(c, c.g[S1][0]);
        buddy_field(c, S3, BUDDY_LOGIN).sb(c, 0);
        let product = addu(c.g[S3][0], BUDDY_PRODUCT_NAME as u64);
        buddy_connect_set_login(c, BUDDY_TAG_PROD, product, 0x003f_0964)?;
        let version = addu(c.g[S3][0], BUDDY_VERSION as u64);
        buddy_connect_set_login(c, BUDDY_TAG_VERS, version, 0x003f_097c)?;
        let pres = addu(c.g[S3][0], BUDDY_PRES as u64);
        buddy_connect_set_login(c, BUDDY_TAG_PRES, pres, 0x003f_0994)?;
        c.g[A0][0] = c.g[S5][0];
        c.g[A1][0] = b':' as u64;
        callee::STRCHR.call(c, 0x003f_09a0)?;
        let has_colon = c.g[V0][0] != 0;
        c.g[S0][0] = sx(frame.at(c, BUDDY_CONNECT_SPLIT));
        'name: {
            'plain: {
                'key: {
                    if !has_colon {
                        // The login is a key: USER = user, LKEY = login.
                        c.cycles += 11;
                        let user = c.g[S6][0];
                        buddy_connect_set_login(c, BUDDY_TAG_USER, user, 0x003f_09c0)?;
                        c.g[A3][0] = c.g[S5][0];
                        break 'key;
                    }
                    // `name:secret` (from a copy cut at 255 characters).
                    c.cycles += 11;
                    c.g[A1][0] = c.g[S5][0];
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A2][0] = (BUDDY_CONNECT_SPLIT_SIZE - 1) as u64;
                    callee::STRNCPY.call(c, 0x003f_09e4)?;
                    Addr(frame.at(c, BUDDY_CONNECT_SPLIT + BUDDY_CONNECT_SPLIT_SIZE - 1)).sb(c, 0);
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = b':' as u64;
                    callee::STRCHR.call(c, 0x003f_09f4)?;
                    c.g[S1][0] = c.g[V0][0];
                    let lost = c.g[S1][0] == 0;
                    c.g[A2][0] = c.g[S0][0];
                    if lost {
                        break 'plain;
                    }
                    c.cycles += 14;
                    Addr(r32(c, S1)).sb(c, 0);
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A1][0] = BUDDY_LOGIN_SIZE as u64;
                    callee::TAG_FIELD_STRNZCPY.call(c, 0x003f_0a10)?;
                    c.g[S1][0] = addu(c.g[S1][0], 1);
                    c.g[A0][0] = c.g[SP][0];
                    callee::STRLEN.call(c, 0x003f_0a1c)?;
                    c.g[S0][0] = c.g[V0][0];
                    c.g[A0][0] = c.g[S6][0];
                    callee::STRLEN.call(c, 0x003f_0a28)?;
                    c.g[S0][0] = addu(c.g[S0][0], c.g[V0][0]);
                    c.g[S0][0] = (c.g[S0][0] < BUDDY_LOGIN_SIZE as u64) as u64;
                    let fits = c.g[S0][0] != 0;
                    c.g[A0][0] = c.g[SP][0];
                    if fits {
                        c.cycles += 2;
                        c.g[A1][0] = c.g[S6][0];
                        callee::STRCAT.call(c, 0x003f_0a40)?;
                    }
                    c.cycles += 10;
                    let name = c.g[SP][0];
                    buddy_connect_set_login(c, BUDDY_TAG_USER, name, 0x003f_0a58)?;
                    c.g[V1][0] = Addr(r32(c, S1)).lb(c);
                    c.g[V0][0] = b'$' as u64;
                    let password = c.g[V1][0] != c.g[V0][0];
                    c.g[A2][0] = buddy_lui(BUDDY_TAG_LKEY);
                    if password {
                        c.cycles += 5;
                        let secret = c.g[S1][0];
                        buddy_connect_set_login(c, BUDDY_TAG_PASS, secret, 0x003f_0a9c)?;
                        break 'plain;
                    }
                    c.cycles += 4;
                    c.g[A3][0] = addu(c.g[S1][0], 1);
                }
                // LKEY = the key (a3).
                c.cycles += 4;
                let key = c.g[A3][0];
                buddy_connect_set_login(c, BUDDY_TAG_LKEY, key, 0x003f_0a80)?;
                c.g[S0][0] = addu(c.g[S3][0], BUDDY_USER as u64);
                break 'name;
            }
            c.cycles += 1;
            c.g[S0][0] = addu(c.g[S3][0], BUDDY_USER as u64);
        }
        // The user name: no leading '/', nothing from the next '/' on.
        c.cycles += 8;
        c.g[A2][0] = c.g[S6][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = BUDDY_USER_SIZE as u64;
        callee::TAG_FIELD_STRNZCPY.call(c, 0x003f_0ab0)?;
        c.g[V1][0] = buddy_field(c, S3, BUDDY_USER).lb(c);
        c.g[V0][0] = b'/' as u64;
        let slash = c.g[V1][0] == c.g[V0][0];
        c.g[A0][0] = c.g[S0][0];
        if slash {
            c.cycles += 3;
            c.g[A1][0] = addu(c.g[S3][0], (BUDDY_USER + 1) as u64);
            callee::STRCPY.call(c, 0x003f_0ac8)?;
            c.g[A0][0] = c.g[S0][0];
        }
        c.cycles += 5;
        c.g[A1][0] = b'/' as u64;
        callee::STRCHR.call(c, 0x003f_0ad4)?;
        c.g[S1][0] = c.g[V0][0];
        if c.g[S1][0] != 0 {
            Addr(r32(c, S1)).sb(c, 0);
        }
        c.cycles += 28;
        c.g[S0][0] = BUDDY_MS_PER_SECOND;
        c.g[A0][0] = c.g[S7][0];
        c.mult(0, S0, FP, S0);
        c.g[A1][0] = c.g[S0][0];
        callee::SOCKET_LOOKUP.call(c, 0x003f_0af4)?;
        buddy_field(c, S3, BUDDY_LOOKUP).sw(c, c.g[V0][0]);
        c.g[A0][0] = c.g[S3][0];
        c.g[A1][0] = BUDDY_STATE_CONNECT_STARTED as u64;
        callee::BUDDY_API_SET_STATE.call(c, 0x003f_0b04)?;
        c.g[A0][0] = buddy_field(c, S3, BUDDY_ARIES).lw(c);
        callee::PROTO_ARIES_TICK.call(c, 0x003f_0b0c)?;
        c.g[V1][0] = addu(c.g[V0][0], c.g[S0][0]);
        frame.restore(c, S1, 0x280);
        c.g[V0][0] = u64::MAX;
        buddy_field(c, S3, BUDDY_LOGIN_DEADLINE).sw(c, c.g[V1][0]);
        buddy_field(c, S3, BUDDY_PRESENCE_SHOW).sw(c, c.g[V0][0]);
        frame.restore(c, S0, 0x290);
        c.g[V0][0] = 0;
        frame.restore(c, S2, 0x270);
        frame.restore(c, S3, 0x260);
        frame.restore(c, S4, 0x250);
        frame.restore(c, S5, 0x240);
        frame.restore(c, S6, 0x230);
        frame.restore(c, S7, 0x220);
        frame.restore(c, FP, 0x210);
        frame.restore_ra(c, 0x200);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Roster, presence and message constants and helpers.

/// `-2`: no roster or no such buddy; `-3`: refused name.
pub(crate) const BUDDY_ERR_NONE: u64 = 0xffff_ffff_ffff_fffe;

pub(crate) const BUDDY_ERR_NAME: u64 = 0xffff_ffff_ffff_fffd;

/// The roster callback's message kind (`BUDDY_TYPE_ROST`) as its `lui` and `ori`
/// halves.
pub(crate) const BUDDY_MSG_ROST_HI: u64 = (BUDDY_TYPE_ROST & 0xffff_0000) as u64;

pub(crate) const BUDDY_MSG_ROST_LO: u64 = (BUDDY_TYPE_ROST & 0xffff) as u64;

// Tag names and values in the game's data.
/// "LRSC"
pub(crate) const BUDDY_TAG_LRSC: u32 = 0x004a_4b38;

/// "LIST"
pub(crate) const BUDDY_TAG_LIST: u32 = 0x004a_4b40;

/// "B"
pub(crate) const BUDDY_STR_B: u32 = 0x004a_4b48;

/// "Y"
pub(crate) const BUDDY_STR_Y: u32 = 0x004a_4b50;

/// "ID"
pub(crate) const BUDDY_TAG_ID: u32 = 0x004a_4b58;

/// "I"
pub(crate) const BUDDY_STR_I: u32 = 0x004a_4b60;

/// "GROUP"
pub(crate) const BUDDY_TAG_GROUP: u32 = 0x004a_4b68;

/// "BODY"
pub(crate) const BUDDY_TAG_BODY: u32 = 0x004a_4b70;

/// "SUBJ"
pub(crate) const BUDDY_TAG_SUBJ: u32 = 0x004a_4b78;

/// A tag record buffer's size (the 0x100-byte stack records).
pub(crate) const BUDDY_RECORD_LEN: u64 = 0x100;

/// `reg + off`, sign-extended (`addiu`).
#[inline]
pub(crate) fn buddy_addr(c: &Cpu, reg: usize, off: u32) -> u64 {
    sx(r32(c, reg).wrapping_add(off))
}

// ---------------------------------------------------------------------------
// Connection and roster.

/// `int BuddyApi_Disconnect(BuddyApiT* api)` (0x3f0b50): drop the buddy
/// connection. A pending host lookup (+0) is released through its method at
/// +0xc and cleared; then the 'DISC' packet (empty body) goes out, the Aries
/// connection is closed, the state passes through 6 when it was 5 and ends
/// at 0, the state callback (+0x4c4) is cleared and the roster flushed.
/// Returns 0.
pub fn buddy_api_disconnect(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x20);
        c.cycles += 7;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[A0][0] = buddy_field(c, S0, BUDDY_LOOKUP).lw(c);
        c.g[A2][0] = buddy_lui(BUDDY_STR_EMPTY);
        if c.g[A0][0] != 0 {
            c.cycles += 5;
            c.g[V0][0] = buddy_field(c, A0, LOOKUP_RELEASE).lw(c);
            let release = r32(c, V0);
            call_ptr(c, release, 0x003f_0b78)?;
            buddy_field(c, S0, BUDDY_LOOKUP).sw(c, 0);
            c.g[A2][0] = buddy_lui(BUDDY_STR_EMPTY);
        }
        c.cycles += 11;
        c.g[A1][0] = sx(BUDDY_KIND_DISC);
        c.g[A2][0] = sx(BUDDY_STR_EMPTY);
        c.g[A0][0] = c.g[S0][0];
        callee::BUDDY_API_SEND_PACKET.call(c, 0x003f_0b94)?;
        c.g[A0][0] = buddy_field(c, S0, BUDDY_ARIES).lw(c);
        callee::PROTO_ARIES_UNCONNECT.call(c, 0x003f_0b9c)?;
        c.g[V1][0] = buddy_field(c, S0, BUDDY_STATE).lw(c);
        c.g[V0][0] = BUDDY_STATE_LINK_UP;
        c.g[A0][0] = c.g[S0][0];
        if c.g[V1][0] == c.g[V0][0] {
            c.cycles += 3;
            c.g[A1][0] = BUDDY_STATE_LINK_DOWN;
            callee::BUDDY_API_SET_STATE.call(c, 0x003f_0bb4)?;
            c.g[A0][0] = c.g[S0][0];
        }
        c.cycles += 10;
        c.g[A1][0] = 0;
        callee::BUDDY_API_SET_STATE.call(c, 0x003f_0bc0)?;
        buddy_field(c, S0, BUDDY_STATE_FN).sw(c, 0);
        c.g[A0][0] = c.g[S0][0];
        callee::BUDDY_API_FLUSH_ROSTER.call(c, 0x003f_0bcc)?;
        frame.restore(c, S0, 0x10);
        c.g[V0][0] = 0;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `BuddyApi_FlushRoster`'s pop loop head: remove the popped entry. The
/// generated body resumes here after an event unwinds its back edge.
pub(crate) const BUDDY_FLUSH_ROSTER_REMOVE: u32 = 0x003f_0c08;

/// `void BuddyApi_FlushRoster(BuddyApiT* api)` (0x3f0be0): empty the roster.
/// With a roster list (+0x4a8), pop every entry from the roster hash
/// (+0x4a4), remove its node from the list and free the record; then call
/// the roster callback (+0x4b8, if any) with `(api, {'rost', 0, 0, list},
/// data at +0x4b4)`, destroy the hash and the list and clear the hash, list,
/// callback and data words.
///
/// The pop loop is dispatcher-shaped in the generated body: an event at its
/// back edge that asks for an unwind returns at once, and the function is
/// re-entered at the head 0x3f0c08 (remove the popped node, then pop again),
/// with the loop state in v0, a0 and s0.
pub fn buddy_api_flush_roster(c: &mut Cpu) {
    const ENTRY: u32 = 0x003f_0be0;
    let frame = Frame { size: 0x30 };
    guest(c, |c| {
        match c.pc {
            ENTRY => {
                c.cycles += 7;
                Frame::open(c, 0x30);
                frame.save(c, S0, 0x20);
                frame.save_ra(c, 0x10);
                c.g[S0][0] = c.g[A0][0];
                c.g[V0][0] = buddy_field(c, S0, BUDDY_ROSTER_LIST).lw(c);
                if c.g[V0][0] == 0 {
                    frame.restore(c, S0, 0x20);
                    buddy_flush_roster_return(c, frame);
                    return Ok(());
                }
                c.cycles += 2;
            }
            BUDDY_FLUSH_ROSTER_REMOVE => buddy_flush_roster_remove(c)?,
            other => {
                c.jump_out(other, ENTRY, 0x003f_0c8c);
                return Ok(());
            }
        }
        // Pop entries until the hash is empty.
        loop {
            c.cycles += 4;
            c.g[A0][0] = buddy_field(c, S0, BUDDY_ROSTER_HASH).lw(c);
            callee::LOBBY_HASH_POP_ANY.call(c, 0x003f_0c20)?;
            if c.g[V0][0] == 0 {
                break;
            }
            c.g[A0][0] = buddy_field(c, S0, BUDDY_ROSTER_LIST).lw(c);
            // gen 0x3f0c08: `if c.event() { return; }`, not a structured check.
            if back_edge_unwinds(c, BUDDY_FLUSH_ROSTER_REMOVE) {
                return Ok(());
            }
            buddy_flush_roster_remove(c)?;
        }
        // Tell the roster callback.
        c.cycles += 3;
        c.g[A3][0] = buddy_field(c, S0, BUDDY_ROSTER_FN).lw(c);
        c.g[V0][0] = BUDDY_MSG_ROST_HI;
        if c.g[A3][0] != 0 {
            c.cycles += 10;
            c.g[V1][0] = buddy_field(c, S0, BUDDY_ROSTER_LIST).lw(c);
            c.g[A2][0] = buddy_field(c, S0, BUDDY_ROSTER_ARG).lw(c);
            c.g[V0][0] |= BUDDY_MSG_ROST_LO;
            let msg = sp(c);
            c.sw(msg, c.g[V0][0]);
            c.g[A0][0] = c.g[S0][0];
            c.sw(msg.wrapping_add(0xc), c.g[V1][0]);
            c.g[A1][0] = c.g[SP][0];
            c.sw(msg.wrapping_add(0x4), 0);
            c.sw(msg.wrapping_add(0x8), 0);
            let proc_ = r32(c, A3);
            call_ptr(c, proc_, 0x003f_0c5c)?;
        }
        c.cycles += 9;
        c.g[A0][0] = buddy_field(c, S0, BUDDY_ROSTER_HASH).lw(c);
        callee::LOBBY_HASH_DESTROY.call(c, 0x003f_0c64)?;
        buddy_field(c, S0, BUDDY_ROSTER_HASH).sw(c, 0);
        c.g[A0][0] = buddy_field(c, S0, BUDDY_ROSTER_LIST).lw(c);
        callee::LOBBY_LIST_DESTROY.call(c, 0x003f_0c70)?;
        buddy_field(c, S0, BUDDY_ROSTER_FN).sw(c, 0);
        buddy_field(c, S0, BUDDY_ROSTER_LIST).sw(c, 0);
        buddy_field(c, S0, BUDDY_ROSTER_ARG).sw(c, 0);
        frame.restore(c, S0, 0x20);
        buddy_flush_roster_return(c, frame);
        Ok(())
    })
}

/// The pop loop's body: remove the popped node (`v0`) from the list (`a0`)
/// and free the record it held.
pub(crate) fn buddy_flush_roster_remove(c: &mut Cpu) -> Flow {
    c.cycles += 4;
    c.g[A1][0] = c.g[V0][0];
    callee::LOBBY_LIST_REMOVE.call(c, 0x003f_0c10)?;
    c.g[A0][0] = c.g[V0][0];
    callee::LOBBY_MEM_FREE.call(c, 0x003f_0c18)
}

/// `BuddyApi_FlushRoster`'s epilogue.
pub(crate) fn buddy_flush_roster_return(c: &mut Cpu, frame: Frame) {
    c.cycles += 3;
    frame.restore_ra(c, 0x10);
    frame.close(c);
}

/// `ListT* BuddyApi_GetRoster(BuddyApiT* api, RosterProc proc, void* data)`
/// (0x3f0c90): return the roster list, first creating it. Without one, keep
/// the roster callback and its data (+0x4b8, +0x4b4), create the roster hash
/// (`Lobby_HashCreate(1000, 2000)`) and list (`Lobby_ListCreate(100, 100,
/// 0)`), ask the server for both rosters with two 'RGET' packets (`LRSC` =
/// the user name, `LIST` = "B" then "I", `PRES` = "Y", `ID` = 1 then 2)
/// and mark the list changed.
pub fn buddy_api_get_roster(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x180);
        c.cycles += 13;
        frame.save(c, S6, 0x110);
        frame.save(c, S0, 0x170);
        c.g[S6][0] = c.g[A0][0];
        frame.save(c, S1, 0x160);
        frame.save(c, S2, 0x150);
        frame.save(c, S3, 0x140);
        frame.save(c, S4, 0x130);
        frame.save(c, S5, 0x120);
        frame.save_ra(c, 0x100);
        c.g[V0][0] = buddy_field(c, S6, BUDDY_ROSTER_LIST).lw(c);
        let have = c.g[V0][0] != 0;
        frame.restore(c, S0, 0x170);
        if !have {
            buddy_get_roster_request(c, frame)?;
        }
        c.cycles += 9;
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

/// `BuddyApi_GetRoster` without a roster: create it and send the requests.
pub(crate) fn buddy_get_roster_request(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 81;
    let (proc_, data) = (c.g[A1][0], c.g[A2][0]);
    buddy_field(c, S6, BUDDY_ROSTER_FN).sw(c, proc_);
    c.g[A0][0] = 1000;
    buddy_field(c, S6, BUDDY_ROSTER_ARG).sw(c, data);
    c.g[A1][0] = 2000;
    callee::LOBBY_HASH_CREATE.call(c, 0x003f_0cd8)?;
    c.g[S5][0] = buddy_addr(c, S6, BUDDY_USER);
    c.g[A0][0] = 100;
    c.g[A1][0] = 100;
    c.g[A2][0] = 0;
    let hash = c.g[V0][0];
    buddy_field(c, S6, BUDDY_ROSTER_HASH).sw(c, hash);
    callee::LOBBY_LIST_CREATE.call(c, 0x003f_0cf0)?;
    let list = c.g[V0][0];
    buddy_field(c, S6, BUDDY_ROSTER_LIST).sw(c, list);
    // First request: LRSC=user LIST=B PRES=Y ID=1.
    c.g[S4][0] = sx(BUDDY_TAG_LRSC);
    c.g[A0][0] = c.g[SP][0];
    let record = sp(c);
    c.sb(record, 0);
    c.g[A3][0] = c.g[S5][0];
    c.g[A1][0] = BUDDY_RECORD_LEN;
    c.g[A2][0] = c.g[S4][0];
    callee::TAG_FIELD_SET_STRING.call(c, 0x003f_0d14)?;
    c.g[S3][0] = sx(BUDDY_TAG_LIST);
    c.g[A0][0] = c.g[SP][0];
    c.g[A3][0] = sx(BUDDY_STR_B);
    c.g[A1][0] = BUDDY_RECORD_LEN;
    c.g[A2][0] = c.g[S3][0];
    callee::TAG_FIELD_SET_STRING.call(c, 0x003f_0d34)?;
    c.g[S1][0] = sx(BUDDY_TAG_PRES);
    c.g[S2][0] = sx(BUDDY_STR_Y);
    c.g[A0][0] = c.g[SP][0];
    c.g[A1][0] = BUDDY_RECORD_LEN;
    c.g[A2][0] = c.g[S1][0];
    c.g[A3][0] = c.g[S2][0];
    callee::TAG_FIELD_SET_STRING.call(c, 0x003f_0d58)?;
    c.g[S0][0] = sx(BUDDY_TAG_ID);
    c.g[A0][0] = c.g[SP][0];
    c.g[A3][0] = 1;
    c.g[A1][0] = BUDDY_RECORD_LEN;
    c.g[A2][0] = c.g[S0][0];
    callee::TAG_FIELD_SET_NUMBER.call(c, 0x003f_0d74)?;
    c.g[A1][0] = sx(BUDDY_KIND_RGET);
    c.g[A2][0] = c.g[SP][0];
    c.g[A0][0] = c.g[S6][0];
    callee::BUDDY_API_SEND_PACKET.call(c, 0x003f_0d88)?;
    // Second request: LRSC=user LIST=I PRES=Y ID=2.
    c.g[A0][0] = c.g[SP][0];
    let record = sp(c);
    c.sb(record, 0);
    c.g[A3][0] = c.g[S5][0];
    c.g[A2][0] = c.g[S4][0];
    c.g[A1][0] = BUDDY_RECORD_LEN;
    callee::TAG_FIELD_SET_STRING.call(c, 0x003f_0da0)?;
    c.g[A0][0] = c.g[SP][0];
    c.g[A2][0] = c.g[S3][0];
    c.g[A3][0] = sx(BUDDY_STR_I);
    c.g[A1][0] = BUDDY_RECORD_LEN;
    callee::TAG_FIELD_SET_STRING.call(c, 0x003f_0db8)?;
    c.g[A0][0] = c.g[SP][0];
    c.g[A2][0] = c.g[S1][0];
    c.g[A3][0] = c.g[S2][0];
    c.g[A1][0] = BUDDY_RECORD_LEN;
    callee::TAG_FIELD_SET_STRING.call(c, 0x003f_0dcc)?;
    c.g[A0][0] = c.g[SP][0];
    c.g[A2][0] = c.g[S0][0];
    c.g[A1][0] = BUDDY_RECORD_LEN;
    c.g[A3][0] = 2;
    callee::TAG_FIELD_SET_NUMBER.call(c, 0x003f_0de0)?;
    c.g[A1][0] = sx(BUDDY_KIND_RGET);
    c.g[A0][0] = c.g[S6][0];
    c.g[A2][0] = c.g[SP][0];
    callee::BUDDY_API_SEND_PACKET.call(c, 0x003f_0df4)?;
    c.g[A0][0] = buddy_field(c, S6, BUDDY_ROSTER_LIST).lw(c);
    c.g[A1][0] = 1;
    callee::LOBBY_LIST_MARK_DIRTY.call(c, 0x003f_0e00)?;
    c.g[V0][0] = buddy_field(c, S6, BUDDY_ROSTER_LIST).lw(c);
    frame.restore(c, S0, 0x170);
    Ok(())
}

// ---------------------------------------------------------------------------
// Adding and deleting buddies.

/// `BuddyApi_Add`'s copy loop heads (aligned and unaligned record copies).
pub(crate) const BUDDY_ADD_COPY_ALIGNED: u32 = 0x003f_0f80;

pub(crate) const BUDDY_ADD_COPY_UNALIGNED: u32 = 0x003f_0f28;

/// `int BuddyApi_Add(BuddyApiT* api, BuddyT* buddy, a2, a3, t0)` (0x3f0e38):
/// add a buddy to the server lists its flags (+0xa8) name. Refuses (-3) a
/// missing buddy, one without flags, an empty name or a name with '/' or
/// '@' (or with '.' while the word at +0x18 is zero). Otherwise records the
/// add as the pending operation (`a2`, `a3` and the timeout `t0` go to
/// `BuddyApi_SetPendingOp`) and, with a roster, merges the flags into the
/// roster record of that name or adds a copy of the buddy (0xb8 bytes) to
/// the roster list and hash; then sends 'RADD' for the B list (flag 4: also
/// `PRES`=Y and the `GROUP` at +0x20) and the I list (0x200), marking the
/// record sent with the current sequence number, and 'PADD' for flag
/// 0x800000. Returns 0.
///
/// The record copy loops check events at their heads (0x3f0f80 aligned,
/// 0x3f0f28 unaligned) and carry on.
pub fn buddy_api_add(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x160);
        'done: {
            c.cycles += 13;
            frame.save(c, S0, 0x150);
            frame.save(c, S1, 0x140);
            c.g[S0][0] = c.g[A1][0];
            frame.save(c, S2, 0x130);
            c.g[S1][0] = c.g[A0][0];
            frame.save(c, S3, 0x120);
            c.g[S2][0] = c.g[A2][0];
            frame.save(c, S4, 0x110);
            c.g[S3][0] = c.g[A3][0];
            frame.save_ra(c, 0x100);
            c.g[S4][0] = c.g[T0][0];
            // The name checks.
            'checked: {
                'refuse: {
                    if c.g[S0][0] == 0 {
                        break 'refuse;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = buddy_field(c, S0, ROSTER_ENTRY_FLAGS).lw(c);
                    let no_flags = c.g[V0][0] == 0;
                    c.g[V0][0] = BUDDY_ERR_NAME;
                    if no_flags {
                        break 'done;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = c.lb(r32(c, S0));
                    let empty = c.g[V0][0] == 0;
                    c.g[A0][0] = c.g[S0][0];
                    if empty {
                        break 'refuse;
                    }
                    c.cycles += 4;
                    c.g[A1][0] = b'/' as u64;
                    callee::STRCHR.call(c, 0x003f_0e8c)?;
                    let slash = c.g[V0][0] != 0;
                    c.g[V0][0] = BUDDY_ERR_NAME;
                    if slash {
                        break 'done;
                    }
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = b'@' as u64;
                    callee::STRCHR.call(c, 0x003f_0ea0)?;
                    if c.g[V0][0] == 0 {
                        c.g[V0][0] = buddy_field(c, S1, BUDDY_ACCEPT_DOTTED).lw(c);
                        break 'checked;
                    }
                }
                c.cycles += 2;
                c.g[V0][0] = BUDDY_ERR_NAME;
                break 'done;
            }
            c.cycles += 2;
            let allow_dot = c.g[V0][0] != 0;
            c.g[A2][0] = c.g[S2][0];
            if !allow_dot {
                c.cycles += 5;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = b'.' as u64;
                callee::STRCHR.call(c, 0x003f_0ec4)?;
                let dot = c.g[V0][0] != 0;
                c.g[V0][0] = BUDDY_ERR_NAME;
                if dot {
                    break 'done;
                }
                c.cycles += 1;
                c.g[A2][0] = c.g[S2][0];
            }
            c.cycles += 8;
            c.g[A3][0] = c.g[S3][0];
            c.g[T0][0] = c.g[S4][0];
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = BUDDY_OP_KIND_ADD;
            callee::BUDDY_API_SET_PENDING_OP.call(c, 0x003f_0ee4)?;
            c.g[V0][0] = buddy_field(c, S1, BUDDY_ROSTER_LIST).lw(c);
            let no_roster = c.g[V0][0] == 0;
            c.g[V0][0] = 0;
            if no_roster {
                break 'done;
            }
            buddy_add_roster_entry(c)?;
            buddy_add_send_lists(c)?;
            c.cycles += 1;
            c.g[V0][0] = 0;
        }
        c.cycles += 8;
        frame.restore(c, S0, 0x150);
        frame.restore(c, S1, 0x140);
        frame.restore(c, S2, 0x130);
        frame.restore(c, S3, 0x120);
        frame.restore(c, S4, 0x110);
        frame.restore_ra(c, 0x100);
        frame.close(c);
        Ok(())
    })
}

/// `BuddyApi_Add` with a roster: find the record of the buddy's name and
/// merge its flags, or add a copy of the buddy; `s2` = the record.
pub(crate) fn buddy_add_roster_entry(c: &mut Cpu) -> Flow {
    c.cycles += 5;
    c.g[A0][0] = buddy_field(c, S1, BUDDY_ROSTER_HASH).lw(c);
    c.g[A1][0] = c.g[S0][0];
    callee::LOBBY_HASH_FIND_STR.call(c, 0x003f_0efc)?;
    if c.g[V0][0] != 0 {
        // Known: merge the flags.
        c.g[A0][0] = buddy_field(c, S1, BUDDY_ROSTER_LIST).lw(c);
        c.cycles += 8;
        c.g[A1][0] = c.g[V0][0];
        callee::LOBBY_LIST_GET_DATA.call(c, 0x003f_1008)?;
        c.g[S2][0] = c.g[V0][0];
        c.g[V1][0] = buddy_field(c, S0, ROSTER_ENTRY_FLAGS).lw(c);
        c.g[V0][0] = buddy_field(c, S2, ROSTER_ENTRY_FLAGS).lw(c);
        c.g[V0][0] |= c.g[V1][0];
        let flags = c.g[V0][0];
        buddy_field(c, S2, ROSTER_ENTRY_FLAGS).sw(c, flags);
        buddy_field(c, S2, ROSTER_ENTRY_OP_ID).sw(c, 0);
        return Ok(());
    }
    // New: copy the buddy into a fresh record, add it to the list and hash.
    c.cycles += 8;
    c.g[A0][0] = ROSTER_ENTRY_SIZE;
    callee::LOBBY_MEM_ALLOC.call(c, 0x003f_0f0c)?;
    c.g[S2][0] = c.g[V0][0];
    c.g[V1][0] = c.g[S0][0];
    c.g[V0][0] = (c.g[S0][0] | c.g[S2][0]) & 0x7;
    let aligned = c.g[V0][0] == 0;
    c.g[A0][0] = c.g[S2][0];
    c.cycles += 1;
    c.g[V0][0] = buddy_addr(c, S0, 0xa0);
    if aligned {
        loop {
            c.cycles += 11;
            c.g[A1][0] = c.ld(r32(c, V1));
            c.g[A2][0] = c.ld(r32(c, V1).wrapping_add(0x8));
            c.g[A3][0] = c.ld(r32(c, V1).wrapping_add(0x10));
            c.g[T0][0] = c.ld(r32(c, V1).wrapping_add(0x18));
            c.sd(r32(c, A0), c.g[A1][0]);
            c.sd(r32(c, A0).wrapping_add(0x8), c.g[A2][0]);
            c.sd(r32(c, A0).wrapping_add(0x10), c.g[A3][0]);
            c.sd(r32(c, A0).wrapping_add(0x18), c.g[T0][0]);
            c.g[V1][0] = buddy_addr(c, V1, 0x20);
            let more = c.g[V1][0] != c.g[V0][0];
            c.g[A0][0] = buddy_addr(c, A0, 0x20);
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, BUDDY_ADD_COPY_ALIGNED);
            }
        }
    } else {
        loop {
            c.cycles += 19;
            for (i, rt) in [A1, A2, A3, T0].into_iter().enumerate() {
                let from = r32(c, V1).wrapping_add(8 * i as u32);
                buddy_load_unaligned(c, rt, from);
            }
            for (i, rt) in [A1, A2, A3, T0].into_iter().enumerate() {
                let to = r32(c, A0).wrapping_add(8 * i as u32);
                buddy_store_unaligned(c, rt, to);
            }
            c.g[V1][0] = buddy_addr(c, V1, 0x20);
            let more = c.g[V1][0] != c.g[V0][0];
            c.g[A0][0] = buddy_addr(c, A0, 0x20);
            if !more {
                c.cycles += 2;
                break;
            }
            if event_due(c) {
                deliver_event_here(c, BUDDY_ADD_COPY_UNALIGNED);
            }
        }
    }
    // The last 0x18 bytes.
    c.cycles += 21;
    for (i, rt) in [V0, A1, A2].into_iter().enumerate() {
        let from = r32(c, V1).wrapping_add(8 * i as u32);
        buddy_load_unaligned(c, rt, from);
    }
    for (i, rt) in [V0, A1, A2].into_iter().enumerate() {
        let to = r32(c, A0).wrapping_add(8 * i as u32);
        buddy_store_unaligned(c, rt, to);
    }
    c.g[A0][0] = buddy_field(c, S1, BUDDY_ROSTER_LIST).lw(c);
    c.g[A1][0] = c.g[S2][0];
    callee::LOBBY_LIST_ADD.call(c, 0x003f_0fe8)?;
    c.g[A0][0] = buddy_field(c, S1, BUDDY_ROSTER_HASH).lw(c);
    c.g[A2][0] = c.g[V0][0];
    c.g[A1][0] = c.g[S2][0];
    callee::LOBBY_HASH_ADD_STR.call(c, 0x003f_0ff8)?;
    buddy_field(c, S2, ROSTER_ENTRY_OP_ID).sw(c, 0);
    Ok(())
}

/// `ldl rt, 7(at); ldr rt, 0(at)`.
#[inline]
pub(crate) fn buddy_load_unaligned(c: &mut Cpu, rt: usize, at: u32) {
    c.ldl(rt, at.wrapping_add(7));
    c.ldr(rt, at);
}

/// `sdl rt, 7(at); sdr rt, 0(at)`.
#[inline]
pub(crate) fn buddy_store_unaligned(c: &mut Cpu, rt: usize, at: u32) {
    c.sdl(rt, at.wrapping_add(7));
    c.sdr(rt, at);
}

/// `BuddyApi_Add`'s requests for the record `s2` of the buddy `s0`.
pub(crate) fn buddy_add_send_lists(c: &mut Cpu) -> Flow {
    let record = sp(c);
    // The B list (flag 4).
    c.cycles += 4;
    c.g[V0][0] = buddy_field(c, S0, ROSTER_ENTRY_FLAGS).lw(c) & ROSTER_FLAG_LIST_B;
    c.g[A0][0] = c.g[SP][0];
    if c.g[V0][0] != 0 {
        c.cycles += 49;
        c.sb(record, 0);
        c.g[A2][0] = sx(BUDDY_TAG_LRSC);
        c.g[A1][0] = BUDDY_RECORD_LEN;
        c.g[A3][0] = buddy_addr(c, S1, BUDDY_USER);
        callee::TAG_FIELD_SET_STRING.call(c, 0x003f_1048)?;
        c.g[A3][0] = buddy_field(c, S1, BUDDY_OPS + BUDDY_OP_ID).lw(c);
        c.g[A0][0] = c.g[SP][0];
        c.g[A2][0] = sx(BUDDY_TAG_ID);
        c.g[A1][0] = BUDDY_RECORD_LEN;
        callee::TAG_FIELD_SET_NUMBER.call(c, 0x003f_1060)?;
        c.g[A0][0] = c.g[SP][0];
        c.g[A2][0] = sx(BUDDY_TAG_LIST);
        c.g[A3][0] = sx(BUDDY_STR_B);
        c.g[A1][0] = BUDDY_RECORD_LEN;
        callee::TAG_FIELD_SET_STRING.call(c, 0x003f_107c)?;
        c.g[A0][0] = c.g[SP][0];
        c.g[A2][0] = sx(BUDDY_TAG_PRES);
        c.g[A3][0] = sx(BUDDY_STR_Y);
        c.g[A1][0] = BUDDY_RECORD_LEN;
        callee::TAG_FIELD_SET_STRING.call(c, 0x003f_1098)?;
        c.g[A0][0] = c.g[SP][0];
        c.g[A2][0] = sx(BUDDY_TAG_USER);
        c.g[A1][0] = BUDDY_RECORD_LEN;
        c.g[A3][0] = c.g[S0][0];
        callee::TAG_FIELD_SET_STRING.call(c, 0x003f_10b0)?;
        c.g[A0][0] = c.g[SP][0];
        c.g[A2][0] = sx(BUDDY_TAG_GROUP);
        c.g[A1][0] = BUDDY_RECORD_LEN;
        c.g[A3][0] = buddy_addr(c, S0, ROSTER_ENTRY_GROUP);
        callee::TAG_FIELD_SET_STRING.call(c, 0x003f_10c8)?;
        c.g[A1][0] = sx(BUDDY_KIND_RADD);
        c.g[A0][0] = c.g[S1][0];
        c.g[A2][0] = c.g[SP][0];
        callee::BUDDY_API_SEND_PACKET.call(c, 0x003f_10dc)?;
        buddy_add_mark_sent(c);
    }
    // The I list (flag 0x200).
    c.cycles += 4;
    c.g[V0][0] = buddy_field(c, S0, ROSTER_ENTRY_FLAGS).lw(c) & ROSTER_FLAG_LIST_I;
    c.g[A0][0] = c.g[SP][0];
    if c.g[V0][0] != 0 {
        c.cycles += 36;
        c.sb(record, 0);
        c.g[A2][0] = sx(BUDDY_TAG_LRSC);
        c.g[A1][0] = BUDDY_RECORD_LEN;
        c.g[A3][0] = buddy_addr(c, S1, BUDDY_USER);
        callee::TAG_FIELD_SET_STRING.call(c, 0x003f_111c)?;
        c.g[A3][0] = buddy_field(c, S1, BUDDY_OPS + BUDDY_OP_ID).lw(c);
        c.g[A0][0] = c.g[SP][0];
        c.g[A2][0] = sx(BUDDY_TAG_ID);
        c.g[A1][0] = BUDDY_RECORD_LEN;
        callee::TAG_FIELD_SET_NUMBER.call(c, 0x003f_1134)?;
        c.g[A0][0] = c.g[SP][0];
        c.g[A2][0] = sx(BUDDY_TAG_LIST);
        c.g[A3][0] = sx(BUDDY_STR_I);
        c.g[A1][0] = BUDDY_RECORD_LEN;
        callee::TAG_FIELD_SET_STRING.call(c, 0x003f_1150)?;
        c.g[A0][0] = c.g[SP][0];
        c.g[A2][0] = sx(BUDDY_TAG_USER);
        c.g[A1][0] = BUDDY_RECORD_LEN;
        c.g[A3][0] = c.g[S0][0];
        callee::TAG_FIELD_SET_STRING.call(c, 0x003f_1168)?;
        c.g[A1][0] = sx(BUDDY_KIND_RADD);
        c.g[A0][0] = c.g[S1][0];
        c.g[A2][0] = c.g[SP][0];
        callee::BUDDY_API_SEND_PACKET.call(c, 0x003f_117c)?;
        buddy_add_mark_sent(c);
    }
    // The P list (flag 0x800000).
    c.cycles += 5;
    c.g[V0][0] = buddy_field(c, S0, ROSTER_ENTRY_FLAGS).lw(c);
    c.g[V1][0] = ROSTER_FLAG_LIST_P;
    c.g[V0][0] &= c.g[V1][0];
    c.g[A0][0] = c.g[SP][0];
    if c.g[V0][0] != 0 {
        c.cycles += 11;
        c.g[A2][0] = sx(BUDDY_TAG_USER);
        c.g[A1][0] = BUDDY_RECORD_LEN;
        c.g[A3][0] = c.g[S0][0];
        c.sb(record, 0);
        callee::TAG_FIELD_SET_STRING.call(c, 0x003f_11c0)?;
        c.g[A1][0] = sx(BUDDY_KIND_PADD);
        c.g[A0][0] = c.g[S1][0];
        c.g[A2][0] = c.g[SP][0];
        callee::BUDDY_API_SEND_PACKET.call(c, 0x003f_11d4)?;
    }
    Ok(())
}

/// After a 'RADD': flag the record (`s2`) sent and stamp it with the
/// current sequence number.
pub(crate) fn buddy_add_mark_sent(c: &mut Cpu) {
    c.g[V0][0] = buddy_field(c, S2, ROSTER_ENTRY_FLAGS).lw(c);
    c.g[V1][0] = ROSTER_FLAG_ADDING;
    c.g[V0][0] |= c.g[V1][0];
    let flags = c.g[V0][0];
    buddy_field(c, S2, ROSTER_ENTRY_FLAGS).sw(c, flags);
    c.g[V1][0] = buddy_field(c, S1, BUDDY_OP_COUNTER).lw(c);
    let seq = c.g[V1][0];
    buddy_field(c, S2, ROSTER_ENTRY_OP_ID).sw(c, seq);
}

/// `int BuddyApi_Del(BuddyApiT* api, BuddyT* buddy, a2, a3, t0)` (0x3f11f8):
/// take a buddy off the server lists its flags (+0xa8) name. Returns -2
/// for a missing buddy, no roster or a name not in the roster, and -3 for a
/// name with '/' or '@' (or with '.' while the word at +0x18 is zero).
/// Otherwise records the delete as the pending operation (`a2`, `a3`,
/// timeout `t0`), clears those flags of the roster record and sends 'RDEL'
/// for the B list (flag 4: also `PRES`=Y and the `GROUP` at +0x20) and the I
/// list (0x200), and 'PDEL' for flag 0x800000, each stamped with the delete
/// slot's sequence number (`ID`, not for 'PDEL'). A record left without
/// flags 0x100204 is removed from the hash and list and freed. Returns 0.
pub fn buddy_api_del(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x170);
        'done: {
            'result: {
                c.cycles += 15;
                frame.save(c, S0, 0x160);
                frame.save(c, S1, 0x150);
                c.g[S0][0] = c.g[A0][0];
                frame.save(c, S2, 0x140);
                c.g[S1][0] = c.g[A1][0];
                frame.save(c, S3, 0x130);
                c.g[S2][0] = c.g[A2][0];
                frame.save(c, S4, 0x120);
                c.g[S3][0] = c.g[A3][0];
                frame.save(c, S5, 0x110);
                c.g[S4][0] = c.g[T0][0];
                frame.save_ra(c, 0x100);
                c.g[S5][0] = BUDDY_ERR_NONE;
                if c.g[S1][0] == 0 {
                    break 'result;
                }
                c.cycles += 3;
                c.g[V0][0] = buddy_field(c, S0, BUDDY_ROSTER_LIST).lw(c);
                let no_roster = c.g[V0][0] == 0;
                c.g[V0][0] = c.g[S5][0];
                if no_roster {
                    break 'done;
                }
                // The name checks.
                c.cycles += 5;
                c.g[A0][0] = c.g[S1][0];
                c.g[A1][0] = b'/' as u64;
                callee::STRCHR.call(c, 0x003f_124c)?;
                let slash = c.g[V0][0] != 0;
                c.g[V0][0] = BUDDY_ERR_NAME;
                if slash {
                    break 'done;
                }
                c.cycles += 5;
                c.g[A0][0] = c.g[S1][0];
                c.g[A1][0] = b'@' as u64;
                callee::STRCHR.call(c, 0x003f_1260)?;
                if c.g[V0][0] != 0 {
                    c.cycles += 2;
                    c.g[V0][0] = BUDDY_ERR_NAME;
                    break 'done;
                }
                c.g[V0][0] = buddy_field(c, S0, BUDDY_ACCEPT_DOTTED).lw(c);
                c.cycles += 2;
                let allow_dot = c.g[V0][0] != 0;
                c.g[T0][0] = c.g[S4][0];
                if !allow_dot {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[A1][0] = b'.' as u64;
                    callee::STRCHR.call(c, 0x003f_1284)?;
                    let dot = c.g[V0][0] != 0;
                    c.g[V0][0] = BUDDY_ERR_NAME;
                    if dot {
                        break 'done;
                    }
                    c.cycles += 1;
                    c.g[T0][0] = c.g[S4][0];
                }
                c.cycles += 11;
                c.g[A2][0] = c.g[S2][0];
                c.g[A3][0] = c.g[S3][0];
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = BUDDY_OP_KIND_DEL;
                callee::BUDDY_API_SET_PENDING_OP.call(c, 0x003f_12a4)?;
                c.g[A0][0] = buddy_field(c, S0, BUDDY_ROSTER_HASH).lw(c);
                c.g[A1][0] = c.g[S1][0];
                callee::LOBBY_HASH_FIND_STR.call(c, 0x003f_12b0)?;
                c.g[S4][0] = c.g[V0][0];
                let unknown = c.g[S4][0] == 0;
                c.g[V0][0] = c.g[S5][0];
                if unknown {
                    break 'done;
                }
                // Clear the buddy's flags on the roster record (s3); s2 =
                // the flags cleared.
                c.cycles += 11;
                c.g[A0][0] = buddy_field(c, S0, BUDDY_ROSTER_LIST).lw(c);
                c.g[A1][0] = c.g[S4][0];
                callee::LOBBY_LIST_GET_DATA.call(c, 0x003f_12c8)?;
                c.g[S3][0] = c.g[V0][0];
                c.g[V0][0] = buddy_field(c, S1, ROSTER_ENTRY_FLAGS).lw(c);
                c.g[V1][0] = buddy_field(c, S3, ROSTER_ENTRY_FLAGS).lw(c);
                c.g[S2][0] = c.g[V1][0] & c.g[V0][0];
                c.g[V1][0] ^= c.g[S2][0];
                let kept = c.g[V1][0];
                buddy_field(c, S3, ROSTER_ENTRY_FLAGS).sw(c, kept);
                buddy_del_send_lists(c)?;
                // Drop a record no list keeps.
                c.cycles += 6;
                c.g[V1][0] = buddy_field(c, S3, ROSTER_ENTRY_FLAGS).lw(c);
                c.g[V0][0] = ROSTER_FLAG_KEEP;
                c.g[V1][0] &= c.g[V0][0];
                let keep = c.g[V1][0] != 0;
                c.g[S5][0] = 0;
                if keep {
                    break 'result;
                }
                c.cycles += 9;
                c.g[A0][0] = buddy_field(c, S0, BUDDY_ROSTER_HASH).lw(c);
                c.g[A1][0] = c.g[S1][0];
                callee::LOBBY_HASH_DEL_STR.call(c, 0x003f_1494)?;
                c.g[A0][0] = buddy_field(c, S0, BUDDY_ROSTER_LIST).lw(c);
                c.g[A1][0] = c.g[S4][0];
                callee::LOBBY_LIST_REMOVE.call(c, 0x003f_14a0)?;
                c.g[A0][0] = c.g[V0][0];
                callee::LOBBY_MEM_FREE.call(c, 0x003f_14a8)?;
                c.g[S5][0] = 0;
            }
            c.cycles += 1;
            c.g[V0][0] = c.g[S5][0];
        }
        c.cycles += 9;
        frame.restore(c, S0, 0x160);
        frame.restore(c, S1, 0x150);
        frame.restore(c, S2, 0x140);
        frame.restore(c, S3, 0x130);
        frame.restore(c, S4, 0x120);
        frame.restore(c, S5, 0x110);
        frame.restore_ra(c, 0x100);
        frame.close(c);
        Ok(())
    })
}

/// `BuddyApi_Del`'s requests for the flags `s2` cleared from the record of
/// the buddy `s1`.
pub(crate) fn buddy_del_send_lists(c: &mut Cpu) -> Flow {
    let record = sp(c);
    let del_seq = BUDDY_OPS + BUDDY_OP_SIZE + BUDDY_OP_ID;
    // The B list (flag 4).
    c.g[V0][0] = c.g[S2][0] & ROSTER_FLAG_LIST_B;
    if c.g[V0][0] != 0 {
        c.cycles += 44;
        c.g[A3][0] = buddy_field(c, S0, del_seq).lw(c);
        c.g[A0][0] = c.g[SP][0];
        c.sb(record, 0);
        c.g[A2][0] = sx(BUDDY_TAG_ID);
        c.g[A1][0] = BUDDY_RECORD_LEN;
        callee::TAG_FIELD_SET_NUMBER.call(c, 0x003f_1304)?;
        c.g[A0][0] = c.g[SP][0];
        c.g[A2][0] = sx(BUDDY_TAG_LRSC);
        c.g[A3][0] = buddy_addr(c, S0, BUDDY_USER);
        c.g[A1][0] = BUDDY_RECORD_LEN;
        callee::TAG_FIELD_SET_STRING.call(c, 0x003f_131c)?;
        c.g[A0][0] = c.g[SP][0];
        c.g[A2][0] = sx(BUDDY_TAG_LIST);
        c.g[A3][0] = sx(BUDDY_STR_B);
        c.g[A1][0] = BUDDY_RECORD_LEN;
        callee::TAG_FIELD_SET_STRING.call(c, 0x003f_1338)?;
        c.g[A0][0] = c.g[SP][0];
        c.g[A2][0] = sx(BUDDY_TAG_PRES);
        c.g[A3][0] = sx(BUDDY_STR_Y);
        c.g[A1][0] = BUDDY_RECORD_LEN;
        callee::TAG_FIELD_SET_STRING.call(c, 0x003f_1354)?;
        c.g[A0][0] = c.g[SP][0];
        c.g[A2][0] = sx(BUDDY_TAG_USER);
        c.g[A1][0] = BUDDY_RECORD_LEN;
        c.g[A3][0] = c.g[S1][0];
        callee::TAG_FIELD_SET_STRING.call(c, 0x003f_136c)?;
        c.g[A0][0] = c.g[SP][0];
        c.g[A2][0] = sx(BUDDY_TAG_GROUP);
        c.g[A1][0] = BUDDY_RECORD_LEN;
        c.g[A3][0] = buddy_addr(c, S1, ROSTER_ENTRY_GROUP);
        callee::TAG_FIELD_SET_STRING.call(c, 0x003f_1384)?;
        c.g[A1][0] = sx(BUDDY_KIND_RDEL);
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[SP][0];
        callee::BUDDY_API_SEND_PACKET.call(c, 0x003f_1398)?;
    }
    // The I list (flag 0x200).
    c.cycles += 3;
    c.g[V0][0] = c.g[S2][0] & ROSTER_FLAG_LIST_I;
    c.g[A0][0] = c.g[SP][0];
    if c.g[V0][0] != 0 {
        c.cycles += 30;
        c.g[A3][0] = buddy_field(c, S0, del_seq).lw(c);
        c.sb(record, 0);
        c.g[A2][0] = sx(BUDDY_TAG_ID);
        c.g[A1][0] = BUDDY_RECORD_LEN;
        callee::TAG_FIELD_SET_NUMBER.call(c, 0x003f_13bc)?;
        c.g[A0][0] = c.g[SP][0];
        c.g[A2][0] = sx(BUDDY_TAG_LRSC);
        c.g[A3][0] = buddy_addr(c, S0, BUDDY_USER);
        c.g[A1][0] = BUDDY_RECORD_LEN;
        callee::TAG_FIELD_SET_STRING.call(c, 0x003f_13d4)?;
        c.g[A2][0] = sx(BUDDY_TAG_LIST);
        c.g[A0][0] = c.g[SP][0];
        c.g[A3][0] = sx(BUDDY_STR_I);
        c.g[A1][0] = BUDDY_RECORD_LEN;
        callee::TAG_FIELD_SET_STRING.call(c, 0x003f_13f0)?;
        c.g[A0][0] = c.g[SP][0];
        c.g[A2][0] = sx(BUDDY_TAG_USER);
        c.g[A1][0] = BUDDY_RECORD_LEN;
        c.g[A3][0] = c.g[S1][0];
        callee::TAG_FIELD_SET_STRING.call(c, 0x003f_1408)?;
        c.g[A1][0] = sx(BUDDY_KIND_RDEL);
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[SP][0];
        callee::BUDDY_API_SEND_PACKET.call(c, 0x003f_141c)?;
    }
    // The P list (flag 0x800000).
    c.cycles += 4;
    c.g[V0][0] = c.g[S2][0] & ROSTER_FLAG_LIST_P;
    c.g[A0][0] = c.g[SP][0];
    if c.g[V0][0] != 0 {
        c.cycles += 17;
        c.g[A2][0] = sx(BUDDY_TAG_LRSC);
        c.sb(record, 0);
        c.g[A1][0] = BUDDY_RECORD_LEN;
        c.g[A3][0] = buddy_addr(c, S0, BUDDY_USER);
        callee::TAG_FIELD_SET_STRING.call(c, 0x003f_1444)?;
        c.g[A2][0] = sx(BUDDY_TAG_USER);
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] = BUDDY_RECORD_LEN;
        c.g[A3][0] = c.g[S1][0];
        callee::TAG_FIELD_SET_STRING.call(c, 0x003f_145c)?;
        c.g[A1][0] = sx(BUDDY_KIND_PDEL);
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[SP][0];
        callee::BUDDY_API_SEND_PACKET.call(c, 0x003f_1470)?;
    }
    Ok(())
}

/// `BuddyT* BuddyApi_Find(BuddyApiT* api, const char* name)` (0x3f14d8):
/// the roster record of `name` (case-insensitive), or 0 without a roster
/// hash, a name or a match.
pub fn buddy_api_find(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x30);
        'done: {
            c.cycles += 8;
            frame.save(c, S0, 0x20);
            frame.save(c, S1, 0x10);
            c.g[S0][0] = c.g[A0][0];
            frame.save_ra(c, 0);
            c.g[A0][0] = buddy_field(c, S0, BUDDY_ROSTER_HASH).lw(c);
            c.g[S1][0] = 0;
            if c.g[A0][0] != 0 {
                c.cycles += 2;
                let no_name = c.g[A1][0] == 0;
                c.g[V0][0] = c.g[S1][0];
                if no_name {
                    break 'done;
                }
                c.cycles += 4;
                callee::LOBBY_HASH_FIND_STR.call(c, 0x003f_1508)?;
                if c.g[V0][0] == 0 {
                    c.g[V0][0] = c.g[S1][0];
                    break 'done;
                }
                c.cycles += 4;
                c.g[A0][0] = buddy_field(c, S0, BUDDY_ROSTER_LIST).lw(c);
                c.g[A1][0] = c.g[V0][0];
                callee::LOBBY_LIST_GET_DATA.call(c, 0x003f_151c)?;
                c.g[S1][0] = c.g[V0][0];
            }
            c.cycles += 1;
            c.g[V0][0] = c.g[S1][0];
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Presence.

/// `void BuddyApi_ClearPresence(BuddyApiT* api)` (0x3f1538): empty the
/// status record (+0x140) and the presence text (+0x40).
pub fn buddy_api_clear_presence(c: &mut Cpu) {
    c.cycles += 3;
    let api = r32(c, A0);
    c.sb(api.wrapping_add(BUDDY_STATUS), 0);
    c.sb(api.wrapping_add(BUDDY_PRESENCE_TEXT), 0);
    ret(c);
}

/// `void BuddyApi_SetPresenceText(BuddyApiT* api, const char* text)`
/// (0x3f1548): copy `text` into the presence text (0x100 bytes, +0x40)
/// when the byte at +0x2c8 is set.
pub fn buddy_api_set_presence_text(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x10);
        c.cycles += 5;
        frame.save_ra(c, 0);
        c.g[V0][0] = c.lb(r32(c, A0).wrapping_add(BUDDY_PRES));
        let off = c.g[V0][0] == 0;
        c.g[A2][0] = c.g[A1][0];
        if !off {
            c.cycles += 3;
            c.g[A0][0] = buddy_addr(c, A0, BUDDY_PRESENCE_TEXT);
            c.g[A1][0] = BUDDY_RECORD_LEN;
            callee::TAG_FIELD_STRNZCPY.call(c, 0x003f_1568)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void BuddyApi_SetPresenceField(BuddyApiT* api, const char* value,
/// const char* name)` (0x3f1578): store the tag `name=value` in the status
/// record (+0x140); only two-character names and non-empty values.
pub fn buddy_api_set_presence_field(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x10);
        c.cycles += 4;
        let name = r32(c, A2);
        c.g[V0][0] = c.lb(name);
        frame.save_ra(c, 0);
        'restored: {
            'stored: {
                if c.g[V0][0] == 0 {
                    break 'stored;
                }
                c.cycles += 3;
                c.g[V0][0] = c.lb(name.wrapping_add(1));
                let short = c.g[V0][0] == 0;
                frame.restore_ra(c, 0);
                if short {
                    break 'restored;
                }
                c.cycles += 3;
                c.g[V0][0] = c.lb(name.wrapping_add(2));
                if c.g[V0][0] != 0 {
                    break 'restored;
                }
                c.cycles += 3;
                c.g[V0][0] = c.lb(r32(c, A1));
                let empty = c.g[V0][0] == 0;
                c.g[A3][0] = c.g[A1][0];
                if empty {
                    break 'restored;
                }
                c.cycles += 3;
                c.g[A0][0] = buddy_addr(c, A0, BUDDY_STATUS);
                c.g[A1][0] = BUDDY_RECORD_LEN;
                callee::TAG_FIELD_SET_STRING.call(c, 0x003f_15b8)?;
            }
            c.cycles += 1;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `int BuddyApi_SetPresence(BuddyApiT* api, const char* resource, int show,
/// int delay)` (0x3f15c8): rebuild the presence record (+0x240:
/// `RSRC` = `resource`, `SHOW` = the show name for `show` 1..5, else entry
/// 0 of the table at 0x4512c0). Without a scheduled send (+0x2c0 zero) one
/// is scheduled `delay` ticks from now; a changed show (+0x2c4) is kept and
/// due now. When the send is due (now at or past it, unsigned), the
/// presence goes out (`BuddyApi_SendPresence`). Returns 0.
pub fn buddy_api_set_presence(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x80);
        c.cycles += 20;
        frame.save(c, S1, 0x60);
        frame.save(c, S2, 0x50);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S3, 0x40);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, S5, 0x20);
        frame.save(c, S6, 0x10);
        c.g[S5][0] = c.g[A1][0];
        frame.save(c, S0, 0x70);
        c.g[S6][0] = c.g[A3][0];
        frame.save(c, S4, 0x30);
        frame.save_ra(c, 0);
        c.g[A0][0] = buddy_field(c, S1, BUDDY_ARIES).lw(c);
        callee::PROTO_ARIES_TICK.call(c, 0x003f_1604)?;
        c.g[S3][0] = c.g[V0][0];
        // The show name.
        c.g[V0][0] = sx(r32(c, S2).wrapping_sub(1));
        let listed = c.g[V0][0] < 5;
        c.g[V0][0] = buddy_lui(BUDDY_SHOW_NAMES);
        if listed {
            c.cycles += 5;
            c.g[V1][0] = sx(r32(c, S2) << 2);
            c.g[V0][0] = sx(BUDDY_SHOW_NAMES);
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
            c.g[S4][0] = c.lw(r32(c, V1));
        } else {
            c.cycles += 1;
            c.g[S4][0] = c.lw(BUDDY_SHOW_NAMES);
        }
        // The presence record.
        c.cycles += 17;
        c.g[S0][0] = buddy_addr(c, S1, BUDDY_PRESENCE);
        let record = r32(c, S0);
        c.sb(record, 0);
        c.g[A2][0] = sx(BUDDY_TAG_RSRC);
        c.g[A3][0] = c.g[S5][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = BUDDY_PRESENCE_SIZE;
        callee::TAG_FIELD_SET_STRING.call(c, 0x003f_1650)?;
        c.g[A2][0] = sx(BUDDY_TAG_SHOW);
        c.g[A0][0] = c.g[S0][0];
        c.g[A3][0] = c.g[S4][0];
        c.g[A1][0] = BUDDY_PRESENCE_SIZE;
        callee::TAG_FIELD_SET_STRING.call(c, 0x003f_1668)?;
        // Schedule.
        c.g[V1][0] = buddy_field(c, S1, BUDDY_PRESENCE_DUE).lw(c);
        if c.g[V1][0] == 0 {
            c.cycles += 3;
            c.g[V0][0] = sx(r32(c, S3).wrapping_add(r32(c, S6)));
            let due = c.g[V0][0];
            buddy_field(c, S1, BUDDY_PRESENCE_DUE).sw(c, due);
        }
        c.g[V0][0] = buddy_field(c, S1, BUDDY_PRESENCE_SHOW).lw(c);
        c.cycles += 2;
        if c.g[V0][0] != c.g[S2][0] {
            c.cycles += 3;
            let (show, now) = (c.g[S2][0], c.g[S3][0]);
            buddy_field(c, S1, BUDDY_PRESENCE_SHOW).sw(c, show);
            buddy_field(c, S1, BUDDY_PRESENCE_DUE).sw(c, now);
        }
        c.g[V0][0] = buddy_field(c, S1, BUDDY_PRESENCE_DUE).lw(c);
        // Send when due.
        c.cycles += 3;
        c.g[V0][0] = (c.g[S3][0] < c.g[V0][0]) as u64;
        let early = c.g[V0][0] != 0;
        frame.restore(c, S0, 0x70);
        if !early {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            callee::BUDDY_API_SEND_PRESENCE.call(c, 0x003f_16a8)?;
            frame.restore(c, S0, 0x70);
        }
        c.cycles += 10;
        c.g[V0][0] = 0;
        frame.restore(c, S1, 0x60);
        frame.restore(c, S2, 0x50);
        frame.restore(c, S3, 0x40);
        frame.restore(c, S4, 0x30);
        frame.restore(c, S5, 0x20);
        frame.restore(c, S6, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Messages and callbacks.

/// `int BuddyApi_Send(BuddyApiT* api, char* message, a2, a3, t0)`
/// (0x3f16d8): send a message tag record as a 'SEND' packet, recorded as the
/// pending send (`a2`, `a3`, timeout `t0`). Refuses (-3) a `BODY` of 64 or
/// more characters and any non-empty `SUBJ`. Returns 0 when sent.
pub fn buddy_api_send(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x70);
        'done: {
            'refuse: {
                c.cycles += 27;
                frame.save(c, S0, 0x60);
                frame.save(c, S2, 0x40);
                c.g[S0][0] = c.g[A1][0];
                frame.save(c, S3, 0x30);
                c.g[S2][0] = c.g[A0][0];
                frame.save(c, S4, 0x20);
                frame.save(c, S5, 0x10);
                c.g[S3][0] = c.g[A2][0];
                frame.save(c, S1, 0x50);
                c.g[S4][0] = c.g[A3][0];
                frame.save_ra(c, 0);
                c.g[S5][0] = c.g[T0][0];
                c.g[A1][0] = sx(BUDDY_TAG_BODY);
                c.g[A0][0] = c.g[S0][0];
                callee::TAG_FIELD_FIND.call(c, 0x003f_171c)?;
                c.g[V1][0] = buddy_lui(BUDDY_STR_EMPTY);
                c.g[A0][0] = c.g[V0][0];
                c.g[S1][0] = sx(BUDDY_STR_EMPTY);
                c.g[A1][0] = 0;
                c.g[A2][0] = 0;
                c.g[A3][0] = c.g[S1][0];
                callee::TAG_FIELD_GET_STRING.call(c, 0x003f_1738)?;
                c.g[V0][0] = ((c.g[V0][0] as i64) < 64) as u64;
                let too_long = c.g[V0][0] == 0;
                c.g[A1][0] = buddy_lui(BUDDY_TAG_SUBJ);
                if too_long {
                    break 'refuse;
                }
                c.cycles += 10;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = sx(BUDDY_TAG_SUBJ);
                callee::TAG_FIELD_FIND.call(c, 0x003f_1750)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A3][0] = c.g[S1][0];
                c.g[A1][0] = 0;
                c.g[A2][0] = 0;
                callee::TAG_FIELD_GET_STRING.call(c, 0x003f_1764)?;
                let subject = (c.g[V0][0] as i64) > 0;
                c.g[V0][0] = BUDDY_ERR_NAME;
                if subject {
                    break 'done;
                }
                c.cycles += 13;
                c.g[A2][0] = c.g[S3][0];
                c.g[A3][0] = c.g[S4][0];
                c.g[T0][0] = c.g[S5][0];
                c.g[A0][0] = c.g[S2][0];
                c.g[A1][0] = BUDDY_OP_KIND_SEND;
                callee::BUDDY_API_SET_PENDING_OP.call(c, 0x003f_1784)?;
                c.g[A1][0] = sx(BUDDY_KIND_SEND);
                c.g[A0][0] = c.g[S2][0];
                c.g[A2][0] = c.g[S0][0];
                callee::BUDDY_API_SEND_PACKET.call(c, 0x003f_1798)?;
                c.g[V0][0] = 0;
                break 'done;
            }
            c.cycles += 1;
            c.g[V0][0] = BUDDY_ERR_NAME;
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

/// `void BuddyApi_SetCallback(BuddyApiT* api, EventProc proc, void* data)`
/// (0x3f17c8): set the event callback (+0x4b0) and its data (+0x4ac).
pub fn buddy_api_set_callback(c: &mut Cpu) {
    c.cycles += 3;
    let api = r32(c, A0);
    c.sw(api.wrapping_add(BUDDY_MESSAGE_ARG), c.g[A2][0]);
    c.sw(api.wrapping_add(BUDDY_MESSAGE_FN), c.g[A1][0]);
    ret(c);
}

// ---------------------------------------------------------------------------
// Buddy client: the per-tick driver.

/// States `BuddyApi_Process` sets or tests, named by what sets them.
pub(crate) const BUDDY_STATE_POLL_FAILED: u64 = 2;

pub(crate) const BUDDY_STATE_LOGIN_TIMEOUT: u64 = 3;

pub(crate) const BUDDY_STATE_LINK_UP: u64 = 5;

pub(crate) const BUDDY_STATE_LINK_DOWN: u64 = 6;

/// The `ID` of the main roster list (`RGET`, `ROST`).
pub(crate) const ROSTER_LIST1: u64 = 1;

pub(crate) const ROSTER_LIST2: u64 = 2;

// Packet kinds (the `kind` word of a received or sent packet).
/// -1: link status.
pub(crate) const BUDDY_KIND_LINK: u64 = 0xffff_ffff_ffff_ffff;

/// "PING"
pub(crate) const BUDDY_KIND_PING: u32 = 0x5049_4e47;

/// "AUTH"
pub(crate) const BUDDY_KIND_AUTH: u32 = 0x4155_5448;

/// "RECV"
pub(crate) const BUDDY_KIND_RECV: u32 = 0x5245_4356;

/// "SEND"
pub(crate) const BUDDY_KIND_SEND: u32 = 0x5345_4e44;

/// "ADMN"
pub(crate) const BUDDY_KIND_ADMN: u32 = 0x4144_4d4e;

/// "RGET"
pub(crate) const BUDDY_KIND_RGET: u32 = 0x5247_4554;

/// "ROST"
pub(crate) const BUDDY_KIND_ROST: u32 = 0x524f_5354;

/// "PGET"
pub(crate) const BUDDY_KIND_PGET: u32 = 0x5047_4554;

/// "RADD"
pub(crate) const BUDDY_KIND_RADD: u32 = 0x5241_4444;

/// "RDEL"
pub(crate) const BUDDY_KIND_RDEL: u32 = 0x5244_454c;

/// "PADD"
pub(crate) const BUDDY_KIND_PADD: u32 = 0x5041_4444;

/// "PDEL"
pub(crate) const BUDDY_KIND_PDEL: u32 = 0x5044_454c;

/// "DISC"
pub(crate) const BUDDY_KIND_DISC: u32 = 0x4449_5343;

// Tags of the messages handed to the callbacks.
/// "auth"
pub(crate) const BUDDY_TYPE_AUTH: u32 = 0x6175_7468;

/// "send"
pub(crate) const BUDDY_TYPE_SEND: u32 = 0x7365_6e64;

/// "admn"
pub(crate) const BUDDY_TYPE_ADMN: u32 = 0x6164_6d6e;

/// "addu"
pub(crate) const BUDDY_TYPE_ADDU: u32 = 0x6164_6475;

/// "delu"
pub(crate) const BUDDY_TYPE_DELU: u32 = 0x6465_6c75;

/// "endr"
pub(crate) const BUDDY_TYPE_ENDR: u32 = 0x656e_6472;

/// "rost"
pub(crate) const BUDDY_TYPE_ROST: u32 = 0x726f_7374;

/// Message code -4: an op cancelled (state 3/6).
pub(crate) const OP_CANCELLED: u64 = 0xffff_ffff_ffff_fffc;

/// Message code -2: an add refused.
pub(crate) const ADD_REFUSED: u64 = 0xffff_ffff_ffff_fffe;

// Strings in the game's data: tag names, formats.
/// The `lui` of the tag names (left in t0/t1/t2, s4 and s5). One load serves
/// several names, so it is not spelled `buddy_lui` of one of them.
pub(crate) const STRINGS_PAGE: u64 = 0x004a_0000;

/// "SIZE"
pub(crate) const BUDDY_TAG_SIZE: u32 = 0x004a_4b88;

/// "FUSR"
pub(crate) const BUDDY_TAG_FUSR: u32 = 0x004a_4b90;

/// "recv: "
pub(crate) const DEBUG_RECV_PREFIX: u32 = 0x004a_4b80;

/// "connecting to %08x:%d\n"
pub(crate) const FMT_CONNECTING: u32 = 0x0049_6ba0;

/// `lui` of the presence format (left in v1).
pub(crate) const FMT_PRESENCE_PAGE: u64 = 0x0049_0000;

/// "PRES: buddy=%s, show=%d, ..."
pub(crate) const FMT_PRESENCE: u32 = 0x0049_6bb8;

/// `BuddyApi_Process`'s frame: the user name buffer at 0, the message handed
/// to the callbacks at 0x20 ({tag, kind, code, data}), the roster entry copy
/// at 0x30, the peeked data pointer, the "roster complete" flag and the
/// kind/code pointers the peek fills.
pub(crate) const BUDDY_PROCESS_FRAME: u32 = 0x1a0;

pub(crate) const BUDDY_PROCESS_MSG: u32 = 0x20;

pub(crate) const BUDDY_PROCESS_MSG_KIND: u32 = 0x24;

pub(crate) const BUDDY_PROCESS_MSG_CODE: u32 = 0x28;

pub(crate) const BUDDY_PROCESS_MSG_DATA: u32 = 0x2c;

pub(crate) const BUDDY_PROCESS_ENTRY_COPY: u32 = 0x30;

pub(crate) const BUDDY_PROCESS_PEEK_DATA: u32 = 0xf0;

pub(crate) const BUDDY_PROCESS_ROSTER_DONE: u32 = 0xf4;

pub(crate) const BUDDY_PROCESS_KIND_PTR: u32 = 0xf8;

pub(crate) const BUDDY_PROCESS_CODE_PTR: u32 = 0xfc;

/// Entry and resume points (loop heads) of `BuddyApi_Process`.
pub(crate) const BUDDY_PROCESS_ENTRY: u32 = 0x003f_17d8;

pub(crate) const BUDDY_PROCESS_END: u32 = 0x003f_221c;

/// The op timeout scan: next op, loop test.
pub(crate) const BUDDY_PROCESS_OP_NEXT: u32 = 0x003f_1958;

pub(crate) const BUDDY_PROCESS_OP_TEST: u32 = 0x003f_195c;

/// The op cancel loop (state 3 or 6).
pub(crate) const BUDDY_PROCESS_CANCEL: u32 = 0x003f_19d0;

/// One received message.
pub(crate) const BUDDY_PROCESS_MESSAGE: u32 = 0x003f_1a10;

/// The roster entry copy before a presence update (entry not 8-aligned,
/// 8-aligned).
pub(crate) const BUDDY_PROCESS_COPY_UNALIGNED: u32 = 0x003f_1e24;

pub(crate) const BUDDY_PROCESS_COPY_ALIGNED: u32 = 0x003f_1e7c;

/// The roster scan of an add reply.
pub(crate) const BUDDY_PROCESS_ADD_SCAN: u32 = 0x003f_1f90;

/// The `BuddyApi` field at `off` (`self` is in s2).
pub(crate) fn buddy_process_field(c: &Cpu, off: u32) -> Addr {
    Addr(r32(c, S2).wrapping_add(off))
}

/// A back edge: deliver a due event at `head`; `Err` when the thread unwinds
/// (the trampoline re-enters the function at `head`).
pub(crate) fn buddy_process_back_edge(c: &mut Cpu, head: u32) -> Flow {
    if back_edge_unwinds(c, head) {
        Err(Unwound)
    } else {
        Ok(())
    }
}

/// `void BuddyApi_Process(BuddyApi* self)` (0x3f17d8): the buddy client's
/// per-tick driver. Ticks the Aries link; while a state callback is set,
/// polls the connector being opened (connected: open the link and send the
/// `AUTH` login; failed: state 2; either way close it); times the login out
/// (state 3); in state 5 sends the presence when due and drops the link
/// (state 6) when a pending op's deadline passes; in state 3 or 6 cancels the
/// pending ops (code -4). Then handles every received packet by kind: link
/// status, `PING`, `AUTH`, `RECV`, `SEND`, `ADMN`, the roster replies
/// (`RGET`, `ROST`, `PGET`, `RADD`, `RDEL`) and the roster-complete report,
/// and finally reports a re-sorted roster (`rost`).
///
/// The original is a `match pc` dispatcher: an event at a loop back edge can
/// unwind, and the function is re-entered at that loop head with its state
/// in the registers (s0-s7, fp, v0/v1/a3 in the entry copy).
pub fn buddy_api_process(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame {
            size: BUDDY_PROCESS_FRAME,
        };
        match c.pc {
            BUDDY_PROCESS_ENTRY => {
                buddy_process_connect(c, frame)?;
                buddy_process_link_up(c, frame)?;
                buddy_process_cancel_ops(c, frame, false)?;
                buddy_process_receive(c, frame, None)
            }
            head @ (BUDDY_PROCESS_OP_NEXT | BUDDY_PROCESS_OP_TEST) => {
                buddy_process_op_timeouts(c, head)?;
                buddy_process_cancel_ops(c, frame, false)?;
                buddy_process_receive(c, frame, None)
            }
            BUDDY_PROCESS_CANCEL => {
                buddy_process_cancel_ops(c, frame, true)?;
                buddy_process_receive(c, frame, None)
            }
            head @ (BUDDY_PROCESS_MESSAGE
            | BUDDY_PROCESS_COPY_UNALIGNED
            | BUDDY_PROCESS_COPY_ALIGNED
            | BUDDY_PROCESS_ADD_SCAN) => buddy_process_receive(c, frame, Some(head)),
            pc => {
                c.jump_out(pc, BUDDY_PROCESS_ENTRY, BUDDY_PROCESS_END);
                Ok(())
            }
        }
    })
}

/// 0x3f17d8..0x3f18e8: the prologue, the link tick, the connector poll and
/// the login timeout. Leaves the tick in s0.
pub(crate) fn buddy_process_connect(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 21;
    Frame::open(c, BUDDY_PROCESS_FRAME);
    frame.save(c, S0, 0x190);
    frame.save(c, S2, 0x170);
    frame.save(c, S1, 0x180);
    c.g[S2][0] = c.g[A0][0];
    frame.save(c, S3, 0x160);
    frame.save(c, S4, 0x150);
    frame.save(c, S5, 0x140);
    frame.save(c, S6, 0x130);
    frame.save(c, S7, 0x120);
    frame.save(c, FP, 0x110);
    frame.save_ra(c, 0x100);
    c.g[A0][0] = buddy_process_field(c, BUDDY_ARIES).lw(c);
    callee::PROTO_ARIES_TICK.call(c, 0x003f_1810)?;
    c.g[A0][0] = buddy_process_field(c, BUDDY_ARIES).lw(c);
    c.g[S0][0] = c.g[V0][0];
    callee::PROTO_ARIES_NOP.call(c, 0x003f_181c)?;
    c.g[V0][0] = buddy_process_field(c, BUDDY_STATE_FN).lw(c);
    c.por(V1, ZERO, ZERO);
    let no_state_fn = c.g[V0][0] == 0;
    let msg = frame.at(c, BUDDY_PROCESS_MSG);
    c.sq(V1, msg);
    // v0 = the login deadline on every path to 0x3f18cc.
    'deadline: {
        'reload: {
            if no_state_fn {
                break 'reload;
            }
            // 0x3f182c
            c.cycles += 3;
            c.g[A0][0] = buddy_process_field(c, BUDDY_LOOKUP).lw(c);
            if c.g[A0][0] == 0 {
                c.g[V0][0] = buddy_process_field(c, BUDDY_LOGIN_DEADLINE).lw(c);
                break 'deadline;
            }
            // 0x3f1838: poll the connector.
            c.cycles += 6;
            c.g[V0][0] = buddy_field(c, A0, LOOKUP_POLL).lw(c);
            let poll = r32(c, V0);
            call_ptr(c, poll, 0x003f_1844)?;
            c.g[S3][0] = c.g[V0][0];
            if c.g[S3][0] == 0 {
                c.g[V0][0] = buddy_process_field(c, BUDDY_LOGIN_DEADLINE).lw(c);
                break 'deadline;
            }
            // 0x3f1850
            c.cycles += 2;
            if (c.g[S3][0] as i64) > 0 {
                // 0x3f1858: connected: open the link and send the login.
                c.cycles += 17;
                c.g[V0][0] = buddy_process_field(c, BUDDY_LOOKUP).lw(c);
                c.g[A0][0] = buddy_process_field(c, BUDDY_ARIES).lw(c);
                c.g[A1][0] = buddy_field(c, V0, LOOKUP_ADDRESS).lw(c);
                c.g[A2][0] = buddy_process_field(c, BUDDY_PORT).lw(c);
                callee::PROTO_ARIES_CONNECT.call(c, 0x003f_186c)?;
                c.g[V1][0] = buddy_process_field(c, BUDDY_LOOKUP).lw(c);
                c.g[A3][0] = buddy_process_field(c, BUDDY_PORT).lw(c);
                c.g[A1][0] = sx(FMT_CONNECTING);
                c.g[A2][0] = buddy_field(c, V1, LOOKUP_ADDRESS).lw(c);
                c.g[A0][0] = c.g[S2][0];
                callee::BUDDY_API_DEBUG_PRINTF.call(c, 0x003f_1888)?;
                c.g[A0][0] = c.g[S2][0];
                c.g[A1][0] = sx(BUDDY_KIND_AUTH);
                c.g[A2][0] = sx(r32(c, S2).wrapping_add(BUDDY_LOGIN));
                callee::BUDDY_API_SEND_PACKET.call(c, 0x003f_189c)?;
            }
            // 0x3f189c
            c.cycles += 2;
            if (c.g[S3][0] as i64) >= 0 {
                c.g[A0][0] = buddy_process_field(c, BUDDY_LOOKUP).lw(c);
            } else {
                // 0x3f18a4: the connector failed.
                c.cycles += 5;
                c.g[A0][0] = c.g[S2][0];
                c.g[A1][0] = BUDDY_STATE_POLL_FAILED;
                callee::BUDDY_API_SET_STATE.call(c, 0x003f_18b0)?;
                buddy_process_field(c, BUDDY_STATE_FN).sw(c, 0);
                c.g[A0][0] = buddy_process_field(c, BUDDY_LOOKUP).lw(c);
            }
            // 0x3f18b8: close the connector.
            c.cycles += 4;
            c.g[V0][0] = buddy_field(c, A0, LOOKUP_RELEASE).lw(c);
            let close = r32(c, V0);
            call_ptr(c, close, 0x003f_18c4)?;
            buddy_process_field(c, BUDDY_LOOKUP).sw(c, 0);
        }
        // 0x3f18c8
        c.cycles += 1;
        c.g[V0][0] = buddy_process_field(c, BUDDY_LOGIN_DEADLINE).lw(c);
    }
    // 0x3f18cc: the login deadline (0: none) against the tick.
    c.cycles += 2;
    let no_deadline = c.g[V0][0] == 0;
    c.g[V0][0] = (c.g[V0][0] < c.g[S0][0]) as u64;
    if !no_deadline {
        // 0x3f18d4
        c.cycles += 2;
        let pending = c.g[V0][0] == 0;
        c.g[A0][0] = c.g[S2][0];
        if !pending {
            // 0x3f18dc: the login timed out.
            c.cycles += 3;
            c.g[A1][0] = BUDDY_STATE_LOGIN_TIMEOUT;
            callee::BUDDY_API_SET_STATE.call(c, 0x003f_18e4)?;
            buddy_process_field(c, BUDDY_STATE_FN).sw(c, 0);
        }
    }
    Ok(())
}

/// The message registers set at 0x3f18e8 and again at 0x3f192c: s6 = the
/// message, fp = the peeked data slot, the kind/code pointers stored. True
/// when the state is 5 (link up).
pub(crate) fn buddy_process_message_regs(
    c: &mut Cpu,
    frame: Frame,
    kind_reg: usize,
    code_reg: usize,
) -> bool {
    c.g[kind_reg][0] = sx(frame.at(c, BUDDY_PROCESS_MSG_KIND));
    c.g[code_reg][0] = sx(frame.at(c, BUDDY_PROCESS_MSG_CODE));
    c.g[V0][0] = BUDDY_STATE_LINK_UP;
    c.g[S6][0] = sx(frame.at(c, BUDDY_PROCESS_MSG));
    Addr(frame.at(c, BUDDY_PROCESS_KIND_PTR)).sw(c, c.g[kind_reg][0]);
    Addr(frame.at(c, BUDDY_PROCESS_CODE_PTR)).sw(c, c.g[code_reg][0]);
    let link_up = c.g[V1][0] == c.g[V0][0];
    c.g[FP][0] = sx(frame.at(c, BUDDY_PROCESS_PEEK_DATA));
    link_up
}

/// 0x3f18e8..0x3f19a4: in state 5, send the presence when due, then check
/// the pending ops' deadlines.
pub(crate) fn buddy_process_link_up(c: &mut Cpu, frame: Frame) -> Flow {
    // 0x3f18e8
    c.cycles += 9;
    c.g[V1][0] = buddy_process_field(c, BUDDY_STATE).lw(c);
    if !buddy_process_message_regs(c, frame, A0, A1) {
        return Ok(());
    }
    // 0x3f190c
    c.cycles += 3;
    c.g[V0][0] = buddy_process_field(c, BUDDY_PRESENCE_DUE).lw(c);
    let not_pending = c.g[V0][0] == 0;
    c.g[V0][0] = (c.g[S0][0] < c.g[V0][0]) as u64;
    'state: {
        if !not_pending {
            // 0x3f1918
            c.cycles += 2;
            if c.g[V0][0] != 0 {
                c.g[V1][0] = buddy_process_field(c, BUDDY_STATE).lw(c);
                break 'state;
            }
            // 0x3f1920: the presence is due.
            c.cycles += 2;
            c.g[A0][0] = c.g[S2][0];
            callee::BUDDY_API_SEND_PRESENCE.call(c, 0x003f_1928)?;
        }
        // 0x3f1928
        c.cycles += 1;
        c.g[V1][0] = buddy_process_field(c, BUDDY_STATE).lw(c);
    }
    // 0x3f192c
    c.cycles += 8;
    if !buddy_process_message_regs(c, frame, A2, T0) {
        return Ok(());
    }
    // 0x3f194c
    c.cycles += 2;
    c.g[S3][0] = 0;
    buddy_process_op_timeouts(c, BUDDY_PROCESS_OP_TEST)
}

/// 0x3f1958..0x3f19a4: the ops' deadlines (index in s3, the tick in s0),
/// entered at `head`; one past its deadline drops the link (state 6).
pub(crate) fn buddy_process_op_timeouts(c: &mut Cpu, head: u32) -> Flow {
    let mut next = head == BUDDY_PROCESS_OP_NEXT;
    loop {
        if next {
            // 0x3f1958
            c.cycles += 1;
            c.g[S3][0] = sx(r32(c, S3).wrapping_add(1));
        }
        // 0x3f195c
        c.cycles += 3;
        c.g[V0][0] = ((c.g[S3][0] as i64) < BUDDY_OP_COUNT) as u64;
        let done = c.g[V0][0] == 0;
        c.g[V0][0] = BUDDY_OP_SIZE as u64;
        if done {
            return Ok(());
        }
        // 0x3f1968
        c.cycles += 5;
        c.mult(0, A0, S3, V0);
        c.g[V1][0] = sx(r32(c, S2).wrapping_add(r32(c, A0)));
        c.g[V0][0] = buddy_field(c, V1, BUDDY_OPS + BUDDY_OP_FN).lw(c);
        let idle = c.g[V0][0] == 0;
        c.g[V0][0] = c.g[V1][0];
        next = true;
        if idle {
            buddy_process_back_edge(c, BUDDY_PROCESS_OP_NEXT)?;
            continue;
        }
        // 0x3f197c
        c.cycles += 3;
        c.g[V0][0] = buddy_field(c, V0, BUDDY_OPS + BUDDY_OP_DEADLINE).lw(c);
        let no_deadline = c.g[V0][0] == 0;
        c.g[V0][0] = (c.g[V0][0] < c.g[S0][0]) as u64;
        if no_deadline {
            buddy_process_back_edge(c, BUDDY_PROCESS_OP_NEXT)?;
            continue;
        }
        // 0x3f1988
        c.cycles += 2;
        if c.g[V0][0] == 0 {
            c.g[S3][0] = sx(r32(c, S3).wrapping_add(1));
            next = false;
            buddy_process_back_edge(c, BUDDY_PROCESS_OP_TEST)?;
            continue;
        }
        // 0x3f1990: the op timed out: drop the link.
        c.cycles += 5;
        c.g[A0][0] = buddy_process_field(c, BUDDY_ARIES).lw(c);
        callee::PROTO_ARIES_UNCONNECT.call(c, 0x003f_1998)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = BUDDY_STATE_LINK_DOWN;
        callee::BUDDY_API_SET_STATE.call(c, 0x003f_19a4)?;
        return Ok(());
    }
}

/// 0x3f19a4..0x3f1a08: in state 3 or 6, call every pending op back with code
/// -4 and clear it (s0 = the fn slot, s1 = the arg slot, s3 = the end, s4 =
/// -4). `resume`: re-entered at the loop head. Leaves the link in a0.
pub(crate) fn buddy_process_cancel_ops(c: &mut Cpu, frame: Frame, resume: bool) -> Flow {
    if !resume {
        // 0x3f19a4
        c.cycles += 4;
        c.g[V1][0] = buddy_process_field(c, BUDDY_STATE).lw(c);
        c.g[V0][0] = BUDDY_STATE_LINK_DOWN;
        let link_down = c.g[V1][0] == c.g[V0][0];
        c.g[V0][0] = BUDDY_STATE_LOGIN_TIMEOUT;
        if !link_down {
            // 0x3f19b4
            c.cycles += 2;
            if c.g[V1][0] != c.g[V0][0] {
                c.g[A0][0] = buddy_process_field(c, BUDDY_ARIES).lw(c);
                return Ok(());
            }
        }
        // 0x3f19bc
        c.cycles += 5;
        c.g[S4][0] = OP_CANCELLED;
        c.g[S1][0] = sx(r32(c, S2).wrapping_add(BUDDY_OPS + BUDDY_OP_ARG));
        c.g[S0][0] = sx(r32(c, S2).wrapping_add(BUDDY_OPS + BUDDY_OP_FN));
        c.g[S3][0] = sx(r32(c, S2).wrapping_add(BUDDY_OPS + BUDDY_OP_FN + 3 * BUDDY_OP_SIZE));
    }
    loop {
        // 0x3f19d0
        c.cycles += 3;
        c.g[V1][0] = buddy_field(c, S0, 0).lw(c);
        let idle = c.g[V1][0] == 0;
        c.g[A0][0] = c.g[S2][0];
        if !idle {
            // 0x3f19dc
            c.cycles += 7;
            c.g[V0][0] = Addr(r32(c, S1).wrapping_sub(BUDDY_OP_ARG - BUDDY_OP_TAG)).lw(c);
            c.g[A2][0] = buddy_field(c, S1, 0).lw(c);
            c.g[A1][0] = c.g[S6][0];
            Addr(frame.at(c, BUDDY_PROCESS_MSG)).sw(c, c.g[V0][0]);
            let f = r32(c, V1);
            Addr(frame.at(c, BUDDY_PROCESS_MSG_CODE)).sw(c, c.g[S4][0]);
            call_ptr(c, f, 0x003f_19f4)?;
            buddy_field(c, S0, 0).sw(c, 0);
        }
        // 0x3f19f8
        c.cycles += 4;
        c.g[S0][0] = sx(r32(c, S0).wrapping_add(BUDDY_OP_SIZE));
        c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S3][0] as i64)) as u64;
        let more = c.g[V0][0] != 0;
        c.g[S1][0] = sx(r32(c, S1).wrapping_add(BUDDY_OP_SIZE));
        if !more {
            break;
        }
        buddy_process_back_edge(c, BUDDY_PROCESS_CANCEL)?;
    }
    // 0x3f1a08
    c.cycles += 2;
    c.g[A0][0] = buddy_process_field(c, BUDDY_ARIES).lw(c);
    Ok(())
}

/// 0x3f218c..0x3f221c: peek and handle packets until none is left (`resume`:
/// re-entered inside a message), report a re-sorted roster, return.
pub(crate) fn buddy_process_receive(c: &mut Cpu, frame: Frame, mut resume: Option<u32>) -> Flow {
    loop {
        if let Some(at) = resume {
            buddy_process_message(c, frame, at)?;
        }
        // 0x3f218c: a0 = the link.
        c.cycles += 6;
        c.g[A3][0] = c.g[FP][0];
        c.g[A1][0] = Addr(frame.at(c, BUDDY_PROCESS_KIND_PTR)).lw(c);
        c.g[A2][0] = Addr(frame.at(c, BUDDY_PROCESS_CODE_PTR)).lw(c);
        callee::PROTO_ARIES_PEEK.call(c, 0x003f_219c)?;
        if (c.g[V0][0] as i64) < 0 {
            break;
        }
        Addr(frame.at(c, BUDDY_PROCESS_ROSTER_DONE)).sw(c, 0);
        buddy_process_back_edge(c, BUDDY_PROCESS_MESSAGE)?;
        resume = Some(BUDDY_PROCESS_MESSAGE);
    }
    // 0x3f21a4: no packet left.
    c.cycles += 3;
    c.g[V0][0] = buddy_process_field(c, BUDDY_ROSTER_FN).lw(c);
    let no_roster_fn = c.g[V0][0] == 0;
    frame.restore(c, S0, 0x190);
    if !no_roster_fn {
        // 0x3f21b0
        c.cycles += 4;
        c.g[A0][0] = buddy_process_field(c, BUDDY_ROSTER_LIST).lw(c);
        callee::LOBBY_LIST_SORT.call(c, 0x003f_21b8)?;
        let unchanged = (c.g[V0][0] as i64) <= 0;
        c.g[V0][0] = buddy_lui(BUDDY_TYPE_ROST);
        if !unchanged {
            // 0x3f21c0: report the roster.
            c.cycles += 11;
            c.g[V1][0] = buddy_process_field(c, BUDDY_ROSTER_LIST).lw(c);
            c.g[V0][0] = sx(BUDDY_TYPE_ROST);
            Addr(frame.at(c, BUDDY_PROCESS_MSG_KIND)).sw(c, 0);
            Addr(frame.at(c, BUDDY_PROCESS_MSG)).sw(c, c.g[V0][0]);
            c.g[A0][0] = c.g[S2][0];
            Addr(frame.at(c, BUDDY_PROCESS_MSG_DATA)).sw(c, c.g[V1][0]);
            c.g[A1][0] = c.g[S6][0];
            Addr(frame.at(c, BUDDY_PROCESS_MSG_CODE)).sw(c, 0);
            c.g[V0][0] = buddy_process_field(c, BUDDY_ROSTER_FN).lw(c);
            let f = r32(c, V0);
            c.g[A2][0] = buddy_field(c, A0, BUDDY_ROSTER_ARG).lw(c);
            call_ptr(c, f, 0x003f_21ec)?;
        }
        // 0x3f21ec
        c.cycles += 1;
        frame.restore(c, S0, 0x190);
    }
    // 0x3f21f0
    c.cycles += 11;
    frame.restore(c, S1, 0x180);
    frame.restore(c, S2, 0x170);
    frame.restore(c, S3, 0x160);
    frame.restore(c, S4, 0x150);
    frame.restore(c, S5, 0x140);
    frame.restore(c, S6, 0x130);
    frame.restore(c, S7, 0x120);
    frame.restore(c, FP, 0x110);
    frame.restore_ra(c, 0x100);
    frame.close(c);
    Ok(())
}

/// One packet (0x3f1a10..0x3f218c), entered at `at`: the message head or a
/// loop head inside the `PGET` or `RADD` handler. Ends with the packet
/// consumed and the link in a0.
pub(crate) fn buddy_process_message(c: &mut Cpu, frame: Frame, at: u32) -> Flow {
    if at == BUDDY_PROCESS_MESSAGE {
        buddy_process_replies(c, frame)?;
        buddy_process_roster_entry(c, frame)?;
    }
    if at != BUDDY_PROCESS_ADD_SCAN {
        buddy_process_presence(c, frame, at)?;
    }
    buddy_process_roster_add(c, frame, at == BUDDY_PROCESS_ADD_SCAN)?;
    buddy_process_roster_del(c, frame)
}

/// 0x3f1a10..0x3f1c04: the debug print, link status, `PING`, `AUTH`,
/// `RECV`, `SEND`, `ADMN` and `RGET`. Leaves `lui "RO"` in v0.
pub(crate) fn buddy_process_replies(c: &mut Cpu, frame: Frame) -> Flow {
    let kind_at = frame.at(c, BUDDY_PROCESS_MSG_KIND);
    let code_at = frame.at(c, BUDDY_PROCESS_MSG_CODE);
    let data_at = frame.at(c, BUDDY_PROCESS_MSG_DATA);
    // 0x3f1a10
    c.cycles += 4;
    c.g[V1][0] = Addr(frame.at(c, BUDDY_PROCESS_PEEK_DATA)).lw(c);
    c.g[V0][0] = buddy_process_field(c, BUDDY_DEBUG_FN).lw(c);
    let quiet = c.g[V0][0] == 0;
    Addr(data_at).sw(c, c.g[V1][0]);
    if !quiet {
        // 0x3f1a20
        c.cycles += 5;
        c.g[A1][0] = sx(DEBUG_RECV_PREFIX);
        c.g[A0][0] = c.g[S2][0];
        c.g[A2][0] = c.g[S6][0];
        callee::BUDDY_API_DEBUG_PACKET.call(c, 0x003f_1a34)?;
    }
    // 0x3f1a34: link status (kind -1).
    c.cycles += 4;
    c.g[S0][0] = Addr(kind_at).lw(c);
    c.g[V0][0] = BUDDY_KIND_LINK;
    let link = c.g[S0][0] == c.g[V0][0];
    c.g[V0][0] = buddy_lui(BUDDY_KIND_PING);
    if link {
        // 0x3f1a44
        c.cycles += 3;
        c.g[V0][0] = Addr(code_at).lw(c);
        let up = c.g[V0][0] == c.g[S0][0];
        c.g[V0][0] = Addr(kind_at).lw(c);
        if up {
            // 0x3f1a50
            c.cycles += 4;
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = BUDDY_STATE_LINK_UP;
            callee::BUDDY_API_SET_STATE.call(c, 0x003f_1a5c)?;
            c.g[V0][0] = Addr(kind_at).lw(c);
        }
        // 0x3f1a60
        c.cycles += 2;
        let still_link = c.g[V0][0] == c.g[S0][0];
        c.g[V0][0] = buddy_lui(BUDDY_KIND_PING);
        if still_link {
            // 0x3f1a68
            c.cycles += 5;
            c.g[V0][0] = sx(BUDDY_CODE_DISCONNECTED);
            c.g[V1][0] = Addr(code_at).lw(c);
            let down = c.g[V1][0] == c.g[V0][0];
            c.g[V0][0] = buddy_lui(BUDDY_KIND_PING);
            if down {
                // 0x3f1a7c
                c.cycles += 4;
                c.g[A0][0] = c.g[S2][0];
                c.g[A1][0] = BUDDY_STATE_LINK_DOWN;
                callee::BUDDY_API_SET_STATE.call(c, 0x003f_1a88)?;
                c.g[V0][0] = buddy_lui(BUDDY_KIND_PING);
            }
        }
    }
    // 0x3f1a8c: PING: answer it.
    c.cycles += 4;
    c.g[V1][0] = Addr(kind_at).lw(c);
    c.g[V0][0] = sx(BUDDY_KIND_PING);
    let ping = c.g[V1][0] == c.g[V0][0];
    c.g[V0][0] = buddy_lui(BUDDY_KIND_AUTH);
    if ping {
        // 0x3f1a9c
        c.cycles += 9;
        c.g[A1][0] = sx(BUDDY_KIND_PING);
        c.g[A0][0] = buddy_process_field(c, BUDDY_ARIES).lw(c);
        c.g[A2][0] = Addr(code_at).lw(c);
        c.g[A3][0] = Addr(data_at).lw(c);
        c.g[T0][0] = u64::MAX;
        callee::PROTO_ARIES_SEND.call(c, 0x003f_1ab8)?;
        c.g[V0][0] = buddy_lui(BUDDY_KIND_AUTH);
        c.g[V1][0] = Addr(kind_at).lw(c);
    }
    // 0x3f1ac0: AUTH: the login reply.
    c.cycles += 3;
    c.g[V0][0] = sx(BUDDY_KIND_AUTH);
    let auth = c.g[V1][0] == c.g[V0][0];
    c.g[V0][0] = buddy_lui(BUDDY_KIND_RECV);
    if auth {
        // 0x3f1acc
        c.cycles += 11;
        c.g[V0][0] = sx(BUDDY_TYPE_AUTH);
        c.g[V1][0] = buddy_process_field(c, BUDDY_STATE_FN).lw(c);
        c.g[A2][0] = buddy_process_field(c, BUDDY_STATE_ARG).lw(c);
        Addr(frame.at(c, BUDDY_PROCESS_MSG)).sw(c, c.g[V0][0]);
        c.g[A0][0] = c.g[S2][0];
        let f = r32(c, V1);
        c.g[A1][0] = c.g[S6][0];
        call_ptr(c, f, 0x003f_1aec)?;
        c.g[V0][0] = Addr(code_at).lw(c);
        let accepted = c.g[V0][0] == 0;
        buddy_process_field(c, BUDDY_LOGIN_DEADLINE).sw(c, 0);
        if !accepted {
            // 0x3f1af8
            c.cycles += 2;
            c.g[A0][0] = c.g[S2][0];
            callee::BUDDY_API_DISCONNECT.call(c, 0x003f_1b00)?;
        }
        // 0x3f1b00
        c.cycles += 1;
        c.g[V0][0] = buddy_lui(BUDDY_KIND_RECV);
    }
    // 0x3f1b04: RECV: a buddy message.
    c.cycles += 4;
    c.g[V1][0] = Addr(kind_at).lw(c);
    c.g[V0][0] = sx(BUDDY_KIND_RECV);
    let recv = c.g[V1][0] == c.g[V0][0];
    c.g[V0][0] = buddy_lui(BUDDY_KIND_SEND);
    if recv {
        // 0x3f1b14
        c.cycles += 3;
        c.g[V0][0] = buddy_process_field(c, BUDDY_MESSAGE_FN).lw(c);
        let no_fn = c.g[V0][0] == 0;
        c.g[A0][0] = c.g[S2][0];
        if !no_fn {
            // 0x3f1b20
            c.cycles += 2;
            c.g[A1][0] = c.g[S6][0];
            callee::BUDDY_API_DISPATCH_MESSAGE.call(c, 0x003f_1b28)?;
        }
        // 0x3f1b28
        c.cycles += 2;
        c.g[V0][0] = buddy_lui(BUDDY_KIND_SEND);
        c.g[V1][0] = Addr(kind_at).lw(c);
    }
    // 0x3f1b30: SEND: the send op's reply.
    c.cycles += 3;
    c.g[V0][0] = sx(BUDDY_KIND_SEND);
    let send = c.g[V1][0] == c.g[V0][0];
    c.g[V0][0] = buddy_lui(BUDDY_KIND_ADMN);
    if send {
        // 0x3f1b3c
        c.cycles += 3;
        c.g[V1][0] = buddy_process_field(c, BUDDY_OP_SLOT_SEND + BUDDY_OP_FN).lw(c);
        let no_fn = c.g[V1][0] == 0;
        c.g[V0][0] = buddy_lui(BUDDY_TYPE_SEND);
        if !no_fn {
            // 0x3f1b48
            c.cycles += 7;
            c.g[A2][0] = buddy_process_field(c, BUDDY_OP_SLOT_SEND + BUDDY_OP_ARG).lw(c);
            c.g[V0][0] = sx(BUDDY_TYPE_SEND);
            c.g[A0][0] = c.g[S2][0];
            Addr(frame.at(c, BUDDY_PROCESS_MSG)).sw(c, c.g[V0][0]);
            let f = r32(c, V1);
            c.g[A1][0] = c.g[S6][0];
            call_ptr(c, f, 0x003f_1b60)?;
            buddy_process_field(c, BUDDY_OP_SLOT_SEND + BUDDY_OP_FN).sw(c, 0);
        }
        // 0x3f1b64
        c.cycles += 1;
        c.g[V0][0] = buddy_lui(BUDDY_KIND_ADMN);
    }
    // 0x3f1b68: ADMN: an admin message.
    c.cycles += 4;
    c.g[V1][0] = Addr(kind_at).lw(c);
    c.g[V0][0] = sx(BUDDY_KIND_ADMN);
    let admn = c.g[V1][0] == c.g[V0][0];
    c.g[V0][0] = buddy_lui(BUDDY_KIND_RGET);
    if admn {
        // 0x3f1b78
        c.cycles += 3;
        c.g[V1][0] = buddy_process_field(c, BUDDY_MESSAGE_FN).lw(c);
        let no_fn = c.g[V1][0] == 0;
        c.g[V0][0] = buddy_lui(BUDDY_TYPE_ADMN);
        if !no_fn {
            // 0x3f1b84
            c.cycles += 6;
            c.g[A2][0] = buddy_process_field(c, BUDDY_MESSAGE_ARG).lw(c);
            c.g[V0][0] = sx(BUDDY_TYPE_ADMN);
            c.g[A0][0] = c.g[S2][0];
            Addr(frame.at(c, BUDDY_PROCESS_MSG)).sw(c, c.g[V0][0]);
            let f = r32(c, V1);
            c.g[A1][0] = c.g[S6][0];
            call_ptr(c, f, 0x003f_1b9c)?;
        }
        // 0x3f1b9c
        c.cycles += 2;
        c.g[V0][0] = buddy_lui(BUDDY_KIND_RGET);
        c.g[V1][0] = Addr(kind_at).lw(c);
    }
    // 0x3f1ba4: RGET: the roster size (list 2 only).
    c.cycles += 3;
    c.g[V0][0] = sx(BUDDY_KIND_RGET);
    let rget = c.g[V1][0] == c.g[V0][0];
    c.g[V0][0] = buddy_lui(BUDDY_KIND_ROST);
    if rget {
        // 0x3f1bb0
        c.cycles += 10;
        c.g[A0][0] = Addr(data_at).lw(c);
        c.g[T1][0] = STRINGS_PAGE;
        c.g[A1][0] = sx(BUDDY_TAG_ID);
        callee::TAG_FIELD_FIND.call(c, 0x003f_1bc0)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        callee::TAG_FIELD_GET_NUMBER.call(c, 0x003f_1bcc)?;
        c.g[V1][0] = ROSTER_LIST2;
        let list2 = c.g[V0][0] == c.g[V1][0];
        c.g[V0][0] = buddy_lui(BUDDY_KIND_ROST);
        if list2 {
            // 0x3f1bd8
            c.cycles += 11;
            c.g[A0][0] = Addr(data_at).lw(c);
            c.g[A1][0] = sx(BUDDY_TAG_SIZE);
            callee::TAG_FIELD_FIND.call(c, 0x003f_1be8)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            callee::TAG_FIELD_GET_NUMBER.call(c, 0x003f_1bf4)?;
            buddy_process_field(c, BUDDY_ROSTER_LEFT).sw(c, c.g[V0][0]);
            c.g[V0][0] = (c.g[V0][0] < 1) as u64;
            Addr(frame.at(c, BUDDY_PROCESS_ROSTER_DONE)).sw(c, c.g[V0][0]);
            c.g[V0][0] = buddy_lui(BUDDY_KIND_ROST);
        }
    }
    Ok(())
}

/// 0x3f1c04..0x3f1da0: `ROST`: one roster entry: find or add the user, copy
/// its group, set the list flag; the last entry of list 2 sets "roster
/// complete". Leaves the kind in v1 and `lui "PG"` in v0.
pub(crate) fn buddy_process_roster_entry(c: &mut Cpu, frame: Frame) -> Flow {
    let data_at = frame.at(c, BUDDY_PROCESS_MSG_DATA);
    // 0x3f1c04
    c.cycles += 4;
    c.g[V1][0] = Addr(frame.at(c, BUDDY_PROCESS_MSG_KIND)).lw(c);
    c.g[V0][0] = sx(BUDDY_KIND_ROST);
    let rost = c.g[V1][0] == c.g[V0][0];
    c.g[V0][0] = buddy_lui(BUDDY_KIND_PGET);
    if !rost {
        return Ok(());
    }
    'reload: {
        'tail: {
            // 0x3f1c14
            c.cycles += 3;
            c.g[V0][0] = buddy_process_field(c, BUDDY_ROSTER_LIST).lw(c);
            let no_roster = c.g[V0][0] == 0;
            c.g[A0][0] = Addr(data_at).lw(c);
            if no_roster {
                break 'tail;
            }
            // 0x3f1c20: the list id picks the flag (s3).
            c.cycles += 15;
            c.g[T2][0] = STRINGS_PAGE;
            c.g[A1][0] = sx(BUDDY_TAG_ID);
            c.g[S3][0] = ROSTER_FLAG_LIST_B;
            callee::TAG_FIELD_FIND.call(c, 0x003f_1c30)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            callee::TAG_FIELD_GET_NUMBER.call(c, 0x003f_1c3c)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[V1][0] = ROSTER_FLAG_LIST_I;
            c.g[V0][0] = c.g[S0][0] ^ ROSTER_LIST1;
            c.g[A0][0] = c.g[S0][0] ^ ROSTER_LIST2;
            if c.g[V0][0] != 0 {
                c.g[S3][0] = 0;
            }
            if c.g[A0][0] == 0 {
                c.g[S3][0] = c.g[V1][0];
            }
            let list2 = c.g[S3][0] == c.g[V1][0];
            c.g[A1][0] = 1;
            if list2 {
                // 0x3f1c5c: count the entry down; the last one completes it.
                c.cycles += 6;
                c.g[V0][0] = buddy_process_field(c, BUDDY_ROSTER_LEFT).lw(c);
                c.g[V0][0] = sx(r32(c, V0).wrapping_sub(1));
                buddy_process_field(c, BUDDY_ROSTER_LEFT).sw(c, c.g[V0][0]);
                c.g[V1][0] = Addr(frame.at(c, BUDDY_PROCESS_ROSTER_DONE)).lw(c);
                if c.g[V0][0] == 0 {
                    c.g[V1][0] = c.g[A1][0];
                }
                Addr(frame.at(c, BUDDY_PROCESS_ROSTER_DONE)).sw(c, c.g[V1][0]);
            }
            // 0x3f1c74: the user name.
            c.cycles += 15;
            c.g[A0][0] = Addr(data_at).lw(c);
            c.g[A2][0] = STRINGS_PAGE;
            c.g[A1][0] = sx(BUDDY_TAG_USER);
            c.g[S4][0] = STRINGS_PAGE;
            callee::TAG_FIELD_FIND.call(c, 0x003f_1c88)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = c.g[SP][0];
            c.g[A2][0] = ROSTER_NAME_SIZE;
            c.g[A3][0] = sx(BUDDY_STR_EMPTY);
            callee::TAG_FIELD_GET_STRING.call(c, 0x003f_1c9c)?;
            c.g[A0][0] = c.g[SP][0];
            callee::BUDDY_API_STRIP_USER_NAME.call(c, 0x003f_1ca4)?;
            c.g[V0][0] = buddy_process_field(c, BUDDY_ACCEPT_DOTTED).lw(c);
            if c.g[V0][0] == 0 {
                // 0x3f1cb0: skip names with a dot.
                c.cycles += 5;
                c.g[A0][0] = c.g[SP][0];
                c.g[A1][0] = b'.' as u64;
                callee::STRCHR.call(c, 0x003f_1cbc)?;
                let dotted = c.g[V0][0] != 0;
                c.g[V0][0] = buddy_lui(BUDDY_KIND_PGET);
                if dotted {
                    break 'reload;
                }
                // 0x3f1cc4
                c.cycles += 1;
            }
            c.g[A0][0] = buddy_process_field(c, BUDDY_ROSTER_HASH).lw(c);
            // 0x3f1cc8: the user's list node (s0), added when new.
            c.cycles += 5;
            c.g[A1][0] = c.g[SP][0];
            callee::LOBBY_HASH_FIND_STR.call(c, 0x003f_1cd0)?;
            c.g[S0][0] = c.g[V0][0];
            if c.g[S0][0] == 0 {
                // 0x3f1cdc
                c.cycles += 5;
                c.g[A0][0] = c.g[SP][0];
                callee::STRLEN.call(c, 0x003f_1ce4)?;
                c.g[V0][0] = (c.g[V0][0] < ROSTER_NAME_SIZE) as u64;
                if c.g[V0][0] != 0 {
                    // 0x3f1cf0: a new entry, named, in the list and the hash.
                    c.cycles += 18;
                    c.g[A0][0] = ROSTER_ENTRY_SIZE;
                    callee::LOBBY_MEM_ALLOC.call(c, 0x003f_1cf8)?;
                    c.g[S1][0] = c.g[V0][0];
                    c.g[A2][0] = ROSTER_ENTRY_SIZE;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[A1][0] = 0;
                    callee::MEMSET.call(c, 0x003f_1d0c)?;
                    c.g[A1][0] = c.g[SP][0];
                    c.g[A0][0] = c.g[S1][0];
                    callee::STRCPY.call(c, 0x003f_1d18)?;
                    c.g[A0][0] = buddy_process_field(c, BUDDY_ROSTER_LIST).lw(c);
                    c.g[A1][0] = c.g[S1][0];
                    callee::LOBBY_LIST_ADD.call(c, 0x003f_1d24)?;
                    c.g[S0][0] = c.g[V0][0];
                    c.g[A0][0] = buddy_process_field(c, BUDDY_ROSTER_HASH).lw(c);
                    c.g[A1][0] = c.g[S1][0];
                    c.g[A2][0] = c.g[S0][0];
                    callee::LOBBY_HASH_ADD_STR.call(c, 0x003f_1d38)?;
                }
                // 0x3f1d38
                c.cycles += 2;
                let no_node = c.g[S0][0] == 0;
                c.g[V0][0] = buddy_lui(BUDDY_KIND_PGET);
                if no_node {
                    break 'reload;
                }
                // 0x3f1d40
                c.cycles += 1;
            }
            c.g[A0][0] = buddy_process_field(c, BUDDY_ROSTER_LIST).lw(c);
            // 0x3f1d44: the entry (s1).
            c.cycles += 5;
            c.g[A1][0] = c.g[S0][0];
            callee::LOBBY_LIST_GET_DATA.call(c, 0x003f_1d4c)?;
            c.g[S1][0] = c.g[V0][0];
            let no_entry = c.g[S1][0] == 0;
            c.g[A0][0] = Addr(data_at).lw(c);
            if no_entry {
                break 'tail;
            }
            // 0x3f1d58: its group.
            c.cycles += 10;
            c.g[A1][0] = sx(BUDDY_TAG_GROUP);
            callee::TAG_FIELD_FIND.call(c, 0x003f_1d64)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A3][0] = sx(r32(c, S4).wrapping_add(BUDDY_STR_EMPTY & 0xffff));
            c.g[A1][0] = sx(r32(c, S1).wrapping_add(ROSTER_ENTRY_GROUP));
            c.g[A2][0] = ROSTER_NAME_SIZE;
            callee::TAG_FIELD_GET_STRING.call(c, 0x003f_1d78)?;
            let no_flag = c.g[S3][0] == 0;
            c.g[V0][0] = buddy_lui(BUDDY_KIND_PGET);
            if no_flag {
                break 'reload;
            }
            // 0x3f1d80: the list flag.
            c.cycles += 6;
            c.g[V1][0] = buddy_field(c, S1, ROSTER_ENTRY_FLAGS).lw(c);
            c.g[V0][0] = ROSTER_FLAG_ROST_CLEARS;
            c.g[V1][0] |= c.g[S3][0];
            c.g[V1][0] &= c.g[V0][0];
            buddy_field(c, S1, ROSTER_ENTRY_FLAGS).sw(c, c.g[V1][0]);
        }
        // 0x3f1d98
        c.cycles += 1;
        c.g[V0][0] = buddy_lui(BUDDY_KIND_PGET);
    }
    // 0x3f1d9c
    c.cycles += 1;
    c.g[V1][0] = Addr(frame.at(c, BUDDY_PROCESS_MSG_KIND)).lw(c);
    Ok(())
}

/// 0x3f1da0..0x3f1f34: `PGET`: a buddy's presence: copy the entry (to the
/// frame, 32 bytes at a time with unaligned or aligned doublewords, then 24),
/// parse the presence into it, and on a change mark the roster dirty and
/// print it. `at`: the message head, or re-entered at a copy loop head.
/// Leaves `lui "RA"` in v0.
pub(crate) fn buddy_process_presence(c: &mut Cpu, frame: Frame, at: u32) -> Flow {
    let aligned = if at == BUDDY_PROCESS_MESSAGE {
        // 0x3f1da0
        c.cycles += 3;
        c.g[V0][0] = sx(BUDDY_KIND_PGET);
        let pget = c.g[V1][0] == c.g[V0][0];
        c.g[V0][0] = buddy_lui(BUDDY_KIND_RADD);
        if !pget {
            return Ok(());
        }
        // 0x3f1dac
        c.cycles += 3;
        c.g[V0][0] = buddy_process_field(c, BUDDY_ROSTER_LIST).lw(c);
        let no_roster = c.g[V0][0] == 0;
        c.g[A0][0] = Addr(frame.at(c, BUDDY_PROCESS_MSG_DATA)).lw(c);
        if no_roster {
            // 0x3f1f30
            c.cycles += 1;
            c.g[V0][0] = buddy_lui(BUDDY_KIND_RADD);
            return Ok(());
        }
        // 0x3f1db8: the user's entry.
        c.cycles += 17;
        c.g[T0][0] = STRINGS_PAGE;
        c.g[A1][0] = sx(BUDDY_TAG_USER);
        callee::TAG_FIELD_FIND.call(c, 0x003f_1dc4)?;
        c.g[T1][0] = STRINGS_PAGE;
        c.g[A1][0] = c.g[SP][0];
        c.g[A2][0] = ROSTER_NAME_SIZE;
        c.g[A3][0] = sx(BUDDY_STR_EMPTY);
        c.g[A0][0] = c.g[V0][0];
        callee::TAG_FIELD_GET_STRING.call(c, 0x003f_1ddc)?;
        c.g[A0][0] = c.g[SP][0];
        callee::BUDDY_API_STRIP_USER_NAME.call(c, 0x003f_1de4)?;
        c.g[A0][0] = buddy_process_field(c, BUDDY_ROSTER_HASH).lw(c);
        c.g[A1][0] = c.g[SP][0];
        callee::LOBBY_HASH_FIND_STR.call(c, 0x003f_1df0)?;
        c.g[S0][0] = c.g[V0][0];
        let unknown = c.g[S0][0] == 0;
        c.g[V0][0] = buddy_lui(BUDDY_KIND_RADD);
        if unknown {
            return Ok(());
        }
        // 0x3f1dfc: copy it (from v1 to a3 = s0, up to v0).
        c.cycles += 9;
        c.g[A0][0] = buddy_process_field(c, BUDDY_ROSTER_LIST).lw(c);
        c.g[A1][0] = c.g[S0][0];
        callee::LOBBY_LIST_GET_DATA.call(c, 0x003f_1e08)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[A3][0] = sx(frame.at(c, BUDDY_PROCESS_ENTRY_COPY));
        c.g[S0][0] = c.g[A3][0];
        c.g[V0][0] = c.g[S1][0] & 0x7;
        let aligned = c.g[V0][0] == 0;
        c.g[V1][0] = c.g[S1][0];
        // 0x3f1e20 / 0x3f1e78
        c.cycles += 1;
        c.g[V0][0] = sx(r32(c, S1).wrapping_add(ROSTER_ENTRY_COPY_END));
        aligned
    } else {
        at == BUDDY_PROCESS_COPY_ALIGNED
    };
    if aligned {
        loop {
            // 0x3f1e7c
            c.cycles += 11;
            let (src, dst) = (r32(c, V1), r32(c, A3));
            c.g[T0][0] = c.ld(src);
            c.g[T1][0] = c.ld(src.wrapping_add(0x8));
            c.g[T2][0] = c.ld(src.wrapping_add(0x10));
            c.g[A0][0] = c.ld(src.wrapping_add(0x18));
            c.sd(dst, c.g[T0][0]);
            c.sd(dst.wrapping_add(0x8), c.g[T1][0]);
            c.sd(dst.wrapping_add(0x10), c.g[T2][0]);
            c.sd(dst.wrapping_add(0x18), c.g[A0][0]);
            c.g[V1][0] = sx(src.wrapping_add(0x20));
            let more = c.g[V1][0] != c.g[V0][0];
            c.g[A3][0] = sx(dst.wrapping_add(0x20));
            if !more {
                break;
            }
            buddy_process_back_edge(c, BUDDY_PROCESS_COPY_ALIGNED)?;
        }
    } else {
        loop {
            // 0x3f1e24
            c.cycles += 19;
            let (src, dst) = (r32(c, V1), r32(c, A3));
            let regs = [(T2, 0x0), (A0, 0x8), (A1, 0x10), (A2, 0x18)];
            for (reg, off) in regs {
                c.ldl(reg, src.wrapping_add(off + 7));
                c.ldr(reg, src.wrapping_add(off));
            }
            for (reg, off) in regs {
                c.sdl(reg, dst.wrapping_add(off + 7));
                c.sdr(reg, dst.wrapping_add(off));
            }
            c.g[V1][0] = sx(src.wrapping_add(0x20));
            let more = c.g[V1][0] != c.g[V0][0];
            c.g[A3][0] = sx(dst.wrapping_add(0x20));
            if !more {
                break;
            }
            buddy_process_back_edge(c, BUDDY_PROCESS_COPY_UNALIGNED)?;
        }
        // 0x3f1e70
        c.cycles += 2;
    }
    // 0x3f1ea8: the last 24 bytes, then parse.
    c.cycles += 22;
    let (src, dst) = (r32(c, V1), r32(c, A3));
    let regs = [(T0, 0x0), (T1, 0x8), (T2, 0x10)];
    for (reg, off) in regs {
        c.ldl(reg, src.wrapping_add(off + 7));
        c.ldr(reg, src.wrapping_add(off));
    }
    for (reg, off) in regs {
        c.sdl(reg, dst.wrapping_add(off + 7));
        c.sdr(reg, dst.wrapping_add(off));
    }
    c.g[A0][0] = c.g[S2][0];
    c.g[A1][0] = c.g[S6][0];
    c.g[A2][0] = c.g[S1][0];
    callee::BUDDY_API_PARSE_PRESENCE.call(c, 0x003f_1ee8)?;
    c.g[A0][0] = c.g[S0][0];
    c.g[A1][0] = c.g[S1][0];
    c.g[A2][0] = ROSTER_ENTRY_SIZE;
    callee::MEMCMP.call(c, 0x003f_1ef8)?;
    let same = c.g[V0][0] == 0;
    c.g[V0][0] = buddy_lui(BUDDY_KIND_RADD);
    if same {
        return Ok(());
    }
    // 0x3f1f00: changed: mark the roster and print the presence.
    c.cycles += 12;
    c.g[A0][0] = buddy_process_field(c, BUDDY_ROSTER_LIST).lw(c);
    c.g[A1][0] = 1;
    callee::LOBBY_LIST_MARK_DIRTY.call(c, 0x003f_1f0c)?;
    c.g[A2][0] = c.g[S1][0];
    c.g[V1][0] = FMT_PRESENCE_PAGE;
    c.g[A3][0] = buddy_field(c, A2, ROSTER_ENTRY_SHOW).lb(c);
    c.g[A0][0] = c.g[S2][0];
    c.g[A1][0] = sx(FMT_PRESENCE);
    c.g[T0][0] = sx(r32(c, A2).wrapping_add(ROSTER_ENTRY_PRODUCT));
    c.g[T1][0] = sx(r32(c, A2).wrapping_add(ROSTER_ENTRY_PRESENCE));
    c.g[T2][0] = sx(r32(c, A2).wrapping_add(ROSTER_ENTRY_LANG));
    callee::BUDDY_API_DEBUG_PRINTF.call(c, 0x003f_1f30)?;
    // 0x3f1f30
    c.cycles += 1;
    c.g[V0][0] = buddy_lui(BUDDY_KIND_RADD);
    Ok(())
}

/// 0x3f1f34..0x3f20d4: `RADD`: the add reply for the entry waiting with
/// this id (index s3, id s0, flag s7): refused (code != 0): call the add op
/// back with code -2 and remove the entry; accepted: rename it to `FUSR`
/// when given, call the add op back, clear the wait. `resume`: re-entered at
/// the scan head (the entry's flags in v0). Leaves the kind in v1 and
/// `lui "RD"` in v0.
pub(crate) fn buddy_process_roster_add(c: &mut Cpu, frame: Frame, resume: bool) -> Flow {
    let code_at = frame.at(c, BUDDY_PROCESS_MSG_CODE);
    let data_at = frame.at(c, BUDDY_PROCESS_MSG_DATA);
    'reload: {
        if !resume {
            // 0x3f1f34
            c.cycles += 4;
            c.g[V1][0] = Addr(frame.at(c, BUDDY_PROCESS_MSG_KIND)).lw(c);
            c.g[V0][0] = sx(BUDDY_KIND_RADD);
            let radd = c.g[V1][0] == c.g[V0][0];
            c.g[V0][0] = buddy_lui(BUDDY_KIND_RDEL);
            if !radd {
                return Ok(());
            }
            // 0x3f1f44
            c.cycles += 3;
            c.g[V0][0] = buddy_process_field(c, BUDDY_ROSTER_LIST).lw(c);
            let no_roster = c.g[V0][0] == 0;
            c.g[V0][0] = buddy_lui(BUDDY_TYPE_ADDU);
            if no_roster {
                break 'reload;
            }
            // 0x3f1f50: the request id.
            c.cycles += 15;
            c.g[A0][0] = Addr(data_at).lw(c);
            c.g[A2][0] = STRINGS_PAGE;
            c.g[V0][0] = sx(BUDDY_TYPE_ADDU);
            c.g[A1][0] = sx(BUDDY_TAG_ID);
            Addr(frame.at(c, BUDDY_PROCESS_MSG)).sw(c, c.g[V0][0]);
            callee::TAG_FIELD_FIND.call(c, 0x003f_1f68)?;
            c.g[S3][0] = 0;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            callee::TAG_FIELD_GET_NUMBER.call(c, 0x003f_1f78)?;
            c.g[S7][0] = ROSTER_FLAG_ADDING;
            c.g[S4][0] = STRINGS_PAGE;
            c.g[S0][0] = c.g[V0][0];
            c.g[S5][0] = STRINGS_PAGE;
        }
        'mark: {
            'done: {
                let mut at_head = resume;
                loop {
                    if !at_head {
                        // 0x3f20a8: the next entry (s1).
                        c.cycles += 6;
                        c.g[A0][0] = buddy_process_field(c, BUDDY_ROSTER_LIST).lw(c);
                        c.g[A1][0] = c.g[S3][0];
                        callee::LOBBY_LIST_GET_AT.call(c, 0x003f_20b4)?;
                        c.g[S1][0] = c.g[V0][0];
                        if c.g[S1][0] == 0 {
                            break 'done;
                        }
                        c.g[V0][0] = buddy_field(c, S1, ROSTER_ENTRY_FLAGS).lw(c);
                        buddy_process_back_edge(c, BUDDY_PROCESS_ADD_SCAN)?;
                    }
                    at_head = false;
                    // 0x3f1f90
                    c.cycles += 3;
                    c.g[V0][0] &= c.g[S7][0];
                    if c.g[V0][0] == 0 {
                        c.g[S3][0] = sx(r32(c, S3).wrapping_add(1));
                        continue;
                    }
                    // 0x3f1f9c
                    c.cycles += 3;
                    c.g[V0][0] = buddy_field(c, S1, ROSTER_ENTRY_OP_ID).lw(c);
                    if c.g[V0][0] != c.g[S0][0] {
                        c.g[S3][0] = sx(r32(c, S3).wrapping_add(1));
                        continue;
                    }
                    // 0x3f1fa8: this entry's reply.
                    c.cycles += 3;
                    c.g[V0][0] = Addr(code_at).lw(c);
                    let accepted = c.g[V0][0] == 0;
                    c.g[A0][0] = Addr(data_at).lw(c);
                    if !accepted {
                        buddy_process_add_refused(c, frame)?;
                        break 'mark;
                    }
                    buddy_process_add_accepted(c, frame)?;
                    break 'done;
                }
            }
            // 0x3f20c0
            c.cycles += 1;
            c.g[A0][0] = buddy_process_field(c, BUDDY_ROSTER_LIST).lw(c);
        }
        // 0x3f20c4
        c.cycles += 2;
        c.g[A1][0] = 1;
        callee::LOBBY_LIST_MARK_DIRTY.call(c, 0x003f_20cc)?;
    }
    // 0x3f20cc
    c.cycles += 2;
    c.g[V0][0] = buddy_lui(BUDDY_KIND_RDEL);
    c.g[V1][0] = Addr(frame.at(c, BUDDY_PROCESS_MSG_KIND)).lw(c);
    Ok(())
}

/// 0x3f1fb4..0x3f2000: the add was refused: call the add op back (code -2,
/// data = the entry), then unhash, unlist and free the entry. Leaves the
/// roster in a0.
pub(crate) fn buddy_process_add_refused(c: &mut Cpu, frame: Frame) -> Flow {
    // 0x3f1fb4
    c.cycles += 3;
    c.g[V1][0] = buddy_process_field(c, BUDDY_OP_SLOT_ADD + BUDDY_OP_FN).lw(c);
    let no_fn = c.g[V1][0] == 0;
    c.g[V0][0] = ADD_REFUSED;
    if !no_fn {
        // 0x3f1fc0
        c.cycles += 6;
        c.g[A2][0] = buddy_process_field(c, BUDDY_OP_SLOT_ADD + BUDDY_OP_ARG).lw(c);
        Addr(frame.at(c, BUDDY_PROCESS_MSG_CODE)).sw(c, c.g[V0][0]);
        c.g[A0][0] = c.g[S2][0];
        Addr(frame.at(c, BUDDY_PROCESS_MSG_DATA)).sw(c, c.g[S1][0]);
        let f = r32(c, V1);
        c.g[A1][0] = c.g[S6][0];
        call_ptr(c, f, 0x003f_1fd8)?;
    }
    // 0x3f1fd8
    c.cycles += 10;
    c.g[A0][0] = buddy_process_field(c, BUDDY_ROSTER_HASH).lw(c);
    c.g[A1][0] = c.g[S1][0];
    callee::LOBBY_HASH_DEL_STR.call(c, 0x003f_1fe4)?;
    c.g[A0][0] = buddy_process_field(c, BUDDY_ROSTER_LIST).lw(c);
    c.g[A1][0] = c.g[V0][0];
    callee::LOBBY_LIST_REMOVE.call(c, 0x003f_1ff0)?;
    c.g[A0][0] = c.g[S1][0];
    callee::LOBBY_MEM_FREE.call(c, 0x003f_1ff8)?;
    c.g[A0][0] = buddy_process_field(c, BUDDY_ROSTER_LIST).lw(c);
    Ok(())
}

/// 0x3f2000..0x3f20a8: the add was accepted: rename the entry to the `FUSR`
/// name when one fits (re-hashed under it), call the add op back (code 0,
/// data = the entry), clear the entry's wait.
pub(crate) fn buddy_process_add_accepted(c: &mut Cpu, frame: Frame) -> Flow {
    // 0x3f2000: a0 = the reply's tags.
    c.cycles += 12;
    c.g[A1][0] = sx(r32(c, S4).wrapping_add(BUDDY_TAG_FUSR & 0xffff));
    callee::TAG_FIELD_FIND.call(c, 0x003f_2008)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A3][0] = sx(r32(c, S5).wrapping_add(BUDDY_STR_EMPTY & 0xffff));
    c.g[A1][0] = c.g[SP][0];
    c.g[A2][0] = ROSTER_NAME_SIZE;
    callee::TAG_FIELD_GET_STRING.call(c, 0x003f_201c)?;
    c.g[A0][0] = c.g[SP][0];
    callee::BUDDY_API_STRIP_USER_NAME.call(c, 0x003f_2024)?;
    c.g[V0][0] = Addr(sp(c)).lb(c);
    // v0 = the add op's fn on every path to 0x3f2078.
    'notify: {
        if c.g[V0][0] == 0 {
            c.g[V0][0] = buddy_process_field(c, BUDDY_OP_SLOT_ADD + BUDDY_OP_FN).lw(c);
            break 'notify;
        }
        // 0x3f2030
        c.cycles += 5;
        c.g[A0][0] = c.g[SP][0];
        callee::STRLEN.call(c, 0x003f_2038)?;
        c.g[V0][0] = (c.g[V0][0] < ROSTER_NAME_SIZE) as u64;
        if c.g[V0][0] == 0 {
            c.g[V0][0] = buddy_process_field(c, BUDDY_OP_SLOT_ADD + BUDDY_OP_FN).lw(c);
            break 'notify;
        }
        // 0x3f2044: rename (the hash keys on the name).
        c.cycles += 13;
        c.g[A0][0] = buddy_process_field(c, BUDDY_ROSTER_HASH).lw(c);
        c.g[A1][0] = c.g[S1][0];
        callee::LOBBY_HASH_DEL_STR.call(c, 0x003f_2050)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = ROSTER_NAME_SIZE;
        c.g[A2][0] = c.g[SP][0];
        callee::TAG_FIELD_STRNZCPY.call(c, 0x003f_2064)?;
        c.g[A0][0] = buddy_process_field(c, BUDDY_ROSTER_HASH).lw(c);
        c.g[A2][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S1][0];
        callee::LOBBY_HASH_ADD_STR.call(c, 0x003f_2074)?;
        c.g[V0][0] = buddy_process_field(c, BUDDY_OP_SLOT_ADD + BUDDY_OP_FN).lw(c);
    }
    // 0x3f2078
    c.cycles += 2;
    let no_fn = c.g[V0][0] == 0;
    c.g[A0][0] = c.g[S2][0];
    if !no_fn {
        // 0x3f2080
        c.cycles += 5;
        c.g[A2][0] = buddy_process_field(c, BUDDY_OP_SLOT_ADD + BUDDY_OP_ARG).lw(c);
        Addr(frame.at(c, BUDDY_PROCESS_MSG_CODE)).sw(c, 0);
        c.g[A1][0] = c.g[S6][0];
        let f = r32(c, V0);
        Addr(frame.at(c, BUDDY_PROCESS_MSG_DATA)).sw(c, c.g[S1][0]);
        call_ptr(c, f, 0x003f_2094)?;
    }
    // 0x3f2094
    c.cycles += 5;
    c.g[V0][0] = buddy_field(c, S1, ROSTER_ENTRY_FLAGS).lw(c);
    buddy_field(c, S1, ROSTER_ENTRY_OP_ID).sw(c, 0);
    c.g[V0][0] ^= c.g[S7][0];
    buddy_field(c, S1, ROSTER_ENTRY_FLAGS).sw(c, c.g[V0][0]);
    Ok(())
}

/// 0x3f20d4..0x3f218c: `RDEL`: the delete reply for the delete op's id
/// (`delu`), the roster-complete report (`endr`), then consume the packet.
/// Leaves the link in a0.
pub(crate) fn buddy_process_roster_del(c: &mut Cpu, frame: Frame) -> Flow {
    let done_at = frame.at(c, BUDDY_PROCESS_ROSTER_DONE);
    // 0x3f20d4
    c.cycles += 3;
    c.g[V0][0] = sx(BUDDY_KIND_RDEL);
    let rdel = c.g[V1][0] == c.g[V0][0];
    c.g[T1][0] = Addr(done_at).lw(c);
    'done: {
        if !rdel {
            break 'done;
        }
        // 0x3f20e0
        c.cycles += 3;
        c.g[V0][0] = buddy_process_field(c, BUDDY_OP_SLOT_DEL + BUDDY_OP_FN).lw(c);
        let no_fn = c.g[V0][0] == 0;
        c.g[A0][0] = Addr(frame.at(c, BUDDY_PROCESS_MSG_DATA)).lw(c);
        if no_fn {
            break 'done;
        }
        // 0x3f20ec
        c.cycles += 9;
        c.g[T0][0] = STRINGS_PAGE;
        c.g[A1][0] = sx(BUDDY_TAG_ID);
        callee::TAG_FIELD_FIND.call(c, 0x003f_20f8)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        callee::TAG_FIELD_GET_NUMBER.call(c, 0x003f_2104)?;
        c.g[V1][0] = buddy_process_field(c, BUDDY_OP_SLOT_DEL + BUDDY_OP_ID).lw(c);
        let other = c.g[V0][0] != c.g[V1][0];
        c.g[T1][0] = Addr(done_at).lw(c);
        if other {
            break 'done;
        }
        // 0x3f2110: call the delete op back and clear it.
        c.cycles += 10;
        c.g[V0][0] = sx(BUDDY_TYPE_DELU);
        c.g[V1][0] = buddy_process_field(c, BUDDY_OP_SLOT_DEL + BUDDY_OP_FN).lw(c);
        c.g[A2][0] = buddy_process_field(c, BUDDY_OP_SLOT_DEL + BUDDY_OP_ARG).lw(c);
        Addr(frame.at(c, BUDDY_PROCESS_MSG)).sw(c, c.g[V0][0]);
        c.g[A0][0] = c.g[S2][0];
        let f = r32(c, V1);
        c.g[A1][0] = c.g[S6][0];
        call_ptr(c, f, 0x003f_2130)?;
        buddy_process_field(c, BUDDY_OP_SLOT_DEL + BUDDY_OP_FN).sw(c, 0);
        c.g[T1][0] = Addr(done_at).lw(c);
    }
    // 0x3f2138
    c.cycles += 2;
    if c.g[T1][0] != 0 {
        // 0x3f2140: the roster is complete.
        c.cycles += 3;
        c.g[V1][0] = buddy_process_field(c, BUDDY_ROSTER_FN).lw(c);
        let no_fn = c.g[V1][0] == 0;
        c.g[V0][0] = buddy_lui(BUDDY_TYPE_ENDR);
        if !no_fn {
            // 0x3f214c
            c.cycles += 9;
            c.g[A2][0] = buddy_process_field(c, BUDDY_ROSTER_ARG).lw(c);
            c.g[V0][0] = sx(BUDDY_TYPE_ENDR);
            Addr(frame.at(c, BUDDY_PROCESS_MSG_KIND)).sw(c, 0);
            Addr(frame.at(c, BUDDY_PROCESS_MSG)).sw(c, c.g[V0][0]);
            c.g[A0][0] = c.g[S2][0];
            Addr(frame.at(c, BUDDY_PROCESS_MSG_CODE)).sw(c, 0);
            c.g[A1][0] = c.g[S6][0];
            let f = r32(c, V1);
            Addr(frame.at(c, BUDDY_PROCESS_MSG_DATA)).sw(c, 0);
            call_ptr(c, f, 0x003f_2170)?;
        }
        // 0x3f2170
        c.cycles += 1;
    }
    c.g[A0][0] = buddy_process_field(c, BUDDY_ARIES).lw(c);
    // 0x3f2174: consume the packet.
    c.cycles += 6;
    c.g[A1][0] = 0;
    c.g[A2][0] = 0;
    c.g[A3][0] = 0;
    c.g[T0][0] = 0;
    callee::PROTO_ARIES_RECV.call(c, 0x003f_2188)?;
    c.g[A0][0] = buddy_process_field(c, BUDDY_ARIES).lw(c);
    Ok(())
}

// ---------------------------------------------------------------------------
// DirtyRpc: the EE side of DirtySDK's IOP RPC transport.
//
// All state lives in one 0x4e40-byte block (`lui 0x52; addiu -0xf40`) that
// `DirtyRpc_Startup` clears. Its first 0x4c00 bytes are four regions the IOP
// also sees (through their uncached `0x2000_0000` aliases): the client table
// (16-byte entries), the request ring and the reply ring (0x20-byte slots, one
// pair per ring index) and the EE data buffer. Behind them come the DMA ids
// (0x40 bytes), one 16-byte waiter record per ring index, and the control
// words below.

/// The transport state block.
pub(crate) const DIRTY_RPC_STATE: u32 = 0x0051_f0c0;

/// Its size (cleared by the startup; the transport thread's stack follows).
pub(crate) const DIRTY_RPC_STATE_SIZE: u32 = 0x4e40;

/// The byte at +0: the IOP sets it when it answers the boot command.
pub(crate) const DIRTY_RPC_BOOT_REPLY: u32 = 0x0;

/// The first two words of the table the startup sends the IOP: 0x10, and
/// the limit request ids count modulo (0x7fff_ffff).
pub(crate) const DIRTY_RPC_TABLE_HEAD: u32 = 0x4d80;

pub(crate) const DIRTY_RPC_ID_LIMIT: u32 = 0x4d84;

/// The client table: start, end (EE uncached addresses) and its IOP address.
pub(crate) const DIRTY_RPC_CLIENTS: u32 = 0x4d88;

pub(crate) const DIRTY_RPC_CLIENTS_END: u32 = 0x4d8c;

pub(crate) const DIRTY_RPC_IOP_CLIENTS: u32 = 0x4d90;

/// The request ring (0x20-byte slots): start, end, IOP address.
pub(crate) const DIRTY_RPC_REQUESTS: u32 = 0x4d94;

pub(crate) const DIRTY_RPC_REQUESTS_END: u32 = 0x4d98;

pub(crate) const DIRTY_RPC_IOP_REQUESTS: u32 = 0x4d9c;

/// The reply ring (0x20-byte slots): start, end, IOP address.
pub(crate) const DIRTY_RPC_REPLIES: u32 = 0x4da0;

pub(crate) const DIRTY_RPC_REPLIES_END: u32 = 0x4da4;

pub(crate) const DIRTY_RPC_IOP_REPLIES: u32 = 0x4da8;

/// The EE data buffer: start, end, and the IOP address of its twin.
pub(crate) const DIRTY_RPC_EE_BUFFER: u32 = 0x4dac;

pub(crate) const DIRTY_RPC_EE_BUFFER_END: u32 = 0x4db0;

pub(crate) const DIRTY_RPC_IOP_BUFFER: u32 = 0x4db4;

/// The `sceSifSetDma` id per ring slot (a 0x40-byte cached area at
/// +0x4c00): start and end.
pub(crate) const DIRTY_RPC_DMA_IDS: u32 = 0x4db8;

pub(crate) const DIRTY_RPC_DMA_IDS_END: u32 = 0x4dbc;

/// The IOP heap block `sceSifAllocIopHeap` gave the transport.
pub(crate) const DIRTY_RPC_IOP_HEAP: u32 = 0x4dc0;

/// The waiter records (16 bytes per ring index, at +0x4c40).
pub(crate) const DIRTY_RPC_WAITERS: u32 = 0x4dc4;

/// A pointer to +0x4d40 the startup stores.
pub(crate) const DIRTY_RPC_WORD_4DC8: u32 = 0x4dc8;

/// The next request id (1 after the startup).
pub(crate) const DIRTY_RPC_NEXT_ID: u32 = 0x4dcc;

/// The ring: the slot the next send fills, the oldest unanswered slot (the
/// next reply to take), and the slot count.
pub(crate) const DIRTY_RPC_HEAD: u32 = 0x4dd0;

pub(crate) const DIRTY_RPC_TAIL: u32 = 0x4dd4;

pub(crate) const DIRTY_RPC_SLOTS: u32 = 0x4dd8;

/// The data buffer's read offset (just past the last taken reply's data).
pub(crate) const DIRTY_RPC_READ: u32 = 0x4de0;

/// The data buffer's size.
pub(crate) const DIRTY_RPC_BUFFER_SIZE: u32 = 0x4de4;

/// The transport thread.
pub(crate) const DIRTY_RPC_THREAD: u32 = 0x4de8;

/// Two alarm ids (negative: none): the poll alarm `DirtyRpc_Send` starts,
/// and the 0x10-tick alarm the alarm handler re-arms itself with.
pub(crate) const DIRTY_RPC_ALARM: u32 = 0x4dec;

pub(crate) const DIRTY_RPC_HANDLER_ALARM: u32 = 0x4df0;

/// The SIF interrupt handler id.
pub(crate) const DIRTY_RPC_INTC_HANDLER: u32 = 0x4df4;

/// The semaphore guarding the rings.
pub(crate) const DIRTY_RPC_SEMA: u32 = 0x4e08;

/// Request and reply slots: 0x20 bytes, matched by the word at +0x1c.
pub(crate) const DIRTY_RPC_SLOT_SHIFT: u32 = 5;

pub(crate) const DIRTY_RPC_SLOT_SEQ: u32 = 0x1c;

/// A request slot's data (an IOP address in the data buffer) and its size.
pub(crate) const DIRTY_RPC_REQUEST_DATA: u32 = 0x0;

pub(crate) const DIRTY_RPC_REQUEST_SIZE: u32 = 0x4;

/// A reply slot's words passed on to a callback (+0x14 is also the result
/// word stored for a waiter).
pub(crate) const DIRTY_RPC_REPLY_WORD_8: u32 = 0x8;

pub(crate) const DIRTY_RPC_REPLY_WORD_C: u32 = 0xc;

pub(crate) const DIRTY_RPC_REPLY_WORD_10: u32 = 0x10;

pub(crate) const DIRTY_RPC_REPLY_RESULT: u32 = 0x14;

/// Waiter records: 16 bytes. +0 is 0 (nobody), 1 (store the result word
/// through +4 and wake thread +0xc) or a callback called with +8 and the
/// reply's words (+4 as its stack argument).
pub(crate) const DIRTY_RPC_WAITER_SHIFT: u32 = 4;

pub(crate) const DIRTY_RPC_WAITER_KIND: u32 = 0x0;

pub(crate) const DIRTY_RPC_WAITER_RESULT: u32 = 0x4;

pub(crate) const DIRTY_RPC_WAITER_ARG: u32 = 0x8;

pub(crate) const DIRTY_RPC_WAITER_THREAD: u32 = 0xc;

/// The waiter kind that stores the result word.
pub(crate) const DIRTY_RPC_WAIT_RESULT: u64 = 1;

/// Client table entries (16 bytes) and the request header `DirtyRpc_InitRequest`
/// fills from them: +0 the entry's index, +4 the id looked up, +8 a second
/// word that must match too, +0xc the client's id word (0 until bound;
/// copied on a match, compared when the cached index is checked).
pub(crate) const DIRTY_RPC_CLIENT_INDEX: u32 = 0x0;

pub(crate) const DIRTY_RPC_CLIENT_SERVER: u32 = 0x4;

pub(crate) const DIRTY_RPC_CLIENT_KEY: u32 = 0x8;

pub(crate) const DIRTY_RPC_CLIENT_ID: u32 = 0xc;

pub(crate) const DIRTY_RPC_CLIENT_SIZE: u32 = 0x10;

/// The cached index is only used below this.
pub(crate) const DIRTY_RPC_MAX_CLIENTS: i64 = 129;

/// gp-relative flags: 1 while the transport thread waits for the semaphore,
/// 1 while it sleeps, and 1 once the transport is started.
pub(crate) const DIRTY_RPC_SEMA_WAIT_GP: u32 = 0x2a10;

pub(crate) const DIRTY_RPC_SLEEPING_GP: u32 = 0x2a14;

pub(crate) const DIRTY_RPC_STARTED_GP: u32 = 0x1ac0;

/// The boot command packet sent to the IOP (first byte: its size). Its word
/// at +0xc is the boot reply byte's address + 1 in `DirtyRpc_WaitIop` and
/// the IOP heap block's address in `DirtyRpc_Startup`.
pub(crate) const DIRTY_RPC_BOOT_CMD: u32 = 0x0045_1340;

pub(crate) const DIRTY_RPC_BOOT_CMD_REPLY: u32 = 0xc;

/// How long the boot handshake waits (COP0 Count ticks).
pub(crate) const DIRTY_RPC_BOOT_TIMEOUT: u64 = 50_000_000;

/// The thread, alarm and interrupt the startup sets up.
pub(crate) const DIRTY_RPC_THREAD_MAIN: u32 = 0x003f_44f8;

pub(crate) const DIRTY_RPC_ALARM_HANDLER: u32 = 0x003f_4448;

pub(crate) const DIRTY_RPC_INTC_HANDLER_FN: u32 = 0x003f_4410;

pub(crate) const DIRTY_RPC_THREAD_STACK: u32 = DIRTY_RPC_STATE + DIRTY_RPC_STATE_SIZE;

pub(crate) const DIRTY_RPC_THREAD_STACK_SIZE: u64 = 0x2000;

pub(crate) const DIRTY_RPC_THREAD_PRIORITY: u64 = 0xa;

pub(crate) const DIRTY_RPC_THREAD_GP: u32 = 0x004a_30f0;

/// The handler alarm's period (ticks) and the SIF interrupt cause.
pub(crate) const DIRTY_RPC_HANDLER_TICKS: u64 = 0x10;

pub(crate) const DIRTY_RPC_INTC_SIF: u64 = 2;

/// `lw` of the word at `base + off`.
pub(crate) fn dirty_rpc_lw(c: &mut Cpu, base: u32, off: u32) -> u64 {
    Addr(base.wrapping_add(off)).lw(c)
}

/// `sw` of `v` to `base + off`.
pub(crate) fn dirty_rpc_sw(c: &mut Cpu, base: u32, off: u32, v: u64) {
    Addr(base.wrapping_add(off)).sw(c, v)
}

/// With `v0` = the read index's slot offset: the seq words of that reply slot
/// (`a1`) and request slot (`v1`), as `a0`/`v0` point at them; true when the
/// reply answers the request.
pub(crate) fn dirty_rpc_reply_arrived(c: &mut Cpu, rpc: u32) -> bool {
    c.g[A0][0] = dirty_rpc_lw(c, rpc, DIRTY_RPC_REPLIES);
    c.g[V1][0] = dirty_rpc_lw(c, rpc, DIRTY_RPC_REQUESTS);
    c.g[A0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
    c.g[A1][0] = dirty_rpc_lw(c, r32(c, A0), DIRTY_RPC_SLOT_SEQ);
    c.g[V1][0] = dirty_rpc_lw(c, r32(c, V0), DIRTY_RPC_SLOT_SEQ);
    c.g[A1][0] == c.g[V1][0]
}

/// `void DirtyRpc_AlarmHandler(int alarm, u16 time, DirtyRpc* rpc)`
/// (0x3f4448): the transport's alarm callback. Forgets whichever of the two
/// alarm ids fired (-1). Then, only while the ring is not empty (tail !=
/// head), it wakes the transport thread (`iWakeupThread`) when the reply at
/// the read index answers its request, and when neither alarm is pending any
/// more (both ids negative) re-arms itself as the 0x10-tick handler alarm.
pub fn dirty_rpc_alarm_handler(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A2][0];
        let rpc = r32(c, S0);
        c.g[V0][0] = dirty_rpc_lw(c, rpc, DIRTY_RPC_ALARM);
        if c.g[A0][0] == c.g[V0][0] {
            c.cycles += 3;
            c.g[V0][0] = u64::MAX;
            dirty_rpc_sw(c, rpc, DIRTY_RPC_ALARM, c.g[V0][0]);
        }
        c.g[V0][0] = dirty_rpc_lw(c, rpc, DIRTY_RPC_HANDLER_ALARM);
        c.cycles += 2;
        if c.g[A0][0] == c.g[V0][0] {
            c.cycles += 3;
            c.g[V0][0] = u64::MAX;
            dirty_rpc_sw(c, rpc, DIRTY_RPC_HANDLER_ALARM, c.g[V0][0]);
        }
        c.g[V1][0] = dirty_rpc_lw(c, rpc, DIRTY_RPC_TAIL);
        // Whether the path ends through the shared `+1` block (nothing
        // pending, or the alarm was re-armed).
        let tail = 'checked: {
            c.cycles += 3;
            c.g[V0][0] = dirty_rpc_lw(c, rpc, DIRTY_RPC_HEAD);
            let empty = c.g[V1][0] == c.g[V0][0];
            c.g[V0][0] = sx(r32(c, V1) << DIRTY_RPC_SLOT_SHIFT);
            if empty {
                break 'checked true;
            }
            c.cycles += 8;
            if dirty_rpc_reply_arrived(c, rpc) {
                c.cycles += 3;
                c.g[A0][0] = dirty_rpc_lw(c, rpc, DIRTY_RPC_THREAD);
                I_WAKEUP_THREAD.call(c, 0x003f_44b8)?;
            }
            let rpc = r32(c, S0);
            c.g[V0][0] = dirty_rpc_lw(c, rpc, DIRTY_RPC_ALARM);
            c.cycles += 2;
            if (c.g[V0][0] as i64) >= 0 {
                break 'checked false;
            }
            c.cycles += 3;
            c.g[V0][0] = dirty_rpc_lw(c, rpc, DIRTY_RPC_HANDLER_ALARM);
            if (c.g[V0][0] as i64) >= 0 {
                break 'checked false;
            }
            c.cycles += 6;
            c.g[A1][0] = sx(DIRTY_RPC_ALARM_HANDLER);
            c.g[A0][0] = DIRTY_RPC_HANDLER_TICKS;
            c.g[A2][0] = c.g[S0][0];
            I_SET_ALARM.call(c, 0x003f_44e4)?;
            dirty_rpc_sw(c, r32(c, S0), DIRTY_RPC_HANDLER_ALARM, c.g[V0][0]);
            true
        };
        if tail {
            c.cycles += 1;
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void DirtyRpc_ThreadMain(void)` (0x3f44f8): the transport thread's body.
/// Forever: handle the replies that arrived (`DirtyRpc_ProcessReplies`), then
/// sleep until the alarm or the SIF interrupt wakes it, with the gp flag
/// at `gp+0x2a14` set to 1 while it sleeps. Never returns.
pub fn dirty_rpc_thread_main(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = 1;
        loop {
            c.cycles += 8;
            DIRTY_RPC_PROCESS_REPLIES.call(c, 0x003f_4510)?;
            dirty_rpc_sw(c, r32(c, GP), DIRTY_RPC_SLEEPING_GP, c.g[S0][0]);
            SLEEP_THREAD.call(c, 0x003f_451c)?;
            dirty_rpc_sw(c, r32(c, GP), DIRTY_RPC_SLEEPING_GP, 0);
            if event_due(c) {
                deliver_event_here(c, 0x003f_4508);
            }
        }
    })
}

/// `int DirtyRpc_ProcessReplies(void)` (0x3f4a78): take every reply that has
/// arrived, in ring order. Each one is taken under the ring semaphore (with
/// `gp+0x2a10` set to 1 while waiting for it): the waiter record of its ring
/// index gets the reply (nothing; the result word stored through +4 and
/// thread +0xc woken; or the callback called with the reply's words), the
/// data cursor moves past the reply's data, the read index advances modulo
/// the slot count. Stops at the write index or at a reply that has not
/// arrived (also when it changed while waiting). Returns how many it took.
pub fn dirty_rpc_process_replies(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x60);
        c.g[V0][0] = 0x52_0000;
        frame.save(c, S0, 0x50);
        frame.save(c, S2, 0x30);
        c.g[S0][0] = sx(DIRTY_RPC_STATE);
        frame.save(c, S1, 0x40);
        frame.save(c, S3, 0x20);
        frame.save_ra(c, 0x10);
        let rpc = r32(c, S0);
        c.g[V1][0] = dirty_rpc_lw(c, rpc, DIRTY_RPC_TAIL);
        c.g[V0][0] = dirty_rpc_lw(c, rpc, DIRTY_RPC_HEAD);
        let empty = c.g[V1][0] == c.g[V0][0];
        c.g[S2][0] = 0;
        // 'counted: return the count (one more cycle); 'out: `v0` is set.
        'out: {
            'counted: {
                if empty {
                    break 'counted;
                }
                c.cycles += 9;
                c.g[V0][0] = sx(r32(c, V1) << DIRTY_RPC_SLOT_SHIFT);
                let arrived = dirty_rpc_reply_arrived(c, rpc);
                c.g[V0][0] = c.g[S2][0];
                if !arrived {
                    break 'out;
                }
                c.cycles += 3;
                c.g[S3][0] = 1;
                c.g[A0][0] = dirty_rpc_lw(c, rpc, DIRTY_RPC_SEMA);
                loop {
                    c.cycles += 14;
                    dirty_rpc_sw(c, r32(c, GP), DIRTY_RPC_SEMA_WAIT_GP, c.g[S3][0]);
                    WAIT_SEMA.call(c, 0x003f_4ae4)?;
                    let rpc = r32(c, S0);
                    // The slots of the read index, again under the semaphore.
                    c.g[A3][0] = dirty_rpc_lw(c, rpc, DIRTY_RPC_TAIL);
                    c.g[V1][0] = dirty_rpc_lw(c, rpc, DIRTY_RPC_REPLIES);
                    c.g[A0][0] = dirty_rpc_lw(c, rpc, DIRTY_RPC_REQUESTS);
                    c.g[V0][0] = sx(r32(c, A3) << DIRTY_RPC_SLOT_SHIFT);
                    c.g[A2][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
                    dirty_rpc_sw(c, r32(c, GP), DIRTY_RPC_SEMA_WAIT_GP, 0);
                    c.g[T1][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
                    c.g[T6][0] = dirty_rpc_lw(c, r32(c, A2), DIRTY_RPC_SLOT_SEQ);
                    c.g[V0][0] = dirty_rpc_lw(c, r32(c, T1), DIRTY_RPC_SLOT_SEQ);
                    if c.g[T6][0] != c.g[V0][0] {
                        c.cycles += 4;
                        c.g[A0][0] = dirty_rpc_lw(c, rpc, DIRTY_RPC_SEMA);
                        SIGNAL_SEMA.call(c, 0x003f_4b18)?;
                        c.g[V0][0] = c.g[S2][0];
                        break 'out;
                    }
                    c.g[A1][0] = dirty_rpc_lw(c, rpc, DIRTY_RPC_WAITERS);
                    // Hand the reply to its waiter (a2: reply slot, t1:
                    // request slot, t4: waiter record).
                    c.cycles += 16;
                    c.g[V0][0] = sx(r32(c, A3) << DIRTY_RPC_WAITER_SHIFT);
                    c.g[V1][0] = dirty_rpc_lw(c, r32(c, T1), DIRTY_RPC_REQUEST_DATA);
                    c.g[S1][0] = 0;
                    c.g[T4][0] = sx(r32(c, V0).wrapping_add(r32(c, A1)));
                    c.g[A0][0] = dirty_rpc_lw(c, rpc, DIRTY_RPC_IOP_BUFFER);
                    c.g[V0][0] = dirty_rpc_lw(c, rpc, DIRTY_RPC_EE_BUFFER);
                    c.g[V1][0] = sx(r32(c, V1).wrapping_sub(r32(c, A0)));
                    c.g[A3][0] = dirty_rpc_lw(c, r32(c, A2), DIRTY_RPC_REPLY_RESULT);
                    c.g[T2][0] = dirty_rpc_lw(c, r32(c, A2), DIRTY_RPC_REPLY_WORD_8);
                    c.g[T0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
                    c.g[T3][0] = dirty_rpc_lw(c, r32(c, A2), DIRTY_RPC_REPLY_WORD_C);
                    c.g[T5][0] = dirty_rpc_lw(c, r32(c, T4), DIRTY_RPC_WAITER_KIND);
                    c.g[A1][0] = dirty_rpc_lw(c, r32(c, T4), DIRTY_RPC_WAITER_ARG);
                    c.g[T1][0] = dirty_rpc_lw(c, r32(c, T1), DIRTY_RPC_REQUEST_SIZE);
                    let nobody = c.g[T5][0] == 0;
                    c.g[A2][0] = dirty_rpc_lw(c, r32(c, A2), DIRTY_RPC_REPLY_WORD_10);
                    if !nobody {
                        c.cycles += 2;
                        let callback = c.g[T5][0] != DIRTY_RPC_WAIT_RESULT;
                        c.g[V0][0] = dirty_rpc_lw(c, r32(c, T4), DIRTY_RPC_WAITER_RESULT);
                        if callback {
                            c.cycles += 5;
                            Addr(sp(c)).sw(c, c.g[V0][0]);
                            c.g[V0][0] = dirty_rpc_lw(c, r32(c, T4), DIRTY_RPC_WAITER_KIND);
                            c.g[A0][0] = c.g[T6][0];
                            call_ptr(c, r32(c, V0), 0x003f_4b98)?;
                        } else {
                            c.cycles += 2;
                            if c.g[V0][0] != 0 {
                                Addr(r32(c, V0)).sw(c, c.g[A3][0]);
                            }
                            c.cycles += 6;
                            let rpc = r32(c, S0);
                            c.g[V0][0] = dirty_rpc_lw(c, rpc, DIRTY_RPC_TAIL);
                            c.g[V1][0] = dirty_rpc_lw(c, rpc, DIRTY_RPC_WAITERS);
                            c.g[V0][0] = sx(r32(c, V0) << DIRTY_RPC_WAITER_SHIFT);
                            c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
                            c.g[S1][0] = dirty_rpc_lw(c, r32(c, V0), DIRTY_RPC_WAITER_THREAD);
                        }
                    }
                    // Move the data cursor past the request's data (rounded
                    // up to 0x40 bytes).
                    c.cycles += 7;
                    let rpc = r32(c, S0);
                    c.g[V0][0] = dirty_rpc_lw(c, rpc, DIRTY_RPC_TAIL);
                    c.g[A0][0] = dirty_rpc_lw(c, rpc, DIRTY_RPC_REQUESTS);
                    c.g[V0][0] = sx(r32(c, V0) << DIRTY_RPC_SLOT_SHIFT);
                    c.g[A1][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
                    c.g[V1][0] = dirty_rpc_lw(c, r32(c, A1), DIRTY_RPC_REQUEST_SIZE);
                    let no_data = (c.g[V1][0] as i64) <= 0;
                    c.g[A0][0] = 0xffff_ffff_ffff_ffc0;
                    if !no_data {
                        c.cycles += 9;
                        c.g[V1][0] = dirty_rpc_lw(c, r32(c, A1), DIRTY_RPC_REQUEST_DATA);
                        c.g[V0][0] = dirty_rpc_lw(c, rpc, DIRTY_RPC_IOP_BUFFER);
                        c.g[V1][0] = sx(r32(c, V1).wrapping_sub(r32(c, V0)));
                        dirty_rpc_sw(c, rpc, DIRTY_RPC_READ, c.g[V1][0]);
                        c.g[V0][0] = dirty_rpc_lw(c, r32(c, A1), DIRTY_RPC_REQUEST_SIZE);
                        c.g[V0][0] = sx(r32(c, V0).wrapping_add(0x3f));
                        c.g[V0][0] &= c.g[A0][0];
                        c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
                        dirty_rpc_sw(c, rpc, DIRTY_RPC_READ, c.g[V1][0]);
                    }
                    // Advance the read index (modulo the slot count).
                    c.cycles += 6;
                    c.g[V0][0] = dirty_rpc_lw(c, rpc, DIRTY_RPC_TAIL);
                    c.g[V1][0] = dirty_rpc_lw(c, rpc, DIRTY_RPC_SLOTS);
                    c.g[V0][0] = sx(r32(c, V0).wrapping_add(1));
                    c.div(0, V0, V1);
                    c.cycles += 6;
                    c.g[A0][0] = dirty_rpc_lw(c, rpc, DIRTY_RPC_SEMA);
                    c.g[A1][0] = c.hi[0];
                    dirty_rpc_sw(c, rpc, DIRTY_RPC_TAIL, c.g[A1][0]);
                    SIGNAL_SEMA.call(c, 0x003f_4c04)?;
                    if c.g[S1][0] != 0 {
                        c.cycles += 3;
                        c.g[A0][0] = c.g[S1][0];
                        WAKEUP_THREAD.call(c, 0x003f_4c14)?;
                    }
                    let rpc = r32(c, S0);
                    c.g[V1][0] = dirty_rpc_lw(c, rpc, DIRTY_RPC_TAIL);
                    c.cycles += 3;
                    c.g[V0][0] = dirty_rpc_lw(c, rpc, DIRTY_RPC_HEAD);
                    let drained = c.g[V1][0] == c.g[V0][0];
                    c.g[S2][0] = sx(r32(c, S2).wrapping_add(1));
                    if drained {
                        break 'counted;
                    }
                    // The next slot pair (registers as the generated code
                    // leaves them: a0 reply, v1 request).
                    c.cycles += 9;
                    c.g[A0][0] = dirty_rpc_lw(c, rpc, DIRTY_RPC_REPLIES);
                    c.g[V1][0] = sx(r32(c, V1) << DIRTY_RPC_SLOT_SHIFT);
                    c.g[V0][0] = dirty_rpc_lw(c, rpc, DIRTY_RPC_REQUESTS);
                    c.g[A0][0] = sx(r32(c, V1).wrapping_add(r32(c, A0)));
                    c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
                    c.g[A1][0] = dirty_rpc_lw(c, r32(c, A0), DIRTY_RPC_SLOT_SEQ);
                    c.g[V0][0] = dirty_rpc_lw(c, r32(c, V1), DIRTY_RPC_SLOT_SEQ);
                    if c.g[A1][0] != c.g[V0][0] {
                        break 'counted;
                    }
                    c.g[A0][0] = dirty_rpc_lw(c, rpc, DIRTY_RPC_SEMA);
                    if event_due(c) {
                        deliver_event_here(c, 0x003f_4ad8);
                    }
                }
            }
            c.cycles += 1;
            c.g[V0][0] = c.g[S2][0];
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

/// `int DirtyRpc_Call(...)` (0x3f4c68): `DirtyRpc_Send` with the same
/// registers, the caller's stack words at +0 and +8 as its first and second
/// stack words and `a3` again as its third; returns what it returns.
pub fn dirty_rpc_call(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x60);
        c.g[T4][0] = c.g[A3][0];
        c.g[V0][0] = Addr(frame.at(c, 0x60)).lw(c);
        c.g[V1][0] = Addr(frame.at(c, 0x68)).lw(c);
        frame.save_ra(c, 0x50);
        Addr(frame.at(c, 0)).sw(c, c.g[V0][0]);
        Addr(frame.at(c, 0x8)).sw(c, c.g[V1][0]);
        Addr(frame.at(c, 0x10)).sw(c, c.g[T4][0]);
        DIRTY_RPC_SEND.call(c, 0x003f_4c8c)?;
        frame.restore_ra(c, 0x50);
        frame.close(c);
        Ok(())
    })
}

/// `int DirtyRpc_InitRequest(Request* req, int id, int key)` (0x3f4c98):
/// bind a request header to the client table entry with this id and key.
/// Returns 0 when the header's cached entry (+0: index, below 129) still has
/// this id and the header's +0xc word; otherwise stores id and key and scans
/// the table: 1 and the entry's index and +0xc word copied on a match, -1 and
/// both cleared when there is none.
pub fn dirty_rpc_init_request(c: &mut Cpu) {
    c.cycles += 6;
    c.g[A3][0] = c.g[A0][0];
    c.g[V0][0] = 0x52_0000;
    let req = r32(c, A3);
    c.g[V1][0] = dirty_rpc_lw(c, req, DIRTY_RPC_CLIENT_SERVER);
    c.g[T0][0] = c.g[A1][0];
    let same_id = c.g[T0][0] == c.g[V1][0];
    c.g[T1][0] = sx(DIRTY_RPC_STATE);
    'scan: {
        // 'stale: the cached entry is not usable (one more cycle, then the
        // key is stored).
        'stale: {
            if !same_id {
                break 'stale;
            }
            c.cycles += 3;
            c.g[V0][0] = dirty_rpc_lw(c, req, DIRTY_RPC_CLIENT_KEY);
            if c.g[A2][0] != c.g[V0][0] {
                dirty_rpc_sw(c, req, DIRTY_RPC_CLIENT_KEY, c.g[A2][0]);
                break 'scan;
            }
            c.cycles += 3;
            c.g[V1][0] = dirty_rpc_lw(c, req, DIRTY_RPC_CLIENT_INDEX);
            let negative = (c.g[V1][0] as i64) < 0;
            c.g[V0][0] = ((c.g[V1][0] as i64) < DIRTY_RPC_MAX_CLIENTS) as u64;
            if negative {
                break 'stale;
            }
            c.cycles += 2;
            let too_big = c.g[V0][0] == 0;
            c.g[V1][0] = sx(r32(c, V1) << 4);
            if too_big {
                break 'stale;
            }
            c.cycles += 6;
            c.g[A0][0] = dirty_rpc_lw(c, r32(c, T1), DIRTY_RPC_CLIENTS);
            c.g[A1][0] = dirty_rpc_lw(c, req, DIRTY_RPC_CLIENT_ID);
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, A0)));
            let entry = r32(c, V1);
            c.g[V0][0] = dirty_rpc_lw(c, entry, DIRTY_RPC_CLIENT_ID);
            if c.g[V0][0] != c.g[A1][0] {
                dirty_rpc_sw(c, req, DIRTY_RPC_CLIENT_KEY, c.g[A2][0]);
                break 'scan;
            }
            c.cycles += 3;
            c.g[V0][0] = dirty_rpc_lw(c, entry, DIRTY_RPC_CLIENT_SERVER);
            if c.g[V0][0] != c.g[T0][0] {
                dirty_rpc_sw(c, req, DIRTY_RPC_CLIENT_KEY, c.g[A2][0]);
                break 'scan;
            }
            c.cycles += 2;
            c.g[V0][0] = 0;
            ret(c);
            return;
        }
        c.cycles += 1;
        dirty_rpc_sw(c, req, DIRTY_RPC_CLIENT_KEY, c.g[A2][0]);
    }
    c.cycles += 5;
    dirty_rpc_sw(c, req, DIRTY_RPC_CLIENT_SERVER, c.g[T0][0]);
    let rpc = r32(c, T1);
    c.g[V0][0] = dirty_rpc_lw(c, rpc, DIRTY_RPC_CLIENTS_END);
    c.g[A2][0] = dirty_rpc_lw(c, rpc, DIRTY_RPC_CLIENTS);
    let empty = c.g[A2][0] == c.g[V0][0];
    c.g[A1][0] = c.g[T0][0];
    if !empty {
        c.cycles += 2;
        c.g[A0][0] = c.g[V0][0];
        c.g[V0][0] = dirty_rpc_lw(c, r32(c, A2), DIRTY_RPC_CLIENT_SERVER);
        loop {
            c.cycles += 2;
            if c.g[V0][0] == c.g[A1][0] {
                c.cycles += 4;
                let entry = r32(c, A2);
                c.g[V1][0] = dirty_rpc_lw(c, entry, DIRTY_RPC_CLIENT_KEY);
                c.g[V0][0] = dirty_rpc_lw(c, req, DIRTY_RPC_CLIENT_KEY);
                if c.g[V1][0] == c.g[V0][0] {
                    c.g[A0][0] = dirty_rpc_lw(c, entry, DIRTY_RPC_CLIENT_INDEX);
                    c.cycles += 5;
                    c.g[V0][0] = 1;
                    dirty_rpc_sw(c, req, DIRTY_RPC_CLIENT_INDEX, c.g[A0][0]);
                    c.g[V1][0] = dirty_rpc_lw(c, entry, DIRTY_RPC_CLIENT_ID);
                    dirty_rpc_sw(c, req, DIRTY_RPC_CLIENT_ID, c.g[V1][0]);
                    ret(c);
                    return;
                }
                c.cycles += 1;
            }
            c.g[A2][0] = sx(r32(c, A2).wrapping_add(DIRTY_RPC_CLIENT_SIZE));
            c.cycles += 2;
            if c.g[A2][0] == c.g[A0][0] {
                break;
            }
            c.g[V0][0] = dirty_rpc_lw(c, r32(c, A2), DIRTY_RPC_CLIENT_SERVER);
            if event_due(c) {
                deliver_event_here(c, 0x003f_4d30);
            }
        }
    }
    c.cycles += 4;
    dirty_rpc_sw(c, req, DIRTY_RPC_CLIENT_ID, 0);
    c.g[V0][0] = u64::MAX;
    dirty_rpc_sw(c, req, DIRTY_RPC_CLIENT_INDEX, 0);
    ret(c);
}

/// `int DirtyRpc_InitIop(void)` (0x3f4d68): send request 1 to the IOP side
/// (`DirtyRpc_Call` with a request header on the stack and the client table's
/// address and size), then take replies (`DirtyRpc_ProcessReplies`) until the
/// read index reaches the write index. Returns 0. The wait loop's head is a
/// resume entry (an event there can unwind).
pub fn dirty_rpc_init_iop(c: &mut Cpu) {
    const ENTRY: u32 = 0x003f_4d68;
    const POLL: u32 = 0x003f_4dc0;
    guest(c, |c| {
        let frame = Frame { size: 0x40 };
        let mut resumed = match c.pc {
            ENTRY => false,
            POLL => true,
            pc => {
                c.jump_out(pc, ENTRY, 0x003f_4de8);
                return Ok(());
            }
        };
        if !resumed {
            c.cycles += 21;
            Frame::open(c, 0x40);
            c.g[V1][0] = 1;
            frame.save(c, S0, 0x30);
            c.g[V0][0] = 0x52_0000;
            Addr(frame.at(c, 0)).sw(c, c.g[V1][0]);
            c.g[S0][0] = sx(DIRTY_RPC_STATE);
            frame.save_ra(c, 0x20);
            c.g[A0][0] = sx(frame.at(c, 0x10));
            c.g[A1][0] = 1;
            c.g[A2][0] = 0;
            c.g[T2][0] = dirty_rpc_lw(c, DIRTY_RPC_STATE, DIRTY_RPC_CLIENTS);
            c.g[A3][0] = 0;
            c.g[T3][0] = dirty_rpc_lw(c, DIRTY_RPC_STATE, DIRTY_RPC_CLIENTS_END);
            c.g[T0][0] = 0;
            Addr(frame.at(c, 0x1c)).sw(c, c.g[V1][0]);
            c.g[T1][0] = 0;
            c.g[T3][0] = sx(r32(c, T3).wrapping_sub(r32(c, T2)));
            Addr(frame.at(c, 0x8)).sw(c, 0);
            DIRTY_RPC_CALL.call(c, 0x003f_4db4)?;
            c.g[V1][0] = dirty_rpc_lw(c, r32(c, S0), DIRTY_RPC_HEAD);
        }
        loop {
            if !resumed {
                c.cycles += 3;
                c.g[V0][0] = dirty_rpc_lw(c, r32(c, S0), DIRTY_RPC_TAIL);
                let pending = c.g[V1][0] != c.g[V0][0];
                c.g[V0][0] = 0;
                if !pending {
                    break;
                }
                if back_edge_unwinds(c, POLL) {
                    return Ok(());
                }
            }
            resumed = false;
            c.cycles += 3;
            DIRTY_RPC_PROCESS_REPLIES.call(c, 0x003f_4dc8)?;
            c.g[V1][0] = dirty_rpc_lw(c, r32(c, S0), DIRTY_RPC_HEAD);
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `void DirtyRpc_SyncDCache(void* start, int size)` (0x3f4e80): write back
/// the data cache over `[start, start + size)` (`SyncDCache(start, start +
/// size)`).
pub fn dirty_rpc_sync_dcache(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A1][0] = sx(r32(c, A0).wrapping_add(r32(c, A1)));
        SYNC_DCACHE.call(c, 0x003f_4e90)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// A receive record's header (read through its uncached alias): +0 the next
/// record, +8 a sequence word (the next record's must equal it), +0xc the
/// data length (0..0x8000); the data follows the 16-byte header.
pub(crate) const NET_RECV_NEXT: u32 = 0x0;

pub(crate) const NET_RECV_SEQ: u32 = 0x8;

pub(crate) const NET_RECV_LEN: u32 = 0xc;

pub(crate) const NET_RECV_MAX_LEN: i64 = 0x8000;

/// `int NetGameLink_CheckRecvBuf(void* buf, int* len)` (0x3f4ea0): check the
/// receive record at `buf` (uncached). Its next pointer must be set, its
/// length 0..0x8000, the next record must start right after the header and
/// the data rounded up to 16 bytes and point back to this one, and the two
/// sequence words must match. Returns the sequence word and stores the
/// length in `*len` (when `len` is set); -1 no record or a bad length, -2 a
/// bad link, -3 the sequence words differ.
pub fn net_game_link_check_recv_buf(c: &mut Cpu) {
    c.cycles += 8;
    c.g[A3][0] = 0x0fff_ffff;
    c.g[T2][0] = 0x2000_0000;
    c.g[A0][0] &= c.g[A3][0];
    c.g[A2][0] = c.g[A0][0] | c.g[T2][0];
    let buf = r32(c, A2);
    c.g[A0][0] = dirty_rpc_lw(c, buf, NET_RECV_NEXT);
    'bad: {
        if c.g[A0][0] == 0 {
            break 'bad;
        }
        c.cycles += 3;
        c.g[T0][0] = dirty_rpc_lw(c, buf, NET_RECV_LEN);
        let negative = (c.g[T0][0] as i64) < 0;
        c.g[V0][0] = NET_RECV_MAX_LEN as u64;
        if negative {
            break 'bad;
        }
        c.cycles += 3;
        c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[T0][0] as i64)) as u64;
        let fits = c.g[V0][0] == 0;
        c.g[V0][0] = 0x7fff_0000;
        if !fits {
            break 'bad;
        }
        'unlinked: {
            c.cycles += 9;
            c.g[V1][0] = sx(r32(c, T0).wrapping_add(0xf));
            c.g[V0][0] |= 0xfff0;
            c.g[A0][0] &= c.g[A3][0];
            c.g[V1][0] &= c.g[V0][0];
            c.g[A0][0] |= c.g[T2][0];
            c.g[V1][0] = sx(r32(c, V1).wrapping_add(0x10));
            c.g[V1][0] = sx(r32(c, A2).wrapping_add(r32(c, V1)));
            let misplaced = c.g[A0][0] != c.g[V1][0];
            c.g[T1][0] = dirty_rpc_lw(c, buf, NET_RECV_SEQ);
            if misplaced {
                break 'unlinked;
            }
            c.cycles += 5;
            let next = r32(c, A0);
            c.g[V0][0] = dirty_rpc_lw(c, next, NET_RECV_NEXT);
            c.g[V0][0] &= c.g[A3][0];
            c.g[V0][0] |= c.g[T2][0];
            if c.g[V0][0] != c.g[A2][0] {
                break 'unlinked;
            }
            c.g[V0][0] = dirty_rpc_lw(c, next, NET_RECV_SEQ);
            c.cycles += 2;
            let torn = c.g[T1][0] != c.g[V0][0];
            c.g[V0][0] = (-3i64) as u64;
            if torn {
                c.cycles += 2;
                ret(c);
                return;
            }
            c.cycles += 2;
            if c.g[A1][0] != 0 {
                Addr(r32(c, A1)).sw(c, c.g[T0][0]);
            }
            c.cycles += 2;
            c.g[V0][0] = c.g[T1][0];
            ret(c);
            return;
        }
        c.cycles += 2;
        c.g[V0][0] = (-2i64) as u64;
        ret(c);
        return;
    }
    c.cycles += 2;
    c.g[V0][0] = u64::MAX;
    ret(c);
}

/// `int DirtyRpc_WaitIop(void)` (0x3f4f40): the boot handshake. Clears the
/// boot reply byte, puts its address + 1 in the boot command packet's reply
/// word (+0xc), flushes the
/// cache and sends the packet (`sceSifSendCmd(1, packet, size, 0, 0, 0)`);
/// then flushes and polls the byte until the IOP sets it (returns 1) or 50
/// million Count ticks pass (returns 0). The poll loop's head is a resume
/// entry (an event there can unwind).
pub fn dirty_rpc_wait_iop(c: &mut Cpu) {
    const ENTRY: u32 = 0x003f_4f40;
    const POLL: u32 = 0x003f_4fb0;
    guest(c, |c| {
        let frame = Frame { size: 0x50 };
        // Whether the next pass runs the poll head (flush, reload the byte).
        let mut at_head = match c.pc {
            ENTRY => false,
            POLL => true,
            pc => {
                c.jump_out(pc, ENTRY, 0x003f_4ffc);
                return Ok(());
            }
        };
        if !at_head {
            c.cycles += 28;
            Frame::open(c, 0x50);
            c.g[V1][0] = 0x52_0000;
            frame.save(c, S1, 0x30);
            c.g[A0][0] = 0;
            frame.save(c, S3, 0x10);
            c.g[S1][0] = buddy_lui(DIRTY_RPC_BOOT_CMD);
            frame.save(c, S0, 0x40);
            c.g[S3][0] = sx(DIRTY_RPC_STATE);
            frame.save(c, S2, 0x20);
            c.g[S0][0] = sx(DIRTY_RPC_BOOT_CMD);
            frame.save_ra(c, 0);
            // The packet's reply word gets the reply byte's address + 1, not
            // the byte's own address.
            c.g[V0][0] = sx(DIRTY_RPC_STATE + DIRTY_RPC_BOOT_REPLY + 1);
            Addr(DIRTY_RPC_STATE + DIRTY_RPC_BOOT_REPLY).sb(c, 0);
            c.g[S2][0] = DIRTY_RPC_BOOT_TIMEOUT;
            dirty_rpc_sw(c, r32(c, S0), DIRTY_RPC_BOOT_CMD_REPLY, c.g[V0][0]);
            FLUSH_CACHE.call(c, 0x003f_4f84)?;
            let size_at = r32(c, S1).wrapping_add(DIRTY_RPC_BOOT_CMD & 0xffff);
            c.g[A2][0] = Addr(size_at).lbu(c);
            c.g[A1][0] = c.g[S0][0];
            c.g[A0][0] = 1;
            c.g[A3][0] = 0;
            c.g[T0][0] = 0;
            c.g[T1][0] = 0;
            SCE_SIF_SEND_CMD.call(c, 0x003f_4fa0)?;
            c.g[S0][0] = 1;
            c.mfc0(S1, 9);
            c.g[V0][0] = Addr(r32(c, S3)).lb(c);
        }
        loop {
            if at_head {
                c.cycles += 3;
                c.g[A0][0] = 0;
                FLUSH_CACHE.call(c, 0x003f_4fb8)?;
                c.g[V0][0] = Addr(r32(c, S3)).lb(c);
            }
            at_head = true;
            c.cycles += 2;
            let answered = c.g[V0][0] != 0;
            c.g[V0][0] = c.g[S0][0];
            if answered {
                break;
            }
            c.cycles += 5;
            c.mfc0(V0, 9);
            c.g[V0][0] = sx(r32(c, V0).wrapping_sub(r32(c, S1)));
            c.g[V0][0] = (c.g[S2][0] < c.g[V0][0]) as u64;
            if c.g[V0][0] != 0 {
                c.cycles += 2;
                c.g[S0][0] = 0;
                c.g[V0][0] = c.g[S0][0];
                break;
            }
            if back_edge_unwinds(c, POLL) {
                return Ok(());
            }
        }
        c.cycles += 7;
        frame.restore(c, S1, 0x30);
        frame.restore(c, S0, 0x40);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int DirtyRpc_Startup(void)` (0x3f5000): bring the transport up. After
/// the boot handshake (`DirtyRpc_WaitIop`; -1 when the IOP never answers) it
/// clears the state block and lays out its regions, takes 0x4c80 bytes of IOP
/// heap for their IOP copies (-2 without), DMAs the 0x40-byte table of
/// region addresses (from the stack) to the heap block and then the 0x200
/// bytes of request slots to the IOP (-3/-4 when a DMA cannot be queued;
/// each waits for its DMA), sends the boot packet again with the heap address,
/// sets the ring indices and sizes, creates the ring semaphore and the
/// transport thread (`DirtyRpc_ThreadMain`, priority 10, a 0x2000-byte stack
/// after the state block; -5 when it fails), installs and enables the SIF
/// interrupt handler, starts the thread and sets the started flag. Returns 0.
pub fn dirty_rpc_startup(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x110);
        c.g[V0][0] = 0x52_0000;
        frame.save(c, S1, 0xf0);
        frame.save(c, S0, 0x100);
        frame.save(c, S2, 0xe0);
        frame.save(c, S3, 0xd0);
        frame.save(c, S4, 0xc0);
        frame.save(c, S5, 0xb0);
        frame.save_ra(c, 0xa0);
        c.g[S1][0] = sx(DIRTY_RPC_STATE);
        DIRTY_RPC_WAIT_IOP.call(c, 0x003f_502c)?;
        'out: {
            let answered = c.g[V0][0] != 0;
            c.g[A0][0] = c.g[S1][0];
            if !answered {
                c.cycles += 2;
                c.g[V0][0] = u64::MAX;
                break 'out;
            }
            // Clear the block, then the region bounds (EE uncached `t1 |`
            // aliases) and the heap size the IOP copies need.
            c.cycles += 60;
            c.g[A1][0] = 0;
            c.g[A2][0] = sx(DIRTY_RPC_STATE_SIZE);
            c.g[S0][0] = 0x0fff_0000;
            MEMSET.call(c, 0x003f_504c)?;
            c.g[S0][0] |= 0xffff;
            let rpc = r32(c, S1);
            c.g[T0][0] = sx(rpc.wrapping_add(0x800));
            c.g[T1][0] = 0x2000_0000;
            c.g[T0][0] &= c.g[S0][0];
            c.g[T4][0] = c.g[S1][0] & c.g[S0][0];
            c.g[A3][0] = sx(rpc.wrapping_add(0xa00));
            c.g[T0][0] |= c.g[T1][0];
            c.g[T4][0] |= c.g[T1][0];
            c.g[A3][0] &= c.g[S0][0];
            c.g[A2][0] = sx(rpc.wrapping_add(0xc00));
            c.g[A3][0] |= c.g[T1][0];
            c.g[A2][0] &= c.g[S0][0];
            c.g[A0][0] = sx(r32(c, T0).wrapping_sub(r32(c, T4)));
            c.g[A2][0] |= c.g[T1][0];
            c.g[A0][0] = sx((r32(c, A0) as i32 >> 4) as u32);
            c.g[V0][0] = sx(r32(c, A3).wrapping_sub(r32(c, T0)));
            c.g[T5][0] = sx(rpc.wrapping_add(0x4c00));
            c.g[A0][0] = sx(r32(c, A0) << 4);
            c.g[V0][0] = sx((r32(c, V0) as i32 >> 5) as u32);
            c.g[V1][0] = sx(r32(c, A2).wrapping_sub(r32(c, A3)));
            c.g[V0][0] = sx(r32(c, V0) << 5);
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x40));
            c.g[V1][0] = sx((r32(c, V1) as i32 >> 5) as u32);
            c.g[T2][0] = c.g[T5][0] & c.g[S0][0];
            c.g[T3][0] = sx(rpc.wrapping_add(0x4c40));
            c.g[T2][0] |= c.g[T1][0];
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, V0)));
            c.g[V1][0] = sx(r32(c, V1) << 5);
            c.g[T1][0] = sx(r32(c, T2).wrapping_sub(r32(c, A2)));
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, V1)));
            c.g[A1][0] = sx(r32(c, T3).wrapping_sub(r32(c, T5)));
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, T1)));
            c.g[A1][0] = sx((r32(c, A1) as i32 >> 2) as u32);
            c.g[V0][0] = 0x7fff_0000;
            c.g[A1][0] = sx(r32(c, A1) << 2);
            c.g[V0][0] |= 0xffff;
            c.g[V1][0] = 0x10;
            c.g[T1][0] = sx(rpc.wrapping_add(0x4d40));
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, A1)));
            dirty_rpc_sw(c, rpc, DIRTY_RPC_TABLE_HEAD, c.g[V1][0]);
            dirty_rpc_sw(c, rpc, DIRTY_RPC_ID_LIMIT, c.g[V0][0]);
            dirty_rpc_sw(c, rpc, DIRTY_RPC_WORD_4DC8, c.g[T1][0]);
            dirty_rpc_sw(c, rpc, DIRTY_RPC_CLIENTS, c.g[T4][0]);
            dirty_rpc_sw(c, rpc, DIRTY_RPC_CLIENTS_END, c.g[T0][0]);
            dirty_rpc_sw(c, rpc, DIRTY_RPC_REQUESTS, c.g[T0][0]);
            dirty_rpc_sw(c, rpc, DIRTY_RPC_REQUESTS_END, c.g[A3][0]);
            dirty_rpc_sw(c, rpc, DIRTY_RPC_REPLIES, c.g[A3][0]);
            dirty_rpc_sw(c, rpc, DIRTY_RPC_REPLIES_END, c.g[A2][0]);
            dirty_rpc_sw(c, rpc, DIRTY_RPC_EE_BUFFER, c.g[A2][0]);
            dirty_rpc_sw(c, rpc, DIRTY_RPC_EE_BUFFER_END, c.g[T2][0]);
            dirty_rpc_sw(c, rpc, DIRTY_RPC_DMA_IDS, c.g[T5][0]);
            dirty_rpc_sw(c, rpc, DIRTY_RPC_DMA_IDS_END, c.g[T3][0]);
            dirty_rpc_sw(c, rpc, DIRTY_RPC_WAITERS, c.g[T3][0]);
            SCE_SIF_ALLOC_IOP_HEAP.call(c, 0x003f_5124)?;
            let rpc = r32(c, S1);
            let heap = c.g[V0][0] != 0;
            dirty_rpc_sw(c, rpc, DIRTY_RPC_IOP_HEAP, c.g[V0][0]);
            if !heap {
                c.cycles += 2;
                c.g[V0][0] = (-2i64) as u64;
                break 'out;
            }
            // The IOP addresses of the regions (packed into the heap block
            // after its 0x40-byte table) and the table itself on the stack.
            c.cycles += 70;
            c.g[T6][0] = dirty_rpc_lw(c, rpc, DIRTY_RPC_CLIENTS);
            c.g[T2][0] = c.g[V0][0] & c.g[S0][0];
            c.g[A2][0] = dirty_rpc_lw(c, rpc, DIRTY_RPC_CLIENTS_END);
            c.g[V0][0] = 0xffff_ffff_a000_0000;
            c.g[T5][0] = dirty_rpc_lw(c, rpc, DIRTY_RPC_REQUESTS);
            c.g[T2][0] |= c.g[V0][0];
            c.g[A1][0] = dirty_rpc_lw(c, rpc, DIRTY_RPC_REQUESTS_END);
            c.g[A2][0] = sx(r32(c, A2).wrapping_sub(r32(c, T6)));
            c.g[T4][0] = dirty_rpc_lw(c, rpc, DIRTY_RPC_REPLIES);
            c.g[A2][0] = sx((r32(c, A2) as i32 >> 4) as u32);
            c.g[V1][0] = dirty_rpc_lw(c, rpc, DIRTY_RPC_REPLIES_END);
            c.g[A1][0] = sx(r32(c, A1).wrapping_sub(r32(c, T5)));
            c.g[A0][0] = dirty_rpc_lw(c, rpc, DIRTY_RPC_DMA_IDS);
            c.g[T1][0] = sx(r32(c, T2).wrapping_add(0x40));
            c.g[V0][0] = dirty_rpc_lw(c, rpc, DIRTY_RPC_DMA_IDS_END);
            c.g[V1][0] = sx(r32(c, V1).wrapping_sub(r32(c, T4)));
            c.g[A2][0] = sx(r32(c, A2) << 4);
            c.g[A1][0] = sx((r32(c, A1) as i32 >> 5) as u32);
            c.g[T3][0] = dirty_rpc_lw(c, rpc, DIRTY_RPC_EE_BUFFER);
            c.g[A2][0] = sx(r32(c, T1).wrapping_add(r32(c, A2)));
            c.g[A3][0] = dirty_rpc_lw(c, rpc, DIRTY_RPC_EE_BUFFER_END);
            c.g[A1][0] = sx(r32(c, A1) << 5);
            c.g[V1][0] = sx((r32(c, V1) as i32 >> 5) as u32);
            c.g[A1][0] = sx(r32(c, A2).wrapping_add(r32(c, A1)));
            c.g[V0][0] = sx(r32(c, V0).wrapping_sub(r32(c, A0)));
            c.g[V1][0] = sx(r32(c, V1) << 5);
            c.g[V1][0] = sx(r32(c, A1).wrapping_add(r32(c, V1)));
            c.g[A3][0] = sx(r32(c, A3).wrapping_sub(r32(c, T3)));
            c.g[V0][0] = sx((r32(c, V0) as i32 >> 2) as u32);
            c.g[A0][0] = dirty_rpc_lw(c, rpc, DIRTY_RPC_TABLE_HEAD);
            c.g[T0][0] = dirty_rpc_lw(c, rpc, DIRTY_RPC_ID_LIMIT);
            c.g[A3][0] = sx(r32(c, V1).wrapping_add(r32(c, A3)));
            c.g[V0][0] = sx(r32(c, V0) << 2);
            let table = sp(c);
            dirty_rpc_sw(c, table, 0x0, c.g[A0][0]);
            c.g[V0][0] = sx(r32(c, A3).wrapping_add(r32(c, V0)));
            dirty_rpc_sw(c, table, 0x4, c.g[T0][0]);
            dirty_rpc_sw(c, table, 0x3c, c.g[V0][0]);
            c.g[A0][0] = 0;
            dirty_rpc_sw(c, table, 0x10, c.g[T6][0]);
            dirty_rpc_sw(c, table, 0x1c, c.g[T5][0]);
            dirty_rpc_sw(c, table, 0x28, c.g[T4][0]);
            dirty_rpc_sw(c, table, 0x34, c.g[T3][0]);
            dirty_rpc_sw(c, rpc, DIRTY_RPC_IOP_CLIENTS, c.g[T1][0]);
            dirty_rpc_sw(c, rpc, DIRTY_RPC_IOP_REQUESTS, c.g[A2][0]);
            dirty_rpc_sw(c, rpc, DIRTY_RPC_IOP_REPLIES, c.g[A1][0]);
            dirty_rpc_sw(c, rpc, DIRTY_RPC_IOP_BUFFER, c.g[V1][0]);
            dirty_rpc_sw(c, table, 0x18, c.g[A1][0]);
            dirty_rpc_sw(c, table, 0x20, c.g[A1][0]);
            dirty_rpc_sw(c, table, 0x24, c.g[V1][0]);
            dirty_rpc_sw(c, table, 0x2c, c.g[V1][0]);
            dirty_rpc_sw(c, table, 0x8, c.g[T1][0]);
            dirty_rpc_sw(c, table, 0xc, c.g[A2][0]);
            dirty_rpc_sw(c, table, 0x14, c.g[A2][0]);
            dirty_rpc_sw(c, table, 0x30, c.g[A3][0]);
            dirty_rpc_sw(c, table, 0x38, c.g[A3][0]);
            dirty_rpc_sw(c, rpc, DIRTY_RPC_IOP_HEAP, c.g[T2][0]);
            FLUSH_CACHE.call(c, 0x003f_5218)?;
            // DMA the table to the heap block: { src, dest, size, attr } at
            // sp+0x40.
            let rpc = r32(c, S1);
            c.g[V1][0] = dirty_rpc_lw(c, rpc, DIRTY_RPC_IOP_HEAP);
            c.g[A0][0] = sx(frame.at(c, 0x40));
            c.g[V0][0] = 0x40;
            c.g[S2][0] = c.g[A0][0];
            Addr(frame.at(c, 0x44)).sw(c, c.g[V1][0]);
            c.g[A1][0] = 1;
            Addr(frame.at(c, 0x48)).sw(c, c.g[V0][0]);
            Addr(frame.at(c, 0x40)).sw(c, c.g[SP][0]);
            Addr(frame.at(c, 0x4c)).sw(c, 0);
            SCE_SIF_SET_DMA.call(c, 0x003f_5240)?;
            c.g[S0][0] = c.g[V0][0];
            let unqueued = c.g[S0][0] == 0;
            c.g[V0][0] = (-3i64) as u64;
            if unqueued {
                break 'out;
            }
            c.cycles += 1;
            loop {
                c.cycles += 4;
                c.g[A0][0] = c.g[S0][0];
                SCE_SIF_DMA_STAT.call(c, 0x003f_5258)?;
                let busy = (c.g[V0][0] as i64) >= 0;
                c.g[A2][0] = 0x200;
                if !busy {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003f_5250);
                }
            }
            // DMA the request slots to their IOP copy.
            c.cycles += 12;
            let rpc = r32(c, S1);
            c.g[V0][0] = dirty_rpc_lw(c, rpc, DIRTY_RPC_REQUESTS);
            c.g[V1][0] = dirty_rpc_lw(c, rpc, DIRTY_RPC_IOP_REQUESTS);
            c.g[A0][0] = c.g[S2][0];
            Addr(frame.at(c, 0x40)).sw(c, c.g[V0][0]);
            c.g[A1][0] = 1;
            Addr(frame.at(c, 0x44)).sw(c, c.g[V1][0]);
            Addr(frame.at(c, 0x48)).sw(c, c.g[A2][0]);
            Addr(frame.at(c, 0x4c)).sw(c, 0);
            SCE_SIF_SET_DMA.call(c, 0x003f_5284)?;
            c.g[S0][0] = c.g[V0][0];
            let queued = c.g[S0][0] != 0;
            c.g[S3][0] = buddy_lui(DIRTY_RPC_BOOT_CMD);
            if !queued {
                c.cycles += 2;
                c.g[V0][0] = (-4i64) as u64;
                break 'out;
            }
            c.cycles += 2;
            c.g[S4][0] = sx(frame.at(c, 0x50));
            c.g[S5][0] = sx(frame.at(c, 0x70));
            loop {
                c.cycles += 4;
                c.g[A0][0] = c.g[S0][0];
                SCE_SIF_DMA_STAT.call(c, 0x003f_52a8)?;
                if (c.g[V0][0] as i64) < 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003f_52a0);
                }
            }
            // Boot packet with the heap address, ring words, semaphore and
            // thread.
            c.cycles += 57;
            c.g[S0][0] = sx(r32(c, S3).wrapping_add(DIRTY_RPC_BOOT_CMD & 0xffff));
            SCE_SIF_INIT_CMD.call(c, 0x003f_52b8)?;
            c.g[V0][0] = dirty_rpc_lw(c, r32(c, S1), DIRTY_RPC_IOP_HEAP);
            c.g[A0][0] = 0;
            c.g[S2][0] = 1;
            dirty_rpc_sw(c, r32(c, S0), DIRTY_RPC_BOOT_CMD_REPLY, c.g[V0][0]);
            FLUSH_CACHE.call(c, 0x003f_52cc)?;
            let size_at = r32(c, S3).wrapping_add(DIRTY_RPC_BOOT_CMD & 0xffff);
            c.g[A2][0] = Addr(size_at).lbu(c);
            c.g[T0][0] = 0;
            c.g[A3][0] = 0;
            c.g[T1][0] = 0;
            c.g[A1][0] = c.g[S0][0];
            c.g[A0][0] = 1;
            SCE_SIF_SEND_CMD.call(c, 0x003f_52e8)?;
            let rpc = r32(c, S1);
            c.g[T0][0] = dirty_rpc_lw(c, rpc, DIRTY_RPC_REQUESTS);
            c.g[A1][0] = 0;
            c.g[V1][0] = dirty_rpc_lw(c, rpc, DIRTY_RPC_REQUESTS_END);
            c.g[A2][0] = 0x18;
            c.g[A3][0] = dirty_rpc_lw(c, rpc, DIRTY_RPC_EE_BUFFER);
            c.g[A0][0] = c.g[S4][0];
            c.g[V0][0] = dirty_rpc_lw(c, rpc, DIRTY_RPC_EE_BUFFER_END);
            c.g[V1][0] = sx(r32(c, V1).wrapping_sub(r32(c, T0)));
            c.g[V1][0] = sx((r32(c, V1) as i32 >> DIRTY_RPC_SLOT_SHIFT) as u32);
            dirty_rpc_sw(c, rpc, DIRTY_RPC_NEXT_ID, c.g[S2][0]);
            c.g[V0][0] = sx(r32(c, V0).wrapping_sub(r32(c, A3)));
            dirty_rpc_sw(c, rpc, DIRTY_RPC_SLOTS, c.g[V1][0]);
            dirty_rpc_sw(c, rpc, DIRTY_RPC_BUFFER_SIZE, c.g[V0][0]);
            dirty_rpc_sw(c, rpc, DIRTY_RPC_TAIL, c.g[S2][0]);
            dirty_rpc_sw(c, rpc, DIRTY_RPC_HEAD, c.g[S2][0]);
            MEMSET.call(c, 0x003f_5328)?;
            // The semaphore parameters at sp+0x50: max and initial count 1.
            c.g[A0][0] = c.g[S4][0];
            Addr(frame.at(c, 0x58)).sw(c, c.g[S2][0]);
            Addr(frame.at(c, 0x54)).sw(c, c.g[S2][0]);
            Addr(frame.at(c, 0x64)).sw(c, 0);
            CREATE_SEMA.call(c, 0x003f_533c)?;
            // The thread parameters at sp+0x70.
            let rpc = r32(c, S1);
            dirty_rpc_sw(c, rpc, DIRTY_RPC_SEMA, c.g[V0][0]);
            c.g[T0][0] = u64::MAX;
            c.g[V1][0] = sx(DIRTY_RPC_THREAD_MAIN);
            c.g[A1][0] = sx(DIRTY_RPC_THREAD_STACK);
            dirty_rpc_sw(c, rpc, DIRTY_RPC_HANDLER_ALARM, c.g[T0][0]);
            c.g[A2][0] = DIRTY_RPC_THREAD_PRIORITY;
            c.g[A3][0] = DIRTY_RPC_THREAD_STACK_SIZE;
            c.g[V0][0] = sx(DIRTY_RPC_THREAD_GP);
            Addr(frame.at(c, 0x74)).sw(c, c.g[V1][0]);
            c.g[A0][0] = c.g[S5][0];
            Addr(frame.at(c, 0x84)).sw(c, c.g[A2][0]);
            Addr(frame.at(c, 0x78)).sw(c, c.g[A1][0]);
            Addr(frame.at(c, 0x7c)).sw(c, c.g[A3][0]);
            Addr(frame.at(c, 0x80)).sw(c, c.g[V0][0]);
            dirty_rpc_sw(c, rpc, DIRTY_RPC_ALARM, c.g[T0][0]);
            Addr(frame.at(c, 0x90)).sw(c, 0);
            CREATE_THREAD.call(c, 0x003f_538c)?;
            let rpc = r32(c, S1);
            let failed = (c.g[V0][0] as i64) <= 0;
            dirty_rpc_sw(c, rpc, DIRTY_RPC_THREAD, c.g[V0][0]);
            if failed {
                c.cycles += 1;
                c.g[V0][0] = (-5i64) as u64;
                break 'out;
            }
            c.cycles += 15;
            c.g[A1][0] = sx(DIRTY_RPC_INTC_HANDLER_FN);
            c.g[A2][0] = u64::MAX;
            c.g[A3][0] = c.g[S1][0];
            c.g[A0][0] = DIRTY_RPC_INTC_SIF;
            ADD_INTC_HANDLER.call(c, 0x003f_53ac)?;
            c.g[A0][0] = DIRTY_RPC_INTC_SIF;
            dirty_rpc_sw(c, r32(c, S1), DIRTY_RPC_INTC_HANDLER, c.g[V0][0]);
            ENABLE_INTC.call(c, 0x003f_53b8)?;
            c.g[A0][0] = dirty_rpc_lw(c, r32(c, S1), DIRTY_RPC_THREAD);
            c.g[A1][0] = c.g[S1][0];
            START_THREAD.call(c, 0x003f_53c4)?;
            dirty_rpc_sw(c, r32(c, GP), DIRTY_RPC_STARTED_GP, c.g[S2][0]);
            c.g[V0][0] = 0;
        }
        c.cycles += 9;
        frame.restore(c, S0, 0x100);
        frame.restore(c, S1, 0xf0);
        frame.restore(c, S2, 0xe0);
        frame.restore(c, S3, 0xd0);
        frame.restore(c, S4, 0xc0);
        frame.restore(c, S5, 0xb0);
        frame.restore_ra(c, 0xa0);
        frame.close(c);
        Ok(())
    })
}

/// `int DirtyRpc_Shutdown(void)` (0x3f53f8): release the two alarms that
/// are set (ids >= 0), remove the SIF interrupt handler, terminate and delete
/// the transport thread, delete the ring semaphore and clear the started
/// flag. Returns 0.
pub fn dirty_rpc_shutdown(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 0x52_0000;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = sx(DIRTY_RPC_STATE);
        c.g[A0][0] = dirty_rpc_lw(c, r32(c, S0), DIRTY_RPC_ALARM);
        if (c.g[A0][0] as i64) >= 0 {
            c.cycles += 3;
            RELEASE_ALARM.call(c, 0x003f_5420)?;
        }
        c.g[A0][0] = dirty_rpc_lw(c, r32(c, S0), DIRTY_RPC_HANDLER_ALARM);
        c.cycles += 2;
        if (c.g[A0][0] as i64) >= 0 {
            c.cycles += 3;
            RELEASE_ALARM.call(c, 0x003f_5434)?;
        }
        c.g[A1][0] = dirty_rpc_lw(c, r32(c, S0), DIRTY_RPC_INTC_HANDLER);
        c.cycles += 14;
        c.g[A0][0] = DIRTY_RPC_INTC_SIF;
        REMOVE_INTC_HANDLER.call(c, 0x003f_5440)?;
        c.g[A0][0] = dirty_rpc_lw(c, r32(c, S0), DIRTY_RPC_THREAD);
        TERMINATE_THREAD.call(c, 0x003f_5448)?;
        c.g[A0][0] = dirty_rpc_lw(c, r32(c, S0), DIRTY_RPC_THREAD);
        DELETE_THREAD.call(c, 0x003f_5450)?;
        c.g[A0][0] = dirty_rpc_lw(c, r32(c, S0), DIRTY_RPC_SEMA);
        DELETE_SEMA.call(c, 0x003f_5458)?;
        frame.restore(c, S0, 0x10);
        c.g[V0][0] = 0;
        frame.restore_ra(c, 0);
        dirty_rpc_sw(c, r32(c, GP), DIRTY_RPC_STARTED_GP, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int DirtyRpc_IsStarted(void)` (0x3f5470): the started flag
/// (`gp+0x1ac0`).
pub fn dirty_rpc_is_started(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = dirty_rpc_lw(c, r32(c, GP), DIRTY_RPC_STARTED_GP);
    ret(c);
}

/// The data buffer's write offset (`DirtyRpc_Send` moves it past the data).
pub(crate) const DIRTY_RPC_WRITE: u32 = 0x4ddc;

/// `gp+0x2a18`: 2 while a sender sleeps until its reply, else 0.
pub(crate) const DIRTY_RPC_SLEEP_GP: u32 = 0x2a18;

/// Send flag 1: after the DMA, send SIF command 1 with the packet at
/// [`DIRTY_RPC_SIF_PACKET`].
pub(crate) const DIRTY_RPC_SEND_SIF_CMD: u64 = 1;

/// Send flag 2: the data is a gather list of `{int len; void* ptr}` pairs
/// ended by a `len <= 0`.
pub(crate) const DIRTY_RPC_SEND_GATHER: u64 = 2;

/// Send flag 4: `SyncDCache` the reply buffer before the DMA.
pub(crate) const DIRTY_RPC_SEND_SYNC_REPLY: u64 = 4;

/// The SIF command packet of flag 1 (its first byte is its size).
pub(crate) const DIRTY_RPC_SIF_PACKET: u32 = 0x0045_1300;

pub(crate) const DIRTY_RPC_ALARM_TICKS: u64 = 8;

/// `DirtyRpc_Send`'s entry and the end of its code.
pub(crate) const DIRTY_RPC_SEND_ENTRY: u32 = callee::DIRTY_RPC_SEND.addr;

pub(crate) const DIRTY_RPC_SEND_END: u32 = 0x003f_4a78;

/// Its back-edge loop heads, where an unwound call resumes: summing the gather
/// list, waiting for a free ring slot, copying the gather list, retrying the
/// DMA, spinning until the next millisecond.
pub(crate) const DIRTY_RPC_SEND_SUM_HEAD: u32 = 0x003f_45e0;

pub(crate) const DIRTY_RPC_SEND_SLOT_HEAD: u32 = 0x003f_4648;

pub(crate) const DIRTY_RPC_SEND_COPY_HEAD: u32 = 0x003f_4778;

pub(crate) const DIRTY_RPC_SEND_RETRY_HEAD: u32 = 0x003f_48f8;

pub(crate) const DIRTY_RPC_SEND_TICK_HEAD: u32 = 0x003f_4908;

/// Its stack frame (the same at every resume point).
pub(crate) const DIRTY_RPC_SEND_FRAME: Frame = Frame { size: 0xe0 };

/// Frame slots: the two `sceSifSetDma` transfers `{src, dst, size, attr}`
/// (the data, then the descriptor), the caller's a2 and a3, the SIF-command
/// and sync flags, `len + 0x3f`; above the frame the caller's stack arguments
/// (the mode word, the context word).
pub(crate) const DIRTY_RPC_SEND_DMA_DATA: u32 = 0x00;

pub(crate) const DIRTY_RPC_SEND_DMA_REQUEST: u32 = 0x10;

pub(crate) const DIRTY_RPC_SEND_WORD2: u32 = 0x20;

pub(crate) const DIRTY_RPC_SEND_WORD3: u32 = 0x24;

pub(crate) const DIRTY_RPC_SEND_SIF_FLAG: u32 = 0x28;

pub(crate) const DIRTY_RPC_SEND_SYNC_FLAG: u32 = 0x2c;

pub(crate) const DIRTY_RPC_SEND_PADDED: u32 = 0x30;

pub(crate) const DIRTY_RPC_SEND_MODE: u32 = 0xe0;

pub(crate) const DIRTY_RPC_SEND_CONTEXT: u32 = 0xe8;

/// Where `DirtyRpc_Send` goes on after a stage (the entry and each resume
/// head start one).
#[derive(Clone, Copy, PartialEq, Debug)]
pub(crate) enum DirtyRpcSendStage {
    /// 0x3f45e0 (loop head): add up the gather list's lengths (a0 walks it).
    SumGather,
    /// 0x3f4600: take the semaphore (a0) and look for a free ring slot.
    Lock,
    /// 0x3f4648 (loop head): the ring is full; process replies until a slot
    /// frees up.
    WaitForSlot,
    /// 0x3f468c: find room for the data from the write offset (a0).
    Reserve,
    /// 0x3f4778 (loop head): copy the gather piece at s0 to s1.
    CopyGather,
    /// 0x3f47b8: fill the request descriptor (v1 = the head slot), queue the
    /// DMA.
    Describe,
    /// 0x3f48f8 (loop head): the DMA got no id; wait for the next tick.
    DmaRetry,
    /// 0x3f4908 (loop head): spin until `NetTick` changes.
    TickWait,
    /// 0x3f496c: the SIF command (v1 = its flag), advance the ring, block if
    /// asked.
    Finish,
}

pub(crate) type DirtyRpcSendStep = Result<Resume<DirtyRpcSendStage>, Unwound>;

/// A field of the DirtyRpc state block, addressed through `s2` as the
/// compiled code does.
#[inline]
pub(crate) fn dirty_rpc_send_state(c: &Cpu, off: u32) -> Addr {
    Addr(r32(c, S2).wrapping_add(off))
}

/// A slot of `DirtyRpc_Send`'s frame.
#[inline]
pub(crate) fn dirty_rpc_send_slot(c: &Cpu, off: u32) -> Addr {
    Addr(DIRTY_RPC_SEND_FRAME.at(c, off))
}

/// A gp-relative global.
#[inline]
pub(crate) fn dirty_rpc_send_gp(c: &Cpu, off: u32) -> Addr {
    Addr(r32(c, GP).wrapping_add(off))
}

/// The head slot's entry of the per-slot table at state field `table`
/// (`dst = table + (head << shift)`), reloading the head into `dst` and the
/// table base into `tmp` as the compiled code does before every field store.
pub(crate) fn dirty_rpc_send_head_entry(
    c: &mut Cpu,
    dst: usize,
    tmp: usize,
    table: u32,
    shift: u32,
) -> Addr {
    c.g[dst][0] = dirty_rpc_send_state(c, DIRTY_RPC_HEAD).lw(c);
    c.g[tmp][0] = dirty_rpc_send_state(c, table).lw(c);
    c.g[dst][0] = sx(r32(c, dst) << shift);
    c.g[dst][0] = sx(r32(c, dst).wrapping_add(r32(c, tmp)));
    Addr(r32(c, dst))
}

/// `int DirtyRpc_Send(client, flags, w2, w3, data, len, reply, reply_size,
/// mode, ctx)` (0x3f4528): queue an RPC request for the IOP; returns its
/// request id, or -1.
///
/// -1 at once when `client` is null, its word at +0xc is 0, or `reply` is not
/// 16-byte aligned (`reply_size` is rounded down to 16). The data length is
/// `len`, or `strlen(data) + 1` when `len < 0`; with flag 2 `data` is a gather
/// list and the length is the sum of its pieces. Under the ring semaphore it
/// waits for a free ring slot (releasing the semaphore and running
/// `DirtyRpc_ProcessReplies` meanwhile) and looks for room in the data buffer:
/// before the read offset, at the write offset up to the end, or else from 0
/// (each less a 0x40 guard); no room releases the semaphore and returns -1.
/// It fills the slot's waiter entry (`mode`, `ctx`, `flags`, the thread id),
/// copies the data, fills the request descriptor, `SyncDCache`s the reply
/// buffer (flag 4) and DMAs the data and the descriptor to the IOP (just the
/// descriptor when there is no data), waiting a `NetTick` millisecond and
/// retrying while `sceSifSetDma` returns 0. Flag 1 then sends SIF command 1.
/// It moves the write offset past the data (64-byte aligned), advances the
/// request id and the ring head, and releases the semaphore. `mode == 1`
/// waits for the reply: it starts the poll alarm if none runs and sleeps.
pub fn dirty_rpc_send(c: &mut Cpu) {
    use DirtyRpcSendStage as Stage;
    let mut step = match c.pc {
        DIRTY_RPC_SEND_ENTRY => dirty_rpc_send_enter(c),
        DIRTY_RPC_SEND_SUM_HEAD => Ok(Resume::Go(Stage::SumGather)),
        DIRTY_RPC_SEND_SLOT_HEAD => Ok(Resume::Go(Stage::WaitForSlot)),
        DIRTY_RPC_SEND_COPY_HEAD => Ok(Resume::Go(Stage::CopyGather)),
        DIRTY_RPC_SEND_RETRY_HEAD => Ok(Resume::Go(Stage::DmaRetry)),
        DIRTY_RPC_SEND_TICK_HEAD => Ok(Resume::Go(Stage::TickWait)),
        pc => {
            c.jump_out(pc, DIRTY_RPC_SEND_ENTRY, DIRTY_RPC_SEND_END);
            return;
        }
    };
    while let Ok(Resume::Go(stage)) = step {
        step = match stage {
            Stage::SumGather => dirty_rpc_send_sum_gather(c),
            Stage::Lock => dirty_rpc_send_lock(c),
            Stage::WaitForSlot => dirty_rpc_send_wait_for_slot(c),
            Stage::Reserve => dirty_rpc_send_reserve(c),
            Stage::CopyGather => dirty_rpc_send_copy_gather(c),
            Stage::Describe => dirty_rpc_send_describe(c),
            Stage::DmaRetry => dirty_rpc_send_dma_retry(c),
            Stage::TickWait => dirty_rpc_send_tick_wait(c),
            Stage::Finish => dirty_rpc_send_finish(c),
        };
    }
}

/// The epilogue (0x3f4a48).
pub(crate) fn dirty_rpc_send_leave(c: &mut Cpu) -> Resume<DirtyRpcSendStage> {
    let frame = DIRTY_RPC_SEND_FRAME;
    c.cycles += 12;
    frame.restore(c, S0, 0xd0);
    frame.restore(c, S1, 0xc0);
    frame.restore(c, S2, 0xb0);
    frame.restore(c, S3, 0xa0);
    frame.restore(c, S4, 0x90);
    frame.restore(c, S5, 0x80);
    frame.restore(c, S6, 0x70);
    frame.restore(c, S7, 0x60);
    frame.restore(c, FP, 0x50);
    frame.restore_ra(c, 0x40);
    frame.close(c);
    Resume::Done
}

/// The entry (0x3f4528): the frame, the argument checks, the length.
pub(crate) fn dirty_rpc_send_enter(c: &mut Cpu) -> DirtyRpcSendStep {
    use DirtyRpcSendStage as Stage;
    c.cycles += 22;
    let frame = Frame::open(c, DIRTY_RPC_SEND_FRAME.size);
    c.g[V0][0] = 0x52_0000;
    frame.save(c, S1, 0xc0);
    frame.save(c, S2, 0xb0);
    frame.save(c, S3, 0xa0);
    frame.save(c, S5, 0x80);
    frame.save(c, S6, 0x70);
    frame.save(c, S7, 0x60);
    frame.save(c, FP, 0x50);
    frame.save(c, S0, 0xd0);
    frame.save(c, S4, 0x90);
    frame.save_ra(c, 0x40);
    c.g[S1][0] = c.g[A1][0]; // flags
    c.g[S2][0] = sx(DIRTY_RPC_STATE);
    c.g[S3][0] = c.g[T1][0]; // len
    c.g[S5][0] = c.g[T0][0]; // data
    c.g[S6][0] = c.g[T3][0]; // reply size
    c.g[S7][0] = c.g[A0][0]; // client
    c.g[FP][0] = c.g[T2][0]; // reply buffer
    dirty_rpc_send_slot(c, DIRTY_RPC_SEND_WORD2).sw(c, c.g[A2][0]);
    dirty_rpc_send_slot(c, DIRTY_RPC_SEND_WORD3).sw(c, c.g[A3][0]);
    'fail: {
        if c.g[S7][0] == 0 {
            break 'fail;
        }
        c.cycles += 3;
        c.g[V0][0] = Addr(r32(c, S7).wrapping_add(DIRTY_RPC_CLIENT_ID)).lw(c);
        let unbound = c.g[V0][0] == 0;
        c.g[V1][0] = c.g[FP][0] & 0xf;
        if unbound {
            break 'fail;
        }
        c.cycles += 4;
        c.g[V0][0] = 0xffff_ffff_ffff_fff0;
        let aligned = c.g[V1][0] == 0;
        c.g[S6][0] = c.g[S6][0] & c.g[V0][0];
        if !aligned {
            break 'fail;
        }
        // The length: strlen + 1 for a negative one; the gather list's sum.
        c.cycles += 2;
        let has_len = (c.g[S3][0] as i64) >= 0;
        c.g[V0][0] = c.g[S1][0] & DIRTY_RPC_SEND_GATHER;
        if !has_len {
            c.cycles += 4;
            c.g[A0][0] = c.g[S5][0];
            callee::STRLEN.call(c, 0x003f_45b4)?;
            c.g[S3][0] = sx(r32(c, V0).wrapping_add(1));
            c.g[V0][0] = c.g[S1][0] & DIRTY_RPC_SEND_GATHER;
        }
        c.cycles += 4;
        c.g[S0][0] = if c.g[V0][0] != 0 { c.g[S5][0] } else { 0 };
        if c.g[S0][0] == 0 {
            c.g[A0][0] = dirty_rpc_send_state(c, DIRTY_RPC_SEMA).lw(c);
            return Ok(Resume::Go(Stage::Lock));
        }
        c.cycles += 3;
        c.g[V0][0] = Addr(r32(c, S0)).lw(c);
        let empty = (c.g[V0][0] as i64) <= 0;
        c.g[S3][0] = 0;
        if empty {
            c.cycles += 1;
            c.g[A0][0] = dirty_rpc_send_state(c, DIRTY_RPC_SEMA).lw(c);
            return Ok(Resume::Go(Stage::Lock));
        }
        c.cycles += 2;
        c.g[A0][0] = c.g[S0][0];
        return Ok(Resume::Go(Stage::SumGather));
    }
    c.cycles += 2;
    c.g[V0][0] = u64::MAX;
    Ok(dirty_rpc_send_leave(c))
}

/// 0x3f45e0: add up the gather pieces' lengths into s3 until a `len <= 0`.
pub(crate) fn dirty_rpc_send_sum_gather(c: &mut Cpu) -> DirtyRpcSendStep {
    loop {
        c.cycles += 7;
        c.g[V1][0] = Addr(r32(c, A0)).lw(c);
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(8));
        c.g[V0][0] = Addr(r32(c, A0)).lw(c);
        let more = (c.g[V0][0] as i64) > 0;
        c.g[S3][0] = sx(r32(c, S3).wrapping_add(r32(c, V1)));
        if !more {
            break;
        }
        if back_edge_unwinds(c, DIRTY_RPC_SEND_SUM_HEAD) {
            return Ok(Resume::Unwind);
        }
    }
    c.cycles += 1;
    c.g[A0][0] = dirty_rpc_send_state(c, DIRTY_RPC_SEMA).lw(c);
    Ok(Resume::Go(DirtyRpcSendStage::Lock))
}

/// 0x3f4600: `WaitSema(a0)` (marked at `gp+0x2a10`), then is the slot after
/// the head the tail (the ring full)?
pub(crate) fn dirty_rpc_send_lock(c: &mut Cpu) -> DirtyRpcSendStep {
    use DirtyRpcSendStage as Stage;
    c.cycles += 11;
    c.g[V0][0] = 2;
    dirty_rpc_send_gp(c, DIRTY_RPC_SEMA_WAIT_GP).sw(c, c.g[V0][0]);
    callee::WAIT_SEMA.call(c, 0x003f_4610)?;
    c.g[V0][0] = dirty_rpc_send_state(c, DIRTY_RPC_HEAD).lw(c);
    c.g[A1][0] = dirty_rpc_send_state(c, DIRTY_RPC_SLOTS).lw(c);
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(1));
    dirty_rpc_send_gp(c, DIRTY_RPC_SEMA_WAIT_GP).sw(c, 0);
    c.div(0, V0, A1);
    c.cycles += 4;
    c.g[V1][0] = dirty_rpc_send_state(c, DIRTY_RPC_TAIL).lw(c);
    c.g[A0][0] = c.hi[0];
    if c.g[A0][0] != c.g[V1][0] {
        c.g[A0][0] = dirty_rpc_send_state(c, DIRTY_RPC_WRITE).lw(c);
        return Ok(Resume::Go(Stage::Reserve));
    }
    c.cycles += 3;
    Ok(Resume::Go(Stage::WaitForSlot))
}

/// 0x3f4648: while the ring is full, release the semaphore, process replies
/// and take it again.
pub(crate) fn dirty_rpc_send_wait_for_slot(c: &mut Cpu) -> DirtyRpcSendStep {
    loop {
        c.cycles += 12;
        c.g[A0][0] = dirty_rpc_send_state(c, DIRTY_RPC_SEMA).lw(c);
        callee::SIGNAL_SEMA.call(c, 0x003f_4650)?;
        callee::DIRTY_RPC_PROCESS_REPLIES.call(c, 0x003f_4658)?;
        c.g[A0][0] = dirty_rpc_send_state(c, DIRTY_RPC_SEMA).lw(c);
        callee::WAIT_SEMA.call(c, 0x003f_4660)?;
        c.g[V1][0] = dirty_rpc_send_state(c, DIRTY_RPC_HEAD).lw(c);
        c.g[V0][0] = dirty_rpc_send_state(c, DIRTY_RPC_SLOTS).lw(c);
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(1));
        c.div(0, V1, V0);
        c.cycles += 4;
        c.g[V1][0] = dirty_rpc_send_state(c, DIRTY_RPC_TAIL).lw(c);
        c.g[A0][0] = c.hi[0];
        if c.g[A0][0] != c.g[V1][0] {
            break;
        }
        if back_edge_unwinds(c, DIRTY_RPC_SEND_SLOT_HEAD) {
            return Ok(Resume::Unwind);
        }
    }
    c.cycles += 1;
    c.g[A0][0] = dirty_rpc_send_state(c, DIRTY_RPC_WRITE).lw(c);
    Ok(Resume::Go(DirtyRpcSendStage::Reserve))
}

/// 0x3f468c: the data's offset (s4) and the room there; then claim the slot.
pub(crate) fn dirty_rpc_send_reserve(c: &mut Cpu) -> DirtyRpcSendStep {
    c.cycles += 4;
    c.g[A1][0] = dirty_rpc_send_state(c, DIRTY_RPC_READ).lw(c);
    let behind = (c.g[A0][0] as i64) < (c.g[A1][0] as i64);
    c.g[V0][0] = behind as u64;
    c.g[S4][0] = c.g[A0][0];
    if behind {
        // Up to the read offset.
        c.cycles += 3;
        c.g[V0][0] = sx(r32(c, A1).wrapping_sub(r32(c, S4)));
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0x40));
    } else {
        // Up to the end, or else from 0 up to the read offset.
        c.cycles += 6;
        c.g[V1][0] = dirty_rpc_send_state(c, DIRTY_RPC_BUFFER_SIZE).lw(c);
        c.g[V1][0] = sx(r32(c, V1).wrapping_sub(r32(c, A0)));
        c.g[V0][0] = sx(r32(c, V1).wrapping_sub(0x40));
        let wrap = (c.g[V0][0] as i64) < (c.g[S3][0] as i64);
        c.g[V0][0] = sx(r32(c, A1).wrapping_sub(0x40));
        if wrap {
            c.cycles += 1;
            c.g[S4][0] = 0;
        } else {
            c.cycles += 3;
            c.g[S4][0] = c.g[A0][0];
            c.g[V0][0] = c.g[V1][0];
        }
    }
    c.cycles += 3;
    let short = (c.g[V0][0] as i64) < (c.g[S3][0] as i64);
    c.g[V0][0] = short as u64;
    if short {
        c.cycles += 4;
        c.g[A0][0] = dirty_rpc_send_state(c, DIRTY_RPC_SEMA).lw(c);
        callee::SIGNAL_SEMA.call(c, 0x003f_46e4)?;
        c.g[V0][0] = u64::MAX;
        return Ok(dirty_rpc_send_leave(c));
    }
    c.g[V0][0] = dirty_rpc_send_state(c, DIRTY_RPC_HEAD).lw(c);
    dirty_rpc_send_claim(c)
}

/// 0x3f46ec: the head slot's waiter entry (v0 = the head), then the data copy.
pub(crate) fn dirty_rpc_send_claim(c: &mut Cpu) -> DirtyRpcSendStep {
    use DirtyRpcSendStage as Stage;
    c.cycles += 29;
    c.g[V1][0] = c.g[S1][0] & DIRTY_RPC_SEND_SIF_CMD;
    dirty_rpc_send_slot(c, DIRTY_RPC_SEND_SIF_FLAG).sw(c, c.g[V1][0]);
    c.g[A0][0] = c.g[S1][0] & DIRTY_RPC_SEND_SYNC_REPLY;
    c.g[V1][0] = dirty_rpc_send_state(c, DIRTY_RPC_WAITERS).lw(c);
    c.g[V0][0] = sx(r32(c, V0) << 4);
    dirty_rpc_send_slot(c, DIRTY_RPC_SEND_SYNC_FLAG).sw(c, c.g[A0][0]);
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
    c.g[A0][0] = sx(r32(c, S3).wrapping_add(0x3f));
    c.g[V1][0] = dirty_rpc_send_slot(c, DIRTY_RPC_SEND_MODE).lw(c);
    dirty_rpc_send_slot(c, DIRTY_RPC_SEND_PADDED).sw(c, c.g[A0][0]);
    Addr(r32(c, V0)).sw(c, c.g[V1][0]); // +0 the mode word
    let waiter = dirty_rpc_send_head_entry(c, V1, V0, DIRTY_RPC_WAITERS, 4);
    c.g[A0][0] = dirty_rpc_send_slot(c, DIRTY_RPC_SEND_CONTEXT).lw(c);
    waiter.off(4).sw(c, c.g[A0][0]); // +4 the context word
    let waiter = dirty_rpc_send_head_entry(c, V0, V1, DIRTY_RPC_WAITERS, 4);
    waiter.off(8).sw(c, c.g[S1][0]); // +8 the flags
    callee::GET_THREAD_ID.call(c, 0x003f_4748)?;
    let waiter = dirty_rpc_send_head_entry(c, V1, A0, DIRTY_RPC_WAITERS, 4);
    let no_data = (c.g[S3][0] as i64) <= 0;
    waiter.off(0xc).sw(c, c.g[V0][0]); // +0xc the thread id
    if !no_data {
        c.cycles += 2;
        if c.g[S0][0] == 0 {
            // One block.
            c.g[A0][0] = dirty_rpc_send_state(c, DIRTY_RPC_EE_BUFFER).lw(c);
            c.cycles += 4;
            c.g[A1][0] = c.g[S5][0];
            c.g[A2][0] = c.g[S3][0];
            c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, S4)));
            callee::MEMCPY.call(c, 0x003f_47b4)?;
        } else {
            // The gather pieces, one after the other from EE buffer + s4.
            c.cycles += 4;
            c.g[V0][0] = dirty_rpc_send_state(c, DIRTY_RPC_EE_BUFFER).lw(c);
            c.g[V1][0] = Addr(r32(c, S0)).lw(c);
            let empty = (c.g[V1][0] as i64) <= 0;
            c.g[S1][0] = sx(r32(c, V0).wrapping_add(r32(c, S4)));
            if !empty {
                return Ok(Resume::Go(Stage::CopyGather));
            }
        }
    }
    c.cycles += 1;
    c.g[V1][0] = dirty_rpc_send_state(c, DIRTY_RPC_HEAD).lw(c);
    Ok(Resume::Go(Stage::Describe))
}

/// 0x3f4778: `memcpy(s1, piece.ptr, piece.len)` for each gather piece at s0.
pub(crate) fn dirty_rpc_send_copy_gather(c: &mut Cpu) -> DirtyRpcSendStep {
    loop {
        c.cycles += 9;
        c.g[A1][0] = Addr(r32(c, S0).wrapping_add(4)).lw(c);
        c.g[A0][0] = c.g[S1][0];
        c.g[A2][0] = Addr(r32(c, S0)).lw(c);
        callee::MEMCPY.call(c, 0x003f_4788)?;
        c.g[V1][0] = Addr(r32(c, S0)).lw(c);
        c.g[S0][0] = sx(r32(c, S0).wrapping_add(8));
        c.g[V0][0] = Addr(r32(c, S0)).lw(c);
        let more = (c.g[V0][0] as i64) > 0;
        c.g[S1][0] = sx(r32(c, S1).wrapping_add(r32(c, V1)));
        if !more {
            break;
        }
        if back_edge_unwinds(c, DIRTY_RPC_SEND_COPY_HEAD) {
            return Ok(Resume::Unwind);
        }
    }
    c.cycles += 2;
    c.g[V1][0] = dirty_rpc_send_state(c, DIRTY_RPC_HEAD).lw(c);
    Ok(Resume::Go(DirtyRpcSendStage::Describe))
}

/// 0x3f47b8: the head slot's request descriptor (v1 = the head), the reply
/// buffer sync, the transfer list, the first DMA.
pub(crate) fn dirty_rpc_send_describe(c: &mut Cpu) -> DirtyRpcSendStep {
    c.cycles += 48;
    c.g[V0][0] = dirty_rpc_send_state(c, DIRTY_RPC_REQUESTS).lw(c);
    c.g[A0][0] = dirty_rpc_send_state(c, DIRTY_RPC_IOP_BUFFER).lw(c);
    c.g[V1][0] = sx(r32(c, V1) << 5);
    c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
    c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, S4)));
    Addr(r32(c, V1)).sw(c, c.g[A0][0]); // +0 the data's IOP address
    let req = dirty_rpc_send_head_entry(c, V0, V1, DIRTY_RPC_REQUESTS, 5);
    req.off(4).sw(c, c.g[S3][0]); // +4 the length
    let req = dirty_rpc_send_head_entry(c, V1, V0, DIRTY_RPC_REQUESTS, 5);
    req.off(8).sw(c, c.g[FP][0]); // +8 the reply buffer
    let req = dirty_rpc_send_head_entry(c, V0, V1, DIRTY_RPC_REQUESTS, 5);
    req.off(0xc).sw(c, c.g[S6][0]); // +0xc the reply size
    let req = dirty_rpc_send_head_entry(c, V1, V0, DIRTY_RPC_REQUESTS, 5);
    c.g[A0][0] = dirty_rpc_send_slot(c, DIRTY_RPC_SEND_WORD2).lw(c);
    req.off(0x10).sw(c, c.g[A0][0]);
    let req = dirty_rpc_send_head_entry(c, V0, V1, DIRTY_RPC_REQUESTS, 5);
    c.g[V1][0] = dirty_rpc_send_slot(c, DIRTY_RPC_SEND_WORD3).lw(c);
    req.off(0x14).sw(c, c.g[V1][0]);
    let req = dirty_rpc_send_head_entry(c, V1, V0, DIRTY_RPC_REQUESTS, 5);
    c.g[A0][0] = Addr(r32(c, S7).wrapping_add(DIRTY_RPC_CLIENT_ID)).lw(c);
    req.off(0x18).sw(c, c.g[A0][0]);
    let req = dirty_rpc_send_head_entry(c, V0, A0, DIRTY_RPC_REQUESTS, 5);
    c.g[V1][0] = dirty_rpc_send_state(c, DIRTY_RPC_NEXT_ID).lw(c);
    req.off(0x1c).sw(c, c.g[V1][0]); // +0x1c the request id
    c.g[A0][0] = dirty_rpc_send_slot(c, DIRTY_RPC_SEND_SYNC_FLAG).lw(c);
    let sync = c.g[A0][0] != 0;
    c.g[A1][0] = sx(r32(c, S6).wrapping_add(0x80));
    if sync {
        c.cycles += 3;
        c.g[A0][0] = c.g[FP][0];
        c.g[A1][0] = sx(r32(c, A0).wrapping_add(r32(c, A1)));
        callee::SYNC_DCACHE.call(c, 0x003f_4884)?;
    }
    // The transfers: the data (64-byte padded) and the descriptor.
    c.cycles += 26;
    c.g[V0][0] = dirty_rpc_send_slot(c, DIRTY_RPC_SEND_PADDED).lw(c);
    c.g[T0][0] = 0xffff_ffff_ffff_ffc0;
    c.g[V1][0] = dirty_rpc_send_state(c, DIRTY_RPC_HEAD).lw(c);
    c.g[T0][0] = c.g[V0][0] & c.g[T0][0];
    c.g[A0][0] = dirty_rpc_send_state(c, DIRTY_RPC_REQUESTS).lw(c);
    c.g[V0][0] = dirty_rpc_send_state(c, DIRTY_RPC_IOP_REQUESTS).lw(c);
    c.g[T1][0] = sx(r32(c, V1) << 5);
    c.g[A2][0] = dirty_rpc_send_state(c, DIRTY_RPC_EE_BUFFER).lw(c);
    c.g[A0][0] = sx(r32(c, A0).wrapping_add(r32(c, T1)));
    c.g[A1][0] = dirty_rpc_send_state(c, DIRTY_RPC_IOP_BUFFER).lw(c);
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, T1)));
    c.g[A3][0] = dirty_rpc_send_state(c, DIRTY_RPC_DMA_IDS).lw(c);
    c.g[A2][0] = sx(r32(c, A2).wrapping_add(r32(c, S4)));
    c.g[A1][0] = sx(r32(c, A1).wrapping_add(r32(c, S4)));
    c.g[V1][0] = sx(r32(c, V1) << 2);
    c.g[T1][0] = 0x20;
    c.g[S0][0] = sx(r32(c, V1).wrapping_add(r32(c, A3))); // the slot's DMA id
    let data = DIRTY_RPC_SEND_DMA_DATA;
    let request = DIRTY_RPC_SEND_DMA_REQUEST;
    dirty_rpc_send_slot(c, data).sw(c, c.g[A2][0]);
    dirty_rpc_send_slot(c, data + 4).sw(c, c.g[A1][0]);
    dirty_rpc_send_slot(c, request).sw(c, c.g[A0][0]);
    dirty_rpc_send_slot(c, request + 4).sw(c, c.g[V0][0]);
    dirty_rpc_send_slot(c, request + 8).sw(c, c.g[T1][0]);
    dirty_rpc_send_slot(c, data + 8).sw(c, c.g[T0][0]);
    dirty_rpc_send_slot(c, data + 0xc).sw(c, 0);
    let with_data = c.g[T0][0] != 0;
    dirty_rpc_send_slot(c, request + 0xc).sw(c, 0);
    if !with_data {
        c.cycles += 2;
        c.g[A0][0] = sx(DIRTY_RPC_SEND_FRAME.at(c, request));
    }
    dirty_rpc_send_dma(c, with_data)
}

/// 0x3f492c / 0x3f4944: `sceSifSetDma` of both transfers (from the frame) or
/// of the descriptor alone (a0), the id into the slot (s0); a 0 id retries
/// after the next tick.
pub(crate) fn dirty_rpc_send_dma(c: &mut Cpu, with_data: bool) -> DirtyRpcSendStep {
    use DirtyRpcSendStage as Stage;
    if with_data {
        c.cycles += 5;
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] = 2;
        callee::SCE_SIF_SET_DMA.call(c, 0x003f_4938)?;
    } else {
        c.cycles += 3;
        c.g[A1][0] = 1;
        callee::SCE_SIF_SET_DMA.call(c, 0x003f_494c)?;
    }
    Addr(r32(c, S0)).sw(c, c.g[V0][0]);
    c.cycles += 7;
    let id = dirty_rpc_send_head_entry(c, V0, A0, DIRTY_RPC_DMA_IDS, 2);
    c.g[V1][0] = id.lw(c);
    let queued = c.g[V1][0] != 0;
    c.g[V1][0] = dirty_rpc_send_slot(c, DIRTY_RPC_SEND_SIF_FLAG).lw(c);
    if queued {
        return Ok(Resume::Go(Stage::Finish));
    }
    if back_edge_unwinds(c, DIRTY_RPC_SEND_RETRY_HEAD) {
        return Ok(Resume::Unwind);
    }
    Ok(Resume::Go(Stage::DmaRetry))
}

/// 0x3f48f8: note the current `NetTick` (s0), then wait for it to change.
pub(crate) fn dirty_rpc_send_dma_retry(c: &mut Cpu) -> DirtyRpcSendStep {
    c.cycles += 4;
    callee::NET_TICK.call(c, 0x003f_4900)?;
    c.g[S0][0] = c.g[V0][0];
    dirty_rpc_send_tick_wait(c)
}

/// 0x3f4908: spin on `NetTick` until it differs from s0, then queue the DMA
/// again (both transfers if there is data).
pub(crate) fn dirty_rpc_send_tick_wait(c: &mut Cpu) -> DirtyRpcSendStep {
    loop {
        c.cycles += 4;
        callee::NET_TICK.call(c, 0x003f_4910)?;
        let same = c.g[S0][0] == c.g[V0][0];
        c.g[V1][0] = dirty_rpc_send_slot(c, DIRTY_RPC_SEND_DMA_DATA + 8).lw(c);
        if !same {
            break;
        }
        if back_edge_unwinds(c, DIRTY_RPC_SEND_TICK_HEAD) {
            return Ok(Resume::Unwind);
        }
    }
    c.cycles += 5;
    c.g[V0][0] = dirty_rpc_send_state(c, DIRTY_RPC_HEAD).lw(c);
    c.g[A0][0] = dirty_rpc_send_state(c, DIRTY_RPC_DMA_IDS).lw(c);
    c.g[V0][0] = sx(r32(c, V0) << 2);
    let with_data = c.g[V1][0] != 0;
    c.g[S0][0] = sx(r32(c, V0).wrapping_add(r32(c, A0)));
    if !with_data {
        c.cycles += 1;
        c.g[A0][0] = sx(DIRTY_RPC_SEND_FRAME.at(c, DIRTY_RPC_SEND_DMA_REQUEST));
    }
    dirty_rpc_send_dma(c, with_data)
}

/// 0x3f496c: the SIF command (v1 = flag 1), the write offset, the next id
/// and ring head, `SignalSema`, then the blocking wait for mode 1.
pub(crate) fn dirty_rpc_send_finish(c: &mut Cpu) -> DirtyRpcSendStep {
    c.cycles += 2;
    let sif_cmd = c.g[V1][0] != 0;
    c.g[A1][0] = buddy_lui(DIRTY_RPC_SIF_PACKET);
    if sif_cmd {
        c.cycles += 7;
        c.g[A0][0] = 1;
        c.g[A2][0] = Addr(DIRTY_RPC_SIF_PACKET).lbu(c);
        c.g[A3][0] = 0;
        c.g[A1][0] = sx(DIRTY_RPC_SIF_PACKET);
        c.g[T0][0] = 0;
        c.g[T1][0] = 0;
        callee::SCE_SIF_SEND_CMD.call(c, 0x003f_4990)?;
    }
    c.cycles += 2;
    let has_data = (c.g[S3][0] as i64) > 0;
    c.g[S0][0] = dirty_rpc_send_state(c, DIRTY_RPC_NEXT_ID).lw(c); // the result
    if has_data {
        c.cycles += 5;
        c.g[A0][0] = dirty_rpc_send_slot(c, DIRTY_RPC_SEND_PADDED).lw(c);
        c.g[V0][0] = 0xffff_ffff_ffff_ffc0;
        c.g[V0][0] = c.g[A0][0] & c.g[V0][0];
        c.g[V0][0] = sx(r32(c, S4).wrapping_add(r32(c, V0)));
        dirty_rpc_send_state(c, DIRTY_RPC_WRITE).sw(c, c.g[V0][0]);
    }
    c.cycles += 6;
    c.g[A0][0] = dirty_rpc_send_state(c, DIRTY_RPC_NEXT_ID).lw(c);
    c.g[V1][0] = dirty_rpc_send_state(c, DIRTY_RPC_ID_LIMIT).lw(c);
    c.g[A0][0] = sx(r32(c, A0).wrapping_add(1));
    c.div(0, A0, V1);
    c.cycles += 13;
    c.g[V0][0] = dirty_rpc_send_state(c, DIRTY_RPC_HEAD).lw(c);
    c.g[V1][0] = dirty_rpc_send_state(c, DIRTY_RPC_SLOTS).lw(c);
    c.g[V0][0] = sx(r32(c, V0).wrapping_add(1));
    c.g[A2][0] = c.hi[0];
    c.div(0, V0, V1);
    dirty_rpc_send_state(c, DIRTY_RPC_NEXT_ID).sw(c, c.g[A2][0]);
    c.g[A0][0] = dirty_rpc_send_state(c, DIRTY_RPC_SEMA).lw(c);
    c.g[A1][0] = c.hi[0];
    dirty_rpc_send_state(c, DIRTY_RPC_HEAD).sw(c, c.g[A1][0]);
    callee::SIGNAL_SEMA.call(c, 0x003f_49ec)?;
    c.g[V1][0] = dirty_rpc_send_state(c, DIRTY_RPC_ALARM).lw(c);
    let alarm_running = (c.g[V1][0] as i64) >= 0;
    c.g[A0][0] = dirty_rpc_send_slot(c, DIRTY_RPC_SEND_MODE).lw(c);
    if !alarm_running {
        c.cycles += 4;
        c.g[V1][0] = dirty_rpc_send_slot(c, DIRTY_RPC_SEND_MODE).lw(c);
        c.g[V0][0] = 1;
        let blocking = c.g[V1][0] == c.g[V0][0];
        c.g[V0][0] = c.g[S0][0];
        if !blocking {
            return Ok(dirty_rpc_send_leave(c));
        }
        c.cycles += 7;
        c.g[A0][0] = DIRTY_RPC_ALARM_TICKS;
        c.g[A1][0] = sx(DIRTY_RPC_ALARM_HANDLER);
        c.g[A2][0] = c.g[S2][0];
        callee::SET_ALARM.call(c, 0x003f_4a1c)?;
        dirty_rpc_send_state(c, DIRTY_RPC_ALARM).sw(c, c.g[V0][0]);
        c.g[A0][0] = dirty_rpc_send_slot(c, DIRTY_RPC_SEND_MODE).lw(c);
    }
    c.cycles += 3;
    c.g[V0][0] = 1;
    let blocking = c.g[A0][0] == c.g[V0][0];
    c.g[V0][0] = c.g[S0][0];
    if blocking {
        c.cycles += 6;
        c.g[V0][0] = 2;
        dirty_rpc_send_gp(c, DIRTY_RPC_SLEEP_GP).sw(c, c.g[V0][0]);
        callee::SLEEP_THREAD.call(c, 0x003f_4a40)?;
        dirty_rpc_send_gp(c, DIRTY_RPC_SLEEP_GP).sw(c, 0);
        c.g[V0][0] = c.g[S0][0];
    }
    Ok(dirty_rpc_send_leave(c))
}

// ---------------------------------------------------------------------------
// ProtoAries: Aries sockets over DirtyRpc. The socket itself lives in the
// IOP's Aries server; the EE object holds its handle, an address block the
// IOP reads and writes, and a receive buffer the IOP fills with records.

/// The socket object's word 0: the IOP-side socket handle (the create
/// request's reply writes it).
pub(crate) const PROTO_ARIES_HANDLE: u32 = 0x00;

/// Pointer to the 0x20-byte address block shared with the IOP (its own
/// 64-byte cache line): the peer address at +0, the local one at +0x10.
pub(crate) const PROTO_ARIES_ADDRS: u32 = 0x04;

/// Non-zero while a receive request is outstanding.
pub(crate) const PROTO_ARIES_RECV_PENDING: u32 = 0x08;

/// Read offset into the receive buffer.
pub(crate) const PROTO_ARIES_RECV_READ: u32 = 0x0c;

/// Bytes the last receive reply put in the buffer (what `'ibuf'` reports).
pub(crate) const PROTO_ARIES_RECV_FILL: u32 = 0x10;

/// Receive buffer size and address.
pub(crate) const PROTO_ARIES_RECV_SIZE: u32 = 0x14;

pub(crate) const PROTO_ARIES_RECV_BUF: u32 = 0x18;

/// Bytes the allocation holds besides the receive buffer: the object's fields
/// (to 0x1c), up to 0x3f bytes of slack to reach a 64-byte line, and that
/// line (the address block); the receive buffer starts on the next line.
pub(crate) const PROTO_ARIES_OBJECT_SIZE: u32 = 0x9c;

/// `(self + 0x5b) & ~0x3f`: the first 64-byte line at or after `self + 0x1c`.
pub(crate) const PROTO_ARIES_ADDRS_ROUND: u32 = 0x5b;

/// The address block's size, and the receive buffer's distance from it (one
/// cache line).
pub(crate) const PROTO_ARIES_ADDRS_SIZE: u32 = 0x20;

pub(crate) const PROTO_ARIES_ADDRS_LINE: u32 = 0x40;

/// Receive buffer size limits (`ProtoAriesCreate` clamps to them).
pub(crate) const PROTO_ARIES_MIN_BUFFER: u64 = 0x1000;

pub(crate) const PROTO_ARIES_MAX_BUFFER: u64 = 0x2000;

/// The two halves of the address block, each laid out like a `sockaddr_in`:
/// family halfword, big-endian port, big-endian IPv4 address.
pub(crate) const PROTO_ARIES_PEER: u32 = 0x00;

pub(crate) const PROTO_ARIES_LOCAL: u32 = 0x10;

pub(crate) const PROTO_ARIES_ADDR_FAMILY: u32 = 0x0;

pub(crate) const PROTO_ARIES_ADDR_PORT: u32 = 0x2;

pub(crate) const PROTO_ARIES_ADDR_IP: u32 = 0x4;

/// `AF_INET`.
pub(crate) const PROTO_ARIES_AF_INET: u64 = 2;

/// A receive-buffer record: two big-endian words, the big-endian record
/// length (header included), then the payload.
pub(crate) const PROTO_ARIES_RECORD_WORD0: u32 = 0x0;

pub(crate) const PROTO_ARIES_RECORD_WORD1: u32 = 0x4;

pub(crate) const PROTO_ARIES_RECORD_LEN: u32 = 0x8;

pub(crate) const PROTO_ARIES_RECORD_HEADER: u32 = 0xc;

/// `""` in the read-only data (`ProtoAriesPeek`'s payload for an empty
/// record).
pub(crate) const PROTO_ARIES_EMPTY_STRING: u32 = 0x004a_4bb8;

/// The DirtyRpc request records, one per call site (`lui 0x52` then the low
/// half; `s0` holds the `lui` half first where a call comes between).
pub(crate) const PROTO_ARIES_REQUESTS_HI: u64 = 0x0052_0000;

pub(crate) const PROTO_ARIES_CREATE_REQUEST: u32 = 0x0052_5f00;

pub(crate) const PROTO_ARIES_DESTROY_REQUEST: u32 = 0x0052_5f10;

pub(crate) const PROTO_ARIES_CONNECT_REQUEST: u32 = 0x0052_5f20;

pub(crate) const PROTO_ARIES_UNCONNECT_REQUEST: u32 = 0x0052_5f30;

pub(crate) const PROTO_ARIES_STATUS_REQUEST: u32 = 0x0052_5f40;

pub(crate) const PROTO_ARIES_SEND_REQUEST: u32 = 0x0052_5f50;

pub(crate) const PROTO_ARIES_PEEK_REQUEST: u32 = 0x0052_5f60;

pub(crate) const PROTO_ARIES_RECV_REQUEST: u32 = 0x0052_5f70;

/// The Aries server's DirtyRpc id, `'sess'`.
pub(crate) const PROTO_ARIES_SERVER: u32 = 0x7365_7373;

/// The server's entry points: `'crea'`, `'dest'`, `'conn'`, `'disc'`,
/// `'stat'`, `'send'`, `'recv'`.
pub(crate) const PROTO_ARIES_CMD_CREATE: u32 = 0x6372_6561;

pub(crate) const PROTO_ARIES_CMD_DESTROY: u32 = 0x6465_7374;

pub(crate) const PROTO_ARIES_CMD_CONNECT: u32 = 0x636f_6e6e;

pub(crate) const PROTO_ARIES_CMD_DISCONNECT: u32 = 0x6469_7363;

pub(crate) const PROTO_ARIES_CMD_STATUS: u32 = 0x7374_6174;

pub(crate) const PROTO_ARIES_CMD_SEND: u32 = 0x7365_6e64;

pub(crate) const PROTO_ARIES_CMD_RECV: u32 = 0x7265_6376;

/// `ProtoAriesStatus` selectors: `'port'`, `'addr'` (the peer's), `'lprt'`,
/// `'ladr'` (the local ones), `'ibuf'` (buffered receive bytes).
pub(crate) const PROTO_ARIES_SEL_PORT: u32 = 0x706f_7274;

pub(crate) const PROTO_ARIES_SEL_ADDR: u32 = 0x6164_6472;

pub(crate) const PROTO_ARIES_SEL_LOCAL_ADDR: u32 = 0x6c61_6472;

pub(crate) const PROTO_ARIES_SEL_LOCAL_PORT: u32 = 0x6c70_7274;

pub(crate) const PROTO_ARIES_SEL_INPUT: u32 = 0x6962_7566;

/// `ProtoAries_RpcReplyCallback`, the receive request's completion callback.
pub(crate) const PROTO_ARIES_RPC_REPLY_CALLBACK: u32 = 0x003f_5a50;

/// The address of the socket object's field `off` (`s1` holds the object in
/// every ProtoAries function).
pub(crate) fn proto_aries_field(c: &Cpu, off: u32) -> u32 {
    r32(c, S1).wrapping_add(off)
}

/// `sra reg, n` on the low word.
pub(crate) fn proto_aries_sra(c: &Cpu, reg: usize, n: u32) -> u64 {
    sx(((c.g[reg][0] as u32) as i32 >> n) as u32)
}

/// Point `s0`/`a0` at `request` and look up the Aries server's entry point
/// `code`: `DirtyRpc_InitRequest(request, 'sess', code)`.
pub(crate) fn proto_aries_init_request(c: &mut Cpu, request: u32, code: u32, ret: u32) -> Flow {
    c.g[S0][0] = sx(request);
    c.g[A0][0] = c.g[S0][0];
    c.g[A1][0] = PROTO_ARIES_SERVER as u64;
    c.g[A2][0] = code as u64;
    callee::DIRTY_RPC_INIT_REQUEST.call(c, ret)
}

/// `ProtoAriesRefT *ProtoAriesCreate(int32_t bufsize)` (0x3f5478): allocate a
/// socket object with its address block and a receive buffer of `bufsize`
/// bytes (clamped to 0x1000..=0x2000, 64-bit compares), zero the first 0x9c
/// bytes, the address block and the receive buffer (three `memset`s), write
/// the block and the buffer back from the data cache and ask the IOP's Aries
/// server for a socket (`'crea'`, synchronous, with the buffer size). Returns
/// the object; 0 when
/// the allocation fails or the reply left no handle in word 0 (the object is
/// freed again).
pub fn proto_aries_create(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x50);
        c.g[V0][0] = PROTO_ARIES_MIN_BUFFER - 1;
        frame.save(c, S2, 0x20);
        c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[A0][0] as i64)) as u64;
        c.g[S2][0] = PROTO_ARIES_MIN_BUFFER;
        c.g[A1][0] = PROTO_ARIES_MAX_BUFFER;
        if c.g[V0][0] != 0 {
            c.g[S2][0] = c.g[A0][0];
        }
        frame.save(c, S1, 0x30);
        c.g[V1][0] = ((c.g[S2][0] as i64) < PROTO_ARIES_MAX_BUFFER as i64 + 1) as u64;
        frame.save(c, S0, 0x40);
        if c.g[V1][0] == 0 {
            c.g[S2][0] = c.g[A1][0];
        }
        frame.save_ra(c, 0x10);
        c.g[A0][0] = sx(r32(c, S2).wrapping_add(PROTO_ARIES_OBJECT_SIZE));
        callee::NET_RPC_ALLOC.call(c, 0x003f_54b0)?;
        c.g[S1][0] = c.g[V0][0];
        if c.g[S1][0] == 0 {
            c.cycles += 2;
            c.g[V0][0] = 0;
        } else {
            c.g[A0][0] = c.g[S1][0];
            c.cycles += 50;
            c.g[A1][0] = 0;
            c.g[A2][0] = PROTO_ARIES_OBJECT_SIZE as u64;
            callee::MEMSET.call(c, 0x003f_54d0)?;
            // The address block's line, then the receive buffer one line on.
            c.g[V0][0] = sx(!0x3f);
            c.g[V1][0] = sx(r32(c, S1).wrapping_add(PROTO_ARIES_ADDRS_ROUND));
            c.g[V1][0] &= c.g[V0][0];
            Addr(proto_aries_field(c, PROTO_ARIES_RECV_SIZE)).sw(c, c.g[S2][0]);
            c.g[V0][0] = sx(r32(c, V1).wrapping_add(PROTO_ARIES_ADDRS_LINE));
            c.g[A0][0] = c.g[V1][0];
            Addr(proto_aries_field(c, PROTO_ARIES_ADDRS)).sw(c, c.g[V1][0]);
            c.g[A1][0] = 0;
            c.g[A2][0] = PROTO_ARIES_ADDRS_SIZE as u64;
            Addr(proto_aries_field(c, PROTO_ARIES_RECV_BUF)).sw(c, c.g[V0][0]);
            callee::MEMSET.call(c, 0x003f_54fc)?;
            c.g[A2][0] = Addr(proto_aries_field(c, PROTO_ARIES_RECV_SIZE)).lw(c);
            c.g[A1][0] = 0;
            c.g[A0][0] = Addr(proto_aries_field(c, PROTO_ARIES_RECV_BUF)).lw(c);
            callee::MEMSET.call(c, 0x003f_550c)?;
            c.g[A0][0] = Addr(proto_aries_field(c, PROTO_ARIES_ADDRS)).lw(c);
            c.g[A1][0] = sx(r32(c, A0).wrapping_add(PROTO_ARIES_ADDRS_SIZE));
            callee::SYNC_DCACHE.call(c, 0x003f_5518)?;
            c.g[A0][0] = Addr(proto_aries_field(c, PROTO_ARIES_RECV_BUF)).lw(c);
            c.g[A1][0] = Addr(proto_aries_field(c, PROTO_ARIES_RECV_SIZE)).lw(c);
            c.g[A1][0] = sx(r32(c, A0).wrapping_add(r32(c, A1)));
            callee::SYNC_DCACHE.call(c, 0x003f_5528)?;
            c.g[A0][0] = 0;
            callee::FLUSH_CACHE.call(c, 0x003f_5530)?;
            proto_aries_init_request(
                c,
                PROTO_ARIES_CREATE_REQUEST,
                PROTO_ARIES_CMD_CREATE,
                0x003f_5550,
            )?;
            // DirtyRpc_Call(request, 1, 0, size, 0, 0, 0, 0; sync, self).
            c.g[V1][0] = 1;
            c.g[A0][0] = c.g[S0][0];
            Addr(frame.at(c, 0)).sw(c, c.g[V1][0]);
            c.g[A3][0] = c.g[S2][0];
            Addr(frame.at(c, 8)).sw(c, c.g[S1][0]);
            c.g[A1][0] = 1;
            c.g[A2][0] = 0;
            c.g[T0][0] = 0;
            c.g[T1][0] = 0;
            c.g[T2][0] = 0;
            c.g[T3][0] = 0;
            callee::DIRTY_RPC_CALL.call(c, 0x003f_5580)?;
            c.g[V1][0] = Addr(proto_aries_field(c, PROTO_ARIES_HANDLE)).lw(c);
            c.g[V0][0] = c.g[S1][0];
            if c.g[V1][0] == 0 {
                c.cycles += 4;
                c.g[A0][0] = c.g[S1][0];
                callee::NET_RPC_FREE.call(c, 0x003f_5594)?;
                c.g[S1][0] = 0;
                c.g[V0][0] = c.g[S1][0];
            }
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void ProtoAriesDestroy(ProtoAriesRefT *ref)` (0x3f55b8): ask the IOP to
/// destroy the socket (`'dest'`, synchronous) and free the object.
pub fn proto_aries_destroy(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 32;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0x10);
        proto_aries_init_request(
            c,
            PROTO_ARIES_DESTROY_REQUEST,
            PROTO_ARIES_CMD_DESTROY,
            0x003f_55ec,
        )?;
        // DirtyRpc_Call(request, 1, handle, 0, 0, 0, 0, 0; sync, 0).
        c.g[A2][0] = Addr(proto_aries_field(c, PROTO_ARIES_HANDLE)).lw(c);
        c.g[V0][0] = 1;
        c.g[A0][0] = c.g[S0][0];
        c.g[A3][0] = 0;
        c.g[T0][0] = 0;
        c.g[T1][0] = 0;
        c.g[T2][0] = 0;
        c.g[T3][0] = 0;
        Addr(frame.at(c, 0)).sw(c, c.g[V0][0]);
        c.g[A1][0] = 1;
        Addr(frame.at(c, 8)).sw(c, 0);
        callee::DIRTY_RPC_CALL.call(c, 0x003f_561c)?;
        c.g[A0][0] = c.g[S1][0];
        callee::NET_RPC_FREE.call(c, 0x003f_5624)?;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `reg = self->addrs` (the original reloads the pointer for every store),
/// then `store(reg + off, value)`.
pub(crate) fn proto_aries_addrs_store(
    c: &mut Cpu,
    reg: usize,
    off: u32,
    store: fn(Addr, &mut Cpu, u64),
    value: u64,
) {
    c.g[reg][0] = Addr(proto_aries_field(c, PROTO_ARIES_ADDRS)).lw(c);
    let at = r32(c, reg).wrapping_add(off);
    store(Addr(at), c, value);
}

/// `int32_t ProtoAriesConnect(ProtoAriesRefT *ref, uint32_t addr, int32_t
/// port)` (0x3f5638): -1 if the peer address's family is already set;
/// otherwise store `AF_INET`, the port and the IPv4 address (network order)
/// as the peer address, write the block back from the data cache, send the
/// 16-byte address with `'conn'` (asynchronous), clear the buffered-bytes
/// word and return 0.
pub fn proto_aries_connect(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        frame.save(c, S0, 0x30);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0x10);
        c.g[V1][0] = Addr(proto_aries_field(c, PROTO_ARIES_ADDRS)).lw(c);
        c.g[V0][0] =
            Addr(r32(c, V1).wrapping_add(PROTO_ARIES_PEER + PROTO_ARIES_ADDR_FAMILY)).lhu(c);
        if c.g[V0][0] != 0 {
            c.g[V0][0] = u64::MAX;
        } else {
            c.cycles += 50;
            c.g[V0][0] = PROTO_ARIES_AF_INET;
            c.g[A0][0] = proto_aries_sra(c, A2, 8);
            Addr(r32(c, V1).wrapping_add(PROTO_ARIES_PEER + PROTO_ARIES_ADDR_FAMILY))
                .sh(c, c.g[V0][0]);
            c.g[A3][0] = sx(r32(c, A1) >> 8);
            // Clear the rest of the peer address, then the port and address
            // bytes, most significant first in memory.
            let port = PROTO_ARIES_PEER + PROTO_ARIES_ADDR_PORT;
            let ip = PROTO_ARIES_PEER + PROTO_ARIES_ADDR_IP;
            proto_aries_addrs_store(c, V0, port, Addr::sh, 0);
            proto_aries_addrs_store(c, V1, ip, Addr::sw, 0);
            proto_aries_addrs_store(c, V0, ip + 4, Addr::sw, 0);
            proto_aries_addrs_store(c, V1, ip + 8, Addr::sw, 0);
            let port_hi = c.g[A0][0];
            proto_aries_addrs_store(c, V0, port, Addr::sb, port_hi);
            let port_lo = c.g[A2][0];
            proto_aries_addrs_store(c, V1, port + 1, Addr::sb, port_lo);
            let ip0 = c.g[A1][0];
            proto_aries_addrs_store(c, V0, ip + 3, Addr::sb, ip0);
            let ip8 = c.g[A3][0];
            proto_aries_addrs_store(c, V1, ip + 2, Addr::sb, ip8);
            c.g[A3][0] = sx(r32(c, A1) >> 16);
            let ip16 = c.g[A3][0];
            proto_aries_addrs_store(c, V0, ip + 1, Addr::sb, ip16);
            c.g[A3][0] = sx(r32(c, A1) >> 24);
            let ip24 = c.g[A3][0];
            proto_aries_addrs_store(c, V0, ip, Addr::sb, ip24);
            c.g[A0][0] = Addr(proto_aries_field(c, PROTO_ARIES_ADDRS)).lw(c);
            c.g[A1][0] = sx(r32(c, A0).wrapping_add(PROTO_ARIES_ADDRS_SIZE));
            callee::SYNC_DCACHE.call(c, 0x003f_56d0)?;
            proto_aries_init_request(
                c,
                PROTO_ARIES_CONNECT_REQUEST,
                PROTO_ARIES_CMD_CONNECT,
                0x003f_56f0,
            )?;
            // DirtyRpc_Call(request, 0, handle, 0, addrs, 16, 0, 0; 0, 0).
            c.g[A2][0] = Addr(proto_aries_field(c, PROTO_ARIES_HANDLE)).lw(c);
            c.g[A0][0] = c.g[S0][0];
            c.g[T0][0] = Addr(proto_aries_field(c, PROTO_ARIES_ADDRS)).lw(c);
            c.g[A1][0] = 0;
            Addr(frame.at(c, 0)).sw(c, 0);
            c.g[A3][0] = 0;
            Addr(frame.at(c, 8)).sw(c, 0);
            c.g[T1][0] = 0x10;
            c.g[T2][0] = 0;
            c.g[T3][0] = 0;
            callee::DIRTY_RPC_CALL.call(c, 0x003f_571c)?;
            Addr(proto_aries_field(c, PROTO_ARIES_RECV_FILL)).sw(c, 0);
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

/// `int32_t ProtoAriesUnconnect(ProtoAriesRefT *ref)` (0x3f5738): send
/// `'disc'` (asynchronous), clear the address block and the buffered-bytes
/// word; returns 0.
pub fn proto_aries_unconnect(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 35;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0x10);
        proto_aries_init_request(
            c,
            PROTO_ARIES_UNCONNECT_REQUEST,
            PROTO_ARIES_CMD_DISCONNECT,
            0x003f_576c,
        )?;
        // DirtyRpc_Call(request, 0, handle, 0, 0, 0, 0, 0; 0, 0).
        c.g[A2][0] = Addr(proto_aries_field(c, PROTO_ARIES_HANDLE)).lw(c);
        c.g[A3][0] = 0;
        c.g[T0][0] = 0;
        c.g[T1][0] = 0;
        c.g[T2][0] = 0;
        c.g[T3][0] = 0;
        c.g[A0][0] = c.g[S0][0];
        Addr(frame.at(c, 0)).sw(c, 0);
        Addr(frame.at(c, 8)).sw(c, 0);
        c.g[A1][0] = 0;
        callee::DIRTY_RPC_CALL.call(c, 0x003f_5798)?;
        c.g[A0][0] = Addr(proto_aries_field(c, PROTO_ARIES_ADDRS)).lw(c);
        c.g[A1][0] = 0;
        c.g[A2][0] = PROTO_ARIES_ADDRS_SIZE as u64;
        callee::MEMSET.call(c, 0x003f_57a8)?;
        Addr(proto_aries_field(c, PROTO_ARIES_RECV_FILL)).sw(c, 0);
        c.g[V0][0] = 0;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `ProtoAriesStatus`'s big-endian halfword at `off` in the address block:
/// `v1 = self->addrs`, the bytes in `v0`/`a0`, folded into `v0`.
pub(crate) fn proto_aries_status_port(c: &mut Cpu, off: u32) {
    c.g[V1][0] = Addr(proto_aries_field(c, PROTO_ARIES_ADDRS)).lw(c);
    let at = r32(c, V1).wrapping_add(off);
    c.g[V0][0] = Addr(at).lbu(c);
    c.g[A0][0] = Addr(at + 1).lbu(c);
    c.cycles += 3;
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 8);
    c.g[V0][0] |= c.g[A0][0];
}

/// `ProtoAriesStatus`'s big-endian word at `off` in the address block:
/// `v1 = self->addrs`, the low three bytes left in `a0`/`a1`/`a2`, the word
/// (sign-extended) in `v0`.
pub(crate) fn proto_aries_status_addr(c: &mut Cpu, off: u32) {
    c.g[V1][0] = Addr(proto_aries_field(c, PROTO_ARIES_ADDRS)).lw(c);
    let at = r32(c, V1).wrapping_add(off);
    c.g[V0][0] = Addr(at).lbu(c);
    c.g[A0][0] = Addr(at + 1).lbu(c);
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 8);
    c.g[A1][0] = Addr(at + 2).lbu(c);
    c.g[V0][0] |= c.g[A0][0];
    c.g[A2][0] = Addr(at + 3).lbu(c);
    c.cycles += 5;
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 8);
    c.g[V0][0] |= c.g[A1][0];
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 8);
    c.g[V0][0] |= c.g[A2][0];
}

/// `int32_t ProtoAriesStatus(ProtoAriesRefT *ref, int32_t select)`
/// (0x3f57c8): refresh the address block from the IOP (`'stat'`,
/// synchronous, 0x20 bytes back into the block), then answer `select` (a
/// 64-bit compare): `'port'`/`'addr'` the peer's port and address,
/// `'lprt'`/`'ladr'` the local ones, `'ibuf'` the buffered receive bytes,
/// anything else 0.
pub fn proto_aries_status(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 31;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S2, 0x20);
        frame.save(c, S1, 0x30);
        c.g[S2][0] = c.g[A1][0];
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0x10);
        proto_aries_init_request(
            c,
            PROTO_ARIES_STATUS_REQUEST,
            PROTO_ARIES_CMD_STATUS,
            0x003f_5804,
        )?;
        // DirtyRpc_Call(request, 0, handle, 0, 0, 0, addrs, 0x20; sync, 0).
        c.g[V1][0] = 1;
        c.g[A2][0] = Addr(proto_aries_field(c, PROTO_ARIES_HANDLE)).lw(c);
        c.g[T2][0] = Addr(proto_aries_field(c, PROTO_ARIES_ADDRS)).lw(c);
        c.g[A0][0] = c.g[S0][0];
        Addr(frame.at(c, 0)).sw(c, c.g[V1][0]);
        c.g[A1][0] = 0;
        Addr(frame.at(c, 8)).sw(c, 0);
        c.g[A3][0] = 0;
        c.g[T0][0] = 0;
        c.g[T1][0] = 0;
        c.g[T3][0] = PROTO_ARIES_ADDRS_SIZE as u64;
        callee::DIRTY_RPC_CALL.call(c, 0x003f_5834)?;
        c.g[V1][0] = PROTO_ARIES_SEL_PORT as u64;
        if c.g[S2][0] == c.g[V1][0] {
            c.cycles += 4;
            proto_aries_status_port(c, PROTO_ARIES_PEER + PROTO_ARIES_ADDR_PORT);
        } else {
            c.cycles += 3;
            c.g[V0][0] = PROTO_ARIES_SEL_ADDR as u64;
            if c.g[S2][0] == c.g[V0][0] {
                c.cycles += 8;
                proto_aries_status_addr(c, PROTO_ARIES_PEER + PROTO_ARIES_ADDR_IP);
            } else {
                c.cycles += 3;
                c.g[V0][0] = PROTO_ARIES_SEL_LOCAL_ADDR as u64;
                if c.g[S2][0] == c.g[V0][0] {
                    c.cycles += 7;
                    proto_aries_status_addr(c, PROTO_ARIES_LOCAL + PROTO_ARIES_ADDR_IP);
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = PROTO_ARIES_SEL_LOCAL_PORT as u64;
                    if c.g[S2][0] == c.g[V0][0] {
                        c.cycles += 3;
                        proto_aries_status_port(c, PROTO_ARIES_LOCAL + PROTO_ARIES_ADDR_PORT);
                    } else {
                        c.cycles += 3;
                        c.g[V0][0] = PROTO_ARIES_SEL_INPUT as u64;
                        let other = c.g[S2][0] != c.g[V0][0];
                        c.g[V0][0] = 0;
                        if !other {
                            c.cycles += 1;
                            c.g[V0][0] = Addr(proto_aries_field(c, PROTO_ARIES_RECV_FILL)).lw(c);
                        }
                    }
                }
            }
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void ProtoAries_Nop(void)` (0x3f5908): does nothing (`ProtoAriesSend`
/// calls it first).
pub fn proto_aries_nop(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `int32_t ProtoAriesSend(ProtoAriesRefT *ref, uint32_t word0, uint32_t
/// word1, const char *data, int32_t len)` (0x3f5910): send one record, a
/// 12-byte big-endian header (`word0`, `word1`, record length) followed by
/// `len` bytes of `data` (`strlen(data) + 1` when `len` is negative, a 64-bit
/// compare), as a two-part send list with `'send'` (asynchronous). Returns 0.
pub fn proto_aries_send(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0xc0);
        frame.save(c, S6, 0x50);
        frame.save(c, S2, 0x90);
        c.g[S6][0] = c.g[A0][0];
        frame.save(c, S3, 0x80);
        c.g[S2][0] = c.g[T0][0];
        frame.save(c, S4, 0x70);
        c.g[S3][0] = c.g[A1][0];
        frame.save(c, S5, 0x60);
        c.g[S4][0] = c.g[A2][0];
        frame.save(c, S0, 0xb0);
        frame.save(c, S1, 0xa0);
        frame.save_ra(c, 0x40);
        c.g[S5][0] = c.g[A3][0];
        callee::PROTO_ARIES_NOP.call(c, 0x003f_594c)?;
        let given = (c.g[S2][0] as i64) >= 0;
        c.g[S2][0] = sx(r32(c, S2).wrapping_add(PROTO_ARIES_RECORD_HEADER));
        if !given {
            c.cycles += 4;
            c.g[A0][0] = c.g[S5][0];
            callee::STRLEN.call(c, 0x003f_595c)?;
            c.g[S2][0] = sx(r32(c, V0).wrapping_add(1));
            c.g[S2][0] = sx(r32(c, S2).wrapping_add(PROTO_ARIES_RECORD_HEADER));
        }
        c.cycles += 59;
        // `s2` is the record length; the header bytes, most significant
        // first: word0 (`s3`), word1 (`s4`), the length.
        c.g[V1][0] = proto_aries_sra(c, S2, 16);
        c.g[T4][0] = proto_aries_sra(c, S4, 16);
        c.g[T5][0] = proto_aries_sra(c, S4, 8);
        c.g[T7][0] = sx(frame.at(c, 0x10));
        c.g[A3][0] = proto_aries_sra(c, S2, 8);
        c.g[T0][0] = proto_aries_sra(c, S3, 24);
        c.g[T1][0] = proto_aries_sra(c, S3, 16);
        c.g[T2][0] = proto_aries_sra(c, S3, 8);
        c.g[T3][0] = proto_aries_sra(c, S4, 24);
        c.g[T6][0] = PROTO_ARIES_RECORD_HEADER as u64;
        c.g[S1][0] = sx(r32(c, S2).wrapping_sub(PROTO_ARIES_RECORD_HEADER));
        c.g[V0][0] = proto_aries_sra(c, S2, 24);
        let header = frame.at(c, 0x10);
        let bytes = [T0, T1, T2, S3, T3, T4, T5, S4, V0, V1, A3, S2];
        for (i, reg) in bytes.into_iter().enumerate() {
            let v = c.g[reg][0];
            Addr(header + i as u32).sb(c, v);
        }
        // The send list, `{length, data}` pairs ended by a zero length: the
        // header, then the data.
        Addr(frame.at(c, 0x20)).sw(c, c.g[T6][0]);
        Addr(frame.at(c, 0x24)).sw(c, c.g[T7][0]);
        Addr(frame.at(c, 0x28)).sw(c, c.g[S1][0]);
        Addr(frame.at(c, 0x2c)).sw(c, c.g[S5][0]);
        Addr(frame.at(c, 0x30)).sw(c, 0);
        proto_aries_init_request(
            c,
            PROTO_ARIES_SEND_REQUEST,
            PROTO_ARIES_CMD_SEND,
            0x003f_59f8,
        )?;
        // DirtyRpc_Call(request, 2 (t0 is a send list), handle, 0, list, 0,
        // 0, 0; 0, 0).
        c.g[A2][0] = Addr(r32(c, S6)).lw(c);
        c.g[A0][0] = c.g[S0][0];
        Addr(frame.at(c, 0)).sw(c, 0);
        c.g[A1][0] = 2;
        Addr(frame.at(c, 8)).sw(c, 0);
        c.g[A3][0] = 0;
        c.g[T0][0] = sx(frame.at(c, 0x20));
        c.g[T1][0] = 0;
        c.g[T2][0] = 0;
        c.g[T3][0] = 0;
        callee::DIRTY_RPC_CALL.call(c, 0x003f_5a24)?;
        frame.restore(c, S0, 0xb0);
        c.g[V0][0] = 0;
        frame.restore(c, S1, 0xa0);
        frame.restore(c, S2, 0x90);
        frame.restore(c, S3, 0x80);
        frame.restore(c, S4, 0x70);
        frame.restore(c, S5, 0x60);
        frame.restore(c, S6, 0x50);
        frame.restore_ra(c, 0x40);
        frame.close(c);
        Ok(())
    })
}

/// `v0 = self->recv_read < self->recv_fill` (signed): unread bytes in the
/// receive buffer.
pub(crate) fn proto_aries_unread(c: &mut Cpu) -> bool {
    c.g[V0][0] = Addr(proto_aries_field(c, PROTO_ARIES_RECV_READ)).lw(c);
    c.g[V1][0] = Addr(proto_aries_field(c, PROTO_ARIES_RECV_FILL)).lw(c);
    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
    c.g[V0][0] != 0
}

/// A big-endian word at `at`, `lbu` into `regs` and folded as the compiled
/// code does: `regs[0..3]` keep the top three bytes shifted into place (the
/// first or-ed together), `regs[3]` the low byte; returns the whole word
/// (sign-extended).
pub(crate) fn proto_aries_be_word(c: &mut Cpu, at: u32, regs: [usize; 4]) -> u64 {
    for (i, reg) in regs.into_iter().enumerate() {
        c.g[reg][0] = Addr(at.wrapping_add(i as u32)).lbu(c);
    }
    c.g[regs[0]][0] = sx((c.g[regs[0]][0] as u32) << 24);
    c.g[regs[1]][0] = sx((c.g[regs[1]][0] as u32) << 16);
    c.g[regs[2]][0] = sx((c.g[regs[2]][0] as u32) << 8);
    c.g[regs[0]][0] |= c.g[regs[1]][0];
    c.g[regs[0]][0] |= c.g[regs[2]][0];
    c.g[regs[0]][0] | c.g[regs[3]][0]
}

/// `ProtoAriesPeek`'s record arm: the record at the read offset (`t0`); its
/// payload length (`t1`, record length - 12), the payload (`a2`, its last
/// byte overwritten with a NUL, or `""` when empty) and the two header words
/// stored through the non-null outputs `t2`, `t3`, `a3`; `v0 = t1`.
pub(crate) fn proto_aries_peek_record(c: &mut Cpu) {
    c.cycles += 19;
    c.g[A0][0] = Addr(proto_aries_field(c, PROTO_ARIES_RECV_READ)).lw(c);
    c.g[V1][0] = Addr(proto_aries_field(c, PROTO_ARIES_RECV_BUF)).lw(c);
    c.g[A1][0] = Addr(proto_aries_field(c, PROTO_ARIES_RECV_READ)).lw(c);
    c.g[T0][0] = sx(r32(c, V1).wrapping_add(r32(c, A0)));
    c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, A1)));
    c.g[A2][0] = sx(r32(c, V1).wrapping_add(PROTO_ARIES_RECORD_HEADER));
    let record = r32(c, T0);
    let len = proto_aries_be_word(
        c,
        record.wrapping_add(PROTO_ARIES_RECORD_LEN),
        [V0, A0, A1, V1],
    );
    c.g[T1][0] = len;
    c.g[T1][0] = sx(r32(c, T1).wrapping_sub(PROTO_ARIES_RECORD_HEADER));
    let has_payload = (c.g[T1][0] as i64) > 0;
    c.g[V0][0] = sx(r32(c, T1).wrapping_add(r32(c, A2)));
    if has_payload {
        c.cycles += 1;
        Addr(r32(c, V0).wrapping_sub(1)).sb(c, 0);
    } else {
        c.cycles += 3;
        c.g[V0][0] = sx(PROTO_ARIES_EMPTY_STRING & 0xffff_0000);
        c.g[A2][0] = sx(PROTO_ARIES_EMPTY_STRING);
    }
    c.cycles += 2;
    if c.g[T2][0] != 0 {
        c.cycles += 11;
        c.g[V0][0] = proto_aries_be_word(
            c,
            record.wrapping_add(PROTO_ARIES_RECORD_WORD0),
            [V0, V1, A0, A1],
        );
        Addr(r32(c, T2)).sw(c, c.g[V0][0]);
    }
    c.cycles += 2;
    if c.g[T3][0] != 0 {
        c.cycles += 11;
        c.g[V0][0] = proto_aries_be_word(
            c,
            record.wrapping_add(PROTO_ARIES_RECORD_WORD1),
            [V0, V1, A0, A1],
        );
        Addr(r32(c, T3)).sw(c, c.g[V0][0]);
    }
    c.cycles += 2;
    if c.g[A3][0] != 0 {
        Addr(r32(c, A3)).sw(c, c.g[A2][0]);
    }
    c.cycles += 1;
    c.g[V0][0] = c.g[T1][0];
}

/// Where `ProtoAriesPeek` and `ProtoAriesRecv` start the next receive (the
/// same code at two places): the request record, the return addresses of
/// its three calls and the charge of the block before `DirtyRpc_InitRequest`.
pub(crate) struct ProtoAriesReceiveSite {
    pub(crate) request: u32,
    pub(crate) invalidate_ret: u32,
    pub(crate) init_ret: u32,
    pub(crate) call_ret: u32,
    pub(crate) cycles: u64,
}

pub(crate) const PROTO_ARIES_PEEK_RECEIVE: ProtoAriesReceiveSite = ProtoAriesReceiveSite {
    request: PROTO_ARIES_PEEK_REQUEST,
    invalidate_ret: 0x003f_5aec,
    init_ret: 0x003f_5b18,
    call_ret: 0x003f_5b4c,
    cycles: 22,
};

pub(crate) const PROTO_ARIES_RECV_RECEIVE: ProtoAriesReceiveSite = ProtoAriesReceiveSite {
    request: PROTO_ARIES_RECV_REQUEST,
    invalidate_ret: 0x003f_5d04,
    init_ret: 0x003f_5d30,
    call_ret: 0x003f_5d64,
    cycles: 20,
};

/// Mark a receive pending; if the last reply put bytes in the buffer, drop
/// the buffer from the data cache and rewind it; then ask the IOP to fill it
/// (`'recv'`, completion through `ProtoAries_RpcReplyCallback(self)`).
pub(crate) fn proto_aries_request_receive(c: &mut Cpu, site: &ProtoAriesReceiveSite) -> Flow {
    c.cycles += 5;
    c.g[V0][0] = 1;
    Addr(proto_aries_field(c, PROTO_ARIES_RECV_PENDING)).sw(c, c.g[V0][0]);
    c.g[V1][0] = Addr(proto_aries_field(c, PROTO_ARIES_RECV_FILL)).lw(c);
    let empty = (c.g[V1][0] as i64) <= 0;
    c.g[S0][0] = PROTO_ARIES_REQUESTS_HI;
    if !empty {
        c.cycles += 10;
        c.g[A1][0] = Addr(proto_aries_field(c, PROTO_ARIES_RECV_BUF)).lw(c);
        c.g[V0][0] = Addr(proto_aries_field(c, PROTO_ARIES_RECV_SIZE)).lw(c);
        c.g[A0][0] = c.g[A1][0];
        c.g[A1][0] = sx(r32(c, A1).wrapping_add(r32(c, V0)));
        c.g[A1][0] = sx(r32(c, A1).wrapping_sub(1));
        callee::INVALID_DCACHE.call(c, site.invalidate_ret)?;
        Addr(proto_aries_field(c, PROTO_ARIES_RECV_FILL)).sw(c, 0);
        c.g[V0][0] = Addr(proto_aries_field(c, PROTO_ARIES_RECV_FILL)).lw(c);
        Addr(proto_aries_field(c, PROTO_ARIES_RECV_READ)).sw(c, c.g[V0][0]);
        c.g[S0][0] = PROTO_ARIES_REQUESTS_HI;
    }
    c.cycles += site.cycles;
    proto_aries_init_request(c, site.request, PROTO_ARIES_CMD_RECV, site.init_ret)?;
    // DirtyRpc_Call(request, 0, handle, 0, 0, 0, buf, size; callback, self).
    c.g[V1][0] = sx(PROTO_ARIES_RPC_REPLY_CALLBACK);
    c.g[T3][0] = Addr(proto_aries_field(c, PROTO_ARIES_RECV_SIZE)).lw(c);
    c.g[A2][0] = Addr(proto_aries_field(c, PROTO_ARIES_HANDLE)).lw(c);
    c.g[T2][0] = Addr(proto_aries_field(c, PROTO_ARIES_RECV_BUF)).lw(c);
    c.g[A0][0] = c.g[S0][0];
    Addr(sp(c)).sw(c, c.g[V1][0]);
    c.g[A1][0] = 0;
    Addr(sp(c).wrapping_add(8)).sw(c, c.g[S1][0]);
    c.g[A3][0] = 0;
    c.g[T0][0] = 0;
    c.g[T1][0] = 0;
    callee::DIRTY_RPC_CALL.call(c, site.call_ret)
}

/// `int32_t ProtoAriesPeek(ProtoAriesRefT *ref, uint32_t *word0, uint32_t
/// *word1, char **data)` (0x3f5a70): with unread bytes in the receive
/// buffer, look at the record at the read offset without consuming it
/// (returns its payload length; see `proto_aries_peek_record`). Otherwise
/// -1, and unless a receive is already pending, start the next one.
pub fn proto_aries_peek(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x40);
        c.g[T2][0] = c.g[A1][0];
        frame.save(c, S1, 0x20);
        c.g[T3][0] = c.g[A2][0];
        frame.save(c, S0, 0x30);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0x10);
        if proto_aries_unread(c) {
            proto_aries_peek_record(c);
        } else {
            c.cycles += 3;
            c.g[V0][0] = Addr(proto_aries_field(c, PROTO_ARIES_RECV_PENDING)).lw(c);
            let pending = c.g[V0][0] != 0;
            c.g[V0][0] = u64::MAX;
            if !pending {
                c.cycles += 5;
                let unread = proto_aries_unread(c);
                c.g[V0][0] = u64::MAX;
                if !unread {
                    proto_aries_request_receive(c, &PROTO_ARIES_PEEK_RECEIVE)?;
                    c.g[V0][0] = u64::MAX;
                }
            }
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `int32_t ProtoAriesRecv(ProtoAriesRefT *ref, uint32_t *word0, uint32_t
/// *word1, char *buf, int32_t len)` (0x3f5c38): peek at the next record; if
/// there is one, copy up to `len` payload bytes into `buf` (when non-null),
/// step the read offset past the record and, once the buffer is drained and
/// no receive is pending, start the next receive. Returns the payload length
/// (or `ProtoAriesPeek`'s negative result).
pub fn proto_aries_recv(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S1, 0x40);
        frame.save(c, S2, 0x30);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x50);
        c.g[S2][0] = c.g[A3][0];
        frame.save(c, S3, 0x20);
        c.g[S0][0] = c.g[T0][0];
        frame.save_ra(c, 0x10);
        c.g[A3][0] = 0;
        callee::PROTO_ARIES_PEEK.call(c, 0x003f_5c64)?;
        c.g[S3][0] = c.g[V0][0];
        let none = (c.g[S3][0] as i64) < 0;
        c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[S0][0] as i64)) as u64;
        // Whether the exit goes through `v0 = s3` at 0x3f5d64 (after a failed
        // peek or a new receive) or straight to the epilogue.
        let via_result = 'consume: {
            if none {
                break 'consume true;
            }
            c.cycles += 2;
            let discard = c.g[S2][0] == 0;
            if c.g[V0][0] != 0 {
                c.g[S0][0] = c.g[S3][0];
            }
            if !discard {
                c.cycles += 7;
                c.g[A1][0] = Addr(proto_aries_field(c, PROTO_ARIES_RECV_BUF)).lw(c);
                c.g[A0][0] = c.g[S2][0];
                c.g[V0][0] = Addr(proto_aries_field(c, PROTO_ARIES_RECV_READ)).lw(c);
                c.g[A2][0] = c.g[S0][0];
                c.g[A1][0] = sx(r32(c, A1).wrapping_add(r32(c, V0)));
                c.g[A1][0] = sx(r32(c, A1).wrapping_add(PROTO_ARIES_RECORD_HEADER));
                callee::MEMCPY.call(c, 0x003f_5c94)?;
            }
            c.cycles += 9;
            c.g[V0][0] = Addr(proto_aries_field(c, PROTO_ARIES_RECV_READ)).lw(c);
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(PROTO_ARIES_RECORD_HEADER));
            c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, S3)));
            Addr(proto_aries_field(c, PROTO_ARIES_RECV_READ)).sw(c, c.g[V0][0]);
            c.g[V1][0] = Addr(proto_aries_field(c, PROTO_ARIES_RECV_READ)).lw(c);
            c.g[V0][0] = Addr(proto_aries_field(c, PROTO_ARIES_RECV_FILL)).lw(c);
            c.g[V1][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
            let more = c.g[V1][0] != 0;
            c.g[V0][0] = c.g[S3][0];
            if more {
                break 'consume false;
            }
            c.cycles += 3;
            c.g[V0][0] = Addr(proto_aries_field(c, PROTO_ARIES_RECV_PENDING)).lw(c);
            let pending = c.g[V0][0] != 0;
            c.g[V0][0] = c.g[S3][0];
            if pending {
                break 'consume false;
            }
            c.cycles += 5;
            let unread = proto_aries_unread(c);
            c.g[V0][0] = c.g[S3][0];
            if unread {
                break 'consume false;
            }
            proto_aries_request_receive(c, &PROTO_ARIES_RECV_RECEIVE)?;
            true
        };
        if via_result {
            c.cycles += 1;
            c.g[V0][0] = c.g[S3][0];
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

/// `uint32_t ProtoAriesTick(void)` (0x3f5d88): the network clock,
/// `NetTick()`.
pub fn proto_aries_tick(c: &mut Cpu) {
    tail_wrapper(c, None, Target::Direct(callee::NET_TICK), 0x003f_5d98);
}

// ---------------------------------------------------------------------------
// The keyboard wrapper's state.

/// The keyboard wrapper's state block (one per game).
pub(crate) const USB_KB_STATE: u32 = 0x0052_5f80;

/// Its low half (`addiu` on the `lui` half the code keeps in a register).
pub(crate) const USB_KB_STATE_LO: u32 = 0x5f80;

// Fields of the state block.
/// Module id of `USBD.IRX` (-1: not loaded).
pub(crate) const USB_KB_USBD_ID: u32 = 0x0;

/// Module id of `USBKB.IRX`.
pub(crate) const USB_KB_USBKB_ID: u32 = 0x4;

/// `sceUsbKbGetInfo`'s buffer.
pub(crate) const USB_KB_INFO: u32 = 0x8;

/// Connection status per keyboard (bytes; inside the info buffer).
pub(crate) const USB_KB_CONNECTED: u32 = 0x10;

/// Keyboards the driver serves (`sceUsbKbInit` writes it; at most 8).
pub(crate) const USB_KB_MAX: u32 = 0x90;

/// Connection status last reported to the connect callback (bytes).
pub(crate) const USB_KB_SEEN: u32 = 0x94;

/// The pending request (`USB_KB_REQ_*`).
pub(crate) const USB_KB_REQUEST: u32 = 0x9c;

/// Byte: a request is in flight.
pub(crate) const USB_KB_BUSY: u32 = 0xa0;

/// Keyboard of the last read.
pub(crate) const USB_KB_READ_PORT: u32 = 0xa4;

/// The caller's keyboard data array (`USB_KB_DATA_SIZE` bytes per keyboard).
pub(crate) const USB_KB_DATA: u32 = 0xa8;

/// The read keyboard's LED and modifier words before the read.
pub(crate) const USB_KB_PREV_LED: u32 = 0xac;

pub(crate) const USB_KB_PREV_MKEY: u32 = 0xb0;

/// Frames since the last info request, and how many frames that may be.
pub(crate) const USB_KB_POLL_COUNT: u32 = 0xb4;

pub(crate) const USB_KB_POLL_PERIOD: u32 = 0xb8;

/// `void (*)(int keyboard, int status)`: told when a keyboard's status changes.
pub(crate) const USB_KB_CONNECT_CB: u32 = 0xbc;

/// `int (*)(int release)`: may hold the library (see [`usb_kb_update`]).
pub(crate) const USB_KB_HOLD_CB: u32 = 0xc0;

// Pending requests.
pub(crate) const USB_KB_REQ_INFO: u64 = 1;

pub(crate) const USB_KB_REQ_READ: u64 = 2;

/// The hold callback holds the library: nothing is requested.
pub(crate) const USB_KB_REQ_HELD: u64 = 3;

/// The info poll period `UsbKb_Init` sets (frames).
pub(crate) const USB_KB_POLL_FRAMES: u64 = 0x3c;

/// `UsbKb_Init` serves at most this many keyboards.
pub(crate) const USB_KB_MAX_KEYBOARDS: u64 = 8;

// One keyboard's data (`sceUsbKbRead`'s buffer).
pub(crate) const USB_KB_DATA_SIZE: u64 = 0x8c;

/// LED word (`USB_KB_LED_*`).
pub(crate) const USB_KB_DATA_LED: u32 = 0x0;

/// Modifier-key word (`USB_KB_MKEY_*`).
pub(crate) const USB_KB_DATA_MKEY: u32 = 0x4;

/// Number of key codes.
pub(crate) const USB_KB_DATA_LEN: u32 = 0x8;

/// Key codes (u16 each).
pub(crate) const USB_KB_DATA_KEYS: u32 = 0xc;

/// Byte: set when a read brought something new, cleared by `UsbKb_GetData`.
pub(crate) const USB_KB_DATA_FRESH: u32 = 0x88;

// LED bits.
pub(crate) const USB_KB_LED_NUM_LOCK: u64 = 0x1;

pub(crate) const USB_KB_LED_CAPS_LOCK: u64 = 0x2;

pub(crate) const USB_KB_LED_SCROLL_LOCK: u64 = 0x4;

// Modifier-key bits.
pub(crate) const USB_KB_MKEY_LCTRL: u64 = 0x01;

pub(crate) const USB_KB_MKEY_LSHIFT: u64 = 0x02;

pub(crate) const USB_KB_MKEY_LALT: u64 = 0x04;

pub(crate) const USB_KB_MKEY_LWIN: u64 = 0x08;

pub(crate) const USB_KB_MKEY_RCTRL: u64 = 0x10;

pub(crate) const USB_KB_MKEY_RSHIFT: u64 = 0x20;

pub(crate) const USB_KB_MKEY_RALT: u64 = 0x40;

pub(crate) const USB_KB_MKEY_RWIN: u64 = 0x80;

// The key codes the game asks about (they match Windows virtual-key codes).
pub(crate) const USB_KB_VK_SHIFT: u64 = 0x10;

pub(crate) const USB_KB_VK_CONTROL: u64 = 0x11;

pub(crate) const USB_KB_VK_MENU: u64 = 0x12;

pub(crate) const USB_KB_VK_CAPITAL: u64 = 0x14;

pub(crate) const USB_KB_VK_LWIN: u64 = 0x5b;

pub(crate) const USB_KB_VK_RWIN: u64 = 0x5c;

pub(crate) const USB_KB_VK_NUMLOCK: u64 = 0x90;

pub(crate) const USB_KB_VK_SCROLL: u64 = 0x91;

pub(crate) const USB_KB_VK_LSHIFT: u64 = 0xa0;

pub(crate) const USB_KB_VK_RSHIFT: u64 = 0xa1;

pub(crate) const USB_KB_VK_LCONTROL: u64 = 0xa2;

pub(crate) const USB_KB_VK_RCONTROL: u64 = 0xa3;

pub(crate) const USB_KB_VK_LMENU: u64 = 0xa4;

pub(crate) const USB_KB_VK_RMENU: u64 = 0xa5;

// USB key usages.
/// The keypad: `/ * - + Enter` (0x54-0x58), then `1`-`9 0 .` (0x59-0x63).
pub(crate) const USB_KB_USAGE_KEYPAD: u32 = 0x54;

pub(crate) const USB_KB_KEYPAD_KEYS: u64 = 0x10;

pub(crate) const USB_KB_USAGE_KEYPAD_1: u32 = 0x59;

pub(crate) const USB_KB_KEYPAD_DIGITS: u64 = 0xb;

/// The usages the other layout maps differently: 0x2e-0x35 and 0x87-0x8b.
pub(crate) const USB_KB_USAGE_ALT_A: u32 = 0x2e;

pub(crate) const USB_KB_USAGE_ALT_A_KEYS: u64 = 8;

pub(crate) const USB_KB_USAGE_ALT_B: u32 = 0x87;

pub(crate) const USB_KB_USAGE_ALT_B_KEYS: u64 = 5;

// Game tables (pairs `{usage, key code}`, read at +1).
/// Pairs indexed by usage (`+ 2 * usage`).
pub(crate) const USB_KB_KEY_TABLE: u32 = 0x0045_1350;

/// The other layout's pairs for 0x2e-0x35 and 0x87-0x8b (indexed by usage).
pub(crate) const USB_KB_KEY_TABLE_ALT_A: u32 = 0x0045_13dc;

pub(crate) const USB_KB_KEY_TABLE_ALT_B: u32 = 0x0045_133a;

/// Num-lock keypad pairs, indexed by `usage - 0x59`.
pub(crate) const USB_KB_KEY_TABLE_NUMPAD: u32 = 0x0045_1420;

/// Keypad characters, indexed by `usage - 0x54`.
pub(crate) const USB_KB_KEYPAD_CHARS: u32 = 0x0045_1458;

/// `UsbKb_TranslateKey`'s result (two bytes; gp-relative).
pub(crate) const USB_KB_KEY_PAIR_GP: u32 = 0x2a20;

/// Module file name (game text): "USBD.IRX".
pub(crate) const USB_KB_USBD_IRX: u32 = 0x0049_7038;

/// Module file name (game text): "USBKB.IRX".
pub(crate) const USB_KB_USBKB_IRX: u32 = 0x0049_7048;

/// `UsbKb_UnloadModule`'s results for the two modules.
pub(crate) const USB_KB_UNLOAD_ERR_USBKB: u64 = 1;

pub(crate) const USB_KB_UNLOAD_ERR_USBD: u64 = 2;

/// The hold callback `UsbKb_Init` installs when given none (the connect
/// callback's default is `callee::USB_KB_DEFAULT_CONNECT_CB`). The name
/// `UsbKb_NullCallback` comes from `generated/ee-symbols.tsv`; it is not in
/// `functions.tsv`.
pub(crate) const USB_KB_NULL_CALLBACK: u32 = 0x003f_61c8;

/// `sceUsbKbSync` modes.
pub(crate) const USB_KB_SYNC_WAIT: u64 = 0;

pub(crate) const USB_KB_SYNC_POLL: u64 = 1;

/// What `UsbKb_DefaultConnectCb` sets up on a new keyboard (besides the LED
/// status `USB_KB_LED_NUM_LOCK`): LED mode 1, key repeat after 30 frames then
/// every 2, code type 0.
pub(crate) const USB_KB_CONNECT_LED_MODE: u64 = 0x1;

pub(crate) const USB_KB_REPEAT_START: u64 = 0x1e;

pub(crate) const USB_KB_REPEAT_INTERVAL: u64 = 0x2;

pub(crate) const USB_KB_CODE_TYPE: u64 = 0;

/// The `lui` half of `addr` (its low half is below 0x8000 for every address
/// here).
#[inline]
pub(crate) fn usb_kb_lui(addr: u32) -> u64 {
    (addr & 0xffff_0000) as u64
}

/// The state block from the `lui` half in register `reg` (`addiu reg, 0x5f80`).
#[inline]
pub(crate) fn usb_kb_state_from(c: &Cpu, reg: usize) -> u64 {
    sx((c.g[reg][0] as u32).wrapping_add(USB_KB_STATE_LO))
}

// ---------------------------------------------------------------------------
// Modules.

/// `int UsbKb_LoadModules(const char *dir, const char *kb_args)` (0x3f5da8):
/// load `USBD.IRX` and then `USBKB.IRX` (with `kb_args`, its length plus the
/// NUL as the argument size) from the directory `dir` into the IOP, keeping
/// each `sceSifLoadModule` result as the module id (+0, +4). A null `dir`
/// records both ids as -1 and returns 0; a null `kb_args` or a failed load
/// returns -1.
pub fn usb_kb_load_modules(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x140);
        frame.save(c, S0, 0x130);
        frame.save(c, S1, 0x120);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x110);
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0x100);
        'done: {
            if c.g[S0][0] == 0 {
                c.cycles += 7;
                c.g[V0][0] = usb_kb_lui(USB_KB_STATE);
                c.g[V1][0] = u64::MAX;
                c.g[A0][0] = sx(USB_KB_STATE);
                Addr(USB_KB_STATE + USB_KB_USBD_ID).sw(c, c.g[V1][0]);
                Addr(USB_KB_STATE + USB_KB_USBKB_ID).sw(c, c.g[V1][0]);
                c.g[V0][0] = 0;
                break 'done;
            }
            'failed: {
                c.cycles += 2;
                c.g[A0][0] = c.g[SP][0];
                if c.g[S1][0] == 0 {
                    break 'failed;
                }
                // The path `dir` + "USBD.IRX" in the frame; no arguments.
                c.cycles += 14;
                c.g[A1][0] = c.g[S0][0];
                callee::STRCPY.call(c, 0x003f_5df4)?;
                c.g[A0][0] = c.g[SP][0];
                c.g[A1][0] = sx(USB_KB_USBD_IRX);
                callee::STRCAT.call(c, 0x003f_5e04)?;
                c.g[A0][0] = c.g[SP][0];
                c.g[A1][0] = 0;
                c.g[A2][0] = 0;
                callee::SCE_SIF_LOAD_MODULE.call(c, 0x003f_5e14)?;
                c.g[V1][0] = usb_kb_lui(USB_KB_STATE);
                c.g[S2][0] = sx(USB_KB_STATE);
                let failed = (c.g[V0][0] as i64) < 0;
                Addr(USB_KB_STATE + USB_KB_USBD_ID).sw(c, c.g[V0][0]);
                if failed {
                    break 'failed;
                }
                // `dir` + "USBKB.IRX" with the keyboard arguments.
                c.cycles += 19;
                c.g[A0][0] = c.g[SP][0];
                c.g[A1][0] = c.g[S0][0];
                callee::STRCPY.call(c, 0x003f_5e30)?;
                c.g[A0][0] = c.g[SP][0];
                c.g[A1][0] = sx(USB_KB_USBKB_IRX);
                callee::STRCAT.call(c, 0x003f_5e40)?;
                c.g[A0][0] = c.g[S1][0];
                callee::STRLEN.call(c, 0x003f_5e48)?;
                c.g[A0][0] = c.g[SP][0];
                c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
                c.g[A2][0] = c.g[S1][0];
                callee::SCE_SIF_LOAD_MODULE.call(c, 0x003f_5e58)?;
                let failed = (c.g[V0][0] as i64) < 0;
                c.g[V1][0] = failed as u64;
                Addr((c.g[S2][0] as u32).wrapping_add(USB_KB_USBKB_ID)).sw(c, c.g[V0][0]);
                c.g[A0][0] = u64::MAX;
                c.g[V0][0] = if failed { c.g[A0][0] } else { 0 };
                break 'done;
            }
            c.cycles += 1;
            c.g[V0][0] = u64::MAX;
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x130);
        frame.restore(c, S1, 0x120);
        frame.restore(c, S2, 0x110);
        frame.restore_ra(c, 0x100);
        frame.close(c);
        Ok(())
    })
}

/// `int UsbKb_UnloadModule(int id, int err)` (0x3f5e90): stop and unload
/// the IOP module `id` (nothing for a negative id). Returns 0, or `err` when
/// the stop or the unload failed.
pub fn usb_kb_unload_module(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S1, 0x40);
        frame.save(c, S2, 0x30);
        c.g[S1][0] = 0;
        frame.save(c, S3, 0x20);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0x50);
        c.g[S3][0] = c.g[A1][0];
        frame.save_ra(c, 0x10);
        if (c.g[S2][0] as i64) >= 0 {
            c.cycles += 11;
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            c.g[A3][0] = c.g[SP][0];
            callee::SCE_SIF_STOP_MODULE.call(c, 0x003f_5ec8)?;
            c.g[S0][0] = u64::MAX;
            c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
            c.g[A0][0] = c.g[S2][0];
            if c.g[V0][0] == 0 {
                c.g[S1][0] = c.g[S3][0];
            }
            callee::SCE_SIF_UNLOAD_MODULE.call(c, 0x003f_5edc)?;
            c.g[S0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
            if c.g[S0][0] == 0 {
                c.g[S1][0] = c.g[S3][0];
            }
        }
        c.cycles += 8;
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Requests to the driver.

/// `void UsbKb_RequestInfo(void)` (0x3f5f08): remember every keyboard's
/// connection status as reported (+0x94 from +0x10), then ask the driver for
/// new ones (`sceUsbKbGetInfo` into +8); when the request went out, mark it
/// pending (request 1, busy).
pub fn usb_kb_request_info(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        c.g[V1][0] = usb_kb_lui(USB_KB_STATE);
        frame.save(c, S0, 0x10);
        c.g[V1][0] = sx(USB_KB_STATE);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr(USB_KB_STATE + USB_KB_MAX).lw(c);
        c.g[A1][0] = 0;
        if (c.g[V0][0] as i64) > 0 {
            c.cycles += 2;
            c.g[A0][0] = sx(USB_KB_STATE + USB_KB_SEEN);
            loop {
                c.cycles += 7;
                let status_at = (c.g[A0][0] as u32).wrapping_sub(USB_KB_SEEN - USB_KB_CONNECTED);
                c.g[V0][0] = Addr(status_at).lbu(c);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
                Addr(c.g[A0][0] as u32).sb(c, c.g[V0][0]);
                c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(USB_KB_MAX)).lw(c);
                c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(1));
                if c.g[V0][0] == 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x003f_5f30);
                }
            }
        }
        c.cycles += 6;
        c.g[V0][0] = usb_kb_lui(USB_KB_STATE);
        c.g[S0][0] = sx(USB_KB_STATE + USB_KB_INFO);
        c.g[A0][0] = c.g[S0][0];
        callee::SCE_USB_KB_GET_INFO.call(c, 0x003f_5f5c)?;
        if c.g[V0][0] == 0 {
            c.cycles += 5;
            c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_sub(USB_KB_INFO));
            c.g[V0][0] = 1;
            Addr((c.g[V1][0] as u32).wrapping_add(USB_KB_BUSY)).sb(c, c.g[V0][0]);
            Addr((c.g[V1][0] as u32).wrapping_add(USB_KB_REQUEST)).sw(c, USB_KB_REQ_INFO);
        }
        frame.restore(c, S0, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int UsbKb_SyncInfo(void)` (0x3f5f88): poll the info request
/// (`sceUsbKbSync(1, &result)`); while it is not done return what the sync
/// said. Once done, call the connect callback (+0xbc) with `(keyboard,
/// status)` for every keyboard whose status differs from the one last
/// reported, clear the request and return 0.
pub fn usb_kb_sync_info(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x70);
        c.g[A0][0] = USB_KB_SYNC_POLL;
        frame.save(c, S3, 0x30);
        frame.save(c, S0, 0x60);
        frame.save(c, S1, 0x50);
        frame.save(c, S2, 0x40);
        frame.save(c, S4, 0x20);
        frame.save_ra(c, 0x10);
        c.g[A1][0] = c.g[SP][0];
        callee::SCE_USB_KB_SYNC.call(c, 0x003f_5fb0)?;
        c.g[S3][0] = c.g[V0][0];
        frame.restore(c, S0, 0x60);
        if c.g[S3][0] == 0 {
            c.cycles += 7;
            c.g[V1][0] = usb_kb_lui(USB_KB_STATE);
            c.g[S4][0] = c.g[V1][0];
            c.g[V1][0] = sx(USB_KB_STATE);
            c.g[V0][0] = Addr(USB_KB_STATE + USB_KB_MAX).lw(c);
            c.g[V0][0] = ((c.g[S3][0] as i64) < (c.g[V0][0] as i64)) as u64;
            c.g[S1][0] = 0;
            if c.g[V0][0] != 0 {
                c.cycles += 2;
                c.g[S2][0] = c.g[V1][0];
                c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(USB_KB_CONNECTED));
                loop {
                    c.cycles += 4;
                    let seen_at = (c.g[S0][0] as u32).wrapping_add(USB_KB_SEEN - USB_KB_CONNECTED);
                    c.g[V1][0] = Addr(seen_at).lbu(c);
                    c.g[V0][0] = Addr(c.g[S0][0] as u32).lbu(c);
                    c.g[A0][0] = c.g[S1][0];
                    if c.g[V1][0] != c.g[V0][0] {
                        c.cycles += 3;
                        let cb_at = (c.g[S2][0] as u32).wrapping_add(USB_KB_CONNECT_CB);
                        c.g[V0][0] = Addr(cb_at).lw(c);
                        let target = c.g[V0][0] as u32;
                        c.g[A1][0] = Addr(c.g[S0][0] as u32).lbu(c);
                        call_ptr(c, target, 0x003f_5ffc)?;
                    }
                    c.cycles += 5;
                    c.g[V0][0] = Addr((c.g[S2][0] as u32).wrapping_add(USB_KB_MAX)).lw(c);
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
                    c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                    if c.g[V0][0] == 0 {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x003f_5fe0);
                    }
                }
            }
            c.cycles += 5;
            c.g[V1][0] = usb_kb_state_from(c, S4);
            c.g[V0][0] = c.g[S3][0];
            Addr((c.g[V1][0] as u32).wrapping_add(USB_KB_REQUEST)).sw(c, 0);
            Addr((c.g[V1][0] as u32).wrapping_add(USB_KB_BUSY)).sb(c, 0);
            frame.restore(c, S0, 0x60);
        }
        c.cycles += 7;
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore(c, S4, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `int UsbKb_FindNextReady(int first)` (0x3f6040): the first keyboard,
/// from `first` on (wrapping to keyboard 0 past the last), that is connected
/// and whose last data was taken (fresh byte clear); -1 when none is.
pub fn usb_kb_find_next_ready(c: &mut Cpu) {
    c.cycles += 6;
    c.g[V0][0] = usb_kb_lui(USB_KB_STATE);
    c.g[A1][0] = c.g[A0][0];
    c.g[V0][0] = sx(USB_KB_STATE);
    c.g[V1][0] = Addr(USB_KB_STATE + USB_KB_MAX).lw(c);
    c.g[A2][0] = 0;
    if (c.g[V1][0] as i64) > 0 {
        c.cycles += 4;
        c.g[T0][0] = c.g[V0][0];
        c.g[A3][0] = c.g[V1][0];
        c.g[T2][0] = sx((c.g[T0][0] as u32).wrapping_add(USB_KB_CONNECTED));
        c.g[T1][0] = USB_KB_DATA_SIZE;
        loop {
            c.cycles += 6;
            c.g[V0][0] = ((c.g[A1][0] as i64) < (c.g[A3][0] as i64)) as u64;
            if c.g[V0][0] == 0 {
                c.g[A1][0] = 0;
            }
            c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[T2][0] as u32));
            c.g[V0][0] = Addr(c.g[V1][0] as u32).lbu(c);
            let connected = c.g[V0][0] != 0;
            c.mult(0, V0, A1, T1);
            if connected {
                c.cycles += 5;
                c.g[V1][0] = Addr((c.g[T0][0] as u32).wrapping_add(USB_KB_DATA)).lw(c);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(USB_KB_DATA_FRESH)).lbu(c);
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
                if c.g[A0][0] == 0 {
                    c.cycles += 2;
                    c.g[V0][0] = c.g[A1][0];
                    ret(c);
                    return;
                }
            } else {
                c.cycles += 1;
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(1));
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[A2][0] as i64) < (c.g[A3][0] as i64)) as u64;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(1));
            if c.g[V0][0] == 0 {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003f_6068);
            }
        }
    }
    c.cycles += 2;
    c.g[V0][0] = u64::MAX;
    ret(c);
}

/// `void UsbKb_RequestRead(void)` (0x3f60b8): pick the next keyboard ready
/// for a read after the last one read (`UsbKb_FindNextReady`), remember it
/// and its LED and modifier words, and ask the driver for its data
/// (`sceUsbKbRead` into its data entry); when the request went out, mark it
/// pending (request 2, busy).
pub fn usb_kb_request_read(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = usb_kb_lui(USB_KB_STATE);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = sx(USB_KB_STATE);
        c.g[A0][0] = Addr(USB_KB_STATE + USB_KB_READ_PORT).lw(c);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(1));
        callee::USB_KB_FIND_NEXT_READY.call(c, 0x003f_60d8)?;
        c.g[A0][0] = c.g[V0][0];
        let none = c.g[A0][0] == u64::MAX;
        c.g[V0][0] = USB_KB_DATA_SIZE;
        'sent: {
            if !none {
                c.cycles += 12;
                let state = c.g[S0][0] as u32;
                c.g[V1][0] = Addr(state.wrapping_add(USB_KB_DATA)).lw(c);
                c.mult(0, V0, A0, V0);
                Addr(state.wrapping_add(USB_KB_READ_PORT)).sw(c, c.g[A0][0]);
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                let data = c.g[V1][0] as u32;
                c.g[A2][0] = Addr(data.wrapping_add(USB_KB_DATA_LED)).lw(c);
                c.g[A1][0] = c.g[V1][0];
                Addr(state.wrapping_add(USB_KB_PREV_LED)).sw(c, c.g[A2][0]);
                c.g[V0][0] = Addr(data.wrapping_add(USB_KB_DATA_MKEY)).lw(c);
                Addr(state.wrapping_add(USB_KB_PREV_MKEY)).sw(c, c.g[V0][0]);
                callee::SCE_USB_KB_READ.call(c, 0x003f_6110)?;
                if c.g[V0][0] != 0 {
                    frame.restore(c, S0, 0x10);
                    break 'sent;
                }
                c.cycles += 4;
                let state = c.g[S0][0] as u32;
                c.g[V1][0] = USB_KB_REQ_READ;
                c.g[V0][0] = 1;
                Addr(state.wrapping_add(USB_KB_BUSY)).sb(c, c.g[V0][0]);
                Addr(state.wrapping_add(USB_KB_REQUEST)).sw(c, c.g[V1][0]);
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int UsbKb_SyncRead(void)` (0x3f6138): wait for the read request
/// (`sceUsbKbSync(0, &result)`); when that fails return its result. Else
/// mark the read keyboard's data fresh if it brought keys, or its modifier
/// or LED word changed, clear the request and return 0.
pub fn usb_kb_sync_read(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        c.g[A0][0] = USB_KB_SYNC_WAIT;
        frame.save_ra(c, 0x10);
        c.g[A1][0] = c.g[SP][0];
        callee::SCE_USB_KB_SYNC.call(c, 0x003f_614c)?;
        c.g[A3][0] = c.g[V0][0];
        frame.restore_ra(c, 0x10);
        if c.g[A3][0] == 0 {
            c.cycles += 11;
            c.g[V1][0] = usb_kb_lui(USB_KB_STATE);
            c.g[A1][0] = USB_KB_DATA_SIZE;
            c.g[A2][0] = sx(USB_KB_STATE);
            c.g[T0][0] = c.g[V1][0];
            let state = c.g[A2][0] as u32;
            c.g[V0][0] = Addr(state.wrapping_add(USB_KB_READ_PORT)).lw(c);
            c.g[A0][0] = Addr(state.wrapping_add(USB_KB_DATA)).lw(c);
            c.mult(0, V0, V0, A1);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V0][0] as u32));
            let data = c.g[A0][0] as u32;
            c.g[V1][0] = Addr(data.wrapping_add(USB_KB_DATA_LEN)).lw(c);
            c.g[V0][0] = 1;
            let mut fresh = (c.g[V1][0] as i64) > 0;
            if !fresh {
                c.cycles += 4;
                c.g[V1][0] = Addr(data.wrapping_add(USB_KB_DATA_MKEY)).lw(c);
                c.g[V0][0] = Addr(state.wrapping_add(USB_KB_PREV_MKEY)).lw(c);
                fresh = c.g[V1][0] != c.g[V0][0];
                c.g[V0][0] = 1;
                if !fresh {
                    c.cycles += 4;
                    c.g[V1][0] = Addr(state.wrapping_add(USB_KB_PREV_LED)).lw(c);
                    c.g[V0][0] = Addr(data.wrapping_add(USB_KB_DATA_LED)).lw(c);
                    fresh = c.g[V0][0] != c.g[V1][0];
                    c.g[V0][0] = 1;
                }
            }
            if fresh {
                c.cycles += 1;
                Addr(data.wrapping_add(USB_KB_DATA_FRESH)).sb(c, c.g[V0][0]);
            }
            c.cycles += 5;
            c.g[V1][0] = usb_kb_state_from(c, T0);
            c.g[V0][0] = c.g[A3][0];
            Addr((c.g[V1][0] as u32).wrapping_add(USB_KB_REQUEST)).sw(c, 0);
            Addr((c.g[V1][0] as u32).wrapping_add(USB_KB_BUSY)).sb(c, 0);
            frame.restore_ra(c, 0x10);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Start-up and shut-down.

/// `int UsbKb_Init(const char *dir, const char *kb_args, KbData *data,
/// void (*connect_cb)(int, int), int (*hold_cb)(int))` (0x3f61d0): load the
/// modules (`UsbKb_LoadModules(dir, kb_args)`) and start the driver
/// (`sceUsbKbInit`, which writes how many keyboards it serves; at most 8
/// are kept). Clear every keyboard's status, last reported status and fresh
/// byte, keep `data` and the callbacks (`UsbKb_DefaultConnectCb` and
/// `UsbKb_NullCallback` when null; the second name is from
/// `generated/ee-symbols.tsv`, not `functions.tsv`), set the info poll to 60
/// frames, then
/// request the keyboards' status and wait for it. Returns 0, or -1 when a
/// module or the driver failed.
pub fn usb_kb_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A2][0];
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[A3][0];
        frame.save(c, S0, 0x40);
        frame.save_ra(c, 0);
        c.g[S3][0] = c.g[T0][0];
        callee::USB_KB_LOAD_MODULES.call(c, 0x003f_61f8)?;
        'done: {
            'failed: {
                let failed = (c.g[V0][0] as i64) < 0;
                c.g[V0][0] = usb_kb_lui(USB_KB_STATE);
                if failed {
                    break 'failed;
                }
                c.cycles += 6;
                c.g[S0][0] = sx(USB_KB_STATE + USB_KB_MAX);
                c.g[A0][0] = c.g[S0][0];
                callee::SCE_USB_KB_INIT.call(c, 0x003f_620c)?;
                c.g[V1][0] = u64::MAX;
                let started = c.g[V0][0] != c.g[V1][0];
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_sub(USB_KB_MAX));
                if !started {
                    break 'failed;
                }
                usb_kb_init_state(c);
                callee::USB_KB_REQUEST_INFO.call(c, 0x003f_62c4)?;
                loop {
                    c.cycles += 4;
                    callee::USB_KB_SYNC_INFO.call(c, 0x003f_62d0)?;
                    let pending = c.g[V0][0] != 0;
                    c.g[V0][0] = 0;
                    if !pending {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x003f_62c8);
                    }
                }
                break 'done;
            }
            c.cycles += 2;
            c.g[V0][0] = u64::MAX;
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

/// [`usb_kb_init`] once the driver runs (a0 = the state block, s0 = its
/// keyboard-count word `USB_KB_STATE + USB_KB_MAX`, s1 = `data`, s2/s3 = the
/// callbacks): clamp the keyboard count, clear the per-keyboard bytes, set up
/// polling and the callbacks.
pub(crate) fn usb_kb_init_state(c: &mut Cpu) {
    c.cycles += 4;
    let max_at = (c.g[A0][0] as u32).wrapping_add(USB_KB_MAX);
    c.g[V0][0] = Addr(max_at).lw(c);
    c.g[V0][0] = ((c.g[V0][0] as i64) < (USB_KB_MAX_KEYBOARDS + 1) as i64) as u64;
    if c.g[V0][0] != 0 {
        c.g[V0][0] = Addr(max_at).lw(c);
    } else {
        c.cycles += 3;
        c.g[V0][0] = USB_KB_MAX_KEYBOARDS;
        Addr(max_at).sw(c, c.g[V0][0]);
        c.g[V0][0] = Addr(max_at).lw(c);
    }
    c.cycles += 3;
    c.g[V1][0] = 0;
    let none = (c.g[V0][0] as i64) <= 0;
    Addr((c.g[A0][0] as u32).wrapping_add(USB_KB_DATA)).sw(c, c.g[S1][0]);
    if !none {
        c.cycles += 4;
        c.g[A1][0] = c.g[A0][0];
        c.g[A2][0] = sx((c.g[S1][0] as u32).wrapping_add(USB_KB_DATA_FRESH));
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(USB_KB_SEEN - USB_KB_MAX));
        loop {
            c.cycles += 9;
            let status_at = (c.g[A0][0] as u32).wrapping_sub(USB_KB_SEEN - USB_KB_CONNECTED);
            Addr(status_at).sb(c, 0);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(1));
            Addr(c.g[A0][0] as u32).sb(c, 0);
            Addr(c.g[A2][0] as u32).sb(c, 0);
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(1));
            c.g[V0][0] = Addr((c.g[A1][0] as u32).wrapping_add(USB_KB_MAX)).lw(c);
            c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(USB_KB_DATA_SIZE as u32));
            if c.g[V0][0] == 0 {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003f_6258);
            }
        }
    }
    c.cycles += 9;
    c.g[V0][0] = usb_kb_lui(USB_KB_STATE);
    c.g[V1][0] = USB_KB_POLL_FRAMES;
    c.g[A0][0] = sx(USB_KB_STATE);
    c.g[A3][0] = c.g[S2][0];
    let state = c.g[A0][0] as u32;
    Addr(state.wrapping_add(USB_KB_POLL_COUNT)).sw(c, c.g[V1][0]);
    Addr(state.wrapping_add(USB_KB_READ_PORT)).sw(c, 0);
    Addr(state.wrapping_add(USB_KB_POLL_PERIOD)).sw(c, c.g[V1][0]);
    Addr(state.wrapping_add(USB_KB_BUSY)).sb(c, 0);
    if c.g[A3][0] == 0 {
        c.cycles += 2;
        c.g[V0][0] = usb_kb_lui(USB_KB_DEFAULT_CONNECT_CB.addr);
        c.g[A3][0] = sx(USB_KB_DEFAULT_CONNECT_CB.addr);
    }
    c.cycles += 3;
    c.g[T0][0] = c.g[S3][0];
    Addr(state.wrapping_add(USB_KB_CONNECT_CB)).sw(c, c.g[A3][0]);
    if c.g[T0][0] == 0 {
        c.cycles += 2;
        c.g[V0][0] = usb_kb_lui(USB_KB_NULL_CALLBACK);
        c.g[T0][0] = sx(USB_KB_NULL_CALLBACK);
    }
    c.cycles += 3;
    Addr(state.wrapping_add(USB_KB_HOLD_CB)).sw(c, c.g[T0][0]);
}

/// `int UsbKb_Shutdown(void)` (0x3f62f8): stop the driver (`sceUsbKbEnd`)
/// and unload `USBKB.IRX`, then `USBD.IRX`. Returns the two
/// `UsbKb_UnloadModule` results or'ed (1: USBKB failed, 2: USBD failed).
pub fn usb_kb_shutdown(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = usb_kb_lui(USB_KB_STATE);
        callee::SCE_USB_KB_END.call(c, 0x003f_6310)?;
        c.g[V0][0] = usb_kb_state_from(c, S0);
        c.g[A1][0] = USB_KB_UNLOAD_ERR_USBKB;
        c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(USB_KB_USBKB_ID)).lw(c);
        callee::USB_KB_UNLOAD_MODULE.call(c, 0x003f_6320)?;
        let state = usb_kb_state_from(c, S0) as u32;
        c.g[A0][0] = Addr(state.wrapping_add(USB_KB_USBD_ID)).lw(c);
        c.g[S1][0] = c.g[V0][0];
        c.g[A1][0] = USB_KB_UNLOAD_ERR_USBD;
        callee::USB_KB_UNLOAD_MODULE.call(c, 0x003f_6330)?;
        c.g[V0][0] = c.g[S1][0] | c.g[V0][0];
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int UsbKb_GetMaxKeyboards(void)` (0x3f6348): how many keyboards the
/// driver serves (the count `UsbKb_Init` kept, at most 8).
pub fn usb_kb_get_max_keyboards(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V1][0] = usb_kb_lui(USB_KB_STATE);
    c.g[V0][0] = Addr(USB_KB_STATE + USB_KB_MAX).lw(c);
    ret(c);
}

/// `void UsbKb_DefaultConnectCb(int keyboard, int status)` (0x3f6358): the
/// connect callback `UsbKb_Init` installs when given none. For a keyboard
/// that is now connected (`status` non-zero): light num lock
/// (`sceUsbKbSetLEDStatus`) and set LED mode 1 (`sceUsbKbSetLEDMode`), each
/// followed by `sceUsbKbSync(0, &result)` when its request went out; clear
/// the keyboard's buffer, repeat keys after 30 frames then every 2, code
/// type 0.
pub fn usb_kb_default_connect_cb(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save_ra(c, 0x10);
        let connected = c.g[A1][0] != 0;
        c.g[S0][0] = c.g[A0][0];
        if connected {
            c.cycles += 4;
            c.g[A1][0] = USB_KB_LED_NUM_LOCK;
            callee::SCE_USB_KB_SET_LED_STATUS.call(c, 0x003f_6374)?;
            let sent = c.g[V0][0] == 0;
            Addr(sp(c)).sw(c, c.g[V0][0]);
            if sent {
                c.cycles += 3;
                c.g[A0][0] = USB_KB_SYNC_WAIT;
                c.g[A1][0] = c.g[SP][0];
                callee::SCE_USB_KB_SYNC.call(c, 0x003f_6388)?;
            }
            c.cycles += 5;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = USB_KB_CONNECT_LED_MODE;
            callee::SCE_USB_KB_SET_LED_MODE.call(c, 0x003f_6394)?;
            let sent = c.g[V0][0] == 0;
            Addr(sp(c)).sw(c, c.g[V0][0]);
            if sent {
                c.cycles += 3;
                c.g[A0][0] = USB_KB_SYNC_WAIT;
                c.g[A1][0] = c.g[SP][0];
                callee::SCE_USB_KB_SYNC.call(c, 0x003f_63a8)?;
            }
            c.cycles += 9;
            c.g[A0][0] = c.g[S0][0];
            callee::SCE_USB_KB_CLEAR_RBUF.call(c, 0x003f_63b0)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = USB_KB_REPEAT_START;
            c.g[A2][0] = USB_KB_REPEAT_INTERVAL;
            callee::SCE_USB_KB_SET_REPEAT.call(c, 0x003f_63c0)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = USB_KB_CODE_TYPE;
            callee::SCE_USB_KB_SET_CODE_TYPE.call(c, 0x003f_63cc)?;
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Polling.

/// `void UsbKb_Update(void)` (0x3f63f0): the per-frame poll. Finish the
/// pending request: an info request (`UsbKb_SyncInfo`), a read
/// (`UsbKb_SyncRead`), or a hold, which ends when the hold callback (+0xc0)
/// called with 1 says so. With nothing pending, the hold callback called
/// with 0 may hold the library (request 3); else count the frame and request
/// the keyboards' status once the count passes the poll period
/// (`UsbKb_RequestInfo`, count reset), or the next keyboard's data
/// (`UsbKb_RequestRead`).
pub fn usb_kb_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = usb_kb_lui(USB_KB_STATE);
        frame.save(c, S0, 0x20);
        c.g[V1][0] = USB_KB_REQ_INFO;
        frame.save(c, S1, 0x10);
        c.g[S0][0] = sx(USB_KB_STATE);
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr(USB_KB_STATE + USB_KB_REQUEST).lw(c);
        c.g[S1][0] = c.g[V0][0];
        // v0: the request once the pending one is dealt with.
        'pending: {
            'reload: {
                if c.g[A0][0] == USB_KB_REQ_INFO {
                    c.cycles += 4;
                    c.g[S0][0] = usb_kb_state_from(c, S1);
                    callee::USB_KB_SYNC_INFO.call(c, 0x003f_6440)?;
                    let request_at = (c.g[S0][0] as u32).wrapping_add(USB_KB_REQUEST);
                    c.g[V0][0] = Addr(request_at).lw(c);
                    break 'pending;
                }
                c.cycles += 2;
                c.g[V0][0] = USB_KB_REQ_READ;
                if c.g[A0][0] == 0 {
                    break 'reload;
                }
                c.cycles += 2;
                c.g[V0][0] = USB_KB_REQ_HELD;
                if c.g[A0][0] == USB_KB_REQ_READ {
                    c.cycles += 4;
                    c.g[S0][0] = usb_kb_state_from(c, S1);
                    callee::USB_KB_SYNC_READ.call(c, 0x003f_6450)?;
                    let request_at = (c.g[S0][0] as u32).wrapping_add(USB_KB_REQUEST);
                    c.g[V0][0] = Addr(request_at).lw(c);
                    break 'pending;
                }
                c.cycles += 2;
                if c.g[A0][0] == USB_KB_REQ_HELD {
                    let hold_cb_at = (c.g[S0][0] as u32).wrapping_add(USB_KB_HOLD_CB);
                    c.g[V0][0] = Addr(hold_cb_at).lw(c);
                    c.cycles += 4;
                    let target = c.g[V0][0] as u32;
                    c.g[A0][0] = 1;
                    call_ptr(c, target, 0x003f_6460)?;
                    if c.g[V0][0] != 0 {
                        let request_at = (c.g[S0][0] as u32).wrapping_add(USB_KB_REQUEST);
                        Addr(request_at).sw(c, 0);
                    }
                    break 'reload;
                }
                c.cycles += 2;
                c.g[S0][0] = usb_kb_state_from(c, S1);
                c.cycles += 1;
                let request_at = (c.g[S0][0] as u32).wrapping_add(USB_KB_REQUEST);
                c.g[V0][0] = Addr(request_at).lw(c);
                break 'pending;
            }
            c.cycles += 1;
            c.g[S0][0] = usb_kb_state_from(c, S1);
            c.cycles += 1;
            let request_at = (c.g[S0][0] as u32).wrapping_add(USB_KB_REQUEST);
            c.g[V0][0] = Addr(request_at).lw(c);
        }
        c.cycles += 2;
        if c.g[V0][0] == 0 {
            c.cycles += 5;
            let hold_cb_at = (c.g[S0][0] as u32).wrapping_add(USB_KB_HOLD_CB);
            c.g[V0][0] = Addr(hold_cb_at).lw(c);
            let target = c.g[V0][0] as u32;
            c.g[A0][0] = 0;
            call_ptr(c, target, 0x003f_6484)?;
            let held = c.g[V0][0] != 0;
            c.g[V0][0] = USB_KB_REQ_HELD;
            let state = c.g[S0][0] as u32;
            if held {
                c.cycles += 1;
                Addr(state.wrapping_add(USB_KB_REQUEST)).sw(c, c.g[V0][0]);
            }
            c.cycles += 3;
            c.g[V0][0] = Addr(state.wrapping_add(USB_KB_REQUEST)).lw(c);
            if c.g[V0][0] == 0 {
                c.cycles += 6;
                c.g[V0][0] = Addr(state.wrapping_add(USB_KB_POLL_COUNT)).lw(c);
                c.g[V1][0] = Addr(state.wrapping_add(USB_KB_POLL_PERIOD)).lw(c);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(1));
                c.g[V1][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                Addr(state.wrapping_add(USB_KB_POLL_COUNT)).sw(c, c.g[V0][0]);
                if c.g[V1][0] != 0 {
                    c.cycles += 4;
                    Addr(state.wrapping_add(USB_KB_POLL_COUNT)).sw(c, 0);
                    callee::USB_KB_REQUEST_INFO.call(c, 0x003f_64bc)?;
                } else {
                    c.cycles += 3;
                    callee::USB_KB_REQUEST_READ.call(c, 0x003f_64cc)?;
                }
            }
        }
        frame.restore(c, S0, 0x20);
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `KbData *UsbKb_GetData(int keyboard)` (0x3f64e0): the keyboard's data
/// entry when it is fresh (fresh byte 1, which this clears), else null.
pub fn usb_kb_get_data(c: &mut Cpu) {
    c.cycles += 10;
    c.g[V0][0] = USB_KB_DATA_SIZE;
    c.g[V1][0] = usb_kb_lui(USB_KB_STATE);
    c.mult(0, A1, A0, V0);
    c.g[A2][0] = sx(USB_KB_STATE);
    c.g[A0][0] = Addr(USB_KB_STATE + USB_KB_DATA).lw(c);
    c.g[V1][0] = 1;
    c.g[A0][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[A0][0] as u32));
    let fresh_at = (c.g[A0][0] as u32).wrapping_add(USB_KB_DATA_FRESH);
    c.g[V0][0] = Addr(fresh_at).lbu(c);
    if c.g[V0][0] == c.g[V1][0] {
        Addr(fresh_at).sb(c, 0);
        c.cycles += 3;
        c.g[V0][0] = Addr((c.g[A2][0] as u32).wrapping_add(USB_KB_DATA)).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
    } else {
        c.cycles += 2;
        c.g[V0][0] = 0;
    }
    ret(c);
}

/// `bool UsbKb_IsConnected(int keyboard)` (0x3f6520): whether the
/// keyboard's status byte is non-zero.
pub fn usb_kb_is_connected(c: &mut Cpu) {
    c.cycles += 6;
    c.g[V1][0] = sx(USB_KB_STATE.wrapping_add(c.g[A0][0] as u32));
    c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(USB_KB_CONNECTED)).lbu(c);
    c.g[V0][0] = (c.g[V0][0] != 0) as u64;
    ret(c);
}

/// `UsbKb_AnyConnected`'s loop head (a resume entry after an event unwind).
pub(crate) const USB_KB_ANY_CONNECTED_HEAD: u32 = 0x003f_6560;

/// `bool UsbKb_AnyConnected(void)` (0x3f6538): poll (`UsbKb_Update`), then
/// whether any of the served keyboards is connected.
///
/// The original is a `match pc` dispatcher whose loop head (0x3f6560) is
/// also a resume entry after an event unwind; the loop state is in the
/// registers (s0 = the next keyboard, s1 = 1).
pub fn usb_kb_any_connected(c: &mut Cpu) {
    guest(c, |c| {
        let resumed = match c.pc {
            0x003f_6538 => false,
            USB_KB_ANY_CONNECTED_HEAD => true,
            pc => {
                c.jump_out(pc, 0x003f_6538, 0x003f_65a0);
                return Ok(());
            }
        };
        let frame = if resumed {
            Frame { size: 0x30 }
        } else {
            c.cycles += 9;
            let frame = Frame::open(c, 0x30);
            frame.save(c, S0, 0x20);
            frame.save(c, S1, 0x10);
            c.g[S0][0] = 0;
            frame.save_ra(c, 0);
            c.g[S1][0] = 1;
            callee::USB_KB_UPDATE.call(c, 0x003f_6554)?;
            frame
        };
        let mut at_head = resumed;
        loop {
            if at_head {
                c.cycles += 4;
                c.g[A0][0] = c.g[S0][0];
                callee::USB_KB_IS_CONNECTED.call(c, 0x003f_6568)?;
                let connected = c.g[V0][0] == c.g[S1][0];
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(1));
                if connected {
                    c.cycles += 2;
                    c.g[V0][0] = 1;
                    break;
                }
            }
            c.cycles += 5;
            callee::USB_KB_GET_MAX_KEYBOARDS.call(c, 0x003f_6580)?;
            c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
            let more = c.g[V0][0] != 0;
            c.g[V0][0] = 0;
            if !more {
                break;
            }
            if back_edge_unwinds(c, USB_KB_ANY_CONNECTED_HEAD) {
                return Ok(());
            }
            at_head = true;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Keys.

/// `const u8 *UsbKb_GetKeypadEntry(int usage)` (0x3f65a0): the
/// `{usage, key code}` pair of the alternate layout: its own pairs for
/// usages 0x2e-0x35 and 0x87-0x8b, the common table's for every other usage.
pub fn usb_kb_get_keypad_entry(c: &mut Cpu) {
    let usage = c.g[A0][0] as u32;
    let in_range = |first: u32, keys: u64| sx(usage.wrapping_sub(first)) < keys;
    let (table, cost) = if in_range(USB_KB_USAGE_ALT_A, USB_KB_USAGE_ALT_A_KEYS) {
        (USB_KB_KEY_TABLE_ALT_A, 9)
    } else if in_range(USB_KB_USAGE_ALT_B, USB_KB_USAGE_ALT_B_KEYS) {
        (USB_KB_KEY_TABLE_ALT_B, 12)
    } else {
        (USB_KB_KEY_TABLE, 11)
    };
    c.cycles += cost;
    c.g[V1][0] = sx(usage << 1);
    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(table));
    ret(c);
}

/// `void UsbKb_LookupKeyChar(u8 *out, const KbData *data, int layout, int
/// usage)` (0x3f65f8): the key code of `usage` into `*out`: with num lock lit
/// a keypad digit's num-lock code; else the common table's code (layout 0)
/// or the alternate layout's (`UsbKb_GetKeypadEntry`).
pub fn usb_kb_lookup_key_char(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = Addr((c.g[A1][0] as u32).wrapping_add(USB_KB_DATA_LED)).lw(c);
        frame.save(c, S0, 0x10);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] &= USB_KB_LED_NUM_LOCK;
        let num_lock = c.g[V0][0] != 0;
        c.g[A0][0] = c.g[A3][0];
        // v0: the pair.
        'pair: {
            if num_lock {
                c.cycles += 4;
                c.g[V1][0] = sx((c.g[A0][0] as u32).wrapping_sub(USB_KB_USAGE_KEYPAD_1));
                let digit = c.g[V1][0] < USB_KB_KEYPAD_DIGITS;
                c.g[V0][0] = usb_kb_lui(USB_KB_KEY_TABLE_NUMPAD);
                if digit {
                    c.cycles += 4;
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 1);
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(USB_KB_KEY_TABLE_NUMPAD));
                    break 'pair;
                }
            }
            c.cycles += 2;
            c.g[V0][0] = usb_kb_lui(USB_KB_KEY_TABLE);
            if c.g[A2][0] == 0 {
                c.cycles += 4;
                c.g[V1][0] = sx((c.g[A0][0] as u32) << 1);
                c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(USB_KB_KEY_TABLE));
            } else {
                c.cycles += 2;
                callee::USB_KB_GET_KEYPAD_ENTRY.call(c, 0x003f_6658)?;
            }
        }
        c.cycles += 6;
        c.g[V0][0] = Addr((c.g[V0][0] as u32).wrapping_add(1)).lbu(c);
        frame.restore_ra(c, 0);
        Addr(c.g[S0][0] as u32).sb(c, c.g[V0][0]);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void UsbKb_ConvertRawCode(u8 *out, const KbData *data, int arrangement,
/// int usage)` (0x3f6670): the character of `usage` into `out[1]`: a keypad
/// operator's always, a keypad digit's only with num lock lit (else 0);
/// any other key's from `sceUsbKbCnvRawCode(arrangement, modifiers, LEDs,
/// usage)`, 0 when that is no character (256 or more).
pub fn usb_kb_convert_raw_code(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        c.g[V1][0] = sx((c.g[A3][0] as u32).wrapping_sub(USB_KB_USAGE_KEYPAD));
        frame.save(c, S0, 0x10);
        let keypad = c.g[V1][0] < USB_KB_KEYPAD_KEYS;
        c.g[V0][0] = keypad as u64;
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[A2][0];
        if keypad {
            c.cycles += 3;
            let digit = c.g[A3][0] >= USB_KB_USAGE_KEYPAD_1 as u64;
            c.g[V0][0] = usb_kb_lui(USB_KB_KEYPAD_CHARS);
            let mut lit = true;
            if digit {
                c.cycles += 4;
                let led_at = (c.g[A1][0] as u32).wrapping_add(USB_KB_DATA_LED);
                c.g[V0][0] = Addr(led_at).lw(c) & USB_KB_LED_NUM_LOCK;
                lit = c.g[V0][0] != 0;
                c.g[V0][0] = usb_kb_lui(USB_KB_KEYPAD_CHARS);
            }
            let char_at = (c.g[S0][0] as u32).wrapping_add(1);
            if lit {
                c.cycles += 5;
                c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(USB_KB_KEYPAD_CHARS));
                c.g[V1][0] = Addr(c.g[V0][0] as u32).lbu(c);
                Addr(char_at).sb(c, c.g[V1][0]);
            } else {
                c.cycles += 1;
                Addr(char_at).sb(c, 0);
            }
        } else {
            c.cycles += 8;
            let data = c.g[A1][0] as u32;
            c.g[A2][0] = Addr(data.wrapping_add(USB_KB_DATA_LED)).lw(c);
            c.g[A3][0] &= 0xffff;
            c.g[A1][0] = Addr(data.wrapping_add(USB_KB_DATA_MKEY)).lw(c);
            callee::SCE_USB_KB_CNV_RAW_CODE.call(c, 0x003f_66d0)?;
            c.g[V1][0] = c.g[V0][0];
            c.g[V0][0] = ((c.g[V1][0] as i64) < 0x100) as u64;
            let char_at = (c.g[S0][0] as u32).wrapping_add(1);
            if c.g[V0][0] == 0 {
                Addr(char_at).sb(c, 0);
            } else {
                c.cycles += 2;
                Addr(char_at).sb(c, c.g[V1][0]);
            }
        }
        c.cycles += 4;
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `bool UsbKb_CheckModifierKey(u32 mkey, u32 key, u32 *down)` (0x3f6700):
/// whether `key` is a modifier key (shift, control, alt or Windows, left or
/// right); if so `*down` = its bits of the modifier word `mkey`. The generic
/// `VK_SHIFT` and `VK_MENU` test only the left key's bit, `VK_CONTROL` both.
pub fn usb_kb_check_modifier_key(c: &mut Cpu) {
    // The key's modifier bits, and the cycles the compare chain takes to it.
    // The arms follow the generated compare chain: the `key < ...` guards
    // only hold in this order (each one sits where the chain branches on
    // that range), so do not sort or merge them.
    let (bits, cost) = match c.g[A1][0] {
        USB_KB_VK_LSHIFT => (Some(USB_KB_MKEY_LSHIFT), 6),
        USB_KB_VK_MENU => (Some(USB_KB_MKEY_LALT), 10),
        USB_KB_VK_SHIFT => (Some(USB_KB_MKEY_LSHIFT), 15),
        USB_KB_VK_CONTROL => (Some(USB_KB_MKEY_LCTRL | USB_KB_MKEY_RCTRL), 15),
        key if key < USB_KB_VK_MENU => (None, 17),
        USB_KB_VK_LWIN => (Some(USB_KB_MKEY_LWIN), 16),
        USB_KB_VK_RWIN => (Some(USB_KB_MKEY_RWIN), 16),
        key if key < USB_KB_VK_LSHIFT => (None, 18),
        USB_KB_VK_RCONTROL => (Some(USB_KB_MKEY_RCTRL), 12),
        USB_KB_VK_RSHIFT => (Some(USB_KB_MKEY_RSHIFT), 16),
        USB_KB_VK_LCONTROL => (Some(USB_KB_MKEY_LCTRL), 16),
        USB_KB_VK_LMENU => (Some(USB_KB_MKEY_LALT), 16),
        USB_KB_VK_RMENU => (Some(USB_KB_MKEY_RALT), 17),
        _ => (None, 19),
    };
    usb_kb_store_key_bits(c, bits, cost);
}

/// `bool UsbKb_CheckLedKey(u32 led, u32 key, u32 *down)` (0x3f67e0): whether
/// `key` is a lock key (num, caps or scroll lock); if so `*down` = its bit
/// of the LED word `led`.
pub fn usb_kb_check_led_key(c: &mut Cpu) {
    let (bits, cost) = match c.g[A1][0] {
        USB_KB_VK_NUMLOCK => (Some(USB_KB_LED_NUM_LOCK), 5),
        USB_KB_VK_CAPITAL => (Some(USB_KB_LED_CAPS_LOCK), 9),
        key if key < USB_KB_VK_NUMLOCK => (None, 11),
        USB_KB_VK_SCROLL => (Some(USB_KB_LED_SCROLL_LOCK), 10),
        _ => (None, 12),
    };
    usb_kb_store_key_bits(c, bits, cost);
}

/// The common tail of the two key checks: found (a3 = v0 = 1) with
/// v1 = `a0 & bits` stored at `a2`, or not (a3 = v0 = v1 = 0).
pub(crate) fn usb_kb_store_key_bits(c: &mut Cpu, bits: Option<u64>, cost: u64) {
    c.cycles += cost + 2;
    let word = c.g[A0][0];
    c.g[A3][0] = bits.is_some() as u64;
    c.g[V1][0] = bits.map_or(0, |b| word & b);
    if bits.is_some() {
        Addr(c.g[A2][0] as u32).sw(c, c.g[V1][0]);
    }
    c.cycles += 2;
    c.g[V0][0] = c.g[A3][0];
    ret(c);
}

/// The result word in `UsbKb_IsKeyDown`'s frame.
pub(crate) const USB_KB_IS_KEY_DOWN_RESULT: u32 = 0x10;

/// `bool UsbKb_IsKeyDown(const KbData *data, int layout, int index, u32
/// key)` (0x3f6838): whether `key` is down by key code `index` of `data`: a
/// modifier key by the modifier word, a lock key by the LED word, any other
/// key when the code's key code (`UsbKb_LookupKeyChar`) is `key`.
pub fn usb_kb_is_key_down(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x80);
        c.g[A2][0] = sx((c.g[A2][0] as u32) << 1);
        frame.save(c, S0, 0x70);
        frame.save(c, S1, 0x60);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x50);
        c.g[A2][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A2][0] as u32));
        frame.save(c, S4, 0x30);
        c.g[S1][0] = c.g[A3][0];
        frame.save(c, S3, 0x40);
        c.g[S4][0] = c.g[A1][0];
        frame.save_ra(c, 0x20);
        c.g[S2][0] = sx(frame.at(c, USB_KB_IS_KEY_DOWN_RESULT));
        c.g[A1][0] = c.g[S1][0];
        c.g[S3][0] = Addr((c.g[A2][0] as u32).wrapping_add(USB_KB_DATA_KEYS)).lhu(c);
        c.g[A0][0] = Addr((c.g[S0][0] as u32).wrapping_add(USB_KB_DATA_MKEY)).lw(c);
        c.g[A2][0] = c.g[S2][0];
        Addr(frame.at(c, USB_KB_IS_KEY_DOWN_RESULT)).sw(c, 0);
        callee::USB_KB_CHECK_MODIFIER_KEY.call(c, 0x003f_6884)?;
        'done: {
            let handled = c.g[V0][0] != 0;
            c.g[V0][0] = Addr(frame.at(c, USB_KB_IS_KEY_DOWN_RESULT)).lw(c);
            if handled {
                break 'done;
            }
            c.cycles += 6;
            c.g[A0][0] = Addr((c.g[S0][0] as u32).wrapping_add(USB_KB_DATA_LED)).lw(c);
            c.g[A2][0] = c.g[S2][0];
            c.g[A1][0] = c.g[S1][0];
            callee::USB_KB_CHECK_LED_KEY.call(c, 0x003f_689c)?;
            let handled = c.g[V0][0] != 0;
            c.g[V0][0] = Addr(frame.at(c, USB_KB_IS_KEY_DOWN_RESULT)).lw(c);
            if handled {
                break 'done;
            }
            c.cycles += 8;
            c.g[A1][0] = c.g[S0][0];
            c.g[A2][0] = c.g[S4][0];
            c.g[A3][0] = c.g[S3][0];
            c.g[A0][0] = c.g[SP][0];
            callee::USB_KB_LOOKUP_KEY_CHAR.call(c, 0x003f_68b8)?;
            c.g[V0][0] = Addr(sp(c)).lbu(c);
            let other = c.g[V0][0] != c.g[S1][0];
            c.g[V0][0] = Addr(frame.at(c, USB_KB_IS_KEY_DOWN_RESULT)).lw(c);
            if other {
                break 'done;
            }
            c.cycles += 3;
            c.g[V0][0] = 1;
            Addr(frame.at(c, USB_KB_IS_KEY_DOWN_RESULT)).sw(c, c.g[V0][0]);
            c.g[V0][0] = Addr(frame.at(c, USB_KB_IS_KEY_DOWN_RESULT)).lw(c);
        }
        c.cycles += 8;
        frame.restore(c, S0, 0x70);
        frame.restore(c, S1, 0x60);
        frame.restore(c, S2, 0x50);
        frame.restore(c, S3, 0x40);
        frame.restore(c, S4, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `const u8 *UsbKb_TranslateKey(const KbData *data, int arrangement, int
/// index)` (0x3f68f0): key code `index` of `data` as `{key code,
/// character}` (`UsbKb_LookupKeyChar`, `UsbKb_ConvertRawCode`) in a static
/// pair (gp+0x2a20), which it returns.
pub fn usb_kb_translate_key(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 27;
        let frame = Frame::open(c, 0x40);
        c.g[A2][0] = sx((c.g[A2][0] as u32) << 1);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A2][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A2][0] as u32));
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A1][0];
        c.g[S2][0] = Addr((c.g[A2][0] as u32).wrapping_add(USB_KB_DATA_KEYS)).lhu(c);
        c.g[A1][0] = c.g[S0][0];
        c.g[A2][0] = c.g[S1][0];
        c.g[A0][0] = sx(r32(c, GP).wrapping_add(USB_KB_KEY_PAIR_GP));
        c.g[A3][0] = c.g[S2][0];
        callee::USB_KB_LOOKUP_KEY_CHAR.call(c, 0x003f_692c)?;
        c.g[A1][0] = c.g[S0][0];
        c.g[A2][0] = c.g[S1][0];
        c.g[A0][0] = sx(r32(c, GP).wrapping_add(USB_KB_KEY_PAIR_GP));
        c.g[A3][0] = c.g[S2][0];
        callee::USB_KB_CONVERT_RAW_CODE.call(c, 0x003f_6940)?;
        frame.restore(c, S0, 0x30);
        c.g[V0][0] = sx(r32(c, GP).wrapping_add(USB_KB_KEY_PAIR_GP));
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// The voice-chat IOP RPC client.

/// gp-relative byte: the client is bound and calls go out (`VoipRpc_Init`
/// sets it to 1, `VoipRpc_Disable` clears it; `VoipRpc_Call` does nothing
/// while it is 0).
pub(crate) const VOIP_RPC_ENABLED_GP: u32 = 0x2a24;

/// gp-relative word: the uncached address of the status block the IOP
/// module shares with the EE (`VoipRpc_Init` stores it).
pub(crate) const VOIP_RPC_SHARED_STATUS_GP: u32 = 0x2a28;

/// The high half (`lui 0x52`) the code builds the client's addresses from.
pub(crate) const VOIP_RPC_HI: u32 = 0x0052_0000;

/// The send buffer: the two argument words of a call (0x80 bytes sent).
pub(crate) const VOIP_RPC_SEND_BUF: u32 = 0x0052_6080;

/// The SIF RPC client (`sceSifClientData`) bound to the voice server.
pub(crate) const VOIP_RPC_CLIENT: u32 = 0x0052_6100;

/// The receive buffer the commands pass (the IOP writes its answer there).
pub(crate) const VOIP_RPC_RECV_BUF: u32 = 0x0052_6140;

/// The status block shared with the IOP module (its first word's bit 0:
/// voice active).
pub(crate) const VOIP_RPC_SHARED_STATUS: u32 = 0x0052_61c0;

/// Bytes sent from the send buffer, and the receive size the commands pass.
pub(crate) const VOIP_RPC_BUF_SIZE: u64 = 0x80;

/// The IOP voice module's RPC server id.
pub(crate) const VOIP_RPC_SERVER_ID: u64 = 0x237;

/// `addr & KSEG_ADDR_MASK | UNCACHED_SEGMENT`: an address's uncached mirror.
pub(crate) const KSEG_ADDR_MASK: u64 = 0x0fff_ffff;

pub(crate) const UNCACHED_SEGMENT: u64 = 0x2000_0000;

/// Shared status bit 0: voice is active.
pub(crate) const VOIP_SHARED_ACTIVE: u64 = 0x1;

/// RPC command numbers (`fno`).
pub(crate) const VOIP_RPC_CMD_INIT: u64 = 0;

pub(crate) const VOIP_RPC_CMD_START: u64 = 2;

pub(crate) const VOIP_RPC_CMD_STOP: u64 = 3;

pub(crate) const VOIP_RPC_CMD_ENABLE_VOICE: u64 = 4;

pub(crate) const VOIP_RPC_CMD_DISABLE_VOICE: u64 = 5;

pub(crate) const VOIP_RPC_CMD_SET_PARAMS: u64 = 6;

pub(crate) const VOIP_RPC_CMD_GET_STATUS: u64 = 7;

/// Loop heads of `VoipRpc_Call`: waiting for the previous call, retrying
/// `sceSifCallRpc`.
pub(crate) const VOIP_RPC_CALL_WAIT_HEAD: u32 = 0x003f_69c0;

pub(crate) const VOIP_RPC_CALL_RETRY_HEAD: u32 = 0x003f_69e0;

/// `int VoipRpc_Call(int fno, u32 arg0, u32 arg1, void* recv, int recv_size)`
/// (0x3f6960): while the client is enabled, wait while the previous call is
/// still busy (`sceSifCheckStatRpc` > 0), put the two argument words in the
/// send buffer and issue `sceSifCallRpc(client, fno, 0, send, 0x80, recv,
/// recv_size, NULL, NULL)`, retrying while it returns a negative value.
/// Returns the first non-negative `sceSifCallRpc` result; disabled, it
/// returns at once with v0 = 0x520000 left over.
pub fn voip_rpc_call(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0xb0);
        c.g[V0][0] = VOIP_RPC_HI as u64;
        frame.save(c, S2, 0x80);
        frame.save(c, S3, 0x70);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, S4, 0x60);
        c.g[S3][0] = c.g[A1][0];
        frame.save(c, S5, 0x50);
        c.g[S4][0] = c.g[T0][0];
        frame.save(c, S6, 0x40);
        c.g[S5][0] = c.g[A3][0];
        frame.save(c, S7, 0x30);
        c.g[S6][0] = c.g[A0][0];
        frame.save(c, FP, 0x20);
        c.g[S7][0] = sx(VOIP_RPC_SEND_BUF);
        let enabled_at = r32(c, GP).wrapping_add(VOIP_RPC_ENABLED_GP);
        c.g[V1][0] = Addr(enabled_at).lbu(c);
        c.g[FP][0] = c.g[V0][0];
        frame.save(c, S0, 0xa0);
        frame.save(c, S1, 0x90);
        frame.save_ra(c, 0x10);
        if c.g[V1][0] != 0 {
            c.cycles += 3;
            c.g[S1][0] = VOIP_RPC_HI as u64;
            c.g[S0][0] = c.g[S1][0];
            // Wait for the previous call to finish.
            loop {
                c.cycles += 4;
                c.g[A0][0] = sx(r32(c, S0).wrapping_add(VOIP_RPC_CLIENT - VOIP_RPC_HI));
                callee::SCE_SIF_CHECK_STAT_RPC.call(c, 0x003f_69c8)?;
                if (c.g[V0][0] as i64) <= 0 {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, VOIP_RPC_CALL_WAIT_HEAD);
                }
            }
            c.cycles += 4;
            let send = r32(c, S7);
            let (arg0, arg1) = (c.g[S3][0], c.g[S2][0]);
            Addr(send.wrapping_add(4)).sw(c, arg1);
            Addr(send).sw(c, arg0);
            // The ninth argument (end_param), on the stack.
            Addr(sp(c)).sw(c, 0);
            loop {
                c.cycles += 11;
                c.g[A0][0] = sx(r32(c, S1).wrapping_add(VOIP_RPC_CLIENT - VOIP_RPC_HI));
                c.g[A1][0] = c.g[S6][0];
                c.g[A2][0] = 0;
                c.g[A3][0] = sx(r32(c, FP).wrapping_add(VOIP_RPC_SEND_BUF - VOIP_RPC_HI));
                c.g[T0][0] = VOIP_RPC_BUF_SIZE;
                c.g[T1][0] = c.g[S5][0];
                c.g[T2][0] = c.g[S4][0];
                c.g[T3][0] = 0;
                callee::SCE_SIF_CALL_RPC.call(c, 0x003f_6a04)?;
                if (c.g[V0][0] as i64) >= 0 {
                    break;
                }
                Addr(sp(c)).sw(c, 0);
                if event_due(c) {
                    deliver_event_here(c, VOIP_RPC_CALL_RETRY_HEAD);
                }
            }
        }
        c.cycles += 12;
        frame.restore(c, S0, 0xa0);
        frame.restore(c, S1, 0x90);
        frame.restore(c, S2, 0x80);
        frame.restore(c, S3, 0x70);
        frame.restore(c, S4, 0x60);
        frame.restore(c, S5, 0x50);
        frame.restore(c, S6, 0x40);
        frame.restore(c, S7, 0x30);
        frame.restore(c, FP, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void VoipRpc_Init(void)` (0x3f6a40): enable the client, bind it to the
/// IOP voice server (`sceSifBindRpc(client, 0x237, 0)`, result ignored),
/// store the shared status block's uncached address at `gp+0x2a28` and send
/// it to the IOP with command 0 (`VoipRpc_Call(0, status, 0, recv, 0x80)`).
pub fn voip_rpc_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 27;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 1;
        frame.save_ra(c, 0);
        c.g[A0][0] = sx(VOIP_RPC_CLIENT);
        c.g[A1][0] = VOIP_RPC_SERVER_ID;
        c.g[A2][0] = 0;
        let enabled_at = r32(c, GP).wrapping_add(VOIP_RPC_ENABLED_GP);
        let enabled = c.g[V0][0];
        Addr(enabled_at).sb(c, enabled);
        callee::SCE_SIF_BIND_RPC.call(c, 0x003f_6a64)?;
        c.g[V0][0] = KSEG_ADDR_MASK;
        c.g[V1][0] = sx(VOIP_RPC_SHARED_STATUS) & c.g[V0][0];
        c.g[A0][0] = UNCACHED_SEGMENT;
        c.g[V1][0] |= c.g[A0][0];
        c.g[A3][0] = sx(VOIP_RPC_RECV_BUF);
        c.g[A1][0] = c.g[V1][0];
        let status_ptr_at = r32(c, GP).wrapping_add(VOIP_RPC_SHARED_STATUS_GP);
        let status = c.g[V1][0];
        Addr(status_ptr_at).sw(c, status);
        c.g[A0][0] = VOIP_RPC_CMD_INIT;
        c.g[A2][0] = 0;
        c.g[T0][0] = VOIP_RPC_BUF_SIZE;
        callee::VOIP_RPC_CALL.call(c, 0x003f_6aa0)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void VoipRpc_Disable(void)` (0x3f6ab0): clear the enabled flag, so
/// later `VoipRpc_Call`s do nothing.
pub fn voip_rpc_disable(c: &mut Cpu) {
    c.cycles += 2;
    let enabled_at = r32(c, GP).wrapping_add(VOIP_RPC_ENABLED_GP);
    Addr(enabled_at).sb(c, 0);
    ret(c);
}

/// The one-command wrappers' common tail: a 0x10 frame holding `ra`, then
/// `VoipRpc_Call(fno, a1, a2, recv, 0x80)` returning to `ret_to` (the caller
/// has charged its cycles and set a1/a2).
pub(crate) fn voip_rpc_send_command(c: &mut Cpu, fno: u64, ret_to: u32) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A3][0] = sx(VOIP_RPC_RECV_BUF);
        c.g[A0][0] = fno;
        c.g[T0][0] = VOIP_RPC_BUF_SIZE;
        callee::VOIP_RPC_CALL.call(c, ret_to)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void VoipRpc_Start(u32 arg, u32 lo, u32 hi)` (0x3f6ab8): command 2 with
/// the word `arg` and the word `hi << 16 | (lo & 0xffff)`.
pub fn voip_rpc_start(c: &mut Cpu) {
    c.cycles += 14;
    c.g[A1][0] &= 0xffff;
    c.g[A2][0] = sx(r32(c, A2) << 16);
    c.g[A2][0] |= c.g[A1][0];
    c.g[A1][0] = c.g[A0][0];
    voip_rpc_send_command(c, VOIP_RPC_CMD_START, 0x003f_6ae4);
}

/// `void VoipRpc_Stop(void)` (0x3f6af0): command 3, arguments 0.
pub fn voip_rpc_stop(c: &mut Cpu) {
    c.cycles += 12;
    c.g[A1][0] = 0;
    c.g[A2][0] = 0;
    voip_rpc_send_command(c, VOIP_RPC_CMD_STOP, 0x003f_6b14);
}

/// `void VoipRpc_EnableVoice(void)` (0x3f6b20): command 4, arguments 0.
pub fn voip_rpc_enable_voice(c: &mut Cpu) {
    c.cycles += 12;
    c.g[A1][0] = 0;
    c.g[A2][0] = 0;
    voip_rpc_send_command(c, VOIP_RPC_CMD_ENABLE_VOICE, 0x003f_6b44);
}

/// `void VoipRpc_DisableVoice(void)` (0x3f6b50): command 5, arguments 0.
pub fn voip_rpc_disable_voice(c: &mut Cpu) {
    c.cycles += 12;
    c.g[A1][0] = 0;
    c.g[A2][0] = 0;
    voip_rpc_send_command(c, VOIP_RPC_CMD_DISABLE_VOICE, 0x003f_6b74);
}

/// `int VoipRpc_IsActive(void)` (0x3f6b80): bit 0 of the shared status word
/// (`VoipRpc_GetSharedStatus() & 1`).
pub fn voip_rpc_is_active(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        callee::VOIP_RPC_GET_SHARED_STATUS.call(c, 0x003f_6b90)?;
        frame.restore_ra(c, 0);
        c.g[V0][0] &= VOIP_SHARED_ACTIVE;
        frame.close(c);
        Ok(())
    })
}

/// `void VoipRpc_GetStatus(u32* out0, u32* out1)` (0x3f6ba0): command 7
/// (arguments 0) into the receive buffer, then copy its first two words to
/// `*out0` and `*out1`.
pub fn voip_rpc_get_status(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 27;
        let frame = Frame::open(c, 0x50);
        c.g[A2][0] = 0;
        frame.save(c, S1, 0x30);
        c.g[T0][0] = VOIP_RPC_BUF_SIZE;
        frame.save(c, S0, 0x40);
        c.g[S1][0] = VOIP_RPC_HI as u64;
        frame.save(c, S2, 0x20);
        c.g[S0][0] = sx(r32(c, S1).wrapping_add(VOIP_RPC_RECV_BUF - VOIP_RPC_HI));
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[A0][0];
        c.g[S3][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A3][0] = c.g[S0][0];
        c.g[A0][0] = VOIP_RPC_CMD_GET_STATUS;
        c.g[A1][0] = 0;
        callee::VOIP_RPC_CALL.call(c, 0x003f_6be0)?;
        let first_at = r32(c, S1).wrapping_add(VOIP_RPC_RECV_BUF - VOIP_RPC_HI);
        c.g[V0][0] = Addr(first_at).lw(c);
        frame.restore(c, S1, 0x30);
        let (out0, first) = (r32(c, S2), c.g[V0][0]);
        Addr(out0).sw(c, first);
        frame.restore(c, S2, 0x20);
        let second_at = r32(c, S0).wrapping_add(4);
        c.g[V1][0] = Addr(second_at).lw(c);
        frame.restore(c, S0, 0x40);
        let (out1, second) = (r32(c, S3), c.g[V1][0]);
        Addr(out1).sw(c, second);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void VoipRpc_SetParams(u32 arg0, u32 arg1)` (0x3f6c10): command 6 with
/// the two words.
pub fn voip_rpc_set_params(c: &mut Cpu) {
    c.cycles += 12;
    c.g[A2][0] = c.g[A1][0];
    c.g[A1][0] = c.g[A0][0];
    voip_rpc_send_command(c, VOIP_RPC_CMD_SET_PARAMS, 0x003f_6c34);
}

/// `u32 VoipRpc_GetSharedStatus(void)` (0x3f6c40): the first word of the
/// status block the IOP shares (read through the pointer at `gp+0x2a28`).
pub fn voip_rpc_get_shared_status(c: &mut Cpu) {
    c.cycles += 3;
    let ptr_at = r32(c, GP).wrapping_add(VOIP_RPC_SHARED_STATUS_GP);
    c.g[V1][0] = Addr(ptr_at).lw(c);
    let status_at = r32(c, V1);
    c.g[V0][0] = Addr(status_at).lw(c);
    ret(c);
}

/// `void DirtyRpc_IntcHandler(int arg0, void *state)` (0x3f4410):
/// ignore `arg0` (overwritten by the Count read): add the Count
/// difference since the last interrupt to the 64-bit total and latch the
/// current Count, returning 0.
pub fn dirty_rpc_intc_handler(c: &mut Cpu) {
    c.cycles += 13;
    c.g[V0][0] = Addr(c.g[A1][0] as u32).off(0x4df8).lw(c);
    c.mfc0(A0, 9);
    c.g[V1][0] = Addr(c.g[A1][0] as u32).off(0x4e00).ld(c);
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_sub(c.g[V0][0] as u32));
    c.g[V0][0] = c.g[V0][0] << 32;
    c.g[V0][0] = c.g[V0][0] >> 32;
    c.g[V1][0] = c.g[V1][0].wrapping_add(c.g[V0][0]);
    Addr(c.g[A1][0] as u32).off(0x4e00).sd(c, c.g[V1][0]);
    Addr(c.g[A1][0] as u32).off(0x4df8).sw(c, c.g[A0][0]);
    c.ei();
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `void ProtoAries_RpcReplyCallback(void)` (0x3f5a50): stash the Aries
/// reply's `t3` context into the reply slot `*(sp)`, clearing both status
/// words.
pub fn proto_aries_rpc_reply_callback(c: &mut Cpu) {
    c.cycles += 7;
    c.g[V1][0] = Addr(sp(c)).lw(c);
    c.g[V0][0] = 0u64;
    Addr((c.g[V1][0] as u32).wrapping_add(0x10)).sw(c, c.g[T3][0]);
    Addr((c.g[V1][0] as u32).wrapping_add(0xc)).sw(c, 0u64);
    Addr((c.g[V1][0] as u32).wrapping_add(0x8)).sw(c, 0u64);
    ret(c);
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    use crate::difftest::check_paths;
    use crate::difftest::check_paths_lohi;
    use crate::difftest::elf_image;
    use crate::difftest::events::check_events;
    use crate::difftest::events::driven;
    use crate::difftest::events::schedule;
    use crate::difftest::events::Events;
    use crate::difftest::fakes;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::fake_syscall;
    use crate::difftest::fakes::fall_back;
    use crate::difftest::fakes::hook;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::mix;
    use crate::difftest::fakes::prev_cell;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::fakes::LOG_COUNT;
    use crate::difftest::fakes::LOG_RESULTS;
    use crate::difftest::fakes::LOG_START;
    use crate::difftest::fakes::LOG_UNWIND_AT;
    use crate::difftest::fill;
    use crate::difftest::fresh;
    use crate::difftest::put_words;
    use crate::difftest::require_paths;
    use crate::difftest::run_generated;
    use crate::difftest::scramble;
    use crate::difftest::Rng;
    use crate::difftest::CYCLE_BUDGET;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::game::dirtysock::tagsunk::tests::mem_fakes_hook_with_new;
    use crate::game::dirtysock::tagsunk::tests::same_event_cycles;
    use crate::game::dirtysock::tagsunk::tests::MEM_FAKES_CTRL;
    use crate::game::dirtysock::tagsunk::tests::MEM_FAKES_MAGIC;
    use std::collections::BTreeSet;
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// `NetTick`'s clock in the transport state block: the count register at
    /// the base (a word) and the base itself in count ticks (a doubleword).
    pub(crate) const DIRTY_RPC_CLOCK_COUNT: u32 = 0x4df8;

    pub(crate) const DIRTY_RPC_CLOCK_BASE: u32 = 0x4e00;

    /// Write `s` and a NUL at `at`.
    pub(crate) fn put_cstr(c: &mut Cpu, at: u32, s: &[u8]) {
        for (i, b) in s.iter().enumerate() {
            c.bus.write8(at + i as u32, *b);
        }
        c.bus.write8(at + s.len() as u32, 0);
    }

    /// A BuddyApi object at `at` with junk in every word.
    pub(crate) fn buddy_junk_object(c: &mut Cpu, r: &mut Rng, at: u32) {
        fill(c, r, at, BUDDY_SIZE / 4 + 1);
    }

    /// Junk in the stack below the test machine's `sp` (the functions'
    /// stack buffers), so a missing store or NUL shows.
    pub(crate) fn buddy_junk_stack(c: &mut Cpu, r: &mut Rng, bytes: u32) {
        fill(c, r, crate::difftest::SP - bytes, bytes / 4);
    }

    /// Junk in the part of a `frame`-byte frame below the 0x100 bytes
    /// `machine` fills under `sp`, so a first zero store there shows. From
    /// a generator of its own: the fixture's other random values stay.
    pub(crate) fn buddy_api_junk_frame(c: &mut Cpu, seed: u32, frame: u32) {
        let mut r = Rng(mix(seed as u64 ^ 0xf4a3_e000) | 1);
        fill(c, &mut r, crate::difftest::SP - frame, (frame - 0x100) / 4);
    }

    /// An Aries connection at `at` whose +4 points at a 0x20-byte buffer
    /// (`ProtoAriesUnconnect` clears it), stored in the object at DATA.
    pub(crate) fn buddy_aries(c: &mut Cpu, r: &mut Rng, at: u32) {
        fill(c, r, at, 0x10);
        c.bus.write32(at + 4, at + 0x100);
        fill(c, r, at + 0x100, 8);
        c.bus.write32(DATA + BUDDY_ARIES, at);
    }

    /// The object at DATA for `Disconnect` (run by `Destroy` and `Connect`)
    /// on short paths: the Aries connection, no roster, and as asked a
    /// pending lookup (released through a fake), state 5 (Disconnect then
    /// sets 6 before 0), a state callback and the debug hook (fakes).
    pub(crate) fn buddy_connected_object(
        c: &mut Cpu,
        r: &mut Rng,
        lookup: bool,
        state5: bool,
        cb: bool,
        debug: bool,
    ) {
        buddy_junk_object(c, r, DATA);
        buddy_aries(c, r, DATA + 0x1000);
        let pending = DATA + 0x3000;
        fill(c, r, pending, 8);
        c.bus.write32(pending + LOOKUP_RELEASE, FAKE);
        c.bus
            .write32(DATA + BUDDY_LOOKUP, if lookup { pending } else { 0 });
        c.bus
            .write32(DATA + BUDDY_STATE, if state5 { 5 } else { 3 });
        c.bus
            .write32(DATA + BUDDY_STATE_FN, if cb { FAKE + 0x10 } else { 0 });
        c.bus.write32(DATA + BUDDY_ROSTER_LIST, 0);
        c.bus
            .write32(DATA + BUDDY_DEBUG_FN, if debug { FAKE + 0x20 } else { 0 });
    }

    // Scripted fakes for callees with override slots: call n returns script
    // word n (0 allowed), logs its target and a0..t0 and costs a fixed
    // BUDDY_FAKE_CYCLES, so arms through them have known cycle totals. Two
    // gates (magic words in the machine's RAM): the allocator group
    // (operator_new_named, EA_MemFree) and the tag group (TagFieldFind,
    // TagFieldGetString, strcmp, TagFieldGetToken).
    pub(crate) const BUDDY_FAKES_CTRL: u32 = DATA + 0xf_0000;

    pub(crate) const BUDDY_FAKES_MAGIC: u32 = 0xb0dd_7e57;

    pub(crate) const BUDDY_ALLOC_FAKES: u32 = BUDDY_FAKES_CTRL;

    pub(crate) const BUDDY_TAG_FAKES: u32 = BUDDY_FAKES_CTRL + 8;

    pub(crate) const BUDDY_FAKE_COUNT: u32 = BUDDY_FAKES_CTRL + 4;

    pub(crate) const BUDDY_FAKE_SCRIPT: u32 = BUDDY_FAKES_CTRL + 0x10;

    pub(crate) const BUDDY_FAKE_LOG: u32 = BUDDY_FAKES_CTRL + 0x100;

    pub(crate) const BUDDY_FAKE_CYCLES: u64 = 4;

    pub(crate) fn buddy_fake_call(c: &mut Cpu, target: u32) {
        let n = c.bus.read32(BUDDY_FAKE_COUNT);
        c.bus.write32(BUDDY_FAKE_COUNT, n + 1);
        let i = n & 31;
        let log = BUDDY_FAKE_LOG + 0x18 * i;
        c.bus.write32(log, target);
        for (k, reg) in [A0, A1, A2, A3, T0].into_iter().enumerate() {
            c.bus.write32(log + 4 + 4 * k as u32, c.g[reg][0] as u32);
        }
        c.g[V0][0] = sx(c.bus.read32(BUDDY_FAKE_SCRIPT + 4 * i));
        c.cycles += BUDDY_FAKE_CYCLES;
        ret(c);
    }

    pub(crate) fn buddy_gate(c: &mut Cpu, gate: u32) -> bool {
        c.bus.read32(gate) == BUDDY_FAKES_MAGIC
    }

    pub(crate) fn buddy_fake_new(c: &mut Cpu) {
        if !buddy_gate(c, BUDDY_ALLOC_FAKES) {
            return fall_back(c, prev_cell(buddy_fake_new), ee::s10::f_00317e30__generated);
        }
        buddy_fake_call(c, 0x0031_7e30);
    }

    pub(crate) fn buddy_fake_free(c: &mut Cpu) {
        if !buddy_gate(c, BUDDY_ALLOC_FAKES) {
            return fall_back(
                c,
                prev_cell(buddy_fake_free),
                ee::s10::f_00317e98__generated,
            );
        }
        buddy_fake_call(c, 0x0031_7e98);
    }

    pub(crate) fn buddy_fake_find(c: &mut Cpu) {
        if !buddy_gate(c, BUDDY_TAG_FAKES) {
            return fall_back(
                c,
                prev_cell(buddy_fake_find),
                ee::s14::f_003eb6d8__generated,
            );
        }
        buddy_fake_call(c, 0x003e_b6d8);
    }

    /// Also writes the script word's low byte (and a NUL) to the buffer.
    pub(crate) fn buddy_fake_get_string(c: &mut Cpu) {
        if !buddy_gate(c, BUDDY_TAG_FAKES) {
            return fall_back(
                c,
                prev_cell(buddy_fake_get_string),
                ee::s14::f_003ed378__generated,
            );
        }
        let dst = r32(c, A1);
        buddy_fake_call(c, 0x003e_d378);
        c.bus.write8(dst, c.g[V0][0] as u8);
        c.bus.write8(dst + 1, 0);
    }

    pub(crate) fn buddy_fake_strcmp(c: &mut Cpu) {
        if !buddy_gate(c, BUDDY_TAG_FAKES) {
            return fall_back(
                c,
                prev_cell(buddy_fake_strcmp),
                ee::s15::f_004165a8__generated,
            );
        }
        buddy_fake_call(c, 0x0041_65a8);
    }

    pub(crate) fn buddy_fake_get_token(c: &mut Cpu) {
        if !buddy_gate(c, BUDDY_TAG_FAKES) {
            return fall_back(
                c,
                prev_cell(buddy_fake_get_token),
                ee::s14::f_003ed070__generated,
            );
        }
        buddy_fake_call(c, 0x003e_d070);
    }

    /// Hook the fakes (once per process) and open `gate` with `script`.
    pub(crate) fn buddy_scripted(c: &mut Cpu, gate: u32, script: &[u32]) {
        hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E30),
            buddy_fake_new,
            prev_cell(buddy_fake_new),
        );
        hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E98),
            buddy_fake_free,
            prev_cell(buddy_fake_free),
        );
        hook(
            addr_of_mut!(ee::s14::OVERRIDE_003EB6D8),
            buddy_fake_find,
            prev_cell(buddy_fake_find),
        );
        hook(
            addr_of_mut!(ee::s14::OVERRIDE_003ED378),
            buddy_fake_get_string,
            prev_cell(buddy_fake_get_string),
        );
        hook(
            addr_of_mut!(ee::s15::OVERRIDE_004165A8),
            buddy_fake_strcmp,
            prev_cell(buddy_fake_strcmp),
        );
        hook(
            addr_of_mut!(ee::s14::OVERRIDE_003ED070),
            buddy_fake_get_token,
            prev_cell(buddy_fake_get_token),
        );
        c.bus.write32(gate, BUDDY_FAKES_MAGIC);
        c.bus.write32(BUDDY_FAKE_COUNT, 0);
        for (i, w) in script.iter().enumerate() {
            c.bus.write32(BUDDY_FAKE_SCRIPT + 4 * i as u32, *w);
        }
    }

    // -----------------------------------------------------------------------
    // BuddyApi_LookupProduct

    pub(crate) const BUDDY_TEST_PRODUCT: u32 = 0x5353_5833;
    // "SSX3"
    pub(crate) const BUDDY_TEST_OTHER_PRODUCT: u32 = 0x4e42_4133;
    // "NBA3"
    /// Codes "en", "fr" and one with the sign bit set (`lh`, not `lhu`).
    pub(crate) const BUDDY_TEST_CODES: [u16; 3] = [0x656e, 0x6672, 0xfffe];

    /// The product table with `entries` entries: entry i is the test product
    /// when bit i of `ids` is set (another product otherwise), with code
    /// `codes[i]` and a name of 5, 16 or 27 characters.
    pub(crate) fn buddy_product_table(
        c: &mut Cpu,
        r: &mut Rng,
        entries: u32,
        ids: u32,
        codes: &[u16],
    ) {
        for i in 0..entries {
            let e = BUDDY_PRODUCT_TABLE + BUDDY_PRODUCT_ENTRY_SIZE * i;
            let id = if ids >> i & 1 == 1 {
                BUDDY_TEST_PRODUCT
            } else {
                BUDDY_TEST_OTHER_PRODUCT
            };
            c.bus.write32(e, id);
            c.bus.write16(e + PRODUCT_ENTRY_CODE, codes[i as usize]);
            c.bus
                .write16(e + PRODUCT_ENTRY_CODE + 2, r.next_u32() as u16);
            let name = DATA + 0x3000 + 0x40 * i;
            c.bus.write32(e + PRODUCT_ENTRY_NAME, name);
            put_cstr(c, name, &vec![b'a' + i as u8; 5 + 11 * i as usize]);
        }
        c.bus
            .write32(BUDDY_PRODUCT_TABLE + BUDDY_PRODUCT_ENTRY_SIZE * entries, 0);
    }

    /// The arguments: our product, `wanted` codes (`count` of them), the
    /// name buffer (junk-filled) and its size.
    pub(crate) fn buddy_lookup_args(
        c: &mut Cpu,
        r: &mut Rng,
        wanted: &[u16],
        count: i64,
        size: u64,
    ) {
        let codes = DATA + 0x2800;
        for (j, code) in wanted.iter().enumerate() {
            c.bus.write16(codes + 2 * j as u32, *code);
        }
        let name = DATA + 0x2000;
        fill(c, r, name, 0x10);
        c.g[A0][0] = sx(DATA);
        c.g[A1][0] = sx(BUDDY_TEST_PRODUCT);
        c.g[A2][0] = sx(codes);
        c.g[A3][0] = count as u64;
        c.g[T0][0] = sx(name);
        c.g[T1][0] = size;
        c.pc = 0x003e_fe20;
    }

    // Steering: seed % 4 table entries; seed / 4 % 5 the code count (1, 2,
    // 0, 3, -1); seed / 20 % 8 which entries are our product; entry codes
    // from seed / 7, wanted codes from seed / 11; the name size (0x14, 1, 8,
    // 2) from seed / 9. Paths: 15 empty table, 25 one other product, 28 one
    // match with no codes, 39 one match whose code differs.
    #[test]
    fn buddy_api_lookup_product_matches() {
        check_paths(
            ee::s14::f_003efe20__generated,
            buddy_api_lookup_product,
            160,
            &[15, 25, 28, 39],
            &|c, seed| {
                let mut r = machine(c, seed, 0x3e_fe20, 0);
                let codes: Vec<u16> = (0..3)
                    .map(|i| BUDDY_TEST_CODES[((seed / 7 + i) % 3) as usize])
                    .collect();
                buddy_product_table(c, &mut r, seed % 4, seed / 20 % 8, &codes);
                let wanted: Vec<u16> = (0..3)
                    .map(|j| BUDDY_TEST_CODES[((seed / 11 + 2 * j) % 3) as usize])
                    .collect();
                let count = [1i64, 2, 0, 3, -1][(seed / 4 % 5) as usize];
                let size = [0x14u64, 1, 8, 2][(seed / 9 % 4) as usize];
                buddy_lookup_args(c, &mut r, &wanted, count, size);
            },
        );
    }

    driven!(buddy_gen_lookup_product, ee::s14::f_003efe20__generated);

    driven!(buddy_hand_lookup_product, buddy_api_lookup_product);

    // Structured loops (events delivered, never an unwind). (seed / 3) % 2:
    // 0 the first entry is another product, so the entry head (0x3efe58) is
    // the first back edge; 1 it is ours with a code not wanted first, so the
    // code head (0x3efe78) is.
    #[test]
    fn buddy_api_lookup_product_events() {
        let setup = |c: &mut Cpu, seed: u32| {
            let mut r = machine(c, seed, 0x3e_fe21, 0);
            let code_head_first = seed / 3 % 2 == 1;
            let ids = if code_head_first { 0b111 } else { 0b110 };
            buddy_product_table(c, &mut r, 3, ids, &BUDDY_TEST_CODES);
            let wanted = [BUDDY_TEST_CODES[1], BUDDY_TEST_CODES[2], 0x1234];
            buddy_lookup_args(c, &mut r, &wanted, 3, 0x14);
            let mode = schedule(c, &mut r, seed, 0x003e_fe20);
            if mode != Events::Off {
                c.next_event = c.cycles + 1 + (seed / 6 % 4) as u64;
            }
            mode
        };
        check_events(
            buddy_gen_lookup_product,
            buddy_hand_lookup_product,
            36,
            &setup,
        );
        // Both sides must deliver at the same cycles (equal `next_event`):
        // a delivery the port's own loop check misses or adds shows here.
        same_event_cycles(
            buddy_gen_lookup_product,
            buddy_hand_lookup_product,
            36,
            &setup,
            &mut |_, _, _, _, _| {},
        );
    }

    // -----------------------------------------------------------------------
    // BuddyApi_DebugPrintf

    // seed % 2: the debug hook (a fake that sometimes unwinds); seed / 2 % 4:
    // the format (plain, %d:%s:%c, seven conversions reading a2..t3 and the
    // caller's stack, empty). vsprintf runs for real, so no cycle totals.
    #[test]
    fn buddy_api_debug_printf_matches() {
        covered(
            ee::s14::f_003efed0__generated,
            buddy_api_debug_printf,
            48,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3e_fed0, 1);
                elf_image(c);
                buddy_junk_object(c, &mut r, DATA);
                buddy_junk_stack(c, &mut r, 0x1200);
                c.bus
                    .write32(DATA + BUDDY_DEBUG_FN, if seed % 2 == 0 { 0 } else { FAKE });
                let fmt = DATA + 0x1000;
                let text = DATA + 0x1100;
                put_cstr(c, text, b"rider");
                let format: &[u8] = match seed / 2 % 4 {
                    0 => b"plain text\n",
                    1 => b"%d:%s:%c",
                    2 => b"%x %d %d %d %d %d %d",
                    _ => b"",
                };
                put_cstr(c, fmt, format);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(fmt);
                c.g[A2][0] = (r.next_u32() % 100_000) as u64;
                c.g[A3][0] = sx(text);
                c.g[T0][0] = (b'A' as u32 + seed % 26) as u64;
                for reg in [T1, T2, T3] {
                    c.g[reg][0] = sx(r.next_u32());
                }
                c.pc = 0x003e_fed0;
            },
        );
    }

    // -----------------------------------------------------------------------
    // BuddyApi_DebugPacket

    /// Packet data: empty, DEL first, text, tab/LF, CR, a control byte, a
    /// byte >= 0x80 first, exactly 512 and 513 characters, DEL mid-way, a
    /// vertical tab (0x0b: just past tab/LF, so binary).
    pub(crate) fn buddy_debug_data(variant: u32) -> Vec<u8> {
        match variant {
            0 => b"".to_vec(),
            1 => b"\x7fabc".to_vec(),
            2 => b"hello world".to_vec(),
            3 => b"a\tb\nc".to_vec(),
            4 => b"a\rb".to_vec(),
            5 => b"ab\x01cd".to_vec(),
            6 => b"\x80zz".to_vec(),
            7 => vec![b'x'; 512],
            8 => vec![b'y'; 513],
            9 => b"abc\x7fdef".to_vec(),
            _ => b"a\x0bb".to_vec(),
        }
    }

    /// A packet at DATA+0x1100 with `kind`, `code` and `data` (at
    /// DATA+0x2000, junk after it), and the arguments.
    pub(crate) fn buddy_debug_packet_args(
        c: &mut Cpu,
        r: &mut Rng,
        kind: u32,
        code: u32,
        data: &[u8],
    ) {
        let prefix = DATA + 0x1000;
        put_cstr(c, prefix, b"recv: ");
        let packet = DATA + 0x1100;
        let text = DATA + 0x2000;
        fill(c, r, text, 0x90);
        put_cstr(c, text, data);
        c.bus.write32(packet + BUDDY_PACKET_TYPE, r.next_u32());
        c.bus.write32(packet + BUDDY_PACKET_KIND, kind);
        c.bus.write32(packet + BUDDY_PACKET_CODE, code);
        c.bus.write32(packet + BUDDY_PACKET_DATA, text);
        c.g[A0][0] = sx(DATA);
        c.g[A1][0] = sx(prefix);
        c.g[A2][0] = sx(packet);
        c.pc = 0x003e_ff40;
    }

    // seed % 2: the debug hook; seed / 2 % 11: the data; seed / 3 % 3: the
    // kind (-1 event, "PSET", sign bit set); seed / 5 % 4: the code (0, -1,
    // 0xfefefefe, "abcd"). The prints run real vsprintf: no cycle totals.
    #[test]
    fn buddy_api_debug_packet_matches() {
        covered(
            ee::s14::f_003eff40__generated,
            buddy_api_debug_packet,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3e_ff40, 2);
                elf_image(c);
                buddy_junk_object(c, &mut r, DATA);
                buddy_junk_stack(c, &mut r, 0x1200);
                c.bus
                    .write32(DATA + BUDDY_DEBUG_FN, if seed % 2 == 0 { 0 } else { FAKE });
                let kind = [u32::MAX, BUDDY_KIND_PSET, 0x8a8b_8c8d][(seed / 3 % 3) as usize];
                let code =
                    [0, u32::MAX, BUDDY_CODE_DISCONNECTED, 0x6162_6364][(seed / 5 % 4) as usize];
                buddy_debug_packet_args(c, &mut r, kind, code, &buddy_debug_data(seed / 2 % 11));
            },
        );
    }

    driven!(buddy_gen_debug_packet, ee::s14::f_003eff40__generated);

    driven!(buddy_hand_debug_packet, buddy_api_debug_packet);

    // The data scan (0x3f0010, a structured loop: delivered, never an
    // unwind): 600 printable characters and the event due 2500..3700 cycles
    // in, past the header's vsprintf and inside the scan. Deliveries are
    // logged, so some seed must deliver first at the scan head.
    #[test]
    fn buddy_api_debug_packet_events() {
        let setup = |c: &mut Cpu, seed: u32| {
            let mut r = machine(c, seed, 0x3e_ff41, 0);
            elf_image(c);
            buddy_junk_object(c, &mut r, DATA);
            c.bus.write32(DATA + BUDDY_DEBUG_FN, 0);
            buddy_debug_packet_args(c, &mut r, BUDDY_KIND_PSET, 0, &vec![b'q'; 600]);
            let mode = schedule(c, &mut r, seed, 0x003e_ff40);
            if mode != Events::Off {
                c.next_event = c.cycles + 2500 + 300 * (seed / 3 % 5) as u64;
            }
            crate::game::dirtysock::tagsunk::tests::record_deliveries(c);
            mode
        };
        check_events(buddy_gen_debug_packet, buddy_hand_debug_packet, 30, &setup);
        crate::game::dirtysock::tagsunk::tests::require_delivery_heads(
            buddy_gen_debug_packet,
            buddy_hand_debug_packet,
            30,
            &[BUDDY_DEBUG_PACKET_SCAN_HEAD],
            &setup,
        );
        // Both sides must deliver at the same cycles (equal `next_event`):
        // a delivery the port's own loop check misses or adds shows here.
        same_event_cycles(
            buddy_gen_debug_packet,
            buddy_hand_debug_packet,
            30,
            &setup,
            &mut |_, _, _, _, _| {},
        );
    }

    // -----------------------------------------------------------------------
    // BuddyApi_SendPacket

    // seed % 2: the debug hook (DebugPacket then prints through the fake);
    // seed / 2 % 3: the data (empty, a record, 600 characters); seed / 6 % 3:
    // the kind. ProtoAriesSend runs real (its RPC finds no IOP and returns).
    #[test]
    fn buddy_api_send_packet_matches() {
        covered(
            ee::s14::f_003f00d8__generated,
            buddy_api_send_packet,
            36,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3f_00d8, 3);
                elf_image(c);
                buddy_junk_object(c, &mut r, DATA);
                buddy_junk_stack(c, &mut r, 0x1400);
                buddy_aries(c, &mut r, DATA + 0x1000);
                c.bus
                    .write32(DATA + BUDDY_DEBUG_FN, if seed % 2 == 0 { 0 } else { FAKE });
                let data = DATA + 0x2000;
                let text: Vec<u8> = match seed / 2 % 3 {
                    0 => vec![],
                    1 => b"SHOW=CHAT\nSTAT=1\n".to_vec(),
                    _ => vec![b'z'; 600],
                };
                put_cstr(c, data, &text);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx([BUDDY_KIND_PSET, 0x4449_5343, u32::MAX][(seed / 6 % 3) as usize]);
                c.g[A2][0] = sx(data);
                c.pc = 0x003f_00d8;
            },
        );
    }

    // -----------------------------------------------------------------------
    // BuddyApi_StripUserName

    // seed % 8: no separator, '@', '/', both orders, empty, only '@', only '/'.
    #[test]
    fn buddy_api_strip_user_name_matches() {
        check(
            ee::s14::f_003f0150__generated,
            buddy_api_strip_user_name,
            32,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3f_0150, 0);
                let name = DATA + 0x100;
                fill(c, &mut r, name, 0x10);
                let text: &[u8] = match seed % 8 {
                    0 => b"rider",
                    1 => b"rider@ea.com",
                    2 => b"rider/ps2",
                    3 => b"rider@ea.com/ps2",
                    4 => b"rider/ps2@ea",
                    5 => b"",
                    6 => b"@",
                    _ => b"/",
                };
                put_cstr(c, name, text);
                c.g[A0][0] = sx(name);
                c.pc = 0x003f_0150;
            },
        );
    }

    // -----------------------------------------------------------------------
    // BuddyApi_DispatchMessage

    // seed % 8: the message's tags (USER plain, with domain, with domain and
    // resource, missing, "@/", '/' before '@', empty, old DOMN/RSRC fields).
    // The tag functions run real; the message callback is a fake.
    #[test]
    fn buddy_api_dispatch_message_matches() {
        covered(
            ee::s14::f_003f0198__generated,
            buddy_api_dispatch_message,
            48,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3f_0198, 1);
                elf_image(c);
                buddy_junk_object(c, &mut r, DATA);
                buddy_junk_stack(c, &mut r, 0xc00);
                c.bus.write32(DATA + BUDDY_MESSAGE_FN, FAKE);
                let msg = DATA + 0x1000;
                let tags = DATA + 0x2000;
                fill(c, &mut r, msg, 4);
                c.bus.write32(msg + BUDDY_PACKET_DATA, tags);
                let record: &[u8] = match seed % 8 {
                    0 => b"USER=bob\nBODY=hi\n",
                    1 => b"USER=bob@ea.com\nBODY=hi\n",
                    2 => b"BODY=yo\nUSER=bob@ea.com/ps2\n",
                    3 => b"BODY=no user\n",
                    4 => b"USER=@/\n",
                    5 => b"USER=a/b@c\n",
                    6 => b"",
                    _ => b"RSRC=old\nUSER=x@y/z/w\nDOMN=old\n",
                };
                put_cstr(c, tags, record);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(msg);
                c.pc = 0x003f_0198;
            },
        );
    }

    // -----------------------------------------------------------------------
    // BuddyApi_ParsePresence

    pub(crate) const BUDDY_TEST_FOUND: u32 = DATA + 0x5000;

    pub(crate) const BUDDY_TEST_TOKEN: u32 = BUDDY_TEST_PRODUCT;

    /// Scripted runs (tag fakes on): the callees' results in call order,
    /// our code count and codes, and the cycle total (fakes cost 4,
    /// `LookupProduct` with the empty table 15).
    ///
    /// Call order: find SHOW, get SHOW, strcmp per state name, find PROD,
    /// get text; with no text find STAT, get STAT, find P, get token, then
    /// one find per code and, for a found code, get its value.
    pub(crate) const BUDDY_PARSE_SCRIPTS: [(&[u32], u32, [u16; 3], u64); 8] = [
        // State 0, no PROD, text: our product and first code.
        (
            &[BUDDY_TEST_FOUND, 0x43, 0, 0, 0x41],
            2,
            [0x656e, 0x6672, 0],
            117,
        ),
        // No state matches, PROD found, text.
        (
            &[
                BUDDY_TEST_FOUND,
                0x43,
                1,
                1,
                1,
                1,
                1,
                1,
                BUDDY_TEST_FOUND,
                0x41,
            ],
            2,
            [0x656e, 0x6672, 0],
            197,
        ),
        // No text, no codes: the token with our codes.
        (
            &[
                BUDDY_TEST_FOUND,
                0x43,
                0,
                0,
                0,
                BUDDY_TEST_FOUND,
                0x50,
                BUDDY_TEST_FOUND,
                BUDDY_TEST_TOKEN,
            ],
            0,
            [0x656e, 0x6672, 0],
            158,
        ),
        // No text, the first of two codes found.
        (
            &[
                BUDDY_TEST_FOUND,
                0x43,
                0,
                0,
                0,
                BUDDY_TEST_FOUND,
                0x50,
                BUDDY_TEST_FOUND,
                BUDDY_TEST_TOKEN,
                BUDDY_TEST_FOUND,
                0x78,
            ],
            2,
            [0x656e, 0x6672, 0],
            188,
        ),
        // No text, neither of two codes found.
        (
            &[
                BUDDY_TEST_FOUND,
                0x43,
                0,
                0,
                0,
                BUDDY_TEST_FOUND,
                0x50,
                BUDDY_TEST_FOUND,
                BUDDY_TEST_TOKEN,
                0,
                0,
            ],
            2,
            [0x656e, 0x6672, 0],
            201,
        ),
        // State 2, PROD found, text.
        (
            &[BUDDY_TEST_FOUND, 0x43, 1, 1, 0, BUDDY_TEST_FOUND, 0x41],
            2,
            [0x656e, 0x6672, 0],
            150,
        ),
        // No text, a found code whose first byte is 0 (then the token).
        (
            &[
                BUDDY_TEST_FOUND,
                0x43,
                0,
                0,
                0,
                BUDDY_TEST_FOUND,
                0x50,
                BUDDY_TEST_FOUND,
                BUDDY_TEST_TOKEN,
                BUDDY_TEST_FOUND,
                0x78,
            ],
            1,
            [0x0065, 0, 0],
            218,
        ),
        // No text, the third of three codes found.
        (
            &[
                BUDDY_TEST_FOUND,
                0x43,
                0,
                0,
                0,
                BUDDY_TEST_FOUND,
                0x50,
                BUDDY_TEST_FOUND,
                BUDDY_TEST_TOKEN,
                0,
                0,
                BUDDY_TEST_FOUND,
                0x78,
            ],
            3,
            [0x656e, 0x6672, 0x6465],
            230,
        ),
    ];

    /// The object at DATA, the message at DATA+0x1000 (tags at +0xc), the
    /// output record at DATA+0x6000, all junk, and the arguments.
    pub(crate) fn buddy_parse_presence_base(c: &mut Cpu, r: &mut Rng) {
        buddy_junk_object(c, r, DATA);
        buddy_junk_stack(c, r, 0x400);
        let msg = DATA + 0x1000;
        fill(c, r, msg, 4);
        let out = DATA + 0x6000;
        fill(c, r, out, 0x30);
        c.g[A0][0] = sx(DATA);
        c.g[A1][0] = sx(msg);
        c.g[A2][0] = sx(out);
        c.pc = BUDDY_PARSE_ENTRY;
    }

    pub(crate) fn buddy_parse_presence_scripted(c: &mut Cpu, r: &mut Rng, config: usize) {
        buddy_parse_presence_base(c, r);
        let (script, count, codes, _) = BUDDY_PARSE_SCRIPTS[config];
        buddy_scripted(c, BUDDY_TAG_FAKES, script);
        c.bus.write32(DATA + BUDDY_CODE_COUNT, count);
        for (i, code) in codes.iter().enumerate() {
            c.bus.write16(DATA + BUDDY_CODES + 2 * i as u32, *code);
        }
    }

    /// Real runs: real tag functions on a presence message (seed / 2 % 8),
    /// a product table, our code count (seed / 16 % 4) and codes.
    pub(crate) fn buddy_parse_presence_real(c: &mut Cpu, r: &mut Rng, seed: u32) {
        elf_image(c);
        buddy_parse_presence_base(c, r);
        buddy_product_table(c, r, 3, 0b011, &[0x656e, 0x6672, 0x656e]);
        c.bus.write32(DATA + BUDDY_PRODUCT, BUDDY_TEST_PRODUCT);
        c.bus.write32(DATA + BUDDY_CODE_COUNT, seed / 16 % 4);
        let ours: [u16; 3] =
            [[0x6672, 0x656e, 0x6465], [0x6465, 0x6672, 0x656e]][(seed / 64 % 2) as usize];
        for (i, code) in ours.iter().enumerate() {
            c.bus.write16(DATA + BUDDY_CODES + 2 * i as u32, *code);
        }
        let tags = DATA + 0x2000;
        let record: &[u8] = match seed / 2 % 8 {
            0 => b"SHOW=CHAT\nPROD=riding\n",
            1 => b"SHOW=AWAY\nSTAT=\"P=SSX3 fr=Salut\"\n",
            2 => b"SHOW=BUSY\nSTAT=\"P=SSX3 en=Hello de=Hallo\"\n",
            3 => b"STAT=\"P=NBA3\"\n",
            4 => b"SHOW=PASS\nPROD=\nSTAT=\"en=x\"\n",
            5 => b"",
            6 => b"SHOW=DND\nPROD=big air\nSTAT=\"P=SSX3 en=y\"\n",
            _ => b"SHOW=XA\nSTAT=P=SSX3\n",
        };
        put_cstr(c, tags, record);
        c.bus.write32(DATA + 0x1000 + BUDDY_PACKET_DATA, tags);
    }

    // Odd seeds: scripted run (seed / 2 % 8), with the totals listed; even
    // seeds: real tag functions on real records (no totals).
    #[test]
    fn buddy_api_parse_presence_matches() {
        let totals: Vec<u64> = BUDDY_PARSE_SCRIPTS.iter().map(|s| s.3).collect();
        check_paths(
            ee::s14::f_003f02d0__generated,
            buddy_api_parse_presence,
            128,
            &totals,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3f_02d0, 0);
                if seed % 2 == 1 {
                    buddy_parse_presence_scripted(c, &mut r, (seed / 2 % 8) as usize);
                } else {
                    buddy_parse_presence_real(c, &mut r, seed);
                }
            },
        );
    }

    driven!(buddy_gen_parse_presence, ee::s14::f_003f02d0__generated);

    driven!(buddy_hand_parse_presence, buddy_api_parse_presence);

    // First unwind at each head, (seed / 3) % 3: 0 the state-name scan
    // (0x3f0350, script 1: no state matches), 1 the code scan (0x3f04b0,
    // script 4: no code found), 2 a found code (0x3f03f8, script 3). The
    // callees are scripted fakes with no loops, so these are the first back
    // edges; the event is due 1..4 cycles in.
    #[test]
    fn buddy_api_parse_presence_events() {
        check_events(
            buddy_gen_parse_presence,
            buddy_hand_parse_presence,
            36,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3f_02d1, 0);
                let config = [1usize, 4, 3][(seed / 3 % 3) as usize];
                buddy_parse_presence_scripted(c, &mut r, config);
                let mode = schedule(c, &mut r, seed, BUDDY_PARSE_ENTRY);
                if mode != Events::Off {
                    c.next_event = c.cycles + 1 + (seed / 9 % 4) as u64;
                }
                mode
            },
        );
    }

    // -----------------------------------------------------------------------
    // BuddyApi_SendPresence

    // seed % 2: the debug hook; seed / 2 % 2: a product (SetToken); seed / 4
    // % 3: the status record (empty, "P=NBA3", another field); seed / 12 %
    // 2: the presence record; seed / 24 % 2: the presence text.
    #[test]
    fn buddy_api_send_presence_matches() {
        covered(
            ee::s14::f_003f0568__generated,
            buddy_api_send_presence,
            48,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3f_0568, 3);
                elf_image(c);
                buddy_junk_object(c, &mut r, DATA);
                buddy_junk_stack(c, &mut r, 0x1400);
                buddy_aries(c, &mut r, DATA + 0x1000);
                c.bus
                    .write32(DATA + BUDDY_DEBUG_FN, if seed % 2 == 0 { 0 } else { FAKE });
                c.bus.write32(
                    DATA + BUDDY_PRODUCT,
                    if seed / 2 % 2 == 0 {
                        0
                    } else {
                        BUDDY_TEST_PRODUCT
                    },
                );
                let status: &[u8] = match seed / 4 % 3 {
                    0 => b"",
                    1 => b"P=NBA3\n",
                    _ => b"en=Hello world\n",
                };
                put_cstr(c, DATA + BUDDY_STATUS, status);
                let presence: &[u8] = if seed / 12 % 2 == 0 {
                    b"SHOW=CHAT\n"
                } else {
                    b""
                };
                put_cstr(c, DATA + BUDDY_PRESENCE, presence);
                let text: &[u8] = if seed / 24 % 2 == 0 {
                    b""
                } else {
                    b"Big Air at Peak 3"
                };
                put_cstr(c, DATA + BUDDY_PRESENCE_TEXT, text);
                c.g[A0][0] = sx(DATA);
                c.pc = 0x003f_0568;
            },
        );
    }

    // -----------------------------------------------------------------------
    // BuddyApi_SetState

    // seed % 2: a state callback (a fake); the state from seed / 2. Path 9:
    // no callback.
    #[test]
    fn buddy_api_set_state_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            let mut r = machine(c, seed, 0x3f_0610, 1);
            buddy_junk_object(c, &mut r, DATA);
            buddy_junk_stack(c, &mut r, 0x100);
            c.bus
                .write32(DATA + BUDDY_STATE_FN, if seed % 2 == 0 { 0 } else { FAKE });
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = (seed / 2 % 7) as u64;
            c.pc = 0x003f_0610;
        };
        covered(
            ee::s14::f_003f0610__generated,
            buddy_api_set_state,
            32,
            &setup,
        );
        require_paths(ee::s14::f_003f0610__generated, 32, &[9], &setup);
    }

    // -----------------------------------------------------------------------
    // BuddyApi_SetPendingOp

    // seed % 3: the op; seed / 3 % 4: the timeout (0, -1, 1, 7; positive
    // ones run ProtoAriesTick for real). Path 41: no timeout. LO/HI too
    // (the `mult`s for the deadline and the slot offset).
    #[test]
    fn buddy_api_set_pending_op_matches() {
        check_paths_lohi(
            ee::s14::f_003f0658__generated,
            buddy_api_set_pending_op,
            24,
            &[41],
            &|c, seed| {
                let mut r = machine(c, seed, 0x3f_0658, 0);
                elf_image(c);
                buddy_junk_object(c, &mut r, DATA);
                buddy_aries(c, &mut r, DATA + 0x1000);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = (seed % 3) as u64;
                c.g[A2][0] = sx(r.next_u32());
                c.g[A3][0] = sx(r.next_u32());
                c.g[T0][0] = [0i64, -1, 1, 7][(seed / 3 % 4) as usize] as u64;
                c.pc = 0x003f_0658;
            },
        );
    }

    // -----------------------------------------------------------------------
    // BuddyApi_Create

    /// `Create`'s object and the Aries connection `ProtoAriesCreate` gets.
    pub(crate) const BUDDY_CREATE_OBJECT: u32 = DATA + 0x1_0000;

    pub(crate) const BUDDY_CREATE_ARIES: u32 = DATA + 0x2_0000;

    /// seed % 3: 0 the allocation fails (path 33: 12, Lobby_MemAlloc's 10,
    /// the fake's 4, then 7); 1 the object (junk) is cleared and filled and
    /// ProtoAriesCreate's own allocation fails (it returns 0); 2 that
    /// allocation succeeds and the Aries server answers (the DMA-id table
    /// is placed over the connection, as in `proto_aries_create_fixture`,
    /// so the id DirtyRpc_Send stores lands in its handle word), so
    /// ProtoAriesCreate returns the connection and the +4 store writes it
    /// over the cleared field. String lengths from seed / 3 % 3 (5, 63, 80;
    /// the copies keep 63).
    pub(crate) fn buddy_api_create_fixture(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x3f_0710, 0);
        let object = BUDDY_CREATE_OBJECT;
        fill(c, &mut r, object, BUDDY_SIZE / 4 + 1);
        let outcome = seed % 3;
        let first = if outcome == 0 { 0 } else { object };
        let aries = if outcome == 2 { BUDDY_CREATE_ARIES } else { 0 };
        buddy_scripted(c, BUDDY_ALLOC_FAKES, &[first, aries]);
        if outcome == 2 {
            let head = proto_aries_rpc_server(c, &mut r);
            c.bus.write32(
                DIRTY_RPC_STATE + DIRTY_RPC_DMA_IDS,
                aries.wrapping_sub(4 * head),
            );
        }
        let len = [5usize, 63, 80][(seed / 3 % 3) as usize];
        put_cstr(c, DATA, &vec![b'p'; len]);
        put_cstr(c, DATA + 0x100, &vec![b'v'; (len + 7) % 81]);
        c.g[A0][0] = sx(DATA);
        c.g[A1][0] = sx(DATA + 0x100);
        c.pc = 0x003f_0710;
    }

    #[test]
    fn buddy_api_create_matches() {
        check_paths(
            ee::s14::f_003f0710__generated,
            buddy_api_create,
            30,
            &[33],
            &buddy_api_create_fixture,
        );
        // Some seed stores a non-zero connection at +4 (a dropped store
        // leaves the cleared 0 there).
        let mut kept = 0;
        for seed in 0..30 {
            let mut c = fresh();
            buddy_api_create_fixture(&mut c, seed);
            run_generated(&mut c, ee::s14::f_003f0710__generated);
            let stored = c.bus.read32(BUDDY_CREATE_OBJECT + BUDDY_ARIES);
            kept += (stored == BUDDY_CREATE_ARIES) as u32;
        }
        assert!(kept > 0, "no seed's ProtoAriesCreate returned a connection");
    }

    // -----------------------------------------------------------------------
    // BuddyApi_Destroy

    // seed % 2: a pending lookup; seed / 2 % 2: state 5; seed / 4 % 2: a
    // state callback; seed / 8 % 2: the debug hook. Disconnect, FlushRoster
    // and ProtoAriesDestroy run real on short paths; the frees are scripted.
    #[test]
    fn buddy_api_destroy_matches() {
        covered(
            ee::s14::f_003f07a0__generated,
            buddy_api_destroy,
            32,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3f_07a0, 6);
                elf_image(c);
                buddy_scripted(c, BUDDY_ALLOC_FAKES, &[]);
                buddy_connected_object(
                    c,
                    &mut r,
                    seed % 2 == 1,
                    seed / 2 % 2 == 0,
                    seed / 4 % 2 == 1,
                    seed / 8 % 2 == 1,
                );
                buddy_junk_stack(c, &mut r, 0x1400);
                c.g[A0][0] = sx(DATA);
                c.pc = 0x003f_07a0;
            },
        );
    }

    // -----------------------------------------------------------------------
    // BuddyApi_Config

    /// Code strings: four codes, two (the second ending in 'a', the lowest
    /// letter kept), one letter, empty, uppercase, '{' and '|' (>= 'a'), a
    /// byte >= 0x80 after a code, more than four codes (from 'a').
    pub(crate) fn buddy_config_codes(variant: u32) -> &'static [u8] {
        match variant {
            0 => b"enfrdejp",
            1 => b"enja",
            2 => b"e",
            3 => b"",
            4 => b"EN",
            5 => b"en{|",
            6 => b"en\x80a",
            _ => b"abcdefghij",
        }
    }

    /// The object, `pres` and the codes, and the arguments.
    pub(crate) fn buddy_config_args(c: &mut Cpu, r: &mut Rng, pres: &[u8], codes: &[u8]) {
        buddy_junk_object(c, r, DATA);
        let pres_at = DATA + 0x1000;
        let codes_at = DATA + 0x1100;
        fill(c, r, codes_at, 4);
        put_cstr(c, pres_at, pres);
        put_cstr(c, codes_at, codes);
        c.g[A0][0] = sx(DATA);
        c.g[A1][0] = sx(pres_at);
        c.g[A2][0] = sx(r.next_u32());
        c.g[A3][0] = sx(codes_at);
        c.g[T0][0] = sx(r.next_u32());
        c.pc = 0x003f_0818;
    }

    // seed % 8: the codes; seed / 8 % 2: an empty `pres` (Strnzcpy then
    // costs its fixed 12) or one longer than 0x40. Paths with an empty
    // `pres`: 43 no code ("", "EN"), 47 one letter, 66 a code then a byte
    // >= 0x80, 89 two codes, 132 four codes (the loop's own exit).
    #[test]
    fn buddy_api_config_matches() {
        check_paths(
            ee::s14::f_003f0818__generated,
            buddy_api_config,
            64,
            &[43, 47, 66, 89, 132],
            &|c, seed| {
                let mut r = machine(c, seed, 0x3f_0818, 0);
                let pres: &[u8] = if seed / 8 % 2 == 0 {
                    b""
                } else {
                    b"a presence string that is longer than the sixty-four bytes kept"
                };
                buddy_config_args(c, &mut r, pres, buddy_config_codes(seed % 8));
            },
        );
    }

    driven!(buddy_gen_config, ee::s14::f_003f0818__generated);

    driven!(buddy_hand_config, buddy_api_config);

    // The code loop (0x3f0860, structured: delivered, never an unwind): four
    // codes; the event due 1..85 cycles in lands on the first back edge (53
    // cycles in) or a later one.
    #[test]
    fn buddy_api_config_events() {
        let setup = |c: &mut Cpu, seed: u32| {
            let mut r = machine(c, seed, 0x3f_0819, 0);
            buddy_config_args(c, &mut r, b"", b"enfrdejp");
            let mode = schedule(c, &mut r, seed, 0x003f_0818);
            if mode != Events::Off {
                c.next_event = c.cycles + 1 + 12 * (seed / 3 % 8) as u64;
            }
            mode
        };
        check_events(buddy_gen_config, buddy_hand_config, 24, &setup);
        // Both sides must deliver at the same cycles (equal `next_event`):
        // a delivery the port's own loop check misses or adds shows here.
        same_event_cycles(
            buddy_gen_config,
            buddy_hand_config,
            24,
            &setup,
            &mut |_, _, _, _, _| {},
        );
    }

    // -----------------------------------------------------------------------
    // BuddyApi_Connect

    /// Logins: a key, name:secret, name:$key, a ':' past the 255-character
    /// copy, an empty secret (a 6-character name: with the 250-character
    /// user exactly 0x100, one too many to append), a 16-character name
    /// with a secret.
    pub(crate) fn buddy_connect_login(variant: u32) -> Vec<u8> {
        match variant {
            0 => b"LOGINKEY123".to_vec(),
            1 => b"bob:secret".to_vec(),
            2 => b"bob:$keyvalue".to_vec(),
            3 => {
                let mut v = vec![b'a'; 300];
                v.extend_from_slice(b":x");
                v
            }
            4 => b"bobcat:".to_vec(),
            _ => b"riderfromthepeak:pw".to_vec(),
        }
    }

    /// Users: plain, a leading '/', a '/suffix', 250 characters (no room to
    /// append to the 16-character name).
    pub(crate) fn buddy_connect_user(variant: u32) -> Vec<u8> {
        match variant {
            0 => b"Rider".to_vec(),
            1 => b"/Rider".to_vec(),
            2 => b"Rider/2".to_vec(),
            _ => vec![b'n'; 250],
        }
    }

    // seed % 6: the login; seed / 6 % 4: the user; seed / 24 % 2: the lookup
    // allocation fails (scripted 0); seed / 5 % 3: the timeout (0, 1, 30 s);
    // seed / 7 % 2: a new state callback; the Disconnect it starts with from
    // seed / 11, / 13, / 17, / 19 (pending lookup, state 5, old callback,
    // debug hook). Everything else runs real (SocketLookup and the RPCs
    // find no IOP and return). A second run compares LO/HI (the timeout's
    // `mult`; `covered` does not).
    #[test]
    fn buddy_api_connect_matches() {
        let setup = buddy_api_connect_fixture;
        covered(
            ee::s14::f_003f08e8__generated,
            buddy_api_connect,
            96,
            &setup,
        );
        check_lohi(
            ee::s14::f_003f08e8__generated,
            buddy_api_connect,
            96,
            &setup,
        );
    }

    pub(crate) fn buddy_api_connect_fixture(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x3f_08e8, 8);
        elf_image(c);
        buddy_connected_object(
            c,
            &mut r,
            seed / 11 % 2 == 1,
            seed / 13 % 2 == 0,
            seed / 17 % 2 == 1,
            seed / 19 % 2 == 1,
        );
        buddy_junk_stack(c, &mut r, 0x1800);
        let block = DATA + 0x2_0000;
        fill(c, &mut r, block, 0x18);
        let lookup = if seed / 24 % 2 == 0 { block } else { 0 };
        buddy_scripted(c, BUDDY_ALLOC_FAKES, &[lookup]);
        put_cstr(c, DATA + BUDDY_PRES, b"Big Air");
        put_cstr(c, DATA + BUDDY_PRODUCT_NAME, b"SSX3");
        put_cstr(c, DATA + BUDDY_VERSION, b"1.0");
        let host = DATA + 0x4000;
        put_cstr(c, host, b"ps2ssx03.ea.com");
        let login = DATA + 0x4100;
        put_cstr(c, login, &buddy_connect_login(seed % 6));
        let user = DATA + 0x4300;
        put_cstr(c, user, &buddy_connect_user(seed / 6 % 4));
        c.g[A0][0] = sx(DATA);
        c.g[A1][0] = sx(host);
        c.g[A2][0] = sx(r.next_u32());
        c.g[A3][0] = [0u64, 1, 30][(seed / 5 % 3) as usize];
        c.g[T0][0] = sx(login);
        c.g[T1][0] = sx(user);
        c.g[T2][0] = if seed / 7 % 2 == 0 {
            0
        } else {
            sx(FAKE + 0x30)
        };
        c.g[T3][0] = sx(r.next_u32());
        c.pc = 0x003f_08e8;
    }

    // Fixture layout (all inside DATA..DATA+0xfffff, above the fakes'
    // default results).
    pub(crate) const BUDDY_T_CTRL: u32 = DATA + 0xf_f000;

    pub(crate) const BUDDY_T_MAGIC: u32 = 0xb0dd_1e51;

    pub(crate) const BUDDY_T_API: u32 = DATA + 0x4_0000;

    pub(crate) const BUDDY_T_ARIES: u32 = DATA + 0x4_1000;

    pub(crate) const BUDDY_T_HASH: u32 = DATA + 0x4_2000;

    pub(crate) const BUDDY_T_LIST: u32 = DATA + 0x4_3000;

    pub(crate) const BUDDY_T_RECORDS: u32 = DATA + 0x4_4000;

    pub(crate) const BUDDY_T_BUDDY: u32 = DATA + 0x4_8000;

    pub(crate) const BUDDY_T_LOOKUP: u32 = DATA + 0x4_9000;

    pub(crate) const BUDDY_T_TEXT: u32 = DATA + 0x4_a000;

    /// Forced allocation results, 0x4000 apart.
    pub(crate) const BUDDY_T_HEAP: u32 = DATA + 0x5_0000;

    /// `Lobby_HashStrCmpNoCase`, the hash's compare function.
    pub(crate) const BUDDY_T_HASH_CMP: u32 = 0x003e_eab8;

    /// Roster names (distinct under the hash's case folding).
    pub(crate) const BUDDY_T_NAMES: [&[u8]; 5] = [b"Kaori", b"mac", b"Elise", b"zoe", b"Moby"];

    pub(crate) static BUDDY_PREV_NEW: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static BUDDY_PREV_FREE: AtomicUsize = AtomicUsize::new(0);

    /// `operator_new_named` under the buddy fixtures: a logged fake whose
    /// result the fixture forces (`LOG_RESULTS`).
    pub(crate) fn buddy_t_fake_new(c: &mut Cpu) {
        if c.bus.read32(BUDDY_T_CTRL) != BUDDY_T_MAGIC {
            return fakes::fall_back(c, &BUDDY_PREV_NEW, ee::s10::f_00317e30__generated);
        }
        let ra = r32(c, RA);
        fakes::fake(c, 0x0031_7e30, ra);
    }

    /// `EA_MemFree` under the buddy fixtures: logged, nothing freed.
    pub(crate) fn buddy_t_fake_free(c: &mut Cpu) {
        if c.bus.read32(BUDDY_T_CTRL) != BUDDY_T_MAGIC {
            return fakes::fall_back(c, &BUDDY_PREV_FREE, ee::s10::f_00317e98__generated);
        }
        let ra = r32(c, RA);
        fakes::fake(c, 0x0031_7e98, ra);
    }

    /// Hook the allocator fakes and turn them on for this machine.
    pub(crate) fn buddy_fakes_on(c: &mut Cpu) {
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E30),
            buddy_t_fake_new,
            &BUDDY_PREV_NEW,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E98),
            buddy_t_fake_free,
            &BUDDY_PREV_FREE,
        );
        c.bus.write32(BUDDY_T_CTRL, BUDDY_T_MAGIC);
    }

    /// The bucket `Lobby_HashString` picks for `name`.
    pub(crate) fn buddy_t_bucket(name: &[u8], buckets: u32) -> u32 {
        let mut h: u32 = 0;
        for &b in name {
            if (b as i8) >= 33 {
                h = h.wrapping_mul(0x83).wrapping_add((b & 0x5f) as u32);
            }
        }
        (h & 0x7fff_ffff) % buckets
    }

    /// The API object at BUDDY_T_API (random words; the Aries object's RPC
    /// requests fail at once on the empty RPC tables; debug dumps off; a
    /// user name; short presence records; no roster) on a fake-hooked
    /// machine. `a0` = the object, `pc` = `entry`.
    pub(crate) fn buddy_api_fixture(
        c: &mut Cpu,
        seed: u32,
        entry: u32,
        calls: u32,
        elf: bool,
    ) -> Rng {
        let mut r = machine(c, seed, entry as u64, calls);
        if elf {
            elf_image(c);
        }
        buddy_fakes_on(c);
        let api = BUDDY_T_API;
        fill(c, &mut r, api, 0x510 / 4);
        c.bus.write32(api + BUDDY_ARIES, BUDDY_T_ARIES);
        fill(c, &mut r, BUDDY_T_ARIES, 0x48);
        c.bus.write32(BUDDY_T_ARIES + 4, BUDDY_T_ARIES + 0x100);
        c.bus.write32(api + BUDDY_DEBUG_FN, 0);
        c.bus.write32(api + BUDDY_PRODUCT, 0);
        put_cstr(c, api + BUDDY_USER, b"skier");
        put_cstr(c, api + BUDDY_PRESENCE_TEXT, b"riding");
        put_cstr(c, api + BUDDY_STATUS, b"LV=3");
        put_cstr(c, api + BUDDY_PRESENCE, b"SHOW=chat");
        c.bus.write32(api + BUDDY_ROSTER_HASH, 0);
        c.bus.write32(api + BUDDY_ROSTER_LIST, 0);
        c.g[A0][0] = api as u64;
        c.pc = entry;
        r
    }

    /// A roster of `count` records (names from BUDDY_T_NAMES, flags
    /// `flags`) in a Lobby hash of `buckets` chains with its pop cursor at
    /// `cursor`, and a Lobby list whose first `visible` entries are
    /// visible; both pools keep free nodes (no growing).
    pub(crate) fn buddy_roster_fixture(
        c: &mut Cpu,
        r: &mut Rng,
        count: u32,
        buckets: u32,
        cursor: u32,
        visible: u32,
        flags: u32,
    ) {
        let (hash, list) = (BUDDY_T_HASH, BUDDY_T_LIST);
        let (heads, hpool) = (hash + 0x100, hash + 0x200);
        let (entries, lpool) = (list + 0x100, list + 0x200);
        fill(c, r, hash, 0x40);
        fill(c, r, list, 0x40);
        fill(c, r, entries, 0x40);
        for b in 0..buckets {
            c.bus.write32(heads + 4 * b, 0);
        }
        c.bus.write32(hpool, 0);
        c.bus.write32(lpool, 0);
        for i in 0..count {
            let name = BUDDY_T_NAMES[i as usize];
            let rec = BUDDY_T_RECORDS + 0xc0 * i;
            fill(c, r, rec, 0x30);
            put_cstr(c, rec, name);
            put_cstr(c, rec + ROSTER_ENTRY_GROUP, b"crew");
            c.bus.write32(rec + ROSTER_ENTRY_FLAGS, flags);
            let lnode = lpool + 4 + 4 * i;
            c.bus.write32(lnode, rec);
            c.bus.write32(entries + 4 * i, lnode);
            let hnode = hpool + 0xc + 0xc * i;
            let head = heads + 4 * buddy_t_bucket(name, buckets);
            let next = c.bus.read32(head);
            c.bus.write32(hnode, next);
            c.bus.write32(hnode + 4, lnode);
            c.bus.write32(hnode + 8, rec);
            c.bus.write32(head, hnode);
        }
        let (mut hfree, mut lfree) = (0, 0);
        for i in (count..8).rev() {
            let hnode = hpool + 0xc + 0xc * i;
            c.bus.write32(hnode, hfree);
            hfree = hnode;
            let lnode = lpool + 4 + 4 * i;
            c.bus.write32(lnode, lfree);
            lfree = lnode;
        }
        c.bus.write32(hash, count);
        c.bus.write32(hash + 4, cursor);
        c.bus.write32(hash + 0x10, buckets);
        c.bus.write32(hash + 0x14, heads);
        c.bus.write32(hash + 0x18, hpool);
        c.bus.write32(hash + 0x1c, hfree);
        c.bus.write32(hash + 0x20, BUDDY_T_HASH_CMP);
        c.bus.write32(list + 0x10, 8 - count);
        c.bus.write32(list + 0x14, visible);
        c.bus.write32(list + 0x18, count);
        c.bus.write32(list + 0x24, 0);
        c.bus.write32(list + 0x38, lfree);
        c.bus.write32(list + 0x3c, lpool);
        c.bus.write32(list + 0x40, entries);
        c.bus.write32(BUDDY_T_API + BUDDY_ROSTER_HASH, hash);
        c.bus.write32(BUDDY_T_API + BUDDY_ROSTER_LIST, list);
    }

    /// A caller's buddy at `at`: random bytes, then `name`, the group and
    /// `flags`.
    pub(crate) fn buddy_t_buddy(c: &mut Cpu, r: &mut Rng, at: u32, name: &[u8], flags: u32) {
        fill(c, r, at, 0x30);
        put_cstr(c, at, name);
        put_cstr(c, at + ROSTER_ENTRY_GROUP, b"lodge");
        c.bus.write32(at + ROSTER_ENTRY_FLAGS, flags);
    }

    /// Disconnect: with and without a pending lookup, state 5 or another,
    /// with and without a state callback, and no roster / an empty roster /
    /// three buddies with a roster callback. SendPacket, Unconnect,
    /// SetState and FlushRoster run real; the lookup's release, the
    /// callbacks and the frees are fakes (some unwind).
    #[test]
    fn buddy_api_disconnect_matches() {
        fakes::covered(
            ee::s14::f_003f0b50__generated,
            buddy_api_disconnect,
            96,
            &|c, seed| {
                let mut r = buddy_api_fixture(c, seed, 0x003f_0b50, 13, false);
                let api = BUDDY_T_API;
                let lookup = if seed % 2 == 1 { BUDDY_T_LOOKUP } else { 0 };
                fill(c, &mut r, BUDDY_T_LOOKUP, 8);
                c.bus.write32(BUDDY_T_LOOKUP + LOOKUP_RELEASE, FAKE + 0x20);
                c.bus.write32(api + BUDDY_LOOKUP, lookup);
                let state = if seed / 2 % 2 == 1 { 5 } else { 3 };
                c.bus.write32(api + BUDDY_STATE, state);
                let state_proc = if seed / 4 % 2 == 1 { FAKE + 0x10 } else { 0 };
                c.bus.write32(api + BUDDY_STATE_FN, state_proc);
                match seed / 8 % 3 {
                    0 => {}
                    1 => {
                        buddy_roster_fixture(c, &mut r, 0, 2, 1, 0, 4);
                        c.bus.write32(api + BUDDY_ROSTER_FN, 0);
                    }
                    _ => {
                        buddy_roster_fixture(c, &mut r, 3, 2, 1, 2, 4);
                        c.bus.write32(api + BUDDY_ROSTER_FN, FAKE + 0x30);
                    }
                }
            },
        );
    }

    /// The flush loop's setup: no list (10 cycles), an empty roster, or 1-4
    /// buddies in 1-4 buckets with the pop cursor on bucket 0 or 1, with
    /// and without a roster callback. Frees and the callback are fakes.
    pub(crate) fn buddy_flush_roster_setup(c: &mut Cpu, seed: u32, calls: u32) -> Rng {
        let mut r = buddy_api_fixture(c, seed, 0x003f_0be0, calls, false);
        let buckets = 1 + seed / 24 % 4;
        let cursor = (seed / 96 % 2) % buckets;
        match seed % 3 {
            0 => {}
            1 => buddy_roster_fixture(c, &mut r, 0, buckets, cursor, 0, 4),
            _ => {
                let count = 1 + seed / 3 % 4;
                buddy_roster_fixture(c, &mut r, count, buckets, cursor, count.min(2), 4);
            }
        }
        let proc_ = if seed / 12 % 2 == 1 { FAKE + 0x30 } else { 0 };
        c.bus.write32(BUDDY_T_API + BUDDY_ROSTER_FN, proc_);
        r
    }

    #[test]
    fn buddy_api_flush_roster_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            buddy_flush_roster_setup(c, seed, 11);
        };
        fakes::covered(
            ee::s14::f_003f0be0__generated,
            buddy_api_flush_roster,
            192,
            &setup,
        );
        require_paths(ee::s14::f_003f0be0__generated, 192, &[10], &setup);
    }

    thread_local! {
        /// Re-entries of the hand `BuddyApi_FlushRoster` at its pop loop head.
        static FLUSH_ROSTER_HAND_RESUMES: std::cell::Cell<u32> = const { std::cell::Cell::new(0) };
    }

    /// The hand body, counting its own re-entries at 0x3f0c08 (`drive`'s
    /// resume count pools both sides).
    fn buddy_flush_roster_counted(c: &mut Cpu) {
        if c.pc == BUDDY_FLUSH_ROSTER_REMOVE {
            FLUSH_ROSTER_HAND_RESUMES.with(|n| n.set(n.get() + 1));
        }
        buddy_api_flush_roster(c);
    }

    /// Four buddies; the first event lands on the pop loop's head 0x3f0c08
    /// (k up to ~37 cycles after entry) or on a later pass of it (or in a
    /// callee's own loop, delivered there). The hand side itself must unwind
    /// at that head and be re-entered there, as the generated body is.
    #[test]
    fn buddy_api_flush_roster_events() {
        driven!(gen, ee::s14::f_003f0be0__generated);
        driven!(hand, buddy_flush_roster_counted);
        let setup = |c: &mut Cpu, seed: u32| {
            let mut r = buddy_api_fixture(c, seed, 0x003f_0be0, 0, false);
            let buckets = 1 + seed / 3 % 4;
            buddy_roster_fixture(c, &mut r, 4, buckets, seed / 12 % buckets, 2, 4);
            c.bus.write32(BUDDY_T_API + BUDDY_ROSTER_FN, FAKE + 0x30);
            schedule(c, &mut r, seed, 0x003f_0be0)
        };
        FLUSH_ROSTER_HAND_RESUMES.with(|n| n.set(0));
        check_events(gen, hand, 60, &setup);
        let resumed = FLUSH_ROSTER_HAND_RESUMES.with(|n| n.get());
        assert!(resumed > 0, "the hand side never resumed at 0x3f0c08");
        // Both sides must deliver at the same cycles (equal `next_event`):
        // a delivery the port's own loop check misses or adds shows here.
        same_event_cycles(gen, hand, 60, &setup, &mut |_, _, _, _, _| {});
    }

    /// GetRoster: an existing list (22 cycles) or the create path, whose
    /// six allocations (hash, buckets, hash pool, list, list pool, entries)
    /// are fakes with forced results (some unwind). Creation, the tag
    /// records and the packets run real.
    #[test]
    fn buddy_api_get_roster_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            let mut r = buddy_api_fixture(c, seed, 0x003f_0c90, 6, true);
            if seed % 2 == 1 {
                c.bus.write32(BUDDY_T_API + BUDDY_ROSTER_LIST, BUDDY_T_LIST);
            }
            for i in 0..6 {
                let block = BUDDY_T_HEAP + 0x4000 * i;
                fill(c, &mut r, block, 0x10);
                c.bus.write32(LOG_RESULTS + 4 * i, block);
            }
            c.g[A1][0] = (FAKE + 0x40) as u64;
            c.g[A2][0] = r.next_u32() as u64;
        };
        fakes::covered(
            ee::s14::f_003f0c90__generated,
            buddy_api_get_roster,
            32,
            &setup,
        );
        require_paths(ee::s14::f_003f0c90__generated, 32, &[22], &setup);
    }

    /// Add's buddy kinds (`kind`): 0 null (23 cycles), 1 no flags (24),
    /// 2 empty name (29), 3 '/', 4 '@', 5 '.', 6 a known name in other
    /// case, 7 a new name copied aligned, 8 from a 4-aligned buddy, 9 to a
    /// 4-aligned record. `roster`: 0 none, 1 three buddies in one bucket,
    /// 2 in four buckets. The record allocation is a fake with a forced
    /// result.
    pub(crate) fn buddy_add_setup(
        c: &mut Cpu,
        seed: u32,
        kind: u32,
        allow_dot: u32,
        roster: u32,
        flags: u32,
        calls: u32,
    ) -> Rng {
        let mut r = buddy_api_fixture(c, seed, 0x003f_0e38, calls, true);
        buddy_api_junk_frame(c, seed, 0x160);
        c.bus.write32(BUDDY_T_API + BUDDY_ACCEPT_DOTTED, allow_dot);
        match roster {
            0 => {}
            1 => buddy_roster_fixture(c, &mut r, 3, 1, 0, 1, 0x10_0000),
            _ => buddy_roster_fixture(c, &mut r, 3, 4, 2, 2, 0x10_0000),
        }
        let buddy = BUDDY_T_BUDDY + if kind == 8 { 4 } else { 0 };
        let name: &[u8] = match kind {
            2 => b"",
            3 => b"Ka/ori",
            4 => b"kaori@ea",
            5 => b"j.d",
            6 => b"KAORI",
            _ => b"Psymon",
        };
        buddy_t_buddy(c, &mut r, buddy, name, if kind == 1 { 0 } else { flags });
        c.g[A1][0] = if kind == 0 { 0 } else { buddy as u64 };
        let record = BUDDY_T_HEAP + if kind == 9 { 4 } else { 0 };
        fill(c, &mut r, BUDDY_T_HEAP, 0x30);
        c.bus.write32(LOG_RESULTS, record);
        c.g[A2][0] = r.next_u32() as u64;
        c.g[A3][0] = r.next_u32() as u64;
        c.g[T0][0] = sx(seed % 7).wrapping_sub(3);
        r
    }

    /// Record flags the Add/Del buddies carry: no list bit, B+P, I, B+I+P.
    pub(crate) const BUDDY_T_ADD_FLAGS: [u32; 4] = [0x1, 0x80_0004, 0x200, 0x80_0204];

    #[test]
    fn buddy_api_add_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            let flags = BUDDY_T_ADD_FLAGS[(seed / 60 % 4) as usize];
            buddy_add_setup(c, seed, seed % 10, seed / 10 % 2, seed / 20 % 3, flags, 1);
        };
        fakes::covered(ee::s14::f_003f0e38__generated, buddy_api_add, 240, &setup);
        require_paths(ee::s14::f_003f0e38__generated, 240, &[23, 24, 29], &setup);
    }

    /// The cycle count logged at the first fake call (the record
    /// allocation) of `BuddyApi_Add` from the state `setup` builds, events
    /// off.
    pub(crate) fn buddy_add_alloc_cycles(setup: &dyn Fn(&mut Cpu)) -> u64 {
        let mut p = fresh();
        setup(&mut p);
        p.next_event = u64::MAX;
        ee::s14::f_003f0e38__generated(&mut p);
        let lo = p.bus.read32(LOG_START + 8) as u64;
        let hi = p.bus.read32(LOG_START + 12) as u64;
        hi << 32 | lo
    }

    /// New buddies copied aligned (head 0x3f0f80: seed / 3 % 3 == 0) or
    /// unaligned (head 0x3f0f28: from a 4-aligned buddy, == 1, or to a
    /// 4-aligned record, == 2), with a roster and no list bits. The due
    /// event is put 22 cycles after the allocation call (measured on a
    /// probe run of the same state), so the first or second back edge of
    /// the copy loop meets it, past the strchr calls (whose loops unwind
    /// mid-call).
    #[test]
    fn buddy_api_add_events() {
        driven!(gen, ee::s14::f_003f0e38__generated);
        driven!(hand, buddy_api_add);
        let state = |c: &mut Cpu, seed: u32| -> Events {
            let kind = 7 + seed / 3 % 3;
            let mut r = buddy_add_setup(c, seed, kind, seed / 9 % 2, 1 + seed / 18 % 2, 0x1, 0);
            schedule(c, &mut r, seed, 0x003f_0e38)
        };
        let setup = |c: &mut Cpu, seed: u32| {
            let mode = state(c, seed);
            if mode != Events::Off {
                let at = buddy_add_alloc_cycles(&|p| {
                    state(p, seed);
                });
                c.next_event = at + 22;
            }
            mode
        };
        check_events(gen, hand, 60, &setup);
        // Both sides must deliver at the same cycles (equal `next_event`):
        // a delivery the port's own loop check misses or adds shows here.
        same_event_cycles(gen, hand, 60, &setup, &mut |_, _, _, _, _| {});
    }

    /// Del's buddy kinds (seed % 6): null (25 cycles), '/', '@', '.', an
    /// unknown name, the known "ELISE"; '.' allowed or not; no roster (27
    /// cycles) or three buddies in one or four buckets; the roster record's
    /// flags (B+I+P, B+0x100000, B+I) crossed with the buddy's (all, I+P),
    /// so packets go out for each list bit and the record is kept or
    /// removed. The free is a fake.
    #[test]
    fn buddy_api_del_matches() {
        check_paths(
            ee::s14::f_003f11f8__generated,
            buddy_api_del,
            216,
            &[25, 27],
            &|c, seed| {
                let mut r = buddy_api_fixture(c, seed, 0x003f_11f8, 1, true);
                buddy_api_junk_frame(c, seed, 0x170);
                c.bus
                    .write32(BUDDY_T_API + BUDDY_ACCEPT_DOTTED, seed / 6 % 2);
                let kept = [0x80_0204, 0x10_0004, 0x204][(seed / 36 % 3) as usize];
                match seed / 12 % 3 {
                    0 => {}
                    1 => buddy_roster_fixture(c, &mut r, 3, 1, 0, 1, kept),
                    _ => buddy_roster_fixture(c, &mut r, 3, 4, 3, 2, kept),
                }
                let name: &[u8] = match seed % 6 {
                    1 => b"El/ise",
                    2 => b"elise@ea",
                    3 => b"e.lise",
                    4 => b"Psymon",
                    _ => b"ELISE",
                };
                let flags = if seed / 108 % 2 == 0 {
                    0xffff_ffff
                } else {
                    0x80_0200
                };
                buddy_t_buddy(c, &mut r, BUDDY_T_BUDDY, name, flags);
                c.g[A1][0] = if seed % 6 == 0 {
                    0
                } else {
                    BUDDY_T_BUDDY as u64
                };
                c.g[A2][0] = r.next_u32() as u64;
                c.g[A3][0] = r.next_u32() as u64;
                c.g[T0][0] = sx(seed % 5).wrapping_sub(2);
            },
        );
    }

    /// Find: no hash (14 cycles), a null name (15), an unknown name, "mac"
    /// and "ZOE" (other case), in one or four buckets.
    #[test]
    fn buddy_api_find_matches() {
        check_paths(
            ee::s14::f_003f14d8__generated,
            buddy_api_find,
            48,
            &[14, 15],
            &|c, seed| {
                let mut r = buddy_api_fixture(c, seed, 0x003f_14d8, 0, true);
                match seed % 3 {
                    0 => {}
                    1 => buddy_roster_fixture(c, &mut r, 4, 1, 0, 2, 4),
                    _ => buddy_roster_fixture(c, &mut r, 4, 4, 1, 2, 4),
                }
                let name: &[u8] = match seed / 3 % 4 {
                    1 => b"Psymon",
                    2 => b"mac",
                    _ => b"ZOE",
                };
                put_cstr(c, BUDDY_T_TEXT, name);
                c.g[A1][0] = if seed / 3 % 4 == 0 {
                    0
                } else {
                    BUDDY_T_TEXT as u64
                };
            },
        );
    }

    #[test]
    fn buddy_api_clear_presence_matches() {
        check_paths(
            ee::s14::f_003f1538__generated,
            buddy_api_clear_presence,
            8,
            &[3],
            &|c, seed| {
                let mut r = machine(c, seed, 0x3f_1538, 0);
                let api = BUDDY_T_API + 0x10 * (seed % 4);
                fill(c, &mut r, api, 0x60);
                c.g[A0][0] = api as u64;
                c.pc = 0x003f_1538;
            },
        );
    }

    /// SetPresenceText: text off (8 cycles) or on, with texts of 0, 5,
    /// 255 and 300 characters (truncated at 0x100).
    #[test]
    fn buddy_api_set_presence_text_matches() {
        check_paths(
            ee::s14::f_003f1548__generated,
            buddy_api_set_presence_text,
            16,
            &[8],
            &|c, seed| {
                let mut r = buddy_api_fixture(c, seed, 0x003f_1548, 0, false);
                let api = BUDDY_T_API;
                fill(c, &mut r, api + BUDDY_PRESENCE_TEXT, 0x40);
                let on = if seed % 2 == 1 {
                    1 + r.next_u32() % 0xff
                } else {
                    0
                };
                c.bus.write8(api + BUDDY_PRES, on as u8);
                let len = [0, 5, 255, 300][(seed / 2 % 4) as usize];
                let text: Vec<u8> = (0..len).map(|i| b'a' + (i % 26) as u8).collect();
                put_cstr(c, BUDDY_T_TEXT, &text);
                c.g[A1][0] = BUDDY_T_TEXT as u64;
            },
        );
    }

    /// SetPresenceField: names "" (7 cycles), "a" (9), "abc" (12), "ab";
    /// values "" (15 with "ab"), "green", "a b=c".
    #[test]
    fn buddy_api_set_presence_field_matches() {
        check_paths(
            ee::s14::f_003f1578__generated,
            buddy_api_set_presence_field,
            24,
            &[7, 9, 12, 15],
            &|c, seed| {
                buddy_api_fixture(c, seed, 0x003f_1578, 0, true);
                let name: &[u8] = [&b""[..], b"a", b"abc", b"ab"][(seed % 4) as usize];
                let value: &[u8] = [&b""[..], b"green", b"a b=c"][(seed / 4 % 3) as usize];
                put_cstr(c, BUDDY_T_TEXT, name);
                put_cstr(c, BUDDY_T_TEXT + 0x10, value);
                c.g[A1][0] = (BUDDY_T_TEXT + 0x10) as u64;
                c.g[A2][0] = BUDDY_T_TEXT as u64;
            },
        );
    }

    /// SetPresence: shows 0..6 and -1 (in and out of the table), a send
    /// due unset, at 1, far ahead or at 0x8000_0000 (ahead of the small
    /// test tick only under the unsigned compare), delay 0 or large, and
    /// the last show equal or not, so the presence goes out or waits. The
    /// tick, the tag records and SendPresence run real.
    pub(crate) fn buddy_api_set_presence_fixture(c: &mut Cpu, seed: u32) {
        buddy_api_fixture(c, seed, 0x003f_15c8, 0, true);
        let api = BUDDY_T_API;
        let show = [0, 1, 2, 3, 4, 5, 6, 0xffff_ffff][(seed % 8) as usize];
        let due = [0, 1, 0x7fff_ffff, 0x8000_0000][(seed / 8 % 4) as usize];
        c.bus.write32(api + BUDDY_PRESENCE_DUE, due);
        let last = if seed / 48 % 2 == 0 { show } else { show ^ 1 };
        c.bus.write32(api + BUDDY_PRESENCE_SHOW, last);
        put_cstr(c, BUDDY_T_TEXT, b"lodge");
        c.g[A1][0] = BUDDY_T_TEXT as u64;
        c.g[A2][0] = sx(show);
        c.g[A3][0] = if seed / 24 % 2 == 0 { 0 } else { 0x7fff_0000 };
    }

    /// Both arms: SendPresence clears the due word, so a run sent when the
    /// word ends 0 and waited otherwise. Due 0x8000_0000 with the show
    /// unchanged (seeds 24..31) must wait: a signed compare would send.
    #[test]
    fn buddy_api_set_presence_matches() {
        let setup = buddy_api_set_presence_fixture;
        check(
            ee::s14::f_003f15c8__generated,
            buddy_api_set_presence,
            96,
            &setup,
        );
        let (mut sent, mut waited) = (0, 0);
        for seed in 0..96 {
            let mut c = fresh();
            setup(&mut c, seed);
            run_generated(&mut c, ee::s14::f_003f15c8__generated);
            let due = c.bus.read32(BUDDY_T_API + BUDDY_PRESENCE_DUE);
            if due == 0 {
                sent += 1;
            } else {
                waited += 1;
            }
            if seed / 8 % 4 == 3 && seed / 48 % 2 == 0 {
                assert_eq!(due, 0x8000_0000, "seed {seed}: due 0x8000_0000 must wait");
            }
        }
        assert!(sent > 0 && waited > 0, "sent {sent}, waited {waited}");
    }

    /// Send: BODY absent or of 5, 62, 63 or 64 characters; SUBJ absent,
    /// empty or set; timeout 0 or 500. Tags, the pending op and the packet
    /// run real.
    #[test]
    fn buddy_api_send_matches() {
        check(
            ee::s14::f_003f16d8__generated,
            buddy_api_send,
            60,
            &|c, seed| {
                let mut r = buddy_api_fixture(c, seed, 0x003f_16d8, 0, true);
                let mut msg: Vec<u8> = b"USER=mac ".to_vec();
                if let Some(len) =
                    [None, Some(5), Some(62), Some(63), Some(64)][(seed % 5) as usize]
                {
                    msg.extend_from_slice(b"BODY=");
                    msg.extend((0..len).map(|i| b'a' + (i % 26) as u8));
                    msg.push(b' ');
                }
                msg.extend_from_slice([&b""[..], b"SUBJ= ", b"SUBJ=hey "][(seed / 5 % 3) as usize]);
                put_cstr(c, BUDDY_T_TEXT, &msg);
                c.g[A1][0] = BUDDY_T_TEXT as u64;
                c.g[A2][0] = r.next_u32() as u64;
                c.g[A3][0] = r.next_u32() as u64;
                c.g[T0][0] = if seed / 15 % 2 == 0 { 0 } else { 500 };
            },
        );
    }

    #[test]
    fn buddy_api_set_callback_matches() {
        check_paths(
            ee::s14::f_003f17c8__generated,
            buddy_api_set_callback,
            8,
            &[3],
            &|c, seed| {
                let mut r = machine(c, seed, 0x3f_17c8, 0);
                let api = BUDDY_T_API + 0x10 * (seed % 4);
                fill(c, &mut r, api + 0x4a0, 0x10);
                c.g[A0][0] = api as u64;
                c.pc = 0x003f_17c8;
            },
        );
    }

    // Fixture layout (all inside DATA..DATA+0xfffff).
    pub(crate) const BP_SELF: u32 = DATA + 0x1000;

    pub(crate) const BP_CONN: u32 = DATA + 0x2000;

    pub(crate) const BP_ARIES: u32 = DATA + 0x3000;

    pub(crate) const BP_LINK: u32 = DATA + 0x3100;

    pub(crate) const BP_BUF: u32 = DATA + 0x4000;

    pub(crate) const BP_HASH: u32 = DATA + 0x6000;

    pub(crate) const BP_BUCKETS: u32 = DATA + 0x6100;

    pub(crate) const BP_HASH_ENTRIES: u32 = DATA + 0x6200;

    pub(crate) const BP_LIST: u32 = DATA + 0x7000;

    pub(crate) const BP_ARRAY: u32 = DATA + 0x7100;

    pub(crate) const BP_NODES: u32 = DATA + 0x7200;

    /// Roster entries: "amy" is 4- but not 8-aligned (the unaligned copy),
    /// "bob" and "cat" are aligned; "nul" is a hash key whose node has no
    /// entry.
    pub(crate) const BP_AMY: u32 = DATA + 0x8004;

    pub(crate) const BP_BOB: u32 = DATA + 0x8100;

    pub(crate) const BP_CAT: u32 = DATA + 0x8200;

    pub(crate) const BP_NUL_KEY: u32 = DATA + 0x8300;

    /// What the allocator fake returns.
    pub(crate) const BP_ALLOC: u32 = DATA + 0x9000;

    /// Gate of the allocator fakes (+4: the allocation).
    pub(crate) const BP_CTRL: u32 = DATA + 0xf_0000;

    pub(crate) const BP_MAGIC: u32 = 0x4255_4459;

    /// The tick `ProtoAriesTick` returns: `NetTick` reads its reference count
    /// word and cycle base (`(count - word + base) / 0x48000`).
    pub(crate) const BP_NOW: u32 = 1000;

    pub(crate) const BP_CYCLES_PER_TICK: u64 = 0x48000;

    /// Callbacks (fakes) and real leaves returning 0 and -1.
    pub(crate) const BP_FN_POLL: u32 = FAKE;

    pub(crate) const BP_FN_CLOSE: u32 = FAKE + 0x10;

    pub(crate) const BP_FN_STATE: u32 = FAKE + 0x20;

    pub(crate) const BP_FN_DEBUG: u32 = FAKE + 0x30;

    pub(crate) const BP_FN_MSG: u32 = FAKE + 0x40;

    pub(crate) const BP_FN_ROSTER: u32 = FAKE + 0x50;

    /// Op i's callback: `BP_FN_OP + 0x10 * i`.
    pub(crate) const BP_FN_OP: u32 = FAKE + 0x60;

    pub(crate) const BP_RETURN_ZERO: u32 = 0x0014_06c0;

    pub(crate) const BP_RETURN_MINUS_ONE: u32 = 0x0042_4488;
    // `_close`

    pub(crate) const BP_KIND_UNKNOWN: u32 = 0x5a5a_5a5a;
    // "ZZZZ"
    /// The function's frame (entered with the test stack pointer).
    pub(crate) const BP_FRAME: u32 = crate::difftest::SP - BUDDY_PROCESS_FRAME;

    pub(crate) fn buddy_process_fake_new(c: &mut Cpu) {
        if c.bus.read32(BP_CTRL) != BP_MAGIC {
            return fall_back(
                c,
                prev_cell(buddy_process_fake_new),
                ee::s10::f_00317e30__generated,
            );
        }
        c.g[V0][0] = c.bus.read32(BP_CTRL + 4) as u64;
        c.cycles += 7;
        ret(c);
    }

    pub(crate) fn buddy_process_fake_free(c: &mut Cpu) {
        if c.bus.read32(BP_CTRL) != BP_MAGIC {
            return fall_back(
                c,
                prev_cell(buddy_process_fake_free),
                ee::s10::f_00317e98__generated,
            );
        }
        c.cycles += 3;
        ret(c);
    }

    /// `Lobby_MemAlloc`/`Lobby_MemFree` reach `operator_new_named` and
    /// `EA_MemFree`: gated fakes (an entry at `BP_ALLOC`, no-op free).
    pub(crate) fn buddy_process_hook_fakes() {
        hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E30),
            buddy_process_fake_new,
            prev_cell(buddy_process_fake_new),
        );
        hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E98),
            buddy_process_fake_free,
            prev_cell(buddy_process_fake_free),
        );
    }

    pub(crate) fn buddy_process_packet(
        kind: u32,
        code: u32,
        text: &'static [u8],
    ) -> (u32, u32, &'static [u8]) {
        (kind, code, text)
    }

    /// The receive queue: per packet big-endian kind, code and length, the
    /// tag text and one byte the peek overwrites with the terminator (an
    /// empty text: length 12, the peek hands out "").
    pub(crate) fn buddy_process_packets(c: &mut Cpu, packets: &[(u32, u32, &[u8])]) {
        let mut at = BP_BUF;
        for &(kind, code, text) in packets {
            let len = if text.is_empty() {
                12
            } else {
                12 + text.len() as u32 + 1
            };
            for (i, w) in [kind, code, len].iter().enumerate() {
                for (j, b) in w.to_be_bytes().iter().enumerate() {
                    c.bus.write8(at + 4 * i as u32 + j as u32, *b);
                }
            }
            for (i, b) in text.iter().enumerate() {
                c.bus.write8(at + 12 + i as u32, *b);
            }
            if !text.is_empty() {
                c.bus.write8(at + 12 + text.len() as u32, b'~');
            }
            at += len;
        }
        c.bus.write32(BP_ARIES + PROTO_ARIES_RECV_FILL, at - BP_BUF);
    }

    /// The common machine: fakes, the game image (tag names, formats, the
    /// presence tables), a junk-filled `BuddyApi` with every field the
    /// function and its real callees read set, an idle connector, the Aries
    /// connection with an empty queue (pending, so the peek ends at once).
    pub(crate) fn buddy_process_machine(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let mut r = machine(c, seed, salt, 0);
        elf_image(c);
        buddy_api_junk_frame(c, seed, BUDDY_PROCESS_FRAME);
        c.pc = BUDDY_PROCESS_ENTRY;
        c.g[A0][0] = BP_SELF as u64;
        c.bus.write32(BP_CTRL, BP_MAGIC);
        c.bus.write32(BP_CTRL + 4, BP_ALLOC);
        c.bus.write32(DIRTY_RPC_STATE + DIRTY_RPC_CLOCK_COUNT, 0);
        c.bus.write64(
            DIRTY_RPC_STATE + DIRTY_RPC_CLOCK_BASE,
            BP_NOW as u64 * BP_CYCLES_PER_TICK,
        );
        fill(c, &mut r, BP_SELF, 0x600 / 4);
        fill(c, &mut r, BP_ALLOC, 0xc0 / 4);
        let s = BP_SELF;
        for (off, v) in [
            (BUDDY_LOOKUP, 0),
            (BUDDY_ARIES, BP_ARIES),
            (BUDDY_STATE, 0),
            (BUDDY_LOGIN_DEADLINE, 0),
            (BUDDY_PORT, 0x1f90 + seed),
            (BUDDY_ACCEPT_DOTTED, 0),
            (0x1c, 0),  // the presence token (`BuddyApi_SendPresence`)
            (0x308, 0), // the product count (`BuddyApi_ParsePresence`)
            (BUDDY_PRESENCE_DUE, 0),
            (BUDDY_ROSTER_HASH, 0),
            (BUDDY_ROSTER_LIST, 0),
            (BUDDY_MESSAGE_ARG, 0xa1),
            (BUDDY_MESSAGE_FN, 0),
            (BUDDY_ROSTER_ARG, 0xa2),
            (BUDDY_ROSTER_FN, 0),
            (BUDDY_ROSTER_LEFT, 7),
            (BUDDY_STATE_ARG, 0xa3),
            (BUDDY_STATE_FN, 0),
            (BUDDY_DEBUG_ARG, 0xa4),
            (BUDDY_DEBUG_FN, 0),
        ] {
            c.bus.write32(s + off, v);
        }
        for i in 0..3 {
            let op = s + BUDDY_OPS + i * BUDDY_OP_SIZE;
            c.bus.write32(op + BUDDY_OP_TAG, 0x6f70_3000 + i);
            c.bus.write32(op + BUDDY_OP_ID, 5 + i);
            c.bus.write32(op + BUDDY_OP_DEADLINE, 0);
            c.bus.write32(op + BUDDY_OP_ARG, 0xb0 + i);
            c.bus.write32(op + BUDDY_OP_FN, 0);
        }
        c.bus.write16(s + 0x30c, 0);
        c.bus.write8(s + 0x40, 0);
        c.bus.write8(s + 0x140, 0);
        put_cstr(c, s + 0x240, b"PRES=1");
        put_cstr(c, s + BUDDY_LOGIN, b"NAME=me");
        c.bus.write32(BP_CONN + LOOKUP_ADDRESS, 0x0a00_0001 + seed);
        c.bus.write32(BP_CONN + LOOKUP_POLL, BP_RETURN_ZERO);
        c.bus.write32(BP_CONN + LOOKUP_RELEASE, BP_FN_CLOSE);
        c.bus.write32(BP_ARIES, 0x11);
        c.bus.write32(BP_ARIES + PROTO_ARIES_ADDRS, BP_LINK);
        c.bus.write16(BP_LINK, 2); // in use: `ProtoAriesConnect` returns -1 at once
        c.bus.write32(BP_ARIES + PROTO_ARIES_RECV_PENDING, 1);
        c.bus.write32(BP_ARIES + PROTO_ARIES_RECV_READ, 0);
        c.bus.write32(BP_ARIES + PROTO_ARIES_RECV_FILL, 0);
        c.bus.write32(BP_ARIES + PROTO_ARIES_RECV_SIZE, 0x1000);
        c.bus.write32(BP_ARIES + PROTO_ARIES_RECV_BUF, BP_BUF);
        r
    }

    /// `seed % 7 == 3`: a fake callback unwinds (call index `seed / 7 % 3`).
    pub(crate) fn buddy_process_unwind_digit(c: &mut Cpu, seed: u32) {
        if seed % 7 == 3 {
            c.bus.write32(LOG_UNWIND_AT, seed / 7 % 3);
        }
    }

    pub(crate) fn buddy_process_entry(
        c: &mut Cpu,
        r: &mut Rng,
        at: u32,
        name: &[u8],
        flags: u32,
        add_id: u32,
    ) {
        fill(c, r, at, 0xb8 / 4);
        put_cstr(c, at, name);
        c.bus.write8(at + 0x1f, 0);
        put_cstr(c, at + ROSTER_ENTRY_GROUP, b"grp");
        c.bus.write8(at + 0x3f, 0);
        put_cstr(c, at + ROSTER_ENTRY_PRODUCT, b"prod");
        put_cstr(c, at + ROSTER_ENTRY_PRESENCE, b"pres");
        put_cstr(c, at + ROSTER_ENTRY_LANG, b"en");
        c.bus.write8(at + ROSTER_ENTRY_SHOW, 3);
        c.bus.write32(at + ROSTER_ENTRY_FLAGS, flags | 0x40);
        c.bus.write32(at + ROSTER_ENTRY_OP_ID, add_id);
    }

    /// The roster: amy (not adding), bob (adding, id 9), cat (adding, id 7),
    /// nul (no entry), in one hash bucket in that order (compared with
    /// `compare`), the list nodes in that order, two free hash entries and a
    /// free list node.
    pub(crate) fn buddy_process_roster(c: &mut Cpu, r: &mut Rng, compare: u32) {
        c.bus.write32(BP_SELF + BUDDY_ROSTER_HASH, BP_HASH);
        c.bus.write32(BP_SELF + BUDDY_ROSTER_LIST, BP_LIST);
        let adding = ROSTER_FLAG_ADDING as u32;
        let entries: [(u32, &[u8], u32, u32); 4] = [
            (BP_AMY, b"amy", 0, 0),
            (BP_BOB, b"bob", adding, 9),
            (BP_CAT, b"cat", adding, 7),
            (BP_NUL_KEY, b"nul", 0, 0),
        ];
        for (i, &(at, name, flags, add_id)) in entries.iter().enumerate() {
            let i = i as u32;
            buddy_process_entry(c, r, at, name, flags, add_id);
            let node = BP_NODES + 0x10 * i;
            c.bus.write32(node, if i == 3 { 0 } else { at });
            c.bus.write32(BP_ARRAY + 4 * i, node);
            let e = BP_HASH_ENTRIES + 0x10 * i;
            c.bus.write32(e, if i == 3 { 0 } else { e + 0x10 });
            c.bus.write32(e + 4, node);
            c.bus.write32(e + 8, at);
        }
        let (free0, free1) = (BP_HASH_ENTRIES + 0x40, BP_HASH_ENTRIES + 0x50);
        c.bus.write32(free0, free1);
        c.bus.write32(free1, 0);
        for i in 0..0x24 / 4 {
            c.bus.write32(BP_HASH + 4 * i, 0);
        }
        c.bus.write32(BP_HASH, 4);
        c.bus.write32(BP_HASH + 0x10, 1);
        c.bus.write32(BP_HASH + 0x14, BP_BUCKETS);
        c.bus.write32(BP_HASH + 0x1c, free0);
        c.bus.write32(BP_HASH + 0x20, compare);
        c.bus.write32(BP_BUCKETS, BP_HASH_ENTRIES);
        for i in 0..0x48 / 4 {
            c.bus.write32(BP_LIST + 4 * i, 0);
        }
        let free_node = BP_NODES + 0x40;
        c.bus.write32(free_node, 0);
        c.bus.write32(BP_LIST + 0x10, 1);
        c.bus.write32(BP_LIST + 0x14, 4);
        c.bus.write32(BP_LIST + 0x18, 4);
        c.bus.write32(BP_LIST + 0x38, free_node);
        c.bus.write32(BP_LIST + 0x40, BP_ARRAY);
    }

    /// One generated run, for the arm checks: the guest calls made (entry,
    /// a0..a3), the fake callbacks reached, the final machine.
    pub(crate) struct BuddyProcessRun {
        seed: u32,
        calls: Vec<ssx3_ee_rt::prof::trace::Call>,
        fakes: Vec<u32>,
        cycles: u64,
        c: Box<Cpu>,
    }

    impl BuddyProcessRun {
        fn called(&self, f: u32) -> bool {
            self.calls.iter().any(|k| k[0] == f as u64)
        }
        fn count(&self, f: u32) -> usize {
            self.calls.iter().filter(|k| k[0] == f as u64).count()
        }
        fn called_with(&self, f: u32, a1: u64) -> bool {
            self.count_with(f, a1) > 0
        }
        fn count_with(&self, f: u32, a1: u64) -> usize {
            self.calls
                .iter()
                .filter(|k| k[0] == f as u64 && k[2] == a1)
                .count()
        }
        fn faked(&self, target: u32) -> bool {
            self.fakes.contains(&target)
        }
        fn word(&self, at: u32) -> u32 {
            let a = at as usize;
            u32::from_le_bytes(self.c.bus.ram[a..a + 4].try_into().unwrap())
        }
        fn str_at(&self, at: u32, text: &[u8]) -> bool {
            let a = at as usize;
            &self.c.bus.ram[a..a + text.len()] == text && self.c.bus.ram[a + text.len()] == 0
        }
    }

    pub(crate) fn buddy_process_run(seed: u32, setup: &dyn Fn(&mut Cpu, u32)) -> BuddyProcessRun {
        let mut c = fresh();
        setup(&mut c, seed);
        let before = c.cycles;
        ssx3_ee_rt::prof::trace::start();
        run_generated(&mut c, ee::s14::f_003f17d8__generated);
        let calls = ssx3_ee_rt::prof::trace::take();
        let n = c.bus.read32(LOG_COUNT).min(64);
        let fakes = (0..n)
            .map(|i| c.bus.read32(LOG_START + 0x220 * i))
            .collect();
        BuddyProcessRun {
            seed,
            calls,
            fakes,
            cycles: c.cycles - before,
            c,
        }
    }

    /// Every arm (an effect only that arm has) is reached by some seed of the
    /// generated body. Also prints the path cycle totals.
    pub(crate) fn buddy_process_require_arms(
        seeds: u32,
        setup: &dyn Fn(&mut Cpu, u32),
        arms: &[(&str, &dyn Fn(&BuddyProcessRun) -> bool)],
    ) {
        let mut seen = vec![false; arms.len()];
        let mut totals = std::collections::BTreeSet::new();
        for seed in 0..seeds {
            let run = buddy_process_run(seed, setup);
            totals.insert(run.cycles);
            for (i, (_, arm)) in arms.iter().enumerate() {
                if arm(&run) {
                    seen[i] = true;
                }
            }
        }
        eprintln!("BuddyApi_Process: {} path totals: {totals:?}", totals.len());
        let missing: Vec<&str> = arms
            .iter()
            .zip(&seen)
            .filter(|(_, s)| !**s)
            .map(|(a, _)| a.0)
            .collect();
        assert!(missing.is_empty(), "arms never reached: {missing:?}");
    }

    /// Connector poll and login timeout. Digits: state callback set (seed %
    /// 2), connector open (seed / 2 % 2), poll result 0 / > 0 / < 0 (seed / 4
    /// % 3), login deadline none / passed / now (pending) / 0x80000000 (a
    /// negative word: pending in the unsigned compare) (seed / 12 % 4), state
    /// 0 / 5 (seed / 48 % 2).
    pub(crate) fn buddy_process_connect_fixture(c: &mut Cpu, seed: u32) {
        let _r = buddy_process_machine(c, seed, 0x3f17d8);
        buddy_process_unwind_digit(c, seed);
        if seed % 2 == 1 {
            c.bus.write32(BP_SELF + BUDDY_STATE_FN, BP_FN_STATE);
        }
        if seed / 2 % 2 == 1 {
            c.bus.write32(BP_SELF + BUDDY_LOOKUP, BP_CONN);
        }
        let poll = [BP_RETURN_ZERO, BP_FN_POLL, BP_RETURN_MINUS_ONE][(seed / 4 % 3) as usize];
        c.bus.write32(BP_CONN + LOOKUP_POLL, poll);
        let deadline = [0, BP_NOW - 2, BP_NOW, 0x8000_0000][(seed / 12 % 4) as usize];
        c.bus.write32(BP_SELF + BUDDY_LOGIN_DEADLINE, deadline);
        let state = [0, BUDDY_STATE_LINK_UP as u32][(seed / 48 % 2) as usize];
        c.bus.write32(BP_SELF + BUDDY_STATE, state);
    }

    /// `BuddyApi_Process` against the generated body: the connector and
    /// login timeout arms (real `ProtoAries`/`BuddyApi` callees on their
    /// short paths, fake callbacks).
    #[test]
    fn buddy_api_process_connect_matches() {
        buddy_process_hook_fakes();
        check(
            ee::s14::f_003f17d8__generated,
            buddy_api_process,
            96,
            &buddy_process_connect_fixture,
        );
        let poll_digit = |run: &BuddyProcessRun| run.seed / 4 % 3;
        buddy_process_require_arms(
            96,
            &buddy_process_connect_fixture,
            &[
                (
                    "no state fn: connector untouched",
                    &|run: &BuddyProcessRun| {
                        run.seed % 2 == 0 && run.seed / 2 % 2 == 1 && run.word(BP_SELF) == BP_CONN
                    },
                ),
                ("state fn, no connector", &|run: &BuddyProcessRun| {
                    run.seed % 2 == 1 && run.seed / 2 % 2 == 0 && !run.faked(BP_FN_CLOSE)
                }),
                ("poll 0: connector kept", &|run: &BuddyProcessRun| {
                    run.seed % 2 == 1 && poll_digit(run) == 0 && run.word(BP_SELF) == BP_CONN
                }),
                ("poll > 0: connect and log in", &|run: &BuddyProcessRun| {
                    run.called(callee::PROTO_ARIES_CONNECT.addr)
                        && run.called_with(callee::BUDDY_API_SEND_PACKET.addr, sx(BUDDY_KIND_AUTH))
                }),
                ("poll < 0: state 2", &|run: &BuddyProcessRun| {
                    run.called_with(callee::BUDDY_API_SET_STATE.addr, BUDDY_STATE_POLL_FAILED)
                }),
                ("connector closed", &|run: &BuddyProcessRun| {
                    run.faked(BP_FN_CLOSE) && run.word(BP_SELF + BUDDY_LOOKUP) == 0
                }),
                ("login timed out: state 3", &|run: &BuddyProcessRun| {
                    run.called_with(callee::BUDDY_API_SET_STATE.addr, BUDDY_STATE_LOGIN_TIMEOUT)
                }),
            ],
        );
    }

    /// Op slots: N no fn, Z fn without deadline, F fn pending (deadline =
    /// now), P fn past its deadline (now - 1).
    pub(crate) const BUDDY_PROCESS_OP_PATTERNS: [&[u8; 3]; 6] =
        [b"NNN", b"ZFN", b"FFP", b"PNN", b"NZF", b"FPF"];

    /// Presence, op timeouts and op cancels. Digits: state 5 / 6 / 3 / 0
    /// (seed % 4), presence due none / now (due) / now + 1 (seed / 4 % 3),
    /// op pattern (seed / 12 % 6), state callback (seed / 72 % 2).
    pub(crate) fn buddy_process_states_fixture(c: &mut Cpu, seed: u32) {
        let _r = buddy_process_machine(c, seed, 0x3f17d9);
        buddy_process_unwind_digit(c, seed);
        let state = [
            BUDDY_STATE_LINK_UP,
            BUDDY_STATE_LINK_DOWN,
            BUDDY_STATE_LOGIN_TIMEOUT,
            0,
        ][(seed % 4) as usize];
        c.bus.write32(BP_SELF + BUDDY_STATE, state as u32);
        let due = [0, BP_NOW, BP_NOW + 1][(seed / 4 % 3) as usize];
        c.bus.write32(BP_SELF + BUDDY_PRESENCE_DUE, due);
        let pattern = BUDDY_PROCESS_OP_PATTERNS[(seed / 12 % 6) as usize];
        for (i, &p) in pattern.iter().enumerate() {
            let i = i as u32;
            let op = BP_SELF + BUDDY_OPS + i * BUDDY_OP_SIZE;
            let f = BP_FN_OP + 0x10 * i;
            let (f, deadline) = match p {
                b'N' => (0, BP_NOW - 1),
                b'Z' => (f, 0),
                b'F' => (f, BP_NOW),
                _ => (f, BP_NOW - 1),
            };
            c.bus.write32(op + BUDDY_OP_FN, f);
            c.bus.write32(op + BUDDY_OP_DEADLINE, deadline);
        }
        if seed / 72 % 2 == 1 {
            c.bus.write32(BP_SELF + BUDDY_STATE_FN, BP_FN_STATE);
        }
    }

    /// `BuddyApi_Process` against the generated body: the state-5 presence
    /// and op deadlines, the state-3/6 op cancels.
    #[test]
    fn buddy_api_process_states_matches() {
        buddy_process_hook_fakes();
        check(
            ee::s14::f_003f17d8__generated,
            buddy_api_process,
            144,
            &buddy_process_states_fixture,
        );
        let op_fn = |i: u32| BP_SELF + BUDDY_OPS + i * BUDDY_OP_SIZE + BUDDY_OP_FN;
        buddy_process_require_arms(
            144,
            &buddy_process_states_fixture,
            &[
                ("presence sent", &|run: &BuddyProcessRun| {
                    run.called(callee::BUDDY_API_SEND_PRESENCE.addr)
                        && run.word(BP_SELF + BUDDY_PRESENCE_DUE) == 0
                }),
                ("presence not due", &|run: &BuddyProcessRun| {
                    run.seed % 4 == 0
                        && run.seed / 4 % 3 == 2
                        && !run.called(callee::BUDDY_API_SEND_PRESENCE.addr)
                }),
                ("op timed out: link dropped", &|run: &BuddyProcessRun| {
                    run.called(callee::PROTO_ARIES_UNCONNECT.addr)
                        && run.called_with(callee::BUDDY_API_SET_STATE.addr, BUDDY_STATE_LINK_DOWN)
                }),
                ("op 0 cancelled", &|run: &BuddyProcessRun| {
                    run.faked(BP_FN_OP) && run.word(op_fn(0)) == 0
                }),
                ("op 1 cancelled", &|run: &BuddyProcessRun| {
                    run.faked(BP_FN_OP + 0x10) && run.word(op_fn(1)) == 0
                }),
                ("op 2 cancelled", &|run: &BuddyProcessRun| {
                    run.faked(BP_FN_OP + 0x20) && run.word(op_fn(2)) == 0
                }),
            ],
        );
    }

    /// The packets of the message fixture: kind `seed % 13`, variant `v`.
    /// Returns the packets and whether the roster exists. ROST's v = 2 is a
    /// new undotted name on list 3 (named and grouped, no list flag); the
    /// dotted names skipped are RGET + ROST's v = 3.
    pub(crate) fn buddy_process_message_packets(
        kind: u32,
        v: usize,
    ) -> (Vec<(u32, u32, &'static [u8])>, bool) {
        let p = buddy_process_packet;
        let even = v % 2 == 0;
        match kind {
            0 => {
                let codes = [u32::MAX, 0xfefe_fefe, 7, u32::MAX, 0xfefe_fefe, 0];
                (vec![p(u32::MAX, codes[v], b"")], true)
            }
            1 => (
                vec![p(
                    BUDDY_KIND_PING,
                    v as u32,
                    if even { &b""[..] } else { &b"T=1"[..] },
                )],
                true,
            ),
            2 => {
                let code = [0, 5, 0, u32::MAX, 0, 1][v];
                (vec![p(BUDDY_KIND_AUTH, code, b"")], code == 0)
            }
            3 => (
                vec![p(BUDDY_KIND_RECV, 0, b"USER=bob@ea.com/ps2\nBODY=hi")],
                true,
            ),
            4 => (vec![p(BUDDY_KIND_SEND, v as u32, b"")], true),
            5 => (vec![p(BUDDY_KIND_ADMN, 0, b"MSG=down")], true),
            6 => {
                let texts: [&'static [u8]; 6] = [
                    b"ID=2\nSIZE=0",
                    b"ID=2\nSIZE=3",
                    b"ID=1\nSIZE=0",
                    b"",
                    b"ID=2",
                    b"SIZE=0",
                ];
                (vec![p(BUDDY_KIND_RGET, 0, texts[v])], true)
            }
            7 => {
                let texts: [&'static [u8]; 6] = [
                    b"ID=1\nUSER=bob@x.y/z\nGROUP=g1",
                    b"ID=2\nUSER=dan\nGROUP=g2",
                    b"ID=3\nUSER=clean\nGROUP=g3",
                    b"ID=2\nUSER=g.h\nGROUP=g4",
                    b"ID=1\nUSER=bob",
                    b"ID=1\nUSER=nul\nGROUP=g5",
                ];
                (vec![p(BUDDY_KIND_ROST, 0, texts[v])], v != 4)
            }
            8 => {
                let packets = match v {
                    0 => vec![p(
                        BUDDY_KIND_PGET,
                        0,
                        b"USER=bob\nSHOW=CHAT\nPROD=PS2\nSTAT=hi",
                    )],
                    1 => vec![p(BUDDY_KIND_PGET, 0, b"USER=amy\nSHOW=AWAY")],
                    2 => vec![p(BUDDY_KIND_PGET, 0, b"USER=zed")],
                    3 => vec![p(BUDDY_KIND_PGET, 0, b"USER=bob")],
                    4 => vec![
                        p(BUDDY_KIND_PGET, 0, b"USER=cat"),
                        p(BUDDY_KIND_PGET, 0, b"USER=cat"),
                    ],
                    _ => vec![p(BUDDY_KIND_PGET, 0, b"")],
                };
                (packets, v != 3)
            }
            9 => {
                let texts: [&'static [u8]; 6] = [
                    b"ID=7\nFUSR=cathy",
                    b"ID=7",
                    b"ID=7",
                    b"ID=9",
                    b"ID=4",
                    b"ID=7",
                ];
                let code = [0, 0, 3, 3, 0, 0][v];
                (vec![p(BUDDY_KIND_RADD, code, texts[v])], v != 5)
            }
            10 => {
                let texts: [&'static [u8]; 6] = [b"ID=5", b"ID=6", b"ID=5", b"", b"ID=5", b"ID=5"];
                (vec![p(BUDDY_KIND_RDEL, 0, texts[v])], true)
            }
            11 => (vec![p(BP_KIND_UNKNOWN, v as u32, b"X=1")], true),
            _ => {
                let texts: [&'static [u8]; 6] = [
                    b"ID=2\nUSER=bob",
                    b"ID=2\nUSER=dan",
                    b"ID=2\nUSER=amy",
                    b"ID=2\nUSER=x.y",
                    b"ID=2\nUSER=cat",
                    b"ID=2\nUSER=nul",
                ];
                (
                    vec![
                        p(BUDDY_KIND_RGET, 0, b"ID=2\nSIZE=1"),
                        p(BUDDY_KIND_ROST, 0, texts[v]),
                    ],
                    true,
                )
            }
        }
    }

    /// Received packets. Digits: kind (seed % 13: link status, PING, AUTH,
    /// RECV, SEND, ADMN, RGET, ROST, PGET, RADD, RDEL, unknown, RGET + ROST),
    /// its variant v (seed / 13 % 6; even v also sets the message callback
    /// and the add/send op callbacks, v % 3 != 2 the delete op callback), the
    /// debug callback (seed / 78 % 2), the roster callback (seed / 156 % 2).
    pub(crate) fn buddy_process_messages_fixture(c: &mut Cpu, seed: u32) {
        let mut r = buddy_process_machine(c, seed, 0x3f17da);
        buddy_process_unwind_digit(c, seed);
        let kind = seed % 13;
        let v = (seed / 13 % 6) as usize;
        let s = BP_SELF;
        c.bus.write32(s + BUDDY_STATE_FN, BP_FN_STATE);
        if seed / 78 % 2 == 1 {
            c.bus.write32(s + BUDDY_DEBUG_FN, BP_FN_DEBUG);
        }
        if seed / 156 % 2 == 1 {
            c.bus.write32(s + BUDDY_ROSTER_FN, BP_FN_ROSTER);
        }
        if v % 2 == 0 {
            c.bus.write32(s + BUDDY_MESSAGE_FN, BP_FN_MSG);
            c.bus.write32(s + BUDDY_OP_SLOT_ADD + BUDDY_OP_FN, BP_FN_OP);
            c.bus
                .write32(s + BUDDY_OP_SLOT_SEND + BUDDY_OP_FN, BP_FN_OP + 0x20);
        }
        if v % 3 != 2 {
            c.bus
                .write32(s + BUDDY_OP_SLOT_DEL + BUDDY_OP_FN, BP_FN_OP + 0x10);
        }
        if kind == 7 {
            c.bus.write32(s + BUDDY_ROSTER_LEFT, [7, 1, 7, 5, 7, 7][v]);
            c.bus.write32(s + BUDDY_ACCEPT_DOTTED, (v == 3) as u32);
        }
        let (packets, roster) = buddy_process_message_packets(kind, v);
        if roster {
            buddy_process_roster(c, &mut r, callee::STRCMP.addr);
        }
        buddy_process_packets(c, &packets);
    }

    /// `BuddyApi_Process` against the generated body: every message kind and
    /// its arms (real tag-field, lobby list/hash, presence and send callees;
    /// fake callbacks and allocator).
    #[test]
    fn buddy_api_process_messages_matches() {
        buddy_process_hook_fakes();
        check(
            ee::s14::f_003f17d8__generated,
            buddy_api_process,
            312,
            &buddy_process_messages_fixture,
        );
        let tag = |run: &BuddyProcessRun| run.word(BP_FRAME + BUDDY_PROCESS_MSG);
        let parse = callee::BUDDY_API_PARSE_PRESENCE.addr;
        let printf = callee::BUDDY_API_DEBUG_PRINTF.addr;
        buddy_process_require_arms(
            312,
            &buddy_process_messages_fixture,
            &[
                ("packet debug print", &|run: &BuddyProcessRun| {
                    run.called_with(callee::BUDDY_API_DEBUG_PACKET.addr, sx(DEBUG_RECV_PREFIX))
                }),
                ("link up: state 5", &|run: &BuddyProcessRun| {
                    run.seed % 13 == 0
                        && run.called_with(callee::BUDDY_API_SET_STATE.addr, BUDDY_STATE_LINK_UP)
                }),
                ("link down: state 6", &|run: &BuddyProcessRun| {
                    run.seed % 13 == 0
                        && run.called_with(callee::BUDDY_API_SET_STATE.addr, BUDDY_STATE_LINK_DOWN)
                }),
                ("PING answered", &|run: &BuddyProcessRun| {
                    run.called_with(callee::PROTO_ARIES_SEND.addr, sx(BUDDY_KIND_PING))
                }),
                ("AUTH accepted", &|run: &BuddyProcessRun| {
                    run.seed % 13 == 2
                        && run.faked(BP_FN_STATE)
                        && !run.called(callee::BUDDY_API_DISCONNECT.addr)
                        && run.word(BP_SELF + BUDDY_LOGIN_DEADLINE) == 0
                }),
                ("AUTH refused: disconnect", &|run: &BuddyProcessRun| {
                    run.called(callee::BUDDY_API_DISCONNECT.addr)
                }),
                ("RECV dispatched", &|run: &BuddyProcessRun| {
                    run.called(callee::BUDDY_API_DISPATCH_MESSAGE.addr)
                }),
                ("SEND op called back", &|run: &BuddyProcessRun| {
                    run.faked(BP_FN_OP + 0x20)
                        && run.word(BP_SELF + BUDDY_OP_SLOT_SEND + BUDDY_OP_FN) == 0
                }),
                ("ADMN called back", &|run: &BuddyProcessRun| {
                    run.seed % 13 == 5 && run.faked(BP_FN_MSG)
                }),
                ("RGET size kept", &|run: &BuddyProcessRun| {
                    run.word(BP_SELF + BUDDY_ROSTER_LEFT) == 3
                }),
                ("ROST new entry", &|run: &BuddyProcessRun| {
                    run.called(callee::LOBBY_MEM_ALLOC.addr)
                        && run.called(callee::LOBBY_HASH_ADD_STR.addr)
                }),
                ("ROST dot check", &|run: &BuddyProcessRun| {
                    run.called_with(callee::STRCHR.addr, b'.' as u64)
                }),
                ("ROST group copied", &|run: &BuddyProcessRun| {
                    run.str_at(BP_BOB + ROSTER_ENTRY_GROUP, b"g1")
                }),
                ("ROST list-2 flag", &|run: &BuddyProcessRun| {
                    run.seed % 13 == 7
                        && run.word(BP_ALLOC + ROSTER_ENTRY_FLAGS) & ROSTER_FLAG_LIST_I as u32 != 0
                }),
                ("ROST list 3: group, no flag", &|run: &BuddyProcessRun| {
                    run.seed % 13 == 7
                        && run.str_at(BP_ALLOC + ROSTER_ENTRY_GROUP, b"g3")
                        && run.word(BP_ALLOC + ROSTER_ENTRY_FLAGS) == 0
                }),
                ("PGET parsed", &|run: &BuddyProcessRun| run.called(parse)),
                ("PGET changed", &|run: &BuddyProcessRun| {
                    run.called_with(printf, sx(FMT_PRESENCE))
                }),
                ("PGET unchanged", &|run: &BuddyProcessRun| {
                    run.count(parse) == 2 && run.count_with(printf, sx(FMT_PRESENCE)) < 2
                }),
                ("RADD refused: entry removed", &|run: &BuddyProcessRun| {
                    run.called(callee::LOBBY_LIST_REMOVE.addr)
                        && run.called(callee::LOBBY_MEM_FREE.addr)
                }),
                ("RADD renamed", &|run: &BuddyProcessRun| {
                    run.called_with(callee::TAG_FIELD_STRNZCPY.addr, ROSTER_NAME_SIZE)
                }),
                ("RADD add op called back", &|run: &BuddyProcessRun| {
                    run.seed % 13 == 9 && run.faked(BP_FN_OP)
                }),
                ("RDEL op called back", &|run: &BuddyProcessRun| {
                    run.faked(BP_FN_OP + 0x10)
                        && run.word(BP_SELF + BUDDY_OP_SLOT_DEL + BUDDY_OP_FN) == 0
                }),
                ("roster complete: endr", &|run: &BuddyProcessRun| {
                    tag(run) == BUDDY_TYPE_ENDR
                }),
                ("roster re-sorted: rost", &|run: &BuddyProcessRun| {
                    tag(run) == BUDDY_TYPE_ROST
                }),
            ],
        );
    }

    /// One fixture per loop head, so the first due event lands on it:
    /// classes 0-3 take `schedule`'s event (due within 48 cycles of entry);
    /// classes 4-6 make it due on the return of the packet's last debug
    /// callback (`buddy_process_arming_call`), past the 0x3f1a10 back edge.
    /// Every callee between that point and the head runs a path without
    /// back edges (empty packet data: the tag finds return at once, `strchr`
    /// finds the terminator in its first quadword, the hash compares with a
    /// leaf returning 0). Class `seed / 3 % 7`:
    /// 0: 0x3f1958 (state 5, every op idle);
    /// 1: 0x3f195c (state 5, every op pending);
    /// 2: 0x3f19d0 (state 6, three ops cancelled);
    /// 3: 0x3f1a10 (two unknown packets);
    /// 4: 0x3f1e24 (PGET of "", the unaligned entry amy);
    /// 5: 0x3f1e7c (PGET of "", the aligned entry bob);
    /// 6: 0x3f1f90 (RADD of id 0: amy not adding, bob adding with id 0).
    pub(crate) fn buddy_process_events_fixture(c: &mut Cpu, seed: u32) -> Events {
        let mut r = buddy_process_machine(c, seed, 0x3f17db);
        let s = BP_SELF;
        let p = buddy_process_packet;
        match seed / 3 % 7 {
            0 => c.bus.write32(s + BUDDY_STATE, BUDDY_STATE_LINK_UP as u32),
            1 | 2 => {
                let state = if seed / 3 % 7 == 1 {
                    BUDDY_STATE_LINK_UP
                } else {
                    BUDDY_STATE_LINK_DOWN
                };
                c.bus.write32(s + BUDDY_STATE, state as u32);
                for i in 0..3 {
                    let op = s + BUDDY_OPS + i * BUDDY_OP_SIZE;
                    c.bus.write32(op + BUDDY_OP_FN, BP_FN_OP + 0x10 * i);
                    c.bus.write32(op + BUDDY_OP_DEADLINE, BP_NOW);
                }
            }
            3 => {
                buddy_process_packets(c, &[p(BP_KIND_UNKNOWN, 0, b""), p(BP_KIND_UNKNOWN, 1, b"")])
            }
            class @ (4 | 5) => {
                buddy_process_roster(c, &mut r, BP_RETURN_ZERO);
                let first = if class == 4 { 0 } else { 0x10 };
                c.bus.write32(BP_BUCKETS, BP_HASH_ENTRIES + first);
                buddy_process_packets(c, &[p(BUDDY_KIND_PGET, 0, b"")]);
            }
            _ => {
                buddy_process_roster(c, &mut r, callee::STRCMP.addr);
                c.bus.write32(BP_BOB + ROSTER_ENTRY_OP_ID, 0);
                c.bus.write32(s + BUDDY_OP_SLOT_ADD + BUDDY_OP_FN, BP_FN_OP);
                buddy_process_packets(c, &[p(BUDDY_KIND_RADD, 0, b"")]);
            }
        }
        let mode = schedule(c, &mut r, seed, BUDDY_PROCESS_ENTRY);
        if seed / 3 % 7 >= 4 {
            // The 0x3f1a10 back edge comes before the packet's handlers, so
            // these classes arm the event themselves: no event until the
            // packet's second debug callback (its last call before the
            // handlers), due on its return.
            c.bus.write32(s + BUDDY_DEBUG_FN, BP_FN_DEBUG);
            c.hooks.miss = buddy_process_arming_call;
            c.bus.write32(BP_CTRL + 8, 1);
            c.bus.write32(BP_CTRL + 0xc, (mode != Events::Off) as u32);
            if mode != Events::Off {
                c.next_event = crate::difftest::CYCLE_BUDGET;
            }
        }
        mode
    }

    /// The miss hook of events classes 4-6: the fakes' callback, and while
    /// armed (`BP_CTRL + 0xc`), the debug callback whose skip count
    /// (`BP_CTRL + 8`) has run out makes an event due as it returns.
    pub(crate) fn buddy_process_arming_call(c: &mut Cpu, target: u32) {
        crate::difftest::fakes::fake_call(c, target);
        if target != BP_FN_DEBUG || c.bus.read32(BP_CTRL + 0xc) == 0 {
            return;
        }
        let skip = c.bus.read32(BP_CTRL + 8);
        if skip == 0 {
            c.bus.write32(BP_CTRL + 0xc, 0);
            c.next_event = c.cycles;
        } else {
            c.bus.write32(BP_CTRL + 8, skip - 1);
        }
    }

    /// The eight back-edge checks (seven heads) with real events, both
    /// sides driven as the trampoline drives them (re-entry at each head).
    #[test]
    fn buddy_api_process_events() {
        buddy_process_hook_fakes();
        driven!(buddy_process_gen, ee::s14::f_003f17d8__generated);
        driven!(buddy_process_hand, buddy_api_process);
        check_events(
            buddy_process_gen,
            buddy_process_hand,
            42,
            &buddy_process_events_fixture,
        );
    }

    // ------------------------------------------------ syscalls of the fixtures

    /// Control block of [`dirty_rpc_syscall`] in the test machine's RAM: the
    /// rule count, the cycles its clock rules added, and up to 8 rules of
    /// 0x20 bytes (syscall code, first and end call index of that code,
    /// kind, value, address, calls seen).
    pub(crate) const DIRTY_RPC_SYS: u32 = DATA + 0xf_0000;

    pub(crate) const DIRTY_RPC_SYS_RULES: u32 = DIRTY_RPC_SYS;

    pub(crate) const DIRTY_RPC_SYS_CLOCK: u32 = DIRTY_RPC_SYS + 4;

    pub(crate) const DIRTY_RPC_SYS_RULE0: u32 = DIRTY_RPC_SYS + 0x20;

    /// Rule kinds: force `v0` (sign-extended, so 0 and negative values too),
    /// write a word (an IOP reply arriving), move the clock on, or make an
    /// event due at the next check after the syscall wrapper's own.
    pub(crate) const DIRTY_RPC_SYS_RESULT: u32 = 1;

    pub(crate) const DIRTY_RPC_SYS_POKE: u32 = 2;

    pub(crate) const DIRTY_RPC_SYS_CLOCK_STEP: u32 = 3;

    pub(crate) const DIRTY_RPC_SYS_ARM: u32 = 4;

    /// Syscall codes (`v1`) of the kernel wrappers the transport calls.
    pub(crate) const DIRTY_RPC_SYS_FLUSH_CACHE: u32 = 0x64;

    pub(crate) const DIRTY_RPC_SYS_SET_DMA: u32 = 0x77;

    pub(crate) const DIRTY_RPC_SYS_DMA_STAT: u32 = 0x76;

    pub(crate) const DIRTY_RPC_SYS_CREATE_THREAD: u32 = 0x20;

    pub(crate) const DIRTY_RPC_SYS_WAIT_SEMA: u32 = 0x44;

    pub(crate) const DIRTY_RPC_SYS_SIGNAL_SEMA: u32 = 0x42;

    /// The syscall hook of the fixtures' machines: the logging fake
    /// (`fakes::fake_syscall`: log, pointer result, unwind at the chosen call),
    /// then the fixture's rules for this syscall code.
    pub(crate) fn dirty_rpc_syscall(c: &mut Cpu, at: u32) {
        let code = c.g[V1][0] as u32;
        fake_syscall(c, at);
        let rules = c.bus.read32(DIRTY_RPC_SYS_RULES).min(8);
        for i in 0..rules {
            let rule = DIRTY_RPC_SYS_RULE0 + 0x20 * i;
            if c.bus.read32(rule) != code {
                continue;
            }
            let n = c.bus.read32(rule + 0x18);
            c.bus.write32(rule + 0x18, n + 1);
            if n < c.bus.read32(rule + 0x4) || n >= c.bus.read32(rule + 0x8) {
                continue;
            }
            let value = c.bus.read32(rule + 0x10);
            match c.bus.read32(rule + 0xc) {
                DIRTY_RPC_SYS_RESULT => c.g[V0][0] = sx(value),
                DIRTY_RPC_SYS_POKE => {
                    let addr = c.bus.read32(rule + 0x14);
                    c.bus.write32(addr, value);
                }
                DIRTY_RPC_SYS_CLOCK_STEP => {
                    c.cycles += value as u64;
                    let total = c.bus.read32(DIRTY_RPC_SYS_CLOCK);
                    c.bus
                        .write32(DIRTY_RPC_SYS_CLOCK, total.wrapping_add(value));
                }
                DIRTY_RPC_SYS_ARM => c.next_event = c.cycles + 1,
                kind => panic!("syscall rule {i}: unknown kind {kind}"),
            }
        }
    }

    /// Add a rule: for the calls `calls` (counted per rule) of syscall `code`.
    pub(crate) fn dirty_rpc_rule(
        c: &mut Cpu,
        code: u32,
        calls: std::ops::Range<u32>,
        kind: u32,
        value: u32,
        addr: u32,
    ) {
        let n = c.bus.read32(DIRTY_RPC_SYS_RULES);
        assert!(n < 8, "too many syscall rules");
        let rule = DIRTY_RPC_SYS_RULE0 + 0x20 * n;
        put_words(
            c,
            rule,
            &[code, calls.start, calls.end, kind, value, addr, 0, 0],
        );
        c.bus.write32(DIRTY_RPC_SYS_RULES, n + 1);
    }

    /// `fakes::machine` with [`dirty_rpc_syscall`] and the unwinding fake
    /// call chosen by the test (`u32::MAX`: none).
    pub(crate) fn dirty_rpc_machine(c: &mut Cpu, seed: u32, salt: u64, unwind_at: u32) -> Rng {
        let r = machine(c, seed, salt, 0);
        c.hooks.syscall = dirty_rpc_syscall;
        c.bus.write32(LOG_UNWIND_AT, unwind_at);
        r
    }

    /// The cycles a run charged outside the fakes: the total since `start`
    /// minus what each logged fake call added (`3 + hash & 7`, the hash
    /// recomputed from its log entry) and the clock rules' steps. With the
    /// fakes left out an arm's cost is a constant.
    pub(crate) fn dirty_rpc_own_cycles(c: &mut Cpu, start: u64) -> u64 {
        let calls = c.bus.read32(LOG_COUNT);
        let mut faked = c.bus.read32(DIRTY_RPC_SYS_CLOCK) as u64;
        for n in 0..calls {
            let entry = LOG_START + 0x220 * n;
            let at = c.bus.read64(entry + 8);
            let a0 = c.bus.read64(entry + 0x10 + 0x10 * A0 as u32);
            let a1 = c.bus.read64(entry + 0x10 + 0x10 * A1 as u32);
            let h = mix(a0 ^ (n as u64) << 40 ^ a1.rotate_left(17) ^ at);
            faked += 3 + (h & 7);
        }
        c.cycles - start - faked
    }

    /// `require_paths` on [`dirty_rpc_own_cycles`].
    pub(crate) fn dirty_rpc_require_paths(
        generated: GuestFn,
        seeds: u32,
        paths: &[u64],
        setup: &dyn Fn(&mut Cpu, u32),
    ) {
        let mut seen = BTreeSet::new();
        for seed in 0..seeds {
            let mut c = fresh();
            setup(&mut c, seed);
            let start = c.cycles;
            run_generated(&mut c, generated);
            seen.insert(dirty_rpc_own_cycles(&mut c, start));
        }
        for p in paths {
            assert!(
                seen.contains(p),
                "path costing {p} cycles (fakes left out) never taken (seen {seen:?})"
            );
        }
    }

    /// `check` plus [`dirty_rpc_require_paths`].
    pub(crate) fn dirty_rpc_check_paths(
        generated: GuestFn,
        hand: GuestFn,
        seeds: u32,
        paths: &[u64],
        setup: &dyn Fn(&mut Cpu, u32),
    ) {
        check(generated, hand, seeds, setup);
        dirty_rpc_require_paths(generated, seeds, paths, setup);
    }

    /// Require that the generated function returns every value in `results`
    /// for some seed (arms told apart by their result).
    pub(crate) fn dirty_rpc_require_results(
        generated: GuestFn,
        seeds: u32,
        results: &[u64],
        setup: &dyn Fn(&mut Cpu, u32),
    ) {
        let mut seen = BTreeSet::new();
        for seed in 0..seeds {
            let mut c = fresh();
            setup(&mut c, seed);
            run_generated(&mut c, generated);
            seen.insert(c.g[V0][0]);
        }
        for v in results {
            assert!(
                seen.contains(v),
                "result {v:#x} never returned (seen {seen:x?})"
            );
        }
    }

    /// Require every `(cost, effect)` in `want` for some seed: the cost (fakes
    /// left out, [`dirty_rpc_own_cycles`]) together with what `effect` reads
    /// from the final state, so arms whose costs collide are each pinned by
    /// an effect only they leave.
    pub(crate) fn dirty_rpc_require_arms<T: Ord + std::fmt::Debug>(
        generated: GuestFn,
        seeds: u32,
        setup: &dyn Fn(&mut Cpu, u32),
        effect: &dyn Fn(&mut Cpu) -> T,
        want: &[(u64, T)],
    ) {
        let mut seen = BTreeSet::new();
        for seed in 0..seeds {
            let mut c = fresh();
            setup(&mut c, seed);
            let start = c.cycles;
            run_generated(&mut c, generated);
            let cost = dirty_rpc_own_cycles(&mut c, start);
            seen.insert((cost, effect(&mut c)));
        }
        for w in want {
            assert!(seen.contains(w), "arm {w:?} never taken (seen {seen:?})");
        }
    }

    /// Require that, for every seed `unwinds_at` maps to `Some(n)` (some seed
    /// must), the generated function ends unwound at fake call `n` with no
    /// fake call after it: the unwind came straight back out, so the `?`
    /// after each call on the way is the exit taken.
    pub(crate) fn dirty_rpc_require_unwound(
        generated: GuestFn,
        seeds: u32,
        setup: &dyn Fn(&mut Cpu, u32),
        unwinds_at: &dyn Fn(u32) -> Option<u32>,
    ) {
        let mut picked = 0;
        for seed in 0..seeds {
            let Some(n) = unwinds_at(seed) else {
                continue;
            };
            picked += 1;
            let mut c = fresh();
            setup(&mut c, seed);
            run_generated(&mut c, generated);
            assert_eq!(
                c.pc,
                0xdead_0000 | n << 4,
                "seed {seed}: no unwind at call {n}"
            );
            assert_eq!(
                c.bus.read32(LOG_COUNT),
                n + 1,
                "seed {seed}: calls after the unwind"
            );
        }
        assert!(picked > 0, "no seed unwinds");
    }

    /// The `a0` of every logged fake call, in call order.
    pub(crate) fn dirty_rpc_logged_a0(c: &mut Cpu) -> Vec<u32> {
        let calls = c.bus.read32(LOG_COUNT);
        (0..calls)
            .map(|n| {
                let a0 = LOG_START + 0x220 * n + 0x10 + 0x10 * A0 as u32;
                c.bus.read32(a0)
            })
            .collect()
    }

    // ------------------------------------------------ the rings

    /// Test layout of the rings: request slots, reply slots, waiter records,
    /// the data area (and its IOP address), result words.
    pub(crate) const DIRTY_RPC_T_REQUESTS: u32 = DATA + 0x1000;

    pub(crate) const DIRTY_RPC_T_REPLIES: u32 = DATA + 0x2000;

    pub(crate) const DIRTY_RPC_T_WAITERS: u32 = DATA + 0x3000;

    pub(crate) const DIRTY_RPC_T_DATA: u32 = DATA + 0x4000;

    pub(crate) const DIRTY_RPC_T_RESULTS: u32 = DATA + 0x5000;

    pub(crate) const DIRTY_RPC_T_DATA_IOP: u32 = 0x000c_0000;

    pub(crate) const DIRTY_RPC_T_SLOTS: u32 = 4;

    pub(crate) fn dirty_rpc_t_request(i: u32) -> u32 {
        DIRTY_RPC_T_REQUESTS + (i << DIRTY_RPC_SLOT_SHIFT)
    }

    pub(crate) fn dirty_rpc_t_reply(i: u32) -> u32 {
        DIRTY_RPC_T_REPLIES + (i << DIRTY_RPC_SLOT_SHIFT)
    }

    pub(crate) fn dirty_rpc_t_waiter(i: u32) -> u32 {
        DIRTY_RPC_T_WAITERS + (i << DIRTY_RPC_WAITER_SHIFT)
    }

    /// Random state words, then the ring words and four slots: `pending`
    /// replies from `read` on, each answering its request (equal seq
    /// words), nobody waiting, no data; everything else in the slots random.
    pub(crate) fn dirty_rpc_ring_fixture(c: &mut Cpu, r: &mut Rng, read: u32, pending: u32) {
        let slots = DIRTY_RPC_T_SLOTS;
        fill(c, r, DIRTY_RPC_STATE + DIRTY_RPC_TABLE_HEAD, 0x24);
        for (off, v) in [
            (DIRTY_RPC_REQUESTS, DIRTY_RPC_T_REQUESTS),
            (DIRTY_RPC_REPLIES, DIRTY_RPC_T_REPLIES),
            (DIRTY_RPC_WAITERS, DIRTY_RPC_T_WAITERS),
            (DIRTY_RPC_EE_BUFFER, DIRTY_RPC_T_DATA),
            (DIRTY_RPC_IOP_BUFFER, DIRTY_RPC_T_DATA_IOP),
            (DIRTY_RPC_TAIL, read),
            (DIRTY_RPC_HEAD, (read + pending) % slots),
            (DIRTY_RPC_SLOTS, slots),
        ] {
            c.bus.write32(DIRTY_RPC_STATE + off, v);
        }
        fill(c, r, DIRTY_RPC_T_REQUESTS, slots * 8);
        fill(c, r, DIRTY_RPC_T_REPLIES, slots * 8);
        fill(c, r, DIRTY_RPC_T_WAITERS, slots * 4);
        fill(c, r, DIRTY_RPC_T_RESULTS, slots);
        for i in 0..slots {
            let seq = 0x5e00 + 0x11 * i;
            c.bus
                .write32(dirty_rpc_t_request(i) + DIRTY_RPC_SLOT_SEQ, seq);
            c.bus
                .write32(dirty_rpc_t_reply(i) + DIRTY_RPC_SLOT_SEQ, seq);
            c.bus
                .write32(dirty_rpc_t_request(i) + DIRTY_RPC_REQUEST_SIZE, 0);
            c.bus
                .write32(dirty_rpc_t_waiter(i) + DIRTY_RPC_WAITER_KIND, 0);
        }
    }

    /// Make the reply in slot `i` not answer its request.
    pub(crate) fn dirty_rpc_t_unanswer(c: &mut Cpu, i: u32) {
        let seq = dirty_rpc_t_reply(i) + DIRTY_RPC_SLOT_SEQ;
        let v = c.bus.read32(seq);
        c.bus.write32(seq, v ^ 0x100);
    }

    // ------------------------------------------------ DirtyRpc_AlarmHandler

    /// `seed % 3`: the alarm that fired is the first (Send's), the handler alarm or
    /// neither; `/3 % 2`: a reply pending; `/6 % 2`: it answers its request
    /// (wake the thread); `/12 % 4`: the alarm ids' signs (bit 0 first, bit 1
    /// poll; both negative re-arms); `/48 % 2`: a fake call unwinds.
    /// Arms (cost with the fakes left out, first id forgotten, handler id
    /// forgotten): 16 nothing pending, +3 for a forgotten id; pending 25, +3
    /// forgotten, +3 first id negative, +11 both negative (re-armed: the
    /// handler id is the new alarm), +22 the wake-up (iWakeupThread's own
    /// 19). The costs 19/28/42/50/64 are each shared by two arms, which the
    /// forgotten ids tell apart.
    #[test]
    fn dirty_rpc_alarm_handler_matches() {
        let (f, t) = (false, true);
        let mut want = vec![(16, (f, f)), (19, (t, f)), (19, (f, t))];
        for wake in [0, 22] {
            for (cost, ids) in [
                (25, (f, f)),
                (28, (f, f)),
                (39, (f, f)),
                (31, (t, f)),
                (42, (t, f)),
                (28, (f, t)),
                (42, (f, f)),
            ] {
                want.push((cost + wake, ids));
            }
        }
        let setup = |c: &mut Cpu, seed: u32| {
            let fired = seed % 3;
            let pending = seed / 3 % 2;
            let answered = seed / 6 % 2 == 1;
            let signs = seed / 12 % 4;
            let unwind = if seed / 48 % 2 == 1 {
                seed % 3
            } else {
                u32::MAX
            };
            let mut r = dirty_rpc_machine(c, seed, 0x3f_4448, unwind);
            c.pc = 0x003f_4448;
            let read = r.next_u32() % DIRTY_RPC_T_SLOTS;
            dirty_rpc_ring_fixture(c, &mut r, read, pending);
            if !answered {
                dirty_rpc_t_unanswer(c, read);
            }
            let alarm = if signs & 1 != 0 {
                0xffff_ff00 + seed
            } else {
                0x100 + seed
            };
            let poll = if signs & 2 != 0 {
                0xffff_fe00 + seed
            } else {
                0x200 + seed
            };
            c.bus.write32(DIRTY_RPC_STATE + DIRTY_RPC_ALARM, alarm);
            c.bus
                .write32(DIRTY_RPC_STATE + DIRTY_RPC_HANDLER_ALARM, poll);
            c.bus
                .write32(DIRTY_RPC_STATE + DIRTY_RPC_THREAD, 0x20 + seed % 8);
            c.g[A0][0] = sx(match fired {
                0 => alarm,
                1 => poll,
                _ => 0x7777_0000 + seed,
            });
            c.g[A2][0] = sx(DIRTY_RPC_STATE);
        };
        let forgotten = |c: &mut Cpu| {
            let gone = |w: u32| w == u32::MAX;
            (
                gone(c.bus.read32(DIRTY_RPC_STATE + DIRTY_RPC_ALARM)),
                gone(c.bus.read32(DIRTY_RPC_STATE + DIRTY_RPC_HANDLER_ALARM)),
            )
        };
        let generated = ee::s14::f_003f4448__generated;
        check(generated, dirty_rpc_alarm_handler, 96, &setup);
        dirty_rpc_require_arms(generated, 96, &setup, &forgotten, &want);
    }

    // ------------------------------------------------ DirtyRpc_ThreadMain

    /// The thread never returns: the fake call `seed % 5` unwinds (a
    /// SleepThread, or a semaphore call inside ProcessReplies); `/5 % 3`
    /// replies are pending for the first round.
    #[test]
    fn dirty_rpc_thread_main_matches() {
        check(
            ee::s14::f_003f44f8__generated,
            dirty_rpc_thread_main,
            45,
            &|c, seed| {
                let mut r = dirty_rpc_machine(c, seed, 0x3f_44f8, seed % 5);
                c.pc = 0x003f_44f8;
                let read = r.next_u32() % DIRTY_RPC_T_SLOTS;
                dirty_rpc_ring_fixture(c, &mut r, read, seed / 5 % 3);
                fill(c, &mut r, GAME_GP + DIRTY_RPC_SLEEPING_GP, 1);
                fill(c, &mut r, GAME_GP + DIRTY_RPC_SEMA_WAIT_GP, 1);
            },
        );
    }

    // ------------------------------------------------ DirtyRpc_ProcessReplies

    /// `seed % 4` replies pending; `/4 % 4` the first one's waiter (nobody,
    /// result word with no pointer, result word stored, callback; the next
    /// ones rotate); `/16 % 2` the result waiters name a thread to wake;
    /// `/32 % 2` the first request has data; `/64 % 4`: all answered, the
    /// last one not answered, the first one changes while waiting for the
    /// semaphore (a WaitSema rule), or all answered with a fake call
    /// unwinding (`seed % 5`).
    pub(crate) fn dirty_rpc_process_replies_fixture(c: &mut Cpu, seed: u32) {
        let pending = seed % 4;
        let first_waiter = seed / 4 % 4;
        let wake = seed / 16 % 2 == 1;
        let sized = seed / 32 % 2 == 1;
        let ending = seed / 64 % 4;
        let unwind = if ending == 3 { seed % 5 } else { u32::MAX };
        let mut r = dirty_rpc_machine(c, seed, 0x3f_4a78, unwind);
        c.pc = 0x003f_4a78;
        let read = r.next_u32() % DIRTY_RPC_T_SLOTS;
        dirty_rpc_ring_fixture(c, &mut r, read, pending);
        for j in 0..pending {
            let i = (read + j) % DIRTY_RPC_T_SLOTS;
            let waiter = dirty_rpc_t_waiter(i);
            let (kind, result) = match (first_waiter + j) % 4 {
                0 => (0, 0),
                1 => (1, 0),
                2 => (1, DIRTY_RPC_T_RESULTS + 4 * i),
                _ => (FAKE + 0x10 * j, r.next_u32()),
            };
            put_words(c, waiter, &[kind, result]);
            c.bus.write32(
                waiter + DIRTY_RPC_WAITER_THREAD,
                if wake { 0x30 + i } else { 0 },
            );
        }
        if sized && pending > 0 {
            c.bus.write32(
                dirty_rpc_t_request(read) + DIRTY_RPC_REQUEST_SIZE,
                0x41 + seed % 0x40,
            );
        }
        match ending {
            1 if pending > 0 => {
                dirty_rpc_t_unanswer(c, (read + pending - 1) % DIRTY_RPC_T_SLOTS);
            }
            2 => {
                let seq = dirty_rpc_t_reply(read) + DIRTY_RPC_SLOT_SEQ;
                let changed = c.bus.read32(seq) ^ 0x100;
                dirty_rpc_rule(
                    c,
                    DIRTY_RPC_SYS_WAIT_SEMA,
                    0..1,
                    DIRTY_RPC_SYS_POKE,
                    changed,
                    seq,
                );
            }
            _ => {}
        }
        fill(c, &mut r, GAME_GP + DIRTY_RPC_SEMA_WAIT_GP, 1);
    }

    /// Paths (fakes left out): 20 empty, 28 first not answered, 57 changed
    /// while waiting, one reply: 92 nobody, 101 with data, 99 callback, 102
    /// result word, 109 result word and wake-up.
    #[test]
    fn dirty_rpc_process_replies_matches() {
        const PATHS: &[u64] = &[20, 28, 57, 92, 99, 101, 102, 109];
        dirty_rpc_check_paths(
            ee::s14::f_003f4a78__generated,
            dirty_rpc_process_replies,
            256,
            PATHS,
            &dirty_rpc_process_replies_fixture,
        );
    }

    /// Three answered replies; a rule makes the event due right after the
    /// first (`seed / 3 % 2` = 0) or second SignalSema's own check, so it
    /// lands on the back edge 0x3f4ad8 (structured: delivered there, no
    /// resume).
    #[test]
    fn dirty_rpc_process_replies_events() {
        driven!(gen, ee::s14::f_003f4a78__generated);
        driven!(hand, dirty_rpc_process_replies);
        let setup = |c: &mut Cpu, seed: u32| {
            let mut r = dirty_rpc_machine(c, seed, 0x3f_4a79, u32::MAX);
            let read = seed / 6 % DIRTY_RPC_T_SLOTS;
            dirty_rpc_ring_fixture(c, &mut r, read, 3);
            let mode = schedule(c, &mut r, seed, 0x003f_4a78);
            if mode != Events::Off {
                c.next_event = u64::MAX;
                let after = seed / 3 % 2;
                dirty_rpc_rule(
                    c,
                    DIRTY_RPC_SYS_SIGNAL_SEMA,
                    after..after + 1,
                    DIRTY_RPC_SYS_ARM,
                    0,
                    0,
                );
            }
            mode
        };
        check_events(gen, hand, 48, &setup);
        // Both sides must deliver at the same cycles (equal `next_event`):
        // a delivery the port's own loop check misses or adds shows here.
        same_event_cycles(gen, hand, 48, &setup, &mut |_, _, _, _, _| {});
    }

    // ------------------------------------------------ DirtyRpc_Call

    /// DirtyRpc_Send's early -1 returns (`seed % 4`): no request (36 cycles),
    /// a request whose +0xc word is 0 (39), a client table start that is not
    /// 16-byte aligned (43); Call adds 12. 3: a bound request, an aligned
    /// buffer and no gather flag, so Send gets to its WaitSema, whose
    /// syscall (the first fake call) unwinds: Call's `?` after Send is taken.
    /// The caller's stack words are random.
    #[test]
    fn dirty_rpc_call_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            let mut r = Rng(0x3f_4c68 ^ seed as u64);
            scramble(c, &mut r);
            c.g[GP][0] = sx(GAME_GP);
            c.pc = 0x003f_4c68;
            let req = DATA + 0x100;
            fill(c, &mut r, req, 8);
            fill(c, &mut r, crate::difftest::SP, 4);
            if seed % 4 == 3 {
                let mut r = dirty_rpc_machine(c, seed, 0x3f_4c69, 0);
                dirty_rpc_ring_fixture(c, &mut r, seed / 4 % DIRTY_RPC_T_SLOTS, 0);
                fill(c, &mut r, req, 8);
                fill(c, &mut r, crate::difftest::SP, 4);
                c.bus.write32(req + DIRTY_RPC_CLIENT_ID, 1 + seed);
                c.g[A0][0] = sx(req);
                c.g[A1][0] = (seed & 5) as u64;
                c.g[T1][0] = (seed % 0x40) as u64;
                c.g[T2][0] = sx(DATA + 0x800);
                return;
            }
            match seed % 4 {
                0 => c.g[A0][0] = 0,
                1 => {
                    c.g[A0][0] = sx(req);
                    c.bus.write32(req + 0xc, 0);
                }
                _ => {
                    c.g[A0][0] = sx(req);
                    c.bus.write32(req + 0xc, 1 + seed);
                    c.g[T2][0] = sx(0x2051_f0c0 | (1 + seed % 15));
                }
            }
        };
        let generated = ee::s14::f_003f4c68__generated;
        check_paths(generated, dirty_rpc_call, 40, &[48, 51, 55], &setup);
        dirty_rpc_require_unwound(generated, 40, &setup, &|seed| (seed % 4 == 3).then_some(0));
    }

    // ------------------------------------------------ DirtyRpc_InitRequest

    pub(crate) const DIRTY_RPC_T_CLIENTS: u32 = DATA + 0x6000;

    pub(crate) const DIRTY_RPC_T_REQUEST: u32 = DATA + 0x100;

    /// `cached`: how the header's cached binding fails (0 other id, 1 other
    /// key, 2 negative index, 3 index 129 (`seed / 28 % 2` = 0: exactly) or
    /// more, 4 the entry's +0xc word
    /// differs, 5 the entry's id differs) or holds (6). `scan` over `entries`
    /// table entries: 0 empty table, 1 the first entry matches, 2 a key
    /// mismatch then a match at the third, 3 a key mismatch and no match, 4
    /// no entry with the id.
    pub(crate) fn dirty_rpc_init_request_fixture(
        c: &mut Cpu,
        seed: u32,
        cached: u32,
        scan: u32,
        entries: u32,
    ) {
        let mut r = Rng(0x3f_4c98 ^ (seed as u64) << 4);
        scramble(c, &mut r);
        c.g[GP][0] = sx(GAME_GP);
        c.pc = 0x003f_4c98;
        let id = 0x1234_0000 + seed;
        let key = 0x77 + seed;
        let req = DIRTY_RPC_T_REQUEST;
        let table = DIRTY_RPC_T_CLIENTS;
        fill(c, &mut r, table, 4 * (entries + 8));
        for e in 0..entries {
            put_words(c, table + 0x10 * e, &[e + 3, 0x5555_0000 + e]);
        }
        let entry = |e: u32| table + 0x10 * e;
        match scan {
            1 => put_words(c, entry(0) + 4, &[id, key]),
            2 => {
                put_words(c, entry(1) + 4, &[id, key ^ 1]);
                put_words(c, entry(2) + 4, &[id, key]);
            }
            3 => put_words(c, entry(1) + 4, &[id, key ^ 1]),
            _ => {}
        }
        let end = if scan == 0 { table } else { entry(entries) };
        c.bus.write32(DIRTY_RPC_STATE + DIRTY_RPC_CLIENTS, table);
        c.bus.write32(DIRTY_RPC_STATE + DIRTY_RPC_CLIENTS_END, end);
        // The header and the (usable) entry its cached index names, past
        // the scanned ones; for a binding that holds and `seed / 28 % 2`,
        // index 0 (the lowest usable one, over the first scanned entry).
        let index = if cached == 6 && seed / 28 % 2 == 1 {
            0
        } else {
            entries + 4 + seed % 4
        };
        let handle = r.next_u32();
        put_words(c, req, &[index, id, key, handle]);
        let named = entry(index);
        put_words(c, named, &[index, id, r.next_u32(), handle]);
        match cached {
            0 => c.bus.write32(req + DIRTY_RPC_CLIENT_SERVER, id ^ 0x10),
            1 => c.bus.write32(req + DIRTY_RPC_CLIENT_KEY, key ^ 1),
            2 => c
                .bus
                .write32(req + DIRTY_RPC_CLIENT_INDEX, 0x8000_0000 | seed),
            3 => c.bus.write32(
                req + DIRTY_RPC_CLIENT_INDEX,
                129 + seed / 28 % 2 * (seed % 0x100),
            ),
            4 => c.bus.write32(named + DIRTY_RPC_CLIENT_ID, handle ^ 1),
            5 => c.bus.write32(named + DIRTY_RPC_CLIENT_SERVER, id ^ 0x20),
            _ => {}
        }
        c.g[A0][0] = sx(req);
        c.g[A1][0] = sx(id);
        c.g[A2][0] = sx(key);
    }

    /// `seed % 7` the cached binding, `/7 % 4` the scan (4 entries). Arms
    /// (cost, result, the index copied on a match): the checks before the
    /// scan cost 7, 9, 13, 15, 20, 23 (25, result 0, when the binding holds),
    /// the scan 9 (empty, -1), 18 (first entry, index 3), 31 (third, index
    /// 5), 32 (none, -1). The costs 25 (7+18 / holds), 38 (7+31 / 20+18) and
    /// 41 (9+32 / 23+18) are shared; result and index tell them apart.
    #[test]
    fn dirty_rpc_init_request_matches() {
        let mut want = vec![(25, (0, 0))];
        for before in [7, 9, 13, 15, 20, 23] {
            for (scan, found) in [(9, (-1, 0)), (18, (1, 3)), (31, (1, 5)), (32, (-1, 0))] {
                want.push((before + scan, found));
            }
        }
        let setup = |c: &mut Cpu, seed: u32| {
            dirty_rpc_init_request_fixture(c, seed, seed % 7, seed / 7 % 4, 4);
        };
        let effect = |c: &mut Cpu| {
            let result = c.g[V0][0] as i64;
            let index = c.bus.read32(DIRTY_RPC_T_REQUEST + DIRTY_RPC_CLIENT_INDEX);
            (result, if result == 1 { index } else { 0 })
        };
        let generated = ee::s14::f_003f4c98__generated;
        check(generated, dirty_rpc_init_request, 56, &setup);
        dirty_rpc_require_arms(generated, 56, &setup, &effect, &want);
    }

    /// Sixteen entries, none with the id: the event lands on the scan's back
    /// edge 0x3f4d30 (structured; leaf).
    #[test]
    fn dirty_rpc_init_request_events() {
        driven!(gen, ee::s14::f_003f4c98__generated);
        driven!(hand, dirty_rpc_init_request);
        let setup = |c: &mut Cpu, seed: u32| {
            dirty_rpc_init_request_fixture(c, seed, 0, 4, 16);
            let mut r = Rng(0x3f_4d30 ^ seed as u64);
            schedule(c, &mut r, seed, 0x003f_4c98)
        };
        check_events(gen, hand, 60, &setup);
        // Both sides must deliver at the same cycles (equal `next_event`):
        // a delivery the port's own loop check misses or adds shows here.
        same_event_cycles(gen, hand, 60, &setup, &mut |_, _, _, _, _| {});
    }

    // ------------------------------------------------ DirtyRpc_InitIop

    /// `pending` answered replies (nobody waiting, no data); a client table
    /// start that is not 16-byte aligned, so DirtyRpc_Send gives up at once.
    pub(crate) fn dirty_rpc_init_iop_fixture(
        c: &mut Cpu,
        seed: u32,
        pending: u32,
        unwind: u32,
    ) -> Rng {
        let mut r = dirty_rpc_machine(c, seed, 0x3f_4d68, unwind);
        c.pc = 0x003f_4d68;
        let read = r.next_u32() % DIRTY_RPC_T_SLOTS;
        dirty_rpc_ring_fixture(c, &mut r, read, pending);
        c.bus
            .write32(DIRTY_RPC_STATE + DIRTY_RPC_CLIENTS, 0x2051_f0c4);
        fill(c, &mut r, GAME_GP + DIRTY_RPC_SEMA_WAIT_GP, 1);
        r
    }

    /// `seed % 3` replies pending (paths 83 none, 181 one poll taking one,
    /// 250 one poll taking two; fakes left out); `/3 % 2` a fake call
    /// unwinds (`/6 % 4`).
    #[test]
    fn dirty_rpc_init_iop_matches() {
        dirty_rpc_check_paths(
            ee::s14::f_003f4d68__generated,
            dirty_rpc_init_iop,
            48,
            &[83, 181, 250],
            &|c, seed| {
                let unwind = if seed / 3 % 2 == 1 {
                    seed / 6 % 4
                } else {
                    u32::MAX
                };
                dirty_rpc_init_iop_fixture(c, seed, seed % 3, unwind);
            },
        );
    }

    /// One or two replies pending (`seed / 3 % 2`): the first back edge
    /// (0x3f4dc0, a resume entry) comes 79 cycles in, before any call that
    /// checks events, so every event seed lands there.
    #[test]
    fn dirty_rpc_init_iop_events() {
        driven!(gen, ee::s14::f_003f4d68__generated);
        driven!(hand, dirty_rpc_init_iop);
        check_events(gen, hand, 60, &|c, seed| {
            let mut r = dirty_rpc_init_iop_fixture(c, seed, 1 + seed / 3 % 2, u32::MAX);
            schedule(c, &mut r, seed, 0x003f_4d68)
        });
    }

    /// A 16-byte aligned client table start (`seed % 4` lines on), so
    /// DirtyRpc_Send gets to its WaitSema, whose syscall (the first fake
    /// call) unwinds: the unwind comes back through DirtyRpc_Call to InitIop's
    /// `?` after it. `seed % 3` replies pending (never taken).
    #[test]
    fn dirty_rpc_init_iop_send_unwinds() {
        let setup = |c: &mut Cpu, seed: u32| {
            dirty_rpc_init_iop_fixture(c, seed, seed % 3, 0);
            let table = 0x2051_f0c0 + 0x10 * (seed % 4);
            c.bus.write32(DIRTY_RPC_STATE + DIRTY_RPC_CLIENTS, table);
        };
        let generated = ee::s14::f_003f4d68__generated;
        check(generated, dirty_rpc_init_iop, 12, &setup);
        dirty_rpc_require_unwound(generated, 12, &setup, &|_| Some(0));
    }

    // ------------------------------------------------ DirtyRpc_SyncDCache

    /// Gate word of [`dirty_rpc_sync_dcache_t_fake`]: its magic turns it on.
    pub(crate) const DIRTY_RPC_SYNC_DCACHE_T_CTRL: u32 = DIRTY_RPC_SYS + 0x200;

    pub(crate) const DIRTY_RPC_SYNC_DCACHE_T_MAGIC: u32 = 0x5dca_c4e0;

    /// `SyncDCache` (0x424698) as a logged fake call while the gate is on
    /// (`fakes::fake_call`: the range lands in the log, a pointer result,
    /// the temporaries clobbered, an unwind at the chosen call); the real
    /// body otherwise.
    pub(crate) fn dirty_rpc_sync_dcache_t_fake(c: &mut Cpu) {
        let prev = prev_cell(dirty_rpc_sync_dcache_t_fake);
        if c.bus.read32(DIRTY_RPC_SYNC_DCACHE_T_CTRL) != DIRTY_RPC_SYNC_DCACHE_T_MAGIC {
            return fall_back(c, prev, ee::s15::f_00424698__generated);
        }
        fakes::fake_call(c, 0x0042_4698);
    }

    /// `seed % 2`: the real SyncDCache, or the logged fake standing in for
    /// it; with the fake, `/2 % 2`: it unwinds, so the `?` after the call is
    /// taken.
    #[test]
    fn dirty_rpc_sync_dcache_matches() {
        hook(
            addr_of_mut!(ee::s15::OVERRIDE_00424698),
            dirty_rpc_sync_dcache_t_fake,
            prev_cell(dirty_rpc_sync_dcache_t_fake),
        );
        let setup = |c: &mut Cpu, seed: u32| {
            let _r = machine(c, seed, 0x3f_4e80, 0);
            c.pc = 0x003f_4e80;
            c.g[A0][0] = sx(DATA + 0x24 * seed);
            c.g[A1][0] = sx(0x40 * (seed % 9) + seed % 5);
            if seed % 2 == 1 {
                c.bus
                    .write32(DIRTY_RPC_SYNC_DCACHE_T_CTRL, DIRTY_RPC_SYNC_DCACHE_T_MAGIC);
                if seed / 2 % 2 == 1 {
                    c.bus.write32(LOG_UNWIND_AT, 0);
                }
            }
        };
        let generated = ee::s14::f_003f4e80__generated;
        check(generated, dirty_rpc_sync_dcache, 32, &setup);
        dirty_rpc_require_unwound(generated, 32, &setup, &|seed| (seed % 4 == 3).then_some(0));
    }

    // ------------------------------------------------ NetGameLink_CheckRecvBuf

    /// `seed % 8`: no next record (10 cycles), negative length (13), length
    /// over 0x8000 (16), next record misplaced (25), not linked back (30),
    /// sequence words differ (32), valid with no length pointer and with one
    /// (34); `/8 % 4` a valid length (0, 1, 0x7ff1, 0x8000); `/32 % 3` the
    /// segment bits of the pointers.
    #[test]
    fn net_game_link_check_recv_buf_matches() {
        check_paths(
            ee::s14::f_003f4ea0__generated,
            net_game_link_check_recv_buf,
            96,
            &[10, 13, 16, 25, 30, 32, 34],
            &|c, seed| {
                let mut r = Rng(0x3f_4ea0 ^ seed as u64);
                scramble(c, &mut r);
                c.pc = 0x003f_4ea0;
                let arm = seed % 8;
                let digit = (seed / 32 % 3) as usize;
                let segment = [0, 0x2000_0000, 0x3000_0000][digit];
                let link = |p: u32| p | [0x2000_0000, 0, 0x3000_0000][digit];
                let buf = DATA + 0x100;
                let out = DATA + 0xa000;
                fill(c, &mut r, out, 1);
                let len: u32 = match arm {
                    1 => 0x8000_0000 | seed,
                    2 => 0x8001 + seed,
                    _ => [0, 1, 0x7ff1, 0x8000][(seed / 8 % 4) as usize],
                };
                let seq = r.next_u32();
                match arm {
                    0 => {
                        let w = r.next_u32();
                        put_words(c, buf, &[0, w, seq, len]);
                    }
                    1 | 2 => {
                        let w = r.next_u32();
                        put_words(c, buf, &[link(buf + 0x40), w, seq, len]);
                    }
                    _ => {
                        let next = buf + 0x10 + (len.wrapping_add(0xf) & 0x7fff_fff0);
                        let at = if arm == 3 { next + 0x10 } else { next };
                        let w = r.next_u32();
                        put_words(c, buf, &[link(at), w, seq, len]);
                        fill(c, &mut r, next, 4);
                        let back = if arm == 4 { buf + 0x20 } else { buf };
                        let next_seq = if arm == 5 { seq ^ 1 } else { seq };
                        let w = r.next_u32();
                        put_words(c, next, &[link(back), w, next_seq]);
                    }
                }
                c.g[A0][0] = sx(buf | segment);
                c.g[A1][0] = sx(if arm == 6 { 0 } else { out });
            },
        );
    }

    // ------------------------------------------------ DirtyRpc_WaitIop

    /// Random boot reply word; a random boot packet of `size` bytes: 0, so
    /// sceSifSendCmd sends nothing (41 cycles), or 0x10..=0x70, so it DMAs
    /// the packet (`sceSifSetDma`'s syscall is the fake call after the
    /// first FlushCache).
    pub(crate) fn dirty_rpc_wait_iop_fixture(c: &mut Cpu, seed: u32, unwind: u32, size: u8) -> Rng {
        let mut r = dirty_rpc_machine(c, seed, 0x3f_4f40, unwind);
        c.pc = 0x003f_4f40;
        fill(c, &mut r, DIRTY_RPC_STATE, 1);
        fill(c, &mut r, DIRTY_RPC_BOOT_CMD, 4);
        c.bus.write8(DIRTY_RPC_BOOT_CMD, size);
        r
    }

    /// `seed % 4`: the IOP answers (a FlushCache rule sets the reply byte)
    /// at the first flush (82 cycles), the first poll (96), the second
    /// (110), or never: each flush then moves the clock 20M cycles on and
    /// the third poll times out (131; events off, past the cycle budget).
    /// `/4 % 2` a fake call unwinds (`/8 % 4`). `/64 % 2`: the packet has a
    /// size (0x10..=0x6c), so sceSifSendCmd makes its call; with `/8 % 4` =
    /// 1 that is the call that unwinds, and WaitIop's `?` after
    /// sceSifSendCmd is taken.
    #[test]
    fn dirty_rpc_wait_iop_matches() {
        let generated = ee::s14::f_003f4f40__generated;
        let sized = |seed: u32| seed / 64 % 2 == 1;
        let setup = |c: &mut Cpu, seed: u32| {
            let arm = seed % 4;
            let unwind = if seed / 4 % 2 == 1 {
                seed / 8 % 4
            } else {
                u32::MAX
            };
            let size = if sized(seed) {
                0x10 + 4 * (seed % 0x18)
            } else {
                0
            };
            dirty_rpc_wait_iop_fixture(c, seed, unwind, size as u8);
            dirty_rpc_wait_iop_arm(c, arm);
        };
        dirty_rpc_check_paths(
            generated,
            dirty_rpc_wait_iop,
            128,
            &[82, 96, 110, 131],
            &setup,
        );
        dirty_rpc_require_unwound(generated, 128, &setup, &|seed| {
            (sized(seed) && seed / 4 % 2 == 1 && seed / 8 % 4 == 1).then_some(1)
        });
    }

    /// The FlushCache rule of `dirty_rpc_wait_iop_matches`'s `arm`.
    pub(crate) fn dirty_rpc_wait_iop_arm(c: &mut Cpu, arm: u32) {
        if arm < 3 {
            dirty_rpc_rule(
                c,
                DIRTY_RPC_SYS_FLUSH_CACHE,
                arm..arm + 1,
                DIRTY_RPC_SYS_POKE,
                1,
                DIRTY_RPC_STATE + DIRTY_RPC_BOOT_REPLY,
            );
        } else {
            dirty_rpc_rule(
                c,
                DIRTY_RPC_SYS_FLUSH_CACHE,
                0..u32::MAX,
                DIRTY_RPC_SYS_CLOCK_STEP,
                20_000_000,
                0,
            );
            c.next_event = u64::MAX;
        }
    }

    /// The IOP answers at the second poll's flush; the first poll's flush
    /// moves the clock `step` cycles on (events off).
    pub(crate) fn dirty_rpc_wait_iop_edge_fixture(c: &mut Cpu, seed: u32, step: u32) {
        dirty_rpc_wait_iop_fixture(c, seed, u32::MAX, 0);
        let reply = DIRTY_RPC_STATE + DIRTY_RPC_BOOT_REPLY;
        let flush = DIRTY_RPC_SYS_FLUSH_CACHE;
        dirty_rpc_rule(c, flush, 1..2, DIRTY_RPC_SYS_CLOCK_STEP, step, 0);
        dirty_rpc_rule(c, flush, 2..3, DIRTY_RPC_SYS_POKE, 1, reply);
        c.next_event = u64::MAX;
    }

    /// The timeout is strict (`50M < elapsed`). Per seed, bisect (on the
    /// generated body) for the largest clock step the first poll survives:
    /// that poll then sees exactly 50M ticks elapsed and must poll again
    /// and find the answer (1); one more tick times out (0). Both sides
    /// compared at both steps.
    #[test]
    fn dirty_rpc_wait_iop_timeout_edge() {
        let generated = ee::s14::f_003f4f40__generated;
        for seed in 0..4 {
            let result = |step: u32| {
                let mut c = fresh();
                dirty_rpc_wait_iop_edge_fixture(&mut c, seed, step);
                run_generated(&mut c, generated);
                c.g[V0][0]
            };
            let (mut lo, mut hi) = (0, DIRTY_RPC_BOOT_TIMEOUT as u32);
            assert_eq!((result(lo), result(hi)), (1, 0), "seed {seed}");
            while hi - lo > 1 {
                let mid = lo + (hi - lo) / 2;
                if result(mid) == 1 {
                    lo = mid;
                } else {
                    hi = mid;
                }
            }
            for step in [lo, hi] {
                check(generated, dirty_rpc_wait_iop, 1, &|c, _| {
                    dirty_rpc_wait_iop_edge_fixture(c, seed, step)
                });
            }
        }
    }

    /// The IOP answers at flush `seed / 3 % 3 + 1`; a rule makes the event
    /// due right after flush `seed / 3 % 3` (its wrapper's own check sees
    /// none), so the first delivery is on the poll head 0x3f4fb0 (a resume
    /// entry), after 0, 1 or 2 polls.
    #[test]
    fn dirty_rpc_wait_iop_events() {
        driven!(gen, ee::s14::f_003f4f40__generated);
        driven!(hand, dirty_rpc_wait_iop);
        check_events(gen, hand, 54, &|c, seed| {
            let polls = seed / 3 % 3;
            let mut r = dirty_rpc_wait_iop_fixture(c, seed, u32::MAX, 0);
            dirty_rpc_rule(
                c,
                DIRTY_RPC_SYS_FLUSH_CACHE,
                polls + 1..polls + 2,
                DIRTY_RPC_SYS_POKE,
                1,
                DIRTY_RPC_STATE + DIRTY_RPC_BOOT_REPLY,
            );
            let mode = schedule(c, &mut r, seed, 0x003f_4f40);
            if mode != Events::Off {
                c.next_event = u64::MAX;
                dirty_rpc_rule(
                    c,
                    DIRTY_RPC_SYS_FLUSH_CACHE,
                    polls..polls + 1,
                    DIRTY_RPC_SYS_ARM,
                    0,
                    0,
                );
            }
            mode
        });
    }

    // ------------------------------------------------ DirtyRpc_Startup

    /// The word `sceSifAllocIopHeap` checks first (negative: it returns 0 at
    /// once), the receive word the IOP heap reply lands in, and
    /// `sceSifInitCmd`'s done flag (set: it returns at once).
    pub(crate) const DIRTY_RPC_T_IOP_HEAP_SERVER: u32 = 0x0045_5224;

    pub(crate) const DIRTY_RPC_T_IOP_HEAP_REPLY: u32 = 0x0052_ef00;

    pub(crate) const DIRTY_RPC_T_SIF_CMD_DONE: u32 = 0x0045_517c;

    /// `seed % 6` the arm: the IOP never answers (-1), no IOP heap (-2), the
    /// first or second DMA is not queued (-3, -4), CreateThread fails (-5;
    /// 0 or negative by `/6 % 2`), started (0). `/6 % 3` and `/18 % 3`: how
    /// often each DMA reports busy first (the second as status 0 when
    /// `/36 % 2`). `/54 % 2`: fake call `seed % 17`
    /// unwinds. The heap comes through the real sceSifAllocIopHeap /
    /// sceSifCallRpc path (one free packet, fake semaphores and DMA).
    pub(crate) fn dirty_rpc_startup_fixture(c: &mut Cpu, seed: u32) {
        let arm = seed % 6;
        let first_busy = seed / 6 % 3;
        let second_busy = seed / 18 % 3;
        let unwind = if seed / 54 % 2 == 1 {
            seed % 17
        } else {
            u32::MAX
        };
        let mut r = dirty_rpc_machine(c, seed, 0x3f_5000, unwind);
        c.pc = 0x003f_5000;
        fill(c, &mut r, DIRTY_RPC_STATE + DIRTY_RPC_TABLE_HEAD, 0x24);
        fill(c, &mut r, DIRTY_RPC_BOOT_CMD, 4);
        c.bus.write8(DIRTY_RPC_BOOT_CMD, 0);
        if arm == 0 {
            dirty_rpc_rule(
                c,
                DIRTY_RPC_SYS_FLUSH_CACHE,
                0..u32::MAX,
                DIRTY_RPC_SYS_CLOCK_STEP,
                20_000_000,
                0,
            );
            c.next_event = u64::MAX;
        } else {
            dirty_rpc_rule(
                c,
                DIRTY_RPC_SYS_FLUSH_CACHE,
                0..1,
                DIRTY_RPC_SYS_POKE,
                1,
                DIRTY_RPC_STATE + DIRTY_RPC_BOOT_REPLY,
            );
        }
        c.bus.write32(
            DIRTY_RPC_T_IOP_HEAP_SERVER,
            if arm == 1 { 0xffff_ffff } else { 0 },
        );
        let packet = DATA + 0x6000;
        fill(c, &mut r, packet, 0x10);
        c.bus.write32(packet + 0x10, 0);
        let counter = r.next_u32();
        put_words(c, SIF_RPC_POOL, &[counter, packet, 1]);
        c.bus
            .write32(DIRTY_RPC_T_IOP_HEAP_REPLY, 0x000a_0000 + 0x40 * seed);
        c.bus.write32(DIRTY_RPC_T_SIF_CMD_DONE, 1);
        // sceSifSetDma call 0 is sceSifCallRpc's; 1 and 2 are the startup's.
        if arm == 2 || arm == 3 {
            dirty_rpc_rule(
                c,
                DIRTY_RPC_SYS_SET_DMA,
                arm - 1..arm,
                DIRTY_RPC_SYS_RESULT,
                0,
                0,
            );
        }
        // sceSifDmaStat: busy (the fakes' pointer result) until -1.
        dirty_rpc_rule(
            c,
            DIRTY_RPC_SYS_DMA_STAT,
            first_busy..first_busy + 1,
            DIRTY_RPC_SYS_RESULT,
            0xffff_ffff,
            0,
        );
        dirty_rpc_rule(
            c,
            DIRTY_RPC_SYS_DMA_STAT,
            first_busy + 1 + second_busy..u32::MAX,
            DIRTY_RPC_SYS_RESULT,
            0xffff_ffff,
            0,
        );
        // `/36 % 2`: the second DMA reports busy as 0 (still busy: the wait
        // ends only on a negative status).
        if seed / 36 % 2 == 1 {
            dirty_rpc_rule(
                c,
                DIRTY_RPC_SYS_DMA_STAT,
                first_busy + 1..first_busy + 1 + second_busy,
                DIRTY_RPC_SYS_RESULT,
                0,
                0,
            );
        }
        if arm == 4 {
            let failed = if seed / 6 % 2 == 0 { 0 } else { 0xffff_fff0 };
            dirty_rpc_rule(
                c,
                DIRTY_RPC_SYS_CREATE_THREAD,
                0..u32::MAX,
                DIRTY_RPC_SYS_RESULT,
                failed,
                0,
            );
        }
        fill(c, &mut r, GAME_GP + DIRTY_RPC_STARTED_GP, 1);
    }

    #[test]
    fn dirty_rpc_startup_matches() {
        let results: Vec<u64> = [-1i64, -2, -3, -4, -5, 0]
            .iter()
            .map(|&v| v as u64)
            .collect();
        check(
            ee::s14::f_003f5000__generated,
            dirty_rpc_startup,
            108,
            &dirty_rpc_startup_fixture,
        );
        dirty_rpc_require_results(
            ee::s14::f_003f5000__generated,
            108,
            &results,
            &dirty_rpc_startup_fixture,
        );
    }

    // ------------------------------------------------ DirtyRpc_Shutdown

    /// `seed % 2` / `/2 % 2`: the first / handler alarm is set (released) or
    /// negative; `/4 % 2` a fake call unwinds (`/8 % 6`). Arms (cost with
    /// the fakes left out, which ids went to ReleaseAlarm): 40 none, 47 the
    /// first or the handler's (the shared cost, told apart by the id), 54
    /// both.
    #[test]
    fn dirty_rpc_shutdown_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            let unwind = if seed / 4 % 2 == 1 {
                seed / 8 % 6
            } else {
                u32::MAX
            };
            let mut r = dirty_rpc_machine(c, seed, 0x3f_53f8, unwind);
            c.pc = 0x003f_53f8;
            fill(c, &mut r, DIRTY_RPC_STATE + DIRTY_RPC_TABLE_HEAD, 0x24);
            let alarm = if seed % 2 == 0 {
                0x10 + seed
            } else {
                0x8000_0000 | seed
            };
            let poll = if seed / 2 % 2 == 0 {
                0x20 + seed
            } else {
                0xffff_ffff
            };
            c.bus.write32(DIRTY_RPC_STATE + DIRTY_RPC_ALARM, alarm);
            c.bus
                .write32(DIRTY_RPC_STATE + DIRTY_RPC_HANDLER_ALARM, poll);
            fill(c, &mut r, GAME_GP + DIRTY_RPC_STARTED_GP, 1);
        };
        let released = |c: &mut Cpu| {
            let ids = dirty_rpc_logged_a0(c);
            let alarm = c.bus.read32(DIRTY_RPC_STATE + DIRTY_RPC_ALARM);
            let poll = c.bus.read32(DIRTY_RPC_STATE + DIRTY_RPC_HANDLER_ALARM);
            (ids.contains(&alarm), ids.contains(&poll))
        };
        let (f, t) = (false, true);
        let want = [(40, (f, f)), (47, (t, f)), (47, (f, t)), (54, (t, t))];
        let generated = ee::s14::f_003f53f8__generated;
        check(generated, dirty_rpc_shutdown, 48, &setup);
        dirty_rpc_require_arms(generated, 48, &setup, &released, &want);
    }

    // ------------------------------------------------ DirtyRpc_IsStarted

    #[test]
    fn dirty_rpc_is_started_matches() {
        check(
            ee::s14::f_003f5470__generated,
            dirty_rpc_is_started,
            16,
            &|c, seed| {
                let mut r = Rng(0x3f_5470 ^ seed as u64);
                scramble(c, &mut r);
                c.g[GP][0] = sx(GAME_GP);
                c.pc = 0x003f_5470;
                let flag = if seed % 2 == 0 { 0 } else { r.next_u32() };
                c.bus.write32(GAME_GP + DIRTY_RPC_STARTED_GP, flag);
            },
        );
    }

    // ------------------------------------------------------- DirtyRpc_Send

    // The fixture's state tables, buffers and data (all pre-filled with junk).
    pub(crate) const DIRTY_RPC_SEND_T_CLIENT: u32 = DATA + 0x100;

    pub(crate) const DIRTY_RPC_SEND_T_REQUESTS: u32 = DATA + 0x1000;

    pub(crate) const DIRTY_RPC_SEND_T_REPLIES: u32 = DATA + 0x1400;

    pub(crate) const DIRTY_RPC_SEND_T_DMA_IDS: u32 = DATA + 0x1800;

    pub(crate) const DIRTY_RPC_SEND_T_WAITERS: u32 = DATA + 0x1900;

    pub(crate) const DIRTY_RPC_SEND_T_EE_BUFFER: u32 = DATA + 0x4000;

    pub(crate) const DIRTY_RPC_SEND_T_BUFFER_SIZE: u32 = 0x800;

    pub(crate) const DIRTY_RPC_SEND_T_REPLY: u32 = DATA + 0x6000;

    pub(crate) const DIRTY_RPC_SEND_T_DATA: u32 = DATA + 0x8000;

    pub(crate) const DIRTY_RPC_SEND_T_GATHER: u32 = DATA + 0x9000;

    pub(crate) const DIRTY_RPC_SEND_T_PIECES: u32 = DATA + 0xa000;

    /// IOP-side addresses (only stored, never dereferenced on the EE).
    pub(crate) const DIRTY_RPC_SEND_T_IOP_BUFFER: u32 = 0x000a_0000;

    pub(crate) const DIRTY_RPC_SEND_T_IOP_REQUESTS: u32 = 0x000b_8000;

    /// The gated kernel-call fakes' control block (in the test machine's RAM;
    /// the magic word turns them on).
    pub(crate) const DIRTY_RPC_SEND_T_CTRL: u32 = DATA + 0xf_0000;

    pub(crate) const DIRTY_RPC_SEND_T_MAGIC: u32 = 0xd1f7_5e4d;

    /// Control words: WaitSema calls so far; the WaitSema call during which
    /// the ring tail's reply lands, where and what; the WaitSema call after
    /// which an event falls due; sceSifSetDma calls so far, how many of the
    /// first ones return id 0, the call after which an event falls due and
    /// how many cycles later; an event due after GetThreadId; SyncDCache
    /// calls so far.
    pub(crate) const DIRTY_RPC_SEND_T_WAITS: u32 = 0x04;

    pub(crate) const DIRTY_RPC_SEND_T_LAND_AT: u32 = 0x08;

    pub(crate) const DIRTY_RPC_SEND_T_LAND_ADDR: u32 = 0x0c;

    pub(crate) const DIRTY_RPC_SEND_T_LAND_VALUE: u32 = 0x10;

    pub(crate) const DIRTY_RPC_SEND_T_WAIT_ARM_AT: u32 = 0x14;

    pub(crate) const DIRTY_RPC_SEND_T_DMAS: u32 = 0x18;

    pub(crate) const DIRTY_RPC_SEND_T_DMA_ZEROS: u32 = 0x1c;

    pub(crate) const DIRTY_RPC_SEND_T_DMA_ARM_AT: u32 = 0x20;

    pub(crate) const DIRTY_RPC_SEND_T_DMA_ARM_DELAY: u32 = 0x24;

    pub(crate) const DIRTY_RPC_SEND_T_TID_ARM: u32 = 0x28;

    pub(crate) const DIRTY_RPC_SEND_T_SYNCS: u32 = 0x2c;

    /// The `syscall` sites of the kernel wrappers (the fake log's targets).
    pub(crate) const DIRTY_RPC_SEND_T_WAIT_SYSCALL: u32 = 0x0042_3de4;

    pub(crate) const DIRTY_RPC_SEND_T_THREAD_SYSCALL: u32 = 0x0042_3c94;

    pub(crate) const DIRTY_RPC_SEND_T_DMA_SYSCALL: u32 = 0x0042_4164;

    pub(crate) const DIRTY_RPC_SEND_T_ALARM_SYSCALL: u32 = 0x0042_3b24;

    pub(crate) const DIRTY_RPC_SEND_T_SLEEP_SYSCALL: u32 = 0x0042_3cc4;

    pub(crate) fn dirty_rpc_send_t_gated(c: &mut Cpu) -> bool {
        c.bus.read32(DIRTY_RPC_SEND_T_CTRL) == DIRTY_RPC_SEND_T_MAGIC
    }

    pub(crate) fn dirty_rpc_send_t_get(c: &mut Cpu, word: u32) -> u32 {
        c.bus.read32(DIRTY_RPC_SEND_T_CTRL + word)
    }

    pub(crate) fn dirty_rpc_send_t_set(c: &mut Cpu, word: u32, v: u32) {
        c.bus.write32(DIRTY_RPC_SEND_T_CTRL + word, v);
    }

    /// Count a call; returns its index.
    pub(crate) fn dirty_rpc_send_t_bump(c: &mut Cpu, word: u32) -> u32 {
        let n = dirty_rpc_send_t_get(c, word);
        dirty_rpc_send_t_set(c, word, n + 1);
        n
    }

    /// `WaitSema` for the fixture: the real wrapper, then (on the chosen
    /// calls) the IOP's reply for the ring tail lands and an event falls due.
    pub(crate) fn dirty_rpc_send_t_wait_sema(c: &mut Cpu) {
        let prev = prev_cell(dirty_rpc_send_t_wait_sema);
        if !dirty_rpc_send_t_gated(c) {
            return fall_back(c, prev, ee::s15::f_00423de0__generated);
        }
        let ra = r32(c, RA);
        let n = dirty_rpc_send_t_bump(c, DIRTY_RPC_SEND_T_WAITS);
        fall_back(c, prev, ee::s15::f_00423de0__generated);
        if c.pc != ra {
            return;
        }
        if n == dirty_rpc_send_t_get(c, DIRTY_RPC_SEND_T_LAND_AT) {
            let at = dirty_rpc_send_t_get(c, DIRTY_RPC_SEND_T_LAND_ADDR);
            let v = dirty_rpc_send_t_get(c, DIRTY_RPC_SEND_T_LAND_VALUE);
            c.bus.write32(at, v);
        }
        if n == dirty_rpc_send_t_get(c, DIRTY_RPC_SEND_T_WAIT_ARM_AT) {
            c.next_event = c.cycles;
        }
    }

    /// `sceSifSetDma` for the fixture: the real wrapper, id 0 for the first
    /// calls, and an event due after the chosen call.
    pub(crate) fn dirty_rpc_send_t_set_dma(c: &mut Cpu) {
        let prev = prev_cell(dirty_rpc_send_t_set_dma);
        if !dirty_rpc_send_t_gated(c) {
            return fall_back(c, prev, ee::s15::f_00424160__generated);
        }
        let ra = r32(c, RA);
        let n = dirty_rpc_send_t_bump(c, DIRTY_RPC_SEND_T_DMAS);
        fall_back(c, prev, ee::s15::f_00424160__generated);
        if c.pc != ra {
            return;
        }
        if n < dirty_rpc_send_t_get(c, DIRTY_RPC_SEND_T_DMA_ZEROS) {
            c.g[V0][0] = 0;
        }
        if n == dirty_rpc_send_t_get(c, DIRTY_RPC_SEND_T_DMA_ARM_AT) {
            let delay = dirty_rpc_send_t_get(c, DIRTY_RPC_SEND_T_DMA_ARM_DELAY);
            c.next_event = c.cycles + delay as u64;
        }
    }

    /// `GetThreadId` for the fixture: the real wrapper, then optionally an
    /// event falls due.
    pub(crate) fn dirty_rpc_send_t_thread_id(c: &mut Cpu) {
        let prev = prev_cell(dirty_rpc_send_t_thread_id);
        if !dirty_rpc_send_t_gated(c) {
            return fall_back(c, prev, ee::s15::f_00423c90__generated);
        }
        let ra = r32(c, RA);
        fall_back(c, prev, ee::s15::f_00423c90__generated);
        if c.pc == ra && dirty_rpc_send_t_get(c, DIRTY_RPC_SEND_T_TID_ARM) != 0 {
            c.next_event = c.cycles;
        }
    }

    /// `SyncDCache` for the fixture: counts the calls (it leaves no trace in
    /// RAM otherwise).
    pub(crate) fn dirty_rpc_send_t_sync_dcache(c: &mut Cpu) {
        let prev = prev_cell(dirty_rpc_send_t_sync_dcache);
        if dirty_rpc_send_t_gated(c) {
            dirty_rpc_send_t_bump(c, DIRTY_RPC_SEND_T_SYNCS);
        }
        fall_back(c, prev, ee::s15::f_00424698__generated)
    }

    pub(crate) fn dirty_rpc_send_t_hook() {
        hook(
            addr_of_mut!(ee::s15::OVERRIDE_00423DE0),
            dirty_rpc_send_t_wait_sema,
            prev_cell(dirty_rpc_send_t_wait_sema),
        );
        hook(
            addr_of_mut!(ee::s15::OVERRIDE_00424160),
            dirty_rpc_send_t_set_dma,
            prev_cell(dirty_rpc_send_t_set_dma),
        );
        hook(
            addr_of_mut!(ee::s15::OVERRIDE_00423C90),
            dirty_rpc_send_t_thread_id,
            prev_cell(dirty_rpc_send_t_thread_id),
        );
        hook(
            addr_of_mut!(ee::s15::OVERRIDE_00424698),
            dirty_rpc_send_t_sync_dcache,
            prev_cell(dirty_rpc_send_t_sync_dcache),
        );
    }

    /// The steering of one `DirtyRpc_Send` fixture.
    #[derive(Clone, Copy)]
    pub(crate) struct DirtyRpcSendPlan {
        /// 0 no client, 1 unbound client (+0xc = 0), 2 unaligned reply buffer,
        /// 3 a string (`len < 0`), 4 and 9 bytes, 5 no bytes, 6 a gather list,
        /// 7 a gather list with `len < 0` (strlen runs first), 8 an empty one.
        shape: u32,
        /// 0 room behind the read offset, 1 the same one byte short, 2 room at
        /// the end, 3 room only from 0, 4 the same one byte short.
        room: u32,
        /// 0: a free slot; else the ring is full and the tail's reply lands
        /// during this WaitSema call (1: one wait round, 2/3: more).
        land: u32,
        /// Gather pieces (1-4), all one byte long if `unit_pieces`.
        pieces: u32,
        unit_pieces: bool,
        sync: bool,
        sif_cmd: bool,
        /// sceSifSetDma calls that return id 0 first.
        dma_zeros: u32,
        /// The mode word (1 blocks).
        mode: u32,
        alarm_running: bool,
    }

    /// The seed digits of `dirty_rpc_send_matches`.
    pub(crate) fn dirty_rpc_send_t_plan(seed: u32) -> DirtyRpcSendPlan {
        DirtyRpcSendPlan {
            shape: seed % 10,
            room: seed / 10 % 5,
            land: [0, 1, 2 + seed % 2][(seed / 50 % 3) as usize],
            pieces: 1 + seed % 4,
            unit_pieces: false,
            sync: seed % 3 == 1,
            sif_cmd: seed % 7 >= 3,
            dma_zeros: seed % 11 % 3,
            mode: [0, 1, 5][(seed % 13 % 3) as usize],
            alarm_running: seed % 17 % 2 == 1,
        }
    }

    /// What a fixture chose, to read the outcome against.
    pub(crate) struct DirtyRpcSendCase {
        plan: DirtyRpcSendPlan,
        /// The length the function arrives at.
        len: u32,
        write: u32,
        head: u32,
        id: u32,
        /// The bytes it copies, in order.
        source: Vec<(u32, u32)>,
    }

    /// `NetTick`'s clock: the next millisecond starts `lead` cycles from now.
    pub(crate) fn dirty_rpc_send_t_clock(c: &mut Cpu, r: &mut Rng, lead: u32) {
        c.bus
            .write32(DIRTY_RPC_STATE + DIRTY_RPC_CLOCK_COUNT, c.cycles as u32);
        let base = (1000 + (r.next_u32() % 1000) as u64) * 0x48000 + 0x48000 - lead as u64;
        c.bus.write64(DIRTY_RPC_STATE + DIRTY_RPC_CLOCK_BASE, base);
    }

    /// A DirtyRpc state block with junk tables, a ring of 2-7 slots, data,
    /// room and arguments as `plan` says, the fakes' control block, and the
    /// game image (the SIF packet's size byte).
    pub(crate) fn dirty_rpc_send_setup(
        c: &mut Cpu,
        seed: u32,
        calls: u32,
        plan: DirtyRpcSendPlan,
    ) -> DirtyRpcSendCase {
        let mut r = machine(c, seed, 0x3f_4528, calls);
        elf_image(c);
        c.pc = DIRTY_RPC_SEND_ENTRY;
        fill(c, &mut r, DATA, 0x4000);
        let slots = 2 + r.next_u32() % 6;
        let head = r.next_u32() % slots;
        let tail = if plan.land == 0 {
            head
        } else {
            (head + 1) % slots
        };
        let limit = 0x100 + r.next_u32() % 0x100;
        let id = r.next_u32() % limit;
        // No reply has arrived for any slot.
        for i in 0..slots {
            let seq = c.bus.read32(DIRTY_RPC_SEND_T_REQUESTS + 0x20 * i + 0x1c);
            c.bus.write32(
                DIRTY_RPC_SEND_T_REPLIES + 0x20 * i + 0x1c,
                seq ^ 0x5a5a_0001,
            );
        }
        if plan.land != 0 {
            // The tail's request: no reply callback, no data (the read offset
            // stays where the room plan puts it).
            c.bus.write32(DIRTY_RPC_SEND_T_WAITERS + 0x10 * tail, 0);
            c.bus
                .write32(DIRTY_RPC_SEND_T_REQUESTS + 0x20 * tail + 4, 0);
        }

        // The data: (pointer, len argument, the length the code computes).
        let mut source = Vec::new();
        let (data, arg_len, len) = match plan.shape {
            3 => {
                let l = 5 + r.next_u32() % 24;
                for i in 0..l {
                    c.bus
                        .write8(DIRTY_RPC_SEND_T_DATA + i, b'a' + (i % 26) as u8);
                }
                c.bus.write8(DIRTY_RPC_SEND_T_DATA + l, 0);
                source.push((DIRTY_RPC_SEND_T_DATA, l + 1));
                let arg = -(1 + (r.next_u32() % 0x100) as i32);
                (DIRTY_RPC_SEND_T_DATA, arg, l + 1)
            }
            5 => (DIRTY_RPC_SEND_T_DATA, 0, 0),
            6..=8 => {
                let n = if plan.shape == 8 { 0 } else { plan.pieces };
                let mut total = 0;
                for i in 0..n {
                    let l = if plan.unit_pieces {
                        1
                    } else {
                        1 + r.next_u32() % 0x30
                    };
                    let p = DIRTY_RPC_SEND_T_PIECES + 0x100 * i;
                    c.bus.write32(DIRTY_RPC_SEND_T_GATHER + 8 * i, l);
                    c.bus.write32(DIRTY_RPC_SEND_T_GATHER + 8 * i + 4, p);
                    source.push((p, l));
                    total += l;
                }
                let end = if r.next_u32() % 2 == 0 {
                    0
                } else {
                    (-(1 + (r.next_u32() % 8) as i32)) as u32
                };
                c.bus.write32(DIRTY_RPC_SEND_T_GATHER + 8 * n, end);
                let arg = if plan.shape == 7 {
                    -(1 + (r.next_u32() % 8) as i32)
                } else {
                    (r.next_u32() % 0x100) as i32
                };
                (DIRTY_RPC_SEND_T_GATHER, arg, total)
            }
            _ => {
                let l = 1 + r.next_u32() % 0x90;
                source.push((DIRTY_RPC_SEND_T_DATA, l));
                (DIRTY_RPC_SEND_T_DATA, l as i32, l)
            }
        };

        // Write and read offsets for the room plan.
        let size = DIRTY_RPC_SEND_T_BUFFER_SIZE;
        let (write, read) = match plan.room {
            0 => {
                let w = 0x100 + (r.next_u32() % 0x10) * 0x10;
                (w, w + 0x40 + len + (r.next_u32() % 3) * 0x10)
            }
            1 => {
                let w = 0x100 + (r.next_u32() % 0x10) * 0x10;
                (w, w + 0x3f + len)
            }
            2 => (
                size - 0x40 - len - (r.next_u32() % 4) * 0x10,
                0x80 + (r.next_u32() % 8) * 0x10,
            ),
            3 => (
                size - 0x3f - len + r.next_u32() % 0x30,
                len + 0x40 + (r.next_u32() % 4) * 0x10,
            ),
            _ => (size - 0x3f - len + r.next_u32() % 0x30, len + 0x3f),
        };

        let alarm = if plan.alarm_running {
            r.next_u32() % 0x40
        } else {
            0x8000_0000 | r.next_u32()
        };
        let sema = 1 + r.next_u32() % 0x40;
        for (off, v) in [
            (DIRTY_RPC_ID_LIMIT, limit),
            (DIRTY_RPC_REQUESTS, DIRTY_RPC_SEND_T_REQUESTS),
            (DIRTY_RPC_IOP_REQUESTS, DIRTY_RPC_SEND_T_IOP_REQUESTS),
            (DIRTY_RPC_REPLIES, DIRTY_RPC_SEND_T_REPLIES),
            (DIRTY_RPC_EE_BUFFER, DIRTY_RPC_SEND_T_EE_BUFFER),
            (DIRTY_RPC_IOP_BUFFER, DIRTY_RPC_SEND_T_IOP_BUFFER),
            (DIRTY_RPC_DMA_IDS, DIRTY_RPC_SEND_T_DMA_IDS),
            (DIRTY_RPC_WAITERS, DIRTY_RPC_SEND_T_WAITERS),
            (DIRTY_RPC_NEXT_ID, id),
            (DIRTY_RPC_HEAD, head),
            (DIRTY_RPC_TAIL, tail),
            (DIRTY_RPC_SLOTS, slots),
            (DIRTY_RPC_WRITE, write),
            (DIRTY_RPC_READ, read),
            (DIRTY_RPC_BUFFER_SIZE, size),
            (DIRTY_RPC_ALARM, alarm),
            (DIRTY_RPC_SEMA, sema),
        ] {
            c.bus.write32(DIRTY_RPC_STATE + off, v);
        }

        // Arguments.
        let bound = if plan.shape == 1 { 0 } else { r.next_u32() | 1 };
        c.bus
            .write32(DIRTY_RPC_SEND_T_CLIENT + DIRTY_RPC_CLIENT_ID, bound);
        c.g[A0][0] = if plan.shape == 0 {
            0
        } else {
            sx(DIRTY_RPC_SEND_T_CLIENT)
        };
        let mut flags = r.next_u32() & !7;
        if matches!(plan.shape, 6..=8) {
            flags |= DIRTY_RPC_SEND_GATHER as u32;
        }
        if plan.sif_cmd {
            flags |= DIRTY_RPC_SEND_SIF_CMD as u32;
        }
        if plan.sync {
            flags |= DIRTY_RPC_SEND_SYNC_REPLY as u32;
        }
        c.g[A1][0] = sx(flags);
        c.g[A2][0] = (r.next_u32() as u64) << 32 | r.next_u32() as u64;
        c.g[A3][0] = (r.next_u32() as u64) << 32 | r.next_u32() as u64;
        c.g[T0][0] = sx(data);
        c.g[T1][0] = sx(arg_len as u32);
        let reply = if plan.shape == 2 {
            DIRTY_RPC_SEND_T_REPLY + 1 + r.next_u32() % 15
        } else {
            DIRTY_RPC_SEND_T_REPLY
        };
        c.g[T2][0] = sx(reply);
        c.g[T3][0] = sx(0x40 + r.next_u32() % 0x100);
        let stack = r32(c, SP);
        c.bus.write32(stack, plan.mode);
        c.bus.write32(stack + 8, r.next_u32());

        // The fakes.
        let land_at = if plan.land == 0 { u32::MAX } else { plan.land };
        let tail_id = c.bus.read32(DIRTY_RPC_SEND_T_REQUESTS + 0x20 * tail + 0x1c);
        for (word, v) in [
            (0, DIRTY_RPC_SEND_T_MAGIC),
            (DIRTY_RPC_SEND_T_WAITS, 0),
            (DIRTY_RPC_SEND_T_LAND_AT, land_at),
            (
                DIRTY_RPC_SEND_T_LAND_ADDR,
                DIRTY_RPC_SEND_T_REPLIES + 0x20 * tail + 0x1c,
            ),
            (DIRTY_RPC_SEND_T_LAND_VALUE, tail_id),
            (DIRTY_RPC_SEND_T_WAIT_ARM_AT, u32::MAX),
            (DIRTY_RPC_SEND_T_DMAS, 0),
            (DIRTY_RPC_SEND_T_DMA_ZEROS, plan.dma_zeros),
            (DIRTY_RPC_SEND_T_DMA_ARM_AT, u32::MAX),
            (DIRTY_RPC_SEND_T_DMA_ARM_DELAY, 0),
            (DIRTY_RPC_SEND_T_TID_ARM, 0),
            (DIRTY_RPC_SEND_T_SYNCS, 0),
        ] {
            dirty_rpc_send_t_set(c, word, v);
        }
        dirty_rpc_send_t_clock(c, &mut r, 12_000);
        DirtyRpcSendCase {
            plan,
            len,
            write,
            head,
            id,
            source,
        }
    }

    /// The arms a finished generated run took, from its fake-call log, the
    /// request descriptor, the data buffer and the SIF packet.
    pub(crate) fn dirty_rpc_send_t_arms(
        c: &mut Cpu,
        case: &DirtyRpcSendCase,
        arms: &mut BTreeSet<&'static str>,
    ) {
        let plan = case.plan;
        let frame = crate::difftest::SP - 0xe0;
        let (mut waits, mut thread, mut alarm, mut sleep) = (0, false, false, false);
        let mut dmas = Vec::new();
        let calls = c.bus.read32(LOG_COUNT);
        for i in 0..calls.min(64) {
            let p = LOG_START + 0x220 * i;
            let target = c.bus.read32(p);
            let a0 = c.bus.read32(p + 0x10 + 16 * A0 as u32);
            let a1 = c.bus.read32(p + 0x10 + 16 * A1 as u32);
            match target {
                DIRTY_RPC_SEND_T_WAIT_SYSCALL if !thread => waits += 1,
                DIRTY_RPC_SEND_T_THREAD_SYSCALL => thread = true,
                DIRTY_RPC_SEND_T_DMA_SYSCALL if a0 == frame || a0 == frame + 0x10 => dmas.push(a1),
                DIRTY_RPC_SEND_T_ALARM_SYSCALL => alarm = true,
                DIRTY_RPC_SEND_T_SLEEP_SYSCALL => sleep = true,
                _ => {}
            }
        }
        let v0 = c.g[V0][0];
        if v0 == u64::MAX && waits >= 1 && !thread {
            match plan.room {
                1 => arms.insert("no room behind the reader"),
                4 => arms.insert("no room after wrapping"),
                _ => false,
            };
        }
        if v0 != case.id as u64 || !thread {
            return;
        }
        let req = DIRTY_RPC_SEND_T_REQUESTS + 0x20 * case.head;
        let at = c.bus.read32(req).wrapping_sub(DIRTY_RPC_SEND_T_IOP_BUFFER);
        let len = c.bus.read32(req + 4);
        match plan.room {
            0 if at == case.write => arms.insert("room behind the reader"),
            2 if at == case.write => arms.insert("room at the end"),
            3 if at == 0 => arms.insert("room after wrapping"),
            _ => false,
        };
        match waits {
            1 => arms.insert("free slot"),
            4 => arms.insert("full ring, one wait"),
            n if n >= 5 => arms.insert("full ring, several waits"),
            _ => false,
        };
        if len == case.len {
            match plan.shape {
                3 => arms.insert("length from strlen"),
                7 => arms.insert("gather after strlen"),
                5 | 8 => arms.insert("no data"),
                _ => false,
            };
        }
        let dst = DIRTY_RPC_SEND_T_EE_BUFFER + at;
        let mut off = 0;
        let mut copied = !case.source.is_empty();
        for &(p, l) in &case.source {
            for k in 0..l {
                if c.bus.read8(dst + off + k) != c.bus.read8(p + k) {
                    copied = false;
                }
            }
            off += l;
        }
        if copied {
            arms.insert(match (plan.shape, case.source.len()) {
                (6 | 7, 1) => "gather copy, one piece",
                (6 | 7, _) => "gather copy loop",
                _ => "flat copy",
            });
        }
        if dirty_rpc_send_t_get(c, DIRTY_RPC_SEND_T_SYNCS) != 0 {
            arms.insert("reply buffer synced");
        }
        match dmas.first().copied() {
            Some(2) => arms.insert("data and descriptor DMA"),
            Some(1) => arms.insert("descriptor-only DMA"),
            _ => false,
        };
        match dmas.get(1).copied() {
            Some(2) => arms.insert("retried data and descriptor"),
            Some(1) => arms.insert("retried descriptor only"),
            _ => false,
        };
        if c.bus.read32(DIRTY_RPC_SIF_PACKET + 8) == 1 {
            arms.insert("SIF command");
        }
        match (alarm, sleep, plan.alarm_running) {
            (true, true, false) => arms.insert("alarm started, slept"),
            (false, true, true) => arms.insert("slept, alarm running"),
            (false, false, true) => arms.insert("no wait, alarm running"),
            (false, false, false) => arms.insert("no wait, no alarm"),
            _ => false,
        };
    }

    /// Seed digits: `% 10` the client and data shape, `/ 10 % 5` the buffer
    /// room, `/ 50 % 3` the ring (free, full for one wait round, for more);
    /// `% 3` the sync flag, `% 7` the SIF-command flag, `% 11 % 3` the zero
    /// DMA ids before success, `% 13 % 3` the mode (0, 1, 5), `% 17 % 2` a
    /// running alarm. The fakes unwind at one of the first eight kernel calls
    /// in about a third of the seeds. A second pass (no unwinds) checks that
    /// the generated body took every arm: the early returns by their fixed
    /// cycle totals, the rest by their effects.
    #[test]
    fn dirty_rpc_send_matches() {
        const SEEDS: u32 = 150;
        // No client, unbound client, unaligned reply buffer.
        const EARLY: [u64; 3] = [36, 39, 43];
        const ARMS: &[&str] = &[
            "no room behind the reader",
            "no room after wrapping",
            "room behind the reader",
            "room at the end",
            "room after wrapping",
            "free slot",
            "full ring, one wait",
            "full ring, several waits",
            "length from strlen",
            "gather after strlen",
            "no data",
            "flat copy",
            "gather copy, one piece",
            "gather copy loop",
            "reply buffer synced",
            "data and descriptor DMA",
            "descriptor-only DMA",
            "retried data and descriptor",
            "retried descriptor only",
            "SIF command",
            "alarm started, slept",
            "slept, alarm running",
            "no wait, alarm running",
            "no wait, no alarm",
        ];
        dirty_rpc_send_t_hook();
        check_lohi(
            ee::s14::f_003f4528__generated,
            dirty_rpc_send,
            SEEDS,
            &|c, seed| {
                dirty_rpc_send_setup(c, seed, 8, dirty_rpc_send_t_plan(seed));
            },
        );
        let mut totals = BTreeSet::new();
        let mut arms = BTreeSet::new();
        for seed in 0..SEEDS {
            let mut c = fresh();
            let case = dirty_rpc_send_setup(&mut c, seed, 8, dirty_rpc_send_t_plan(seed));
            c.bus.write32(LOG_UNWIND_AT, u32::MAX);
            let before = c.cycles;
            run_generated(&mut c, ee::s14::f_003f4528__generated);
            totals.insert(c.cycles - before);
            dirty_rpc_send_t_arms(&mut c, &case, &mut arms);
        }
        for p in EARLY {
            assert!(
                totals.contains(&p),
                "early return costing {p} cycles never taken"
            );
        }
        for a in ARMS {
            assert!(arms.contains(a), "arm {a:?} never taken (taken: {arms:?})");
        }
    }

    /// One loop head per `seed / 3 % 5` (`seed % 3`: no event, an unwinding
    /// VBlank, a delivered event). 0x3f45e0: four gather pieces, the
    /// scheduled event (due within 48 cycles, before the third back edge).
    /// 0x3f4648: a full ring, the event due after the first WaitSema in the
    /// wait loop. 0x3f4778: three one-byte gather pieces (memcpy makes no
    /// event check), the event due after GetThreadId. 0x3f48f8: the first DMA
    /// id 0, the event due at once. 0x3f4908: the same, due 8-39 cycles later
    /// (past the retry back edge). The first unwind lands on head `h` at
    /// seeds `15k + 3h + 1`.
    #[test]
    fn dirty_rpc_send_events() {
        driven!(dirty_rpc_send_t_gen, ee::s14::f_003f4528__generated);
        driven!(dirty_rpc_send_t_hand, dirty_rpc_send);
        dirty_rpc_send_t_hook();
        check_events(
            dirty_rpc_send_t_gen,
            dirty_rpc_send_t_hand,
            60,
            &|c, seed| {
                let head = seed / 3 % 5;
                let plan = DirtyRpcSendPlan {
                    shape: if head == 0 || head == 2 { 6 } else { 4 },
                    room: 0,
                    land: (head == 1) as u32,
                    pieces: if head == 0 { 4 } else { 3 },
                    unit_pieces: head == 2,
                    sync: false,
                    sif_cmd: false,
                    dma_zeros: (head >= 3) as u32,
                    mode: 0,
                    alarm_running: true,
                };
                dirty_rpc_send_setup(c, seed, 0, plan);
                let mut r = Rng(0x3f_4528 ^ (seed as u64) << 8);
                let mode = schedule(c, &mut r, seed, DIRTY_RPC_SEND_ENTRY);
                dirty_rpc_send_t_clock(c, &mut r, if head == 4 { 40_000 } else { 12_000 });
                if mode != Events::Off && head != 0 {
                    c.next_event = CYCLE_BUDGET;
                    match head {
                        1 => dirty_rpc_send_t_set(c, DIRTY_RPC_SEND_T_WAIT_ARM_AT, 1),
                        2 => dirty_rpc_send_t_set(c, DIRTY_RPC_SEND_T_TID_ARM, 1),
                        3 => dirty_rpc_send_t_set(c, DIRTY_RPC_SEND_T_DMA_ARM_AT, 0),
                        _ => {
                            dirty_rpc_send_t_set(c, DIRTY_RPC_SEND_T_DMA_ARM_AT, 0);
                            dirty_rpc_send_t_set(
                                c,
                                DIRTY_RPC_SEND_T_DMA_ARM_DELAY,
                                8 + seed / 15 % 32,
                            );
                        }
                    }
                }
                mode
            },
        );
    }

    // The test socket: the object, its address block, its receive buffer.
    pub(crate) const PROTO_ARIES_SOCK: u32 = DATA + 0x1_0000;

    pub(crate) const PROTO_ARIES_SOCK_ADDRS: u32 = DATA + 0x1_0100;

    pub(crate) const PROTO_ARIES_SOCK_BUF: u32 = DATA + 0x1_1000;

    pub(crate) const PROTO_ARIES_SOCK_BUF_SIZE: u32 = 0x1000;

    /// Where `ProtoAriesCreate`'s allocation lands.
    pub(crate) const PROTO_ARIES_NEW: u32 = DATA + 0x2_0000;

    /// Output cells, and the send data / receive destination.
    pub(crate) const PROTO_ARIES_OUT: u32 = DATA + 0x5_0000;

    pub(crate) const PROTO_ARIES_PAYLOAD: u32 = DATA + 0x6_0000;

    /// The tables DirtyRpc's state fields point at in the test machine.
    pub(crate) const PROTO_ARIES_RPC_TABLE: u32 = DATA + 0x4_0000;

    pub(crate) const PROTO_ARIES_RPC_SENDS: u32 = DATA + 0x4_1000;

    pub(crate) const PROTO_ARIES_RPC_IDS: u32 = DATA + 0x4_2000;

    pub(crate) const PROTO_ARIES_RPC_REPLY_SLOTS: u32 = DATA + 0x4_2100;

    pub(crate) const PROTO_ARIES_RPC_REQUESTS: u32 = DATA + 0x4_2200;

    /// Register the Aries server's entry points with DirtyRpc and give its
    /// send path an empty 8-entry ring, room in the send buffer and tables
    /// (pre-filled) to write; the kernel calls it makes are the machine's
    /// fake syscalls. Returns the ring head.
    pub(crate) fn proto_aries_rpc_server(c: &mut Cpu, r: &mut Rng) -> u32 {
        let codes = [
            PROTO_ARIES_CMD_CREATE,
            PROTO_ARIES_CMD_DESTROY,
            PROTO_ARIES_CMD_CONNECT,
            PROTO_ARIES_CMD_DISCONNECT,
            PROTO_ARIES_CMD_STATUS,
            PROTO_ARIES_CMD_SEND,
            PROTO_ARIES_CMD_RECV,
        ];
        for (i, code) in codes.into_iter().enumerate() {
            let e = PROTO_ARIES_RPC_TABLE + 0x10 * i as u32;
            c.bus.write32(e, i as u32);
            c.bus.write32(e + 4, PROTO_ARIES_SERVER);
            c.bus.write32(e + 8, code);
            c.bus.write32(e + 0xc, r.next_u32() | 1);
        }
        let head = r.next_u32() % 8;
        let rpc = DIRTY_RPC_STATE;
        let fields = [
            (DIRTY_RPC_CLIENTS, PROTO_ARIES_RPC_TABLE),
            (
                DIRTY_RPC_CLIENTS_END,
                PROTO_ARIES_RPC_TABLE + 0x10 * codes.len() as u32,
            ),
            (DIRTY_RPC_ID_LIMIT, 0x100),
            (DIRTY_RPC_NEXT_ID, r.next_u32() % 0x100),
            (DIRTY_RPC_SLOTS, 8),
            (DIRTY_RPC_HEAD, head),
            (DIRTY_RPC_TAIL, head),
            (DIRTY_RPC_WRITE, 0),
            (DIRTY_RPC_READ, 0x800),
            (DIRTY_RPC_BUFFER_SIZE, 0x800),
            (DIRTY_RPC_EE_BUFFER, PROTO_ARIES_RPC_SENDS),
            (DIRTY_RPC_IOP_BUFFER, r.next_u32() & !0x3f),
            (DIRTY_RPC_DMA_IDS, PROTO_ARIES_RPC_IDS),
            (DIRTY_RPC_WAITERS, PROTO_ARIES_RPC_REPLY_SLOTS),
            (DIRTY_RPC_REQUESTS, PROTO_ARIES_RPC_REQUESTS),
            (DIRTY_RPC_IOP_REQUESTS, r.next_u32() & !0x1f),
            (DIRTY_RPC_ALARM, r.next_u32() >> 1),
            (DIRTY_RPC_SEMA, r.next_u32() & 0xff),
        ];
        for (off, v) in fields {
            c.bus.write32(rpc + off, v);
        }
        fill(c, r, PROTO_ARIES_RPC_SENDS, 0x80);
        fill(c, r, PROTO_ARIES_RPC_IDS, 8);
        fill(c, r, PROTO_ARIES_RPC_REPLY_SLOTS, 0x20);
        fill(c, r, PROTO_ARIES_RPC_REQUESTS, 0x40);
        head
    }

    /// A fake machine entering at `entry` with `a0` = a socket object at
    /// `PROTO_ARIES_SOCK` (fields, address block and receive buffer filled
    /// with junk) and the allocator fakes on. With `server` the Aries server
    /// is registered (`proto_aries_rpc_server`); without it
    /// `DirtyRpc_InitRequest` finds no client and `DirtyRpc_Send` returns at
    /// once.
    pub(crate) fn proto_aries_socket_fixture(
        c: &mut Cpu,
        seed: u32,
        entry: u32,
        server: bool,
    ) -> Rng {
        mem_fakes_hook_with_new();
        let mut r = machine(c, seed, entry as u64, 0);
        c.pc = entry;
        c.bus.write32(MEM_FAKES_CTRL, MEM_FAKES_MAGIC);
        let sock = PROTO_ARIES_SOCK;
        fill(c, &mut r, sock, 0x28);
        fill(c, &mut r, PROTO_ARIES_SOCK_ADDRS, 0x10);
        fill(
            c,
            &mut r,
            PROTO_ARIES_SOCK_BUF - 0x100,
            (PROTO_ARIES_SOCK_BUF_SIZE + 0x200) / 4,
        );
        c.bus
            .write32(sock + PROTO_ARIES_ADDRS, PROTO_ARIES_SOCK_ADDRS);
        c.bus
            .write32(sock + PROTO_ARIES_RECV_SIZE, PROTO_ARIES_SOCK_BUF_SIZE);
        c.bus
            .write32(sock + PROTO_ARIES_RECV_BUF, PROTO_ARIES_SOCK_BUF);
        c.g[A0][0] = sx(sock);
        if server {
            proto_aries_rpc_server(c, &mut r);
        }
        r
    }

    /// Whether `request` found the server (its client handle was copied in).
    pub(crate) fn proto_aries_sent(c: &mut Cpu, request: u32) -> bool {
        c.bus.read32(request + DIRTY_RPC_CLIENT_ID) != 0
    }

    pub(crate) fn proto_aries_put_be(c: &mut Cpu, at: u32, v: u32) {
        for (i, b) in v.to_be_bytes().into_iter().enumerate() {
            c.bus.write8(at + i as u32, b);
        }
    }

    pub(crate) fn proto_aries_get_be(c: &mut Cpu, at: u32) -> u32 {
        let b = [
            c.bus.read8(at),
            c.bus.read8(at + 1),
            c.bus.read8(at + 2),
            c.bus.read8(at + 3),
        ];
        u32::from_be_bytes(b)
    }

    /// Run the generated body over the seeds and require every `want` arm,
    /// as `arm` reads it from the final state (for arms whose cycle totals
    /// depend on callee costs: fake syscalls, cache loops, `memset`).
    pub(crate) fn require_arm_values<T: Ord + std::fmt::Debug>(
        generated: GuestFn,
        seeds: u32,
        setup: &dyn Fn(&mut Cpu, u32),
        arm: &dyn Fn(&mut Cpu, u32) -> T,
        want: &[T],
    ) {
        let mut seen = BTreeSet::new();
        for seed in 0..seeds {
            let mut c = fresh();
            setup(&mut c, seed);
            run_generated(&mut c, generated);
            seen.insert(arm(&mut c, seed));
        }
        for w in want {
            assert!(seen.contains(w), "arm {w:?} never taken (seen {seen:?})");
        }
    }

    // --------------------------------------------------- ProtoAriesCreate

    /// `bufsize` for `seed % 8`, and the size the clamp keeps (checked for
    /// every seed that leaves one).
    pub(crate) const PROTO_ARIES_CREATE_SIZES: [(u64, u32); 8] = [
        (0x800, 0x1000),
        (0xffff_ffff_ffff_f000, 0x1000),
        (0x1801, 0x1801),
        (0x2000, 0x2000),
        (0x2001, 0x2000),
        (0x0000_0001_0000_1000, 0x2000),
        (0xfff, 0x1000),
        (0x1001, 0x1001),
    ];

    /// The new object: two placements whose address blocks round to
    /// different lines.
    pub(crate) fn proto_aries_create_object(seed: u32) -> u32 {
        PROTO_ARIES_NEW + 0x28 * (seed / 32 % 2)
    }

    /// `seed % 8`: the size (below the minimum, negative, in range, the
    /// maximum, above it, above it only in the upper word, one below the
    /// minimum, one above it: the lower clamp's threshold exactly).
    /// `seed / 8 % 4`: the allocation fails; no server; a server whose reply
    /// leaves no handle; a reply that leaves one (the DMA-id table is placed
    /// over the new object, so the id `DirtyRpc_Send` stores for this request
    /// lands in word 0 as the IOP's reply would). `seed / 32 % 2`: the
    /// placement.
    pub(crate) fn proto_aries_create_fixture(c: &mut Cpu, seed: u32) {
        let outcome = seed / 8 % 4;
        let mut r = proto_aries_socket_fixture(c, seed, 0x003f_5478, outcome >= 2);
        c.g[A0][0] = PROTO_ARIES_CREATE_SIZES[(seed % 8) as usize].0;
        let obj = proto_aries_create_object(seed);
        let span = 0x40 + PROTO_ARIES_OBJECT_SIZE + PROTO_ARIES_MAX_BUFFER as u32 + 0x40;
        fill(c, &mut r, obj - 0x40, span / 4);
        c.bus
            .write32(MEM_FAKES_CTRL + 0x18, if outcome == 0 { 0 } else { obj });
        if outcome == 3 {
            let head = c.bus.read32(DIRTY_RPC_STATE + DIRTY_RPC_HEAD);
            c.bus.write32(
                DIRTY_RPC_STATE + DIRTY_RPC_DMA_IDS,
                obj.wrapping_sub(4 * head),
            );
        }
    }

    /// (result, kept buffer size, request sent).
    pub(crate) fn proto_aries_create_arm(c: &mut Cpu, seed: u32) -> (&'static str, u32, bool) {
        let obj = proto_aries_create_object(seed);
        let size = c.bus.read32(obj + PROTO_ARIES_RECV_SIZE);
        let sent = proto_aries_sent(c, PROTO_ARIES_CREATE_REQUEST);
        let clamped = PROTO_ARIES_CREATE_SIZES[(seed % 8) as usize].1;
        if c.g[V0][0] != 0 {
            assert_eq!(size, clamped, "seed {seed}: kept size");
            ("kept", size, sent)
        } else if (0x1000..=0x2000).contains(&size) {
            assert_eq!(size, clamped, "seed {seed}: released size");
            ("released", size, sent)
        } else {
            ("no memory", 0, sent)
        }
    }

    #[test]
    fn proto_aries_create_matches() {
        let setup = proto_aries_create_fixture;
        check(
            ee::s14::f_003f5478__generated,
            proto_aries_create,
            64,
            &setup,
        );
        let mut want: Vec<(&'static str, u32, bool)> = vec![("no memory", 0, false)];
        for size in [0x1000, 0x1001, 0x1801, 0x2000] {
            want.push(("kept", size, true));
            want.push(("released", size, false));
            want.push(("released", size, true));
        }
        require_arm_values(
            ee::s14::f_003f5478__generated,
            64,
            &setup,
            &proto_aries_create_arm,
            &want[..],
        );
    }

    // -------------------------------------------------- ProtoAriesDestroy

    /// `seed % 2`: no server / the server.
    #[test]
    fn proto_aries_destroy_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            proto_aries_socket_fixture(c, seed, 0x003f_55b8, seed % 2 == 1);
        };
        check(
            ee::s14::f_003f55b8__generated,
            proto_aries_destroy,
            16,
            &setup,
        );
        require_arm_values(
            ee::s14::f_003f55b8__generated,
            16,
            &setup,
            &|c, _| proto_aries_sent(c, PROTO_ARIES_DESTROY_REQUEST),
            &[false, true],
        );
    }

    // -------------------------------------------------- ProtoAriesConnect

    /// `seed % 2`: the peer family is 0 / already set (-1). `seed / 2 % 2`:
    /// no server / the server. Address and port carry junk upper halves.
    pub(crate) fn proto_aries_connect_fixture(c: &mut Cpu, seed: u32) {
        let mut r = proto_aries_socket_fixture(c, seed, 0x003f_5638, seed / 2 % 2 == 1);
        let family = if seed % 2 == 0 {
            0
        } else {
            r.next_u32() as u16 | 1
        };
        c.bus
            .write16(PROTO_ARIES_SOCK_ADDRS + PROTO_ARIES_PEER, family);
        c.g[A1][0] = (r.next_u32() as u64) << 32 | r.next_u32() as u64;
        c.g[A2][0] = (r.next_u32() as u64) << 32 | r.next_u32() as u64;
    }

    #[test]
    fn proto_aries_connect_matches() {
        let setup = proto_aries_connect_fixture;
        // Already connected: 9 + 5.
        check_paths(
            ee::s14::f_003f5638__generated,
            proto_aries_connect,
            32,
            &[14],
            &setup,
        );
        require_arm_values(
            ee::s14::f_003f5638__generated,
            32,
            &setup,
            &|c, _| {
                let busy = c.g[V0][0] == u64::MAX;
                (busy, proto_aries_sent(c, PROTO_ARIES_CONNECT_REQUEST))
            },
            &[(true, false), (false, false), (false, true)],
        );
    }

    // ------------------------------------------------ ProtoAriesUnconnect

    /// `seed % 2`: no server / the server.
    #[test]
    fn proto_aries_unconnect_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            proto_aries_socket_fixture(c, seed, 0x003f_5738, seed % 2 == 1);
        };
        check(
            ee::s14::f_003f5738__generated,
            proto_aries_unconnect,
            16,
            &setup,
        );
        require_arm_values(
            ee::s14::f_003f5738__generated,
            16,
            &setup,
            &|c, _| proto_aries_sent(c, PROTO_ARIES_UNCONNECT_REQUEST),
            &[false, true],
        );
    }

    // --------------------------------------------------- ProtoAriesStatus

    /// `seed % 7`: `'port'`, `'addr'`, `'ladr'`, `'lprt'`, `'ibuf'`, another
    /// code, `'port'` with a non-zero upper word (the compare is 64-bit).
    /// `seed / 7 % 2`: no server / the server.
    pub(crate) fn proto_aries_status_fixture(c: &mut Cpu, seed: u32) {
        proto_aries_socket_fixture(c, seed, 0x003f_57c8, seed / 7 % 2 == 1);
        c.g[A1][0] = match seed % 7 {
            0 => PROTO_ARIES_SEL_PORT as u64,
            1 => PROTO_ARIES_SEL_ADDR as u64,
            2 => PROTO_ARIES_SEL_LOCAL_ADDR as u64,
            3 => PROTO_ARIES_SEL_LOCAL_PORT as u64,
            4 => PROTO_ARIES_SEL_INPUT as u64,
            5 => 0x7a7a_7a7a,
            _ => 1 << 32 | PROTO_ARIES_SEL_PORT as u64,
        };
    }

    #[test]
    fn proto_aries_status_matches() {
        let setup = proto_aries_status_fixture;
        // Without the server: 31 + 16 (InitRequest) + 51 (Call), then 'port'
        // 13, 'addr' 22, 'ladr' 24, 'lprt' 21, 'ibuf' 19, other 18.
        check_paths(
            ee::s14::f_003f57c8__generated,
            proto_aries_status,
            42,
            &[111, 120, 122, 119, 117, 116],
            &setup,
        );
        require_arm_values(
            ee::s14::f_003f57c8__generated,
            42,
            &setup,
            &|c, _| proto_aries_sent(c, PROTO_ARIES_STATUS_REQUEST),
            &[false, true],
        );
    }

    // ---------------------------------------------------- ProtoAries_Nop

    #[test]
    fn proto_aries_nop_matches() {
        check(
            ee::s14::f_003f5908__generated,
            proto_aries_nop,
            8,
            &|c, seed| {
                let _r = machine(c, seed, 0x3f5908, 0);
                c.pc = 0x003f_5908;
            },
        );
    }

    // ----------------------------------------------------- ProtoAriesSend

    /// `seed % 3`: a length (0..=0x30), a negative length (the data is a
    /// string: `strlen + 1`), 0x8000_0000 zero-extended (non-negative as a
    /// 64-bit value). `seed / 3 % 2`: no server / the server. `seed / 6 % 4`:
    /// the string length (0, 5, 10, 15). The header words carry junk upper
    /// halves.
    pub(crate) fn proto_aries_send_fixture(c: &mut Cpu, seed: u32) {
        let mut r = proto_aries_socket_fixture(c, seed, 0x003f_5910, seed / 3 % 2 == 1);
        for i in 0..0x10 {
            c.bus
                .write32(PROTO_ARIES_PAYLOAD + 4 * i, r.next_u32() | 0x0101_0101);
        }
        c.bus.write8(PROTO_ARIES_PAYLOAD + seed / 6 % 4 * 5, 0);
        c.g[A1][0] = (r.next_u32() as u64) << 32 | r.next_u32() as u64;
        c.g[A2][0] = (r.next_u32() as u64) << 32 | r.next_u32() as u64;
        c.g[A3][0] = sx(PROTO_ARIES_PAYLOAD);
        c.g[T0][0] = match seed % 3 {
            0 => (r.next_u32() % 0x31) as u64,
            1 => sx(r.next_u32() | 0x8000_0000),
            _ => 0x8000_0000,
        };
    }

    /// (length form, the header's length right for the string form, sent).
    pub(crate) fn proto_aries_send_arm(c: &mut Cpu, seed: u32) -> (&'static str, bool, bool) {
        let header = crate::difftest::SP - 0xc0 + 0x10;
        let total = proto_aries_get_be(c, header + PROTO_ARIES_RECORD_LEN);
        let sent = proto_aries_sent(c, PROTO_ARIES_SEND_REQUEST);
        if seed % 3 == 1 {
            (
                "string",
                total == seed / 6 % 4 * 5 + 1 + PROTO_ARIES_RECORD_HEADER,
                sent,
            )
        } else {
            ("given", true, sent)
        }
    }

    #[test]
    fn proto_aries_send_matches() {
        let setup = proto_aries_send_fixture;
        // A given length without the server: 17 + 2 (Nop) + 59 + 16 + 51.
        check_paths(
            ee::s14::f_003f5910__generated,
            proto_aries_send,
            48,
            &[145],
            &setup,
        );
        require_arm_values(
            ee::s14::f_003f5910__generated,
            48,
            &setup,
            &proto_aries_send_arm,
            &[
                ("given", true, false),
                ("given", true, true),
                ("string", true, false),
                ("string", true, true),
            ],
        );
    }

    // ----------------------------------------------------- ProtoAriesPeek

    /// `seed % 4`: a record at the read offset; nothing unread with a receive
    /// pending; nothing unread after a reply with bytes (the buffer is
    /// dropped and rewound, then a new receive); nothing unread and the fill
    /// word 0 or negative (a new receive only). For a record, `seed / 4 % 4`:
    /// its length (a payload of 1..=0x20 bytes, exactly the header, shorter
    /// than the header, a huge one that is negative as a word) and
    /// `seed / 16 % 8`: which of the three outputs are non-null. Otherwise
    /// `seed / 4 % 2`: no server / the server, and `seed / 8 % 2` the read
    /// offset equal to or past the fill (pending, drained) or the fill 0 or
    /// negative.
    pub(crate) fn proto_aries_peek_fixture(c: &mut Cpu, seed: u32) {
        let state = seed % 4;
        let server = state >= 2 && seed / 4 % 2 == 1;
        let mut r = proto_aries_socket_fixture(c, seed, 0x003f_5a70, server);
        fill(c, &mut r, PROTO_ARIES_OUT, 4);
        let outs = seed / 16 % 8;
        c.g[A1][0] = if outs & 1 != 0 {
            sx(PROTO_ARIES_OUT)
        } else {
            0
        };
        c.g[A2][0] = if outs & 2 != 0 {
            sx(PROTO_ARIES_OUT + 4)
        } else {
            0
        };
        c.g[A3][0] = if outs & 4 != 0 {
            sx(PROTO_ARIES_OUT + 8)
        } else {
            0
        };
        let (read, filled, pending) = match state {
            0 => {
                let read = r.next_u32() % 8 * 0x10;
                let len = match seed / 4 % 4 {
                    0 => PROTO_ARIES_RECORD_HEADER + 1 + r.next_u32() % 0x20,
                    1 => PROTO_ARIES_RECORD_HEADER,
                    2 => 5,
                    _ => 0xffff_fff0,
                };
                proto_aries_put_be(c, PROTO_ARIES_SOCK_BUF + read + PROTO_ARIES_RECORD_LEN, len);
                (read, read + 0x100, r.next_u32())
            }
            1 => (0x40, 0x40 - 4 * (seed / 8 % 2), r.next_u32() | 1),
            2 => (0x40 + 0x10 * (seed / 8 % 2), 0x40, 0),
            _ => (0x10, if seed / 8 % 2 == 0 { 0 } else { 0xffff_ffe0 }, 0),
        };
        let sock = PROTO_ARIES_SOCK;
        c.bus.write32(sock + PROTO_ARIES_RECV_READ, read);
        c.bus.write32(sock + PROTO_ARIES_RECV_FILL, filled);
        c.bus.write32(sock + PROTO_ARIES_RECV_PENDING, pending);
    }

    /// (the buffer was rewound, a receive was sent).
    pub(crate) fn proto_aries_peek_arm(c: &mut Cpu, _seed: u32) -> (bool, bool) {
        let sock = PROTO_ARIES_SOCK;
        let rewound = c.g[V0][0] == u64::MAX
            && c.bus.read32(sock + PROTO_ARIES_RECV_FILL) == 0
            && c.bus.read32(sock + PROTO_ARIES_RECV_READ) == 0;
        (rewound, proto_aries_sent(c, PROTO_ARIES_PEEK_REQUEST))
    }

    #[test]
    fn proto_aries_peek_matches() {
        let setup = proto_aries_peek_fixture;
        // A record: 44 with a payload, 46 without, + 11 per non-null header
        // output (the data output costs nothing extra). Pending: 20. A new
        // receive without the server and without a rewind: 119.
        check_paths(
            ee::s14::f_003f5a70__generated,
            proto_aries_peek,
            128,
            &[44, 55, 66, 46, 57, 68, 20, 119],
            &setup,
        );
        require_arm_values(
            ee::s14::f_003f5a70__generated,
            128,
            &setup,
            &proto_aries_peek_arm,
            &[(true, false), (true, true), (false, true)],
        );
    }

    // ----------------------------------------------------- ProtoAriesRecv

    /// `seed % 3`: a record; a record shorter than its header (Peek's
    /// negative length); nothing unread with a receive pending (-1). For a
    /// record, `seed / 3 % 4`: after it, more is unread; the buffer is drained
    /// with a receive pending; drained after a reply with bytes (rewind and a
    /// new receive); drained with the fill word negative (a record ending at
    /// the buffer start: a new receive only). `seed / 12 % 2`: no destination
    /// / a destination. `seed / 24 % 4`: (payload, len) = (0x15, 0x12),
    /// (0x15, 0x15), (0x15, 0x40), (0, 4). `seed / 96 % 2`: no server / the
    /// server. For the short record `seed / 3 % 4` picks its length.
    pub(crate) fn proto_aries_recv_fixture(c: &mut Cpu, seed: u32) {
        let after = seed / 3 % 4;
        let mut r = proto_aries_socket_fixture(c, seed, 0x003f_5c38, seed / 96 % 2 == 1);
        fill(c, &mut r, PROTO_ARIES_OUT, 2);
        fill(c, &mut r, PROTO_ARIES_PAYLOAD, 0x20);
        c.g[A1][0] = sx(PROTO_ARIES_OUT);
        c.g[A2][0] = sx(PROTO_ARIES_OUT + 4);
        c.g[A3][0] = if seed / 12 % 2 == 1 {
            sx(PROTO_ARIES_PAYLOAD)
        } else {
            0
        };
        let buf = PROTO_ARIES_SOCK_BUF;
        let (read, filled, pending, len) = match seed % 3 {
            0 => {
                let (payload, len) = match seed / 24 % 4 {
                    0 => (0x15, 0x12),
                    1 => (0x15, 0x15),
                    2 => (0x15, 0x40),
                    _ => (0, 4),
                };
                let record = PROTO_ARIES_RECORD_HEADER + payload;
                let read = if after == 3 {
                    record.wrapping_neg()
                } else {
                    0x20
                };
                let at = buf.wrapping_add(read) + PROTO_ARIES_RECORD_LEN;
                proto_aries_put_be(c, at, record);
                let (filled, pending) = match after {
                    0 => (read + record + 0x10, r.next_u32()),
                    1 => (read + record, r.next_u32() | 1),
                    2 => (read + record, 0),
                    _ => (0xffff_fffc, 0),
                };
                (read, filled, pending, len)
            }
            1 => {
                proto_aries_put_be(c, buf + 0x20 + PROTO_ARIES_RECORD_LEN, after * 3);
                (0x20, 0x80, r.next_u32(), 0x10)
            }
            _ => (0x40, 0x40, r.next_u32() | 1, 0x10),
        };
        let sock = PROTO_ARIES_SOCK;
        c.bus.write32(sock + PROTO_ARIES_RECV_READ, read);
        c.bus.write32(sock + PROTO_ARIES_RECV_FILL, filled);
        c.bus.write32(sock + PROTO_ARIES_RECV_PENDING, pending);
        c.g[T0][0] = len as u64;
    }

    /// (negative result, the buffer was rewound, a receive was sent).
    pub(crate) fn proto_aries_recv_arm(c: &mut Cpu, _seed: u32) -> (bool, bool, bool) {
        let sock = PROTO_ARIES_SOCK;
        let none = (c.g[V0][0] as i64) < 0;
        let rewound = c.bus.read32(sock + PROTO_ARIES_RECV_FILL) == 0
            && c.bus.read32(sock + PROTO_ARIES_RECV_READ) == 0;
        (none, rewound, proto_aries_sent(c, PROTO_ARIES_RECV_REQUEST))
    }

    #[test]
    fn proto_aries_recv_matches() {
        let setup = proto_aries_recv_fixture;
        // No destination, no server: pending 14 + 20 + 1 + 7 = 42; the short
        // record 14 + 68 + 8 = 90; a record (Peek 66, or 68 for an empty
        // payload) then 32 more unread, 35 pending, 133 a new receive.
        check_paths(
            ee::s14::f_003f5c38__generated,
            proto_aries_recv,
            192,
            &[42, 90, 98, 101, 199, 100, 103, 201],
            &setup,
        );
        require_arm_values(
            ee::s14::f_003f5c38__generated,
            192,
            &setup,
            &proto_aries_recv_arm,
            &[
                (true, false, false),
                (false, false, false),
                (false, true, false),
                (false, true, true),
                (false, false, true),
            ],
        );
    }

    // ----------------------------------------------------- ProtoAriesTick

    #[test]
    fn proto_aries_tick_matches() {
        check(
            ee::s14::f_003f5d88__generated,
            proto_aries_tick,
            8,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3f5d88, 0);
                c.pc = 0x003f_5d88;
                c.bus
                    .write32(DIRTY_RPC_STATE + DIRTY_RPC_CLOCK_COUNT, r.next_u32());
                let base = (r.next_u32() as u64) << 16 | r.next_u32() as u64;
                c.bus.write64(DIRTY_RPC_STATE + DIRTY_RPC_CLOCK_BASE, base);
            },
        );
    }

    // The IOP side the fixtures stand in for (libusbkb and the SIF loader).
    /// libusbkb's semaphore (-1 until `sceUsbKbInit`: every `sceUsbKb*`
    /// call then fails at once).
    pub(crate) const USB_KB_TEST_LIB_SEMA: u32 = 0x0045_3438;

    /// libusbkb's state: +0 connectable keyboards, +4 the per-keyboard
    /// settings (0x38 bytes each), +0x90 the pending operation.
    pub(crate) const USB_KB_TEST_LIB: u32 = 0x0052_9000;

    pub(crate) const USB_KB_TEST_LIB_LIMIT: u32 = 0x0;

    pub(crate) const USB_KB_TEST_LIB_SETTINGS: u32 = 0x4;

    pub(crate) const USB_KB_TEST_LIB_OP: u32 = 0x90;

    /// libusbkb's RPC client (+0 its packet, +4 its request id).
    pub(crate) const USB_KB_TEST_LIB_RPC: u32 = 0x0052_90a0;

    /// libusbkb's result callbacks by pending operation.
    pub(crate) const USB_KB_TEST_LIB_RESULT_FNS: u32 = 0x0045_3420;

    /// libkernel's SIF RPC packet pool: +0 the request counter, +4 the packets
    /// (0x40 bytes each, +0x10 flags with bit 0 in use, +0x18 request id), +8
    /// their number.
    pub(crate) const SIF_RPC_POOL: u32 = 0x0052_d680;

    pub(crate) const SIF_RPC_PACKET_FLAGS: u32 = 0x10;

    pub(crate) const SIF_RPC_PACKET_ID: u32 = 0x18;

    /// The SIF command word `_sceSifSendCmd` reads.
    pub(crate) const SIF_CMD_WORD: u32 = 0x0052_bce0;

    /// sceSifLoadFileInit's bind word (>= 0: bound).
    pub(crate) const USB_KB_TEST_LOADFILE_BOUND: u32 = 0x0045_5228;

    /// The words `libkernl_LoadfileNeedsRebind` compares: the loader's id,
    /// the reference id, and a pointer to a third.
    pub(crate) const USB_KB_TEST_REBIND_ID: u32 = 0x0052_f2a8;

    pub(crate) const USB_KB_TEST_REBIND_REF: u32 = 0x0045_5164;

    pub(crate) const USB_KB_TEST_REBIND_PTR: u32 = 0x0045_522c;

    // Fixture data.
    /// The keyboard data array.
    pub(crate) const USB_KB_TEST_DATA: u32 = DATA + 0x2000;

    /// libusbkb's settings array.
    pub(crate) const USB_KB_TEST_SETTINGS: u32 = DATA + 0x7400;

    /// The SIF pool's packets (four) and the RPC client's last packet.
    pub(crate) const USB_KB_TEST_PACKETS: u32 = DATA + 0x6_0000;

    pub(crate) const USB_KB_TEST_CLIENT_PACKET: u32 = DATA + 0x6_0400;

    /// A word no code writes (the ids, request and saved words start as it).
    pub(crate) const USB_KB_TEST_MARK: u32 = 0x5555_5555;

    /// The fake connect and hold callbacks.
    pub(crate) const USB_KB_TEST_CONNECT_CB: u32 = FAKE + 0x20;

    pub(crate) const USB_KB_TEST_HOLD_CB: u32 = FAKE + 0x30;

    /// Event hook of the plain tests whose real callees reach `EIntr` (it
    /// makes the next check due): nothing is delivered; past the cycle
    /// budget the test fails.
    pub(crate) fn usb_kb_quiet_event(c: &mut Cpu) -> bool {
        assert!(c.cycles < CYCLE_BUDGET, "runaway at pc {:#x}", c.pc);
        false
    }

    /// The test's IOP side (see [`usb_kb_iop_event`]).
    pub(crate) const USB_KB_TEST_IOP: u32 = DATA + 0x6_0800;

    /// The server the IOP side lets a bind find.
    pub(crate) const USB_KB_TEST_SERVER: u32 = DATA + 0x6_0600;

    /// `WaitSema`'s due-event check after its trap, and the return
    /// addresses (still in ra there) of the waits in `sceSifBindRpc` and in
    /// `sceSifCallRpc`.
    pub(crate) const USB_KB_TEST_WAIT_SEMA_RET: u32 = 0x0042_3de8;

    pub(crate) const USB_KB_TEST_BIND_WAIT_RA: u32 = 0x0042_6c20;

    pub(crate) const USB_KB_TEST_CALL_WAIT_RA: u32 = 0x0042_6ec8;

    /// The SIF loader's RPC client and its buffer (the request, then the
    /// IOP's reply: +0 the module id or result).
    pub(crate) const USB_KB_TEST_LOADER_CLIENT: u32 = 0x0052_f280;

    pub(crate) const USB_KB_TEST_LOADER_BUF: u32 = 0x0052_f080;

    /// An RPC client's server pointer (0 until the bind is answered).
    pub(crate) const USB_KB_TEST_CLIENT_SERVER: u32 = 0x24;

    /// `sceSifInitRpc`'s done flag (non-zero: it returns at once).
    pub(crate) const USB_KB_TEST_SIF_INIT_DONE: u32 = 0x0045_5180;

    /// libusbkb's receive buffer (the info reply: +0 keyboards served, +1
    /// connected, +2 the statuses) and its info result callback
    /// (`libusbkb_GetInfoCallback`, which copies the reply).
    pub(crate) const USB_KB_TEST_LIB_REPLY: u32 = 0x0052_9140;

    pub(crate) const USB_KB_TEST_GET_INFO_CB: u32 = 0x0040_c928;

    /// `UsbKb_Init`'s wait loop head (after a `UsbKb_SyncInfo` still
    /// pending).
    pub(crate) const USB_KB_TEST_INIT_SYNC_HEAD: u32 = 0x003f_62c8;

    /// Event hook standing in for the IOP (`next_event` 0: every check
    /// delivers; nothing else happens). Words at [`USB_KB_TEST_IOP`]: +0
    /// loader replies given, +4 how many to give, +8/+0xc the replies, each
    /// written into the loader's buffer as its `sceSifCallRpc` waits; +0x10
    /// deliveries at `UsbKb_Init`'s wait head before the info request
    /// completes (its packet's in-use bit clears), +0x14 those seen; +0x18
    /// the server a bind finds, written into its client as it waits.
    pub(crate) fn usb_kb_iop_event(c: &mut Cpu) -> bool {
        assert!(c.cycles < CYCLE_BUDGET, "runaway at pc {:#x}", c.pc);
        let iop = USB_KB_TEST_IOP;
        let waiting = c.pc == USB_KB_TEST_WAIT_SEMA_RET;
        let ra = c.g[RA][0] as u32;
        let client = c.g[S1][0] as u32;
        if waiting && ra == USB_KB_TEST_CALL_WAIT_RA && client == USB_KB_TEST_LOADER_CLIENT {
            let n = c.bus.read32(iop);
            if n < c.bus.read32(iop + 4) {
                let reply = c.bus.read32(iop + 8 + 4 * n);
                c.bus.write32(USB_KB_TEST_LOADER_BUF, reply);
                c.bus.write32(iop, n + 1);
            }
        } else if waiting && ra == USB_KB_TEST_BIND_WAIT_RA {
            let server = c.bus.read32(iop + 0x18);
            c.bus
                .write32(client.wrapping_add(USB_KB_TEST_CLIENT_SERVER), server);
        } else if c.pc == USB_KB_TEST_INIT_SYNC_HEAD {
            let n = c.bus.read32(iop + 0x14) + 1;
            c.bus.write32(iop + 0x14, n);
            if n == c.bus.read32(iop + 0x10) {
                let packet = c.bus.read32(USB_KB_TEST_LIB_RPC);
                let at = packet.wrapping_add(SIF_RPC_PACKET_FLAGS);
                let flags = c.bus.read32(at);
                c.bus.write32(at, flags & !1);
            }
        }
        false
    }

    /// The IOP side: the loader's `replies` (at most two), the deliveries
    /// before the info request completes (`rounds`), the server a bind
    /// finds (0: none); the hook on, every check due.
    pub(crate) fn usb_kb_iop_fixture(c: &mut Cpu, replies: &[u32], rounds: u32, server: u32) {
        assert!(replies.len() <= 2);
        let iop = USB_KB_TEST_IOP;
        c.bus.write32(iop, 0);
        c.bus.write32(iop + 4, replies.len() as u32);
        for (i, &w) in replies.iter().enumerate() {
            c.bus.write32(iop + 8 + 4 * i as u32, w);
        }
        c.bus.write32(iop + 0x10, rounds);
        c.bus.write32(iop + 0x14, 0);
        c.bus.write32(iop + 0x18, server);
        c.next_event = 0;
        c.hooks.event = usb_kb_iop_event;
    }

    /// Run the generated body for every seed (fakes never unwinding) and
    /// require that `arm`, read from the machine afterwards, takes every
    /// value in `want`: the fixture really reaches each arm, where the arms'
    /// cycle totals depend on fake callees.
    pub(crate) fn usb_kb_require_arms(
        generated: GuestFn,
        seeds: u32,
        want: &[u32],
        setup: &dyn Fn(&mut Cpu, u32),
        arm: &dyn Fn(&mut Cpu, u32) -> u32,
    ) {
        let no_unwind = |c: &mut Cpu, seed: u32| {
            setup(c, seed);
            c.bus.write32(LOG_UNWIND_AT, u32::MAX);
        };
        require_arm_values(generated, seeds, &no_unwind, arm, want);
    }

    /// Status bytes of eight keyboards from the base-4 digits of `pattern`:
    /// 0, 1, 2 or 0x80.
    pub(crate) fn usb_kb_status(pattern: u32) -> [u8; 8] {
        std::array::from_fn(|i| [0u8, 1, 2, 0x80][(pattern >> (2 * i) & 3) as usize])
    }

    /// The state block: junk in every word, then `max` keyboards, their
    /// status and last reported status bytes, `request`, the mark in the
    /// busy byte and the saved words, the data array (junk) and the fake
    /// callbacks.
    pub(crate) fn usb_kb_state_fixture(
        c: &mut Cpu,
        r: &mut Rng,
        max: u32,
        connected: [u8; 8],
        seen: [u8; 8],
        request: u32,
    ) {
        fill(c, r, USB_KB_STATE, 0x34);
        c.bus.write32(USB_KB_STATE + USB_KB_MAX, max);
        for i in 0..8u32 {
            c.bus
                .write8(USB_KB_STATE + USB_KB_CONNECTED + i, connected[i as usize]);
            c.bus
                .write8(USB_KB_STATE + USB_KB_SEEN + i, seen[i as usize]);
        }
        c.bus.write32(USB_KB_STATE + USB_KB_REQUEST, request);
        c.bus.write8(USB_KB_STATE + USB_KB_BUSY, 0x55);
        c.bus
            .write32(USB_KB_STATE + USB_KB_PREV_LED, USB_KB_TEST_MARK);
        c.bus
            .write32(USB_KB_STATE + USB_KB_PREV_MKEY, USB_KB_TEST_MARK);
        c.bus.write32(USB_KB_STATE + USB_KB_DATA, USB_KB_TEST_DATA);
        fill(c, r, USB_KB_TEST_DATA - 0x200, 0x80 + 0x8c * 10 / 4);
        c.bus
            .write32(USB_KB_STATE + USB_KB_CONNECT_CB, USB_KB_TEST_CONNECT_CB);
        c.bus
            .write32(USB_KB_STATE + USB_KB_HOLD_CB, USB_KB_TEST_HOLD_CB);
    }

    /// Keyboard `i` per `kinds[i]`: not connected (0), connected with its data
    /// not yet taken (1: fresh byte 1; 3: status 0x80, fresh 0xff), or
    /// connected and ready for a read (2: fresh 0).
    pub(crate) fn usb_kb_keyboards_fixture(c: &mut Cpu, kinds: [u32; 8]) {
        for (i, &kind) in kinds.iter().enumerate() {
            let (status, fresh) = [(0u8, 1u8), (1, 1), (2, 0), (0x80, 0xff)][kind as usize % 4];
            c.bus
                .write8(USB_KB_STATE + USB_KB_CONNECTED + i as u32, status);
            let entry = USB_KB_TEST_DATA + USB_KB_DATA_SIZE as u32 * i as u32;
            c.bus.write8(entry + USB_KB_DATA_FRESH, fresh);
        }
    }

    /// libusbkb: semaphore valid or -1, `limit` connectable keyboards, the
    /// settings array, nothing pending, fake result callbacks; the SIF pool
    /// with `packets` free packets (junk, in-use bit clear).
    pub(crate) fn usb_kb_lib_fixture(
        c: &mut Cpu,
        r: &mut Rng,
        sema_ok: bool,
        limit: u32,
        packets: u32,
    ) {
        c.bus
            .write32(USB_KB_TEST_LIB_SEMA, if sema_ok { 0x2a } else { u32::MAX });
        c.bus
            .write32(USB_KB_TEST_LIB + USB_KB_TEST_LIB_LIMIT, limit);
        c.bus.write32(
            USB_KB_TEST_LIB + USB_KB_TEST_LIB_SETTINGS,
            USB_KB_TEST_SETTINGS,
        );
        c.bus.write32(USB_KB_TEST_LIB + USB_KB_TEST_LIB_OP, 0);
        fill(c, r, USB_KB_TEST_SETTINGS - 0x100, 0x140);
        for k in 0..6u32 {
            c.bus
                .write32(USB_KB_TEST_LIB_RESULT_FNS + 4 * k, FAKE + 0x100 + 0x10 * k);
        }
        fill(c, r, SIF_CMD_WORD, 1);
        let counter = r.next_u32() | 0x10;
        c.bus.write32(SIF_RPC_POOL, counter);
        c.bus.write32(SIF_RPC_POOL + 4, USB_KB_TEST_PACKETS);
        c.bus.write32(SIF_RPC_POOL + 8, packets);
        fill(c, r, USB_KB_TEST_PACKETS, 0x40);
        for i in 0..4u32 {
            let at = USB_KB_TEST_PACKETS + i * 0x40 + SIF_RPC_PACKET_FLAGS;
            let flags = c.bus.read32(at);
            c.bus.write32(at, flags & !1);
        }
        usb_kb_rpc_client(c, r, false);
    }

    /// libusbkb's RPC client: its last request still running (`busy`: the
    /// packet's in-use bit set, same request id) or none.
    pub(crate) fn usb_kb_rpc_client(c: &mut Cpu, r: &mut Rng, busy: bool) {
        let packet = USB_KB_TEST_CLIENT_PACKET;
        fill(c, r, packet, 0x10);
        let id = r.next_u32();
        c.bus.write32(packet + SIF_RPC_PACKET_ID, id);
        let flags = c.bus.read32(packet + SIF_RPC_PACKET_FLAGS);
        c.bus.write32(packet + SIF_RPC_PACKET_FLAGS, flags | 1);
        c.bus
            .write32(USB_KB_TEST_LIB_RPC, if busy { packet } else { 0 });
        c.bus.write32(USB_KB_TEST_LIB_RPC + 4, id);
    }

    /// The SIF loader bound, and `libkernl_LoadfileNeedsRebind`'s compares
    /// all equal (no rebind) or all different (`rebind`: every load, stop and
    /// unload fails at once).
    pub(crate) fn usb_kb_loadfile_fixture(c: &mut Cpu, rebind: bool) {
        c.bus.write32(USB_KB_TEST_LOADFILE_BOUND, 0);
        c.bus
            .write32(USB_KB_TEST_REBIND_ID, if rebind { 0x3131_3131 } else { 0 });
        c.bus
            .write32(USB_KB_TEST_REBIND_REF, if rebind { 0x3232_3232 } else { 0 });
        c.bus.write32(USB_KB_TEST_REBIND_PTR, 0);
    }

    // -----------------------------------------------------------------------
    // UsbKb_LoadModules

    /// `seed % 6`: no directory (0), no keyboard arguments (1), `USBD.IRX`
    /// failing for want of an RPC packet (2) or because the loader must
    /// rebind (3), `USBKB.IRX` failing (one packet: 4), both loading (5).
    /// `seed / 6 % 2`: keyboard arguments or an empty string. The loads run
    /// the real SIF loader; its four syscalls per load are fakes (some
    /// unwinding). `seed / 12 % 3`: the module ids the IOP replies with:
    /// none (the buffer keeps the request: `USBD.IRX` 0, `USBKB.IRX` the
    /// argument size), `USBD.IRX` 0x10 and `USBKB.IRX` 0 (a valid id; no
    /// unwinds, so the load always gets that far), or `USBKB.IRX` failing
    /// on the IOP (-0x190).
    pub(crate) fn usb_kb_load_modules_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x3f_5da8, 8);
        let replies: &[u32] = match seed / 12 % 3 {
            0 => &[],
            1 => &[0x10, 0],
            _ => &[0x22, 0xffff_fe70], // -0x190
        };
        usb_kb_iop_fixture(c, replies, 0, 0);
        if seed / 12 % 3 == 1 {
            c.bus.write32(LOG_UNWIND_AT, u32::MAX);
        }
        elf_image(c);
        c.pc = 0x003f_5da8;
        let arm = seed % 6;
        let sp_now = c.g[SP][0] as u32;
        fill(c, &mut r, sp_now - 0x140, 0x50);
        fill(c, &mut r, USB_KB_STATE, 0x34);
        c.bus
            .write32(USB_KB_STATE + USB_KB_USBD_ID, USB_KB_TEST_MARK);
        c.bus
            .write32(USB_KB_STATE + USB_KB_USBKB_ID, USB_KB_TEST_MARK);
        let packets = [2, 2, 0, 2, 1, 2][arm as usize];
        usb_kb_lib_fixture(c, &mut r, true, 8, packets);
        usb_kb_loadfile_fixture(c, arm == 3);
        let dir = DATA + 0x100;
        put_cstr(c, dir, b"cdrom0:\\MODULES\\");
        let args = DATA + 0x200;
        let text: &[u8] = if seed / 6 % 2 == 0 { b"keybd=1" } else { b"" };
        put_cstr(c, args, text);
        c.g[A0][0] = if arm == 0 { 0 } else { sx(dir) };
        c.g[A1][0] = if arm == 1 { 0 } else { sx(args) };
    }

    /// The arm by the result and the two ids (the mark: never written); 5:
    /// both loaded with `USBKB.IRX`'s id 0 (still a success).
    pub(crate) fn usb_kb_load_modules_arm(c: &mut Cpu, _seed: u32) -> u32 {
        let v0 = c.g[V0][0] as i64;
        let usbd = c.bus.read32(USB_KB_STATE + USB_KB_USBD_ID);
        let usbkb = c.bus.read32(USB_KB_STATE + USB_KB_USBKB_ID);
        let written = |id: u32| id != USB_KB_TEST_MARK;
        match (v0, usbd as i32, usbkb as i32) {
            (0, -1, -1) => 0,
            (-1, _, _) if !written(usbd) && !written(usbkb) => 1,
            (-1, d, _) if d < 0 && !written(usbkb) => 2,
            (-1, d, k) if d >= 0 && written(usbd) && k < 0 => 3,
            (0, d, 0) if d >= 0 && written(usbd) => 5,
            (0, d, k) if d >= 0 && k >= 0 && written(usbd) && written(usbkb) => 4,
            _ => 99,
        }
    }

    #[test]
    fn usb_kb_load_modules_matches() {
        check(
            ee::s14::f_003f5da8__generated,
            usb_kb_load_modules,
            72,
            &usb_kb_load_modules_setup,
        );
        usb_kb_require_arms(
            ee::s14::f_003f5da8__generated,
            72,
            &[0, 1, 2, 3, 4, 5],
            &usb_kb_load_modules_setup,
            &usb_kb_load_modules_arm,
        );
    }

    // -----------------------------------------------------------------------
    // UsbKb_UnloadModule

    /// `seed % 4 == 0`: a negative module id (nothing to do); else id 1-3,
    /// stopped and unloaded by the real SIF loader with `seed / 4 % 3` free
    /// RPC packets (none: both fail; one: the stop works, the unload fails;
    /// two: both work), every call failing at once when `seed / 12 % 2` says
    /// the loader must rebind. Syscalls are fakes (some unwinding). For an id
    /// of 1-3, `seed / 24 % 3`: the IOP's results for the stop and the unload
    /// (see [`usb_kb_unload_module_replies`]).
    pub(crate) fn usb_kb_unload_module_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x3f_5e90, 8);
        let replies = usb_kb_unload_module_replies(seed);
        usb_kb_iop_fixture(c, replies, 0, 0);
        if !replies.is_empty() {
            c.bus.write32(LOG_UNWIND_AT, u32::MAX);
        }
        c.pc = 0x003f_5e90;
        usb_kb_lib_fixture(c, &mut r, true, 8, seed / 4 % 3);
        usb_kb_loadfile_fixture(c, seed / 12 % 2 == 1);
        c.g[A0][0] = if seed % 4 == 0 {
            sx(0u32.wrapping_sub(1 + seed / 24 % 3))
        } else {
            (seed % 4) as u64
        };
        c.g[A1][0] = r.next_u32() as u64 | 1 << 40;
    }

    /// The IOP's results for an id of 1-3 (`seed / 24 % 3`): none (the
    /// buffer keeps the request, the id: both work), the stop failing with
    /// -1 and the unload working, or the stop working and the unload
    /// failing with -1.
    pub(crate) fn usb_kb_unload_module_replies(seed: u32) -> &'static [u32] {
        if seed % 4 == 0 {
            return &[];
        }
        match seed / 24 % 3 {
            0 => &[],
            1 => &[u32::MAX, 5],
            _ => &[7, u32::MAX],
        }
    }

    /// Skipped (0), both done (1: fakes ran, result 0), failed (2); both
    /// answered by the IOP and failed: the stop with -1 (3: only the first
    /// move sets the result) or the unload with -1 (4).
    pub(crate) fn usb_kb_unload_module_arm(c: &mut Cpu, seed: u32) -> u32 {
        let answered = c.bus.read32(USB_KB_TEST_IOP) == 2;
        let replies = usb_kb_unload_module_replies(seed);
        if c.g[V0][0] != 0 && answered {
            3 + (replies[1] == u32::MAX) as u32
        } else if c.g[V0][0] != 0 {
            2
        } else if c.bus.read32(LOG_COUNT) == 0 {
            0
        } else {
            1
        }
    }

    #[test]
    fn usb_kb_unload_module_matches() {
        check(
            ee::s14::f_003f5e90__generated,
            usb_kb_unload_module,
            72,
            &usb_kb_unload_module_setup,
        );
        usb_kb_require_arms(
            ee::s14::f_003f5e90__generated,
            72,
            &[0, 1, 2, 3, 4],
            &usb_kb_unload_module_setup,
            &usb_kb_unload_module_arm,
        );
    }

    // -----------------------------------------------------------------------
    // UsbKb_RequestInfo

    /// `seed % 5`: 0, 1, 3 or 8 keyboards or a negative count (no copy);
    /// `seed / 5 % 2`: libusbkb's semaphore -1 (the request fails at once)
    /// or valid (it goes out over the real `sceSifCallRpc`, two fake
    /// syscalls, some unwinding).
    pub(crate) fn usb_kb_request_info_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x3f_5f08, 2);
        c.hooks.event = usb_kb_quiet_event;
        c.pc = 0x003f_5f08;
        let max = [0u32, 1, 3, 8, u32::MAX - 2][(seed % 5) as usize];
        let connected = usb_kb_status((seed / 10).wrapping_mul(0x9e37_79b9));
        usb_kb_state_fixture(c, &mut r, max, connected, [0x55; 8], USB_KB_TEST_MARK);
        usb_kb_lib_fixture(c, &mut r, seed / 5 % 2 == 1, 8, 2);
    }

    /// Bit 1: the request went out (request 1); bit 0: the statuses were
    /// copied (the first last-reported byte is no longer 0x55).
    pub(crate) fn usb_kb_request_info_arm(c: &mut Cpu, _seed: u32) -> u32 {
        let sent = c.bus.read32(USB_KB_STATE + USB_KB_REQUEST) == USB_KB_REQ_INFO as u32
            && c.bus.read8(USB_KB_STATE + USB_KB_BUSY) == 1;
        let copied = c.bus.read8(USB_KB_STATE + USB_KB_SEEN) != 0x55;
        (sent as u32) << 1 | copied as u32
    }

    #[test]
    fn usb_kb_request_info_matches() {
        // Semaphore -1: 29 cycles plus 2 + 7 per keyboard copied.
        check_paths(
            ee::s14::f_003f5f08__generated,
            usb_kb_request_info,
            60,
            &[29, 38, 52, 87],
            &usb_kb_request_info_setup,
        );
        usb_kb_require_arms(
            ee::s14::f_003f5f08__generated,
            60,
            &[0, 1, 2, 3],
            &usb_kb_request_info_setup,
            &usb_kb_request_info_arm,
        );
    }

    /// Eight keyboards: the copy loop's head 0x3f5f30 meets the due event
    /// (`schedule`: `seed % 3` none, an unwinding VBlank, a plain delivery).
    #[test]
    fn usb_kb_request_info_events() {
        driven!(gen, ee::s14::f_003f5f08__generated);
        driven!(hand, usb_kb_request_info);
        let setup = |c: &mut Cpu, seed: u32| {
            let mut r = machine(c, seed, 0x3f_5f09, 0);
            let connected = usb_kb_status((seed / 3).wrapping_mul(0x9e37_79b9));
            usb_kb_state_fixture(c, &mut r, 8, connected, [0x55; 8], USB_KB_TEST_MARK);
            usb_kb_lib_fixture(c, &mut r, false, 8, 0);
            schedule(c, &mut r, seed, 0x003f_5f08)
        };
        check_events(gen, hand, 60, &setup);
        // Both sides must deliver at the same cycles (equal `next_event`):
        // a delivery the port's own loop check misses or adds shows here.
        same_event_cycles(gen, hand, 60, &setup, &mut |_, _, _, _, _| {});
    }

    // -----------------------------------------------------------------------
    // UsbKb_SyncInfo

    /// `seed % 3`: `sceUsbKbSync` fails (semaphore -1), says the info
    /// request still runs (1), or that it is done (0: the walk over the
    /// keyboards). `seed / 3 % 4`: 0, 2 or 8 keyboards or a negative count.
    /// `seed / 12` picks each keyboard's status and last reported status
    /// (0, 1, 2, 0x80), so some differ and call the connect callback (a
    /// fake, some unwinding).
    pub(crate) fn usb_kb_sync_info_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x3f_5f88, 4);
        c.pc = 0x003f_5f88;
        let max = [0u32, 2, 8, u32::MAX][(seed / 3 % 4) as usize];
        let digit = seed / 12;
        let connected = usb_kb_status(digit.wrapping_mul(0x9e37_79b9));
        let seen = usb_kb_status(digit.wrapping_mul(0x85eb_ca6b));
        usb_kb_state_fixture(c, &mut r, max, connected, seen, USB_KB_TEST_MARK);
        usb_kb_lib_fixture(c, &mut r, seed % 3 != 0, 8, 0);
        usb_kb_rpc_client(c, &mut r, seed % 3 == 1);
    }

    /// Sync failed (0), still running (1), done with no change (2) or with
    /// the connect callback called (3).
    pub(crate) fn usb_kb_sync_info_arm(c: &mut Cpu, _seed: u32) -> u32 {
        match c.g[V0][0] {
            u64::MAX => 0,
            1 => 1,
            0 if c.bus.read32(LOG_COUNT) == 0 => 2,
            0 => 3,
            _ => 99,
        }
    }

    #[test]
    fn usb_kb_sync_info_matches() {
        check(
            ee::s14::f_003f5f88__generated,
            usb_kb_sync_info,
            144,
            &usb_kb_sync_info_setup,
        );
        usb_kb_require_arms(
            ee::s14::f_003f5f88__generated,
            144,
            &[0, 1, 2, 3],
            &usb_kb_sync_info_setup,
            &usb_kb_sync_info_arm,
        );
    }

    /// The sync done, eight keyboards: the walk's head 0x3f5fe0 meets the
    /// due event (first back edge already past it: the sync takes longer
    /// than the scheduled delay).
    #[test]
    fn usb_kb_sync_info_events() {
        driven!(gen, ee::s14::f_003f5f88__generated);
        driven!(hand, usb_kb_sync_info);
        let setup = |c: &mut Cpu, seed: u32| {
            let mut r = machine(c, seed, 0x3f_5f89, 0);
            let digit = seed / 3;
            let connected = usb_kb_status(digit.wrapping_mul(0x9e37_79b9));
            let seen = usb_kb_status(digit.wrapping_mul(0x85eb_ca6b));
            usb_kb_state_fixture(c, &mut r, 8, connected, seen, USB_KB_TEST_MARK);
            usb_kb_lib_fixture(c, &mut r, true, 8, 0);
            schedule(c, &mut r, seed, 0x003f_5f88)
        };
        check_events(gen, hand, 60, &setup);
        // Both sides must deliver at the same cycles (equal `next_event`):
        // a delivery the port's own loop check misses or adds shows here.
        same_event_cycles(gen, hand, 60, &setup, &mut |_, _, _, _, _| {});
    }

    // -----------------------------------------------------------------------
    // UsbKb_FindNextReady

    /// `seed % 6`: 0, 1, 2, 5 or 8 keyboards or a negative count;
    /// `seed / 6 % 6`: the first keyboard to look at (0, 1, the last, one
    /// past it, far past it (both wrap), -1); `seed / 36 % 4`: keyboard 0's
    /// kind (see `usb_kb_keyboards_fixture`), the others' from that digit
    /// scrambled.
    pub(crate) fn usb_kb_find_next_ready_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x3f_6040, 0);
        c.pc = 0x003f_6040;
        let max = [0u32, 1, 2, 5, 8, u32::MAX][(seed % 6) as usize];
        usb_kb_state_fixture(c, &mut r, max, [0; 8], [0; 8], USB_KB_TEST_MARK);
        let digit = seed / 36;
        let others = digit.wrapping_mul(0x2545_f491);
        let kinds: [u32; 8] = std::array::from_fn(|i| {
            if i == 0 {
                digit % 4
            } else {
                others >> (2 * i) & 3
            }
        });
        usb_kb_keyboards_fixture(c, kinds);
        let first = [
            0,
            1,
            max.wrapping_sub(1),
            max,
            max.wrapping_add(5),
            u32::MAX,
        ][(seed / 6 % 6) as usize];
        c.g[A0][0] = sx(first);
    }

    #[test]
    fn usb_kb_find_next_ready_matches() {
        // No keyboards (8); one keyboard: not connected (22), ready (23),
        // data not taken (26).
        check_paths_lohi(
            ee::s14::f_003f6040__generated,
            usb_kb_find_next_ready,
            216,
            &[8, 22, 23, 26],
            &usb_kb_find_next_ready_setup,
        );
    }

    /// Eight keyboards, none or only the seventh ready: the search's head
    /// 0x3f6068 meets the due event.
    #[test]
    fn usb_kb_find_next_ready_events() {
        driven!(gen, ee::s14::f_003f6040__generated);
        driven!(hand, usb_kb_find_next_ready);
        let setup = |c: &mut Cpu, seed: u32| {
            let mut r = machine(c, seed, 0x3f_6041, 0);
            usb_kb_state_fixture(c, &mut r, 8, [0; 8], [0; 8], USB_KB_TEST_MARK);
            let kinds = if seed / 3 % 2 == 0 {
                [0, 1, 0, 3, 1, 0, 1, 0]
            } else {
                [1, 0, 3, 1, 0, 1, 2, 0]
            };
            usb_kb_keyboards_fixture(c, kinds);
            c.g[A0][0] = (seed / 6 % 3) as u64;
            schedule(c, &mut r, seed, 0x003f_6040)
        };
        check_events(gen, hand, 60, &setup);
        // Both sides must deliver at the same cycles (equal `next_event`):
        // a delivery the port's own loop check misses or adds shows here.
        same_event_cycles(gen, hand, 60, &setup, &mut |_, _, _, _, _| {});
    }

    // -----------------------------------------------------------------------
    // UsbKb_RequestRead

    /// `seed % 3`: no keyboard ready (0); four ready keyboards whose read
    /// fails (1: semaphore -1 or, `seed / 3 % 2`, the keyboard past
    /// libusbkb's limit) or goes out over the real `sceSifCallRpc` (2: two
    /// fake syscalls, some unwinding). `seed / 6 % 4`: the last keyboard
    /// read, so the search starts after it (and wraps).
    pub(crate) fn usb_kb_request_read_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x3f_60b8, 2);
        c.hooks.event = usb_kb_quiet_event;
        c.pc = 0x003f_60b8;
        let arm = seed % 3;
        usb_kb_state_fixture(c, &mut r, 4, [0; 8], [0; 8], USB_KB_TEST_MARK);
        let kind = if arm == 0 { 1 } else { 2 };
        usb_kb_keyboards_fixture(c, [kind, kind, kind, kind, 0, 0, 0, 0]);
        c.bus.write32(USB_KB_STATE + USB_KB_READ_PORT, seed / 6 % 4);
        let (sema_ok, limit) = match arm {
            1 if seed / 3 % 2 == 0 => (false, 8),
            1 => (true, 0),
            _ => (true, 8),
        };
        usb_kb_lib_fixture(c, &mut r, sema_ok, limit, 2);
    }

    /// No read (0: the saved LED word still the mark), the read failed (1),
    /// the read went out (2: request 2).
    pub(crate) fn usb_kb_request_read_arm(c: &mut Cpu, _seed: u32) -> u32 {
        if c.bus.read32(USB_KB_STATE + USB_KB_PREV_LED) == USB_KB_TEST_MARK {
            0
        } else if c.bus.read32(USB_KB_STATE + USB_KB_REQUEST) == USB_KB_REQ_READ as u32 {
            2
        } else {
            1
        }
    }

    #[test]
    fn usb_kb_request_read_matches() {
        check_lohi(
            ee::s14::f_003f60b8__generated,
            usb_kb_request_read,
            48,
            &usb_kb_request_read_setup,
        );
        usb_kb_require_arms(
            ee::s14::f_003f60b8__generated,
            48,
            &[0, 1, 2],
            &usb_kb_request_read_setup,
            &usb_kb_request_read_arm,
        );
    }

    // -----------------------------------------------------------------------
    // UsbKb_SyncRead

    /// `seed % 2`: `sceUsbKbSync` fails (semaphore -1) or completes (two
    /// fake syscalls, some unwinding; `seed / 2 % 2`: with libusbkb's read
    /// result pending, its result callback, a fake, runs too). Then
    /// `seed / 4 % 4`: the read keyboard's data brought keys, a new modifier
    /// word, a new LED word (key count negative), or nothing new;
    /// `seed / 16 % 3`: which keyboard was read.
    pub(crate) fn usb_kb_sync_read_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x3f_6138, 3);
        c.hooks.event = usb_kb_quiet_event;
        c.pc = 0x003f_6138;
        usb_kb_state_fixture(c, &mut r, 4, [1; 8], [1; 8], USB_KB_TEST_MARK);
        usb_kb_lib_fixture(c, &mut r, seed % 2 == 1, 8, 0);
        if seed / 2 % 2 == 1 {
            c.bus.write32(USB_KB_TEST_LIB + USB_KB_TEST_LIB_OP, 2);
        }
        let port = seed / 16 % 3;
        c.bus.write32(USB_KB_STATE + USB_KB_READ_PORT, port);
        let entry = USB_KB_TEST_DATA + USB_KB_DATA_SIZE as u32 * port;
        c.bus.write8(entry + USB_KB_DATA_FRESH, 0x55);
        let led = r.next_u32();
        let mkey = r.next_u32();
        c.bus.write32(USB_KB_STATE + USB_KB_PREV_LED, led);
        c.bus.write32(USB_KB_STATE + USB_KB_PREV_MKEY, mkey);
        let (len, new_mkey, new_led) = match seed / 4 % 4 {
            0 => (1 + seed / 48 % 5, false, false),
            1 => (0, true, false),
            2 => (u32::MAX, false, true),
            _ => (0, false, false),
        };
        c.bus.write32(entry + USB_KB_DATA_LEN, len);
        c.bus.write32(
            entry + USB_KB_DATA_MKEY,
            if new_mkey { mkey ^ 0x22 } else { mkey },
        );
        c.bus.write32(
            entry + USB_KB_DATA_LED,
            if new_led { led ^ 0x1 } else { led },
        );
    }

    /// Sync failed (0); marked fresh for keys (1), the modifier word (2),
    /// the LED word (3); nothing new (4).
    pub(crate) fn usb_kb_sync_read_arm(c: &mut Cpu, _seed: u32) -> u32 {
        if c.g[V0][0] != 0 {
            return 0;
        }
        let port = c.bus.read32(USB_KB_STATE + USB_KB_READ_PORT);
        let entry = USB_KB_TEST_DATA + USB_KB_DATA_SIZE as u32 * port;
        let fresh = c.bus.read8(entry + USB_KB_DATA_FRESH);
        let len = c.bus.read32(entry + USB_KB_DATA_LEN) as i32;
        let mkey_new =
            c.bus.read32(entry + USB_KB_DATA_MKEY) != c.bus.read32(USB_KB_STATE + USB_KB_PREV_MKEY);
        let led_new =
            c.bus.read32(entry + USB_KB_DATA_LED) != c.bus.read32(USB_KB_STATE + USB_KB_PREV_LED);
        match (fresh, len > 0, mkey_new, led_new) {
            (1, true, _, _) => 1,
            (1, false, true, _) => 2,
            (1, false, false, true) => 3,
            (0x55, false, false, false) => 4,
            _ => 99,
        }
    }

    #[test]
    fn usb_kb_sync_read_matches() {
        check_lohi(
            ee::s14::f_003f6138__generated,
            usb_kb_sync_read,
            96,
            &usb_kb_sync_read_setup,
        );
        usb_kb_require_arms(
            ee::s14::f_003f6138__generated,
            96,
            &[0, 1, 2, 3, 4],
            &usb_kb_sync_read_setup,
            &usb_kb_sync_read_arm,
        );
    }

    // -----------------------------------------------------------------------
    // UsbKb_Init

    /// The keyboards the driver reports serving: 0, 128 or 255 for the
    /// failing driver (`seed % 8 == 5`), else 1, 3, 8, 9 or 0x10 (both
    /// clamped to 8) or 2 (`seed / 16 % 6`).
    pub(crate) fn usb_kb_init_count(seed: u32) -> u32 {
        if seed % 8 == 5 {
            [0, 0x80, 0xff][(seed / 16 % 3) as usize]
        } else {
            [1, 3, 8, 9, 0x10, 2][(seed / 16 % 6) as usize]
        }
    }

    /// `seed % 8`: `UsbKb_LoadModules` failing (0-3: no keyboard arguments,
    /// the loader must rebind, no RPC packet for `USBD.IRX`, none for
    /// `USBKB.IRX`); both modules loaded and `sceUsbKbInit` failing (4: its
    /// info sync gives a non-zero result, the result callback being a fake;
    /// 5: the driver reports 0, 128 or 255 keyboards); the driver running
    /// (6, 7). `sceUsbKbInit` and all after it run for real: the IOP side
    /// ([`usb_kb_iop_event`]) answers the bind, the info reply is in the
    /// receive buffer (`usb_kb_init_count` keyboards), and the info request
    /// `UsbKb_Init` waits for completes after 1-3 rounds (`seed % 8 - 5 +
    /// seed / 48 % 2`). `seed / 8 % 2`: callbacks given (the reply's
    /// statuses random: the fake connect callback runs for every changed
    /// one) or null (the defaults installed; statuses 0, so
    /// `UsbKb_DefaultConnectCb` is never called). Syscalls are fakes (some
    /// unwinding).
    pub(crate) fn usb_kb_init_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x3f_61d0, 40);
        elf_image(c);
        c.pc = 0x003f_61d0;
        let arm = seed % 8;
        let rounds = if arm >= 6 { arm - 5 + seed / 48 % 2 } else { 0 };
        usb_kb_iop_fixture(c, &[], rounds, USB_KB_TEST_SERVER);
        let sp_now = c.g[SP][0] as u32;
        fill(c, &mut r, sp_now - 0x300, 0x80);
        usb_kb_state_fixture(c, &mut r, 3, [1; 8], [1; 8], USB_KB_TEST_MARK);
        // Six packets once the modules load: two loads, the bind and two
        // info requests.
        let packets = [2, 2, 0, 1, 6, 6, 6, 6][arm as usize];
        usb_kb_lib_fixture(c, &mut r, true, 8, packets);
        fill(c, &mut r, USB_KB_TEST_PACKETS + 0x100, 0x20);
        for i in 4..6u32 {
            let at = USB_KB_TEST_PACKETS + i * 0x40 + SIF_RPC_PACKET_FLAGS;
            let flags = c.bus.read32(at);
            c.bus.write32(at, flags & !1);
        }
        usb_kb_loadfile_fixture(c, arm == 1);
        c.bus.write32(USB_KB_TEST_SIF_INIT_DONE, 1);
        c.bus.write32(
            USB_KB_TEST_LIB_RPC + USB_KB_TEST_CLIENT_SERVER,
            USB_KB_TEST_MARK,
        );
        if arm != 4 {
            // Operation 1 (info): the real callback copies the reply.
            c.bus
                .write32(USB_KB_TEST_LIB_RESULT_FNS + 4, USB_KB_TEST_GET_INFO_CB);
        }
        let given = seed / 8 % 2 == 0;
        fill(c, &mut r, USB_KB_TEST_LIB_REPLY, 0x24);
        c.bus
            .write8(USB_KB_TEST_LIB_REPLY, usb_kb_init_count(seed) as u8);
        let status = if given {
            usb_kb_status(r.next_u32())
        } else {
            [0; 8]
        };
        for (i, &s) in status.iter().enumerate() {
            c.bus.write8(USB_KB_TEST_LIB_REPLY + 2 + i as u32, s);
        }
        let dir = DATA + 0x100;
        put_cstr(c, dir, b"host0:");
        let args = DATA + 0x200;
        put_cstr(c, args, b"kb");
        c.g[A0][0] = sx(dir);
        c.g[A1][0] = if arm == 0 { 0 } else { sx(args) };
        c.g[A2][0] = sx(USB_KB_TEST_DATA);
        c.g[A3][0] = if given { sx(USB_KB_TEST_CONNECT_CB) } else { 0 };
        c.g[T0][0] = if given { sx(USB_KB_TEST_HOLD_CB) } else { 0 };
    }

    /// `UsbKb_LoadModules` failed (0: the bind never ran), the driver failed
    /// (1: bound, then -1), or the driver runs (2 + null callbacks + 2 *
    /// clamped): the keyboard count kept (at most 8), the callbacks given
    /// or the defaults, the info request done after exactly its rounds;
    /// 99 for anything else.
    pub(crate) fn usb_kb_init_arm(c: &mut Cpu, seed: u32) -> u32 {
        let server = c
            .bus
            .read32(USB_KB_TEST_LIB_RPC + USB_KB_TEST_CLIENT_SERVER);
        if c.g[V0][0] == u64::MAX {
            return match server {
                USB_KB_TEST_MARK => 0,
                USB_KB_TEST_SERVER => 1,
                _ => 99,
            };
        }
        let rounds = c.bus.read32(USB_KB_TEST_IOP + 0x10);
        let seen = c.bus.read32(USB_KB_TEST_IOP + 0x14);
        let max = c.bus.read32(USB_KB_STATE + USB_KB_MAX);
        let request = c.bus.read32(USB_KB_STATE + USB_KB_REQUEST);
        let callbacks = (
            c.bus.read32(USB_KB_STATE + USB_KB_CONNECT_CB),
            c.bus.read32(USB_KB_STATE + USB_KB_HOLD_CB),
        );
        let defaults = callbacks == (USB_KB_DEFAULT_CONNECT_CB.addr, USB_KB_NULL_CALLBACK);
        let given = callbacks == (USB_KB_TEST_CONNECT_CB, USB_KB_TEST_HOLD_CB);
        let count = usb_kb_init_count(seed);
        let ok = c.g[V0][0] == 0 && seen == rounds && request == 0 && max == count.min(8);
        if !ok || !(defaults || given) {
            return 99;
        }
        2 + defaults as u32 + 2 * (count > 8) as u32
    }

    #[test]
    fn usb_kb_init_matches() {
        check(
            ee::s14::f_003f61d0__generated,
            usb_kb_init,
            96,
            &usb_kb_init_setup,
        );
        usb_kb_require_arms(
            ee::s14::f_003f61d0__generated,
            96,
            &[0, 1, 2, 3, 4, 5],
            &usb_kb_init_setup,
            &usb_kb_init_arm,
        );
    }

    // -----------------------------------------------------------------------
    // UsbKb_Shutdown

    /// `sceUsbKbEnd` runs for real (its syscalls are fakes, some unwinding);
    /// `seed % 2`: libusbkb's semaphore valid (deleted) or -1. Both modules
    /// unload for real: `seed / 2 % 3` and `seed / 6 % 3` the `USBKB.IRX` and
    /// `USBD.IRX` ids (negative: skipped), `seed / 18 % 3` the loader (must
    /// rebind: every call fails; one free packet; four).
    pub(crate) fn usb_kb_shutdown_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x3f_62f8, 12);
        c.hooks.event = usb_kb_quiet_event;
        c.pc = 0x003f_62f8;
        usb_kb_state_fixture(c, &mut r, 3, [1; 8], [1; 8], USB_KB_TEST_MARK);
        let id = |d: u32| [u32::MAX, 3, 5][d as usize];
        c.bus
            .write32(USB_KB_STATE + USB_KB_USBKB_ID, id(seed / 2 % 3));
        c.bus
            .write32(USB_KB_STATE + USB_KB_USBD_ID, id(seed / 6 % 3));
        let loader = seed / 18 % 3;
        usb_kb_lib_fixture(c, &mut r, seed % 2 == 1, 8, [0, 1, 4][loader as usize]);
        usb_kb_loadfile_fixture(c, loader == 0);
    }

    #[test]
    fn usb_kb_shutdown_matches() {
        check(
            ee::s14::f_003f62f8__generated,
            usb_kb_shutdown,
            54,
            &usb_kb_shutdown_setup,
        );
    }

    // -----------------------------------------------------------------------
    // UsbKb_GetMaxKeyboards

    #[test]
    fn usb_kb_get_max_keyboards_matches() {
        check(
            ee::s14::f_003f6348__generated,
            usb_kb_get_max_keyboards,
            30,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3f_6348, 0);
                c.pc = 0x003f_6348;
                fill(c, &mut r, USB_KB_STATE, 0x34);
            },
        );
    }

    // -----------------------------------------------------------------------
    // UsbKb_DefaultConnectCb

    /// `seed % 4 == 0`: the keyboard went away (nothing to do); else (status
    /// 1, 2 or only upper bits) its settings go out: `seed / 4 % 3` every
    /// `sceUsbKb*` call fails (semaphore -1, or the keyboard past libusbkb's
    /// limit) or the requests go out over the real `sceSifCallRpc`, each
    /// followed by a real `sceUsbKbSync` (fake syscalls and result callbacks,
    /// some unwinding). `seed / 12 % 4`: the keyboard.
    pub(crate) fn usb_kb_default_connect_cb_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x3f_6358, 12);
        c.hooks.event = usb_kb_quiet_event;
        c.pc = 0x003f_6358;
        let keyboard = seed / 12 % 4;
        let calls = seed / 4 % 3;
        let limit = if calls == 1 { keyboard } else { 8 };
        usb_kb_lib_fixture(c, &mut r, calls != 0, limit, 4);
        c.g[A0][0] = keyboard as u64;
        c.g[A1][0] = [0u64, 1, 2, 1 << 33][(seed % 4) as usize];
    }

    /// Nothing done (0: the repeat setting untouched), every call failed
    /// (1: no fakes), the requests went out (2).
    pub(crate) fn usb_kb_default_connect_cb_arm(c: &mut Cpu, seed: u32) -> u32 {
        let keyboard = seed / 12 % 4;
        let repeat_at = USB_KB_TEST_SETTINGS + 0x38 * keyboard + 0x18;
        if c.bus.read32(repeat_at) != USB_KB_REPEAT_START as u32 {
            0
        } else if c.bus.read32(LOG_COUNT) == 0 {
            1
        } else {
            2
        }
    }

    #[test]
    fn usb_kb_default_connect_cb_matches() {
        check(
            ee::s14::f_003f6358__generated,
            usb_kb_default_connect_cb,
            48,
            &usb_kb_default_connect_cb_setup,
        );
        usb_kb_require_arms(
            ee::s14::f_003f6358__generated,
            48,
            &[0, 1, 2],
            &usb_kb_default_connect_cb_setup,
            &usb_kb_default_connect_cb_arm,
        );
    }

    // -----------------------------------------------------------------------
    // UsbKb_Update

    /// The pending requests `seed % 5` picks.
    pub(crate) const USB_KB_TEST_UPDATE_REQUESTS: [u32; 5] = [0, 1, 2, 3, 5];

    /// `UsbKb_Update` from each pending request (`seed % 5`: none, info,
    /// read, held, an unknown value). `seed / 5 % 2`: libusbkb's semaphore
    /// -1 (every library call fails at once) or valid (the syncs complete;
    /// new requests go out over the real `sceSifCallRpc`, fake syscalls,
    /// some unwinding). The hold callback is `UsbKb_IsConnected`, so
    /// keyboards 1 (`seed / 10 % 2`) and 0 (`seed / 20 % 2`) connected are
    /// its answers to "release" and "hold", or (`seed / 80 % 2`) a fake
    /// (always yes). `seed / 40 % 2`: the poll count passes the period (an
    /// info request) or not (a read; from seed 160 on it reaches the period
    /// exactly, still a read: see [`usb_kb_update_count`]).
    pub(crate) fn usb_kb_update_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x3f_63f0, 6);
        c.hooks.event = usb_kb_quiet_event;
        c.pc = 0x003f_63f0;
        let request = USB_KB_TEST_UPDATE_REQUESTS[(seed % 5) as usize];
        let mut connected = [0u8; 8];
        connected[1] = (seed / 10 % 2) as u8;
        connected[0] = (seed / 20 % 2) as u8 * 2;
        usb_kb_state_fixture(c, &mut r, 2, connected, [1, 1, 0, 0, 0, 0, 0, 0], request);
        let hold = if seed / 80 % 2 == 0 {
            0x003f_6520
        } else {
            USB_KB_TEST_HOLD_CB
        };
        c.bus.write32(USB_KB_STATE + USB_KB_HOLD_CB, hold);
        c.bus.write32(USB_KB_STATE + USB_KB_POLL_PERIOD, 60);
        let count = usb_kb_update_count(seed);
        c.bus.write32(USB_KB_STATE + USB_KB_POLL_COUNT, count);
        c.bus.write32(USB_KB_STATE + USB_KB_READ_PORT, 1);
        for i in 0..2u32 {
            let entry = USB_KB_TEST_DATA + USB_KB_DATA_SIZE as u32 * i;
            c.bus.write8(entry + USB_KB_DATA_FRESH, 0);
        }
        usb_kb_lib_fixture(c, &mut r, seed / 5 % 2 == 1, 8, 4);
    }

    /// The poll count before the frame (period 60): 60 (this frame passes
    /// it: an info request), 7, or 59 (this frame reaches 60, which is not
    /// past the period: a read).
    pub(crate) fn usb_kb_update_count(seed: u32) -> u32 {
        if seed / 40 % 2 == 0 {
            60
        } else if seed < 160 {
            7
        } else {
            59
        }
    }

    /// `request * 100 + poll * 10 + held`: the starting request; the poll
    /// count untouched (0), counted for a read (1) or reset for an info
    /// request (2); the library held afterwards (request 3).
    pub(crate) fn usb_kb_update_arm(c: &mut Cpu, seed: u32) -> u32 {
        let request = USB_KB_TEST_UPDATE_REQUESTS[(seed % 5) as usize];
        let start = usb_kb_update_count(seed);
        let count = c.bus.read32(USB_KB_STATE + USB_KB_POLL_COUNT);
        let poll = match count {
            0 => 2,
            n if n == start + 1 => 1,
            n if n == start => 0,
            _ => 9,
        };
        let held = c.bus.read32(USB_KB_STATE + USB_KB_REQUEST) == USB_KB_REQ_HELD as u32;
        let edge = (start == 59) as u32 * 1000;
        edge + request * 100 + poll * 10 + held as u32
    }

    #[test]
    fn usb_kb_update_matches() {
        check(
            ee::s14::f_003f63f0__generated,
            usb_kb_update,
            240,
            &usb_kb_update_setup,
        );
        usb_kb_require_arms(
            ee::s14::f_003f63f0__generated,
            240,
            &[
                1, 10, 20, // idle: held, read, info
                100, 101, 110, 120, // info: still running, then idle
                200, 201, 210, 220, // read: failed, then idle
                301, 310, 320,  // held: stays (or held again), released
                500,  // unknown request
                1010, // idle, the count reaching the period: a read
            ],
            &usb_kb_update_setup,
            &usb_kb_update_arm,
        );
    }

    // -----------------------------------------------------------------------
    // UsbKb_GetData

    #[test]
    fn usb_kb_get_data_matches() {
        // Fresh (13) or not (12).
        check_paths_lohi(
            ee::s14::f_003f64e0__generated,
            usb_kb_get_data,
            40,
            &[12, 13],
            &|c, seed| {
                let mut r = machine(c, seed, 0x3f_64e0, 0);
                c.pc = 0x003f_64e0;
                usb_kb_state_fixture(c, &mut r, 8, [1; 8], [1; 8], USB_KB_TEST_MARK);
                let keyboard = [0u32, 1, 7, 8, u32::MAX][(seed % 5) as usize];
                let entry =
                    USB_KB_TEST_DATA.wrapping_add((USB_KB_DATA_SIZE as u32).wrapping_mul(keyboard));
                let fresh = [0u8, 1, 2, 0xff][(seed / 5 % 4) as usize];
                c.bus.write8(entry + USB_KB_DATA_FRESH, fresh);
                c.g[A0][0] = sx(keyboard);
            },
        );
    }

    // -----------------------------------------------------------------------
    // UsbKb_IsConnected

    #[test]
    fn usb_kb_is_connected_matches() {
        check(
            ee::s14::f_003f6520__generated,
            usb_kb_is_connected,
            40,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3f_6520, 0);
                c.pc = 0x003f_6520;
                let connected = usb_kb_status((seed / 10).wrapping_mul(0x9e37_79b9));
                usb_kb_state_fixture(c, &mut r, 8, connected, [0; 8], USB_KB_TEST_MARK);
                c.g[A0][0] = [0u64, 1, 5, 7, 0xffff_ffff_ffff_fff0][(seed % 5) as usize]
                    | ((seed / 5 % 2) as u64) << 36;
            },
        );
    }

    // -----------------------------------------------------------------------
    // UsbKb_AnyConnected

    /// `seed % 4`: 0, 1, 3 or 8 keyboards; `seed / 4 % 5`: none connected,
    /// or the first connected one is 0, 1, 2 or 5 (status `seed / 20 % 3`:
    /// 1, 2 or 0x80). `UsbKb_Update` finds an unknown request (no calls).
    pub(crate) fn usb_kb_any_connected_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x3f_6538, 0);
        c.pc = 0x003f_6538;
        let max = [0u32, 1, 3, 8][(seed % 4) as usize];
        let mut connected = [0u8; 8];
        if let Some(k) = [None, Some(0), Some(1), Some(2), Some(5)][(seed / 4 % 5) as usize] {
            connected[k] = [1u8, 2, 0x80][(seed / 20 % 3) as usize];
        }
        usb_kb_state_fixture(c, &mut r, max, connected, [0; 8], 5);
    }

    #[test]
    fn usb_kb_any_connected_matches() {
        // No keyboards (47); keyboard 0 connected of 1 (59); keyboard 2 of 3
        // (95); none of 3 (101).
        check_paths(
            ee::s14::f_003f6538__generated,
            usb_kb_any_connected,
            60,
            &[47, 59, 95, 101],
            &usb_kb_any_connected_setup,
        );
    }

    /// Eight keyboards, none or only the eighth connected: the loop head
    /// 0x3f6560 meets the due event (an unwinding VBlank returns there, and
    /// the function is re-entered at it).
    #[test]
    fn usb_kb_any_connected_events() {
        driven!(gen, ee::s14::f_003f6538__generated);
        driven!(hand, usb_kb_any_connected);
        check_events(gen, hand, 60, &|c, seed| {
            let mut r = machine(c, seed, 0x3f_6539, 0);
            let mut connected = [0u8; 8];
            if seed / 3 % 2 == 1 {
                connected[7] = 1;
            }
            usb_kb_state_fixture(c, &mut r, 8, connected, [0; 8], 5);
            schedule(c, &mut r, seed, 0x003f_6538)
        });
    }

    // -----------------------------------------------------------------------
    // Keys

    #[test]
    fn usb_kb_get_keypad_entry_matches() {
        // Usages 0x2e-0x35 (9), 0x87-0x8b (12), any other (11).
        const USAGES: [u64; 14] = [
            0x00,
            0x04,
            0x2d,
            0x2e,
            0x31,
            0x35,
            0x36,
            0x86,
            0x87,
            0x89,
            0x8b,
            0x8c,
            0xffff,
            0x8000_002e,
        ];
        check_paths(
            ee::s14::f_003f65a0__generated,
            usb_kb_get_keypad_entry,
            56,
            &[9, 11, 12],
            &|c, seed| {
                let _r = machine(c, seed, 0x3f_65a0, 0);
                c.pc = 0x003f_65a0;
                c.g[A0][0] = USAGES[(seed % 14) as usize] | ((seed / 14 % 2) as u64) << 40;
            },
        );
    }

    #[test]
    fn usb_kb_lookup_key_char_matches() {
        const USAGES: [u64; 12] = [
            0x04, 0x1e, 0x2f, 0x35, 0x58, 0x59, 0x62, 0x63, 0x64, 0x87, 0x8a, 0xe1,
        ];
        // No num lock: layout 0 (20) or the alternate's three kinds (27, 29,
        // 30); num lock: a keypad digit (22), else layout 0 (24) or the
        // alternate's (31, 33, 34).
        check_paths(
            ee::s14::f_003f65f8__generated,
            usb_kb_lookup_key_char,
            144,
            &[20, 22, 24, 27, 29, 30, 31, 33, 34],
            &|c, seed| {
                let mut r = machine(c, seed, 0x3f_65f8, 0);
                elf_image(c);
                c.pc = 0x003f_65f8;
                let out = DATA + 0x40;
                let data = DATA + 0x80;
                fill(c, &mut r, out, 4);
                let led = r.next_u32() & !1 | seed % 2;
                c.bus.write32(data + USB_KB_DATA_LED, led);
                c.g[A0][0] = sx(out);
                c.g[A1][0] = sx(data);
                c.g[A2][0] = [0u64, 1, 1 << 32][(seed / 2 % 3) as usize];
                c.g[A3][0] = USAGES[(seed / 6 % 12) as usize] | ((seed / 72 % 2) as u64) << 36;
            },
        );
    }

    /// The usages `UsbKb_ConvertRawCode`'s test picks: keypad operators,
    /// digits, ordinary keys, usages past 0xff (0x100: `sceUsbKbCnvRawCode`
    /// returns it, the first that is no character), the first usage past
    /// the keypad (0x64), and keypad usages with upper bits (the 64-bit
    /// `< 0x59` compare takes them for digits).
    pub(crate) const USB_KB_TEST_CONVERT_USAGES: [u64; 14] = [
        0x54,
        0x58,
        0x59,
        0x63,
        0x04,
        0x1e,
        0x2c,
        0x3a,
        0x139,
        0x87,
        0x1_0000_0055,
        0x1_0000_0059,
        0x64,
        0x100,
    ];

    /// `seed % 3`: the arrangement (0-2: `sceUsbKbCnvRawCode` prints and
    /// fails on others); `seed / 3 % 4`: LED and modifier words (num lock
    /// off/on, caps lock, shifts); `seed / 12 % 14`: the usage.
    pub(crate) fn usb_kb_convert_raw_code_setup(c: &mut Cpu, seed: u32) {
        let mut r = machine(c, seed, 0x3f_6670, 0);
        elf_image(c);
        c.pc = 0x003f_6670;
        let out = DATA + 0x40;
        let data = DATA + 0x80;
        fill(c, &mut r, out, 4);
        let (led, mkey) = [(0u32, 0u32), (1, 0), (2, 0x02), (3, 0x20)][(seed / 3 % 4) as usize];
        c.bus.write32(data + USB_KB_DATA_LED, led);
        c.bus.write32(data + USB_KB_DATA_MKEY, mkey);
        c.g[A0][0] = sx(out);
        c.g[A1][0] = sx(data);
        c.g[A2][0] = (seed % 3) as u64;
        c.g[A3][0] = USB_KB_TEST_CONVERT_USAGES[(seed / 12 % 14) as usize];
    }

    /// Keypad, by what each arm leaves in `out[1]` (the two 20-cycle arms
    /// apart): an operator its character (4), a digit without num lock 0
    /// where its character is not (0), a digit with num lock its character
    /// (1, 24 cycles); 99 for anything else. Other keys:
    /// `sceUsbKbCnvRawCode` gave a character (2) or 256 or more (3, left in
    /// v1).
    pub(crate) fn usb_kb_convert_raw_code_arm(c: &mut Cpu, seed: u32) -> u32 {
        let usage = USB_KB_TEST_CONVERT_USAGES[(seed / 12 % 14) as usize];
        let index = (usage as u32).wrapping_sub(USB_KB_USAGE_KEYPAD);
        if index as u64 >= USB_KB_KEYPAD_KEYS {
            return 2 + ((c.g[V1][0] as i64) >= 0x100) as u32;
        }
        let ch = c.bus.read8(USB_KB_KEYPAD_CHARS + index);
        let out = c.bus.read8(DATA + 0x41);
        let digit = usage >= USB_KB_USAGE_KEYPAD_1 as u64;
        let lit = c.bus.read32(DATA + 0x80 + USB_KB_DATA_LED) & 1 != 0;
        match (digit, lit) {
            (false, _) if out == ch => 4,
            (true, false) if out == 0 && ch != 0 => 0,
            (true, true) if out == ch => 1,
            _ => 99,
        }
    }

    #[test]
    fn usb_kb_convert_raw_code_matches() {
        check(
            ee::s14::f_003f6670__generated,
            usb_kb_convert_raw_code,
            168,
            &usb_kb_convert_raw_code_setup,
        );
        // Keypad: an operator or a digit without num lock (20: told apart by
        // the arms below), a digit with num lock (24).
        require_paths(
            ee::s14::f_003f6670__generated,
            168,
            &[20, 24],
            &usb_kb_convert_raw_code_setup,
        );
        usb_kb_require_arms(
            ee::s14::f_003f6670__generated,
            168,
            &[0, 1, 2, 3, 4],
            &usb_kb_convert_raw_code_setup,
            &usb_kb_convert_raw_code_arm,
        );
    }

    #[test]
    fn usb_kb_check_modifier_key_matches() {
        const KEYS: [u64; 16] = [
            0xa0,
            0x12,
            0x10,
            0x11,
            0x05,
            0x5b,
            0x5c,
            0x41,
            0xa3,
            0xa1,
            0xa2,
            0xa4,
            0xa5,
            0xff,
            0x1_0000_00a0,
            0x13,
        ];
        check_paths(
            ee::s14::f_003f6700__generated,
            usb_kb_check_modifier_key,
            64,
            &[10, 14, 16, 19, 20, 21, 22, 23],
            &|c, seed| {
                let mut r = machine(c, seed, 0x3f_6700, 0);
                c.pc = 0x003f_6700;
                let out = DATA + 0x10 + 4 * (seed / 16 % 4);
                fill(c, &mut r, DATA, 0x10);
                c.g[A0][0] = r.next_u32() as u64;
                c.g[A1][0] = KEYS[(seed % 16) as usize];
                c.g[A2][0] = sx(out);
            },
        );
        // The totals 19, 20 and 21 belong to several arms: pin each key's
        // arm by its total and its effect (every modifier bit set, so v1 and
        // `*down` are the arm's bits; nothing stored when not a modifier).
        const ARMS: [(u64, u64, u64, u64); 14] = [
            // (key, total, v0, v1)
            (0xa0, 10, 1, 0x02), // left shift
            (0x12, 14, 1, 0x04), // alt: left alt
            (0xa3, 16, 1, 0x10), // right control
            (0x10, 19, 1, 0x02), // shift: left shift
            (0x11, 19, 1, 0x11), // control: both
            (0x5b, 20, 1, 0x08), // left Windows
            (0x5c, 20, 1, 0x80), // right Windows
            (0xa1, 20, 1, 0x20), // right shift
            (0xa2, 20, 1, 0x01), // left control
            (0xa4, 20, 1, 0x04), // left alt
            (0x05, 21, 0, 0),    // below alt: none
            (0xa5, 21, 1, 0x40), // right alt
            (0x41, 22, 0, 0),    // below left shift: none
            (0xff, 23, 0, 0),    // past right alt: none
        ];
        for (key, total, v0, v1) in ARMS {
            let mut c = fresh();
            let _r = machine(&mut c, 0, 0x3f_6701, 0);
            c.pc = 0x003f_6700;
            let out = DATA + 0x10;
            c.bus.write32(out, USB_KB_TEST_MARK);
            c.g[A0][0] = u32::MAX as u64;
            c.g[A1][0] = key;
            c.g[A2][0] = sx(out);
            let before = c.cycles;
            run_generated(&mut c, ee::s14::f_003f6700__generated);
            let down = if v0 == 1 { v1 as u32 } else { USB_KB_TEST_MARK };
            let got = (c.cycles - before, c.g[V0][0], c.g[V1][0], c.bus.read32(out));
            assert_eq!(got, (total, v0, v1, down), "key {key:#x}");
        }
    }

    #[test]
    fn usb_kb_check_led_key_matches() {
        const KEYS: [u64; 8] = [0x90, 0x14, 0x13, 0x91, 0x92, 0x05, 0x1_0000_0090, 0x8f];
        check_paths(
            ee::s14::f_003f67e0__generated,
            usb_kb_check_led_key,
            32,
            &[9, 13, 14, 15, 16],
            &|c, seed| {
                let mut r = machine(c, seed, 0x3f_67e0, 0);
                c.pc = 0x003f_67e0;
                let out = DATA + 0x10 + 4 * (seed / 8 % 4);
                fill(c, &mut r, DATA, 0x10);
                c.g[A0][0] = r.next_u32() as u64;
                c.g[A1][0] = KEYS[(seed % 8) as usize];
                c.g[A2][0] = sx(out);
            },
        );
    }

    /// `seed % 8`: the key asked about and the usage at the index: left
    /// shift, right Windows (modifiers), num and caps lock (lock keys), 'A'
    /// with usage 4 ('A': down) or 5 ('B'), keypad 1's num-lock code with
    /// usage 0x59, 'A' with upper bits (never down). `seed / 8 % 4`: LED and
    /// modifier words; `seed / 32 % 3`: the layout.
    #[test]
    fn usb_kb_is_key_down_matches() {
        const CASES: [(u64, u16); 8] = [
            (0xa0, 0x04),
            (0x5c, 0x04),
            (0x90, 0x04),
            (0x14, 0x04),
            (0x41, 0x04),
            (0x41, 0x05),
            (0x61, 0x59),
            (0x1_0000_0041, 0x04),
        ];
        let setup = |c: &mut Cpu, seed: u32| {
            let mut r = machine(c, seed, 0x3f_6838, 0);
            elf_image(c);
            c.pc = 0x003f_6838;
            let data = DATA + 0x100;
            fill(c, &mut r, data, 0x24);
            let (key, usage) = CASES[(seed % 8) as usize];
            let (led, mkey) =
                [(0u32, 0u32), (1, 0x02), (2, 0x80), (7, 0xff)][(seed / 8 % 4) as usize];
            c.bus.write32(data + USB_KB_DATA_LED, led);
            c.bus.write32(data + USB_KB_DATA_MKEY, mkey);
            let index = seed / 8 % 6 * 9;
            c.bus.write16(data + USB_KB_DATA_KEYS + 2 * index, usage);
            c.g[A0][0] = sx(data);
            c.g[A1][0] = (seed / 32 % 3) as u64;
            c.g[A2][0] = index as u64;
            c.g[A3][0] = key;
        };
        check(
            ee::s14::f_003f6838__generated,
            usb_kb_is_key_down,
            96,
            &setup,
        );
        // Left shift (39), num lock (66), 'A' with LEDs off and layout 0:
        // other key (100), down (103).
        require_paths(
            ee::s14::f_003f6838__generated,
            96,
            &[39, 66, 100, 103],
            &setup,
        );
    }

    /// `seed % 3`: the arrangement; `seed / 3 % 4`: LED and modifier words;
    /// `seed / 12 % 10`: the usage at the index.
    #[test]
    fn usb_kb_translate_key_matches() {
        const USAGES: [u16; 10] = [0x04, 0x1e, 0x2c, 0x2f, 0x3a, 0x54, 0x59, 0x63, 0x87, 0x139];
        check(
            ee::s14::f_003f68f0__generated,
            usb_kb_translate_key,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3f_68f0, 0);
                elf_image(c);
                c.pc = 0x003f_68f0;
                let data = DATA + 0x100;
                fill(c, &mut r, data, 0x24);
                fill(c, &mut r, USB_KB_TEST_KEY_PAIR, 1);
                let (led, mkey) =
                    [(0u32, 0u32), (1, 0x02), (2, 0x20), (3, 0x11)][(seed / 3 % 4) as usize];
                c.bus.write32(data + USB_KB_DATA_LED, led);
                c.bus.write32(data + USB_KB_DATA_MKEY, mkey);
                let index = seed / 12 % 10 * 5;
                c.bus.write16(
                    data + USB_KB_DATA_KEYS + 2 * index,
                    USAGES[(seed / 12 % 10) as usize],
                );
                c.g[A0][0] = sx(data);
                c.g[A1][0] = (seed % 3) as u64;
                c.g[A2][0] = index as u64;
            },
        );
    }

    /// `UsbKb_TranslateKey`'s pair with the game's gp.
    pub(crate) const USB_KB_TEST_KEY_PAIR: u32 = crate::difftest::GAME_GP + USB_KB_KEY_PAIR_GP;

    /// The client's current packet and request id.
    pub(crate) const VOIP_RPC_TEST_CLIENT_PACKET: u32 = 0x0;

    pub(crate) const VOIP_RPC_TEST_CLIENT_ID: u32 = 0x4;

    /// The previous call's packet (the client's, when there is one).
    pub(crate) const VOIP_RPC_TEST_PREV_PACKET: u32 = DATA + 0x400;

    /// The pool's two packets.
    pub(crate) const VOIP_RPC_TEST_PACKETS: u32 = DATA + 0x1000;

    /// The test's IOP side: +0 deliveries at the wait head before the
    /// pending call completes, +4 those seen; +8 deliveries at the retry
    /// head before a packet frees up, +0xc those seen.
    pub(crate) const VOIP_RPC_TEST_IOP: u32 = DATA + 0x700;

    /// What the IOP side looks like when `VoipRpc_Call` runs.
    #[derive(Clone, Copy)]
    pub(crate) struct VoipRpcIop {
        /// The enabled flag byte.
        enabled: u8,
        /// Wait-loop rounds before the previous call completes (0: none
        /// pending).
        waits: u32,
        /// With nothing pending, why `sceSifCheckStatRpc` says 0: no
        /// packet (0), another request's packet (1), its in-use bit clear (2).
        idle: u32,
        /// `sceSifCallRpc` failures (no free packet) before one frees up.
        retries: u32,
        /// The failing pool is empty (else both packets are in use).
        empty_pool: bool,
    }

    /// The client, its buffers and the SIF packet pool as `iop` says, all
    /// other words junk.
    pub(crate) fn voip_rpc_fixture(c: &mut Cpu, r: &mut Rng, iop: VoipRpcIop) {
        c.bus.write8(GAME_GP + VOIP_RPC_ENABLED_GP, iop.enabled);
        // Send buffer, client, receive buffer, shared status block.
        fill(c, r, VOIP_RPC_SEND_BUF, 0x60);
        fill(c, r, SIF_CMD_WORD, 1);
        let prev = VOIP_RPC_TEST_PREV_PACKET;
        fill(c, r, prev, 0x10);
        let id = r.next_u32();
        c.bus.write32(prev + SIF_RPC_PACKET_ID, id);
        let flags = r.next_u32();
        let (packet, client_id, flags) = if iop.waits > 0 {
            (prev, id, flags | 1)
        } else {
            match iop.idle {
                0 => (0, id, flags | 1),
                1 => (prev, id ^ 0x10, flags | 1),
                _ => (prev, id, flags & !1),
            }
        };
        c.bus
            .write32(VOIP_RPC_CLIENT + VOIP_RPC_TEST_CLIENT_PACKET, packet);
        c.bus
            .write32(VOIP_RPC_CLIENT + VOIP_RPC_TEST_CLIENT_ID, client_id);
        c.bus.write32(prev + SIF_RPC_PACKET_FLAGS, flags);
        let count = if iop.retries > 0 && iop.empty_pool {
            0
        } else {
            2
        };
        let counter = r.next_u32() | 0x10;
        c.bus.write32(SIF_RPC_POOL, counter);
        c.bus.write32(SIF_RPC_POOL + 4, VOIP_RPC_TEST_PACKETS);
        c.bus.write32(SIF_RPC_POOL + 8, count);
        fill(c, r, VOIP_RPC_TEST_PACKETS, 0x20);
        for i in 0..2u32 {
            let f = r.next_u32();
            let in_use = i == 1 || iop.retries > 0;
            let f = if in_use { f | 1 } else { f & !1 };
            c.bus
                .write32(VOIP_RPC_TEST_PACKETS + i * 0x40 + SIF_RPC_PACKET_FLAGS, f);
        }
        let iop_words = [iop.waits, 0, iop.retries, 0];
        for (i, w) in iop_words.into_iter().enumerate() {
            c.bus.write32(VOIP_RPC_TEST_IOP + 4 * i as u32, w);
        }
    }

    /// `sceSifRpc_GetPacket`'s scan loop head, its saved ra (+0x20) when
    /// `sceSifCallRpc` called it, and the two frames from its sp up to
    /// `VoipRpc_Call`'s (0x30, 0xc0).
    pub(crate) const VOIP_RPC_TEST_GET_PACKET_HEAD: u32 = 0x0042_6600;

    pub(crate) const VOIP_RPC_TEST_CALL_RPC_RA: u32 = 0x0042_6d70;

    pub(crate) const VOIP_RPC_TEST_CALL_FRAMES: u32 = 0x30 + 0xc0;

    /// What the IOP side leaves in `VoipRpc_Call`'s end_param stack word
    /// during a call that will fail.
    pub(crate) const VOIP_RPC_TEST_DIRT: u32 = 0x5a5a_5a5a;

    /// The IOP's progress at an event: at the wait head the pending call
    /// completes (its packet's in-use bit clears) after the set number of
    /// deliveries; at the retry head a packet frees up (the pool's last;
    /// an empty pool gets one). In the packet scan of a `sceSifCallRpc`
    /// that will fail (no packet freed yet), `VoipRpc_Call`'s end_param
    /// word gets dirt: only the retry loop's own `sw zero` clears it again
    /// before the next call reads it.
    pub(crate) fn voip_rpc_iop_step(c: &mut Cpu) {
        if c.pc == VOIP_RPC_TEST_GET_PACKET_HEAD {
            let frame = c.g[SP][0] as u32;
            let seen = c.bus.read32(VOIP_RPC_TEST_IOP + 0xc);
            let failing = seen < c.bus.read32(VOIP_RPC_TEST_IOP + 8);
            if failing && c.bus.read32(frame + 0x20) == VOIP_RPC_TEST_CALL_RPC_RA {
                c.bus
                    .write32(frame + VOIP_RPC_TEST_CALL_FRAMES, VOIP_RPC_TEST_DIRT);
            }
            return;
        }
        let (target, seen) = match c.pc {
            VOIP_RPC_CALL_WAIT_HEAD => (0, 4),
            VOIP_RPC_CALL_RETRY_HEAD => (8, 0xc),
            _ => return,
        };
        let n = c.bus.read32(VOIP_RPC_TEST_IOP + seen) + 1;
        c.bus.write32(VOIP_RPC_TEST_IOP + seen, n);
        if n < c.bus.read32(VOIP_RPC_TEST_IOP + target) {
            return;
        }
        let flags_at = if c.pc == VOIP_RPC_CALL_WAIT_HEAD {
            c.bus.read32(VOIP_RPC_CLIENT + VOIP_RPC_TEST_CLIENT_PACKET) + SIF_RPC_PACKET_FLAGS
        } else {
            let mut count = c.bus.read32(SIF_RPC_POOL + 8);
            if count == 0 {
                count = 1;
                c.bus.write32(SIF_RPC_POOL + 8, count);
            }
            c.bus.read32(SIF_RPC_POOL + 4) + (count - 1) * 0x40 + SIF_RPC_PACKET_FLAGS
        };
        let f = c.bus.read32(flags_at);
        c.bus.write32(flags_at, f & !1);
    }

    /// Event hook of the plain tests (`next_event` 0: every check delivers):
    /// the IOP steps, nothing else happens.
    pub(crate) fn voip_rpc_iop_event(c: &mut Cpu) -> bool {
        assert!(c.cycles < CYCLE_BUDGET, "runaway at pc {:#x}", c.pc);
        voip_rpc_iop_step(c);
        false
    }

    /// Event hook of the event tests: the IOP steps, then the machine's real
    /// event delivery.
    pub(crate) fn voip_rpc_iop_live_event(c: &mut Cpu) -> bool {
        assert!(c.cycles < CYCLE_BUDGET, "runaway at pc {:#x}", c.pc);
        voip_rpc_iop_step(c);
        (ssx3_ps2::platform::HOOKS.event)(c)
    }

    /// The arms of `VoipRpc_Call`: disabled (`seed % 4 == 0`), else
    /// 0-2 wait rounds (`seed / 4 % 3`) crossed with 0-2 call retries
    /// (`seed / 36 % 3`). `seed / 12 % 3` picks why nothing is pending
    /// (no wait rounds), `seed / 108 % 2` an empty or full pool for the
    /// retries (in a full one each failing call meets the IOP side's dirt
    /// on the stack word, `voip_rpc_iop_step`) and the receive buffer. The
    /// four syscalls of a successful `sceSifCallRpc` are fakes (some
    /// unwinding); everything else is real.
    #[test]
    fn voip_rpc_call_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            let mut r = machine(c, seed, 0x3f_6960, 4);
            c.pc = 0x003f_6960;
            let iop = VoipRpcIop {
                enabled: [0u8, 1, 0x80, 0xff][(seed % 4) as usize],
                waits: seed / 4 % 3,
                idle: seed / 12 % 3,
                retries: seed / 36 % 3,
                empty_pool: seed / 108 % 2 == 1,
            };
            voip_rpc_fixture(c, &mut r, iop);
            c.next_event = 0;
            c.hooks.event = voip_rpc_iop_event;
            let (recv, size) = if seed / 108 % 2 == 0 {
                (VOIP_RPC_RECV_BUF, 0x80)
            } else {
                (DATA + 0x800, 0x40)
            };
            fill(c, &mut r, DATA + 0x800, 0x10);
            c.g[A0][0] = sx(r.next_u32() % 16);
            c.g[A1][0] = (r.next_u32() as u64) << 32 | r.next_u32() as u64;
            c.g[A2][0] = (r.next_u32() as u64) << 32 | r.next_u32() as u64;
            c.g[A3][0] = sx(recv);
            c.g[T0][0] = size;
        };
        covered(ee::s14::f_003f6960__generated, voip_rpc_call, 216, &setup);
        // Every arm ran: (enabled, wait rounds, retries) as the IOP side saw them.
        let mut seen = std::collections::BTreeSet::new();
        for seed in 0..216 {
            let mut c = fresh();
            setup(&mut *c, seed);
            run_generated(&mut c, ee::s14::f_003f6960__generated);
            let enabled = c.bus.read8(GAME_GP + VOIP_RPC_ENABLED_GP) != 0;
            let waits = c.bus.read32(VOIP_RPC_TEST_IOP + 4);
            let retries = c.bus.read32(VOIP_RPC_TEST_IOP + 0xc);
            seen.insert((enabled, waits, retries));
        }
        assert!(
            seen.contains(&(false, 0, 0)),
            "disabled arm not run: {seen:?}"
        );
        for waits in 0..3 {
            for retries in 0..3 {
                assert!(
                    seen.contains(&(true, waits, retries)),
                    "arm (waits {waits}, retries {retries}) not run: {seen:?}"
                );
            }
        }
    }

    /// `VoipRpc_Call` with real events: `seed % 3` the event mode; with
    /// events on, `seed / 3 % 2` makes the first due check the wait head
    /// (one round pending) or the retry head (an empty pool, one retry).
    #[test]
    fn voip_rpc_call_events() {
        driven!(gen, ee::s14::f_003f6960__generated);
        driven!(hand, voip_rpc_call);
        check_events(gen, hand, 120, &|c, seed| {
            let mut r = machine(c, seed, 0x3f_6961, 0);
            let (waits, retries) = match (seed % 3, seed / 3 % 2) {
                (0, _) => (0, 0),
                (_, 0) => (1, 0),
                _ => (0, 1),
            };
            let iop = VoipRpcIop {
                enabled: 1,
                waits,
                idle: seed / 6 % 3,
                retries,
                empty_pool: true,
            };
            voip_rpc_fixture(c, &mut r, iop);
            c.g[A0][0] = sx(r.next_u32() % 16);
            c.g[A1][0] = r.next_u32() as u64;
            c.g[A2][0] = r.next_u32() as u64;
            c.g[A3][0] = sx(VOIP_RPC_RECV_BUF);
            c.g[T0][0] = VOIP_RPC_BUF_SIZE;
            let mode = schedule(c, &mut r, seed, 0x003f_6960);
            c.hooks.event = voip_rpc_iop_live_event;
            mode
        });
    }

    /// `VoipRpc_Init`: the bind succeeds (`seed % 2 == 0`; then 1-2 wait
    /// rounds for it, `seed / 2 % 2`) or finds no packet (an empty or full
    /// pool, `seed / 2 % 2`; then 1-2 call retries, `seed / 4 % 2`). Bind
    /// and call syscalls are fakes, some unwinding.
    #[test]
    fn voip_rpc_init_matches() {
        covered(
            ee::s14::f_003f6a40__generated,
            voip_rpc_init,
            64,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3f_6a40, 8);
                c.pc = 0x003f_6a40;
                let bind_ok = seed % 2 == 0;
                let iop = VoipRpcIop {
                    enabled: r.next_u32() as u8,
                    waits: if bind_ok { 1 + seed / 2 % 2 } else { 0 },
                    idle: 0,
                    retries: if bind_ok { 0 } else { 1 + seed / 4 % 2 },
                    empty_pool: seed / 2 % 2 == 1,
                };
                voip_rpc_fixture(c, &mut r, iop);
                c.next_event = 0;
                c.hooks.event = voip_rpc_iop_event;
            },
        );
    }

    #[test]
    fn voip_rpc_disable_matches() {
        check(
            ee::s14::f_003f6ab0__generated,
            voip_rpc_disable,
            16,
            &|c, seed| {
                let mut r = Rng(0x3f_6ab0 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.pc = 0x003f_6ab0;
                fill(c, &mut r, GAME_GP + (VOIP_RPC_ENABLED_GP & !3), 2);
            },
        );
    }

    /// The command wrappers: `VoipRpc_Call` runs for real, enabled or not
    /// (`seed % 3 == 0`: not), with a wait round (`seed / 3 % 2`), a retry
    /// (`seed / 6 % 2`; empty or full pool, `seed / 12 % 2`), fake syscalls
    /// (some unwinding). `args` sets the wrapper's arguments (its own
    /// steering from `seed / 24 % 2`).
    pub(crate) fn voip_rpc_wrapper_check(
        gen: GuestFn,
        hand: GuestFn,
        entry: u32,
        args: fn(&mut Cpu, &mut Rng, u32),
    ) {
        covered(gen, hand, 48, &|c, seed| {
            let mut r = machine(c, seed, entry as u64, 4);
            c.pc = entry;
            let iop = VoipRpcIop {
                enabled: if seed % 3 == 0 { 0 } else { 1 },
                waits: seed / 3 % 2,
                idle: 0,
                retries: seed / 6 % 2,
                empty_pool: seed / 12 % 2 == 1,
            };
            voip_rpc_fixture(c, &mut r, iop);
            c.next_event = 0;
            c.hooks.event = voip_rpc_iop_event;
            args(c, &mut r, seed);
        });
    }

    /// No arguments read (the scrambled a0-t0 are overwritten).
    pub(crate) fn voip_rpc_no_args(_: &mut Cpu, _: &mut Rng, _: u32) {}

    /// `VoipRpc_Start`: `hi`'s bit 15 (the shifted word's sign) set or clear
    /// (`seed / 24 % 2`), junk upper halves everywhere.
    #[test]
    fn voip_rpc_start_matches() {
        fn args(c: &mut Cpu, r: &mut Rng, seed: u32) {
            c.g[A0][0] = (r.next_u32() as u64) << 32 | r.next_u32() as u64;
            c.g[A1][0] = (r.next_u32() as u64) << 32 | r.next_u32() as u64;
            let hi = r.next_u32() & !0x8000 | (seed / 24 % 2) << 15;
            c.g[A2][0] = (r.next_u32() as u64) << 32 | hi as u64;
        }
        voip_rpc_wrapper_check(
            ee::s14::f_003f6ab8__generated,
            voip_rpc_start,
            0x003f_6ab8,
            args,
        );
    }

    #[test]
    fn voip_rpc_stop_matches() {
        voip_rpc_wrapper_check(
            ee::s14::f_003f6af0__generated,
            voip_rpc_stop,
            0x003f_6af0,
            voip_rpc_no_args,
        );
    }

    #[test]
    fn voip_rpc_enable_voice_matches() {
        voip_rpc_wrapper_check(
            ee::s14::f_003f6b20__generated,
            voip_rpc_enable_voice,
            0x003f_6b20,
            voip_rpc_no_args,
        );
    }

    #[test]
    fn voip_rpc_disable_voice_matches() {
        voip_rpc_wrapper_check(
            ee::s14::f_003f6b50__generated,
            voip_rpc_disable_voice,
            0x003f_6b50,
            voip_rpc_no_args,
        );
    }

    /// `VoipRpc_IsActive`: the status block in RAM or at its real address,
    /// bit 0 set or clear (`seed / 2 % 2`).
    #[test]
    fn voip_rpc_is_active_matches() {
        check(
            ee::s14::f_003f6b80__generated,
            voip_rpc_is_active,
            32,
            &|c, seed| {
                let mut r = Rng(0x3f_6b80 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.pc = 0x003f_6b80;
                let status = if seed % 2 == 0 {
                    DATA + 0x300
                } else {
                    VOIP_RPC_SHARED_STATUS
                };
                c.bus.write32(GAME_GP + VOIP_RPC_SHARED_STATUS_GP, status);
                c.bus.write32(status, r.next_u32() & !1 | seed / 2 % 2);
            },
        );
    }

    /// `VoipRpc_GetStatus`: two output words, or both pointers at one word
    /// (`seed / 24 % 2`: the second store wins), pre-filled.
    #[test]
    fn voip_rpc_get_status_matches() {
        fn args(c: &mut Cpu, r: &mut Rng, seed: u32) {
            let out0 = DATA + 0x600;
            let out1 = if seed / 24 % 2 == 0 {
                DATA + 0x608
            } else {
                out0
            };
            fill(c, r, out0, 4);
            c.g[A0][0] = sx(out0);
            c.g[A1][0] = sx(out1);
        }
        voip_rpc_wrapper_check(
            ee::s14::f_003f6ba0__generated,
            voip_rpc_get_status,
            0x003f_6ba0,
            args,
        );
    }

    #[test]
    fn voip_rpc_set_params_matches() {
        fn args(c: &mut Cpu, r: &mut Rng, _: u32) {
            c.g[A0][0] = (r.next_u32() as u64) << 32 | r.next_u32() as u64;
            c.g[A1][0] = (r.next_u32() as u64) << 32 | r.next_u32() as u64;
        }
        voip_rpc_wrapper_check(
            ee::s14::f_003f6c10__generated,
            voip_rpc_set_params,
            0x003f_6c10,
            args,
        );
    }

    /// `VoipRpc_GetSharedStatus`: the status block in RAM or at its real
    /// address.
    #[test]
    fn voip_rpc_get_shared_status_matches() {
        check(
            ee::s14::f_003f6c40__generated,
            voip_rpc_get_shared_status,
            16,
            &|c, seed| {
                let mut r = Rng(0x3f_6c40 ^ (seed as u64) << 8);
                scramble(c, &mut r);
                c.g[GP][0] = GAME_GP as u64;
                c.pc = 0x003f_6c40;
                let status = if seed % 2 == 0 {
                    DATA + 0x300
                } else {
                    VOIP_RPC_SHARED_STATUS
                };
                c.bus.write32(GAME_GP + VOIP_RPC_SHARED_STATUS_GP, status);
                fill(c, &mut r, status, 1);
            },
        );
    }

    /// The Aries reply callback stashes `t3` and clears both status words.
    #[test]
    fn proto_aries_rpc_reply_callback_matches() {
        check(
            ee::s14::f_003f5a50__generated,
            proto_aries_rpc_reply_callback,
            200,
            &|c, seed| {
                machine(c, seed, 0x3f5a_50, 0);
                let slot = DATA + 0x2000 + (seed / 8 % 8) * 0x10;
                c.bus.write32(crate::difftest::SP, slot);
                c.g[T3][0] = (DATA + 0x1000 + (seed % 8) * 0x10) as u64;
                // Pre-fill the cleared status words: the stores must turn
                // nonzero into zero, not zero into zero.
                c.bus.write32(slot + 8, 0xBEEF_0000 | seed);
                c.bus.write32(slot + 0xc, 0xBEEF_0001 | seed);
            },
        );
    }

    // ------------------------------------------------------------------
    // `DirtyRpc_IntcHandler` (0x3f4410).

    pub(crate) const INTC_STATE: u32 = DATA + 0x1000;

    /// The interrupt handler adds the count difference to the 64-bit
    /// total and latches the new count.
    #[test]
    fn dirty_rpc_intc_handler_matches() {
        check(
            ee::s14::f_003f4410__generated,
            dirty_rpc_intc_handler,
            200,
            &|c, seed| {
                machine(c, seed, 0x3f44_10, 0);
                c.g[A0][0] = [
                    0,
                    1,
                    0x7fff_ffff,
                    0x8000_0000u32.cast_signed(),
                    0xffff_ffffu32.cast_signed(),
                ][seed as usize % 5] as u64;
                c.g[A1][0] = sx(INTC_STATE);
                c.bus.write32(
                    INTC_STATE + 0x4df8,
                    [0, 100, 0xffff_ffffu32][seed as usize / 5 % 3],
                );
                c.bus.write32(INTC_STATE + 0x4e00, 0x1111_1111 + seed);
                c.bus.write32(INTC_STATE + 0x4e04, 0x2222_2222 + seed);
            },
        );
    }
}
