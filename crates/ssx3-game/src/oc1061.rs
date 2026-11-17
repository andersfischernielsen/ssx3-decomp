//! Staging module oc1061 (wave oc1061): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1061`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
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
];

// ---------------------------------------------------------------------------
// The buddy API object (`BuddyApiT`, 0x50c bytes from `BuddyApi_Create`).

/// The pending host lookup (`SocketLookup` result, set by `BuddyApi_Connect`).
pub(crate) const BUDDY_API_LOOKUP: u32 = 0x0;
/// The lookup's release method.
pub(crate) const BUDDY_LOOKUP_RELEASE: u32 = 0xc;
/// The `ProtoAries` connection.
pub(crate) const BUDDY_API_ARIES: u32 = 0x4;
/// The connection state (`BuddyApi_SetState`).
pub(crate) const BUDDY_API_STATE: u32 = 0x8;
/// The request sequence counter `BuddyApi_SetPendingOp` advances.
pub(crate) const BUDDY_API_SEQUENCE: u32 = 0x14;
/// When zero, buddy names containing a '.' are refused.
pub(crate) const BUDDY_API_ALLOW_DOT: u32 = 0x18;
/// The presence text (0x100 bytes).
pub(crate) const BUDDY_API_PRESENCE_TEXT: u32 = 0x40;
/// The presence field record (0x100-byte tag record).
pub(crate) const BUDDY_API_PRESENCE_FIELDS: u32 = 0x140;
/// The presence status record (0x80-byte tag record: RSRC and SHOW).
pub(crate) const BUDDY_API_PRESENCE_STATUS: u32 = 0x240;
/// The tick at which the presence goes out (0: none scheduled).
pub(crate) const BUDDY_API_PRESENCE_DUE: u32 = 0x2c0;
/// The last presence show value.
pub(crate) const BUDDY_API_PRESENCE_SHOW: u32 = 0x2c4;
/// Byte: the presence text may be set.
pub(crate) const BUDDY_API_PRESENCE_TEXT_ON: u32 = 0x2c8;
/// The user name (the `LRSC` tag of roster requests).
pub(crate) const BUDDY_API_USER: u32 = 0x494;
/// The roster hash (name -> list node).
pub(crate) const BUDDY_API_ROSTER_HASH: u32 = 0x4a4;
/// The roster list (one record per buddy).
pub(crate) const BUDDY_API_ROSTER_LIST: u32 = 0x4a8;
/// The event callback's data and function (`BuddyApi_SetCallback`).
pub(crate) const BUDDY_API_CALLBACK_DATA: u32 = 0x4ac;
pub(crate) const BUDDY_API_CALLBACK_PROC: u32 = 0x4b0;
/// The roster callback's data and function (`BuddyApi_GetRoster`).
pub(crate) const BUDDY_API_ROSTER_DATA: u32 = 0x4b4;
pub(crate) const BUDDY_API_ROSTER_PROC: u32 = 0x4b8;
/// The state callback's function (`BuddyApi_SetState` calls it).
pub(crate) const BUDDY_API_STATE_PROC: u32 = 0x4c4;
/// The pending operations, one 0x14-byte slot per kind (add, delete,
/// send); each slot holds its request sequence number at +4.
pub(crate) const BUDDY_API_PENDING_OPS: u32 = 0x4c8;
pub(crate) const BUDDY_PENDING_OP_SIZE: u32 = 0x14;
pub(crate) const BUDDY_PENDING_OP_SEQ: u32 = 0x4;
/// Pending operation kinds (`BuddyApi_SetPendingOp`'s second argument).
pub(crate) const BUDDY_OP_ADD: u64 = 0;
pub(crate) const BUDDY_OP_DEL: u64 = 1;
pub(crate) const BUDDY_OP_SEND: u64 = 2;

/// A roster record (0xb8 bytes, copied from the caller's buddy).
/// Its name is at +0; the group string at +0x20.
pub(crate) const BUDDY_ENTRY_GROUP: u32 = 0x20;
/// The record's flags (bits 4, 0x200, 0x800000 pick the server lists).
pub(crate) const BUDDY_ENTRY_FLAGS: u32 = 0xa8;
/// The sequence number of the request that last changed the record.
pub(crate) const BUDDY_ENTRY_SEQ: u32 = 0xac;
pub(crate) const BUDDY_ENTRY_SIZE: u64 = 0xb8;
/// Record flags that keep a record in the roster after a delete.
pub(crate) const BUDDY_ENTRY_KEEP: u64 = 0x10_0204;
/// Set when a list add was sent for the record.
pub(crate) const BUDDY_ENTRY_SENT: u64 = 0x1_0000;
/// Flag bits for the roster lists the add/delete requests name.
pub(crate) const BUDDY_LIST_BIT_B: u64 = 0x4;
pub(crate) const BUDDY_LIST_BIT_I: u64 = 0x200;
pub(crate) const BUDDY_LIST_BIT_P: u64 = 0x80_0000;

/// `-2`: no roster or no such buddy; `-3`: refused name.
pub(crate) const BUDDY_ERR_NONE: u64 = 0xffff_ffff_ffff_fffe;
pub(crate) const BUDDY_ERR_NAME: u64 = 0xffff_ffff_ffff_fffd;

// Packet kinds (the word the transport sends), with the ASCII.
pub(crate) const BUDDY_PACKET_DISC: u64 = 0x4449_5343; // 'DISC'
pub(crate) const BUDDY_PACKET_RGET: u64 = 0x5247_4554; // 'RGET'
pub(crate) const BUDDY_PACKET_RADD: u64 = 0x5241_4444; // 'RADD'
pub(crate) const BUDDY_PACKET_PADD: u64 = 0x5041_4444; // 'PADD'
pub(crate) const BUDDY_PACKET_RDEL: u64 = 0x5244_454c; // 'RDEL'
pub(crate) const BUDDY_PACKET_PDEL: u64 = 0x5044_454c; // 'PDEL'
pub(crate) const BUDDY_PACKET_SEND: u64 = 0x5345_4e44; // 'SEND'
/// The roster callback's message kind ('rost').
pub(crate) const BUDDY_MSG_ROST_HI: u64 = 0x726f_0000; // 'ro..'
pub(crate) const BUDDY_MSG_ROST_LO: u64 = 0x7374; // '..st'

