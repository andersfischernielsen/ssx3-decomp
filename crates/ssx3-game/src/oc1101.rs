//! Staging module oc1101 (wave wave): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1101`.

#![allow(unused_imports)]

use crate::call::{call_ptr, guest, Callee, Frame};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

// ---------------------------------------------------------------------------
// LobbyApi object layout (all names provisional: inferred from use only).
//
// The `LobbyApi` object (`self`, `a0` at entry) carries the ProtoAries
// connection, the ProtoPing channel, per-index list tables, the request
// queue and a few callbacks. Every name below is a guess from how the
// words are read and written in this batch.

/// `+0x00`: ProtoAries connection handle (`?`).
pub(crate) const LOBBY_API_ARIES: u32 = 0x00;
/// `+0x04`: ProtoPing channel state (`?`).
pub(crate) const LOBBY_API_PING: u32 = 0x04;
/// `+0x08`: flags OR-accumulated by `Terminate` (`?`).
pub(crate) const LOBBY_API_FLAGS: u32 = 0x08;
/// `+0x0c`: connection state tag (`'term'`/`'offl'`/`'rdir'`/`'conn'`) (`?`).
pub(crate) const LOBBY_API_STATE_TAG: u32 = 0x0c;
/// `+0x10`: first `ProtoAriesTick` result slot (`?`).
pub(crate) const LOBBY_API_TICK: u32 = 0x10;
/// `+0x14`: second `ProtoAriesTick` result slot (`?`).
pub(crate) const LOBBY_API_TICK2: u32 = 0x14;
/// `+0x18`/`+0x1c`/`+0x20`: server-ping words (`?`).
pub(crate) const LOBBY_API_SERVER_PING_A: u32 = 0x18;
pub(crate) const LOBBY_API_SERVER_PING_B: u32 = 0x1c;
pub(crate) const LOBBY_API_SERVER_PING_C: u32 = 0x20;
/// `+0x34`: 0x100-byte tag/buffer used for requests and sends (`?`).
pub(crate) const LOBBY_API_TAG_BUF: u32 = 0x34;
/// `+0x310`: per-index hash-table slots (8 × 0x14) (`?`).
pub(crate) const LOBBY_API_TABLES: u32 = 0x310;
/// `+0x314`: per-index list slots parallel to `TABLES` (`?`).
pub(crate) const LOBBY_API_TABLES2: u32 = 0x314;
/// `+0x324`/`+0x328`: room/user list slots scanned by `ProcessPings` (`?`).
pub(crate) const LOBBY_API_ROOM_LIST: u32 = 0x324;
pub(crate) const LOBBY_API_USER_LIST: u32 = 0x328;
/// `+0x3b0`/`+0x3b4`: connect peer port/host (`?`).
pub(crate) const LOBBY_API_PEER_PORT: u32 = 0x3b0;
pub(crate) const LOBBY_API_PEER_HOST: u32 = 0x3b4;
/// `+0x3b8`: connected byte cleared by `Connect` (`?`).
pub(crate) const LOBBY_API_CONNECTED: u32 = 0x3b8;
/// `+0x50c`: word cleared by `Connect` (`?`).
pub(crate) const LOBBY_API_CONNECT_WORD: u32 = 0x50c;
/// `+0x514`/`+0x518`: per-index callback argument/function (`?`).
pub(crate) const LOBBY_API_CB_ARG: u32 = 0x514;
pub(crate) const LOBBY_API_CB_FUNC: u32 = 0x518;
/// `+0x52c`/`+0x530`: terminate callback and its argument (`?`).
pub(crate) const LOBBY_API_TERM_CB: u32 = 0x52c;
pub(crate) const LOBBY_API_TERM_ARG: u32 = 0x530;
/// `+0x53c`: request-queue head (`?`).
pub(crate) const LOBBY_API_REQUESTS: u32 = 0x53c;
/// `+0x540`: outstanding ping-request count (`?`).
pub(crate) const LOBBY_API_PING_COUNT: u32 = 0x540;
/// `+0x544`/`+0x548`: debug-printf argument/function (`?`).
pub(crate) const LOBBY_API_DEBUG_ARG: u32 = 0x544;
pub(crate) const LOBBY_API_DEBUG_CB: u32 = 0x548;
/// `+0x54c`: request-id counter (`?`).
pub(crate) const LOBBY_API_REQ_ID: u32 = 0x54c;

// Request-queue node layout (0x44 bytes): `+0x00` head link/count (`?`),
// `+0x04` request tag (`?`), `+0x38`/`+0x3c` request args (`?`),
// `+0x40` next link (`?`).
/// `+0x04`: request tag word (`?`).
pub(crate) const LOBBY_REQ_TAG: u32 = 0x04;
/// `+0x38`: second request word (`?`).
pub(crate) const LOBBY_REQ_ARG2: u32 = 0x38;
/// `+0x3c`: third request word (`?`).
pub(crate) const LOBBY_REQ_ARG3: u32 = 0x3c;
/// `+0x40`: next-request link (`?`).
pub(crate) const LOBBY_REQ_NEXT: u32 = 0x40;
/// Request node size.
pub(crate) const LOBBY_REQ_SIZE: u32 = 0x44;

// Ping-request node: `+0x04` must be `'ping'`, `+0x08` id (`?`),
// `+0x0c` generation (`?`), `+0x18` name (`?`), `+0x40` next (`?`).
/// `+0x08`: ping-request id (`?`).
pub(crate) const LOBBY_PINGREQ_ID: u32 = 0x08;
/// `+0x0c`: ping-request generation (`?`).
pub(crate) const LOBBY_PINGREQ_GEN: u32 = 0x0c;
/// `+0x18`: ping-request name (`?`).
pub(crate) const LOBBY_PINGREQ_NAME: u32 = 0x18;

// ---------------------------------------------------------------------------
// Direct-call targets.

pub(crate) mod callee {
    pub(crate) use crate::game::dirtysock::tags::callee::MEMCPY;
    pub(crate) use crate::game::dirtysock::tags::callee::MEMSET;
    pub(crate) use crate::game::dirtysock::tags::callee::SPRINTF;
    pub(crate) use crate::game::dirtysock::tags::callee::STRCAT;
    pub(crate) use crate::game::dirtysock::tags::callee::TAG_FIELD_FIND;
    pub(crate) use crate::game::dirtysock::tags::callee::TAG_FIELD_GET_STRING;
    pub(crate) use crate::game::dirtysock::tags::callee::TAG_FIELD_SET_NUMBER;
    pub(crate) use crate::game::dirtysock::tags::callee::TAG_FIELD_SET_STRING;
    pub(crate) use crate::game::lib::libc::callee::STRCMP;
    pub(crate) use crate::game::sound::bankmonitor::callee::STR_CASE_CMP;

    use crate::call::Callee;
    use super::ee;

