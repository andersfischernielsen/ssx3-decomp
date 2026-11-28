//! The lobby client of `src/dirtysock/` (`LobbyApi_*`) with the NetGame link
//! and utility layers under it (`NetGameLink*`, `NetGameUtil*`) and
//! `ProtoPing*`: hand-written replacements. The reference names no file for
//! them (they sit in the anonymous `seg_2DCAE8`), so this unit is placed by
//! hand, beside `buddyapi.rs`.

use crate::abi::ret;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::returned;
use crate::call::Frame;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
use crate::guest::Addr;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
use ssx3_ee_rt::funcs::call_indirect;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this unit: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x003e_66f0, net_game_link_create, "NetGameLinkCreate"),
    (0x003e_67f8, net_game_link_destroy, "NetGameLinkDestroy"),
    (
        0x003e_6878,
        net_game_link_copy_stats,
        "NetGameLink_CopyStats",
    ),
    (0x003e_6958, net_game_link_status, "NetGameLinkStatus"),
    (
        0x003e_6a70,
        net_game_link_peek_packet,
        "NetGameLink_PeekPacket",
    ),
    (0x003e_6b10, net_game_link_recv, "NetGameLinkRecv"),
    (0x003e_6c80, net_game_link_send, "NetGameLinkSend"),
    (0x003e_6d78, net_game_link_control, "NetGameLinkControl"),
    (0x003e_6e08, net_game_util_create, "NetGameUtilCreate"),
    (0x003e_6e70, net_game_util_destroy, "NetGameUtilDestroy"),
    (0x003e_6ed8, net_game_util_reset, "NetGameUtilReset"),
    (0x003e_6f40, net_game_util_connect, "NetGameUtilConnect"),
    (0x003e_6fc8, net_game_util_complete, "NetGameUtilComplete"),
    (0x003e_7038, net_game_util_status, "NetGameUtilStatus"),
    (0x003e_70b8, net_game_util_stat, "NetGameUtilStat"),
    (0x003e_7120, net_game_util_advert, "NetGameUtilAdvert"),
    (0x003e_71b0, net_game_util_withdraw, "NetGameUtilWithdraw"),
    (0x003e_7218, net_game_util_locate, "NetGameUtilLocate"),
    (0x003e_72a0, net_game_util_query, "NetGameUtilQuery"),
    (0x003e_7340, proto_ping_create, "ProtoPingCreate"),
    (0x003e_73e0, proto_ping_destroy, "ProtoPingDestroy"),
    (0x003e_7450, proto_ping_request, "ProtoPingRequest"),
    (0x003e_7500, proto_ping_response, "ProtoPingResponse"),
    (0x003e_76a8, lobby_api_debug_printf, "LobbyApi_DebugPrintf"),
    (0x003e_7718, lobby_api_add_request, "LobbyApi_AddRequest"),
    (0x003e_77f0, lobby_api_pop_request, "LobbyApi_PopRequest"),
    (
        0x003e_7918,
        lobby_api_pop_ping_request,
        "LobbyApi_PopPingRequest",
    ),
    (
        0x003e_7ac0,
        lobby_api_flush_requests,
        "LobbyApi_FlushRequests",
    ),
    (
        0x003e_7b00,
        lobby_api_update_room_count,
        "LobbyApi_UpdateRoomCount",
    ),
    (0x003e_7ba0, lobby_api_sort_lists, "LobbyApi_SortLists"),
    (
        0x003e_7c40,
        lobby_api_process_pings,
        "LobbyApi_ProcessPings",
    ),
    (
        0x003e_7f58,
        lobby_api_reset_server_ping,
        "LobbyApi_ResetServerPing",
    ),
    (0x003e_7f70, lobby_api_terminate, "LobbyApi_Terminate"),
    (0x003e_7ff0, lobby_api_create, "LobbyApi_Create"),
    (0x003e_80c0, lobby_api_reset, "LobbyApi_Reset"),
    (0x003e_81c8, lobby_api_destroy, "LobbyApi_Destroy"),
    (0x003e_8248, lobby_api_set_callback, "LobbyApi_SetCallback"),
    (0x003e_8260, lobby_api_connect, "LobbyApi_Connect"),
    (0x003e_8368, lobby_api_select_list, "LobbyApi_SelectList"),
    (0x003e_85b0, lobby_api_create_list, "LobbyApi_CreateList"),
    (0x003e_8688, lobby_api_destroy_list, "LobbyApi_DestroyList"),
    (
        0x003e_8778,
        lobby_api_add_list_entry,
        "LobbyApi_AddListEntry",
    ),
    (
        0x003e_8968,
        lobby_api_remove_list_entry,
        "LobbyApi_RemoveListEntry",
    ),
    (
        0x003e_8a28,
        lobby_api_has_list_entry,
        "LobbyApi_HasListEntry",
    ),
    (0x003e_8a70, lobby_api_get_record, "LobbyApi_GetRecord"),
    (0x003e_8b10, lobby_api_send, "LobbyApi_Send"),
    (
        0x003e_8d70,
        lobby_api_cancel_request,
        "LobbyApi_CancelRequest",
    ),
    (0x003e_8dc0, lobby_api_ping, "LobbyApi_Ping"),
    (0x003e_8f08, lobby_api_status, "LobbyApi_Status"),
    (0x003e_8fb8, lobby_api_get_event, "LobbyApi_GetEvent"),
    (0x003e_8fd0, lobby_api_debug_packet, "LobbyApi_DebugPacket"),
    (
        0x003e_9160,
        lobby_api_trim_rank_list,
        "LobbyApi_TrimRankList",
    ),
    (0x003e_9378, lobby_api_pick_room, "LobbyApi_PickRoom"),
    (
        0x003e_94f0,
        lobby_api_server_ping_callback,
        "LobbyApi_ServerPingCallback",
    ),
    (0x003e_9590, lobby_api_ping_server, "LobbyApi_PingServer"),
    (0x003e_95c8, lobby_api_update, "LobbyApi_Update"),
];

// ---------------------------------------------------------------------------
// Direct-call targets (the generated body's `c.pc = addr; Name(c)` calls).

pub(crate) mod callee {
    pub(crate) use crate::game::dirtysock::buddyapi::callee::{
        DIRTY_RPC_CALL, DIRTY_RPC_INIT_REQUEST, INVALID_DCACHE, LOBBY_HASH_ADD_STR,
        LOBBY_HASH_CREATE, LOBBY_HASH_DEL_STR, LOBBY_HASH_DESTROY, LOBBY_HASH_FIND_STR,
        LOBBY_HASH_POP_ANY, LOBBY_LIST_ADD, LOBBY_LIST_CREATE, LOBBY_LIST_DESTROY,
        LOBBY_LIST_GET_DATA, LOBBY_LIST_MARK_DIRTY, LOBBY_LIST_REMOVE, LOBBY_LIST_SORT,
        LOBBY_MEM_ALLOC, LOBBY_MEM_FREE, NET_RPC_ALLOC, NET_RPC_FREE, NET_TICK,
        PROTO_ARIES_CONNECT, PROTO_ARIES_CREATE, PROTO_ARIES_DESTROY, PROTO_ARIES_SEND,
        PROTO_ARIES_TICK, PROTO_ARIES_UNCONNECT, STRCHR, TAG_FIELD_STRNZCPY, VSPRINTF,
    };
    pub(crate) use crate::game::dirtysock::tags::callee::{
        MEMCPY, MEMSET, SPRINTF, STRCAT, STRCPY, STRLEN, TAG_FIELD_FIND, TAG_FIELD_GET_STRING,
        TAG_FIELD_SET_ADDRESS, TAG_FIELD_SET_NUMBER, TAG_FIELD_SET_STRING,
    };
    pub(crate) use crate::game::dirtysock::tagsunk::callee::{
        LOBBY_LIST_COUNT, LOBBY_LIST_GET_AT, LOBBY_LIST_VISIBLE_COUNT, TAG_FIELD_GET_NUMBER,
    };
    pub(crate) use crate::game::lib::libc::callee::STRCMP;
    pub(crate) use crate::game::render::ps2graphicsman::TEX_LOAD_STRNCPY as STRNCPY;
    pub(crate) use crate::game::sound::bankmonitor::callee::STR_CASE_CMP;
    pub(crate) use crate::game::world::worldcache::STRNCMP;

    use crate::call::Callee;
    use ssx3_ee as ee;

    pub const TAG_FIELD_GET_ADDRESS: Callee = Callee {
        addr: 0x003e_cff8,
        f: ee::s14::TagFieldGetAddress,
    };
    pub const DIRTY_RPC_SYNC_DCACHE: Callee = Callee {
        addr: 0x003f_4e80,
        f: ee::s14::DirtyRpc_SyncDCache,
    };
    pub const NET_GAME_LINK_CHECK_RECV_BUF: Callee = Callee {
        addr: 0x003f_4ea0,
        f: ee::s14::NetGameLink_CheckRecvBuf,
    };
    pub const NET_TICK_WRAPPER2: Callee = Callee {
        addr: 0x003f_4330,
        f: ee::s14::NetTick_Wrapper2,
    };
    pub const LOBBY_ENCRYPT_STRING: Callee = Callee {
        addr: 0x003f_35d0,
        f: ee::s14::Lobby_EncryptString,
    };
    pub const TAG_FIELD_SET_ENCODED: Callee = Callee {
        addr: 0x003e_c6d0,
        f: ee::s14::TagFieldSetEncoded,
    };
    pub const TAG_FIELD_MERGE: Callee = Callee {
        addr: 0x003e_cb00,
        f: ee::s14::TagFieldMerge,
    };
    pub const LOBBY_LIST_UPDATE: Callee = Callee {
        addr: 0x003e_e3e0,
        f: ee::s14::Lobby_ListUpdate,
    };
    pub const LOBBY_LIST_SET_HASH: Callee = Callee {
        addr: 0x003e_e768,
        f: ee::s14::Lobby_ListSetHash,
    };
    pub const LOBBY_LIST_GET_HASH: Callee = Callee {
        addr: 0x003e_e770,
        f: ee::s14::Lobby_ListGetHash,
    };
    pub const LOBBY_HASH_FIND_INT: Callee = Callee {
        addr: 0x003e_eee8,
        f: ee::s14::Lobby_HashFindInt,
    };

    // This unit's own functions, called through their slots.
    pub const NET_GAME_LINK_COPY_STATS: Callee = Callee {
        addr: 0x003e_6878,
        f: ee::s14::NetGameLink_CopyStats,
    };
    pub const NET_GAME_LINK_PEEK_PACKET: Callee = Callee {
        addr: 0x003e_6a70,
        f: ee::s14::NetGameLink_PeekPacket,
    };
    pub const PROTO_PING_CREATE: Callee = Callee {
        addr: 0x003e_7340,
        f: ee::s14::ProtoPingCreate,
    };
    pub const PROTO_PING_DESTROY: Callee = Callee {
        addr: 0x003e_73e0,
        f: ee::s14::ProtoPingDestroy,
    };
    pub const PROTO_PING_REQUEST: Callee = Callee {
        addr: 0x003e_7450,
        f: ee::s14::ProtoPingRequest,
    };
    pub const PROTO_PING_RESPONSE: Callee = Callee {
        addr: 0x003e_7500,
        f: ee::s14::ProtoPingResponse,
    };
    pub const LOBBY_API_DEBUG_PRINTF: Callee = Callee {
        addr: 0x003e_76a8,
        f: ee::s14::LobbyApi_DebugPrintf,
    };
    pub const LOBBY_API_ADD_REQUEST: Callee = Callee {
        addr: 0x003e_7718,
        f: ee::s14::LobbyApi_AddRequest,
    };
    pub const LOBBY_API_POP_REQUEST: Callee = Callee {
        addr: 0x003e_77f0,
        f: ee::s14::LobbyApi_PopRequest,
    };
    pub const LOBBY_API_POP_PING_REQUEST: Callee = Callee {
        addr: 0x003e_7918,
        f: ee::s14::LobbyApi_PopPingRequest,
    };
    pub const LOBBY_API_FLUSH_REQUESTS: Callee = Callee {
        addr: 0x003e_7ac0,
        f: ee::s14::LobbyApi_FlushRequests,
    };
    pub const LOBBY_API_RESET_SERVER_PING: Callee = Callee {
        addr: 0x003e_7f58,
        f: ee::s14::LobbyApi_ResetServerPing,
    };
    pub const LOBBY_API_PING: Callee = Callee {
        addr: 0x003e_8dc0,
        f: ee::s14::LobbyApi_Ping,
    };
    pub const LOBBY_API_RESET: Callee = Callee {
        addr: 0x003e_80c0,
        f: ee::s14::LobbyApi_Reset,
    };
    pub(crate) use crate::game::dirtysock::buddyapi::callee::{PROTO_ARIES_PEEK, PROTO_ARIES_RECV};

    pub(crate) const LOBBY_API_TERMINATE: Callee = Callee {
        addr: 0x003e_7f70,
        f: ee::s14::LobbyApi_Terminate,
    };

    pub(crate) const LOBBY_API_PROCESS_PINGS: Callee = Callee {
        addr: 0x003e_7c40,
        f: ee::s14::LobbyApi_ProcessPings,
    };

    pub(crate) const LOBBY_API_UPDATE_ROOM_COUNT: Callee = Callee {
        addr: 0x003e_7b00,
        f: ee::s14::LobbyApi_UpdateRoomCount,
    };

    pub(crate) const LOBBY_API_PING_SERVER: Callee = Callee {
        addr: 0x003e_9590,
        f: ee::s14::LobbyApi_PingServer,
    };

    pub(crate) const LOBBY_API_SORT_LISTS: Callee = Callee {
        addr: 0x003e_7ba0,
        f: ee::s14::LobbyApi_SortLists,
    };

    pub(crate) const PROTO_ARIES_STATUS: Callee = Callee {
        addr: 0x003f_57c8,
        f: ee::s14::ProtoAriesStatus,
    };

    pub(crate) const TAG_FIELD_SET_BINARY: Callee = Callee {
        addr: 0x003e_c300,
        f: ee::s14::TagFieldSetBinary,
    };

    pub(crate) const TAG_FIELD_DECODE_HEX: Callee = Callee {
        addr: 0x003e_d578,
        f: ee::s14::TagFieldDecodeHex,
    };

    pub(crate) const TAG_FIELD_GET_FLAGS: Callee = Callee {
        addr: 0x003e_cfb0,
        f: ee::s14::TagFieldGetFlags,
    };

    pub(crate) const TAG_FIELD_GET_DATE: Callee = Callee {
        addr: 0x003e_db08,
        f: ee::s14::TagFieldGetDate,
    };

    pub(crate) const LOBBY_HASH_ADD_INT: Callee = Callee {
        addr: 0x003e_eda0,
        f: ee::s14::Lobby_HashAddInt,
    };

    pub(crate) const LOBBY_HASH_DEL_INT: Callee = Callee {
        addr: 0x003e_ef40,
        f: ee::s14::Lobby_HashDelInt,
    };
}

/// `lui` page of the tag keys, default strings and short formats this unit
/// reads (`0x4a4860`..`0x4a4900`).
const LOBBY_STRINGS_PAGE: u64 = 0x004a_0000;

/// `lui` page of the debug-print and `sprintf` formats (`0x495f30`..`0x495f98`).
const LOBBY_FORMATS_PAGE: u64 = 0x0049_0000;

/// `lui` page of the static RPC request blocks the link and ping calls pass to
/// `DirtyRpc_InitRequest` (`0x51ee98`..`0x51eee8`).
const RPC_REQUESTS_PAGE: u64 = 0x0052_0000;

// ---------------------------------------------------------------------------
// The `LobbyApi` object. Field names here and parameter names in the port
// docs below are provisional, read from how these functions use them.

/// `+0x00`: the ProtoAries connection (`ProtoAriesCreate`'s result).
pub(crate) const LOBBY_API_ARIES: u32 = 0x00;

/// `+0x04`: the ProtoPing channel (`ProtoPingCreate`'s result).
pub(crate) const LOBBY_API_PING_CHAN: u32 = 0x04;

/// `+0x08`: pending event bits (`Terminate` ORs its flags in, `GetEvent`
/// takes the lowest) (`?`).
pub(crate) const LOBBY_API_EVENTS: u32 = 0x08;

/// `+0x0c`: connection state tag (`'offl'`, `'rdir'`, `'conn'`, `'idle'`,
/// `'auth'`, `'acct'`, `'skey'`, `'term'` seen here) (`?`).
pub(crate) const LOBBY_API_STATE_TAG: u32 = 0x0c;

/// `+0x10`: connect deadline (`ProtoAriesTick` + 120000, set by `Connect`)
/// (`?`).
pub(crate) const LOBBY_API_TICK: u32 = 0x10;

/// `+0x14`: `ProtoAriesTick` at connect time (`?`).
pub(crate) const LOBBY_API_TICK2: u32 = 0x14;

/// `+0x18`: tick of the next server ping (`?`).
pub(crate) const LOBBY_API_SERVER_PING_A: u32 = 0x18;

/// `+0x1c`: last server-ping reply value, -1 after `ResetServerPing` (`?`).
pub(crate) const LOBBY_API_SERVER_PING_B: u32 = 0x1c;

/// `+0x20`: server pings missed in a row (`ServerPingCallback`) (`?`).
pub(crate) const LOBBY_API_SERVER_PING_C: u32 = 0x20;

/// `+0x24`: the 16-byte login key; a non-zero first byte makes `Send`
/// encrypt the password (`?`).
pub(crate) const LOBBY_API_KEY: u32 = 0x24;

/// `+0x34`: 0x100-byte tag/buffer used for requests and sends (`?`).
pub(crate) const LOBBY_API_TAG_BUF: u32 = 0x34;

/// `+0x310`: 8 list slots of 0x14 bytes: `+0` hash, `+4` list, `+8` notify
/// callback, `+0xc` its argument; this is the hash word (`?`).
pub(crate) const LOBBY_API_TABLES: u32 = 0x310;

/// `+0x314`: the list word of slot 0 (`?`).
pub(crate) const LOBBY_API_TABLES2: u32 = 0x314;

/// A list slot's size; slot `i` is at `LOBBY_API_TABLES + i * LOBBY_SLOT_SIZE`.
pub(crate) const LOBBY_SLOT_SIZE: u32 = 0x14;

/// Slot `+0x04`: the list.
pub(crate) const LOBBY_SLOT_LIST: u32 = 0x04;

/// Slot `+0x08`: the notify callback (`CreateList`'s `a2`).
pub(crate) const LOBBY_SLOT_NOTIFY: u32 = 0x08;

/// Slot `+0x0c`: the notify callback's argument (`CreateList`'s `a3`).
pub(crate) const LOBBY_SLOT_ARG: u32 = 0x0c;

/// `+0x324`/`+0x328`: slot 1's hash and list words (`TABLES`/`TABLES2` of
/// slot 1), which `ProcessPings` scans as the user list (`?`).
pub(crate) const LOBBY_API_USER_HASH: u32 = 0x324;

pub(crate) const LOBBY_API_USER_LIST: u32 = 0x328;

/// `+0x3b0`/`+0x3b4`: `Connect`'s `a2`/`a1` (peer port and host?); `+0x3b4`
/// is also the address `PingServer` pings (`?`).
pub(crate) const LOBBY_API_PEER_PORT: u32 = 0x3b0;

pub(crate) const LOBBY_API_PEER_HOST: u32 = 0x3b4;

/// `+0x3b8`: byte cleared by `Connect`; `GetRecord('down')` returns this
/// address while it is set (`?`).
pub(crate) const LOBBY_API_CONNECTED: u32 = 0x3b8;

/// `+0x50c`: counter cleared by `Connect` and bumped by a `'snap'` send
/// (`?`).
pub(crate) const LOBBY_API_CONNECT_WORD: u32 = 0x50c;

/// `+0x4b8`: the number `Send` writes beside an encoded password (`?`).
pub(crate) const LOBBY_API_PASS_NUMBER: u32 = 0x4b8;

/// `+0x4bc`: what `Send` encodes an unencrypted password with (`?`).
pub(crate) const LOBBY_API_PASS_CODE: u32 = 0x4bc;

/// `+0x514`/`+0x518`: per-index callback function and its argument
/// (`SetCallback` stores them; `LobbyApi_Update` calls through `+0x514`
/// with `+0x518` as the argument).
pub(crate) const LOBBY_API_CB_FUNC: u32 = 0x514;

pub(crate) const LOBBY_API_CB_ARG: u32 = 0x518;

/// `+0x52c`/`+0x530`: terminate callback and its argument (`?`).
pub(crate) const LOBBY_API_TERM_CB: u32 = 0x52c;

pub(crate) const LOBBY_API_TERM_ARG: u32 = 0x530;

/// `+0x53c`: request-queue head (`?`).
pub(crate) const LOBBY_API_REQUESTS: u32 = 0x53c;

/// `+0x540`: queued `'ping'` requests (`Ping` counts up to 64) (`?`).
pub(crate) const LOBBY_API_PING_COUNT: u32 = 0x540;

/// `+0x544`/`+0x548`: debug-print callback argument and function
/// (`Create`'s `a1`/`a2`) (`?`).
pub(crate) const LOBBY_API_DEBUG_ARG: u32 = 0x544;

pub(crate) const LOBBY_API_DEBUG_CB: u32 = 0x548;

/// `+0x54c`: next request id (31 bits) (`?`).
pub(crate) const LOBBY_API_REQ_ID: u32 = 0x54c;

// Request-queue node layout (0x44 bytes): `+0x00` request id, `+0x04` tag,
// `+0x38`/`+0x3c` the two words `AddRequest` stores, `+0x40` next link
// (all `?`).
/// `+0x04`: request tag word (`?`).
pub(crate) const LOBBY_REQ_TAG: u32 = 0x04;

/// `+0x38`: `AddRequest`'s `a2` (for `Ping`/`PingServer` the reply
/// callback); `CancelRequest` clears it (`?`).
pub(crate) const LOBBY_REQ_ARG2: u32 = 0x38;

/// `+0x3c`: `AddRequest`'s `a3` (`?`).
pub(crate) const LOBBY_REQ_ARG3: u32 = 0x3c;

/// `+0x40`: next-request link (`?`).
pub(crate) const LOBBY_REQ_NEXT: u32 = 0x40;

/// Request node size.
pub(crate) const LOBBY_REQ_SIZE: u32 = 0x44;

// Ping-request node: `+0x04` is `'ping'`, `+0x08` the pinged address,
// `+0x0c` the reply deadline, `+0x18` the name, `+0x40` next (all `?`).
/// `+0x08`: the pinged address (`Ping`'s `a2`) (`?`).
pub(crate) const LOBBY_PINGREQ_ADDR: u32 = 0x08;

/// `+0x0c`: reply deadline (`ProtoAriesTick` + `Ping`'s `a3`) (`?`).
pub(crate) const LOBBY_PINGREQ_DEADLINE: u32 = 0x0c;

/// `+0x18`: ping-request name (31 characters, `Ping`'s `a1`) (`?`).
pub(crate) const LOBBY_PINGREQ_NAME: u32 = 0x18;

// ---------------------------------------------------------------------------
// NetGameLink* (link-object lifecycle and ring-buffer I/O).
//
// Link-object words read by more than one port. The buffers are shared with
// the IOP and accessed through uncached (`0x2000_0000 |`) addresses. Names
// are provisional, inferred from use.

/// `+0x04`: the stats state: -1 never fetched, 0 idle, else a request in
/// flight (`?`).
pub(crate) const NET_GAME_LINK_STATS_STATE: u32 = 0x04;

/// `+0x08`: the stat area `NetGameLinkStatus` returns, copied from the stats
/// block (`?`).
pub(crate) const NET_GAME_LINK_STAT_AREA: u32 = 0x08;

/// The stats block's size in bytes.
pub(crate) const NET_GAME_LINK_STATS_SIZE: u64 = 0x150;

/// `+0x158`: the 0x150-byte stats block the IOP fills (`?`).
pub(crate) const NET_GAME_LINK_STATS: u32 = 0x158;

/// `+0x160`: receive read cursor, a byte offset into the receive buffer
/// (`?`).
pub(crate) const NET_GAME_LINK_RECV_READ: u32 = 0x160;

/// `+0x164`: receive write cursor; negative while the buffer needs a refill
/// (`?`).
pub(crate) const NET_GAME_LINK_RECV_WRITE: u32 = 0x164;

/// `+0x168`: receive buffer size in bytes (`?`).
pub(crate) const NET_GAME_LINK_RECV_SIZE: u32 = 0x168;

/// `+0x16c`: the receive buffer (`?`).
pub(crate) const NET_GAME_LINK_RECV_BUF: u32 = 0x16c;

/// `+0x170`: the tick the receive buffer was last handed back (`?`).
pub(crate) const NET_GAME_LINK_RECV_TICK: u32 = 0x170;

/// `+0x174`: send window size in bytes (`?`).
pub(crate) const NET_GAME_LINK_SEND_SIZE: u32 = 0x174;

/// `+0x15c`: the sequence number of the last receive block taken (`?`).
pub(crate) const NET_GAME_LINK_RECV_SEQ: u32 = 0x15c;

/// `+0x178`: bytes sent so far (`?`).
pub(crate) const NET_GAME_LINK_SEND_SENT: u32 = 0x178;

/// `+0x17c`: the word where the IOP counts the sent bytes it has consumed
/// (`?`).
pub(crate) const NET_GAME_LINK_SEND_DONE: u32 = 0x17c;

/// `NetGameLink* NetGameLinkCreate(int ctx, int unused, int size)` (0x3e66f0):
/// allocate a `size + 0x394`-byte link object and zero it, carve the
/// 64-byte-aligned receive buffer (`size` bytes), stats block and
/// sent-bytes word out of it, store `size` as the receive and send sizes,
/// mark the stats never fetched (`+4 = -1`) and issue the `gmlk/crea` RPC
/// with `ctx` and `size`. Returns the object.
pub fn net_game_link_create(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 66;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S2, 0x40);
        frame.save(c, S1, 0x50);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, S3, 0x30);
        c.g[S1][0] = sx((c.g[S2][0] as u32).wrapping_add(0x394));
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S0, 0x60);
        frame.save_ra(c, 0x20);
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, callee::NET_RPC_ALLOC.addr, 0x003e_671c)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A2][0] = c.g[S1][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0u64;
        call_ptr(c, callee::MEMSET.addr, 0x003e_6730)?;
        c.g[V1][0] = 0xffffffffffffffc0;
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1bf));
        c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
        c.g[A0][0] = 0xffffffffffffffff;
        c.sw(
            (c.g[S0][0] as u32).wrapping_add(NET_GAME_LINK_RECV_BUF),
            c.g[V0][0],
        );
        c.g[A1][0] = NET_GAME_LINK_STATS_SIZE;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S2][0] as u32));
        c.sw(
            (c.g[S0][0] as u32).wrapping_add(NET_GAME_LINK_STATS_STATE),
            c.g[A0][0],
        );
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3f));
        c.sw(
            (c.g[S0][0] as u32).wrapping_add(NET_GAME_LINK_RECV_SIZE),
            c.g[S2][0],
        );
        c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
        c.sw(
            (c.g[S0][0] as u32).wrapping_add(NET_GAME_LINK_SEND_SIZE),
            c.g[S2][0],
        );
        c.sw(
            (c.g[S0][0] as u32).wrapping_add(NET_GAME_LINK_STATS),
            c.g[V0][0],
        );
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x18f));
        c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(NET_GAME_LINK_STATS));
        c.sw(
            (c.g[S0][0] as u32).wrapping_add(NET_GAME_LINK_SEND_DONE),
            c.g[V0][0],
        );
        callee::DIRTY_RPC_SYNC_DCACHE.call(c, 0x003e_6778)?;
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(NET_GAME_LINK_RECV_BUF));
        c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(NET_GAME_LINK_RECV_SIZE));
        callee::DIRTY_RPC_SYNC_DCACHE.call(c, 0x003e_6784)?;
        c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(NET_GAME_LINK_SEND_DONE));
        c.g[A1][0] = 0x4;
        callee::DIRTY_RPC_SYNC_DCACHE.call(c, 0x003e_6790)?;
        c.g[A1][0] = 0x676d0000;
        c.g[A2][0] = 0x63720000;
        c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[A1][0] = c.g[A1][0] | 0x6c6b;
        c.g[A2][0] = c.g[A2][0] | 0x6561;
        callee::DIRTY_RPC_INIT_REQUEST.call(c, 0x003e_67a8)?;
        c.g[V1][0] = 0x1;
        c.g[A2][0] = c.g[S3][0];
        c.g[A3][0] = c.g[S2][0];
        c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[S0][0]);
        c.g[T0][0] = sx((c.g[S0][0] as u32).wrapping_add(NET_GAME_LINK_SEND_DONE));
        c.sw(c.g[SP][0] as u32, c.g[V1][0]);
        c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[A1][0] = 0x1;
        c.g[T1][0] = 0x4;
        c.g[T2][0] = 0u64;
        c.g[T3][0] = 0u64;
        callee::DIRTY_RPC_CALL.call(c, 0x003e_67d8)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x50);
        frame.restore(c, S0, 0x60);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `void NetGameLinkDestroy(NetGameLink* link)` (0x3e67f8): issue the
/// `gmlk/dest` RPC with the link's handle word, then free the object.
pub fn net_game_link_destroy(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 32;
        let frame = Frame::open(c, 0x40);
        c.g[A1][0] = 0x676d0000;
        frame.save(c, S0, 0x30);
        c.g[A2][0] = 0x64650000;
        frame.save(c, S1, 0x20);
        c.g[S0][0] = RPC_REQUESTS_PAGE;
        c.g[S1][0] = c.g[A0][0];
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1168));
        frame.save_ra(c, 0x10);
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[A1][0] | 0x6c6b;
        c.g[A2][0] = c.g[A2][0] | 0x7374;
        callee::DIRTY_RPC_INIT_REQUEST.call(c, 0x003e_682c)?;
        c.g[A2][0] = c.lw(c.g[S1][0] as u32);
        c.g[V0][0] = 0x1;
        c.g[A0][0] = c.g[S0][0];
        c.g[A3][0] = 0u64;
        c.g[T0][0] = 0u64;
        c.g[T1][0] = 0u64;
        c.g[T2][0] = 0u64;
        c.g[T3][0] = 0u64;
        c.sw(c.g[SP][0] as u32, c.g[V0][0]);
        c.g[A1][0] = 0u64;
        c.sw((c.g[SP][0] as u32).wrapping_add(0x8), 0u64);
        callee::DIRTY_RPC_CALL.call(c, 0x003e_685c)?;
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, callee::NET_RPC_FREE.addr, 0x003e_6864)?;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `NetGameLink_CopyStats` (0x3e6878): the stats RPC's completion step.
/// Takes the link from the caller's `0(sp)` word (the RPC callback's
/// argument; nothing in `a0`) and copies 0x150 bytes from its stats
/// block (uncached) to the stat area at `link + 8`, with unaligned loads and
/// stores when either address is not 8-byte aligned. Clears `+4` (no stats
/// request in flight). Returns 0.
pub fn net_game_link_copy_stats(c: &mut Cpu) {
    c.cycles += 12;
    c.g[A2][0] = c.lw(c.g[SP][0] as u32);
    c.g[V0][0] = 0xfff0000;
    c.g[V0][0] = c.g[V0][0] | 0xffff;
    c.g[A0][0] = 0x20000000;
    c.g[V1][0] = c.lw((c.g[A2][0] as u32).wrapping_add(NET_GAME_LINK_STATS));
    c.g[A1][0] = sx((c.g[A2][0] as u32).wrapping_add(NET_GAME_LINK_STAT_AREA));
    c.g[V1][0] = c.g[V1][0] & c.g[V0][0];
    c.g[V1][0] = c.g[V1][0] | c.g[A0][0];
    c.g[V0][0] = c.g[V1][0] | c.g[A1][0];
    c.g[V0][0] = c.g[V0][0] & 7;
    let aligned = c.g[V0][0] == 0;
    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x140));
    if !aligned {
        loop {
            c.cycles += 19;
            c.ldl(4, (c.g[V1][0] as u32).wrapping_add(0x7));
            c.ldr(4, c.g[V1][0] as u32);
            c.ldl(7, (c.g[V1][0] as u32).wrapping_add(0xf));
            c.ldr(7, (c.g[V1][0] as u32).wrapping_add(0x8));
            c.ldl(8, (c.g[V1][0] as u32).wrapping_add(0x17));
            c.ldr(8, (c.g[V1][0] as u32).wrapping_add(0x10));
            c.ldl(9, (c.g[V1][0] as u32).wrapping_add(0x1f));
            c.ldr(9, (c.g[V1][0] as u32).wrapping_add(0x18));
            c.sdl(4, (c.g[A1][0] as u32).wrapping_add(0x7));
            c.sdr(4, c.g[A1][0] as u32);
            c.sdl(7, (c.g[A1][0] as u32).wrapping_add(0xf));
            c.sdr(7, (c.g[A1][0] as u32).wrapping_add(0x8));
            c.sdl(8, (c.g[A1][0] as u32).wrapping_add(0x17));
            c.sdr(8, (c.g[A1][0] as u32).wrapping_add(0x10));
            c.sdl(9, (c.g[A1][0] as u32).wrapping_add(0x1f));
            c.sdr(9, (c.g[A1][0] as u32).wrapping_add(0x18));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
            let more = c.g[V1][0] != c.g[V0][0];
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
            if !more {
                c.cycles += 2;
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003e_68a8);
            }
        }
    } else {
        loop {
            c.cycles += 11;
            c.g[A0][0] = c.ld(c.g[V1][0] as u32);
            c.g[A3][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x8));
            c.g[T0][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x10));
            c.g[T1][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x18));
            c.sd(c.g[A1][0] as u32, c.g[A0][0]);
            c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[A3][0]);
            c.sd((c.g[A1][0] as u32).wrapping_add(0x10), c.g[T0][0]);
            c.sd((c.g[A1][0] as u32).wrapping_add(0x18), c.g[T1][0]);
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
            let more = c.g[V1][0] != c.g[V0][0];
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003e_68fc);
            }
        }
    }
    c.cycles += 11;
    c.ldl(2, (c.g[V1][0] as u32).wrapping_add(0x7));
    c.ldr(2, c.g[V1][0] as u32);
    c.ldl(4, (c.g[V1][0] as u32).wrapping_add(0xf));
    c.ldr(4, (c.g[V1][0] as u32).wrapping_add(0x8));
    c.sdl(2, (c.g[A1][0] as u32).wrapping_add(0x7));
    c.sdr(2, c.g[A1][0] as u32);
    c.sdl(4, (c.g[A1][0] as u32).wrapping_add(0xf));
    c.sdr(4, (c.g[A1][0] as u32).wrapping_add(0x8));
    c.g[V0][0] = 0u64;
    c.sw(
        (c.g[A2][0] as u32).wrapping_add(NET_GAME_LINK_STATS_STATE),
        0u64,
    );
    ret(c);
}

/// `long NetGameLinkStatus(NetGameLink* link)` (0x3e6958): refresh the
/// link's stat area and return `link + 8`. The word at `+4` is the stats
/// state: -1 never fetched, 0 idle, anything else a request in flight
/// (returns at once). Otherwise it is set to 1 and the `gmlk/stat` RPC
/// issued: on the first call blocking, followed by `NetGameLink_CopyStats`
/// directly; when idle asynchronous, with `NetGameLink_CopyStats` registered
/// as the completion callback (link as its argument), which clears `+4`.
pub fn net_game_link_status(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x60);
        c.g[V0][0] = 0xffffffffffffffff;
        frame.save(c, S0, 0x50);
        frame.save(c, S3, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x40);
        frame.save(c, S2, 0x30);
        frame.save_ra(c, 0x10);
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(NET_GAME_LINK_STATS_STATE));
        let first = c.g[V1][0] == c.g[V0][0];
        c.g[S3][0] = 0u64;
        if first {
            c.cycles += 2;
            c.sw(
                (c.g[S0][0] as u32).wrapping_add(NET_GAME_LINK_STATS_STATE),
                0u64,
            );
            c.g[S3][0] = 0x1;
        }
        c.cycles += 3;
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(NET_GAME_LINK_STATS_STATE));
        let busy = c.g[V0][0] != 0;
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(NET_GAME_LINK_STAT_AREA));
        if !busy {
            c.cycles += 12;
            c.g[S2][0] = 0x1;
            c.g[V0][0] = RPC_REQUESTS_PAGE;
            c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1158));
            c.g[A1][0] = 0x676d0000;
            c.g[A2][0] = 0x73740000;
            c.sw(
                (c.g[S0][0] as u32).wrapping_add(NET_GAME_LINK_STATS_STATE),
                c.g[S2][0],
            );
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = c.g[A1][0] | 0x6c6b;
            c.g[A2][0] = c.g[A2][0] | 0x6174;
            callee::DIRTY_RPC_INIT_REQUEST.call(c, 0x003e_69c0)?;
            // s3 (callee-saved) still says whether this is the first call.
            let blocking = c.g[S3][0] != 0;
            c.g[A2][0] = c.lw(c.g[S0][0] as u32);
            if !blocking {
                c.cycles += 14;
                c.g[V0][0] = 0x3e0000;
                c.g[T2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(NET_GAME_LINK_STATS));
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x6878));
                c.sw(c.g[SP][0] as u32, c.g[V0][0]);
                c.g[A0][0] = c.g[S1][0];
                c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[S0][0]);
                c.g[A1][0] = 0u64;
                c.g[A3][0] = 0u64;
                c.g[T0][0] = 0u64;
                c.g[T1][0] = 0u64;
                c.g[T3][0] = NET_GAME_LINK_STATS_SIZE;
                callee::DIRTY_RPC_CALL.call(c, 0x003e_69f8)?;
                c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(NET_GAME_LINK_STAT_AREA));
            } else {
                c.cycles += 21;
                c.g[A0][0] = c.g[S1][0];
                c.g[T2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(NET_GAME_LINK_STATS));
                c.g[A1][0] = 0x1;
                c.g[A3][0] = 0u64;
                c.g[T0][0] = 0u64;
                c.g[T1][0] = 0u64;
                c.g[T3][0] = NET_GAME_LINK_STATS_SIZE;
                c.sw(c.g[SP][0] as u32, c.g[S2][0]);
                c.sw((c.g[SP][0] as u32).wrapping_add(0x8), 0u64);
                callee::DIRTY_RPC_CALL.call(c, 0x003e_6a28)?;
                c.sw(c.g[SP][0] as u32, c.g[S0][0]);
                c.g[A0][0] = 0u64;
                c.g[A1][0] = 0u64;
                c.g[A2][0] = 0u64;
                c.g[A3][0] = 0u64;
                c.g[T0][0] = 0u64;
                c.g[T1][0] = 0u64;
                c.g[T2][0] = 0u64;
                c.g[T3][0] = 0u64;
                callee::NET_GAME_LINK_COPY_STATS.call(c, 0x003e_6a50)?;
                c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(NET_GAME_LINK_STAT_AREA));
            }
        }
        c.cycles += 7;
        frame.restore(c, S1, 0x40);
        frame.restore(c, S0, 0x50);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `long NetGameLink_PeekPacket(NetGameLink* link, void** out)` (0x3e6a70):
/// when a packet is queued at the read cursor, copy the whole packet
/// (length in its first halfword, header included) from the receive buffer
/// to the start of `*out`; when none is queued set `*out = NULL`; with no
/// `out` store nothing. The read cursor is not advanced. Returns the queued
/// byte count (write cursor minus read cursor).
pub fn net_game_link_peek_packet(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x20);
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        let have_out = c.g[A1][0] != 0;
        c.g[S0][0] = c.g[A0][0];
        'peeked: {
            if have_out {
                c.cycles += 5;
                c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(NET_GAME_LINK_RECV_READ));
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(NET_GAME_LINK_RECV_WRITE));
                c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                let empty = c.g[V0][0] == 0;
                c.g[V1][0] = 0xfff0000;
                if !empty {
                    c.cycles += 21;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(NET_GAME_LINK_RECV_BUF));
                    c.g[V1][0] = c.g[V1][0] | 0xffff;
                    c.g[A3][0] = 0x20000000;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.g[A0][0] = c.lw(c.g[A1][0] as u32);
                    c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
                    c.g[V0][0] = c.g[V0][0] | c.g[A3][0];
                    c.lwl(6, (c.g[V0][0] as u32).wrapping_add(0x3));
                    c.lwr(6, c.g[V0][0] as u32);
                    c.swl(6, (c.g[A0][0] as u32).wrapping_add(0x3));
                    c.swr(6, c.g[A0][0] as u32);
                    c.g[A0][0] = c.lw(c.g[A1][0] as u32);
                    c.g[A1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(NET_GAME_LINK_RECV_BUF));
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(NET_GAME_LINK_RECV_READ));
                    c.g[A2][0] = c.lh(c.g[A0][0] as u32);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[A1][0] = c.g[A1][0] & c.g[V1][0];
                    c.g[A1][0] = c.g[A1][0] | c.g[A3][0];
                    call_ptr(c, callee::MEMCPY.addr, 0x003e_6ae4)?;
                    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(NET_GAME_LINK_RECV_READ));
                    break 'peeked;
                }
                c.cycles += 1;
                c.sw(c.g[A1][0] as u32, 0u64);
            }
            c.cycles += 1;
            c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(NET_GAME_LINK_RECV_READ));
        }
        c.cycles += 6;
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(NET_GAME_LINK_RECV_WRITE));
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
        frame.close(c);
        Ok(())
    })
}

/// `long NetGameLinkRecv(NetGameLink* link, void* out, long maxlen)`
/// (0x3e6b10): drain queued bytes into `out`. With a negative write cursor
/// the receive buffer is checked first (`NetGameLink_CheckRecvBuf`): a
/// block with a new sequence number sets the cursors to its data, the
/// sequence already taken leaves them alone. With `maxlen == 1` one packet
/// is peeked into `out` and the read cursor advanced past it; otherwise all
/// queued bytes are copied when they fit in `maxlen`, none when they do
/// not. When the cursors meet, the buffer is handed back (cursors reset,
/// the tick stored at `+0x170`, the `gmlk/recv` RPC). Returns the bytes
/// moved (or 0).
pub fn net_game_link_recv(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x50);
        frame.save(c, S1, 0x40);
        c.g[S0][0] = c.g[A2][0];
        frame.save(c, S2, 0x30);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0x20);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x14), c.g[A1][0]);
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(NET_GAME_LINK_RECV_WRITE));
        let refill = (c.g[V0][0] as i64) < 0;
        c.g[S2][0] = 0u64;
        if refill {
            c.cycles += 6;
            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(NET_GAME_LINK_RECV_BUF));
            c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
            callee::NET_GAME_LINK_CHECK_RECV_BUF.call(c, 0x003e_6b48)?;
            c.g[A0][0] = c.g[V0][0];
            if (c.g[A0][0] as i64) < 0 {
                // Refused: rejoin at the drain check (no tail charge).
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(NET_GAME_LINK_RECV_READ));
            } else {
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(NET_GAME_LINK_RECV_SEQ));
                let same = c.g[A0][0] == c.g[V0][0];
                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
                if !same {
                    c.cycles += 5;
                    c.g[V0][0] = 0x10;
                    c.sw(
                        (c.g[S1][0] as u32).wrapping_add(NET_GAME_LINK_RECV_READ),
                        c.g[V0][0],
                    );
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x10));
                    c.sw(
                        (c.g[S1][0] as u32).wrapping_add(NET_GAME_LINK_RECV_SEQ),
                        c.g[A0][0],
                    );
                    c.sw(
                        (c.g[S1][0] as u32).wrapping_add(NET_GAME_LINK_RECV_WRITE),
                        c.g[V1][0],
                    );
                }
                // Both refill arms rejoin through the inner break: reload the
                // read cursor (the refill arm just stored it).
                c.cycles += 1;
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(NET_GAME_LINK_RECV_READ));
            }
        } else {
            c.cycles += 1;
            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(NET_GAME_LINK_RECV_READ));
        }
        c.cycles += 4;
        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(NET_GAME_LINK_RECV_WRITE));
        c.g[V0][0] = ((c.g[V1][0] as i64) < (c.g[A0][0] as i64)) as u64;
        let drained = c.g[V0][0] == 0;
        c.g[V0][0] = 0x1;
        if !drained {
            c.cycles += 2;
            let peek = c.g[S0][0] == c.g[V0][0];
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
            if peek {
                c.cycles += 9;
                c.g[A0][0] = c.g[S1][0];
                c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x14));
                callee::NET_GAME_LINK_PEEK_PACKET.call(c, 0x003e_6b9c)?;
                c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14));
                c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(NET_GAME_LINK_RECV_READ));
                c.g[S2][0] = c.lh(c.g[V1][0] as u32);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S2][0] as u32));
                c.sw(
                    (c.g[S1][0] as u32).wrapping_add(NET_GAME_LINK_RECV_READ),
                    c.g[V0][0],
                );
                // Peek arm rejoins through the cursor-reload tail.
                c.cycles += 1;
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(NET_GAME_LINK_RECV_READ));
            } else {
                c.cycles += 3;
                c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[A0][0] as i64)) as u64;
                if c.g[V0][0] != 0 {
                    // Too much queued for `maxlen`: rejoin at the sync check
                    // (no tail charge).
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(NET_GAME_LINK_RECV_READ));
                } else {
                    c.cycles += 13;
                    c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(NET_GAME_LINK_RECV_BUF));
                    c.g[V0][0] = 0xfff0000;
                    c.g[S2][0] = c.g[A0][0];
                    c.g[V0][0] = c.g[V0][0] | 0xffff;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14));
                    c.g[V1][0] = 0x20000000;
                    c.g[A1][0] = c.g[A1][0] & c.g[V0][0];
                    c.g[A1][0] = c.g[A1][0] | c.g[V1][0];
                    c.g[A2][0] = c.g[S2][0];
                    call_ptr(c, callee::MEMCPY.addr, 0x003e_6bec)?;
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(NET_GAME_LINK_RECV_WRITE));
                    c.sw(
                        (c.g[S1][0] as u32).wrapping_add(NET_GAME_LINK_RECV_READ),
                        c.g[V1][0],
                    );
                    // Memcpy arm rejoins through the cursor-reload tail.
                    c.cycles += 1;
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(NET_GAME_LINK_RECV_READ));
                }
            }
        } else {
            c.cycles += 1;
            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(NET_GAME_LINK_RECV_READ));
        }
        c.cycles += 3;
        c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(NET_GAME_LINK_RECV_WRITE));
        let synced = c.g[V1][0] == c.g[V0][0];
        c.g[V0][0] = c.g[S2][0];
        if synced {
            c.cycles += 25;
            c.g[V0][0] = 0xffffffffffffffff;
            c.sw(
                (c.g[S1][0] as u32).wrapping_add(NET_GAME_LINK_RECV_READ),
                0u64,
            );
            c.sw(
                (c.g[S1][0] as u32).wrapping_add(NET_GAME_LINK_RECV_WRITE),
                c.g[V0][0],
            );
            callee::NET_TICK.call(c, 0x003e_6c14)?;
            c.g[S0][0] = RPC_REQUESTS_PAGE;
            c.g[A1][0] = 0x676d0000;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1148));
            c.g[A2][0] = 0x72650000;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[A1][0] | 0x6c6b;
            c.g[A2][0] = c.g[A2][0] | 0x6376;
            c.sw(
                (c.g[S1][0] as u32).wrapping_add(NET_GAME_LINK_RECV_TICK),
                c.g[V0][0],
            );
            callee::DIRTY_RPC_INIT_REQUEST.call(c, 0x003e_6c38)?;
            c.g[A3][0] = c.lw((c.g[S1][0] as u32).wrapping_add(NET_GAME_LINK_RECV_BUF));
            c.g[A0][0] = c.g[S0][0];
            c.g[A2][0] = c.lw(c.g[S1][0] as u32);
            c.g[A1][0] = 0x1;
            c.sw(c.g[SP][0] as u32, 0u64);
            c.g[T0][0] = 0u64;
            c.sw((c.g[SP][0] as u32).wrapping_add(0x8), 0u64);
            c.g[T1][0] = 0u64;
            c.g[T2][0] = 0u64;
            c.g[T3][0] = 0u64;
            callee::DIRTY_RPC_CALL.call(c, 0x003e_6c64)?;
            c.g[V0][0] = c.g[S2][0];
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

/// `long NetGameLinkSend(NetGameLink* link, void* buf, long len)`
/// (0x3e6c80): queue `len` bytes from `buf` for sending. With `len == 1`
/// `buf` is a packet: `len` becomes `(buf[3] + 0x17) & 0x7ffc` (rounded
/// down to a multiple of 4), written back to its first halfword. When the
/// send window has room (window size minus the bytes sent and not yet
/// consumed by the IOP is at least `len`), add `len` to the sent count at
/// `+0x178`, issue the `gmlk/send` RPC with `buf` and `len` and return 1;
/// else return 0.
pub fn net_game_link_send(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x70);
        c.g[V0][0] = 0x1;
        frame.save(c, S1, 0x50);
        frame.save(c, S2, 0x40);
        c.g[S1][0] = c.g[A2][0];
        frame.save(c, S3, 0x30);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0x60);
        c.g[S3][0] = c.g[A1][0];
        let fixup = c.g[S1][0] == c.g[V0][0];
        frame.save_ra(c, 0x20);
        if fixup {
            c.cycles += 4;
            c.g[V0][0] = c.lbu((c.g[S3][0] as u32).wrapping_add(0x3));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x17));
            c.g[S1][0] = c.g[V0][0] & 0x7ffc;
            c.sh(c.g[S3][0] as u32, c.g[S1][0]);
        }
        c.cycles += 18;
        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(NET_GAME_LINK_SEND_DONE));
        c.g[V0][0] = 0xfff0000;
        c.g[V0][0] = c.g[V0][0] | 0xffff;
        c.g[A1][0] = 0x20000000;
        c.g[A0][0] = c.g[A0][0] & c.g[V0][0];
        c.g[A2][0] = c.lw((c.g[S2][0] as u32).wrapping_add(NET_GAME_LINK_SEND_SENT));
        c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(NET_GAME_LINK_SEND_SIZE));
        c.g[A0][0] = c.g[A0][0] | c.g[A1][0];
        c.lwl(2, (c.g[A0][0] as u32).wrapping_add(0x3));
        c.lwr(2, c.g[A0][0] as u32);
        c.swl(2, (c.g[SP][0] as u32).wrapping_add(0x13));
        c.swr(2, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_sub(c.g[V0][0] as u32));
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
        c.g[V1][0] = ((c.g[V1][0] as i64) < (c.g[S1][0] as i64)) as u64;
        if c.g[V1][0] != 0 {
            c.g[V0][0] = 0u64;
        } else {
            c.cycles += 22;
            c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[S1][0] as u32));
            c.g[S0][0] = RPC_REQUESTS_PAGE;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1138));
            c.g[A1][0] = 0x676d0000;
            c.g[A2][0] = 0x73650000;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[A1][0] | 0x6c6b;
            c.g[A2][0] = c.g[A2][0] | 0x6e64;
            c.sw(
                (c.g[S2][0] as u32).wrapping_add(NET_GAME_LINK_SEND_SENT),
                c.g[V0][0],
            );
            callee::DIRTY_RPC_INIT_REQUEST.call(c, 0x003e_6d2c)?;
            c.g[A2][0] = c.lw(c.g[S2][0] as u32);
            c.g[A0][0] = c.g[S0][0];
            c.g[T0][0] = c.g[S3][0];
            c.g[T1][0] = c.g[S1][0];
            c.sw(c.g[SP][0] as u32, 0u64);
            c.g[A1][0] = 0x1;
            c.sw((c.g[SP][0] as u32).wrapping_add(0x8), 0u64);
            c.g[A3][0] = 0u64;
            c.g[T2][0] = 0u64;
            c.g[T3][0] = 0u64;
            callee::DIRTY_RPC_CALL.call(c, 0x003e_6d58)?;
            c.g[V0][0] = 0x1;
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

/// `long NetGameLinkControl(NetGameLink* link, long op, long arg)`
/// (0x3e6d78): issue the `gmlk/ctrl` RPC with the link word, `op` and a
/// pointer to `arg`. Returns the reply word the RPC left on the frame.
pub fn net_game_link_control(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 36;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x50);
        frame.save(c, S2, 0x30);
        c.g[S0][0] = RPC_REQUESTS_PAGE;
        frame.save(c, S1, 0x40);
        c.g[S2][0] = c.g[A1][0];
        c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[A2][0]);
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1128));
        c.g[S1][0] = c.g[A0][0];
        c.g[A1][0] = 0x676d0000;
        c.g[A2][0] = 0x63740000;
        frame.save_ra(c, 0x20);
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[A1][0] | 0x6c6b;
        c.g[A2][0] = c.g[A2][0] | 0x726c;
        callee::DIRTY_RPC_INIT_REQUEST.call(c, 0x003e_6db8)?;
        c.g[A2][0] = c.lw(c.g[S1][0] as u32);
        c.g[V0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x14));
        c.g[V1][0] = 0x1;
        c.g[A0][0] = c.g[S0][0];
        c.g[A3][0] = c.g[S2][0];
        c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V0][0]);
        c.sw(c.g[SP][0] as u32, c.g[V1][0]);
        c.g[A1][0] = 0x1;
        c.g[T0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[T1][0] = 0x4;
        c.g[T2][0] = 0u64;
        c.g[T3][0] = 0u64;
        callee::DIRTY_RPC_CALL.call(c, 0x003e_6dec)?;
        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14));
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// NetGameUtil* (matchmaking RPC wrappers).

/// `long NetGameUtilCreate()` (0x3e6e08): issue the `gmut/crea` RPC.
/// Returns the reply word the RPC left on the frame.
pub fn net_game_util_create(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 25;
        let frame = Frame::open(c, 0x40);
        c.g[A1][0] = 0x676d0000;
        c.g[A2][0] = 0x63720000;
        frame.save_ra(c, 0x30);
        c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[A1][0] = c.g[A1][0] | 0x7574;
        c.g[A2][0] = c.g[A2][0] | 0x6561;
        callee::DIRTY_RPC_INIT_REQUEST.call(c, 0x003e_6e28)?;
        c.g[V1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[V0][0] = 0x1;
        c.sw(c.g[SP][0] as u32, c.g[V0][0]);
        c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V1][0]);
        c.g[A1][0] = 0x1;
        c.g[A2][0] = 0u64;
        c.g[A3][0] = 0u64;
        c.g[T0][0] = 0u64;
        c.g[T1][0] = 0u64;
        c.g[T2][0] = 0u64;
        c.g[T3][0] = 0u64;
        callee::DIRTY_RPC_CALL.call(c, 0x003e_6e5c)?;
        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
        frame.restore_ra(c, 0x30);
        frame.close(c);
        Ok(())
    })
}

/// The body of the `NetGameUtil*` calls that issue `gmut/<tag>` for `ctx` (`a0`)
/// and read no reply; `entry` is the port's address.
fn net_game_util_ctx_rpc(c: &mut Cpu, entry: u32, tag: u32) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x40);
        c.g[A1][0] = 0x676d0000;
        frame.save(c, S0, 0x30);
        c.g[A2][0] = u64::from(tag & 0xffff_0000);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x20);
        c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[A1][0] = c.g[A1][0] | 0x7574;
        c.g[A2][0] = c.g[A2][0] | u64::from(tag & 0xffff);
        callee::DIRTY_RPC_INIT_REQUEST.call(c, entry + 0x28)?;
        c.g[V1][0] = 0x1;
        c.g[A2][0] = c.g[S0][0];
        c.sw(c.g[SP][0] as u32, c.g[V1][0]);
        c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x8), 0u64);
        c.g[A1][0] = 0x1;
        c.g[A3][0] = 0u64;
        c.g[T0][0] = 0u64;
        c.g[T1][0] = 0u64;
        c.g[T2][0] = 0u64;
        c.g[T3][0] = 0u64;
        callee::DIRTY_RPC_CALL.call(c, entry + 0x58)?;
        frame.restore(c, S0, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `void NetGameUtilDestroy(long ctx)` (0x3e6e70): issue the `gmut/dest`
/// RPC for `ctx`.
pub fn net_game_util_destroy(c: &mut Cpu) {
    net_game_util_ctx_rpc(c, 0x003e_6e70, u32::from_be_bytes(*b"dest"));
}

/// `void NetGameUtilReset(long ctx)` (0x3e6ed8): issue the `gmut/rset`
/// RPC for `ctx`.
pub fn net_game_util_reset(c: &mut Cpu) {
    net_game_util_ctx_rpc(c, 0x003e_6ed8, u32::from_be_bytes(*b"rset"));
}

/// `long NetGameUtilConnect(long a, long b, long c)` (0x3e6f40): issue the
/// `gmut/conn` RPC. Returns the reply word the RPC left on the frame.
pub fn net_game_util_connect(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 34;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S1, 0x50);
        frame.save(c, S2, 0x40);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S0, 0x60);
        c.g[S2][0] = c.g[A2][0];
        c.g[S0][0] = c.g[A0][0];
        c.g[A1][0] = 0x676d0000;
        c.g[A2][0] = 0x636f0000;
        frame.save_ra(c, 0x30);
        c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[A1][0] = c.g[A1][0] | 0x7574;
        c.g[A2][0] = c.g[A2][0] | 0x6e6e;
        callee::DIRTY_RPC_INIT_REQUEST.call(c, 0x003e_6f78)?;
        c.g[V1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[V0][0] = 0x1;
        c.g[A2][0] = c.g[S0][0];
        c.g[A3][0] = c.g[S1][0];
        c.g[T0][0] = c.g[S2][0];
        c.sw(c.g[SP][0] as u32, c.g[V0][0]);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V1][0]);
        c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[A1][0] = 0x1;
        c.g[T1][0] = 0xffffffffffffffff;
        c.g[T2][0] = 0u64;
        c.g[T3][0] = 0u64;
        callee::DIRTY_RPC_CALL.call(c, 0x003e_6fac)?;
        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore_ra(c, 0x30);
        frame.close(c);
        Ok(())
    })
}

/// `long NetGameUtilComplete(long ctx)` (0x3e6fc8): issue the `gmut/comp`
/// RPC. Returns the reply word the RPC left on the frame.
pub fn net_game_util_complete(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 28;
        let frame = Frame::open(c, 0x50);
        c.g[A1][0] = 0x676d0000;
        frame.save(c, S0, 0x40);
        c.g[A2][0] = 0x636f0000;
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x30);
        c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[A1][0] = c.g[A1][0] | 0x7574;
        c.g[A2][0] = c.g[A2][0] | 0x6d70;
        callee::DIRTY_RPC_INIT_REQUEST.call(c, 0x003e_6ff0)?;
        c.g[V1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[V0][0] = 0x1;
        c.g[A2][0] = c.g[S0][0];
        c.sw(c.g[SP][0] as u32, c.g[V0][0]);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V1][0]);
        c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[A1][0] = 0x1;
        c.g[A3][0] = 0u64;
        c.g[T0][0] = 0u64;
        c.g[T1][0] = 0u64;
        c.g[T2][0] = 0u64;
        c.g[T3][0] = 0u64;
        callee::DIRTY_RPC_CALL.call(c, 0x003e_7024)?;
        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
        frame.restore(c, S0, 0x40);
        frame.restore_ra(c, 0x30);
        frame.close(c);
        Ok(())
    })
}

/// `long NetGameUtilStatus(long ctx, long arg)` (0x3e7038): issue the
/// `gmut/stat` RPC with `ctx` and `arg`. Returns the reply word the RPC left
/// on the frame.
pub fn net_game_util_status(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 31;
        let frame = Frame::open(c, 0x60);
        c.g[A2][0] = 0x73740000;
        frame.save(c, S1, 0x40);
        c.g[A2][0] = c.g[A2][0] | 0x6174;
        frame.save(c, S0, 0x50);
        c.g[S1][0] = c.g[A1][0];
        c.g[S0][0] = c.g[A0][0];
        c.g[A1][0] = 0x676d0000;
        frame.save_ra(c, 0x30);
        c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[A1][0] = c.g[A1][0] | 0x7574;
        callee::DIRTY_RPC_INIT_REQUEST.call(c, 0x003e_7068)?;
        c.g[V1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[V0][0] = 0x1;
        c.g[A2][0] = c.g[S0][0];
        c.g[A3][0] = c.g[S1][0];
        c.sw(c.g[SP][0] as u32, c.g[V0][0]);
        c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V1][0]);
        c.g[A1][0] = 0x1;
        c.g[T0][0] = 0u64;
        c.g[T1][0] = 0u64;
        c.g[T2][0] = 0u64;
        c.g[T3][0] = 0u64;
        callee::DIRTY_RPC_CALL.call(c, 0x003e_709c)?;
        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore_ra(c, 0x30);
        frame.close(c);
        Ok(())
    })
}

/// `void NetGameUtilStat(long ctx)` (0x3e70b8): issue the `gmut/stat` RPC
/// for `ctx` (no reply read back).
pub fn net_game_util_stat(c: &mut Cpu) {
    net_game_util_ctx_rpc(c, 0x003e_70b8, u32::from_be_bytes(*b"stat"));
}

/// `void NetGameUtilAdvert(long ctx, const char* a, const char* b)`
/// (0x3e7120): format `a` and `b` with `"%s\t%s"` and issue the
/// `gmut/advt` RPC for `ctx` with the result.
pub fn net_game_util_advert(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 36;
        let frame = Frame::open(c, 0x150);
        c.g[V0][0] = c.g[A1][0];
        frame.save(c, S1, 0x130);
        c.g[A3][0] = c.g[A2][0];
        c.g[S1][0] = c.g[A0][0];
        c.g[A1][0] = LOBBY_STRINGS_PAGE;
        frame.save(c, S0, 0x140);
        c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        frame.save_ra(c, 0x120);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4860));
        c.g[A2][0] = c.g[V0][0];
        call_ptr(c, callee::SPRINTF.addr, 0x003e_7150)?;
        c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x110));
        c.g[A1][0] = 0x676d0000;
        c.g[A2][0] = 0x61640000;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[A1][0] | 0x7574;
        c.g[A2][0] = c.g[A2][0] | 0x7674;
        callee::DIRTY_RPC_INIT_REQUEST.call(c, 0x003e_716c)?;
        c.g[V1][0] = 0x1;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[S1][0];
        c.sw(c.g[SP][0] as u32, c.g[V1][0]);
        c.sw((c.g[SP][0] as u32).wrapping_add(0x8), 0u64);
        c.g[A1][0] = 0x1;
        c.g[A3][0] = 0u64;
        c.g[T0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[T1][0] = 0xffffffffffffffff;
        c.g[T2][0] = 0u64;
        c.g[T3][0] = 0u64;
        callee::DIRTY_RPC_CALL.call(c, 0x003e_719c)?;
        frame.restore(c, S0, 0x140);
        frame.restore(c, S1, 0x130);
        frame.restore_ra(c, 0x120);
        frame.close(c);
        Ok(())
    })
}

/// `void NetGameUtilWithdraw(long ctx)` (0x3e71b0): issue the `gmut/with`
/// RPC for `ctx`.
pub fn net_game_util_withdraw(c: &mut Cpu) {
    net_game_util_ctx_rpc(c, 0x003e_71b0, u32::from_be_bytes(*b"with"));
}

/// `long NetGameUtilLocate(long ctx, ?, const char* name)` (0x3e7218):
/// issue the `gmut/loca` RPC with `ctx` and `name` (read from `a2`; `a1` is
/// not read); a null `name` falls back to the default string. Returns the
/// reply word the RPC left on the frame.
pub fn net_game_util_locate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x50);
        frame.save(c, S1, 0x40);
        c.g[S0][0] = c.g[A2][0];
        frame.save_ra(c, 0x30);
        let have_name = c.g[S0][0] != 0;
        c.g[S1][0] = c.g[A0][0];
        if !have_name {
            c.cycles += 2;
            c.g[V0][0] = LOBBY_STRINGS_PAGE;
            c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4868));
        }
        c.cycles += 25;
        c.g[A1][0] = 0x676d0000;
        c.g[A2][0] = 0x6c6f0000;
        c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[A1][0] = c.g[A1][0] | 0x7574;
        c.g[A2][0] = c.g[A2][0] | 0x6361;
        callee::DIRTY_RPC_INIT_REQUEST.call(c, 0x003e_7254)?;
        c.g[V1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[V0][0] = 0x1;
        c.g[A2][0] = c.g[S1][0];
        c.g[T0][0] = c.g[S0][0];
        c.sw(c.g[SP][0] as u32, c.g[V0][0]);
        c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V1][0]);
        c.g[A1][0] = 0x1;
        c.g[A3][0] = 0u64;
        c.g[T1][0] = 0xffffffffffffffff;
        c.g[T2][0] = 0u64;
        c.g[T3][0] = 0u64;
        callee::DIRTY_RPC_CALL.call(c, 0x003e_7288)?;
        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore_ra(c, 0x30);
        frame.close(c);
        Ok(())
    })
}

/// `long NetGameUtilQuery(long ctx, const char* name, long a, long b)`
/// (0x3e72a0): issue the `gmut/quer` RPC; a null `name` falls back to the
/// default string. Returns the reply word the RPC left on the frame.
pub fn net_game_util_query(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x80);
        frame.save(c, S0, 0x70);
        frame.save(c, S1, 0x60);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S2, 0x50);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S3, 0x40);
        c.g[S2][0] = c.g[A2][0];
        frame.save_ra(c, 0x30);
        let have_name = c.g[S0][0] != 0;
        c.g[S3][0] = c.g[A3][0];
        if !have_name {
            c.cycles += 2;
            c.g[V0][0] = LOBBY_STRINGS_PAGE;
            c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4868));
        }
        c.cycles += 27;
        c.g[A1][0] = 0x676d0000;
        c.g[A2][0] = 0x71750000;
        c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[A1][0] = c.g[A1][0] | 0x7574;
        c.g[A2][0] = c.g[A2][0] | 0x6572;
        callee::DIRTY_RPC_INIT_REQUEST.call(c, 0x003e_72ec)?;
        c.g[V1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[V0][0] = 0x1;
        c.g[A2][0] = c.g[S1][0];
        c.g[T0][0] = c.g[S0][0];
        c.g[T2][0] = c.g[S2][0];
        c.g[T3][0] = c.g[S3][0];
        c.sw(c.g[SP][0] as u32, c.g[V0][0]);
        c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V1][0]);
        c.g[A1][0] = 0x5;
        c.g[A3][0] = 0u64;
        c.g[T1][0] = 0xffffffffffffffff;
        callee::DIRTY_RPC_CALL.call(c, 0x003e_7320)?;
        c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x20));
        frame.restore(c, S0, 0x70);
        frame.restore(c, S1, 0x60);
        frame.restore(c, S2, 0x50);
        frame.restore(c, S3, 0x40);
        frame.restore_ra(c, 0x30);
        frame.close(c);
        Ok(())
    })
}

/// `ProtoPing* ProtoPingCreate()` (0x3e7340): allocate a 0x854-byte ping
/// object and zero it, store its reply buffer (the first 64-byte boundary
/// at or after `obj + 0x14`) at `obj + 0x10`, and issue the `ping/crea` RPC
/// with its handle reply going to `obj + 0xc`. Returns the object.
pub fn proto_ping_create(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 39;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save_ra(c, 0x20);
        c.g[A0][0] = 0x854;
        call_ptr(c, callee::NET_RPC_ALLOC.addr, 0x003e_7354)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A1][0] = 0u64;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = 0x854;
        call_ptr(c, callee::MEMSET.addr, 0x003e_7368)?;
        c.g[V1][0] = 0xffffffffffff0000;
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x53));
        c.g[V1][0] = c.g[V1][0] | 0xffc0;
        c.g[A1][0] = 0x70690000;
        c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
        c.g[A2][0] = 0x63720000;
        c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[V0][0]);
        c.g[A1][0] = c.g[A1][0] | 0x6e67;
        c.g[A2][0] = c.g[A2][0] | 0x6561;
        callee::DIRTY_RPC_INIT_REQUEST.call(c, 0x003e_7394)?;
        c.g[V1][0] = 0x1;
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xc));
        c.sw(c.g[SP][0] as u32, c.g[V1][0]);
        c.g[A0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[V0][0]);
        c.g[A1][0] = 0x1;
        c.g[A2][0] = 0u64;
        c.g[A3][0] = 0u64;
        c.g[T0][0] = 0u64;
        c.g[T1][0] = 0u64;
        c.g[T2][0] = 0u64;
        c.g[T3][0] = 0u64;
        callee::DIRTY_RPC_CALL.call(c, 0x003e_73c8)?;
        c.g[V0][0] = c.g[S0][0];
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
        frame.restore(c, S0, 0x30);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// ProtoPing* (the ping channel).

/// `void ProtoPingDestroy(ping)` (0x3e73e0): issue the `ping/dest` RPC with
/// the handle at `ping+0xc`, then free the object.
pub fn proto_ping_destroy(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 28;
        let frame = Frame::open(c, 0x40);
        c.g[A1][0] = 0x7069_0000;
        frame.save(c, S0, 0x30);
        c.g[A2][0] = 0x6465_0000;
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x20);
        c.g[A0][0] = sx(frame.at(c, 0x10));
        c.g[A1][0] |= 0x6e67; // 'ping'
        c.g[A2][0] |= 0x7374; // 'dest'
        callee::DIRTY_RPC_INIT_REQUEST.call(c, 0x003e_7408)?;
        // The ping handle at ping+0xc, re-read from s0 after the call.
        c.g[A2][0] = Addr((c.g[S0][0] as u32).wrapping_add(0xc)).lw(c);
        c.g[V0][0] = 0x1;
        c.g[A0][0] = sx(frame.at(c, 0x10));
        c.g[A3][0] = 0;
        c.g[T0][0] = 0;
        c.g[T1][0] = 0;
        c.g[T2][0] = 0;
        c.g[T3][0] = 0;
        c.sw(frame.at(c, 0), c.g[V0][0]);
        c.g[A1][0] = 0;
        c.sw(frame.at(c, 8), 0);
        callee::DIRTY_RPC_CALL.call(c, 0x003e_7438)?;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, callee::NET_RPC_FREE.addr, 0x003e_7440)?;
        frame.restore(c, S0, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `u32 ProtoPingRequest(ping, a1, a2, a3)` (0x3e7450): issue the
/// `ping/requ` RPC with the handle at `ping+0xc` and `a1..a3`, and return
/// the reply word.
pub fn proto_ping_request(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 37;
        let frame = Frame::open(c, 0x80);
        frame.save(c, S2, 0x50);
        frame.save(c, S3, 0x40);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S0, 0x70);
        c.g[S3][0] = c.g[A2][0];
        frame.save(c, S1, 0x60);
        c.g[S0][0] = c.g[A0][0];
        c.g[A1][0] = 0x7069_0000;
        c.g[A2][0] = 0x7265_0000;
        frame.save_ra(c, 0x30);
        c.g[S1][0] = c.g[A3][0];
        c.g[A0][0] = sx(frame.at(c, 0x10));
        c.g[A1][0] |= 0x6e67; // 'ping'
        c.g[A2][0] |= 0x7175; // 'requ'
        callee::DIRTY_RPC_INIT_REQUEST.call(c, 0x003e_7490)?;
        // The ping handle at ping+0xc, re-read from s0 after the call.
        c.g[A2][0] = Addr((c.g[S0][0] as u32).wrapping_add(0xc)).lw(c);
        c.g[V0][0] = sx(frame.at(c, 0x20));
        c.g[V1][0] = 0x1;
        c.g[A3][0] = c.g[S2][0];
        c.g[T0][0] = c.g[S3][0];
        c.g[T1][0] = c.g[S1][0];
        c.sw(frame.at(c, 8), c.g[V0][0]);
        c.g[A0][0] = sx(frame.at(c, 0x10));
        c.sw(frame.at(c, 0), c.g[V1][0]);
        c.g[A1][0] = 0x1;
        c.g[T2][0] = 0;
        c.g[T3][0] = 0;
        callee::DIRTY_RPC_CALL.call(c, 0x003e_74c4)?;
        c.g[V0][0] = c.lw(frame.at(c, 0x20));
        frame.restore(c, S0, 0x70);
        frame.restore(c, S1, 0x60);
        frame.restore(c, S2, 0x50);
        frame.restore(c, S3, 0x40);
        frame.restore_ra(c, 0x30);
        frame.close(c);
        Ok(())
    })
}

/// `u32 ProtoPingResponse(ping, dst, lenp, out)` (0x3e7500): take a
/// queued ping reply and re-arm the reply RPC. When the reply byte count at
/// `+0` is positive: copy the reply's 16-byte header to `out` (if set), cap
/// `*lenp` (if set) at the count minus the header and copy that many bytes
/// after the header to `dst`, then clear the count and `+8`. Then, with no
/// request flagged at `+4` and no reply queued, unless the last re-arm
/// (tick at `+8`) is under 6 ticks old: store the tick, flag `+4`, mark an
/// empty buffer -1 (after invalidating its cache lines) and issue the
/// `ping/resp` RPC into the 0x800-byte reply buffer. Returns the header's
/// first word (0 when nothing was queued).
pub fn proto_ping_response(c: &mut Cpu) {
    guest(c, |c| {
        let frame: Frame;
        c.cycles += 9;
        frame = Frame::open(c, 0x50);
        frame.save(c, S1, 0x30);
        frame.save(c, S0, 0x40);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0x20);
        c.g[A0][0] = c.g[A1][0];
        c.g[V0][0] = Addr(c.g[S1][0] as u32).lw(c);
        let empty = (c.g[V0][0] as i64) <= 0;
        c.sw(frame.at(c, 0x10), 0u64);
        // 0x3e75c4: a non-positive count skips the copy section below.
        if !empty {
            // 0x3e7580: stage the reply word, then the payload when a
            // destination is present.
            c.cycles += 7;
            c.g[V0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x10)).lw(c);
            c.lwl(3, (c.g[V0][0] as u32).wrapping_add(0x3));
            c.lwr(3, c.g[V0][0] as u32);
            c.swl(3, frame.at(c, 0x13));
            c.swr(3, frame.at(c, 0x10));
            let no_out = c.g[A3][0] == 0;
            c.g[V0][0] = c.lw(frame.at(c, 0x10));
            if !no_out {
                c.cycles += 16;
                c.sw(c.g[A3][0] as u32, c.g[V0][0]);
                c.g[V1][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x10)).lw(c);
                c.lwl(2, (c.g[V1][0] as u32).wrapping_add(0x7));
                c.lwr(2, (c.g[V1][0] as u32).wrapping_add(0x4));
                c.swl(2, (c.g[A3][0] as u32).wrapping_add(0x7));
                c.swr(2, (c.g[A3][0] as u32).wrapping_add(0x4));
                c.g[V0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x10)).lw(c);
                c.lwl(3, (c.g[V0][0] as u32).wrapping_add(0xb));
                c.lwr(3, (c.g[V0][0] as u32).wrapping_add(0x8));
                c.swl(3, (c.g[A3][0] as u32).wrapping_add(0xb));
                c.swr(3, (c.g[A3][0] as u32).wrapping_add(0x8));
                c.g[V1][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x10)).lw(c);
                c.lwl(2, (c.g[V1][0] as u32).wrapping_add(0xf));
                c.lwr(2, (c.g[V1][0] as u32).wrapping_add(0xc));
                c.swl(2, (c.g[A3][0] as u32).wrapping_add(0xf));
                c.swr(2, (c.g[A3][0] as u32).wrapping_add(0xc));
            }
            // 0x3e75bc: copy the queued bytes unless the length word is
            // absent, capping at the queued length.
            c.cycles += 2;
            let no_len = c.g[A2][0] == 0;
            if !no_len {
                // 0x3e75b0: cap the copy at the queued length.
                c.cycles += 6;
                c.g[V0][0] = Addr(c.g[S1][0] as u32).lw(c);
                c.g[V1][0] = Addr(c.g[A2][0] as u32).lw(c);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x10));
                c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                if c.g[V0][0] == 0 {
                    c.g[A1][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x10)).lw(c);
                } else {
                    c.cycles += 4;
                    c.g[V0][0] = Addr(c.g[S1][0] as u32).lw(c);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x10));
                    c.sw(c.g[A2][0] as u32, c.g[V0][0]);
                    c.g[A1][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x10)).lw(c);
                }
                c.cycles += 3;
                c.g[A2][0] = Addr(c.g[A2][0] as u32).lw(c);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x10));
                call_ptr(c, callee::MEMCPY.addr, 0x003e_75bc)?;
            }
            c.cycles += 2;
            c.sw(c.g[S1][0] as u32, 0u64);
            c.sw((c.g[S1][0] as u32).wrapping_add(0x8), 0u64);
        }
        // 0x3e7690: a flagged board or a live count skips the send tail;
        // below, a fresh tick response skips it too.
        c.cycles += 3;
        c.g[V0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x4)).lw(c);
        let flagged = c.g[V0][0] != 0;
        c.g[V0][0] = c.lw(frame.at(c, 0x10));
        if !flagged {
            c.cycles += 3;
            c.g[V0][0] = Addr(c.g[S1][0] as u32).lw(c);
            let pending = (c.g[V0][0] as i64) > 0;
            c.g[V0][0] = c.lw(frame.at(c, 0x10));
            if !pending {
                // 0x3e7604: quiet links skip the tick; otherwise only a
                // stale tick runs the send tail below.
                c.cycles += 3;
                c.g[V0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x8)).lw(c);
                let quiet = c.g[V0][0] == 0;
                let mut send = true;
                if !quiet {
                    c.cycles += 7;
                    callee::NET_TICK.call(c, 0x003e_75f0)?;
                    c.g[V1][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x8)).lw(c);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
                    c.g[V0][0] = ((c.g[V0][0] as i64) < 6) as u64;
                    let fresh = c.g[V0][0] != 0;
                    c.g[V0][0] = c.lw(frame.at(c, 0x10));
                    send = !fresh;
                }
                if send {
                    // 0x3e763c: send the ping, then re-check the count.
                    c.cycles += 8;
                    callee::NET_TICK.call(c, 0x003e_760c)?;
                    c.g[V1][0] = 0x1;
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x4), c.g[V1][0]);
                    c.g[V0][0] = Addr(c.g[S1][0] as u32).lw(c);
                    let pending = c.g[V0][0] != 0;
                    c.g[S0][0] = RPC_REQUESTS_PAGE;
                    if !pending {
                        c.cycles += 6;
                        c.g[A0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x10)).lw(c);
                        c.g[A1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x7ff));
                        call_ptr(c, callee::INVALID_DCACHE.addr, 0x003e_7630)?;
                        c.g[V0][0] = 0xffff_ffff_ffff_ffff;
                        c.sw(c.g[S1][0] as u32, c.g[V0][0]);
                        c.g[S0][0] = RPC_REQUESTS_PAGE;
                    }
                    c.cycles += 21;
                    c.g[A1][0] = 0x7069_0000;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x1118));
                    c.g[A2][0] = 0x7265_0000;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] |= 0x6e67; // 'ping'
                    c.g[A2][0] |= 0x7370; // 'resp'
                    callee::DIRTY_RPC_INIT_REQUEST.call(c, 0x003e_7658)?;
                    c.g[V1][0] = 0x3e_0000;
                    c.g[T2][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x10)).lw(c);
                    c.g[A2][0] = Addr((c.g[S1][0] as u32).wrapping_add(0xc)).lw(c);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x74e8));
                    c.sw(frame.at(c, 0), c.g[V1][0]);
                    c.g[A0][0] = c.g[S0][0];
                    c.sw(frame.at(c, 8), c.g[S1][0]);
                    c.g[A1][0] = 0x1;
                    c.g[A3][0] = 0;
                    c.g[T0][0] = 0;
                    c.g[T1][0] = 0;
                    c.g[T3][0] = 0x800;
                    callee::DIRTY_RPC_CALL.call(c, 0x003e_768c)?;
                    c.g[V0][0] = c.lw(frame.at(c, 0x10));
                }
            }
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `void LobbyApi_DebugPrintf(api, fmt, ...)` (0x3e76a8): format into a
/// 0x1000-byte frame buffer, then pass it to the debug callback at
/// `api+0x548` (with the argument at `api+0x544`) when one is set.
pub fn lobby_api_debug_printf(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x1090);
        frame.save(c, S0, 0x1010);
        c.g[S0][0] = c.g[A0][0];
        c.sd(frame.at(c, 0x1060), c.g[A2][0]);
        frame.save_ra(c, 0x1000);
        c.g[A0][0] = c.g[SP][0];
        c.sd(frame.at(c, 0x1068), c.g[A3][0]);
        c.g[A2][0] = sx(frame.at(c, 0x1060));
        c.sd(frame.at(c, 0x1070), c.g[T0][0]);
        c.sd(frame.at(c, 0x1078), c.g[T1][0]);
        c.sd(frame.at(c, 0x1080), c.g[T2][0]);
        c.sd(frame.at(c, 0x1088), c.g[T3][0]);
        c.swc1(12, frame.at(c, 0x1050));
        c.swc1(14, frame.at(c, 0x1054));
        c.swc1(16, frame.at(c, 0x1058));
        c.swc1(18, frame.at(c, 0x105c));
        call_ptr(c, callee::VSPRINTF.addr, 0x003e_76ec)?;
        c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(LOBBY_API_DEBUG_CB)).lw(c);
        let no_callback = c.g[V0][0] == 0;
        if no_callback {
            frame.restore(c, S0, 0x1010);
        } else {
            c.cycles += 4;
            c.g[A0][0] = Addr((c.g[S0][0] as u32).wrapping_add(LOBBY_API_DEBUG_ARG)).lw(c);
            let target = c.g[V0][0] as u32;
            c.g[A1][0] = c.g[SP][0];
            call_ptr(c, target, 0x003e_7704)?;
            frame.restore(c, S0, 0x1010);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0x1000);
        frame.close(c);
        Ok(())
    })
}

/// `void* LobbyApi_AddRequest(api, tag, a2, a3)` (0x3e7718): allocate a
/// 0x44-byte request (`tag`, `a2` at `+0x38`, `a3` at `+0x3c`), give it the
/// next id from `api+0x54c` and link it into the queue at `api+0x53c` in
/// front of the first `'ping'` request, or at the tail when there is none.
/// Returns the request, null when the allocation fails.
pub fn lobby_api_add_request(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[S3][0] = c.g[A3][0];
        c.g[A0][0] = LOBBY_REQ_SIZE as u64;
        call_ptr(c, callee::LOBBY_MEM_ALLOC.addr, 0x003e_7748)?;
        c.g[A1][0] = c.g[V0][0];
        let alloc_failed = c.g[A1][0] == 0;
        c.g[V1][0] = 0x7fff_0000;
        'b_3e77cc: {
            if alloc_failed {
                break 'b_3e77cc;
            }
            'b_3e77c4: {
                'b_3e77c0: {
                    c.cycles += 14;
                    c.sw((c.g[A1][0] as u32).wrapping_add(LOBBY_REQ_TAG), c.g[S1][0]);
                    c.sw((c.g[A1][0] as u32).wrapping_add(LOBBY_REQ_ARG2), c.g[S2][0]);
                    c.g[V1][0] |= 0xffff;
                    c.sw((c.g[A1][0] as u32).wrapping_add(LOBBY_REQ_ARG3), c.g[S3][0]);
                    c.g[A2][0] = sx((c.g[S0][0] as u32).wrapping_add(LOBBY_API_REQUESTS));
                    c.sw((c.g[A1][0] as u32).wrapping_add(LOBBY_REQ_NEXT), 0u64);
                    c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(LOBBY_API_REQ_ID)).lw(c);
                    c.sw(c.g[A1][0] as u32, c.g[V0][0]);
                    c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(LOBBY_API_REQ_ID)).lw(c);
                    c.g[A0][0] = Addr((c.g[S0][0] as u32).wrapping_add(LOBBY_API_REQUESTS)).lw(c);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] &= c.g[V1][0];
                    let head_empty = c.g[A0][0] == 0;
                    c.sw(
                        (c.g[S0][0] as u32).wrapping_add(LOBBY_API_REQ_ID),
                        c.g[V0][0],
                    );
                    if head_empty {
                        break 'b_3e77c0;
                    }
                    c.cycles += 5;
                    c.g[V0][0] = 0x7069_0000;
                    c.g[V1][0] = Addr((c.g[A0][0] as u32).wrapping_add(4)).lw(c);
                    c.g[V0][0] |= 0x6e67;
                    let head_is_ping = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = Addr(c.g[A2][0] as u32).lw(c);
                    if head_is_ping {
                        break 'b_3e77c4;
                    }
                    c.cycles += 2;
                    c.g[A0][0] = 0x7069_0000;
                    c.g[A0][0] |= 0x6e67;
                    'l_3e77a8: loop {
                        c.cycles += 3;
                        c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(LOBBY_REQ_NEXT)).lw(c);
                        let at_end = c.g[V1][0] == 0;
                        c.g[A2][0] = sx((c.g[V0][0] as u32).wrapping_add(LOBBY_REQ_NEXT));
                        if at_end {
                            break 'b_3e77c0;
                        }
                        c.cycles += 3;
                        c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(LOBBY_REQ_TAG)).lw(c);
                        if c.g[V0][0] != c.g[A0][0] {
                            c.g[V0][0] = Addr(c.g[A2][0] as u32).lw(c);
                            if event_due(c) {
                                deliver_event_here(c, 0x003e_77a8);
                            }
                            continue 'l_3e77a8;
                        } else {
                            break 'b_3e77c0;
                        }
                    }
                }
                c.cycles += 1;
                c.g[V0][0] = Addr(c.g[A2][0] as u32).lw(c);
                break 'b_3e77c4;
            }
            c.cycles += 2;
            c.sw(c.g[A2][0] as u32, c.g[A1][0]);
            c.sw((c.g[A1][0] as u32).wrapping_add(LOBBY_REQ_NEXT), c.g[V0][0]);
            break 'b_3e77cc;
        }
        c.cycles += 8;
        frame.restore(c, S0, 0x40);
        c.g[V0][0] = c.g[A1][0];
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int LobbyApi_PopRequest(api, out, tag)` (0x3e77f0): zero the 0x44-byte
/// `out`, then look at the head of the queue at `api+0x53c` only: when its
/// tag is `tag`, or `tag` is the wildcard `'DQUE'`, unlink it, copy it into
/// `out` and free it. Returns whether the queue is non-empty afterwards.
pub fn lobby_api_pop_request(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A2][0];
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A0][0];
        c.g[S2][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = 0;
        c.g[A2][0] = LOBBY_REQ_SIZE as u64;
        call_ptr(c, callee::MEMSET.addr, 0x003e_7820)?;
        c.g[A0][0] = Addr((c.g[S1][0] as u32).wrapping_add(LOBBY_API_REQUESTS)).lw(c);
        'b_3e78f8: {
            if c.g[A0][0] == 0 {
                c.g[V0][0] = Addr((c.g[S1][0] as u32).wrapping_add(LOBBY_API_REQUESTS)).lw(c);
                break 'b_3e78f8;
            }
            'b_3e7844: {
                c.cycles += 3;
                c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(LOBBY_REQ_TAG)).lw(c);
                let tag_matches = c.g[V0][0] == c.g[S0][0];
                c.g[V0][0] = 0x4451_0000;
                if tag_matches {
                    break 'b_3e7844;
                }
                c.cycles += 3;
                c.g[V0][0] |= 0x5545;
                if c.g[S0][0] != c.g[V0][0] {
                    c.g[V0][0] = Addr((c.g[S1][0] as u32).wrapping_add(LOBBY_API_REQUESTS)).lw(c);
                    break 'b_3e78f8;
                } else {
                    break 'b_3e7844;
                }
            }
            'b_3e78e8: {
                'b_3e78b8: {
                    c.cycles += 7;
                    c.g[A1][0] = c.g[S2][0];
                    c.g[V1][0] = Addr((c.g[A0][0] as u32).wrapping_add(LOBBY_REQ_NEXT)).lw(c);
                    c.g[V0][0] = c.g[A0][0] | c.g[A1][0];
                    c.g[A2][0] = c.g[A0][0];
                    c.g[V0][0] &= 0x7;
                    let aligned = c.g[V0][0] == 0;
                    c.sw(
                        (c.g[S1][0] as u32).wrapping_add(LOBBY_API_REQUESTS),
                        c.g[V1][0],
                    );
                    if aligned {
                        break 'b_3e78b8;
                    }
                    c.cycles += 1;
                    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(LOBBY_REQ_NEXT));
                    'l_3e7864: loop {
                        c.cycles += 19;
                        c.ldl(3, (c.g[A2][0] as u32).wrapping_add(0x7));
                        c.ldr(3, c.g[A2][0] as u32);
                        c.ldl(7, (c.g[A2][0] as u32).wrapping_add(0xf));
                        c.ldr(7, (c.g[A2][0] as u32).wrapping_add(0x8));
                        c.ldl(8, (c.g[A2][0] as u32).wrapping_add(0x17));
                        c.ldr(8, (c.g[A2][0] as u32).wrapping_add(0x10));
                        c.ldl(9, (c.g[A2][0] as u32).wrapping_add(0x1f));
                        c.ldr(9, (c.g[A2][0] as u32).wrapping_add(0x18));
                        c.sdl(3, (c.g[A1][0] as u32).wrapping_add(0x7));
                        c.sdr(3, c.g[A1][0] as u32);
                        c.sdl(7, (c.g[A1][0] as u32).wrapping_add(0xf));
                        c.sdr(7, (c.g[A1][0] as u32).wrapping_add(0x8));
                        c.sdl(8, (c.g[A1][0] as u32).wrapping_add(0x17));
                        c.sdr(8, (c.g[A1][0] as u32).wrapping_add(0x10));
                        c.sdl(9, (c.g[A1][0] as u32).wrapping_add(0x1f));
                        c.sdr(9, (c.g[A1][0] as u32).wrapping_add(0x18));
                        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
                        let more = c.g[A2][0] != c.g[V0][0];
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                        if more {
                            if event_due(c) {
                                deliver_event_here(c, 0x003e_7864);
                            }
                            continue 'l_3e7864;
                        }
                        c.cycles += 2;
                        break 'b_3e78e8;
                    }
                }
                c.cycles += 1;
                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(LOBBY_REQ_NEXT));
                'l_3e78bc: loop {
                    c.cycles += 11;
                    c.g[V1][0] = c.ld(c.g[A2][0] as u32);
                    c.g[A3][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
                    c.g[T0][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x10));
                    c.g[T1][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x18));
                    c.sd(c.g[A1][0] as u32, c.g[V1][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[A3][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x10), c.g[T0][0]);
                    c.sd((c.g[A1][0] as u32).wrapping_add(0x18), c.g[T1][0]);
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
                    let more = c.g[A2][0] != c.g[V0][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                    if more {
                        if event_due(c) {
                            deliver_event_here(c, 0x003e_78bc);
                        }
                        continue 'l_3e78bc;
                    }
                    break 'b_3e78e8;
                }
            }
            c.cycles += 4;
            c.g[V0][0] = c.lw(c.g[A2][0] as u32);
            c.sw(c.g[A1][0] as u32, c.g[V0][0]);
            call_ptr(c, callee::LOBBY_MEM_FREE.addr, 0x003e_78f4)?;
            c.g[V0][0] = Addr((c.g[S1][0] as u32).wrapping_add(LOBBY_API_REQUESTS)).lw(c);
            break 'b_3e78f8;
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int LobbyApi_PopPingRequest(api, out, name, addr, now)` (0x3e7918):
/// find the first `'ping'` request in the queue at `api+0x53c` whose address
/// is `addr` and whose name is `name`, or, when `addr` is -1, whose deadline
/// is before `now`; unlink it, copy it into `out`, free it and count it off
/// `api+0x540`. Returns 1 when one was taken, else 0.
pub fn lobby_api_pop_ping_request(c: &mut Cpu) {
    guest(c, |c| {
        let frame: Frame;
        c.cycles += 19;
        frame = Frame::open(c, 0xa0);
        frame.save(c, S1, 0x80);
        frame.save(c, S3, 0x60);
        frame.save(c, S4, 0x50);
        c.g[S3][0] = c.g[A3][0];
        frame.save(c, S5, 0x40);
        c.g[S4][0] = c.g[A0][0];
        frame.save(c, S6, 0x30);
        c.g[S5][0] = c.g[T0][0];
        frame.save(c, S7, 0x20);
        c.g[S6][0] = c.g[A2][0];
        frame.save(c, S0, 0x90);
        c.g[S7][0] = c.g[A1][0];
        frame.save(c, S2, 0x70);
        frame.save(c, FP, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = Addr((c.g[S4][0] as u32).wrapping_add(LOBBY_API_REQUESTS)).lw(c);
        let queue_empty = c.g[V0][0] == 0;
        c.g[S1][0] = sx((c.g[S4][0] as u32).wrapping_add(LOBBY_API_REQUESTS));
        // 0x3e7a88: an empty queue skips the walk below.
        let mut popped = false;
        if !queue_empty {
            c.cycles += 3;
            c.g[S2][0] = 0x7069_0000;
            c.g[FP][0] = 0xffff_ffff_ffff_ffff;
            c.g[S2][0] |= 0x6e67;
            // 0x3e7970: walk the queue for a poppable ping request.
            loop {
                // 0x3e7a7c: tag check; a non-ping node advances below.
                c.cycles += 4;
                c.g[S0][0] = Addr(c.g[S1][0] as u32).lw(c);
                c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(LOBBY_REQ_TAG)).lw(c);
                if c.g[V0][0] != c.g[S2][0] {
                    c.g[V0][0] = Addr(c.g[S1][0] as u32).lw(c);
                } else {
                    // 0x3e799c: an address match runs the name compare;
                    // anything else runs the id check below.
                    c.cycles += 3;
                    c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(LOBBY_PINGREQ_ADDR)).lw(c);
                    let other_addr = c.g[V0][0] != c.g[S3][0];
                    c.g[A0][0] = c.g[S6][0];
                    let mut pop = false;
                    if !other_addr {
                        c.cycles += 4;
                        c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(LOBBY_PINGREQ_NAME));
                        call_ptr(c, callee::STRCMP.addr, 0x003e_7994)?;
                        if c.g[V0][0] == 0 {
                            c.g[A1][0] = c.g[S7][0];
                            pop = true;
                        }
                    }
                    if !pop {
                        // A named id advances; id -1 runs the deadline
                        // below.
                        c.cycles += 2;
                        if c.g[S3][0] != c.g[FP][0] {
                            c.g[V0][0] = Addr(c.g[S1][0] as u32).lw(c);
                        } else {
                            c.cycles += 4;
                            c.g[V0][0] =
                                Addr((c.g[S0][0] as u32).wrapping_add(LOBBY_PINGREQ_DEADLINE))
                                    .lw(c);
                            c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[S5][0] as i64)) as u64;
                            let not_expired = c.g[V0][0] == 0;
                            c.g[A1][0] = c.g[S7][0];
                            if !not_expired {
                                pop = true;
                            } else {
                                // Kept: rejoin the walk below.
                                c.cycles += 1;
                                c.g[V0][0] = Addr(c.g[S1][0] as u32).lw(c);
                            }
                        }
                    }
                    if pop {
                        // 0x3e7a54: unlink the node.
                        c.cycles += 6;
                        c.g[V1][0] = Addr((c.g[S0][0] as u32).wrapping_add(LOBBY_REQ_NEXT)).lw(c);
                        c.g[V0][0] = c.g[S0][0] | c.g[A1][0];
                        c.g[A2][0] = c.g[S0][0];
                        c.g[V0][0] &= 0x7;
                        let aligned = c.g[V0][0] == 0;
                        c.sw(c.g[S1][0] as u32, c.g[V1][0]);
                        // The aligned node copies below with `ld`/`sd`,
                        // the unaligned one here with `ldl`/`ldr`.
                        if aligned {
                            c.cycles += 1;
                            c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(LOBBY_REQ_NEXT));
                            loop {
                                c.cycles += 11;
                                c.g[V1][0] = c.ld(c.g[A2][0] as u32);
                                c.g[A0][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
                                c.g[A3][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x10));
                                c.g[T0][0] = c.ld((c.g[A2][0] as u32).wrapping_add(0x18));
                                c.sd(c.g[A1][0] as u32, c.g[V1][0]);
                                c.sd((c.g[A1][0] as u32).wrapping_add(0x8), c.g[A0][0]);
                                c.sd((c.g[A1][0] as u32).wrapping_add(0x10), c.g[A3][0]);
                                c.sd((c.g[A1][0] as u32).wrapping_add(0x18), c.g[T0][0]);
                                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
                                let more = c.g[A2][0] != c.g[V0][0];
                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                                if more {
                                    if event_due(c) {
                                        deliver_event_here(c, 0x003e_7a28);
                                    }
                                    continue;
                                }
                                break;
                            }
                        } else {
                            c.cycles += 1;
                            c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(LOBBY_REQ_NEXT));
                            loop {
                                c.cycles += 19;
                                c.ldl(3, (c.g[A2][0] as u32).wrapping_add(0x7));
                                c.ldr(3, c.g[A2][0] as u32);
                                c.ldl(4, (c.g[A2][0] as u32).wrapping_add(0xf));
                                c.ldr(4, (c.g[A2][0] as u32).wrapping_add(0x8));
                                c.ldl(7, (c.g[A2][0] as u32).wrapping_add(0x17));
                                c.ldr(7, (c.g[A2][0] as u32).wrapping_add(0x10));
                                c.ldl(8, (c.g[A2][0] as u32).wrapping_add(0x1f));
                                c.ldr(8, (c.g[A2][0] as u32).wrapping_add(0x18));
                                c.sdl(3, (c.g[A1][0] as u32).wrapping_add(0x7));
                                c.sdr(3, c.g[A1][0] as u32);
                                c.sdl(4, (c.g[A1][0] as u32).wrapping_add(0xf));
                                c.sdr(4, (c.g[A1][0] as u32).wrapping_add(0x8));
                                c.sdl(7, (c.g[A1][0] as u32).wrapping_add(0x17));
                                c.sdr(7, (c.g[A1][0] as u32).wrapping_add(0x10));
                                c.sdl(8, (c.g[A1][0] as u32).wrapping_add(0x1f));
                                c.sdr(8, (c.g[A1][0] as u32).wrapping_add(0x18));
                                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
                                let more = c.g[A2][0] != c.g[V0][0];
                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                                if more {
                                    if event_due(c) {
                                        deliver_event_here(c, 0x003e_79d0);
                                    }
                                    continue;
                                }
                                c.cycles += 2;
                                break;
                            }
                        }
                        // Free the node and count the pop.
                        c.cycles += 9;
                        c.g[V0][0] = c.lw(c.g[A2][0] as u32);
                        c.g[A0][0] = c.g[S0][0];
                        c.sw(c.g[A1][0] as u32, c.g[V0][0]);
                        call_ptr(c, callee::LOBBY_MEM_FREE.addr, 0x003e_7a64)?;
                        c.g[V1][0] =
                            Addr((c.g[S4][0] as u32).wrapping_add(LOBBY_API_PING_COUNT)).lw(c);
                        c.g[V0][0] = 0x1;
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1));
                        c.sw(
                            (c.g[S4][0] as u32).wrapping_add(LOBBY_API_PING_COUNT),
                            c.g[V1][0],
                        );
                        popped = true;
                        break;
                    }
                }
                c.cycles += 3;
                c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(LOBBY_REQ_NEXT)).lw(c);
                let more = c.g[V1][0] != 0;
                c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add(LOBBY_REQ_NEXT));
                if more {
                    if event_due(c) {
                        deliver_event_here(c, 0x003e_7970);
                    }
                    continue;
                }
                break;
            }
        }
        if !popped {
            c.cycles += 1;
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

/// `void LobbyApi_FlushRequests(api)` (0x3e7ac0): pop (and free) requests
/// with the `'DQUE'` wildcard until the queue at `api+0x53c` is empty.
pub fn lobby_api_flush_requests(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S0, 0x60);
        frame.save_ra(c, 0x50);
        c.g[S0][0] = c.g[A0][0];
        c.g[A2][0] = 0x4451_0000;
        'l_3e7ad8: loop {
            c.cycles += 6;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[SP][0];
            c.g[A2][0] |= 0x5545; // 'DQUE'
            callee::LOBBY_API_POP_REQUEST.call(c, 0x003e_7ae8)?;
            if c.g[V0][0] != 0 {
                c.g[A2][0] = 0x4451_0000;
                if event_due(c) {
                    deliver_event_here(c, 0x003e_7ad8);
                }
                continue 'l_3e7ad8;
            } else {
                c.cycles += 4;
                frame.restore(c, S0, 0x60);
                frame.restore_ra(c, 0x50);
                frame.close(c);
                return Ok(());
            }
        }
    })
}

/// `void LobbyApi_UpdateRoomCount(api, room, count)` (0x3e7b00): look `room`
/// up in slot 0's hash; when its entry's count (`+0x10`) differs from
/// `count`, store `count`, print it into the entry's text at `+0x14` and
/// update the entry in slot 0's list.
pub fn lobby_api_update_room_count(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S2, 0x20);
        frame.save(c, S3, 0x10);
        frame.save(c, S0, 0x40);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S1, 0x30);
        frame.save_ra(c, 0);
        c.g[A0][0] = Addr((c.g[S3][0] as u32).wrapping_add(LOBBY_API_TABLES)).lw(c);
        let no_table = c.g[A0][0] == 0;
        c.g[S2][0] = c.g[A2][0];
        'b_3e7b88: {
            'b_3e7b84: {
                if no_table {
                    break 'b_3e7b84;
                }
                c.cycles += 5;
                callee::LOBBY_HASH_FIND_INT.call(c, 0x003e_7b30)?;
                c.g[S1][0] = c.g[V0][0];
                let missing = c.g[S1][0] == 0;
                frame.restore(c, S0, 0x40);
                if missing {
                    break 'b_3e7b88;
                }
                c.cycles += 6;
                c.g[A0][0] = Addr((c.g[S3][0] as u32).wrapping_add(LOBBY_API_TABLES2)).lw(c);
                c.g[A1][0] = c.g[S1][0];
                callee::LOBBY_LIST_GET_DATA.call(c, 0x003e_7b48)?;
                c.g[S0][0] = c.g[V0][0];
                if c.g[S0][0] == 0 {
                    frame.restore(c, S0, 0x40);
                    break 'b_3e7b88;
                }
                c.cycles += 3;
                c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x10)).lw(c);
                let same = c.g[S2][0] == c.g[V0][0];
                c.g[A1][0] = LOBBY_STRINGS_PAGE;
                if same {
                    break 'b_3e7b84;
                }
                c.cycles += 9;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[S2][0]);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4870));
                c.g[A2][0] = c.g[S2][0];
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x14));
                call_ptr(c, callee::SPRINTF.addr, 0x003e_7b74)?;
                c.g[A0][0] = Addr((c.g[S3][0] as u32).wrapping_add(LOBBY_API_TABLES2)).lw(c);
                c.g[A1][0] = c.g[S1][0];
                c.g[A2][0] = c.g[S0][0];
                callee::LOBBY_LIST_UPDATE.call(c, 0x003e_7b84)?;
                break 'b_3e7b84;
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x40);
            break 'b_3e7b88;
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

/// `void LobbyApi_SortLists(api)` (0x3e7ba0): for each of the 8 list slots
/// at `api+0x310` that has a hash and a notify callback (`+8`), sort its
/// list; when the sort reports a change, call the callback with `api`, a
/// `{2, slot, 0, list}` event and the slot's argument (`+0xc`).
pub fn lobby_api_sort_lists(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S2, 0x30);
        frame.save(c, S0, 0x50);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x40);
        c.g[S0][0] = sx((c.g[S2][0] as u32).wrapping_add(LOBBY_API_TABLES));
        frame.save(c, S3, 0x20);
        c.g[S1][0] = 0;
        frame.save_ra(c, 0x10);
        c.g[S3][0] = 0x2;
        'l_3e7bc8: loop {
            'b_3e7c18: {
                c.cycles += 3;
                c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
                if c.g[V0][0] == 0 {
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                    break 'b_3e7c18;
                }
                c.cycles += 3;
                c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x8)).lw(c);
                if c.g[V0][0] == 0 {
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                    break 'b_3e7c18;
                }
                'b_3e7c14: {
                    c.cycles += 4;
                    c.g[A0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x4)).lw(c);
                    callee::LOBBY_LIST_SORT.call(c, 0x003e_7be8)?;
                    let clean = (c.g[V0][0] as i64) <= 0;
                    c.g[A0][0] = c.g[S2][0];
                    if clean {
                        break 'b_3e7c14;
                    }
                    c.cycles += 9;
                    c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x4)).lw(c);
                    c.g[V1][0] = Addr((c.g[S0][0] as u32).wrapping_add(0x8)).lw(c);
                    c.g[A1][0] = c.g[SP][0];
                    c.g[A2][0] = Addr((c.g[S0][0] as u32).wrapping_add(0xc)).lw(c);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0xc), c.g[V0][0]);
                    c.sw(c.g[SP][0] as u32, c.g[S3][0]);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[S1][0]);
                    let target = c.g[V1][0] as u32;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x8), 0u64);
                    call_ptr(c, target, 0x003e_7c14)?;
                    break 'b_3e7c14;
                }
                c.cycles += 1;
                c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                break 'b_3e7c18;
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S1][0] as i64) < 8) as u64;
            let more = c.g[V0][0] != 0;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x14));
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x003e_7bc8);
                }
                continue 'l_3e7bc8;
            }
            c.cycles += 7;
            frame.restore(c, S0, 0x50);
            frame.restore(c, S1, 0x40);
            frame.restore(c, S2, 0x30);
            frame.restore(c, S3, 0x20);
            frame.restore_ra(c, 0x10);
            frame.close(c);
            return Ok(());
        }
    })
}

/// `void LobbyApi_ProcessPings(api)` (0x3e7c40): for each reply on the ping
/// channel, pop the `'ping'` request with the reply's name and address and
/// call its reply callback (`+0x38`, when set) with `api`, a
/// `{0, 'ping', 0, text}` event (text fields: name, address, the reply's
/// first word) and `+0x3c`; then print that word into the slot 0 entries
/// with that address and the slot 1 entries with that name. Afterwards pop
/// every request whose deadline has passed and call its callback the same
/// way with -1 as the third event word and 0 as the value. A dispatcher: it
/// resumes at every block the original can unwind at.
pub fn lobby_api_process_pings(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x003e_7c40 => {
                    c.cycles += 17;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x440));
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x420));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x430));
                    c.g[S1][0] = c.g[A0][0];
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x410));
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x400));
                    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x3f0));
                    c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x3e0));
                    c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x3d0));
                    c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x3c0));
                    c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x3b0));
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x3a0), c.g[RA][0]);
                    c.g[S3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x110));
                    c.g[S6][0] = sx((c.g[SP][0] as u32).wrapping_add(0x130));
                    c.g[S4][0] = sx((c.g[SP][0] as u32).wrapping_add(0x10));
                    c.g[S7][0] = sx((c.g[SP][0] as u32).wrapping_add(0x390));
                    pc = 0x003e_7e4c;
                    continue;
                }
                0x003e_7c84 => {
                    c.cycles += 1;
                    pc = 0x003e_7c88;
                    continue;
                }
                0x003e_7c88 => {
                    c.cycles += 26;
                    c.g[A0][0] = c.g[S4][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4878));
                    callee::TAG_FIELD_FIND.call(c, 0x003e_7c94)?;
                    c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                    c.g[A0][0] = c.g[V0][0];
                    callee::TAG_FIELD_GET_ADDRESS.call(c, 0x003e_7ca0)?;
                    c.g[S5][0] = c.g[V0][0];
                    c.g[A0][0] = c.g[S4][0];
                    c.g[V0][0] = LOBBY_STRINGS_PAGE;
                    c.g[S2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4880));
                    c.g[A1][0] = c.g[S2][0];
                    callee::TAG_FIELD_FIND.call(c, 0x003e_7cb8)?;
                    c.g[A3][0] = LOBBY_STRINGS_PAGE;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x4888));
                    c.g[A1][0] = c.g[S3][0];
                    c.g[A2][0] = 0x20;
                    callee::TAG_FIELD_GET_STRING.call(c, 0x003e_7cd0)?;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[A1][0] = c.g[S6][0];
                    c.g[A2][0] = c.g[S3][0];
                    c.g[A3][0] = c.g[S5][0];
                    c.g[T0][0] = 0;
                    callee::LOBBY_API_POP_PING_REQUEST.call(c, 0x003e_7ce8)?;
                    let popped = c.g[V0][0] == 0;
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x168));
                    if popped {
                        pc = 0x003e_7d6c;
                    } else {
                        pc = 0x003e_7cf0;
                    }
                    continue;
                }
                0x003e_7cf0 => {
                    c.cycles += 2;
                    let no_callback = c.g[V0][0] == 0;
                    c.g[V0][0] = 0x7069_0000;
                    if no_callback {
                        pc = 0x003e_7d6c;
                    } else {
                        pc = 0x003e_7cf8;
                    }
                    continue;
                }
                0x003e_7cf8 => {
                    c.cycles += 29;
                    c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x190));
                    c.g[V0][0] |= 0x6e67;
                    c.g[A2][0] = c.g[S2][0];
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x184), c.g[V0][0]);
                    c.g[A0][0] = c.g[S0][0];
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x180), 0u64);
                    c.g[A1][0] = 0x200;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x188), 0u64);
                    c.g[A3][0] = c.g[S3][0];
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x18c), c.g[S0][0]);
                    c.sb((c.g[SP][0] as u32).wrapping_add(0x190), 0u64);
                    callee::TAG_FIELD_SET_STRING.call(c, 0x003e_7d28)?;
                    c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x138));
                    c.g[A2][0] = LOBBY_STRINGS_PAGE;
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x4890));
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = 0x200;
                    callee::TAG_FIELD_SET_NUMBER.call(c, 0x003e_7d40)?;
                    c.g[A2][0] = LOBBY_STRINGS_PAGE;
                    c.g[A3][0] = c.lw(c.g[SP][0] as u32);
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x4898));
                    c.g[A1][0] = 0x200;
                    callee::TAG_FIELD_SET_NUMBER.call(c, 0x003e_7d58)?;
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x168));
                    c.g[A0][0] = c.g[S1][0];
                    c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x16c));
                    let target = c.g[V1][0] as u32;
                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x180));
                    call_ptr(c, target, 0x003e_7d6c)?;
                    pc = 0x003e_7d6c;
                    continue;
                }
                0x003e_7d6c => {
                    c.cycles += 3;
                    c.g[V0][0] = Addr((c.g[S1][0] as u32).wrapping_add(LOBBY_API_TABLES)).lw(c);
                    if c.g[V0][0] == 0 {
                        c.g[V0][0] =
                            Addr((c.g[S1][0] as u32).wrapping_add(LOBBY_API_USER_HASH)).lw(c);
                        pc = 0x003e_7de4;
                    } else {
                        pc = 0x003e_7d78;
                    }
                    continue;
                }
                0x003e_7d78 => {
                    c.cycles += 3;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x390), 0u64);
                    c.g[S0][0] = LOBBY_STRINGS_PAGE;
                    pc = 0x003e_7dc8;
                    continue;
                }
                0x003e_7d84 => {
                    c.cycles += 1;
                    pc = 0x003e_7d88;
                    continue;
                }
                0x003e_7d88 => {
                    c.cycles += 3;
                    c.g[V0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x64)).lw(c);
                    let stale = c.g[V0][0] != c.g[S5][0];
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x390));
                    if stale {
                        pc = 0x003e_7dc0;
                    } else {
                        pc = 0x003e_7d94;
                    }
                    continue;
                }
                0x003e_7d94 => {
                    c.cycles += 8;
                    c.g[A2][0] = c.lw(c.g[SP][0] as u32);
                    call_ptr(c, callee::SPRINTF.addr, 0x003e_7d9c)?;
                    c.g[A0][0] = Addr((c.g[S1][0] as u32).wrapping_add(LOBBY_API_TABLES2)).lw(c);
                    callee::LOBBY_LIST_VISIBLE_COUNT.call(c, 0x003e_7da4)?;
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x390));
                    c.g[V1][0] = ((c.g[V1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let dirty = c.g[V1][0] == 0;
                    c.g[A1][0] = 0x1;
                    if dirty {
                        pc = 0x003e_7dbc;
                    } else {
                        pc = 0x003e_7db4;
                    }
                    continue;
                }
                0x003e_7db4 => {
                    c.cycles += 2;
                    c.g[A0][0] = Addr((c.g[S1][0] as u32).wrapping_add(LOBBY_API_TABLES2)).lw(c);
                    callee::LOBBY_LIST_MARK_DIRTY.call(c, 0x003e_7dbc)?;
                    pc = 0x003e_7dbc;
                    continue;
                }
                0x003e_7dbc => {
                    c.cycles += 1;
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x390));
                    pc = 0x003e_7dc0;
                    continue;
                }
                0x003e_7dc0 => {
                    c.cycles += 2;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x390), c.g[V0][0]);
                    pc = 0x003e_7dc8;
                    continue;
                }
                0x003e_7dc8 => {
                    c.cycles += 6;
                    c.g[A0][0] = Addr((c.g[S1][0] as u32).wrapping_add(LOBBY_API_TABLES2)).lw(c);
                    c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x390));
                    callee::LOBBY_LIST_GET_AT.call(c, 0x003e_7dd4)?;
                    c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x48a0));
                    let more = c.g[V0][0] != 0;
                    c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x5c));
                    if more {
                        if back_edge_unwinds(c, 0x003e_7d88) {
                            return Ok(());
                        }
                        pc = 0x003e_7d88;
                    } else {
                        pc = 0x003e_7de0;
                    }
                    continue;
                }
                0x003e_7de0 => {
                    c.cycles += 1;
                    c.g[V0][0] = Addr((c.g[S1][0] as u32).wrapping_add(LOBBY_API_USER_HASH)).lw(c);
                    pc = 0x003e_7de4;
                    continue;
                }
                0x003e_7de4 => {
                    c.cycles += 2;
                    let no_users = c.g[V0][0] == 0;
                    c.g[S2][0] = LOBBY_STRINGS_PAGE;
                    if no_users {
                        pc = 0x003e_7e4c;
                    } else {
                        pc = 0x003e_7dec;
                    }
                    continue;
                }
                0x003e_7dec => {
                    c.cycles += 2;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x390), 0u64);
                    pc = 0x003e_7e30;
                    continue;
                }
                0x003e_7df4 => {
                    c.cycles += 1;
                    pc = 0x003e_7df8;
                    continue;
                }
                0x003e_7df8 => {
                    c.cycles += 4;
                    call_ptr(c, callee::STR_CASE_CMP.addr, 0x003e_7e00)?;
                    let differs = c.g[V0][0] != 0;
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x390));
                    if differs {
                        pc = 0x003e_7e28;
                    } else {
                        pc = 0x003e_7e08;
                    }
                    continue;
                }
                0x003e_7e08 => {
                    c.cycles += 8;
                    c.g[A2][0] = c.lw(c.g[SP][0] as u32);
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x28));
                    c.g[A1][0] = sx((c.g[S2][0] as u32).wrapping_add(0x48a0));
                    call_ptr(c, callee::SPRINTF.addr, 0x003e_7e18)?;
                    c.g[A0][0] = Addr((c.g[S1][0] as u32).wrapping_add(LOBBY_API_USER_LIST)).lw(c);
                    c.g[A1][0] = 0x1;
                    callee::LOBBY_LIST_MARK_DIRTY.call(c, 0x003e_7e24)?;
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x390));
                    pc = 0x003e_7e28;
                    continue;
                }
                0x003e_7e28 => {
                    c.cycles += 2;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x390), c.g[V0][0]);
                    pc = 0x003e_7e30;
                    continue;
                }
                0x003e_7e30 => {
                    c.cycles += 7;
                    c.g[A0][0] = Addr((c.g[S1][0] as u32).wrapping_add(LOBBY_API_USER_LIST)).lw(c);
                    c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x390));
                    callee::LOBBY_LIST_GET_AT.call(c, 0x003e_7e3c)?;
                    c.g[S0][0] = c.g[V0][0];
                    c.g[A1][0] = c.g[S3][0];
                    let more = c.g[S0][0] != 0;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x8));
                    if more {
                        if back_edge_unwinds(c, 0x003e_7df8) {
                            return Ok(());
                        }
                        pc = 0x003e_7df8;
                    } else {
                        pc = 0x003e_7e4c;
                    }
                    continue;
                }
                0x003e_7e4c => {
                    c.cycles += 9;
                    c.g[V0][0] = 0x100;
                    c.g[A0][0] = Addr((c.g[S1][0] as u32).wrapping_add(LOBBY_API_PING_CHAN)).lw(c);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x390), c.g[V0][0]);
                    c.g[A1][0] = c.g[S4][0];
                    c.g[A2][0] = c.g[S7][0];
                    c.g[A3][0] = c.g[SP][0];
                    callee::PROTO_PING_RESPONSE.call(c, 0x003e_7e68)?;
                    let replied = c.g[V0][0] != 0;
                    c.g[A1][0] = LOBBY_STRINGS_PAGE;
                    if replied {
                        if back_edge_unwinds(c, 0x003e_7c88) {
                            return Ok(());
                        }
                        pc = 0x003e_7c88;
                    } else {
                        pc = 0x003e_7e70;
                    }
                    continue;
                }
                0x003e_7e70 => {
                    c.cycles += 10;
                    c.g[A0][0] = Addr(c.g[S1][0] as u32).lw(c);
                    c.g[S2][0] = 0x7069_0000;
                    c.g[S2][0] |= 0x6e67;
                    c.g[FP][0] = LOBBY_STRINGS_PAGE;
                    callee::PROTO_ARIES_TICK.call(c, 0x003e_7e84)?;
                    c.g[S7][0] = LOBBY_STRINGS_PAGE;
                    c.g[S4][0] = c.g[V0][0];
                    c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x190));
                    c.g[S5][0] = LOBBY_STRINGS_PAGE;
                    pc = 0x003e_7ef8;
                    continue;
                }
                0x003e_7e98 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x168));
                    let no_callback = c.g[V0][0] == 0;
                    c.g[V0][0] = 0xffff_ffff_ffff_ffff;
                    if no_callback {
                        pc = 0x003e_7ef8;
                    } else {
                        pc = 0x003e_7ea4;
                    }
                    continue;
                }
                0x003e_7ea4 => {
                    c.cycles += 21;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x180), 0u64);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x184), c.g[S2][0]);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x188), c.g[V0][0]);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x18c), c.g[S0][0]);
                    c.sb((c.g[SP][0] as u32).wrapping_add(0x190), 0u64);
                    callee::TAG_FIELD_SET_STRING.call(c, 0x003e_7ebc)?;
                    c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x138));
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = 0x200;
                    c.g[A2][0] = sx((c.g[S7][0] as u32).wrapping_add(0x4890));
                    callee::TAG_FIELD_SET_NUMBER.call(c, 0x003e_7ed0)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = 0x200;
                    c.g[A2][0] = sx((c.g[S5][0] as u32).wrapping_add(0x4898));
                    c.g[A3][0] = 0;
                    callee::TAG_FIELD_SET_NUMBER.call(c, 0x003e_7ee4)?;
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x168));
                    c.g[A0][0] = c.g[S1][0];
                    c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x16c));
                    let target = c.g[V1][0] as u32;
                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x180));
                    call_ptr(c, target, 0x003e_7ef8)?;
                    pc = 0x003e_7ef8;
                    continue;
                }
                0x003e_7ef8 => {
                    c.cycles += 11;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[A1][0] = c.g[S6][0];
                    c.g[A2][0] = c.g[S3][0];
                    c.g[A3][0] = 0xffff_ffff_ffff_ffff;
                    c.g[T0][0] = c.g[S4][0];
                    callee::LOBBY_API_POP_PING_REQUEST.call(c, 0x003e_7f10)?;
                    c.g[A3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x148));
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = 0x200;
                    let popped = c.g[V0][0] != 0;
                    c.g[A2][0] = sx((c.g[FP][0] as u32).wrapping_add(0x4880));
                    if popped {
                        if back_edge_unwinds(c, 0x003e_7e98) {
                            return Ok(());
                        }
                        pc = 0x003e_7e98;
                    } else {
                        pc = 0x003e_7f24;
                    }
                    continue;
                }
                0x003e_7f24 => {
                    c.cycles += 12;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x430));
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x420));
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x410));
                    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x400));
                    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x3f0));
                    c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x3e0));
                    c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x3d0));
                    c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x3c0));
                    c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x3b0));
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x3a0));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x440));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x003e_7c40, 0x003e_7f54);
                    return Ok(());
                }
            }
        }
    })
}

/// `void LobbyApi_ResetServerPing(api)` (0x3e7f58): clear the server-ping
/// words at `api+0x18` and `+0x20` and set `+0x1c` to -1.
pub fn lobby_api_reset_server_ping(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        c.g[V0][0] = 0xffff_ffff_ffff_0000;
        c.sw(
            (c.g[A0][0] as u32).wrapping_add(LOBBY_API_SERVER_PING_C),
            0u64,
        );
        c.g[V0][0] |= 0xffff;
        c.sw(
            (c.g[A0][0] as u32).wrapping_add(LOBBY_API_SERVER_PING_A),
            0u64,
        );
        c.sw(
            (c.g[A0][0] as u32).wrapping_add(LOBBY_API_SERVER_PING_B),
            c.g[V0][0],
        );
        let t = c.g[RA][0] as u32;
        c.pc = t;
        Ok(())
    })
}

/// `void LobbyApi_Terminate(api, reason, flags)` (0x3e7f70): enter state
/// `'term'`, OR `flags` into `api+0x8`, reset the server ping, and when a
/// terminate callback is set at `api+0x52c`, call it with `api`, a
/// `{3, 'disc', reason, 0}` event and its argument at `api+0x530`.
pub fn lobby_api_terminate(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = 0x7465_0000;
        frame.save(c, S0, 0x30);
        c.g[V0][0] |= 0x726d;
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x10);
        c.g[S1][0] = c.g[A1][0];
        c.sw(
            (c.g[S0][0] as u32).wrapping_add(LOBBY_API_STATE_TAG),
            c.g[V0][0],
        );
        c.g[V1][0] = Addr((c.g[S0][0] as u32).wrapping_add(LOBBY_API_EVENTS)).lw(c);
        c.g[V1][0] |= c.g[A2][0];
        c.sw(
            (c.g[S0][0] as u32).wrapping_add(LOBBY_API_EVENTS),
            c.g[V1][0],
        );
        callee::LOBBY_API_RESET_SERVER_PING.call(c, 0x003e_7fa4)?;
        c.g[A3][0] = Addr((c.g[S0][0] as u32).wrapping_add(LOBBY_API_TERM_CB)).lw(c);
        let no_callback = c.g[A3][0] == 0;
        c.g[V0][0] = 0x6469_0000;
        'b_3e7fd8: {
            if no_callback {
                break 'b_3e7fd8;
            }
            c.cycles += 10;
            c.g[V1][0] = 0x3;
            c.g[V0][0] |= 0x7363;
            c.sw(c.g[SP][0] as u32, c.g[V1][0]);
            c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[V0][0]);
            c.g[A0][0] = c.g[S0][0];
            c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[S1][0]);
            c.g[A1][0] = c.g[SP][0];
            c.sw((c.g[SP][0] as u32).wrapping_add(0xc), 0u64);
            let target = c.g[A3][0] as u32;
            c.g[A2][0] = Addr((c.g[A0][0] as u32).wrapping_add(LOBBY_API_TERM_ARG)).lw(c);
            call_ptr(c, target, 0x003e_7fd8)?;
            break 'b_3e7fd8;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x30);
        frame.restore(c, S1, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `LobbyApi* LobbyApi_Create(params, debug_arg, debug_cb)` (0x3e7ff0):
/// allocate and zero a 0x550-byte object; when `params` is given, copy it
/// into the tag buffer at `+0x34` (as is when it holds a `=`, else as the
/// value of one tag field); create the ProtoAries connection and the ping
/// channel, store the debug callback, and reset the object. Null when the
/// allocation fails.
pub fn lobby_api_create(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S0, 0x40);
        c.g[S3][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = 0x550;
        call_ptr(c, callee::LOBBY_MEM_ALLOC.addr, 0x003e_801c)?;
        c.g[S0][0] = c.g[V0][0];
        let alloc_failed = c.g[S0][0] == 0;
        c.g[A0][0] = c.g[S0][0];
        'b_3e809c: {
            if alloc_failed {
                break 'b_3e809c;
            }
            'b_3e8078: {
                c.cycles += 5;
                c.g[A1][0] = 0;
                c.g[A2][0] = 0x550;
                call_ptr(c, callee::MEMSET.addr, 0x003e_8034)?;
                let no_name = c.g[S1][0] == 0;
                c.g[A0][0] = c.g[S1][0];
                if no_name {
                    break 'b_3e8078;
                }
                'b_3e8060: {
                    c.cycles += 4;
                    c.g[A1][0] = 0x3d;
                    call_ptr(c, callee::STRCHR.addr, 0x003e_8044)?;
                    let no_eq = c.g[V0][0] == 0;
                    c.g[A2][0] = c.g[S1][0];
                    if no_eq {
                        break 'b_3e8060;
                    }
                    c.cycles += 5;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(LOBBY_API_TAG_BUF));
                    c.g[A1][0] = 0x100;
                    callee::TAG_FIELD_STRNZCPY.call(c, 0x003e_8058)?;
                    break 'b_3e8078;
                }
                c.cycles += 6;
                c.g[A2][0] = LOBBY_STRINGS_PAGE;
                c.g[A3][0] = c.g[S1][0];
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x48a8));
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(LOBBY_API_TAG_BUF));
                c.g[A1][0] = 0x100;
                callee::TAG_FIELD_SET_STRING.call(c, 0x003e_8078)?;
                break 'b_3e8078;
            }
            c.cycles += 9;
            c.g[A0][0] = 0x8000;
            callee::PROTO_ARIES_CREATE.call(c, 0x003e_8080)?;
            c.sw(
                (c.g[S0][0] as u32).wrapping_add(LOBBY_API_ARIES),
                c.g[V0][0],
            );
            // A0 left stale for ProtoPingCreate, as generated.
            callee::PROTO_PING_CREATE.call(c, 0x003e_8088)?;
            c.sw(
                (c.g[S0][0] as u32).wrapping_add(LOBBY_API_DEBUG_ARG),
                c.g[S2][0],
            );
            c.g[A0][0] = c.g[S0][0];
            c.sw(
                (c.g[S0][0] as u32).wrapping_add(LOBBY_API_DEBUG_CB),
                c.g[S3][0],
            );
            c.sw(
                (c.g[S0][0] as u32).wrapping_add(LOBBY_API_PING_CHAN),
                c.g[V0][0],
            );
            callee::LOBBY_API_RESET.call(c, 0x003e_809c)?;
            break 'b_3e809c;
        }
        c.cycles += 8;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x30);
        frame.restore(c, S0, 0x40);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void LobbyApi_Reset(api)` (0x3e80c0): enter state `'offl'`, unconnect
/// ProtoAries, empty every list slot (pop each hash entry, remove it from
/// the list, free it), free every queued request, reset the server ping and
/// clear the request id and ping count. A dispatcher: it resumes at its
/// three loop heads ([`RESET_SLOT`], [`RESET_ENTRY`], [`RESET_REQUEST`]).
pub fn lobby_api_reset(c: &mut Cpu) {
    guest(c, |c| {
        let mut head = match c.pc {
            0x003e_80c0 => {
                c.cycles += 20;
                c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x80));
                c.g[V0][0] = 0x6f66_0000;
                c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
                c.g[V0][0] |= 0x666c;
                c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x50));
                c.g[S1][0] = c.g[A0][0];
                c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x30));
                c.g[S2][0] = 0;
                c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[S4][0] = sx((c.g[S1][0] as u32).wrapping_add(LOBBY_API_TABLES));
                c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x10));
                c.g[S5][0] = sx((c.g[S1][0] as u32).wrapping_add(LOBBY_API_TABLES2));
                c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
                c.g[S6][0] = 0x14;
                c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x40));
                c.sd(c.g[SP][0] as u32, c.g[RA][0]);
                c.sw(
                    (c.g[S1][0] as u32).wrapping_add(LOBBY_API_STATE_TAG),
                    c.g[V0][0],
                );
                c.g[A0][0] = Addr(c.g[S1][0] as u32).lw(c);
                callee::PROTO_ARIES_UNCONNECT.call(c, 0x003e_810c)?;
                c.mult(0, 2, 18, 22);
                RESET_SLOT
            }
            pc @ (RESET_SLOT | RESET_ENTRY | RESET_REQUEST) => pc,
            pc => {
                c.jump_out(pc, 0x003e_80c0, 0x003e_81c8);
                return Ok(());
            }
        };
        loop {
            let next = match head {
                RESET_SLOT => reset_slot(c)?,
                RESET_ENTRY => {
                    reset_remove_entry(c)?;
                    reset_pop_entry(c)?
                }
                _ => reset_free_request(c)?,
            };
            match next {
                Some(h) => head = h,
                None => return Ok(()),
            }
        }
    })
}

/// `LobbyApi_Reset`'s slot loop head: `s2` is the slot, `v0` its offset.
const RESET_SLOT: u32 = 0x003e_8110;
/// `LobbyApi_Reset`'s entry loop head: `v0` points at the slot's list word, `a1`
/// the popped hash entry.
const RESET_ENTRY: u32 = 0x003e_8128;
/// `LobbyApi_Reset`'s request loop head: `a0` is the queue head.
const RESET_REQUEST: u32 = 0x003e_8178;

/// Where a stage of `lobby_api_reset` leads: `Some(head)` to go on at that
/// loop head, `None` once the function has returned. `Err` when a callee
/// or a back edge unwound the thread (return at once).
type ResetNext = Result<Option<u32>, crate::call::Unwound>;

/// 0x3e8110: an empty slot goes on to the next one, else pop its entries.
fn reset_slot(c: &mut Cpu) -> ResetNext {
    c.cycles += 4;
    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S4][0] as u32));
    c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
    let empty = c.g[V0][0] == 0;
    c.g[S3][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
    if empty {
        return reset_next_slot(c);
    }
    c.cycles += 2;
    c.mult(0, 16, 18, 22);
    reset_pop_entry(c)
}

/// 0x3e8128: remove the popped entry from the slot's list and free it.
fn reset_remove_entry(c: &mut Cpu) -> crate::call::Flow {
    c.cycles += 5;
    c.g[A0][0] = Addr(c.g[V0][0] as u32).lw(c);
    callee::LOBBY_LIST_REMOVE.call(c, 0x003e_8130)?;
    c.g[A0][0] = c.g[V0][0];
    call_ptr(c, callee::LOBBY_MEM_FREE.addr, 0x003e_8138)?;
    c.mult(0, 16, 18, 22);
    Ok(())
}

/// 0x3e813c: pop the slot's next hash entry (back to [`RESET_ENTRY`]), or
/// go on to the next slot.
fn reset_pop_entry(c: &mut Cpu) -> ResetNext {
    c.cycles += 6;
    c.g[V0][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[S0][0] as u32));
    c.g[A0][0] = Addr(c.g[V0][0] as u32).lw(c);
    callee::LOBBY_HASH_POP_ANY.call(c, 0x003e_8148)?;
    c.g[A1][0] = c.g[V0][0];
    let popped = c.g[A1][0] != 0;
    c.g[V0][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[S0][0] as u32));
    if popped {
        if back_edge_unwinds(c, RESET_ENTRY) {
            return Err(crate::call::Unwound);
        }
        return Ok(Some(RESET_ENTRY));
    }
    reset_next_slot(c)
}

/// 0x3e8154: the next of the 8 slots (back to [`RESET_SLOT`]); after the
/// last, the request queue (0x3e8164) or, when it is empty, the exit.
fn reset_next_slot(c: &mut Cpu) -> ResetNext {
    c.cycles += 4;
    c.g[S2][0] = c.g[S3][0];
    c.g[V0][0] = ((c.g[S2][0] as i64) < 8) as u64;
    let more = c.g[V0][0] != 0;
    c.mult(0, 2, 18, 22);
    if more {
        if back_edge_unwinds(c, RESET_SLOT) {
            return Err(crate::call::Unwound);
        }
        return Ok(Some(RESET_SLOT));
    }
    c.cycles += 3;
    c.g[V0][0] = Addr((c.g[S1][0] as u32).wrapping_add(LOBBY_API_REQUESTS)).lw(c);
    if c.g[V0][0] == 0 {
        reset_exit(c)?;
        return Ok(None);
    }
    c.cycles += 2;
    c.g[A0][0] = Addr((c.g[S1][0] as u32).wrapping_add(LOBBY_API_REQUESTS)).lw(c);
    Ok(Some(RESET_REQUEST))
}

/// 0x3e8178: unlink and free the queue head (back to [`RESET_REQUEST`]
/// while requests remain), then the exit.
fn reset_free_request(c: &mut Cpu) -> ResetNext {
    c.cycles += 6;
    c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(LOBBY_REQ_NEXT)).lw(c);
    c.sw(
        (c.g[S1][0] as u32).wrapping_add(LOBBY_API_REQUESTS),
        c.g[V0][0],
    );
    call_ptr(c, callee::LOBBY_MEM_FREE.addr, 0x003e_8184)?;
    c.g[V0][0] = Addr((c.g[S1][0] as u32).wrapping_add(LOBBY_API_REQUESTS)).lw(c);
    if c.g[V0][0] != 0 {
        c.g[A0][0] = Addr((c.g[S1][0] as u32).wrapping_add(LOBBY_API_REQUESTS)).lw(c);
        if back_edge_unwinds(c, RESET_REQUEST) {
            return Err(crate::call::Unwound);
        }
        return Ok(Some(RESET_REQUEST));
    }
    reset_exit(c)?;
    Ok(None)
}

/// 0x3e8190: reset the server ping, clear the request id and ping count,
/// restore the frame and return.
fn reset_exit(c: &mut Cpu) -> crate::call::Flow {
    c.cycles += 14;
    c.g[A0][0] = c.g[S1][0];
    callee::LOBBY_API_RESET_SERVER_PING.call(c, 0x003e_8198)?;
    c.sw((c.g[S1][0] as u32).wrapping_add(LOBBY_API_REQ_ID), 0u64);
    c.sw((c.g[S1][0] as u32).wrapping_add(LOBBY_API_PING_COUNT), 0u64);
    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x50));
    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x40));
    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x30));
    c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x20));
    c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x10));
    c.g[RA][0] = c.ld(c.g[SP][0] as u32);
    let t = c.g[RA][0] as u32;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
    c.pc = t;
    Ok(())
}

/// `void LobbyApi_Destroy(api)` (0x3e81c8): reset the object, destroy the
/// hash and list of every slot that has a hash, destroy the ping channel
/// and the ProtoAries connection, and free the object.
pub fn lobby_api_destroy(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S1, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x30);
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[S2][0] = 0x7;
        c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(LOBBY_API_TABLES2));
        callee::LOBBY_API_RESET.call(c, 0x003e_81ec)?;
        'l_3e81f0: loop {
            'b_3e820c: {
                c.cycles += 3;
                c.g[A0][0] = Addr((c.g[S0][0] as u32).wrapping_sub(LOBBY_SLOT_LIST)).lw(c);
                let empty = c.g[A0][0] == 0;
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                if empty {
                    break 'b_3e820c;
                }
                c.cycles += 4;
                callee::LOBBY_HASH_DESTROY.call(c, 0x003e_8204)?;
                c.g[A0][0] = Addr(c.g[S0][0] as u32).lw(c);
                callee::LOBBY_LIST_DESTROY.call(c, 0x003e_820c)?;
                break 'b_3e820c;
            }
            c.cycles += 2;
            let more = (c.g[S2][0] as i64) >= 0;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x14));
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x003e_81f0);
                }
                continue 'l_3e81f0;
            }
            c.cycles += 12;
            c.g[A0][0] = Addr((c.g[S1][0] as u32).wrapping_add(LOBBY_API_PING_CHAN)).lw(c);
            callee::PROTO_PING_DESTROY.call(c, 0x003e_821c)?;
            c.g[A0][0] = Addr(c.g[S1][0] as u32).lw(c);
            callee::PROTO_ARIES_DESTROY.call(c, 0x003e_8224)?;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, callee::LOBBY_MEM_FREE.addr, 0x003e_822c)?;
            frame.restore(c, S0, 0x30);
            frame.restore(c, S1, 0x20);
            frame.restore(c, S2, 0x10);
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
    })
}

/// `void LobbyApi_SetCallback(api, i, func, arg)` (0x3e8248): store `func`
/// and `arg` in callback slot `i` (8 bytes each) at `api+0x514`/`+0x518`.
pub fn lobby_api_set_callback(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        c.g[A1][0] = sx((c.g[A1][0] as u32) << 3);
        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
        c.g[A0][0] = c.g[V0][0];
        c.sw(
            (c.g[V0][0] as u32).wrapping_add(LOBBY_API_CB_FUNC),
            c.g[A2][0],
        );
        let t = c.g[RA][0] as u32;
        c.sw(
            (c.g[A0][0] as u32).wrapping_add(LOBBY_API_CB_ARG),
            c.g[A3][0],
        );
        c.pc = t;
        Ok(())
    })
}

/// `int LobbyApi_Connect(api, host, port, a3, t0)` (0x3e8260): remember
/// `host`/`port` at `+0x3b4`/`+0x3b0`, flush the queue, clear `+0x50c`,
/// reset the server ping and queue a `'skey'` request (`a3`, `t0`); then
/// reconnect ProtoAries to the peer, send the tag buffer as an `'@dir'`
/// packet, enter `'rdir'` with a deadline 120000 ticks out and clear the
/// byte at `+0x3b8`. Returns 0, -1 when the request cannot be queued.
pub fn lobby_api_connect(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 22;
        let frame = Frame::open(c, 0x140);
        frame.save(c, S2, 0x110);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0x130);
        frame.save(c, S1, 0x120);
        c.g[S0][0] = c.g[A3][0];
        frame.save_ra(c, 0x100);
        c.g[S1][0] = c.g[T0][0];
        c.sw(
            (c.g[S2][0] as u32).wrapping_add(LOBBY_API_PEER_HOST),
            c.g[A1][0],
        );
        c.sw(
            (c.g[S2][0] as u32).wrapping_add(LOBBY_API_PEER_PORT),
            c.g[A2][0],
        );
        callee::LOBBY_API_FLUSH_REQUESTS.call(c, 0x003e_828c)?;
        c.sw(
            (c.g[S2][0] as u32).wrapping_add(LOBBY_API_CONNECT_WORD),
            0u64,
        );
        c.g[A0][0] = c.g[S2][0];
        callee::LOBBY_API_RESET_SERVER_PING.call(c, 0x003e_8298)?;
        c.g[A1][0] = 0x736b_0000;
        c.g[A2][0] = c.g[S0][0];
        c.g[A3][0] = c.g[S1][0];
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] |= 0x6579; // 'skey'
        callee::LOBBY_API_ADD_REQUEST.call(c, 0x003e_82b0)?;
        'b_3e834c: {
            if c.g[V0][0] == 0 {
                c.g[V0][0] = 0xffff_ffff_ffff_ffff;
                break 'b_3e834c;
            }
            c.cycles += 37;
            c.g[A0][0] = Addr(c.g[S2][0] as u32).lw(c);
            callee::PROTO_ARIES_UNCONNECT.call(c, 0x003e_82c0)?;
            c.g[A1][0] = Addr((c.g[S2][0] as u32).wrapping_add(LOBBY_API_PEER_HOST)).lw(c);
            c.g[A0][0] = Addr(c.g[S2][0] as u32).lw(c);
            c.g[A2][0] = Addr((c.g[S2][0] as u32).wrapping_add(LOBBY_API_PEER_PORT)).lw(c);
            callee::PROTO_ARIES_CONNECT.call(c, 0x003e_82d0)?;
            c.g[A3][0] = Addr((c.g[S2][0] as u32).wrapping_add(LOBBY_API_PEER_PORT)).lw(c);
            c.g[A1][0] = LOBBY_FORMATS_PAGE;
            c.g[A2][0] = Addr((c.g[S2][0] as u32).wrapping_add(LOBBY_API_PEER_HOST)).lw(c);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x5f30));
            c.g[A0][0] = c.g[S2][0];
            callee::LOBBY_API_DEBUG_PRINTF.call(c, 0x003e_82e8)?;
            c.g[A0][0] = c.g[SP][0];
            c.g[A1][0] = 0x100;
            c.g[A2][0] = sx((c.g[S2][0] as u32).wrapping_add(LOBBY_API_TAG_BUF));
            callee::TAG_FIELD_STRNZCPY.call(c, 0x003e_82f8)?;
            c.g[A0][0] = Addr(c.g[S2][0] as u32).lw(c);
            c.g[A1][0] = 0x4064_0000;
            c.g[T0][0] = 0xffff_ffff_ffff_ffff;
            c.g[A2][0] = 0;
            c.g[A1][0] |= 0x6972;
            c.g[A3][0] = c.g[SP][0];
            callee::PROTO_ARIES_SEND.call(c, 0x003e_8314)?;
            c.g[V1][0] = 0x7264_0000;
            c.g[A0][0] = Addr(c.g[S2][0] as u32).lw(c);
            c.g[V1][0] |= 0x6972;
            c.sw(
                (c.g[S2][0] as u32).wrapping_add(LOBBY_API_STATE_TAG),
                c.g[V1][0],
            );
            callee::PROTO_ARIES_TICK.call(c, 0x003e_8328)?;
            c.g[V1][0] = 0x1_0000;
            c.g[A0][0] = Addr(c.g[S2][0] as u32).lw(c);
            c.g[V1][0] |= 0xd4c0;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.sw((c.g[S2][0] as u32).wrapping_add(LOBBY_API_TICK), c.g[V0][0]);
            callee::PROTO_ARIES_TICK.call(c, 0x003e_8340)?;
            c.sw(
                (c.g[S2][0] as u32).wrapping_add(LOBBY_API_TICK2),
                c.g[V0][0],
            );
            c.sb((c.g[S2][0] as u32).wrapping_add(LOBBY_API_CONNECTED), 0u64);
            c.g[V0][0] = 0;
            break 'b_3e834c;
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

/// `void* LobbyApi_SelectList(api, i, select)` (0x3e8368): for slot `i`
/// (0-7; slots 0-2 have names): with `select` set and no hash yet, create
/// its hash and list; with it clear and a hash present, empty and destroy
/// them. After either change, unless the state is `'offl'`, `'rdir'` or
/// `'conn'`, announce a named slot in a `'sele'` packet. Returns the slot's list word, 0 for
/// `i` of 8 or more. A dispatcher: it resumes at every block the original
/// can unwind at.
pub fn lobby_api_select_list(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x003e_8368 => {
                    c.cycles += 14;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x180));
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x150));
                    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x130));
                    c.g[S2][0] = c.g[A1][0];
                    c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x110));
                    c.g[S4][0] = c.g[A0][0];
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x170));
                    c.g[S6][0] = c.g[A2][0];
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x160));
                    c.g[V0][0] = (c.g[S2][0] < 0x8u64) as u64;
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x140));
                    c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x120));
                    let in_range = c.g[V0][0] != 0;
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x100), c.g[RA][0]);
                    if in_range {
                        pc = 0x003e_83a8;
                    } else {
                        pc = 0x003e_83a0;
                    }
                    continue;
                }
                0x003e_83a0 => {
                    c.cycles += 2;
                    c.g[V0][0] = 0;
                    pc = 0x003e_8584;
                    continue;
                }
                0x003e_83a8 => {
                    c.cycles += 2;
                    let nonzero = c.g[S2][0] != 0;
                    c.g[V0][0] = 0x2;
                    if nonzero {
                        pc = 0x003e_83c0;
                    } else {
                        pc = 0x003e_83b0;
                    }
                    continue;
                }
                0x003e_83b0 => {
                    c.cycles += 4;
                    c.g[V0][0] = LOBBY_STRINGS_PAGE;
                    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x48b0));
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x48b0));
                    pc = 0x003e_83ec;
                    continue;
                }
                0x003e_83c0 => {
                    c.cycles += 2;
                    let not_two = c.g[S2][0] != c.g[V0][0];
                    c.g[V0][0] = 0x1;
                    if not_two {
                        pc = 0x003e_83d8;
                    } else {
                        pc = 0x003e_83c8;
                    }
                    continue;
                }
                0x003e_83c8 => {
                    c.cycles += 4;
                    c.g[V0][0] = LOBBY_STRINGS_PAGE;
                    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x48b8));
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x48b8));
                    pc = 0x003e_83ec;
                    continue;
                }
                0x003e_83d8 => {
                    c.cycles += 2;
                    if c.g[S2][0] != c.g[V0][0] {
                        c.sb(c.g[SP][0] as u32, 0u64);
                        pc = 0x003e_8400;
                    } else {
                        pc = 0x003e_83e0;
                    }
                    continue;
                }
                0x003e_83e0 => {
                    c.cycles += 3;
                    c.g[V0][0] = LOBBY_STRINGS_PAGE;
                    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x48c0));
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x48c0));
                    pc = 0x003e_83ec;
                    continue;
                }
                0x003e_83ec => {
                    c.cycles += 5;
                    c.g[A1][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0x6));
                    c.g[V0][0] = c.lhu((c.g[V1][0] as u32).wrapping_add(0x4));
                    c.sw(c.g[SP][0] as u32, c.g[A0][0]);
                    c.sh((c.g[SP][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                    c.sb((c.g[SP][0] as u32).wrapping_add(0x6), c.g[A1][0]);
                    pc = 0x003e_8400;
                    continue;
                }
                0x003e_8400 => {
                    c.cycles += 3;
                    c.g[S3][0] = sx((c.g[S4][0] as u32).wrapping_add(LOBBY_API_TABLES));
                    let deselect = c.g[S6][0] == 0;
                    c.g[S5][0] = sx((c.g[S4][0] as u32).wrapping_add(LOBBY_API_TABLES2));
                    if deselect {
                        pc = 0x003e_84ac;
                    } else {
                        pc = 0x003e_840c;
                    }
                    continue;
                }
                0x003e_840c => {
                    c.cycles += 6;
                    c.g[V0][0] = 0x14;
                    c.mult(0, 17, 18, 2);
                    c.g[S0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[S1][0] as u32));
                    c.g[V0][0] = Addr(c.g[S0][0] as u32).lw(c);
                    let exists = c.g[V0][0] != 0;
                    c.g[A0][0] = 0x3e8;
                    if exists {
                        pc = 0x003e_84a4;
                    } else {
                        pc = 0x003e_8424;
                    }
                    continue;
                }
                0x003e_8424 => {
                    c.cycles += 14;
                    c.g[A1][0] = 0x7d0;
                    callee::LOBBY_HASH_CREATE.call(c, 0x003e_842c)?;
                    c.g[A0][0] = 0x64;
                    c.sw(c.g[S0][0] as u32, c.g[V0][0]);
                    c.g[A1][0] = 0x64;
                    c.g[A2][0] = 0;
                    callee::LOBBY_LIST_CREATE.call(c, 0x003e_8440)?;
                    c.g[A0][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[S1][0] as u32));
                    c.g[V1][0] = 0x6f66_0000;
                    c.sw(c.g[A0][0] as u32, c.g[V0][0]);
                    c.g[V1][0] |= 0x666c;
                    c.g[A0][0] = Addr((c.g[S4][0] as u32).wrapping_add(LOBBY_API_STATE_TAG)).lw(c);
                    let offline = c.g[A0][0] == c.g[V1][0];
                    c.g[V0][0] = 0x7264_0000;
                    if offline {
                        pc = 0x003e_84a4;
                    } else {
                        pc = 0x003e_845c;
                    }
                    continue;
                }
                0x003e_845c => {
                    c.cycles += 3;
                    c.g[V0][0] |= 0x6972;
                    let redir = c.g[A0][0] == c.g[V0][0];
                    c.g[V0][0] = 0x636f_0000;
                    if redir {
                        pc = 0x003e_84a4;
                    } else {
                        pc = 0x003e_8468;
                    }
                    continue;
                }
                0x003e_8468 => {
                    c.cycles += 3;
                    c.g[V0][0] |= 0x6e6e;
                    let conn = c.g[A0][0] == c.g[V0][0];
                    c.g[V0][0] = c.lb(c.g[SP][0] as u32);
                    if conn {
                        pc = 0x003e_84a4;
                    } else {
                        pc = 0x003e_8474;
                    }
                    continue;
                }
                0x003e_8474 => {
                    c.cycles += 2;
                    let unnamed = c.g[V0][0] == 0;
                    c.g[A1][0] = LOBBY_STRINGS_PAGE;
                    if unnamed {
                        pc = 0x003e_84a4;
                    } else {
                        pc = 0x003e_847c;
                    }
                    continue;
                }
                0x003e_847c => {
                    c.cycles += 10;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x48c8));
                    call_ptr(c, callee::STRCAT.addr, 0x003e_8488)?;
                    c.g[A1][0] = 0x7365_0000;
                    c.g[A0][0] = Addr(c.g[S4][0] as u32).lw(c);
                    c.g[A1][0] |= 0x6c65;
                    c.g[A2][0] = 0;
                    c.g[A3][0] = c.g[SP][0];
                    c.g[T0][0] = 0xffff_ffff_ffff_ffff;
                    callee::PROTO_ARIES_SEND.call(c, 0x003e_84a4)?;
                    pc = 0x003e_84a4;
                    continue;
                }
                0x003e_84a4 => {
                    c.cycles += 2;
                    let select = c.g[S6][0] != 0;
                    c.g[V1][0] = 0x14;
                    if select {
                        pc = 0x003e_8578;
                    } else {
                        pc = 0x003e_84ac;
                    }
                    continue;
                }
                0x003e_84ac => {
                    c.cycles += 6;
                    c.g[V0][0] = 0x14;
                    c.mult(0, 3, 18, 2);
                    c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.g[V0][0] = Addr(c.g[A0][0] as u32).lw(c);
                    let empty = c.g[V0][0] == 0;
                    c.g[S1][0] = c.g[A0][0];
                    if empty {
                        pc = 0x003e_8574;
                    } else {
                        pc = 0x003e_84c4;
                    }
                    continue;
                }
                0x003e_84c4 => {
                    c.cycles += 2;
                    c.g[S0][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    pc = 0x003e_84e0;
                    continue;
                }
                0x003e_84cc => {
                    c.cycles += 1;
                    pc = 0x003e_84d0;
                    continue;
                }
                0x003e_84d0 => {
                    c.cycles += 4;
                    c.g[A1][0] = c.g[V0][0];
                    callee::LOBBY_LIST_REMOVE.call(c, 0x003e_84d8)?;
                    c.g[A0][0] = c.g[V0][0];
                    call_ptr(c, callee::LOBBY_MEM_FREE.addr, 0x003e_84e0)?;
                    pc = 0x003e_84e0;
                    continue;
                }
                0x003e_84e0 => {
                    c.cycles += 4;
                    c.g[A0][0] = Addr(c.g[S1][0] as u32).lw(c);
                    callee::LOBBY_HASH_POP_ANY.call(c, 0x003e_84e8)?;
                    if c.g[V0][0] != 0 {
                        c.g[A0][0] = Addr(c.g[S0][0] as u32).lw(c);
                        if back_edge_unwinds(c, 0x003e_84d0) {
                            return Ok(());
                        }
                        pc = 0x003e_84d0;
                    } else {
                        pc = 0x003e_84f0;
                    }
                    continue;
                }
                0x003e_84f0 => {
                    c.cycles += 15;
                    c.g[S0][0] = 0x14;
                    c.mult(0, 16, 18, 16);
                    c.g[S1][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[S0][0] as u32));
                    c.g[A0][0] = Addr(c.g[S1][0] as u32).lw(c);
                    c.g[S0][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[S0][0] as u32));
                    callee::LOBBY_HASH_DESTROY.call(c, 0x003e_8508)?;
                    c.sw(c.g[S1][0] as u32, 0u64);
                    c.g[A0][0] = Addr(c.g[S0][0] as u32).lw(c);
                    callee::LOBBY_LIST_DESTROY.call(c, 0x003e_8514)?;
                    c.sw(c.g[S0][0] as u32, 0u64);
                    c.g[V0][0] = 0x6f66_0000;
                    c.g[V0][0] |= 0x666c;
                    c.g[V1][0] = Addr((c.g[S4][0] as u32).wrapping_add(LOBBY_API_STATE_TAG)).lw(c);
                    let offline = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = 0x7264_0000;
                    if offline {
                        pc = 0x003e_8574;
                    } else {
                        pc = 0x003e_852c;
                    }
                    continue;
                }
                0x003e_852c => {
                    c.cycles += 3;
                    c.g[V0][0] |= 0x6972;
                    let redir = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = 0x636f_0000;
                    if redir {
                        pc = 0x003e_8574;
                    } else {
                        pc = 0x003e_8538;
                    }
                    continue;
                }
                0x003e_8538 => {
                    c.cycles += 3;
                    c.g[V0][0] |= 0x6e6e;
                    let conn = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = c.lb(c.g[SP][0] as u32);
                    if conn {
                        pc = 0x003e_8574;
                    } else {
                        pc = 0x003e_8544;
                    }
                    continue;
                }
                0x003e_8544 => {
                    c.cycles += 2;
                    let unnamed = c.g[V0][0] == 0;
                    c.g[A1][0] = LOBBY_STRINGS_PAGE;
                    if unnamed {
                        pc = 0x003e_8574;
                    } else {
                        pc = 0x003e_854c;
                    }
                    continue;
                }
                0x003e_854c => {
                    c.cycles += 10;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x48d0));
                    call_ptr(c, callee::STRCAT.addr, 0x003e_8558)?;
                    c.g[A1][0] = 0x7365_0000;
                    c.g[A0][0] = Addr(c.g[S4][0] as u32).lw(c);
                    c.g[A1][0] |= 0x6c65;
                    c.g[A2][0] = 0;
                    c.g[A3][0] = c.g[SP][0];
                    c.g[T0][0] = 0xffff_ffff_ffff_ffff;
                    callee::PROTO_ARIES_SEND.call(c, 0x003e_8574)?;
                    pc = 0x003e_8574;
                    continue;
                }
                0x003e_8574 => {
                    c.cycles += 1;
                    c.g[V1][0] = 0x14;
                    pc = 0x003e_8578;
                    continue;
                }
                0x003e_8578 => {
                    c.cycles += 3;
                    c.mult(0, 2, 18, 3);
                    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S5][0] as u32));
                    c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
                    pc = 0x003e_8584;
                    continue;
                }
                0x003e_8584 => {
                    c.cycles += 10;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x170));
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x160));
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x150));
                    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x140));
                    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x130));
                    c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x120));
                    c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x110));
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x100));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x180));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x003e_8368, 0x003e_85ac);
                    return Ok(());
                }
            }
        }
    })
}

/// `void* LobbyApi_CreateList(api, i, notify, arg)` (0x3e85b0): store the
/// notify callback and its argument in slot `i`, create the slot's hash and
/// list and attach the hash to the list. Returns the list, 0 for `i` of 8
/// or more.
pub fn lobby_api_create_list(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        c.g[V1][0] = 0;
        frame.save(c, S0, 0x30);
        c.g[V0][0] = (c.g[A1][0] < 0x8u64) as u64;
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        let out_of_range = c.g[V0][0] == 0;
        frame.save_ra(c, 0);
        'b_3e8630: {
            if out_of_range {
                break 'b_3e8630;
            }
            c.cycles += 23;
            c.g[S2][0] = 0x14;
            c.g[A0][0] = 0x3e8;
            c.mult(0, 18, 5, 18);
            c.g[A1][0] = 0x7d0;
            c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[S2][0] as u32));
            c.g[V1][0] = c.g[V0][0];
            c.sw(
                (c.g[V0][0] as u32).wrapping_add(LOBBY_API_TABLES + LOBBY_SLOT_NOTIFY),
                c.g[A2][0],
            );
            c.sw(
                (c.g[V1][0] as u32).wrapping_add(LOBBY_API_TABLES + LOBBY_SLOT_ARG),
                c.g[A3][0],
            );
            callee::LOBBY_HASH_CREATE.call(c, 0x003e_85f8)?;
            c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(LOBBY_API_TABLES));
            c.g[A2][0] = 0;
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[S2][0] as u32));
            c.g[A0][0] = 0x64;
            c.g[A1][0] = 0x64;
            c.sw(c.g[S1][0] as u32, c.g[V0][0]);
            callee::LOBBY_LIST_CREATE.call(c, 0x003e_8614)?;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(LOBBY_API_TABLES2));
            c.g[A0][0] = c.g[V0][0];
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[S2][0] as u32));
            c.sw(c.g[S0][0] as u32, c.g[V0][0]);
            c.g[A1][0] = Addr(c.g[S1][0] as u32).lw(c);
            callee::LOBBY_LIST_SET_HASH.call(c, 0x003e_862c)?;
            c.g[V1][0] = Addr(c.g[S0][0] as u32).lw(c);
            break 'b_3e8630;
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x30);
        c.g[V0][0] = c.g[V1][0];
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void LobbyApi_DestroyList(api, i, list)` (0x3e8688): for `i` below 8
/// and a non-null `list`: pop every entry off the list's hash (remove it
/// from `list`, free it), destroy the hash and `list`, clear slot `i`'s
/// notify callback and argument and, when the slot holds `list`, its hash
/// and list words. Returns nothing (`v0` is left as scratch). A dispatcher:
/// it resumes at the drain loop's head ([`DESTROY_LIST_REMOVE`]).
pub fn lobby_api_destroy_list(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0x80 };
        match c.pc {
            0x003e_8688 => {
                c.cycles += 14;
                Frame::open(c, frame.size);
                frame.save(c, S6, 0x10);
                frame.save(c, S0, 0x70);
                c.g[S6][0] = c.g[A1][0];
                frame.save(c, S2, 0x50);
                c.g[S0][0] = c.g[A0][0];
                frame.save(c, S1, 0x60);
                c.g[S2][0] = c.g[A2][0];
                frame.save(c, S3, 0x40);
                c.g[V0][0] = (c.g[S6][0] < 0x8u64) as u64;
                frame.save(c, S4, 0x30);
                frame.save(c, S5, 0x20);
                let out_of_range = c.g[V0][0] == 0u64;
                frame.save_ra(c, 0);
                if out_of_range {
                    c.cycles += 1;
                    frame.restore(c, S0, 0x70);
                } else {
                    c.cycles += 2;
                    if c.g[S2][0] == 0u64 {
                        frame.restore(c, S0, 0x70);
                    } else {
                        c.cycles += 7;
                        // Registers at the call: list in a0.
                        c.g[A0][0] = c.g[S2][0];
                        callee::LOBBY_LIST_GET_HASH.call(c, 0x003e_86d0)?;
                        c.g[S1][0] = c.g[V0][0];
                        c.g[S3][0] = sx((c.g[S0][0] as u32).wrapping_add(LOBBY_API_TABLES2));
                        c.g[S4][0] =
                            sx((c.g[S0][0] as u32)
                                .wrapping_add(LOBBY_API_TABLES + LOBBY_SLOT_NOTIFY));
                        c.g[S5][0] =
                            sx((c.g[S0][0] as u32).wrapping_add(LOBBY_API_TABLES + LOBBY_SLOT_ARG));
                        destroy_list_drain(c, frame)?;
                    }
                }
            }
            DESTROY_LIST_REMOVE => {
                destroy_list_remove(c)?;
                destroy_list_drain(c, frame)?;
            }
            pc => {
                c.jump_out(pc, 0x003e_8688, 0x003e_8774);
                return Ok(());
            }
        }
        c.cycles += 9;
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

/// `LobbyApi_DestroyList`'s drain loop head: `a0` is the list, `v0` the
/// popped entry.
const DESTROY_LIST_REMOVE: u32 = 0x003e_86e8;

/// 0x3e86e8: remove the popped entry from the list and free it.
fn destroy_list_remove(c: &mut Cpu) -> crate::call::Flow {
    c.cycles += 4;
    // Registers at the call: list in a0, popped entry in a1.
    c.g[A1][0] = c.g[V0][0];
    callee::LOBBY_LIST_REMOVE.call(c, 0x003e_86f0)?;
    // Registers at the call: removed entry in a0.
    c.g[A0][0] = c.g[V0][0];
    call_ptr(c, callee::LOBBY_MEM_FREE.addr, 0x003e_86f8)
}

/// 0x3e86f8..0x3e874c: pop entries off the hash until it is empty (each
/// back through [`DESTROY_LIST_REMOVE`]), destroy the hash and the list,
/// clear the slot's words, and restore `s0`.
fn destroy_list_drain(c: &mut Cpu, frame: Frame) -> crate::call::Flow {
    loop {
        c.cycles += 4;
        // Registers at the call: hash in a0.
        c.g[A0][0] = c.g[S1][0];
        callee::LOBBY_HASH_POP_ANY.call(c, 0x003e_8700)?;
        let popped = c.g[V0][0] != 0u64;
        c.g[A0][0] = c.g[S2][0];
        if !popped {
            break;
        }
        if back_edge_unwinds(c, DESTROY_LIST_REMOVE) {
            return Err(crate::call::Unwound);
        }
        destroy_list_remove(c)?;
    }
    c.cycles += 10;
    // Registers at the call: hash in a0.
    c.g[A0][0] = c.g[S1][0];
    callee::LOBBY_HASH_DESTROY.call(c, 0x003e_8710)?;
    // Registers at the call: list in a0.
    c.g[A0][0] = c.g[S2][0];
    callee::LOBBY_LIST_DESTROY.call(c, 0x003e_8718)?;
    c.g[V0][0] = 0x14;
    c.mult(0, 3, 22, 2);
    c.g[A0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.g[V0][0] = c.lw(c.g[A0][0] as u32);
    let other_list = c.g[V0][0] != c.g[S2][0];
    c.g[V0][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[V1][0] as u32));
    if !other_list {
        c.cycles += 4;
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.sw((c.g[V0][0] as u32).wrapping_add(LOBBY_API_TABLES), 0u64);
        c.sw(c.g[A0][0] as u32, 0u64);
        c.g[V0][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[V1][0] as u32));
    }
    c.cycles += 3;
    c.g[V1][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[V1][0] as u32));
    c.sw(c.g[V0][0] as u32, 0u64);
    c.sw(c.g[V1][0] as u32, 0u64);
    c.cycles += 1;
    frame.restore(c, S0, 0x70);
    Ok(())
}

/// `int LobbyApi_AddListEntry(api, i, entry)` (0x3e8778): add `entry` to
/// slot `i`'s list and hash. Slot 3 holds names: `entry` is a string,
/// copied into a fresh 0x138-byte block. Slots 4-7 hold 0xa8-byte records
/// keyed by the name at `+8`, copied into a fresh block. Returns 1 when
/// added, 0 when the name is already there, -1 for any other slot or a slot
/// without a hash.
pub fn lobby_api_add_list_entry(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S1, 0x40);
        frame.save(c, S2, 0x30);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S3, 0x20);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0x50);
        c.g[S3][0] = c.g[A2][0];
        frame.save(c, S4, 0x10);
        c.g[V0][0] = (c.g[S1][0] < 0x8u64) as u64;
        let t = c.g[V0][0] == 0u64;
        frame.save_ra(c, 0);
        if t {
            add_list_entry_fail(c, &frame);
            return Ok(());
        }
        c.cycles += 8;
        c.g[V1][0] = 0x14;
        c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(LOBBY_API_TABLES));
        c.mult(0, 3, 17, 3);
        c.g[S4][0] = c.g[V0][0];
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[A0][0] = c.lw(c.g[V0][0] as u32);
        let t = c.g[A0][0] == 0u64;
        c.g[V0][0] = 0xffff_ffff_ffff_ffff;
        if t {
            frame_epilogue_add_list_entry(c, &frame);
            return Ok(());
        }
        c.cycles += 3;
        c.g[V0][0] = 0x3;
        let t = c.g[S1][0] != c.g[V0][0];
        c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_sub(0x4));
        if t {
            // Slots 4-7: a 0xa8-byte record.
            c.cycles += 3;
            c.g[V0][0] = (c.g[V0][0] < 0x4u64) as u64;
            let t = c.g[V0][0] == 0u64;
            c.g[V0][0] = 0xffff_ffff_ffff_ffff;
            if t {
                frame_epilogue_add_list_entry(c, &frame);
                return Ok(());
            }
            c.cycles += 4;
            // Registers at the call: hash in a0, the record's name (+8) in a1.
            c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_add(0x8));
            callee::LOBBY_HASH_FIND_STR.call(c, 0x003e_8840)?;
            let t = c.g[V0][0] != 0u64;
            c.g[V0][0] = 0u64;
            if t {
                frame_epilogue_add_list_entry(c, &frame);
                return Ok(());
            }
            c.cycles += 8;
            // Registers at the call: size in a0.
            c.g[A0][0] = 0xa8;
            call_ptr(c, callee::LOBBY_MEM_ALLOC.addr, 0x003e_8850)?;
            c.g[A3][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S3][0];
            c.g[V0][0] = c.g[A1][0] | c.g[A3][0];
            c.g[V0][0] = c.g[V0][0] & 0x7;
            let t = c.g[V0][0] == 0u64;
            c.g[A2][0] = c.g[A3][0];
            if t {
                // Aligned: 0xa0-byte block copy with `ld`/`sd`.
                c.cycles += 3;
                c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(LOBBY_API_TABLES2));
                c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xa0));
                c.g[S2][0] = sx((c.g[A3][0] as u32).wrapping_add(0x8));
                loop {
                    c.cycles += 11;
                    c.g[A0][0] = c.ld(c.g[A1][0] as u32);
                    c.g[T0][0] = c.ld((c.g[A1][0] as u32).wrapping_add(0x8));
                    c.g[T1][0] = c.ld((c.g[A1][0] as u32).wrapping_add(0x10));
                    c.g[T2][0] = c.ld((c.g[A1][0] as u32).wrapping_add(0x18));
                    c.sd(c.g[A2][0] as u32, c.g[A0][0]);
                    c.sd((c.g[A2][0] as u32).wrapping_add(0x8), c.g[T0][0]);
                    c.sd((c.g[A2][0] as u32).wrapping_add(0x10), c.g[T1][0]);
                    c.sd((c.g[A2][0] as u32).wrapping_add(0x18), c.g[T2][0]);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                    let t = c.g[A1][0] != c.g[V1][0];
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x003e_88d4);
                        }
                        continue;
                    } else {
                        c.cycles += 1;
                        c.g[S0][0] = 0x14;
                        break;
                    }
                }
            } else {
                // Unaligned: 0xa0-byte block copy with `ldl`/`ldr`.
                c.cycles += 3;
                c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(LOBBY_API_TABLES2));
                c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xa0));
                c.g[S2][0] = sx((c.g[A3][0] as u32).wrapping_add(0x8));
                loop {
                    c.cycles += 19;
                    c.ldl(4, (c.g[A1][0] as u32).wrapping_add(0x7));
                    c.ldr(4, c.g[A1][0] as u32);
                    c.ldl(8, (c.g[A1][0] as u32).wrapping_add(0xf));
                    c.ldr(8, (c.g[A1][0] as u32).wrapping_add(0x8));
                    c.ldl(9, (c.g[A1][0] as u32).wrapping_add(0x17));
                    c.ldr(9, (c.g[A1][0] as u32).wrapping_add(0x10));
                    c.ldl(10, (c.g[A1][0] as u32).wrapping_add(0x1f));
                    c.ldr(10, (c.g[A1][0] as u32).wrapping_add(0x18));
                    c.sdl(4, (c.g[A2][0] as u32).wrapping_add(0x7));
                    c.sdr(4, c.g[A2][0] as u32);
                    c.sdl(8, (c.g[A2][0] as u32).wrapping_add(0xf));
                    c.sdr(8, (c.g[A2][0] as u32).wrapping_add(0x8));
                    c.sdl(9, (c.g[A2][0] as u32).wrapping_add(0x17));
                    c.sdr(9, (c.g[A2][0] as u32).wrapping_add(0x10));
                    c.sdl(10, (c.g[A2][0] as u32).wrapping_add(0x1f));
                    c.sdr(10, (c.g[A2][0] as u32).wrapping_add(0x18));
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x20));
                    let t = c.g[A1][0] != c.g[V1][0];
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x20));
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x003e_8874);
                        }
                        continue;
                    } else {
                        c.cycles += 2;
                        c.g[S0][0] = 0x14;
                        break;
                    }
                }
            }
            c.cycles += 12;
            c.ldl(3, (c.g[A1][0] as u32).wrapping_add(0x7));
            c.ldr(3, c.g[A1][0] as u32);
            c.sdl(3, (c.g[A2][0] as u32).wrapping_add(0x7));
            c.mult(0, 16, 17, 16);
            c.sdr(3, c.g[A2][0] as u32);
            c.g[A1][0] = c.g[A3][0];
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S0][0] as u32));
            // Registers at the call: list in a0, entry in a1.
            c.g[A0][0] = c.lw(c.g[V0][0] as u32);
            callee::LOBBY_LIST_ADD.call(c, 0x003e_8928)?;
            c.g[S0][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[S0][0] as u32));
            c.g[A1][0] = c.g[S2][0];
            c.g[A0][0] = c.lw(c.g[S0][0] as u32);
            add_list_entry_finish_hash(c, &frame)?;
            return Ok(());
        }
        // Slot 3: a bare name.
        c.cycles += 5;
        // Registers at the call: hash in a0, name in a1.
        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(LOBBY_API_TABLES + 3 * LOBBY_SLOT_SIZE));
        c.g[A1][0] = c.g[S3][0];
        callee::LOBBY_HASH_FIND_STR.call(c, 0x003e_87e0)?;
        if c.g[V0][0] != 0u64 {
            c.g[V0][0] = 0u64;
            frame_epilogue_add_list_entry(c, &frame);
            return Ok(());
        }
        c.cycles += 17;
        // Registers at the call: size in a0.
        c.g[A0][0] = 0x138;
        call_ptr(c, callee::LOBBY_MEM_ALLOC.addr, 0x003e_87f0)?;
        c.g[S0][0] = c.g[V0][0];
        // Registers at the call: block, zero, size (memset).
        c.g[A2][0] = 0x138;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0u64;
        call_ptr(c, callee::MEMSET.addr, 0x003e_8804)?;
        c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x8));
        // Registers at the call: dst, src (strcpy).
        c.g[A1][0] = c.g[S3][0];
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, callee::STRCPY.addr, 0x003e_8814)?;
        // Registers at the call: list in a0, entry in a1.
        c.g[A0][0] =
            c.lw((c.g[S2][0] as u32).wrapping_add(LOBBY_API_TABLES2 + 3 * LOBBY_SLOT_SIZE));
        c.g[A1][0] = c.g[S0][0];
        callee::LOBBY_LIST_ADD.call(c, 0x003e_8820)?;
        // Registers at the call: hash in a0, name in a1.
        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(LOBBY_API_TABLES + 3 * LOBBY_SLOT_SIZE));
        c.g[A1][0] = c.g[S1][0];
        add_list_entry_finish_hash(c, &frame)
    })
}

/// 0x3e893c: hash the copied name into the kind's string hash, return 1.
/// Falls into the shared 0x3e8948 epilogue (+8) like the generated `break`.
pub(crate) fn add_list_entry_finish_hash(c: &mut Cpu, frame: &Frame) -> crate::call::Flow {
    c.cycles += 4;
    // Registers at the call: hash in a0, name in a1, entry in a2.
    c.g[A2][0] = c.g[V0][0];
    callee::LOBBY_HASH_ADD_STR.call(c, 0x003e_893c)?;
    c.g[V0][0] = 0x1;
    frame_epilogue_add_list_entry(c, frame);
    Ok(())
}

/// 0x3e8944 exit of `lobby_api_add_list_entry`: return -1.
pub(crate) fn add_list_entry_fail(c: &mut Cpu, frame: &Frame) {
    c.cycles += 1;
    c.g[V0][0] = 0xffff_ffff_ffff_ffff;
    frame_epilogue_add_list_entry(c, frame);
}

/// 0x3e8948 epilogue of `lobby_api_add_list_entry` (V0 already set).
pub(crate) fn frame_epilogue_add_list_entry(c: &mut Cpu, frame: &Frame) {
    c.cycles += 8;
    add_list_entry_restore(c, frame);
}

/// Restore the frame of `lobby_api_add_list_entry` and return.
pub(crate) fn add_list_entry_restore(c: &mut Cpu, frame: &Frame) {
    frame.restore(c, S0, 0x50);
    frame.restore(c, S1, 0x40);
    frame.restore(c, S2, 0x30);
    frame.restore(c, S3, 0x20);
    frame.restore(c, S4, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
}

/// `int LobbyApi_RemoveListEntry(api, i, name)` (0x3e8968): find `name` in
/// slot `i`'s hash, delete it there, remove its entry from the slot's list
/// and free it. Returns 1 when removed, 0 when not found, -1 for a slot
/// outside 3-7 or without a hash.
pub fn lobby_api_remove_list_entry(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x60);
        c.g[V0][0] = (c.g[A1][0] < 0x8u64) as u64;
        frame.save(c, S3, 0x20);
        frame.save(c, S4, 0x10);
        c.g[S3][0] = c.g[A2][0];
        frame.save(c, S0, 0x50);
        c.g[S4][0] = c.g[A0][0];
        frame.save(c, S1, 0x40);
        frame.save(c, S2, 0x30);
        let t = c.g[V0][0] == 0u64;
        frame.save_ra(c, 0);
        if t {
            remove_list_entry_fail(c, &frame);
            return Ok(());
        }
        c.cycles += 7;
        c.g[V0][0] = 0x14;
        c.g[V1][0] = sx((c.g[S4][0] as u32).wrapping_add(LOBBY_API_TABLES));
        c.mult(0, 18, 5, 2);
        c.g[S1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S2][0] as u32));
        c.g[A0][0] = c.lw(c.g[S1][0] as u32);
        let t = c.g[A0][0] == 0u64;
        c.g[V0][0] = 0xffff_ffff_ffff_ffff;
        if t {
            frame_epilogue_remove_list_entry(c, &frame);
            return Ok(());
        }
        c.cycles += 4;
        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x3));
        c.g[V0][0] = (c.g[V0][0] < 0x5u64) as u64;
        let t = c.g[V0][0] == 0u64;
        c.g[V0][0] = 0xffff_ffff_ffff_ffff;
        if t {
            frame_epilogue_remove_list_entry(c, &frame);
            return Ok(());
        }
        c.cycles += 5;
        // Registers at the call: hash in a0, name in a1.
        c.g[A1][0] = c.g[S3][0];
        callee::LOBBY_HASH_FIND_STR.call(c, 0x003e_89c8)?;
        c.g[S0][0] = c.g[V0][0];
        if c.g[S0][0] == 0u64 {
            c.cycles += 2;
            c.g[V0][0] = 0u64;
            frame_epilogue_remove_list_entry(c, &frame);
            return Ok(());
        }
        c.g[A0][0] = c.lw(c.g[S1][0] as u32);
        c.cycles += 10;
        // Registers at the call: hash in a0, name in a1.
        c.g[A1][0] = c.g[S3][0];
        callee::LOBBY_HASH_DEL_STR.call(c, 0x003e_89e4)?;
        c.g[V1][0] = sx((c.g[S4][0] as u32).wrapping_add(c.g[S2][0] as u32));
        c.g[A1][0] = c.g[S0][0];
        // Registers at the call: the slot's list in a0, hash entry in a1.
        c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(LOBBY_API_TABLES2));
        callee::LOBBY_LIST_REMOVE.call(c, 0x003e_89f4)?;
        // Registers at the call: removed entry in a0.
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, callee::LOBBY_MEM_FREE.addr, 0x003e_89fc)?;
        c.g[V0][0] = 0x1;
        frame_epilogue_remove_list_entry(c, &frame);
        Ok(())
    })
}

/// 0x3e8a04 exit of `lobby_api_remove_list_entry`: return -1.
pub(crate) fn remove_list_entry_fail(c: &mut Cpu, frame: &Frame) {
    c.cycles += 1;
    c.g[V0][0] = 0xffff_ffff_ffff_ffff;
    frame_epilogue_remove_list_entry(c, frame);
}

/// 0x3e8a08 epilogue of `lobby_api_remove_list_entry` (V0 already set).
pub(crate) fn frame_epilogue_remove_list_entry(c: &mut Cpu, frame: &Frame) {
    c.cycles += 8;
    frame.restore(c, S0, 0x50);
    frame.restore(c, S1, 0x40);
    frame.restore(c, S2, 0x30);
    frame.restore(c, S3, 0x20);
    frame.restore(c, S4, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
}

/// `int LobbyApi_HasListEntry(api, i, name)` (0x3e8a28): whether slot `i`'s
/// hash holds `name`: 1 or 0, -1 for `i` of 8 or more or a slot without a
/// hash.
pub fn lobby_api_has_list_entry(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = (c.g[A1][0] < 0x8u64) as u64;
        let t = c.g[V0][0] == 0u64;
        frame.save_ra(c, 0);
        if t {
            has_list_entry_fail(c, &frame);
            return Ok(());
        }
        c.cycles += 6;
        c.g[V0][0] = 0x14;
        c.mult(0, 3, 5, 2);
        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(LOBBY_API_TABLES));
        if c.g[A0][0] == 0u64 {
            c.g[V0][0] = 0xffff_ffff_ffff_ffff;
            frame_epilogue_has_list_entry(c, &frame);
            return Ok(());
        }
        c.cycles += 4;
        // Registers at the call: hash in a0, name in a1.
        c.g[A1][0] = c.g[A2][0];
        callee::LOBBY_HASH_FIND_STR.call(c, 0x003e_8a58)?;
        c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
        frame_epilogue_has_list_entry(c, &frame);
        Ok(())
    })
}

/// 0x3e8a60 exit of `lobby_api_has_list_entry`: return -1.
pub(crate) fn has_list_entry_fail(c: &mut Cpu, frame: &Frame) {
    c.cycles += 1;
    c.g[V0][0] = 0xffff_ffff_ffff_ffff;
    frame_epilogue_has_list_entry(c, frame);
}

/// 0x3e8a64 epilogue of `lobby_api_has_list_entry` (V0 already set).
pub(crate) fn frame_epilogue_has_list_entry(c: &mut Cpu, frame: &Frame) {
    c.cycles += 3;
    frame.restore_ra(c, 0);
    frame.close(c);
}

/// `void* LobbyApi_GetRecord(api, tag)` (0x3e8a70): the record for `tag`:
/// `'user'` and `'acct'` give `api+0x134`, `'pers'` `+0x174`, `'room'`
/// `+0x1c0`, `'play'` `+0x200` (null while the word at `+0x2cc` is 0),
/// `'down'` `+0x3b8` (null while the byte there is 0); any other tag null.
pub fn lobby_api_get_record(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V0][0] = 0x7573_0000;
    c.g[V0][0] = c.g[V0][0] | 0x6572;
    let user = c.g[A1][0] == c.g[V0][0];
    c.g[V0][0] = 0x6163_0000;
    if !user {
        c.cycles += 3;
        c.g[V0][0] = c.g[V0][0] | 0x6374;
        let t = c.g[A1][0] != c.g[V0][0];
        c.g[V0][0] = 0x7065_0000;
        if t {
            c.cycles += 3;
            c.g[V0][0] = c.g[V0][0] | 0x7273;
            let t = c.g[A1][0] != c.g[V0][0];
            c.g[V0][0] = 0x726f_0000;
            if t {
                c.cycles += 3;
                c.g[V0][0] = c.g[V0][0] | 0x6f6d;
                let t = c.g[A1][0] != c.g[V0][0];
                c.g[V0][0] = 0x706c_0000;
                if t {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[V0][0] | 0x6179;
                    if c.g[A1][0] != c.g[V0][0] {
                        c.g[V0][0] = 0x646f_0000;
                        c.cycles += 3;
                        c.g[V0][0] = c.g[V0][0] | 0x776e;
                        let t = c.g[A1][0] != c.g[V0][0];
                        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(LOBBY_API_CONNECTED));
                        if t {
                            c.cycles += 2;
                            let t = c.g[RA][0] as u32;
                            c.g[V0][0] = 0u64;
                            c.pc = t;
                            return;
                        }
                        c.cycles += 3;
                        c.g[V1][0] = c.lb((c.g[A0][0] as u32).wrapping_add(LOBBY_API_CONNECTED));
                        let t = c.g[RA][0] as u32;
                        if c.g[V1][0] == 0 {
                            c.g[V0][0] = 0u64;
                        }
                        c.pc = t;
                        return;
                    }
                    c.cycles += 4;
                    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x2cc));
                    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x200));
                    let t = c.g[RA][0] as u32;
                    if c.g[V1][0] == 0 {
                        c.g[V0][0] = 0u64;
                    }
                    c.pc = t;
                    return;
                }
                c.cycles += 2;
                let t = c.g[RA][0] as u32;
                c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1c0));
                c.pc = t;
                return;
            }
            c.cycles += 2;
            let t = c.g[RA][0] as u32;
            c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x174));
            c.pc = t;
            return;
        }
    }
    // 'user' and 'acct'.
    c.cycles += 2;
    let t = c.g[RA][0] as u32;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x134));
    c.pc = t;
}

/// `int LobbyApi_Send(api, tag, buf, a3, t0)` (0x3e8b10): only in state
/// `'idle'`, `'auth'`, `'acct'` or `'skey'` (else -1): queue a request
/// (`tag`, `a3`, `t0`; -2 when that fails). For `'auth'`, `'acct'` and
/// `'pass'` the packet is a 0x200-byte copy of `buf` merged with the tag
/// buffer at `+0x34`, with one field re-stored: encoded with the words at
/// `+0x4bc`/`+0x4b8`, or encrypted with the key at `+0x24` when its first
/// byte is set. For `'snap'`, empty slot `n`'s list when the packet's number
/// field `n` is 4-7, then clear `+0x508` and bump `+0x50c`. Then send the
/// packet with `ProtoAriesSend` and return the request's id. A dispatcher: it
/// resumes at every block the original can unwind at.
pub fn lobby_api_send(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc = c.pc;
        loop {
            match pc {
                0x003e_8b10 => {
                    c.cycles += 17;
                    let frame = Frame::open(c, 0x290);
                    c.g[V0][0] = 0x6964_0000;
                    frame.save(c, S1, 0x270);
                    c.g[V0][0] = c.g[V0][0] | 0x6c65;
                    frame.save(c, S3, 0x250);
                    c.g[S1][0] = c.g[A0][0];
                    frame.save(c, S4, 0x240);
                    c.g[S3][0] = c.g[A2][0];
                    frame.save(c, S0, 0x280);
                    c.g[A2][0] = c.g[A3][0];
                    frame.save(c, S2, 0x260);
                    c.g[S4][0] = c.g[A1][0];
                    frame.save(c, S5, 0x230);
                    frame.save_ra(c, 0x220);
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(LOBBY_API_STATE_TAG));
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[A3][0] = c.g[T0][0];
                    if t {
                        pc = 0x003e_8b8c;
                    } else {
                        pc = 0x003e_8b54;
                    }
                }
                0x003e_8b54 => {
                    c.cycles += 4;
                    c.g[V0][0] = 0x6175_0000;
                    c.g[V0][0] = c.g[V0][0] | 0x7468;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[A0][0] = c.g[S1][0];
                    if t {
                        pc = 0x003e_8b90;
                    } else {
                        pc = 0x003e_8b64;
                    }
                }
                0x003e_8b64 => {
                    c.cycles += 4;
                    c.g[V0][0] = 0x6163_0000;
                    c.g[V0][0] = c.g[V0][0] | 0x6374;
                    let t = c.g[V1][0] == c.g[V0][0];
                    if t {
                        pc = 0x003e_8b90;
                    } else {
                        pc = 0x003e_8b74;
                    }
                }
                0x003e_8b74 => {
                    c.cycles += 4;
                    c.g[V0][0] = 0x736b_0000;
                    c.g[V0][0] = c.g[V0][0] | 0x6579;
                    let t = c.g[V1][0] == c.g[V0][0];
                    if t {
                        pc = 0x003e_8b90;
                    } else {
                        pc = 0x003e_8b84;
                    }
                }
                0x003e_8b84 => {
                    c.cycles += 2;
                    c.g[V0][0] = 0xffff_ffff_ffff_ffff;
                    pc = 0x003e_8d4c;
                }
                0x003e_8b8c => {
                    c.cycles += 1;
                    c.g[A0][0] = c.g[S1][0];
                    pc = 0x003e_8b90;
                }
                0x003e_8b90 => {
                    c.cycles += 5;
                    // Registers at the call: api, tag, a3, t0 (the request words).
                    c.g[A1][0] = c.g[S4][0];
                    callee::LOBBY_API_ADD_REQUEST.call(c, 0x003e_8b98)?;
                    c.g[S5][0] = c.g[V0][0];
                    let t = c.g[S5][0] == 0u64;
                    c.g[V0][0] = 0xffff_ffff_ffff_fffe;
                    if t {
                        pc = 0x003e_8d4c;
                    } else {
                        pc = 0x003e_8ba4;
                    }
                }
                0x003e_8ba4 => {
                    c.cycles += 4;
                    c.g[V0][0] = 0x6175_0000;
                    c.g[V0][0] = c.g[V0][0] | 0x7468;
                    let t = c.g[S4][0] == c.g[V0][0];
                    c.g[V0][0] = 0x6163_0000;
                    if t {
                        pc = 0x003e_8bcc;
                    } else {
                        pc = 0x003e_8bb4;
                    }
                }
                0x003e_8bb4 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[V0][0] | 0x6374;
                    let t = c.g[S4][0] == c.g[V0][0];
                    c.g[V0][0] = 0x7061_0000;
                    if t {
                        pc = 0x003e_8bcc;
                    } else {
                        pc = 0x003e_8bc0;
                    }
                }
                0x003e_8bc0 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[V0][0] | 0x7373;
                    let t = c.g[S4][0] != c.g[V0][0];
                    c.g[V0][0] = 0x736e_0000;
                    if t {
                        pc = 0x003e_8ca0;
                    } else {
                        pc = 0x003e_8bcc;
                    }
                }
                0x003e_8bcc => {
                    c.cycles += 24;
                    // Registers at the call: dst, size, src (strnzcpy).
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A2][0] = c.g[S3][0];
                    c.g[A1][0] = 0x200;
                    callee::TAG_FIELD_STRNZCPY.call(c, 0x003e_8bdc)?;
                    // Registers at the call: dst, size, src (merge).
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A2][0] = sx((c.g[S1][0] as u32).wrapping_add(LOBBY_API_TAG_BUF));
                    c.g[A1][0] = 0x200;
                    callee::TAG_FIELD_MERGE.call(c, 0x003e_8bec)?;
                    c.g[V1][0] = LOBBY_STRINGS_PAGE;
                    // Registers at the call: buf, key (find).
                    c.g[A0][0] = c.g[SP][0];
                    c.g[S2][0] = sx((c.g[V1][0] as u32).wrapping_add(0x48d8));
                    c.g[A1][0] = c.g[S2][0];
                    callee::TAG_FIELD_FIND.call(c, 0x003e_8c00)?;
                    c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x201));
                    c.g[A3][0] = LOBBY_STRINGS_PAGE;
                    c.g[S3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x200));
                    // Registers at the call: field, out, size, def (get).
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x4888));
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A2][0] = 0x1f;
                    callee::TAG_FIELD_GET_STRING.call(c, 0x003e_8c20)?;
                    c.g[V1][0] = c.lb((c.g[S1][0] as u32).wrapping_add(LOBBY_API_KEY));
                    let t = c.g[V1][0] != 0u64;
                    c.g[A0][0] = c.g[S0][0];
                    if t {
                        pc = 0x003e_8c64;
                    } else {
                        pc = 0x003e_8c2c;
                    }
                }
                0x003e_8c2c => {
                    c.cycles += 14;
                    // Registers at the call: buf, size, key, src, tmp.
                    c.g[A2][0] = c.g[S2][0];
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A1][0] = 0x200;
                    c.g[A3][0] = sx((c.g[S1][0] as u32).wrapping_add(LOBBY_API_PASS_CODE));
                    c.g[T0][0] = c.g[S0][0];
                    callee::TAG_FIELD_SET_ENCODED.call(c, 0x003e_8c44)?;
                    c.g[A2][0] = LOBBY_STRINGS_PAGE;
                    c.g[A3][0] = c.lw((c.g[S1][0] as u32).wrapping_add(LOBBY_API_PASS_NUMBER));
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x48e0));
                    // Registers at the call: buf, size, key, number.
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A1][0] = 0x200;
                    callee::TAG_FIELD_SET_NUMBER.call(c, 0x003e_8c5c)?;
                    c.g[S3][0] = c.g[SP][0];
                    pc = 0x003e_8c9c;
                }
                0x003e_8c64 => {
                    c.cycles += 14;
                    c.g[V0][0] = 0x7e;
                    c.g[A2][0] = c.g[A0][0];
                    c.g[A1][0] = 0x1f;
                    c.g[A3][0] = sx((c.g[S1][0] as u32).wrapping_add(LOBBY_API_KEY));
                    c.sb((c.g[SP][0] as u32).wrapping_add(0x200), c.g[V0][0]);
                    c.g[T0][0] = 0x10;
                    c.g[T1][0] = 0x10;
                    // Registers at the call: dst, len, src, key, klen, klen.
                    callee::LOBBY_ENCRYPT_STRING.call(c, 0x003e_8c84)?;
                    c.g[A2][0] = c.g[S2][0];
                    c.g[A3][0] = c.g[S3][0];
                    // Registers at the call: buf, size, key, value.
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A1][0] = 0x200;
                    callee::TAG_FIELD_SET_STRING.call(c, 0x003e_8c98)?;
                    c.g[S3][0] = c.g[SP][0];
                    pc = 0x003e_8c9c;
                }
                0x003e_8c9c => {
                    c.cycles += 1;
                    c.g[V0][0] = 0x736e_0000;
                    pc = 0x003e_8ca0;
                }
                0x003e_8ca0 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[V0][0] | 0x6170;
                    if c.g[S4][0] != c.g[V0][0] {
                        c.g[A0][0] = c.lw(c.g[S1][0] as u32);
                        pc = 0x003e_8d34;
                    } else {
                        pc = 0x003e_8cac;
                    }
                }
                0x003e_8cac => {
                    c.cycles += 12;
                    c.g[A1][0] = LOBBY_STRINGS_PAGE;
                    // Registers at the call: buf, key (find).
                    c.g[A0][0] = c.g[S3][0];
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x48e8));
                    callee::TAG_FIELD_FIND.call(c, 0x003e_8cbc)?;
                    // Registers at the call: field, def (get number).
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A1][0] = 0u64;
                    callee::TAG_FIELD_GET_NUMBER.call(c, 0x003e_8cc8)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x4));
                    c.g[V0][0] = (c.g[V0][0] < 0x4u64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = 0x14;
                    if t {
                        pc = 0x003e_8d20;
                    } else {
                        pc = 0x003e_8cdc;
                    }
                }
                0x003e_8cdc => {
                    c.cycles += 6;
                    c.g[V1][0] = sx((c.g[S1][0] as u32).wrapping_add(LOBBY_API_TABLES));
                    c.mult(0, 4, 4, 2);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    c.g[V0][0] = c.lw(c.g[V1][0] as u32);
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(LOBBY_API_TABLES2));
                    if t {
                        pc = 0x003e_8d20;
                    } else {
                        pc = 0x003e_8cf4;
                    }
                }
                0x003e_8cf4 => {
                    c.cycles += 3;
                    c.g[S2][0] = c.g[V1][0];
                    c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    pc = 0x003e_8d10;
                }
                0x003e_8d00 => {
                    c.cycles += 4;
                    // Registers at the call: list, entry.
                    c.g[A1][0] = c.g[V0][0];
                    callee::LOBBY_LIST_REMOVE.call(c, 0x003e_8d08)?;
                    // Registers at the call: removed entry.
                    c.g[A0][0] = c.g[V0][0];
                    call_ptr(c, callee::LOBBY_MEM_FREE.addr, 0x003e_8d10)?;
                    pc = 0x003e_8d10;
                }
                0x003e_8d10 => {
                    c.cycles += 4;
                    // Registers at the call: hash.
                    c.g[A0][0] = c.lw(c.g[S2][0] as u32);
                    callee::LOBBY_HASH_POP_ANY.call(c, 0x003e_8d18)?;
                    if c.g[V0][0] != 0u64 {
                        c.g[A0][0] = c.lw(c.g[S0][0] as u32);
                        if back_edge_unwinds(c, 0x003e_8d00) {
                            return Ok(());
                        }
                        pc = 0x003e_8d00;
                    } else {
                        pc = 0x003e_8d20;
                    }
                }
                0x003e_8d20 => {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(LOBBY_API_CONNECT_WORD));
                    c.sw((c.g[S1][0] as u32).wrapping_add(0x508), 0u64);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                    c.sw(
                        (c.g[S1][0] as u32).wrapping_add(LOBBY_API_CONNECT_WORD),
                        c.g[V0][0],
                    );
                    c.g[A0][0] = c.lw(c.g[S1][0] as u32);
                    pc = 0x003e_8d34;
                }
                0x003e_8d34 => {
                    c.cycles += 6;
                    c.g[A1][0] = c.g[S4][0];
                    c.g[A3][0] = c.g[S3][0];
                    c.g[A2][0] = 0u64;
                    c.g[T0][0] = 0xffff_ffff_ffff_ffff;
                    // Registers at the call: connection, tag, 0, packet, -1.
                    callee::PROTO_ARIES_SEND.call(c, 0x003e_8d48)?;
                    c.g[V0][0] = c.lw(c.g[S5][0] as u32);
                    pc = 0x003e_8d4c;
                }
                0x003e_8d4c => {
                    c.cycles += 9;
                    let frame = Frame { size: 0x290 };
                    frame.restore(c, S0, 0x280);
                    frame.restore(c, S1, 0x270);
                    frame.restore(c, S2, 0x260);
                    frame.restore(c, S3, 0x250);
                    frame.restore(c, S4, 0x240);
                    frame.restore(c, S5, 0x230);
                    frame.restore_ra(c, 0x220);
                    frame.close(c);
                    return Ok(());
                }
                pc => {
                    c.jump_out(pc, 0x003e_8b10, 0x003e_8d70);
                    return Ok(());
                }
            }
        }
    })
}

/// `int LobbyApi_CancelRequest(api, id)` (0x3e8d70): walk the whole queue
/// at `api+0x53c` and clear word `+0x38` (the reply callback) of every
/// request whose id (`+0x00`) is `id`, or of every request when `id` is -1.
/// Returns 1 when any was cleared, else 0.
pub fn lobby_api_cancel_request(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(LOBBY_API_REQUESTS));
    c.g[A2][0] = 0u64;
    let empty = c.g[V0][0] == 0u64;
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(LOBBY_API_REQUESTS));
    if !empty {
        c.cycles += 2;
        c.g[A3][0] = 0xffff_ffff_ffff_ffff;
        'l_3e8d88: loop {
            c.cycles += 4;
            c.g[V1][0] = c.lw(c.g[A0][0] as u32);
            c.g[V0][0] = c.lw(c.g[V1][0] as u32);
            let found = if c.g[V0][0] == c.g[A1][0] {
                c.sw((c.g[V1][0] as u32).wrapping_add(LOBBY_REQ_ARG2), 0u64);
                true
            } else {
                c.cycles += 2;
                if c.g[A1][0] != c.g[A3][0] {
                    // No match and not match-all: rejoin at the advance,
                    // skipping the found tail.
                    c.g[V0][0] = c.lw(c.g[A0][0] as u32);
                    false
                } else {
                    c.cycles += 1;
                    c.sw((c.g[V1][0] as u32).wrapping_add(LOBBY_REQ_ARG2), 0u64);
                    true
                }
            };
            if found {
                c.cycles += 2;
                c.g[A2][0] = 0x1;
                c.g[V0][0] = c.lw(c.g[A0][0] as u32);
            }
            c.cycles += 3;
            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(LOBBY_REQ_NEXT));
            let more = c.g[V1][0] != 0u64;
            c.g[A0][0] = sx((c.g[V0][0] as u32).wrapping_add(LOBBY_REQ_NEXT));
            if !more {
                break 'l_3e8d88;
            }
            if event_due(c) {
                deliver_event_here(c, 0x003e_8d88);
            }
        }
    }
    c.cycles += 2;
    let t = c.g[RA][0] as u32;
    c.g[V0][0] = c.g[A2][0];
    c.pc = t;
}

/// `int LobbyApi_Ping(api, name, addr, timeout, t0, t1)` (0x3e8dc0): in
/// state `'conn'`, `'idle'`, `'auth'` or `'acct'` (else -1), with a ping
/// channel (else -2) and fewer than 64 pings queued (else -3): queue a
/// `'ping'` request (`t0`, `t1`; -4 when that fails) holding `addr`, the
/// deadline `ProtoAriesTick + timeout` and `name`, send a ping to `addr`
/// carrying `name` and `addr` as tag fields, and count it. Returns the
/// request's id.
pub fn lobby_api_ping(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x160);
        c.g[V0][0] = 0x636f_0000;
        frame.save(c, S0, 0x150);
        c.g[V0][0] = c.g[V0][0] | 0x6e6e;
        frame.save(c, S2, 0x130);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S3, 0x120);
        c.g[S2][0] = c.g[A2][0];
        frame.save(c, S4, 0x110);
        c.g[S3][0] = c.g[A1][0];
        frame.save(c, S1, 0x140);
        c.g[S4][0] = c.g[A3][0];
        frame.save_ra(c, 0x100);
        c.g[A2][0] = c.g[T0][0];
        c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(LOBBY_API_STATE_TAG));
        let t = c.g[V1][0] == c.g[V0][0];
        c.g[A3][0] = c.g[T1][0];
        if t {
            ping_body(c, &frame)?;
            return Ok(());
        }
        c.cycles += 4;
        c.g[V0][0] = 0x6964_0000;
        c.g[V0][0] = c.g[V0][0] | 0x6c65;
        let t = c.g[V1][0] == c.g[V0][0];
        c.g[V0][0] = 0x6175_0000;
        if t {
            ping_body(c, &frame)?;
            return Ok(());
        }
        c.cycles += 3;
        c.g[V0][0] = c.g[V0][0] | 0x7468;
        let t = c.g[V1][0] == c.g[V0][0];
        c.g[V0][0] = 0x6163_0000;
        if t {
            ping_body(c, &frame)?;
            return Ok(());
        }
        c.cycles += 3;
        c.g[V0][0] = c.g[V0][0] | 0x6374;
        let t = c.g[V1][0] != c.g[V0][0];
        c.g[V0][0] = 0xffff_ffff_ffff_ffff;
        if t {
            frame_epilogue_ping(c, &frame);
            return Ok(());
        }
        ping_body(c, &frame)?;
        Ok(())
    })
}

/// 0x3e8e40 body of `lobby_api_ping` after the state-tag check.
pub(crate) fn ping_body(c: &mut Cpu, frame: &Frame) -> crate::call::Flow {
    c.cycles += 3;
    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(LOBBY_API_PING_CHAN));
    if c.g[V0][0] == 0u64 {
        c.cycles += 2;
        c.g[V0][0] = 0xffff_ffff_ffff_fffe;
        frame_epilogue_ping(c, frame);
        return Ok(());
    }
    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(LOBBY_API_PING_COUNT));
    c.cycles += 3;
    c.g[V0][0] = ((c.g[V0][0] as i64) < 64i64) as u64;
    let t = c.g[V0][0] != 0u64;
    c.g[A1][0] = 0x7069_0000;
    if !t {
        c.cycles += 2;
        c.g[V0][0] = 0xffff_ffff_ffff_fffd;
        frame_epilogue_ping(c, frame);
        return Ok(());
    }
    c.cycles += 6;
    // Registers at the call: api, 'ping', t0, t1.
    c.g[A0][0] = c.g[S0][0];
    c.g[A1][0] = c.g[A1][0] | 0x6e67;
    callee::LOBBY_API_ADD_REQUEST.call(c, 0x003e_8e60)?;
    c.g[S1][0] = c.g[V0][0];
    if c.g[S1][0] == 0u64 {
        c.g[V0][0] = 0xffff_ffff_ffff_fffc;
        frame_epilogue_ping(c, frame);
        return Ok(());
    }
    c.cycles += 31;
    c.sw(
        (c.g[S1][0] as u32).wrapping_add(LOBBY_PINGREQ_ADDR),
        c.g[S2][0],
    );
    // Registers at the call: ProtoAries connection.
    c.g[A0][0] = c.lw(c.g[S0][0] as u32);
    callee::PROTO_ARIES_TICK.call(c, 0x003e_8e78)?;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S4][0] as u32));
    c.g[A2][0] = 0x1f;
    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(LOBBY_PINGREQ_NAME));
    c.g[A1][0] = c.g[S3][0];
    c.sw(
        (c.g[S1][0] as u32).wrapping_add(LOBBY_PINGREQ_DEADLINE),
        c.g[V0][0],
    );
    // Registers at the call: dst, src, size (strncpy).
    call_ptr(c, callee::STRNCPY, 0x003e_8e90)?;
    c.g[A2][0] = LOBBY_STRINGS_PAGE;
    c.g[A0][0] = c.g[SP][0];
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x4880));
    c.sb(c.g[SP][0] as u32, 0u64);
    c.g[A3][0] = c.g[S3][0];
    // Registers at the call: buf, size, key, value (set string).
    c.g[A1][0] = 0x100;
    callee::TAG_FIELD_SET_STRING.call(c, 0x003e_8eac)?;
    c.g[A2][0] = LOBBY_STRINGS_PAGE;
    c.g[A0][0] = c.g[SP][0];
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x4890));
    c.g[A1][0] = 0x100;
    c.g[A3][0] = c.g[S2][0];
    // Registers at the call: buf, size, key, addr (set address).
    callee::TAG_FIELD_SET_ADDRESS.call(c, 0x003e_8ec4)?;
    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(LOBBY_API_PING_CHAN));
    c.g[A1][0] = c.g[S2][0];
    c.g[A2][0] = c.g[SP][0];
    c.g[A3][0] = 0x100;
    // Registers at the call: ping channel, addr, buf, size (ping request).
    callee::PROTO_PING_REQUEST.call(c, 0x003e_8ed8)?;
    c.g[V1][0] = c.lw((c.g[S0][0] as u32).wrapping_add(LOBBY_API_PING_COUNT));
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
    c.sw(
        (c.g[S0][0] as u32).wrapping_add(LOBBY_API_PING_COUNT),
        c.g[V1][0],
    );
    c.g[V0][0] = c.lw(c.g[S1][0] as u32);
    frame_epilogue_ping(c, frame);
    Ok(())
}

/// 0x3e8ee8 epilogue of `lobby_api_ping` (V0 already set).
pub(crate) fn frame_epilogue_ping(c: &mut Cpu, frame: &Frame) {
    c.cycles += 8;
    frame.restore(c, S0, 0x150);
    frame.restore(c, S1, 0x140);
    frame.restore(c, S2, 0x130);
    frame.restore(c, S3, 0x120);
    frame.restore(c, S4, 0x110);
    frame.restore_ra(c, 0x100);
    frame.close(c);
}

/// `int LobbyApi_Status(api, tag)` (0x3e8f08): one status word by tag:
/// `'stat'` the state tag (`+0xc`), `'rflg'` `+0x1bc`, `'extn'` `+0x4fc`,
/// `'locl'` `+0x500`, `'inat'` 1 when both of those are set and differ
/// (else 0), `'more'` `+0x504`, `'slot'` `+0x510`; any other tag 0.
pub fn lobby_api_status(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V0][0] = 0x7374_0000;
    c.g[V0][0] = c.g[V0][0] | 0x6174;
    let t = c.g[A1][0] != c.g[V0][0];
    c.g[V0][0] = 0x7266_0000;
    if t {
        c.cycles += 3;
        c.g[V0][0] = c.g[V0][0] | 0x6c67;
        let t = c.g[A1][0] != c.g[V0][0];
        c.g[V0][0] = 0x6578_0000;
        if t {
            c.cycles += 3;
            c.g[V0][0] = c.g[V0][0] | 0x746e;
            let t = c.g[A1][0] != c.g[V0][0];
            c.g[V0][0] = 0x6c6f_0000;
            if t {
                c.cycles += 3;
                c.g[V0][0] = c.g[V0][0] | 0x636c;
                let t = c.g[A1][0] != c.g[V0][0];
                c.g[V0][0] = 0x696e_0000;
                if t {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[V0][0] | 0x6174;
                    let t = c.g[A1][0] != c.g[V0][0];
                    c.g[V0][0] = 0x6d6f_0000;
                    if t {
                        c.cycles += 3;
                        c.g[V0][0] = c.g[V0][0] | 0x7265;
                        let t = c.g[A1][0] != c.g[V0][0];
                        c.g[V0][0] = 0x736c_0000;
                        if t {
                            c.cycles += 3;
                            c.g[V0][0] = c.g[V0][0] | 0x6f74;
                            let t = c.g[A1][0] == c.g[V0][0];
                            if t {
                                c.cycles += 2;
                                let t = c.g[RA][0] as u32;
                                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x510));
                                c.pc = t;
                                return;
                            }
                        } else {
                            c.cycles += 2;
                            let t = c.g[RA][0] as u32;
                            c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x504));
                            c.pc = t;
                            return;
                        }
                    } else {
                        c.cycles += 3;
                        c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4fc));
                        if c.g[V1][0] != 0u64 {
                            c.cycles += 3;
                            c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x500));
                            let both = c.g[V0][0] != 0u64;
                            c.g[V0][0] = c.g[V1][0] ^ c.g[V0][0];
                            if both {
                                c.cycles += 2;
                                let t = c.g[RA][0] as u32;
                                c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
                                c.pc = t;
                                return;
                            }
                        }
                    }
                    c.cycles += 2;
                    let t = c.g[RA][0] as u32;
                    c.g[V0][0] = 0u64;
                    c.pc = t;
                    return;
                }
                c.cycles += 2;
                let t = c.g[RA][0] as u32;
                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x500));
                c.pc = t;
                return;
            }
            c.cycles += 2;
            let t = c.g[RA][0] as u32;
            c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4fc));
            c.pc = t;
            return;
        }
        c.cycles += 2;
        let t = c.g[RA][0] as u32;
        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x1bc));
        c.pc = t;
        return;
    }
    c.cycles += 2;
    let t = c.g[RA][0] as u32;
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(LOBBY_API_STATE_TAG));
    c.pc = t;
}

/// `int LobbyApi_GetEvent(api)` (0x3e8fb8): take the lowest set bit out of
/// the event word at `api+0x8` and return it.
pub fn lobby_api_get_event(c: &mut Cpu) {
    c.cycles += 6;
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(LOBBY_API_EVENTS));
    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
    c.g[V1][0] = c.g[V0][0] & c.g[V1][0];
    c.g[V0][0] = c.g[V0][0] ^ c.g[V1][0];
    let t = c.g[RA][0] as u32;
    c.sw(
        (c.g[A0][0] as u32).wrapping_add(LOBBY_API_EVENTS),
        c.g[V1][0],
    );
    c.pc = t;
}

/// `void LobbyApi_DebugPacket(api, packet)` (0x3e8fd0): log one packet
/// through `LobbyApi_DebugPrintf`: a line with its tag (`+4`) and kind
/// (`+8`) bytes, then one of four lines picked by a scan of the payload
/// text at `+0xc` (empty, control bytes, more than 240 characters, or the
/// text itself).
pub fn lobby_api_debug_packet(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = 0x2d2d_0000;
        frame.save(c, S0, 0x30);
        c.g[V0][0] = c.g[V0][0] | 0x2d2d;
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0x10);
        c.g[V1][0] = 0xffff_ffff_ffff_ffff;
        c.g[T2][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8));
        c.g[T0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4));
        c.g[S1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xc));
        let t = c.g[T0][0] != c.g[V1][0];
        if c.g[T2][0] == 0 {
            c.g[T2][0] = c.g[V0][0];
        }
        if t {
            // Tagged (`+4` not -1): print tag and kind as they are.
            debug_packet_print(c, &frame)?;
            return Ok(());
        }
        // Untagged: the tag prints as '****', the kind is replaced.
        c.cycles += 3;
        c.g[T0][0] = 0x2a2a_0000;
        let t = c.g[T2][0] != c.g[V1][0];
        c.g[T0][0] = c.g[T0][0] | 0x2a2a;
        if t {
            debug_packet_untagged_kind(c, &frame)?;
            return Ok(());
        }
        c.cycles += 3;
        c.g[T2][0] = 0x636f_0000;
        c.g[T2][0] = c.g[T2][0] | 0x6e6e; // kind -1 prints as 'conn'
        debug_packet_print(c, &frame)
    })
}

/// 0x3e9038 of `lobby_api_debug_packet`, the kind fix-up for an untagged
/// packet whose kind is not -1: 0xfefefefe prints as 'disc', anything else
/// as '----'; then [`debug_packet_print`].
pub(crate) fn debug_packet_untagged_kind(c: &mut Cpu, frame: &Frame) -> crate::call::Flow {
    c.cycles += 4;
    c.g[V0][0] = 0xffff_ffff_fefe_0000;
    c.g[V0][0] = c.g[V0][0] | 0xfefe;
    if c.g[T2][0] != c.g[V0][0] {
        c.g[T2][0] = 0x2d2d_0000;
        c.cycles += 1;
        c.g[T2][0] = c.g[T2][0] | 0x2d2d;
        return debug_packet_print(c, frame);
    }
    c.cycles += 3;
    c.g[T2][0] = 0x6469_0000;
    c.g[T2][0] = c.g[T2][0] | 0x7363;
    debug_packet_print(c, frame)
}

/// 0x3e903c of `lobby_api_debug_packet`: print the tag/kind line, then scan
/// the payload text (up to a NUL, a byte of 127 or more, or a control byte
/// other than tab, newline or carriage return) and go on to
/// [`debug_packet_length_line`].
pub(crate) fn debug_packet_print(c: &mut Cpu, frame: &Frame) -> crate::call::Flow {
    c.cycles += 17;
    c.g[V0][0] = sx(((c.g[T2][0] as u32) as i32 >> 8) as u32);
    c.g[A2][0] = sx(((c.g[T0][0] as u32) as i32 >> 24) as u32);
    c.g[T1][0] = c.g[T0][0];
    c.g[T3][0] = sx(((c.g[T2][0] as u32) as i32 >> 16) as u32);
    c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[T2][0]);
    c.g[A3][0] = sx(((c.g[T0][0] as u32) as i32 >> 16) as u32);
    c.g[A1][0] = LOBBY_FORMATS_PAGE;
    c.sw(c.g[SP][0] as u32, c.g[V0][0]);
    c.g[T0][0] = sx(((c.g[T0][0] as u32) as i32 >> 8) as u32);
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x5f48));
    c.g[A0][0] = c.g[S0][0];
    c.g[T2][0] = sx(((c.g[T2][0] as u32) as i32 >> 24) as u32);
    // Registers at the call: api, format, bytes... (debug printf).
    callee::LOBBY_API_DEBUG_PRINTF.call(c, 0x003e_9070)?;
    c.g[V0][0] = c.lb(c.g[S1][0] as u32);
    c.g[A2][0] = 0u64;
    let t = c.g[V0][0] == 0u64;
    c.g[T0][0] = 0u64;
    if t {
        debug_packet_length_line(c, frame)?;
        return Ok(());
    }
    c.cycles += 3;
    c.g[V0][0] = ((c.g[V0][0] as i64) < 127i64) as u64;
    let t = c.g[V0][0] == 0u64;
    if t {
        debug_packet_length_line(c, frame)?;
        return Ok(());
    }
    c.cycles += 3;
    c.g[A3][0] = 0xd;
    c.g[V1][0] = c.g[S1][0];
    c.g[A1][0] = c.lb(c.g[S1][0] as u32);
    'l_3e9098: loop {
        c.cycles += 3;
        c.g[V0][0] = ((c.g[A1][0] as i64) < 32i64) as u64;
        let printable = c.g[V0][0] == 0u64;
        c.g[A0][0] = c.lbu(c.g[V1][0] as u32);
        // Tab/newline skips the advance below (breaks the inner block).
        let mut tab = false;
        if !printable {
            c.cycles += 4;
            c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x9));
            c.g[V0][0] = (c.g[V0][0] < 0x2u64) as u64;
            if c.g[V0][0] != 0u64 {
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
                tab = true;
            } else {
                c.cycles += 2;
                if c.g[A1][0] != c.g[A3][0] {
                    c.g[T0][0] = 0x1;
                    return debug_packet_length_line(c, frame);
                }
            }
        }
        if !tab {
            c.cycles += 1;
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1));
        }
        c.cycles += 3;
        c.g[V0][0] = c.lb(c.g[V1][0] as u32);
        let t = c.g[V0][0] == 0u64;
        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x1));
        if t {
            debug_packet_length_line(c, frame)?;
            return Ok(());
        }
        c.cycles += 3;
        c.g[V0][0] = ((c.g[V0][0] as i64) < 127i64) as u64;
        if c.g[V0][0] != 0u64 {
            c.g[A1][0] = c.lb(c.g[V1][0] as u32);
            if event_due(c) {
                deliver_event_here(c, 0x003e_9098);
            }
            continue 'l_3e9098;
        } else {
            debug_packet_length_line(c, frame)?;
            return Ok(());
        }
    }
}

/// 0x3e90f8/0x3e9118/0x3e9138/0x3e9148 length lines of
/// `lobby_api_debug_packet` (empty payload, control byte seen, more than
/// 240 characters, or the text itself), then the epilogue.
pub(crate) fn debug_packet_length_line(c: &mut Cpu, frame: &Frame) -> crate::call::Flow {
    c.cycles += 2;
    let empty = c.g[A2][0] == 0u64;
    c.g[V0][0] = 0x1;
    if empty {
        c.cycles += 6;
        c.g[A1][0] = LOBBY_FORMATS_PAGE;
        c.g[A0][0] = c.g[S0][0];
        // Registers at the call: api, format (debug printf).
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x5f68));
        callee::LOBBY_API_DEBUG_PRINTF.call(c, 0x003e_90f0)?;
        frame.restore(c, S0, 0x30);
        frame_epilogue_debug_packet(c, frame);
        return Ok(());
    }
    c.cycles += 2;
    let control = c.g[T0][0] == c.g[V0][0];
    c.g[V0][0] = ((c.g[A2][0] as i64) < 241i64) as u64;
    if control {
        c.cycles += 6;
        c.g[A1][0] = LOBBY_FORMATS_PAGE;
        c.g[A0][0] = c.g[S0][0];
        // Registers at the call: api, format (debug printf).
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x5f78));
        callee::LOBBY_API_DEBUG_PRINTF.call(c, 0x003e_9110)?;
        frame.restore(c, S0, 0x30);
        frame_epilogue_debug_packet(c, frame);
        return Ok(());
    }
    c.cycles += 2;
    let long = c.g[V0][0] == 0u64;
    c.g[A1][0] = LOBBY_STRINGS_PAGE;
    if long {
        c.cycles += 6;
        c.g[A1][0] = LOBBY_FORMATS_PAGE;
        c.g[A0][0] = c.g[S0][0];
        // Registers at the call: api, format (debug printf).
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x5f88));
        callee::LOBBY_API_DEBUG_PRINTF.call(c, 0x003e_9130)?;
        frame.restore(c, S0, 0x30);
        frame_epilogue_debug_packet(c, frame);
        return Ok(());
    }
    c.cycles += 5;
    // Registers at the call: api, format, payload (debug printf).
    c.g[A0][0] = c.g[S0][0];
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x48f0));
    c.g[A2][0] = c.g[S1][0];
    callee::LOBBY_API_DEBUG_PRINTF.call(c, 0x003e_9148)?;
    frame.restore(c, S0, 0x30);
    frame_epilogue_debug_packet(c, frame);
    Ok(())
}

/// 0x3e914c epilogue of `lobby_api_debug_packet`.
pub(crate) fn frame_epilogue_debug_packet(c: &mut Cpu, frame: &Frame) {
    c.cycles += 4;
    frame.restore(c, S1, 0x20);
    frame.restore_ra(c, 0x10);
    frame.close(c);
}

/// `int LobbyApi_TrimRankList(api, cap)` (0x3e9160): trim slot 2's list to
/// about `cap` entries: count the entries per rank (`+0`; ranks outside
/// 1-0x1fff become 500 with a default name) in a 0x4000-byte table, walk
/// the ranks from the top to find the cutoff, zero the ranks at or below
/// it, number the others (`+4`) by rank, and delete (hash, list, free) the
/// zeroed entries. Returns the number kept, -1 when slot 2 has no hash or
/// list. A dispatcher: it resumes at every block the original can unwind
/// at.
pub fn lobby_api_trim_rank_list(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc = c.pc;
        loop {
            match pc {
                0x003e_9160 => {
                    c.cycles += 13;
                    let frame = Frame::open(c, 0x80);
                    frame.save(c, S6, 0x10);
                    frame.save(c, S0, 0x70);
                    c.g[S6][0] = c.g[A1][0];
                    frame.save(c, S1, 0x60);
                    frame.save(c, S2, 0x50);
                    frame.save(c, S3, 0x40);
                    frame.save(c, S4, 0x30);
                    frame.save_ra(c, 0);
                    frame.save(c, S5, 0x20);
                    c.g[S5][0] = c
                        .lw((c.g[A0][0] as u32)
                            .wrapping_add(LOBBY_API_TABLES + 2 * LOBBY_SLOT_SIZE));
                    let t = c.g[S5][0] == 0u64;
                    c.g[S3][0] = c
                        .lw((c.g[A0][0] as u32)
                            .wrapping_add(LOBBY_API_TABLES2 + 2 * LOBBY_SLOT_SIZE));
                    if t {
                        pc = 0x003e_919c;
                    } else {
                        pc = 0x003e_9194;
                    }
                }
                0x003e_9194 => {
                    c.cycles += 2;
                    if c.g[S3][0] != 0u64 {
                        c.g[A0][0] = 0x4000;
                        pc = 0x003e_91a4;
                    } else {
                        pc = 0x003e_919c;
                    }
                }
                0x003e_919c => {
                    c.cycles += 2;
                    c.g[V0][0] = 0xffff_ffff_ffff_ffff;
                    pc = 0x003e_934c;
                }
                0x003e_91a4 => {
                    c.cycles += 10;
                    // Registers at the call: size in a0.
                    c.g[S2][0] = 0u64;
                    call_ptr(c, callee::LOBBY_MEM_ALLOC.addr, 0x003e_91ac)?;
                    c.g[S1][0] = 0x1f4;
                    c.g[S4][0] = c.g[V0][0];
                    c.g[A1][0] = 0u64;
                    // Registers at the call: block, zero, size (memset).
                    c.g[A0][0] = c.g[S4][0];
                    c.g[A2][0] = 0x4000;
                    call_ptr(c, callee::MEMSET.addr, 0x003e_91c4)?;
                    c.g[S0][0] = LOBBY_STRINGS_PAGE;
                    pc = 0x003e_9214;
                }
                0x003e_91cc => {
                    c.cycles += 1;
                    pc = 0x003e_91d0;
                }
                0x003e_91d0 => {
                    c.cycles += 4;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                    c.g[V0][0] = (c.g[V0][0] < 0x1fffu64) as u64;
                    if c.g[V0][0] != 0u64 {
                        c.g[V1][0] = c.lw(c.g[A1][0] as u32);
                        pc = 0x003e_91fc;
                    } else {
                        pc = 0x003e_91e0;
                    }
                }
                0x003e_91e0 => {
                    c.cycles += 7;
                    c.sw(c.g[A1][0] as u32, c.g[S1][0]);
                    c.g[A2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x48f8));
                    c.g[V0][0] = c.lw(c.g[A2][0] as u32);
                    c.g[V1][0] = c.lh((c.g[A2][0] as u32).wrapping_add(0x4));
                    c.sw((c.g[A1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                    c.sh((c.g[A1][0] as u32).wrapping_add(0xc), c.g[V1][0]);
                    c.g[V1][0] = c.lw(c.g[A1][0] as u32);
                    pc = 0x003e_91fc;
                }
                0x003e_91fc => {
                    c.cycles += 6;
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                    c.g[V1][0] = sx((c.g[V1][0] as u32) << 1);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S4][0] as u32));
                    c.g[V0][0] = c.lhu(c.g[V1][0] as u32);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                    c.sh(c.g[V1][0] as u32, c.g[V0][0]);
                    pc = 0x003e_9214;
                }
                0x003e_9214 => {
                    c.cycles += 6;
                    // Registers at the call: list, index.
                    c.g[A1][0] = c.g[S2][0];
                    c.g[A0][0] = c.g[S3][0];
                    callee::LOBBY_LIST_GET_AT.call(c, 0x003e_9220)?;
                    c.g[A1][0] = c.g[V0][0];
                    if c.g[A1][0] != 0u64 {
                        c.g[V0][0] = c.lw(c.g[A1][0] as u32);
                        if back_edge_unwinds(c, 0x003e_91d0) {
                            return Ok(());
                        }
                        pc = 0x003e_91d0;
                    } else {
                        pc = 0x003e_922c;
                    }
                }
                0x003e_922c => {
                    c.cycles += 3;
                    c.g[S0][0] = 0x1;
                    c.g[S2][0] = 0x1fff;
                    c.g[V1][0] = sx((c.g[S4][0] as u32).wrapping_add(0x3ffe));
                    pc = 0x003e_9238;
                }
                0x003e_9238 => {
                    c.cycles += 6;
                    c.g[S1][0] = c.lh(c.g[V1][0] as u32);
                    c.sh(c.g[V1][0] as u32, c.g[S0][0]);
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[S1][0] as u32));
                    c.g[V0][0] = ((c.g[S6][0] as i64) < (c.g[S0][0] as i64)) as u64;
                    if c.g[V0][0] == 0u64 {
                        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                        pc = 0x003e_92a0;
                    } else {
                        pc = 0x003e_9250;
                    }
                }
                0x003e_9250 => {
                    c.cycles += 5;
                    // Registers at the call: list.
                    c.g[A0][0] = c.g[S3][0];
                    callee::LOBBY_LIST_COUNT.call(c, 0x003e_9258)?;
                    c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[S0][0] as i64)) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[S0][0] = 0u64;
                    if t {
                        pc = 0x003e_92a8;
                    } else {
                        pc = 0x003e_9264;
                    }
                }
                0x003e_9264 => {
                    c.cycles += 2;
                    c.g[A1][0] = c.g[S0][0];
                    pc = 0x003e_9284;
                }
                0x003e_926c => {
                    c.cycles += 1;
                    pc = 0x003e_9270;
                }
                0x003e_9270 => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    if c.g[V0][0] == 0u64 {
                        c.sw(c.g[A1][0] as u32, 0u64);
                        pc = 0x003e_927c;
                    } else {
                        pc = 0x003e_927c;
                    }
                }
                0x003e_927c => {
                    c.cycles += 2;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    c.g[A1][0] = c.g[S0][0];
                    pc = 0x003e_9284;
                }
                0x003e_9284 => {
                    c.cycles += 5;
                    // Registers at the call: list, index.
                    c.g[A0][0] = c.g[S3][0];
                    callee::LOBBY_LIST_GET_AT.call(c, 0x003e_928c)?;
                    c.g[A1][0] = c.g[V0][0];
                    if c.g[A1][0] != 0u64 {
                        c.g[V0][0] = c.lw(c.g[A1][0] as u32);
                        if back_edge_unwinds(c, 0x003e_9270) {
                            return Ok(());
                        }
                        pc = 0x003e_9270;
                    } else {
                        pc = 0x003e_9298;
                    }
                }
                0x003e_9298 => {
                    c.cycles += 2;
                    c.g[S1][0] = 0u64;
                    pc = 0x003e_92ac;
                }
                0x003e_92a0 => {
                    c.cycles += 2;
                    let t = (c.g[S2][0] as i64) >= 0;
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x2));
                    if t {
                        if back_edge_unwinds(c, 0x003e_9238) {
                            return Ok(());
                        }
                        pc = 0x003e_9238;
                    } else {
                        pc = 0x003e_92a8;
                    }
                }
                0x003e_92a8 => {
                    c.cycles += 1;
                    c.g[S1][0] = 0u64;
                    pc = 0x003e_92ac;
                }
                0x003e_92ac => {
                    c.cycles += 2;
                    c.g[S2][0] = 0u64;
                    pc = 0x003e_92d4;
                }
                0x003e_92b4 => {
                    c.cycles += 1;
                    pc = 0x003e_92b8;
                }
                0x003e_92b8 => {
                    c.cycles += 2;
                    let t = (c.g[V0][0] as i64) <= 0;
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 1);
                    if t {
                        pc = 0x003e_92d0;
                    } else {
                        pc = 0x003e_92c0;
                    }
                }
                0x003e_92c0 => {
                    c.cycles += 4;
                    c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S4][0] as u32));
                    c.g[V1][0] = c.lh(c.g[V0][0] as u32);
                    c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[V1][0]);
                    pc = 0x003e_92d0;
                }
                0x003e_92d0 => {
                    c.cycles += 1;
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                    pc = 0x003e_92d4;
                }
                0x003e_92d4 => {
                    c.cycles += 6;
                    // Registers at the call: list, index.
                    c.g[A1][0] = c.g[S2][0];
                    c.g[A0][0] = c.g[S3][0];
                    callee::LOBBY_LIST_GET_AT.call(c, 0x003e_92e0)?;
                    c.g[A1][0] = c.g[V0][0];
                    if c.g[A1][0] != 0u64 {
                        c.g[V0][0] = c.lw(c.g[A1][0] as u32);
                        if back_edge_unwinds(c, 0x003e_92b8) {
                            return Ok(());
                        }
                        pc = 0x003e_92b8;
                    } else {
                        pc = 0x003e_92ec;
                    }
                }
                0x003e_92ec => {
                    c.cycles += 2;
                    c.g[S2][0] = 0u64;
                    pc = 0x003e_9328;
                }
                0x003e_92f4 => {
                    c.cycles += 1;
                    pc = 0x003e_92f8;
                }
                0x003e_92f8 => {
                    c.cycles += 2;
                    if c.g[V0][0] != 0u64 {
                        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                        pc = 0x003e_9328;
                    } else {
                        pc = 0x003e_9300;
                    }
                }
                0x003e_9300 => {
                    c.cycles += 10;
                    // Registers at the call: hash, name.
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.g[S5][0];
                    callee::LOBBY_HASH_DEL_STR.call(c, 0x003e_930c)?;
                    // Registers at the call: list, entry.
                    c.g[A0][0] = c.g[S3][0];
                    c.g[A1][0] = c.g[V0][0];
                    callee::LOBBY_LIST_REMOVE.call(c, 0x003e_9318)?;
                    // Registers at the call: removed entry.
                    c.g[A0][0] = c.g[V0][0];
                    call_ptr(c, callee::LOBBY_MEM_FREE.addr, 0x003e_9320)?;
                    c.g[A1][0] = c.g[S2][0];
                    pc = 0x003e_932c;
                }
                0x003e_9328 => {
                    c.cycles += 1;
                    c.g[A1][0] = c.g[S2][0];
                    pc = 0x003e_932c;
                }
                0x003e_932c => {
                    c.cycles += 5;
                    // Registers at the call: list, index (a1, set by both
                    // predecessors).
                    c.g[A0][0] = c.g[S3][0];
                    callee::LOBBY_LIST_GET_AT.call(c, 0x003e_9334)?;
                    c.g[A1][0] = c.g[V0][0];
                    if c.g[A1][0] != 0u64 {
                        c.g[V0][0] = c.lw(c.g[A1][0] as u32);
                        if back_edge_unwinds(c, 0x003e_92f8) {
                            return Ok(());
                        }
                        pc = 0x003e_92f8;
                    } else {
                        pc = 0x003e_9340;
                    }
                }
                0x003e_9340 => {
                    c.cycles += 3;
                    // Registers at the call: scratch table.
                    c.g[A0][0] = c.g[S4][0];
                    call_ptr(c, callee::LOBBY_MEM_FREE.addr, 0x003e_9348)?;
                    c.g[V0][0] = c.g[S1][0];
                    pc = 0x003e_934c;
                }
                0x003e_934c => {
                    c.cycles += 10;
                    let frame = Frame { size: 0x80 };
                    frame.restore(c, S0, 0x70);
                    frame.restore(c, S1, 0x60);
                    frame.restore(c, S2, 0x50);
                    frame.restore(c, S3, 0x40);
                    frame.restore(c, S4, 0x30);
                    frame.restore(c, S5, 0x20);
                    frame.restore(c, S6, 0x10);
                    frame.restore_ra(c, 0);
                    frame.close(c);
                    return Ok(());
                }
                pc => {
                    c.jump_out(pc, 0x003e_9160, 0x003e_9374);
                    return Ok(());
                }
            }
        }
    })
}

/// `int LobbyApi_PickRoom(api, want, mask, bits, name, best)` (0x3e9378):
/// sort slot 0's (room) list, then walk its visible entries whose flags
/// (`+8`) masked by `mask` equal `bits` and whose name (`+0x1c`) starts with
/// `name` (a default when null), scoring each `100 * +0x10 / +0xc` (players
/// over capacity?). Returns the index of the fullest room scoring below
/// `want`, else of the emptiest matching room, else `best`. A dispatcher: it
/// resumes at the walk's head.
pub fn lobby_api_pick_room(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame { size: 0xb0 };
        match c.pc {
            0x003e_9378 => {
                c.cycles += 21;
                Frame::open(c, frame.size);
                frame.save(c, S5, 0x50);
                frame.save(c, S3, 0x70);
                c.g[S5][0] = c.g[T1][0];
                frame.save(c, S4, 0x60);
                c.g[S3][0] = c.g[A0][0];
                frame.save(c, S6, 0x40);
                c.g[S4][0] = c.g[T0][0];
                frame.save(c, S7, 0x30);
                c.g[S6][0] = c.g[S5][0];
                frame.save(c, FP, 0x20);
                c.g[S7][0] = 0xffff_ffff_ffff_ffff;
                frame.save(c, S0, 0xa0);
                c.g[FP][0] = 0x3e7;
                frame.save(c, S1, 0x90);
                frame.save(c, S2, 0x80);
                frame.save_ra(c, 0x10);
                c.sw(c.g[SP][0] as u32, c.g[A1][0]);
                c.sw((c.g[SP][0] as u32).wrapping_add(0x4), c.g[A2][0]);
                let named = c.g[S4][0] != 0u64;
                c.sw((c.g[SP][0] as u32).wrapping_add(0x8), c.g[A3][0]);
                if !named {
                    c.cycles += 2;
                    c.g[V0][0] = LOBBY_STRINGS_PAGE;
                    c.g[S4][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4888));
                }
                c.cycles += 3;
                c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(LOBBY_API_TABLES));
                let no_hash = c.g[V0][0] == 0u64;
                c.g[V0][0] = c.g[S5][0];
                if !no_hash {
                    c.cycles += 8;
                    // Registers at the call: list.
                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(LOBBY_API_TABLES2));
                    c.g[S2][0] = 0u64;
                    callee::LOBBY_LIST_SORT.call(c, 0x003e_93ec)?;
                    // Registers at the call: list, the sort's result.
                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(LOBBY_API_TABLES2));
                    c.g[A1][0] = c.g[V0][0];
                    callee::LOBBY_LIST_MARK_DIRTY.call(c, 0x003e_93f8)?;
                    pick_room_walk(c)?;
                }
            }
            PICK_ROOM_NEXT => {
                pick_room_score(c)?;
                pick_room_walk(c)?;
            }
            pc => {
                c.jump_out(pc, 0x003e_9378, 0x003e_94f0);
                return Ok(());
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

/// `LobbyApi_PickRoom`'s walk loop head: `a0` is the list, `s2` the index.
const PICK_ROOM_NEXT: u32 = 0x003e_9400;

/// 0x3e94a0..0x3e94bc: score the visible entries in turn (each back
/// through [`PICK_ROOM_NEXT`]), then pick the result into `v0`.
fn pick_room_walk(c: &mut Cpu) -> crate::call::Flow {
    loop {
        c.cycles += 5;
        // Registers at the call: list.
        c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(LOBBY_API_TABLES2));
        callee::LOBBY_LIST_VISIBLE_COUNT.call(c, 0x003e_94a8)?;
        c.g[V0][0] = ((c.g[S2][0] as i64) < (c.g[V0][0] as i64)) as u64;
        if c.g[V0][0] == 0u64 {
            break;
        }
        c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(LOBBY_API_TABLES2));
        if back_edge_unwinds(c, PICK_ROOM_NEXT) {
            return Err(crate::call::Unwound);
        }
        pick_room_score(c)?;
    }
    c.cycles += 2;
    if (c.g[S7][0] as i64) < 0 {
        c.g[S5][0] = c.g[S6][0];
    }
    c.cycles += 1;
    c.g[V0][0] = c.g[S5][0];
    Ok(())
}

/// 0x3e9400..0x3e9494: score entry `s2` when its flags and name match:
/// `s5`/`s7` keep the fullest below `want`, `s6`/`fp` the emptiest; then
/// the next index.
fn pick_room_score(c: &mut Cpu) -> crate::call::Flow {
    c.cycles += 9;
    // Registers at the call: list, index.
    c.g[A1][0] = c.g[S2][0];
    callee::LOBBY_LIST_GET_AT.call(c, 0x003e_9408)?;
    c.g[S1][0] = c.g[V0][0];
    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x8));
    c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x8));
    if c.g[V0][0] != c.g[V1][0] {
        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
        return Ok(());
    }
    c.cycles += 9;
    // Registers at the call: name (strlen).
    c.g[A0][0] = c.g[S4][0];
    c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1c));
    call_ptr(c, callee::STRLEN.addr, 0x003e_9430)?;
    // Registers at the call: room name, name, len (strncmp).
    c.g[A0][0] = c.g[S0][0];
    c.g[A2][0] = c.g[V0][0];
    c.g[A1][0] = c.g[S4][0];
    call_ptr(c, callee::STRNCMP, 0x003e_9440)?;
    if c.g[V0][0] != 0u64 {
        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
        return Ok(());
    }
    c.cycles += 6;
    c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x10));
    c.g[V1][0] = 0x64;
    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xc));
    c.mult(0, 4, 4, 3);
    c.cycles += 6;
    c.div(0, 4, 2);
    c.g[V0][0] = c.lw(c.g[SP][0] as u32);
    c.g[A0][0] = c.lo[0];
    c.g[V1][0] = ((c.g[A0][0] as i64) < (c.g[V0][0] as i64)) as u64;
    let at_least_want = c.g[V1][0] == 0u64;
    c.g[V0][0] = ((c.g[S7][0] as i64) < (c.g[A0][0] as i64)) as u64;
    if at_least_want {
        c.cycles += 1;
        c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[FP][0] as i64)) as u64;
    } else {
        c.cycles += 2;
        let not_fuller = c.g[V0][0] == 0u64;
        c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[FP][0] as i64)) as u64;
        if !not_fuller {
            c.cycles += 2;
            c.g[S5][0] = c.g[S2][0];
            c.g[S7][0] = c.g[A0][0];
            c.cycles += 1;
            c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[FP][0] as i64)) as u64;
        }
    }
    c.cycles += 2;
    if c.g[V0][0] == 0u64 {
        c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
        return Ok(());
    }
    c.cycles += 3;
    c.g[FP][0] = c.g[A0][0];
    c.g[S6][0] = c.g[S2][0];
    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
    Ok(())
}

/// `void LobbyApi_ServerPingCallback(api, reply)` (0x3e94f0): the reply
/// callback of `PingServer`. Ignores anything but a `'ping'` reply; reads a
/// number field from the reply's text. Non-zero (answered): store it at
/// `+0x1c`, schedule the next ping 30000 ticks out and clear the miss count.
/// Zero: log, count a miss at `+0x20` and ping again at once, or, on the
/// third miss in a row, reset the count instead.
pub fn lobby_api_server_ping_callback(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 0x7069_0000;
        frame.save(c, S0, 0x10);
        c.g[V0][0] = c.g[V0][0] | 0x6e67;
        frame.save_ra(c, 0);
        c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4));
        let t = c.g[V1][0] != c.g[V0][0];
        c.g[S0][0] = c.g[A0][0];
        if t {
            frame_epilogue_server_ping_callback(c, &frame);
            return Ok(());
        }
        c.cycles += 9;
        // Registers at the call: reply text, key (find).
        c.g[A0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xc));
        c.g[A1][0] = LOBBY_STRINGS_PAGE;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4898));
        callee::TAG_FIELD_FIND.call(c, 0x003e_9520)?;
        // Registers at the call: field, def (get number).
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0u64;
        callee::TAG_FIELD_GET_NUMBER.call(c, 0x003e_952c)?;
        let t = c.g[V0][0] != 0u64;
        c.g[A1][0] = LOBBY_FORMATS_PAGE;
        if t {
            // Answered: the 0x3e956c tail, then the shared epilogue.
            server_ping_callback_tick(c)?;
            frame_epilogue_server_ping_callback(c, &frame);
            return Ok(());
        }
        c.cycles += 8;
        // Registers at the call: api, format (debug printf).
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x5f98));
        callee::LOBBY_API_DEBUG_PRINTF.call(c, 0x003e_9540)?;
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(LOBBY_API_SERVER_PING_C));
        c.g[V1][0] = 0x3;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
        let t = c.g[V0][0] != c.g[V1][0];
        c.sw(
            (c.g[S0][0] as u32).wrapping_add(LOBBY_API_SERVER_PING_C),
            c.g[V0][0],
        );
        if t {
            // Fewer than 3 misses: ping again now (0x9564), then epilogue.
            c.cycles += 4;
            callee::NET_TICK_WRAPPER2.call(c, 0x003e_9564)?;
            c.sw(
                (c.g[S0][0] as u32).wrapping_add(LOBBY_API_SERVER_PING_A),
                c.g[V0][0],
            );
            frame_epilogue_server_ping_callback(c, &frame);
            return Ok(());
        }
        c.cycles += 2;
        c.sw(
            (c.g[S0][0] as u32).wrapping_add(LOBBY_API_SERVER_PING_C),
            0u64,
        );
        frame_epilogue_server_ping_callback(c, &frame);
        Ok(())
    })
}

/// 0x3e956c of `lobby_api_server_ping_callback`, an answered ping: store
/// the reply value, schedule the next ping 30000 ticks out, clear the
/// misses.
pub(crate) fn server_ping_callback_tick(c: &mut Cpu) -> crate::call::Flow {
    c.cycles += 5;
    // The reply value, then tick the net clock (no arguments).
    c.sw(
        (c.g[S0][0] as u32).wrapping_add(LOBBY_API_SERVER_PING_B),
        c.g[V0][0],
    );
    callee::NET_TICK_WRAPPER2.call(c, 0x003e_9574)?;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x7530));
    c.sw(
        (c.g[S0][0] as u32).wrapping_add(LOBBY_API_SERVER_PING_C),
        0u64,
    );
    c.sw(
        (c.g[S0][0] as u32).wrapping_add(LOBBY_API_SERVER_PING_A),
        c.g[V0][0],
    );
    Ok(())
}

/// 0x3e9580 epilogue of `lobby_api_server_ping_callback`.
pub(crate) fn frame_epilogue_server_ping_callback(c: &mut Cpu, frame: &Frame) {
    c.cycles += 4;
    frame.restore(c, S0, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
}

/// `void LobbyApi_PingServer(api)` (0x3e9590): `LobbyApi_Ping` the address
/// at `api+0x3b4` with a fixed name, a 3000-tick timeout and
/// `LobbyApi_ServerPingCallback` as the reply callback (`t0`, `t1` = 0).
pub fn lobby_api_ping_server(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = LOBBY_STRINGS_PAGE;
        frame.save_ra(c, 0);
        c.g[T0][0] = 0x003f_0000;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4900));
        c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x6b10));
        c.g[A2][0] = c.lw((c.g[A0][0] as u32).wrapping_add(LOBBY_API_PEER_HOST));
        c.g[A3][0] = 0xbb8;
        c.g[T1][0] = 0u64;
        // Registers at the call: api, name, address, timeout, reply
        // callback (`LobbyApi_ServerPingCallback`), its argument.
        callee::LOBBY_API_PING.call(c, 0x003e_95b8)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// Per-frame lobby pump (`LobbyApi_Update`, 0x3e95c8): sample the clock,
/// run the watchdog, answer ping traffic, then drain one batch of inbound
/// server messages.
///
/// The client word at `+0x00` holds the transport handle and `+0x0c` the
/// state tag (`offl`, `term`, `rdir`, `skey`, `idle`, `conn`, `auth`,
/// `acct`). A null client returns at once; `term` and `offl` skip the
/// network below. Otherwise the pump re-arms the timeout from the sampled
/// tick, runs `LobbyApi_ProcessPings`, then loops on `ProtoAriesPeek`:
/// each message runs `LobbyApi_PopRequest` plus its handler, then the
/// matching tag reads update the cached session, room, user and rank
/// lists, the event flags and the registered callbacks (when set). A ping
/// timer re-arms the server ping on the way out. Tail calls route through
/// the override table so replaced callees are reached too.
pub fn lobby_api_update(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                // Client tick: a null client returns; else sample the clock.
                0x003e95c8 => {
                    c.cycles += 15;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x420));
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x3e0));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x410));
                    c.g[S3][0] = c.g[A0][0];
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x400));
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x3f0));
                    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x3d0));
                    c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x3c0));
                    c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x3b0));
                    c.sq(23, (c.g[SP][0] as u32).wrapping_add(0x3a0));
                    c.sq(30, (c.g[SP][0] as u32).wrapping_add(0x390));
                    c.sd((c.g[SP][0] as u32).wrapping_add(0x380), c.g[RA][0]);
                    c.g[A0][0] = c.lw(c.g[S3][0] as u32);
                    let t = c.g[A0][0] == 0u64;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x410));
                    if t {
                        pc = 0x003eae38;
                        continue;
                    }
                    pc = 0x003e9604;
                    continue;
                }
                // Watchdog: sample the tick, catch up a late client, time out.
                0x003e9604 => {
                    c.cycles += 8;
                    c.g[RA][0] = sx(0x003e960c);
                    callee::PROTO_ARIES_TICK.call(c, 0x003e960c)?;
                    c.g[S7][0] = c.g[V0][0];
                    c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xc));
                    c.g[V0][0] = 0x6f660000;
                    c.g[V0][0] = c.g[V0][0] | 0x666c;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = 0x74650000;
                    if t {
                        pc = 0x003e9670;
                        continue;
                    }
                    pc = 0x003e9624;
                    continue;
                }
                0x003e9624 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x14));
                    let t = c.g[V0][0] == 0u64;
                    c.g[V1][0] = sx((c.g[S7][0] as u32).wrapping_sub(c.g[V0][0] as u32));
                    if t {
                        pc = 0x003e9644;
                        continue;
                    }
                    pc = 0x003e9630;
                    continue;
                }
                0x003e9630 => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[V1][0] as i64) < 5001i64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x10));
                    if t {
                        pc = 0x003e9648;
                        continue;
                    }
                    pc = 0x003e963c;
                    continue;
                }
                0x003e963c => {
                    c.cycles += 2;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x10), c.g[V0][0]);
                    pc = 0x003e9644;
                    continue;
                }
                0x003e9644 => {
                    c.cycles += 1;
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x10));
                    pc = 0x003e9648;
                    continue;
                }
                0x003e9648 => {
                    c.cycles += 3;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[S7][0] as u32));
                    let t = (c.g[V0][0] as i64) >= 0;
                    c.g[V0][0] = 0x74650000;
                    if t {
                        pc = 0x003e966c;
                        continue;
                    }
                    pc = 0x003e9654;
                    continue;
                }
                0x003e9654 => {
                    c.cycles += 6;
                    c.g[A1][0] = 0x74690000;
                    c.g[A0][0] = c.g[S3][0];
                    c.g[A1][0] = c.g[A1][0] | 0x6d65;
                    c.g[RA][0] = sx(0x003e9668);
                    c.g[A2][0] = 0x800;
                    callee::LOBBY_API_TERMINATE.call(c, 0x003e9668)?;
                    c.g[V0][0] = 0x74650000;
                    pc = 0x003e966c;
                    continue;
                }
                0x003e966c => {
                    c.cycles += 1;
                    c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xc));
                    pc = 0x003e9670;
                    continue;
                }
                // State gate: `term` and `offl` skip the network below.
                0x003e9670 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[V0][0] | 0x726d;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x14), c.g[S7][0]);
                    if t {
                        pc = 0x003eae34;
                        continue;
                    }
                    pc = 0x003e967c;
                    continue;
                }
                0x003e967c => {
                    c.cycles += 4;
                    c.g[V0][0] = 0x6f660000;
                    c.g[V0][0] = c.g[V0][0] | 0x666c;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x410));
                    if t {
                        pc = 0x003eae38;
                        continue;
                    }
                    pc = 0x003e968c;
                    continue;
                }
                // Ping service, then take the first inbound message.
                0x003e968c => {
                    c.cycles += 8;
                    c.g[RA][0] = sx(0x003e9694);
                    c.g[A0][0] = c.g[S3][0];
                    callee::LOBBY_API_PROCESS_PINGS.call(c, 0x003e9694)?;
                    c.g[FP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x360));
                    c.g[V0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x364));
                    c.g[V1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x36c), c.g[V0][0]);
                    let t = true;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x370), c.g[V1][0]);
                    if t {
                        pc = 0x003eade8;
                        continue;
                    }
                    pc = 0x003e96ac;
                    continue;
                }
                0x003e96ac => {
                    c.cycles += 1;
                    pc = 0x003e96b0;
                    continue;
                }
                // Message loop head: re-arm the watchdog, check `~png`.
                0x003e96b0 => {
                    c.cycles += 6;
                    c.g[V1][0] = 0x7e700000;
                    c.g[V0][0] = sx((c.g[S7][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x360));
                    c.g[V1][0] = c.g[V1][0] | 0x6e67;
                    let t = c.g[A0][0] != c.g[V1][0];
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x10), c.g[V0][0]);
                    if t {
                        pc = 0x003e9728;
                        continue;
                    }
                    pc = 0x003e96c8;
                    continue;
                }
                0x003e96c8 => {
                    c.cycles += 5;
                    c.g[V0][0] = 0xffffffffffff0000;
                    c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x1c));
                    c.g[V0][0] = c.g[V0][0] | 0xffff;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[A0][0] = c.g[SP][0];
                    if t {
                        pc = 0x003e9708;
                        continue;
                    }
                    pc = 0x003e96dc;
                    continue;
                }
                0x003e96dc => {
                    c.cycles += 11;
                    c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.g[RA][0] = sx(0x003e96e8);
                    c.g[A1][0] = 0x100;
                    callee::TAG_FIELD_STRNZCPY.call(c, 0x003e96e8)?;
                    c.g[A2][0] = 0x4a0000;
                    c.g[A3][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x1c));
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x4908));
                    c.g[A0][0] = c.g[SP][0];
                    c.g[RA][0] = sx(0x003e9700);
                    c.g[A1][0] = 0x100;
                    callee::TAG_FIELD_SET_NUMBER.call(c, 0x003e9700)?;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x368), c.g[SP][0]);
                    c.g[A2][0] = 0xffffffffffffffff;
                    pc = 0x003e9708;
                    continue;
                }
                0x003e9708 => {
                    c.cycles += 8;
                    c.g[T0][0] = c.g[A2][0];
                    c.g[A0][0] = c.lw(c.g[S3][0] as u32);
                    c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x360));
                    c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x364));
                    c.g[RA][0] = sx(0x003e9720);
                    c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    callee::PROTO_ARIES_SEND.call(c, 0x003e9720)?;
                    let t = true;
                    c.g[A0][0] = c.lw(c.g[S3][0] as u32);
                    if t {
                        pc = 0x003eadd4;
                        continue;
                    }
                    pc = 0x003e9728;
                    continue;
                }
                // Dispatch: pop the pending request and run its handler.
                0x003e9728 => {
                    c.cycles += 8;
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x364));
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.g[A3][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x514));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x104), c.g[A0][0]);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x108), c.g[V0][0]);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x10c), c.g[V1][0]);
                    let t = c.g[A3][0] == 0u64;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x100), 0u64);
                    if t {
                        pc = 0x003e9758;
                        continue;
                    }
                    pc = 0x003e9748;
                    continue;
                }
                0x003e9748 => {
                    c.cycles += 4;
                    c.g[A2][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x518));
                    c.g[A0][0] = c.g[S3][0];
                    let t = c.g[A3][0] as u32;
                    c.g[RA][0] = sx(0x003e9758);
                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x100));
                    c.pc = t;
                    call_indirect(c, t);
                    returned(c, 0x003e9758)?;
                    pc = 0x003e9758;
                    continue;
                }
                0x003e9758 => {
                    c.cycles += 7;
                    c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x360));
                    c.g[A0][0] = c.g[S3][0];
                    c.g[RA][0] = sx(0x003e9768);
                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x110));
                    callee::LOBBY_API_POP_REQUEST.call(c, 0x003e9768)?;
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x148));
                    let t = c.g[V0][0] == 0u64;
                    c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x14c));
                    if t {
                        pc = 0x003e9780;
                        continue;
                    }
                    pc = 0x003e9774;
                    continue;
                }
                0x003e9774 => {
                    c.cycles += 3;
                    c.g[A0][0] = c.g[S3][0];
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x003e9780);
                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x100));
                    c.pc = t;
                    call_indirect(c, t);
                    returned(c, 0x003e9780)?;
                    pc = 0x003e9780;
                    continue;
                }
                0x003e9780 => {
                    c.cycles += 5;
                    c.g[V0][0] = 0x72640000;
                    c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xc));
                    c.g[V0][0] = c.g[V0][0] | 0x6972;
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x360));
                    if t {
                        pc = 0x003e9948;
                        continue;
                    }
                    pc = 0x003e9794;
                    continue;
                }
                0x003e9794 => {
                    c.cycles += 4;
                    c.g[V0][0] = 0x40640000;
                    c.g[V0][0] = c.g[V0][0] | 0x6972;

                    if c.g[V1][0] != c.g[V0][0] {
                        c.sw((c.g[SP][0] as u32).wrapping_add(0x360), 0u64);
                        pc = 0x003e9944;
                        continue;
                    }
                    pc = 0x003e97a4;
                    continue;
                }
                0x003e97a4 => {
                    c.cycles += 16;
                    c.g[A0][0] = c.lw(c.g[S3][0] as u32);
                    c.g[A1][0] = 0x6c700000;
                    c.g[RA][0] = sx(0x003e97b4);
                    c.g[A1][0] = c.g[A1][0] | 0x7274;
                    callee::PROTO_ARIES_STATUS.call(c, 0x003e97b4)?;
                    c.g[A0][0] = c.lw(c.g[S3][0] as u32);
                    c.g[A1][0] = 0x6c610000;
                    c.g[A1][0] = c.g[A1][0] | 0x6472;
                    c.g[RA][0] = sx(0x003e97c8);
                    c.g[S1][0] = c.g[V0][0];
                    callee::PROTO_ARIES_STATUS.call(c, 0x003e97c8)?;
                    c.g[A1][0] = 0x4a0000;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x500), c.g[V0][0]);
                    c.g[RA][0] = sx(0x003e97dc);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4910));
                    callee::TAG_FIELD_FIND.call(c, 0x003e97dc)?;
                    let t = c.g[V0][0] != 0u64;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    if t {
                        pc = 0x003e9824;
                        continue;
                    }
                    pc = 0x003e97e4;
                    continue;
                }
                0x003e97e4 => {
                    c.cycles += 16;
                    c.g[A1][0] = 0x4a0000;
                    c.g[RA][0] = sx(0x003e97f0);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4890));
                    callee::TAG_FIELD_FIND.call(c, 0x003e97f0)?;
                    c.g[A1][0] = 0u64;
                    c.g[RA][0] = sx(0x003e97fc);
                    c.g[A0][0] = c.g[V0][0];
                    callee::TAG_FIELD_GET_ADDRESS.call(c, 0x003e97fc)?;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.g[A1][0] = 0x4a0000;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4918));
                    c.g[RA][0] = sx(0x003e9810);
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x3b4), c.g[V0][0]);
                    callee::TAG_FIELD_FIND.call(c, 0x003e9810)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x003e981c);
                    c.g[A1][0] = 0u64;
                    callee::TAG_FIELD_GET_NUMBER.call(c, 0x003e981c)?;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x3b0), c.g[V0][0]);
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    pc = 0x003e9824;
                    continue;
                }
                0x003e9824 => {
                    c.cycles += 23;
                    c.g[A1][0] = 0x4a0000;
                    c.g[RA][0] = sx(0x003e9830);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4920));
                    callee::TAG_FIELD_FIND.call(c, 0x003e9830)?;
                    c.g[A1][0] = 0u64;
                    c.g[RA][0] = sx(0x003e983c);
                    c.g[A0][0] = c.g[V0][0];
                    callee::TAG_FIELD_GET_NUMBER.call(c, 0x003e983c)?;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.g[A1][0] = 0x4a0000;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x48e0));
                    c.g[RA][0] = sx(0x003e9850);
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x4b8), c.g[V0][0]);
                    callee::TAG_FIELD_FIND.call(c, 0x003e9850)?;
                    c.g[V1][0] = 0x4a0000;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[S0][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4888));
                    c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_add(0x4bc));
                    c.g[A2][0] = 0x40;
                    c.g[RA][0] = sx(0x003e986c);
                    c.g[A3][0] = c.g[S0][0];
                    callee::TAG_FIELD_GET_STRING.call(c, 0x003e986c)?;
                    c.g[RA][0] = sx(0x003e9874);
                    c.g[A0][0] = c.lw(c.g[S3][0] as u32);
                    callee::PROTO_ARIES_UNCONNECT.call(c, 0x003e9874)?;
                    c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x3b4));
                    let t = c.g[V1][0] == 0u64;
                    c.g[V0][0] = 0x636f0000;
                    if t {
                        pc = 0x003e9910;
                        continue;
                    }
                    pc = 0x003e9880;
                    continue;
                }
                0x003e9880 => {
                    c.cycles += 36;
                    c.g[A0][0] = c.g[S3][0];
                    c.g[V0][0] = c.g[V0][0] | 0x6e6e;
                    c.g[RA][0] = sx(0x003e9890);
                    c.sw((c.g[S3][0] as u32).wrapping_add(0xc), c.g[V0][0]);
                    callee::LOBBY_API_PING_SERVER.call(c, 0x003e9890)?;
                    c.g[A3][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x3b0));
                    c.g[A1][0] = 0x490000;
                    c.g[A2][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x3b4));
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x5f30));
                    c.g[RA][0] = sx(0x003e98a8);
                    c.g[A0][0] = c.g[S3][0];
                    callee::LOBBY_API_DEBUG_PRINTF.call(c, 0x003e98a8)?;
                    c.g[A2][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x3b0));
                    c.g[A0][0] = c.lw(c.g[S3][0] as u32);
                    c.g[RA][0] = sx(0x003e98b8);
                    c.g[A1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x3b4));
                    callee::PROTO_ARIES_CONNECT.call(c, 0x003e98b8)?;
                    c.g[A3][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x500));
                    c.g[A2][0] = 0x4a0000;
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x4890));
                    c.g[A0][0] = c.g[SP][0];
                    c.sb(c.g[SP][0] as u32, 0u64);
                    c.g[RA][0] = sx(0x003e98d4);
                    c.g[A1][0] = 0x100;
                    callee::TAG_FIELD_SET_ADDRESS.call(c, 0x003e98d4)?;
                    c.g[A2][0] = 0x4a0000;
                    c.g[A3][0] = c.g[S1][0];
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x4918));
                    c.g[A0][0] = c.g[SP][0];
                    c.g[RA][0] = sx(0x003e98ec);
                    c.g[A1][0] = 0x100;
                    callee::TAG_FIELD_SET_NUMBER.call(c, 0x003e98ec)?;
                    c.g[A1][0] = 0x61640000;
                    c.g[A0][0] = c.lw(c.g[S3][0] as u32);
                    c.g[A1][0] = c.g[A1][0] | 0x6472;
                    c.g[A2][0] = 0u64;
                    c.g[A3][0] = c.g[SP][0];
                    c.g[RA][0] = sx(0x003e9908);
                    c.g[T0][0] = 0xffffffffffffffff;
                    callee::PROTO_ARIES_SEND.call(c, 0x003e9908)?;
                    let t = true;

                    if t {
                        pc = 0x003eae08;
                        continue;
                    }
                    pc = 0x003e9910;
                    continue;
                }
                0x003e9910 => {
                    c.cycles += 13;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.g[A1][0] = 0x4a0000;
                    c.g[RA][0] = sx(0x003e9920);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4928));
                    callee::TAG_FIELD_FIND.call(c, 0x003e9920)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A3][0] = c.g[S0][0];
                    c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_add(0x3b8));
                    c.g[RA][0] = sx(0x003e9934);
                    c.g[A2][0] = 0x100;
                    callee::TAG_FIELD_GET_STRING.call(c, 0x003e9934)?;
                    c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = c.g[V1][0] | 0x400;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x360), 0u64);
                    pc = 0x003e9944;
                    continue;
                }
                0x003e9944 => {
                    c.cycles += 1;
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x360));
                    pc = 0x003e9948;
                    continue;
                }
                0x003e9948 => {
                    c.cycles += 3;
                    c.g[V0][0] = 0xffffffffffffffff;
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[V0][0] = 0x736b0000;
                    if t {
                        pc = 0x003e99c8;
                        continue;
                    }
                    pc = 0x003e9954;
                    continue;
                }
                0x003e9954 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x364));
                    let t = c.g[V0][0] != c.g[V1][0];
                    c.g[V0][0] = 0x736b0000;
                    if t {
                        pc = 0x003e99c8;
                        continue;
                    }
                    pc = 0x003e9960;
                    continue;
                }
                0x003e9960 => {
                    c.cycles += 26;
                    c.g[A2][0] = 0x4a0000;
                    c.g[A3][0] = 0x490000;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x4930));
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x5fb0));
                    c.sb(c.g[SP][0] as u32, 0u64);
                    c.g[A1][0] = 0x100;
                    c.g[RA][0] = sx(0x003e9984);
                    c.g[T0][0] = 0xa;
                    callee::TAG_FIELD_SET_BINARY.call(c, 0x003e9984)?;
                    c.g[A0][0] = c.lw(c.g[S3][0] as u32);
                    c.g[A1][0] = 0x736b0000;
                    c.g[A1][0] = c.g[A1][0] | 0x6579;
                    c.g[A2][0] = 0u64;
                    c.g[A3][0] = c.g[SP][0];
                    c.g[RA][0] = sx(0x003e99a0);
                    c.g[T0][0] = 0xffffffffffffffff;
                    callee::PROTO_ARIES_SEND.call(c, 0x003e99a0)?;
                    c.g[V1][0] = 0x736b0000;
                    c.g[V0][0] = 0x636f0000;
                    c.g[A0][0] = 0x1;
                    c.g[V1][0] = c.g[V1][0] | 0x6579;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x504), c.g[A0][0]);
                    c.g[V0][0] = c.g[V0][0] | 0x6e6e;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0xc), c.g[V1][0]);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x360), c.g[V0][0]);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x364), 0u64);
                    c.g[V0][0] = 0x736b0000;
                    pc = 0x003e99c8;
                    continue;
                }
                0x003e99c8 => {
                    c.cycles += 4;
                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xc));
                    c.g[V0][0] = c.g[V0][0] | 0x6579;
                    let t = c.g[A0][0] != c.g[V0][0];
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x360));
                    if t {
                        pc = 0x003e9a7c;
                        continue;
                    }
                    pc = 0x003e99d8;
                    continue;
                }
                0x003e99d8 => {
                    c.cycles += 2;
                    let t = c.g[V1][0] == c.g[A0][0];
                    c.g[V0][0] = 0x73650000;
                    if t {
                        pc = 0x003e99ec;
                        continue;
                    }
                    pc = 0x003e99e0;
                    continue;
                }
                0x003e99e0 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[V0][0] | 0x6c65;
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[V0][0] = 0xffffffffffffffff;
                    if t {
                        pc = 0x003e9a80;
                        continue;
                    }
                    pc = 0x003e99ec;
                    continue;
                }
                0x003e99ec => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x364));

                    if c.g[V0][0] != 0u64 {
                        c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x360));
                        pc = 0x003e9a7c;
                        continue;
                    }
                    pc = 0x003e99f8;
                    continue;
                }
                0x003e99f8 => {
                    c.cycles += 2;

                    if c.g[V1][0] != c.g[A0][0] {
                        c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
                        pc = 0x003e9a24;
                        continue;
                    }
                    pc = 0x003e9a00;
                    continue;
                }
                0x003e9a00 => {
                    c.cycles += 9;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.g[A1][0] = 0x4a0000;
                    c.g[RA][0] = sx(0x003e9a10);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4930));
                    callee::TAG_FIELD_FIND.call(c, 0x003e9a10)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_add(0x24));
                    c.g[RA][0] = sx(0x003e9a20);
                    c.g[A2][0] = 0x10;
                    callee::TAG_FIELD_DECODE_HEX.call(c, 0x003e9a20)?;
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
                    pc = 0x003e9a24;
                    continue;
                }
                0x003e9a24 => {
                    c.cycles += 12;
                    c.g[V1][0] = 0x69640000;
                    c.g[A1][0] = 0x636f0000;
                    c.g[V1][0] = c.g[V1][0] | 0x6c65;
                    c.g[V0][0] = c.g[V0][0] | 0x1;
                    c.g[A3][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x52c));
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                    c.g[A1][0] = c.g[A1][0] | 0x6e6e;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0xc), c.g[V1][0]);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x360), c.g[A1][0]);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x364), 0u64);
                    let t = c.g[A3][0] == 0u64;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x18), c.g[S7][0]);
                    if t {
                        pc = 0x003e9a78;
                        continue;
                    }
                    pc = 0x003e9a54;
                    continue;
                }
                0x003e9a54 => {
                    c.cycles += 9;
                    c.g[A2][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x530));
                    c.g[V0][0] = 0x3;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x104), c.g[A1][0]);
                    c.g[A0][0] = c.g[S3][0];
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x100), c.g[V0][0]);
                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x100));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x108), 0u64);
                    let t = c.g[A3][0] as u32;
                    c.g[RA][0] = sx(0x003e9a78);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x10c), 0u64);
                    c.pc = t;
                    call_indirect(c, t);
                    returned(c, 0x003e9a78)?;
                    pc = 0x003e9a78;
                    continue;
                }
                0x003e9a78 => {
                    c.cycles += 1;
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x360));
                    pc = 0x003e9a7c;
                    continue;
                }
                0x003e9a7c => {
                    c.cycles += 1;
                    c.g[V0][0] = 0xffffffffffffffff;
                    pc = 0x003e9a80;
                    continue;
                }
                0x003e9a80 => {
                    c.cycles += 2;
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[S0][0] = 0x61750000;
                    if t {
                        pc = 0x003e9ac4;
                        continue;
                    }
                    pc = 0x003e9a88;
                    continue;
                }
                0x003e9a88 => {
                    c.cycles += 5;
                    c.g[V0][0] = 0xfffffffffefe0000;
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x364));
                    c.g[V0][0] = c.g[V0][0] | 0xfefe;
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x364));
                    if t {
                        pc = 0x003e9ac8;
                        continue;
                    }
                    pc = 0x003e9a9c;
                    continue;
                }
                0x003e9a9c => {
                    c.cycles += 5;
                    c.g[V0][0] = 0x636f0000;
                    c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xc));
                    c.g[V0][0] = c.g[V0][0] | 0x6e6e;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[A0][0] = c.g[S3][0];
                    if t {
                        pc = 0x003e9ac4;
                        continue;
                    }
                    pc = 0x003e9ab0;
                    continue;
                }
                0x003e9ab0 => {
                    c.cycles += 5;
                    c.g[A1][0] = 0xfffffffffefe0000;
                    c.g[A1][0] = c.g[A1][0] | 0xfefe;
                    c.g[RA][0] = sx(0x003e9ac0);
                    c.g[A2][0] = 0x800;
                    callee::LOBBY_API_TERMINATE.call(c, 0x003e9ac0)?;
                    c.g[S0][0] = 0x61750000;
                    pc = 0x003e9ac4;
                    continue;
                }
                0x003e9ac4 => {
                    c.cycles += 1;
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x364));
                    pc = 0x003e9ac8;
                    continue;
                }
                0x003e9ac8 => {
                    c.cycles += 3;
                    c.g[S0][0] = c.g[S0][0] | 0x7468;
                    let t = c.g[V0][0] != c.g[S0][0];
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x360));
                    if t {
                        pc = 0x003e9aec;
                        continue;
                    }
                    pc = 0x003e9ad4;
                    continue;
                }
                0x003e9ad4 => {
                    c.cycles += 6;
                    c.g[A1][0] = 0x61750000;
                    c.g[A0][0] = c.g[S3][0];
                    c.g[A1][0] = c.g[A1][0] | 0x7468;
                    c.g[RA][0] = sx(0x003e9ae8);
                    c.g[A2][0] = 0x2;
                    callee::LOBBY_API_TERMINATE.call(c, 0x003e9ae8)?;
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x360));
                    pc = 0x003e9aec;
                    continue;
                }
                0x003e9aec => {
                    c.cycles += 2;
                    let t = c.g[V1][0] != c.g[S0][0];
                    c.g[V0][0] = 0x61630000;
                    if t {
                        pc = 0x003e9b5c;
                        continue;
                    }
                    pc = 0x003e9af4;
                    continue;
                }
                0x003e9af4 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x364));
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x61630000;
                    if t {
                        pc = 0x003e9b5c;
                        continue;
                    }
                    pc = 0x003e9b00;
                    continue;
                }
                0x003e9b00 => {
                    c.cycles += 23;
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
                    c.g[A1][0] = 0x4a0000;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4880));
                    c.g[V0][0] = c.g[V0][0] | 0x2;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0xc), c.g[V1][0]);
                    c.g[RA][0] = sx(0x003e9b20);
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                    callee::TAG_FIELD_FIND.call(c, 0x003e9b20)?;
                    c.g[A3][0] = 0x4a0000;
                    c.g[A2][0] = 0x40;
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x4888));
                    c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_add(0x134));
                    c.g[RA][0] = sx(0x003e9b38);
                    c.g[A0][0] = c.g[V0][0];
                    callee::TAG_FIELD_GET_STRING.call(c, 0x003e9b38)?;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.g[A1][0] = 0x4a0000;
                    c.g[RA][0] = sx(0x003e9b48);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4890));
                    callee::TAG_FIELD_FIND.call(c, 0x003e9b48)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x003e9b54);
                    c.g[A1][0] = 0u64;
                    callee::TAG_FIELD_GET_ADDRESS.call(c, 0x003e9b54)?;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x4fc), c.g[V0][0]);
                    c.g[V0][0] = 0x61630000;
                    pc = 0x003e9b5c;
                    continue;
                }
                0x003e9b5c => {
                    c.cycles += 4;
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x360));
                    c.g[V0][0] = c.g[V0][0] | 0x6374;
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[V0][0] = 0x736e0000;
                    if t {
                        pc = 0x003e9b90;
                        continue;
                    }
                    pc = 0x003e9b6c;
                    continue;
                }
                0x003e9b6c => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x364));
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x736e0000;
                    if t {
                        pc = 0x003e9b90;
                        continue;
                    }
                    pc = 0x003e9b78;
                    continue;
                }
                0x003e9b78 => {
                    c.cycles += 6;
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
                    c.sw((c.g[S3][0] as u32).wrapping_add(0xc), c.g[V1][0]);
                    c.g[V0][0] = c.g[V0][0] | 0x2;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                    c.g[V0][0] = 0x736e0000;
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x360));
                    pc = 0x003e9b90;
                    continue;
                }
                0x003e9b90 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[V0][0] | 0x6170;
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[V0][0] = 0x73650000;
                    if t {
                        pc = 0x003e9bdc;
                        continue;
                    }
                    pc = 0x003e9b9c;
                    continue;
                }
                0x003e9b9c => {
                    c.cycles += 4;
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x50c));
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x1));
                    let t = c.g[V0][0] != 0u64;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x50c), c.g[V0][0]);
                    if t {
                        pc = 0x003e9bd8;
                        continue;
                    }
                    pc = 0x003e9bac;
                    continue;
                }
                0x003e9bac => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x364));
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x73650000;
                    if t {
                        pc = 0x003e9bdc;
                        continue;
                    }
                    pc = 0x003e9bb8;
                    continue;
                }
                0x003e9bb8 => {
                    c.cycles += 8;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.g[A1][0] = 0x4a0000;
                    c.g[RA][0] = sx(0x003e9bc8);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x48e8));
                    callee::TAG_FIELD_FIND.call(c, 0x003e9bc8)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x003e9bd4);
                    c.g[A1][0] = 0u64;
                    callee::TAG_FIELD_GET_NUMBER.call(c, 0x003e9bd4)?;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x508), c.g[V0][0]);
                    pc = 0x003e9bd8;
                    continue;
                }
                0x003e9bd8 => {
                    c.cycles += 1;
                    c.g[V0][0] = 0x73650000;
                    pc = 0x003e9bdc;
                    continue;
                }
                0x003e9bdc => {
                    c.cycles += 4;
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x360));
                    c.g[V0][0] = c.g[V0][0] | 0x6c65;
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[V0][0] = 0x70650000;
                    if t {
                        pc = 0x003e9c40;
                        continue;
                    }
                    pc = 0x003e9bec;
                    continue;
                }
                0x003e9bec => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x364));
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x70650000;
                    if t {
                        pc = 0x003e9c40;
                        continue;
                    }
                    pc = 0x003e9bf8;
                    continue;
                }
                0x003e9bf8 => {
                    c.cycles += 18;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.g[A1][0] = 0x4a0000;
                    c.g[RA][0] = sx(0x003e9c08);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4938));
                    callee::TAG_FIELD_FIND.call(c, 0x003e9c08)?;
                    c.g[A1][0] = 0u64;
                    c.g[RA][0] = sx(0x003e9c14);
                    c.g[A0][0] = c.g[V0][0];
                    callee::TAG_FIELD_GET_NUMBER.call(c, 0x003e9c14)?;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.g[A1][0] = 0x4a0000;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4940));
                    c.g[RA][0] = sx(0x003e9c28);
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x504), c.g[V0][0]);
                    callee::TAG_FIELD_FIND.call(c, 0x003e9c28)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x003e9c34);
                    c.g[A1][0] = 0u64;
                    callee::TAG_FIELD_GET_NUMBER.call(c, 0x003e9c34)?;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x510), c.g[V0][0]);
                    c.g[V0][0] = 0x70650000;
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x360));
                    pc = 0x003e9c40;
                    continue;
                }
                0x003e9c40 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[V0][0] | 0x7273;
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[V0][0] = 0x2b730000;
                    if t {
                        pc = 0x003e9c88;
                        continue;
                    }
                    pc = 0x003e9c4c;
                    continue;
                }
                0x003e9c4c => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x364));
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x2b730000;
                    if t {
                        pc = 0x003e9c88;
                        continue;
                    }
                    pc = 0x003e9c58;
                    continue;
                }
                0x003e9c58 => {
                    c.cycles += 12;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.g[A1][0] = 0x4a0000;
                    c.g[RA][0] = sx(0x003e9c68);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4948));
                    callee::TAG_FIELD_FIND.call(c, 0x003e9c68)?;
                    c.g[A3][0] = 0x4a0000;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x4888));
                    c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_add(0x174));
                    c.g[RA][0] = sx(0x003e9c80);
                    c.g[A2][0] = 0x40;
                    callee::TAG_FIELD_GET_STRING.call(c, 0x003e9c80)?;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x18), c.g[S7][0]);
                    c.g[V0][0] = 0x2b730000;
                    pc = 0x003e9c88;
                    continue;
                }
                0x003e9c88 => {
                    c.cycles += 4;
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x360));
                    c.g[V0][0] = c.g[V0][0] | 0x6573;
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[V0][0] = 0x2b6d0000;
                    if t {
                        pc = 0x003e9ec4;
                        continue;
                    }
                    pc = 0x003e9c98;
                    continue;
                }
                0x003e9c98 => {
                    c.cycles += 125;
                    c.g[S1][0] = sx((c.g[S3][0] as u32).wrapping_add(0x200));
                    c.g[A2][0] = 0x110;
                    c.g[A1][0] = 0u64;
                    c.g[RA][0] = sx(0x003e9cac);
                    c.g[A0][0] = c.g[S1][0];
                    callee::MEMSET.call(c, 0x003e9cac)?;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.g[A1][0] = 0x4a0000;
                    c.g[RA][0] = sx(0x003e9cbc);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4880));
                    callee::TAG_FIELD_FIND.call(c, 0x003e9cbc)?;
                    c.g[S0][0] = 0x4a0000;
                    c.g[A1][0] = c.g[S1][0];
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4888));
                    c.g[A2][0] = 0x20;
                    c.g[A3][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x003e9cd8);
                    c.g[A0][0] = c.g[V0][0];
                    callee::TAG_FIELD_GET_STRING.call(c, 0x003e9cd8)?;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.g[A1][0] = 0x4a0000;
                    c.g[RA][0] = sx(0x003e9ce8);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4950));
                    callee::TAG_FIELD_FIND.call(c, 0x003e9ce8)?;
                    c.g[A2][0] = 0x20;
                    c.g[A3][0] = c.g[S0][0];
                    c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_add(0x220));
                    c.g[RA][0] = sx(0x003e9cfc);
                    c.g[A0][0] = c.g[V0][0];
                    callee::TAG_FIELD_GET_STRING.call(c, 0x003e9cfc)?;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.g[A1][0] = 0x4a0000;
                    c.g[RA][0] = sx(0x003e9d0c);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4958));
                    callee::TAG_FIELD_FIND.call(c, 0x003e9d0c)?;
                    c.g[A2][0] = 0x20;
                    c.g[A3][0] = c.g[S0][0];
                    c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_add(0x240));
                    c.g[RA][0] = sx(0x003e9d20);
                    c.g[A0][0] = c.g[V0][0];
                    callee::TAG_FIELD_GET_STRING.call(c, 0x003e9d20)?;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.g[A1][0] = 0x4a0000;
                    c.g[RA][0] = sx(0x003e9d30);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4960));
                    callee::TAG_FIELD_FIND.call(c, 0x003e9d30)?;
                    c.g[A2][0] = 0x20;
                    c.g[A3][0] = c.g[S0][0];
                    c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_add(0x260));
                    c.g[RA][0] = sx(0x003e9d44);
                    c.g[A0][0] = c.g[V0][0];
                    callee::TAG_FIELD_GET_STRING.call(c, 0x003e9d44)?;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.g[A1][0] = 0x4a0000;
                    c.g[RA][0] = sx(0x003e9d54);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4968));
                    callee::TAG_FIELD_FIND.call(c, 0x003e9d54)?;
                    c.g[A2][0] = 0x10;
                    c.g[A3][0] = c.g[S0][0];
                    c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_add(0x280));
                    c.g[RA][0] = sx(0x003e9d68);
                    c.g[A0][0] = c.g[V0][0];
                    callee::TAG_FIELD_GET_STRING.call(c, 0x003e9d68)?;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.g[A1][0] = 0x4a0000;
                    c.g[RA][0] = sx(0x003e9d78);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4970));
                    callee::TAG_FIELD_FIND.call(c, 0x003e9d78)?;
                    c.g[A2][0] = 0x10;
                    c.g[A3][0] = c.g[S0][0];
                    c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_add(0x290));
                    c.g[RA][0] = sx(0x003e9d8c);
                    c.g[A0][0] = c.g[V0][0];
                    callee::TAG_FIELD_GET_STRING.call(c, 0x003e9d8c)?;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.g[A1][0] = 0x4a0000;
                    c.g[RA][0] = sx(0x003e9d9c);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4978));
                    callee::TAG_FIELD_FIND.call(c, 0x003e9d9c)?;
                    c.g[A2][0] = 0x10;
                    c.g[A3][0] = c.g[S0][0];
                    c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_add(0x2a0));
                    c.g[RA][0] = sx(0x003e9db0);
                    c.g[A0][0] = c.g[V0][0];
                    callee::TAG_FIELD_GET_STRING.call(c, 0x003e9db0)?;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.g[A1][0] = 0x4a0000;
                    c.g[RA][0] = sx(0x003e9dc0);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4980));
                    callee::TAG_FIELD_FIND.call(c, 0x003e9dc0)?;
                    c.g[A2][0] = 0x10;
                    c.g[A3][0] = c.g[S0][0];
                    c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_add(0x2b0));
                    c.g[RA][0] = sx(0x003e9dd4);
                    c.g[A0][0] = c.g[V0][0];
                    callee::TAG_FIELD_GET_STRING.call(c, 0x003e9dd4)?;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.g[A1][0] = 0x4a0000;
                    c.g[RA][0] = sx(0x003e9de4);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4988));
                    callee::TAG_FIELD_FIND.call(c, 0x003e9de4)?;
                    c.g[A3][0] = c.g[S0][0];
                    c.g[A2][0] = 0x40;
                    c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_add(0x2d0));
                    c.g[RA][0] = sx(0x003e9df8);
                    c.g[A0][0] = c.g[V0][0];
                    callee::TAG_FIELD_GET_STRING.call(c, 0x003e9df8)?;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.g[A1][0] = 0x4a0000;
                    c.g[RA][0] = sx(0x003e9e08);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4890));
                    callee::TAG_FIELD_FIND.call(c, 0x003e9e08)?;
                    c.g[A1][0] = 0u64;
                    c.g[RA][0] = sx(0x003e9e14);
                    c.g[A0][0] = c.g[V0][0];
                    callee::TAG_FIELD_GET_ADDRESS.call(c, 0x003e9e14)?;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.g[A1][0] = 0x4a0000;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4990));
                    c.g[RA][0] = sx(0x003e9e28);
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x2c0), c.g[V0][0]);
                    callee::TAG_FIELD_FIND.call(c, 0x003e9e28)?;
                    c.g[A1][0] = 0u64;
                    c.g[RA][0] = sx(0x003e9e34);
                    c.g[A0][0] = c.g[V0][0];
                    callee::TAG_FIELD_GET_ADDRESS.call(c, 0x003e9e34)?;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.g[A1][0] = 0x4a0000;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4998));
                    c.g[RA][0] = sx(0x003e9e48);
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x2c4), c.g[V0][0]);
                    callee::TAG_FIELD_FIND.call(c, 0x003e9e48)?;
                    c.g[A1][0] = 0u64;
                    c.g[RA][0] = sx(0x003e9e54);
                    c.g[A0][0] = c.g[V0][0];
                    callee::TAG_FIELD_GET_NUMBER.call(c, 0x003e9e54)?;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.g[A1][0] = 0x4a0000;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x49a0));
                    c.g[RA][0] = sx(0x003e9e68);
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x2c8), c.g[V0][0]);
                    callee::TAG_FIELD_FIND.call(c, 0x003e9e68)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x003e9e74);
                    c.g[A1][0] = 0u64;
                    callee::TAG_FIELD_GET_DATE.call(c, 0x003e9e74)?;
                    c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
                    c.g[T0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x534));
                    c.g[V1][0] = c.g[V1][0] | 0x200;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x2cc), c.g[V0][0]);
                    let t = c.g[T0][0] == 0u64;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                    if t {
                        pc = 0x003e9ebc;
                        continue;
                    }
                    pc = 0x003e9e8c;
                    continue;
                }
                0x003e9e8c => {
                    c.cycles += 12;
                    c.g[V0][0] = 0x706c0000;
                    c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.g[A2][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x538));
                    c.g[V1][0] = 0x4;
                    c.g[V0][0] = c.g[V0][0] | 0x6179;
                    c.g[A0][0] = c.g[S3][0];
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x100), c.g[V1][0]);
                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x100));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x104), c.g[V0][0]);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x10c), c.g[A3][0]);
                    let t = c.g[T0][0] as u32;
                    c.g[RA][0] = sx(0x003e9ebc);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x108), 0u64);
                    c.pc = t;
                    call_indirect(c, t);
                    returned(c, 0x003e9ebc)?;
                    pc = 0x003e9ebc;
                    continue;
                }
                0x003e9ebc => {
                    c.cycles += 2;
                    c.g[V0][0] = 0x2b6d0000;
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x360));
                    pc = 0x003e9ec4;
                    continue;
                }
                0x003e9ec4 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[V0][0] | 0x7367;
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[V0][0] = 0x2b770000;
                    if t {
                        pc = 0x003e9f64;
                        continue;
                    }
                    pc = 0x003e9ed0;
                    continue;
                }
                0x003e9ed0 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x51c));
                    let t = c.g[V0][0] == 0u64;
                    c.g[S0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x51c));
                    if t {
                        pc = 0x003e9f60;
                        continue;
                    }
                    pc = 0x003e9edc;
                    continue;
                }
                0x003e9edc => {
                    c.cycles += 18;
                    c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.g[V0][0] = 0x63680000;
                    c.g[V1][0] = 0x1;
                    c.g[V0][0] = c.g[V0][0] | 0x6174;
                    c.g[A1][0] = 0x4a0000;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x100), c.g[V1][0]);
                    c.g[A0][0] = c.g[A2][0];
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x104), c.g[V0][0]);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x49a8));
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x108), 0u64);
                    c.g[RA][0] = sx(0x003e9f0c);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x10c), c.g[A2][0]);
                    callee::TAG_FIELD_FIND.call(c, 0x003e9f0c)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x003e9f18);
                    c.g[A1][0] = 0u64;
                    callee::TAG_FIELD_GET_FLAGS.call(c, 0x003e9f18)?;
                    c.g[V1][0] = c.g[V0][0] & 0x4;
                    let t = c.g[V1][0] == 0u64;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x108), c.g[V0][0]);
                    if t {
                        pc = 0x003e9f30;
                        continue;
                    }
                    pc = 0x003e9f24;
                    continue;
                }
                0x003e9f24 => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x63610000;
                    c.g[V0][0] = c.g[V0][0] | 0x7374;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x104), c.g[V0][0]);
                    pc = 0x003e9f30;
                    continue;
                }
                0x003e9f30 => {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x108));
                    c.g[V1][0] = 0x10000;
                    c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = 0x70720000;
                    if t {
                        pc = 0x003e9f4c;
                        continue;
                    }
                    pc = 0x003e9f44;
                    continue;
                }
                0x003e9f44 => {
                    c.cycles += 2;
                    c.g[V0][0] = c.g[V0][0] | 0x6976;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x104), c.g[V0][0]);
                    pc = 0x003e9f4c;
                    continue;
                }
                0x003e9f4c => {
                    c.cycles += 5;
                    c.g[A2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                    c.g[A0][0] = c.g[S3][0];
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x51c));
                    let t = c.g[V0][0] as u32;
                    c.g[RA][0] = sx(0x003e9f60);
                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x100));
                    c.pc = t;
                    call_indirect(c, t);
                    returned(c, 0x003e9f60)?;
                    pc = 0x003e9f60;
                    continue;
                }
                0x003e9f60 => {
                    c.cycles += 1;
                    c.g[V0][0] = 0x2b770000;
                    pc = 0x003e9f64;
                    continue;
                }
                0x003e9f64 => {
                    c.cycles += 4;
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x360));
                    c.g[V0][0] = c.g[V0][0] | 0x686f;
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[V0][0] = 0x2b720000;
                    if t {
                        pc = 0x003ea0b0;
                        continue;
                    }
                    pc = 0x003e9f74;
                    continue;
                }
                0x003e9f74 => {
                    c.cycles += 38;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.g[A1][0] = 0x4a0000;
                    c.g[RA][0] = sx(0x003e9f84);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x49b0));
                    callee::TAG_FIELD_FIND.call(c, 0x003e9f84)?;
                    c.g[V1][0] = 0x4a0000;
                    c.g[A2][0] = 0x40;
                    c.g[S1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4888));
                    c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_add(0x174));
                    c.g[A3][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x003e9fa0);
                    c.g[A0][0] = c.g[V0][0];
                    callee::TAG_FIELD_GET_STRING.call(c, 0x003e9fa0)?;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.g[A1][0] = 0x4a0000;
                    c.g[RA][0] = sx(0x003e9fb0);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x49a8));
                    callee::TAG_FIELD_FIND.call(c, 0x003e9fb0)?;
                    c.g[A1][0] = 0u64;
                    c.g[RA][0] = sx(0x003e9fbc);
                    c.g[A0][0] = c.g[V0][0];
                    callee::TAG_FIELD_GET_FLAGS.call(c, 0x003e9fbc)?;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.g[A1][0] = 0x4a0000;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x49b8));
                    c.g[RA][0] = sx(0x003e9fd0);
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x1b4), c.g[V0][0]);
                    callee::TAG_FIELD_FIND.call(c, 0x003e9fd0)?;
                    c.g[A1][0] = 0u64;
                    c.g[RA][0] = sx(0x003e9fdc);
                    c.g[A0][0] = c.g[V0][0];
                    callee::TAG_FIELD_GET_NUMBER.call(c, 0x003e9fdc)?;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.g[A1][0] = 0x4a0000;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x49c0));
                    c.g[RA][0] = sx(0x003e9ff0);
                    c.g[S0][0] = c.g[V0][0];
                    callee::TAG_FIELD_FIND.call(c, 0x003e9ff0)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x003e9ffc);
                    c.g[A1][0] = 0u64;
                    callee::TAG_FIELD_GET_NUMBER.call(c, 0x003e9ffc)?;
                    c.g[A2][0] = c.g[V0][0];
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x1b8));
                    let t = c.g[S0][0] == c.g[V0][0];
                    c.g[A1][0] = c.g[S0][0];
                    if t {
                        pc = 0x003ea0a8;
                        continue;
                    }
                    pc = 0x003ea00c;
                    continue;
                }
                0x003ea00c => {
                    c.cycles += 25;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x1b8), c.g[S0][0]);
                    c.g[RA][0] = sx(0x003ea018);
                    c.g[A0][0] = c.g[S3][0];
                    callee::LOBBY_API_UPDATE_ROOM_COUNT.call(c, 0x003ea018)?;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.g[A1][0] = 0x4a0000;
                    c.g[RA][0] = sx(0x003ea028);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x49c8));
                    callee::TAG_FIELD_FIND.call(c, 0x003ea028)?;
                    c.g[A3][0] = c.g[S1][0];
                    c.g[A2][0] = 0x40;
                    c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1c0));
                    c.g[RA][0] = sx(0x003ea03c);
                    c.g[A0][0] = c.g[V0][0];
                    callee::TAG_FIELD_GET_STRING.call(c, 0x003ea03c)?;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.g[A1][0] = 0x4a0000;
                    c.g[RA][0] = sx(0x003ea04c);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x49d0));
                    callee::TAG_FIELD_FIND.call(c, 0x003ea04c)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x003ea058);
                    c.g[A1][0] = 0u64;
                    callee::TAG_FIELD_GET_FLAGS.call(c, 0x003ea058)?;
                    c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x324));
                    c.g[V1][0] = c.g[V1][0] | 0x8;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x1bc), c.g[V0][0]);
                    let t = c.g[A0][0] == 0u64;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                    if t {
                        pc = 0x003ea0a8;
                        continue;
                    }
                    pc = 0x003ea070;
                    continue;
                }
                0x003ea070 => {
                    c.cycles += 2;
                    let t = true;

                    if t {
                        pc = 0x003ea088;
                        continue;
                    }
                    pc = 0x003ea078;
                    continue;
                }
                0x003ea078 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x003ea080);
                    c.g[A1][0] = c.g[S1][0];
                    callee::LOBBY_LIST_REMOVE.call(c, 0x003ea080)?;
                    c.g[RA][0] = sx(0x003ea088);
                    c.g[A0][0] = c.g[V0][0];
                    callee::LOBBY_MEM_FREE.call(c, 0x003ea088)?;
                    pc = 0x003ea088;
                    continue;
                }
                0x003ea088 => {
                    c.cycles += 5;
                    c.g[RA][0] = sx(0x003ea090);
                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x324));
                    callee::LOBBY_HASH_POP_ANY.call(c, 0x003ea090)?;
                    c.g[S1][0] = c.g[V0][0];

                    if c.g[S1][0] != 0u64 {
                        c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x328));
                        if c.cycles >= c.next_event {
                            c.pc = 0x003ea078;
                            if c.event() {
                                return Ok(());
                            }
                        }
                        pc = 0x003ea078;
                        continue;
                    }
                    pc = 0x003ea09c;
                    continue;
                }
                0x003ea09c => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = c.g[V0][0] | 0x40;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                    pc = 0x003ea0a8;
                    continue;
                }
                0x003ea0a8 => {
                    c.cycles += 2;
                    c.g[V0][0] = 0x2b720000;
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x360));
                    pc = 0x003ea0b0;
                    continue;
                }
                0x003ea0b0 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[V0][0] | 0x6f6d;
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[V0][0] = 0x2b700000;
                    if t {
                        pc = 0x003ea35c;
                        continue;
                    }
                    pc = 0x003ea0bc;
                    continue;
                }
                0x003ea0bc => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x310));
                    let t = c.g[V0][0] == 0u64;
                    c.g[A1][0] = 0x4a0000;
                    if t {
                        pc = 0x003ea358;
                        continue;
                    }
                    pc = 0x003ea0c8;
                    continue;
                }
                0x003ea0c8 => {
                    c.cycles += 9;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.g[RA][0] = sx(0x003ea0d4);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x49d8));
                    callee::TAG_FIELD_FIND.call(c, 0x003ea0d4)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x003ea0e0);
                    c.g[A1][0] = 0xffffffffffffffff;
                    callee::TAG_FIELD_GET_NUMBER.call(c, 0x003ea0e0)?;
                    c.g[S2][0] = c.g[V0][0];
                    let t = (c.g[S2][0] as i64) < 0;
                    c.g[V0][0] = 0x4a0000;
                    if t {
                        pc = 0x003ea358;
                        continue;
                    }
                    pc = 0x003ea0ec;
                    continue;
                }
                0x003ea0ec => {
                    c.cycles += 6;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x49b0));
                    c.g[RA][0] = sx(0x003ea0fc);
                    c.g[A1][0] = c.g[S0][0];
                    callee::TAG_FIELD_FIND.call(c, 0x003ea0fc)?;

                    if c.g[V0][0] == 0u64 {
                        c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x310));
                        pc = 0x003ea30c;
                        continue;
                    }
                    pc = 0x003ea104;
                    continue;
                }
                0x003ea104 => {
                    c.cycles += 69;
                    c.g[RA][0] = sx(0x003ea10c);
                    c.g[A0][0] = 0x68;
                    callee::LOBBY_MEM_ALLOC.call(c, 0x003ea10c)?;
                    c.g[S4][0] = c.g[V0][0];
                    c.g[A2][0] = 0x68;
                    c.g[A0][0] = c.g[S4][0];
                    c.g[RA][0] = sx(0x003ea120);
                    c.g[A1][0] = 0u64;
                    callee::MEMSET.call(c, 0x003ea120)?;
                    c.sw(c.g[S4][0] as u32, c.g[S2][0]);
                    c.g[A1][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x003ea130);
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    callee::TAG_FIELD_FIND.call(c, 0x003ea130)?;
                    c.g[S0][0] = 0x4a0000;
                    c.g[A2][0] = 0x20;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4888));
                    c.g[A1][0] = sx((c.g[S4][0] as u32).wrapping_add(0x1c));
                    c.g[A3][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x003ea14c);
                    c.g[A0][0] = c.g[V0][0];
                    callee::TAG_FIELD_GET_STRING.call(c, 0x003ea14c)?;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.g[A1][0] = 0x4a0000;
                    c.g[RA][0] = sx(0x003ea15c);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x49e0));
                    callee::TAG_FIELD_FIND.call(c, 0x003ea15c)?;
                    c.g[A3][0] = c.g[S0][0];
                    c.g[A2][0] = 0x20;
                    c.g[A1][0] = sx((c.g[S4][0] as u32).wrapping_add(0x3c));
                    c.g[RA][0] = sx(0x003ea170);
                    c.g[A0][0] = c.g[V0][0];
                    callee::TAG_FIELD_GET_STRING.call(c, 0x003ea170)?;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.g[A1][0] = 0x4a0000;
                    c.g[RA][0] = sx(0x003ea180);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x49a8));
                    callee::TAG_FIELD_FIND.call(c, 0x003ea180)?;
                    c.g[A1][0] = 0xffffffffffffffff;
                    c.g[RA][0] = sx(0x003ea18c);
                    c.g[A0][0] = c.g[V0][0];
                    callee::TAG_FIELD_GET_FLAGS.call(c, 0x003ea18c)?;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.g[A1][0] = 0x4a0000;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x49e8));
                    c.g[RA][0] = sx(0x003ea1a0);
                    c.sw((c.g[S4][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                    callee::TAG_FIELD_FIND.call(c, 0x003ea1a0)?;
                    c.g[A1][0] = 0u64;
                    c.g[RA][0] = sx(0x003ea1ac);
                    c.g[A0][0] = c.g[V0][0];
                    callee::TAG_FIELD_GET_ADDRESS.call(c, 0x003ea1ac)?;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.g[A1][0] = 0x4a0000;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x49f0));
                    c.g[RA][0] = sx(0x003ea1c0);
                    c.sw((c.g[S4][0] as u32).wrapping_add(0x64), c.g[V0][0]);
                    callee::TAG_FIELD_FIND.call(c, 0x003ea1c0)?;
                    c.g[A1][0] = 0u64;
                    c.g[RA][0] = sx(0x003ea1cc);
                    c.g[A0][0] = c.g[V0][0];
                    callee::TAG_FIELD_GET_NUMBER.call(c, 0x003ea1cc)?;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.g[A1][0] = 0x4a0000;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x49f8));
                    c.g[RA][0] = sx(0x003ea1e0);
                    c.sw((c.g[S4][0] as u32).wrapping_add(0x10), c.g[V0][0]);
                    callee::TAG_FIELD_FIND.call(c, 0x003ea1e0)?;
                    c.g[A1][0] = 0u64;
                    c.g[RA][0] = sx(0x003ea1ec);
                    c.g[A0][0] = c.g[V0][0];
                    callee::TAG_FIELD_GET_NUMBER.call(c, 0x003ea1ec)?;
                    c.g[A1][0] = 0x4a0000;
                    c.g[A2][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x10));
                    c.g[A0][0] = sx((c.g[S4][0] as u32).wrapping_add(0x14));
                    c.sw((c.g[S4][0] as u32).wrapping_add(0xc), c.g[V0][0]);
                    c.g[RA][0] = sx(0x003ea204);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4870));
                    call_ptr(c, 0x0041_7828, 0x003ea204)?;
                    c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = 0x10000;
                    c.g[V0][0] = c.g[V0][0] & c.g[V1][0];
                    let t = c.g[V0][0] == 0u64;
                    c.g[A0][0] = sx((c.g[S4][0] as u32).wrapping_add(0x18));
                    if t {
                        pc = 0x003ea224;
                        continue;
                    }
                    pc = 0x003ea218;
                    continue;
                }
                0x003ea218 => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x4a0000;
                    let t = true;
                    c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4a00));
                    if t {
                        pc = 0x003ea22c;
                        continue;
                    }
                    pc = 0x003ea224;
                    continue;
                }
                0x003ea224 => {
                    c.cycles += 2;
                    c.g[V0][0] = 0x4a0000;
                    c.g[A1][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4a08));
                    pc = 0x003ea22c;
                    continue;
                }
                0x003ea22c => {
                    c.cycles += 13;
                    c.g[RA][0] = sx(0x003ea234);
                    callee::STRCPY.call(c, 0x003ea234)?;
                    c.sb((c.g[S4][0] as u32).wrapping_add(0x5c), 0u64);
                    c.g[A1][0] = 0x4a0000;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4a10));
                    c.g[RA][0] = sx(0x003ea248);
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    callee::TAG_FIELD_FIND.call(c, 0x003ea248)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x003ea254);
                    c.g[A1][0] = 0xffffffffffffffff;
                    callee::TAG_FIELD_GET_NUMBER.call(c, 0x003ea254)?;
                    c.g[A2][0] = c.g[V0][0];
                    let t = c.g[A2][0] != 0u64;
                    c.g[V0][0] = 0x4a0000;
                    if t {
                        pc = 0x003ea26c;
                        continue;
                    }
                    pc = 0x003ea260;
                    continue;
                }
                0x003ea260 => {
                    c.cycles += 3;
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4a18));
                    let t = true;
                    c.sw((c.g[S4][0] as u32).wrapping_add(0x5c), c.g[V1][0]);
                    if t {
                        pc = 0x003ea280;
                        continue;
                    }
                    pc = 0x003ea26c;
                    continue;
                }
                0x003ea26c => {
                    c.cycles += 2;
                    let t = (c.g[A2][0] as i64) <= 0;
                    c.g[A1][0] = 0x4a0000;
                    if t {
                        pc = 0x003ea280;
                        continue;
                    }
                    pc = 0x003ea274;
                    continue;
                }
                0x003ea274 => {
                    c.cycles += 3;
                    c.g[A0][0] = sx((c.g[S4][0] as u32).wrapping_add(0x5c));
                    c.g[RA][0] = sx(0x003ea280);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4a20));
                    call_ptr(c, 0x0041_7828, 0x003ea280)?;
                    pc = 0x003ea280;
                    continue;
                }
                0x003ea280 => {
                    c.cycles += 6;
                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x310));
                    c.g[RA][0] = sx(0x003ea28c);
                    c.g[A1][0] = c.lw(c.g[S4][0] as u32);
                    callee::LOBBY_HASH_FIND_INT.call(c, 0x003ea28c)?;
                    c.g[S1][0] = c.g[V0][0];
                    let t = c.g[S1][0] == 0u64;
                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x314));
                    if t {
                        pc = 0x003ea2ec;
                        continue;
                    }
                    pc = 0x003ea298;
                    continue;
                }
                0x003ea298 => {
                    c.cycles += 5;
                    c.g[RA][0] = sx(0x003ea2a0);
                    c.g[A1][0] = c.g[S1][0];
                    callee::LOBBY_LIST_GET_DATA.call(c, 0x003ea2a0)?;
                    c.g[S0][0] = c.g[V0][0];
                    let t = c.g[S0][0] == 0u64;

                    if t {
                        pc = 0x003ea2cc;
                        continue;
                    }
                    pc = 0x003ea2ac;
                    continue;
                }
                0x003ea2ac => {
                    c.cycles += 4;
                    c.g[V1][0] = c.lb((c.g[S0][0] as u32).wrapping_add(0x5c));
                    c.g[V0][0] = ((c.g[V1][0] as i64) < 48i64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = ((c.g[V1][0] as i64) < 58i64) as u64;
                    if t {
                        pc = 0x003ea2cc;
                        continue;
                    }
                    pc = 0x003ea2bc;
                    continue;
                }
                0x003ea2bc => {
                    c.cycles += 2;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A0][0] = sx((c.g[S4][0] as u32).wrapping_add(0x5c));
                    if t {
                        pc = 0x003ea2cc;
                        continue;
                    }
                    pc = 0x003ea2c4;
                    continue;
                }
                0x003ea2c4 => {
                    c.cycles += 2;
                    c.g[RA][0] = sx(0x003ea2cc);
                    c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x5c));
                    callee::STRCPY.call(c, 0x003ea2cc)?;
                    pc = 0x003ea2cc;
                    continue;
                }
                0x003ea2cc => {
                    c.cycles += 8;
                    c.g[RA][0] = sx(0x003ea2d4);
                    c.g[A0][0] = c.g[S0][0];
                    callee::LOBBY_MEM_FREE.call(c, 0x003ea2d4)?;
                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x314));
                    c.g[A1][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x003ea2e4);
                    c.g[A2][0] = c.g[S4][0];
                    callee::LOBBY_LIST_UPDATE.call(c, 0x003ea2e4)?;
                    let t = true;
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
                    if t {
                        pc = 0x003ea350;
                        continue;
                    }
                    pc = 0x003ea2ec;
                    continue;
                }
                0x003ea2ec => {
                    c.cycles += 8;
                    c.g[RA][0] = sx(0x003ea2f4);
                    c.g[A1][0] = c.g[S4][0];
                    callee::LOBBY_LIST_ADD.call(c, 0x003ea2f4)?;
                    c.g[A1][0] = c.lw(c.g[S4][0] as u32);
                    c.g[A2][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x003ea304);
                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x310));
                    callee::LOBBY_HASH_ADD_INT.call(c, 0x003ea304)?;
                    let t = true;
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
                    if t {
                        pc = 0x003ea350;
                        continue;
                    }
                    pc = 0x003ea30c;
                    continue;
                }
                0x003ea30c => {
                    c.cycles += 5;
                    c.g[RA][0] = sx(0x003ea314);
                    c.g[A1][0] = c.g[S2][0];
                    callee::LOBBY_HASH_FIND_INT.call(c, 0x003ea314)?;
                    c.g[S1][0] = c.g[V0][0];

                    if c.g[S1][0] == 0u64 {
                        c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
                        pc = 0x003ea350;
                        continue;
                    }
                    pc = 0x003ea320;
                    continue;
                }
                0x003ea320 => {
                    c.cycles += 12;
                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x310));
                    c.g[RA][0] = sx(0x003ea32c);
                    c.g[A1][0] = c.g[S2][0];
                    callee::LOBBY_HASH_DEL_INT.call(c, 0x003ea32c)?;
                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x314));
                    c.g[RA][0] = sx(0x003ea338);
                    c.g[A1][0] = c.g[S1][0];
                    callee::LOBBY_LIST_REMOVE.call(c, 0x003ea338)?;
                    c.g[RA][0] = sx(0x003ea340);
                    c.g[A0][0] = c.g[V0][0];
                    callee::LOBBY_MEM_FREE.call(c, 0x003ea340)?;
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = c.g[V0][0] | 0x20;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
                    pc = 0x003ea350;
                    continue;
                }
                0x003ea350 => {
                    c.cycles += 2;
                    c.g[V0][0] = c.g[V0][0] | 0x20;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                    pc = 0x003ea358;
                    continue;
                }
                0x003ea358 => {
                    c.cycles += 1;
                    c.g[V0][0] = 0x2b700000;
                    pc = 0x003ea35c;
                    continue;
                }
                0x003ea35c => {
                    c.cycles += 4;
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x360));
                    c.g[V0][0] = c.g[V0][0] | 0x6f70;
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[V0][0] = 0x2b750000;
                    if t {
                        pc = 0x003ea468;
                        continue;
                    }
                    pc = 0x003ea36c;
                    continue;
                }
                0x003ea36c => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x310));
                    let t = c.g[V0][0] == 0u64;
                    c.g[A1][0] = 0x4a0000;
                    if t {
                        pc = 0x003ea460;
                        continue;
                    }
                    pc = 0x003ea378;
                    continue;
                }
                0x003ea378 => {
                    c.cycles += 13;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.g[RA][0] = sx(0x003ea384);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4a28));
                    callee::TAG_FIELD_FIND.call(c, 0x003ea384)?;
                    c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x160));
                    c.g[A3][0] = 0x4a0000;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x4888));
                    c.g[A1][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x003ea3a0);
                    c.g[A2][0] = 0x200;
                    callee::TAG_FIELD_GET_STRING.call(c, 0x003ea3a0)?;
                    c.g[V0][0] = c.lb((c.g[SP][0] as u32).wrapping_add(0x160));
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = 0x2b750000;
                    if t {
                        pc = 0x003ea464;
                        continue;
                    }
                    pc = 0x003ea3ac;
                    continue;
                }
                0x003ea3ac => {
                    c.cycles += 1;
                    c.g[A2][0] = c.lbu(c.g[S0][0] as u32);
                    pc = 0x003ea3b0;
                    continue;
                }
                0x003ea3b0 => {
                    c.cycles += 4;
                    c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x30));
                    c.g[V0][0] = (c.g[V0][0] < 0xau64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[S2][0] = 0u64;
                    if t {
                        pc = 0x003ea3e8;
                        continue;
                    }
                    pc = 0x003ea3c0;
                    continue;
                }
                0x003ea3c0 => {
                    c.cycles += 2;
                    c.g[A1][0] = 0xa;
                    pc = 0x003ea3c8;
                    continue;
                }
                0x003ea3c8 => {
                    c.cycles += 8;
                    c.g[A0][0] = c.g[A2][0] & 0xf;
                    c.mult(0, 3, 18, 5);
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    c.g[A2][0] = c.lbu(c.g[S0][0] as u32);
                    c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x30));
                    c.g[V0][0] = (c.g[V0][0] < 0xau64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[S2][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    if t {
                        if c.cycles >= c.next_event {
                            c.pc = 0x003ea3c8;
                            if c.event() {
                                return Ok(());
                            }
                        }
                        pc = 0x003ea3c8;
                        continue;
                    }
                    pc = 0x003ea3e8;
                    continue;
                }
                0x003ea3e8 => {
                    c.cycles += 2;
                    let t = c.g[S2][0] == 0u64;
                    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    if t {
                        pc = 0x003ea460;
                        continue;
                    }
                    pc = 0x003ea3f0;
                    continue;
                }
                0x003ea3f0 => {
                    c.cycles += 8;
                    c.g[V1][0] = c.lb(c.g[S0][0] as u32);
                    c.g[A2][0] = 0u64;
                    if c.g[V1][0] != 0 {
                        c.g[S0][0] = c.g[V0][0];
                    }
                    c.g[A1][0] = c.lbu(c.g[S0][0] as u32);
                    c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x30));
                    c.g[V0][0] = (c.g[V0][0] < 0xau64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[T0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x1b8));
                    if t {
                        pc = 0x003ea438;
                        continue;
                    }
                    pc = 0x003ea410;
                    continue;
                }
                0x003ea410 => {
                    c.cycles += 2;
                    c.g[A3][0] = 0xa;
                    pc = 0x003ea418;
                    continue;
                }
                0x003ea418 => {
                    c.cycles += 8;
                    c.g[A0][0] = c.g[A1][0] & 0xf;
                    c.mult(0, 3, 6, 7);
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    c.g[A1][0] = c.lbu(c.g[S0][0] as u32);
                    c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_sub(0x30));
                    c.g[V0][0] = (c.g[V0][0] < 0xau64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[A2][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    if t {
                        if c.cycles >= c.next_event {
                            c.pc = 0x003ea418;
                            if c.event() {
                                return Ok(());
                            }
                        }
                        pc = 0x003ea418;
                        continue;
                    }
                    pc = 0x003ea438;
                    continue;
                }
                0x003ea438 => {
                    c.cycles += 4;
                    c.g[V1][0] = c.lb(c.g[S0][0] as u32);
                    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    let t = c.g[S2][0] == c.g[T0][0];
                    if c.g[V1][0] != 0 {
                        c.g[S0][0] = c.g[V0][0];
                    }
                    if t {
                        pc = 0x003ea454;
                        continue;
                    }
                    pc = 0x003ea448;
                    continue;
                }
                0x003ea448 => {
                    c.cycles += 3;
                    c.g[A1][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x003ea454);
                    c.g[A0][0] = c.g[S3][0];
                    callee::LOBBY_API_UPDATE_ROOM_COUNT.call(c, 0x003ea454)?;
                    pc = 0x003ea454;
                    continue;
                }
                0x003ea454 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lb(c.g[S0][0] as u32);

                    if c.g[V0][0] != 0u64 {
                        c.g[A2][0] = c.lbu(c.g[S0][0] as u32);
                        if c.cycles >= c.next_event {
                            c.pc = 0x003ea3b0;
                            if c.event() {
                                return Ok(());
                            }
                        }
                        pc = 0x003ea3b0;
                        continue;
                    }
                    pc = 0x003ea460;
                    continue;
                }
                0x003ea460 => {
                    c.cycles += 1;
                    c.g[V0][0] = 0x2b750000;
                    pc = 0x003ea464;
                    continue;
                }
                0x003ea464 => {
                    c.cycles += 1;
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x360));
                    pc = 0x003ea468;
                    continue;
                }
                0x003ea468 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[V0][0] | 0x7372;
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[V0][0] = 0x2b720000;
                    if t {
                        pc = 0x003ea790;
                        continue;
                    }
                    pc = 0x003ea474;
                    continue;
                }
                0x003ea474 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x324));
                    let t = c.g[V0][0] == 0u64;
                    c.g[A1][0] = 0x4a0000;
                    if t {
                        pc = 0x003ea78c;
                        continue;
                    }
                    pc = 0x003ea480;
                    continue;
                }
                0x003ea480 => {
                    c.cycles += 9;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.g[RA][0] = sx(0x003ea48c);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x49d8));
                    callee::TAG_FIELD_FIND.call(c, 0x003ea48c)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x003ea498);
                    c.g[A1][0] = 0xffffffffffffffff;
                    callee::TAG_FIELD_GET_NUMBER.call(c, 0x003ea498)?;
                    c.g[S2][0] = c.g[V0][0];
                    let t = (c.g[S2][0] as i64) < 0;
                    c.g[V0][0] = 0x4a0000;
                    if t {
                        pc = 0x003ea78c;
                        continue;
                    }
                    pc = 0x003ea4a4;
                    continue;
                }
                0x003ea4a4 => {
                    c.cycles += 6;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x49b0));
                    c.g[RA][0] = sx(0x003ea4b4);
                    c.g[A1][0] = c.g[S0][0];
                    callee::TAG_FIELD_FIND.call(c, 0x003ea4b4)?;

                    if c.g[V0][0] == 0u64 {
                        c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x324));
                        pc = 0x003ea720;
                        continue;
                    }
                    pc = 0x003ea4bc;
                    continue;
                }
                0x003ea4bc => {
                    c.cycles += 75;
                    c.g[RA][0] = sx(0x003ea4c4);
                    c.g[A0][0] = 0x138;
                    callee::LOBBY_MEM_ALLOC.call(c, 0x003ea4c4)?;
                    c.g[S4][0] = c.g[V0][0];
                    c.g[A2][0] = 0x138;
                    c.g[A0][0] = c.g[S4][0];
                    c.g[RA][0] = sx(0x003ea4d8);
                    c.g[A1][0] = 0u64;
                    callee::MEMSET.call(c, 0x003ea4d8)?;
                    c.sw(c.g[S4][0] as u32, c.g[S2][0]);
                    c.g[A1][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x003ea4e8);
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    callee::TAG_FIELD_FIND.call(c, 0x003ea4e8)?;
                    c.g[S0][0] = 0x4a0000;
                    c.g[A2][0] = 0x20;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4888));
                    c.g[A1][0] = sx((c.g[S4][0] as u32).wrapping_add(0x8));
                    c.g[A3][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x003ea504);
                    c.g[A0][0] = c.g[V0][0];
                    callee::TAG_FIELD_GET_STRING.call(c, 0x003ea504)?;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.g[A1][0] = 0x4a0000;
                    c.g[RA][0] = sx(0x003ea514);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x49e8));
                    callee::TAG_FIELD_FIND.call(c, 0x003ea514)?;
                    c.g[A1][0] = 0u64;
                    c.g[RA][0] = sx(0x003ea520);
                    c.g[A0][0] = c.g[V0][0];
                    callee::TAG_FIELD_GET_ADDRESS.call(c, 0x003ea520)?;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.g[A1][0] = 0x4a0000;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4a10));
                    c.g[RA][0] = sx(0x003ea534);
                    c.sw((c.g[S4][0] as u32).wrapping_add(0x30), c.g[V0][0]);
                    callee::TAG_FIELD_FIND.call(c, 0x003ea534)?;
                    c.g[A2][0] = 0x8;
                    c.g[A3][0] = c.g[S0][0];
                    c.g[A1][0] = sx((c.g[S4][0] as u32).wrapping_add(0x28));
                    c.g[RA][0] = sx(0x003ea548);
                    c.g[A0][0] = c.g[V0][0];
                    callee::TAG_FIELD_GET_STRING.call(c, 0x003ea548)?;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.g[A1][0] = 0x4a0000;
                    c.g[RA][0] = sx(0x003ea558);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x49a8));
                    callee::TAG_FIELD_FIND.call(c, 0x003ea558)?;
                    c.g[A1][0] = 0u64;
                    c.g[RA][0] = sx(0x003ea564);
                    c.g[A0][0] = c.g[V0][0];
                    callee::TAG_FIELD_GET_FLAGS.call(c, 0x003ea564)?;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.g[A1][0] = 0x4a0000;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4a30));
                    c.g[RA][0] = sx(0x003ea578);
                    c.sw((c.g[S4][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                    callee::TAG_FIELD_FIND.call(c, 0x003ea578)?;
                    c.g[A2][0] = 0x80;
                    c.g[A3][0] = c.g[S0][0];
                    c.g[A1][0] = sx((c.g[S4][0] as u32).wrapping_add(0x38));
                    c.g[RA][0] = sx(0x003ea58c);
                    c.g[A0][0] = c.g[V0][0];
                    callee::TAG_FIELD_GET_STRING.call(c, 0x003ea58c)?;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.g[A1][0] = 0x4a0000;
                    c.g[RA][0] = sx(0x003ea59c);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x49c8));
                    callee::TAG_FIELD_FIND.call(c, 0x003ea59c)?;
                    c.g[A1][0] = 0u64;
                    c.g[RA][0] = sx(0x003ea5a8);
                    c.g[A0][0] = c.g[V0][0];
                    callee::TAG_FIELD_GET_NUMBER.call(c, 0x003ea5a8)?;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.g[A1][0] = 0x4a0000;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4a38));
                    c.g[RA][0] = sx(0x003ea5bc);
                    c.sw((c.g[S4][0] as u32).wrapping_add(0x34), c.g[V0][0]);
                    callee::TAG_FIELD_FIND.call(c, 0x003ea5bc)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A3][0] = c.g[S0][0];
                    c.g[A1][0] = sx((c.g[S4][0] as u32).wrapping_add(0xb8));
                    c.g[RA][0] = sx(0x003ea5d0);
                    c.g[A2][0] = 0x80;
                    callee::TAG_FIELD_GET_STRING.call(c, 0x003ea5d0)?;
                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x324));
                    c.g[RA][0] = sx(0x003ea5dc);
                    c.g[A1][0] = c.lw(c.g[S4][0] as u32);
                    callee::LOBBY_HASH_FIND_INT.call(c, 0x003ea5dc)?;
                    c.g[S1][0] = c.g[V0][0];
                    let t = c.g[S1][0] == 0u64;
                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x328));
                    if t {
                        pc = 0x003ea6e8;
                        continue;
                    }
                    pc = 0x003ea5e8;
                    continue;
                }
                0x003ea5e8 => {
                    c.cycles += 8;
                    c.g[RA][0] = sx(0x003ea5f0);
                    c.g[A1][0] = c.g[S1][0];
                    callee::LOBBY_LIST_GET_DATA.call(c, 0x003ea5f0)?;
                    c.g[S0][0] = c.g[V0][0];
                    c.g[V1][0] = c.g[S4][0];
                    c.g[V0][0] = c.g[S4][0] | c.g[S0][0];
                    c.g[V0][0] = c.g[V0][0] & 0x7;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A0][0] = c.g[S0][0];
                    if t {
                        pc = 0x003ea664;
                        continue;
                    }
                    pc = 0x003ea608;
                    continue;
                }
                0x003ea608 => {
                    c.cycles += 2;
                    c.g[V0][0] = sx((c.g[S4][0] as u32).wrapping_add(0x120));
                    c.g[S5][0] = 0x4a0000;
                    pc = 0x003ea610;
                    continue;
                }
                0x003ea610 => {
                    c.cycles += 19;
                    c.ldl(5, (c.g[V1][0] as u32).wrapping_add(0x7));
                    c.ldr(5, c.g[V1][0] as u32);
                    c.ldl(6, (c.g[V1][0] as u32).wrapping_add(0xf));
                    c.ldr(6, (c.g[V1][0] as u32).wrapping_add(0x8));
                    c.ldl(7, (c.g[V1][0] as u32).wrapping_add(0x17));
                    c.ldr(7, (c.g[V1][0] as u32).wrapping_add(0x10));
                    c.ldl(8, (c.g[V1][0] as u32).wrapping_add(0x1f));
                    c.ldr(8, (c.g[V1][0] as u32).wrapping_add(0x18));
                    c.sdl(5, (c.g[A0][0] as u32).wrapping_add(0x7));
                    c.sdr(5, c.g[A0][0] as u32);
                    c.sdl(6, (c.g[A0][0] as u32).wrapping_add(0xf));
                    c.sdr(6, (c.g[A0][0] as u32).wrapping_add(0x8));
                    c.sdl(7, (c.g[A0][0] as u32).wrapping_add(0x17));
                    c.sdr(7, (c.g[A0][0] as u32).wrapping_add(0x10));
                    c.sdl(8, (c.g[A0][0] as u32).wrapping_add(0x1f));
                    c.sdr(8, (c.g[A0][0] as u32).wrapping_add(0x18));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
                    if t {
                        if c.cycles >= c.next_event {
                            c.pc = 0x003ea610;
                            if c.event() {
                                return Ok(());
                            }
                        }
                        pc = 0x003ea610;
                        continue;
                    }
                    pc = 0x003ea65c;
                    continue;
                }
                0x003ea65c => {
                    c.cycles += 2;
                    let t = true;

                    if t {
                        pc = 0x003ea698;
                        continue;
                    }
                    pc = 0x003ea664;
                    continue;
                }
                0x003ea664 => {
                    c.cycles += 2;
                    c.g[V0][0] = sx((c.g[S4][0] as u32).wrapping_add(0x120));
                    c.g[S5][0] = 0x4a0000;
                    pc = 0x003ea66c;
                    continue;
                }
                0x003ea66c => {
                    c.cycles += 11;
                    c.g[T1][0] = c.ld(c.g[V1][0] as u32);
                    c.g[A1][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x8));
                    c.g[A2][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x10));
                    c.g[A3][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x18));
                    c.sd(c.g[A0][0] as u32, c.g[T1][0]);
                    c.sd((c.g[A0][0] as u32).wrapping_add(0x8), c.g[A1][0]);
                    c.sd((c.g[A0][0] as u32).wrapping_add(0x10), c.g[A2][0]);
                    c.sd((c.g[A0][0] as u32).wrapping_add(0x18), c.g[A3][0]);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
                    if t {
                        if c.cycles >= c.next_event {
                            c.pc = 0x003ea66c;
                            if c.event() {
                                return Ok(());
                            }
                        }
                        pc = 0x003ea66c;
                        continue;
                    }
                    pc = 0x003ea698;
                    continue;
                }
                0x003ea698 => {
                    c.cycles += 20;
                    c.ldl(8, (c.g[V1][0] as u32).wrapping_add(0x7));
                    c.ldr(8, c.g[V1][0] as u32);
                    c.ldl(9, (c.g[V1][0] as u32).wrapping_add(0xf));
                    c.ldr(9, (c.g[V1][0] as u32).wrapping_add(0x8));
                    c.ldl(2, (c.g[V1][0] as u32).wrapping_add(0x17));
                    c.ldr(2, (c.g[V1][0] as u32).wrapping_add(0x10));
                    c.sdl(8, (c.g[A0][0] as u32).wrapping_add(0x7));
                    c.sdr(8, c.g[A0][0] as u32);
                    c.sdl(9, (c.g[A0][0] as u32).wrapping_add(0xf));
                    c.sdr(9, (c.g[A0][0] as u32).wrapping_add(0x8));
                    c.sdl(2, (c.g[A0][0] as u32).wrapping_add(0x17));
                    c.g[A2][0] = c.g[S0][0];
                    c.sdr(2, (c.g[A0][0] as u32).wrapping_add(0x10));
                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x328));
                    c.g[RA][0] = sx(0x003ea6d8);
                    c.g[A1][0] = c.g[S1][0];
                    callee::LOBBY_LIST_UPDATE.call(c, 0x003ea6d8)?;
                    c.g[RA][0] = sx(0x003ea6e0);
                    c.g[A0][0] = c.g[S4][0];
                    callee::LOBBY_MEM_FREE.call(c, 0x003ea6e0)?;
                    let t = true;
                    c.g[S4][0] = c.g[S0][0];
                    if t {
                        pc = 0x003ea704;
                        continue;
                    }
                    pc = 0x003ea6e8;
                    continue;
                }
                0x003ea6e8 => {
                    c.cycles += 7;
                    c.g[A1][0] = c.g[S4][0];
                    c.g[RA][0] = sx(0x003ea6f4);
                    c.g[S5][0] = 0x4a0000;
                    callee::LOBBY_LIST_ADD.call(c, 0x003ea6f4)?;
                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x324));
                    c.g[A2][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x003ea704);
                    c.g[A1][0] = c.lw(c.g[S4][0] as u32);
                    callee::LOBBY_HASH_ADD_INT.call(c, 0x003ea704)?;
                    pc = 0x003ea704;
                    continue;
                }
                0x003ea704 => {
                    c.cycles += 5;
                    c.g[A0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x4));
                    c.g[V0][0] = 0x200000;
                    c.g[V0][0] = c.g[A0][0] & c.g[V0][0];

                    if c.g[V0][0] != 0u64 {
                        c.sw((c.g[S3][0] as u32).wrapping_add(0x1b4), c.g[A0][0]);
                        pc = 0x003ea758;
                        continue;
                    }
                    pc = 0x003ea718;
                    continue;
                }
                0x003ea718 => {
                    c.cycles += 2;
                    let t = true;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    if t {
                        pc = 0x003ea75c;
                        continue;
                    }
                    pc = 0x003ea720;
                    continue;
                }
                0x003ea720 => {
                    c.cycles += 6;
                    c.g[A1][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x003ea72c);
                    c.g[S5][0] = 0x4a0000;
                    callee::LOBBY_HASH_FIND_INT.call(c, 0x003ea72c)?;
                    c.g[S1][0] = c.g[V0][0];

                    if c.g[S1][0] == 0u64 {
                        c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                        pc = 0x003ea75c;
                        continue;
                    }
                    pc = 0x003ea738;
                    continue;
                }
                0x003ea738 => {
                    c.cycles += 8;
                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x324));
                    c.g[RA][0] = sx(0x003ea744);
                    c.g[A1][0] = c.g[S2][0];
                    callee::LOBBY_HASH_DEL_INT.call(c, 0x003ea744)?;
                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x328));
                    c.g[RA][0] = sx(0x003ea750);
                    c.g[A1][0] = c.g[S1][0];
                    callee::LOBBY_LIST_REMOVE.call(c, 0x003ea750)?;
                    c.g[RA][0] = sx(0x003ea758);
                    c.g[A0][0] = c.g[V0][0];
                    callee::LOBBY_MEM_FREE.call(c, 0x003ea758)?;
                    pc = 0x003ea758;
                    continue;
                }
                0x003ea758 => {
                    c.cycles += 1;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    pc = 0x003ea75c;
                    continue;
                }
                0x003ea75c => {
                    c.cycles += 12;
                    c.g[RA][0] = sx(0x003ea764);
                    c.g[A1][0] = sx((c.g[S5][0] as u32).wrapping_add(0x49f0));
                    callee::TAG_FIELD_FIND.call(c, 0x003ea764)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x003ea770);
                    c.g[A1][0] = 0u64;
                    callee::TAG_FIELD_GET_NUMBER.call(c, 0x003ea770)?;
                    c.g[A1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x1b8));
                    c.g[A2][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x003ea780);
                    c.g[A0][0] = c.g[S3][0];
                    callee::LOBBY_API_UPDATE_ROOM_COUNT.call(c, 0x003ea780)?;
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = c.g[V0][0] | 0x40;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                    pc = 0x003ea78c;
                    continue;
                }
                0x003ea78c => {
                    c.cycles += 1;
                    c.g[V0][0] = 0x2b720000;
                    pc = 0x003ea790;
                    continue;
                }
                0x003ea790 => {
                    c.cycles += 4;
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x360));
                    c.g[V0][0] = c.g[V0][0] | 0x6e6b;
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[V0][0] = 0x2b730000;
                    if t {
                        pc = 0x003eaa04;
                        continue;
                    }
                    pc = 0x003ea7a0;
                    continue;
                }
                0x003ea7a0 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x338));
                    let t = c.g[V0][0] == 0u64;
                    c.g[A1][0] = 0x4a0000;
                    if t {
                        pc = 0x003ea9fc;
                        continue;
                    }
                    pc = 0x003ea7ac;
                    continue;
                }
                0x003ea7ac => {
                    c.cycles += 9;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.g[RA][0] = sx(0x003ea7b8);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4a40));
                    callee::TAG_FIELD_FIND.call(c, 0x003ea7b8)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x003ea7c4);
                    c.g[A1][0] = 0u64;
                    callee::TAG_FIELD_GET_NUMBER.call(c, 0x003ea7c4)?;
                    c.g[V1][0] = 0xfffffffffffffffe;
                    let t = c.g[V0][0] != c.g[V1][0];

                    if t {
                        pc = 0x003ea804;
                        continue;
                    }
                    pc = 0x003ea7d0;
                    continue;
                }
                0x003ea7d0 => {
                    c.cycles += 2;
                    let t = true;

                    if t {
                        pc = 0x003ea7e8;
                        continue;
                    }
                    pc = 0x003ea7d8;
                    continue;
                }
                0x003ea7d8 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x003ea7e0);
                    c.g[A1][0] = c.g[S1][0];
                    callee::LOBBY_LIST_REMOVE.call(c, 0x003ea7e0)?;
                    c.g[RA][0] = sx(0x003ea7e8);
                    c.g[A0][0] = c.g[V0][0];
                    callee::LOBBY_MEM_FREE.call(c, 0x003ea7e8)?;
                    pc = 0x003ea7e8;
                    continue;
                }
                0x003ea7e8 => {
                    c.cycles += 5;
                    c.g[RA][0] = sx(0x003ea7f0);
                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x338));
                    callee::LOBBY_HASH_POP_ANY.call(c, 0x003ea7f0)?;
                    c.g[S1][0] = c.g[V0][0];

                    if c.g[S1][0] != 0u64 {
                        c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x33c));
                        if c.cycles >= c.next_event {
                            c.pc = 0x003ea7d8;
                            if c.event() {
                                return Ok(());
                            }
                        }
                        pc = 0x003ea7d8;
                        continue;
                    }
                    pc = 0x003ea7fc;
                    continue;
                }
                0x003ea7fc => {
                    c.cycles += 2;
                    let t = true;
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
                    if t {
                        pc = 0x003ea9f4;
                        continue;
                    }
                    pc = 0x003ea804;
                    continue;
                }
                0x003ea804 => {
                    c.cycles += 37;
                    c.g[RA][0] = sx(0x003ea80c);
                    c.g[A0][0] = 0xa8;
                    callee::LOBBY_MEM_ALLOC.call(c, 0x003ea80c)?;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.g[S2][0] = c.g[V0][0];
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x4), 0u64);
                    c.g[A1][0] = 0x4a0000;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x49e8));
                    c.g[RA][0] = sx(0x003ea828);
                    c.g[S4][0] = sx((c.g[S2][0] as u32).wrapping_add(0x8));
                    callee::TAG_FIELD_FIND.call(c, 0x003ea828)?;
                    c.g[A1][0] = 0u64;
                    c.g[RA][0] = sx(0x003ea834);
                    c.g[A0][0] = c.g[V0][0];
                    callee::TAG_FIELD_GET_NUMBER.call(c, 0x003ea834)?;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.g[A1][0] = 0x4a0000;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x49b0));
                    c.g[RA][0] = sx(0x003ea848);
                    c.sw(c.g[S2][0] as u32, c.g[V0][0]);
                    callee::TAG_FIELD_FIND.call(c, 0x003ea848)?;
                    c.g[S0][0] = 0x4a0000;
                    c.g[A2][0] = 0x20;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4888));
                    c.g[A1][0] = c.g[S4][0];
                    c.g[A3][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x003ea864);
                    c.g[A0][0] = c.g[V0][0];
                    callee::TAG_FIELD_GET_STRING.call(c, 0x003ea864)?;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.g[A1][0] = 0x4a0000;
                    c.g[RA][0] = sx(0x003ea874);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4a30));
                    callee::TAG_FIELD_FIND.call(c, 0x003ea874)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A3][0] = c.g[S0][0];
                    c.g[A1][0] = sx((c.g[S2][0] as u32).wrapping_add(0x28));
                    c.g[RA][0] = sx(0x003ea888);
                    c.g[A2][0] = 0x80;
                    callee::TAG_FIELD_GET_STRING.call(c, 0x003ea888)?;
                    c.g[V1][0] = c.lw(c.g[S2][0] as u32);
                    c.g[V1][0] = ((c.g[V1][0] as i64) < 8192i64) as u64;

                    if c.g[V1][0] != 0u64 {
                        c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x338));
                        pc = 0x003ea8a4;
                        continue;
                    }
                    pc = 0x003ea898;
                    continue;
                }
                0x003ea898 => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x1fff;
                    c.sw(c.g[S2][0] as u32, c.g[V0][0]);
                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x338));
                    pc = 0x003ea8a4;
                    continue;
                }
                0x003ea8a4 => {
                    c.cycles += 5;
                    c.g[RA][0] = sx(0x003ea8ac);
                    c.g[A1][0] = c.g[S4][0];
                    callee::LOBBY_HASH_FIND_STR.call(c, 0x003ea8ac)?;
                    c.g[S1][0] = c.g[V0][0];
                    let t = c.g[S1][0] != 0u64;
                    c.g[V0][0] = c.lw(c.g[S2][0] as u32);
                    if t {
                        pc = 0x003ea8e4;
                        continue;
                    }
                    pc = 0x003ea8b8;
                    continue;
                }
                0x003ea8b8 => {
                    c.cycles += 2;
                    let t = (c.g[V0][0] as i64) <= 0;

                    if t {
                        pc = 0x003ea9e8;
                        continue;
                    }
                    pc = 0x003ea8c0;
                    continue;
                }
                0x003ea8c0 => {
                    c.cycles += 9;
                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x33c));
                    c.g[RA][0] = sx(0x003ea8cc);
                    c.g[A1][0] = c.g[S2][0];
                    callee::LOBBY_LIST_ADD.call(c, 0x003ea8cc)?;
                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x338));
                    c.g[A1][0] = c.g[S4][0];
                    c.g[RA][0] = sx(0x003ea8dc);
                    c.g[A2][0] = c.g[V0][0];
                    callee::LOBBY_HASH_ADD_STR.call(c, 0x003ea8dc)?;
                    let t = true;
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
                    if t {
                        pc = 0x003ea9f4;
                        continue;
                    }
                    pc = 0x003ea8e4;
                    continue;
                }
                0x003ea8e4 => {
                    c.cycles += 2;

                    if (c.g[V0][0] as i64) <= 0 {
                        c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x338));
                        pc = 0x003ea9bc;
                        continue;
                    }
                    pc = 0x003ea8ec;
                    continue;
                }
                0x003ea8ec => {
                    c.cycles += 9;
                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x33c));
                    c.g[RA][0] = sx(0x003ea8f8);
                    c.g[A1][0] = c.g[S1][0];
                    callee::LOBBY_LIST_GET_DATA.call(c, 0x003ea8f8)?;
                    c.g[A2][0] = c.g[V0][0];
                    c.g[V1][0] = c.g[S2][0];
                    c.g[V0][0] = c.g[S2][0] | c.g[A2][0];
                    c.g[V0][0] = c.g[V0][0] & 0x7;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A0][0] = c.g[A2][0];
                    if t {
                        pc = 0x003ea968;
                        continue;
                    }
                    pc = 0x003ea910;
                    continue;
                }
                0x003ea910 => {
                    c.cycles += 1;
                    c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(0xa0));
                    pc = 0x003ea914;
                    continue;
                }
                0x003ea914 => {
                    c.cycles += 19;
                    c.ldl(7, (c.g[V1][0] as u32).wrapping_add(0x7));
                    c.ldr(7, c.g[V1][0] as u32);
                    c.ldl(8, (c.g[V1][0] as u32).wrapping_add(0xf));
                    c.ldr(8, (c.g[V1][0] as u32).wrapping_add(0x8));
                    c.ldl(9, (c.g[V1][0] as u32).wrapping_add(0x17));
                    c.ldr(9, (c.g[V1][0] as u32).wrapping_add(0x10));
                    c.ldl(5, (c.g[V1][0] as u32).wrapping_add(0x1f));
                    c.ldr(5, (c.g[V1][0] as u32).wrapping_add(0x18));
                    c.sdl(7, (c.g[A0][0] as u32).wrapping_add(0x7));
                    c.sdr(7, c.g[A0][0] as u32);
                    c.sdl(8, (c.g[A0][0] as u32).wrapping_add(0xf));
                    c.sdr(8, (c.g[A0][0] as u32).wrapping_add(0x8));
                    c.sdl(9, (c.g[A0][0] as u32).wrapping_add(0x17));
                    c.sdr(9, (c.g[A0][0] as u32).wrapping_add(0x10));
                    c.sdl(5, (c.g[A0][0] as u32).wrapping_add(0x1f));
                    c.sdr(5, (c.g[A0][0] as u32).wrapping_add(0x18));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
                    if t {
                        if c.cycles >= c.next_event {
                            c.pc = 0x003ea914;
                            if c.event() {
                                return Ok(());
                            }
                        }
                        pc = 0x003ea914;
                        continue;
                    }
                    pc = 0x003ea960;
                    continue;
                }
                0x003ea960 => {
                    c.cycles += 2;
                    let t = true;

                    if t {
                        pc = 0x003ea998;
                        continue;
                    }
                    pc = 0x003ea968;
                    continue;
                }
                0x003ea968 => {
                    c.cycles += 1;
                    c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(0xa0));
                    pc = 0x003ea96c;
                    continue;
                }
                0x003ea96c => {
                    c.cycles += 11;
                    c.g[A3][0] = c.ld(c.g[V1][0] as u32);
                    c.g[T0][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x8));
                    c.g[T1][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x10));
                    c.g[A1][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x18));
                    c.sd(c.g[A0][0] as u32, c.g[A3][0]);
                    c.sd((c.g[A0][0] as u32).wrapping_add(0x8), c.g[T0][0]);
                    c.sd((c.g[A0][0] as u32).wrapping_add(0x10), c.g[T1][0]);
                    c.sd((c.g[A0][0] as u32).wrapping_add(0x18), c.g[A1][0]);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
                    if t {
                        if c.cycles >= c.next_event {
                            c.pc = 0x003ea96c;
                            if c.event() {
                                return Ok(());
                            }
                        }
                        pc = 0x003ea96c;
                        continue;
                    }
                    pc = 0x003ea998;
                    continue;
                }
                0x003ea998 => {
                    c.cycles += 9;
                    c.ldl(7, (c.g[V1][0] as u32).wrapping_add(0x7));
                    c.ldr(7, c.g[V1][0] as u32);
                    c.sdl(7, (c.g[A0][0] as u32).wrapping_add(0x7));
                    c.g[A1][0] = c.g[S1][0];
                    c.sdr(7, c.g[A0][0] as u32);
                    c.g[RA][0] = sx(0x003ea9b4);
                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x33c));
                    callee::LOBBY_LIST_UPDATE.call(c, 0x003ea9b4)?;
                    let t = true;

                    if t {
                        pc = 0x003ea9d8;
                        continue;
                    }
                    pc = 0x003ea9bc;
                    continue;
                }
                0x003ea9bc => {
                    c.cycles += 7;
                    c.g[RA][0] = sx(0x003ea9c4);
                    c.g[A1][0] = c.g[S4][0];
                    callee::LOBBY_HASH_DEL_STR.call(c, 0x003ea9c4)?;
                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x33c));
                    c.g[RA][0] = sx(0x003ea9d0);
                    c.g[A1][0] = c.g[V0][0];
                    callee::LOBBY_LIST_REMOVE.call(c, 0x003ea9d0)?;
                    c.g[RA][0] = sx(0x003ea9d8);
                    c.g[A0][0] = c.g[V0][0];
                    callee::LOBBY_MEM_FREE.call(c, 0x003ea9d8)?;
                    pc = 0x003ea9d8;
                    continue;
                }
                0x003ea9d8 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x003ea9e0);
                    c.g[A0][0] = c.g[S2][0];
                    callee::LOBBY_MEM_FREE.call(c, 0x003ea9e0)?;
                    let t = true;
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
                    if t {
                        pc = 0x003ea9f4;
                        continue;
                    }
                    pc = 0x003ea9e8;
                    continue;
                }
                0x003ea9e8 => {
                    c.cycles += 3;
                    c.g[RA][0] = sx(0x003ea9f0);
                    c.g[A0][0] = c.g[S2][0];
                    callee::LOBBY_MEM_FREE.call(c, 0x003ea9f0)?;
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
                    pc = 0x003ea9f4;
                    continue;
                }
                0x003ea9f4 => {
                    c.cycles += 2;
                    c.g[V0][0] = c.g[V0][0] | 0x80;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                    pc = 0x003ea9fc;
                    continue;
                }
                0x003ea9fc => {
                    c.cycles += 2;
                    c.g[V0][0] = 0x2b730000;
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x360));
                    pc = 0x003eaa04;
                    continue;
                }
                0x003eaa04 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[V0][0] | 0x6e70;
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[V0][0] = 0x6d6f0000;
                    if t {
                        pc = 0x003eac54;
                        continue;
                    }
                    pc = 0x003eaa10;
                    continue;
                }
                0x003eaa10 => {
                    c.cycles += 4;
                    c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x508));
                    c.g[V0][0] = ((c.g[V1][0] as i64) < 4i64) as u64;
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x6d6f0000;
                    if t {
                        pc = 0x003eac54;
                        continue;
                    }
                    pc = 0x003eaa20;
                    continue;
                }
                0x003eaa20 => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[V1][0] as i64) < 8i64) as u64;
                    let t = c.g[V0][0] == 0u64;
                    c.g[S6][0] = 0x14;
                    if t {
                        pc = 0x003eac50;
                        continue;
                    }
                    pc = 0x003eaa2c;
                    continue;
                }
                0x003eaa2c => {
                    c.cycles += 6;
                    c.g[S5][0] = sx((c.g[S3][0] as u32).wrapping_add(0x310));
                    c.mult(0, 2, 3, 22);
                    c.g[V0][0] = sx((c.g[S5][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                    let t = c.g[V1][0] == 0u64;
                    c.g[V0][0] = 0x6d6f0000;
                    if t {
                        pc = 0x003eac54;
                        continue;
                    }
                    pc = 0x003eaa44;
                    continue;
                }
                0x003eaa44 => {
                    c.cycles += 44;
                    c.g[RA][0] = sx(0x003eaa4c);
                    c.g[A0][0] = 0xa8;
                    callee::LOBBY_MEM_ALLOC.call(c, 0x003eaa4c)?;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.g[A1][0] = 0x4a0000;
                    c.g[S2][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x003eaa60);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x49c8));
                    callee::TAG_FIELD_FIND.call(c, 0x003eaa60)?;
                    c.g[S4][0] = sx((c.g[S2][0] as u32).wrapping_add(0x8));
                    c.g[A1][0] = 0u64;
                    c.g[RA][0] = sx(0x003eaa70);
                    c.g[A0][0] = c.g[V0][0];
                    callee::TAG_FIELD_GET_NUMBER.call(c, 0x003eaa70)?;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.g[A1][0] = 0x4a0000;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4a10));
                    c.g[RA][0] = sx(0x003eaa84);
                    c.sw((c.g[S2][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                    callee::TAG_FIELD_FIND.call(c, 0x003eaa84)?;
                    c.g[A1][0] = 0u64;
                    c.g[RA][0] = sx(0x003eaa90);
                    c.g[A0][0] = c.g[V0][0];
                    callee::TAG_FIELD_GET_NUMBER.call(c, 0x003eaa90)?;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.g[A1][0] = 0x4a0000;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x49b0));
                    c.g[RA][0] = sx(0x003eaaa4);
                    c.sw(c.g[S2][0] as u32, c.g[V0][0]);
                    callee::TAG_FIELD_FIND.call(c, 0x003eaaa4)?;
                    c.g[S0][0] = 0x4a0000;
                    c.g[A2][0] = 0x20;
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x4888));
                    c.g[A1][0] = c.g[S4][0];
                    c.g[A3][0] = c.g[S0][0];
                    c.g[RA][0] = sx(0x003eaac0);
                    c.g[A0][0] = c.g[V0][0];
                    callee::TAG_FIELD_GET_STRING.call(c, 0x003eaac0)?;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.g[A1][0] = 0x4a0000;
                    c.g[RA][0] = sx(0x003eaad0);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4a30));
                    callee::TAG_FIELD_FIND.call(c, 0x003eaad0)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A3][0] = c.g[S0][0];
                    c.g[A1][0] = sx((c.g[S2][0] as u32).wrapping_add(0x28));
                    c.g[RA][0] = sx(0x003eaae4);
                    c.g[A2][0] = 0x80;
                    callee::TAG_FIELD_GET_STRING.call(c, 0x003eaae4)?;
                    c.g[V1][0] = c.lw(c.g[S2][0] as u32);
                    c.g[V1][0] = ((c.g[V1][0] as i64) < 8192i64) as u64;

                    if c.g[V1][0] != 0u64 {
                        c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x508));
                        pc = 0x003eab00;
                        continue;
                    }
                    pc = 0x003eaaf4;
                    continue;
                }
                0x003eaaf4 => {
                    c.cycles += 3;
                    c.g[V0][0] = 0x1fff;
                    c.sw(c.g[S2][0] as u32, c.g[V0][0]);
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x508));
                    pc = 0x003eab00;
                    continue;
                }
                0x003eab00 => {
                    c.cycles += 8;
                    c.g[A1][0] = c.g[S4][0];
                    c.mult(0, 3, 2, 22);
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S5][0] as u32));
                    c.g[RA][0] = sx(0x003eab14);
                    c.g[A0][0] = c.lw(c.g[V0][0] as u32);
                    callee::LOBBY_HASH_FIND_STR.call(c, 0x003eab14)?;
                    c.g[S1][0] = c.g[V0][0];
                    let t = c.g[S1][0] != 0u64;
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x508));
                    if t {
                        pc = 0x003eab58;
                        continue;
                    }
                    pc = 0x003eab20;
                    continue;
                }
                0x003eab20 => {
                    c.cycles += 14;
                    c.g[A1][0] = c.g[S2][0];
                    c.mult(0, 3, 2, 22);
                    c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[S3][0] as u32));
                    c.g[RA][0] = sx(0x003eab34);
                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x314));
                    callee::LOBBY_LIST_ADD.call(c, 0x003eab34)?;
                    c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x508));
                    c.g[A2][0] = c.g[V0][0];
                    c.g[A1][0] = c.g[S4][0];
                    c.mult(0, 2, 3, 22);
                    c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S5][0] as u32));
                    c.g[RA][0] = sx(0x003eab50);
                    c.g[A0][0] = c.lw(c.g[V1][0] as u32);
                    callee::LOBBY_HASH_ADD_STR.call(c, 0x003eab50)?;
                    let t = true;
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
                    if t {
                        pc = 0x003eac48;
                        continue;
                    }
                    pc = 0x003eab58;
                    continue;
                }
                0x003eab58 => {
                    c.cycles += 12;
                    c.g[S0][0] = sx((c.g[S3][0] as u32).wrapping_add(0x314));
                    c.g[A1][0] = c.g[S1][0];
                    c.mult(0, 2, 2, 22);
                    c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.g[RA][0] = sx(0x003eab70);
                    c.g[A0][0] = c.lw(c.g[V0][0] as u32);
                    callee::LOBBY_LIST_GET_DATA.call(c, 0x003eab70)?;
                    c.g[A2][0] = c.g[V0][0];
                    c.g[V1][0] = c.g[S2][0];
                    c.g[V0][0] = c.g[S2][0] | c.g[A2][0];
                    c.g[V0][0] = c.g[V0][0] & 0x7;
                    let t = c.g[V0][0] == 0u64;
                    c.g[A0][0] = c.g[A2][0];
                    if t {
                        pc = 0x003eabe0;
                        continue;
                    }
                    pc = 0x003eab88;
                    continue;
                }
                0x003eab88 => {
                    c.cycles += 1;
                    c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(0xa0));
                    pc = 0x003eab8c;
                    continue;
                }
                0x003eab8c => {
                    c.cycles += 19;
                    c.ldl(5, (c.g[V1][0] as u32).wrapping_add(0x7));
                    c.ldr(5, c.g[V1][0] as u32);
                    c.ldl(7, (c.g[V1][0] as u32).wrapping_add(0xf));
                    c.ldr(7, (c.g[V1][0] as u32).wrapping_add(0x8));
                    c.ldl(8, (c.g[V1][0] as u32).wrapping_add(0x17));
                    c.ldr(8, (c.g[V1][0] as u32).wrapping_add(0x10));
                    c.ldl(9, (c.g[V1][0] as u32).wrapping_add(0x1f));
                    c.ldr(9, (c.g[V1][0] as u32).wrapping_add(0x18));
                    c.sdl(5, (c.g[A0][0] as u32).wrapping_add(0x7));
                    c.sdr(5, c.g[A0][0] as u32);
                    c.sdl(7, (c.g[A0][0] as u32).wrapping_add(0xf));
                    c.sdr(7, (c.g[A0][0] as u32).wrapping_add(0x8));
                    c.sdl(8, (c.g[A0][0] as u32).wrapping_add(0x17));
                    c.sdr(8, (c.g[A0][0] as u32).wrapping_add(0x10));
                    c.sdl(9, (c.g[A0][0] as u32).wrapping_add(0x1f));
                    c.sdr(9, (c.g[A0][0] as u32).wrapping_add(0x18));
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
                    if t {
                        if c.cycles >= c.next_event {
                            c.pc = 0x003eab8c;
                            if c.event() {
                                return Ok(());
                            }
                        }
                        pc = 0x003eab8c;
                        continue;
                    }
                    pc = 0x003eabd8;
                    continue;
                }
                0x003eabd8 => {
                    c.cycles += 2;
                    let t = true;

                    if t {
                        pc = 0x003eac10;
                        continue;
                    }
                    pc = 0x003eabe0;
                    continue;
                }
                0x003eabe0 => {
                    c.cycles += 1;
                    c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(0xa0));
                    pc = 0x003eabe4;
                    continue;
                }
                0x003eabe4 => {
                    c.cycles += 11;
                    c.g[A1][0] = c.ld(c.g[V1][0] as u32);
                    c.g[A3][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x8));
                    c.g[T0][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x10));
                    c.g[T1][0] = c.ld((c.g[V1][0] as u32).wrapping_add(0x18));
                    c.sd(c.g[A0][0] as u32, c.g[A1][0]);
                    c.sd((c.g[A0][0] as u32).wrapping_add(0x8), c.g[A3][0]);
                    c.sd((c.g[A0][0] as u32).wrapping_add(0x10), c.g[T0][0]);
                    c.sd((c.g[A0][0] as u32).wrapping_add(0x18), c.g[T1][0]);
                    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x20));
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
                    if t {
                        if c.cycles >= c.next_event {
                            c.pc = 0x003eabe4;
                            if c.event() {
                                return Ok(());
                            }
                        }
                        pc = 0x003eabe4;
                        continue;
                    }
                    pc = 0x003eac10;
                    continue;
                }
                0x003eac10 => {
                    c.cycles += 14;
                    c.ldl(2, (c.g[V1][0] as u32).wrapping_add(0x7));
                    c.ldr(2, c.g[V1][0] as u32);
                    c.sdl(2, (c.g[A0][0] as u32).wrapping_add(0x7));
                    c.sdr(2, c.g[A0][0] as u32);
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x508));
                    c.g[V1][0] = 0x14;
                    c.g[A1][0] = c.g[S1][0];
                    c.mult(0, 4, 2, 3);
                    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[S0][0] as u32));
                    c.g[RA][0] = sx(0x003eac3c);
                    c.g[A0][0] = c.lw(c.g[V0][0] as u32);
                    callee::LOBBY_LIST_UPDATE.call(c, 0x003eac3c)?;
                    c.g[RA][0] = sx(0x003eac44);
                    c.g[A0][0] = c.g[S2][0];
                    callee::LOBBY_MEM_FREE.call(c, 0x003eac44)?;
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
                    pc = 0x003eac48;
                    continue;
                }
                0x003eac48 => {
                    c.cycles += 2;
                    c.g[V0][0] = c.g[V0][0] | 0x80;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                    pc = 0x003eac50;
                    continue;
                }
                0x003eac50 => {
                    c.cycles += 1;
                    c.g[V0][0] = 0x6d6f0000;
                    pc = 0x003eac54;
                    continue;
                }
                0x003eac54 => {
                    c.cycles += 4;
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x360));
                    c.g[V0][0] = c.g[V0][0] | 0x7665;
                    let t = c.g[V1][0] == c.g[V0][0];
                    c.g[V0][0] = 0x726f0000;
                    if t {
                        pc = 0x003eac70;
                        continue;
                    }
                    pc = 0x003eac64;
                    continue;
                }
                0x003eac64 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[V0][0] | 0x6f6d;
                    let t = c.g[V1][0] != c.g[V0][0];
                    c.g[V0][0] = 0x6d6f0000;
                    if t {
                        pc = 0x003eacd4;
                        continue;
                    }
                    pc = 0x003eac70;
                    continue;
                }
                0x003eac70 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x364));
                    let t = c.g[V0][0] != 0u64;
                    c.g[V0][0] = 0x6d6f0000;
                    if t {
                        pc = 0x003eacd0;
                        continue;
                    }
                    pc = 0x003eac7c;
                    continue;
                }
                0x003eac7c => {
                    c.cycles += 10;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.g[A1][0] = 0x4a0000;
                    c.g[RA][0] = sx(0x003eac8c);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4a48));
                    callee::TAG_FIELD_FIND.call(c, 0x003eac8c)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x003eac98);
                    c.g[A1][0] = 0xffffffffffffffff;
                    callee::TAG_FIELD_GET_NUMBER.call(c, 0x003eac98)?;
                    c.g[S0][0] = c.g[V0][0];
                    let t = (c.g[S0][0] as i64) < 0;
                    c.g[A1][0] = 0x4a0000;
                    if t {
                        pc = 0x003eaccc;
                        continue;
                    }
                    pc = 0x003eaca4;
                    continue;
                }
                0x003eaca4 => {
                    c.cycles += 10;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.g[RA][0] = sx(0x003eacb0);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4a50));
                    callee::TAG_FIELD_FIND.call(c, 0x003eacb0)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x003eacbc);
                    c.g[A1][0] = 0u64;
                    callee::TAG_FIELD_GET_NUMBER.call(c, 0x003eacbc)?;
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A2][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x003eaccc);
                    c.g[A0][0] = c.g[S3][0];
                    callee::LOBBY_API_UPDATE_ROOM_COUNT.call(c, 0x003eaccc)?;
                    pc = 0x003eaccc;
                    continue;
                }
                0x003eaccc => {
                    c.cycles += 1;
                    c.g[V0][0] = 0x6d6f0000;
                    pc = 0x003eacd0;
                    continue;
                }
                0x003eacd0 => {
                    c.cycles += 1;
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x360));
                    pc = 0x003eacd4;
                    continue;
                }
                0x003eacd4 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.g[V0][0] | 0x7665;

                    if c.g[V1][0] != c.g[V0][0] {
                        c.g[A0][0] = c.lw(c.g[S3][0] as u32);
                        pc = 0x003eadd4;
                        continue;
                    }
                    pc = 0x003eace0;
                    continue;
                }
                0x003eace0 => {
                    c.cycles += 19;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.g[A1][0] = 0x4a0000;
                    c.g[RA][0] = sx(0x003eacf0);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4a58));
                    callee::TAG_FIELD_FIND.call(c, 0x003eacf0)?;
                    c.g[A1][0] = 0u64;
                    c.g[RA][0] = sx(0x003eacfc);
                    c.g[A0][0] = c.g[V0][0];
                    callee::TAG_FIELD_GET_NUMBER.call(c, 0x003eacfc)?;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.g[A1][0] = 0x4a0000;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4a60));
                    c.g[RA][0] = sx(0x003ead10);
                    c.g[S0][0] = c.g[V0][0];
                    callee::TAG_FIELD_FIND.call(c, 0x003ead10)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x003ead1c);
                    c.g[A1][0] = 0u64;
                    callee::TAG_FIELD_GET_NUMBER.call(c, 0x003ead1c)?;
                    c.g[A2][0] = c.g[V0][0];
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x1b8));
                    let t = c.g[S0][0] == c.g[V0][0];
                    c.g[A1][0] = c.g[S0][0];
                    if t {
                        pc = 0x003eadd0;
                        continue;
                    }
                    pc = 0x003ead2c;
                    continue;
                }
                0x003ead2c => {
                    c.cycles += 26;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x1b8), c.g[S0][0]);
                    c.g[RA][0] = sx(0x003ead38);
                    c.g[A0][0] = c.g[S3][0];
                    callee::LOBBY_API_UPDATE_ROOM_COUNT.call(c, 0x003ead38)?;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.g[A1][0] = 0x4a0000;
                    c.g[RA][0] = sx(0x003ead48);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4880));
                    callee::TAG_FIELD_FIND.call(c, 0x003ead48)?;
                    c.g[A3][0] = 0x4a0000;
                    c.g[A2][0] = 0x40;
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x4888));
                    c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_add(0x1c0));
                    c.g[RA][0] = sx(0x003ead60);
                    c.g[A0][0] = c.g[V0][0];
                    callee::TAG_FIELD_GET_STRING.call(c, 0x003ead60)?;
                    c.g[A0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x368));
                    c.g[A1][0] = 0x4a0000;
                    c.g[RA][0] = sx(0x003ead70);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4a68));
                    callee::TAG_FIELD_FIND.call(c, 0x003ead70)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x003ead7c);
                    c.g[A1][0] = 0u64;
                    callee::TAG_FIELD_GET_FLAGS.call(c, 0x003ead7c)?;
                    c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x324));
                    c.g[V1][0] = c.g[V1][0] | 0x8;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x1bc), c.g[V0][0]);
                    let t = c.g[A0][0] == 0u64;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x8), c.g[V1][0]);
                    if t {
                        pc = 0x003eadd0;
                        continue;
                    }
                    pc = 0x003ead94;
                    continue;
                }
                0x003ead94 => {
                    c.cycles += 2;
                    let t = true;

                    if t {
                        pc = 0x003eadb0;
                        continue;
                    }
                    pc = 0x003ead9c;
                    continue;
                }
                0x003ead9c => {
                    c.cycles += 1;
                    pc = 0x003eada0;
                    continue;
                }
                0x003eada0 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x003eada8);
                    c.g[A1][0] = c.g[S1][0];
                    callee::LOBBY_LIST_REMOVE.call(c, 0x003eada8)?;
                    c.g[RA][0] = sx(0x003eadb0);
                    c.g[A0][0] = c.g[V0][0];
                    callee::LOBBY_MEM_FREE.call(c, 0x003eadb0)?;
                    pc = 0x003eadb0;
                    continue;
                }
                0x003eadb0 => {
                    c.cycles += 5;
                    c.g[RA][0] = sx(0x003eadb8);
                    c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x324));
                    callee::LOBBY_HASH_POP_ANY.call(c, 0x003eadb8)?;
                    c.g[S1][0] = c.g[V0][0];

                    if c.g[S1][0] != 0u64 {
                        c.g[A0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x328));
                        if c.cycles >= c.next_event {
                            c.pc = 0x003eada0;
                            if c.event() {
                                return Ok(());
                            }
                        }
                        pc = 0x003eada0;
                        continue;
                    }
                    pc = 0x003eadc4;
                    continue;
                }
                0x003eadc4 => {
                    c.cycles += 3;
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
                    c.g[V0][0] = c.g[V0][0] | 0x40;
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                    pc = 0x003eadd0;
                    continue;
                }
                0x003eadd0 => {
                    c.cycles += 1;
                    c.g[A0][0] = c.lw(c.g[S3][0] as u32);
                    pc = 0x003eadd4;
                    continue;
                }
                0x003eadd4 => {
                    c.cycles += 5;
                    c.g[A1][0] = 0u64;
                    c.g[A2][0] = 0u64;
                    c.g[A3][0] = 0u64;
                    c.g[RA][0] = sx(0x003eade8);
                    c.g[T0][0] = 0u64;
                    callee::PROTO_ARIES_RECV.call(c, 0x003eade8)?;
                    pc = 0x003eade8;
                    continue;
                }
                // Peek the next message; empty means fall out of the loop.
                0x003eade8 => {
                    c.cycles += 8;
                    c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x36c));
                    c.g[A1][0] = c.g[FP][0];
                    c.g[A0][0] = c.lw(c.g[S3][0] as u32);
                    c.g[RA][0] = sx(0x003eadfc);
                    c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x370));
                    callee::PROTO_ARIES_PEEK.call(c, 0x003eadfc)?;
                    c.g[A2][0] = c.g[V0][0];
                    let t = (c.g[A2][0] as i64) >= 0;
                    c.g[V0][0] = 0xea60;
                    if t {
                        if c.cycles >= c.next_event {
                            c.pc = 0x003e96b0;
                            if c.event() {
                                return Ok(());
                            }
                        }
                        pc = 0x003e96b0;
                        continue;
                    }
                    pc = 0x003eae08;
                    continue;
                }
                // Lists are sorted; re-arm the server ping when due.
                0x003eae08 => {
                    c.cycles += 5;
                    c.g[RA][0] = sx(0x003eae10);
                    c.g[A0][0] = c.g[S3][0];
                    callee::LOBBY_API_SORT_LISTS.call(c, 0x003eae10)?;
                    c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x18));
                    let t = c.g[V0][0] == 0u64;
                    c.g[V0][0] = (c.g[S7][0] < c.g[V0][0]) as u64;
                    if t {
                        pc = 0x003eae34;
                        continue;
                    }
                    pc = 0x003eae1c;
                    continue;
                }
                0x003eae1c => {
                    c.cycles += 2;
                    let t = c.g[V0][0] != 0u64;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x410));
                    if t {
                        pc = 0x003eae38;
                        continue;
                    }
                    pc = 0x003eae24;
                    continue;
                }
                0x003eae24 => {
                    c.cycles += 4;
                    c.g[RA][0] = sx(0x003eae2c);
                    c.g[A0][0] = c.g[S3][0];
                    callee::LOBBY_API_PING_SERVER.call(c, 0x003eae2c)?;
                    c.g[V0][0] = sx((c.g[S7][0] as u32).wrapping_add(0x7530));
                    c.sw((c.g[S3][0] as u32).wrapping_add(0x18), c.g[V0][0]);
                    pc = 0x003eae34;
                    continue;
                }
                // Epilogue: restore the frame and return to the caller.
                0x003eae34 => {
                    c.cycles += 1;
                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x410));
                    pc = 0x003eae38;
                    continue;
                }
                0x003eae38 => {
                    c.cycles += 11;
                    c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x400));
                    c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x3f0));
                    c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x3e0));
                    c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x3d0));
                    c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x3c0));
                    c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x3b0));
                    c.lq(23, (c.g[SP][0] as u32).wrapping_add(0x3a0));
                    c.lq(30, (c.g[SP][0] as u32).wrapping_add(0x390));
                    c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x380));
                    let t = c.g[RA][0] as u32;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x420));
                    c.pc = t;
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x003e95c8, 0x003eae64);
                    return Ok(());
                }
            }
        }
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::check_lohi;
    use crate::difftest::check_paths;
    use crate::difftest::check_paths_lohi;
    use crate::difftest::elf_image;
    use crate::difftest::events::{check_events, driven, schedule, Events};
    use crate::difftest::fakes;
    use crate::difftest::fakes::machine;
    use crate::difftest::fakes::FAKE;
    use crate::difftest::fill;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::difftest::RA as TEST_RA;
    use crate::game::dirtysock::buddyapi::tests::require_arm_values;
    use ssx3_ee as ee;
    use ssx3_ee_rt::funcs::GuestFn;
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// The main heap over 256 KiB of test RAM, so `Net_RpcAlloc` has an
    /// arena to allocate from.
    pub(crate) fn link_arena(c: &mut Cpu) {
        c.g[GP][0] = GAME_GP as u64;
        c.g[A0][0] = 0x004f_f250;
        c.g[A1][0] = (DATA + 0x6_0000) as u64;
        c.g[A2][0] = 0x4_0000;
        c.g[ssx3_ee_rt::cpu::reg::RA][0] = TEST_RA as u64;
        c.pc = 0x0031_ff60;
        ee::s10::DLHeap_InitArena(c);
        assert_eq!(c.pc, TEST_RA, "arena init did not return");
    }

    /// A refill block at `buf` as `NetGameLink_CheckRecvBuf` accepts it:
    /// header {end pointer, -, sequence, length}, `len` data bytes, then a
    /// trailer {uncached start, -, sequence} at the 16-byte-rounded end.
    fn link_recv_block(c: &mut Cpu, buf: u32, len: u32, seq: u32) {
        let end = buf + ((len + 0xf) & !0xf) + 0x10;
        c.bus.write32(buf, end);
        c.bus.write32(buf + 8, seq);
        c.bus.write32(buf + 0xc, len);
        c.bus.write32(end, (buf & 0x0fff_ffff) | 0x2000_0000);
        c.bus.write32(end + 8, seq);
    }

    #[test]
    fn net_game_link_create_matches() {
        check(
            ee::s14::f_003e66f0__generated,
            net_game_link_create,
            24,
            &|c, seed| {
                machine(c, seed, 0x3e66f0, 0);
                link_arena(c);
                c.pc = 0x003e_66f0;
                c.g[A0][0] = (DATA + 0x100) as u64;
                c.bus.write32(DATA + 0x100, 0x5555_0000 + seed);
                c.g[A1][0] = (seed as u64).wrapping_mul(0x9e37);
                c.g[A2][0] = (0x40 + (seed % 4) * 0x40) as u64;
            },
        );
    }

    #[test]
    fn net_game_link_destroy_matches() {
        check(
            ee::s14::f_003e67f8__generated,
            net_game_link_destroy,
            16,
            &|c, seed| {
                machine(c, seed, 0x3e67f8, 0);
                link_arena(c);
                if seed % 2 == 0 {
                    // A live heap object, like the create path hands out.
                    c.g[A0][0] = 0x394;
                    c.g[ssx3_ee_rt::cpu::reg::RA][0] = TEST_RA as u64;
                    c.pc = 0x0025_6bb0;
                    ee::s06::f_00256bb0(c);
                    assert_eq!(c.pc, TEST_RA);
                    let link = c.g[V0][0] as u32;
                    for i in 0..8u32 {
                        c.bus.write32(link + 4 * i, 0xA500_0000 + seed + i);
                    }
                    c.g[A0][0] = link as u64;
                } else {
                    c.g[A0][0] = 0;
                }
                c.pc = 0x003e_67f8;
            },
        );
    }

    driven!(gen_copy_stats, ee::s14::f_003e6878__generated);

    driven!(hand_copy_stats, net_game_link_copy_stats);

    #[test]
    fn net_game_link_copy_stats_events_matches() {
        check_events(gen_copy_stats, hand_copy_stats, 48, &|c, seed| {
            let mut r = machine(c, seed, 0x3e6878, 0);
            // Aligned ld/sd loop on even seeds, unaligned ldl/ldr loop when
            // either endpoint is unaligned.
            let link = DATA + 0x100 + (seed % 2) * 4;
            let src = DATA + 0x2000 + (seed / 2 % 2) * 4;
            for i in 0..0x54u32 {
                c.bus.write32(src + 4 * i, r.next_u32() | 0x0101_0101);
            }
            for i in 0..0x54u32 {
                c.bus.write32(link + 8 + 4 * i, 0xA5A5_0000 + seed + i);
            }
            c.bus.write32(link + 4, 0xDEAD_0000 + seed);
            c.bus.write32(link + 0x158, src);
            let mode = schedule(c, &mut r, seed, 0x003e_6878);
            // Land the first event on the first back edge of whichever loop
            // runs (entry 12 cycles, then 19/11 per iteration).
            if mode != Events::Off {
                c.next_event = c.cycles + 4;
            }
            c.bus.write32(c.g[SP][0] as u32, link);
            mode
        });
    }

    // Totals: in flight 21, idle 114, first call 256 (aligned stats copy)
    // and 338 (unaligned).
    #[test]
    fn net_game_link_status_matches() {
        check_paths(
            ee::s14::f_003e6958__generated,
            net_game_link_status,
            36,
            &[21, 114, 256, 338],
            &|c, seed| {
                let mut r = machine(c, seed, 0x3e6958, 0);
                c.pc = 0x003e_6958;
                let link = DATA + 0x100;
                // Never fetched (-1: blocking RPC plus copy), idle (0:
                // asynchronous RPC), or a request in flight (returns at once).
                c.bus
                    .write32(link + 4, [0xffff_ffff, 0, 5][seed as usize % 3]);
                c.bus.write32(link, 0x1111_0000 + seed);
                // Stats block aligned or not (the copy's two loops).
                let src = DATA + 0x2000 + (seed / 3 % 2) * 4;
                for i in 0..0x54u32 {
                    c.bus.write32(src + 4 * i, r.next_u32() | 0x0202_0202);
                }
                for i in 0..0x54u32 {
                    c.bus.write32(link + 8 + 4 * i, 0xA5A5_0000 + seed + i);
                }
                c.bus.write32(link + NET_GAME_LINK_STATS, src);
                c.g[A0][0] = link as u64;
            },
        );
    }

    // Totals: no `out` 12, empty 18, a packet 75/96/117/138 (by length).
    #[test]
    fn net_game_link_peek_packet_matches() {
        check_paths(
            ee::s14::f_003e6a70__generated,
            net_game_link_peek_packet,
            36,
            &[12, 18, 75, 96, 117, 138],
            &|c, seed| {
                let mut r = machine(c, seed, 0x3e6a70, 0);
                c.pc = 0x003e_6a70;
                let link = DATA + 0x100;
                let base = DATA + 0x3000;
                let out_ptr = DATA + 0x200;
                let out = DATA + 0x400;
                c.bus.write32(link + NET_GAME_LINK_RECV_BUF, base);
                c.bus.write32(out_ptr, out);
                for i in 0..0x10u32 {
                    c.bus.write32(out + 4 * i, 0xA5A5_0000 + seed + i);
                }
                for i in 0..0x20u32 {
                    c.bus.write32(base + 4 * i, r.next_u32() | 0x0303_0303);
                }
                // A non-zero read cursor, and a different length at the
                // buffer start, so a missing `buf + read` copies the wrong
                // bytes.
                let read = 0x10 + 4 * (seed / 3 % 3);
                let len = 4 + seed / 9 % 4 * 3;
                c.bus.write16(base, 0x3c);
                c.bus.write16(base + read, len as u16);
                c.bus.write32(link + NET_GAME_LINK_RECV_READ, read);
                match seed % 3 {
                    // No out pointer: nothing stored.
                    0 => {
                        c.g[A1][0] = 0;
                        c.bus.write32(link + NET_GAME_LINK_RECV_WRITE, read + 0x20);
                    }
                    // Empty: read caught up with write, `*out = NULL`.
                    1 => {
                        c.g[A1][0] = out_ptr as u64;
                        c.bus.write32(link + NET_GAME_LINK_RECV_WRITE, read);
                    }
                    // A queued packet: copied whole to `*out`.
                    _ => {
                        c.g[A1][0] = out_ptr as u64;
                        c.bus.write32(link + NET_GAME_LINK_RECV_WRITE, read + 0x20);
                    }
                }
                c.g[A0][0] = link as u64;
            },
        );
    }

    // Totals by arm (seed % 8): drained 235; peek 112/154 and peek-last
    // 321/363 (by length); copy too small 30; copy 282/490 (source aligned or
    // not); refill refused 41, same sequence 69, new sequence 78 (copy too
    // small) and 330 (copied).
    #[test]
    fn net_game_link_recv_matches() {
        check_paths(
            ee::s14::f_003e6b10__generated,
            net_game_link_recv,
            64,
            &[30, 41, 69, 78, 112, 154, 235, 282, 321, 330, 363, 490],
            &|c, seed| {
                let mut r = machine(c, seed, 0x3e6b10, 0);
                c.pc = 0x003e_6b10;
                let link = DATA + 0x100;
                let base = DATA + 0x3000;
                let out = DATA + 0x400;
                c.bus.write32(link + NET_GAME_LINK_RECV_BUF, base);
                c.bus.write32(link, 0x2222_0000 + seed);
                for i in 0..0x20u32 {
                    c.bus.write32(out + 4 * i, 0xA5A5_0000 + seed + i);
                }
                for i in 0..0x40u32 {
                    c.bus.write32(base + 4 * i, r.next_u32() | 0x0404_0404);
                }
                // A non-zero read cursor with a packet length there, and a
                // different length at the buffer start, so a missing
                // `buf + read` copies the wrong bytes.
                let read = 0x10 + 4 * (seed / 8 % 4);
                let len = 4 + seed / 32 % 2 * 6;
                c.bus.write16(base, 0x3c);
                c.bus.write16(base + read, len as u16);
                let (maxlen, write) = match seed % 8 {
                    // Drained: cursors meet, buffer handed back.
                    0 => (0x10, read),
                    // Peek: advance past one packet, bytes remain.
                    1 => (1, read + len + 0x10),
                    // Peek: the packet was the last one, buffer handed back.
                    2 => (1, read + len),
                    // Copy: too much queued for `maxlen`, return 0.
                    3 => (8, read + 0x20),
                    // Copy: all queued bytes (room to spare or exactly
                    // `maxlen`), then buffer handed back.
                    4 => ([0x40, 0x20][seed as usize / 32 % 2], read + 0x20),
                    // Refill refused (no block): return 0.
                    5 => {
                        c.bus.write32(base, 0);
                        (8, 0xffff_ffff)
                    }
                    // Refill reports the sequence already taken: cursors
                    // untouched, return 0.
                    6 => {
                        // Sequence 0 (the refusal test is `< 0`) or another.
                        let seq = if seed / 32 % 2 == 1 {
                            0
                        } else {
                            0x1111_0000 + seed
                        };
                        link_recv_block(c, base, 0x20, seq);
                        c.bus.write32(link + NET_GAME_LINK_RECV_SEQ, seq);
                        (8, 0xffff_ffff)
                    }
                    // Refill reports a new sequence: cursors set to the
                    // block, then a copy too small (return 0) or of all 0x20
                    // bytes.
                    _ => {
                        let seq = 0x1111_0000 + seed;
                        link_recv_block(c, base, 0x20, seq);
                        c.bus.write32(link + NET_GAME_LINK_RECV_SEQ, seq ^ 0x1111);
                        ([8, 0x40][seed as usize / 8 % 2], 0xffff_ffff)
                    }
                };
                if seed % 8 >= 5 {
                    c.bus.write32(link + NET_GAME_LINK_RECV_READ, 0);
                } else {
                    c.bus.write32(link + NET_GAME_LINK_RECV_READ, read);
                }
                c.bus.write32(link + NET_GAME_LINK_RECV_WRITE, write);
                if seed % 8 == 0 && seed / 32 % 2 == 1 {
                    // Drained at cursor 0 (the refill test is `write < 0`).
                    c.bus.write32(link + NET_GAME_LINK_RECV_READ, 0);
                    c.bus.write32(link + NET_GAME_LINK_RECV_WRITE, 0);
                }
                c.g[A0][0] = link as u64;
                c.g[A1][0] = out as u64;
                c.g[A2][0] = maxlen;
            },
        );
    }

    // Totals: window full 36 (packet 40), sent 125 (packet 129).
    #[test]
    fn net_game_link_send_matches() {
        check_paths(
            ee::s14::f_003e6c80__generated,
            net_game_link_send,
            48,
            &[36, 40, 125, 129],
            &|c, seed| {
                let mut r = machine(c, seed, 0x3e6c80, 0);
                c.pc = 0x003e_6c80;
                let link = DATA + 0x100;
                let done = DATA + 0x3000;
                let buf = DATA + 0x200;
                for i in 0..0x10u32 {
                    c.bus.write32(buf + 4 * i, r.next_u32() | 0x0505_0505);
                }
                // Window 0x400 bytes, the IOP has consumed all but `in_flight`
                // of the sent bytes: free space 0x3f0 (room), 0x10 (full for
                // every length below) or, for a given length, exactly it.
                let sent = 0x1_2340 + seed * 0x100;
                let len = 0x18 + seed / 4 % 4 * 0x20;
                let exact = if seed % 2 == 1 { 0x400 - len } else { 0x10 };
                let in_flight = [0x10, 0x3f0, exact][seed as usize / 16 % 3];
                c.bus.write32(done, sent - in_flight);
                c.bus.write32(link, 0x3333_0000 + seed);
                c.bus.write32(link + NET_GAME_LINK_SEND_DONE, done);
                c.bus.write32(link + NET_GAME_LINK_SEND_SENT, sent);
                c.bus.write32(link + NET_GAME_LINK_SEND_SIZE, 0x400);
                if seed % 2 == 0 {
                    // Packet (`len == 1`): length 0x14..0x54 from buf[3].
                    c.g[A2][0] = 1;
                    c.bus.write8(buf + 3, (seed / 4 % 64) as u8);
                } else {
                    c.g[A2][0] = len as u64;
                }
                c.g[A0][0] = link as u64;
                c.g[A1][0] = buf as u64;
            },
        );
    }

    #[test]
    fn net_game_link_control_matches() {
        check(
            ee::s14::f_003e6d78__generated,
            net_game_link_control,
            24,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3e6d78, 0);
                c.pc = 0x003e_6d78;
                let link = DATA + 0x100;
                for i in 0..8u32 {
                    c.bus.write32(link + 4 * i, r.next_u32() | 0x0606_0606);
                }
                c.bus.write32(link, 0x4444_0000 + seed);
                // The reply word slot on the frame: pre-filled non-zero so
                // a missing load changes RAM.
                c.bus
                    .write32(crate::difftest::SP - 0x60 + 0x14, 0x1234_0000 + seed);
                c.g[A0][0] = link as u64;
                c.g[A1][0] = (0x1000 + seed) as u64;
                c.g[A2][0] = (0x2000 + seed) as u64;
            },
        );
    }

    #[test]
    fn net_game_util_create_matches() {
        check(
            ee::s14::f_003e6e08__generated,
            net_game_util_create,
            24,
            &|c, seed| {
                machine(c, seed, 0x3e6e08, 0);
                c.pc = 0x003e_6e08;
                c.bus
                    .write32(crate::difftest::SP - 0x40 + 0x20, 0x2345_0000 + seed);
            },
        );
    }

    #[test]
    fn net_game_util_destroy_matches() {
        check(
            ee::s14::f_003e6e70__generated,
            net_game_util_destroy,
            16,
            &|c, seed| {
                machine(c, seed, 0x3e6e70, 0);
                c.pc = 0x003e_6e70;
                if seed % 2 == 0 {
                    c.bus.write32(DATA + 0x100, 0x7777_0000 + seed);
                    c.g[A0][0] = (DATA + 0x100) as u64;
                } else {
                    c.g[A0][0] = 0;
                }
            },
        );
    }

    #[test]
    fn net_game_util_reset_matches() {
        check(
            ee::s14::f_003e6ed8__generated,
            net_game_util_reset,
            16,
            &|c, seed| {
                machine(c, seed, 0x3e6ed8, 0);
                c.pc = 0x003e_6ed8;
                if seed % 2 == 0 {
                    c.bus.write32(DATA + 0x100, 0x8888_0000 + seed);
                    c.g[A0][0] = (DATA + 0x100) as u64;
                } else {
                    c.g[A0][0] = 0;
                }
            },
        );
    }

    #[test]
    fn net_game_util_connect_matches() {
        check(
            ee::s14::f_003e6f40__generated,
            net_game_util_connect,
            24,
            &|c, seed| {
                machine(c, seed, 0x3e6f40, 0);
                c.pc = 0x003e_6f40;
                if seed % 2 == 0 {
                    c.bus.write32(DATA + 0x100, 0x9999_0000 + seed);
                    c.g[A0][0] = (DATA + 0x100) as u64;
                } else {
                    c.g[A0][0] = 0;
                }
                c.g[A1][0] = (0x3000 + seed) as u64;
                c.g[A2][0] = (0x4000 + seed) as u64;
                c.bus
                    .write32(crate::difftest::SP - 0x70 + 0x20, 0x3456_0000 + seed);
            },
        );
    }

    #[test]
    fn net_game_util_complete_matches() {
        check(
            ee::s14::f_003e6fc8__generated,
            net_game_util_complete,
            24,
            &|c, seed| {
                machine(c, seed, 0x3e6fc8, 0);
                c.pc = 0x003e_6fc8;
                if seed % 2 == 0 {
                    c.bus.write32(DATA + 0x100, 0xAAAA_0000 + seed);
                    c.g[A0][0] = (DATA + 0x100) as u64;
                } else {
                    c.g[A0][0] = 0;
                }
                c.bus
                    .write32(crate::difftest::SP - 0x50 + 0x20, 0x4567_0000 + seed);
            },
        );
    }

    #[test]
    fn net_game_util_status_matches() {
        check(
            ee::s14::f_003e7038__generated,
            net_game_util_status,
            24,
            &|c, seed| {
                machine(c, seed, 0x3e7038, 0);
                c.pc = 0x003e_7038;
                c.bus.write32(DATA + 0x100, 0xBBBB_0000 + seed);
                c.g[A0][0] = (DATA + 0x100) as u64;
                c.g[A1][0] = (seed % 5) as u64;
                c.bus
                    .write32(crate::difftest::SP - 0x60 + 0x20, 0x5678_0000 + seed);
            },
        );
    }

    #[test]
    fn net_game_util_stat_matches() {
        check(
            ee::s14::f_003e70b8__generated,
            net_game_util_stat,
            16,
            &|c, seed| {
                machine(c, seed, 0x3e70b8, 0);
                c.pc = 0x003e_70b8;
                if seed % 2 == 0 {
                    c.bus.write32(DATA + 0x100, 0xCCCC_0000 + seed);
                    c.g[A0][0] = (DATA + 0x100) as u64;
                } else {
                    c.g[A0][0] = 0;
                }
            },
        );
    }

    #[test]
    fn net_game_util_advert_matches() {
        check(
            ee::s14::f_003e7120__generated,
            net_game_util_advert,
            24,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3e7120, 0);
                elf_image(c);
                // The format buffer at sp+0x10 of the 0x150 frame: non-zero
                // below the part `machine` fills, so a missing NUL shows.
                fill(c, &mut r, crate::difftest::SP - 0x150, 0x14);
                c.pc = 0x003e_7120;
                c.bus.write32(DATA + 0x100, 0xDDDD_0000 + seed);
                c.g[A0][0] = (DATA + 0x100) as u64;
                let names: [&[u8]; 4] = [b"srv1", b"a-longer-name", b"", b"x"];
                let rooms: [&[u8]; 4] = [b"room", b"r2", b"the-lobby", b""];
                lobby_str(c, DATA + 0x500, names[seed as usize % 4]);
                lobby_str(c, DATA + 0x600, rooms[seed as usize / 4 % 4]);
                c.g[A1][0] = (DATA + 0x500) as u64;
                c.g[A2][0] = (DATA + 0x600) as u64;
            },
        );
    }

    #[test]
    fn net_game_util_withdraw_matches() {
        check(
            ee::s14::f_003e71b0__generated,
            net_game_util_withdraw,
            16,
            &|c, seed| {
                machine(c, seed, 0x3e71b0, 0);
                c.pc = 0x003e_71b0;
                if seed % 2 == 0 {
                    c.bus.write32(DATA + 0x100, 0xEEEE_0000 + seed);
                    c.g[A0][0] = (DATA + 0x100) as u64;
                } else {
                    c.g[A0][0] = 0;
                }
            },
        );
    }

    // Totals: default name 101, given name 99.
    #[test]
    fn net_game_util_locate_matches() {
        check_paths(
            ee::s14::f_003e7218__generated,
            net_game_util_locate,
            24,
            &[99, 101],
            &|c, seed| {
                machine(c, seed, 0x3e7218, 0);
                c.pc = 0x003e_7218;
                c.bus.write32(DATA + 0x100, 0xFFFF_0000 + seed);
                c.g[A0][0] = (DATA + 0x100) as u64;
                if seed % 2 == 0 {
                    c.g[A2][0] = 0;
                } else {
                    lobby_str(c, DATA + 0x500, b"arena-7");
                    c.g[A2][0] = (DATA + 0x500) as u64;
                }
                c.bus
                    .write32(crate::difftest::SP - 0x60 + 0x20, 0x6789_0000 + seed);
            },
        );
    }

    // Totals: default name 107, given name 105.
    #[test]
    fn net_game_util_query_matches() {
        check_paths(
            ee::s14::f_003e72a0__generated,
            net_game_util_query,
            32,
            &[105, 107],
            &|c, seed| {
                machine(c, seed, 0x3e72a0, 0);
                c.pc = 0x003e_72a0;
                c.bus.write32(DATA + 0x100, 0x1212_0000 + seed);
                c.g[A0][0] = (DATA + 0x100) as u64;
                if seed % 2 == 0 {
                    c.g[A1][0] = 0;
                } else {
                    lobby_str(c, DATA + 0x500, b"query-x");
                    c.g[A1][0] = (DATA + 0x500) as u64;
                }
                c.g[A2][0] = (0x5000 + seed / 2 % 0x100) as u64;
                c.g[A3][0] = (0x6000 + seed / 4 % 0x100) as u64;
                c.bus
                    .write32(crate::difftest::SP - 0x80 + 0x20, 0x789A_0000 + seed);
            },
        );
    }

    #[test]
    fn proto_ping_create_matches() {
        check(
            ee::s14::f_003e7340__generated,
            proto_ping_create,
            24,
            &|c, seed| {
                machine(c, seed, 0x3e7340, 0);
                link_arena(c);
                c.pc = 0x003e_7340;
            },
        );
    }

    // -----------------------------------------------------------------------
    // The lobby fixtures: one `LobbyApi` object, the game's image, and gated
    // fakes for the heap and the tick.

    // Fixture regions (clear of the logging fake's pointers at 0x800000-
    // 0x83fff0 below and its call log at 0x900000 above).
    pub(crate) const LOBBY: u32 = DATA + 0x41_000;

    pub(crate) const LOBBY_BUF: u32 = DATA + 0x42_000;

    pub(crate) const LOBBY_NODE: u32 = DATA + 0x43_000;

    pub(crate) const LOBBY_REC: u32 = DATA + 0x44_000;

    pub(crate) const LOBBY_TAB: u32 = DATA + 0x45_000;

    pub(crate) const LOBBY_OUT: u32 = DATA + 0x46_000;

    pub(crate) const LOBBY_STR: u32 = DATA + 0x47_000;

    /// The `LobbyApi` object's size in words (the fixtures clear it).
    pub(crate) const LOBBY_WORDS: u32 = 0x180;

    /// Control block of the fakes: `+0` the gate ([`LOBBY_ALLOC_MAGIC`]),
    /// `+4` fail (non-zero: every allocation returns NULL), `+8` the
    /// allocator mode ([`LOBBY_LOGGED`] or [`LOBBY_BUMP`]), `+0xc` the bump
    /// offset, `+0x10`/`+0x14` the last size and return address, `+0x20` the
    /// last freed block, `+0x18` the tick gate ([`LOBBY_TICK_MAGIC`]) and
    /// `+0x1c` the tick it returns.
    pub(crate) const LOBBY_ALLOC_CTRL: u32 = DATA + 0x40_000;

    pub(crate) const LOBBY_ALLOC_MAGIC: u32 = 0x10B1_0C01;

    pub(crate) const LOBBY_ALLOC_FAIL: u32 = LOBBY_ALLOC_CTRL + 4;

    const LOBBY_ALLOC_MODE: u32 = LOBBY_ALLOC_CTRL + 8;

    const LOBBY_BUMP_OFF: u32 = LOBBY_ALLOC_CTRL + 0xc;

    const LOBBY_TICK_GATE: u32 = LOBBY_ALLOC_CTRL + 0x18;

    const LOBBY_TICK_VALUE: u32 = LOBBY_ALLOC_CTRL + 0x1c;

    /// Allocator mode: the logging fake (`fakes::fake`: a pointer from a
    /// hash or the forced result, unwinding at the chosen call).
    pub(crate) const LOBBY_LOGGED: u32 = 0;

    /// Allocator mode: consecutive 16-aligned blocks from [`LOBBY_POOL`], so
    /// the lists and hashes the code builds are real memory.
    pub(crate) const LOBBY_BUMP: u32 = 1;

    /// The bump allocator's pool.
    pub(crate) const LOBBY_POOL: u32 = DATA + 0xe_0000;

    pub(crate) const LOBBY_TICK_MAGIC: u32 = 0x71_cc10;

    /// `NetTick`'s call of `__udivdi3` (its return address): the only divide
    /// the tick fake answers.
    const NET_TICK_DIVIDE_RET: u32 = 0x003f_4e68;

    /// Event setup that never unwinds, for structured ports: they cannot be
    /// re-entered mid-call, so an Unwind seed whose event falls due inside a
    /// dispatcher-shaped callee would stop the drive instead of testing
    /// anything. Unwind seeds become Deliver seeds (delivery and
    /// rescheduling still run every head check on both sides).
    pub(crate) fn schedule_no_unwind(c: &mut Cpu, r: &mut Rng, seed: u32, entry: u32) -> Events {
        let mode = schedule(c, r, seed, entry);
        if mode == Events::Unwind {
            ssx3_ps2::platform::machine_of(&mut c.bus.plat)
                .sched
                .next_vblank = 1 << 40;
            return Events::Deliver;
        }
        mode
    }

    thread_local! {
        /// Re-entries of a hand body at each resume head ([`counted!`]);
        /// `drive`'s own count pools both sides.
        static HAND_RESUMES: std::cell::RefCell<std::collections::BTreeMap<u32, u32>> =
            const { std::cell::RefCell::new(std::collections::BTreeMap::new()) };
    }

    /// Count a hand body's re-entries: every call with `pc` other than the
    /// function's entry is a resume at a loop head.
    pub(crate) fn count_resume(c: &Cpu, entry: u32) {
        if c.pc != entry {
            HAND_RESUMES.with(|r| *r.borrow_mut().entry(c.pc).or_default() += 1);
        }
    }

    /// A hand body wrapped in [`count_resume`].
    macro_rules! counted {
        ($name:ident, $f:path, $entry:expr) => {
            fn $name(c: &mut Cpu) {
                count_resume(c, $entry);
                $f(c);
            }
        };
    }

    /// `check_events`, then require that the hand side itself was re-entered
    /// at each of `heads` (pass a [`counted!`] hand).
    pub(crate) fn check_resumes(
        name: &str,
        generated: GuestFn,
        hand: GuestFn,
        seeds: u32,
        heads: &[u32],
        setup: &dyn Fn(&mut Cpu, u32) -> Events,
    ) {
        HAND_RESUMES.with(|r| r.borrow_mut().clear());
        check_events(generated, hand, seeds, setup);
        let got = HAND_RESUMES.with(|r| std::mem::take(&mut *r.borrow_mut()));
        for h in heads {
            assert!(
                got.get(h).copied().unwrap_or(0) > 0,
                "{name}: the hand side never resumed at {h:#x} ({got:x?})"
            );
        }
    }

    pub(crate) static LOBBY_PREV_NEW: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static LOBBY_PREV_FREE: AtomicUsize = AtomicUsize::new(0);

    pub(crate) static LOBBY_PREV_DIVIDE: AtomicUsize = AtomicUsize::new(0);

    /// `operator_new_named` (0x317e30, under `Lobby_MemAlloc`) for the lobby
    /// tests, behind the gate: the logging fake or the bump pool (the mode
    /// word), NULL when the fail word is set.
    pub(crate) fn lobby_fake_new(c: &mut Cpu) {
        if c.bus.read32(LOBBY_ALLOC_CTRL) != LOBBY_ALLOC_MAGIC {
            return fakes::fall_back(c, &LOBBY_PREV_NEW, ee::s10::f_00317e30__generated);
        }
        let fail = c.bus.read32(LOBBY_ALLOC_FAIL) != 0;
        if c.bus.read32(LOBBY_ALLOC_MODE) == LOBBY_LOGGED {
            let ra = c.g[RA][0] as u32;
            fakes::fake(c, 0x0031_7e30, ra);
            if c.pc == ra && fail {
                c.g[V0][0] = 0;
            }
            return;
        }
        c.bus.write32(LOBBY_ALLOC_CTRL + 0x10, c.g[A0][0] as u32);
        c.bus.write32(LOBBY_ALLOC_CTRL + 0x14, c.g[RA][0] as u32);
        if fail {
            c.g[V0][0] = 0;
        } else {
            let off = c.bus.read32(LOBBY_BUMP_OFF);
            let size = (c.g[A0][0] as u32 + 15) & !15;
            c.bus.write32(LOBBY_BUMP_OFF, off + size);
            c.g[V0][0] = (LOBBY_POOL + off) as u64;
        }
        c.cycles += 11;
        ret(c);
    }

    /// `EA_MemFree` (0x317e98, under `Lobby_MemFree`) for the lobby tests,
    /// behind the gate: the logging fake, or (bump mode) a logged no-op, the
    /// freed blocks being fixture memory.
    pub(crate) fn lobby_fake_free(c: &mut Cpu) {
        if c.bus.read32(LOBBY_ALLOC_CTRL) != LOBBY_ALLOC_MAGIC {
            return fakes::fall_back(c, &LOBBY_PREV_FREE, ee::s10::f_00317e98__generated);
        }
        if c.bus.read32(LOBBY_ALLOC_MODE) == LOBBY_LOGGED {
            let ra = c.g[RA][0] as u32;
            fakes::fake(c, 0x0031_7e98, ra);
            return;
        }
        c.bus.write32(LOBBY_ALLOC_CTRL + 0x20, c.g[A0][0] as u32);
        c.g[V0][0] = 0;
        c.g[V1][0] = 0;
        c.cycles += 5;
        ret(c);
    }

    /// `__udivdi3` (0x411538) for the lobby tests: `NetTick` turns the timer
    /// count into milliseconds with this divide, so with the tick gate on,
    /// that one call (and no other divide) returns the fixture's tick.
    pub(crate) fn lobby_fake_divide(c: &mut Cpu) {
        if c.bus.read32(LOBBY_TICK_GATE) != LOBBY_TICK_MAGIC
            || c.g[RA][0] as u32 != NET_TICK_DIVIDE_RET
        {
            return fakes::fall_back(c, &LOBBY_PREV_DIVIDE, ee::s15::f_00411538__generated);
        }
        c.g[V0][0] = c.bus.read32(LOBBY_TICK_VALUE) as u64;
        c.cycles += 7;
        ret(c);
    }

    pub(crate) fn lobby_install_fakes() {
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E30),
            lobby_fake_new,
            &LOBBY_PREV_NEW,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E98),
            lobby_fake_free,
            &LOBBY_PREV_FREE,
        );
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_00411538),
            lobby_fake_divide,
            &LOBBY_PREV_DIVIDE,
        );
    }

    /// Scrambled machine with the game's image and the gated fakes (logging
    /// allocator, no forced failure, tick gate off).
    pub(crate) fn lobby_base(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        let r = machine(c, seed, salt, calls);
        elf_image(c);
        lobby_install_fakes();
        c.bus.write32(LOBBY_ALLOC_CTRL, LOBBY_ALLOC_MAGIC);
        c.bus.write32(LOBBY_ALLOC_FAIL, 0);
        c.bus.write32(LOBBY_ALLOC_MODE, LOBBY_LOGGED);
        c.bus.write32(LOBBY_TICK_GATE, 0);
        r
    }

    /// Bump allocation from [`LOBBY_POOL`] (`words` of it cleared), failing
    /// every call when `fail`.
    pub(crate) fn lobby_bump(c: &mut Cpu, words: u32, fail: bool) {
        c.bus.write32(LOBBY_ALLOC_MODE, LOBBY_BUMP);
        c.bus.write32(LOBBY_ALLOC_FAIL, fail as u32);
        c.bus.write32(LOBBY_BUMP_OFF, 0);
        lobby_zero(c, LOBBY_POOL, words);
    }

    /// `NetTick` returns `tick`.
    pub(crate) fn lobby_tick(c: &mut Cpu, tick: u32) {
        c.bus.write32(LOBBY_TICK_GATE, LOBBY_TICK_MAGIC);
        c.bus.write32(LOBBY_TICK_VALUE, tick);
    }

    /// A cleared `LobbyApi` object at [`LOBBY`], in `a0`.
    pub(crate) fn lobby_object(c: &mut Cpu) {
        lobby_zero(c, LOBBY, LOBBY_WORDS);
        c.g[A0][0] = sx(LOBBY);
    }

    pub(crate) fn lobby_zero(c: &mut Cpu, at: u32, words: u32) {
        for i in 0..words {
            c.bus.write32(at + 4 * i, 0);
        }
    }

    /// Non-zero filler (output buffers must show a missing store).
    pub(crate) fn lobby_fill_nonzero(c: &mut Cpu, seed: u32, at: u32, words: u32) {
        for i in 0..words {
            let w = 0xA5A5_0001u32
                .wrapping_add((seed + 1).wrapping_mul(0x0101_0101))
                .wrapping_add(i.wrapping_mul(0x0011_0011));
            c.bus.write32(at + 4 * i, w);
        }
    }

    /// Write `s` and a terminating NUL at `at`.
    pub(crate) fn lobby_str(c: &mut Cpu, at: u32, s: &[u8]) {
        for (i, b) in s.iter().enumerate() {
            c.bus.write8(at + i as u32, *b);
        }
        c.bus.write8(at + s.len() as u32, 0);
    }

    /// Tag keys in the game image the ports look up: the reply's address and
    /// name (`ProcessPings`), the login record's password and the `'snap'`
    /// channel (`Send`), the server ping's reply number (`ServerPingCallback`).
    pub(crate) const REPLY_ADDRESS_KEY: u32 = 0x004a_4878;

    pub(crate) const REPLY_NAME_KEY: u32 = 0x004a_4880;

    pub(crate) const LOGIN_PASSWORD_KEY: u32 = 0x004a_48d8;

    pub(crate) const SNAP_CHANNEL_KEY: u32 = 0x004a_48e8;

    pub(crate) const SERVER_PING_KEY: u32 = 0x004a_4898;

    /// The NUL-terminated string at `at` in the loaded image (a tag key the
    /// code looks up, so a fixture can build a record that has it).
    pub(crate) fn image_str(c: &mut Cpu, at: u32) -> Vec<u8> {
        (at..)
            .map(|a| c.bus.read8(a))
            .take_while(|&b| b != 0)
            .collect()
    }

    /// A tag record `KEY=value` (`KEY` read from the image at `key`).
    pub(crate) fn lobby_tag(c: &mut Cpu, at: u32, key: u32, value: &[u8]) {
        let mut s = image_str(c, key);
        s.push(b'=');
        s.extend_from_slice(value);
        lobby_str(c, at, &s);
    }

    /// Run a guest function in a fixture (for building valid records and
    /// reading back what the test then seeds): `f.f` entered at `f.addr`
    /// (a dispatcher-shaped body starts from `pc`).
    pub(crate) fn lobby_call(
        c: &mut Cpu,
        f: crate::call::Callee,
        a0: u64,
        a1: u64,
        a2: u64,
        a3: u64,
    ) -> u64 {
        let f = {
            c.pc = f.addr;
            f.f
        };
        c.g[A0][0] = a0;
        c.g[A1][0] = a1;
        c.g[A2][0] = a2;
        c.g[A3][0] = a3;
        c.g[RA][0] = TEST_RA as u64;
        f(c);
        assert_eq!(c.pc, TEST_RA, "setup call did not return");
        c.g[V0][0]
    }

    /// A `Lobby_List` at `a` with `n` entries: `+0x14` visible count,
    /// `+0x18` count, `+0x40` entry array, `+0x38` grow flag, `+0x24`
    /// callback guard (0 skips the sort callback).
    pub(crate) fn make_list(c: &mut Cpu, a: u32, n: u32, array: u32) {
        c.bus.write32(a + 0x8, 0);
        c.bus.write32(a + 0xc, 0);
        c.bus.write32(a + 0x10, 0);
        c.bus.write32(a + 0x14, n);
        c.bus.write32(a + 0x18, n);
        c.bus.write32(a + 0x24, 0);
        c.bus.write32(a + 0x38, 1);
        c.bus.write32(a + 0x3c, 0);
        c.bus.write32(a + 0x40, array);
    }

    /// A `Lobby_Hash` at `a` with one bucket holding `head`: `+0x10`
    /// bucket count 1, `+0x14` buckets, `+0x20` `strcmp` compare.
    pub(crate) fn make_hash(c: &mut Cpu, a: u32, buckets: u32, head: u32, count: u32, free: u32) {
        c.bus.write32(a, count);
        c.bus.write32(a + 4, 0);
        c.bus.write32(a + 0x10, 1);
        c.bus.write32(a + 0x14, buckets);
        c.bus.write32(a + 0x18, 0);
        c.bus.write32(a + 0x1c, free);
        c.bus.write32(a + 0x20, 0x0041_65a8);
        c.bus.write32(buckets, head);
    }

    #[test]
    fn proto_ping_destroy_matches() {
        check(
            ee::s14::f_003e73e0__generated,
            proto_ping_destroy,
            32,
            &|c, seed| {
                let _r = lobby_base(c, seed, 0x3e73_e0, 2);
                lobby_zero(c, LOBBY, 4);
                c.g[A0][0] = LOBBY as u64;
                c.bus.write32(LOBBY + 0xc, 0x1000 + seed);
            },
        );
    }

    #[test]
    fn proto_ping_request_matches() {
        check(
            ee::s14::f_003e7450__generated,
            proto_ping_request,
            32,
            &|c, seed| {
                let _r = lobby_base(c, seed, 0x3e74_50, 2);
                lobby_zero(c, LOBBY, 4);
                c.g[A0][0] = LOBBY as u64;
                c.g[A1][0] = (0x100 + seed % 5) as u64;
                c.g[A2][0] = (0x200 + seed / 5 % 7) as u64;
                c.g[A3][0] = (seed / 35) as u64;
                c.bus.write32(LOBBY + 0xc, 0x2000 + seed);
                // The reply word slot on the frame: pre-filled non-zero so
                // a missing load changes RAM.
                c.bus
                    .write32(crate::difftest::SP - 0x80 + 0x20, 0x89AB_0000 + seed);
            },
        );
    }

    // Seed digits: count (% 3), flag (/ 3 % 2), re-arm tick (/ 6 % 4: never,
    // 3 or exactly 6 ticks ago, long ago), `lenp` (/ 24 % 3), `out`
    // (/ 72 % 2). Totals: flagged 17; a reply taken with
    // the flag set 28/44 (no `lenp`, without/with `out`), 75/91 (`lenp`
    // kept), 79/95 (`lenp` capped); re-armed after a reply 1883..1950 (the
    // same six, plus the cache invalidate); no reply: re-arm too recent 147,
    // re-arm 236 (never armed) and 360 (stale), with an empty buffer 1872
    // and 1996 (plus the invalidate).
    #[test]
    fn proto_ping_response_matches() {
        check_paths(
            ee::s14::f_003e7500__generated,
            proto_ping_response,
            144,
            &[
                17, 28, 44, 75, 79, 91, 95, 147, 236, 360, 1872, 1883, 1899, 1930, 1934, 1946,
                1950, 1996,
            ],
            &|c, seed| {
                let _r = lobby_base(c, seed, 0x3e75_00, 2);
                lobby_zero(c, LOBBY, 8);
                // count: negative, zero, positive (positive stays >= 0x10:
                // smaller counts make the copy length negative and run away,
                // in the original too).
                let count = match seed % 3 {
                    0 => 0xFFFF_FFFC,
                    1 => 0,
                    _ => 0x14,
                };
                c.bus.write32(LOBBY + 0, count);
                let flag = seed / 3 % 2;
                c.bus.write32(LOBBY + 4, flag);
                // +8: quiet (send path), fresh (early return), stale (send).
                if flag == 0 && count != 0x14 {
                    let tick = lobby_call(c, callee::NET_TICK, 0, 0, 0, 0);
                    let v = match seed / 6 % 4 {
                        0 => 0,
                        1 => (tick as u32).wrapping_sub(3),
                        2 => (tick as u32).wrapping_sub(6),
                        _ => (tick as u32).wrapping_sub(100),
                    };
                    c.bus.write32(LOBBY + 8, v);
                } else {
                    c.bus.write32(LOBBY + 8, 0x5555_0000 + seed);
                }
                // Data record: non-zero reply word first.
                lobby_fill_nonzero(c, seed, LOBBY_REC, 64);
                c.bus.write32(LOBBY_REC, 0x2468_0001 + (seed & 0xffff));
                c.bus.write32(LOBBY + 0x10, LOBBY_REC);
                c.bus.write32(LOBBY + 0xc, 0x3000 + seed);
                lobby_fill_nonzero(c, seed + 1, LOBBY_OUT, 64);
                c.g[A0][0] = LOBBY as u64;
                c.g[A1][0] = LOBBY_OUT as u64;
                // `lenp`: none, at the queued length (kept), or above it
                // (capped).
                match seed / 24 % 3 {
                    0 => c.g[A2][0] = 0,
                    k => {
                        c.g[A2][0] = (LOBBY_OUT + 0x100) as u64;
                        c.bus
                            .write32(LOBBY_OUT + 0x100, [4, 0x1000][k as usize - 1]);
                    }
                }
                if seed / 72 % 2 == 0 {
                    c.g[A3][0] = 0;
                } else {
                    c.g[A3][0] = (LOBBY_OUT + 0x200) as u64;
                }
            },
        );
    }

    #[test]
    fn lobby_api_debug_printf_matches() {
        check(
            ee::s14::f_003e76a8__generated,
            lobby_api_debug_printf,
            40,
            &|c, seed| {
                let _r = lobby_base(c, seed, 0x3e76_a8, 1);
                lobby_zero(c, LOBBY, 4);
                c.g[A0][0] = LOBBY as u64;
                match seed % 3 {
                    0 => {
                        lobby_str(c, LOBBY_STR, b"v=%d x=%x");
                        c.g[A1][0] = LOBBY_STR as u64;
                        c.g[A2][0] = (0x1100 + seed) as u64;
                        c.g[A3][0] = (0x2200 + seed * 3) as u64;
                    }
                    1 => {
                        lobby_str(c, LOBBY_STR, b"s=%s!");
                        lobby_str(c, LOBBY_STR + 0x40, b"hi");
                        c.g[A1][0] = LOBBY_STR as u64;
                        c.g[A2][0] = (LOBBY_STR + 0x40) as u64;
                        c.g[A3][0] = (0x3300 + seed) as u64;
                    }
                    _ => {
                        lobby_str(c, LOBBY_STR, b"m=%d%d%d");
                        c.g[A1][0] = LOBBY_STR as u64;
                        c.g[A2][0] = (0x4400 + seed) as u64;
                        c.g[A3][0] = (0x5500 + seed * 5) as u64;
                    }
                }
                c.g[T0][0] = seed as u64;
                c.g[T1][0] = (seed * 7) as u64;
                // Callback: none vs the miss-hook fake.
                if seed / 3 % 2 == 0 {
                    c.bus.write32(LOBBY + LOBBY_API_DEBUG_CB, 0);
                } else {
                    c.bus.write32(LOBBY + LOBBY_API_DEBUG_CB, FAKE);
                }
                c.bus.write32(LOBBY + LOBBY_API_DEBUG_ARG, 0x6600 + seed);
            },
        );
    }

    /// Build a request chain: tags and links for `count` nodes.
    pub(crate) fn lobby_req_chain(c: &mut Cpu, tags: &[u32]) {
        lobby_zero(c, LOBBY_NODE, 0x40);
        for (i, &tag) in tags.iter().enumerate() {
            let n = LOBBY_NODE + 0x44 * i as u32;
            c.bus.write32(n + LOBBY_REQ_TAG, tag);
            c.bus.write32(n + LOBBY_REQ_ARG2, 0x400 + i as u32);
            c.bus.write32(n + LOBBY_REQ_ARG3, 0x500 + i as u32);
            let next = if i + 1 < tags.len() {
                LOBBY_NODE + 0x44 * (i + 1) as u32
            } else {
                0
            };
            c.bus.write32(n + LOBBY_REQ_NEXT, next);
        }
    }

    #[test]
    fn lobby_api_add_request_matches() {
        check(
            ee::s14::f_003e7718__generated,
            lobby_api_add_request,
            48,
            &|c, seed| {
                let _r = lobby_base(c, seed, 0x3e77_18, 0);
                lobby_zero(c, LOBBY, 0x160);
                c.bus.write32(LOBBY + LOBBY_API_REQ_ID, seed);
                // Queue shape: fail, empty, ping head, end, match, long.
                match seed % 7 {
                    0 => {
                        c.bus.write32(LOBBY_ALLOC_FAIL, 1);
                    }
                    1 => {}
                    2 => {
                        lobby_req_chain(c, &[0x7069_6e67]);
                        c.bus.write32(LOBBY + LOBBY_API_REQUESTS, LOBBY_NODE);
                    }
                    3 => {
                        lobby_req_chain(c, &[0x7878_7878]);
                        c.bus.write32(LOBBY + LOBBY_API_REQUESTS, LOBBY_NODE);
                    }
                    4 => {
                        lobby_req_chain(c, &[0x7878_7878, 0x7069_6e67]);
                        c.bus.write32(LOBBY + LOBBY_API_REQUESTS, LOBBY_NODE);
                    }
                    5 => {
                        lobby_req_chain(c, &[0x6161_6161, 0x6262_6262, 0x6363_6363]);
                        c.bus.write32(LOBBY + LOBBY_API_REQUESTS, LOBBY_NODE);
                    }
                    _ => {
                        lobby_req_chain(c, &[0x7878_7878, 0x7979_7979]);
                        c.bus.write32(LOBBY + LOBBY_API_REQUESTS, LOBBY_NODE);
                    }
                }
                c.g[A0][0] = LOBBY as u64;
                c.g[A1][0] = match seed / 7 % 3 {
                    0 => 0x7069_6e67,
                    1 => 0x4451_5545,
                    _ => 0x5a5a_5a5a,
                } as u64;
                c.g[A2][0] = (0x400 + seed) as u64;
                c.g[A3][0] = (0x500 + seed * 2) as u64;
            },
        );
    }

    #[test]
    fn lobby_api_add_request_events() {
        driven!(gen_add_request, ee::s14::f_003e7718__generated);
        driven!(hand_add_request, lobby_api_add_request);
        check_events(gen_add_request, hand_add_request, 16, &|c, seed| {
            let mut r = lobby_base(c, seed, 0x3e77_19, 0);
            lobby_zero(c, LOBBY, 0x160);
            // Always a two-node non-ping chain so the walk iterates.
            lobby_req_chain(c, &[0x6161_6161, 0x6262_6262]);
            c.bus.write32(LOBBY + LOBBY_API_REQUESTS, LOBBY_NODE);
            c.bus.write32(LOBBY + LOBBY_API_REQ_ID, seed);
            c.g[A0][0] = LOBBY as u64;
            c.g[A1][0] = 0x5a5a_5a5a;
            c.g[A2][0] = seed as u64;
            c.g[A3][0] = (seed * 2) as u64;
            schedule(c, &mut r, seed, 0x003e_7718)
        });
    }

    #[test]
    fn lobby_api_pop_request_matches() {
        check(
            ee::s14::f_003e77f0__generated,
            lobby_api_pop_request,
            48,
            &|c, seed| {
                let _r = lobby_base(c, seed, 0x3e77_f0, 0);
                lobby_zero(c, LOBBY, 0x160);
                lobby_fill_nonzero(c, seed, LOBBY_OUT, 20);
                // Head: empty, `'ping'` (matches the `'ping'` wanted below),
                // `'DQUE'` (matches any), another tag.
                let tag = match seed % 4 {
                    1 => 0x7069_6e67,
                    2 => 0x4451_5545,
                    3 => 0x1111_1111,
                    _ => 0,
                };
                if seed % 4 != 0 {
                    lobby_req_chain(c, &[tag]);
                    c.bus.write32(LOBBY + LOBBY_API_REQUESTS, LOBBY_NODE);
                    if seed / 24 % 2 == 1 {
                        // Trailer so something remains.
                        c.bus
                            .write32(LOBBY_NODE + LOBBY_REQ_NEXT, LOBBY_NODE + 0x44);
                        c.bus
                            .write32(LOBBY_NODE + 0x44 + LOBBY_REQ_TAG, 0x4451_5545);
                    }
                }
                c.g[A0][0] = LOBBY as u64;
                // Output alignment steers the aligned/unaligned copy loops.
                c.g[A1][0] = if seed / 4 % 3 == 2 {
                    (LOBBY_OUT + 4) as u64
                } else {
                    LOBBY_OUT as u64
                };
                c.g[A2][0] = match seed / 12 % 2 {
                    0 => 0x4451_5545,
                    _ => 0x7069_6e67,
                } as u64;
            },
        );
    }

    /// `LobbyApi_PopRequest` for the events test: one request popped into
    /// an aligned or unaligned `out` (its copy loop 0x3e78bc or 0x3e7864),
    /// events delivered, never unwinding, `k` cycles in.
    pub(crate) fn lobby_pop_request_events_setup(
        c: &mut Cpu,
        seed: u32,
        aligned: bool,
        k: u64,
    ) -> Events {
        let mut r = lobby_base(c, 1, 0x3e77_f1, 0);
        lobby_object(c);
        lobby_fill_nonzero(c, 1, LOBBY_OUT, 20);
        lobby_req_chain(c, &[0x4451_5545]);
        c.bus.write32(LOBBY + LOBBY_API_REQUESTS, LOBBY_NODE);
        let mode = schedule_no_unwind(c, &mut r, seed, 0x003e_77f0);
        c.g[A0][0] = sx(LOBBY);
        c.g[A1][0] = sx(if aligned { LOBBY_OUT } else { LOBBY_OUT + 4 });
        c.g[A2][0] = sx(0x4451_5545);
        if mode != Events::Off {
            c.next_event = c.cycles + k;
        }
        mode
    }

    /// The event falls due on a copy loop's own back edge, past the leading
    /// `memset` (which would otherwise take it): delays 80-110 aligned,
    /// 498-536 unaligned, as measured.
    #[test]
    fn lobby_api_pop_request_events() {
        driven!(gen_pop_request, ee::s14::f_003e77f0__generated);
        driven!(hand_pop_request, lobby_api_pop_request);
        check_events(gen_pop_request, hand_pop_request, 24, &|c, seed| {
            let aligned = seed / 3 % 2 == 0;
            let k = if aligned { 80 } else { 500 } + (seed / 6 % 4 * 10) as u64;
            lobby_pop_request_events_setup(c, seed, aligned, k)
        });
    }

    /// Build ping-request nodes: (tag, id, gen, name) each 0x44 bytes.
    pub(crate) fn lobby_ping_chain(c: &mut Cpu, nodes: &[(u32, u32, u32, &[u8])]) {
        lobby_zero(c, LOBBY_NODE, 0x40);
        for (i, &(tag, addr, deadline, name)) in nodes.iter().enumerate() {
            let n = LOBBY_NODE + 0x44 * i as u32;
            c.bus.write32(n + LOBBY_REQ_TAG, tag);
            c.bus.write32(n + LOBBY_PINGREQ_ADDR, addr);
            c.bus.write32(n + LOBBY_PINGREQ_DEADLINE, deadline);
            lobby_str(c, n + LOBBY_PINGREQ_NAME, name);
            let next = if i + 1 < nodes.len() {
                LOBBY_NODE + 0x44 * (i + 1) as u32
            } else {
                0
            };
            c.bus.write32(n + LOBBY_REQ_NEXT, next);
        }
    }

    #[test]
    fn lobby_api_pop_ping_request_matches() {
        check_paths(
            ee::s14::f_003e7918__generated,
            lobby_api_pop_ping_request,
            84,
            &[
                32, 42, 47, 52, 59, 85, 87, 102, 104, 114, 115, 118, 119, 129, 135, 147, 181, 194,
                196,
            ],
            &|c, seed| {
                let _r = lobby_base(c, seed, 0x3e79_18, 1);
                lobby_zero(c, LOBBY, 0x160);
                lobby_fill_nonzero(c, seed, LOBBY_OUT, 20);
                lobby_str(c, LOBBY_STR, b"pn");
                lobby_str(c, LOBBY_STR + 0x20, b"qx");
                // Queue shapes cover every match arm.
                match seed % 7 {
                    0 => {}
                    1 => {
                        lobby_ping_chain(c, &[(0x7878_7878, 1, 0, b"pn")]);
                        c.bus.write32(LOBBY + LOBBY_API_REQUESTS, LOBBY_NODE);
                    }
                    2 => {
                        lobby_ping_chain(
                            c,
                            &[(0x7069_6e67, 9, 0, b"pn"), (0x7069_6e67, 9, 0, b"pn")],
                        );
                        c.bus.write32(LOBBY + LOBBY_API_REQUESTS, LOBBY_NODE);
                    }
                    3 => {
                        lobby_ping_chain(
                            c,
                            &[(0x7069_6e67, 7, 0, b"qx"), (0x7069_6e67, 7, 0, b"pn")],
                        );
                        c.bus.write32(LOBBY + LOBBY_API_REQUESTS, LOBBY_NODE);
                    }
                    4 => {
                        lobby_ping_chain(c, &[(0x7069_6e67, 7, 0, b"pn")]);
                        c.bus.write32(LOBBY + LOBBY_API_REQUESTS, LOBBY_NODE);
                    }
                    5 => {
                        lobby_ping_chain(
                            c,
                            // Deadlines exactly at the tick (10, kept) and
                            // before it (5, popped).
                            &[(0x7069_6e67, 3, 10, b"zz"), (0x7069_6e67, 3, 5, b"zz")],
                        );
                        c.bus.write32(LOBBY + LOBBY_API_REQUESTS, LOBBY_NODE);
                    }
                    _ => {
                        lobby_ping_chain(c, &[(0x7069_6e67, 3, 50, b"zz")]);
                        c.bus.write32(LOBBY + LOBBY_API_REQUESTS, LOBBY_NODE);
                    }
                }
                c.bus.write32(LOBBY + LOBBY_API_PING_COUNT, 3);
                c.g[A0][0] = LOBBY as u64;
                c.g[A1][0] = if seed / 7 % 3 == 2 {
                    (LOBBY_OUT + 4) as u64
                } else {
                    LOBBY_OUT as u64
                };
                c.g[A2][0] = LOBBY_STR as u64;
                // id: match the node, mismatch, or -1 with old/new gen.
                c.g[A3][0] = match seed / 21 % 4 {
                    0 => 7,
                    1 => 8,
                    2 => -1,
                    _ => 3,
                } as u64;
                c.g[T0][0] = 10;
            },
        );
    }

    #[test]
    fn lobby_api_pop_ping_request_events() {
        driven!(gen_pop_ping, ee::s14::f_003e7918__generated);
        driven!(hand_pop_ping, lobby_api_pop_ping_request);
        check_events(gen_pop_ping, hand_pop_ping, 16, &|c, seed| {
            let mut r = lobby_base(c, seed, 0x3e79_19, 0);
            lobby_zero(c, LOBBY, 0x160);
            lobby_fill_nonzero(c, seed, LOBBY_OUT, 20);
            lobby_str(c, LOBBY_STR, b"pn");
            // Two matching nodes so the walk loop iterates.
            lobby_ping_chain(c, &[(0x7069_6e67, 7, 0, b"xx"), (0x7069_6e67, 7, 0, b"pn")]);
            c.bus.write32(LOBBY + LOBBY_API_REQUESTS, LOBBY_NODE);
            c.bus.write32(LOBBY + LOBBY_API_PING_COUNT, 3);
            c.g[A0][0] = LOBBY as u64;
            c.g[A1][0] = LOBBY_OUT as u64;
            c.g[A2][0] = LOBBY_STR as u64;
            c.g[A3][0] = 7;
            c.g[T0][0] = 10;
            schedule_no_unwind(c, &mut r, seed, 0x003e_7918)
        });
    }

    #[test]
    fn lobby_api_flush_requests_matches() {
        check(
            ee::s14::f_003e7ac0__generated,
            lobby_api_flush_requests,
            24,
            &|c, seed| {
                let _r = lobby_base(c, seed, 0x3e7a_c0, 0);
                lobby_zero(c, LOBBY, 0x160);
                match seed % 3 {
                    0 => {}
                    1 => {
                        lobby_req_chain(c, &[0x4451_5545]);
                    }
                    _ => {
                        lobby_req_chain(c, &[0x4451_5545, 0x4451_5545]);
                    }
                }
                if seed % 3 != 0 {
                    c.bus.write32(LOBBY + LOBBY_API_REQUESTS, LOBBY_NODE);
                }
                c.g[A0][0] = LOBBY as u64;
                c.pc = 0x003e_7ac0;
            },
        );
    }

    /// `LobbyApi_FlushRequests` for the events test: `n` queued requests,
    /// events delivered, never unwinding, `k` cycles in.
    pub(crate) fn lobby_flush_requests_events_setup(
        c: &mut Cpu,
        seed: u32,
        n: usize,
        k: u64,
    ) -> Events {
        let mut r = lobby_base(c, 1, 0x3e7a_c1, 0);
        lobby_object(c);
        lobby_req_chain(c, &[0x4451_5545, 0x4451_5545][..n]);
        c.bus.write32(LOBBY + LOBBY_API_REQUESTS, LOBBY_NODE);
        let mode = schedule_no_unwind(c, &mut r, seed, 0x003e_7ac0);
        c.g[A0][0] = sx(LOBBY);
        if mode != Events::Off {
            c.next_event = c.cycles + k;
        }
        mode
    }

    /// The event falls due on the flush loop's own back edge 0x3e7ad8, past
    /// `PopRequest`'s `memset` (delays 123-162, as measured).
    #[test]
    fn lobby_api_flush_requests_events() {
        driven!(gen_flush, ee::s14::f_003e7ac0__generated);
        driven!(hand_flush, lobby_api_flush_requests);
        check_events(gen_flush, hand_flush, 24, &|c, seed| {
            lobby_flush_requests_events_setup(c, seed, 2, 125 + (seed / 3 % 4 * 12) as u64)
        });
    }

    /// Hand-built int hash: one bucket pointing at one entry.
    pub(crate) fn lobby_hash_one(c: &mut Cpu, h: u32, t: u32, e: u32, key: u32, val: u32) {
        lobby_zero(c, h, 8);
        c.bus.write32(h + 0x10, 1);
        c.bus.write32(h + 0x14, t);
        c.bus.write32(t, e);
        c.bus.write32(e, 0);
        c.bus.write32(e + 4, val);
        c.bus.write32(e + 8, key);
    }

    #[test]
    fn lobby_api_update_room_count_matches() {
        check(
            ee::s14::f_003e7b00__generated,
            lobby_api_update_room_count,
            40,
            &|c, seed| {
                let _r = lobby_base(c, seed, 0x3e7b_00, 0);
                lobby_zero(c, LOBBY, 0x160);
                lobby_fill_nonzero(c, seed, LOBBY_REC, 0x80);
                let key = 0x77 + seed;
                let count = seed % 256;
                // Table: none, missing entry, same count, new count, an
                // entry whose list node holds no record.
                match seed % 5 {
                    0 => {}
                    1 => {
                        let h = LOBBY_TAB;
                        let t = LOBBY_TAB + 0x40;
                        lobby_zero(c, h, 8);
                        c.bus.write32(h + 0x10, 1);
                        c.bus.write32(h + 0x14, t);
                        c.bus.write32(t, 0);
                        c.bus.write32(LOBBY + LOBBY_API_TABLES, h);
                        c.bus.write32(LOBBY + LOBBY_API_TABLES2, LOBBY_TAB + 0x80);
                    }
                    _ => {
                        let h = LOBBY_TAB;
                        let t = LOBBY_TAB + 0x40;
                        let e = LOBBY_TAB + 0x60;
                        let w = LOBBY_TAB + 0x80;
                        lobby_hash_one(c, h, t, e, key, w);
                        c.bus.write32(w, if seed % 5 == 4 { 0 } else { LOBBY_REC });
                        let same = seed % 5 == 2;
                        c.bus
                            .write32(LOBBY_REC + 0x10, if same { count } else { count + 1 });
                        c.bus.write32(LOBBY + LOBBY_API_TABLES, h);
                        c.bus.write32(LOBBY + LOBBY_API_TABLES2, LOBBY_TAB + 0xa0);
                    }
                }
                c.g[A0][0] = LOBBY as u64;
                c.g[A1][0] = key as u64;
                c.g[A2][0] = count as u64;
            },
        );
    }

    #[test]
    fn lobby_api_sort_lists_matches() {
        check(
            ee::s14::f_003e7ba0__generated,
            lobby_api_sort_lists,
            32,
            &|c, seed| {
                let _r = lobby_base(c, seed, 0x3e7b_a0, 0);
                lobby_zero(c, LOBBY, 0x160);
                // A clean list sorts to 0; a dirty one (seed / 4 % 2) to 1,
                // which calls the slot's notify callback (a fake).
                let lz = LOBBY_TAB + 0x100;
                lobby_zero(c, lz, 0x12);
                c.bus.write32(lz + 0xc, seed / 4 % 2);
                let h = LOBBY_TAB;
                lobby_zero(c, h, 4);
                c.bus.write32(h + 0x10, 1);
                // Hand slots steer the skip arms.
                match seed % 4 {
                    0 => {}
                    1 => {
                        let s = LOBBY + LOBBY_API_TABLES;
                        c.bus.write32(s, h);
                        c.bus.write32(s + 4, lz);
                        c.bus.write32(s + 8, FAKE + 0x20);
                    }
                    2 => {
                        for i in [0u32, 5] {
                            let s = LOBBY + LOBBY_API_TABLES + 0x14 * i;
                            c.bus.write32(s, h);
                            c.bus.write32(s + 4, lz);
                            c.bus.write32(s + 8, FAKE + 0x20);
                        }
                    }
                    _ => {
                        let s1 = LOBBY + LOBBY_API_TABLES + 0x14;
                        c.bus.write32(s1, 0);
                        c.bus.write32(s1 + 4, lz);
                        c.bus.write32(s1 + 8, FAKE + 0x20);
                        let s6 = LOBBY + LOBBY_API_TABLES + 0x14 * 6;
                        c.bus.write32(s6, h);
                        c.bus.write32(s6 + 4, 0);
                        c.bus.write32(s6 + 8, FAKE + 0x20);
                        let s7 = LOBBY + LOBBY_API_TABLES + 0x14 * 7;
                        c.bus.write32(s7, h);
                        c.bus.write32(s7 + 4, lz);
                        c.bus.write32(s7 + 8, 0);
                    }
                }
                c.g[A0][0] = LOBBY as u64;
            },
        );
    }

    #[test]
    fn lobby_api_sort_lists_events() {
        driven!(gen_sort, ee::s14::f_003e7ba0__generated);
        driven!(hand_sort, lobby_api_sort_lists);
        check_events(gen_sort, hand_sort, 16, &|c, seed| {
            let mut r = lobby_base(c, seed, 0x3e7b_a1, 0);
            lobby_zero(c, LOBBY, 0x160);
            let lz = LOBBY_TAB + 0x100;
            lobby_zero(c, lz, 0x12);
            let h = LOBBY_TAB;
            lobby_zero(c, h, 4);
            c.bus.write32(h + 0x10, 1);
            let s = LOBBY + LOBBY_API_TABLES + 0x14 * (seed % 8);
            c.bus.write32(s, h);
            c.bus.write32(s + 4, lz);
            c.bus.write32(s + 8, 1);
            c.g[A0][0] = LOBBY as u64;
            schedule(c, &mut r, seed, 0x003e_7ba0)
        });
    }

    /// A ping reply at `rec` as `ProtoPingResponse` hands it out: a 16-byte
    /// header (word 0 the non-zero result, word 1 the default address) then
    /// the tag text with the server's address (`ORIG` key) and name (`NAME`
    /// key, `pn`). Returns the address the code parses (the queue nodes and
    /// rooms must carry it to match).
    pub(crate) fn lobby_ping_record(c: &mut Cpu, rec: u32) -> u32 {
        lobby_fill_nonzero(c, 0x70c0, rec, 4);
        c.bus.write32(rec, 1);
        lobby_zero(c, rec + 0x10, 0x80);
        lobby_str(c, LOBBY_STR, b"10.1.2.3");
        for (key, value) in [
            (REPLY_ADDRESS_KEY as u64, LOBBY_STR),
            (REPLY_NAME_KEY as u64, LOBBY_STR + 0x20),
        ] {
            lobby_call(
                c,
                callee::TAG_FIELD_SET_STRING,
                (rec + 0x10) as u64,
                0x200,
                key,
                value as u64,
            );
        }
        let found = lobby_call(
            c,
            callee::TAG_FIELD_FIND,
            (rec + 0x10) as u64,
            REPLY_ADDRESS_KEY as u64,
            0,
            0,
        );
        lobby_call(c, callee::TAG_FIELD_GET_ADDRESS, found, 0, 0, 0) as u32
    }

    /// A list at `list` of `n` entries at `entry(i)` (`Lobby_ListGetAt`
    /// returns `*array[i]`, so the array holds nodes whose first word is the
    /// entry), with `visible` entries counted as visible.
    pub(crate) fn lobby_node_list(c: &mut Cpu, list: u32, entries: &[u32], visible: u32) {
        let (array, nodes) = (list + 0x80, list + 0xc0);
        make_list(c, list, entries.len() as u32, array);
        c.bus.write32(list + 0x14, visible);
        for (i, &e) in entries.iter().enumerate() {
            let node = nodes + 0x10 * i as u32;
            c.bus.write32(node, e);
            c.bus.write32(array + 4 * i as u32, node);
        }
    }

    /// `LobbyApi_ProcessPings`'s fixture. `reply`: the ping channel holds one
    /// reply for the server `sid` named `pn` (else it is quiet); `queue`: the
    /// request queue (0 empty, 1 the matching request, 2 a request for
    /// another name, 3 the matching request and one more that expires at the
    /// tick); `cb`: requests carry a callback (a fake); `rooms`: rooms in the
    /// slot-0 list (0 none, 1 one for `sid`, 2 one for another server, 3 both,
    /// the first visible); `users`: users in the slot-1 list (0 none, 1 `pn`,
    /// 2 another, 3 both). The frame is filled after `schedule`'s thread start
    /// (call [`lobby_pings_frame`]).
    pub(crate) fn lobby_pings_setup(
        c: &mut Cpu,
        seed: u32,
        salt: u64,
        reply: bool,
        queue: u32,
        cb: bool,
        rooms: u32,
        users: u32,
    ) -> Rng {
        let mut r = lobby_base(c, seed, salt, 0);
        lobby_object(c);
        lobby_str(c, LOBBY_STR + 0x20, b"pn");
        lobby_aries(c, &mut r, DATA + 0x8000);
        lobby_tick(c, 1000);
        let p = LOBBY_REC;
        lobby_zero(c, p, 8);
        c.bus.write32(LOBBY + LOBBY_API_PING_CHAN, p);
        let sid = if reply {
            c.bus.write32(p, 0x90);
            c.bus.write32(p + 4, 1);
            c.bus.write32(p + 0x10, LOBBY_REC + 0x100);
            lobby_ping_record(c, LOBBY_REC + 0x100)
        } else {
            0x5151_5151
        };
        let nodes: &[(u32, u32, u32, &[u8])] = match queue {
            0 => &[],
            1 => &[(0x7069_6e67, sid, 5000, b"pn")],
            2 => &[(0x7069_6e67, sid, 5000, b"xx")],
            _ => &[
                (0x7069_6e67, sid, 5000, b"pn"),
                (0x7069_6e67, 0x1234, 0, b"qq"),
            ],
        };
        lobby_ping_chain(c, nodes);
        for i in 0..nodes.len() as u32 {
            c.bus.write32(
                LOBBY_NODE + 0x44 * i + LOBBY_REQ_ARG2,
                if cb { FAKE + 0x10 * i } else { 0 },
            );
            c.bus
                .write32(LOBBY_NODE + 0x44 * i + LOBBY_REQ_ARG3, 0x700 + i);
        }
        if queue != 0 {
            c.bus.write32(LOBBY + LOBBY_API_REQUESTS, LOBBY_NODE);
        }
        let room = |c: &mut Cpu, i: u32, server: u32| {
            let e = LOBBY_TAB + 0x200 + i * 0x80;
            lobby_fill_nonzero(c, seed + i, e, 0x20);
            c.bus.write32(e + 0x64, server);
            e
        };
        let rooms: Vec<u32> = match rooms {
            0 => vec![],
            1 => vec![room(c, 0, sid)],
            2 => vec![room(c, 0, 0x9999_9999)],
            // Two rooms for the server, only the first visible.
            _ => vec![room(c, 0, sid), room(c, 1, sid)],
        };
        if !rooms.is_empty() {
            lobby_node_list(c, LOBBY_TAB, &rooms, 1);
            c.bus.write32(LOBBY + LOBBY_API_TABLES, LOBBY_TAB + 0x700);
            c.bus.write32(LOBBY + LOBBY_API_TABLES2, LOBBY_TAB);
        }
        let user = |c: &mut Cpu, i: u32, name: &[u8]| {
            let u = LOBBY_TAB + 0x500 + i * 0x80;
            lobby_fill_nonzero(c, seed + 7 + i, u, 0x20);
            lobby_str(c, u + 8, name);
            u
        };
        let users: Vec<u32> = match users {
            0 => vec![],
            1 => vec![user(c, 0, b"pn")],
            2 => vec![user(c, 0, b"zz")],
            _ => vec![user(c, 0, b"zz"), user(c, 1, b"pn")],
        };
        if !users.is_empty() {
            lobby_node_list(c, LOBBY_TAB + 0x400, &users, 2);
            c.bus
                .write32(LOBBY + LOBBY_API_TABLES + 0x14, LOBBY_TAB + 0x780);
            c.bus
                .write32(LOBBY + LOBBY_API_TABLES2 + 0x14, LOBBY_TAB + 0x400);
        }
        lobby_fill_nonzero(c, seed, LOBBY + LOBBY_API_PING_COUNT, 1);
        c.g[A0][0] = sx(LOBBY);
        c.pc = 0x003e_7c40;
        r
    }

    /// Fill `ProcessPings`' 0x440-byte frame below the current `sp` (it may
    /// have moved to a thread stack), leaving the popped request's callback
    /// word clear (it is read whether or not a request was popped).
    pub(crate) fn lobby_pings_frame(c: &mut Cpu, r: &mut Rng) {
        let frame = (c.g[SP][0] as u32).wrapping_sub(0x440);
        fill(c, r, frame, 0x110);
        c.bus.write32(frame + 0x168, 0);
    }

    /// `lobby_api_process_pings`: reply (seed % 2), queue (seed / 2 % 4),
    /// callbacks (seed / 8 % 2), rooms (seed / 16 % 4), users (seed / 64 % 4).
    #[test]
    fn lobby_api_process_pings_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            let mut r = lobby_pings_setup(
                c,
                seed,
                0x3e7c_40,
                seed % 2 == 1,
                seed / 2 % 4,
                seed / 8 % 2 == 1,
                seed / 16 % 4,
                seed / 64 % 4,
            );
            lobby_pings_frame(c, &mut r);
        };
        check(
            ee::s14::f_003e7c40__generated,
            lobby_api_process_pings,
            256,
            &setup,
        );
        require_arm_values(
            ee::s14::f_003e7c40__generated,
            256,
            &setup,
            &lobby_pings_arm,
            &[
                (false, false, false, false),
                (true, false, false, false),
                (true, true, false, false),
                (true, false, true, false),
                (true, true, true, false),
                (true, false, false, true),
                (true, true, true, true),
                (false, false, false, true),
            ],
        );
    }

    /// What `ProcessPings` did: (a reply popped its request, a room was
    /// updated, a user was updated, a request expired).
    pub(crate) fn lobby_pings_arm(c: &mut Cpu, seed: u32) -> (bool, bool, bool, bool) {
        let queue = seed / 2 % 4;
        let mut chain = Vec::new();
        let mut n = c.bus.read32(LOBBY + LOBBY_API_REQUESTS);
        while n != 0 && chain.len() < 4 {
            chain.push(n);
            n = c.bus.read32(n + LOBBY_REQ_NEXT);
        }
        let popped = matches!(queue, 1 | 3) && !chain.contains(&LOBBY_NODE);
        let expired = queue == 3 && !chain.contains(&(LOBBY_NODE + 0x44));
        let dirty = |c: &mut Cpu, list: u32| c.bus.read32(list + 0xc) != 0;
        let rooms = seed / 16 % 4 != 0 && dirty(c, LOBBY_TAB);
        let users = seed / 64 % 4 != 0 && dirty(c, LOBBY_TAB + 0x400);
        (popped, rooms, users, expired)
    }

    counted!(pings_counted, lobby_api_process_pings, 0x003e_7c40);

    driven!(pings_gen, ee::s14::f_003e7c40__generated);

    driven!(pings_hand, pings_counted);

    /// `LobbyApi_ProcessPings` for the events test: one reply, the matching
    /// request with an expiring one behind it (callbacks), both rooms and
    /// both users; the event falls due `k` cycles in.
    pub(crate) fn lobby_pings_events_setup(c: &mut Cpu, seed: u32, k: u64) -> Events {
        // A fixed fixture: the delays below are measured on it.
        let mut r = lobby_pings_setup(c, 1, 0x3e7c_41, true, 3, true, 3, 3);
        let mode = schedule(c, &mut r, seed, 0x003e_7c40);
        c.g[A0][0] = sx(LOBBY);
        lobby_pings_frame(c, &mut r);
        if mode != Events::Off {
            c.next_event = c.cycles + k;
        }
        mode
    }

    /// Each of the four heads takes the first unwind in turn: the reply loop
    /// 0x3e7c88 (delays 120-145), the room scan 0x3e7d88 (2420-2475,
    /// 3155-3215), the user scan 0x3e7df8 (3895-4120) and the expiry loop
    /// 0x3e7e98 (5050-5095), as measured; other delays fall inside
    /// dispatcher-shaped tag callees or pass the end.
    #[test]
    fn lobby_api_process_pings_events() {
        check_resumes(
            "LobbyApi_ProcessPings",
            pings_gen,
            pings_hand,
            48,
            &[0x003e_7c88, 0x003e_7d88, 0x003e_7df8, 0x003e_7e98],
            &|c, seed| {
                let k = [125u64, 140, 2440, 3950, 5070][(seed / 3 % 5) as usize];
                lobby_pings_events_setup(c, seed, k)
            },
        );
    }

    #[test]
    fn lobby_api_reset_server_ping_matches() {
        check_paths(
            ee::s14::f_003e7f58__generated,
            lobby_api_reset_server_ping,
            16,
            &[6],
            &|c, seed| {
                let _r = lobby_base(c, seed, 0x3e7f_58, 0);
                lobby_fill_nonzero(c, seed, LOBBY, 8);
                c.g[A0][0] = LOBBY as u64;
            },
        );
    }

    #[test]
    fn lobby_api_terminate_matches() {
        check(
            ee::s14::f_003e7f70__generated,
            lobby_api_terminate,
            32,
            &|c, seed| {
                let _r = lobby_base(c, seed, 0x3e7f_70, 1);
                lobby_zero(c, LOBBY, 0x160);
                // The event word the flags are OR-ed into: non-zero.
                c.bus.write32(LOBBY + LOBBY_API_EVENTS, 0x0100_0010 + seed);
                lobby_fill_nonzero(c, seed, LOBBY_BUF, 4);
                c.g[A0][0] = LOBBY as u64;
                c.g[A1][0] = LOBBY_BUF as u64;
                c.g[A2][0] = (0x11 << (seed % 5)) as u64;
                if seed % 3 == 0 {
                    c.bus.write32(LOBBY + LOBBY_API_TERM_CB, 0);
                } else {
                    c.bus.write32(LOBBY + LOBBY_API_TERM_CB, FAKE);
                    c.bus.write32(LOBBY + LOBBY_API_TERM_ARG, 0x4400 + seed);
                }
            },
        );
    }

    #[test]
    fn lobby_api_create_matches() {
        check(
            ee::s14::f_003e7ff0__generated,
            lobby_api_create,
            32,
            &|c, seed| {
                let _r = lobby_base(c, seed, 0x3e7f_f0, 2);
                lobby_str(c, LOBBY_STR, b"plainname");
                lobby_str(c, LOBBY_STR + 0x40, b"a=b=c");
                if seed % 5 == 4 {
                    c.bus.write32(LOBBY_ALLOC_FAIL, 1);
                }
                c.g[A0][0] = match seed % 3 {
                    0 => 0,
                    1 => LOBBY_STR as u64,
                    _ => (LOBBY_STR + 0x40) as u64,
                };
                c.g[A1][0] = (0x7700 + seed) as u64;
                c.g[A2][0] = (0x8800 + seed * 3) as u64;
            },
        );
    }

    /// A hash at `hash` (one bucket) holding `n` entries whose values are
    /// listed in the list at `list` (array at `list + 0x80`, entries from
    /// `list + 0x100`), so a pop / remove / free loop drains both.
    pub(crate) fn lobby_hash_list(c: &mut Cpu, hash: u32, list: u32, n: u32) {
        let array = list + 0x80;
        let entry = |i: u32| list + 0x100 + i * 0x40;
        for i in 0..n {
            c.bus
                .write32(entry(i), if i + 1 < n { entry(i + 1) } else { 0 });
            c.bus.write32(entry(i) + 4, 0x1111 * (i + 1));
            c.bus.write32(array + 4 * i, 0x1111 * (i + 1));
        }
        make_hash(
            c,
            hash,
            hash + 0x40,
            if n == 0 { 0 } else { entry(0) },
            n,
            0,
        );
        make_list(c, list, n, array);
    }

    /// `LobbyApi_Reset`'s fixture: `slots` (bit i: slot i has a hash of
    /// `1 + i % 2` entries and its list), a request queue of `queue` nodes,
    /// and non-zero id and ping-count words (both are cleared).
    pub(crate) fn lobby_reset_setup(
        c: &mut Cpu,
        seed: u32,
        salt: u64,
        slots: u32,
        queue: usize,
    ) -> Rng {
        let r = lobby_base(c, seed, salt, 0);
        lobby_object(c);
        for i in 0..8u32 {
            if slots & 1 << i != 0 {
                let (hash, list) = (LOBBY_TAB + i * 0x400, LOBBY_TAB + i * 0x400 + 0x100);
                lobby_hash_list(c, hash, list, 1 + i % 2);
                c.bus
                    .write32(LOBBY + LOBBY_API_TABLES + i * LOBBY_SLOT_SIZE, hash);
                c.bus
                    .write32(LOBBY + LOBBY_API_TABLES2 + i * LOBBY_SLOT_SIZE, list);
            }
        }
        lobby_req_chain(c, &[0x4451_5545, 0x4451_5545][..queue]);
        if queue != 0 {
            c.bus.write32(LOBBY + LOBBY_API_REQUESTS, LOBBY_NODE);
        }
        lobby_fill_nonzero(c, seed, LOBBY + LOBBY_API_PING_COUNT, 4);
        c.g[A0][0] = sx(LOBBY);
        c.pc = 0x003e_80c0;
        r
    }

    /// `lobby_api_reset`: queue (seed % 3: empty, one node, two nodes),
    /// slots (seed / 3 % 4: none, slot 0, slot 3, slots 0 and 7).
    #[test]
    fn lobby_api_reset_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            let slots = [0, 0x01, 0x08, 0x81][(seed / 3 % 4) as usize];
            lobby_reset_setup(c, seed, 0x3e80_c0, slots, (seed % 3) as usize);
        };
        check_paths_lohi(
            ee::s14::f_003e80c0__generated,
            lobby_api_reset,
            48,
            &[245, 267, 284, 330, 350, 374, 411, 433, 457, 502, 528, 550],
            &setup,
        );
    }

    counted!(reset_counted, lobby_api_reset, 0x003e_80c0);

    driven!(reset_gen, ee::s14::f_003e80c0__generated);

    driven!(reset_hand, reset_counted);

    /// `LobbyApi_Reset` for the events test: slots 1 and 2 (two and one
    /// entries) and a two-node queue; the event falls due `k` cycles in.
    pub(crate) fn lobby_reset_events_setup(c: &mut Cpu, seed: u32, k: u64) -> Events {
        let mut r = lobby_reset_setup(c, seed, 0x3e80_c1, 0x06, 2);
        let mode = schedule(c, &mut r, seed, 0x003e_80c0);
        if mode != Events::Off {
            c.next_event = c.cycles + k;
        }
        mode
    }

    /// Each head takes the first unwind in turn: the slot loop 0x3e8110
    /// (delays 0-166), the pop loop 0x3e8128 (167-283, 341-376) and the
    /// queue loop 0x3e8178 (475-510), as measured for this fixture.
    #[test]
    fn lobby_api_reset_events() {
        check_resumes(
            "LobbyApi_Reset",
            reset_gen,
            reset_hand,
            48,
            &[0x003e_8110, 0x003e_8128, 0x003e_8178],
            &|c, seed| {
                let k = [60u64, 220, 260, 360, 420, 490][(seed / 3 % 6) as usize];
                lobby_reset_events_setup(c, seed, k)
            },
        );
    }

    #[test]
    fn lobby_api_destroy_matches() {
        check(
            ee::s14::f_003e81c8__generated,
            lobby_api_destroy,
            32,
            &|c, seed| {
                let _r = lobby_base(c, seed, 0x3e81_c8, 2);
                lobby_zero(c, LOBBY, 0x160);
                // Slots 1 and 2 hold a hash and list to tear down (seed / 6 % 2).
                if seed / 6 % 2 == 1 {
                    for i in [1u32, 2] {
                        let (hash, list) = (LOBBY_TAB + i * 0x400, LOBBY_TAB + i * 0x400 + 0x100);
                        lobby_hash_list(c, hash, list, i);
                        c.bus
                            .write32(LOBBY + LOBBY_API_TABLES + i * LOBBY_SLOT_SIZE, hash);
                        c.bus
                            .write32(LOBBY + LOBBY_API_TABLES2 + i * LOBBY_SLOT_SIZE, list);
                    }
                }
                // Queue the reset pass frees; ping/aries states it tears down.
                if seed % 2 == 0 {
                    lobby_req_chain(c, &[0x4451_5545]);
                    c.bus.write32(LOBBY + LOBBY_API_REQUESTS, LOBBY_NODE);
                }
                if seed % 3 != 0 {
                    let p = LOBBY_BUF;
                    lobby_zero(c, p, 8);
                    c.bus.write32(LOBBY + LOBBY_API_PING_CHAN, p);
                    let a = LOBBY_BUF + 0x40;
                    lobby_zero(c, a, 8);
                    c.bus.write32(LOBBY, a);
                }
                c.g[A0][0] = LOBBY as u64;
            },
        );
    }

    /// `LobbyApi_Destroy` for the events test: `n` queued requests and the
    /// ping channel and connection, events delivered, never unwinding, `k`
    /// cycles in.
    pub(crate) fn lobby_destroy_events_setup(c: &mut Cpu, seed: u32, n: usize, k: u64) -> Events {
        let mut r = lobby_base(c, 1, 0x3e81_c9, 0);
        lobby_object(c);
        lobby_req_chain(c, &[0x4451_5545, 0x4451_5545][..n]);
        c.bus.write32(LOBBY + LOBBY_API_REQUESTS, LOBBY_NODE);
        let p = LOBBY_BUF;
        lobby_zero(c, p, 8);
        c.bus.write32(LOBBY + LOBBY_API_PING_CHAN, p);
        let a = LOBBY_BUF + 0x40;
        lobby_zero(c, a, 8);
        c.bus.write32(LOBBY, a);
        let mode = schedule_no_unwind(c, &mut r, seed, 0x003e_81c8);
        c.g[A0][0] = sx(LOBBY);
        if mode != Events::Off {
            c.next_event = c.cycles + k;
        }
        mode
    }

    /// The event falls due on the teardown loop's own back edge 0x3e81f0,
    /// past `Reset` (which would otherwise take it): delays 260-334, as
    /// measured.
    #[test]
    fn lobby_api_destroy_events() {
        driven!(gen_destroy, ee::s14::f_003e81c8__generated);
        driven!(hand_destroy, lobby_api_destroy);
        check_events(gen_destroy, hand_destroy, 24, &|c, seed| {
            lobby_destroy_events_setup(c, seed, 2, 262 + (seed / 3 % 4 * 24) as u64)
        });
    }

    #[test]
    fn lobby_api_set_callback_matches() {
        check_paths(
            ee::s14::f_003e8248__generated,
            lobby_api_set_callback,
            16,
            &[6],
            &|c, seed| {
                let _r = lobby_base(c, seed, 0x3e82_48, 0);
                lobby_zero(c, LOBBY, 0x160);
                c.g[A0][0] = LOBBY as u64;
                c.g[A1][0] = (seed % 8) as u64;
                c.g[A2][0] = (0x9900 + seed) as u64;
                c.g[A3][0] = (0xaa00 + seed * 3) as u64;
            },
        );
    }

    #[test]
    fn lobby_api_connect_matches() {
        check(
            ee::s14::f_003e8260__generated,
            lobby_api_connect,
            32,
            &|c, seed| {
                let _r = lobby_base(c, seed, 0x3e82_60, 2);
                lobby_zero(c, LOBBY, 0x160);
                // The counter and the connected byte it clears: non-zero.
                lobby_fill_nonzero(c, seed, LOBBY + LOBBY_API_CONNECT_WORD, 1);
                c.bus.write8(LOBBY + LOBBY_API_CONNECTED, 0x5a);
                lobby_fill_nonzero(c, seed, LOBBY_BUF, 4);
                // Host name for the tag copy; a zeroed Aries channel.
                lobby_str(c, LOBBY_STR, b"host-name");
                c.bus.write32(LOBBY + LOBBY_API_TAG_BUF, LOBBY_STR);
                let a = LOBBY_BUF + 0x40;
                lobby_zero(c, a, 8);
                c.bus.write32(LOBBY + LOBBY_API_ARIES, a);
                if seed % 5 == 4 {
                    c.bus.write32(LOBBY_ALLOC_FAIL, 1);
                }
                if seed / 5 % 2 == 0 {
                    c.bus.write32(LOBBY + LOBBY_API_DEBUG_CB, 0);
                } else {
                    c.bus.write32(LOBBY + LOBBY_API_DEBUG_CB, FAKE);
                    c.bus.write32(LOBBY + LOBBY_API_DEBUG_ARG, 0xbb00 + seed);
                }
                c.g[A0][0] = LOBBY as u64;
                // Host doubles as a %s-safe pointer; port is a small int.
                c.g[A1][0] = LOBBY_STR as u64;
                c.g[A2][0] = (0x1f90 + seed % 7) as u64;
                c.g[A3][0] = (0xcc00 + seed) as u64;
                c.g[T0][0] = (0xdd00 + seed * 2) as u64;
            },
        );
    }

    /// `LobbyApi_SelectList`'s fixture: index (seed % 5: out of range, the
    /// three named lists, another), on/off (seed / 5 % 2), connection tag
    /// (seed / 10 % 4: `'offl'`, other, `'rdir'`, `'conn'`: steers the
    /// announce sends), and the slot (seed / 40 % 3: empty, an empty hash and
    /// list, a hash and list of two entries the off path drains).
    pub(crate) fn lobby_select_list_setup(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        let r = lobby_base(c, seed, salt, calls);
        lobby_object(c);
        let a = LOBBY_BUF + 0x40;
        lobby_zero(c, a, 8);
        c.bus.write32(LOBBY + LOBBY_API_ARIES, a);
        let index = match seed % 5 {
            0 => 8 + seed % 3,
            1 => 0,
            2 => 1,
            3 => 2,
            _ => 3 + seed % 5,
        };
        if index < 8 && seed / 40 % 3 != 0 {
            let (hash, list) = (LOBBY_TAB, LOBBY_TAB + 0x100);
            lobby_hash_list(c, hash, list, 2 * (seed / 40 % 3 - 1));
            c.bus
                .write32(LOBBY + LOBBY_API_TABLES + index * LOBBY_SLOT_SIZE, hash);
            c.bus
                .write32(LOBBY + LOBBY_API_TABLES2 + index * LOBBY_SLOT_SIZE, list);
        }
        c.bus.write32(
            LOBBY + LOBBY_API_STATE_TAG,
            [0x6f66_666c, 0x1234_5678, 0x7264_6972, 0x636f_6e6e][(seed / 10 % 4) as usize],
        );
        c.g[A0][0] = sx(LOBBY);
        c.g[A1][0] = index as u64;
        c.g[A2][0] = (seed / 5 % 2) as u64;
        c.pc = 0x003e_8368;
        r
    }

    #[test]
    fn lobby_api_select_list_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            lobby_select_list_setup(c, seed, 0x3e83_68, 2);
        };
        check_lohi(
            ee::s14::f_003e8368__generated,
            lobby_api_select_list,
            120,
            &setup,
        );
    }

    counted!(select_list_counted, lobby_api_select_list, 0x003e_8368);

    driven!(select_list_gen, ee::s14::f_003e8368__generated);

    driven!(select_list_hand, select_list_counted);

    /// `LobbyApi_SelectList` for the events test: fixture seed 94 (index 7
    /// switched off, a two-entry slot), the event falling due `k` cycles in.
    pub(crate) fn lobby_select_list_events_setup(c: &mut Cpu, seed: u32, k: u64) -> Events {
        let mut r = lobby_select_list_setup(c, 94, 0x3e83_69, 0);
        let regs = (c.g[A1], c.g[A2]);
        let mode = schedule(c, &mut r, seed, 0x003e_8368);
        (c.g[A1], c.g[A2]) = regs;
        if mode != Events::Off {
            c.next_event = c.cycles + k;
        }
        mode
    }

    /// The drain loop's head 0x3e84d0 takes the first unwind (any delay up
    /// to 137 cycles, as measured: the earlier blocks have no event check).
    #[test]
    fn lobby_api_select_list_events() {
        check_resumes(
            "LobbyApi_SelectList",
            select_list_gen,
            select_list_hand,
            48,
            &[0x003e_84d0],
            &|c, seed| lobby_select_list_events_setup(c, seed, (seed / 3 % 8 * 17) as u64),
        );
    }

    #[test]
    fn lobby_api_create_list_matches() {
        check_lohi(
            ee::s14::f_003e85b0__generated,
            lobby_api_create_list,
            32,
            &|c, seed| {
                let _r = lobby_base(c, seed, 0x3e85_b0, 0);
                lobby_zero(c, LOBBY, 0x160);
                c.g[A0][0] = LOBBY as u64;
                c.g[A1][0] = match seed % 3 {
                    0 => 8 + seed % 3,
                    1 => seed % 8,
                    _ => 0,
                } as u64;
                c.g[A2][0] = if seed / 9 % 2 == 0 { 0 } else { FAKE as u64 };
                c.g[A3][0] = (0xee00 + seed) as u64;
            },
        );
    }

    counted!(destroy_list_counted, lobby_api_destroy_list, 0x003e_8688);

    driven!(destroy_list_gen, ee::s14::f_003e8688__generated);

    driven!(destroy_list_hand, destroy_list_counted);

    /// `lobby_api_destroy_list`: kind (seed % 9, 8 = out of range), list
    /// (seed / 3 % 3 == 1: none, independent of the event mode `seed % 3`),
    /// slot word (seed / 9 % 2: cleared or still the list); unwind seeds with
    /// a list resume at the pop loop's head 0x3e86e8.
    /// | kind>=8 | list null | slot mismatch | slot match |
    #[test]
    fn lobby_api_destroy_list_matches() {
        check_resumes(
            "LobbyApi_DestroyList",
            destroy_list_gen,
            destroy_list_hand,
            72,
            &[0x003e_86e8],
            &|c, seed| {
                let mut r = lobby_base(c, seed, 0x8688, 0);
                lobby_object(c);
                lobby_bump(c, 0x100, false);
                let kind = seed % 9;
                let list = DATA + 0x1000;
                let hash = DATA + 0x2000;
                let buckets = DATA + 0x2100;
                let array = DATA + 0x2200;
                let entry = DATA + 0x2300;
                let with_list = kind < 8 && seed / 3 % 3 != 1;
                if with_list {
                    // List whose hash holds one entry (two pops: value, empty).
                    make_list(c, list, 2, array);
                    c.bus.write32(list + 0x34, hash);
                    make_hash(c, hash, buckets, entry, 1, 0);
                    // Entry: no chain, pop value 0x1111.
                    c.bus.write32(entry, 0);
                    c.bus.write32(entry + 4, 0x1111);
                    lobby_str(c, entry + 8, b"kay");
                    // Array: [hash, 0x1111] so both removes hit index 0.
                    c.bus.write32(array, hash);
                    c.bus.write32(array + 4, 0x1111);
                    // The slot's words, non-zero so the tail's clears show.
                    lobby_fill_nonzero(
                        c,
                        seed,
                        LOBBY + LOBBY_API_TABLES + kind * LOBBY_SLOT_SIZE,
                        4,
                    );
                    let slot = LOBBY + LOBBY_API_TABLES2 + kind * LOBBY_SLOT_SIZE;
                    c.bus
                        .write32(slot, if seed / 9 % 2 == 0 { 0 } else { list });
                }
                let mode = schedule(c, &mut r, seed, 0x003e_8688);
                // Set argument registers after schedule (thread start clobbers them).
                c.g[A1][0] = sx(kind);
                c.g[A2][0] = if with_list {
                    sx(list)
                } else if kind < 8 {
                    0
                } else {
                    sx(list)
                };
                c.pc = 0x003e_8688;
                mode
            },
        );
    }

    /// `lobby_api_add_list_entry`: kind (seed % 10: 8,9 bad; 0-2 range or
    /// empty by seed; 3 name list; 4-7 ranked), dup-hit (seed / 10 % 2),
    /// entry alignment (seed / 20 % 2: aligned `ld/sd` vs unaligned
    /// `ldl/sdr` copy). | bad kind | empty slot | range fail | dup |
    /// added (name) | added (ranked, both copies) |
    #[test]
    fn lobby_api_add_list_entry_matches() {
        check_paths_lohi(
            ee::s14::f_003e8778__generated,
            lobby_api_add_list_entry,
            64,
            &[21, 28, 34, 333, 374, 375, 417, 422, 441, 465, 491],
            &|c, seed| {
                lobby_base(c, seed, 0x8778, 0);
                lobby_object(c);
                lobby_bump(c, 0x200, false);
                let kind = seed % 10;
                let hit = seed / 10 % 2 == 0;
                let off = (seed / 20 % 2) as u32;
                let slot_list = DATA + 0x1000;
                let hash = DATA + 0x1400;
                let buckets = DATA + 0x1500;
                let array = DATA + 0x1600;
                let entry = DATA + 0x1700;
                let free = DATA + 0x1800;
                let name = DATA + 0x9000 + off;
                let entr = DATA + 0x9100 + off;
                for w in 0..0x60u32 {
                    c.bus.write32(DATA + 0x9000 + 4 * w, 0);
                    c.bus.write32(DATA + 0x9100 + 4 * w, 0);
                }
                lobby_str(c, name, b"kay");
                lobby_str(c, entr + 8, b"kay");
                make_list(c, slot_list, 2, array);
                c.bus.write32(array, DATA + 0x4000);
                c.bus.write32(array + 4, DATA + 0x4100);
                make_hash(c, hash, buckets, if hit { entry } else { 0 }, 1, free);
                c.bus.write32(free, 0);
                c.bus.write32(free + 4, 0);
                // Hash entries point at their key.
                c.bus.write32(entry + 8, name);
                c.bus.write32(entry, 0);
                if kind < 3 {
                    // Range-fail (slot set) or empty-slot fail by seed.
                    let slot = LOBBY + LOBBY_API_TABLES + kind * LOBBY_SLOT_SIZE;
                    c.bus.write32(slot, if hit { slot_list } else { 0 });
                } else if kind == 3 {
                    c.bus.write32(LOBBY + 0x34c, hash);
                    c.bus.write32(LOBBY + 0x350, slot_list);
                } else if kind < 8 {
                    let slot = LOBBY + LOBBY_API_TABLES + kind * LOBBY_SLOT_SIZE;
                    c.bus.write32(slot, hash);
                    c.bus.write32(
                        LOBBY + LOBBY_API_TABLES2 + kind * LOBBY_SLOT_SIZE,
                        slot_list,
                    );
                }
                c.g[A1][0] = sx(kind);
                c.g[A2][0] = sx(if kind == 3 { name } else { entr });
                c.pc = 0x003e_8778;
            },
        );
    }

    /// `lobby_api_remove_list_entry`: kind (seed % 10: 8,9 bad; 0-2 range or
    /// empty; 3-7 hashed), dup-hit (seed / 10 % 2). | bad kind | empty |
    /// range fail | miss (0) | removed (1) |
    #[test]
    fn lobby_api_remove_list_entry_matches() {
        check_paths_lohi(
            ee::s14::f_003e8968__generated,
            lobby_api_remove_list_entry,
            64,
            &[20, 26, 30, 106, 327],
            &|c, seed| {
                lobby_base(c, seed, 0x8968, 0);
                lobby_object(c);
                lobby_bump(c, 0, false);
                let kind = seed % 10;
                let hit = seed / 10 % 2 == 0;
                let slot_list = DATA + 0x1000;
                let hash = DATA + 0x1400;
                let buckets = DATA + 0x1500;
                let array = DATA + 0x1600;
                let entry = DATA + 0x1700;
                let name = DATA + 0x9000;
                lobby_str(c, name, b"kay");
                // Removal target: value word is its own address so the
                // list scan hits index 0.
                make_list(c, slot_list, 1, array);
                c.bus.write32(array, entry);
                make_hash(c, hash, buckets, if hit { entry } else { 0 }, 1, 0);
                c.bus.write32(entry, 0);
                c.bus.write32(entry + 4, entry);
                // Hash entries point at their key.
                c.bus.write32(entry + 8, name);
                if kind < 3 {
                    let slot = LOBBY + LOBBY_API_TABLES + kind * LOBBY_SLOT_SIZE;
                    c.bus.write32(slot, if hit { slot_list } else { 0 });
                } else if kind < 8 {
                    let slot = LOBBY + LOBBY_API_TABLES + kind * LOBBY_SLOT_SIZE;
                    c.bus.write32(slot, hash);
                    c.bus.write32(
                        LOBBY + LOBBY_API_TABLES2 + kind * LOBBY_SLOT_SIZE,
                        slot_list,
                    );
                }
                c.g[A1][0] = sx(kind);
                c.g[A2][0] = sx(name);
                c.pc = 0x003e_8968;
            },
        );
    }

    /// `lobby_api_get_record`: tag word (seed % 8: user/act/pers/room/play/
    /// down/zero/bogus) with flag words (seed / 8 % 4) steering the
    /// conditional-null arms.
    #[test]
    fn lobby_api_get_record_matches() {
        check(
            ee::s14::f_003e8a70__generated,
            lobby_api_get_record,
            64,
            &|c, seed| {
                let mut r = lobby_base(c, seed, 0x8a70, 0);
                lobby_object(c);
                let tag = match seed % 8 {
                    0 => 0x7573_6572u32,
                    1 => 0x6163_6374u32,
                    2 => 0x7065_7273u32,
                    3 => 0x726f_6f6du32,
                    4 => 0x706c_6179u32,
                    5 => 0x646f_776eu32,
                    6 => 0,
                    _ => r.next_u32(),
                };
                let flags = seed / 8 % 4;
                c.bus.write32(LOBBY + 0x2cc, flags & 1);
                c.bus.write8(LOBBY + LOBBY_API_CONNECTED, (flags & 2) as u8);
                c.g[A1][0] = sx(tag);
                c.pc = 0x003e_8a70;
            },
        );
    }

    /// A ProtoAries connection at `at` as `api+0` (filled words, its send
    /// state block at `+0x100`), enough for `ProtoAriesSend` without a server.
    pub(crate) fn lobby_aries(c: &mut Cpu, r: &mut Rng, at: u32) {
        fill(c, r, at, 0x10);
        c.bus.write32(at + 4, at + 0x100);
        fill(c, r, at + 0x100, 8);
        c.bus.write32(LOBBY + LOBBY_API_ARIES, at);
    }

    /// `LobbyApi_Send`'s fixture. `seed % 5`: the state (`'idle'`, `'auth'`,
    /// `'acct'`, `'skey'`, another: -1); `seed / 5 % 5`: the tag (`'auth'`,
    /// `'acct'`, `'pass'`: the login record is built on the stack; `'snap'`:
    /// the channel's lists are flushed; another: sent as given);
    /// `seed / 25 % 4`: for a login tag, bit 0 the encrypt flag (`+0x24`) and
    /// bit 1 a password field in the data; for `'snap'`, the channel (3: below
    /// the range, 4: no hash, 5: two entries, 6: one entry); `seed / 100 % 4
    /// == 0`: the request allocation fails (-2). The frame is filled, so every
    /// byte of the record the function builds shows.
    pub(crate) fn lobby_send_setup(c: &mut Cpu, seed: u32) -> Rng {
        let mut r = lobby_base(c, seed, 0x8b10, 0);
        lobby_object(c);
        lobby_bump(c, 0x100, seed / 100 % 4 == 0);
        fill(c, &mut r, crate::difftest::SP - 0x300, 0xc0);
        lobby_aries(c, &mut r, DATA + 0x8000);
        let state = [
            0x6964_6c65u32,
            0x6175_7468,
            0x6163_6374,
            0x736b_6579,
            0xdead_beef,
        ];
        c.bus
            .write32(LOBBY + LOBBY_API_STATE_TAG, state[(seed % 5) as usize]);
        let tag = [
            0x6175_7468u32,
            0x6163_6374,
            0x7061_7373,
            0x736e_6170,
            0x7573_6572,
        ];
        let tag = tag[(seed / 5 % 5) as usize];
        let sub = seed / 25 % 4;
        // The merge source and the encoding words the login arms read.
        lobby_str(c, LOBBY + LOBBY_API_TAG_BUF, b"LANG=en\tVERS=3");
        lobby_fill_nonzero(c, seed, LOBBY + LOBBY_API_PASS_NUMBER, 8);
        c.bus.write32(LOBBY + LOBBY_API_REQUESTS, 0);
        c.bus.write32(LOBBY + LOBBY_API_REQ_ID, seed);
        let data = DATA + 0x5000;
        lobby_zero(c, data, 0x40);
        if tag == 0x736e_6170 {
            // Channel 8 (just past the 4-7 range), 4 (no hash), 5, 6.
            let chan = [8, 4, 5, 6][sub as usize];
            lobby_tag(c, data, SNAP_CHANNEL_KEY, format!("{chan}").as_bytes());
            let (hash, buckets, list, array) =
                (DATA + 0x2000, DATA + 0x2100, DATA + 0x2200, DATA + 0x2300);
            let entries = if chan == 5 { 2 } else { 1 };
            let e = [DATA + 0x2400, DATA + 0x2440];
            for i in 0..2u32 {
                c.bus
                    .write32(e[i as usize], if i + 1 < entries { e[1] } else { 0 });
                c.bus.write32(e[i as usize] + 4, 0x1111 * (i + 1));
                c.bus.write32(array + 4 * i, 0x1111 * (i + 1));
            }
            make_hash(c, hash, buckets, e[0], entries, 0);
            make_list(c, list, entries, array);
            if chan < 8 {
                let slot = LOBBY + LOBBY_API_TABLES + chan * LOBBY_SLOT_SIZE;
                c.bus.write32(slot, if chan == 4 { 0 } else { hash });
                c.bus.write32(slot + 4, list);
            }
        } else {
            lobby_str(c, data, b"USER=kay");
            if sub & 2 != 0 {
                let mut s = b"USER=kay\t".to_vec();
                s.extend(image_str(c, LOGIN_PASSWORD_KEY));
                s.extend_from_slice(b"=hunter2");
                lobby_str(c, data, &s);
            }
            c.bus.write8(LOBBY + LOBBY_API_KEY, (sub & 1) as u8);
            for i in 0..0x10u32 {
                c.bus
                    .write8(LOBBY + LOBBY_API_KEY + 1 + i, (0x41 + i) as u8);
            }
        }
        c.g[A0][0] = sx(LOBBY);
        c.g[A1][0] = sx(tag);
        c.g[A2][0] = sx(data);
        c.g[A3][0] = sx(0x40 + seed % 7);
        c.g[T0][0] = sx(0x11 + seed % 5);
        c.pc = 0x003e_8b10;
        r
    }

    /// Which arm `LobbyApi_Send` took, from the final state.
    pub(crate) fn lobby_send_arm(c: &mut Cpu, seed: u32) -> &'static str {
        let v0 = c.g[V0][0] as u32;
        if v0 == u32::MAX {
            return "bad state";
        }
        if v0 == u32::MAX - 1 {
            return "no request";
        }
        let frame = crate::difftest::SP - 0x290;
        match seed / 5 % 5 {
            0..=2 if c.bus.read8(frame + 0x200) == 0x7e => "login, encrypted",
            0..=2 => "login, encoded",
            3 => match (seed / 25 % 4, c.bus.read32(DATA + 0x2000)) {
                (0, _) => "snap, channel out of range",
                (1, _) => "snap, no hash",
                (_, 0) => "snap, flushed",
                _ => "snap, not flushed",
            },
            _ => "sent as given",
        }
    }

    /// `lobby_api_send`: every state, tag and sub-case (see the fixture).
    #[test]
    fn lobby_api_send_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            lobby_send_setup(c, seed);
        };
        check_lohi(ee::s14::f_003e8b10__generated, lobby_api_send, 400, &setup);
        require_arm_values(
            ee::s14::f_003e8b10__generated,
            400,
            &setup,
            &lobby_send_arm,
            &[
                "bad state",
                "no request",
                "login, encrypted",
                "login, encoded",
                "snap, channel out of range",
                "snap, no hash",
                "snap, flushed",
                "sent as given",
            ],
        );
    }

    /// `LobbyApi_Send` for the events test: state `'idle'`, tag `'snap'` on
    /// channel 5 (two entries to flush), so the flush loop's head 0x3e8d00
    /// runs twice; the event falls due `k` cycles after entry.
    pub(crate) fn lobby_send_events_setup(c: &mut Cpu, seed: u32, k: u64) -> Events {
        // seed % 5 = 0 ('idle'), seed / 5 % 5 = 3 ('snap'), seed / 25 % 4 = 2.
        let mut r = lobby_send_setup(c, 65 + 100 * (1 + seed / 3 % 3));
        let regs = (c.g[A2], c.g[A3], c.g[T0]);
        let mode = schedule(c, &mut r, seed, 0x003e_8b10);
        (c.g[A2], c.g[A3], c.g[T0]) = regs;
        if mode != Events::Off {
            c.next_event = c.cycles + k;
        }
        mode
    }

    driven!(send_gen_driven, ee::s14::f_003e8b10__generated);

    driven!(send_hand_driven, send_hand_counted);

    /// The flush loop's head 0x3e8d00 takes the first unwind: the event falls
    /// due 254-394 cycles after entry, past the dispatcher-shaped tag callees
    /// (an unwind inside one of those cannot be re-entered by either side) and
    /// on one of the loop's two back edges (measured: 254-401).
    #[test]
    fn lobby_api_send_events() {
        let setup = |c: &mut Cpu, seed: u32| {
            lobby_send_events_setup(c, seed, 254 + (seed / 3 % 8 * 20) as u64)
        };
        check_resumes(
            "LobbyApi_Send",
            send_gen_driven,
            send_hand_driven,
            48,
            &[0x003e_8d00],
            &setup,
        );
    }

    counted!(send_hand_counted, lobby_api_send, 0x003e_8b10);

    /// `LobbyApi_CancelRequest`'s fixture: `nodes` queued requests with ids
    /// 0x1111, 0x2223, 0x2224 and non-zero `+0x38` words, the id `want`.
    pub(crate) fn lobby_cancel_request_setup(c: &mut Cpu, seed: u32, nodes: u32, want: u32) -> Rng {
        let r = lobby_base(c, seed, 0x8d70, 0);
        lobby_object(c);
        let head = DATA + 0x3000;
        c.bus.write32(
            LOBBY + LOBBY_API_REQUESTS,
            if nodes == 0 { 0 } else { head },
        );
        for i in 0..nodes {
            let n = head + i * 0x80;
            c.bus.write32(n, if i == 0 { 0x1111 } else { 0x2222 + i });
            c.bus.write32(n + LOBBY_REQ_ARG2, 0x0505_0501 + i);
            c.bus
                .write32(n + LOBBY_REQ_NEXT, if i + 1 < nodes { n + 0x80 } else { 0 });
        }
        c.g[A0][0] = sx(LOBBY);
        c.g[A1][0] = sx(want);
        c.pc = 0x003e_8d70;
        r
    }

    /// `lobby_api_cancel_request`: queue length (seed % 4: 0-3), id
    /// (seed / 4 % 4: -1 for all, the first, the second, absent).
    #[test]
    fn lobby_api_cancel_request_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            let want = [0xffff_ffffu32, 0x1111, 0x2223, 0x9999][(seed / 4 % 4) as usize];
            lobby_cancel_request_setup(c, seed, seed % 4, want);
        };
        check_paths(
            ee::s14::f_003e8d70__generated,
            lobby_api_cancel_request,
            48,
            &[6, 17, 20, 26, 32, 35, 44],
            &setup,
        );
    }

    /// The queue walk's back edge 0x3e8d88 delivers (no callees: any delay
    /// inside the walk lands there).
    #[test]
    fn lobby_api_cancel_request_events() {
        driven!(gen_cancel, ee::s14::f_003e8d70__generated);
        driven!(hand_cancel, lobby_api_cancel_request);
        check_events(gen_cancel, hand_cancel, 24, &|c, seed| {
            let mut r = lobby_cancel_request_setup(c, 1, 3, 0xffff_ffff);
            let mode = schedule_no_unwind(c, &mut r, seed, 0x003e_8d70);
            c.g[A0][0] = sx(LOBBY);
            c.g[A1][0] = sx(0xffff_ffffu32);
            if mode != Events::Off {
                c.next_event = c.cycles + (seed / 3 % 4 * 6) as u64;
            }
            mode
        });
    }

    /// `lobby_api_status`: tag (seed % 9: `'stat'`, `'rflg'`, `'extn'`,
    /// `'locl'`, `'inat'`, `'more'`, `'slot'`, zero, unknown) over an object
    /// whose words all differ; for `'inat'` the `+0x4fc`/`+0x500` words
    /// (seed / 9 % 6: first zero or not, crossed with second zero / equal /
    /// different).
    #[test]
    fn lobby_api_status_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            lobby_base(c, seed, 0x8f08, 0);
            lobby_object(c);
            lobby_fill_nonzero(c, seed, LOBBY, LOBBY_WORDS);
            let tag = match seed % 9 {
                0 => 0x7374_6174u32,
                1 => 0x7266_6c67u32,
                2 => 0x6578_746eu32,
                3 => 0x6c6f_636cu32,
                4 => 0x696e_6174u32,
                5 => 0x6d6f_7265u32,
                6 => 0x736c_6f74u32,
                7 => 0,
                _ => 0x1234_5678u32,
            };
            let flags = seed / 9 % 6;
            let first = if flags % 2 == 0 { 0 } else { 0x55 };
            let second = match flags / 2 {
                0 => 0,
                1 => first,
                _ => 0x66,
            };
            c.bus.write32(LOBBY + 0x4fc, first);
            c.bus.write32(LOBBY + 0x500, second);
            c.g[A1][0] = sx(tag);
            c.pc = 0x003e_8f08;
        };
        check_paths(
            ee::s14::f_003e8f08__generated,
            lobby_api_status,
            64,
            &[6, 9, 12, 15, 21, 24],
            &setup,
        );
    }

    /// `lobby_api_get_event`: event word (seed bits) — returns the lowest
    /// set bit, leaves the rest.
    #[test]
    fn lobby_api_get_event_matches() {
        check(
            ee::s14::f_003e8fb8__generated,
            lobby_api_get_event,
            32,
            &|c, seed| {
                let mut r = lobby_base(c, seed, 0x8fb8, 0);
                lobby_object(c);
                let word = match seed % 5 {
                    0 => 0,
                    1 => 1u32 << (seed % 31),
                    2 => 0xffff_ffff,
                    _ => r.next_u32(),
                };
                c.bus.write32(LOBBY + 0x8, word);
                c.pc = 0x003e_8fb8;
            },
        );
    }

    /// `lobby_api_debug_packet`: kind words (seed % 6: a tagged kind, an
    /// untagged one with a type, untagged and untyped, untagged `'disc'`
    /// marker, untagged with no type, tagged with no type), payload
    /// (seed / 6 % 9: empty, printable, a control byte, long with a DEL, long
    /// and clean, tab and CR, a leading DEL, a vertical tab, exactly 241
    /// bytes), and the debug callback (seed / 54 % 2: none, or a fake).
    #[test]
    fn lobby_api_debug_packet_matches() {
        check(
            ee::s14::f_003e8fd0__generated,
            lobby_api_debug_packet,
            108,
            &|c, seed| {
                lobby_base(c, seed, 0x8fd0, 0);
                lobby_object(c);
                if seed / 54 % 2 == 1 {
                    c.bus.write32(LOBBY + LOBBY_API_DEBUG_CB, 1);
                    c.bus.write32(LOBBY + LOBBY_API_DEBUG_ARG, FAKE);
                }
                let (t0, t2) = match seed % 6 {
                    0 => (0x1111_1111u32, 0x2222_2222u32),
                    1 => (0xffff_ffffu32, 0x2222_2222u32),
                    2 => (0xffff_ffffu32, 0xffff_ffffu32),
                    3 => (0xffff_ffffu32, 0xfefe_fefeu32),
                    4 => (0xffff_ffffu32, 0),
                    _ => (0x1111_1111u32, 0),
                };
                let payload = DATA + 0x4000;
                for w in 0..0x80u32 {
                    c.bus.write32(payload + 4 * w, 0x4141_4141);
                }
                match seed / 6 % 9 {
                    0 => lobby_str(c, payload, b""),
                    1 => lobby_str(c, payload, b"hello world"),
                    2 => lobby_str(c, payload, b"hi\x01there"),
                    3 => {
                        lobby_str(c, payload, &[b'x'; 250]);
                        c.bus.write8(payload + 10, 0x7f);
                    }
                    4 => lobby_str(c, payload, &[b'y'; 250]),
                    5 => lobby_str(c, payload, b"a\tb\x0d"),
                    6 => lobby_str(c, payload, b"\x7fdel"),
                    7 => lobby_str(c, payload, b"a\x0bb"),
                    _ => lobby_str(c, payload, &[b'z'; 241]),
                }
                let packet = DATA + 0x6000;
                c.bus.write32(packet + 4, t0);
                c.bus.write32(packet + 8, t2);
                c.bus.write32(packet + 0xc, payload);
                c.g[A1][0] = sx(packet);
                c.pc = 0x003e_8fd0;
            },
        );
    }

    /// `LobbyApi_TrimRankList`'s fixture: seed % 3 (no hash: -1, no list:
    /// -1, a live ranked list); for a live list `2 + seed / 12 % 4` entries
    /// named `kay0`.. (each also in the hash, by name) and `seed / 3 % 4` the
    /// ranks and cap: distinct ranks, cap above the total (all kept); ranks
    /// 1, 5, 5.. with cap 1; all rank 0 (the default rank and template) with
    /// cap 0; ranks 3, 1, 4, 1, 5 with cap 2. Ranks are counted in a
    /// 0x4000-byte histogram from the bump pool.
    pub(crate) fn lobby_trim_rank_setup(c: &mut Cpu, seed: u32) -> Rng {
        let r = lobby_base(c, seed, 0x9160, 0);
        lobby_object(c);
        lobby_bump(c, 0x200, false);
        let (list, hash, buckets, array) =
            (DATA + 0x1000, DATA + 0x2000, DATA + 0x2100, DATA + 0x2200);
        match seed % 3 {
            0 => {
                c.bus
                    .write32(LOBBY + LOBBY_API_TABLES + 2 * LOBBY_SLOT_SIZE, 0);
                c.bus
                    .write32(LOBBY + LOBBY_API_TABLES2 + 2 * LOBBY_SLOT_SIZE, list);
                c.g[A1][0] = sx(seed % 9);
            }
            1 => {
                c.bus
                    .write32(LOBBY + LOBBY_API_TABLES + 2 * LOBBY_SLOT_SIZE, hash);
                c.bus
                    .write32(LOBBY + LOBBY_API_TABLES2 + 2 * LOBBY_SLOT_SIZE, 0);
                c.g[A1][0] = sx(seed % 9);
            }
            _ => {
                let n = 2 + seed / 12 % 4;
                let mode = seed / 3 % 4;
                make_list(c, list, n, array);
                for i in 0..n {
                    // The list holds nodes whose first word is the entry
                    // (`Lobby_ListGetAt` returns `*array[i]`); the hash maps
                    // each entry's name to its node (what `Lobby_ListRemove`
                    // finds).
                    let (entry, link) = (DATA + 0x3000 + i * 0x40, DATA + 0x4000 + i * 0x40);
                    let node = DATA + 0x5000 + i * 0x10;
                    let rank = match mode {
                        0 if i + 1 == n => 0x2000,
                        0 => 1 + i,
                        1 if i == 0 => 1,
                        1 => 5,
                        2 => 0,
                        _ => [3, 1, 4, 1, 5][i as usize],
                    };
                    let name = [b'k', b'a', b'y', b'0' + i as u8];
                    c.bus.write32(entry, rank);
                    c.bus.write32(entry + 4, 0x7777);
                    lobby_str(c, entry + 8, &name);
                    c.bus.write32(node, entry);
                    c.bus.write32(array + 4 * i, node);
                    c.bus.write32(link, if i + 1 < n { link + 0x40 } else { 0 });
                    c.bus.write32(link + 4, node);
                    // Hash entries point at their key.
                    c.bus.write32(link + 8, entry + 8);
                }
                make_hash(c, hash, buckets, DATA + 0x4000, n, 0);
                c.bus
                    .write32(LOBBY + LOBBY_API_TABLES + 2 * LOBBY_SLOT_SIZE, hash);
                c.bus
                    .write32(LOBBY + LOBBY_API_TABLES2 + 2 * LOBBY_SLOT_SIZE, list);
                c.g[A1][0] = sx([n + 1, 1, 0, 2][mode as usize]);
            }
        }
        c.pc = 0x003e_9160;
        r
    }

    #[test]
    fn lobby_api_trim_rank_list_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            lobby_trim_rank_setup(c, seed);
        };
        check_paths_lohi(
            ee::s14::f_003e9160__generated,
            lobby_api_trim_rank_list,
            72,
            &[
                25, 27, 4372, 4686, 5009, 5341, 69426, 69436, 69512, 69588, 69664, 69824, 69958,
                70120, 70334, 70425, 70569, 70739,
            ],
            &setup,
        );
    }

    counted!(trim_rank_counted, lobby_api_trim_rank_list, 0x003e_9160);

    driven!(trim_rank_gen, ee::s14::f_003e9160__generated);

    driven!(trim_rank_hand, trim_rank_counted);

    /// `LobbyApi_TrimRankList` for the events test: fixture seed `fixture`
    /// (a live list), the event falling due `k` cycles in.
    pub(crate) fn lobby_trim_rank_events_setup(
        c: &mut Cpu,
        seed: u32,
        fixture: u32,
        k: u64,
    ) -> Events {
        let mut r = lobby_trim_rank_setup(c, fixture);
        let a1 = c.g[A1];
        let mode = schedule(c, &mut r, seed, 0x003e_9160);
        c.g[A1] = a1;
        if mode != Events::Off {
            c.next_event = c.cycles + k;
        }
        mode
    }

    /// Each of the five heads takes the first unwind in turn, past the
    /// histogram's `memset` (dispatcher-shaped): on fixture 44 (all rank 0,
    /// cap 0: the scan stops at once and all five are deleted) the count loop
    /// 0x3e91d0 (delays 3640-3820), the mark loop 0x3e9270 (3830-3980), the
    /// position loop 0x3e92b8 (3990-4110) and the delete loop 0x3e92f8
    /// (4120-4150); on fixture 29 (cap 1, ranks 1, 5, 5, 5) the rank scan
    /// 0x3e9238 (3770-9280). Delays as measured.
    #[test]
    fn lobby_api_trim_rank_list_events() {
        check_resumes(
            "LobbyApi_TrimRankList",
            trim_rank_gen,
            trim_rank_hand,
            48,
            &[
                0x003e_91d0,
                0x003e_9238,
                0x003e_9270,
                0x003e_92b8,
                0x003e_92f8,
            ],
            &|c, seed| {
                let (fixture, k) = [(44, 3700), (44, 3900), (44, 4050), (44, 4130), (29, 5000)]
                    [(seed / 3 % 5) as usize];
                lobby_trim_rank_events_setup(c, seed, fixture, k)
            },
        );
    }

    /// `lobby_api_ping_server`: `LobbyApi_Ping`'s fixture (state, channel,
    /// count, allocation by digit) with the server address at `+0x3b4`.
    #[test]
    fn lobby_api_ping_server_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            lobby_ping_setup(c, seed, 0x9590);
            c.bus.write32(LOBBY + LOBBY_API_PEER_HOST, 0x0a01_0204);
            c.pc = 0x003e_9590;
        };
        check_paths(
            ee::s14::f_003e9590__generated,
            lobby_api_ping_server,
            80,
            &[
                43, 46, 47, 48, 50, 53, 56, 94, 98, 101, 104, 1656, 1660, 1663, 1666,
            ],
            &setup,
        );
    }

    /// `LobbyApi_PickRoom`'s fixture: a room list (seed % 4 == 0: none) of
    /// `1 + seed / 4 % 4` rooms, each with flags `i % 2`, capacity 10, a fill
    /// of `(seed + 3i) % 11` and a name `rm0`.. (room 2 `lobby`); the wanted
    /// fill (seed / 16 % 4: 0, 30, 60, 150 percent), the flag value
    /// (seed / 64 % 2) under mask 3, and the name prefix (seed / 128 % 2:
    /// the default empty one, or `rm`).
    pub(crate) fn lobby_pick_room_setup(c: &mut Cpu, seed: u32) -> Rng {
        let r = lobby_base(c, seed, 0x9378, 0);
        lobby_object(c);
        // Seeds 256-271: one empty room (fill 0), wanted 30 percent. Seeds
        // from 272: four rooms at 30, 30, 70 and 70 percent (ties), wanted 30
        // or 70, any name.
        let empty = (256..272).contains(&seed);
        let tie = seed >= 272;
        if seed % 4 != 0 || empty || tie {
            let n = match (empty, tie) {
                (true, _) => 1,
                (_, true) => 4,
                _ => 1 + seed / 4 % 4,
            };
            let rooms: Vec<u32> = (0..n)
                .map(|i| {
                    let room = DATA + 0x2000 + i * 0x40;
                    lobby_fill_nonzero(c, seed + i, room, 0x10);
                    c.bus.write32(room + 8, if tie { 0 } else { i % 2 });
                    c.bus.write32(room + 0xc, 10);
                    let fill = match (empty, tie) {
                        (true, _) => 0,
                        (_, true) => [3, 3, 7, 7][i as usize],
                        _ => (seed + i * 3) % 11,
                    };
                    c.bus.write32(room + 0x10, fill);
                    let name: &[u8] = if i == 2 {
                        b"lobby"
                    } else {
                        &[b'r', b'm', b'0' + i as u8]
                    };
                    lobby_str(c, room + 0x1c, name);
                    room
                })
                .collect();
            lobby_node_list(c, DATA + 0x1000, &rooms, n);
            c.bus.write32(LOBBY + LOBBY_API_TABLES, DATA + 0x1800);
            c.bus.write32(LOBBY + LOBBY_API_TABLES2, DATA + 0x1000);
        }
        lobby_str(c, DATA + 0x4000, b"rm");
        let want = match (empty, tie) {
            (true, _) => 30,
            (_, true) => [30, 70][seed as usize % 2],
            _ => [0, 30, 60, 150][(seed / 16 % 4) as usize],
        };
        c.g[A0][0] = sx(LOBBY);
        c.g[A1][0] = sx(want);
        c.g[A2][0] = sx(0x3);
        c.g[A3][0] = if tie { 0 } else { sx(seed / 64 % 2) };
        c.g[T0][0] = if seed / 128 % 2 == 0 || tie {
            0
        } else {
            sx(DATA + 0x4000)
        };
        c.g[T1][0] = sx(0x77);
        c.pc = 0x003e_9378;
        r
    }

    #[test]
    fn lobby_api_pick_room_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            lobby_pick_room_setup(c, seed);
        };
        check_paths_lohi(
            ee::s14::f_003e9378__generated,
            lobby_api_pick_room,
            280,
            &[
                36, 38, 106, 108, 175, 179, 204, 208, 213, 217, 233, 237, 242, 246, 271, 275, 297,
                300, 304, 305, 326, 329, 330, 333, 334, 350, 354, 379, 383, 404, 407, 408, 411,
                412,
            ],
            &setup,
        );
    }

    counted!(pick_room_counted, lobby_api_pick_room, 0x003e_9378);

    driven!(pick_room_gen, ee::s14::f_003e9378__generated);

    driven!(pick_room_hand, pick_room_counted);

    /// `LobbyApi_PickRoom` for the events test: four rooms, the `rm` prefix,
    /// the event falling due `k` cycles in.
    pub(crate) fn lobby_pick_room_events_setup(c: &mut Cpu, seed: u32, k: u64) -> Events {
        let mut r = lobby_pick_room_setup(c, 13 + 16 + 128);
        let regs = (c.g[A1], c.g[A2], c.g[A3], c.g[T0], c.g[T1]);
        let mode = schedule(c, &mut r, seed, 0x003e_9378);
        (c.g[A1], c.g[A2], c.g[A3], c.g[T0], c.g[T1]) = regs;
        if mode != Events::Off {
            c.next_event = c.cycles + k;
        }
        mode
    }

    /// The room loop's head 0x3e9400 takes the first unwind on its first,
    /// second or third pass (delays 0-62, 172-227, 304-335, as measured).
    #[test]
    fn lobby_api_pick_room_events() {
        check_resumes(
            "LobbyApi_PickRoom",
            pick_room_gen,
            pick_room_hand,
            48,
            &[0x003e_9400],
            &|c, seed| {
                lobby_pick_room_events_setup(c, seed, [20u64, 200, 320][(seed / 3 % 3) as usize])
            },
        );
    }

    /// `lobby_api_server_ping_callback`: packet kind (seed % 3 == 0: not a
    /// `'ping'` reply), reply number (seed / 3 % 2: live non-zero / stale
    /// zero), miss count before the call (seed / 6 % 4: 0-3, so a stale
    /// reply brings it to 3 and resets it on count 2).
    /// | not a ping | live (deadline) | stale, count != 3 | stale, reset |
    #[test]
    fn lobby_api_server_ping_callback_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            lobby_base(c, seed, 0x94f0, 0);
            lobby_object(c);
            lobby_tick(c, 0x1234);
            let buf = DATA + 0x5000;
            lobby_zero(c, buf, 0x40);
            let number: &[u8] = if seed / 3 % 2 == 0 { b"5" } else { b"0" };
            lobby_tag(c, buf, SERVER_PING_KEY, number);
            let packet = DATA + 0x6000;
            let kind = if seed % 3 == 0 {
                0x1234_5678u32
            } else {
                0x7069_6e67u32
            };
            c.bus.write32(packet + 4, kind);
            c.bus.write32(packet + 0xc, buf);
            // Deadline and reply words non-zero, so their stores show.
            lobby_fill_nonzero(c, seed, LOBBY + LOBBY_API_SERVER_PING_A, 2);
            c.bus.write32(LOBBY + LOBBY_API_SERVER_PING_C, seed / 6 % 4);
            c.g[A1][0] = sx(packet);
            c.pc = 0x003e_94f0;
        };
        check_paths(
            ee::s14::f_003e94f0__generated,
            lobby_api_server_ping_callback,
            48,
            &[12, 254, 2594, 2647],
            &setup,
        );
    }

    /// `LobbyApi_Ping`'s fixture: state (seed % 5: `'conn'`, `'idle'`,
    /// `'auth'`, `'acct'`, another), ping channel (seed / 5 % 2: none, or a
    /// channel whose request the RPC layer sends), outstanding pings
    /// (seed / 10 % 4: 7, 63, 64, 100; full at 64) and the request allocation
    /// (seed / 40 % 2: it fails, or the bump pool). The tick is fixed.
    pub(crate) fn lobby_ping_setup(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let r = lobby_base(c, seed, salt, 0);
        lobby_object(c);
        lobby_tick(c, 0x1234);
        lobby_bump(c, 0x40, seed / 40 % 2 == 0);
        let state = [
            0x636f_6e6eu32,
            0x6964_6c65,
            0x6175_7468,
            0x6163_6374,
            0xdead_beef,
        ];
        c.bus
            .write32(LOBBY + LOBBY_API_STATE_TAG, state[(seed % 5) as usize]);
        let chan = DATA + 0x6000;
        lobby_zero(c, chan, 4);
        c.bus.write32(chan + 0xc, 0x2000 + seed);
        c.bus.write32(
            LOBBY + LOBBY_API_PING_CHAN,
            if seed / 5 % 2 == 0 { 0 } else { chan },
        );
        c.bus.write32(
            LOBBY + LOBBY_API_PING_COUNT,
            [7, 63, 64, 100][(seed / 10 % 4) as usize],
        );
        c.bus.write32(LOBBY + LOBBY_API_REQUESTS, 0);
        c.bus.write32(LOBBY + LOBBY_API_REQ_ID, 0x300 + seed);
        lobby_str(c, DATA + 0x7000, b"pn");
        c.g[A0][0] = sx(LOBBY);
        c.g[A1][0] = sx(DATA + 0x7000);
        c.g[A2][0] = sx(0x0a01_0203);
        c.g[A3][0] = (seed % 251) as u64;
        c.g[T0][0] = sx(FAKE + 0x40);
        c.g[T1][0] = sx(0x7700 + seed);
        c.pc = 0x003e_8dc0;
        r
    }

    /// `lobby_api_ping`: every state, channel, count and allocation (see the
    /// fixture). | bad state (-1) | no channel (-2) | full (-3) | no request
    /// (-4) | sent (the request id) |
    #[test]
    fn lobby_api_ping_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            lobby_ping_setup(c, seed, 0x8dc0);
        };
        check_paths(
            ee::s14::f_003e8dc0__generated,
            lobby_api_ping,
            80,
            &[
                30, 33, 34, 35, 37, 40, 43, 81, 85, 88, 91, 1423, 1427, 1430, 1433,
            ],
            &setup,
        );
    }

    /// `lobby_api_has_list_entry`: kind (seed % 10), dup-hit (seed % 2).
    /// | bad kind (-1) | empty (-1) | miss (0) | hit (1) |
    #[test]
    fn lobby_api_has_list_entry_matches() {
        check_paths(
            ee::s14::f_003e8a28__generated,
            lobby_api_has_list_entry,
            48,
            &[8, 13, 86, 136],
            &|c, seed| {
                lobby_base(c, seed, 0x8a28, 0);
                lobby_object(c);
                let kind = seed % 10;
                let hit = seed % 2 == 0;
                let hash = DATA + 0x1400;
                let buckets = DATA + 0x1500;
                let entry = DATA + 0x1700;
                let name = DATA + 0x9000;
                lobby_str(c, name, b"kay");
                make_hash(c, hash, buckets, if hit { entry } else { 0 }, 1, 0);
                c.bus.write32(entry, 0);
                // The value a hit returns (non-zero: the result is `0 < value`).
                c.bus.write32(entry + 4, 0x2468);
                // Hash entries point at their key.
                c.bus.write32(entry + 8, name);
                if kind < 8 {
                    let slot = LOBBY + LOBBY_API_TABLES + kind * LOBBY_SLOT_SIZE;
                    c.bus
                        .write32(slot, if hit || seed % 3 == 0 { hash } else { 0 });
                }
                c.g[A1][0] = sx(kind);
                c.g[A2][0] = sx(name);
                c.pc = 0x003e_8a28;
            },
        );
    }
    use crate::difftest::fresh;
    use ssx3_ee_rt::Cpu;

    /// Seeds per row (every row runs its setup on each seed, both sides).
    pub(crate) const SEEDS: u32 = 24;

    /// Function entry (every full-pump test starts here).
    pub(crate) const ENTRY: u32 = 0x003e_95c8;

    /// Resume head at the message dispatcher (message rows start here).
    pub(crate) const DISPATCH: u32 = 0x003e_9728;

    /// Resume head at the ping-reply check (the tagged-`~png` row starts here).
    pub(crate) const PING_CHECK: u32 = 0x003e_96b0;

    /// Lobby client block under test (0x600 bytes).
    pub(crate) const API: u32 = DATA;

    /// Tag area the scripted peek hands out (never parsed: tag reads are faked).
    pub(crate) const TAGPTR: u32 = DATA + 0x0c00;

    /// Readable node the list-data fake hands out.
    pub(crate) const NODE: u32 = DATA + 0x1800;

    /// Bump pool for the allocator fakes.
    pub(crate) const POOL: u32 = DATA + 0x2000;

    /// Scripted-result cells (all `u32`, set per row).
    pub(crate) const CELLS: u32 = DATA + 0x1000;

    /// Gate word: the fakes below run only while it holds the magic.
    pub(crate) const CTRL: u32 = CELLS + 0x00;

    pub(crate) const MAGIC: u32 = 0x4741_5445;

    pub(crate) const TICK: u32 = CELLS + 0x04;

    pub(crate) const BASE_TICK: u32 = 0x0010_0000;

    pub(crate) const PEEK_RET: u32 = CELLS + 0x08;

    pub(crate) const PEEK_N: u32 = CELLS + 0x0c;

    pub(crate) const PEEK_TYPE: u32 = CELLS + 0x10;

    pub(crate) const PEEK_BC: u32 = CELLS + 0x14;

    pub(crate) const PEEK_PTR: u32 = CELLS + 0x18;

    pub(crate) const FIND: u32 = CELLS + 0x1c;

    pub(crate) const GETNUM: u32 = CELLS + 0x20;

    pub(crate) const GETADDR: u32 = CELLS + 0x24;

    pub(crate) const GETFLAGS: u32 = CELLS + 0x28;

    pub(crate) const GETDATE: u32 = CELLS + 0x2c;

    pub(crate) const POP_RET: u32 = CELLS + 0x30;

    pub(crate) const HFIND: u32 = CELLS + 0x34;

    pub(crate) const HPOP: u32 = CELLS + 0x38;

    pub(crate) const HPOP_N: u32 = CELLS + 0x3c;

    pub(crate) const HPOP_LIMIT: u32 = CELLS + 0x40;

    pub(crate) const LADD: u32 = CELLS + 0x44;

    pub(crate) const LREM: u32 = CELLS + 0x48;

    pub(crate) const STATUS_RET: u32 = CELLS + 0x4c;

    pub(crate) const BUMP: u32 = CELLS + 0x50;

    pub(crate) const LDATA: u32 = CELLS + 0x54;

    /// Bytes the string getter hands out (fixture-set per row, NUL-padded).
    pub(crate) const STRING_SRC: u32 = CELLS + 0x100;

    /// Bytes the hex decoder hands out.
    pub(crate) const HEX_SRC: u32 = CELLS + 0x180;

    /// Per-fake call counters (asserted nonzero to prove a branch ran).
    pub(crate) const CNT: u32 = CELLS + 0x200;

    pub(crate) const N_TICK: u32 = 0;

    pub(crate) const N_PEEK: u32 = 1;

    pub(crate) const N_RECV: u32 = 2;

    pub(crate) const N_SEND: u32 = 3;

    pub(crate) const N_STATUS: u32 = 4;

    pub(crate) const N_UNCONN: u32 = 5;

    pub(crate) const N_CONN: u32 = 6;

    pub(crate) const N_TERM: u32 = 7;

    pub(crate) const N_PINGS: u32 = 8;

    pub(crate) const N_POP: u32 = 9;

    pub(crate) const N_SORT: u32 = 10;

    pub(crate) const N_PINGSRV: u32 = 11;

    pub(crate) const N_ROOMCNT: u32 = 12;

    pub(crate) const N_PRINTF: u32 = 13;

    pub(crate) const N_FIND: u32 = 14;

    pub(crate) const N_GETNUM: u32 = 15;

    pub(crate) const N_GETSTR: u32 = 16;

    pub(crate) const N_GETADDR: u32 = 17;

    pub(crate) const N_GETFLAGS: u32 = 18;

    pub(crate) const N_GETDATE: u32 = 19;

    pub(crate) const N_DECHEX: u32 = 20;

    pub(crate) const N_SETBIN: u32 = 21;

    pub(crate) const N_SETNUM: u32 = 22;

    pub(crate) const N_SETADDR: u32 = 23;

    pub(crate) const N_STRNZ: u32 = 24;

    pub(crate) const N_HFINDI: u32 = 25;

    pub(crate) const N_HDELI: u32 = 26;

    pub(crate) const N_HADDI: u32 = 27;

    pub(crate) const N_HFINDS: u32 = 28;

    pub(crate) const N_HADDS: u32 = 29;

    pub(crate) const N_HDELS: u32 = 30;

    pub(crate) const N_HPOP: u32 = 31;

    pub(crate) const N_LADD: u32 = 32;

    pub(crate) const N_LREM: u32 = 33;

    pub(crate) const N_LDATA: u32 = 34;

    pub(crate) const N_LUPD: u32 = 35;

    pub(crate) const N_EA_NEW: u32 = 36;

    pub(crate) const N_EA_FREE: u32 = 37;

    /// Client state tags (word at `+0x0c`).
    pub(crate) const ST_OFFL: u32 = 0x6f66_666c;

    pub(crate) const ST_TERM: u32 = 0x7465_726d;

    pub(crate) const ST_SKEY: u32 = 0x736b_6579;

    pub(crate) const ST_IDLE: u32 = 0x6964_6c65;

    pub(crate) const ST_CONN: u32 = 0x636f_6e6e;

    pub(crate) const ST_RDIR: u32 = 0x7264_6972;

    pub(crate) const ST_AUTH: u32 = 0x6175_7468;

    pub(crate) const ST_ACCT: u32 = 0x6163_6374;

    /// Message types (word at frame `+0x360`).
    pub(crate) const MSG_PING: u32 = 0x7e70_6e67;

    pub(crate) const MSG_DIR: u32 = 0x4064_6972;

    pub(crate) const MSG_SELE: u32 = 0x7365_6c65;

    pub(crate) const MSG_SNAP: u32 = 0x736e_6170;

    pub(crate) const MSG_PERS: u32 = 0x7065_7273;

    pub(crate) const MSG_SESS: u32 = 0x2b73_6573;

    pub(crate) const MSG_MSG: u32 = 0x2b6d_7367;

    pub(crate) const MSG_WHO: u32 = 0x2b77_686f;

    pub(crate) const MSG_ROM: u32 = 0x2b72_6f6d;

    pub(crate) const MSG_POP: u32 = 0x2b70_6f70;

    pub(crate) const MSG_USR: u32 = 0x2b75_7372;

    pub(crate) const MSG_RNK: u32 = 0x2b72_6e6b;

    pub(crate) const MSG_SNP: u32 = 0x2b73_6e70;

    pub(crate) const MSG_MOVE: u32 = 0x6d6f_7665;

    pub(crate) const MSG_ROOM: u32 = 0x726f_6f6d;

    pub(crate) const MSG_AUTH: u32 = 0x6175_7468;

    pub(crate) const MSG_ACCT: u32 = 0x6163_6374;

    /// Chat callback leaf: returns 0, touches nothing (0x1406c0).
    pub(crate) const LEAF: u32 = 0x0014_06c0;

    /// Reads the room word back (`+0x6e` is a word index).
    pub(crate) fn room(c: &mut Cpu) -> u32 {
        c.bus.read32(API + 4 * 0x6e)
    }

    /// Reads the event-flags word back.
    pub(crate) fn flags(c: &mut Cpu) -> u32 {
        c.bus.read32(API + 0x08)
    }

    /// Reads the state tag back.
    pub(crate) fn state(c: &mut Cpu) -> u32 {
        c.bus.read32(API + 0x0c)
    }

    /// How many times fake `i` ran.
    pub(crate) fn hits(c: &mut Cpu, i: u32) -> u32 {
        c.bus.read32(CNT + 4 * i)
    }

    pub(crate) fn lobbyupd_gate(c: &mut Cpu) -> bool {
        c.bus.read32(CTRL) == MAGIC
    }

    pub(crate) fn lobbyupd_done(c: &mut Cpu, v0: u64, cycles: u64) {
        c.g[V0][0] = v0;
        c.cycles += cycles;
        c.pc = c.g[RA][0] as u32;
    }

    pub(crate) fn lobbyupd_bump(c: &mut Cpu, i: u32) {
        let a = CNT + 4 * i;
        let v = c.bus.read32(a) + 1;
        c.bus.write32(a, v);
    }

    pub(crate) fn lobbyupd_fake_tick(c: &mut Cpu) {
        if !lobbyupd_gate(c) {
            return fakes::fall_back(c, &LOBBYUPD_PREV_TICK, ee::s14::ProtoAriesTick);
        }
        lobbyupd_bump(c, N_TICK);
        let v0 = c.bus.read32(TICK) as u64;
        lobbyupd_done(c, v0, 7);
    }

    pub(crate) static LOBBYUPD_PREV_TICK: AtomicUsize = AtomicUsize::new(0);

    pub(crate) fn lobbyupd_fake_peek(c: &mut Cpu) {
        if !lobbyupd_gate(c) {
            return fakes::fall_back(c, &LOBBYUPD_PREV_PEEK, ee::s14::ProtoAriesPeek);
        }
        lobbyupd_bump(c, N_PEEK);
        let n = c.bus.read32(PEEK_N);
        c.bus.write32(PEEK_N, n + 1);
        if n == 0 {
            let sp = c.g[SP][0] as u32;
            let ty = c.bus.read32(PEEK_TYPE);
            let bc = c.bus.read32(PEEK_BC);
            let pp = c.bus.read32(PEEK_PTR);
            c.bus.write32(sp.wrapping_add(0x360), ty);
            c.bus.write32(sp.wrapping_add(0x364), bc);
            c.bus.write32(sp.wrapping_add(0x368), pp);
            let v0 = c.bus.read32(PEEK_RET) as u64;
            lobbyupd_done(c, v0, 9);
        } else {
            lobbyupd_done(c, 0xffff_ffff_ffff_ffff, 9);
        }
    }

    pub(crate) static LOBBYUPD_PREV_PEEK: AtomicUsize = AtomicUsize::new(0);

    pub(crate) fn lobbyupd_fake_recv(c: &mut Cpu) {
        if !lobbyupd_gate(c) {
            return fakes::fall_back(c, &LOBBYUPD_PREV_RECV, ee::s14::ProtoAriesRecv);
        }
        lobbyupd_bump(c, N_RECV);
        lobbyupd_done(c, 0, 5);
    }

    pub(crate) static LOBBYUPD_PREV_RECV: AtomicUsize = AtomicUsize::new(0);

    pub(crate) fn lobbyupd_fake_send(c: &mut Cpu) {
        if !lobbyupd_gate(c) {
            return fakes::fall_back(c, &LOBBYUPD_PREV_SEND, ee::s14::ProtoAriesSend);
        }
        lobbyupd_bump(c, N_SEND);
        lobbyupd_done(c, 0, 9);
    }

    pub(crate) static LOBBYUPD_PREV_SEND: AtomicUsize = AtomicUsize::new(0);

    pub(crate) fn lobbyupd_fake_status(c: &mut Cpu) {
        if !lobbyupd_gate(c) {
            return fakes::fall_back(c, &LOBBYUPD_PREV_STATUS, ee::s14::ProtoAriesStatus);
        }
        lobbyupd_bump(c, N_STATUS);
        let v0 = c.bus.read32(STATUS_RET) as u64;
        lobbyupd_done(c, v0, 7);
    }

    pub(crate) static LOBBYUPD_PREV_STATUS: AtomicUsize = AtomicUsize::new(0);

    pub(crate) fn lobbyupd_fake_unconnect(c: &mut Cpu) {
        if !lobbyupd_gate(c) {
            return fakes::fall_back(c, &LOBBYUPD_PREV_UNCONN, ee::s14::ProtoAriesUnconnect);
        }
        lobbyupd_bump(c, N_UNCONN);
        lobbyupd_done(c, 0, 5);
    }

    pub(crate) static LOBBYUPD_PREV_UNCONN: AtomicUsize = AtomicUsize::new(0);

    pub(crate) fn lobbyupd_fake_connect(c: &mut Cpu) {
        if !lobbyupd_gate(c) {
            return fakes::fall_back(c, &LOBBYUPD_PREV_CONN, ee::s14::ProtoAriesConnect);
        }
        lobbyupd_bump(c, N_CONN);
        lobbyupd_done(c, 0, 5);
    }

    pub(crate) static LOBBYUPD_PREV_CONN: AtomicUsize = AtomicUsize::new(0);

    pub(crate) fn lobbyupd_fake_terminate(c: &mut Cpu) {
        if !lobbyupd_gate(c) {
            return fakes::fall_back(c, &LOBBYUPD_PREV_TERM, ee::s14::LobbyApi_Terminate);
        }
        lobbyupd_bump(c, N_TERM);
        c.bus.write32(API + 0x0c, ST_TERM);
        lobbyupd_done(c, 0, 9);
    }

    pub(crate) static LOBBYUPD_PREV_TERM: AtomicUsize = AtomicUsize::new(0);

    pub(crate) fn lobbyupd_fake_pings(c: &mut Cpu) {
        if !lobbyupd_gate(c) {
            return fakes::fall_back(c, &LOBBYUPD_PREV_PINGS, ee::s14::LobbyApi_ProcessPings);
        }
        lobbyupd_bump(c, N_PINGS);
        lobbyupd_done(c, 0, 11);
    }

    pub(crate) static LOBBYUPD_PREV_PINGS: AtomicUsize = AtomicUsize::new(0);

    pub(crate) fn lobbyupd_fake_pop(c: &mut Cpu) {
        if !lobbyupd_gate(c) {
            return fakes::fall_back(c, &LOBBYUPD_PREV_POP, ee::s14::LobbyApi_PopRequest);
        }
        lobbyupd_bump(c, N_POP);
        let out = c.g[A1][0] as u32;
        let h = c.bus.read32(POP_RET);
        c.bus.write32(out + 0x38, h);
        lobbyupd_done(c, h as u64, 11);
    }

    pub(crate) static LOBBYUPD_PREV_POP: AtomicUsize = AtomicUsize::new(0);

    pub(crate) fn lobbyupd_fake_sort(c: &mut Cpu) {
        if !lobbyupd_gate(c) {
            return fakes::fall_back(c, &LOBBYUPD_PREV_SORT, ee::s14::LobbyApi_SortLists);
        }
        lobbyupd_bump(c, N_SORT);
        lobbyupd_done(c, 0, 7);
    }

    pub(crate) static LOBBYUPD_PREV_SORT: AtomicUsize = AtomicUsize::new(0);

    pub(crate) fn lobbyupd_fake_pingserver(c: &mut Cpu) {
        if !lobbyupd_gate(c) {
            return fakes::fall_back(c, &LOBBYUPD_PREV_PINGSRV, ee::s14::LobbyApi_PingServer);
        }
        lobbyupd_bump(c, N_PINGSRV);
        lobbyupd_done(c, 0, 7);
    }

    pub(crate) static LOBBYUPD_PREV_PINGSRV: AtomicUsize = AtomicUsize::new(0);

    pub(crate) fn lobbyupd_fake_roomcount(c: &mut Cpu) {
        if !lobbyupd_gate(c) {
            return fakes::fall_back(c, &LOBBYUPD_PREV_ROOMCNT, ee::s14::LobbyApi_UpdateRoomCount);
        }
        lobbyupd_bump(c, N_ROOMCNT);
        lobbyupd_done(c, 0, 7);
    }

    pub(crate) static LOBBYUPD_PREV_ROOMCNT: AtomicUsize = AtomicUsize::new(0);

    pub(crate) fn lobbyupd_fake_printf(c: &mut Cpu) {
        if !lobbyupd_gate(c) {
            return fakes::fall_back(c, &LOBBYUPD_PREV_PRINTF, ee::s14::LobbyApi_DebugPrintf);
        }
        lobbyupd_bump(c, N_PRINTF);
        lobbyupd_done(c, 0, 5);
    }

    pub(crate) static LOBBYUPD_PREV_PRINTF: AtomicUsize = AtomicUsize::new(0);

    pub(crate) fn lobbyupd_fake_find(c: &mut Cpu) {
        if !lobbyupd_gate(c) {
            return fakes::fall_back(c, &LOBBYUPD_PREV_FIND, ee::s14::TagFieldFind);
        }
        lobbyupd_bump(c, N_FIND);
        let v0 = c.bus.read32(FIND) as u64;
        lobbyupd_done(c, v0, 6);
    }

    pub(crate) static LOBBYUPD_PREV_FIND: AtomicUsize = AtomicUsize::new(0);

    pub(crate) fn lobbyupd_fake_getnum(c: &mut Cpu) {
        if !lobbyupd_gate(c) {
            return fakes::fall_back(c, &LOBBYUPD_PREV_GETNUM, ee::s14::TagFieldGetNumber);
        }
        lobbyupd_bump(c, N_GETNUM);
        // Sign-extended: the game compares the result against sign-extended
        // sentinels (e.g. -2 for "remove all").
        let v0 = c.bus.read32(GETNUM) as i32 as u64;
        lobbyupd_done(c, v0, 6);
    }

    pub(crate) static LOBBYUPD_PREV_GETNUM: AtomicUsize = AtomicUsize::new(0);

    pub(crate) fn lobbyupd_fake_getstr(c: &mut Cpu) {
        if !lobbyupd_gate(c) {
            return fakes::fall_back(c, &LOBBYUPD_PREV_GETSTR, ee::s14::TagFieldGetString);
        }
        lobbyupd_bump(c, N_GETSTR);
        let dst = c.g[A1][0] as u32;
        let max = c.g[A2][0] as u32;
        let n = (max as usize).min(64);
        let words = n / 4;
        for i in 0..words {
            let w = c.bus.read32(STRING_SRC + 4 * i as u32);
            c.bus.write32(dst + 4 * i as u32, w);
        }
        for i in 4 * words..n {
            let b = (c.bus.read32(STRING_SRC + (i as u32 & !3)) >> (8 * (i as u32 & 3))) as u8;
            c.bus.write8(dst + i as u32, b);
        }
        lobbyupd_done(c, 0, 8);
    }

    pub(crate) static LOBBYUPD_PREV_GETSTR: AtomicUsize = AtomicUsize::new(0);

    pub(crate) fn lobbyupd_fake_getaddr(c: &mut Cpu) {
        if !lobbyupd_gate(c) {
            return fakes::fall_back(c, &LOBBYUPD_PREV_GETADDR, ee::s14::TagFieldGetAddress);
        }
        lobbyupd_bump(c, N_GETADDR);
        let v0 = c.bus.read32(GETADDR) as u64;
        lobbyupd_done(c, v0, 6);
    }

    pub(crate) static LOBBYUPD_PREV_GETADDR: AtomicUsize = AtomicUsize::new(0);

    pub(crate) fn lobbyupd_fake_getflags(c: &mut Cpu) {
        if !lobbyupd_gate(c) {
            return fakes::fall_back(c, &LOBBYUPD_PREV_GETFLAGS, ee::s14::TagFieldGetFlags);
        }
        lobbyupd_bump(c, N_GETFLAGS);
        let v0 = c.bus.read32(GETFLAGS) as u64;
        lobbyupd_done(c, v0, 6);
    }

    pub(crate) static LOBBYUPD_PREV_GETFLAGS: AtomicUsize = AtomicUsize::new(0);

    pub(crate) fn lobbyupd_fake_getdate(c: &mut Cpu) {
        if !lobbyupd_gate(c) {
            return fakes::fall_back(c, &LOBBYUPD_PREV_GETDATE, ee::s14::TagFieldGetDate);
        }
        lobbyupd_bump(c, N_GETDATE);
        let v0 = c.bus.read32(GETDATE) as u64;
        lobbyupd_done(c, v0, 6);
    }

    pub(crate) static LOBBYUPD_PREV_GETDATE: AtomicUsize = AtomicUsize::new(0);

    pub(crate) fn lobbyupd_fake_dechex(c: &mut Cpu) {
        if !lobbyupd_gate(c) {
            return fakes::fall_back(c, &LOBBYUPD_PREV_DECHEX, ee::s14::TagFieldDecodeHex);
        }
        lobbyupd_bump(c, N_DECHEX);
        let dst = c.g[A1][0] as u32;
        for i in 0..4u32 {
            let w = c.bus.read32(HEX_SRC + 4 * i);
            c.bus.write32(dst + 4 * i, w);
        }
        lobbyupd_done(c, 0, 8);
    }

    pub(crate) static LOBBYUPD_PREV_DECHEX: AtomicUsize = AtomicUsize::new(0);

    pub(crate) fn lobbyupd_fake_setbin(c: &mut Cpu) {
        if !lobbyupd_gate(c) {
            return fakes::fall_back(c, &LOBBYUPD_PREV_SETBIN, ee::s14::TagFieldSetBinary);
        }
        lobbyupd_bump(c, N_SETBIN);
        lobbyupd_done(c, 0, 7);
    }

    pub(crate) static LOBBYUPD_PREV_SETBIN: AtomicUsize = AtomicUsize::new(0);

    pub(crate) fn lobbyupd_fake_setnum(c: &mut Cpu) {
        if !lobbyupd_gate(c) {
            return fakes::fall_back(c, &LOBBYUPD_PREV_SETNUM, ee::s14::TagFieldSetNumber);
        }
        lobbyupd_bump(c, N_SETNUM);
        lobbyupd_done(c, 0, 6);
    }

    pub(crate) static LOBBYUPD_PREV_SETNUM: AtomicUsize = AtomicUsize::new(0);

    pub(crate) fn lobbyupd_fake_setaddr(c: &mut Cpu) {
        if !lobbyupd_gate(c) {
            return fakes::fall_back(c, &LOBBYUPD_PREV_SETADDR, ee::s14::TagFieldSetAddress);
        }
        lobbyupd_bump(c, N_SETADDR);
        lobbyupd_done(c, 0, 6);
    }

    pub(crate) static LOBBYUPD_PREV_SETADDR: AtomicUsize = AtomicUsize::new(0);

    pub(crate) fn lobbyupd_fake_strnz(c: &mut Cpu) {
        if !lobbyupd_gate(c) {
            return fakes::fall_back(c, &LOBBYUPD_PREV_STRNZ, ee::s14::TagFieldStrnzcpy);
        }
        lobbyupd_bump(c, N_STRNZ);
        let dst = c.g[A0][0] as u32;
        let src = c.g[A2][0] as u32;
        for i in 0..8u32 {
            let b = (c.bus.read32(src + (i & !3)) >> (8 * (i & 3))) as u8;
            c.bus.write8(dst + i, b);
        }
        c.bus.write8(dst + 8, 0);
        lobbyupd_done(c, 0, 6);
    }

    pub(crate) static LOBBYUPD_PREV_STRNZ: AtomicUsize = AtomicUsize::new(0);

    pub(crate) fn lobbyupd_fake_hfindi(c: &mut Cpu) {
        if !lobbyupd_gate(c) {
            return fakes::fall_back(c, &LOBBYUPD_PREV_HFINDI, ee::s14::Lobby_HashFindInt);
        }
        lobbyupd_bump(c, N_HFINDI);
        let v0 = c.bus.read32(HFIND) as u64;
        lobbyupd_done(c, v0, 6);
    }

    pub(crate) static LOBBYUPD_PREV_HFINDI: AtomicUsize = AtomicUsize::new(0);

    pub(crate) fn lobbyupd_fake_hdeli(c: &mut Cpu) {
        if !lobbyupd_gate(c) {
            return fakes::fall_back(c, &LOBBYUPD_PREV_HDELI, ee::s14::Lobby_HashDelInt);
        }
        lobbyupd_bump(c, N_HDELI);
        lobbyupd_done(c, 0, 6);
    }

    pub(crate) static LOBBYUPD_PREV_HDELI: AtomicUsize = AtomicUsize::new(0);

    pub(crate) fn lobbyupd_fake_haddi(c: &mut Cpu) {
        if !lobbyupd_gate(c) {
            return fakes::fall_back(c, &LOBBYUPD_PREV_HADDI, ee::s14::Lobby_HashAddInt);
        }
        lobbyupd_bump(c, N_HADDI);
        lobbyupd_done(c, 0, 6);
    }

    pub(crate) static LOBBYUPD_PREV_HADDI: AtomicUsize = AtomicUsize::new(0);

    pub(crate) fn lobbyupd_fake_hfinds(c: &mut Cpu) {
        if !lobbyupd_gate(c) {
            return fakes::fall_back(c, &LOBBYUPD_PREV_HFINDS, ee::s14::Lobby_HashFindStr);
        }
        lobbyupd_bump(c, N_HFINDS);
        let v0 = c.bus.read32(HFIND) as u64;
        lobbyupd_done(c, v0, 6);
    }

    pub(crate) static LOBBYUPD_PREV_HFINDS: AtomicUsize = AtomicUsize::new(0);

    pub(crate) fn lobbyupd_fake_hadds(c: &mut Cpu) {
        if !lobbyupd_gate(c) {
            return fakes::fall_back(c, &LOBBYUPD_PREV_HADDS, ee::s14::Lobby_HashAddStr);
        }
        lobbyupd_bump(c, N_HADDS);
        lobbyupd_done(c, 0, 6);
    }

    pub(crate) static LOBBYUPD_PREV_HADDS: AtomicUsize = AtomicUsize::new(0);

    pub(crate) fn lobbyupd_fake_hdels(c: &mut Cpu) {
        if !lobbyupd_gate(c) {
            return fakes::fall_back(c, &LOBBYUPD_PREV_HDELS, ee::s14::Lobby_HashDelStr);
        }
        lobbyupd_bump(c, N_HDELS);
        lobbyupd_done(c, 0, 6);
    }

    pub(crate) static LOBBYUPD_PREV_HDELS: AtomicUsize = AtomicUsize::new(0);

    pub(crate) fn lobbyupd_fake_hpop(c: &mut Cpu) {
        if !lobbyupd_gate(c) {
            return fakes::fall_back(c, &LOBBYUPD_PREV_HPOP, ee::s14::Lobby_HashPopAny);
        }
        lobbyupd_bump(c, N_HPOP);
        let n = c.bus.read32(HPOP_N);
        c.bus.write32(HPOP_N, n + 1);
        if n < c.bus.read32(HPOP_LIMIT) {
            let v0 = c.bus.read32(HPOP) as u64;
            lobbyupd_done(c, v0, 6);
        } else {
            lobbyupd_done(c, 0, 6);
        }
    }

    pub(crate) static LOBBYUPD_PREV_HPOP: AtomicUsize = AtomicUsize::new(0);

    pub(crate) fn lobbyupd_fake_ladd(c: &mut Cpu) {
        if !lobbyupd_gate(c) {
            return fakes::fall_back(c, &LOBBYUPD_PREV_LADD, ee::s14::Lobby_ListAdd);
        }
        lobbyupd_bump(c, N_LADD);
        let v0 = c.bus.read32(LADD) as u64;
        lobbyupd_done(c, v0, 7);
    }

    pub(crate) static LOBBYUPD_PREV_LADD: AtomicUsize = AtomicUsize::new(0);

    pub(crate) fn lobbyupd_fake_lrem(c: &mut Cpu) {
        if !lobbyupd_gate(c) {
            return fakes::fall_back(c, &LOBBYUPD_PREV_LREM, ee::s14::Lobby_ListRemove);
        }
        lobbyupd_bump(c, N_LREM);
        let v0 = c.bus.read32(LREM) as u64;
        lobbyupd_done(c, v0, 7);
    }

    pub(crate) static LOBBYUPD_PREV_LREM: AtomicUsize = AtomicUsize::new(0);

    pub(crate) fn lobbyupd_fake_ldata(c: &mut Cpu) {
        if !lobbyupd_gate(c) {
            return fakes::fall_back(c, &LOBBYUPD_PREV_LDATA, ee::s14::Lobby_ListGetData);
        }
        lobbyupd_bump(c, N_LDATA);
        // Scripted per row (the base plants `NODE`, the rnl row steers the
        // low bits to take either copy-loop direction).
        let v0 = c.bus.read32(LDATA) as u64;
        lobbyupd_done(c, v0, 6);
    }

    pub(crate) static LOBBYUPD_PREV_LDATA: AtomicUsize = AtomicUsize::new(0);

    pub(crate) fn lobbyupd_fake_lupd(c: &mut Cpu) {
        if !lobbyupd_gate(c) {
            return fakes::fall_back(c, &LOBBYUPD_PREV_LUPD, ee::s14::Lobby_ListUpdate);
        }
        lobbyupd_bump(c, N_LUPD);
        lobbyupd_done(c, 0, 6);
    }

    pub(crate) static LOBBYUPD_PREV_LUPD: AtomicUsize = AtomicUsize::new(0);

    pub(crate) fn lobbyupd_fake_ea_new(c: &mut Cpu) {
        if !lobbyupd_gate(c) {
            return fakes::fall_back(c, &LOBBYUPD_PREV_EA_NEW, ee::s10::f_00317e30);
        }
        lobbyupd_bump(c, N_EA_NEW);
        let size = (c.g[A0][0] as u32 + 15) & !15;
        let at = c.bus.read32(BUMP);
        c.bus.write32(BUMP, at + size);
        c.g[V0][0] = at as u64;
        c.g[V1][0] = 0;
        c.cycles += 11;
        c.pc = c.g[RA][0] as u32;
    }

    pub(crate) static LOBBYUPD_PREV_EA_NEW: AtomicUsize = AtomicUsize::new(0);

    pub(crate) fn lobbyupd_fake_ea_free(c: &mut Cpu) {
        if !lobbyupd_gate(c) {
            return fakes::fall_back(c, &LOBBYUPD_PREV_EA_FREE, ee::s10::f_00317e98);
        }
        lobbyupd_bump(c, N_EA_FREE);
        c.g[V0][0] = 0;
        c.g[V1][0] = 0;
        c.cycles += 5;
        c.pc = c.g[RA][0] as u32;
    }

    pub(crate) static LOBBYUPD_PREV_EA_FREE: AtomicUsize = AtomicUsize::new(0);

    /// Install every fake once per process (later calls are no-ops).
    pub(crate) fn lobbyupd_install_fakes() {
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003F5D88),
            lobbyupd_fake_tick,
            &LOBBYUPD_PREV_TICK,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003F5A70),
            lobbyupd_fake_peek,
            &LOBBYUPD_PREV_PEEK,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003F5C38),
            lobbyupd_fake_recv,
            &LOBBYUPD_PREV_RECV,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003F5910),
            lobbyupd_fake_send,
            &LOBBYUPD_PREV_SEND,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003F57C8),
            lobbyupd_fake_status,
            &LOBBYUPD_PREV_STATUS,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003F5738),
            lobbyupd_fake_unconnect,
            &LOBBYUPD_PREV_UNCONN,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003F5638),
            lobbyupd_fake_connect,
            &LOBBYUPD_PREV_CONN,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003E7F70),
            lobbyupd_fake_terminate,
            &LOBBYUPD_PREV_TERM,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003E7C40),
            lobbyupd_fake_pings,
            &LOBBYUPD_PREV_PINGS,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003E77F0),
            lobbyupd_fake_pop,
            &LOBBYUPD_PREV_POP,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003E7BA0),
            lobbyupd_fake_sort,
            &LOBBYUPD_PREV_SORT,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003E9590),
            lobbyupd_fake_pingserver,
            &LOBBYUPD_PREV_PINGSRV,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003E7B00),
            lobbyupd_fake_roomcount,
            &LOBBYUPD_PREV_ROOMCNT,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003E76A8),
            lobbyupd_fake_printf,
            &LOBBYUPD_PREV_PRINTF,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003EB6D8),
            lobbyupd_fake_find,
            &LOBBYUPD_PREV_FIND,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003ECF30),
            lobbyupd_fake_getnum,
            &LOBBYUPD_PREV_GETNUM,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003ED378),
            lobbyupd_fake_getstr,
            &LOBBYUPD_PREV_GETSTR,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003ECFF8),
            lobbyupd_fake_getaddr,
            &LOBBYUPD_PREV_GETADDR,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003ECFB0),
            lobbyupd_fake_getflags,
            &LOBBYUPD_PREV_GETFLAGS,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003EDB08),
            lobbyupd_fake_getdate,
            &LOBBYUPD_PREV_GETDATE,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003ED578),
            lobbyupd_fake_dechex,
            &LOBBYUPD_PREV_DECHEX,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003EC300),
            lobbyupd_fake_setbin,
            &LOBBYUPD_PREV_SETBIN,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003EBE40),
            lobbyupd_fake_setnum,
            &LOBBYUPD_PREV_SETNUM,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003EBFC8),
            lobbyupd_fake_setaddr,
            &LOBBYUPD_PREV_SETADDR,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003EBB60),
            lobbyupd_fake_strnz,
            &LOBBYUPD_PREV_STRNZ,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003EEEE8),
            lobbyupd_fake_hfindi,
            &LOBBYUPD_PREV_HFINDI,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003EEF40),
            lobbyupd_fake_hdeli,
            &LOBBYUPD_PREV_HDELI,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003EEDA0),
            lobbyupd_fake_haddi,
            &LOBBYUPD_PREV_HADDI,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003EEC88),
            lobbyupd_fake_hfinds,
            &LOBBYUPD_PREV_HFINDS,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003EEB30),
            lobbyupd_fake_hadds,
            &LOBBYUPD_PREV_HADDS,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003EED08),
            lobbyupd_fake_hdels,
            &LOBBYUPD_PREV_HDELS,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003EE9A0),
            lobbyupd_fake_hpop,
            &LOBBYUPD_PREV_HPOP,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003EE320),
            lobbyupd_fake_ladd,
            &LOBBYUPD_PREV_LADD,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003EE480),
            lobbyupd_fake_lrem,
            &LOBBYUPD_PREV_LREM,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003EE468),
            lobbyupd_fake_ldata,
            &LOBBYUPD_PREV_LDATA,
        );
        fakes::hook(
            addr_of_mut!(ee::s14::OVERRIDE_003EE3E0),
            lobbyupd_fake_lupd,
            &LOBBYUPD_PREV_LUPD,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E30),
            lobbyupd_fake_ea_new,
            &LOBBYUPD_PREV_EA_NEW,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E98),
            lobbyupd_fake_ea_free,
            &LOBBYUPD_PREV_EA_FREE,
        );
    }

    /// Shared base: installs the fakes, sets gp, fills the client block and
    /// scratch areas with distinctive nonzero words, zeroes the callback
    /// slots and list tables (rows opt in), and writes default scripted
    /// cells (rows tweak from here).
    pub(crate) fn lobbyupd_base(c: &mut Cpu) {
        lobbyupd_install_fakes();
        c.g[GP][0] = GAME_GP as u64;
        for i in 0..0x180u32 {
            c.bus.write32(API + 4 * i, 0xC700_0000 | (i + 1));
        }
        for i in 0..0x50u32 {
            c.bus.write32(TAGPTR + 4 * i, 0x07A6_0000 | (i + 1));
        }
        for i in 0..0x50u32 {
            c.bus.write32(NODE + 4 * i, 0xB0DE_0000 | (i + 1));
        }
        for i in 0..0x400u32 {
            c.bus.write32(POOL + 4 * i, 0xE001_0000 | (i + 1));
        }
        for i in 0..11u32 {
            c.bus.write32(API + 0x514 + 4 * i, 0);
        }
        for i in 0..16u32 {
            c.bus.write32(API + 0x310 + 4 * i, 0);
        }
        c.bus.write32(API + 0x08, 0);
        c.bus.write32(API + 0x0c, ST_CONN);
        c.bus.write32(API + 0x10, 0);
        c.bus.write32(API + 0x14, 0);
        c.bus.write32(API + 0x18, 0);
        c.bus.write32(API + 0x1c, 0);
        c.bus.write32(API + 4 * 0x6e, 99);
        c.bus.write32(CTRL, MAGIC);
        c.bus.write32(TICK, BASE_TICK);
        c.bus.write32(PEEK_RET, 0xffff_ffff);
        c.bus.write32(PEEK_N, 0);
        c.bus.write32(PEEK_TYPE, 0);
        c.bus.write32(PEEK_BC, 0);
        c.bus.write32(PEEK_PTR, TAGPTR);
        c.bus.write32(FIND, 0);
        c.bus.write32(GETNUM, 0x2e);
        c.bus.write32(GETADDR, 0);
        c.bus.write32(GETFLAGS, 0);
        c.bus.write32(GETDATE, 0x5d1e_0001);
        c.bus.write32(POP_RET, 0);
        c.bus.write32(HFIND, 0);
        c.bus.write32(HPOP, NODE);
        c.bus.write32(HPOP_N, 0);
        c.bus.write32(HPOP_LIMIT, 0);
        c.bus.write32(LADD, NODE);
        c.bus.write32(LREM, NODE);
        c.bus.write32(LDATA, NODE);
        c.bus.write32(STATUS_RET, 0x0700_0001);
        c.bus.write32(BUMP, POOL);
        for i in 0..16u32 {
            c.bus.write32(STRING_SRC + 4 * i, 0);
        }
        for i in 0..4u32 {
            c.bus.write32(HEX_SRC + 4 * i, 0x5EED_0000 | (i + 1));
        }
        for i in 0..38u32 {
            c.bus.write32(CNT + 4 * i, 0);
        }
        c.g[S3][0] = API as u64;
        c.g[S7][0] = BASE_TICK as u64;
        c.g[S0][0] = 0x9700_5000;
        c.g[S1][0] = 0x9700_5001;
        c.g[S2][0] = 0x9700_5002;
        c.g[S4][0] = 0x9700_5004;
        c.g[S5][0] = 0x9700_5005;
        c.g[S6][0] = 0x9700_5006;
        c.g[FP][0] = 0x9700_501c;
        c.g[V0][0] = 0x9700_5002_0000;
        c.g[V1][0] = 0x9700_5003_0000;
        c.g[A0][0] = 0x9700_5004_0000;
        c.g[A1][0] = 0x9700_5005_0000;
        c.g[A2][0] = 0x9700_5006_0000;
        c.g[A3][0] = 0x9700_5007_0000;
        c.g[RA][0] = TEST_RA as u64;
    }

    /// Full-pump entry: the argument register points at the client.
    pub(crate) fn lobbyupd_entry(c: &mut Cpu) {
        c.g[A0][0] = API as u64;
        c.pc = ENTRY;
    }

    /// Direct entry at a resume head: carve the frame the prologue would
    /// have made, stash the sentinel return address in the saved-ra slot,
    /// and rewire the callee-saved registers.
    pub(crate) fn lobbyupd_direct(c: &mut Cpu, head: u32) {
        let sp = (c.g[SP][0] as u32).wrapping_sub(0x420);
        c.g[SP][0] = sp as u64;
        for i in 0..0x108u32 {
            c.bus.write32(sp + 4 * i, 0xD700_0000 | (i + 1));
        }
        c.bus.write32(sp + 0x380, TEST_RA);
        c.pc = head;
    }

    /// Message-row entry at the dispatcher: the type, subtype and payload
    /// words the peek would have delivered.
    pub(crate) fn lobbyupd_msg_at(c: &mut Cpu, ty: u32, bc: u32, head: u32) {
        lobbyupd_base(c);
        lobbyupd_direct(c, head);
        let sp = c.g[SP][0] as u32;
        c.bus.write32(sp + 0x360, ty);
        c.bus.write32(sp + 0x364, bc);
        c.bus.write32(sp + 0x368, TAGPTR);
    }

    pub(crate) fn lobbyupd_msg(c: &mut Cpu, ty: u32, bc: u32) {
        lobbyupd_msg_at(c, ty, bc, DISPATCH);
    }

    /// The string the string-getter hands out (NUL-padded by the base).
    pub(crate) fn lobbyupd_string(c: &mut Cpu, s: &[u8]) {
        for (i, b) in s.iter().enumerate() {
            c.bus.write8(STRING_SRC + i as u32, *b);
        }
    }

    /// A real heap pointer from the bump pool (the free fake is a no-op,
    /// so anything it frees stays valid).
    pub(crate) fn lobbyupd_alloc(c: &mut Cpu, size: u32) -> u32 {
        c.g[A0][0] = size as u64;
        ee::s06::Lobby_MemAlloc(c);
        c.g[V0][0] as u32
    }

    pub(crate) fn lobbyupd_setup_null(c: &mut Cpu, _seed: u32) {
        lobbyupd_base(c);
        c.bus.write32(API, 0);
        lobbyupd_entry(c);
    }

    pub(crate) fn lobbyupd_setup_offl(c: &mut Cpu, _seed: u32) {
        lobbyupd_base(c);
        c.bus.write32(API, 0xC700_0001);
        c.bus.write32(API + 0x0c, ST_OFFL);
        lobbyupd_entry(c);
    }

    pub(crate) fn lobbyupd_setup_term(c: &mut Cpu, _seed: u32) {
        lobbyupd_base(c);
        c.bus.write32(API, 0xC700_0001);
        c.bus.write32(API + 0x0c, ST_CONN);
        c.bus.write32(API + 0x10, BASE_TICK - 100);
        c.bus.write32(API + 0x14, 0);
        lobbyupd_entry(c);
    }

    pub(crate) fn lobbyupd_setup_idle(c: &mut Cpu, _seed: u32) {
        lobbyupd_base(c);
        c.bus.write32(API, 0xC700_0001);
        c.bus.write32(API + 0x0c, ST_CONN);
        c.bus.write32(API + 0x10, BASE_TICK + 50);
        c.bus.write32(API + 0x14, BASE_TICK);
        c.bus.write32(API + 0x18, 0);
        lobbyupd_entry(c);
    }

    pub(crate) fn lobbyupd_setup_pingtail(c: &mut Cpu, _seed: u32) {
        lobbyupd_setup_idle(c, 0);
        c.bus.write32(API + 0x18, BASE_TICK - 1);
    }

    /// Idle with the tick delta exactly at the `0x003e9630` bound: the
    /// `< 5001` mutant would skip the tick advance.
    pub(crate) fn lobbyupd_setup_idle_tick5001(c: &mut Cpu, _seed: u32) {
        lobbyupd_setup_idle(c, 0);
        c.bus.write32(API + 0x14, BASE_TICK - 5001);
    }

    pub(crate) fn lobbyupd_setup_ping(c: &mut Cpu, _seed: u32) {
        lobbyupd_setup_idle(c, 0);
        c.bus.write32(API + 0x1c, 0xffff_ffff);
        c.bus.write32(PEEK_RET, 0);
        c.bus.write32(PEEK_TYPE, MSG_PING);
        c.bus.write32(PEEK_BC, 0);
        c.bus.write32(PEEK_PTR, TAGPTR);
    }

    pub(crate) fn lobbyupd_setup_pingtag(c: &mut Cpu, _seed: u32) {
        lobbyupd_base(c);
        lobbyupd_direct(c, PING_CHECK);
        c.g[V0][0] = 0xEA60;
        let sp = c.g[SP][0] as u32;
        c.bus.write32(sp + 0x360, MSG_PING);
        c.bus.write32(sp + 0x364, 0);
        c.bus.write32(sp + 0x368, STRING_SRC);
        c.bus.write32(API + 0x1c, 0x1234_5678);
        c.bus.write32(API + 0x10, BASE_TICK + 50);
        c.bus.write32(API + 0x14, BASE_TICK);
        lobbyupd_string(c, b"ping-tag");
    }

    pub(crate) fn lobbyupd_setup_rdir_noroot(c: &mut Cpu, _seed: u32) {
        lobbyupd_msg(c, MSG_DIR, 0);
        // The directory arm only runs in the rdir state.
        c.bus.write32(API + 0x0c, ST_RDIR);
        lobbyupd_string(c, b"rdir-cache");
    }

    pub(crate) fn lobbyupd_setup_rdir_root(c: &mut Cpu, _seed: u32) {
        lobbyupd_msg(c, MSG_DIR, 0);
        // The directory arm only runs in the rdir state.
        c.bus.write32(API + 0x0c, ST_RDIR);
        c.bus.write32(FIND, TAGPTR);
        lobbyupd_string(c, b"rdir-root");
    }

    pub(crate) fn lobbyupd_setup_skey(c: &mut Cpu, _seed: u32) {
        lobbyupd_msg(c, 0xffff_ffff, 0xffff_ffff);
    }

    pub(crate) fn lobbyupd_setup_skey2(c: &mut Cpu, _seed: u32) {
        lobbyupd_msg(c, ST_SKEY, 0);
        c.bus.write32(API + 0x0c, ST_SKEY);
    }

    pub(crate) fn lobbyupd_setup_auth(c: &mut Cpu, _seed: u32) {
        lobbyupd_msg(c, MSG_AUTH, 0);
        c.bus.write32(GETADDR, 0x0A00_0001);
        lobbyupd_string(c, b"auth-name");
    }

    pub(crate) fn lobbyupd_setup_acct(c: &mut Cpu, _seed: u32) {
        lobbyupd_msg(c, MSG_ACCT, 0);
    }

    pub(crate) fn lobbyupd_setup_snap(c: &mut Cpu, _seed: u32) {
        lobbyupd_msg(c, MSG_SNAP, 0);
        c.bus.write32(API + 0x50c, 1);
        c.bus.write32(GETNUM, 0x41);
    }

    pub(crate) fn lobbyupd_setup_sele(c: &mut Cpu, _seed: u32) {
        lobbyupd_msg(c, MSG_SELE, 0);
        c.bus.write32(GETNUM, 0x42);
    }

    pub(crate) fn lobbyupd_setup_pers(c: &mut Cpu, _seed: u32) {
        lobbyupd_msg(c, MSG_PERS, 0);
        lobbyupd_string(c, b"persona");
    }

    pub(crate) fn lobbyupd_setup_sess(c: &mut Cpu, _seed: u32) {
        lobbyupd_msg(c, MSG_SESS, 0);
        lobbyupd_string(c, b"sess-data");
    }

    pub(crate) fn lobbyupd_setup_chat(c: &mut Cpu, seed: u32) {
        // Entered at the dispatcher: DISPATCH routes `+msg` past the `+ses`
        // compare to the `+msg` arm (`0x003e9ed0`).
        lobbyupd_msg(c, MSG_MSG, 0);
        c.bus.write32(API + 0x51c, LEAF);
        c.bus
            .write32(GETFLAGS, [0u32, 4, 0x10000][seed as usize % 3]);
    }

    pub(crate) fn lobbyupd_setup_who(c: &mut Cpu, seed: u32) {
        lobbyupd_msg(c, MSG_WHO, 0);
        c.bus.write32(API + 0x324, 0xC700_0324);
        c.bus.write32(GETNUM, if seed % 2 == 0 { 99 } else { 100 });
        lobbyupd_string(c, b"who-room");
        // Take the pop loop (`0x003e9f74` via `0x003ea070`) once on odd
        // seeds, then fall out of it.
        if seed % 2 == 1 {
            c.bus.write32(HPOP_LIMIT, 2);
        }
    }

    pub(crate) fn lobbyupd_setup_rom_del(c: &mut Cpu, _seed: u32) {
        lobbyupd_msg(c, MSG_ROM, 0);
        c.bus.write32(API + 0x310, 0xC700_0310);
    }

    /// `+rom` delete with no room field but a hash hit: the find miss
    /// (`0x003ea30c`) runs the hash find, which hits and runs the
    /// `0x003ea320` delete path (kills the `|0x20` mutant there, which
    /// would clear the word instead of setting it).
    pub(crate) fn lobbyupd_setup_rom_del_found(c: &mut Cpu, _seed: u32) {
        lobbyupd_setup_rom_del(c, 0);
        c.bus.write32(HFIND, NODE);
        // A set flag bit unmasks the `0x003ea320` store: the common
        // `0x003ea350` exit re-sets bit 0x20, so a clear entry word would
        // converge there and hide a clearing mutant.
        c.bus.write32(API + 0x08, 0x40);
    }

    pub(crate) fn lobbyupd_setup_rom_add(c: &mut Cpu, _seed: u32) {
        elf_image(c);
        lobbyupd_msg(c, MSG_ROM, 0);
        c.bus.write32(API + 0x310, 0xC700_0310);
        c.bus.write32(FIND, TAGPTR);
        lobbyupd_string(c, b"rom-room");
    }

    pub(crate) fn lobbyupd_setup_pop(c: &mut Cpu, _seed: u32) {
        lobbyupd_msg(c, MSG_POP, 0);
        c.bus.write32(API + 0x310, 0xC700_0310);
        lobbyupd_string(c, b"3 7 0");
    }

    /// Full-pump variant of the `+pop` row: same scripted traffic, but
    /// entered at the top so the driven tests run the whole loop.
    pub(crate) fn lobbyupd_setup_pop_entry(c: &mut Cpu, _seed: u32) {
        lobbyupd_setup_idle(c, 0);
        c.bus.write32(API + 0x310, 0xC700_0310);
        c.bus.write32(PEEK_RET, 0);
        c.bus.write32(PEEK_TYPE, MSG_POP);
        c.bus.write32(PEEK_BC, 0);
        c.bus.write32(PEEK_PTR, TAGPTR);
        lobbyupd_string(c, b"3 7 0");
    }

    pub(crate) fn lobbyupd_setup_usr_add(c: &mut Cpu, _seed: u32) {
        lobbyupd_msg(c, MSG_USR, 0);
        c.bus.write32(API + 0x324, 0xC700_0324);
        c.bus.write32(FIND, TAGPTR);
        c.bus.write32(GETFLAGS, 0x0020_0001);
        lobbyupd_string(c, b"usr-name");
    }

    /// `+usr` add with a zero number: the `0x003ea480` test misses, so
    /// the arm continues into the add path (the `< 0` mutant would exit
    /// through `0x003ea78c` instead).
    pub(crate) fn lobbyupd_setup_usr_add_empty(c: &mut Cpu, _seed: u32) {
        lobbyupd_setup_usr_add(c, 0);
        c.bus.write32(GETNUM, 0);
        // A previous pump's flag word: the `+usr` exit sets bit 0x40, so
        // re-entering with it set exercises the set-bit side of the
        // read-modify-write flag store (kills the `|0x40` → `^0x40`
        // mutant, which would clear it).
        c.bus.write32(API + 0x08, 0x40);
    }

    pub(crate) fn lobbyupd_setup_usr_del(c: &mut Cpu, _seed: u32) {
        lobbyupd_msg(c, MSG_USR, 0);
        c.bus.write32(API + 0x324, 0xC700_0324);
        let node = lobbyupd_alloc(c, 0x138);
        c.bus.write32(HFIND, node);
        c.bus.write32(LREM, node);
        c.g[RA][0] = TEST_RA as u64;
        c.g[A0][0] = API as u64;
        c.pc = DISPATCH;
    }

    pub(crate) fn lobbyupd_setup_rnl_add(c: &mut Cpu, seed: u32) {
        // Entered at the dispatcher: DISPATCH routes `+rnk` past the `+rom`
        // compare to the room-name-list arm (`0x003ea7a0`).
        lobbyupd_msg(c, MSG_RNK, 0);
        c.bus.write32(API + 0x338, 0xC700_0338);
        lobbyupd_string(c, b"rnl-name");
        // Steer the handler's branches by seed: the find hit (`0x003ea8e4`
        // vs `0x003ea8b8`), the stored value (`0x003ea898` vs `0x003ea8a4`,
        // `0x003ea9e8` vs `0x003ea8c0`, `0x003ea9bc` vs `0x003ea8ec`) and
        // the copy direction (`0x003ea910` vs `0x003ea968`).
        match seed % 6 {
            0 => {
                c.bus.write32(GETNUM, 0);
                c.bus.write32(HFIND, NODE);
            }
            1 => {
                c.bus.write32(GETNUM, 0);
                c.bus.write32(HFIND, 0);
            }
            2 => {
                // Exactly the clamp boundary (see the snp row).
                c.bus.write32(GETNUM, 8192);
                c.bus.write32(HFIND, 0);
            }
            3 => {
                c.bus.write32(GETNUM, 5);
                c.bus.write32(HFIND, 0);
            }
            _ => {
                c.bus.write32(GETNUM, 5);
                c.bus.write32(HFIND, NODE);
                c.bus.write32(LDATA, NODE + (seed % 2) * 7);
            }
        }
    }

    pub(crate) fn lobbyupd_setup_rnl_clear(c: &mut Cpu, _seed: u32) {
        // Entered at the dispatcher, like the add row above.
        lobbyupd_msg(c, MSG_RNK, 0);
        c.bus.write32(API + 0x338, 0xC700_0338);
        c.bus.write32(GETNUM, 0xffff_fffe);
        c.bus.write32(HPOP_LIMIT, 2);
    }

    pub(crate) fn lobbyupd_setup_snp(c: &mut Cpu, _seed: u32) {
        lobbyupd_msg(c, MSG_SNP, 0);
        c.bus.write32(API + 0x508, 5);
        c.bus.write32(API + 0x374, 0xC700_0374);
        c.bus.write32(GETNUM, 6);
    }

    pub(crate) fn lobbyupd_setup_move(c: &mut Cpu, _seed: u32) {
        lobbyupd_msg(c, MSG_MOVE, 0);
        c.bus.write32(API + 0x324, 0xC700_0324);
        c.bus.write32(GETNUM, 100);
        lobbyupd_string(c, b"move-room");
        // Take the pop loop (`0x003eada0`) once, then fall out of it.
        c.bus.write32(HPOP_LIMIT, 2);
    }

    pub(crate) fn lobbyupd_setup_room(c: &mut Cpu, _seed: u32) {
        lobbyupd_msg(c, MSG_ROOM, 0);
    }

    /// Watchdog catch-up (`0x003e963c`): the saved tick (`+0x14`) is 6000
    /// behind the tick, so the handler advances the deadline (`+0x10`) by
    /// the delta before the expiry test terminates.
    pub(crate) fn lobbyupd_setup_catchup(c: &mut Cpu, _seed: u32) {
        lobbyupd_setup_term(c, 0);
        c.bus.write32(API + 0x14, BASE_TICK - 6000);
        c.bus.write32(API + 0x10, BASE_TICK - 7000);
    }

    /// `+skey` with a zero subtype: the `0x003e9954` store misses, so the
    /// `0x003e99c8` fallthrough reaches the `0x003e9a80` ladder on the
    /// equal side (`0x003e9a88`).
    pub(crate) fn lobbyupd_setup_skey_bc0(c: &mut Cpu, _seed: u32) {
        lobbyupd_msg(c, 0xffff_ffff, 0);
    }

    /// `+skey` with an `auth` subtype: the ladder is skipped at
    /// `0x003e9a88`, and the `0x003e9ac8` compare finds `auth` in the
    /// subtype slot, so the equal side (`0x003e9ad4`) terminates.
    pub(crate) fn lobbyupd_setup_skey_bcauth(c: &mut Cpu, _seed: u32) {
        lobbyupd_msg(c, 0xffff_ffff, MSG_AUTH);
    }

    /// `+skey` with a `-0x1010102` subtype: the `0x003e9a88` compare
    /// matches (the word loads sign-extended), so the ladder continues to
    /// `0x003e9a9c`, where the `conn` state matches and skips the
    /// terminate arm.
    pub(crate) fn lobbyupd_setup_skey_bcfefe(c: &mut Cpu, _seed: u32) {
        lobbyupd_msg(c, 0xffff_ffff, 0xfefe_fefe);
    }

    /// Like the row above but in the idle state: the `0x003e9a9c`
    /// compare misses, so the ladder falls into the terminate arm
    /// (`0x003e9ab0`).
    pub(crate) fn lobbyupd_setup_skey_bcfefe_idle(c: &mut Cpu, _seed: u32) {
        lobbyupd_msg(c, 0xffff_ffff, 0xfefe_fefe);
        c.bus.write32(API + 0x0c, ST_IDLE);
    }

    /// `skey2` with the callback word set: the `0x003e9a24` test finds it
    /// nonzero and calls through it (`0x003e9a54`).
    pub(crate) fn lobbyupd_setup_skey2_cb(c: &mut Cpu, _seed: u32) {
        lobbyupd_setup_skey2(c, 0);
        c.bus.write32(API + 0x52c, LEAF);
    }

    /// `sess` with its callback word set: the `0x003e9c98` tail finds it
    /// nonzero and calls through it (`0x003e9e8c`).
    pub(crate) fn lobbyupd_setup_sess_cb(c: &mut Cpu, _seed: u32) {
        lobbyupd_setup_sess(c, 0);
        c.bus.write32(API + 0x534, LEAF);
    }

    /// `+rom` add with the flag bit set: the `0x003ea218` arm runs instead
    /// of `0x003ea224`.
    pub(crate) fn lobbyupd_setup_rom_add_flags(c: &mut Cpu, _seed: u32) {
        lobbyupd_setup_rom_add(c, 0);
        c.bus.write32(GETFLAGS, 0x1_0000);
    }

    /// `+rom` add with a zero value: the `0x003ea260` arm runs instead of
    /// the copy tail.
    pub(crate) fn lobbyupd_setup_rom_add_empty(c: &mut Cpu, _seed: u32) {
        lobbyupd_setup_rom_add(c, 0);
        c.bus.write32(GETNUM, 0);
    }

    /// `+rom` add over an existing room: the find hit (`0x003ea298`)
    /// runs the list-data ladder instead of the add path.
    pub(crate) fn lobbyupd_setup_rom_add_hfind(c: &mut Cpu, _seed: u32) {
        lobbyupd_setup_rom_add(c, 0);
        c.bus.write32(HFIND, NODE);
    }

    /// `+rom` delete of an existing room: the find hit with no find word
    /// (`0x003ea320`) runs the delete path.
    pub(crate) fn lobbyupd_setup_rom_del_hfind(c: &mut Cpu, _seed: u32) {
        lobbyupd_setup_rom_del(c, 0);
        c.bus.write32(HFIND, NODE);
    }

    /// `+usr` add over an existing user: the find hit (`0x003ea5e8`)
    /// runs the update path instead of the add path.
    pub(crate) fn lobbyupd_setup_usr_add_hfind(c: &mut Cpu, _seed: u32) {
        lobbyupd_setup_usr_add(c, 0);
        c.bus.write32(HFIND, NODE);
        // A previous pump's flag word (see `lobbyupd_setup_usr_add_empty`).
        c.bus.write32(API + 0x08, 0x40);
    }

    /// `+snp` with a value at the clamp (`0x003eaaf4`): the stored value
    /// is replaced by `0x1fff` before the find.
    pub(crate) fn lobbyupd_setup_snp_big(c: &mut Cpu, _seed: u32) {
        lobbyupd_setup_snp(c, 0);
        // Exactly the clamp boundary: the `0x003eaaf4` arm runs, and the
        // `< 8192` mutant (true only at the boundary) is killed.
        c.bus.write32(GETNUM, 8192);
    }

    /// `+snp` over an existing entry: the find hit (`0x003eab58`) runs
    /// the compare ladder instead of the add path.
    pub(crate) fn lobbyupd_setup_snp_hfind(c: &mut Cpu, _seed: u32) {
        lobbyupd_setup_snp(c, 0);
        c.bus.write32(HFIND, NODE);
    }

    /// `+rom` add over an existing room whose tag byte steers the
    /// `0x003ea2bc` compare: `'7'` and `'0'` take the `0x003ea2c4` call,
    /// `':'` takes the `0x003ea2cc` fallthrough. The boundary bytes kill
    /// the `< 48` (`'0'`) and `< 58` (`':'`) mutants. The byte the
    /// `0x003ea2ac` compare reads is at S0+0x5c, where S0 is the
    /// list-data word (the `LDATA` fake's value): point it at scratch and
    /// plant the byte there (it was previously planted at `POOL+0x5c`,
    /// which nothing on this path reads).
    pub(crate) fn lobbyupd_setup_rom_add_hfind_digit(c: &mut Cpu, seed: u32) {
        lobbyupd_setup_rom_add_hfind(c, 0);
        c.bus.write32(LDATA, STRING_SRC);
        c.bus
            .write8(STRING_SRC + 0x5c, [b'7', b'0', b':'][seed as usize % 3]);
        c.bus.write8(STRING_SRC + 0x5d, 0);
    }

    /// `+rom` add with no room-list word: the `0x003ea0bc` test misses,
    /// so the arm exits through `0x003ea358` without touching the room.
    pub(crate) fn lobbyupd_setup_rom_add_no310(c: &mut Cpu, _seed: u32) {
        lobbyupd_setup_rom_add(c, 0);
        c.bus.write32(API + 0x310, 0);
    }

    /// `+usr` add with no user-list word: the `0x003ea474` test misses, so
    /// the arm exits through `0x003ea78c` without touching the user.
    pub(crate) fn lobbyupd_setup_usr_add_no324(c: &mut Cpu, _seed: u32) {
        lobbyupd_setup_usr_add(c, 0);
        c.bus.write32(API + 0x324, 0);
    }

    /// `+pop` with an empty name: the `0x003ea378` test misses, so the arm
    /// exits through `0x003ea460` without counting rooms.
    pub(crate) fn lobbyupd_setup_pop_empty(c: &mut Cpu, _seed: u32) {
        lobbyupd_msg(c, MSG_POP, 0);
        c.bus.write32(API + 0x310, 0xC700_0310);
    }

    /// `+pop` whose name starts with a colon: the digit loop's `< 0xa`
    /// tests miss on the first byte, killing those mutants, and the arm
    /// exits the same way as the empty row.
    pub(crate) fn lobbyupd_setup_pop_colon(c: &mut Cpu, _seed: u32) {
        lobbyupd_msg(c, MSG_POP, 0);
        c.bus.write32(API + 0x310, 0xC700_0310);
        lobbyupd_string(c, b":70");
    }

    /// `+pop` whose name starts with digits: the digit loop runs, and a
    /// colon in a later position takes the other side of its `< 0xa`
    /// checks (killing the boundary mutants the colon-first row never
    /// reaches). Like the plain row, it counts rooms.
    pub(crate) fn lobbyupd_setup_pop_digits(c: &mut Cpu, seed: u32) {
        lobbyupd_msg(c, MSG_POP, 0);
        c.bus.write32(API + 0x310, 0xC700_0310);
        match seed % 3 {
            0 => lobbyupd_string(c, b"7:"),
            1 => lobbyupd_string(c, b"7::"),
            _ => lobbyupd_string(c, b"7:7:"),
        }
    }

    /// `+mov` with a zero value: the `0x003eac7c` test misses, killing
    /// the `== 0` mutant, and the arm exits without counting.
    pub(crate) fn lobbyupd_setup_move_zero(c: &mut Cpu, _seed: u32) {
        lobbyupd_setup_move(c, 0);
        c.bus.write32(GETNUM, 0);
    }

    /// `+snp` with a small node value: the `0x003eaa10` test hits, killing
    /// the `== 4` mutant, and the arm falls out to dispatch.
    pub(crate) fn lobbyupd_setup_snp_v3(c: &mut Cpu, _seed: u32) {
        lobbyupd_setup_snp(c, 0);
        c.bus.write32(API + 0x508, 3);
    }

    /// `+snp` with node value 8: the `0x003eaa20` test misses, killing
    /// the `<= 8` mutant, and the arm falls out to dispatch.
    pub(crate) fn lobbyupd_setup_snp_v8(c: &mut Cpu, _seed: u32) {
        lobbyupd_setup_snp(c, 0);
        c.bus.write32(API + 0x508, 8);
    }

    /// `+snp` with node value 4: the `0x003eaa10` test misses (the `< 4`
    /// mutant would take the `0x003eac54` exit), so the arm continues
    /// into the `0x003eaa20` gate like the plain row.
    pub(crate) fn lobbyupd_setup_snp_v4(c: &mut Cpu, _seed: u32) {
        lobbyupd_setup_snp(c, 0);
        c.bus.write32(API + 0x508, 4);
    }

    /// `+usr` add over an existing user at an odd data address: the
    /// `0x003ea5e8` test takes the `0x003ea608` copy loop instead of
    /// `0x003ea664`.
    pub(crate) fn lobbyupd_setup_usr_add_hfind_odd(c: &mut Cpu, _seed: u32) {
        lobbyupd_setup_usr_add_hfind(c, 0);
        c.bus.write32(LDATA, NODE + 1);
    }

    /// `+snp` over an existing entry at an odd data address: the
    /// `0x003eab58` test takes the `0x003eab88` copy loop instead of
    /// `0x003eabe0`.
    pub(crate) fn lobbyupd_setup_snp_hfind_odd(c: &mut Cpu, _seed: u32) {
        lobbyupd_setup_snp_hfind(c, 0);
        c.bus.write32(LDATA, NODE + 1);
    }

    pub(crate) fn lobbyupd_check_null(c: &mut Cpu, _seed: u32) {
        assert_eq!(c.pc, TEST_RA);
    }

    pub(crate) fn lobbyupd_check_offl(c: &mut Cpu, _seed: u32) {
        assert_eq!(c.pc, TEST_RA);
        assert!(hits(c, N_TICK) > 0);
    }

    pub(crate) fn lobbyupd_check_term(c: &mut Cpu, _seed: u32) {
        assert_eq!(c.pc, TEST_RA);
        assert_eq!(state(c), ST_TERM);
        assert!(hits(c, N_TERM) > 0);
    }

    pub(crate) fn lobbyupd_check_idle(c: &mut Cpu, _seed: u32) {
        assert_eq!(c.pc, TEST_RA);
        assert!(hits(c, N_PINGS) > 0);
        assert!(hits(c, N_PEEK) > 0);
        assert!(hits(c, N_SORT) > 0);
        assert_eq!(hits(c, N_SEND), 0);
    }

    pub(crate) fn lobbyupd_check_idle_tick5001(c: &mut Cpu, _seed: u32) {
        lobbyupd_check_idle(c, _seed);
        // The boundary delta ran the advance arm (0x003e963c), not the
        // skip arm: +50/+5001 from the entry advance composed with the
        // message-loop tick update.
        assert_eq!(c.bus.read32(API + 0x10), BASE_TICK + 60000);
    }

    pub(crate) fn lobbyupd_check_pingtail(c: &mut Cpu, _seed: u32) {
        assert_eq!(c.pc, TEST_RA);
        assert!(hits(c, N_PINGSRV) > 0);
    }

    pub(crate) fn lobbyupd_check_ping(c: &mut Cpu, _seed: u32) {
        assert_eq!(c.pc, TEST_RA);
        assert_eq!(c.bus.read32(API + 0x10), BASE_TICK + 0xEA60);
        assert!(hits(c, N_SEND) > 0);
        assert!(hits(c, N_RECV) > 0);
    }

    /// Same auth run, but the dispatch prefix calls through the per-index
    /// callback slot (`+0x514`, arm `0x003e9748`) and, on odd seeds, through
    /// the pop callback (`0x003e9774`, via `POP_RET`). Both targets are
    /// `LEAF`, which returns 0 and touches nothing, so the outcomes match
    /// the auth row's with the pop fake also hit.
    pub(crate) fn lobbyupd_setup_cb(c: &mut Cpu, seed: u32) {
        lobbyupd_setup_auth(c, seed);
        c.bus.write32(API + 0x514, LEAF);
        c.bus.write32(API + 0x518, LEAF);
        c.bus.write32(POP_RET, if seed % 2 == 0 { 0 } else { LEAF });
    }

    pub(crate) fn lobbyupd_check_cb(c: &mut Cpu, _seed: u32) {
        assert_eq!(c.pc, TEST_RA);
        assert_eq!(state(c), ST_AUTH);
        assert_ne!(flags(c) & 2, 0);
        assert_eq!(c.bus.read32(API + 0x134), u32::from_le_bytes(*b"auth"));
        assert_eq!(c.bus.read32(API + 0x4fc), 0x0A00_0001);
        assert!(hits(c, N_POP) > 0);
    }

    pub(crate) fn lobbyupd_check_pingtag(c: &mut Cpu, _seed: u32) {
        assert_eq!(c.pc, TEST_RA);
        assert_eq!(c.bus.read32(API + 0x10), BASE_TICK + 0xEA60);
        assert!(hits(c, N_STRNZ) > 0);
        assert!(hits(c, N_SETNUM) > 0);
    }

    pub(crate) fn lobbyupd_check_rdir_noroot(c: &mut Cpu, _seed: u32) {
        assert_eq!(c.pc, TEST_RA);
        assert_ne!(flags(c) & 0x400, 0);
        assert_eq!(c.bus.read32(API + 0x3b8), u32::from_le_bytes(*b"rdir"));
    }

    pub(crate) fn lobbyupd_check_rdir_root(c: &mut Cpu, _seed: u32) {
        assert_eq!(c.pc, TEST_RA);
        assert_eq!(state(c), ST_CONN);
        assert!(hits(c, N_CONN) > 0);
        assert!(hits(c, N_PRINTF) > 0);
        assert!(hits(c, N_SEND) > 0);
        assert_eq!(c.bus.read32(API + 0x500), 0x0700_0001);
    }

    pub(crate) fn lobbyupd_check_skey(c: &mut Cpu, _seed: u32) {
        assert_eq!(c.pc, TEST_RA);
        assert_eq!(state(c), ST_SKEY);
        assert!(hits(c, N_SETBIN) > 0);
        assert!(hits(c, N_SEND) > 0);
    }

    pub(crate) fn lobbyupd_check_skey2(c: &mut Cpu, _seed: u32) {
        assert_eq!(c.pc, TEST_RA);
        assert_eq!(state(c), ST_IDLE);
        assert_ne!(flags(c) & 1, 0);
        // Rearm deadline: the tick plus the 0x7530 rearm window (not the bare tick).
        assert_eq!(c.bus.read32(API + 0x18), BASE_TICK + 0x7530);
        assert!(hits(c, N_DECHEX) > 0);
    }

    pub(crate) fn lobbyupd_check_auth(c: &mut Cpu, _seed: u32) {
        assert_eq!(c.pc, TEST_RA);
        assert_eq!(state(c), ST_AUTH);
        assert_ne!(flags(c) & 2, 0);
        assert_eq!(c.bus.read32(API + 0x134), u32::from_le_bytes(*b"auth"));
        assert_eq!(c.bus.read32(API + 0x4fc), 0x0A00_0001);
    }

    pub(crate) fn lobbyupd_check_acct(c: &mut Cpu, _seed: u32) {
        assert_eq!(c.pc, TEST_RA);
        assert_eq!(state(c), ST_ACCT);
        assert_ne!(flags(c) & 2, 0);
    }

    pub(crate) fn lobbyupd_check_snap(c: &mut Cpu, _seed: u32) {
        assert_eq!(c.pc, TEST_RA);
        assert_eq!(c.bus.read32(API + 0x508), 0x41);
    }

    pub(crate) fn lobbyupd_check_sele(c: &mut Cpu, _seed: u32) {
        assert_eq!(c.pc, TEST_RA);
        assert_eq!(c.bus.read32(API + 0x504), 0x42);
        assert_eq!(c.bus.read32(API + 0x510), 0x42);
    }

    pub(crate) fn lobbyupd_check_pers(c: &mut Cpu, _seed: u32) {
        assert_eq!(c.pc, TEST_RA);
        assert_eq!(c.bus.read32(API + 0x174), u32::from_le_bytes(*b"pers"));
        // Rearm deadline: the tick plus the 0x7530 rearm window (not the bare tick).
        assert_eq!(c.bus.read32(API + 0x18), BASE_TICK + 0x7530);
    }

    pub(crate) fn lobbyupd_check_sess(c: &mut Cpu, _seed: u32) {
        assert_eq!(c.pc, TEST_RA);
        assert_ne!(flags(c) & 0x200, 0);
        assert_eq!(c.bus.read32(API + 0x200), u32::from_le_bytes(*b"sess"));
    }

    pub(crate) fn lobbyupd_check_chat(c: &mut Cpu, _seed: u32) {
        assert_eq!(c.pc, TEST_RA);
        // The `+msg` arm names the message from the tag-flag bits and calls
        // the chat callback; it leaves the `+ses` effects alone (no chat
        // flag, no scratch clear) and never reaches the `+who` arm.
        assert_eq!(flags(c) & 0x200, 0);
        assert_eq!(room(c), 99);
        assert!(hits(c, N_GETFLAGS) > 0);
    }

    pub(crate) fn lobbyupd_check_who(c: &mut Cpu, seed: u32) {
        assert_eq!(c.pc, TEST_RA);
        if seed % 2 == 0 {
            assert_eq!(room(c), 99);
            assert_eq!(flags(c) & (8 | 0x40), 0);
        } else {
            assert_eq!(room(c), 100);
            assert_eq!(flags(c) & (8 | 0x40), 8 | 0x40);
            assert_eq!(c.bus.read32(API + 0x1c0), u32::from_le_bytes(*b"who-"));
        }
    }

    pub(crate) fn lobbyupd_check_rom_del(c: &mut Cpu, _seed: u32) {
        assert_eq!(c.pc, TEST_RA);
        assert_ne!(flags(c) & 0x20, 0);
        assert!(hits(c, N_HFINDI) > 0);
    }

    pub(crate) fn lobbyupd_check_rom_del_found(c: &mut Cpu, _seed: u32) {
        lobbyupd_check_rom_del(c, _seed);
        // The find hit runs the `0x003ea320` delete (hash-del + free),
        // not just the `0x003ea350` exit.
        assert!(hits(c, N_HDELI) > 0);
    }

    pub(crate) fn lobbyupd_check_rom_add(c: &mut Cpu, _seed: u32) {
        assert_eq!(c.pc, TEST_RA);
        assert_ne!(flags(c) & 0x20, 0);
        assert!(hits(c, N_LADD) > 0);
        assert!(hits(c, N_EA_NEW) > 0);
    }

    pub(crate) fn lobbyupd_check_pop(c: &mut Cpu, _seed: u32) {
        assert_eq!(c.pc, TEST_RA);
        assert!(hits(c, N_ROOMCNT) > 0);
    }

    pub(crate) fn lobbyupd_check_usr_add(c: &mut Cpu, _seed: u32) {
        assert_eq!(c.pc, TEST_RA);
        assert_ne!(flags(c) & 0x40, 0);
        assert_eq!(c.bus.read32(API + 0x1b4), 0x0020_0001);
    }

    pub(crate) fn lobbyupd_check_usr_add_empty(c: &mut Cpu, _seed: u32) {
        assert_eq!(c.pc, TEST_RA);
        // The zero number still runs the add path (not the `0x003ea78c`
        // exit the `< 0` mutant takes): the user flag is set.
        assert_ne!(flags(c) & 0x40, 0);
    }

    pub(crate) fn lobbyupd_check_usr_del(c: &mut Cpu, _seed: u32) {
        assert_eq!(c.pc, TEST_RA);
        assert_ne!(flags(c) & 0x40, 0);
        assert!(hits(c, N_EA_FREE) > 0);
    }

    pub(crate) fn lobbyupd_check_rnl_add(c: &mut Cpu, seed: u32) {
        assert_eq!(c.pc, TEST_RA);
        assert_ne!(flags(c) & 0x80, 0);
        // Each seed variant takes a different branch out of the find/value
        // ladder; the fake counters prove which one ran.
        match seed % 6 {
            // Find hit, stored value clear: delete path, no adds.
            0 => {
                assert!(hits(c, N_HDELS) > 0);
                assert_eq!(hits(c, N_HADDS), 0);
            }
            // Find miss, stored value clear: free path, no adds.
            1 => {
                assert_eq!(hits(c, N_HADDS), 0);
                assert_eq!(hits(c, N_LADD), 0);
                assert_eq!(hits(c, N_HDELS), 0);
            }
            // Find miss, stored value set (directly, or clamped by the
            // `>= 8192` arm): add path.
            2 | 3 => {
                assert!(hits(c, N_HADDS) > 0);
                assert!(hits(c, N_LADD) > 0);
            }
            // Find hit with flags set: copy loop, no adds.
            _ => {
                assert!(hits(c, N_LDATA) > 0);
                assert_eq!(hits(c, N_HADDS), 0);
            }
        }
    }

    pub(crate) fn lobbyupd_check_rnl_clear(c: &mut Cpu, _seed: u32) {
        assert_eq!(c.pc, TEST_RA);
        assert_ne!(flags(c) & 0x80, 0);
        assert!(hits(c, N_EA_FREE) >= 2);
    }

    pub(crate) fn lobbyupd_check_snp(c: &mut Cpu, _seed: u32) {
        assert_eq!(c.pc, TEST_RA);
        assert_ne!(flags(c) & 0x80, 0);
    }

    pub(crate) fn lobbyupd_check_move(c: &mut Cpu, _seed: u32) {
        assert_eq!(c.pc, TEST_RA);
        assert_eq!(room(c), 100);
        assert_eq!(flags(c) & (8 | 0x40), 8 | 0x40);
    }

    pub(crate) fn lobbyupd_check_room(c: &mut Cpu, _seed: u32) {
        assert_eq!(c.pc, TEST_RA);
        assert!(hits(c, N_ROOMCNT) > 0);
    }

    pub(crate) fn lobbyupd_check_catchup(c: &mut Cpu, _seed: u32) {
        assert_eq!(c.pc, TEST_RA);
        assert_eq!(state(c), ST_TERM);
        assert!(hits(c, N_TERM) > 0);
        // The catch-up arm ran: the deadline advanced by the 6000-tick
        // delta instead of staying at the planted value.
        assert_eq!(c.bus.read32(API + 0x10), BASE_TICK - 1000);
    }

    pub(crate) fn lobbyupd_check_skey_bc0(c: &mut Cpu, _seed: u32) {
        assert_eq!(c.pc, TEST_RA);
        // The `0x003e9954` store missed, so the fallthrough reached the
        // `0x003e9a80` ladder on the equal side instead of the skey-send
        // path: the state never moved to skey.
        assert_eq!(state(c), ST_CONN);
    }

    pub(crate) fn lobbyupd_check_skey_bcauth(c: &mut Cpu, _seed: u32) {
        assert_eq!(c.pc, TEST_RA);
        // The equal side terminated: the state is term, not idle.
        assert_eq!(state(c), ST_TERM);
        assert!(hits(c, N_TERM) > 0);
    }

    pub(crate) fn lobbyupd_check_skey_bcfefe(c: &mut Cpu, _seed: u32) {
        assert_eq!(c.pc, TEST_RA);
        // The ladder continued past `0x003e9a88` to `0x003e9a9c`, where the
        // `conn` state matched and skipped the terminate arm (the extra
        // arms show in the path totals, pinned below).
        assert_eq!(state(c), ST_CONN);
    }

    pub(crate) fn lobbyupd_check_skey_bcfefe_idle(c: &mut Cpu, _seed: u32) {
        assert_eq!(c.pc, TEST_RA);
        // The ladder fell into its terminate arm.
        assert_eq!(state(c), ST_TERM);
        assert!(hits(c, N_TERM) > 0);
    }

    pub(crate) fn lobbyupd_check_skey2_cb(c: &mut Cpu, _seed: u32) {
        assert_eq!(c.pc, TEST_RA);
        assert_eq!(state(c), ST_IDLE);
        assert_ne!(flags(c) & 1, 0);
        assert_eq!(c.bus.read32(API + 0x18), BASE_TICK + 0x7530);
    }

    pub(crate) fn lobbyupd_check_sess_cb(c: &mut Cpu, _seed: u32) {
        assert_eq!(c.pc, TEST_RA);
        assert_ne!(flags(c) & 0x200, 0);
        assert_eq!(c.bus.read32(API + 0x200), u32::from_le_bytes(*b"sess"));
    }

    pub(crate) fn lobbyupd_check_rom_add_flags(c: &mut Cpu, _seed: u32) {
        assert_eq!(c.pc, TEST_RA);
        assert_ne!(flags(c) & 0x20, 0);
        assert!(hits(c, N_LADD) > 0);
        assert!(hits(c, N_EA_NEW) > 0);
    }

    pub(crate) fn lobbyupd_check_rom_add_empty(c: &mut Cpu, _seed: u32) {
        assert_eq!(c.pc, TEST_RA);
        assert_ne!(flags(c) & 0x20, 0);
        assert!(hits(c, N_LADD) > 0);
        assert!(hits(c, N_EA_NEW) > 0);
    }

    pub(crate) fn lobbyupd_check_rom_add_hfind(c: &mut Cpu, _seed: u32) {
        assert_eq!(c.pc, TEST_RA);
        assert_ne!(flags(c) & 0x20, 0);
        assert!(hits(c, N_HFINDI) > 0);
    }

    pub(crate) fn lobbyupd_check_rom_del_hfind(c: &mut Cpu, _seed: u32) {
        assert_eq!(c.pc, TEST_RA);
        assert_ne!(flags(c) & 0x20, 0);
        assert!(hits(c, N_HFINDI) > 0);
    }

    pub(crate) fn lobbyupd_check_usr_add_hfind(c: &mut Cpu, _seed: u32) {
        assert_eq!(c.pc, TEST_RA);
        assert_ne!(flags(c) & 0x40, 0);
        // The `+usr` find is `HashFindInt`, like `+rom` (not `FindStr`).
        assert!(hits(c, N_HFINDI) > 0);
    }

    pub(crate) fn lobbyupd_check_snp_big(c: &mut Cpu, _seed: u32) {
        assert_eq!(c.pc, TEST_RA);
        assert_ne!(flags(c) & 0x80, 0);
    }

    pub(crate) fn lobbyupd_check_snp_hfind(c: &mut Cpu, _seed: u32) {
        assert_eq!(c.pc, TEST_RA);
        assert_ne!(flags(c) & 0x80, 0);
        assert!(hits(c, N_HFINDS) > 0);
    }

    pub(crate) fn lobbyupd_check_rom_add_hfind_digit(c: &mut Cpu, _seed: u32) {
        assert_eq!(c.pc, TEST_RA);
        assert_ne!(flags(c) & 0x20, 0);
        assert!(hits(c, N_HFINDI) > 0);
        // `'7'`/`'0'` reach the `0x003ea2c4` call, `':'` the `0x003ea2cc`
        // fallthrough (same cycle total either way); the boundary bytes
        // kill the `< 48` / `< 58` mutants, which take the other side.
    }

    pub(crate) fn lobbyupd_check_rom_add_no310(c: &mut Cpu, _seed: u32) {
        assert_eq!(c.pc, TEST_RA);
        // The arm exited before the find: no room effects at all.
        assert_eq!(flags(c) & 0x20, 0);
        assert_eq!(hits(c, N_HFINDI), 0);
    }

    pub(crate) fn lobbyupd_check_usr_add_no324(c: &mut Cpu, _seed: u32) {
        assert_eq!(c.pc, TEST_RA);
        // The arm exited before the find: no user effects at all.
        assert_eq!(flags(c) & 0x40, 0);
        assert_eq!(hits(c, N_HFINDI), 0);
    }

    pub(crate) fn lobbyupd_check_pop_empty(c: &mut Cpu, _seed: u32) {
        assert_eq!(c.pc, TEST_RA);
        // The arm exited before the name loop: no room effects at all.
        assert_eq!(flags(c) & 0x20, 0);
        assert_eq!(hits(c, N_ROOMCNT), 0);
    }

    pub(crate) fn lobbyupd_check_pop_colon(c: &mut Cpu, _seed: u32) {
        assert_eq!(c.pc, TEST_RA);
        // The digit loop bailed on the first byte: no room effects.
        assert_eq!(flags(c) & 0x20, 0);
        assert_eq!(hits(c, N_ROOMCNT), 0);
    }

    pub(crate) fn lobbyupd_check_pop_digits(c: &mut Cpu, _seed: u32) {
        assert_eq!(c.pc, TEST_RA);
        // The digit runs count rooms (like the plain row).
        assert!(hits(c, N_ROOMCNT) > 0);
    }

    pub(crate) fn lobbyupd_check_move_zero(c: &mut Cpu, _seed: u32) {
        assert_eq!(c.pc, TEST_RA);
        // The arm exited before the count: the room word stays clear.
        assert_eq!(room(c), 0);
    }

    pub(crate) fn lobbyupd_check_snp_v3(c: &mut Cpu, _seed: u32) {
        assert_eq!(c.pc, TEST_RA);
        // Fell out to dispatch before the node block: the flag stays clear.
        assert_eq!(flags(c) & 0x80, 0);
    }

    pub(crate) fn lobbyupd_check_snp_v8(c: &mut Cpu, _seed: u32) {
        assert_eq!(c.pc, TEST_RA);
        // Same fallthrough as the row above, past the second gate.
        assert_eq!(flags(c) & 0x80, 0);
    }

    pub(crate) fn lobbyupd_check_snp_v4(c: &mut Cpu, _seed: u32) {
        assert_eq!(c.pc, TEST_RA);
        // The `0x003eaa20` gate hits, so the arm runs on like the plain
        // row: the flag is set.
        assert_ne!(flags(c) & 0x80, 0);
    }

    pub(crate) fn lobbyupd_check_usr_add_hfind_odd(c: &mut Cpu, _seed: u32) {
        assert_eq!(c.pc, TEST_RA);
        assert_ne!(flags(c) & 0x40, 0);
        assert!(hits(c, N_HFINDI) > 0);
    }

    pub(crate) fn lobbyupd_check_snp_hfind_odd(c: &mut Cpu, _seed: u32) {
        assert_eq!(c.pc, TEST_RA);
        assert_ne!(flags(c) & 0x80, 0);
        assert!(hits(c, N_HFINDS) > 0);
    }

    /// Every named state: its entry head, its fixture, and what proves the
    /// state ran (a stored marker plus the fake-call counters).
    pub(crate) struct LobbyUpdRow {
        _name: &'static str,
        setup: fn(&mut Cpu, u32),
        check_row: fn(&mut Cpu, u32),
    }

    pub(crate) const LOBBYUPD_ROWS: &[LobbyUpdRow] = &[
        LobbyUpdRow {
            _name: "null",
            setup: lobbyupd_setup_null,
            check_row: lobbyupd_check_null,
        },
        LobbyUpdRow {
            _name: "offl",
            setup: lobbyupd_setup_offl,
            check_row: lobbyupd_check_offl,
        },
        LobbyUpdRow {
            _name: "term",
            setup: lobbyupd_setup_term,
            check_row: lobbyupd_check_term,
        },
        LobbyUpdRow {
            _name: "idle",
            setup: lobbyupd_setup_idle,
            check_row: lobbyupd_check_idle,
        },
        LobbyUpdRow {
            _name: "idle_tick5001",
            setup: lobbyupd_setup_idle_tick5001,
            check_row: lobbyupd_check_idle_tick5001,
        },
        LobbyUpdRow {
            _name: "pingtail",
            setup: lobbyupd_setup_pingtail,
            check_row: lobbyupd_check_pingtail,
        },
        LobbyUpdRow {
            _name: "ping",
            setup: lobbyupd_setup_ping,
            check_row: lobbyupd_check_ping,
        },
        LobbyUpdRow {
            _name: "cb",
            setup: lobbyupd_setup_cb,
            check_row: lobbyupd_check_cb,
        },
        LobbyUpdRow {
            _name: "pingtag",
            setup: lobbyupd_setup_pingtag,
            check_row: lobbyupd_check_pingtag,
        },
        LobbyUpdRow {
            _name: "rdir_noroot",
            setup: lobbyupd_setup_rdir_noroot,
            check_row: lobbyupd_check_rdir_noroot,
        },
        LobbyUpdRow {
            _name: "rdir_root",
            setup: lobbyupd_setup_rdir_root,
            check_row: lobbyupd_check_rdir_root,
        },
        LobbyUpdRow {
            _name: "skey",
            setup: lobbyupd_setup_skey,
            check_row: lobbyupd_check_skey,
        },
        LobbyUpdRow {
            _name: "skey2",
            setup: lobbyupd_setup_skey2,
            check_row: lobbyupd_check_skey2,
        },
        LobbyUpdRow {
            _name: "auth",
            setup: lobbyupd_setup_auth,
            check_row: lobbyupd_check_auth,
        },
        LobbyUpdRow {
            _name: "acct",
            setup: lobbyupd_setup_acct,
            check_row: lobbyupd_check_acct,
        },
        LobbyUpdRow {
            _name: "snap",
            setup: lobbyupd_setup_snap,
            check_row: lobbyupd_check_snap,
        },
        LobbyUpdRow {
            _name: "sele",
            setup: lobbyupd_setup_sele,
            check_row: lobbyupd_check_sele,
        },
        LobbyUpdRow {
            _name: "pers",
            setup: lobbyupd_setup_pers,
            check_row: lobbyupd_check_pers,
        },
        LobbyUpdRow {
            _name: "sess",
            setup: lobbyupd_setup_sess,
            check_row: lobbyupd_check_sess,
        },
        LobbyUpdRow {
            _name: "chat",
            setup: lobbyupd_setup_chat,
            check_row: lobbyupd_check_chat,
        },
        LobbyUpdRow {
            _name: "who",
            setup: lobbyupd_setup_who,
            check_row: lobbyupd_check_who,
        },
        LobbyUpdRow {
            _name: "rom_del",
            setup: lobbyupd_setup_rom_del,
            check_row: lobbyupd_check_rom_del,
        },
        LobbyUpdRow {
            _name: "rom_del_found",
            setup: lobbyupd_setup_rom_del_found,
            check_row: lobbyupd_check_rom_del_found,
        },
        LobbyUpdRow {
            _name: "rom_add",
            setup: lobbyupd_setup_rom_add,
            check_row: lobbyupd_check_rom_add,
        },
        LobbyUpdRow {
            _name: "pop",
            setup: lobbyupd_setup_pop,
            check_row: lobbyupd_check_pop,
        },
        LobbyUpdRow {
            _name: "usr_add",
            setup: lobbyupd_setup_usr_add,
            check_row: lobbyupd_check_usr_add,
        },
        LobbyUpdRow {
            _name: "usr_add_empty",
            setup: lobbyupd_setup_usr_add_empty,
            check_row: lobbyupd_check_usr_add_empty,
        },
        LobbyUpdRow {
            _name: "usr_del",
            setup: lobbyupd_setup_usr_del,
            check_row: lobbyupd_check_usr_del,
        },
        LobbyUpdRow {
            _name: "rnl_add",
            setup: lobbyupd_setup_rnl_add,
            check_row: lobbyupd_check_rnl_add,
        },
        LobbyUpdRow {
            _name: "rnl_clear",
            setup: lobbyupd_setup_rnl_clear,
            check_row: lobbyupd_check_rnl_clear,
        },
        LobbyUpdRow {
            _name: "snp",
            setup: lobbyupd_setup_snp,
            check_row: lobbyupd_check_snp,
        },
        LobbyUpdRow {
            _name: "move",
            setup: lobbyupd_setup_move,
            check_row: lobbyupd_check_move,
        },
        LobbyUpdRow {
            _name: "room",
            setup: lobbyupd_setup_room,
            check_row: lobbyupd_check_room,
        },
        LobbyUpdRow {
            _name: "catchup",
            setup: lobbyupd_setup_catchup,
            check_row: lobbyupd_check_catchup,
        },
        LobbyUpdRow {
            _name: "skey_bc0",
            setup: lobbyupd_setup_skey_bc0,
            check_row: lobbyupd_check_skey_bc0,
        },
        LobbyUpdRow {
            _name: "skey_bcauth",
            setup: lobbyupd_setup_skey_bcauth,
            check_row: lobbyupd_check_skey_bcauth,
        },
        LobbyUpdRow {
            _name: "skey_bcfefe",
            setup: lobbyupd_setup_skey_bcfefe,
            check_row: lobbyupd_check_skey_bcfefe,
        },
        LobbyUpdRow {
            _name: "skey_bcfefe_idle",
            setup: lobbyupd_setup_skey_bcfefe_idle,
            check_row: lobbyupd_check_skey_bcfefe_idle,
        },
        LobbyUpdRow {
            _name: "skey2_cb",
            setup: lobbyupd_setup_skey2_cb,
            check_row: lobbyupd_check_skey2_cb,
        },
        LobbyUpdRow {
            _name: "sess_cb",
            setup: lobbyupd_setup_sess_cb,
            check_row: lobbyupd_check_sess_cb,
        },
        LobbyUpdRow {
            _name: "rom_add_flags",
            setup: lobbyupd_setup_rom_add_flags,
            check_row: lobbyupd_check_rom_add_flags,
        },
        LobbyUpdRow {
            _name: "rom_add_empty",
            setup: lobbyupd_setup_rom_add_empty,
            check_row: lobbyupd_check_rom_add_empty,
        },
        LobbyUpdRow {
            _name: "rom_add_hfind",
            setup: lobbyupd_setup_rom_add_hfind,
            check_row: lobbyupd_check_rom_add_hfind,
        },
        LobbyUpdRow {
            _name: "rom_del_hfind",
            setup: lobbyupd_setup_rom_del_hfind,
            check_row: lobbyupd_check_rom_del_hfind,
        },
        LobbyUpdRow {
            _name: "usr_add_hfind",
            setup: lobbyupd_setup_usr_add_hfind,
            check_row: lobbyupd_check_usr_add_hfind,
        },
        LobbyUpdRow {
            _name: "snp_big",
            setup: lobbyupd_setup_snp_big,
            check_row: lobbyupd_check_snp_big,
        },
        LobbyUpdRow {
            _name: "snp_hfind",
            setup: lobbyupd_setup_snp_hfind,
            check_row: lobbyupd_check_snp_hfind,
        },
        LobbyUpdRow {
            _name: "rom_add_hfind_digit",
            setup: lobbyupd_setup_rom_add_hfind_digit,
            check_row: lobbyupd_check_rom_add_hfind_digit,
        },
        LobbyUpdRow {
            _name: "rom_add_no310",
            setup: lobbyupd_setup_rom_add_no310,
            check_row: lobbyupd_check_rom_add_no310,
        },
        LobbyUpdRow {
            _name: "usr_add_no324",
            setup: lobbyupd_setup_usr_add_no324,
            check_row: lobbyupd_check_usr_add_no324,
        },
        LobbyUpdRow {
            _name: "pop_empty",
            setup: lobbyupd_setup_pop_empty,
            check_row: lobbyupd_check_pop_empty,
        },
        LobbyUpdRow {
            _name: "pop_colon",
            setup: lobbyupd_setup_pop_colon,
            check_row: lobbyupd_check_pop_colon,
        },
        LobbyUpdRow {
            _name: "pop_digits",
            setup: lobbyupd_setup_pop_digits,
            check_row: lobbyupd_check_pop_digits,
        },
        LobbyUpdRow {
            _name: "move_zero",
            setup: lobbyupd_setup_move_zero,
            check_row: lobbyupd_check_move_zero,
        },
        LobbyUpdRow {
            _name: "snp_v3",
            setup: lobbyupd_setup_snp_v3,
            check_row: lobbyupd_check_snp_v3,
        },
        LobbyUpdRow {
            _name: "snp_v8",
            setup: lobbyupd_setup_snp_v8,
            check_row: lobbyupd_check_snp_v8,
        },
        LobbyUpdRow {
            _name: "snp_v4",
            setup: lobbyupd_setup_snp_v4,
            check_row: lobbyupd_check_snp_v4,
        },
        LobbyUpdRow {
            _name: "usr_add_hfind_odd",
            setup: lobbyupd_setup_usr_add_hfind_odd,
            check_row: lobbyupd_check_usr_add_hfind_odd,
        },
        LobbyUpdRow {
            _name: "snp_hfind_odd",
            setup: lobbyupd_setup_snp_hfind_odd,
            check_row: lobbyupd_check_snp_hfind_odd,
        },
    ];

    #[test]
    fn lobby_api_update_states() {
        for row in LOBBYUPD_ROWS {
            check(
                ee::s14::f_003e95c8__generated,
                lobby_api_update,
                SEEDS,
                &|c, seed| {
                    (row.setup)(c, seed);
                },
            );
            for seed in 0..SEEDS {
                let mut c = fresh();
                (row.setup)(&mut c, seed);
                ee::s14::f_003e95c8__generated(&mut c);
                (row.check_row)(&mut c, seed);
            }
        }
    }

    #[test]
    fn lobby_api_update_paths() {
        check_paths(
            ee::s14::f_003e95c8__generated,
            lobby_api_update,
            25,
            &[
                26,  // null client: entry + epilogue
                48,  // offl: entry + tick + gate + epilogue
                68,  // term: entry + tick + timeout + terminate + exit
                239, // idle: entry + tick + pings + peek(-1) + sort + exit
                252, // ping tail: idle + server ping + rearm
            ],
            &|c, seed| match seed % 5 {
                0 => lobbyupd_setup_null(c, seed),
                1 => lobbyupd_setup_offl(c, seed),
                2 => lobbyupd_setup_term(c, seed),
                3 => lobbyupd_setup_idle(c, seed),
                _ => lobbyupd_setup_pingtail(c, seed),
            },
        );
    }

    /// Path totals for the arms the new rows cover: each total differs
    /// from its sibling row's by exactly the added arm's cycles, so the
    /// pin proves the arm runs on both sides.
    #[test]
    fn lobby_api_update_new_arm_paths() {
        check_paths(
            ee::s14::f_003e95c8__generated,
            lobby_api_update,
            26,
            &[
                73,   // catchup: term plus the `0x003e963c` advance
                289,  // skey_bc0: the `0x003e9a80` ladder, skipped at `0x003e9a88`
                304,  // skey_bcauth: the ladder plus the `0x003e9ad4` terminate
                295,  // skey_bcfefe: the ladder through `0x003e9a9c`
                309,  // skey_bcfefe_idle: the ladder into `0x003e9ab0`
                349,  // skey2_cb: skey2 plus the `0x003e9a54` callback
                696,  // sess_cb: sess plus the `0x003e9e8c` callback
                2296, // rom_add_flags: the `0x003ea218` arm
                1120, // rom_add_empty: the `0x003ea260` arm
                2323, // rom_add_hfind: the `0x003ea298` ladder
                370,  // rom_del_hfind: the `0x003ea320` delete
                817,  // usr_add_hfind: the `0x003ea5e8` update
                459,  // snp_big: the `0x003eaaf4` clamp
                536,  // snp_hfind: the `0x003eab58` ladder
                // The three tag bytes take two totals now that the fixture
                // plants them where the `0x003ea2ac` compare reads: '7' and
                // '0' run the `0x003ea2c4` copy arm, ':' falls through.
                // The boundary bytes kill the `< 48` / `< 58` mutants.
                2348, // rom_add_hfind_digit '7'
                2348, // rom_add_hfind_digit '0'
                2325, // rom_add_hfind_digit ':'
                // The two missing exits and snp_v3 share one total: all
                // three fall out of dispatch after their compare.
                286, // rom_add_no310 (`0x003ea358`)
                286, // usr_add_no324 (`0x003ea78c`)
                892, // usr_add_hfind_odd: the `0x003ea608` copy loop
                578, // snp_hfind_odd: the `0x003eab88` copy loop
                313, // pop_empty: the `0x003ea460` exit
                321, // pop_colon: the digit-loop boundary exit
                524, // move_zero: the `0x003eaccc` exit
                286, // snp_v3: the `0x003eac54` fallthrough
                290, // snp_v8: the `0x003eac50` fallthrough
            ],
            &|c, seed| match seed {
                0 => lobbyupd_setup_catchup(c, seed),
                1 => lobbyupd_setup_skey_bc0(c, seed),
                2 => lobbyupd_setup_skey_bcauth(c, seed),
                3 => lobbyupd_setup_skey_bcfefe(c, seed),
                4 => lobbyupd_setup_skey_bcfefe_idle(c, seed),
                5 => lobbyupd_setup_skey2_cb(c, seed),
                6 => lobbyupd_setup_sess_cb(c, seed),
                7 => lobbyupd_setup_rom_add_flags(c, seed),
                8 => lobbyupd_setup_rom_add_empty(c, seed),
                9 => lobbyupd_setup_rom_add_hfind(c, seed),
                10 => lobbyupd_setup_rom_del_hfind(c, seed),
                11 => lobbyupd_setup_usr_add_hfind(c, seed),
                12 => lobbyupd_setup_snp_big(c, seed),
                13 => lobbyupd_setup_snp_hfind(c, seed),
                14 => lobbyupd_setup_rom_add_hfind_digit(c, 12),
                15 => lobbyupd_setup_rom_add_hfind_digit(c, 13),
                16 => lobbyupd_setup_rom_add_hfind_digit(c, 14),
                17 => lobbyupd_setup_rom_add_no310(c, seed),
                18 => lobbyupd_setup_usr_add_no324(c, seed),
                19 => lobbyupd_setup_usr_add_hfind_odd(c, seed),
                20 => lobbyupd_setup_snp_hfind_odd(c, seed),
                21 => lobbyupd_setup_pop_empty(c, seed),
                22 => lobbyupd_setup_pop_colon(c, seed),
                23 => lobbyupd_setup_move_zero(c, seed),
                24 => lobbyupd_setup_snp_v3(c, seed),
                _ => lobbyupd_setup_snp_v8(c, seed),
            },
        );
    }

    /// A second full pump from the post-first-pump state: flag words the
    /// first pump set in the client object persist, so read-modify-write
    /// flag mutants (`word|bit` → `word^bit`, which clear the bit on
    /// re-entry) diverge here even though the first pump agrees with the
    /// fixtures' clear words. The peek script is rewound so the second
    /// pump re-dispatches the same message.
    #[test]
    fn lobby_api_update_second_pump() {
        for row in LOBBYUPD_ROWS {
            check(
                ee::s14::f_003e95c8__generated,
                lobby_api_update,
                SEEDS,
                &|c, seed| {
                    (row.setup)(c, seed);
                    lobby_api_update(c);
                    c.bus.write32(PEEK_N, 0);
                    lobbyupd_entry(c);
                },
            );
        }
    }

    driven!(gen_update_driven, ee::s14::f_003e95c8__generated);

    driven!(hand_update_driven, lobby_api_update);

    pub(crate) fn lobbyupd_events_idle_setup(c: &mut Cpu, seed: u32) -> Events {
        let mut r = Rng(0x4741_5445_0000_0001 ^ seed as u64);
        lobbyupd_setup_ping(c, seed);
        schedule(c, &mut r, seed, ENTRY)
    }

    pub(crate) fn lobbyupd_events_pop_setup(c: &mut Cpu, seed: u32) -> Events {
        let mut r = Rng(0x4741_5445_0000_0002 ^ seed as u64);
        lobbyupd_setup_pop_entry(c, seed);
        schedule(c, &mut r, seed, ENTRY)
    }

    #[test]
    fn lobby_api_update_events_idle() {
        check_events(
            gen_update_driven,
            hand_update_driven,
            60,
            &lobbyupd_events_idle_setup,
        );
    }

    #[test]
    fn lobby_api_update_events_pop() {
        check_events(
            gen_update_driven,
            hand_update_driven,
            60,
            &lobbyupd_events_pop_setup,
        );
    }

    /// Every event-resume head of `LobbyApi_Update`: the `pc` an event check
    /// unwinds with, re-entered by `drive` after delivery.
    pub(crate) const EVENT_HEADS: [u32; 13] = [
        0x003e_96b0,
        0x003e_a078,
        0x003e_a3b0,
        0x003e_a3c8,
        0x003e_a418,
        0x003e_a610,
        0x003e_a66c,
        0x003e_a7d8,
        0x003e_a914,
        0x003e_a96c,
        0x003e_ab8c,
        0x003e_abe4,
        0x003e_ada0,
    ];

    /// Plant the loop state head `head` needs so its event check fires:
    /// one pop entry for the pop loops, one digit for the string scans,
    /// one 0x20 block for the copy loops.
    pub(crate) fn lobbyupd_head_plant(c: &mut Cpu, head: u32) {
        match head {
            0x003e_a078 | 0x003e_a7d8 | 0x003e_ada0 => {
                c.bus.write32(HPOP, NODE);
                c.bus.write32(HPOP_LIMIT, 1);
            }
            0x003e_a3c8 => {
                c.bus.write8(POOL, b'7');
                c.bus.write8(POOL + 1, 0);
                c.g[S0][0] = (POOL - 1) as u64;
                c.g[A2][0] = b'0' as u64;
            }
            0x003e_a418 => {
                c.bus.write8(POOL, b'7');
                c.bus.write8(POOL + 1, 0);
                c.g[S0][0] = (POOL - 1) as u64;
                c.g[A1][0] = b'0' as u64;
            }
            0x003e_a3b0 => {
                c.bus.write8(POOL, b'6');
                c.bus.write8(POOL + 1, 0);
                c.g[S0][0] = POOL as u64;
            }
            0x003e_a610 | 0x003e_a66c | 0x003e_a914 | 0x003e_a96c | 0x003e_ab8c | 0x003e_abe4 => {
                // Two blocks: the check fires on the loop-back edge, the
                // second pass falls out.
                c.g[V1][0] = POOL as u64;
                c.g[V0][0] = (POOL + 0x40) as u64;
                c.g[A0][0] = (POOL + 0x200) as u64;
            }
            _ => {}
        }
    }

    /// Enter at `head` (the message loop via its idle row, the rest by
    /// direct entry) with a VBlank due at once, so the head's own check is
    /// the first to fire and the run resumes at `head`. Heads whose check
    /// sits in a predecessor arm enter there (`a3b0`'s check is in `a454`).
    pub(crate) fn lobbyupd_head_entry(head: u32) -> u32 {
        match head {
            0x003e_a3b0 => 0x003e_a454,
            _ => head,
        }
    }

    pub(crate) fn lobbyupd_head_setup(c: &mut Cpu, seed: u32, head: u32) -> Events {
        let mut r = Rng(0x4741_5445_0000_0007 ^ seed as u64);
        if head == 0x003e_96b0 {
            lobbyupd_setup_idle(c, seed);
        } else {
            lobbyupd_base(c);
            lobbyupd_head_plant(c, head);
        }
        // Unwind programming whatever the seed says, then due at once.
        let _ = schedule(c, &mut r, 1, ENTRY);
        c.next_event = c.cycles;
        if head != 0x003e_96b0 {
            lobbyupd_direct(c, lobbyupd_head_entry(head));
        }
        Events::Unwind
    }

    #[test]
    fn lobby_api_update_event_heads() {
        for head in EVENT_HEADS {
            let resumes = check_events(gen_update_driven, hand_update_driven, 4, &|c, seed| {
                lobbyupd_head_setup(c, seed, head)
            });
            assert!(
                resumes.get(&head).copied().unwrap_or(0) > 0,
                "head {head:#x} never resumed (map: {resumes:x?})",
            );
        }
    }
}