// Tag names and values in the game's data.
pub(crate) const BUDDY_STR_EMPTY: u32 = 0x004a_4ae8; // ""
pub(crate) const BUDDY_TAG_USER: u32 = 0x004a_4ae0; // "USER"
pub(crate) const BUDDY_TAG_RSRC: u32 = 0x004a_4af8; // "RSRC"
pub(crate) const BUDDY_TAG_SHOW: u32 = 0x004a_4b00; // "SHOW"
pub(crate) const BUDDY_TAG_PRES: u32 = 0x004a_4b28; // "PRES"
pub(crate) const BUDDY_TAG_LRSC: u32 = 0x004a_4b38; // "LRSC"
pub(crate) const BUDDY_TAG_LIST: u32 = 0x004a_4b40; // "LIST"
pub(crate) const BUDDY_STR_B: u32 = 0x004a_4b48; // "B"
pub(crate) const BUDDY_STR_Y: u32 = 0x004a_4b50; // "Y"
pub(crate) const BUDDY_TAG_ID: u32 = 0x004a_4b58; // "ID"
pub(crate) const BUDDY_STR_I: u32 = 0x004a_4b60; // "I"
pub(crate) const BUDDY_TAG_GROUP: u32 = 0x004a_4b68; // "GROUP"
pub(crate) const BUDDY_TAG_BODY: u32 = 0x004a_4b70; // "BODY"
pub(crate) const BUDDY_TAG_SUBJ: u32 = 0x004a_4b78; // "SUBJ"
/// The presence show names (6 string pointers; index 0 for out-of-range
/// shows).
pub(crate) const BUDDY_SHOW_NAMES: u32 = 0x0045_12c0;

/// A tag record buffer's size (the 0x100-byte stack records).
const BUDDY_RECORD_LEN: u64 = 0x100;

/// The word at `reg + off`.
#[inline]
fn buddy_word(c: &mut Cpu, reg: usize, off: u32) -> u64 {
    let a = r32(c, reg).wrapping_add(off);
    c.lw(a)
}

/// Store the low word of `v` at `reg + off`.
#[inline]
fn buddy_set_word(c: &mut Cpu, reg: usize, off: u32, v: u64) {
    let a = r32(c, reg).wrapping_add(off);
    c.sw(a, v);
}

/// `reg + off`, sign-extended (`addiu`).
#[inline]
fn buddy_addr(c: &Cpu, reg: usize, off: u32) -> u64 {
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
        c.g[A0][0] = buddy_word(c, S0, BUDDY_API_LOOKUP);
        c.g[A2][0] = 0x4a_0000;
        if c.g[A0][0] != 0 {
            c.cycles += 5;
            c.g[V0][0] = buddy_word(c, A0, BUDDY_LOOKUP_RELEASE);
            let release = r32(c, V0);
            call_ptr(c, release, 0x003f_0b78)?;
            buddy_set_word(c, S0, BUDDY_API_LOOKUP, 0);
            c.g[A2][0] = 0x4a_0000;
        }
        c.cycles += 11;
        c.g[A1][0] = BUDDY_PACKET_DISC;
        c.g[A2][0] = sx(BUDDY_STR_EMPTY);
        c.g[A0][0] = c.g[S0][0];
        callee::BUDDY_API_SEND_PACKET.call(c, 0x003f_0b94)?;
        c.g[A0][0] = buddy_word(c, S0, BUDDY_API_ARIES);
        callee::PROTO_ARIES_UNCONNECT.call(c, 0x003f_0b9c)?;
        c.g[V1][0] = buddy_word(c, S0, BUDDY_API_STATE);
        c.g[V0][0] = 5;
        c.g[A0][0] = c.g[S0][0];
        if c.g[V1][0] == c.g[V0][0] {
            c.cycles += 3;
            c.g[A1][0] = 6;
            callee::BUDDY_API_SET_STATE.call(c, 0x003f_0bb4)?;
            c.g[A0][0] = c.g[S0][0];
        }
        c.cycles += 10;
        c.g[A1][0] = 0;
        callee::BUDDY_API_SET_STATE.call(c, 0x003f_0bc0)?;
        buddy_set_word(c, S0, BUDDY_API_STATE_PROC, 0);
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
/// The pop loop's back edge checks events at its head 0x3f0c08 and carries
/// on; the head is also accepted as a resume entry.
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
                c.g[V0][0] = buddy_word(c, S0, BUDDY_API_ROSTER_LIST);
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
            c.g[A0][0] = buddy_word(c, S0, BUDDY_API_ROSTER_HASH);
            callee::LOBBY_HASH_POP_ANY.call(c, 0x003f_0c20)?;
            if c.g[V0][0] == 0 {
                break;
            }
            c.g[A0][0] = buddy_word(c, S0, BUDDY_API_ROSTER_LIST);
            if back_edge_unwinds(c, BUDDY_FLUSH_ROSTER_REMOVE) {
                return Ok(());
            }
            buddy_flush_roster_remove(c)?;
        }
        // Tell the roster callback.
        c.cycles += 3;
        c.g[A3][0] = buddy_word(c, S0, BUDDY_API_ROSTER_PROC);
        c.g[V0][0] = BUDDY_MSG_ROST_HI;
        if c.g[A3][0] != 0 {
            c.cycles += 10;
            c.g[V1][0] = buddy_word(c, S0, BUDDY_API_ROSTER_LIST);
            c.g[A2][0] = buddy_word(c, S0, BUDDY_API_ROSTER_DATA);
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
        c.g[A0][0] = buddy_word(c, S0, BUDDY_API_ROSTER_HASH);
        callee::LOBBY_HASH_DESTROY.call(c, 0x003f_0c64)?;
        buddy_set_word(c, S0, BUDDY_API_ROSTER_HASH, 0);
        c.g[A0][0] = buddy_word(c, S0, BUDDY_API_ROSTER_LIST);
        callee::LOBBY_LIST_DESTROY.call(c, 0x003f_0c70)?;
        buddy_set_word(c, S0, BUDDY_API_ROSTER_PROC, 0);
        buddy_set_word(c, S0, BUDDY_API_ROSTER_LIST, 0);
        buddy_set_word(c, S0, BUDDY_API_ROSTER_DATA, 0);
        frame.restore(c, S0, 0x20);
        buddy_flush_roster_return(c, frame);
        Ok(())
    })
}

/// The pop loop's body: remove the popped node (`v0`) from the list (`a0`)
/// and free the record it held.
fn buddy_flush_roster_remove(c: &mut Cpu) -> crate::call::Flow {
    c.cycles += 4;
    c.g[A1][0] = c.g[V0][0];
    callee::LOBBY_LIST_REMOVE.call(c, 0x003f_0c10)?;
    c.g[A0][0] = c.g[V0][0];
    call_ptr(c, callee::LOBBY_MEM_FREE, 0x003f_0c18)
}