    pub const DIRTY_RPC_INIT_REQUEST: Callee = Callee {
        addr: 0x003f_4c98,
        f: ee::s14::DirtyRpc_InitRequest,
    };
    pub const DIRTY_RPC_CALL: Callee = Callee {
        addr: 0x003f_4c68,
        f: ee::s14::DirtyRpc_Call,
    };
    pub const NET_RPC_FREE: Callee = Callee {
        addr: 0x0025_6bd8,
        f: ee::s06::Net_RpcFree,
    };
    pub const NET_TICK: Callee = Callee {
        addr: 0x003f_4de8,
        f: ee::s14::NetTick,
    };
    pub const PROTO_ARIES_CREATE: Callee = Callee {
        addr: 0x003f_5478,
        f: ee::s14::ProtoAriesCreate,
    };
    pub const PROTO_ARIES_DESTROY: Callee = Callee {
        addr: 0x003f_55b8,
        f: ee::s14::ProtoAriesDestroy,
    };
    pub const PROTO_ARIES_CONNECT: Callee = Callee {
        addr: 0x003f_5638,
        f: ee::s14::ProtoAriesConnect,
    };
    pub const PROTO_ARIES_UNCONNECT: Callee = Callee {
        addr: 0x003f_5738,
        f: ee::s14::ProtoAriesUnconnect,
    };
    pub const PROTO_ARIES_SEND: Callee = Callee {
        addr: 0x003f_5910,
        f: ee::s14::ProtoAriesSend,
    };
    pub const PROTO_ARIES_TICK: Callee = Callee {
        addr: 0x003f_5d88,
        f: ee::s14::ProtoAriesTick,
    };
    pub const PROTO_PING_CREATE: Callee = Callee {
        addr: 0x003e_7340,
        f: ee::s14::ProtoPingCreate,
    };
    pub const LOBBY_MEM_ALLOC: Callee = Callee {
        addr: 0x0025_91e8,
        f: ee::s06::Lobby_MemAlloc,
    };
    pub const LOBBY_MEM_FREE: Callee = Callee {
        addr: 0x0025_9210,
        f: ee::s06::Lobby_MemFree,
    };
    pub const INVALID_DCACHE: Callee = Callee {
        addr: 0x0042_47d8,
        f: ee::s15::InvalidDCache,
    };
    pub const VSPRINTF: Callee = Callee {
        addr: 0x0041_86c8,
        f: ee::s15::vsprintf,
    };
    pub const STRCHR: Callee = Callee {
        addr: 0x0041_6404,
        f: ee::s15::strchr,
    };
    pub const TAG_FIELD_GET_ADDRESS: Callee = Callee {
        addr: 0x003e_cff8,
        f: ee::s14::TagFieldGetAddress,
    };
    pub const TAG_FIELD_STRNZCPY: Callee = Callee {
        addr: 0x003e_bb60,
        f: ee::s14::TagFieldStrnzcpy,
    };
    pub const LOBBY_HASH_CREATE: Callee = Callee {
        addr: 0x003e_e778,
        f: ee::s14::Lobby_HashCreate,
    };
    pub const LOBBY_HASH_FIND_INT: Callee = Callee {
        addr: 0x003e_eee8,
        f: ee::s14::Lobby_HashFindInt,
    };
    pub const LOBBY_HASH_POP_ANY: Callee = Callee {
        addr: 0x003e_e9a0,
        f: ee::s14::Lobby_HashPopAny,
    };
    pub const LOBBY_HASH_DESTROY: Callee = Callee {
        addr: 0x003e_e820,
        f: ee::s14::Lobby_HashDestroy,
    };
    pub const LOBBY_LIST_CREATE: Callee = Callee {
        addr: 0x003e_e1e8,
        f: ee::s14::Lobby_ListCreate,
    };
    pub const LOBBY_LIST_GET_DATA: Callee = Callee {
        addr: 0x003e_e468,
        f: ee::s14::Lobby_ListGetData,
    };
    pub const LOBBY_LIST_UPDATE: Callee = Callee {
        addr: 0x003e_e3e0,
        f: ee::s14::Lobby_ListUpdate,
    };
    pub const LOBBY_LIST_GET_VISIBLE_COUNT: Callee = Callee {
        addr: 0x003e_e558,
        f: ee::s14::Lobby_ListGetVisibleCount,
    };
    pub const LOBBY_LIST_MARK_DIRTY: Callee = Callee {
        addr: 0x003e_e600,
        f: ee::s14::Lobby_ListMarkDirty,
    };
    pub const LOBBY_LIST_GET_AT: Callee = Callee {
        addr: 0x003e_e560,
        f: ee::s14::Lobby_ListGetAt,
    };
    pub const LOBBY_LIST_REMOVE: Callee = Callee {
        addr: 0x003e_e480,
        f: ee::s14::Lobby_ListRemove,
    };
    pub const LOBBY_LIST_DESTROY: Callee = Callee {
        addr: 0x003e_e260,
        f: ee::s14::Lobby_ListDestroy,
    };
    pub const LOBBY_LIST_SORT: Callee = Callee {
        addr: 0x003e_e620,
        f: ee::s14::Lobby_ListSort,
    };
    pub const LOBBY_LIST_SET_HASH: Callee = Callee {
        addr: 0x003e_e768,
        f: ee::s14::Lobby_ListSetHash,
    };
    // Own-batch siblings (no override slots yet; the readable alias is the
    // generated body, reached identically from both sides of the diff).
    pub const LOBBY_API_POP_REQUEST: Callee = Callee {
        addr: 0x003e_77f0,
        f: ee::s14::LobbyApi_PopRequest,
    };
    pub const LOBBY_API_POP_PING_REQUEST: Callee = Callee {
        addr: 0x003e_7918,
        f: ee::s14::LobbyApi_PopPingRequest,
    };
    pub const LOBBY_API_RESET: Callee = Callee {
        addr: 0x003e_80c0,
        f: ee::s14::LobbyApi_Reset,
    };
    pub const LOBBY_API_RESET_SERVER_PING: Callee = Callee {
        addr: 0x003e_7f58,
        f: ee::s14::LobbyApi_ResetServerPing,
    };
    pub const LOBBY_API_DEBUG_PRINTF: Callee = Callee {
        addr: 0x003e_76a8,
        f: ee::s14::LobbyApi_DebugPrintf,
    };
    pub const LOBBY_API_ADD_REQUEST: Callee = Callee {
        addr: 0x003e_7718,
        f: ee::s14::LobbyApi_AddRequest,
    };
    pub const LOBBY_API_FLUSH_REQUESTS: Callee = Callee {
        addr: 0x003e_7ac0,
        f: ee::s14::LobbyApi_FlushRequests,
    };
    pub const PROTO_PING_DESTROY: Callee = Callee {
        addr: 0x003e_73e0,
        f: ee::s14::ProtoPingDestroy,
    };
    pub const PROTO_PING_RESPONSE: Callee = Callee {
        addr: 0x003e_7500,
        f: ee::s14::ProtoPingResponse,
    };
}

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x003e_73e0, proto_ping_destroy, "ProtoPingDestroy"),
    (0x003e_7450, proto_ping_request, "ProtoPingRequest"),
    (0x003e_7500, proto_ping_response, "ProtoPingResponse"),
    (0x003e_76a8, lobby_api_debug_printf, "LobbyApi_DebugPrintf"),
    (0x003e_7718, lobby_api_add_request, "LobbyApi_AddRequest"),
    (0x003e_77f0, lobby_api_pop_request, "LobbyApi_PopRequest"),
    (0x003e_7918, lobby_api_pop_ping_request, "LobbyApi_PopPingRequest"),
    (0x003e_7ac0, lobby_api_flush_requests, "LobbyApi_FlushRequests"),
    (0x003e_7b00, lobby_api_update_room_count, "LobbyApi_UpdateRoomCount"),
    (0x003e_7ba0, lobby_api_sort_lists, "LobbyApi_SortLists"),
    (0x003e_7c40, lobby_api_process_pings, "LobbyApi_ProcessPings"),
    (0x003e_7f58, lobby_api_reset_server_ping, "LobbyApi_ResetServerPing"),
    (0x003e_7f70, lobby_api_terminate, "LobbyApi_Terminate"),
    (0x003e_7ff0, lobby_api_create, "LobbyApi_Create"),
    (0x003e_80c0, lobby_api_reset, "LobbyApi_Reset"),
    (0x003e_81c8, lobby_api_destroy, "LobbyApi_Destroy"),
    (0x003e_8248, lobby_api_set_callback, "LobbyApi_SetCallback"),
    (0x003e_8260, lobby_api_connect, "LobbyApi_Connect"),
    (0x003e_8368, lobby_api_select_list, "LobbyApi_SelectList"),
    (0x003e_85b0, lobby_api_create_list, "LobbyApi_CreateList"),
];

// ---------------------------------------------------------------------------
// Ports, in list order.

