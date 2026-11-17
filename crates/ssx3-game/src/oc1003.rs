//! Staging module oc1003 (wave wave): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1003`.

#![allow(unused_imports)]

use crate::abi;
use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0025_e590,
        lobby_man_on_snapshot_reply,
        "LobbyMan_OnSnapshotReply",
    ),
    (
        0x0025_e420,
        lobby_man_send_snapshot,
        "LobbyMan_SendSnapshot",
    ),
    (
        0x0025_e348,
        lobby_man_request_stats_page,
        "LobbyMan_RequestStatsPage",
    ),
    (0x0025_e138, lobby_man_on_news_reply, "LobbyMan_OnNewsReply"),
    (
        0x0025_df98,
        lobby_man_on_report_user_reply,
        "LobbyMan_OnReportUserReply",
    ),
    (0x0025_df00, lobby_man_report_user, "LobbyMan_ReportUser"),
    (0x0025_dde0, lobby_man_on_ping_reply, "LobbyMan_OnPingReply"),
    (
        0x0025_dd68,
        lobby_man_ping_selected_room_user,
        "LobbyMan_PingSelectedRoomUser",
    ),
    (
        0x0025_db80,
        lobby_man_on_create_room_reply,
        "LobbyMan_OnCreateRoomReply",
    ),
    (
        0x0025_d8f8,
        lobby_man_on_move_to_room_reply,
        "LobbyMan_OnMoveToRoomReply",
    ),
];

/// Callees used by the lobby snapshot/news/ping reply handlers in this module.
pub(crate) mod callee {
    use super::ee;
    use super::Callee;

    /// `cMemMan_alloc` (0x317d70), via `call_indirect`.
    pub const EA_MEM_ALLOC_NAMED: u32 = 0x0031_7d70;
    /// `TagFieldFind` (0x3eb6d8), via `call_indirect`.
    pub const TAG_FIELD_FIND: u32 = 0x003e_b6d8;
    /// `TagFieldGetNumber` (0x3ecf30), via `call_indirect`.
    pub const TAG_FIELD_GET_NUMBER: u32 = 0x003e_cf30;
    /// `TagFieldGetString4` (0x3ed378), via `call_indirect`.
    pub const TAG_FIELD_GET_STRING: u32 = 0x003e_d378;
    /// `TagFieldSetNumber` (0x3ebe40), via `call_indirect`.
    pub const TAG_FIELD_SET_NUMBER: u32 = 0x003e_be40;
    /// `TagFieldSetString` (0x3ec180), via `call_indirect`.
    pub const TAG_FIELD_SET_STRING: u32 = 0x003e_c180;
    /// `cBXString` assign-from-C-string (0x3184e8), via `call_indirect`.
    pub const C_BX_STRING_ASSIGN_CSTR: u32 = 0x0031_84e8;
    /// `cBXString` construct-from-C-string (0x3182d8), via `call_indirect`.
    pub const C_BX_STRING_CTOR_CSTR: u32 = 0x0031_82d8;
    /// `cBXString_Reset` (0x3180b8), via `call_indirect`.
    pub const C_BX_STRING_RELEASE: u32 = 0x0031_80b8;
    /// Lobby list visible-entry getter (0x3ee560), via `call_indirect`.
    pub const LOBBY_LIST_GET_AT: u32 = 0x003e_e560;
    /// Lobby peer ping sender (0x3e8dc0), via `call_indirect`.
    pub const LOBBY_API_PING: u32 = 0x003e_8dc0;
    /// Snapshot slot-tag lookup (0x3e8f08), via `call_indirect`.
    pub const LOBBY_API_STATUS: u32 = 0x003e_8f08;
    /// Snapshot handle release (0x3e8688), via `call_indirect`.
    pub const LOBBY_API_DESTROY_LIST: u32 = 0x003e_8688;
    /// Snapshot handle create (0x3e85b0), via `call_indirect`.
    pub const LOBBY_API_CREATE_LIST: u32 = 0x003e_85b0;
    /// Snapshot limit apply (0x3e8d70), via `call_indirect`.
    pub const LOBBY_API_CANCEL_REQUEST: u32 = 0x003e_8d70;

    pub const LOBBY_MAN_SEND_REQUEST: Callee = Callee {
        addr: 0x0025_b608,
        f: ee::s06::LobbyMan_SendRequest,
    };
    pub const LOBBY_MAN_GET_LANGUAGE: Callee = Callee {
        addr: 0x0025_9198,
        f: ee::s06::LobbyMan_GetLanguage,
    };
    pub const LOBBY_MAN_LOOKUP_PEER_ADDR: Callee = Callee {
        addr: 0x0025_a730,
        f: ee::s06::LobbyMan_LookupPeerAddr,
    };
    pub const BUDDY_MAN_IS_CONNECTED: Callee = Callee {
        addr: 0x0026_1460,
        f: ee::s06::BuddyMan_IsConnected,
    };
    pub const BUDDY_MAN_CONNECT: Callee = Callee {
        addr: 0x0026_1248,
        f: ee::s06::BuddyMan_Connect,
    };
    pub const LOBBY_MAN_GET_RANK_LIST_COUNT: Callee = Callee {
        addr: 0x0025_f338,
        f: ee::s06::LobbyMan_GetRankListCount,
    };
    pub const LOBBY_MAN_ON_RANK_LIST_CHANGED: Callee = Callee {
        addr: 0x0025_9a60,
        f: ee::s06::LobbyMan_OnRankListChanged,
    };
    pub const LOBBY_MAN_SEND_SNAPSHOT: Callee = Callee {
        addr: 0x0025_e420,
        f: ee::s06::f_0025e420,
    };
}

/// Tag key for the first score value in `LobbyMan_OnSnapshotReply`.
pub(crate) const ON_SNAPSHOT_REPLY_TAG_A: u32 = 0x004a_3198;
/// Tag key for the slot index in `LobbyMan_OnSnapshotReply`.
pub(crate) const ON_SNAPSHOT_REPLY_TAG_IDX: u32 = 0x004a_31a8;
/// Tag key for the second score value in `LobbyMan_OnSnapshotReply`.
pub(crate) const ON_SNAPSHOT_REPLY_TAG_B: u32 = 0x004a_31a0;