/// `BuddyApi_FlushRoster`'s epilogue.
fn buddy_flush_roster_return(c: &mut Cpu, frame: Frame) {
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
        c.g[V0][0] = buddy_word(c, S6, BUDDY_API_ROSTER_LIST);
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
fn buddy_get_roster_request(c: &mut Cpu, frame: Frame) -> crate::call::Flow {
    c.cycles += 81;
    let (proc_, data) = (c.g[A1][0], c.g[A2][0]);
    buddy_set_word(c, S6, BUDDY_API_ROSTER_PROC, proc_);
    c.g[A0][0] = 1000;
    buddy_set_word(c, S6, BUDDY_API_ROSTER_DATA, data);
    c.g[A1][0] = 2000;
    callee::LOBBY_HASH_CREATE.call(c, 0x003f_0cd8)?;
    c.g[S5][0] = buddy_addr(c, S6, BUDDY_API_USER);
    c.g[A0][0] = 100;
    c.g[A1][0] = 100;
    c.g[A2][0] = 0;
    let hash = c.g[V0][0];
    buddy_set_word(c, S6, BUDDY_API_ROSTER_HASH, hash);
    callee::LOBBY_LIST_CREATE.call(c, 0x003f_0cf0)?;
    let list = c.g[V0][0];
    buddy_set_word(c, S6, BUDDY_API_ROSTER_LIST, list);
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
    c.g[A1][0] = BUDDY_PACKET_RGET;
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
    c.g[A1][0] = BUDDY_PACKET_RGET;
    c.g[A0][0] = c.g[S6][0];
    c.g[A2][0] = c.g[SP][0];
    callee::BUDDY_API_SEND_PACKET.call(c, 0x003f_0df4)?;
    c.g[A0][0] = buddy_word(c, S6, BUDDY_API_ROSTER_LIST);
    c.g[A1][0] = 1;
    callee::LOBBY_LIST_MARK_DIRTY.call(c, 0x003f_0e00)?;
    c.g[V0][0] = buddy_word(c, S6, BUDDY_API_ROSTER_LIST);
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
                    c.g[V0][0] = buddy_word(c, S0, BUDDY_ENTRY_FLAGS);
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
                    call_ptr(c, callee::STRCHR, 0x003f_0e8c)?;
                    let slash = c.g[V0][0] != 0;
                    c.g[V0][0] = BUDDY_ERR_NAME;
                    if slash {
                        break 'done;
                    }
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A1][0] = b'@' as u64;
                    call_ptr(c, callee::STRCHR, 0x003f_0ea0)?;
                    if c.g[V0][0] == 0 {
                        c.g[V0][0] = buddy_word(c, S1, BUDDY_API_ALLOW_DOT);
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
                call_ptr(c, callee::STRCHR, 0x003f_0ec4)?;
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
            c.g[A1][0] = BUDDY_OP_ADD;
            callee::BUDDY_API_SET_PENDING_OP.call(c, 0x003f_0ee4)?;
            c.g[V0][0] = buddy_word(c, S1, BUDDY_API_ROSTER_LIST);
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
fn buddy_add_roster_entry(c: &mut Cpu) -> crate::call::Flow {
    c.cycles += 5;
    c.g[A0][0] = buddy_word(c, S1, BUDDY_API_ROSTER_HASH);
    c.g[A1][0] = c.g[S0][0];
    callee::LOBBY_HASH_FIND_STR.call(c, 0x003f_0efc)?;
    if c.g[V0][0] != 0 {
        // Known: merge the flags.
        c.g[A0][0] = buddy_word(c, S1, BUDDY_API_ROSTER_LIST);
        c.cycles += 8;
        c.g[A1][0] = c.g[V0][0];
        callee::LOBBY_LIST_GET_DATA.call(c, 0x003f_1008)?;
        c.g[S2][0] = c.g[V0][0];
        c.g[V1][0] = buddy_word(c, S0, BUDDY_ENTRY_FLAGS);
        c.g[V0][0] = buddy_word(c, S2, BUDDY_ENTRY_FLAGS);
        c.g[V0][0] |= c.g[V1][0];
        let flags = c.g[V0][0];
        buddy_set_word(c, S2, BUDDY_ENTRY_FLAGS, flags);
        buddy_set_word(c, S2, BUDDY_ENTRY_SEQ, 0);
        return Ok(());
    }
    // New: copy the buddy into a fresh record, add it to the list and hash.
    c.cycles += 8;
    c.g[A0][0] = BUDDY_ENTRY_SIZE;
    call_ptr(c, callee::LOBBY_MEM_ALLOC, 0x003f_0f0c)?;
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
    c.g[A0][0] = buddy_word(c, S1, BUDDY_API_ROSTER_LIST);
    c.g[A1][0] = c.g[S2][0];
    callee::LOBBY_LIST_ADD.call(c, 0x003f_0fe8)?;
    c.g[A0][0] = buddy_word(c, S1, BUDDY_API_ROSTER_HASH);
    c.g[A2][0] = c.g[V0][0];
    c.g[A1][0] = c.g[S2][0];
    callee::LOBBY_HASH_ADD_STR.call(c, 0x003f_0ff8)?;
    buddy_set_word(c, S2, BUDDY_ENTRY_SEQ, 0);
    Ok(())
}

/// `ldl rt, 7(at); ldr rt, 0(at)`.
#[inline]
fn buddy_load_unaligned(c: &mut Cpu, rt: usize, at: u32) {
    c.ldl(rt, at.wrapping_add(7));
    c.ldr(rt, at);
}

/// `sdl rt, 7(at); sdr rt, 0(at)`.
#[inline]
fn buddy_store_unaligned(c: &mut Cpu, rt: usize, at: u32) {
    c.sdl(rt, at.wrapping_add(7));
    c.sdr(rt, at);
}

/// `BuddyApi_Add`'s requests for the record `s2` of the buddy `s0`.
fn buddy_add_send_lists(c: &mut Cpu) -> crate::call::Flow {
    let record = sp(c);
    // The B list (flag 4).
    c.cycles += 4;
    c.g[V0][0] = buddy_word(c, S0, BUDDY_ENTRY_FLAGS) & BUDDY_LIST_BIT_B;
    c.g[A0][0] = c.g[SP][0];
    if c.g[V0][0] != 0 {
        c.cycles += 49;
        c.sb(record, 0);
        c.g[A2][0] = sx(BUDDY_TAG_LRSC);
        c.g[A1][0] = BUDDY_RECORD_LEN;
        c.g[A3][0] = buddy_addr(c, S1, BUDDY_API_USER);
        callee::TAG_FIELD_SET_STRING.call(c, 0x003f_1048)?;
        c.g[A3][0] = buddy_word(c, S1, BUDDY_API_PENDING_OPS + BUDDY_PENDING_OP_SEQ);
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
        c.g[A3][0] = buddy_addr(c, S0, BUDDY_ENTRY_GROUP);
        callee::TAG_FIELD_SET_STRING.call(c, 0x003f_10c8)?;
        c.g[A1][0] = BUDDY_PACKET_RADD;
        c.g[A0][0] = c.g[S1][0];
        c.g[A2][0] = c.g[SP][0];
        callee::BUDDY_API_SEND_PACKET.call(c, 0x003f_10dc)?;
        buddy_add_mark_sent(c);
    }
    // The I list (flag 0x200).
    c.cycles += 4;
    c.g[V0][0] = buddy_word(c, S0, BUDDY_ENTRY_FLAGS) & BUDDY_LIST_BIT_I;
    c.g[A0][0] = c.g[SP][0];
    if c.g[V0][0] != 0 {
        c.cycles += 36;
        c.sb(record, 0);
        c.g[A2][0] = sx(BUDDY_TAG_LRSC);
        c.g[A1][0] = BUDDY_RECORD_LEN;
        c.g[A3][0] = buddy_addr(c, S1, BUDDY_API_USER);
        callee::TAG_FIELD_SET_STRING.call(c, 0x003f_111c)?;
        c.g[A3][0] = buddy_word(c, S1, BUDDY_API_PENDING_OPS + BUDDY_PENDING_OP_SEQ);
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
        c.g[A1][0] = BUDDY_PACKET_RADD;
        c.g[A0][0] = c.g[S1][0];
        c.g[A2][0] = c.g[SP][0];
        callee::BUDDY_API_SEND_PACKET.call(c, 0x003f_117c)?;
        buddy_add_mark_sent(c);
    }
    // The P list (flag 0x800000).
    c.cycles += 5;
    c.g[V0][0] = buddy_word(c, S0, BUDDY_ENTRY_FLAGS);
    c.g[V1][0] = BUDDY_LIST_BIT_P;
    c.g[V0][0] &= c.g[V1][0];
    c.g[A0][0] = c.g[SP][0];
    if c.g[V0][0] != 0 {
        c.cycles += 11;
        c.g[A2][0] = sx(BUDDY_TAG_USER);
        c.g[A1][0] = BUDDY_RECORD_LEN;
        c.g[A3][0] = c.g[S0][0];
        c.sb(record, 0);
        callee::TAG_FIELD_SET_STRING.call(c, 0x003f_11c0)?;
        c.g[A1][0] = BUDDY_PACKET_PADD;
        c.g[A0][0] = c.g[S1][0];
        c.g[A2][0] = c.g[SP][0];
        callee::BUDDY_API_SEND_PACKET.call(c, 0x003f_11d4)?;
    }
    Ok(())
}

/// After a 'RADD': flag the record (`s2`) sent and stamp it with the
/// current sequence number.
fn buddy_add_mark_sent(c: &mut Cpu) {
    c.g[V0][0] = buddy_word(c, S2, BUDDY_ENTRY_FLAGS);
    c.g[V1][0] = BUDDY_ENTRY_SENT;
    c.g[V0][0] |= c.g[V1][0];
    let flags = c.g[V0][0];
    buddy_set_word(c, S2, BUDDY_ENTRY_FLAGS, flags);
    c.g[V1][0] = buddy_word(c, S1, BUDDY_API_SEQUENCE);
    let seq = c.g[V1][0];
    buddy_set_word(c, S2, BUDDY_ENTRY_SEQ, seq);
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
                c.g[V0][0] = buddy_word(c, S0, BUDDY_API_ROSTER_LIST);
                let no_roster = c.g[V0][0] == 0;
                c.g[V0][0] = c.g[S5][0];
                if no_roster {
                    break 'done;
                }
                // The name checks.
                c.cycles += 5;
                c.g[A0][0] = c.g[S1][0];
                c.g[A1][0] = b'/' as u64;
                call_ptr(c, callee::STRCHR, 0x003f_124c)?;
                let slash = c.g[V0][0] != 0;
                c.g[V0][0] = BUDDY_ERR_NAME;
                if slash {
                    break 'done;
                }
                c.cycles += 5;
                c.g[A0][0] = c.g[S1][0];
                c.g[A1][0] = b'@' as u64;
                call_ptr(c, callee::STRCHR, 0x003f_1260)?;
                if c.g[V0][0] != 0 {
                    c.cycles += 2;
                    c.g[V0][0] = BUDDY_ERR_NAME;
                    break 'done;
                }
                c.g[V0][0] = buddy_word(c, S0, BUDDY_API_ALLOW_DOT);
                c.cycles += 2;
                let allow_dot = c.g[V0][0] != 0;
                c.g[T0][0] = c.g[S4][0];
                if !allow_dot {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[A1][0] = b'.' as u64;
                    call_ptr(c, callee::STRCHR, 0x003f_1284)?;
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
                c.g[A1][0] = BUDDY_OP_DEL;
                callee::BUDDY_API_SET_PENDING_OP.call(c, 0x003f_12a4)?;
                c.g[A0][0] = buddy_word(c, S0, BUDDY_API_ROSTER_HASH);
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
                c.g[A0][0] = buddy_word(c, S0, BUDDY_API_ROSTER_LIST);
                c.g[A1][0] = c.g[S4][0];
                callee::LOBBY_LIST_GET_DATA.call(c, 0x003f_12c8)?;
                c.g[S3][0] = c.g[V0][0];
                c.g[V0][0] = buddy_word(c, S1, BUDDY_ENTRY_FLAGS);
                c.g[V1][0] = buddy_word(c, S3, BUDDY_ENTRY_FLAGS);
                c.g[S2][0] = c.g[V1][0] & c.g[V0][0];
                c.g[V1][0] ^= c.g[S2][0];
                let kept = c.g[V1][0];
                buddy_set_word(c, S3, BUDDY_ENTRY_FLAGS, kept);
                buddy_del_send_lists(c)?;
                // Drop a record no list keeps.
                c.cycles += 6;
                c.g[V1][0] = buddy_word(c, S3, BUDDY_ENTRY_FLAGS);
                c.g[V0][0] = BUDDY_ENTRY_KEEP;
                c.g[V1][0] &= c.g[V0][0];
                let keep = c.g[V1][0] != 0;
                c.g[S5][0] = 0;
                if keep {
                    break 'result;
                }
                c.cycles += 9;
                c.g[A0][0] = buddy_word(c, S0, BUDDY_API_ROSTER_HASH);
                c.g[A1][0] = c.g[S1][0];
                callee::LOBBY_HASH_DEL_STR.call(c, 0x003f_1494)?;
                c.g[A0][0] = buddy_word(c, S0, BUDDY_API_ROSTER_LIST);
                c.g[A1][0] = c.g[S4][0];
                callee::LOBBY_LIST_REMOVE.call(c, 0x003f_14a0)?;
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, callee::LOBBY_MEM_FREE, 0x003f_14a8)?;
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
fn buddy_del_send_lists(c: &mut Cpu) -> crate::call::Flow {
    let record = sp(c);
    let del_seq = BUDDY_API_PENDING_OPS + BUDDY_PENDING_OP_SIZE + BUDDY_PENDING_OP_SEQ;
    // The B list (flag 4).
    c.g[V0][0] = c.g[S2][0] & BUDDY_LIST_BIT_B;
    if c.g[V0][0] != 0 {
        c.cycles += 44;
        c.g[A3][0] = buddy_word(c, S0, del_seq);
        c.g[A0][0] = c.g[SP][0];
        c.sb(record, 0);
        c.g[A2][0] = sx(BUDDY_TAG_ID);
        c.g[A1][0] = BUDDY_RECORD_LEN;
        callee::TAG_FIELD_SET_NUMBER.call(c, 0x003f_1304)?;
        c.g[A0][0] = c.g[SP][0];
        c.g[A2][0] = sx(BUDDY_TAG_LRSC);
        c.g[A3][0] = buddy_addr(c, S0, BUDDY_API_USER);
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
        c.g[A3][0] = buddy_addr(c, S1, BUDDY_ENTRY_GROUP);
        callee::TAG_FIELD_SET_STRING.call(c, 0x003f_1384)?;
        c.g[A1][0] = BUDDY_PACKET_RDEL;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[SP][0];
        callee::BUDDY_API_SEND_PACKET.call(c, 0x003f_1398)?;
    }
    // The I list (flag 0x200).
    c.cycles += 3;
    c.g[V0][0] = c.g[S2][0] & BUDDY_LIST_BIT_I;
    c.g[A0][0] = c.g[SP][0];
    if c.g[V0][0] != 0 {
        c.cycles += 30;
        c.g[A3][0] = buddy_word(c, S0, del_seq);
        c.sb(record, 0);
        c.g[A2][0] = sx(BUDDY_TAG_ID);
        c.g[A1][0] = BUDDY_RECORD_LEN;
        callee::TAG_FIELD_SET_NUMBER.call(c, 0x003f_13bc)?;
        c.g[A0][0] = c.g[SP][0];
        c.g[A2][0] = sx(BUDDY_TAG_LRSC);
        c.g[A3][0] = buddy_addr(c, S0, BUDDY_API_USER);
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
        c.g[A1][0] = BUDDY_PACKET_RDEL;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[SP][0];
        callee::BUDDY_API_SEND_PACKET.call(c, 0x003f_141c)?;
    }
    // The P list (flag 0x800000).
    c.cycles += 4;
    c.g[V0][0] = c.g[S2][0] & BUDDY_LIST_BIT_P;
    c.g[A0][0] = c.g[SP][0];
    if c.g[V0][0] != 0 {
        c.cycles += 17;
        c.g[A2][0] = sx(BUDDY_TAG_LRSC);
        c.sb(record, 0);
        c.g[A1][0] = BUDDY_RECORD_LEN;
        c.g[A3][0] = buddy_addr(c, S0, BUDDY_API_USER);
        callee::TAG_FIELD_SET_STRING.call(c, 0x003f_1444)?;
        c.g[A2][0] = sx(BUDDY_TAG_USER);
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] = BUDDY_RECORD_LEN;
        c.g[A3][0] = c.g[S1][0];
        callee::TAG_FIELD_SET_STRING.call(c, 0x003f_145c)?;
        c.g[A1][0] = BUDDY_PACKET_PDEL;
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
            c.g[A0][0] = buddy_word(c, S0, BUDDY_API_ROSTER_HASH);
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
                c.g[A0][0] = buddy_word(c, S0, BUDDY_API_ROSTER_LIST);
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
/// presence field record (+0x140) and text (+0x40).
pub fn buddy_api_clear_presence(c: &mut Cpu) {
    c.cycles += 3;
    let api = r32(c, A0);
    c.sb(api.wrapping_add(BUDDY_API_PRESENCE_FIELDS), 0);
    c.sb(api.wrapping_add(BUDDY_API_PRESENCE_TEXT), 0);
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
        c.g[V0][0] = c.lb(r32(c, A0).wrapping_add(BUDDY_API_PRESENCE_TEXT_ON));
        let off = c.g[V0][0] == 0;
        c.g[A2][0] = c.g[A1][0];
        if !off {
            c.cycles += 3;
            c.g[A0][0] = buddy_addr(c, A0, BUDDY_API_PRESENCE_TEXT);
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
/// const char* name)` (0x3f1578): store the tag `name=value` in the presence
/// field record (+0x140); only two-character names and non-empty values.
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
                c.g[A0][0] = buddy_addr(c, A0, BUDDY_API_PRESENCE_FIELDS);
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
/// int delay)` (0x3f15c8): rebuild the presence status record (+0x240:
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
        c.g[A0][0] = buddy_word(c, S1, BUDDY_API_ARIES);
        callee::PROTO_ARIES_TICK.call(c, 0x003f_1604)?;
        c.g[S3][0] = c.g[V0][0];
        // The show name.
        c.g[V0][0] = sx(r32(c, S2).wrapping_sub(1));
        let listed = c.g[V0][0] < 5;
        c.g[V0][0] = 0x45_0000;
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
        // The status record.
        c.cycles += 17;
        c.g[S0][0] = buddy_addr(c, S1, BUDDY_API_PRESENCE_STATUS);
        let status = r32(c, S0);
        c.sb(status, 0);
        c.g[A2][0] = sx(BUDDY_TAG_RSRC);
        c.g[A3][0] = c.g[S5][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 0x80;
        callee::TAG_FIELD_SET_STRING.call(c, 0x003f_1650)?;
        c.g[A2][0] = sx(BUDDY_TAG_SHOW);
        c.g[A0][0] = c.g[S0][0];
        c.g[A3][0] = c.g[S4][0];
        c.g[A1][0] = 0x80;
        callee::TAG_FIELD_SET_STRING.call(c, 0x003f_1668)?;
        // Schedule.
        c.g[V1][0] = buddy_word(c, S1, BUDDY_API_PRESENCE_DUE);
        if c.g[V1][0] == 0 {
            c.cycles += 3;
            c.g[V0][0] = sx(r32(c, S3).wrapping_add(r32(c, S6)));
            let due = c.g[V0][0];
            buddy_set_word(c, S1, BUDDY_API_PRESENCE_DUE, due);
        }
        c.g[V0][0] = buddy_word(c, S1, BUDDY_API_PRESENCE_SHOW);
        c.cycles += 2;
        if c.g[V0][0] != c.g[S2][0] {
            c.cycles += 3;
            let (show, now) = (c.g[S2][0], c.g[S3][0]);
            buddy_set_word(c, S1, BUDDY_API_PRESENCE_SHOW, show);
            buddy_set_word(c, S1, BUDDY_API_PRESENCE_DUE, now);
        }
        c.g[V0][0] = buddy_word(c, S1, BUDDY_API_PRESENCE_DUE);
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
                c.g[V1][0] = 0x4a_0000;
                c.g[A0][0] = c.g[V0][0];
                c.g[S1][0] = sx(BUDDY_STR_EMPTY);
                c.g[A1][0] = 0;
                c.g[A2][0] = 0;
                c.g[A3][0] = c.g[S1][0];
                callee::TAG_FIELD_GET_STRING.call(c, 0x003f_1738)?;
                c.g[V0][0] = ((c.g[V0][0] as i64) < 64) as u64;
                let too_long = c.g[V0][0] == 0;
                c.g[A1][0] = 0x4a_0000;
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
                c.g[A1][0] = BUDDY_OP_SEND;
                callee::BUDDY_API_SET_PENDING_OP.call(c, 0x003f_1784)?;
                c.g[A1][0] = BUDDY_PACKET_SEND;
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
    c.sw(api.wrapping_add(BUDDY_API_CALLBACK_DATA), c.g[A2][0]);
    c.sw(api.wrapping_add(BUDDY_API_CALLBACK_PROC), c.g[A1][0]);
    ret(c);
}

// ---------------------------------------------------------------------------
// Callees.

pub(crate) mod callee {
    use super::ee;
    use super::Callee;

    pub const LOBBY_MEM_ALLOC: u32 = 0x0025_91e8;
    pub const LOBBY_MEM_FREE: u32 = 0x0025_9210;
    pub const STRCHR: u32 = 0x0041_6404;

    pub const TAG_FIELD_FIND: Callee = Callee {
        addr: 0x003e_b6d8,
        f: ee::s14::TagFieldFind,
    };
    pub const TAG_FIELD_STRNZCPY: Callee = Callee {
        addr: 0x003e_bb60,
        f: ee::s14::TagFieldStrnzcpy,
    };
    pub const TAG_FIELD_SET_NUMBER: Callee = Callee {
        addr: 0x003e_be40,
        f: ee::s14::TagFieldSetNumber,
    };
    pub const TAG_FIELD_SET_STRING: Callee = Callee {
        addr: 0x003e_c180,
        f: ee::s14::TagFieldSetString,
    };
    pub const TAG_FIELD_GET_STRING: Callee = Callee {
        addr: 0x003e_d378,
        f: ee::s14::TagFieldGetString,
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
    pub const BUDDY_API_SEND_PACKET: Callee = Callee {
        addr: 0x003f_00d8,
        f: ee::s14::BuddyApi_SendPacket,
    };
    pub const BUDDY_API_SEND_PRESENCE: Callee = Callee {
        addr: 0x003f_0568,
        f: ee::s14::BuddyApi_SendPresence,
    };
    pub const BUDDY_API_SET_STATE: Callee = Callee {
        addr: 0x003f_0610,
        f: ee::s14::BuddyApi_SetState,
    };
    pub const BUDDY_API_SET_PENDING_OP: Callee = Callee {
        addr: 0x003f_0658,
        f: ee::s14::BuddyApi_SetPendingOp,
    };
    pub const BUDDY_API_FLUSH_ROSTER: Callee = Callee {
        addr: 0x003f_0be0,
        f: ee::s14::BuddyApi_FlushRoster,
    };
    pub const PROTO_ARIES_UNCONNECT: Callee = Callee {
        addr: 0x003f_5738,
        f: ee::s14::ProtoAriesUnconnect,
    };
    pub const PROTO_ARIES_TICK: Callee = Callee {
        addr: 0x003f_5d88,
        f: ee::s14::ProtoAriesTick,
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::events::{check_events, driven, schedule, Events};
    use crate::difftest::fakes::{self, machine, FAKE, LOG_RESULTS, LOG_START};
    use crate::difftest::{check, check_paths, elf_image, fill, fresh, require_paths, Rng, DATA};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    // Fixture layout (all inside DATA..DATA+0xfffff, above the fakes'
    // default results).
    const BUDDY_T_CTRL: u32 = DATA + 0xf_f000;
    const BUDDY_T_MAGIC: u32 = 0xb0dd_1e51;
    const BUDDY_T_API: u32 = DATA + 0x4_0000;
    const BUDDY_T_ARIES: u32 = DATA + 0x4_1000;
    const BUDDY_T_HASH: u32 = DATA + 0x4_2000;
    const BUDDY_T_LIST: u32 = DATA + 0x4_3000;
    const BUDDY_T_RECORDS: u32 = DATA + 0x4_4000;
    const BUDDY_T_BUDDY: u32 = DATA + 0x4_8000;
    const BUDDY_T_LOOKUP: u32 = DATA + 0x4_9000;
    const BUDDY_T_TEXT: u32 = DATA + 0x4_a000;
    /// Forced allocation results, 0x4000 apart.
    const BUDDY_T_HEAP: u32 = DATA + 0x5_0000;
    /// The transport's debug-dump flag (kept off: `BuddyApi_DebugPacket`).
    const BUDDY_T_DEBUG: u32 = 0x508;
    /// The token `BuddyApi_SendPresence` adds when non-zero.
    const BUDDY_T_PRESENCE_TOKEN: u32 = 0x1c;
    /// `Lobby_HashStrCmpNoCase`, the hash's compare function.
    const BUDDY_T_HASH_CMP: u32 = 0x003e_eab8;
    /// Roster names (distinct under the hash's case folding).
    const BUDDY_T_NAMES: [&[u8]; 5] = [b"Kaori", b"mac", b"Elise", b"zoe", b"Moby"];

    static BUDDY_PREV_NEW: AtomicUsize = AtomicUsize::new(0);
    static BUDDY_PREV_FREE: AtomicUsize = AtomicUsize::new(0);

    /// `operator_new_named` under the buddy fixtures: a logged fake whose
    /// result the fixture forces (`LOG_RESULTS`).
    fn buddy_fake_new(c: &mut Cpu) {
        if c.bus.read32(BUDDY_T_CTRL) != BUDDY_T_MAGIC {
            return fakes::fall_back(c, &BUDDY_PREV_NEW, ee::s10::f_00317e30__generated);
        }
        let ra = r32(c, RA);
        fakes::fake(c, 0x0031_7e30, ra);
    }

    /// `EA_MemFree` under the buddy fixtures: logged, nothing freed.
    fn buddy_fake_free(c: &mut Cpu) {
        if c.bus.read32(BUDDY_T_CTRL) != BUDDY_T_MAGIC {
            return fakes::fall_back(c, &BUDDY_PREV_FREE, ee::s10::f_00317e98__generated);
        }
        let ra = r32(c, RA);
        fakes::fake(c, 0x0031_7e98, ra);
    }

    /// Hook the allocator fakes and turn them on for this machine.
    fn buddy_fakes_on(c: &mut Cpu) {
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E30),
            buddy_fake_new,
            &BUDDY_PREV_NEW,
        );
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E98),
            buddy_fake_free,
            &BUDDY_PREV_FREE,
        );
        c.bus.write32(BUDDY_T_CTRL, BUDDY_T_MAGIC);
    }

    fn buddy_put_str(c: &mut Cpu, at: u32, s: &[u8]) {
        for (i, &b) in s.iter().enumerate() {
            c.bus.write8(at + i as u32, b);
        }
        c.bus.write8(at + s.len() as u32, 0);
    }

    /// The bucket `Lobby_HashString` picks for `name`.
    fn buddy_t_bucket(name: &[u8], buckets: u32) -> u32 {
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
    fn buddy_api_fixture(c: &mut Cpu, seed: u32, entry: u32, calls: u32, elf: bool) -> Rng {
        let mut r = machine(c, seed, entry as u64, calls);
        if elf {
            elf_image(c);
        }
        buddy_fakes_on(c);
        let api = BUDDY_T_API;
        fill(c, &mut r, api, 0x510 / 4);
        c.bus.write32(api + BUDDY_API_ARIES, BUDDY_T_ARIES);
        fill(c, &mut r, BUDDY_T_ARIES, 0x48);
        c.bus.write32(BUDDY_T_ARIES + 4, BUDDY_T_ARIES + 0x100);
        c.bus.write32(api + BUDDY_T_DEBUG, 0);
        c.bus.write32(api + BUDDY_T_PRESENCE_TOKEN, 0);
        buddy_put_str(c, api + BUDDY_API_USER, b"skier");
        buddy_put_str(c, api + BUDDY_API_PRESENCE_TEXT, b"riding");
        buddy_put_str(c, api + BUDDY_API_PRESENCE_FIELDS, b"LV=3");
        buddy_put_str(c, api + BUDDY_API_PRESENCE_STATUS, b"SHOW=chat");
        c.bus.write32(api + BUDDY_API_ROSTER_HASH, 0);
        c.bus.write32(api + BUDDY_API_ROSTER_LIST, 0);
        c.g[A0][0] = api as u64;
        c.pc = entry;
        r
    }

    /// A roster of `count` records (names from BUDDY_T_NAMES, flags
    /// `flags`) in a Lobby hash of `buckets` chains with its pop cursor at
    /// `cursor`, and a Lobby list whose first `visible` entries are
    /// visible; both pools keep free nodes (no growing).
    fn buddy_roster_fixture(
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
            buddy_put_str(c, rec, name);
            buddy_put_str(c, rec + BUDDY_ENTRY_GROUP, b"crew");
            c.bus.write32(rec + BUDDY_ENTRY_FLAGS, flags);
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
        c.bus.write32(BUDDY_T_API + BUDDY_API_ROSTER_HASH, hash);
        c.bus.write32(BUDDY_T_API + BUDDY_API_ROSTER_LIST, list);
    }

    /// A caller's buddy at `at`: random bytes, then `name`, the group and
    /// `flags`.
    fn buddy_t_buddy(c: &mut Cpu, r: &mut Rng, at: u32, name: &[u8], flags: u32) {
        fill(c, r, at, 0x30);
        buddy_put_str(c, at, name);
        buddy_put_str(c, at + BUDDY_ENTRY_GROUP, b"lodge");
        c.bus.write32(at + BUDDY_ENTRY_FLAGS, flags);
    }

    /// Disconnect: with and without a pending lookup, state 5 or another,
    /// with and without a state callback, and no roster / an empty roster /
    /// three buddies with a roster callback. SendPacket, Unconnect,
    /// SetState and FlushRoster run real; the lookup's release, the
    /// callbacks and the frees are fakes (some unwind).
    #[test]
    fn buddy_api_disconnect_matches() {
        fakes::covered(ee::s14::f_003f0b50__generated, buddy_api_disconnect, 96, &|c, seed| {
            let mut r = buddy_api_fixture(c, seed, 0x003f_0b50, 13, false);
            let api = BUDDY_T_API;
            let lookup = if seed % 2 == 1 { BUDDY_T_LOOKUP } else { 0 };
            fill(c, &mut r, BUDDY_T_LOOKUP, 8);
            c.bus
                .write32(BUDDY_T_LOOKUP + BUDDY_LOOKUP_RELEASE, FAKE + 0x20);
            c.bus.write32(api + BUDDY_API_LOOKUP, lookup);
            let state = if seed / 2 % 2 == 1 { 5 } else { 3 };
            c.bus.write32(api + BUDDY_API_STATE, state);
            let state_proc = if seed / 4 % 2 == 1 { FAKE + 0x10 } else { 0 };
            c.bus.write32(api + BUDDY_API_STATE_PROC, state_proc);
            match seed / 8 % 3 {
                0 => {}
                1 => {
                    buddy_roster_fixture(c, &mut r, 0, 2, 1, 0, 4);
                    c.bus.write32(api + BUDDY_API_ROSTER_PROC, 0);
                }
                _ => {
                    buddy_roster_fixture(c, &mut r, 3, 2, 1, 2, 4);
                    c.bus.write32(api + BUDDY_API_ROSTER_PROC, FAKE + 0x30);
                }
            }
        });
    }

    /// The flush loop's setup: no list (10 cycles), an empty roster, or 1-4
    /// buddies in 1-4 buckets with the pop cursor on bucket 0 or 1, with
    /// and without a roster callback. Frees and the callback are fakes.
    fn buddy_flush_roster_setup(c: &mut Cpu, seed: u32, calls: u32) -> Rng {
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
        c.bus.write32(BUDDY_T_API + BUDDY_API_ROSTER_PROC, proc_);
        r
    }

    #[test]
    fn buddy_api_flush_roster_matches() {
        let setup = |c: &mut Cpu, seed: u32| {
            buddy_flush_roster_setup(c, seed, 11);
        };
        fakes::covered(ee::s14::f_003f0be0__generated, buddy_api_flush_roster, 192, &setup);
        require_paths(ee::s14::f_003f0be0__generated, 192, &[10], &setup);
    }

    /// Four buddies; the first event lands on the pop loop's head 0x3f0c08
    /// (k up to ~37 cycles after entry) or on a later pass of it (or in a
    /// callee's own loop, delivered there).
    #[test]
    fn buddy_api_flush_roster_events() {
        driven!(gen, ee::s14::f_003f0be0__generated);
        driven!(hand, buddy_api_flush_roster);
        check_events(gen, hand, 60, &|c, seed| {
            let mut r = buddy_api_fixture(c, seed, 0x003f_0be0, 0, false);
            let buckets = 1 + seed / 3 % 4;
            buddy_roster_fixture(c, &mut r, 4, buckets, seed / 12 % buckets, 2, 4);
            c.bus
                .write32(BUDDY_T_API + BUDDY_API_ROSTER_PROC, FAKE + 0x30);
            schedule(c, &mut r, seed, 0x003f_0be0)
        });
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
                c.bus
                    .write32(BUDDY_T_API + BUDDY_API_ROSTER_LIST, BUDDY_T_LIST);
            }
            for i in 0..6 {
                let block = BUDDY_T_HEAP + 0x4000 * i;
                fill(c, &mut r, block, 0x10);
                c.bus.write32(LOG_RESULTS + 4 * i, block);
            }
            c.g[A1][0] = (FAKE + 0x40) as u64;
            c.g[A2][0] = r.next_u32() as u64;
        };
        fakes::covered(ee::s14::f_003f0c90__generated, buddy_api_get_roster, 32, &setup);
        require_paths(ee::s14::f_003f0c90__generated, 32, &[22], &setup);
    }

    /// Add's buddy kinds (`kind`): 0 null (23 cycles), 1 no flags (24),
    /// 2 empty name (29), 3 '/', 4 '@', 5 '.', 6 a known name in other
    /// case, 7 a new name copied aligned, 8 from a 4-aligned buddy, 9 to a
    /// 4-aligned record. `roster`: 0 none, 1 three buddies in one bucket,
    /// 2 in four buckets. The record allocation is a fake with a forced
    /// result.
    fn buddy_add_setup(
        c: &mut Cpu,
        seed: u32,
        kind: u32,
        allow_dot: u32,
        roster: u32,
        flags: u32,
        calls: u32,
    ) -> Rng {
        let mut r = buddy_api_fixture(c, seed, 0x003f_0e38, calls, true);
        c.bus.write32(BUDDY_T_API + BUDDY_API_ALLOW_DOT, allow_dot);
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
    const BUDDY_T_ADD_FLAGS: [u32; 4] = [0x1, 0x80_0004, 0x200, 0x80_0204];

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
    fn buddy_add_alloc_cycles(setup: &dyn Fn(&mut Cpu)) -> u64 {
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
        check_events(gen, hand, 60, &|c, seed| {
            let mode = state(c, seed);
            if mode != Events::Off {
                let at = buddy_add_alloc_cycles(&|p| {
                    state(p, seed);
                });
                c.next_event = at + 22;
            }
            mode
        });
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
                c.bus
                    .write32(BUDDY_T_API + BUDDY_API_ALLOW_DOT, seed / 6 % 2);
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
                buddy_put_str(c, BUDDY_T_TEXT, name);
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
                fill(c, &mut r, api + BUDDY_API_PRESENCE_TEXT, 0x40);
                let on = if seed % 2 == 1 {
                    1 + r.next_u32() % 0xff
                } else {
                    0
                };
                c.bus.write8(api + BUDDY_API_PRESENCE_TEXT_ON, on as u8);
                let len = [0, 5, 255, 300][(seed / 2 % 4) as usize];
                let text: Vec<u8> = (0..len).map(|i| b'a' + (i % 26) as u8).collect();
                buddy_put_str(c, BUDDY_T_TEXT, &text);
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
                buddy_put_str(c, BUDDY_T_TEXT, name);
                buddy_put_str(c, BUDDY_T_TEXT + 0x10, value);
                c.g[A1][0] = (BUDDY_T_TEXT + 0x10) as u64;
                c.g[A2][0] = BUDDY_T_TEXT as u64;
            },
        );
    }

    /// SetPresence: shows 0..6 and -1 (in and out of the table), a send
    /// due unset, at 1 or far ahead, delay 0 or large, and the last show
    /// equal or not, so the presence goes out or waits. The tick, the tag
    /// records and SendPresence run real.
    #[test]
    fn buddy_api_set_presence_matches() {
        check(
            ee::s14::f_003f15c8__generated,
            buddy_api_set_presence,
            96,
            &|c, seed| {
                buddy_api_fixture(c, seed, 0x003f_15c8, 0, true);
                let api = BUDDY_T_API;
                let show = [0, 1, 2, 3, 4, 5, 6, 0xffff_ffff][(seed % 8) as usize];
                let due = [0, 1, 0x7fff_ffff][(seed / 8 % 3) as usize];
                c.bus.write32(api + BUDDY_API_PRESENCE_DUE, due);
                let last = if seed / 48 % 2 == 0 { show } else { show ^ 1 };
                c.bus.write32(api + BUDDY_API_PRESENCE_SHOW, last);
                buddy_put_str(c, BUDDY_T_TEXT, b"lodge");
                c.g[A1][0] = BUDDY_T_TEXT as u64;
                c.g[A2][0] = sx(show);
                c.g[A3][0] = if seed / 24 % 2 == 0 { 0 } else { 0x7fff_0000 };
            },
        );
    }

    /// Send: BODY absent or of 5, 62, 63 or 64 characters; SUBJ absent,
    /// empty or set; timeout 0 or 500. Tags, the pending op and the packet
    /// run real.
    #[test]
    fn buddy_api_send_matches() {
        check(ee::s14::f_003f16d8__generated, buddy_api_send, 60, &|c, seed| {
            let mut r = buddy_api_fixture(c, seed, 0x003f_16d8, 0, true);
            let mut msg: Vec<u8> = b"USER=mac ".to_vec();
            if let Some(len) = [None, Some(5), Some(62), Some(63), Some(64)][(seed % 5) as usize] {
                msg.extend_from_slice(b"BODY=");
                msg.extend((0..len).map(|i| b'a' + (i % 26) as u8));
                msg.push(b' ');
            }
            msg.extend_from_slice([&b""[..], b"SUBJ= ", b"SUBJ=hey "][(seed / 5 % 3) as usize]);
            buddy_put_str(c, BUDDY_T_TEXT, &msg);
            c.g[A1][0] = BUDDY_T_TEXT as u64;
            c.g[A2][0] = r.next_u32() as u64;
            c.g[A3][0] = r.next_u32() as u64;
            c.g[T0][0] = if seed / 15 % 2 == 0 { 0 } else { 500 };
        });
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
}