/// `void ProtoPingDestroy(ping)` (0x3e73e0): issue the `'ping'`/`'dest'`
/// teardown RPC carrying the word at `ping+0xc`, then free the ping state.
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
        // The teardown tag at ping+0xc, re-read from s0 after the call.
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
        callee::NET_RPC_FREE.call(c, 0x003e_7440)?;
        frame.restore(c, S0, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `u32 ProtoPingRequest(ping, a1, a2, a3)` (0x3e7450): issue the
/// `'ping'`/`'requ'` request RPC carrying the word at `ping+0xc` and return
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
        // The request tag at ping+0xc, re-read from s0 after the call.
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

/// `u32 ProtoPingResponse(ping, dst, lenp, out)` (0x3e7500): drain one
/// pending ping reply into the frame buffer (and `*out`, `*dst`), refresh
/// the ping channel when it goes quiet, and return the reply word.
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
                                c.g[V0][0] =
                                    sx((c.g[V0][0] as u32).wrapping_sub(0x10));
                                c.sw(c.g[A2][0] as u32, c.g[V0][0]);
                                c.g[A1][0] =
                                    Addr((c.g[S1][0] as u32).wrapping_add(0x10)).lw(c);
                                break 'b_3e75b0;
                            }
                        }
                        c.cycles += 3;
                        c.g[A2][0] = Addr(c.g[A2][0] as u32).lw(c);
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x10));
                        callee::MEMCPY.call(c, 0x003e_75bc)?;
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
                            c.g[V0][0] =
                                sx((c.g[V0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
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
                        c.g[S0][0] = 0x520000;
                        if pending {
                            break 'b_3e763c;
                        } else {
                            c.cycles += 6;
                            c.g[A0][0] = Addr((c.g[S1][0] as u32).wrapping_add(0x10)).lw(c);
                            c.g[A1][0] = sx((c.g[A0][0] as u32).wrapping_add(0x7ff));
                            callee::INVALID_DCACHE.call(c, 0x003e_7630)?;
                            c.g[V0][0] = 0xffff_ffff_ffff_ffff;
                            c.sw(c.g[S1][0] as u32, c.g[V0][0]);
                            c.g[S0][0] = 0x520000;
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
                    c.g[V1][0] = 0x3e00_00;
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

/// `void LobbyApi_DebugPrintf(api, fmt, ...)` (0x3e76a8): format into the
/// frame buffer, then hand it to the debug callback at `api+0x548` (with
/// the word at `api+0x544`) when one is set.
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
        callee::VSPRINTF.call(c, 0x003e_76ec)?;
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

/// `u32 LobbyApi_AddRequest(api, tag, arg2, arg3)` (0x3e7718): queue a
/// 0x44-byte request (`tag`, `arg2`, `arg3`) on the `'ping'` chain at
/// `api+0x53c`, stamping it with the id at `api+0x54c`; 0 when the
/// allocation fails.
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
        callee::LOBBY_MEM_ALLOC.call(c, 0x003e_7748)?;
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
                    c.sw(
                        (c.g[A1][0] as u32).wrapping_add(LOBBY_REQ_TAG),
                        c.g[S1][0],
                    );
                    c.sw(
                        (c.g[A1][0] as u32).wrapping_add(LOBBY_REQ_ARG2),
                        c.g[S2][0],
                    );
                    c.g[V1][0] |= 0xffff;
                    c.sw(
                        (c.g[A1][0] as u32).wrapping_add(LOBBY_REQ_ARG3),
                        c.g[S3][0],
                    );
                    c.g[A2][0] =
                        sx((c.g[S0][0] as u32).wrapping_add(LOBBY_API_REQUESTS));
                    c.sw((c.g[A1][0] as u32).wrapping_add(LOBBY_REQ_NEXT), 0u64);
                    c.g[V0][0] =
                        Addr((c.g[S0][0] as u32).wrapping_add(LOBBY_API_REQ_ID)).lw(c);
                    c.sw(c.g[A1][0] as u32, c.g[V0][0]);
                    c.g[V0][0] =
                        Addr((c.g[S0][0] as u32).wrapping_add(LOBBY_API_REQ_ID)).lw(c);
                    c.g[A0][0] =
                        Addr((c.g[S0][0] as u32).wrapping_add(LOBBY_API_REQUESTS)).lw(c);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1));
                    c.g[V0][0] &= c.g[V1][0];
                    let head_empty = c.g[A0][0] == 0;
                    c.sw((c.g[S0][0] as u32).wrapping_add(LOBBY_API_REQ_ID), c.g[V0][0]);
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
                        c.g[V1][0] = Addr(
                            (c.g[V0][0] as u32).wrapping_add(LOBBY_REQ_NEXT),
                        )
                        .lw(c);
                        let at_end = c.g[V1][0] == 0;
                        c.g[A2][0] =
                            sx((c.g[V0][0] as u32).wrapping_add(LOBBY_REQ_NEXT));
                        if at_end {
                            break 'b_3e77c0;
                        }
                        c.cycles += 3;
                        c.g[V0][0] =
                            Addr((c.g[V1][0] as u32).wrapping_add(LOBBY_REQ_TAG)).lw(c);
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
            c.sw(
                (c.g[A1][0] as u32).wrapping_add(LOBBY_REQ_NEXT),
                c.g[V0][0],
            );
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

/// `u32 LobbyApi_PopRequest(api, out, tag)` (0x3e77f0): pop the first
/// request tagged `tag` (or `'DQUE'`) off `api+0x53c` into `out`, free it,
/// and return whether any requests remain.
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
        callee::MEMSET.call(c, 0x003e_7820)?;
        c.g[A0][0] = Addr((c.g[S1][0] as u32).wrapping_add(LOBBY_API_REQUESTS)).lw(c);
        'b_3e78f8: {
            if c.g[A0][0] == 0 {
                c.g[V0][0] =
                    Addr((c.g[S1][0] as u32).wrapping_add(LOBBY_API_REQUESTS)).lw(c);
                break 'b_3e78f8;
            }
            'b_3e7844: {
                c.cycles += 3;
                c.g[V0][0] =
                    Addr((c.g[A0][0] as u32).wrapping_add(LOBBY_REQ_TAG)).lw(c);
                let tag_matches = c.g[V0][0] == c.g[S0][0];
                c.g[V0][0] = 0x4451_0000;
                if tag_matches {
                    break 'b_3e7844;
                }
                c.cycles += 3;
                c.g[V0][0] |= 0x5545;
                if c.g[S0][0] != c.g[V0][0] {
                    c.g[V0][0] = Addr(
                        (c.g[S1][0] as u32).wrapping_add(LOBBY_API_REQUESTS),
                    )
                    .lw(c);
                    break 'b_3e78f8;
                } else {
                    break 'b_3e7844;
                }
            }
            'b_3e78e8: {
                'b_3e78b8: {
                    c.cycles += 7;
                    c.g[A1][0] = c.g[S2][0];
                    c.g[V1][0] = Addr(
                        (c.g[A0][0] as u32).wrapping_add(LOBBY_REQ_NEXT),
                    )
                    .lw(c);
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
                    c.g[V0][0] =
                        sx((c.g[A0][0] as u32).wrapping_add(LOBBY_REQ_NEXT));
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
            callee::LOBBY_MEM_FREE.call(c, 0x003e_78f4)?;
            c.g[V0][0] =
                Addr((c.g[S1][0] as u32).wrapping_add(LOBBY_API_REQUESTS)).lw(c);
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

/// `u32 LobbyApi_PopPingRequest(api, out, name, id, gen)` (0x3e7918): pop
/// the first `'ping'` request off `api+0x53c` matching (`id`, `name`) — or,
/// when `id` is -1, the first older than `gen` — into `out`, and return 1
/// (0 when the queue is empty).
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
                c.g[V0][0] = Addr(
                    (c.g[S4][0] as u32).wrapping_add(LOBBY_API_REQUESTS),
                )
                .lw(c);
                let queue_empty = c.g[V0][0] == 0;
                c.g[S1][0] =
                    sx((c.g[S4][0] as u32).wrapping_add(LOBBY_API_REQUESTS));
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
                        c.g[V0][0] = Addr(
                            (c.g[S0][0] as u32).wrapping_add(LOBBY_REQ_TAG),
                        )
                        .lw(c);
                        if c.g[V0][0] != c.g[S2][0] {
                            c.g[V0][0] = Addr(c.g[S1][0] as u32).lw(c);
                            break 'b_3e7a7c;
                        }
                        'b_3e7a78: {
                            'b_3e79b4: {
                                'b_3e799c: {
                                    c.cycles += 3;
                                    c.g[V0][0] = Addr(
                                        (c.g[S0][0] as u32).wrapping_add(LOBBY_PINGREQ_ID),
                                    )
                                    .lw(c);
                                    let id_matches = c.g[V0][0] != c.g[S3][0];
                                    c.g[A0][0] = c.g[S6][0];
                                    if id_matches {
                                        break 'b_3e799c;
                                    }
                                    c.cycles += 4;
                                    c.g[A1][0] = sx(
                                        (c.g[S0][0] as u32)
                                            .wrapping_add(LOBBY_PINGREQ_NAME),
                                    );
                                    callee::STRCMP.call(c, 0x003e_7994)?;
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
                                c.g[V0][0] = Addr(
                                    (c.g[S0][0] as u32).wrapping_add(LOBBY_PINGREQ_GEN),
                                )
                                .lw(c);
                                c.g[V0][0] =
                                    ((c.g[V0][0] as i64) < (c.g[S5][0] as i64)) as u64;
                                let gen_matches = c.g[V0][0] == 0;
                                c.g[A1][0] = c.g[S7][0];
                                if gen_matches {
                                    break 'b_3e7a78;
                                } else {
                                    break 'b_3e79b4;
                                }
                            }
                            'b_3e7a54: {
                                'b_3e7a24: {
                                    c.cycles += 6;
                                    c.g[V1][0] = Addr(
                                        (c.g[S0][0] as u32).wrapping_add(LOBBY_REQ_NEXT),
                                    )
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
                                    c.g[V0][0] = sx(
                                        (c.g[S0][0] as u32).wrapping_add(LOBBY_REQ_NEXT),
                                    );
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
                                        c.g[A2][0] =
                                            sx((c.g[A2][0] as u32).wrapping_add(0x20));
                                        let more = c.g[A2][0] != c.g[V0][0];
                                        c.g[A1][0] =
                                            sx((c.g[A1][0] as u32).wrapping_add(0x20));
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
                                c.g[V0][0] = sx(
                                    (c.g[S0][0] as u32).wrapping_add(LOBBY_REQ_NEXT),
                                );
                                'l_3e7a28: loop {
                                    c.cycles += 11;
                                    c.g[V1][0] = c.ld(c.g[A2][0] as u32);
                                    c.g[A0][0] =
                                        c.ld((c.g[A2][0] as u32).wrapping_add(0x8));
                                    c.g[A3][0] =
                                        c.ld((c.g[A2][0] as u32).wrapping_add(0x10));
                                    c.g[T0][0] =
                                        c.ld((c.g[A2][0] as u32).wrapping_add(0x18));
                                    c.sd(c.g[A1][0] as u32, c.g[V1][0]);
                                    c.sd(
                                        (c.g[A1][0] as u32).wrapping_add(0x8),
                                        c.g[A0][0],
                                    );
                                    c.sd(
                                        (c.g[A1][0] as u32).wrapping_add(0x10),
                                        c.g[A3][0],
                                    );
                                    c.sd(
                                        (c.g[A1][0] as u32).wrapping_add(0x18),
                                        c.g[T0][0],
                                    );
                                    c.g[A2][0] =
                                        sx((c.g[A2][0] as u32).wrapping_add(0x20));
                                    let more = c.g[A2][0] != c.g[V0][0];
                                    c.g[A1][0] =
                                        sx((c.g[A1][0] as u32).wrapping_add(0x20));
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
                            callee::LOBBY_MEM_FREE.call(c, 0x003e_7a64)?;
                            c.g[V1][0] = Addr(
                                (c.g[S4][0] as u32).wrapping_add(LOBBY_API_PING_COUNT),
                            )
                            .lw(c);
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
                    c.g[V1][0] = Addr(
                        (c.g[V0][0] as u32).wrapping_add(LOBBY_REQ_NEXT),
                    )
                    .lw(c);
                    let more = c.g[V1][0] != 0;
                    c.g[S1][0] =
                        sx((c.g[V0][0] as u32).wrapping_add(LOBBY_REQ_NEXT));
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

/// `void LobbyApi_FlushRequests(api)` (0x3e7ac0): pop `'DQUE'` requests
/// until the queue at `api+0x53c` is empty.
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

/// `void LobbyApi_UpdateRoomCount(api, room, count)` (0x3e7b00): store
/// `count` for `room` in the list at `api+0x314` and refresh its text.
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
                c.g[A1][0] = 0x4a00_00;
                if same {
                    break 'b_3e7b84;
                }
                c.cycles += 9;
                c.sw((c.g[S0][0] as u32).wrapping_add(0x10), c.g[S2][0]);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4870));
                c.g[A2][0] = c.g[S2][0];
                c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x14));
                callee::SPRINTF.call(c, 0x003e_7b74)?;
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

/// `void LobbyApi_SortLists(api)` (0x3e7ba0): sort each of the 8 list
/// slots at `api+0x310`, notifying the per-slot comparator when a sort
/// actually reorders.
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