/// Splice `node` (in `a1`) into the pending-message list whose head pointer
/// was saved at `temp_off(sp)`, mirroring the generated insert sequence;
/// the iterator copy lands at `out_off(sp)`. Reads guest state via `c`.
fn msg_list_insert_splice(c: &mut Cpu, temp_off: u32, out_off: u32) {
    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(temp_off));
    c.sw(c.g[A1][0] as u32, c.g[V0][0]);
    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(4));
    c.sw((c.g[A1][0] as u32).wrapping_add(4), c.g[V1][0]);
    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(4));
    c.sw(c.g[A0][0] as u32, c.g[A1][0]);
    c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(temp_off));
    c.sw((c.g[V0][0] as u32).wrapping_add(4), c.g[A1][0]);
    c.sw((c.g[SP][0] as u32).wrapping_add(out_off), c.g[A1][0]);
}

/// `void LobbyMan_OnSnapshotReply(LobbyMan *self, Msg *msg)` (0x25e590):
/// a snapshot reply either stores the summed score tags at
/// `self + idx*4 + 0x104`, or queues an error message on the pending list.
pub fn lobby_man_on_snapshot_reply(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S0, 0x60);
        frame.save(c, S3, 0x30);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S1, 0x50);
        frame.save_ra(c, 0x20);
        frame.save(c, S2, 0x40);
        c.g[S2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
        let is_error = c.g[S2][0] != 0;
        c.g[S3][0] = c.g[A0][0];
        if !is_error {
            c.cycles += 28;
            let msg = c.g[S0][0] as u32;
            c.g[A0][0] = c.lw(msg.wrapping_add(0xc));
            c.g[A1][0] = sx(ON_SNAPSHOT_REPLY_TAG_A);
            call_ptr(c, callee::TAG_FIELD_FIND, 0x0025_e5c8)?;
            c.g[A1][0] = 0;
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, callee::TAG_FIELD_GET_NUMBER, 0x0025_e5d4)?;
            c.g[A0][0] = c.lw(msg.wrapping_add(0xc));
            c.g[A1][0] = 0x4a0000;
            c.g[S1][0] = c.g[V0][0];
            c.g[A1][0] = sx(ON_SNAPSHOT_REPLY_TAG_IDX);
            call_ptr(c, callee::TAG_FIELD_FIND, 0x0025_e5e8)?;
            c.g[A1][0] = 0;
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, callee::TAG_FIELD_GET_NUMBER, 0x0025_e5f4)?;
            c.g[A0][0] = c.lw(msg.wrapping_add(0xc));
            c.g[A1][0] = 0x4a0000;
            c.g[S0][0] = c.g[V0][0];
            c.g[A1][0] = sx(ON_SNAPSHOT_REPLY_TAG_B);
            call_ptr(c, callee::TAG_FIELD_FIND, 0x0025_e608)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            call_ptr(c, callee::TAG_FIELD_GET_NUMBER, 0x0025_e614)?;
            c.g[S0][0] = sx((c.g[S0][0] as u32) << 2);
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[S0][0] = sx((c.g[S3][0] as u32).wrapping_add(c.g[S0][0] as u32));
            c.sw((c.g[S0][0] as u32).wrapping_add(0x104), c.g[S1][0]);
            frame.restore(c, S0, 0x60);
            frame.restore(c, S1, 0x50);
            frame.restore(c, S2, 0x40);
            frame.restore(c, S3, 0x30);
            c.cycles += 7;
            frame.restore_ra(c, 0x20);
            frame.close(c);
            return Ok(());
        }
        c.cycles += 25;
        c.g[A1][0] = 0x480000;
        c.g[A3][0] = 0;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x488));
        c.g[A0][0] = 0xc;
        c.g[A2][0] = 0x20000000;
        call_ptr(c, callee::EA_MEM_ALLOC_NAMED, 0x0025_e640)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[A3][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xf0));
        c.g[V0][0] = 0x480000;
        c.g[V1][0] = 0x102;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1290));
        c.sw(c.g[S1][0] as u32, c.g[V1][0]);
        c.sw((c.g[S1][0] as u32).wrapping_add(4), c.g[V0][0]);
        c.sw((c.g[S1][0] as u32).wrapping_add(8), c.g[S2][0]);
        c.g[A1][0] = 0x480000;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4a8));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[A3][0]);
        c.g[A0][0] = 0xc;
        c.g[A2][0] = 0x20000000;
        c.g[A3][0] = 0;
        call_ptr(c, callee::EA_MEM_ALLOC_NAMED, 0x0025_e67c)?;
        c.g[A1][0] = c.g[V0][0];
        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(8));
        if c.g[V0][0] != 0 {
            c.sw((c.g[A1][0] as u32).wrapping_add(8), c.g[S1][0]);
        }
        c.cycles += 9;
        msg_list_insert_splice(c, 0x10, 0);
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        c.cycles += 7;
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `void LobbyMan_SendSnapshot(LobbyMan *self, int a1, int a2, int a3)`
/// (0x25e420): send the snapshot request for the current slot, creating the
/// slot handle on first use and stamping the slot limit afterwards.
pub fn lobby_man_send_snapshot(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x160);
        frame.save(c, S1, 0x140);
        frame.save(c, S2, 0x130);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S3, 0x120);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S4, 0x110);
        c.g[S3][0] = c.g[A2][0];
        frame.save(c, S0, 0x150);
        frame.save_ra(c, 0x100);
        c.g[V0][0] = c.lw(c.g[S1][0] as u32);
        let inactive = c.g[V0][0] == 0;
        c.g[S4][0] = c.g[A3][0];
        if !inactive {
            c.cycles += 7;
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xf8));
            c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x1ec));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[V1][0] = c.lw(c.g[V0][0] as u32);
            if c.g[V1][0] != 0 {
                c.g[A3][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xf8));
            } else {
                c.cycles += 11;
                c.g[A2][0] = 0x270000;
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x54));
                c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x7520));
                c.g[A1][0] = 0x4;
                c.g[A3][0] = c.g[S1][0];
                call_ptr(c, callee::LOBBY_API_CREATE_LIST, 0x0025_e488)?;
                c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xf8));
                c.g[V1][0] = sx((c.g[V1][0] as u32) << 2);
                c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V1][0] as u32));
                c.sw(c.g[V1][0] as u32, c.g[V0][0]);
                c.g[A3][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xf8));
            }
            c.cycles += 52;
            let buf = c.g[SP][0];
            c.g[A2][0] = sx(0x004a_3188);
            c.g[A0][0] = buf;
            Addr(buf as u32).sb(c, 0);
            c.g[A1][0] = 0x100;
            call_ptr(c, callee::TAG_FIELD_SET_NUMBER, 0x0025_e4b4)?;
            c.g[S0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x178));
            c.g[A3][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x90));
            c.g[A2][0] = sx(0x004a_3190);
            c.g[A0][0] = buf;
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(4));
            c.g[A1][0] = 0x100;
            call_ptr(c, callee::TAG_FIELD_SET_NUMBER, 0x0025_e4d4)?;
            c.g[A2][0] = sx(0x004a_3198);
            c.g[A0][0] = buf;
            c.g[A3][0] = c.g[S2][0];
            c.g[A1][0] = 0x100;
            call_ptr(c, callee::TAG_FIELD_SET_NUMBER, 0x0025_e4ec)?;
            c.g[A2][0] = sx(0x004a_31a0);
            c.g[A0][0] = buf;
            c.g[A3][0] = c.g[S3][0];
            c.g[A1][0] = 0x100;
            call_ptr(c, callee::TAG_FIELD_SET_NUMBER, 0x0025_e504)?;
            c.g[A3][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xf8));
            c.g[A2][0] = sx(0x004a_31a8);
            c.g[A0][0] = buf;
            c.g[A1][0] = 0x100;
            call_ptr(c, callee::TAG_FIELD_SET_NUMBER, 0x0025_e51c)?;
            c.g[V1][0] = sx((c.g[S4][0] as u32) << 2);
            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x54));
            c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[A1][0] = c.lw(c.g[V1][0] as u32);
            call_ptr(c, callee::LOBBY_API_CANCEL_REQUEST, 0x0025_e530)?;
            c.g[T0][0] = 0x270000;
            c.g[A1][0] = c.lw(c.g[S1][0] as u32);
            c.g[A2][0] = 0x736e0000;
            c.g[AT][0] = 0x41a00000;
            c.mtc1(AT, 12);
            c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x7200));
            c.g[A0][0] = c.g[S1][0];
            c.g[A2][0] |= 0x6170;
            c.g[A3][0] = buf;
            callee::LOBBY_MAN_SEND_REQUEST.call(c, 0x0025_e558)?;
            c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0xf8));
            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x40));
            c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.sw(c.g[S0][0] as u32, c.g[V1][0]);
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

