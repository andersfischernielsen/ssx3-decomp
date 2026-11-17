//! Staging module oc1060 (wave oc1060): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1060`.

#![allow(unused_imports)]

use crate::abi::{addu, r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
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
];

pub(crate) mod callee {
    use super::ee;
    use super::Callee;

    pub const TAG_FIELD_FIND: Callee = Callee {
        addr: 0x003e_b6d8,
        f: ee::s14::TagFieldFind,
    };

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
}

// ---------------------------------------------------------------------------
// The BuddyApi object (0x50c bytes, `BuddyApi_Create`).

/// The pending host lookup `Connect` starts (`SocketLookup`'s result), 0 when
/// none; its word at +0xc is the function that releases it.
const BUDDY_LOOKUP: u32 = 0x0;
/// In the lookup object: the function `Destroy` calls to release it.
const BUDDY_LOOKUP_RELEASE: u32 = 0xc;
/// The `ProtoAries` connection (`Create`).
const BUDDY_ARIES: u32 = 0x4;
/// The connection state (`SetState`).
const BUDDY_STATE: u32 = 0x8;
/// `Connect`'s deadline: the tick when it started plus the timeout in ms.
const BUDDY_CONNECT_DEADLINE: u32 = 0xc;
/// `Connect`'s third argument, stored as is.
const BUDDY_CONNECT_ARG: u32 = 0x10;
/// The op counter: `Create` starts it at 100, `SetPendingOp` bumps it.
const BUDDY_OP_COUNTER: u32 = 0x14;
/// `Config`'s last argument, stored as is.
const BUDDY_CONFIG_ARG: u32 = 0x18;
/// The product id (`Config`); presence packets carry it as `P`.
const BUDDY_PRODUCT: u32 = 0x1c;
/// The presence text, sent as `PROD` in presence packets.
const BUDDY_PRESENCE_TEXT: u32 = 0x40;
/// The status record (0x100 bytes), sent as `STAT`; it carries `P`.
const BUDDY_STATUS: u32 = 0x140;
/// The presence record `SendPresence` starts its packet from (0x200 bytes).
const BUDDY_PRESENCE: u32 = 0x240;
/// Cleared once `SendPresence` has sent.
const BUDDY_PRESENCE_PENDING: u32 = 0x2c0;
/// Set to -1 by `Connect`.
const BUDDY_WORD_2C4: u32 = 0x2c4;
/// `Config`'s first string (0x40 bytes), sent as `PRES` at connect.
const BUDDY_PRES: u32 = 0x2c8;
/// How many two-letter codes `Config` stored (at most 4).
const BUDDY_CODE_COUNT: u32 = 0x308;
/// The two-letter codes, one halfword each, first letter in the high byte.
const BUDDY_CODES: u32 = 0x30c;
/// `Create`'s first string (0x40 bytes), sent as `PROD` at connect.
const BUDDY_PRODUCT_NAME: u32 = 0x314;
/// `Create`'s second string (0x40 bytes), sent as `VERS` at connect.
const BUDDY_VERSION: u32 = 0x354;
/// The login record `Connect` builds (0x100 bytes).
const BUDDY_LOGIN: u32 = 0x394;
/// The user name `Connect` keeps (0x10 bytes): no leading `/`, nothing
/// from the next `/` on.
const BUDDY_NAME: u32 = 0x494;
/// Message callback context and function (`BuddyApi_SetCallback`).
const BUDDY_MSG_CB_CTX: u32 = 0x4ac;
const BUDDY_MSG_CB: u32 = 0x4b0;
/// State callback context and function (`Connect`'s last two arguments).
const BUDDY_STATE_CB_CTX: u32 = 0x4c0;
const BUDDY_STATE_CB: u32 = 0x4c4;
/// The pending-op slots `SetPendingOp` fills, one per op.
const BUDDY_PENDING_OPS: u32 = 0x4c8;
const BUDDY_PENDING_OP_SIZE: u32 = 0x14;
/// In a pending-op slot: the op's four-char code, its sequence number, its
/// deadline (0 for none) and the two arguments.
const PENDING_OP_CODE: u32 = 0x0;
const PENDING_OP_SEQ: u32 = 0x4;
const PENDING_OP_DEADLINE: u32 = 0x8;
const PENDING_OP_ARG3: u32 = 0xc;
const PENDING_OP_ARG2: u32 = 0x10;
/// Debug print hook: its context and function (0: no debug output).
const BUDDY_DEBUG_CTX: u32 = 0x504;
const BUDDY_DEBUG_HOOK: u32 = 0x508;
/// The object's size.
const BUDDY_SIZE: u32 = 0x50c;

/// Sizes of the strings `Create`/`Config` copy, and of the login record and
/// user name `Connect` builds.
const BUDDY_STRING_SIZE: u32 = 0x40;
const BUDDY_LOGIN_SIZE: u32 = 0x100;
const BUDDY_NAME_SIZE: u32 = 0x10;
/// The receive-buffer size `Create` asks `ProtoAriesCreate` for.
const BUDDY_ARIES_BUFFER: u32 = 0x8000;
/// The op counter's first value.
const BUDDY_FIRST_OP: u32 = 100;
/// The state `Connect` sets once the lookup is under way.
const BUDDY_STATE_CONNECT_STARTED: u32 = 1;
/// At most this many two-letter codes.
const BUDDY_MAX_CODES: i64 = 4;
/// Codes are lowercase letters: a byte below `'a'` ends them.
const BUDDY_CODE_FIRST_LETTER: i64 = b'a' as i64;
const BUDDY_MS_PER_SECOND: u64 = 1000;