/// `void LobbyApi_ProcessPings(api)` (0x3e7c40): drain ping replies into
/// requests, refresh the room/user lists, then answer outstanding ping
/// requests. A dispatcher: it resumes at every block the original can
/// unwind at.
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
                    c.g[RA][0] = sx(0x003e_7c94);
                    callee::TAG_FIELD_FIND.call(c, 0x003e_7c94)?;
                    c.g[A1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x4));
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x003e_7ca0);
                    callee::TAG_FIELD_GET_ADDRESS.call(c, 0x003e_7ca0)?;
                    c.g[S5][0] = c.g[V0][0];
                    c.g[A0][0] = c.g[S4][0];
                    c.g[V0][0] = 0x4a00_00;
                    c.g[S2][0] = sx((c.g[V0][0] as u32).wrapping_add(0x4880));
                    c.g[A1][0] = c.g[S2][0];
                    c.g[RA][0] = sx(0x003e_7cb8);
                    callee::TAG_FIELD_FIND.call(c, 0x003e_7cb8)?;
                    c.g[A3][0] = 0x4a00_00;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x4888));
                    c.g[A1][0] = c.g[S3][0];
                    c.g[A2][0] = 0x20;
                    c.g[RA][0] = sx(0x003e_7cd0);
                    callee::TAG_FIELD_GET_STRING.call(c, 0x003e_7cd0)?;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[A1][0] = c.g[S6][0];
                    c.g[A2][0] = c.g[S3][0];
                    c.g[A3][0] = c.g[S5][0];
                    c.g[T0][0] = 0;
                    c.g[RA][0] = sx(0x003e_7ce8);
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
                    c.g[RA][0] = sx(0x003e_7d28);
                    callee::TAG_FIELD_SET_STRING.call(c, 0x003e_7d28)?;
                    c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x138));
                    c.g[A2][0] = 0x4a00_00;
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x4890));
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = 0x200;
                    c.g[RA][0] = sx(0x003e_7d40);
                    callee::TAG_FIELD_SET_NUMBER.call(c, 0x003e_7d40)?;
                    c.g[A2][0] = 0x4a00_00;
                    c.g[A3][0] = c.lw(c.g[SP][0] as u32);
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x4898));
                    c.g[A1][0] = 0x200;
                    c.g[RA][0] = sx(0x003e_7d58);
                    callee::TAG_FIELD_SET_NUMBER.call(c, 0x003e_7d58)?;
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x168));
                    c.g[A0][0] = c.g[S1][0];
                    c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x16c));
                    let target = c.g[V1][0] as u32;
                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x180));
                    c.g[RA][0] = sx(0x003e_7d6c);
                    call_ptr(c, target, 0x003e_7d6c)?;
                    pc = 0x003e_7d6c;
                    continue;
                }
                0x003e_7d6c => {
                    c.cycles += 3;
                    c.g[V0][0] = Addr((c.g[S1][0] as u32).wrapping_add(LOBBY_API_TABLES)).lw(c);
                    if c.g[V0][0] == 0 {
                        c.g[V0][0] =
                            Addr((c.g[S1][0] as u32).wrapping_add(LOBBY_API_ROOM_LIST)).lw(c);
                        pc = 0x003e_7de4;
                    } else {
                        pc = 0x003e_7d78;
                    }
                    continue;
                }
                0x003e_7d78 => {
                    c.cycles += 3;
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x390), 0u64);
                    c.g[S0][0] = 0x4a00_00;
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
                    c.g[RA][0] = sx(0x003e_7d9c);
                    callee::SPRINTF.call(c, 0x003e_7d9c)?;
                    c.g[A0][0] = Addr((c.g[S1][0] as u32).wrapping_add(LOBBY_API_TABLES2)).lw(c);
                    c.g[RA][0] = sx(0x003e_7da4);
                    callee::LOBBY_LIST_GET_VISIBLE_COUNT.call(c, 0x003e_7da4)?;
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
                    c.g[RA][0] = sx(0x003e_7dbc);
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
                    c.g[RA][0] = sx(0x003e_7dd4);
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
                    c.g[V0][0] = Addr((c.g[S1][0] as u32).wrapping_add(LOBBY_API_ROOM_LIST)).lw(c);
                    pc = 0x003e_7de4;
                    continue;
                }
                0x003e_7de4 => {
                    c.cycles += 2;
                    let no_users = c.g[V0][0] == 0;
                    c.g[S2][0] = 0x4a00_00;
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
                    c.g[RA][0] = sx(0x003e_7e00);
                    callee::STR_CASE_CMP.call(c, 0x003e_7e00)?;
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
                    c.g[RA][0] = sx(0x003e_7e18);
                    callee::SPRINTF.call(c, 0x003e_7e18)?;
                    c.g[A0][0] = Addr((c.g[S1][0] as u32).wrapping_add(LOBBY_API_USER_LIST)).lw(c);
                    c.g[A1][0] = 0x1;
                    c.g[RA][0] = sx(0x003e_7e24);
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
                    c.g[RA][0] = sx(0x003e_7e3c);
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
                    c.g[A0][0] = Addr((c.g[S1][0] as u32).wrapping_add(LOBBY_API_PING)).lw(c);
                    c.sw((c.g[SP][0] as u32).wrapping_add(0x390), c.g[V0][0]);
                    c.g[A1][0] = c.g[S4][0];
                    c.g[A2][0] = c.g[S7][0];
                    c.g[A3][0] = c.g[SP][0];
                    c.g[RA][0] = sx(0x003e_7e68);
                    callee::PROTO_PING_RESPONSE.call(c, 0x003e_7e68)?;
                    let replied = c.g[V0][0] != 0;
                    c.g[A1][0] = 0x4a00_00;
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
                    c.g[FP][0] = 0x4a00_00;
                    c.g[RA][0] = sx(0x003e_7e84);
                    callee::PROTO_ARIES_TICK.call(c, 0x003e_7e84)?;
                    c.g[S7][0] = 0x4a00_00;
                    c.g[S4][0] = c.g[V0][0];
                    c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x190));
                    c.g[S5][0] = 0x4a00_00;
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
                    c.g[RA][0] = sx(0x003e_7ebc);
                    callee::TAG_FIELD_SET_STRING.call(c, 0x003e_7ebc)?;
                    c.g[A3][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x138));
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = 0x200;
                    c.g[A2][0] = sx((c.g[S7][0] as u32).wrapping_add(0x4890));
                    c.g[RA][0] = sx(0x003e_7ed0);
                    callee::TAG_FIELD_SET_NUMBER.call(c, 0x003e_7ed0)?;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = 0x200;
                    c.g[A2][0] = sx((c.g[S5][0] as u32).wrapping_add(0x4898));
                    c.g[A3][0] = 0;
                    c.g[RA][0] = sx(0x003e_7ee4);
                    callee::TAG_FIELD_SET_NUMBER.call(c, 0x003e_7ee4)?;
                    c.g[V1][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x168));
                    c.g[A0][0] = c.g[S1][0];
                    c.g[A2][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x16c));
                    let target = c.g[V1][0] as u32;
                    c.g[A1][0] = sx((c.g[SP][0] as u32).wrapping_add(0x180));
                    c.g[RA][0] = sx(0x003e_7ef8);
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
                    c.g[RA][0] = sx(0x003e_7f10);
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
/// words at `api+0x18`/`+0x1c`/`+0x20` (the last to -1).
pub fn lobby_api_reset_server_ping(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        c.g[V0][0] = 0xffff_ffff_ffff_0000;
        c.sw((c.g[A0][0] as u32).wrapping_add(LOBBY_API_SERVER_PING_C), 0u64);
        c.g[V0][0] |= 0xffff;
        c.sw((c.g[A0][0] as u32).wrapping_add(LOBBY_API_SERVER_PING_A), 0u64);
        c.sw((c.g[A0][0] as u32).wrapping_add(LOBBY_API_SERVER_PING_B), c.g[V0][0]);
        let t = c.g[RA][0] as u32;
        c.pc = t;
        Ok(())
    })
}

/// `void LobbyApi_Terminate(api, a1, flags)` (0x3e7f70): mark the
/// connection `'term'`, OR `flags` in, reset the server ping, and fire the
/// terminate callback at `api+0x52c` when one is set.
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
        c.sw((c.g[S0][0] as u32).wrapping_add(LOBBY_API_STATE_TAG), c.g[V0][0]);
        c.g[V1][0] = Addr((c.g[S0][0] as u32).wrapping_add(LOBBY_API_FLAGS)).lw(c);
        c.g[V1][0] |= c.g[A2][0];
        c.sw((c.g[S0][0] as u32).wrapping_add(LOBBY_API_FLAGS), c.g[V1][0]);
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

/// `api LobbyApi_Create(name, a1, a2)` (0x3e7ff0): allocate and zero an
/// `api`, attach the ProtoAries channel and a ping channel, remember the
/// debug callback, and reset it; null when allocation fails.
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
        callee::LOBBY_MEM_ALLOC.call(c, 0x003e_801c)?;
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
                callee::MEMSET.call(c, 0x003e_8034)?;
                let no_name = c.g[S1][0] == 0;
                c.g[A0][0] = c.g[S1][0];
                if no_name {
                    break 'b_3e8078;
                }
                'b_3e8060: {
                    c.cycles += 4;
                    c.g[A1][0] = 0x3d;
                    callee::STRCHR.call(c, 0x003e_8044)?;
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
                c.g[A2][0] = 0x4a00_00;
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
            c.sw((c.g[S0][0] as u32).wrapping_add(LOBBY_API_ARIES), c.g[V0][0]);
            // A0 left stale for ProtoPingCreate, as generated.
            callee::PROTO_PING_CREATE.call(c, 0x003e_8088)?;
            c.sw((c.g[S0][0] as u32).wrapping_add(LOBBY_API_DEBUG_ARG), c.g[S2][0]);
            c.g[A0][0] = c.g[S0][0];
            c.sw((c.g[S0][0] as u32).wrapping_add(LOBBY_API_DEBUG_CB), c.g[S3][0]);
            c.sw((c.g[S0][0] as u32).wrapping_add(LOBBY_API_PING), c.g[V0][0]);
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