/// `void LobbyMan_RequestStatsPage(LobbyMan *self, int slot)` (0x25e348):
/// switch the stats slot, release any stale handle, then either request a
/// fresh snapshot or notify that the rank list changed.
pub fn lobby_man_request_stats_page(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = c.lw(c.g[S0][0] as u32);
        let inactive = c.g[V0][0] == 0;
        c.g[S1][0] = c.g[A1][0];
        if !inactive {
            c.cycles += 15;
            c.g[S2][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xf8));
            c.g[A1][0] = 0x736c0000;
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x54));
            c.g[A1][0] |= 0x6f74;
            c.sw((c.g[S0][0] as u32).wrapping_add(0xf8), c.g[S1][0]);
            call_ptr(c, callee::LOBBY_API_STATUS, 0x0025_e384)?;
            c.g[A0][0] = c.g[S0][0];
            c.sw((c.g[S0][0] as u32).wrapping_add(0x100), c.g[V0][0]);
            callee::LOBBY_MAN_GET_RANK_LIST_COUNT.call(c, 0x0025_e390)?;
            c.g[V1][0] = sx((c.g[S1][0] as u32) << 2);
            c.g[V1][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[V1][0] as u32));
            c.g[A0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x104));
            c.g[V0][0] = (c.g[V0][0] < c.g[A0][0]) as u64;
            let below_limit = c.g[V0][0] != 0;
            c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1ec));
            if below_limit {
                c.cycles += 6;
                c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xf8));
                c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[A2][0] = c.lw(c.g[V0][0] as u32);
                let has_handle = c.g[A2][0] != 0;
                c.g[A0][0] = c.g[S0][0];
                if has_handle {
                    c.cycles += 8;
                    c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x54));
                    c.g[A1][0] = 0x4;
                    call_ptr(c, callee::LOBBY_API_DESTROY_LIST, 0x0025_e3cc)?;
                    c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0xf8));
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                    c.sw(c.g[V0][0] as u32, 0);
                    c.g[A0][0] = c.g[S0][0];
                }
                c.cycles += 6;
                c.g[A3][0] = c.g[S2][0];
                c.g[A1][0] = 0;
                c.g[A2][0] = 0x64;
                callee::LOBBY_MAN_SEND_SNAPSHOT.call(c, 0x0025_e3f0)?;
                frame.restore(c, S0, 0x30);
                c.cycles += 5;
                frame.restore(c, S1, 0x20);
                frame.restore(c, S2, 0x10);
                frame.restore_ra(c, 0);
                frame.close(c);
                return Ok(());
            }
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 0;
            callee::LOBBY_MAN_ON_RANK_LIST_CHANGED.call(c, 0x0025_e404)?;
        }
        c.cycles += 1;
        frame.restore(c, S0, 0x30);
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void LobbyMan_OnNewsReply(LobbyMan *self, Msg *msg)` (0x25e138): handle
/// a news reply — queue the news text, or connect to the news host and
/// queue the connected notice.
pub fn lobby_man_on_news_reply(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x280);
        c.g[A2][0] = 0x1;
        frame.save(c, S4, 0x230);
        c.g[V0][0] = 0x6e650000;
        frame.save(c, S3, 0x240);
        c.g[S4][0] = c.g[A0][0];
        frame.save(c, S0, 0x270);
        c.g[S3][0] = c.g[A1][0];
        frame.save(c, S1, 0x260);
        c.g[V0][0] |= 0x7731;
        frame.save(c, S2, 0x250);
        frame.save_ra(c, 0x220);
        c.sw(c.g[S4][0] as u32, c.g[A2][0]);
        c.g[V1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x8));
        let not_news_text = c.g[V1][0] != c.g[V0][0];
        c.g[V0][0] = 0x6e650000;
        // Shared node-insert tail state (set on both queue paths).
        if !not_news_text {
            c.cycles += 21;
            c.g[A1][0] = 0x480000;
            c.g[A2][0] = 0x20000000;
            c.g[A3][0] = 0;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x488));
            c.g[A0][0] = 0xc;
            call_ptr(c, callee::EA_MEM_ALLOC_NAMED, 0x0025_e190)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[A1][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0xc));
            c.g[V0][0] = 0x480000;
            c.g[V1][0] = 0xe2;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1210));
            c.sw(c.g[S0][0] as u32, c.g[V1][0]);
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x8));
            c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
            call_ptr(c, callee::C_BX_STRING_CTOR_CSTR, 0x0025_e1b4)?;
            c.g[V1][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0xf0));
            c.g[A1][0] = 0x480000;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4a8));
            c.g[A0][0] = 0xc;
            c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[V1][0]);
        } else {
            c.cycles += 3;
            c.g[V0][0] |= 0x7730;
            let not_news_connect = c.g[V1][0] != c.g[V0][0];
            frame.restore(c, S0, 0x270);
            if not_news_connect {
                c.cycles += 7;
                frame.restore(c, S1, 0x260);
                frame.restore(c, S2, 0x250);
                frame.restore(c, S3, 0x240);
                frame.restore(c, S4, 0x230);
                frame.restore_ra(c, 0x220);
                frame.close(c);
                return Ok(());
            }
            c.cycles += 5;
            c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
            c.sw((c.g[S4][0] as u32).wrapping_add(0xc), c.g[A2][0]);
            callee::BUDDY_MAN_IS_CONNECTED.call(c, 0x0025_e1e4)?;
            if c.g[V0][0] != 0 {
                c.g[A1][0] = 0x480000;
            } else {
                c.cycles += 47;
                let msg = c.g[S3][0] as u32;
                c.g[A0][0] = c.lw(msg.wrapping_add(0xc));
                c.g[A1][0] = 0x480000;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x890));
                call_ptr(c, callee::TAG_FIELD_FIND, 0x0025_e1fc)?;
                c.g[S2][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
                c.g[A3][0] = 0x4a0000;
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x2f40));
                c.g[A2][0] = 0x100;
                c.g[A1][0] = c.g[S2][0];
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, callee::TAG_FIELD_GET_STRING, 0x0025_e218)?;
                c.g[A0][0] = c.lw(msg.wrapping_add(0xc));
                c.g[A1][0] = 0x480000;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x8a0));
                call_ptr(c, callee::TAG_FIELD_FIND, 0x0025_e228)?;
                c.g[A1][0] = 0x34c1;
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, callee::TAG_FIELD_GET_NUMBER, 0x0025_e234)?;
                c.g[A0][0] = c.lw(msg.wrapping_add(0xc));
                c.g[A1][0] = 0x480000;
                c.g[S1][0] = c.g[V0][0];
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x8b0));
                call_ptr(c, callee::TAG_FIELD_FIND, 0x0025_e248)?;
                c.g[S0][0] = sx((c.g[SP][0] as u32).wrapping_add(0x120));
                c.g[A3][0] = 0x480000;
                c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x13a0));
                c.g[A2][0] = 0x100;
                c.g[A1][0] = c.g[S0][0];
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, callee::TAG_FIELD_GET_STRING, 0x0025_e264)?;
                c.g[A0][0] = c.lw(msg.wrapping_add(0xc));
                c.g[A1][0] = 0x480000;
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x8c0));
                call_ptr(c, callee::TAG_FIELD_FIND, 0x0025_e274)?;
                c.g[A1][0] = 0xd0000;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] |= 0x2f00;
                call_ptr(c, callee::TAG_FIELD_GET_NUMBER, 0x0025_e284)?;
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                c.g[A1][0] = c.g[S2][0];
                c.g[A2][0] = c.g[S1][0];
                c.g[T1][0] = c.g[S0][0];
                c.sw((c.g[A0][0] as u32).wrapping_add(0xa4), c.g[V0][0]);
                c.g[A3][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x80));
                c.g[T0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x4c));
                callee::BUDDY_MAN_CONNECT.call(c, 0x0025_e2a4)?;
                c.g[A1][0] = 0x480000;
            }
            c.cycles += 16;
            c.g[A2][0] = 0x20000000;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x488));
            c.g[A0][0] = 0x8;
            c.g[A3][0] = 0;
            call_ptr(c, callee::EA_MEM_ALLOC_NAMED, 0x0025_e2bc)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[A2][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0xf0));
            c.g[V0][0] = 0x480000;
            c.g[V1][0] = 0x111;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x12b0));
            c.sw(c.g[S0][0] as u32, c.g[V1][0]);
            c.sw((c.g[S0][0] as u32).wrapping_add(4), c.g[V0][0]);
            c.g[A1][0] = 0x480000;
            c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[A2][0]);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4a8));
            c.g[A0][0] = 0xc;
        }
        c.cycles += 7;
        c.g[A2][0] = 0x20000000;
        c.g[A3][0] = 0;
        call_ptr(c, callee::EA_MEM_ALLOC_NAMED, 0x0025_e2f4)?;
        c.g[A1][0] = c.g[V0][0];
        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(8));
        if c.g[V0][0] != 0 {
            c.sw((c.g[A1][0] as u32).wrapping_add(8), c.g[S0][0]);
        }
        c.cycles += 10;
        msg_list_insert_splice(c, 0x10, 0);
        frame.restore(c, S0, 0x270);
        c.cycles += 7;
        frame.restore(c, S1, 0x260);
        frame.restore(c, S2, 0x250);
        frame.restore(c, S3, 0x240);
        frame.restore(c, S4, 0x230);
        frame.restore_ra(c, 0x220);
        frame.close(c);
        Ok(())
    })
}

/// `void LobbyMan_OnReportUserReply(LobbyMan *self, Msg *msg)` (0x25df98):
/// mark the lobby active and queue the report outcome message.
pub fn lobby_man_on_report_user_reply(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x60);
        c.g[V0][0] = 0x1;
        frame.save(c, S2, 0x30);
        frame.save(c, S0, 0x50);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x40);
        frame.save_ra(c, 0x20);
        c.sw(c.g[S2][0] as u32, c.g[V0][0]);
        c.g[S1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8));
        let is_error = c.g[S1][0] != 0;
        c.g[A1][0] = 0x480000;
        if !is_error {
            c.cycles += 23;
            c.g[A0][0] = 0x8;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x488));
            c.g[A2][0] = 0x20000000;
            c.g[A3][0] = 0;
            c.g[S0][0] = 0xe8;
            call_ptr(c, callee::EA_MEM_ALLOC_NAMED, 0x0025_dfdc)?;
            c.g[S1][0] = c.g[V0][0];
            c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xf0));
            c.g[V0][0] = 0x480000;
            c.sw(c.g[S1][0] as u32, c.g[S0][0]);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x12b0));
            c.g[A1][0] = 0x480000;
            c.sw((c.g[S1][0] as u32).wrapping_add(4), c.g[V0][0]);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4a8));
            c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[V1][0]);
            c.g[A0][0] = 0xc;
            c.g[A2][0] = 0x20000000;
            c.g[A3][0] = 0;
            call_ptr(c, callee::EA_MEM_ALLOC_NAMED, 0x0025_e010)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(8));
            if c.g[V0][0] != 0 {
                c.sw((c.g[A1][0] as u32).wrapping_add(8), c.g[S1][0]);
                c.cycles += 1;
            } else {
                c.cycles += 2;
            }
            c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
        } else {
            c.cycles += 24;
            c.g[A3][0] = 0;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x488));
            c.g[A0][0] = 0xc;
            c.g[A2][0] = 0x20000000;
            call_ptr(c, callee::EA_MEM_ALLOC_NAMED, 0x0025_e03c)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[A3][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xf0));
            c.g[V0][0] = 0x480000;
            c.g[V1][0] = 0x102;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1290));
            c.sw(c.g[S0][0] as u32, c.g[V1][0]);
            c.sw((c.g[S0][0] as u32).wrapping_add(4), c.g[V0][0]);
            c.g[A1][0] = 0x480000;
            c.sw((c.g[S0][0] as u32).wrapping_add(8), c.g[S1][0]);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4a8));
            c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[A3][0]);
            c.g[A0][0] = 0xc;
            c.g[A2][0] = 0x20000000;
            c.g[A3][0] = 0;
            call_ptr(c, callee::EA_MEM_ALLOC_NAMED, 0x0025_e078)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(8));
            if c.g[V0][0] != 0 {
                c.sw((c.g[A1][0] as u32).wrapping_add(8), c.g[S0][0]);
            }
            c.cycles += 1;
            c.g[V0][0] = c.lw((c.g[SP][0] as u32).wrapping_add(0x10));
        }
        c.cycles += 14;
        msg_list_insert_splice(c, 0x10, 0);
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `void LobbyMan_ReportUser(LobbyMan *self, const char *name)` (0x25df00):
/// tag the reported user's name and language, then send the report request.
pub fn lobby_man_report_user(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x130);
        frame.save(c, S1, 0x110);
        frame.save(c, S0, 0x120);
        frame.save_ra(c, 0x100);
        c.g[S1][0] = c.g[A0][0];
        let name = c.g[A1][0];
        c.g[V0][0] = c.lw(c.g[S1][0] as u32);
        c.g[A2][0] = 0x4a0000;
        if c.g[V0][0] != 0 {
            c.cycles += 24;
            c.g[A3][0] = name;
            c.g[A0][0] = c.g[SP][0];
            c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x3108));
            Addr(c.g[SP][0] as u32).sb(c, 0);
            c.g[A1][0] = 0x100;
            call_ptr(c, callee::TAG_FIELD_SET_STRING, 0x0025_df38)?;
            c.g[S0][0] = 0x4a0000;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x2fe0));
            callee::LOBBY_MAN_GET_LANGUAGE.call(c, 0x0025_df44)?;
            c.g[A2][0] = c.g[S0][0];
            c.g[A3][0] = c.g[V0][0];
            c.g[A0][0] = c.g[SP][0];
            c.g[A1][0] = 0x100;
            call_ptr(c, callee::TAG_FIELD_SET_STRING, 0x0025_df58)?;
            c.g[T0][0] = 0x270000;
            c.g[A2][0] = 0x72650000;
            c.g[AT][0] = 0x41a00000;
            c.mtc1(AT, 12);
            c.g[A0][0] = c.g[S1][0];
            c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x7250));
            c.g[A1][0] = 0x10;
            c.g[A2][0] |= 0x7074;
            c.g[A3][0] = c.g[SP][0];
            callee::LOBBY_MAN_SEND_REQUEST.call(c, 0x0025_df80)?;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x120);
        frame.restore(c, S1, 0x110);
        frame.restore_ra(c, 0x100);
        frame.close(c);
        Ok(())
    })
}

/// `void LobbyMan_OnPingReply(LobbyMan *self, Msg *msg)` (0x25dde0): build
/// the ping-reply message from the sender name and queued value.
pub fn lobby_man_on_ping_reply(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 20;
        let frame = Frame::open(c, 0x80);
        frame.save(c, S0, 0x70);
        frame.save(c, S2, 0x50);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S1, 0x60);
        c.g[S2][0] = c.g[A0][0];
        frame.save_ra(c, 0x40);
        let msg = c.g[S0][0] as u32;
        c.g[A1][0] = sx(0x004a_2f58);
        c.g[A0][0] = c.lw(msg.wrapping_add(0xc));
        call_ptr(c, callee::TAG_FIELD_FIND, 0x0025_de0c)?;
        c.g[A3][0] = 0x4a0000;
        c.g[A0][0] = c.g[V0][0];
        c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x2f40));
        c.g[A1][0] = c.g[SP][0];
        c.g[A2][0] = 0x20;
        call_ptr(c, callee::TAG_FIELD_GET_STRING, 0x0025_de24)?;
        c.g[V1][0] = c.lw(msg.wrapping_add(0x8));
        let is_error = c.g[V1][0] != 0;
        c.g[S1][0] = 0x7f90;
        if !is_error {
            c.cycles += 8;
            c.g[A0][0] = c.lw(msg.wrapping_add(0xc));
            c.g[A1][0] = sx(0x004a_3168);
            call_ptr(c, callee::TAG_FIELD_FIND, 0x0025_de40)?;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            call_ptr(c, callee::TAG_FIELD_GET_NUMBER, 0x0025_de4c)?;
            c.g[S1][0] = c.g[V0][0];
        }
        c.cycles += 28;
        c.g[A1][0] = 0x480000;
        c.g[A2][0] = 0x20000000;
        c.g[A3][0] = 0;
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x488));
        c.g[A0][0] = 0x10;
        call_ptr(c, callee::EA_MEM_ALLOC_NAMED, 0x0025_de68)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[V1][0] = 0x480000;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1170));
        c.g[V0][0] = 0xe5;
        c.g[A1][0] = c.g[SP][0];
        c.sw((c.g[S0][0] as u32).wrapping_add(4), c.g[V1][0]);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(8));
        c.sw(c.g[S0][0] as u32, c.g[V0][0]);
        call_ptr(c, callee::C_BX_STRING_CTOR_CSTR, 0x0025_de8c)?;
        c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xf0));
        c.g[A1][0] = 0x480000;
        c.sw((c.g[S0][0] as u32).wrapping_add(0xc), c.g[S1][0]);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4a8));
        c.sw((c.g[SP][0] as u32).wrapping_add(0x30), c.g[V1][0]);
        c.g[A0][0] = 0xc;
        c.g[A2][0] = 0x20000000;
        c.g[A3][0] = 0;
        call_ptr(c, callee::EA_MEM_ALLOC_NAMED, 0x0025_deb0)?;
        c.g[A1][0] = c.g[V0][0];
        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(8));
        if c.g[V0][0] != 0 {
            c.sw((c.g[A1][0] as u32).wrapping_add(8), c.g[S0][0]);
        }
        c.cycles += 15;
        msg_list_insert_splice(c, 0x30, 0x20);
        frame.restore(c, S0, 0x70);
        frame.restore(c, S1, 0x60);
        frame.restore(c, S2, 0x50);
        frame.restore_ra(c, 0x40);
        frame.close(c);
        Ok(())
    })
}

/// `void LobbyMan_PingSelectedRoomUser(LobbyMan *self)` (0x25dd68): ping
/// the selected room user through their peer address.
pub fn lobby_man_ping_selected_room_user(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[V0][0] = c.lw(c.g[S1][0] as u32);
        let inactive = c.g[V0][0] == 0;
        frame.restore(c, S0, 0x20);
        if !inactive {
            c.cycles += 4;
            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x60));
            call_ptr(c, callee::LOBBY_LIST_GET_AT, 0x0025_dd90)?;
            let no_entry = c.g[V0][0] == 0;
            c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(8));
            if !no_entry {
                c.cycles += 12;
                c.g[A2][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x30));
                c.g[A0][0] = c.g[S1][0];
                c.g[A1][0] = c.g[S0][0];
                callee::LOBBY_MAN_LOOKUP_PEER_ADDR.call(c, 0x0025_dda8)?;
                c.g[T0][0] = 0x270000;
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x54));
                c.g[A1][0] = c.g[S0][0];
                c.g[A2][0] = c.g[V0][0];
                c.g[T0][0] = sx((c.g[T0][0] as u32).wrapping_sub(0x7278));
                c.g[T1][0] = c.g[S1][0];
                c.g[A3][0] = 0x1388;
                call_ptr(c, callee::LOBBY_API_PING, 0x0025_ddc8)?;
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void LobbyMan_OnCreateRoomReply(LobbyMan *self, Msg *msg)` (0x25db80):
/// mark the lobby active and queue the create-room outcome, adopting the
/// room name on success.
pub fn lobby_man_on_create_room_reply(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x160);
        c.g[V0][0] = 0x1;
        frame.save(c, S2, 0x130);
        frame.save(c, S0, 0x150);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x140);
        frame.save_ra(c, 0x120);
        c.sw(c.g[S2][0] as u32, c.g[V0][0]);
        c.g[S1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8));
        let is_error = c.g[S1][0] != 0;
        if !is_error {
            c.cycles += 37;
            let msg = c.g[A1][0] as u32;
            c.g[A0][0] = c.lw(msg.wrapping_add(0xc));
            c.g[S0][0] = 0xe4;
            c.g[A1][0] = sx(0x004a_2f58);
            call_ptr(c, callee::TAG_FIELD_FIND, 0x0025_dbc0)?;
            c.g[A3][0] = 0x4a0000;
            c.g[A1][0] = c.g[SP][0];
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x2f40));
            c.g[A2][0] = 0x100;
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, callee::TAG_FIELD_GET_STRING, 0x0025_dbd8)?;
            c.g[A1][0] = c.g[SP][0];
            c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x84));
            call_ptr(c, callee::C_BX_STRING_ASSIGN_CSTR, 0x0025_dbe4)?;
            c.g[A1][0] = 0x480000;
            c.g[A0][0] = 0x8;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x488));
            c.g[A2][0] = 0x20000000;
            c.g[A3][0] = 0;
            call_ptr(c, callee::EA_MEM_ALLOC_NAMED, 0x0025_dbfc)?;
            c.g[S1][0] = c.g[V0][0];
            c.g[V1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xf0));
            c.g[V0][0] = 0x480000;
            c.sw(c.g[S1][0] as u32, c.g[S0][0]);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x12b0));
            c.g[A1][0] = 0x480000;
            c.sw((c.g[S1][0] as u32).wrapping_add(4), c.g[V0][0]);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4a8));
            c.sw((c.g[SP][0] as u32).wrapping_add(0x110), c.g[V1][0]);
            c.g[A0][0] = 0xc;
            c.g[A2][0] = 0x20000000;
            c.g[A3][0] = 0;
            call_ptr(c, callee::EA_MEM_ALLOC_NAMED, 0x0025_dc30)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(8));
            if c.g[V0][0] != 0 {
                c.sw((c.g[A1][0] as u32).wrapping_add(8), c.g[S1][0]);
            }
            c.cycles += 10;
            msg_list_insert_splice(c, 0x110, 0x100);
        } else {
            c.g[A1][0] = 0x480000;
            c.cycles += 24;
            c.g[A3][0] = 0;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x488));
            c.g[A0][0] = 0xc;
            c.g[A2][0] = 0x20000000;
            call_ptr(c, callee::EA_MEM_ALLOC_NAMED, 0x0025_dc7c)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[A3][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xf0));
            c.g[V0][0] = 0x480000;
            c.g[V1][0] = 0x102;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1290));
            c.sw(c.g[S0][0] as u32, c.g[V1][0]);
            c.sw((c.g[S0][0] as u32).wrapping_add(4), c.g[V0][0]);
            c.g[A1][0] = 0x480000;
            c.sw((c.g[S0][0] as u32).wrapping_add(8), c.g[S1][0]);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4a8));
            c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[A3][0]);
            c.g[A0][0] = 0xc;
            c.g[A2][0] = 0x20000000;
            c.g[A3][0] = 0;
            call_ptr(c, callee::EA_MEM_ALLOC_NAMED, 0x0025_dcb8)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(8));
            if c.g[V0][0] != 0 {
                c.sw((c.g[A1][0] as u32).wrapping_add(8), c.g[S0][0]);
            }
            c.cycles += 9;
            msg_list_insert_splice(c, 0x10, 0);
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x150);
        frame.restore(c, S1, 0x140);
        frame.restore(c, S2, 0x130);
        frame.restore_ra(c, 0x120);
        frame.close(c);
        Ok(())
    })
}

