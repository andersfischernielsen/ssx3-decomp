//! The lobby client of `src/dirtysock/` (`LobbyApi_*`) with the NetGame link
//! and utility layers under it (`NetGameLink*`, `NetGameUtil*`) and
//! `ProtoPing*`: hand-written replacements. The reference names no file for
//! them (they sit in the anonymous `seg_2DCAE8`), so this unit is placed by
//! hand, beside `buddyapi.rs`.

use crate::abi::ret;
use crate::call::call_ptr;
use crate::call::guest;
use crate::call::Frame;
use crate::events::back_edge_unwinds;
use crate::events::deliver_event_here;
use crate::events::event_due;
use crate::guest::Addr;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::cpu::sx;
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
        'b_3e75c4: {
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
            if empty {
                break 'b_3e75c4;
            } else {
                'b_3e7580: {
                    c.cycles += 7;
                    c.g[V0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x10)).lw(c);
                    c.lwl(3, (c.g[V0][0] as u32).wrapping_add(0x3));
                    c.lwr(3, c.g[V0][0] as u32);
                    c.swl(3, frame.at(c, 0x13));
                    c.swr(3, frame.at(c, 0x10));
                    let no_out = c.g[A3][0] == 0;
                    c.g[V0][0] = c.lw(frame.at(c, 0x10));
                    if no_out {
                        break 'b_3e7580;
                    } else {
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
                        break 'b_3e7580;
                    }
                }
                'b_3e75bc: {
                    c.cycles += 2;
                    let no_len = c.g[A2][0] == 0;
                    if no_len {
                        break 'b_3e75bc;
                    } else {
                        'b_3e75b0: {
                            c.cycles += 6;
                            c.g[V0][0] = Addr(c.g[S1][0] as u32).lw(c);
                            c.g[V1][0] = Addr(c.g[A2][0] as u32).lw(c);
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x10));
                            c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[V1][0] as i64)) as u64;
                            if c.g[V0][0] == 0 {
                                c.g[A1][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x10)).lw(c);
                                break 'b_3e75b0;
                            } else {
                                c.cycles += 4;
                                c.g[V0][0] = Addr(c.g[S1][0] as u32).lw(c);
                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x10));
                                c.sw(c.g[A2][0] as u32, c.g[V0][0]);
                                c.g[A1][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x10)).lw(c);
                                break 'b_3e75b0;
                            }
                        }
                        c.cycles += 3;
                        c.g[A2][0] = Addr(c.g[A2][0] as u32).lw(c);
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x10));
                        call_ptr(c, callee::MEMCPY.addr, 0x003e_75bc)?;
                        break 'b_3e75bc;
                    }
                }
                c.cycles += 2;
                c.sw(c.g[S1][0] as u32, 0u64);
                c.sw((c.g[S1][0] as u32).wrapping_add(0x8), 0u64);
                break 'b_3e75c4;
            }
        }
        'b_3e7690: {
            c.cycles += 3;
            c.g[V0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x4)).lw(c);
            let flagged = c.g[V0][0] != 0;
            c.g[V0][0] = c.lw(frame.at(c, 0x10));
            if flagged {
                break 'b_3e7690;
            } else {
                c.cycles += 3;
                c.g[V0][0] = Addr(c.g[S1][0] as u32).lw(c);
                let pending = (c.g[V0][0] as i64) > 0;
                c.g[V0][0] = c.lw(frame.at(c, 0x10));
                if pending {
                    break 'b_3e7690;
                } else {
                    'b_3e7604: {
                        c.cycles += 3;
                        c.g[V0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x8)).lw(c);
                        let quiet = c.g[V0][0] == 0;
                        if quiet {
                            break 'b_3e7604;
                        } else {
                            c.cycles += 7;
                            callee::NET_TICK.call(c, 0x003e_75f0)?;
                            c.g[V1][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x8)).lw(c);
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
                            c.g[V0][0] = ((c.g[V0][0] as i64) < 6) as u64;
                            let fresh = c.g[V0][0] != 0;
                            c.g[V0][0] = c.lw(frame.at(c, 0x10));
                            if fresh {
                                break 'b_3e7690;
                            } else {
                                break 'b_3e7604;
                            }
                        }
                    }
                    'b_3e763c: {
                        c.cycles += 8;
                        callee::NET_TICK.call(c, 0x003e_760c)?;
                        c.g[V1][0] = 0x1;
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                        c.sw((c.g[S1][0] as u32).wrapping_add(0x4), c.g[V1][0]);
                        c.g[V0][0] = Addr(c.g[S1][0] as u32).lw(c);
                        let pending = c.g[V0][0] != 0;
                        c.g[S0][0] = RPC_REQUESTS_PAGE;
                        if pending {
                            break 'b_3e763c;
                        } else {
                            c.cycles += 6;
                            c.g[A0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x10)).lw(c);
                            c.g[A1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x7ff));
                            call_ptr(c, callee::INVALID_DCACHE.addr, 0x003e_7630)?;
                            c.g[V0][0] = 0xffff_ffff_ffff_ffff;
                            c.sw(c.g[S1][0] as u32, c.g[V0][0]);
                            c.g[S0][0] = RPC_REQUESTS_PAGE;
                            break 'b_3e763c;
                        }
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
                    break 'b_3e7690;
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
        'b_3e7a8c: {
            'b_3e7a88: {
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
                if queue_empty {
                    break 'b_3e7a88;
                }
                c.cycles += 3;
                c.g[S2][0] = 0x7069_0000;
                c.g[FP][0] = 0xffff_ffff_ffff_ffff;
                c.g[S2][0] |= 0x6e67;
                'l_3e7970: loop {
                    'b_3e7a7c: {
                        c.cycles += 4;
                        c.g[S0][0] = Addr(c.g[S1][0] as u32).lw(c);
                        c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(LOBBY_REQ_TAG)).lw(c);
                        if c.g[V0][0] != c.g[S2][0] {
                            c.g[V0][0] = Addr(c.g[S1][0] as u32).lw(c);
                            break 'b_3e7a7c;
                        }
                        'b_3e7a78: {
                            'b_3e79b4: {
                                'b_3e799c: {
                                    c.cycles += 3;
                                    c.g[V0][0] =
                                        Addr((c.g[S0][0] as u32).wrapping_add(LOBBY_PINGREQ_ADDR))
                                            .lw(c);
                                    let other_addr = c.g[V0][0] != c.g[S3][0];
                                    c.g[A0][0] = c.g[S6][0];
                                    if other_addr {
                                        break 'b_3e799c;
                                    }
                                    c.cycles += 4;
                                    c.g[A1][0] =
                                        sx((c.g[S0][0] as u32).wrapping_add(LOBBY_PINGREQ_NAME));
                                    call_ptr(c, callee::STRCMP.addr, 0x003e_7994)?;
                                    if c.g[V0][0] == 0 {
                                        c.g[A1][0] = c.g[S7][0];
                                        break 'b_3e79b4;
                                    } else {
                                        break 'b_3e799c;
                                    }
                                }
                                c.cycles += 2;
                                if c.g[S3][0] != c.g[FP][0] {
                                    c.g[V0][0] = Addr(c.g[S1][0] as u32).lw(c);
                                    break 'b_3e7a7c;
                                }
                                c.cycles += 4;
                                c.g[V0][0] =
                                    Addr((c.g[S0][0] as u32).wrapping_add(LOBBY_PINGREQ_DEADLINE))
                                        .lw(c);
                                c.g[V0][0] = ((c.g[V0][0] as i64) < (c.g[S5][0] as i64)) as u64;
                                let not_expired = c.g[V0][0] == 0;
                                c.g[A1][0] = c.g[S7][0];
                                if not_expired {
                                    break 'b_3e7a78;
                                } else {
                                    break 'b_3e79b4;
                                }
                            }
                            'b_3e7a54: {
                                'b_3e7a24: {
                                    c.cycles += 6;
                                    c.g[V1][0] =
                                        Addr((c.g[S0][0] as u32).wrapping_add(LOBBY_REQ_NEXT))
                                            .lw(c);
                                    c.g[V0][0] = c.g[S0][0] | c.g[A1][0];
                                    c.g[A2][0] = c.g[S0][0];
                                    c.g[V0][0] &= 0x7;
                                    let aligned = c.g[V0][0] == 0;
                                    c.sw(c.g[S1][0] as u32, c.g[V1][0]);
                                    if aligned {
                                        break 'b_3e7a24;
                                    }
                                    c.cycles += 1;
                                    c.g[V0][0] =
                                        sx((c.g[S0][0] as u32).wrapping_add(LOBBY_REQ_NEXT));
                                    'l_3e79d0: loop {
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
                                            continue 'l_3e79d0;
                                        }
                                        c.cycles += 2;
                                        break 'b_3e7a54;
                                    }
                                }
                                c.cycles += 1;
                                c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(LOBBY_REQ_NEXT));
                                'l_3e7a28: loop {
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
                                        continue 'l_3e7a28;
                                    }
                                    break 'b_3e7a54;
                                }
                            }
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
                            break 'b_3e7a8c;
                        }
                        c.cycles += 1;
                        c.g[V0][0] = Addr(c.g[S1][0] as u32).lw(c);
                        break 'b_3e7a7c;
                    }
                    c.cycles += 3;
                    c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(LOBBY_REQ_NEXT)).lw(c);
                    let more = c.g[V1][0] != 0;
                    c.g[S1][0] = sx((c.g[V0][0] as u32).wrapping_add(LOBBY_REQ_NEXT));
                    if more {
                        if event_due(c) {
                            deliver_event_here(c, 0x003e_7970);
                        }
                        continue 'l_3e7970;
                    }
                    break 'b_3e7a88;
                }
            }
            c.cycles += 1;
            c.g[V0][0] = 0;
            break 'b_3e7a8c;
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
                'l_3e88d4: loop {
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
                        continue 'l_3e88d4;
                    } else {
                        c.cycles += 1;
                        c.g[S0][0] = 0x14;
                        break 'l_3e88d4;
                    }
                }
            } else {
                // Unaligned: 0xa0-byte block copy with `ldl`/`ldr`.
                c.cycles += 3;
                c.g[V0][0] = sx((c.g[S2][0] as u32).wrapping_add(LOBBY_API_TABLES2));
                c.g[V1][0] = sx((c.g[A1][0] as u32).wrapping_add(0xa0));
                c.g[S2][0] = sx((c.g[A3][0] as u32).wrapping_add(0x8));
                'l_3e8874: loop {
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
                        continue 'l_3e8874;
                    } else {
                        c.cycles += 2;
                        c.g[S0][0] = 0x14;
                        break 'l_3e8874;
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
        check(
            ee::s14::f_003e7918__generated,
            lobby_api_pop_ping_request,
            84,
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
}