/// `void LobbyApi_Reset(api)` (0x3e80c0): unconnect, drop every list entry
/// and queued request, reset the server ping and the counters. A
/// dispatcher: it resumes at every block the original can unwind at.
pub fn lobby_api_reset(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
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
                    c.sw((c.g[S1][0] as u32).wrapping_add(LOBBY_API_STATE_TAG), c.g[V0][0]);
                    c.g[A0][0] = Addr(c.g[S1][0] as u32).lw(c);
                    c.g[RA][0] = sx(0x003e_810c);
                    callee::PROTO_ARIES_UNCONNECT.call(c, 0x003e_810c)?;
                    c.mult(0, 2, 18, 22);
                    pc = 0x003e_8110;
                    continue;
                }
                0x003e_8110 => {
                    c.cycles += 4;
                    c.g[V1][0] =
                        sx((c.g[V0][0] as u32).wrapping_add(c.g[S4][0] as u32));
                    c.g[V0][0] = Addr(c.g[V1][0] as u32).lw(c);
                    let empty = c.g[V0][0] == 0;
                    c.g[S3][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                    if empty {
                        pc = 0x003e_8154;
                    } else {
                        pc = 0x003e_8120;
                    }
                    continue;
                }
                0x003e_8120 => {
                    c.cycles += 2;
                    c.mult(0, 16, 18, 22);
                    pc = 0x003e_813c;
                    continue;
                }
                0x003e_8128 => {
                    c.cycles += 5;
                    c.g[A0][0] = Addr(c.g[V0][0] as u32).lw(c);
                    c.g[RA][0] = sx(0x003e_8130);
                    callee::LOBBY_LIST_REMOVE.call(c, 0x003e_8130)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x003e_8138);
                    callee::LOBBY_MEM_FREE.call(c, 0x003e_8138)?;
                    c.mult(0, 16, 18, 22);
                    pc = 0x003e_813c;
                    continue;
                }
                0x003e_813c => {
                    c.cycles += 6;
                    c.g[V0][0] =
                        sx((c.g[S4][0] as u32).wrapping_add(c.g[S0][0] as u32));
                    c.g[A0][0] = Addr(c.g[V0][0] as u32).lw(c);
                    c.g[RA][0] = sx(0x003e_8148);
                    callee::LOBBY_HASH_POP_ANY.call(c, 0x003e_8148)?;
                    c.g[A1][0] = c.g[V0][0];
                    let popped = c.g[A1][0] != 0;
                    c.g[V0][0] =
                        sx((c.g[S5][0] as u32).wrapping_add(c.g[S0][0] as u32));
                    if popped {
                        if back_edge_unwinds(c, 0x003e_8128) {
                            return Ok(());
                        }
                        pc = 0x003e_8128;
                    } else {
                        pc = 0x003e_8154;
                    }
                    continue;
                }
                0x003e_8154 => {
                    c.cycles += 4;
                    c.g[S2][0] = c.g[S3][0];
                    c.g[V0][0] = ((c.g[S2][0] as i64) < 8) as u64;
                    let more = c.g[V0][0] != 0;
                    c.mult(0, 2, 18, 22);
                    if more {
                        if back_edge_unwinds(c, 0x003e_8110) {
                            return Ok(());
                        }
                        pc = 0x003e_8110;
                    } else {
                        pc = 0x003e_8164;
                    }
                    continue;
                }
                0x003e_8164 => {
                    c.cycles += 3;
                    c.g[V0][0] = Addr(
                        (c.g[S1][0] as u32).wrapping_add(LOBBY_API_REQUESTS),
                    )
                    .lw(c);
                    if c.g[V0][0] == 0 {
                        pc = 0x003e_8190;
                    } else {
                        pc = 0x003e_8170;
                    }
                    continue;
                }
                0x003e_8170 => {
                    c.cycles += 2;
                    c.g[A0][0] = Addr(
                        (c.g[S1][0] as u32).wrapping_add(LOBBY_API_REQUESTS),
                    )
                    .lw(c);
                    pc = 0x003e_8178;
                    continue;
                }
                0x003e_8178 => {
                    c.cycles += 6;
                    c.g[V0][0] = Addr(
                        (c.g[A0][0] as u32).wrapping_add(LOBBY_REQ_NEXT),
                    )
                    .lw(c);
                    c.sw(
                        (c.g[S1][0] as u32).wrapping_add(LOBBY_API_REQUESTS),
                        c.g[V0][0],
                    );
                    c.g[RA][0] = sx(0x003e_8184);
                    callee::LOBBY_MEM_FREE.call(c, 0x003e_8184)?;
                    c.g[V0][0] = Addr(
                        (c.g[S1][0] as u32).wrapping_add(LOBBY_API_REQUESTS),
                    )
                    .lw(c);
                    if c.g[V0][0] != 0 {
                        c.g[A0][0] = Addr(
                            (c.g[S1][0] as u32).wrapping_add(LOBBY_API_REQUESTS),
                        )
                        .lw(c);
                        if back_edge_unwinds(c, 0x003e_8178) {
                            return Ok(());
                        }
                        pc = 0x003e_8178;
                    } else {
                        pc = 0x003e_8190;
                    }
                    continue;
                }
                0x003e_8190 => {
                    c.cycles += 14;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[RA][0] = sx(0x003e_8198);
                    callee::LOBBY_API_RESET_SERVER_PING.call(c, 0x003e_8198)?;
                    c.sw((c.g[S1][0] as u32).wrapping_add(LOBBY_API_REQ_ID), 0u64);
                    c.sw(
                        (c.g[S1][0] as u32).wrapping_add(LOBBY_API_PING_COUNT),
                        0u64,
                    );
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
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x003e_80c0, 0x003e_81c8);
                    return Ok(());
                }
            }
        }
    })
}

/// `void LobbyApi_Destroy(api)` (0x3e81c8): reset the `api`, destroy its 8
/// list slots, tear down the ping and Aries channels, and free it.
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
                c.g[A0][0] = Addr((c.g[S0][0] as u32).wrapping_sub(0x4)).lw(c);
                let empty = c.g[A0][0] == 0;
                c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(0x1));
                if empty {
                    break 'b_3e820c;
                }
                c.cycles += 4;
                c.g[RA][0] = sx(0x003e_8204);
                callee::LOBBY_HASH_DESTROY.call(c, 0x003e_8204)?;
                c.g[A0][0] = Addr(c.g[S0][0] as u32).lw(c);
                c.g[RA][0] = sx(0x003e_820c);
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
            c.g[A0][0] = Addr((c.g[S1][0] as u32).wrapping_add(LOBBY_API_PING)).lw(c);
            c.g[RA][0] = sx(0x003e_821c);
            callee::PROTO_PING_DESTROY.call(c, 0x003e_821c)?;
            c.g[A0][0] = Addr(c.g[S1][0] as u32).lw(c);
            c.g[RA][0] = sx(0x003e_8224);
            callee::PROTO_ARIES_DESTROY.call(c, 0x003e_8224)?;
            c.g[A0][0] = c.g[S1][0];
            c.g[RA][0] = sx(0x003e_822c);
            callee::LOBBY_MEM_FREE.call(c, 0x003e_822c)?;
            frame.restore(c, S0, 0x30);
            frame.restore(c, S1, 0x20);
            frame.restore(c, S2, 0x10);
            frame.restore_ra(c, 0);
            frame.close(c);
            return Ok(());
        }
    })
}

/// `void LobbyApi_SetCallback(api, i, func, arg)` (0x3e8248): store the
/// callback `func`/`arg` in slot `i` at `api+0x514`/`+0x518`.
pub fn lobby_api_set_callback(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        c.g[A1][0] = sx((c.g[A1][0] as u32) << 3);
        c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[A1][0] as u32));
        c.g[A0][0] = c.g[V0][0];
        c.sw(
            (c.g[V0][0] as u32).wrapping_add(LOBBY_API_CB_ARG),
            c.g[A2][0],
        );
        let t = c.g[RA][0] as u32;
        c.sw((c.g[A0][0] as u32).wrapping_add(LOBBY_API_CB_FUNC), c.g[A3][0]);
        c.pc = t;
        Ok(())
    })
}

/// `u32 LobbyApi_Connect(api, host, port, a3, t0)` (0x3e8260): flush,
/// queue the `'skey'` request, reconnect the Aries channel and send the
/// connect record; nonzero (all -1) when the queueing fails.
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
        c.sw((c.g[S2][0] as u32).wrapping_add(LOBBY_API_PEER_HOST), c.g[A1][0]);
        c.sw((c.g[S2][0] as u32).wrapping_add(LOBBY_API_PEER_PORT), c.g[A2][0]);
        callee::LOBBY_API_FLUSH_REQUESTS.call(c, 0x003e_828c)?;
        c.sw((c.g[S2][0] as u32).wrapping_add(LOBBY_API_CONNECT_WORD), 0u64);
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
            c.g[RA][0] = sx(0x003e_82c0);
            callee::PROTO_ARIES_UNCONNECT.call(c, 0x003e_82c0)?;
            c.g[A1][0] = Addr((c.g[S2][0] as u32).wrapping_add(LOBBY_API_PEER_HOST)).lw(c);
            c.g[A0][0] = Addr(c.g[S2][0] as u32).lw(c);
            c.g[A2][0] = Addr((c.g[S2][0] as u32).wrapping_add(LOBBY_API_PEER_PORT)).lw(c);
            c.g[RA][0] = sx(0x003e_82d0);
            callee::PROTO_ARIES_CONNECT.call(c, 0x003e_82d0)?;
            c.g[A3][0] = Addr((c.g[S2][0] as u32).wrapping_add(LOBBY_API_PEER_PORT)).lw(c);
            c.g[A1][0] = 0x4900_00;
            c.g[A2][0] = Addr((c.g[S2][0] as u32).wrapping_add(LOBBY_API_PEER_HOST)).lw(c);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x5f30));
            c.g[A0][0] = c.g[S2][0];
            c.g[RA][0] = sx(0x003e_82e8);
            callee::LOBBY_API_DEBUG_PRINTF.call(c, 0x003e_82e8)?;
            c.g[A0][0] = c.g[SP][0];
            c.g[A1][0] = 0x100;
            c.g[A2][0] = sx((c.g[S2][0] as u32).wrapping_add(LOBBY_API_TAG_BUF));
            c.g[RA][0] = sx(0x003e_82f8);
            callee::TAG_FIELD_STRNZCPY.call(c, 0x003e_82f8)?;
            c.g[A0][0] = Addr(c.g[S2][0] as u32).lw(c);
            c.g[A1][0] = 0x4064_0000;
            c.g[T0][0] = 0xffff_ffff_ffff_ffff;
            c.g[A2][0] = 0;
            c.g[A1][0] |= 0x6972;
            c.g[A3][0] = c.g[SP][0];
            c.g[RA][0] = sx(0x003e_8314);
            callee::PROTO_ARIES_SEND.call(c, 0x003e_8314)?;
            c.g[V1][0] = 0x7264_0000;
            c.g[A0][0] = Addr(c.g[S2][0] as u32).lw(c);
            c.g[V1][0] |= 0x6972;
            c.sw((c.g[S2][0] as u32).wrapping_add(LOBBY_API_STATE_TAG), c.g[V1][0]);
            c.g[RA][0] = sx(0x003e_8328);
            callee::PROTO_ARIES_TICK.call(c, 0x003e_8328)?;
            c.g[V1][0] = 0x1_0000;
            c.g[A0][0] = Addr(c.g[S2][0] as u32).lw(c);
            c.g[V1][0] |= 0xd4c0;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.sw((c.g[S2][0] as u32).wrapping_add(LOBBY_API_TICK), c.g[V0][0]);
            c.g[RA][0] = sx(0x003e_8340);
            callee::PROTO_ARIES_TICK.call(c, 0x003e_8340)?;
            c.sw((c.g[S2][0] as u32).wrapping_add(LOBBY_API_TICK2), c.g[V0][0]);
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