/// `void LobbyMan_OnMoveToRoomReply(LobbyMan *self, Msg *msg)` (0x25d8f8):
/// mark the lobby active and queue the move outcome, adopting the room
/// name on success and clearing it on failure.
pub fn lobby_man_on_move_to_room_reply(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x160);
        c.g[V0][0] = 0x1;
        frame.save(c, S2, 0x130);
        frame.save(c, S0, 0x150);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x140);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0x120);
        c.sw(c.g[S2][0] as u32, c.g[V0][0]);
        c.g[V0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
        let is_error = c.g[V0][0] != 0;
        c.g[A1][0] = 0x4a0000;
        if !is_error {
            c.cycles += 36;
            let msg = c.g[S0][0] as u32;
            c.g[A0][0] = c.lw(msg.wrapping_add(0xc));
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2f58));
            call_ptr(c, callee::TAG_FIELD_FIND, 0x0025_d934)?;
            c.g[A3][0] = 0x4a0000;
            c.g[A1][0] = c.g[SP][0];
            c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_add(0x2f40));
            c.g[A2][0] = 0x100;
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, callee::TAG_FIELD_GET_STRING, 0x0025_d94c)?;
            c.g[A1][0] = c.g[SP][0];
            c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x84));
            call_ptr(c, callee::C_BX_STRING_ASSIGN_CSTR, 0x0025_d958)?;
            c.g[A1][0] = 0x480000;
            c.g[A2][0] = 0x20000000;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x488));
            c.g[A0][0] = 0x8;
            c.g[A3][0] = 0;
            call_ptr(c, callee::EA_MEM_ALLOC_NAMED, 0x0025_d970)?;
            c.g[S0][0] = c.g[V0][0];
            c.g[A2][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xf0));
            c.g[V0][0] = 0x480000;
            c.g[V1][0] = 0xdc;
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x12b0));
            c.sw(c.g[S0][0] as u32, c.g[V1][0]);
            c.sw((c.g[S0][0] as u32).wrapping_add(4), c.g[V0][0]);
            c.g[A1][0] = 0x480000;
            c.sw((c.g[SP][0] as u32).wrapping_add(0x110), c.g[A2][0]);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4a8));
            c.g[A0][0] = 0xc;
            c.g[A2][0] = 0x20000000;
            c.g[A3][0] = 0;
            call_ptr(c, callee::EA_MEM_ALLOC_NAMED, 0x0025_d9a8)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(8));
            if c.g[V0][0] != 0 {
                c.sw((c.g[A1][0] as u32).wrapping_add(8), c.g[S0][0]);
            }
            c.cycles += 10;
            msg_list_insert_splice(c, 0x110, 0x100);
        } else {
            c.cycles += 28;
            c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x84));
            call_ptr(c, callee::C_BX_STRING_RELEASE, 0x0025_d9e8)?;
            c.g[A1][0] = 0x480000;
            c.g[A0][0] = 0xc;
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x488));
            c.g[A2][0] = 0x20000000;
            c.g[A3][0] = 0;
            call_ptr(c, callee::EA_MEM_ALLOC_NAMED, 0x0025_da00)?;
            c.g[S1][0] = c.g[V0][0];
            c.g[A0][0] = c.lw((c.g[S0][0] as u32).wrapping_add(0x8));
            c.g[V0][0] = 0x480000;
            c.g[A2][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0xf0));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1290));
            c.g[V1][0] = 0x102;
            c.sw((c.g[S1][0] as u32).wrapping_add(8), c.g[A0][0]);
            c.g[A1][0] = 0x480000;
            c.sw(c.g[S1][0] as u32, c.g[V1][0]);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4a8));
            c.sw((c.g[S1][0] as u32).wrapping_add(4), c.g[V0][0]);
            c.g[A0][0] = 0xc;
            c.sw((c.g[SP][0] as u32).wrapping_add(0x10), c.g[A2][0]);
            c.g[A3][0] = 0;
            c.g[A2][0] = 0x20000000;
            call_ptr(c, callee::EA_MEM_ALLOC_NAMED, 0x0025_da40)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(8));
            if c.g[V0][0] != 0 {
                c.sw((c.g[A1][0] as u32).wrapping_add(8), c.g[S1][0]);
            }
            c.cycles += 9;
            msg_list_insert_splice(c, 0x10, 0);
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x150);
        frame.restore(c, S1, 0x140);
        frame.restore(c, S2, 0x130);
        frame.restore_ra(c, 0x120);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, DATA, GAME_GP};

    /// Lobby object at `DATA`, reply message at `DATA+0x800`, with the
    /// allocator, BELibrary dummy and world the real callees need.
    fn lobby_setup(c: &mut Cpu, seed: u32, entry: u32) {
        let _r = crate::game::dirtysock::tagsunk::tests::alloc_machine(c, seed, entry as u64);
        c.pc = entry;
        c.bus.write32(GAME_GP.wrapping_sub(0x1f04), DATA + 0x3800);
        c.bus.write32(GAME_GP.wrapping_sub(0x848), DATA + 0x3900);
        c.bus.write32(GAME_GP + 0x238, DATA + 0x3400);
        let self_ = DATA;
        let msg = DATA + 0x800;
        c.bus.write32(self_ + 0x54, DATA + 0x3000);
        c.bus.write32(self_ + 0x60, DATA + 0x2800);
        c.bus.write32(self_ + 0xf0, DATA + 0x2000);
        c.bus.write32(DATA + 0x2000, DATA + 0x2000);
        c.bus.write32(DATA + 0x2004, DATA + 0x2000);
        c.bus.write32(msg + 0xc, DATA + 0x1000);
        c.g[A0][0] = sx(self_);
        c.g[A1][0] = sx(msg);
    }

    #[test]
    fn lobby_man_on_snapshot_reply_matches() {
        check(
            ee::s06::f_0025e590__generated,
            lobby_man_on_snapshot_reply,
            60,
            &|c, seed| {
                lobby_setup(c, seed, 0x0025_e590);
                // seed%2: score-sum path / error-message path.
                c.bus.write32(
                    DATA + 0x800 + 0x8,
                    if seed % 2 == 0 { 0 } else { 0x40 + seed },
                );
            },
        );
    }

    #[test]
    fn lobby_man_send_snapshot_matches() {
        check(
            ee::s06::f_0025e420__generated,
            lobby_man_send_snapshot,
            90,
            &|c, seed| {
                lobby_setup(c, seed, 0x0025_e420);
                let slot = seed % 3;
                c.g[A1][0] = (seed % 5) as u64;
                c.g[A2][0] = (seed % 7) as u64;
                c.g[A3][0] = (seed % 3) as u64;
                // seed%3==0: inactive; else active. seed%2: handle present/absent.
                c.bus.write32(DATA, if seed % 3 == 0 { 0 } else { 1 });
                c.bus.write32(DATA + 0xf8, slot);
                c.bus.write32(
                    DATA + 0x1ec + 4 * slot,
                    if seed % 2 == 0 { 0 } else { DATA + 0x3200 },
                );
                c.bus.write32(DATA + 0x178 + 4 * (seed % 3), 0x100 + seed);
                c.bus.write32(DATA + 0x40, 0x200 + seed);
            },
        );
    }

    #[test]
    fn lobby_man_request_stats_page_matches() {
        check(
            ee::s06::f_0025e348__generated,
            lobby_man_request_stats_page,
            90,
            &|c, seed| {
                lobby_setup(c, seed, 0x0025_e348);
                let slot = seed % 3;
                c.g[A1][0] = slot as u64;
                c.bus.write32(DATA, if seed % 4 == 0 { 0 } else { 1 });
                c.bus.write32(DATA + 0xf8, seed % 2);
                // seed%3==0: limit large (snapshot path); else 0 (changed path).
                c.bus.write32(
                    DATA + 0x104 + 4 * slot,
                    if seed % 3 == 0 { 0x10000 } else { 0 },
                );
                // No live list handle: a non-zero handle would make the
                // generated LobbyApi_DestroyList free list nodes out of the
                // zeroed session object and run away in the heap consolidate.
                c.bus.write32(DATA + 0x1ec + 4 * slot, 0);
                c.bus.write32(DATA + 0x54, DATA + 0x3000);
            },
        );
    }

    #[test]
    fn lobby_man_on_news_reply_matches() {
        check(
            ee::s06::f_0025e138__generated,
            lobby_man_on_news_reply,
            80,
            &|c, seed| {
                lobby_setup(c, seed, 0x0025_e138);
                // seed%4: news-text / news-connect / other / other.
                let code = match seed % 4 {
                    0 => 0x6e65_7731,
                    1 => 0x6e65_7730,
                    _ => seed.wrapping_mul(0x1111),
                };
                c.bus.write32(DATA + 0x800 + 0x8, code);
                c.bus.write32(DATA + 0x80, DATA + 0x3600);
                c.bus.write32(DATA + 0x4c, DATA + 0x3700);
            },
        );
    }

    #[test]
    fn lobby_man_on_report_user_reply_matches() {
        check(
            ee::s06::f_0025df98__generated,
            lobby_man_on_report_user_reply,
            60,
            &|c, seed| {
                lobby_setup(c, seed, 0x0025_df98);
                c.bus.write32(
                    DATA + 0x800 + 0x8,
                    if seed % 2 == 0 { 0 } else { 0x20 + seed },
                );
            },
        );
    }

    #[test]
    fn lobby_man_report_user_matches() {
        check(
            ee::s06::f_0025df00__generated,
            lobby_man_report_user,
            60,
            &|c, seed| {
                lobby_setup(c, seed, 0x0025_df00);
                c.g[A1][0] = sx(DATA + 0x1800);
                c.bus.write32(DATA, if seed % 2 == 0 { 0 } else { 1 });
            },
        );
    }

    #[test]
    fn lobby_man_on_ping_reply_matches() {
        check(
            ee::s06::f_0025dde0__generated,
            lobby_man_on_ping_reply,
            60,
            &|c, seed| {
                lobby_setup(c, seed, 0x0025_dde0);
                c.bus.write32(
                    DATA + 0x800 + 0x8,
                    if seed % 2 == 0 { 0 } else { 0x30 + seed },
                );
            },
        );
    }

    #[test]
    fn lobby_man_ping_selected_room_user_matches() {
        check(
            ee::s06::f_0025dd68__generated,
            lobby_man_ping_selected_room_user,
            60,
            &|c, seed| {
                lobby_setup(c, seed, 0x0025_dd68);
                c.g[A1][0] = 0;
                c.bus.write32(DATA, if seed % 2 == 0 { 0 } else { 1 });
            },
        );
    }

    #[test]
    fn lobby_man_on_create_room_reply_matches() {
        check(
            ee::s06::f_0025db80__generated,
            lobby_man_on_create_room_reply,
            60,
            &|c, seed| {
                lobby_setup(c, seed, 0x0025_db80);
                c.bus.write32(
                    DATA + 0x800 + 0x8,
                    if seed % 2 == 0 { 0 } else { 0x50 + seed },
                );
            },
        );
    }

    #[test]
    fn lobby_man_on_move_to_room_reply_matches() {
        check(
            ee::s06::f_0025d8f8__generated,
            lobby_man_on_move_to_room_reply,
            60,
            &|c, seed| {
                lobby_setup(c, seed, 0x0025_d8f8);
                c.bus.write32(
                    DATA + 0x800 + 0x8,
                    if seed % 2 == 0 { 0 } else { 0x60 + seed },
                );
            },
        );
    }
}
