//! Staging module oc1062 (wave oc1062): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1062`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame, Unwound};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] =
    &[(0x003f_17d8, buddy_api_process, "BuddyApi_Process")];

pub(crate) mod callee {
    use super::ee;
    use super::Callee;

    pub(crate) use crate::game::dirtysock::tagsunk::callee::LOBBY_LIST_GET_AT;
    pub(crate) use crate::game::dirtysock::tagsunk::callee::TAG_FIELD_FIND;
    pub(crate) use crate::game::dirtysock::tagsunk::callee::TAG_FIELD_GET_NUMBER;
    pub(crate) use crate::game::movie::movieplayer::callee::TAG_FIELD_GET_STRING;

    pub const PROTO_ARIES_TICK: Callee = Callee {
        addr: 0x003f_5d88,
        f: ee::s14::ProtoAriesTick,
    };

    pub const PROTO_ARIES_NOP: Callee = Callee {
        addr: 0x003f_5908,
        f: ee::s14::ProtoAries_Nop,
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

    pub const PROTO_ARIES_PEEK: Callee = Callee {
        addr: 0x003f_5a70,
        f: ee::s14::ProtoAriesPeek,
    };

    pub const PROTO_ARIES_RECV: Callee = Callee {
        addr: 0x003f_5c38,
        f: ee::s14::ProtoAriesRecv,
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

    pub const BUDDY_API_SEND_PRESENCE: Callee = Callee {
        addr: 0x003f_0568,
        f: ee::s14::BuddyApi_SendPresence,
    };

    pub const BUDDY_API_SET_STATE: Callee = Callee {
        addr: 0x003f_0610,
        f: ee::s14::BuddyApi_SetState,
    };

    pub const BUDDY_API_DISCONNECT: Callee = Callee {
        addr: 0x003f_0b50,
        f: ee::s14::BuddyApi_Disconnect,
    };

    pub const TAG_FIELD_STRNZCPY: Callee = Callee {
        addr: 0x003e_bb60,
        f: ee::s14::TagFieldStrnzcpy,
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

    pub const LOBBY_LIST_SORT: Callee = Callee {
        addr: 0x003e_e620,
        f: ee::s14::Lobby_ListSort,
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

    /// Cross-shard calls (made through the function table).
    pub const LOBBY_MEM_ALLOC: u32 = 0x0025_91e8;
    pub const LOBBY_MEM_FREE: u32 = 0x0025_9210;
    pub const MEMCMP: u32 = 0x0041_5fc8;
    pub const MEMSET: u32 = 0x0041_6210;
    pub const STRCHR: u32 = 0x0041_6404;
    pub const STRCPY: u32 = 0x0041_66f4;
    pub const STRLEN: u32 = 0x0041_6810;
}

// ---------------------------------------------------------------------------
// Buddy client: the per-tick driver.

/// `BuddyApi` fields `BuddyApi_Process` reads and writes.
///
/// The connector being opened (0 when none): +4 its server address, +8 its
/// poll function, +0xc its close function.
pub(crate) const BUDDY_CONNECTOR: u32 = 0x0;
/// The ProtoAries connection.
pub(crate) const BUDDY_ARIES: u32 = 0x4;
pub(crate) const BUDDY_STATE: u32 = 0x8;
/// Tick after which the login times out (0: none); an `AUTH` reply clears it.
pub(crate) const BUDDY_LOGIN_DEADLINE: u32 = 0xc;
pub(crate) const BUDDY_PORT: u32 = 0x10;
/// Non-zero: roster names containing a `.` are accepted.
pub(crate) const BUDDY_ACCEPT_DOTTED: u32 = 0x18;
/// Tick at which the presence is sent again (0: not pending).
pub(crate) const BUDDY_PRESENCE_DUE: u32 = 0x2c0;
/// The login tags sent in the `AUTH` packet.
pub(crate) const BUDDY_LOGIN_TAGS: u32 = 0x394;
/// The roster: name hash and list of 0xb8-byte entries.
pub(crate) const BUDDY_ROSTER_HASH: u32 = 0x4a4;
pub(crate) const BUDDY_ROSTER: u32 = 0x4a8;
/// Callback for `RECV` (through `BuddyApi_DispatchMessage`) and `ADMN`.
pub(crate) const BUDDY_MESSAGE_ARG: u32 = 0x4ac;
pub(crate) const BUDDY_MESSAGE_FN: u32 = 0x4b0;
/// Callback for the roster reports (`rost`, `endr`).
pub(crate) const BUDDY_ROSTER_ARG: u32 = 0x4b4;
pub(crate) const BUDDY_ROSTER_FN: u32 = 0x4b8;
/// Roster entries still expected (from an `RGET` reply's `SIZE`).
pub(crate) const BUDDY_ROSTER_LEFT: u32 = 0x4bc;
/// State callback (`BuddyApi_SetState`, `auth`); also gates the connector poll.
pub(crate) const BUDDY_STATE_ARG: u32 = 0x4c0;
pub(crate) const BUDDY_STATE_FN: u32 = 0x4c4;
/// Three pending operations of 0x14 bytes: {tag, id, deadline, arg, fn}.
pub(crate) const BUDDY_OPS: u32 = 0x4c8;
pub(crate) const BUDDY_OP_SIZE: u32 = 0x14;
pub(crate) const BUDDY_OP_COUNT: i64 = 3;
pub(crate) const BUDDY_OP_TAG: u32 = 0x0;
pub(crate) const BUDDY_OP_ID: u32 = 0x4;
pub(crate) const BUDDY_OP_DEADLINE: u32 = 0x8;
pub(crate) const BUDDY_OP_ARG: u32 = 0xc;
pub(crate) const BUDDY_OP_FN: u32 = 0x10;
/// The op answered by `RADD`, `RDEL` and `SEND` replies.
pub(crate) const BUDDY_OP_ADD: u32 = BUDDY_OPS;
pub(crate) const BUDDY_OP_DEL: u32 = BUDDY_OPS + BUDDY_OP_SIZE;
pub(crate) const BUDDY_OP_SEND: u32 = BUDDY_OPS + 2 * BUDDY_OP_SIZE;
/// Debug print callback.
#[cfg(test)]
pub(crate) const BUDDY_DEBUG_ARG: u32 = 0x504;
pub(crate) const BUDDY_DEBUG_FN: u32 = 0x508;

/// The connector's fields.
pub(crate) const CONNECTOR_ADDRESS: u32 = 0x4;
pub(crate) const CONNECTOR_POLL: u32 = 0x8;
pub(crate) const CONNECTOR_CLOSE: u32 = 0xc;

/// States `BuddyApi_Process` sets or tests, named by what sets them.
pub(crate) const BUDDY_STATE_POLL_FAILED: u64 = 2;
pub(crate) const BUDDY_STATE_LOGIN_TIMEOUT: u64 = 3;
pub(crate) const BUDDY_STATE_LINK_UP: u64 = 5;
pub(crate) const BUDDY_STATE_LINK_DOWN: u64 = 6;

/// Roster entry fields (0xb8 bytes; the user name at 0).
pub(crate) const ROSTER_ENTRY_SIZE: u64 = 0xb8;
pub(crate) const ROSTER_ENTRY_GROUP: u32 = 0x20;
pub(crate) const ROSTER_ENTRY_PRODUCT: u32 = 0x40;
pub(crate) const ROSTER_ENTRY_PRESENCE: u32 = 0x54;
pub(crate) const ROSTER_ENTRY_COPY_END: u32 = 0xa0;
pub(crate) const ROSTER_ENTRY_LANG: u32 = 0xa4;
pub(crate) const ROSTER_ENTRY_SHOW: u32 = 0xa7;
pub(crate) const ROSTER_ENTRY_FLAGS: u32 = 0xa8;
/// The id of the add request the entry waits for (`RADD`).
pub(crate) const ROSTER_ENTRY_ADD_ID: u32 = 0xac;
/// The user names are at most 0x1f characters (0x20 buffers).
pub(crate) const ROSTER_NAME_SIZE: u64 = 0x20;
/// Flags: waiting for an add reply; set by a `ROST` entry of list id 1 / 2;
/// cleared by every `ROST` entry.
pub(crate) const ROSTER_FLAG_ADDING: u64 = 0x1_0000;
pub(crate) const ROSTER_FLAG_LIST1: u64 = 0x4;
pub(crate) const ROSTER_FLAG_LIST2: u64 = 0x200;
pub(crate) const ROSTER_FLAG_ROST_CLEARS: u64 = 0xffff_ffff_ffef_ffff;
/// The `ID` of the main roster list (`RGET`, `ROST`).
pub(crate) const ROSTER_LIST1: u64 = 1;
pub(crate) const ROSTER_LIST2: u64 = 2;

/// Message kinds (the `kind` word of a received packet).
pub(crate) const KIND_LINK: u64 = 0xffff_ffff_ffff_ffff; // -1: link status
pub(crate) const KIND_PING: u32 = 0x5049_4e47; // "PING"
pub(crate) const KIND_AUTH: u32 = 0x4155_5448; // "AUTH"
pub(crate) const KIND_RECV: u32 = 0x5245_4356; // "RECV"
pub(crate) const KIND_SEND: u32 = 0x5345_4e44; // "SEND"
pub(crate) const KIND_ADMN: u32 = 0x4144_4d4e; // "ADMN"
pub(crate) const KIND_RGET: u32 = 0x5247_4554; // "RGET"
pub(crate) const KIND_ROST: u32 = 0x524f_5354; // "ROST"
pub(crate) const KIND_PGET: u32 = 0x5047_4554; // "PGET"
pub(crate) const KIND_RADD: u32 = 0x5241_4444; // "RADD"
pub(crate) const KIND_RDEL: u32 = 0x5244_454c; // "RDEL"
/// Link status code (a kind -1 message whose code word is this): link down.
pub(crate) const LINK_DOWN: u64 = 0xffff_ffff_fefe_fefe;

/// Tags of the messages handed to the callbacks.
pub(crate) const TAG_AUTH: u32 = 0x6175_7468; // "auth"
pub(crate) const TAG_SEND: u32 = 0x7365_6e64; // "send"
pub(crate) const TAG_ADMN: u32 = 0x6164_6d6e; // "admn"
pub(crate) const TAG_ADDU: u32 = 0x6164_6475; // "addu"
pub(crate) const TAG_DELU: u32 = 0x6465_6c75; // "delu"
pub(crate) const TAG_ENDR: u32 = 0x656e_6472; // "endr"
pub(crate) const TAG_ROST: u32 = 0x726f_7374; // "rost"
/// Message codes: an op cancelled (state 3/6), an add refused.
pub(crate) const OP_CANCELLED: u64 = 0xffff_ffff_ffff_fffc; // -4
pub(crate) const ADD_REFUSED: u64 = 0xffff_ffff_ffff_fffe; // -2

/// Strings in the game's data: tag names, formats.
/// The `lui` of the tag names (left in t0/t1/t2, s4 and s5).
pub(crate) const STRINGS_PAGE: u64 = 0x004a_0000;
pub(crate) const TAG_NAME_ID: u32 = 0x004a_4b58; // "ID"
pub(crate) const TAG_NAME_SIZE: u32 = 0x004a_4b88; // "SIZE"
pub(crate) const TAG_NAME_USER: u32 = 0x004a_4ae0; // "USER"
pub(crate) const TAG_NAME_GROUP: u32 = 0x004a_4b68; // "GROUP"
pub(crate) const TAG_NAME_FUSR: u32 = 0x004a_4b90; // "FUSR"
pub(crate) const EMPTY_STRING: u32 = 0x004a_4ae8; // ""
pub(crate) const DEBUG_RECV_PREFIX: u32 = 0x004a_4b80; // "recv: "
pub(crate) const FMT_CONNECTING: u32 = 0x0049_6ba0; // "connecting to %08x:%d\n"
pub(crate) const FMT_PRESENCE_PAGE: u64 = 0x0049_0000; // `lui` of the presence format (left in v1)
pub(crate) const FMT_PRESENCE: u32 = 0x0049_6bb8; // "PRES: buddy=%s, show=%d, ..."

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
fn buddy_process_field(c: &Cpu, off: u32) -> Addr {
    Addr(r32(c, S2).wrapping_add(off))
}

/// `off(reg)`.
fn buddy_process_at(c: &Cpu, reg: usize, off: u32) -> Addr {
    Addr(r32(c, reg).wrapping_add(off))
}

/// The `lui` half of a four-character code, as the delay slots load it.
fn buddy_process_lui(word: u32) -> u64 {
    sx(word & 0xffff_0000)
}

/// A back edge: deliver a due event at `head`; `Err` when the thread unwinds
/// (the trampoline re-enters the function at `head`).
fn buddy_process_back_edge(c: &mut Cpu, head: u32) -> Flow {
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
fn buddy_process_connect(c: &mut Cpu, frame: Frame) -> Flow {
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
            c.g[A0][0] = buddy_process_field(c, BUDDY_CONNECTOR).lw(c);
            if c.g[A0][0] == 0 {
                c.g[V0][0] = buddy_process_field(c, BUDDY_LOGIN_DEADLINE).lw(c);
                break 'deadline;
            }
            // 0x3f1838: poll the connector.
            c.cycles += 6;
            c.g[V0][0] = buddy_process_at(c, A0, CONNECTOR_POLL).lw(c);
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
                c.g[V0][0] = buddy_process_field(c, BUDDY_CONNECTOR).lw(c);
                c.g[A0][0] = buddy_process_field(c, BUDDY_ARIES).lw(c);
                c.g[A1][0] = buddy_process_at(c, V0, CONNECTOR_ADDRESS).lw(c);
                c.g[A2][0] = buddy_process_field(c, BUDDY_PORT).lw(c);
                callee::PROTO_ARIES_CONNECT.call(c, 0x003f_186c)?;
                c.g[V1][0] = buddy_process_field(c, BUDDY_CONNECTOR).lw(c);
                c.g[A3][0] = buddy_process_field(c, BUDDY_PORT).lw(c);
                c.g[A1][0] = sx(FMT_CONNECTING);
                c.g[A2][0] = buddy_process_at(c, V1, CONNECTOR_ADDRESS).lw(c);
                c.g[A0][0] = c.g[S2][0];
                callee::BUDDY_API_DEBUG_PRINTF.call(c, 0x003f_1888)?;
                c.g[A0][0] = c.g[S2][0];
                c.g[A1][0] = sx(KIND_AUTH);
                c.g[A2][0] = sx(r32(c, S2).wrapping_add(BUDDY_LOGIN_TAGS));
                callee::BUDDY_API_SEND_PACKET.call(c, 0x003f_189c)?;
            }
            // 0x3f189c
            c.cycles += 2;
            if (c.g[S3][0] as i64) >= 0 {
                c.g[A0][0] = buddy_process_field(c, BUDDY_CONNECTOR).lw(c);
            } else {
                // 0x3f18a4: the connector failed.
                c.cycles += 5;
                c.g[A0][0] = c.g[S2][0];
                c.g[A1][0] = BUDDY_STATE_POLL_FAILED;
                callee::BUDDY_API_SET_STATE.call(c, 0x003f_18b0)?;
                buddy_process_field(c, BUDDY_STATE_FN).sw(c, 0);
                c.g[A0][0] = buddy_process_field(c, BUDDY_CONNECTOR).lw(c);
            }
            // 0x3f18b8: close the connector.
            c.cycles += 4;
            c.g[V0][0] = buddy_process_at(c, A0, CONNECTOR_CLOSE).lw(c);
            let close = r32(c, V0);
            call_ptr(c, close, 0x003f_18c4)?;
            buddy_process_field(c, BUDDY_CONNECTOR).sw(c, 0);
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
fn buddy_process_message_regs(c: &mut Cpu, frame: Frame, kind_reg: usize, code_reg: usize) -> bool {
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
fn buddy_process_link_up(c: &mut Cpu, frame: Frame) -> Flow {
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
fn buddy_process_op_timeouts(c: &mut Cpu, head: u32) -> Flow {
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
        c.g[V0][0] = buddy_process_at(c, V1, BUDDY_OPS + BUDDY_OP_FN).lw(c);
        let idle = c.g[V0][0] == 0;
        c.g[V0][0] = c.g[V1][0];
        next = true;
        if idle {
            buddy_process_back_edge(c, BUDDY_PROCESS_OP_NEXT)?;
            continue;
        }
        // 0x3f197c
        c.cycles += 3;
        c.g[V0][0] = buddy_process_at(c, V0, BUDDY_OPS + BUDDY_OP_DEADLINE).lw(c);
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
fn buddy_process_cancel_ops(c: &mut Cpu, frame: Frame, resume: bool) -> Flow {
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
        c.g[V1][0] = buddy_process_at(c, S0, 0).lw(c);
        let idle = c.g[V1][0] == 0;
        c.g[A0][0] = c.g[S2][0];
        if !idle {
            // 0x3f19dc
            c.cycles += 7;
            c.g[V0][0] = Addr(r32(c, S1).wrapping_sub(BUDDY_OP_ARG - BUDDY_OP_TAG)).lw(c);
            c.g[A2][0] = buddy_process_at(c, S1, 0).lw(c);
            c.g[A1][0] = c.g[S6][0];
            Addr(frame.at(c, BUDDY_PROCESS_MSG)).sw(c, c.g[V0][0]);
            let f = r32(c, V1);
            Addr(frame.at(c, BUDDY_PROCESS_MSG_CODE)).sw(c, c.g[S4][0]);
            call_ptr(c, f, 0x003f_19f4)?;
            buddy_process_at(c, S0, 0).sw(c, 0);
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
fn buddy_process_receive(c: &mut Cpu, frame: Frame, mut resume: Option<u32>) -> Flow {
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
        c.g[A0][0] = buddy_process_field(c, BUDDY_ROSTER).lw(c);
        callee::LOBBY_LIST_SORT.call(c, 0x003f_21b8)?;
        let unchanged = (c.g[V0][0] as i64) <= 0;
        c.g[V0][0] = buddy_process_lui(TAG_ROST);
        if !unchanged {
            // 0x3f21c0: report the roster.
            c.cycles += 11;
            c.g[V1][0] = buddy_process_field(c, BUDDY_ROSTER).lw(c);
            c.g[V0][0] = sx(TAG_ROST);
            Addr(frame.at(c, BUDDY_PROCESS_MSG_KIND)).sw(c, 0);
            Addr(frame.at(c, BUDDY_PROCESS_MSG)).sw(c, c.g[V0][0]);
            c.g[A0][0] = c.g[S2][0];
            Addr(frame.at(c, BUDDY_PROCESS_MSG_DATA)).sw(c, c.g[V1][0]);
            c.g[A1][0] = c.g[S6][0];
            Addr(frame.at(c, BUDDY_PROCESS_MSG_CODE)).sw(c, 0);
            c.g[V0][0] = buddy_process_field(c, BUDDY_ROSTER_FN).lw(c);
            let f = r32(c, V0);
            c.g[A2][0] = buddy_process_at(c, A0, BUDDY_ROSTER_ARG).lw(c);
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
fn buddy_process_message(c: &mut Cpu, frame: Frame, at: u32) -> Flow {
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
fn buddy_process_replies(c: &mut Cpu, frame: Frame) -> Flow {
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
    c.g[V0][0] = KIND_LINK;
    let link = c.g[S0][0] == c.g[V0][0];
    c.g[V0][0] = buddy_process_lui(KIND_PING);
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
        c.g[V0][0] = buddy_process_lui(KIND_PING);
        if still_link {
            // 0x3f1a68
            c.cycles += 5;
            c.g[V0][0] = LINK_DOWN;
            c.g[V1][0] = Addr(code_at).lw(c);
            let down = c.g[V1][0] == c.g[V0][0];
            c.g[V0][0] = buddy_process_lui(KIND_PING);
            if down {
                // 0x3f1a7c
                c.cycles += 4;
                c.g[A0][0] = c.g[S2][0];
                c.g[A1][0] = BUDDY_STATE_LINK_DOWN;
                callee::BUDDY_API_SET_STATE.call(c, 0x003f_1a88)?;
                c.g[V0][0] = buddy_process_lui(KIND_PING);
            }
        }
    }
    // 0x3f1a8c: PING: answer it.
    c.cycles += 4;
    c.g[V1][0] = Addr(kind_at).lw(c);
    c.g[V0][0] = sx(KIND_PING);
    let ping = c.g[V1][0] == c.g[V0][0];
    c.g[V0][0] = buddy_process_lui(KIND_AUTH);
    if ping {
        // 0x3f1a9c
        c.cycles += 9;
        c.g[A1][0] = sx(KIND_PING);
        c.g[A0][0] = buddy_process_field(c, BUDDY_ARIES).lw(c);
        c.g[A2][0] = Addr(code_at).lw(c);
        c.g[A3][0] = Addr(data_at).lw(c);
        c.g[T0][0] = u64::MAX;
        callee::PROTO_ARIES_SEND.call(c, 0x003f_1ab8)?;
        c.g[V0][0] = buddy_process_lui(KIND_AUTH);
        c.g[V1][0] = Addr(kind_at).lw(c);
    }
    // 0x3f1ac0: AUTH: the login reply.
    c.cycles += 3;
    c.g[V0][0] = sx(KIND_AUTH);
    let auth = c.g[V1][0] == c.g[V0][0];
    c.g[V0][0] = buddy_process_lui(KIND_RECV);
    if auth {
        // 0x3f1acc
        c.cycles += 11;
        c.g[V0][0] = sx(TAG_AUTH);
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
        c.g[V0][0] = buddy_process_lui(KIND_RECV);
    }
    // 0x3f1b04: RECV: a buddy message.
    c.cycles += 4;
    c.g[V1][0] = Addr(kind_at).lw(c);
    c.g[V0][0] = sx(KIND_RECV);
    let recv = c.g[V1][0] == c.g[V0][0];
    c.g[V0][0] = buddy_process_lui(KIND_SEND);
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
        c.g[V0][0] = buddy_process_lui(KIND_SEND);
        c.g[V1][0] = Addr(kind_at).lw(c);
    }
    // 0x3f1b30: SEND: the send op's reply.
    c.cycles += 3;
    c.g[V0][0] = sx(KIND_SEND);
    let send = c.g[V1][0] == c.g[V0][0];
    c.g[V0][0] = buddy_process_lui(KIND_ADMN);
    if send {
        // 0x3f1b3c
        c.cycles += 3;
        c.g[V1][0] = buddy_process_field(c, BUDDY_OP_SEND + BUDDY_OP_FN).lw(c);
        let no_fn = c.g[V1][0] == 0;
        c.g[V0][0] = buddy_process_lui(TAG_SEND);
        if !no_fn {
            // 0x3f1b48
            c.cycles += 7;
            c.g[A2][0] = buddy_process_field(c, BUDDY_OP_SEND + BUDDY_OP_ARG).lw(c);
            c.g[V0][0] = sx(TAG_SEND);
            c.g[A0][0] = c.g[S2][0];
            Addr(frame.at(c, BUDDY_PROCESS_MSG)).sw(c, c.g[V0][0]);
            let f = r32(c, V1);
            c.g[A1][0] = c.g[S6][0];
            call_ptr(c, f, 0x003f_1b60)?;
            buddy_process_field(c, BUDDY_OP_SEND + BUDDY_OP_FN).sw(c, 0);
        }
        // 0x3f1b64
        c.cycles += 1;
        c.g[V0][0] = buddy_process_lui(KIND_ADMN);
    }
    // 0x3f1b68: ADMN: an admin message.
    c.cycles += 4;
    c.g[V1][0] = Addr(kind_at).lw(c);
    c.g[V0][0] = sx(KIND_ADMN);
    let admn = c.g[V1][0] == c.g[V0][0];
    c.g[V0][0] = buddy_process_lui(KIND_RGET);
    if admn {
        // 0x3f1b78
        c.cycles += 3;
        c.g[V1][0] = buddy_process_field(c, BUDDY_MESSAGE_FN).lw(c);
        let no_fn = c.g[V1][0] == 0;
        c.g[V0][0] = buddy_process_lui(TAG_ADMN);
        if !no_fn {
            // 0x3f1b84
            c.cycles += 6;
            c.g[A2][0] = buddy_process_field(c, BUDDY_MESSAGE_ARG).lw(c);
            c.g[V0][0] = sx(TAG_ADMN);
            c.g[A0][0] = c.g[S2][0];
            Addr(frame.at(c, BUDDY_PROCESS_MSG)).sw(c, c.g[V0][0]);
            let f = r32(c, V1);
            c.g[A1][0] = c.g[S6][0];
            call_ptr(c, f, 0x003f_1b9c)?;
        }
        // 0x3f1b9c
        c.cycles += 2;
        c.g[V0][0] = buddy_process_lui(KIND_RGET);
        c.g[V1][0] = Addr(kind_at).lw(c);
    }
    // 0x3f1ba4: RGET: the roster size (list 2 only).
    c.cycles += 3;
    c.g[V0][0] = sx(KIND_RGET);
    let rget = c.g[V1][0] == c.g[V0][0];
    c.g[V0][0] = buddy_process_lui(KIND_ROST);
    if rget {
        // 0x3f1bb0
        c.cycles += 10;
        c.g[A0][0] = Addr(data_at).lw(c);
        c.g[T1][0] = STRINGS_PAGE;
        c.g[A1][0] = sx(TAG_NAME_ID);
        callee::TAG_FIELD_FIND.call(c, 0x003f_1bc0)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        callee::TAG_FIELD_GET_NUMBER.call(c, 0x003f_1bcc)?;
        c.g[V1][0] = ROSTER_LIST2;
        let list2 = c.g[V0][0] == c.g[V1][0];
        c.g[V0][0] = buddy_process_lui(KIND_ROST);
        if list2 {
            // 0x3f1bd8
            c.cycles += 11;
            c.g[A0][0] = Addr(data_at).lw(c);
            c.g[A1][0] = sx(TAG_NAME_SIZE);
            callee::TAG_FIELD_FIND.call(c, 0x003f_1be8)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            callee::TAG_FIELD_GET_NUMBER.call(c, 0x003f_1bf4)?;
            buddy_process_field(c, BUDDY_ROSTER_LEFT).sw(c, c.g[V0][0]);
            c.g[V0][0] = (c.g[V0][0] < 1) as u64;
            Addr(frame.at(c, BUDDY_PROCESS_ROSTER_DONE)).sw(c, c.g[V0][0]);
            c.g[V0][0] = buddy_process_lui(KIND_ROST);
        }
    }
    Ok(())
}

/// 0x3f1c04..0x3f1da0: `ROST`: one roster entry: find or add the user, copy
/// its group, set the list flag; the last entry of list 2 sets "roster
/// complete". Leaves the kind in v1 and `lui "PG"` in v0.
fn buddy_process_roster_entry(c: &mut Cpu, frame: Frame) -> Flow {
    let data_at = frame.at(c, BUDDY_PROCESS_MSG_DATA);
    // 0x3f1c04
    c.cycles += 4;
    c.g[V1][0] = Addr(frame.at(c, BUDDY_PROCESS_MSG_KIND)).lw(c);
    c.g[V0][0] = sx(KIND_ROST);
    let rost = c.g[V1][0] == c.g[V0][0];
    c.g[V0][0] = buddy_process_lui(KIND_PGET);
    if !rost {
        return Ok(());
    }
    'reload: {
        'tail: {
            // 0x3f1c14
            c.cycles += 3;
            c.g[V0][0] = buddy_process_field(c, BUDDY_ROSTER).lw(c);
            let no_roster = c.g[V0][0] == 0;
            c.g[A0][0] = Addr(data_at).lw(c);
            if no_roster {
                break 'tail;
            }
            // 0x3f1c20: the list id picks the flag (s3).
            c.cycles += 15;
            c.g[T2][0] = STRINGS_PAGE;
            c.g[A1][0] = sx(TAG_NAME_ID);
            c.g[S3][0] = ROSTER_FLAG_LIST1;
            callee::TAG_FIELD_FIND.call(c, 0x003f_1c30)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            callee::TAG_FIELD_GET_NUMBER.call(c, 0x003f_1c3c)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[V1][0] = ROSTER_FLAG_LIST2;
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
            c.g[A1][0] = sx(TAG_NAME_USER);
            c.g[S4][0] = STRINGS_PAGE;
            callee::TAG_FIELD_FIND.call(c, 0x003f_1c88)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = c.g[SP][0];
            c.g[A2][0] = ROSTER_NAME_SIZE;
            c.g[A3][0] = sx(EMPTY_STRING);
            callee::TAG_FIELD_GET_STRING.call(c, 0x003f_1c9c)?;
            c.g[A0][0] = c.g[SP][0];
            callee::BUDDY_API_STRIP_USER_NAME.call(c, 0x003f_1ca4)?;
            c.g[V0][0] = buddy_process_field(c, BUDDY_ACCEPT_DOTTED).lw(c);
            if c.g[V0][0] == 0 {
                // 0x3f1cb0: skip names with a dot.
                c.cycles += 5;
                c.g[A0][0] = c.g[SP][0];
                c.g[A1][0] = b'.' as u64;
                call_ptr(c, callee::STRCHR, 0x003f_1cbc)?;
                let dotted = c.g[V0][0] != 0;
                c.g[V0][0] = buddy_process_lui(KIND_PGET);
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
                call_ptr(c, callee::STRLEN, 0x003f_1ce4)?;
                c.g[V0][0] = (c.g[V0][0] < ROSTER_NAME_SIZE) as u64;
                if c.g[V0][0] != 0 {
                    // 0x3f1cf0: a new entry, named, in the list and the hash.
                    c.cycles += 18;
                    c.g[A0][0] = ROSTER_ENTRY_SIZE;
                    call_ptr(c, callee::LOBBY_MEM_ALLOC, 0x003f_1cf8)?;
                    c.g[S1][0] = c.g[V0][0];
                    c.g[A2][0] = ROSTER_ENTRY_SIZE;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[A1][0] = 0;
                    call_ptr(c, callee::MEMSET, 0x003f_1d0c)?;
                    c.g[A1][0] = c.g[SP][0];
                    c.g[A0][0] = c.g[S1][0];
                    call_ptr(c, callee::STRCPY, 0x003f_1d18)?;
                    c.g[A0][0] = buddy_process_field(c, BUDDY_ROSTER).lw(c);
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
                c.g[V0][0] = buddy_process_lui(KIND_PGET);
                if no_node {
                    break 'reload;
                }
                // 0x3f1d40
                c.cycles += 1;
            }
            c.g[A0][0] = buddy_process_field(c, BUDDY_ROSTER).lw(c);
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
            c.g[A1][0] = sx(TAG_NAME_GROUP);
            callee::TAG_FIELD_FIND.call(c, 0x003f_1d64)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A3][0] = sx(r32(c, S4).wrapping_add(EMPTY_STRING & 0xffff));
            c.g[A1][0] = sx(r32(c, S1).wrapping_add(ROSTER_ENTRY_GROUP));
            c.g[A2][0] = ROSTER_NAME_SIZE;
            callee::TAG_FIELD_GET_STRING.call(c, 0x003f_1d78)?;
            let no_flag = c.g[S3][0] == 0;
            c.g[V0][0] = buddy_process_lui(KIND_PGET);
            if no_flag {
                break 'reload;
            }
            // 0x3f1d80: the list flag.
            c.cycles += 6;
            c.g[V1][0] = buddy_process_at(c, S1, ROSTER_ENTRY_FLAGS).lw(c);
            c.g[V0][0] = ROSTER_FLAG_ROST_CLEARS;
            c.g[V1][0] |= c.g[S3][0];
            c.g[V1][0] &= c.g[V0][0];
            buddy_process_at(c, S1, ROSTER_ENTRY_FLAGS).sw(c, c.g[V1][0]);
        }
        // 0x3f1d98
        c.cycles += 1;
        c.g[V0][0] = buddy_process_lui(KIND_PGET);
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
fn buddy_process_presence(c: &mut Cpu, frame: Frame, at: u32) -> Flow {
    let aligned = if at == BUDDY_PROCESS_MESSAGE {
        // 0x3f1da0
        c.cycles += 3;
        c.g[V0][0] = sx(KIND_PGET);
        let pget = c.g[V1][0] == c.g[V0][0];
        c.g[V0][0] = buddy_process_lui(KIND_RADD);
        if !pget {
            return Ok(());
        }
        // 0x3f1dac
        c.cycles += 3;
        c.g[V0][0] = buddy_process_field(c, BUDDY_ROSTER).lw(c);
        let no_roster = c.g[V0][0] == 0;
        c.g[A0][0] = Addr(frame.at(c, BUDDY_PROCESS_MSG_DATA)).lw(c);
        if no_roster {
            // 0x3f1f30
            c.cycles += 1;
            c.g[V0][0] = buddy_process_lui(KIND_RADD);
            return Ok(());
        }
        // 0x3f1db8: the user's entry.
        c.cycles += 17;
        c.g[T0][0] = STRINGS_PAGE;
        c.g[A1][0] = sx(TAG_NAME_USER);
        callee::TAG_FIELD_FIND.call(c, 0x003f_1dc4)?;
        c.g[T1][0] = STRINGS_PAGE;
        c.g[A1][0] = c.g[SP][0];
        c.g[A2][0] = ROSTER_NAME_SIZE;
        c.g[A3][0] = sx(EMPTY_STRING);
        c.g[A0][0] = c.g[V0][0];
        callee::TAG_FIELD_GET_STRING.call(c, 0x003f_1ddc)?;
        c.g[A0][0] = c.g[SP][0];
        callee::BUDDY_API_STRIP_USER_NAME.call(c, 0x003f_1de4)?;
        c.g[A0][0] = buddy_process_field(c, BUDDY_ROSTER_HASH).lw(c);
        c.g[A1][0] = c.g[SP][0];
        callee::LOBBY_HASH_FIND_STR.call(c, 0x003f_1df0)?;
        c.g[S0][0] = c.g[V0][0];
        let unknown = c.g[S0][0] == 0;
        c.g[V0][0] = buddy_process_lui(KIND_RADD);
        if unknown {
            return Ok(());
        }
        // 0x3f1dfc: copy it (from v1 to a3 = s0, up to v0).
        c.cycles += 9;
        c.g[A0][0] = buddy_process_field(c, BUDDY_ROSTER).lw(c);
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
    call_ptr(c, callee::MEMCMP, 0x003f_1ef8)?;
    let same = c.g[V0][0] == 0;
    c.g[V0][0] = buddy_process_lui(KIND_RADD);
    if same {
        return Ok(());
    }
    // 0x3f1f00: changed: mark the roster and print the presence.
    c.cycles += 12;
    c.g[A0][0] = buddy_process_field(c, BUDDY_ROSTER).lw(c);
    c.g[A1][0] = 1;
    callee::LOBBY_LIST_MARK_DIRTY.call(c, 0x003f_1f0c)?;
    c.g[A2][0] = c.g[S1][0];
    c.g[V1][0] = FMT_PRESENCE_PAGE;
    c.g[A3][0] = buddy_process_at(c, A2, ROSTER_ENTRY_SHOW).lb(c);
    c.g[A0][0] = c.g[S2][0];
    c.g[A1][0] = sx(FMT_PRESENCE);
    c.g[T0][0] = sx(r32(c, A2).wrapping_add(ROSTER_ENTRY_PRODUCT));
    c.g[T1][0] = sx(r32(c, A2).wrapping_add(ROSTER_ENTRY_PRESENCE));
    c.g[T2][0] = sx(r32(c, A2).wrapping_add(ROSTER_ENTRY_LANG));
    callee::BUDDY_API_DEBUG_PRINTF.call(c, 0x003f_1f30)?;
    // 0x3f1f30
    c.cycles += 1;
    c.g[V0][0] = buddy_process_lui(KIND_RADD);
    Ok(())
}

/// 0x3f1f34..0x3f20d4: `RADD`: the add reply for the entry waiting with
/// this id (index s3, id s0, flag s7): refused (code != 0): call the add op
/// back with code -2 and remove the entry; accepted: rename it to `FUSR`
/// when given, call the add op back, clear the wait. `resume`: re-entered at
/// the scan head (the entry's flags in v0). Leaves the kind in v1 and
/// `lui "RD"` in v0.
fn buddy_process_roster_add(c: &mut Cpu, frame: Frame, resume: bool) -> Flow {
    let code_at = frame.at(c, BUDDY_PROCESS_MSG_CODE);
    let data_at = frame.at(c, BUDDY_PROCESS_MSG_DATA);
    'reload: {
        if !resume {
            // 0x3f1f34
            c.cycles += 4;
            c.g[V1][0] = Addr(frame.at(c, BUDDY_PROCESS_MSG_KIND)).lw(c);
            c.g[V0][0] = sx(KIND_RADD);
            let radd = c.g[V1][0] == c.g[V0][0];
            c.g[V0][0] = buddy_process_lui(KIND_RDEL);
            if !radd {
                return Ok(());
            }
            // 0x3f1f44
            c.cycles += 3;
            c.g[V0][0] = buddy_process_field(c, BUDDY_ROSTER).lw(c);
            let no_roster = c.g[V0][0] == 0;
            c.g[V0][0] = buddy_process_lui(TAG_ADDU);
            if no_roster {
                break 'reload;
            }
            // 0x3f1f50: the request id.
            c.cycles += 15;
            c.g[A0][0] = Addr(data_at).lw(c);
            c.g[A2][0] = STRINGS_PAGE;
            c.g[V0][0] = sx(TAG_ADDU);
            c.g[A1][0] = sx(TAG_NAME_ID);
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
                        c.g[A0][0] = buddy_process_field(c, BUDDY_ROSTER).lw(c);
                        c.g[A1][0] = c.g[S3][0];
                        callee::LOBBY_LIST_GET_AT.call(c, 0x003f_20b4)?;
                        c.g[S1][0] = c.g[V0][0];
                        if c.g[S1][0] == 0 {
                            break 'done;
                        }
                        c.g[V0][0] = buddy_process_at(c, S1, ROSTER_ENTRY_FLAGS).lw(c);
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
                    c.g[V0][0] = buddy_process_at(c, S1, ROSTER_ENTRY_ADD_ID).lw(c);
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
            c.g[A0][0] = buddy_process_field(c, BUDDY_ROSTER).lw(c);
        }
        // 0x3f20c4
        c.cycles += 2;
        c.g[A1][0] = 1;
        callee::LOBBY_LIST_MARK_DIRTY.call(c, 0x003f_20cc)?;
    }
    // 0x3f20cc
    c.cycles += 2;
    c.g[V0][0] = buddy_process_lui(KIND_RDEL);
    c.g[V1][0] = Addr(frame.at(c, BUDDY_PROCESS_MSG_KIND)).lw(c);
    Ok(())
}

/// 0x3f1fb4..0x3f2000: the add was refused: call the add op back (code -2,
/// data = the entry), then unhash, unlist and free the entry. Leaves the
/// roster in a0.
fn buddy_process_add_refused(c: &mut Cpu, frame: Frame) -> Flow {
    // 0x3f1fb4
    c.cycles += 3;
    c.g[V1][0] = buddy_process_field(c, BUDDY_OP_ADD + BUDDY_OP_FN).lw(c);
    let no_fn = c.g[V1][0] == 0;
    c.g[V0][0] = ADD_REFUSED;
    if !no_fn {
        // 0x3f1fc0
        c.cycles += 6;
        c.g[A2][0] = buddy_process_field(c, BUDDY_OP_ADD + BUDDY_OP_ARG).lw(c);
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
    c.g[A0][0] = buddy_process_field(c, BUDDY_ROSTER).lw(c);
    c.g[A1][0] = c.g[V0][0];
    callee::LOBBY_LIST_REMOVE.call(c, 0x003f_1ff0)?;
    c.g[A0][0] = c.g[S1][0];
    call_ptr(c, callee::LOBBY_MEM_FREE, 0x003f_1ff8)?;
    c.g[A0][0] = buddy_process_field(c, BUDDY_ROSTER).lw(c);
    Ok(())
}

/// 0x3f2000..0x3f20a8: the add was accepted: rename the entry to the `FUSR`
/// name when one fits (re-hashed under it), call the add op back (code 0,
/// data = the entry), clear the entry's wait.
fn buddy_process_add_accepted(c: &mut Cpu, frame: Frame) -> Flow {
    // 0x3f2000: a0 = the reply's tags.
    c.cycles += 12;
    c.g[A1][0] = sx(r32(c, S4).wrapping_add(TAG_NAME_FUSR & 0xffff));
    callee::TAG_FIELD_FIND.call(c, 0x003f_2008)?;
    c.g[A0][0] = c.g[V0][0];
    c.g[A3][0] = sx(r32(c, S5).wrapping_add(EMPTY_STRING & 0xffff));
    c.g[A1][0] = c.g[SP][0];
    c.g[A2][0] = ROSTER_NAME_SIZE;
    callee::TAG_FIELD_GET_STRING.call(c, 0x003f_201c)?;
    c.g[A0][0] = c.g[SP][0];
    callee::BUDDY_API_STRIP_USER_NAME.call(c, 0x003f_2024)?;
    c.g[V0][0] = Addr(sp(c)).lb(c);
    // v0 = the add op's fn on every path to 0x3f2078.
    'notify: {
        if c.g[V0][0] == 0 {
            c.g[V0][0] = buddy_process_field(c, BUDDY_OP_ADD + BUDDY_OP_FN).lw(c);
            break 'notify;
        }
        // 0x3f2030
        c.cycles += 5;
        c.g[A0][0] = c.g[SP][0];
        call_ptr(c, callee::STRLEN, 0x003f_2038)?;
        c.g[V0][0] = (c.g[V0][0] < ROSTER_NAME_SIZE) as u64;
        if c.g[V0][0] == 0 {
            c.g[V0][0] = buddy_process_field(c, BUDDY_OP_ADD + BUDDY_OP_FN).lw(c);
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
        c.g[V0][0] = buddy_process_field(c, BUDDY_OP_ADD + BUDDY_OP_FN).lw(c);
    }
    // 0x3f2078
    c.cycles += 2;
    let no_fn = c.g[V0][0] == 0;
    c.g[A0][0] = c.g[S2][0];
    if !no_fn {
        // 0x3f2080
        c.cycles += 5;
        c.g[A2][0] = buddy_process_field(c, BUDDY_OP_ADD + BUDDY_OP_ARG).lw(c);
        Addr(frame.at(c, BUDDY_PROCESS_MSG_CODE)).sw(c, 0);
        c.g[A1][0] = c.g[S6][0];
        let f = r32(c, V0);
        Addr(frame.at(c, BUDDY_PROCESS_MSG_DATA)).sw(c, c.g[S1][0]);
        call_ptr(c, f, 0x003f_2094)?;
    }
    // 0x3f2094
    c.cycles += 5;
    c.g[V0][0] = buddy_process_at(c, S1, ROSTER_ENTRY_FLAGS).lw(c);
    buddy_process_at(c, S1, ROSTER_ENTRY_ADD_ID).sw(c, 0);
    c.g[V0][0] ^= c.g[S7][0];
    buddy_process_at(c, S1, ROSTER_ENTRY_FLAGS).sw(c, c.g[V0][0]);
    Ok(())
}

/// 0x3f20d4..0x3f218c: `RDEL`: the delete reply for the delete op's id
/// (`delu`), the roster-complete report (`endr`), then consume the packet.
/// Leaves the link in a0.
fn buddy_process_roster_del(c: &mut Cpu, frame: Frame) -> Flow {
    let done_at = frame.at(c, BUDDY_PROCESS_ROSTER_DONE);
    // 0x3f20d4
    c.cycles += 3;
    c.g[V0][0] = sx(KIND_RDEL);
    let rdel = c.g[V1][0] == c.g[V0][0];
    c.g[T1][0] = Addr(done_at).lw(c);
    'done: {
        if !rdel {
            break 'done;
        }
        // 0x3f20e0
        c.cycles += 3;
        c.g[V0][0] = buddy_process_field(c, BUDDY_OP_DEL + BUDDY_OP_FN).lw(c);
        let no_fn = c.g[V0][0] == 0;
        c.g[A0][0] = Addr(frame.at(c, BUDDY_PROCESS_MSG_DATA)).lw(c);
        if no_fn {
            break 'done;
        }
        // 0x3f20ec
        c.cycles += 9;
        c.g[T0][0] = STRINGS_PAGE;
        c.g[A1][0] = sx(TAG_NAME_ID);
        callee::TAG_FIELD_FIND.call(c, 0x003f_20f8)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        callee::TAG_FIELD_GET_NUMBER.call(c, 0x003f_2104)?;
        c.g[V1][0] = buddy_process_field(c, BUDDY_OP_DEL + BUDDY_OP_ID).lw(c);
        let other = c.g[V0][0] != c.g[V1][0];
        c.g[T1][0] = Addr(done_at).lw(c);
        if other {
            break 'done;
        }
        // 0x3f2110: call the delete op back and clear it.
        c.cycles += 10;
        c.g[V0][0] = sx(TAG_DELU);
        c.g[V1][0] = buddy_process_field(c, BUDDY_OP_DEL + BUDDY_OP_FN).lw(c);
        c.g[A2][0] = buddy_process_field(c, BUDDY_OP_DEL + BUDDY_OP_ARG).lw(c);
        Addr(frame.at(c, BUDDY_PROCESS_MSG)).sw(c, c.g[V0][0]);
        c.g[A0][0] = c.g[S2][0];
        let f = r32(c, V1);
        c.g[A1][0] = c.g[S6][0];
        call_ptr(c, f, 0x003f_2130)?;
        buddy_process_field(c, BUDDY_OP_DEL + BUDDY_OP_FN).sw(c, 0);
        c.g[T1][0] = Addr(done_at).lw(c);
    }
    // 0x3f2138
    c.cycles += 2;
    if c.g[T1][0] != 0 {
        // 0x3f2140: the roster is complete.
        c.cycles += 3;
        c.g[V1][0] = buddy_process_field(c, BUDDY_ROSTER_FN).lw(c);
        let no_fn = c.g[V1][0] == 0;
        c.g[V0][0] = buddy_process_lui(TAG_ENDR);
        if !no_fn {
            // 0x3f214c
            c.cycles += 9;
            c.g[A2][0] = buddy_process_field(c, BUDDY_ROSTER_ARG).lw(c);
            c.g[V0][0] = sx(TAG_ENDR);
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::events::{check_events, driven, schedule, Events};
    use crate::difftest::fakes::{
        fall_back, hook, machine, prev_cell, FAKE, LOG_COUNT, LOG_START, LOG_UNWIND_AT,
    };
    use crate::difftest::{check, elf_image, fill, fresh, run_generated, Rng, DATA};
    use std::ptr::addr_of_mut;

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
    pub(crate) const BP_TICK_COUNT: u32 = 0x0052_3eb8;
    pub(crate) const BP_TICK_BASE: u32 = 0x0052_3ec0;
    pub(crate) const BP_CYCLES_PER_TICK: u64 = 0x48000;
    /// ProtoAries connection fields `ProtoAriesPeek`/`Recv`/`Connect` read.
    pub(crate) const BP_ARIES_LINK: u32 = 0x4;
    pub(crate) const BP_ARIES_PENDING: u32 = 0x8;
    pub(crate) const BP_ARIES_READ: u32 = 0xc;
    pub(crate) const BP_ARIES_END: u32 = 0x10;
    pub(crate) const BP_ARIES_SIZE: u32 = 0x14;
    pub(crate) const BP_ARIES_BUF: u32 = 0x18;
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
    pub(crate) const BP_RETURN_MINUS_ONE: u32 = 0x0042_4488; // `_close`
    pub(crate) const BP_STRCMP: u32 = 0x0041_65a8;
    pub(crate) const BP_KIND_UNKNOWN: u32 = 0x5a5a_5a5a; // "ZZZZ"
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

    pub(crate) fn buddy_process_put_str(c: &mut Cpu, at: u32, text: &[u8]) {
        for (i, b) in text.iter().enumerate() {
            c.bus.write8(at + i as u32, *b);
        }
        c.bus.write8(at + text.len() as u32, 0);
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
        c.bus.write32(BP_ARIES + BP_ARIES_END, at - BP_BUF);
    }

    /// The common machine: fakes, the game image (tag names, formats, the
    /// presence tables), a junk-filled `BuddyApi` with every field the
    /// function and its real callees read set, an idle connector, the Aries
    /// connection with an empty queue (pending, so the peek ends at once).
    pub(crate) fn buddy_process_machine(c: &mut Cpu, seed: u32, salt: u64) -> Rng {
        let mut r = machine(c, seed, salt, 0);
        elf_image(c);
        c.pc = BUDDY_PROCESS_ENTRY;
        c.g[A0][0] = BP_SELF as u64;
        c.bus.write32(BP_CTRL, BP_MAGIC);
        c.bus.write32(BP_CTRL + 4, BP_ALLOC);
        c.bus.write32(BP_TICK_COUNT, 0);
        c.bus
            .write64(BP_TICK_BASE, BP_NOW as u64 * BP_CYCLES_PER_TICK);
        fill(c, &mut r, BP_SELF, 0x600 / 4);
        fill(c, &mut r, BP_ALLOC, 0xc0 / 4);
        let s = BP_SELF;
        for (off, v) in [
            (BUDDY_CONNECTOR, 0),
            (BUDDY_ARIES, BP_ARIES),
            (BUDDY_STATE, 0),
            (BUDDY_LOGIN_DEADLINE, 0),
            (BUDDY_PORT, 0x1f90 + seed),
            (BUDDY_ACCEPT_DOTTED, 0),
            (0x1c, 0),  // the presence token (`BuddyApi_SendPresence`)
            (0x308, 0), // the product count (`BuddyApi_ParsePresence`)
            (BUDDY_PRESENCE_DUE, 0),
            (BUDDY_ROSTER_HASH, 0),
            (BUDDY_ROSTER, 0),
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
        buddy_process_put_str(c, s + 0x240, b"PRES=1");
        buddy_process_put_str(c, s + BUDDY_LOGIN_TAGS, b"NAME=me");
        c.bus
            .write32(BP_CONN + CONNECTOR_ADDRESS, 0x0a00_0001 + seed);
        c.bus.write32(BP_CONN + CONNECTOR_POLL, BP_RETURN_ZERO);
        c.bus.write32(BP_CONN + CONNECTOR_CLOSE, BP_FN_CLOSE);
        c.bus.write32(BP_ARIES, 0x11);
        c.bus.write32(BP_ARIES + BP_ARIES_LINK, BP_LINK);
        c.bus.write16(BP_LINK, 2); // in use: `ProtoAriesConnect` returns -1 at once
        c.bus.write32(BP_ARIES + BP_ARIES_PENDING, 1);
        c.bus.write32(BP_ARIES + BP_ARIES_READ, 0);
        c.bus.write32(BP_ARIES + BP_ARIES_END, 0);
        c.bus.write32(BP_ARIES + BP_ARIES_SIZE, 0x1000);
        c.bus.write32(BP_ARIES + BP_ARIES_BUF, BP_BUF);
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
        buddy_process_put_str(c, at, name);
        c.bus.write8(at + 0x1f, 0);
        buddy_process_put_str(c, at + ROSTER_ENTRY_GROUP, b"grp");
        c.bus.write8(at + 0x3f, 0);
        buddy_process_put_str(c, at + ROSTER_ENTRY_PRODUCT, b"prod");
        buddy_process_put_str(c, at + ROSTER_ENTRY_PRESENCE, b"pres");
        buddy_process_put_str(c, at + ROSTER_ENTRY_LANG, b"en");
        c.bus.write8(at + ROSTER_ENTRY_SHOW, 3);
        c.bus.write32(at + ROSTER_ENTRY_FLAGS, flags | 0x40);
        c.bus.write32(at + ROSTER_ENTRY_ADD_ID, add_id);
    }

    /// The roster: amy (not adding), bob (adding, id 9), cat (adding, id 7),
    /// nul (no entry), in one hash bucket in that order (compared with
    /// `compare`), the list nodes in that order, two free hash entries and a
    /// free list node.
    pub(crate) fn buddy_process_roster(c: &mut Cpu, r: &mut Rng, compare: u32) {
        c.bus.write32(BP_SELF + BUDDY_ROSTER_HASH, BP_HASH);
        c.bus.write32(BP_SELF + BUDDY_ROSTER, BP_LIST);
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

    const BP_SET_STATE: u32 = callee::BUDDY_API_SET_STATE.addr;

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
            c.bus.write32(BP_SELF + BUDDY_CONNECTOR, BP_CONN);
        }
        let poll = [BP_RETURN_ZERO, BP_FN_POLL, BP_RETURN_MINUS_ONE][(seed / 4 % 3) as usize];
        c.bus.write32(BP_CONN + CONNECTOR_POLL, poll);
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
                        && run.called_with(callee::BUDDY_API_SEND_PACKET.addr, sx(KIND_AUTH))
                }),
                ("poll < 0: state 2", &|run: &BuddyProcessRun| {
                    run.called_with(BP_SET_STATE, BUDDY_STATE_POLL_FAILED)
                }),
                ("connector closed", &|run: &BuddyProcessRun| {
                    run.faked(BP_FN_CLOSE) && run.word(BP_SELF + BUDDY_CONNECTOR) == 0
                }),
                ("login timed out: state 3", &|run: &BuddyProcessRun| {
                    run.called_with(BP_SET_STATE, BUDDY_STATE_LOGIN_TIMEOUT)
                }),
            ],
        );
    }

    /// Op slots: N no fn, Z fn without deadline, F fn pending (deadline =
    /// now), P fn past its deadline (now - 1).
    const BUDDY_PROCESS_OP_PATTERNS: [&[u8; 3]; 6] =
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
                        && run.called_with(BP_SET_STATE, BUDDY_STATE_LINK_DOWN)
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
    /// Returns the packets and whether the roster exists.
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
                    KIND_PING,
                    v as u32,
                    if even { &b""[..] } else { &b"T=1"[..] },
                )],
                true,
            ),
            2 => {
                let code = [0, 5, 0, u32::MAX, 0, 1][v];
                (vec![p(KIND_AUTH, code, b"")], code == 0)
            }
            3 => (vec![p(KIND_RECV, 0, b"USER=bob@ea.com/ps2\nBODY=hi")], true),
            4 => (vec![p(KIND_SEND, v as u32, b"")], true),
            5 => (vec![p(KIND_ADMN, 0, b"MSG=down")], true),
            6 => {
                let texts: [&'static [u8]; 6] = [
                    b"ID=2\nSIZE=0",
                    b"ID=2\nSIZE=3",
                    b"ID=1\nSIZE=0",
                    b"",
                    b"ID=2",
                    b"SIZE=0",
                ];
                (vec![p(KIND_RGET, 0, texts[v])], true)
            }
            7 => {
                let texts: [&'static [u8]; 6] = [
                    b"ID=1\nUSER=bob@x.y/z\nGROUP=g1",
                    b"ID=2\nUSER=dan\nGROUP=g2",
                    b"ID=3\nUSER=e.f\nGROUP=g3",
                    b"ID=2\nUSER=g.h\nGROUP=g4",
                    b"ID=1\nUSER=bob",
                    b"ID=1\nUSER=nul\nGROUP=g5",
                ];
                (vec![p(KIND_ROST, 0, texts[v])], v != 4)
            }
            8 => {
                let packets = match v {
                    0 => vec![p(KIND_PGET, 0, b"USER=bob\nSHOW=CHAT\nPROD=PS2\nSTAT=hi")],
                    1 => vec![p(KIND_PGET, 0, b"USER=amy\nSHOW=AWAY")],
                    2 => vec![p(KIND_PGET, 0, b"USER=zed")],
                    3 => vec![p(KIND_PGET, 0, b"USER=bob")],
                    4 => vec![p(KIND_PGET, 0, b"USER=cat"), p(KIND_PGET, 0, b"USER=cat")],
                    _ => vec![p(KIND_PGET, 0, b"")],
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
                (vec![p(KIND_RADD, code, texts[v])], v != 5)
            }
            10 => {
                let texts: [&'static [u8]; 6] = [b"ID=5", b"ID=6", b"ID=5", b"", b"ID=5", b"ID=5"];
                (vec![p(KIND_RDEL, 0, texts[v])], true)
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
                    vec![p(KIND_RGET, 0, b"ID=2\nSIZE=1"), p(KIND_ROST, 0, texts[v])],
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
            c.bus.write32(s + BUDDY_OP_ADD + BUDDY_OP_FN, BP_FN_OP);
            c.bus
                .write32(s + BUDDY_OP_SEND + BUDDY_OP_FN, BP_FN_OP + 0x20);
        }
        if v % 3 != 2 {
            c.bus
                .write32(s + BUDDY_OP_DEL + BUDDY_OP_FN, BP_FN_OP + 0x10);
        }
        if kind == 7 {
            c.bus.write32(s + BUDDY_ROSTER_LEFT, [7, 1, 7, 5, 7, 7][v]);
            c.bus.write32(s + BUDDY_ACCEPT_DOTTED, (v == 3) as u32);
        }
        let (packets, roster) = buddy_process_message_packets(kind, v);
        if roster {
            buddy_process_roster(c, &mut r, BP_STRCMP);
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
                    run.seed % 13 == 0 && run.called_with(BP_SET_STATE, BUDDY_STATE_LINK_UP)
                }),
                ("link down: state 6", &|run: &BuddyProcessRun| {
                    run.seed % 13 == 0 && run.called_with(BP_SET_STATE, BUDDY_STATE_LINK_DOWN)
                }),
                ("PING answered", &|run: &BuddyProcessRun| {
                    run.called_with(callee::PROTO_ARIES_SEND.addr, sx(KIND_PING))
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
                        && run.word(BP_SELF + BUDDY_OP_SEND + BUDDY_OP_FN) == 0
                }),
                ("ADMN called back", &|run: &BuddyProcessRun| {
                    run.seed % 13 == 5 && run.faked(BP_FN_MSG)
                }),
                ("RGET size kept", &|run: &BuddyProcessRun| {
                    run.word(BP_SELF + BUDDY_ROSTER_LEFT) == 3
                }),
                ("ROST new entry", &|run: &BuddyProcessRun| {
                    run.called(callee::LOBBY_MEM_ALLOC)
                        && run.called(callee::LOBBY_HASH_ADD_STR.addr)
                }),
                ("ROST dot check", &|run: &BuddyProcessRun| {
                    run.called_with(callee::STRCHR, b'.' as u64)
                }),
                ("ROST group copied", &|run: &BuddyProcessRun| {
                    run.str_at(BP_BOB + ROSTER_ENTRY_GROUP, b"g1")
                }),
                ("ROST list-2 flag", &|run: &BuddyProcessRun| {
                    run.seed % 13 == 7
                        && run.word(BP_ALLOC + ROSTER_ENTRY_FLAGS) & ROSTER_FLAG_LIST2 as u32 != 0
                }),
                ("PGET parsed", &|run: &BuddyProcessRun| run.called(parse)),
                ("PGET changed", &|run: &BuddyProcessRun| {
                    run.called_with(printf, sx(FMT_PRESENCE))
                }),
                ("PGET unchanged", &|run: &BuddyProcessRun| {
                    run.count(parse) == 2 && run.count_with(printf, sx(FMT_PRESENCE)) < 2
                }),
                ("RADD refused: entry removed", &|run: &BuddyProcessRun| {
                    run.called(callee::LOBBY_LIST_REMOVE.addr) && run.called(callee::LOBBY_MEM_FREE)
                }),
                ("RADD renamed", &|run: &BuddyProcessRun| {
                    run.called_with(callee::TAG_FIELD_STRNZCPY.addr, ROSTER_NAME_SIZE)
                }),
                ("RADD add op called back", &|run: &BuddyProcessRun| {
                    run.seed % 13 == 9 && run.faked(BP_FN_OP)
                }),
                ("RDEL op called back", &|run: &BuddyProcessRun| {
                    run.faked(BP_FN_OP + 0x10)
                        && run.word(BP_SELF + BUDDY_OP_DEL + BUDDY_OP_FN) == 0
                }),
                ("roster complete: endr", &|run: &BuddyProcessRun| {
                    tag(run) == TAG_ENDR
                }),
                ("roster re-sorted: rost", &|run: &BuddyProcessRun| {
                    tag(run) == TAG_ROST
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
                buddy_process_packets(c, &[p(KIND_PGET, 0, b"")]);
            }
            _ => {
                buddy_process_roster(c, &mut r, BP_STRCMP);
                c.bus.write32(BP_BOB + ROSTER_ENTRY_ADD_ID, 0);
                c.bus.write32(s + BUDDY_OP_ADD + BUDDY_OP_FN, BP_FN_OP);
                buddy_process_packets(c, &[p(KIND_RADD, 0, b"")]);
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
}