/// `u32 LobbyApi_SelectList(api, i, select)` (0x3e8368): with `select`
/// set, create list slot `i` (and announce it); with it clear, tear the
/// slot down. Returns the slot's list. A dispatcher: it resumes at every
/// block the original can unwind at.
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
                    c.g[V0][0] = 0x4a00_00;
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
                    c.g[V0][0] = 0x4a00_00;
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
                    c.g[V0][0] = 0x4a00_00;
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
                    c.g[S0][0] =
                        sx((c.g[S3][0] as u32).wrapping_add(c.g[S1][0] as u32));
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
                    c.g[RA][0] = sx(0x003e_842c);
                    callee::LOBBY_HASH_CREATE.call(c, 0x003e_842c)?;
                    c.g[A0][0] = 0x64;
                    c.sw(c.g[S0][0] as u32, c.g[V0][0]);
                    c.g[A1][0] = 0x64;
                    c.g[A2][0] = 0;
                    c.g[RA][0] = sx(0x003e_8440);
                    callee::LOBBY_LIST_CREATE.call(c, 0x003e_8440)?;
                    c.g[A0][0] =
                        sx((c.g[S5][0] as u32).wrapping_add(c.g[S1][0] as u32));
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
                    c.g[A1][0] = 0x4a00_00;
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
                    c.g[RA][0] = sx(0x003e_8488);
                    callee::STRCAT.call(c, 0x003e_8488)?;
                    c.g[A1][0] = 0x7365_0000;
                    c.g[A0][0] = Addr(c.g[S4][0] as u32).lw(c);
                    c.g[A1][0] |= 0x6c65;
                    c.g[A2][0] = 0;
                    c.g[A3][0] = c.g[SP][0];
                    c.g[T0][0] = 0xffff_ffff_ffff_ffff;
                    c.g[RA][0] = sx(0x003e_84a4);
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
                    c.g[A0][0] =
                        sx((c.g[S3][0] as u32).wrapping_add(c.g[V1][0] as u32));
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
                    c.g[S0][0] =
                        sx((c.g[S5][0] as u32).wrapping_add(c.g[V1][0] as u32));
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
                    c.g[RA][0] = sx(0x003e_84d8);
                    callee::LOBBY_LIST_REMOVE.call(c, 0x003e_84d8)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[RA][0] = sx(0x003e_84e0);
                    callee::LOBBY_MEM_FREE.call(c, 0x003e_84e0)?;
                    pc = 0x003e_84e0;
                    continue;
                }
                0x003e_84e0 => {
                    c.cycles += 4;
                    c.g[A0][0] = Addr(c.g[S1][0] as u32).lw(c);
                    c.g[RA][0] = sx(0x003e_84e8);
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
                    c.g[S1][0] =
                        sx((c.g[S3][0] as u32).wrapping_add(c.g[S0][0] as u32));
                    c.g[A0][0] = Addr(c.g[S1][0] as u32).lw(c);
                    c.g[S0][0] =
                        sx((c.g[S5][0] as u32).wrapping_add(c.g[S0][0] as u32));
                    c.g[RA][0] = sx(0x003e_8508);
                    callee::LOBBY_HASH_DESTROY.call(c, 0x003e_8508)?;
                    c.sw(c.g[S1][0] as u32, 0u64);
                    c.g[A0][0] = Addr(c.g[S0][0] as u32).lw(c);
                    c.g[RA][0] = sx(0x003e_8514);
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
                    c.g[A1][0] = 0x4a00_00;
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
                    c.g[RA][0] = sx(0x003e_8558);
                    callee::STRCAT.call(c, 0x003e_8558)?;
                    c.g[A1][0] = 0x7365_0000;
                    c.g[A0][0] = Addr(c.g[S4][0] as u32).lw(c);
                    c.g[A1][0] |= 0x6c65;
                    c.g[A2][0] = 0;
                    c.g[A3][0] = c.g[SP][0];
                    c.g[T0][0] = 0xffff_ffff_ffff_ffff;
                    c.g[RA][0] = sx(0x003e_8574);
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
                    c.g[V1][0] =
                        sx((c.g[V0][0] as u32).wrapping_add(c.g[S5][0] as u32));
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

/// `u32 LobbyApi_CreateList(api, i, cmp, arg)` (0x3e85b0): create the
/// hash/list pair for slot `i`, remembering `cmp`/`arg`; the list, or 0
/// for an out-of-range slot.
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
            c.sw((c.g[V0][0] as u32).wrapping_add(0x318), c.g[A2][0]);
            c.sw((c.g[V1][0] as u32).wrapping_add(0x31c), c.g[A3][0]);
            c.g[RA][0] = sx(0x003e_85f8);
            callee::LOBBY_HASH_CREATE.call(c, 0x003e_85f8)?;
            c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(LOBBY_API_TABLES));
            c.g[A2][0] = 0;
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[S2][0] as u32));
            c.g[A0][0] = 0x64;
            c.g[A1][0] = 0x64;
            c.sw(c.g[S1][0] as u32, c.g[V0][0]);
            c.g[RA][0] = sx(0x003e_8614);
            callee::LOBBY_LIST_CREATE.call(c, 0x003e_8614)?;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(LOBBY_API_TABLES2));
            c.g[A0][0] = c.g[V0][0];
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[S2][0] as u32));
            c.sw(c.g[S0][0] as u32, c.g[V0][0]);
            c.g[A1][0] = Addr(c.g[S1][0] as u32).lw(c);
            c.g[RA][0] = sx(0x003e_862c);
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::events::{check_events, driven, schedule, Events};
    use crate::difftest::fakes::{self, machine, FAKE};
    use crate::difftest::{check, check_lohi, check_paths, elf_image, Rng, DATA};
    use crate::difftest::RA as TEST_RA;
    use ssx3_ee_rt::funcs::GuestFn;
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    // Fixture regions (clear of the fake allocator's 0x800000-0x83fff0
    // range below and its call log at 0x900000 above).
    pub(crate) const LOBBY: u32 = DATA + 0x41_000;
    pub(crate) const LOBBY_BUF: u32 = DATA + 0x42_000;
    pub(crate) const LOBBY_NODE: u32 = DATA + 0x43_000;
    pub(crate) const LOBBY_REC: u32 = DATA + 0x44_000;
    pub(crate) const LOBBY_TAB: u32 = DATA + 0x45_000;
    pub(crate) const LOBBY_OUT: u32 = DATA + 0x46_000;
    pub(crate) const LOBBY_STR: u32 = DATA + 0x47_000;

    /// Gate of the allocator fakes (+4: make the allocation fail).
    pub(crate) const LOBBY_ALLOC_CTRL: u32 = DATA + 0x40_000;
    pub(crate) const LOBBY_ALLOC_MAGIC: u32 = 0x10B1_0C01;
    pub(crate) const LOBBY_ALLOC_FAIL: u32 = LOBBY_ALLOC_CTRL + 4;

    /// Event setup that never unwinds. Structured ports cannot resume
    /// mid-call (a callee-head re-entry reruns the whole port), so a
    /// schedule-Unwind event inside `memset`/`strcmp`/nested ports would
    /// stop the drive trampoline instead of testing anything. Convert
    /// Unwind seeds to Deliver: delivery + rescheduling still exercise
    /// every head check on both sides, with no re-entry anywhere.
    pub(crate) fn schedule_no_unwind(
        c: &mut Cpu,
        r: &mut Rng,
        seed: u32,
        entry: u32,
    ) -> Events {
        let mode = schedule(c, r, seed, entry);
        if mode == Events::Unwind {
            ssx3_ps2::platform::machine_of(&mut c.bus.plat)
                .sched
                .next_vblank = 1 << 40;
            return Events::Deliver;
        }
        mode
    }

    pub(crate) static LOBBY_PREV_NEW: AtomicUsize = AtomicUsize::new(0);
    pub(crate) static LOBBY_PREV_FREE: AtomicUsize = AtomicUsize::new(0);

    /// `EA_MemAllocNamed` replacement for tests: the logging fake (seeded
    /// forced results through `LOG_RESULTS`, hash pointers otherwise),
    /// failing when the fixture asks for it. Reusable by the later
    /// `LobbyApi_Update` work that needs a deterministic heap: set the
    /// magic, allocate, clear it.
    pub(crate) fn lobby_fake_new(c: &mut Cpu) {
        if c.bus.read32(LOBBY_ALLOC_CTRL) != LOBBY_ALLOC_MAGIC {
            return fakes::fall_back(c, &LOBBY_PREV_NEW, ee::s10::f_00317e30__generated);
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0x0031_7e30, ra);
        if c.pc == ra && c.bus.read32(LOBBY_ALLOC_FAIL) != 0 {
            c.g[V0][0] = 0;
        }
    }

    /// `EA_MemFree` replacement for tests: the logging fake.
    pub(crate) fn lobby_fake_free(c: &mut Cpu) {
        if c.bus.read32(LOBBY_ALLOC_CTRL) != LOBBY_ALLOC_MAGIC {
            return fakes::fall_back(c, &LOBBY_PREV_FREE, ee::s10::f_00317e98__generated);
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0x0031_7e98, ra);
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
    }

    /// Scrambled machine with game data and the gated heap fakes.
    pub(crate) fn lobby_base(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        let r = machine(c, seed, salt, calls);
        elf_image(c);
        lobby_install_fakes();
        c.bus.write32(LOBBY_ALLOC_CTRL, LOBBY_ALLOC_MAGIC);
        c.bus.write32(LOBBY_ALLOC_FAIL, 0);
        r
    }

    pub(crate) fn lobby_zero(c: &mut Cpu, at: u32, words: u32) {
        for i in 0..words {
            c.bus.write32(at + 4 * i, 0);
        }
    }

    /// Non-zero filler (output buffers must show a missing store).
    pub(crate) fn lobby_nz(c: &mut Cpu, seed: u32, at: u32, words: u32) {
        for i in 0..words {
            let w = 0xA5A5_0001u32
                .wrapping_add((seed + 1).wrapping_mul(0x0101_0101))
                .wrapping_add(i.wrapping_mul(0x0011_0011));
            c.bus.write32(at + 4 * i, w);
        }
    }

    pub(crate) fn lobby_str(c: &mut Cpu, at: u32, s: &[u8]) {
        for (i, b) in s.iter().enumerate() {
            c.bus.write8(at + i as u32, *b);
        }
        c.bus.write8(at + s.len() as u32, 0);
    }

    /// Run a generated function in a fixture (for building valid records
    /// and reading back addresses the test then seeds).
    pub(crate) fn lobby_call(c: &mut Cpu, f: GuestFn, a0: u64, a1: u64, a2: u64, a3: u64) -> u64 {
        c.g[A0][0] = a0;
        c.g[A1][0] = a1;
        c.g[A2][0] = a2;
        c.g[A3][0] = a3;
        c.g[RA][0] = TEST_RA as u64;
        f(c);
        assert_eq!(c.pc, TEST_RA, "setup call did not return");
        c.g[V0][0]
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
            },
        );
    }

    #[test]
    fn proto_ping_response_matches() {
        check(
            ee::s14::f_003e7500__generated,
            proto_ping_response,
            40,
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
                    let tick = lobby_call(c, ee::s14::NetTick, 0, 0, 0, 0);
                    let v = match seed / 6 % 3 {
                        0 => 0,
                        1 => (tick as u32).wrapping_sub(3),
                        _ => (tick as u32).wrapping_sub(100),
                    };
                    c.bus.write32(LOBBY + 8, v);
                } else {
                    c.bus.write32(LOBBY + 8, 0x5555_0000 + seed);
                }
                // Data record: non-zero reply word first.
                lobby_nz(c, seed, LOBBY_REC, 64);
                c.bus.write32(LOBBY_REC, 0x2468_0001 + (seed & 0xffff));
                c.bus.write32(LOBBY + 0x10, LOBBY_REC);
                c.bus.write32(LOBBY + 0xc, 0x3000 + seed);
                lobby_nz(c, seed + 1, LOBBY_OUT, 64);
                c.g[A0][0] = LOBBY as u64;
                c.g[A1][0] = LOBBY_OUT as u64;
                if seed / 18 % 2 == 0 {
                    c.g[A2][0] = 0;
                } else {
                    c.g[A2][0] = (LOBBY_OUT + 0x100) as u64;
                    let len = match seed / 9 % 2 {
                        0 => 4,
                        _ => 0x1000,
                    };
                    c.bus.write32(LOBBY_OUT + 0x100, len);
                }
                if seed / 36 % 2 == 0 {
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
                lobby_nz(c, seed, LOBBY_OUT, 20);
                // Head: empty, matching tag, 'DQUE', other tag.
                let tag = match seed % 4 {
                    1 => 0x4451_5545,
                    2 => 0x4451_5545,
                    3 => 0x1111_1111,
                    _ => 0,
                };
                if seed % 4 != 0 {
                    lobby_req_chain(c, &[tag]);
                    c.bus.write32(LOBBY + LOBBY_API_REQUESTS, LOBBY_NODE);
                    if seed / 24 % 2 == 1 {
                        // Trailer so something remains.
                        c.bus.write32(LOBBY_NODE + LOBBY_REQ_NEXT, LOBBY_NODE + 0x44);
                        c.bus.write32(LOBBY_NODE + 0x44 + LOBBY_REQ_TAG, 0x4451_5545);
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

    #[test]
    fn lobby_api_pop_request_events() {
        driven!(gen_pop_request, ee::s14::f_003e77f0__generated);
        driven!(hand_pop_request, lobby_api_pop_request);
        check_events(gen_pop_request, hand_pop_request, 16, &|c, seed| {
            let mut r = lobby_base(c, seed, 0x3e77_f1, 0);
            lobby_zero(c, LOBBY, 0x160);
            lobby_nz(c, seed, LOBBY_OUT, 20);
            lobby_req_chain(c, &[0x4451_5545]);
            c.bus.write32(LOBBY + LOBBY_API_REQUESTS, LOBBY_NODE);
            c.g[A0][0] = LOBBY as u64;
            c.g[A1][0] = if seed % 2 == 0 {
                LOBBY_OUT as u64
            } else {
                (LOBBY_OUT + 4) as u64
            };
            c.g[A2][0] = 0x4451_5545;
            schedule_no_unwind(c, &mut r, seed, 0x003e_77f0)
        });
    }

    /// Build ping-request nodes: (tag, id, gen, name) each 0x44 bytes.
    pub(crate) fn lobby_ping_chain(c: &mut Cpu, nodes: &[(u32, u32, u32, &[u8])]) {
        lobby_zero(c, LOBBY_NODE, 0x40);
        for (i, &(tag, id, gen, name)) in nodes.iter().enumerate() {
            let n = LOBBY_NODE + 0x44 * i as u32;
            c.bus.write32(n + LOBBY_REQ_TAG, tag);
            c.bus.write32(n + LOBBY_PINGREQ_ID, id);
            c.bus.write32(n + LOBBY_PINGREQ_GEN, gen);
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
            48,
            &|c, seed| {
                let _r = lobby_base(c, seed, 0x3e79_18, 1);
                lobby_zero(c, LOBBY, 0x160);
                lobby_nz(c, seed, LOBBY_OUT, 20);
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
                        lobby_ping_chain(c, &[(0x7069_6e67, 9, 0, b"pn"), (0x7069_6e67, 9, 0, b"pn")]);
                        c.bus.write32(LOBBY + LOBBY_API_REQUESTS, LOBBY_NODE);
                    }
                    3 => {
                        lobby_ping_chain(c, &[(0x7069_6e67, 7, 0, b"qx"), (0x7069_6e67, 7, 0, b"pn")]);
                        c.bus.write32(LOBBY + LOBBY_API_REQUESTS, LOBBY_NODE);
                    }
                    4 => {
                        lobby_ping_chain(c, &[(0x7069_6e67, 7, 0, b"pn")]);
                        c.bus.write32(LOBBY + LOBBY_API_REQUESTS, LOBBY_NODE);
                    }
                    5 => {
                        lobby_ping_chain(c, &[(0x7069_6e67, 3, 5, b"zz"), (0x7069_6e67, 3, 5, b"zz")]);
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
                c.g[A3][0] = match seed / 7 % 4 {
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
            lobby_nz(c, seed, LOBBY_OUT, 20);
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

    #[test]
    fn lobby_api_flush_requests_events() {
        driven!(gen_flush, ee::s14::f_003e7ac0__generated);
        driven!(hand_flush, lobby_api_flush_requests);
        check_events(gen_flush, hand_flush, 16, &|c, seed| {
            let mut r = lobby_base(c, seed, 0x3e7a_c1, 0);
            lobby_zero(c, LOBBY, 0x160);
            if seed % 2 == 0 {
                lobby_req_chain(c, &[0x4451_5545]);
            } else {
                lobby_req_chain(c, &[0x4451_5545, 0x4451_5545]);
            }
            c.bus.write32(LOBBY + LOBBY_API_REQUESTS, LOBBY_NODE);
            c.g[A0][0] = LOBBY as u64;
            schedule_no_unwind(c, &mut r, seed, 0x003e_7ac0)
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
                lobby_nz(c, seed, LOBBY_REC, 0x80);
                let key = 0x77 + seed;
                let count = seed % 256;
                // Table: none, missing entry, same count, new count.
                match seed % 4 {
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
                        c.bus.write32(w, LOBBY_REC);
                        let same = seed % 4 == 2;
                        c.bus.write32(LOBBY_REC + 0x10, if same { count } else { count + 1 });
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
                // Zeroed list sorts clean; hand slots steer the skip arms.
                let lz = LOBBY_TAB + 0x100;
                lobby_zero(c, lz, 0x12);
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
                        c.bus.write32(s + 8, 1);
                    }
                    2 => {
                        for i in [0u32, 5] {
                            let s = LOBBY + LOBBY_API_TABLES + 0x14 * i;
                            c.bus.write32(s, h);
                            c.bus.write32(s + 4, lz);
                            c.bus.write32(s + 8, 1);
                        }
                    }
                    _ => {
                        let s1 = LOBBY + LOBBY_API_TABLES + 0x14;
                        c.bus.write32(s1, 0);
                        c.bus.write32(s1 + 4, lz);
                        c.bus.write32(s1 + 8, 1);
                        let s6 = LOBBY + LOBBY_API_TABLES + 0x14 * 6;
                        c.bus.write32(s6, h);
                        c.bus.write32(s6 + 4, 0);
                        c.bus.write32(s6 + 8, 1);
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

    /// Valid ping record plus the parsed id, for the ProcessPings tests.
    /// Builds the fields the parser looks up and resolves the address the
    /// queue nodes and room entries must carry.
    pub(crate) fn lobby_ping_record(c: &mut Cpu, rec: u32) -> u32 {
        lobby_nz(c, 0x70c0, rec, 0x80);
        lobby_call(c, ee::s14::TagFieldSetString, rec as u64, 0x200, 0x4a48_78, LOBBY_STR as u64);
        lobby_call(c, ee::s14::TagFieldSetString, rec as u64, 0x200, 0x4a48_80, (LOBBY_STR + 0x20) as u64);
        let found = lobby_call(c, ee::s14::TagFieldFind, rec as u64, 0x4a48_78, 0, 0);
        lobby_call(c, ee::s14::TagFieldGetAddress, found as u64, 0, 0, 0) as u32
    }

    #[test]
    fn lobby_api_process_pings_matches() {
        check(
            ee::s14::f_003e7c40__generated,
            lobby_api_process_pings,
            96,
            &|c, seed| {
                let _r = lobby_base(c, seed, 0x3e7c_40, 0);
                lobby_zero(c, LOBBY, 0x160);
                lobby_str(c, LOBBY_STR, b"idval");
                lobby_str(c, LOBBY_STR + 0x20, b"pn");
                c.g[A0][0] = LOBBY as u64;
                c.g[A1][0] = 0;
                c.g[A2][0] = 0;
                c.g[A3][0] = 0;
                c.pc = 0x003e_7c40;
                // The frame words the parser and callbacks read (zeroed RAM
                // is assumed nowhere: write them explicitly).
                let frame0 = (c.g[SP][0] as u32).wrapping_sub(0x440);
                c.bus.write32(frame0, 0);
                c.bus.write32(frame0 + 4, 0);
                c.bus.write32(frame0 + 0x168, 0);
                c.bus.write32(frame0 + 0x16c, 0);
                if seed % 2 == 0 {
                    // Quiet channel: empty ping state, no lists, no queue.
                    let p = LOBBY_REC;
                    lobby_zero(c, p, 8);
                    c.bus.write32(LOBBY + LOBBY_API_PING, p);
                } else {
                    // Live reply: count large enough for the length path.
                    let p = LOBBY_REC;
                    lobby_zero(c, p, 8);
                    c.bus.write32(p, 0x20);
                    c.bus.write32(p + 4, 1);
                    c.bus.write32(p + 0x10, LOBBY_REC + 0x100);
                    c.bus.write32(LOBBY + LOBBY_API_PING, p);
                    let sid = lobby_ping_record(c, LOBBY_REC + 0x100);
                    let r = seed / 2;
                    // Queue: match (+trailer), mismatch, or empty.
                    match r % 3 {
                        0 => {
                            lobby_ping_chain(
                                c,
                                &[
                                    (0x7069_6e67, sid, 0, b"pn"),
                                    (0x7069_6e67, 0x1234, 0, b"pn"),
                                ],
                            );
                        }
                        1 => {
                            lobby_ping_chain(c, &[(0x7069_6e67, sid, 0, b"xx")]);
                        }
                        _ => {}
                    }
                    if r % 3 != 2 {
                        c.bus.write32(LOBBY + LOBBY_API_REQUESTS, LOBBY_NODE);
                    }
                    // Callback word the processed reply carries.
                    if r % 3 == 0 {
                        c.bus.write32(LOBBY_NODE + LOBBY_REQ_ARG2, if r / 3 % 2 == 0 { 0 } else { FAKE });
                    }
                    // Room list: none, matching entry, other entry.
                    if r / 9 % 3 != 0 {
                        let rl = LOBBY_TAB;
                        let arr = LOBBY_TAB + 0x40;
                        let pp = LOBBY_TAB + 0x60;
                        let e = LOBBY_TAB + 0x80;
                        lobby_zero(c, rl, 8);
                        c.bus.write32(rl + 0x18, 1);
                        c.bus.write32(rl + 0x14, if r / 9 % 3 == 1 { 5 } else { 0 });
                        c.bus.write32(rl + 0x40, arr);
                        c.bus.write32(arr, pp);
                        c.bus.write32(pp, e);
                        lobby_nz(c, seed, e, 0x80);
                        c.bus.write32(e, 0xEEEE_EEEE);
                        c.bus.write32(
                            e + 0x64,
                            if r / 9 % 3 == 1 { sid } else { 0x9999_9999 },
                        );
                        c.bus.write32(LOBBY + LOBBY_API_ROOM_LIST, rl);
                    }
                    // User list: none, matching name, other name.
                    if r / 27 == 1 {
                        let ul = LOBBY_TAB + 0x300;
                        let arr = LOBBY_TAB + 0x340;
                        let pp = LOBBY_TAB + 0x360;
                        let u = LOBBY_TAB + 0x380;
                        lobby_zero(c, ul, 8);
                        c.bus.write32(ul + 0x18, 1);
                        c.bus.write32(ul + 0x40, arr);
                        c.bus.write32(arr, pp);
                        c.bus.write32(pp, u);
                        lobby_nz(c, seed + 2, u, 0x40);
                        c.bus.write32(u, 0xFFFF_FFFF);
                        lobby_str(c, u + 8, if r % 2 == 0 { b"pn" } else { b"zz" });
                        c.bus.write32(LOBBY + LOBBY_API_USER_LIST, ul);
                    }
                    c.bus.write32(LOBBY + LOBBY_API_PING_COUNT, 2);
                }
            },
        );
    }

    #[test]
    fn lobby_api_process_pings_events() {
        driven!(gen_pings, ee::s14::f_003e7c40__generated);
        driven!(hand_pings, lobby_api_process_pings);
        check_events(gen_pings, hand_pings, 16, &|c, seed| {
            let mut r = lobby_base(c, seed, 0x3e7c_41, 0);
            lobby_zero(c, LOBBY, 0x160);
            lobby_str(c, LOBBY_STR, b"idval");
            lobby_str(c, LOBBY_STR + 0x20, b"pn");
            let p = LOBBY_REC;
            lobby_zero(c, p, 8);
            c.bus.write32(p, 0x20);
            c.bus.write32(p + 4, 1);
            c.bus.write32(p + 0x10, LOBBY_REC + 0x100);
            c.bus.write32(LOBBY + LOBBY_API_PING, p);
            let sid = lobby_ping_record(c, LOBBY_REC + 0x100);
            lobby_ping_chain(
                c,
                &[
                    (0x7069_6e67, sid, 0, b"pn"),
                    (0x7069_6e67, 0x1234, 0, b"pn"),
                ],
            );
            c.bus.write32(LOBBY + LOBBY_API_REQUESTS, LOBBY_NODE);
            c.bus.write32(LOBBY_NODE + LOBBY_REQ_ARG2, if seed % 2 == 0 { 0 } else { FAKE });
            if seed / 2 % 2 == 0 {
                let rl = LOBBY_TAB;
                let arr = LOBBY_TAB + 0x40;
                let pp = LOBBY_TAB + 0x60;
                let e = LOBBY_TAB + 0x80;
                lobby_zero(c, rl, 8);
                c.bus.write32(rl + 0x18, 1);
                c.bus.write32(rl + 0x14, 5);
                c.bus.write32(rl + 0x40, arr);
                c.bus.write32(arr, pp);
                c.bus.write32(pp, e);
                lobby_nz(c, seed, e, 0x80);
                c.bus.write32(e, 0xEEEE_EEEE);
                c.bus.write32(e + 0x64, sid);
                c.bus.write32(LOBBY + LOBBY_API_ROOM_LIST, rl);
            }
            if seed / 4 % 2 == 0 {
                let ul = LOBBY_TAB + 0x300;
                let arr = LOBBY_TAB + 0x340;
                let pp = LOBBY_TAB + 0x360;
                let u = LOBBY_TAB + 0x380;
                lobby_zero(c, ul, 8);
                c.bus.write32(ul + 0x18, 1);
                c.bus.write32(ul + 0x40, arr);
                c.bus.write32(arr, pp);
                c.bus.write32(pp, u);
                lobby_nz(c, seed + 2, u, 0x40);
                c.bus.write32(u, 0xFFFF_FFFF);
                lobby_str(c, u + 8, b"pn");
                c.bus.write32(LOBBY + LOBBY_API_USER_LIST, ul);
            }
            c.bus.write32(LOBBY + LOBBY_API_PING_COUNT, 3);
            c.g[A0][0] = LOBBY as u64;
            let ev = schedule_no_unwind(c, &mut r, seed, 0x003e_7c40);
            // Frame words after schedule: SP may have moved threads.
            let frame = (c.g[SP][0] as u32).wrapping_sub(0x440);
            c.bus.write32(frame, 0);
            c.bus.write32(frame + 4, 0);
            c.bus.write32(frame + 0x168, 0);
            c.bus.write32(frame + 0x16c, 0);
            c.g[A0][0] = LOBBY as u64;
            ev
        });
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
                lobby_nz(c, seed, LOBBY, 8);
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
                lobby_nz(c, seed, LOBBY_BUF, 4);
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

    #[test]
    fn lobby_api_reset_matches() {
        check_lohi(
            ee::s14::f_003e80c0__generated,
            lobby_api_reset,
            32,
            &|c, seed| {
                let _r = lobby_base(c, seed, 0x3e80_c0, 0);
                lobby_zero(c, LOBBY, 0x160);
                // Queue: empty, one node, two nodes.
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
                c.pc = 0x003e_80c0;
            },
        );
    }

    #[test]
    fn lobby_api_reset_events() {
        driven!(gen_reset, ee::s14::f_003e80c0__generated);
        driven!(hand_reset, lobby_api_reset);
        check_events(gen_reset, hand_reset, 16, &|c, seed| {
            let mut r = lobby_base(c, seed, 0x3e80_c1, 0);
            lobby_zero(c, LOBBY, 0x160);
            if seed % 2 == 0 {
                lobby_req_chain(c, &[0x4451_5545]);
            } else {
                lobby_req_chain(c, &[0x4451_5545, 0x4451_5545]);
            }
            c.bus.write32(LOBBY + LOBBY_API_REQUESTS, LOBBY_NODE);
            c.g[A0][0] = LOBBY as u64;
            schedule(c, &mut r, seed, 0x003e_80c0)
        });
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
                // Queue the reset pass frees; ping/aries states it tears down.
                if seed % 2 == 0 {
                    lobby_req_chain(c, &[0x4451_5545]);
                    c.bus.write32(LOBBY + LOBBY_API_REQUESTS, LOBBY_NODE);
                }
                if seed % 3 != 0 {
                    let p = LOBBY_BUF;
                    lobby_zero(c, p, 8);
                    c.bus.write32(LOBBY + LOBBY_API_PING, p);
                    let a = LOBBY_BUF + 0x40;
                    lobby_zero(c, a, 8);
                    c.bus.write32(LOBBY, a);
                }
                c.g[A0][0] = LOBBY as u64;
            },
        );
    }

    #[test]
    fn lobby_api_destroy_events() {
        driven!(gen_destroy, ee::s14::f_003e81c8__generated);
        driven!(hand_destroy, lobby_api_destroy);
        check_events(gen_destroy, hand_destroy, 16, &|c, seed| {
            let mut r = lobby_base(c, seed, 0x3e81_c9, 0);
            lobby_zero(c, LOBBY, 0x160);
            if seed % 2 == 0 {
                lobby_req_chain(c, &[0x4451_5545]);
            } else {
                lobby_req_chain(c, &[0x4451_5545, 0x4451_5545]);
            }
            c.bus.write32(LOBBY + LOBBY_API_REQUESTS, LOBBY_NODE);
            let p = LOBBY_BUF;
            lobby_zero(c, p, 8);
            c.bus.write32(LOBBY + LOBBY_API_PING, p);
            let a = LOBBY_BUF + 0x40;
            lobby_zero(c, a, 8);
            c.bus.write32(LOBBY, a);
            c.g[A0][0] = LOBBY as u64;
            schedule_no_unwind(c, &mut r, seed, 0x003e_81c8)
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
                lobby_nz(c, seed, LOBBY_BUF, 4);
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

    #[test]
    fn lobby_api_select_list_matches() {
        check_lohi(
            ee::s14::f_003e8368__generated,
            lobby_api_select_list,
            40,
            &|c, seed| {
                let _r = lobby_base(c, seed, 0x3e83_68, 2);
                lobby_zero(c, LOBBY, 0x160);
                let a = LOBBY_BUF + 0x40;
                lobby_zero(c, a, 8);
                c.bus.write32(LOBBY + LOBBY_API_ARIES, a);
                // Index: out of range, the three named lists, the rest.
                c.g[A1][0] = match seed % 5 {
                    0 => 8 + seed % 3,
                    1 => 0,
                    2 => 1,
                    3 => 2,
                    _ => 3 + seed % 5,
                } as u64;
                c.g[A0][0] = LOBBY as u64;
                c.g[A2][0] = (seed / 5 % 2) as u64;
                // Connection tag steers the announce sends.
                c.bus.write32(
                    LOBBY + LOBBY_API_STATE_TAG,
                    match seed / 10 % 4 {
                        0 => 0x6f66_666c,
                        1 => 0x1234_5678,
                        2 => 0x7264_6972,
                        _ => 0x636f_6e6e,
                    },
                );
                c.pc = 0x003e_8368;
            },
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
}