/// A packet, message or event record: its four-char type, kind, code and
/// data pointer.
const BUDDY_PACKET_TYPE: u32 = 0x0;
const BUDDY_PACKET_KIND: u32 = 0x4;
const BUDDY_PACKET_CODE: u32 = 0x8;
const BUDDY_PACKET_DATA: u32 = 0xc;

/// Four-char codes, as the words the code compares and stores.
const BUDDY_TYPE_CHAT: u32 = 0x6368_6174; // "chat"
const BUDDY_EVENT_CONN: u32 = 0x636f_6e6e; // "conn"
const BUDDY_CODE_DISC: u32 = 0x6469_7363; // "disc"
const BUDDY_KIND_PSET: u32 = 0x5053_4554; // "PSET"
const BUDDY_CODE_NONE: u32 = 0x2d2d_2d2d; // "----"
const BUDDY_KIND_EVENT: u32 = 0x2a2a_2a2a; // "****"
/// The event code `DebugPacket` prints as "disc".
const BUDDY_CODE_DISCONNECTED: u32 = 0xfefe_fefe;

/// Tag names (game strings).
const BUDDY_TAG_PASS: u32 = 0x004a_4aa0; // "PASS"
const BUDDY_TAG_USER: u32 = 0x004a_4ae0; // "USER"
const BUDDY_EMPTY: u32 = 0x004a_4ae8; // ""
const BUDDY_TAG_DOMN: u32 = 0x004a_4af0; // "DOMN"
const BUDDY_TAG_RSRC: u32 = 0x004a_4af8; // "RSRC"
const BUDDY_TAG_SHOW: u32 = 0x004a_4b00; // "SHOW"
const BUDDY_TAG_PROD: u32 = 0x004a_4b08; // "PROD"
const BUDDY_TAG_STAT: u32 = 0x004a_4b10; // "STAT"
const BUDDY_TAG_P: u32 = 0x004a_4b18; // "P"
const BUDDY_TAG_VERS: u32 = 0x004a_4b20; // "VERS"
const BUDDY_TAG_PRES: u32 = 0x004a_4b28; // "PRES"
const BUDDY_TAG_LKEY: u32 = 0x004a_4b30; // "LKEY"

/// Debug formats (game strings).
const BUDDY_FMT_PACKET_HEADER: u32 = 0x0049_6b58; // "%s%c%c%c%c/%c%c%c%c\n"
const BUDDY_FMT_NULL_DATA: u32 = 0x0049_6b70; // " [null data]\n"
const BUDDY_FMT_BINARY_DATA: u32 = 0x0049_6b80; // " [binary data]\n"
const BUDDY_FMT_LONG_DATA: u32 = 0x0049_6b90; // " [long data]\n"
const BUDDY_FMT_LINE: u32 = 0x004a_4ad0; // "%s\n"
const BUDDY_FMT_SEND: u32 = 0x004a_4ad8; // "send: "

/// Game tables: the pending-op codes (`addu`, `delu`, `send`), the six
/// presence-state names (`DISC`, `CHAT`, `AWAY`, `XA`, `DND`, `PASS`) and
/// the product table `LookupProduct` searches.
const BUDDY_OP_CODES: u32 = 0x0045_12b0;
const BUDDY_STATE_NAMES: u32 = 0x0045_12c0;
const BUDDY_STATE_NAME_COUNT: i64 = 6;
const BUDDY_PRODUCT_TABLE: u32 = 0x0045_12d8;
/// A product-table entry: the product id, a code (halfword) and the name
/// pointer; the table ends at an id of 0.
const BUDDY_PRODUCT_ENTRY_SIZE: u32 = 0xc;
const PRODUCT_ENTRY_CODE: u32 = 0x4;
const PRODUCT_ENTRY_NAME: u32 = 0x8;

/// `lui` of an address or four-char code: the high half the register holds
/// before the `addiu`/`ori` (every low half here is below 0x8000).
fn buddy_lui(word: u32) -> u64 {
    sx(word & 0xffff_0000)
}

/// `addiu rd, base, %lo(addr)` from a register holding `addr`'s high half.
fn buddy_lo(base: u64, addr: u32) -> u64 {
    addu(base, (addr & 0xffff) as u64)
}

/// The field at `off` of the object whose address is in register `reg`.
fn buddy_field(c: &Cpu, reg: usize, off: u32) -> Addr {
    Addr(r32(c, reg).wrapping_add(off))
}

// ---------------------------------------------------------------------------
// BuddyApi_LookupProduct (0x3efe20)

const BUDDY_LOOKUP_ENTRY_HEAD: u32 = 0x003e_fe58;
const BUDDY_LOOKUP_CODE_HEAD: u32 = 0x003e_fe78;

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
const BUDDY_PRINTF_VARARGS: u32 = 0x1060;
const BUDDY_PRINTF_FLOATS: u32 = 0x1050;

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
        c.g[V0][0] = buddy_field(c, S0, BUDDY_DEBUG_HOOK).lw(c);
        if c.g[V0][0] != 0 {
            c.cycles += 4;
            c.g[A0][0] = buddy_field(c, S0, BUDDY_DEBUG_CTX).lw(c);
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

const BUDDY_DEBUG_PACKET_SCAN_HEAD: u32 = 0x003f_0010;
/// Data longer than this prints as "[long data]".
const BUDDY_DEBUG_MAX_TEXT: i64 = 513;
/// The first byte that ends printable data.
const BUDDY_DEBUG_DEL: i64 = 127;

/// `void BuddyApi_DebugPacket(BuddyApi* self, const char* prefix,
/// const Packet* packet)` (0x3eff40): print the packet's kind and code as
/// four characters each (an event, kind -1, prints as `****` with
/// `conn`/`disc`/`----`; a code of 0 as `----`), then its data: as text when
/// it is all printable (tab, LF and CR allowed) and at most 512 characters,
/// otherwise "[null data]", "[binary data]" or "[long data]".
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
        c.g[V0][0] = u64::MAX;
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
        c.g[V1][0] = buddy_field(c, S0, BUDDY_DEBUG_HOOK).lw(c);
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
const BUDDY_MSG_TAGS_SIZE: u32 = 0x800;
const BUDDY_MSG_USER: u32 = 0x800;
const BUDDY_MSG_USER_SIZE: u32 = 0x100;

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
        c.g[A3][0] = sx(BUDDY_EMPTY);
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
        c.g[V0][0] = buddy_field(c, S4, BUDDY_MSG_CB).lw(c);
        let callback = r32(c, V0);
        c.g[A2][0] = buddy_field(c, A0, BUDDY_MSG_CB_CTX).lw(c);
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

const BUDDY_PARSE_ENTRY: u32 = 0x003f_02d0;
/// Back-edge heads (the function is re-entered there after an unwind): the
/// state-name scan, a code found in the status record, and the code scan.
const BUDDY_PARSE_STATE_HEAD: u32 = 0x003f_0350;
const BUDDY_PARSE_CODE_FOUND: u32 = 0x003f_03f8;
const BUDDY_PARSE_CODE_HEAD: u32 = 0x003f_04b0;
const BUDDY_PARSE_END: u32 = 0x003f_0564;
const BUDDY_PARSE_FRAME: u32 = 0x1b0;
/// Stack buffers: the code being looked up (two letters and a NUL), the
/// `SHOW`/`STAT` text (0x100 bytes) and the found code as a halfword.
const BUDDY_PARSE_CODE_NAME: u32 = 0x0;
const BUDDY_PARSE_TEXT: u32 = 0x10;
const BUDDY_PARSE_TEXT_SIZE: u32 = 0x100;
const BUDDY_PARSE_CODE: u32 = 0x110;

/// The presence record `ParsePresence` fills: the product name (0x14
/// bytes), the presence text (0x50 bytes), the code's two letters and a
/// NUL, the presence-state index, and flags.
const PRESENCE_PRODUCT_NAME: u32 = 0x40;
const PRESENCE_PRODUCT_NAME_SIZE: u32 = 0x14;
const PRESENCE_TEXT: u32 = 0x54;
const PRESENCE_TEXT_SIZE: u32 = 0x50;
const PRESENCE_CODE_HI: u32 = 0xa4;
const PRESENCE_CODE_LO: u32 = 0xa5;
const PRESENCE_CODE_END: u32 = 0xa6;
const PRESENCE_STATE: u32 = 0xa7;
const PRESENCE_FLAGS: u32 = 0xa8;
/// Set when the packet has a `PROD` field.
const PRESENCE_HAS_TEXT: u64 = 0x2000;

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
                    c.g[S1][0] = buddy_lui(BUDDY_EMPTY);
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
                        PRESENCE_STATE,
                        PRESENCE_CODE_HI,
                        PRESENCE_TEXT,
                        PRESENCE_PRODUCT_NAME,
                    ] {
                        buddy_field(c, S2, off).sb(c, 0);
                    }
                    c.g[A0][0] = buddy_field(c, S6, BUDDY_PACKET_DATA).lw(c);
                    callee::TAG_FIELD_FIND.call(c, 0x003f_032c)?;
                    c.g[S0][0] = sx(frame.at(c, BUDDY_PARSE_TEXT));
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A2][0] = BUDDY_PARSE_TEXT_SIZE as u64;
                    c.g[A3][0] = buddy_lo(c.g[S1][0], BUDDY_EMPTY);
                    callee::TAG_FIELD_GET_STRING.call(c, 0x003f_0344)?;
                    c.g[S5][0] = c.g[S0][0];
                    c.g[S0][0] = buddy_lui(BUDDY_TAG_PROD);
                    BUDDY_PARSE_STATE_HEAD
                }
                BUDDY_PARSE_STATE_HEAD => {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[S3][0] as i64) < BUDDY_STATE_NAME_COUNT) as u64;
                    let past_end = c.g[V0][0] == 0;
                    c.g[V0][0] = buddy_lui(BUDDY_STATE_NAMES);
                    if !past_end {
                        c.cycles += 8;
                        c.g[V1][0] = sx(r32(c, S3) << 2);
                        c.g[V0][0] = sx(BUDDY_STATE_NAMES);
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
                        buddy_field(c, S2, PRESENCE_STATE).sb(c, c.g[S3][0]);
                    }
                    // The presence text.
                    c.cycles += 6;
                    c.g[A0][0] = buddy_field(c, S6, BUDDY_PACKET_DATA).lw(c);
                    c.g[A1][0] = buddy_lo(c.g[S0][0], BUDDY_TAG_PROD);
                    callee::TAG_FIELD_FIND.call(c, 0x003f_038c)?;
                    c.g[A0][0] = c.g[V0][0];
                    let no_text = c.g[A0][0] == 0;
                    c.g[A1][0] = addu(c.g[S2][0], PRESENCE_TEXT as u64);
                    if !no_text {
                        c.cycles += 3;
                        c.g[V0][0] = buddy_field(c, S2, PRESENCE_FLAGS).lw(c);
                        c.g[V0][0] |= PRESENCE_HAS_TEXT;
                        buddy_field(c, S2, PRESENCE_FLAGS).sw(c, c.g[V0][0]);
                    }
                    c.cycles += 8;
                    c.g[S0][0] = buddy_lo(c.g[S1][0], BUDDY_EMPTY);
                    c.g[S7][0] = c.g[A1][0];
                    c.g[A2][0] = PRESENCE_TEXT_SIZE as u64;
                    c.g[A3][0] = c.g[S0][0];
                    callee::TAG_FIELD_GET_STRING.call(c, 0x003f_03b8)?;
                    c.g[V1][0] = buddy_field(c, S2, PRESENCE_TEXT).lb(c);
                    let empty = c.g[V1][0] == 0;
                    c.g[A2][0] = addu(c.g[S4][0], BUDDY_CODES as u64);
                    if !empty {
                        // Text: our product, with our first code.
                        c.cycles += 13;
                        c.g[A1][0] = buddy_field(c, S4, BUDDY_PRODUCT).lw(c);
                        c.g[A0][0] = c.g[S4][0];
                        c.g[A3][0] = 1;
                        c.g[T0][0] = addu(c.g[S2][0], PRESENCE_PRODUCT_NAME as u64);
                        c.g[T1][0] = PRESENCE_PRODUCT_NAME_SIZE as u64;
                        callee::BUDDY_API_LOOKUP_PRODUCT.call(c, 0x003f_03dc)?;
                        c.g[V1][0] = buddy_field(c, S4, BUDDY_CODES).lhu(c);
                        c.g[V1][0] = sx(r32(c, V1) >> 8);
                        buddy_field(c, S2, PRESENCE_CODE_HI).sb(c, c.g[V1][0]);
                        c.g[V0][0] = buddy_field(c, S4, BUDDY_CODES).lh(c);
                        buddy_field(c, S2, PRESENCE_CODE_END).sb(c, 0);
                        buddy_field(c, S2, PRESENCE_CODE_LO).sb(c, c.g[V0][0]);
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
                    c.g[A3][0] = buddy_lo(c.g[S1][0], BUDDY_EMPTY);
                    c.g[A2][0] = PRESENCE_TEXT_SIZE as u64;
                    c.g[A1][0] = c.g[S7][0];
                    callee::TAG_FIELD_GET_STRING.call(c, 0x003f_0408)?;
                    c.g[A3][0] = Addr(frame.at(c, BUDDY_PARSE_CODE_NAME + 1)).lbu(c);
                    c.g[A0][0] = c.g[S4][0];
                    c.g[A2][0] = Addr(frame.at(c, BUDDY_PARSE_CODE_NAME)).lbu(c);
                    c.g[A1][0] = c.g[S6][0];
                    c.g[V0][0] = sx(r32(c, A3) << 24);
                    buddy_field(c, S2, PRESENCE_CODE_LO).sb(c, c.g[A3][0]);
                    c.g[V1][0] = sx(r32(c, A2) << 24);
                    c.g[V0][0] = sx((r32(c, V0) as i32 >> 8) as u32);
                    c.g[V0][0] |= c.g[V1][0];
                    buddy_field(c, S2, PRESENCE_CODE_HI).sb(c, c.g[A2][0]);
                    buddy_field(c, S2, PRESENCE_CODE_END).sb(c, 0);
                    c.g[V0][0] = sx((r32(c, V0) as i32 >> 16) as u32);
                    Addr(frame.at(c, BUDDY_PARSE_CODE)).sh(c, c.g[V0][0]);
                    c.g[A2][0] = sx(frame.at(c, BUDDY_PARSE_CODE));
                    c.g[A3][0] = 1;
                    c.g[T0][0] = addu(c.g[S2][0], PRESENCE_PRODUCT_NAME as u64);
                    c.g[T1][0] = PRESENCE_PRODUCT_NAME_SIZE as u64;
                    callee::BUDDY_API_LOOKUP_PRODUCT.call(c, 0x003f_0450)?;
                    c.g[V0][0] = buddy_field(c, S2, PRESENCE_CODE_HI).lb(c);
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
fn buddy_parse_presence_no_code(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 1;
    c.g[V0][0] = buddy_field(c, S2, PRESENCE_CODE_HI).lb(c);
    buddy_parse_presence_finish(c, frame)
}

/// `ParsePresence`'s finish (0x3f04f8, v0 = the code's first letter): with
/// no code yet, the product `P` (s6) with any of our codes.
fn buddy_parse_presence_finish(c: &mut Cpu, frame: Frame) -> Flow {
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
    c.g[T0][0] = addu(c.g[S2][0], PRESENCE_PRODUCT_NAME as u64);
    c.g[T1][0] = PRESENCE_PRODUCT_NAME_SIZE as u64;
    callee::BUDDY_API_LOOKUP_PRODUCT.call(c, 0x003f_051c)?;
    buddy_field(c, S2, PRESENCE_CODE_END).sb(c, 0);
    c.g[V1][0] = sx(r32(c, V0) << 16);
    Addr(frame.at(c, BUDDY_PARSE_CODE)).sh(c, c.g[V0][0]);
    c.g[V0][0] = sx((r32(c, V1) as i32 >> 24) as u32);
    c.g[V1][0] = sx((r32(c, V1) as i32 >> 16) as u32);
    buddy_field(c, S2, PRESENCE_CODE_HI).sb(c, c.g[V0][0]);
    buddy_field(c, S2, PRESENCE_CODE_LO).sb(c, c.g[V1][0]);
    buddy_parse_presence_leave(c, frame);
    Ok(())
}

/// `ParsePresence`'s common exit (0x3f0538).
fn buddy_parse_presence_leave(c: &mut Cpu, frame: Frame) {
    c.cycles += 1;
    frame.restore(c, S0, 0x1a0);
    buddy_parse_presence_epilogue(c, frame);
}

/// `ParsePresence`'s epilogue (0x3f053c; s0 is already restored).
fn buddy_parse_presence_epilogue(c: &mut Cpu, frame: Frame) {
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
const BUDDY_PRESENCE_PACKET_SIZE: u32 = 0x200;
const BUDDY_STATUS_SIZE: u32 = 0x100;

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
        buddy_field(c, S0, BUDDY_PRESENCE_PENDING).sw(c, 0);
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
        c.g[V1][0] = buddy_field(c, A0, BUDDY_STATE_CB).lw(c);
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
            c.g[A2][0] = buddy_field(c, A0, BUDDY_STATE_CB_CTX).lw(c);
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

/// `void BuddyApi_SetPendingOp(BuddyApi* self, int op, int arg2, int arg3,
/// int timeout)` (0x3f0658): fill op `op`'s pending slot with its four-char
/// code, the next op number, its deadline (the tick plus 1000 x `timeout`,
/// or 0 when `timeout` is not positive) and the two arguments.
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
        c.g[A0][0] = BUDDY_PENDING_OP_SIZE as u64;
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
        let slot = r32(c, V1).wrapping_add(BUDDY_PENDING_OPS);
        Addr(slot + PENDING_OP_CODE).sw(c, c.g[A1][0]);
        c.g[A3][0] = c.g[V1][0];
        c.g[A2][0] = c.g[V1][0];
        c.g[A0][0] = c.g[V1][0];
        c.g[V0][0] = buddy_field(c, S0, BUDDY_OP_COUNTER).lw(c);
        c.g[V0][0] = addu(c.g[V0][0], 1);
        buddy_field(c, S0, BUDDY_OP_COUNTER).sw(c, c.g[V0][0]);
        Addr(slot.wrapping_add(PENDING_OP_SEQ)).sw(c, c.g[V0][0]);
        Addr(slot.wrapping_add(PENDING_OP_ARG3)).sw(c, c.g[S3][0]);
        Addr(slot.wrapping_add(PENDING_OP_ARG2)).sw(c, c.g[S4][0]);
        frame.restore(c, S0, 0x50);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        Addr(slot.wrapping_add(PENDING_OP_DEADLINE)).sw(c, c.g[T0][0]);
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
            c.g[V0][0] = buddy_field(c, A0, BUDDY_LOOKUP_RELEASE).lw(c);
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

const BUDDY_CONFIG_CODE_HEAD: u32 = 0x003f_0860;

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
        buddy_field(c, S2, BUDDY_CONFIG_ARG).sw(c, c.g[S3][0]);
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
const BUDDY_CONNECT_SPLIT: u32 = 0x100;
const BUDDY_CONNECT_SPLIT_SIZE: u32 = 0x100;

/// `TagFieldSetString(self->login, 0x100, tag, value)` with the registers
/// `Connect` leaves at those calls (a0 = s4, the login record).
fn buddy_connect_set_login(c: &mut Cpu, tag: u32, value: u64, ret: u32) -> Flow {
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
        buddy_field(c, S3, BUDDY_STATE_CB_CTX).sw(c, c.g[S0][0]);
        buddy_field(c, S3, BUDDY_STATE_CB).sw(c, c.g[S2][0]);
        buddy_field(c, S3, BUDDY_CONNECT_ARG).sw(c, c.g[S1][0]);
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
                c.g[S0][0] = addu(c.g[S3][0], BUDDY_NAME as u64);
                break 'name;
            }
            c.cycles += 1;
            c.g[S0][0] = addu(c.g[S3][0], BUDDY_NAME as u64);
        }
        // The user name: no leading '/', nothing from the next '/' on.
        c.cycles += 8;
        c.g[A2][0] = c.g[S6][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = BUDDY_NAME_SIZE as u64;
        callee::TAG_FIELD_STRNZCPY.call(c, 0x003f_0ab0)?;
        c.g[V1][0] = buddy_field(c, S3, BUDDY_NAME).lb(c);
        c.g[V0][0] = b'/' as u64;
        let slash = c.g[V1][0] == c.g[V0][0];
        c.g[A0][0] = c.g[S0][0];
        if slash {
            c.cycles += 3;
            c.g[A1][0] = addu(c.g[S3][0], (BUDDY_NAME + 1) as u64);
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
        buddy_field(c, S3, BUDDY_CONNECT_DEADLINE).sw(c, c.g[V1][0]);
        buddy_field(c, S3, BUDDY_WORD_2C4).sw(c, c.g[V0][0]);
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::events::{check_events, driven, schedule, Events};
    use crate::difftest::fakes::{covered, fall_back, hook, machine, prev_cell, FAKE};
    use crate::difftest::{check, check_paths, elf_image, fill, require_paths, Rng, DATA};
    use std::ptr::addr_of_mut;

    /// Write `s` and a NUL at `at`.
    fn buddy_put_str(c: &mut Cpu, at: u32, s: &[u8]) {
        for (i, b) in s.iter().enumerate() {
            c.bus.write8(at + i as u32, *b);
        }
        c.bus.write8(at + s.len() as u32, 0);
    }

    /// A BuddyApi object at `at` with junk in every word.
    fn buddy_junk_object(c: &mut Cpu, r: &mut Rng, at: u32) {
        fill(c, r, at, BUDDY_SIZE / 4 + 1);
    }

    /// Junk in the stack below the test machine's `sp` (the functions'
    /// stack buffers), so a missing store or NUL shows.
    fn buddy_junk_stack(c: &mut Cpu, r: &mut Rng, bytes: u32) {
        fill(c, r, crate::difftest::SP - bytes, bytes / 4);
    }

    /// An Aries connection at `at` whose +4 points at a 0x20-byte buffer
    /// (`ProtoAriesUnconnect` clears it), stored in the object at DATA.
    fn buddy_aries(c: &mut Cpu, r: &mut Rng, at: u32) {
        fill(c, r, at, 0x10);
        c.bus.write32(at + 4, at + 0x100);
        fill(c, r, at + 0x100, 8);
        c.bus.write32(DATA + BUDDY_ARIES, at);
    }

    /// The roster list `FlushRoster` checks first: 0 takes its short path.
    const BUDDY_TEST_ROSTER: u32 = 0x4a8;

    /// The object at DATA for `Disconnect` (run by `Destroy` and `Connect`)
    /// on short paths: the Aries connection, no roster, and as asked a
    /// pending lookup (released through a fake), state 5 (Disconnect then
    /// sets 6 before 0), a state callback and the debug hook (fakes).
    fn buddy_connected_object(
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
        c.bus.write32(pending + BUDDY_LOOKUP_RELEASE, FAKE);
        c.bus
            .write32(DATA + BUDDY_LOOKUP, if lookup { pending } else { 0 });
        c.bus
            .write32(DATA + BUDDY_STATE, if state5 { 5 } else { 3 });
        c.bus
            .write32(DATA + BUDDY_STATE_CB, if cb { FAKE + 0x10 } else { 0 });
        c.bus.write32(DATA + BUDDY_TEST_ROSTER, 0);
        c.bus
            .write32(DATA + BUDDY_DEBUG_HOOK, if debug { FAKE + 0x20 } else { 0 });
    }

    // Scripted fakes for callees with override slots: call n returns script
    // word n (0 allowed), logs its target and a0..t0 and costs a fixed
    // BUDDY_FAKE_CYCLES, so arms through them have known cycle totals. Two
    // gates (magic words in the machine's RAM): the allocator group
    // (operator_new_named, EA_MemFree) and the tag group (TagFieldFind,
    // TagFieldGetString, strcmp, TagFieldGetToken).
    pub(crate) const BUDDY_FAKES_CTRL: u32 = DATA + 0xf_0000;
    pub(crate) const BUDDY_FAKES_MAGIC: u32 = 0xb0dd_7e57;
    const BUDDY_ALLOC_FAKES: u32 = BUDDY_FAKES_CTRL;
    const BUDDY_TAG_FAKES: u32 = BUDDY_FAKES_CTRL + 8;
    const BUDDY_FAKE_COUNT: u32 = BUDDY_FAKES_CTRL + 4;
    const BUDDY_FAKE_SCRIPT: u32 = BUDDY_FAKES_CTRL + 0x10;
    const BUDDY_FAKE_LOG: u32 = BUDDY_FAKES_CTRL + 0x100;
    const BUDDY_FAKE_CYCLES: u64 = 4;

    fn buddy_fake_call(c: &mut Cpu, target: u32) {
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

    fn buddy_gate(c: &mut Cpu, gate: u32) -> bool {
        c.bus.read32(gate) == BUDDY_FAKES_MAGIC
    }

    fn buddy_fake_new(c: &mut Cpu) {
        if !buddy_gate(c, BUDDY_ALLOC_FAKES) {
            return fall_back(c, prev_cell(buddy_fake_new), ee::s10::f_00317e30__generated);
        }
        buddy_fake_call(c, 0x0031_7e30);
    }

    fn buddy_fake_free(c: &mut Cpu) {
        if !buddy_gate(c, BUDDY_ALLOC_FAKES) {
            return fall_back(
                c,
                prev_cell(buddy_fake_free),
                ee::s10::f_00317e98__generated,
            );
        }
        buddy_fake_call(c, 0x0031_7e98);
    }

    fn buddy_fake_find(c: &mut Cpu) {
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
    fn buddy_fake_get_string(c: &mut Cpu) {
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

    fn buddy_fake_strcmp(c: &mut Cpu) {
        if !buddy_gate(c, BUDDY_TAG_FAKES) {
            return fall_back(
                c,
                prev_cell(buddy_fake_strcmp),
                ee::s15::f_004165a8__generated,
            );
        }
        buddy_fake_call(c, 0x0041_65a8);
    }

    fn buddy_fake_get_token(c: &mut Cpu) {
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
    fn buddy_scripted(c: &mut Cpu, gate: u32, script: &[u32]) {
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

    const BUDDY_TEST_PRODUCT: u32 = 0x5353_5833; // "SSX3"
    const BUDDY_TEST_OTHER_PRODUCT: u32 = 0x4e42_4133; // "NBA3"
    /// Codes "en", "fr" and one with the sign bit set (`lh`, not `lhu`).
    const BUDDY_TEST_CODES: [u16; 3] = [0x656e, 0x6672, 0xfffe];

    /// The product table with `entries` entries: entry i is the test product
    /// when bit i of `ids` is set (another product otherwise), with code
    /// `codes[i]` and a name of 5, 16 or 27 characters.
    fn buddy_product_table(c: &mut Cpu, r: &mut Rng, entries: u32, ids: u32, codes: &[u16]) {
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
            buddy_put_str(c, name, &vec![b'a' + i as u8; 5 + 11 * i as usize]);
        }
        c.bus
            .write32(BUDDY_PRODUCT_TABLE + BUDDY_PRODUCT_ENTRY_SIZE * entries, 0);
    }

    /// The arguments: our product, `wanted` codes (`count` of them), the
    /// name buffer (junk-filled) and its size.
    fn buddy_lookup_args(c: &mut Cpu, r: &mut Rng, wanted: &[u16], count: i64, size: u64) {
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
        check_events(
            buddy_gen_lookup_product,
            buddy_hand_lookup_product,
            36,
            &|c, seed| {
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
            },
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
                c.bus.write32(
                    DATA + BUDDY_DEBUG_HOOK,
                    if seed % 2 == 0 { 0 } else { FAKE },
                );
                let fmt = DATA + 0x1000;
                let text = DATA + 0x1100;
                buddy_put_str(c, text, b"rider");
                let format: &[u8] = match seed / 2 % 4 {
                    0 => b"plain text\n",
                    1 => b"%d:%s:%c",
                    2 => b"%x %d %d %d %d %d %d",
                    _ => b"",
                };
                buddy_put_str(c, fmt, format);
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
    /// byte >= 0x80 first, exactly 512 and 513 characters, DEL mid-way.
    fn buddy_debug_data(variant: u32) -> Vec<u8> {
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
            _ => b"abc\x7fdef".to_vec(),
        }
    }

    /// A packet at DATA+0x1100 with `kind`, `code` and `data` (at
    /// DATA+0x2000, junk after it), and the arguments.
    fn buddy_debug_packet_args(c: &mut Cpu, r: &mut Rng, kind: u32, code: u32, data: &[u8]) {
        let prefix = DATA + 0x1000;
        buddy_put_str(c, prefix, b"recv: ");
        let packet = DATA + 0x1100;
        let text = DATA + 0x2000;
        fill(c, r, text, 0x90);
        buddy_put_str(c, text, data);
        c.bus.write32(packet + BUDDY_PACKET_TYPE, r.next_u32());
        c.bus.write32(packet + BUDDY_PACKET_KIND, kind);
        c.bus.write32(packet + BUDDY_PACKET_CODE, code);
        c.bus.write32(packet + BUDDY_PACKET_DATA, text);
        c.g[A0][0] = sx(DATA);
        c.g[A1][0] = sx(prefix);
        c.g[A2][0] = sx(packet);
        c.pc = 0x003e_ff40;
    }

    // seed % 2: the debug hook; seed / 2 % 10: the data; seed / 3 % 3: the
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
                c.bus.write32(
                    DATA + BUDDY_DEBUG_HOOK,
                    if seed % 2 == 0 { 0 } else { FAKE },
                );
                let kind = [u32::MAX, BUDDY_KIND_PSET, 0x8a8b_8c8d][(seed / 3 % 3) as usize];
                let code =
                    [0, u32::MAX, BUDDY_CODE_DISCONNECTED, 0x6162_6364][(seed / 5 % 4) as usize];
                buddy_debug_packet_args(c, &mut r, kind, code, &buddy_debug_data(seed / 2 % 10));
            },
        );
    }

    driven!(buddy_gen_debug_packet, ee::s14::f_003eff40__generated);
    driven!(buddy_hand_debug_packet, buddy_api_debug_packet);

    // The data scan (0x3f0010, a structured loop: delivered, never an
    // unwind): 600 printable characters and the event due 2500..3700 cycles
    // in, meant to fall after the header's vsprintf and inside the scan.
    #[test]
    fn buddy_api_debug_packet_events() {
        check_events(
            buddy_gen_debug_packet,
            buddy_hand_debug_packet,
            30,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3e_ff41, 0);
                elf_image(c);
                buddy_junk_object(c, &mut r, DATA);
                c.bus.write32(DATA + BUDDY_DEBUG_HOOK, 0);
                buddy_debug_packet_args(c, &mut r, BUDDY_KIND_PSET, 0, &vec![b'q'; 600]);
                let mode = schedule(c, &mut r, seed, 0x003e_ff40);
                if mode != Events::Off {
                    c.next_event = c.cycles + 2500 + 300 * (seed / 3 % 5) as u64;
                }
                mode
            },
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
                c.bus.write32(
                    DATA + BUDDY_DEBUG_HOOK,
                    if seed % 2 == 0 { 0 } else { FAKE },
                );
                let data = DATA + 0x2000;
                let text: Vec<u8> = match seed / 2 % 3 {
                    0 => vec![],
                    1 => b"SHOW=CHAT\nSTAT=1\n".to_vec(),
                    _ => vec![b'z'; 600],
                };
                buddy_put_str(c, data, &text);
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
                buddy_put_str(c, name, text);
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
                c.bus.write32(DATA + BUDDY_MSG_CB, FAKE);
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
                buddy_put_str(c, tags, record);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(msg);
                c.pc = 0x003f_0198;
            },
        );
    }

    // -----------------------------------------------------------------------
    // BuddyApi_ParsePresence

    const BUDDY_TEST_FOUND: u32 = DATA + 0x5000;
    const BUDDY_TEST_TOKEN: u32 = BUDDY_TEST_PRODUCT;

    /// Scripted runs (tag fakes on): the callees' results in call order,
    /// our code count and codes, and the cycle total (fakes cost 4,
    /// `LookupProduct` with the empty table 15).
    ///
    /// Call order: find SHOW, get SHOW, strcmp per state name, find PROD,
    /// get text; with no text find STAT, get STAT, find P, get token, then
    /// one find per code and, for a found code, get its value.
    const BUDDY_PARSE_SCRIPTS: [(&[u32], u32, [u16; 3], u64); 8] = [
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
    fn buddy_parse_presence_base(c: &mut Cpu, r: &mut Rng) {
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

    fn buddy_parse_presence_scripted(c: &mut Cpu, r: &mut Rng, config: usize) {
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
    fn buddy_parse_presence_real(c: &mut Cpu, r: &mut Rng, seed: u32) {
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
        buddy_put_str(c, tags, record);
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
                c.bus.write32(
                    DATA + BUDDY_DEBUG_HOOK,
                    if seed % 2 == 0 { 0 } else { FAKE },
                );
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
                buddy_put_str(c, DATA + BUDDY_STATUS, status);
                let presence: &[u8] = if seed / 12 % 2 == 0 {
                    b"SHOW=CHAT\n"
                } else {
                    b""
                };
                buddy_put_str(c, DATA + BUDDY_PRESENCE, presence);
                let text: &[u8] = if seed / 24 % 2 == 0 {
                    b""
                } else {
                    b"Big Air at Peak 3"
                };
                buddy_put_str(c, DATA + BUDDY_PRESENCE_TEXT, text);
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
                .write32(DATA + BUDDY_STATE_CB, if seed % 2 == 0 { 0 } else { FAKE });
            c.g[A0][0] = sx(DATA);
            c.g[A1][0] = (seed / 2 % 7) as u64;
            c.pc = 0x003f_0610;
        };
        covered(ee::s14::f_003f0610__generated, buddy_api_set_state, 32, &setup);
        require_paths(ee::s14::f_003f0610__generated, 32, &[9], &setup);
    }

    // -----------------------------------------------------------------------
    // BuddyApi_SetPendingOp

    // seed % 3: the op; seed / 3 % 4: the timeout (0, -1, 1, 7; positive
    // ones run ProtoAriesTick for real). Path 41: no timeout.
    #[test]
    fn buddy_api_set_pending_op_matches() {
        check_paths(
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

    // seed % 3 == 0: the allocation fails (path 33: 12 + Lobby_MemAlloc's
    // 10 + the fake's 4 + 7). Otherwise the object (junk) is cleared and
    // filled; ProtoAriesCreate's own allocation is scripted to fail, so it
    // returns 0 (the only value the +4 store is checked with). String
    // lengths from seed / 3 % 3 (5, 63, 80; the copies keep 63).
    #[test]
    fn buddy_api_create_matches() {
        check_paths(
            ee::s14::f_003f0710__generated,
            buddy_api_create,
            30,
            &[33],
            &|c, seed| {
                let mut r = machine(c, seed, 0x3f_0710, 0);
                let object = DATA + 0x1_0000;
                fill(c, &mut r, object, BUDDY_SIZE / 4 + 1);
                let first = if seed % 3 == 0 { 0 } else { object };
                buddy_scripted(c, BUDDY_ALLOC_FAKES, &[first, 0]);
                let len = [5usize, 63, 80][(seed / 3 % 3) as usize];
                buddy_put_str(c, DATA, &vec![b'p'; len]);
                buddy_put_str(c, DATA + 0x100, &vec![b'v'; (len + 7) % 81]);
                c.g[A0][0] = sx(DATA);
                c.g[A1][0] = sx(DATA + 0x100);
                c.pc = 0x003f_0710;
            },
        );
    }

    // -----------------------------------------------------------------------
    // BuddyApi_Destroy

    // seed % 2: a pending lookup; seed / 2 % 2: state 5; seed / 4 % 2: a
    // state callback; seed / 8 % 2: the debug hook. Disconnect, FlushRoster
    // and ProtoAriesDestroy run real on short paths; the frees are scripted.
    #[test]
    fn buddy_api_destroy_matches() {
        covered(ee::s14::f_003f07a0__generated, buddy_api_destroy, 32, &|c, seed| {
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
        });
    }

    // -----------------------------------------------------------------------
    // BuddyApi_Config

    /// Code strings: four codes, two, one letter, empty, uppercase, '{' and
    /// '|' (>= 'a'), a byte >= 0x80 after a code, more than four codes.
    fn buddy_config_codes(variant: u32) -> &'static [u8] {
        match variant {
            0 => b"enfrdejp",
            1 => b"enfr",
            2 => b"e",
            3 => b"",
            4 => b"EN",
            5 => b"en{|",
            6 => b"en\x80a",
            _ => b"abcdefghij",
        }
    }

    /// The object, `pres` and the codes, and the arguments.
    fn buddy_config_args(c: &mut Cpu, r: &mut Rng, pres: &[u8], codes: &[u8]) {
        buddy_junk_object(c, r, DATA);
        let pres_at = DATA + 0x1000;
        let codes_at = DATA + 0x1100;
        fill(c, r, codes_at, 4);
        buddy_put_str(c, pres_at, pres);
        buddy_put_str(c, codes_at, codes);
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
        check_events(buddy_gen_config, buddy_hand_config, 24, &|c, seed| {
            let mut r = machine(c, seed, 0x3f_0819, 0);
            buddy_config_args(c, &mut r, b"", b"enfrdejp");
            let mode = schedule(c, &mut r, seed, 0x003f_0818);
            if mode != Events::Off {
                c.next_event = c.cycles + 1 + 12 * (seed / 3 % 8) as u64;
            }
            mode
        });
    }

    // -----------------------------------------------------------------------
    // BuddyApi_Connect

    /// Logins: a key, name:secret, name:$key, a ':' past the 255-character
    /// copy, an empty secret, a 16-character name with a secret.
    fn buddy_connect_login(variant: u32) -> Vec<u8> {
        match variant {
            0 => b"LOGINKEY123".to_vec(),
            1 => b"bob:secret".to_vec(),
            2 => b"bob:$keyvalue".to_vec(),
            3 => {
                let mut v = vec![b'a'; 300];
                v.extend_from_slice(b":x");
                v
            }
            4 => b"bob:".to_vec(),
            _ => b"riderfromthepeak:pw".to_vec(),
        }
    }

    /// Users: plain, a leading '/', a '/suffix', 250 characters (no room to
    /// append to the 16-character name).
    fn buddy_connect_user(variant: u32) -> Vec<u8> {
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
    // find no IOP and return).
    #[test]
    fn buddy_api_connect_matches() {
        covered(ee::s14::f_003f08e8__generated, buddy_api_connect, 96, &|c, seed| {
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
            buddy_put_str(c, DATA + BUDDY_PRES, b"Big Air");
            buddy_put_str(c, DATA + BUDDY_PRODUCT_NAME, b"SSX3");
            buddy_put_str(c, DATA + BUDDY_VERSION, b"1.0");
            let host = DATA + 0x4000;
            buddy_put_str(c, host, b"ps2ssx03.ea.com");
            let login = DATA + 0x4100;
            buddy_put_str(c, login, &buddy_connect_login(seed % 6));
            let user = DATA + 0x4300;
            buddy_put_str(c, user, &buddy_connect_user(seed / 6 % 4));
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
        });
    }
}
