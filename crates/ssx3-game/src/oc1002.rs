//! Staging module oc1002 (wave wave): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1002`.
//!
//! The ten `LobbyMan_*` functions of `src/dirtysock/tagsunk.cpp` (list
//! `tagsunk3/list-oc1002.txt`): challenge send/cancel, room-user stats and
//! rank-list accessors, quick-play cancel, and the user-stats / lookup-user
//! request-reply pair. None of the ten bodies contains a loop, so there are
//! no `back_edge_unwinds` resume heads and no `check_events` tests; every
//! call is a plain guest call whose unwind propagates with `?`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

// ---------------------------------------------------------------------------
// Lobby manager (`LobbyMan`) field offsets: one name per offset, shared by
// every port and fixture in this module (the same offsets the merged unit
// `game/dirtysock/tagsunk.rs` uses).

/// Session state word: 0 = lobby down; 1 = idle; 0x15/0x16/0x18 = challenge
/// pending; 0x19 = quick-play / lock pending. (The ports read it as the
/// object base itself; the name documents the offset.)
#[allow(dead_code)]
pub(crate) const LOBBY_STATE: u32 = 0x00;
/// Outstanding request handle returned by the lobby API (`+0x40`).
pub(crate) const LOBBY_REQUEST: u32 = 0x40;
/// Lookup-in-progress flag (`+0x44`).
pub(crate) const LOBBY_LOOKUP_PENDING: u32 = 0x44;
/// Lobby API object (`+0x54`).
pub(crate) const LOBBY_API: u32 = 0x54;
/// `Lobby_List*` of rooms (`+0x5c`); named for the shared offset table
/// (no port in this list reads it).
#[allow(dead_code)]
pub(crate) const LOBBY_ROOM_LIST: u32 = 0x5c;
/// `Lobby_List*` of the current room's users (`+0x60`).
pub(crate) const LOBBY_USER_LIST: u32 = 0x60;
/// Peer/name set consulted by `LobbyMan_LookupUser` (`+0x80`).
pub(crate) const LOBBY_PEER_SET: u32 = 0x80;
/// Quick-play timeout, seconds as float (`+0xe0`).
pub(crate) const LOBBY_QUICK_TIMER: u32 = 0xe0;
/// Head node of the outbound message list (`+0xf0`).
pub(crate) const LOBBY_MSG_LIST: u32 = 0xf0;
/// Selected rank-list index (`+0xf8`); the lists themselves follow at
/// [`RANK_TABLE`].
pub(crate) const LOBBY_RANK_INDEX: u32 = 0xf8;
/// Rank-list pointer table (`self + 0x1ec + 4 * index`).
pub(crate) const RANK_TABLE: u32 = 0x1ec;

/// Callees of the ports in this module: direct same-shard calls as
/// [`Callee`], cross-shard calls (emitted as `call_indirect` by the
/// generator) as plain addresses for [`call_ptr`].
pub(crate) mod callee {
    use super::{ee, Callee};

    pub const SEND_ABORT_CHAL: Callee = Callee {
        addr: 0x0025_fc48,
        f: ee::s06::LobbyMan_SendAbortChal,
    };
    pub const RESET_CHAL_STATE: Callee = Callee {
        addr: 0x0025_f858,
        f: ee::s06::LobbyMan_ResetChalState,
    };
    pub const CLEAR_CHALLENGE_LOCK: Callee = Callee {
        addr: 0x0025_f740,
        f: ee::s06::LobbyMan_ClearChallengeLock,
    };
    pub const GET_ROOM_USER_NAME: Callee = Callee {
        addr: 0x0025_f1e0,
        f: ee::s06::LobbyMan_GetRoomUserName,
    };
    pub const SEND_CHALLENGE: Callee = Callee {
        addr: 0x0025_f4c0,
        f: ee::s06::LobbyMan_SendChallenge,
    };
    pub const SEND_PRIVATE_MESSAGE: Callee = Callee {
        addr: 0x0025_fb80,
        f: ee::s06::LobbyMan_SendPrivateMessage,
    };
    pub const SEND_REQUEST: Callee = Callee {
        addr: 0x0025_b608,
        f: ee::s06::LobbyMan_SendRequest,
    };
    pub const NET_GET_GAME_SETUP: Callee = Callee {
        addr: 0x0025_91d0,
        f: ee::s06::Net_GetGameSetup,
    };
    pub const GET_VERSION_TAG: Callee = Callee {
        addr: 0x0025_5a10,
        f: ee::s06::cCommSystem_GetVersionTag,
    };

    /// `cMemMan_alloc` (the game's `operator new(size, tag, flags, align)`).
    pub const EA_MEM_ALLOC_NAMED: u32 = 0x0031_7d70;
    /// Lobby API request send (`LobbyApi_*` at 0x3e8b10).
    pub const LOBBY_API_SEND: u32 = 0x003e_8b10;
    /// `LobbyApi_CancelRequest`.
    pub const LOBBY_API_CANCEL_REQUEST: u32 = 0x003e_8d70;
    /// `Lobby_ListGetCount`.
    pub const LOBBY_LIST_GET_COUNT: u32 = 0x003e_e550;
    /// `Lobby_ListGetAt`.
    pub const LOBBY_LIST_GET_AT: u32 = 0x003e_e560;
    /// `cDirtysock_tag_TagFieldFind`.
    pub const TAG_FIELD_FIND: u32 = 0x003e_b6d8;
    /// `cDirtysock_tag_TagFieldGetString`.
    pub const TAG_FIELD_GET_STRING: u32 = 0x003e_d378;
    /// `cDirtysock_tag_TagFieldGetNumber`.
    pub const TAG_FIELD_GET_NUMBER: u32 = 0x003e_cf30;
    /// `cDirtysock_tag_TagFieldGetStructure`.
    pub const TAG_FIELD_GET_BINARY: u32 = 0x003e_d678;
    /// `cDirtysock_tag_TagFieldSetString`.
    pub const TAG_FIELD_SET_STRING: u32 = 0x003e_c180;
    /// `cBXString` constructor from a C string (0x3182d8).
    pub const C_BX_STRING_CTOR_CSTR: u32 = 0x0031_82d8;
    /// String field initialiser called by `LobbyMan_SendChallenge`
    /// (0x3184e8).
    pub const C_BX_STRING_ASSIGN_CSTR: u32 = 0x0031_84e8;
    pub const STRCPY: u32 = 0x0041_66f4;
    /// String length (0x4187c0).
    pub const ATOI: u32 = 0x0041_87c0;
    pub const SPRINTF: u32 = 0x0041_7828;
    /// Name/set compare consulted by `LobbyMan_LookupUser` (0x41aa88).
    pub const STRCASECMP: u32 = 0x0041_aa88;
    /// `cBE_getBE`.
    pub const BE_LIBRARY_GET_INSTANCE: u32 = 0x0014_dc80;
    /// `BELibrary_GetInterface`-style accessor (0x14dd58).
    pub const BE_LIBRARY_GET_INTERFACE: u32 = 0x0014_dd58;
    /// Back-end rider lookup (0x144be0).
    pub const BE_NEW_RACE_INT_GET_TRACK: u32 = 0x0014_4be0;
    /// Game-setup cancel/notify (0x263ba8).
    pub const NET_ABORT_GAME_SETUP: u32 = 0x0026_3ba8;
}

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0025_f628,
        lobby_man_cancel_challenge,
        "LobbyMan_CancelChallenge",
    ),
    (
        0x0025_f4c0,
        lobby_man_send_challenge,
        "LobbyMan_SendChallenge",
    ),
    (
        0x0025_f478,
        lobby_man_challenge_room_user,
        "LobbyMan_ChallengeRoomUser",
    ),
    (
        0x0025_f338,
        lobby_man_get_rank_list_count,
        "LobbyMan_GetRankListCount",
    ),
    (
        0x0025_f288,
        lobby_man_get_room_user_stats,
        "LobbyMan_GetRoomUserStats",
    ),
    (
        0x0025_eed8,
        lobby_man_cancel_quick_play,
        "LobbyMan_CancelQuickPlay",
    ),
    (
        0x0025_eb00,
        lobby_man_on_user_stats_reply,
        "LobbyMan_OnUserStatsReply",
    ),
    (
        0x0025_eaa0,
        lobby_man_query_user_stats,
        "LobbyMan_QueryUserStats",
    ),
    (
        0x0025_e7f0,
        lobby_man_on_lookup_user_reply,
        "LobbyMan_OnLookupUserReply",
    ),
    (0x0025_e6d0, lobby_man_lookup_user, "LobbyMan_LookupUser"),
];

// ---------------------------------------------------------------------------
// Challenge send / cancel.

/// `void LobbyMan_CancelChallenge(LobbyMan* self, int notify, int abort)`
/// (0x25f628): tear down a pending challenge. In states 0x15/0x16/0x18 the
/// abort message is sent (when `abort` is set), the outstanding lobby
/// request is cancelled and the challenge state is reset; in state 0x19 the
/// challenge lock is cleared instead. When `notify` is set, a 0xEE notice
/// is posted to the message list at `self + 0xf0`.
pub fn lobby_man_cancel_challenge(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S0, 0x50);
        frame.save(c, S1, 0x40);
        frame.save(c, S2, 0x30);
        frame.save_ra(c, 0x20);
        let self_ = r32(c, A0);
        c.g[S1][0] = c.g[A0][0];
        let state = Addr(self_).lw(c);
        c.g[V1][0] = state;
        let notify = c.g[A1][0];
        c.g[S0][0] = notify;
        if state != 0 {
            c.cycles += 4;
            let in_challenge = (state as u32).wrapping_sub(0x15) < 2;
            c.g[V0][0] = in_challenge as u64;
            c.g[V0][0] = 0x18;
            if in_challenge || state == 0x18 {
                // States 0x15, 0x16 and 0x18 share the abort/cancel path;
                // 0x18 arrives through the dispatch's second test (+2).
                if state == 0x18 {
                    c.cycles += 2;
                }
                c.cycles += 2;
                if c.g[A2][0] != 0 {
                    c.cycles += 3;
                    c.g[A0][0] = c.g[S1][0];
                    callee::SEND_ABORT_CHAL.call(c, 0x0025_f674)?;
                }
                c.g[A0][0] = Addr(self_).off(LOBBY_API).lw(c);
                c.cycles += 6;
                c.g[A1][0] = Addr(self_).off(LOBBY_REQUEST).lw(c);
                call_ptr(c, callee::LOBBY_API_CANCEL_REQUEST, 0x0025_f680)?;
                c.g[A0][0] = c.g[S1][0];
                callee::RESET_CHAL_STATE.call(c, 0x0025_f688)?;
            } else if state == 0x19 {
                c.cycles += 2;
                c.g[V0][0] = 0x19;
                c.cycles += 2;
                // `V1 != V0` is false here (both are the state, 0x19).
                c.cycles += 2;
                c.g[A0][0] = c.g[S1][0];
                c.g[A1][0] = 0;
                callee::CLEAR_CHALLENGE_LOCK.call(c, 0x0025_f6a0)?;
            } else {
                c.cycles += 2;
                c.g[V0][0] = 0x19;
                c.cycles += 2;
                // Any other state: no teardown, only the notice below.
            }
            c.cycles += 2;
            c.g[A1][0] = 0x480000;
            if c.g[S0][0] != 0 {
                c.cycles += 23;
                c.g[A0][0] = 8;
                c.g[A1][0] = sx(0x0048_0488);
                c.g[A2][0] = 0x2000_0000;
                c.g[A3][0] = 0;
                c.g[S0][0] = 0xee;
                call_ptr(c, callee::EA_MEM_ALLOC_NAMED, 0x0025_f6c0)?;
                let msg = c.g[V0][0];
                c.g[S2][0] = msg;
                let head = Addr(self_).off(LOBBY_MSG_LIST).lw(c);
                c.g[V1][0] = head;
                Addr(msg as u32).sw(c, 0xee);
                Addr((msg as u32).wrapping_add(4)).sw(c, sx(0x0048_12b0));
                Addr(frame.at(c, 0x10)).sw(c, head);
                c.g[A0][0] = 0xc;
                c.g[A1][0] = sx(0x0048_04a8);
                c.g[A2][0] = 0x2000_0000;
                c.g[A3][0] = 0;
                call_ptr(c, callee::EA_MEM_ALLOC_NAMED, 0x0025_f6f4)?;
                let node = c.g[V0][0];
                c.g[A1][0] = node;
                if (node as u32).wrapping_add(8) != 0 {
                    Addr((node as u32).wrapping_add(8)).sw(c, msg);
                }
                c.cycles += 9;
                let head = Addr(frame.at(c, 0x10)).lw(c);
                c.g[V0][0] = head;
                Addr(node as u32).sw(c, head);
                let prev = Addr((head as u32).wrapping_add(4)).lw(c);
                c.g[V1][0] = prev;
                Addr((node as u32).wrapping_add(4)).sw(c, prev);
                let prev2 = Addr((head as u32).wrapping_add(4)).lw(c);
                c.g[A0][0] = prev2;
                Addr(prev2 as u32).sw(c, node);
                let head = Addr(frame.at(c, 0x10)).lw(c);
                c.g[V0][0] = head;
                Addr((head as u32).wrapping_add(4)).sw(c, node);
                Addr(frame.at(c, 0)).sw(c, node);
            }
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

/// `void LobbyMan_SendChallenge(LobbyMan* self, char* name, int id)`
/// (0x25f4c0, asm-only in the reference): build the challenge text from the
/// local rider's version/character data, send it as a private message to
/// `name`, and move the manager to state 0x16. Does nothing while the
/// lobby is down (`*self == 0`).
pub fn lobby_man_send_challenge(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x120);
        frame.save(c, S0, 0x110);
        frame.save(c, FP, 0x90);
        frame.save(c, S1, 0x100);
        let self_ = r32(c, A0);
        c.g[FP][0] = c.g[A0][0];
        frame.save(c, S2, 0xf0);
        frame.save(c, S3, 0xe0);
        frame.save(c, S4, 0xd0);
        frame.save(c, S5, 0xc0);
        frame.save(c, S6, 0xb0);
        frame.save(c, S7, 0xa0);
        frame.save_ra(c, 0x80);
        let state = Addr(self_).lw(c);
        c.g[V0][0] = state;
        c.g[S0][0] = c.g[A2][0];
        if state != 0 {
            c.cycles += 63;
            c.g[V0][0] = 1;
            Addr(self_).off(0xb4).sw(c, 1);
            c.g[A0][0] = sx(self_.wrapping_add(0x98));
            call_ptr(c, callee::C_BX_STRING_ASSIGN_CSTR, 0x0025_f50c)?;
            Addr(self_).off(0xa4).sw(c, c.g[S0][0]);
            c.g[A1][0] = 7;
            let world = (c.g[GP][0] as u32).wrapping_sub(0x848);
            let world = Addr(world).lw(c);
            c.g[V0][0] = world;
            c.g[A0][0] = Addr((world as u32).wrapping_add(0x78)).lw(c);
            call_ptr(c, callee::BE_LIBRARY_GET_INTERFACE, 0x0025_f520)?;
            call_ptr(c, callee::BE_LIBRARY_GET_INSTANCE, 0x0025_f528)?;
            c.g[A1][0] = 0;
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, callee::BE_LIBRARY_GET_INTERFACE, 0x0025_f534)?;
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, callee::BE_NEW_RACE_INT_GET_TRACK, 0x0025_f53c)?;
            let rider = c.g[V0][0];
            let flags = Addr(0x0053_08d0).lw(c);
            c.g[S0][0] = flags;
            let profile = 0x0053_5bc8u32;
            c.g[S2][0] = sx(((flags as u32) as i32 >> 7) as u32);
            c.g[S1][0] = sx(((flags as u32) as i32 >> 6) as u32);
            c.g[A1][0] = 0x530000;
            let comm = Addr((c.g[GP][0] as u32).wrapping_sub(0x238)).lw(c);
            c.g[A0][0] = comm;
            c.g[S6][0] = Addr(0x0053_4b34).lw(c);
            c.g[S5][0] = rider;
            c.g[S7][0] = Addr(profile.wrapping_add(0x3c)).lw(c);
            c.g[S0][0] = sx(((flags as u32) as i32 >> 8) as u32);
            c.g[S4][0] = Addr(profile.wrapping_add(0x48)).lb(c);
            c.g[S1][0] &= 1;
            c.g[S3][0] = Addr(profile.wrapping_add(0x4a)).lb(c);
            c.g[S0][0] &= 1;
            callee::GET_VERSION_TAG.call(c, 0x0025_f580)?;
            let version = c.g[V0][0];
            c.g[S2][0] &= 1;
            let buf = frame.at(c, 0x40);
            c.g[T6][0] = Addr(self_).off(0xbc).lw(c);
            c.g[T5][0] = Addr(self_).off(0xc0).lw(c);
            c.g[T4][0] = Addr(self_).off(0xc4).lw(c);
            c.g[V1][0] = Addr(self_).off(0xb8).lw(c);
            c.g[A0][0] = sx(buf);
            c.g[A3][0] = c.g[S5][0];
            c.g[T0][0] = c.g[S4][0];
            c.g[T1][0] = c.g[S3][0];
            c.g[T2][0] = c.g[S6][0];
            c.g[T3][0] = c.g[S1][0];
            Addr(frame.at(c, 0)).sw(c, c.g[S0][0]);
            Addr(frame.at(c, 8)).sw(c, c.g[S7][0]);
            Addr(frame.at(c, 0x10)).sw(c, c.g[S2][0]);
            Addr(frame.at(c, 0x18)).sw(c, version);
            Addr(frame.at(c, 0x20)).sw(c, c.g[T6][0]);
            Addr(frame.at(c, 0x28)).sw(c, c.g[T5][0]);
            Addr(frame.at(c, 0x30)).sw(c, c.g[T4][0]);
            Addr(frame.at(c, 0x38)).sw(c, c.g[V1][0]);
            c.g[A1][0] = sx(0x0048_08e0);
            c.g[A2][0] = sx(0x0048_0610);
            call_ptr(c, callee::SPRINTF, 0x0025_f5e0)?;
            c.g[A1][0] = Addr(self_).off(0x98).lw(c);
            c.g[A0][0] = c.g[FP][0];
            c.g[A2][0] = sx(buf);
            callee::SEND_PRIVATE_MESSAGE.call(c, 0x0025_f5f0)?;
            Addr(self_).sw(c, 0x16);
            c.g[V0][0] = 0x16;
        }
        c.cycles += 12;
        frame.restore(c, S0, 0x110);
        frame.restore(c, S1, 0x100);
        frame.restore(c, S2, 0xf0);
        frame.restore(c, S3, 0xe0);
        frame.restore(c, S4, 0xd0);
        frame.restore(c, S5, 0xc0);
        frame.restore(c, S6, 0xb0);
        frame.restore(c, S7, 0xa0);
        frame.restore(c, FP, 0x90);
        frame.restore_ra(c, 0x80);
        frame.close(c);
        Ok(())
    })
}

/// `void LobbyMan_ChallengeRoomUser(LobbyMan* self, int index, int flag)`
/// (0x25f478): challenge the room user at `index`: resolve the user's name
/// and send the challenge to it.
pub fn lobby_man_challenge_room_user(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A2][0];
        c.g[A2][0] = 0;
        callee::GET_ROOM_USER_NAME.call(c, 0x0025_f498)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[S1][0];
        c.g[A1][0] = c.g[V0][0];
        callee::SEND_CHALLENGE.call(c, 0x0025_f4a8)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Rank list and room-user accessors.

/// `unsigned int LobbyMan_GetRankListCount(LobbyMan* self)` (0x25f338,
/// asm-only in the reference): the entry count of the rank list selected
/// by `self + 0xf8` in the table at `self + 0x1ec`, or 0 while the lobby
/// is down or no list is selected.
pub fn lobby_man_get_rank_list_count(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let self_ = r32(c, A0);
        let state = Addr(self_).lw(c);
        c.g[V0][0] = state;
        if state == 0 {
            c.cycles += 2;
            c.g[V0][0] = 0;
            c.cycles += 1;
            frame.restore_ra(c, 0);
            c.cycles += 2;
            frame.close(c);
            return Ok(());
        }
        let index = Addr(self_).off(LOBBY_RANK_INDEX).lw(c);
        c.g[V0][0] = index;
        c.cycles += 5;
        let slot = self_.wrapping_add(((index as u32) << 2) as u32);
        c.g[V0][0] = sx(slot);
        let list = Addr(slot).off(RANK_TABLE).lw(c);
        c.g[A0][0] = list;
        c.g[V0][0] = 0;
        if list == 0 {
            c.cycles += 1;
            frame.restore_ra(c, 0);
            c.cycles += 2;
            frame.close(c);
            return Ok(());
        }
        c.cycles += 4;
        call_ptr(c, callee::LOBBY_LIST_GET_COUNT, 0x0025_f370)?;
        frame.restore_ra(c, 0);
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// `void LobbyMan_GetRoomUserStats(LobbyMan* self, int index, void* out)`
/// (0x25f288): fill `out` (address, name length and the parsed stat totals)
/// for the room user at `index`. With no live session nothing is written;
/// with no user (or no record) the address/length are zeroed and the stats
/// parse runs on an empty record.
pub fn lobby_man_get_room_user_stats(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S0, 0x60);
        frame.save(c, S1, 0x50);
        frame.save_ra(c, 0x40);
        let self_ = r32(c, A0);
        c.g[S0][0] = c.g[A2][0];
        let out = r32(c, A2);
        let state = Addr(self_).lw(c);
        c.g[V0][0] = state;
        if state != 0 {
            c.cycles += 5;
            c.g[A0][0] = Addr(self_).off(LOBBY_USER_LIST).lw(c);
            call_ptr(c, callee::LOBBY_LIST_GET_AT, 0x0025_f2ac)?;
            let entry = c.g[V0][0];
            c.g[A0][0] = entry;
            // The record is the inline tag buffer at `entry + 0x38`; its
            // address doubles as the null check in the generated body.
            let mut record = sx((entry as u32).wrapping_add(0x38));
            c.g[V1][0] = sx(record as u32);
            if entry == 0 || record == 0 {
                c.cycles += 3;
                Addr(out).sw(c, 0);
                c.g[S1][0] = 0;
                Addr(out).off(0x10).sw(c, 0);
                record = 0;
            } else {
                c.cycles += 2;
                c.g[S1][0] = c.g[V1][0];
                c.cycles += 6;
                let addr_word = Addr((entry as u32).wrapping_add(0x34)).lw(c);
                c.g[V0][0] = addr_word;
                Addr(out).sw(c, addr_word);
                c.g[A0][0] = sx((entry as u32).wrapping_add(0x28));
                call_ptr(c, callee::ATOI, 0x0025_f2d0)?;
                Addr(out).off(0x10).sw(c, c.g[V0][0]);
            }
            c.cycles += 15;
            c.g[A0][0] = record;
            c.g[S1][0] = record;
            c.g[A1][0] = sx(frame.at(c, 0));
            c.g[A2][0] = 0x40;
            c.g[A3][0] = sx(0x004a_2f30);
            call_ptr(c, callee::TAG_FIELD_GET_BINARY, 0x0025_f2fc)?;
            let first = Addr(frame.at(c, 8)).lw(c);
            let second = Addr(frame.at(c, 0xc)).lw(c);
            let third = Addr(frame.at(c, 0x10)).lw(c);
            let sixth = Addr(frame.at(c, 0x18)).lw(c);
            c.g[A1][0] = first;
            c.g[V0][0] = sx((first as u32).wrapping_add(second as u32));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(third as u32));
            c.g[V1][0] = third;
            c.g[A0][0] = sixth;
            Addr(out).off(4).sw(c, first);
            Addr(out).off(0xc).sw(c, sixth);
            Addr(out).off(8).sw(c, c.g[V0][0]);
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore_ra(c, 0x40);
        frame.close(c);
        Ok(())
    })
}

// ---------------------------------------------------------------------------
// Quick play.

/// `void LobbyMan_CancelQuickPlay(LobbyMan* self)` (0x25eed8): cancel a
/// quick-play search. From state 0x19 the game setup is notified; from any
/// other live state except idle (1) a "quik" lobby request is sent and the
/// reply timer is armed at 20 seconds. The manager returns to state 1 and
/// a 0xE7 notice is posted to the message list.
pub fn lobby_man_cancel_quick_play(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x140);
        frame.save(c, S1, 0x120);
        frame.save(c, S0, 0x130);
        frame.save(c, S2, 0x110);
        frame.save_ra(c, 0x100);
        let self_ = r32(c, A0);
        c.g[S1][0] = c.g[A0][0];
        let state = Addr(self_).lw(c);
        c.g[V1][0] = state;
        c.g[V0][0] = 0x19;
        if state != 0 {
            if state == 0x19 {
                c.cycles += 2;
                c.g[V0][0] = 1;
                c.cycles += 5;
                callee::NET_GET_GAME_SETUP.call(c, 0x0025_ef0c)?;
                let setup = c.g[V0][0];
                let setup_state = Addr(setup as u32).lw(c);
                c.g[V1][0] = setup_state;
                c.g[V0][0] = 1;
                if setup_state != 0 {
                    c.cycles += 7;
                    callee::NET_GET_GAME_SETUP.call(c, 0x0025_ef20)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A1][0] = 2;
                    call_ptr(c, callee::NET_ABORT_GAME_SETUP, 0x0025_ef2c)?;
                    c.g[V0][0] = 1;
                }
            } else if state == 1 {
                c.cycles += 2;
                c.g[V0][0] = 1;
                c.cycles += 2;
                c.cycles += 1;
                c.g[V0][0] = 1;
            } else {
                c.cycles += 2;
                c.g[V0][0] = 1;
                c.cycles += 2;
                c.cycles += 19;
                let buf = frame.at(c, 0);
                Addr(buf).sb(c, 0);
                c.g[A0][0] = sx(buf);
                c.g[A1][0] = 0x100;
                c.g[A2][0] = sx(0x004a_3200);
                c.g[A3][0] = sx(0x004a_3218);
                call_ptr(c, callee::TAG_FIELD_SET_STRING, 0x0025_ef58)?;
                c.g[AT][0] = 0x41a0_0000;
                c.mtc1(1, 0);
                // `swc1 f0`: the generated body stores 20.0f here.
                c.swc1(0, self_.wrapping_add(LOBBY_QUICK_TIMER));
                c.g[A0][0] = Addr(self_).off(LOBBY_API).lw(c);
                c.g[A1][0] = 0x7175_696b;
                c.g[A2][0] = sx(buf);
                c.g[A3][0] = sx(0x0026_8e78);
                c.g[T0][0] = c.g[S1][0];
                call_ptr(c, callee::LOBBY_API_SEND, 0x0025_ef84)?;
                Addr(self_).off(LOBBY_REQUEST).sw(c, c.g[V0][0]);
                c.cycles += 1;
                c.g[V0][0] = 1;
            }
            c.cycles += 25;
            Addr(self_).sw(c, c.g[V0][0]);
            c.g[A0][0] = 8;
            c.g[A1][0] = sx(0x0048_0488);
            c.g[A2][0] = 0x2000_0000;
            c.g[A3][0] = 0;
            c.g[S0][0] = 0xe7;
            call_ptr(c, callee::EA_MEM_ALLOC_NAMED, 0x0025_efac)?;
            let msg = c.g[V0][0];
            c.g[S2][0] = msg;
            let head = Addr(self_).off(LOBBY_MSG_LIST).lw(c);
            c.g[V1][0] = head;
            Addr(msg as u32).sw(c, c.g[S0][0]);
            Addr((msg as u32).wrapping_add(4)).sw(c, sx(0x0048_12b0));
            Addr(frame.at(c, 0x10)).sw(c, head);
            c.g[A0][0] = 0xc;
            c.g[A1][0] = sx(0x0048_04a8);
            c.g[A2][0] = 0x2000_0000;
            c.g[A3][0] = 0;
            call_ptr(c, callee::EA_MEM_ALLOC_NAMED, 0x0025_efe0)?;
            let node = c.g[V0][0];
            c.g[A1][0] = node;
            if (node as u32).wrapping_add(8) != 0 {
                Addr((node as u32).wrapping_add(8)).sw(c, msg);
            }
            c.cycles += 9;
            let head = Addr(frame.at(c, 0x10)).lw(c);
            c.g[V0][0] = head;
            Addr(node as u32).sw(c, head);
            let prev = Addr((head as u32).wrapping_add(4)).lw(c);
            c.g[V1][0] = prev;
            Addr((node as u32).wrapping_add(4)).sw(c, prev);
            let prev2 = Addr((head as u32).wrapping_add(4)).lw(c);
            c.g[A0][0] = prev2;
            Addr(prev2 as u32).sw(c, node);
            let head = Addr(frame.at(c, 0x10)).lw(c);
            c.g[V0][0] = head;
            Addr((head as u32).wrapping_add(4)).sw(c, node);
            Addr(frame.at(c, 0)).sw(c, node);
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

// ---------------------------------------------------------------------------
// User stats and lookup-user request/reply pairs.

/// `void LobbyMan_OnUserStatsReply(LobbyMan* self, void* msg)` (0x25eb00):
/// handle a user-stats reply. A reply with no type code carries a tagged
/// user record: its name fields, counts and stat structure are parsed into
/// a 0x10D message posted to the list at `self + 0xf0`. Any other type
/// except `uusr` (0x75757372) becomes a bare 0x102 error message; `uusr`
/// itself is ignored.
pub fn lobby_man_on_user_stats_reply(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x430);
        frame.save(c, S6, 0x3c0);
        frame.save(c, S7, 0x3b0);
        let msg = r32(c, A1);
        c.g[S6][0] = c.g[A1][0];
        frame.save(c, S0, 0x420);
        frame.save(c, S2, 0x400);
        frame.save(c, S3, 0x3f0);
        frame.save(c, S4, 0x3e0);
        frame.save(c, S5, 0x3d0);
        frame.save_ra(c, 0x3a0);
        frame.save(c, S1, 0x410);
        let self_ = r32(c, A0);
        c.g[S7][0] = c.g[A0][0];
        let reply_type = Addr(msg).off(8).lw(c);
        c.g[S1][0] = reply_type;
        if reply_type == 0 {
            c.cycles += 127;
            let record = Addr(msg).off(0xc).lw(c) as u32;
            // Three name strings, two counts, the raw stat string, a third
            // count, then the parsed structure.
            c.g[A0][0] = sx(record);
            c.g[A1][0] = sx(0x004a_31d8);
            call_ptr(c, callee::TAG_FIELD_FIND, 0x0025_eb48)?;
            c.g[S0][0] = sx(0x004a_2f40);
            c.g[A1][0] = sx(frame.at(c, 0));
            c.g[A2][0] = 0x100;
            c.g[A3][0] = c.g[S0][0];
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, callee::TAG_FIELD_GET_STRING, 0x0025_eb64)?;
            c.g[A0][0] = sx(record);
            c.g[A1][0] = sx(0x004a_2f38);
            call_ptr(c, callee::TAG_FIELD_FIND, 0x0025_eb74)?;
            let s2 = frame.at(c, 0x100);
            c.g[S5][0] = sx(s2);
            c.g[A3][0] = c.g[S0][0];
            c.g[A2][0] = 0x100;
            c.g[A1][0] = sx(s2);
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, callee::TAG_FIELD_GET_STRING, 0x0025_eb8c)?;
            c.g[A0][0] = sx(record);
            c.g[A1][0] = sx(0x004a_31e0);
            call_ptr(c, callee::TAG_FIELD_FIND, 0x0025_eb9c)?;
            let s3 = frame.at(c, 0x200);
            c.g[S4][0] = sx(s3);
            c.g[A3][0] = c.g[S0][0];
            c.g[A2][0] = 0x100;
            c.g[A1][0] = sx(s3);
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, callee::TAG_FIELD_GET_STRING, 0x0025_ebb4)?;
            c.g[A0][0] = sx(record);
            c.g[A1][0] = sx(0x004a_31e8);
            call_ptr(c, callee::TAG_FIELD_FIND, 0x0025_ebc4)?;
            c.g[A1][0] = u64::MAX;
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, callee::TAG_FIELD_GET_NUMBER, 0x0025_ebd0)?;
            let count1 = c.g[V0][0];
            c.g[S2][0] = count1;
            c.g[A0][0] = sx(record);
            c.g[A1][0] = sx(0x004a_31f0);
            call_ptr(c, callee::TAG_FIELD_FIND, 0x0025_ebe4)?;
            c.g[A1][0] = u64::MAX;
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, callee::TAG_FIELD_GET_NUMBER, 0x0025_ebf0)?;
            let count2 = c.g[V0][0];
            c.g[S3][0] = count2;
            c.g[A0][0] = sx(record);
            c.g[A1][0] = sx(0x004a_31f8);
            call_ptr(c, callee::TAG_FIELD_FIND, 0x0025_ec04)?;
            let raw = frame.at(c, 0x300);
            c.g[S1][0] = sx(raw);
            c.g[A3][0] = c.g[S0][0];
            c.g[A2][0] = 0x14;
            c.g[A1][0] = sx(raw);
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, callee::TAG_FIELD_GET_STRING, 0x0025_ec1c)?;
            c.g[A0][0] = sx(record);
            c.g[A1][0] = sx(0x004a_3130);
            call_ptr(c, callee::TAG_FIELD_FIND, 0x0025_ec2c)?;
            c.g[A1][0] = 0;
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, callee::TAG_FIELD_GET_NUMBER, 0x0025_ec38)?;
            let count3 = c.g[V0][0];
            c.g[S0][0] = count3;
            let stats = frame.at(c, 0x320);
            c.g[A3][0] = sx(0x004a_2f30);
            c.g[A1][0] = sx(stats);
            c.g[A0][0] = sx(raw);
            c.g[A2][0] = 0x40;
            call_ptr(c, callee::TAG_FIELD_GET_BINARY, 0x0025_ec54)?;
            let st2 = Addr(frame.at(c, 0x328)).lw(c);
            let st3 = Addr(frame.at(c, 0x32c)).lw(c);
            let st4 = Addr(frame.at(c, 0x330)).lw(c);
            let st6 = Addr(frame.at(c, 0x338)).lw(c);
            c.g[T0][0] = st2;
            c.g[V0][0] = sx((st2 as u32).wrapping_add(st3 as u32));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(st4 as u32));
            c.g[V1][0] = st4;
            Addr(frame.at(c, 0x360)).sw(c, count3);
            Addr(frame.at(c, 0x36c)).sw(c, st6);
            Addr(frame.at(c, 0x368)).sw(c, c.g[V0][0]);
            Addr(frame.at(c, 0x364)).sw(c, st2);
            Addr(frame.at(c, 0x370)).sw(c, count1);
            c.g[A0][0] = 0x164;
            c.g[A1][0] = sx(0x0048_0488);
            c.g[A2][0] = 0x2000_0000;
            c.g[A3][0] = 0;
            call_ptr(c, callee::EA_MEM_ALLOC_NAMED, 0x0025_ec98)?;
            let out_msg = c.g[V0][0] as u32;
            c.g[S0][0] = c.g[V0][0];
            Addr(out_msg).off(4).sw(c, sx(0x0048_1110));
            Addr(out_msg).sw(c, 0x10d);
            Addr(out_msg).off(0x148).sw(c, count2);
            Addr(out_msg).off(0x14c).sw(c, count1);
            c.g[A0][0] = sx(out_msg.wrapping_add(8));
            c.g[A1][0] = sx(frame.at(c, 0));
            call_ptr(c, callee::STRCPY, 0x0025_ecc4)?;
            c.g[A0][0] = sx(out_msg.wrapping_add(0x28));
            c.g[A1][0] = c.g[S5][0];
            call_ptr(c, callee::STRCPY, 0x0025_ecd0)?;
            c.g[A0][0] = sx(out_msg.wrapping_add(0x48));
            c.g[A1][0] = c.g[S4][0];
            call_ptr(c, callee::STRCPY, 0x0025_ecdc)?;
            // Copy the five-word stat block (frame +0x360) to `msg + 0x150`,
            // with the generated body's unaligned `ldl/ldr` pair (through
            // v0/a0, which stay clobbered exactly as generated).
            c.ldl(2, frame.at(c, 0x367));
            c.ldr(2, frame.at(c, 0x360));
            c.ldl(4, frame.at(c, 0x36f));
            c.ldr(4, frame.at(c, 0x368));
            c.g[A2][0] = Addr(frame.at(c, 0x370)).lw(c);
            c.sdl(2, out_msg.wrapping_add(0x157));
            c.sdr(2, out_msg.wrapping_add(0x150));
            c.sdl(4, out_msg.wrapping_add(0x15f));
            c.sdr(4, out_msg.wrapping_add(0x158));
            Addr(out_msg).off(0x160).sw(c, c.g[A2][0]);
            let head = Addr(self_).off(LOBBY_MSG_LIST).lw(c);
            c.g[V1][0] = head;
            Addr(frame.at(c, 0x390)).sw(c, head);
            c.g[A0][0] = 0xc;
            c.g[A1][0] = sx(0x0048_04a8);
            c.g[A2][0] = 0x2000_0000;
            c.g[A3][0] = 0;
            call_ptr(c, callee::EA_MEM_ALLOC_NAMED, 0x0025_ed24)?;
            let node = c.g[V0][0];
            c.g[A1][0] = node;
            if (node as u32).wrapping_add(8) != 0 {
                Addr((node as u32).wrapping_add(8)).sw(c, c.g[S0][0]);
            }
            c.cycles += 10;
            let head = Addr(frame.at(c, 0x390)).lw(c);
            c.g[V0][0] = head;
            Addr(node as u32).sw(c, head);
            let prev = Addr((head as u32).wrapping_add(4)).lw(c);
            c.g[V1][0] = prev;
            Addr((node as u32).wrapping_add(4)).sw(c, prev);
            let prev2 = Addr((head as u32).wrapping_add(4)).lw(c);
            c.g[A0][0] = prev2;
            Addr(prev2 as u32).sw(c, node);
            let head = Addr(frame.at(c, 0x390)).lw(c);
            c.g[V0][0] = head;
            Addr((head as u32).wrapping_add(4)).sw(c, node);
            Addr(frame.at(c, 0x380)).sw(c, node);
        } else {
            c.cycles += 4;
            c.g[V0][0] = 0x7575_7372;
            c.g[A1][0] = 0x480000;
            if reply_type != 0x7575_7372 {
                c.cycles += 24;
                c.g[A0][0] = 0xc;
                c.g[A1][0] = sx(0x0048_0488);
                c.g[A2][0] = 0x2000_0000;
                c.g[A3][0] = 0;
                call_ptr(c, callee::EA_MEM_ALLOC_NAMED, 0x0025_ed80)?;
                let err_msg = c.g[V0][0];
                c.g[S0][0] = err_msg;
                let head = Addr(self_).off(LOBBY_MSG_LIST).lw(c);
                Addr(err_msg as u32).sw(c, 0x102);
                Addr((err_msg as u32).wrapping_add(4)).sw(c, sx(0x0048_1290));
                Addr((err_msg as u32).wrapping_add(8)).sw(c, reply_type);
                Addr(frame.at(c, 0x10)).sw(c, head);
                c.g[A0][0] = 0xc;
                c.g[A1][0] = sx(0x0048_04a8);
                c.g[A2][0] = 0x2000_0000;
                c.g[A3][0] = 0;
                call_ptr(c, callee::EA_MEM_ALLOC_NAMED, 0x0025_edbc)?;
                let node = c.g[V0][0];
                c.g[A1][0] = node;
                if (node as u32).wrapping_add(8) != 0 {
                    Addr((node as u32).wrapping_add(8)).sw(c, err_msg);
                }
                c.cycles += 9;
                let head = Addr(frame.at(c, 0x10)).lw(c);
                c.g[V0][0] = head;
                Addr(node as u32).sw(c, head);
                let prev = Addr((head as u32).wrapping_add(4)).lw(c);
                c.g[V1][0] = prev;
                Addr((node as u32).wrapping_add(4)).sw(c, prev);
                let prev2 = Addr((head as u32).wrapping_add(4)).lw(c);
                c.g[A0][0] = prev2;
                Addr(prev2 as u32).sw(c, node);
                let head = Addr(frame.at(c, 0x10)).lw(c);
                c.g[V0][0] = head;
                Addr((head as u32).wrapping_add(4)).sw(c, node);
                Addr(frame.at(c, 0)).sw(c, node);
            }
        }
        c.cycles += 11;
        frame.restore(c, S0, 0x420);
        frame.restore(c, S1, 0x410);
        frame.restore(c, S2, 0x400);
        frame.restore(c, S3, 0x3f0);
        frame.restore(c, S4, 0x3e0);
        frame.restore(c, S5, 0x3d0);
        frame.restore(c, S6, 0x3c0);
        frame.restore(c, S7, 0x3b0);
        frame.restore_ra(c, 0x3a0);
        frame.close(c);
        Ok(())
    })
}

/// `void LobbyMan_QueryUserStats(LobbyMan* self, char* name)` (0x25eaa0):
/// ask the lobby service for `name`'s stats (an "onln" request carrying
/// the name as a tagged string).
pub fn lobby_man_query_user_stats(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0x120);
        frame.save(c, S0, 0x110);
        frame.save_ra(c, 0x100);
        let self_ = r32(c, A0);
        c.g[S0][0] = c.g[A0][0];
        let name = c.g[A1][0];
        c.g[A3][0] = name;
        let buf = frame.at(c, 0);
        Addr(buf).sb(c, 0);
        c.g[A0][0] = sx(buf);
        c.g[A1][0] = 0x100;
        c.g[A2][0] = sx(0x004a_3108);
        call_ptr(c, callee::TAG_FIELD_SET_STRING, 0x0025_eacc)?;
        c.g[A0][0] = Addr(self_).off(LOBBY_API).lw(c);
        c.g[A1][0] = 0x6f6e_6c6e;
        c.g[A2][0] = sx(buf);
        c.g[A3][0] = sx(0x0026_8e50);
        c.g[T0][0] = c.g[S0][0];
        call_ptr(c, callee::LOBBY_API_SEND, 0x0025_eaec)?;
        frame.restore(c, S0, 0x110);
        frame.restore_ra(c, 0x100);
        frame.close(c);
        Ok(())
    })
}

/// `void LobbyMan_OnLookupUserReply(LobbyMan* self, void* msg)` (0x25e7f0):
/// handle a lookup-user reply. The pending flag is cleared; a successful
/// reply's tagged record (name, stat structure, counts and text) becomes a
/// 0xE3 message on the list at `self + 0xf0`, any error code becomes a
/// bare 0x102 message carrying the code.
pub fn lobby_man_on_lookup_user_reply(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x3e0);
        frame.save(c, S4, 0x390);
        frame.save(c, S2, 0x3b0);
        let self_ = r32(c, A0);
        c.g[S4][0] = c.g[A0][0];
        frame.save(c, S0, 0x3d0);
        let msg = r32(c, A1);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S1, 0x3c0);
        frame.save(c, S3, 0x3a0);
        frame.save_ra(c, 0x380);
        Addr(self_).off(LOBBY_LOOKUP_PENDING).sw(c, 0);
        let err = Addr(msg).off(8).lw(c);
        c.g[S1][0] = err;
        if err == 0 {
            c.cycles += 107;
            let record = Addr(msg).off(0xc).lw(c) as u32;
            c.g[A0][0] = sx(record);
            c.g[A1][0] = sx(0x004a_3108);
            call_ptr(c, callee::TAG_FIELD_FIND, 0x0025_e834)?;
            c.g[S1][0] = sx(0x004a_2f40);
            let fmt = c.g[S1][0];
            c.g[A1][0] = sx(frame.at(c, 0));
            c.g[A2][0] = 0x100;
            c.g[A3][0] = fmt;
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, callee::TAG_FIELD_GET_STRING, 0x0025_e850)?;
            c.g[A0][0] = sx(record);
            c.g[A1][0] = sx(0x004a_3128);
            call_ptr(c, callee::TAG_FIELD_FIND, 0x0025_e860)?;
            let stat_text = frame.at(c, 0x220);
            c.g[S0][0] = sx(stat_text);
            c.g[A3][0] = fmt;
            c.g[A1][0] = sx(stat_text);
            c.g[A0][0] = c.g[V0][0];
            c.g[A2][0] = 0x100;
            call_ptr(c, callee::TAG_FIELD_GET_STRING, 0x0025_e878)?;
            let stats = frame.at(c, 0x320);
            c.g[A3][0] = sx(0x004a_2f30);
            c.g[A1][0] = sx(stats);
            c.g[A0][0] = sx(stat_text);
            c.g[A2][0] = 0x40;
            call_ptr(c, callee::TAG_FIELD_GET_BINARY, 0x0025_e890)?;
            let st2 = Addr(frame.at(c, 0x328)).lw(c);
            let st3 = Addr(frame.at(c, 0x32c)).lw(c);
            let st4 = Addr(frame.at(c, 0x330)).lw(c);
            let st6 = Addr(frame.at(c, 0x338)).lw(c);
            c.g[A3][0] = st2;
            c.g[V0][0] = sx((st2 as u32).wrapping_add(st3 as u32));
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(st4 as u32));
            c.g[V1][0] = st4;
            c.g[A2][0] = st6;
            Addr(frame.at(c, 0x20c)).sw(c, st6);
            Addr(frame.at(c, 0x204)).sw(c, st2);
            Addr(frame.at(c, 0x208)).sw(c, c.g[V0][0]);
            c.g[A0][0] = sx(record);
            c.g[A1][0] = sx(0x004a_31c0);
            call_ptr(c, callee::TAG_FIELD_FIND, 0x0025_e8c4)?;
            c.g[A1][0] = u64::MAX;
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, callee::TAG_FIELD_GET_NUMBER, 0x0025_e8d0)?;
            let rec_id = c.g[V0][0];
            Addr(frame.at(c, 0x210)).sw(c, u64::MAX);
            Addr(frame.at(c, 0x200)).sw(c, rec_id);
            c.g[V1][0] = u64::MAX;
            c.g[A0][0] = sx(record);
            c.g[A1][0] = sx(0x004a_2fa0);
            call_ptr(c, callee::TAG_FIELD_FIND, 0x0025_e8ec)?;
            c.g[A1][0] = 0;
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, callee::TAG_FIELD_GET_NUMBER, 0x0025_e8f8)?;
            let count = c.g[V0][0];
            c.g[S2][0] = count;
            c.g[A0][0] = sx(record);
            c.g[A1][0] = sx(0x004a_31c8);
            call_ptr(c, callee::TAG_FIELD_FIND, 0x0025_e90c)?;
            let online = ((count as i64) > 0) as u64;
            c.g[S3][0] = online;
            let text = frame.at(c, 0x100);
            c.g[S0][0] = sx(text);
            c.g[A3][0] = fmt;
            c.g[A1][0] = sx(text);
            c.g[A0][0] = c.g[V0][0];
            c.g[A2][0] = 0x100;
            call_ptr(c, callee::TAG_FIELD_GET_STRING, 0x0025_e928)?;
            c.g[A0][0] = 0x2c;
            c.g[A1][0] = sx(0x0048_0488);
            c.g[A2][0] = 0x2000_0000;
            c.g[A3][0] = 0;
            call_ptr(c, callee::EA_MEM_ALLOC_NAMED, 0x0025_e940)?;
            let out_msg = c.g[V0][0] as u32;
            c.g[S1][0] = c.g[V0][0];
            Addr(out_msg).off(4).sw(c, sx(0x0048_1190));
            Addr(out_msg).sw(c, 0xe3);
            c.g[A0][0] = sx(out_msg.wrapping_add(8));
            c.g[A1][0] = sx(frame.at(c, 0));
            call_ptr(c, callee::C_BX_STRING_CTOR_CSTR, 0x0025_e964)?;
            c.g[A0][0] = sx(out_msg.wrapping_add(0xc));
            c.g[A1][0] = sx(text);
            call_ptr(c, callee::C_BX_STRING_CTOR_CSTR, 0x0025_e970)?;
            Addr(out_msg).off(0x10).sw(c, online);
            Addr(out_msg).off(0x14).sw(c, count);
            // The five-word record (frame +0x200) to `msg + 0x18`, again
            // through the generated `ldl/ldr` registers (v1/a2).
            c.ldl(3, frame.at(c, 0x207));
            c.ldr(3, frame.at(c, 0x200));
            c.ldl(6, frame.at(c, 0x20f));
            c.ldr(6, frame.at(c, 0x208));
            c.g[A3][0] = Addr(frame.at(c, 0x210)).lw(c);
            c.sdl(3, out_msg.wrapping_add(0x1f));
            c.sdr(3, out_msg.wrapping_add(0x18));
            c.sdl(6, out_msg.wrapping_add(0x27));
            c.sdr(6, out_msg.wrapping_add(0x20));
            Addr(out_msg).off(0x28).sw(c, c.g[A3][0]);
            let head = Addr(self_).off(LOBBY_MSG_LIST).lw(c);
            c.g[V0][0] = head;
            Addr(frame.at(c, 0x370)).sw(c, head);
            c.g[A0][0] = 0xc;
            c.g[A1][0] = sx(0x0048_04a8);
            c.g[A2][0] = 0x2000_0000;
            c.g[A3][0] = 0;
            call_ptr(c, callee::EA_MEM_ALLOC_NAMED, 0x0025_e9c0)?;
            let node = c.g[V0][0];
            c.g[A1][0] = node;
            if (node as u32).wrapping_add(8) != 0 {
                Addr((node as u32).wrapping_add(8)).sw(c, c.g[S1][0]);
            }
            c.cycles += 10;
            let head = Addr(frame.at(c, 0x370)).lw(c);
            c.g[V0][0] = head;
            Addr(node as u32).sw(c, head);
            let prev = Addr((head as u32).wrapping_add(4)).lw(c);
            c.g[V1][0] = prev;
            Addr((node as u32).wrapping_add(4)).sw(c, prev);
            let prev2 = Addr((head as u32).wrapping_add(4)).lw(c);
            c.g[A0][0] = prev2;
            Addr(prev2 as u32).sw(c, node);
            let head = Addr(frame.at(c, 0x370)).lw(c);
            c.g[V0][0] = head;
            Addr((head as u32).wrapping_add(4)).sw(c, node);
            Addr(frame.at(c, 0x360)).sw(c, node);
        } else {
            c.cycles += 24;
            c.g[A0][0] = 0xc;
            c.g[A1][0] = sx(0x0048_0488);
            c.g[A2][0] = 0x2000_0000;
            c.g[A3][0] = 0;
            call_ptr(c, callee::EA_MEM_ALLOC_NAMED, 0x0025_ea0c)?;
            let err_msg = c.g[V0][0];
            c.g[S0][0] = err_msg;
            let head = Addr(self_).off(LOBBY_MSG_LIST).lw(c);
            Addr(err_msg as u32).sw(c, 0x102);
            Addr((err_msg as u32).wrapping_add(4)).sw(c, sx(0x0048_1290));
            Addr((err_msg as u32).wrapping_add(8)).sw(c, err);
            Addr(frame.at(c, 0x10)).sw(c, head);
            c.g[A0][0] = 0xc;
            c.g[A1][0] = sx(0x0048_04a8);
            c.g[A2][0] = 0x2000_0000;
            c.g[A3][0] = 0;
            call_ptr(c, callee::EA_MEM_ALLOC_NAMED, 0x0025_ea48)?;
            let node = c.g[V0][0];
            c.g[A1][0] = node;
            if (node as u32).wrapping_add(8) != 0 {
                Addr((node as u32).wrapping_add(8)).sw(c, err_msg);
            }
            c.cycles += 9;
            let head = Addr(frame.at(c, 0x10)).lw(c);
            c.g[V0][0] = head;
            Addr(node as u32).sw(c, head);
            let prev = Addr((head as u32).wrapping_add(4)).lw(c);
            c.g[V1][0] = prev;
            Addr((node as u32).wrapping_add(4)).sw(c, prev);
            let prev2 = Addr((head as u32).wrapping_add(4)).lw(c);
            c.g[A0][0] = prev2;
            Addr(prev2 as u32).sw(c, node);
            let head = Addr(frame.at(c, 0x10)).lw(c);
            c.g[V0][0] = head;
            Addr((head as u32).wrapping_add(4)).sw(c, node);
            Addr(frame.at(c, 0)).sw(c, node);
        }
        c.cycles += 8;
        frame.restore(c, S0, 0x3d0);
        frame.restore(c, S1, 0x3c0);
        frame.restore(c, S2, 0x3b0);
        frame.restore(c, S3, 0x3a0);
        frame.restore(c, S4, 0x390);
        frame.restore_ra(c, 0x380);
        frame.close(c);
        Ok(())
    })
}

/// `void LobbyMan_LookupUser(LobbyMan* self, char* name)` (0x25e6d0): look
/// a user up by name. A name already in the peer set just posts a 0x10E
/// notice; any other name is sent as a "user" lobby request (20 second
/// timeout) and the lookup flag at `self + 0x44` is set.
pub fn lobby_man_lookup_user(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x160);
        frame.save(c, S1, 0x140);
        frame.save(c, S2, 0x130);
        frame.save(c, S0, 0x150);
        frame.save_ra(c, 0x120);
        let self_ = r32(c, A0);
        c.g[S2][0] = c.g[A0][0];
        let name = c.g[A1][0];
        c.g[S1][0] = name;
        let state = Addr(self_).lw(c);
        c.g[V0][0] = state;
        if state != 0 {
            c.cycles += 4;
            c.g[A0][0] = Addr(self_).off(LOBBY_PEER_SET).lw(c);
            c.g[A1][0] = name;
            call_ptr(c, callee::STRCASECMP, 0x0025_e6fc)?;
            let buf = frame.at(c, 0x20);
            c.g[S0][0] = sx(buf);
            if c.g[V0][0] == 0 {
                c.cycles += 24;
                c.g[A0][0] = 8;
                c.g[A1][0] = sx(0x0048_0488);
                c.g[A2][0] = 0x2000_0000;
                c.g[A3][0] = 0;
                c.g[S0][0] = 0x10e;
                call_ptr(c, callee::EA_MEM_ALLOC_NAMED, 0x0025_e720)?;
                let msg = c.g[V0][0];
                c.g[S1][0] = msg;
                let head = Addr(self_).off(LOBBY_MSG_LIST).lw(c);
                c.g[V1][0] = head;
                Addr(msg as u32).sw(c, 0x10e);
                Addr((msg as u32).wrapping_add(4)).sw(c, sx(0x0048_12b0));
                Addr(frame.at(c, 0x10)).sw(c, head);
                c.g[A0][0] = 0xc;
                c.g[A1][0] = sx(0x0048_04a8);
                c.g[A2][0] = 0x2000_0000;
                c.g[A3][0] = 0;
                call_ptr(c, callee::EA_MEM_ALLOC_NAMED, 0x0025_e754)?;
                let node = c.g[V0][0];
                c.g[A1][0] = node;
                if (node as u32).wrapping_add(8) != 0 {
                    Addr((node as u32).wrapping_add(8)).sw(c, msg);
                }
                c.cycles += 10;
                let head = Addr(frame.at(c, 0x10)).lw(c);
                c.g[V0][0] = head;
                Addr(node as u32).sw(c, head);
                let prev = Addr((head as u32).wrapping_add(4)).lw(c);
                c.g[V1][0] = prev;
                Addr((node as u32).wrapping_add(4)).sw(c, prev);
                let prev2 = Addr((head as u32).wrapping_add(4)).lw(c);
                c.g[A0][0] = prev2;
                Addr(prev2 as u32).sw(c, node);
                let head = Addr(frame.at(c, 0x10)).lw(c);
                c.g[V0][0] = head;
                Addr((head as u32).wrapping_add(4)).sw(c, node);
                Addr(frame.at(c, 0)).sw(c, node);
            } else {
                c.cycles += 19;
                Addr(buf).sb(c, 0);
                c.g[A0][0] = sx(buf);
                c.g[A1][0] = 0x100;
                c.g[A2][0] = sx(0x004a_3108);
                c.g[A3][0] = name;
                call_ptr(c, callee::TAG_FIELD_SET_STRING, 0x0025_e7a8)?;
                Addr(self_).off(LOBBY_LOOKUP_PENDING).sw(c, 1);
                c.g[V1][0] = 1;
                c.g[A0][0] = c.g[S2][0];
                c.g[A1][0] = Addr(self_).lw(c);
                c.g[A2][0] = 0x7573_6572;
                c.g[A3][0] = sx(buf);
                c.g[T0][0] = sx(0x0026_8e28);
                c.g[AT][0] = 0x41a0_0000;
                c.mtc1(1, 12);
                callee::SEND_REQUEST.call(c, 0x0025_e7d8)?;
            }
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
    use crate::difftest::fakes::machine;
    use crate::difftest::{check, DATA, GAME_GP};
    use crate::game::be::beintstat::tests::allocator;
    use crate::game::dirtysock::tagsunk::tests::lobby_object;

    /// A circular message-list head for the lobby at `self_`: head node and
    /// its predecessor, as the insert sequence in every port expects.
    fn msg_list(c: &mut Cpu, self_: u32) {
        let head = DATA + 0x900;
        let prev = DATA + 0x9c0;
        c.bus.write32(self_.wrapping_add(LOBBY_MSG_LIST), head);
        c.bus.write32(head.wrapping_add(4), prev);
        c.bus.write32(prev, 0);
        c.bus.write32(prev.wrapping_add(4), head);
    }

    fn put_str(c: &mut Cpu, addr: u32, text: &[u8]) {
        for (i, b) in text.iter().enumerate() {
            c.bus.write8(addr + i as u32, *b);
        }
        c.bus.write8(addr + text.len() as u32, 0);
    }

    // Branch steering is always `seed % n` arithmetic (never the RNG):
    // state 0 (down), 0x15, 0x16, 0x18, 0x19, other (1); notify = seed % 2;
    // abort = (seed / 2) % 2.
    #[test]
    fn lobby_man_cancel_challenge_matches() {
        check(
            ee::s06::f_0025f628__generated,
            lobby_man_cancel_challenge,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x25f628, 3);
                allocator(c);
                let self_ = DATA + 0x100;
                let state = [0u32, 0x15, 0x16, 0x18, 0x19, 1][(seed % 6) as usize];
                c.bus.write32(self_, state);
                c.bus.write32(self_.wrapping_add(LOBBY_API), DATA + 0x400);
                c.bus.write32(self_.wrapping_add(LOBBY_REQUEST), 0x2a);
                msg_list(c, self_);
                c.g[A0][0] = sx(self_);
                c.g[A1][0] = (seed % 2) as u64;
                c.g[A2][0] = ((seed / 2) % 2) as u64;
                c.pc = 0x0025_f628;
            },
        );
    }

    // States: down (seed % 2 == 0) and live (1). The live arm runs the real
    // back-end chain on a dummy BELibrary/world, as the merged unit's
    // fixtures do.
    #[test]
    fn lobby_man_send_challenge_matches() {
        check(
            ee::s06::f_0025f4c0__generated,
            lobby_man_send_challenge,
            60,
            &|c, seed| {
                let _r = machine(c, seed, 0x25f4c0, 4);
                allocator(c);
                c.bus.write32(GAME_GP.wrapping_sub(0x1f04), DATA + 0x3_0000);
                let world = DATA + 0x3_1000;
                c.bus.write32(GAME_GP.wrapping_sub(0x848), world);
                c.bus.write32(world.wrapping_add(0x78), DATA + 0x3_2000);
                c.bus.write32(GAME_GP.wrapping_sub(0x238), DATA + 0x3_3000);
                let self_ = DATA + 0x100;
                c.bus.write32(self_, (seed % 2) as u32);
                put_str(c, DATA + 0x500, b"bob");
                c.g[A0][0] = sx(self_);
                c.g[A1][0] = sx(DATA + 0x500);
                c.g[A2][0] = (seed % 4) as u64;
                c.pc = 0x0025_f4c0;
            },
        );
    }

    // The lobby is down for every seed, so the name lookup and the send
    // both take their early-outs; `index`/`flag` still vary by seed.
    #[test]
    fn lobby_man_challenge_room_user_matches() {
        check(
            ee::s06::f_0025f478__generated,
            lobby_man_challenge_room_user,
            60,
            &|c, seed| {
                let _r = machine(c, seed, 0x25f478, 2);
                let self_ = DATA + 0x100;
                c.bus.write32(self_, 0);
                c.g[A0][0] = sx(self_);
                c.g[A1][0] = (seed % 4) as u64;
                c.g[A2][0] = (seed % 2) as u64;
                c.pc = 0x0025_f478;
            },
        );
    }

    // Down / no list selected / list present, by seed arithmetic; the
    // selected index is `seed % 4`.
    #[test]
    fn lobby_man_get_rank_list_count_matches() {
        check(
            ee::s06::f_0025f338__generated,
            lobby_man_get_rank_list_count,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x25f338, 1);
                let self_ = DATA + 0x100;
                c.bus.write32(self_, if seed % 3 == 0 { 0 } else { 1 });
                c.bus
                    .write32(self_.wrapping_add(LOBBY_RANK_INDEX), seed % 4);
                let list = DATA + 0x2000;
                c.bus.write32(list.wrapping_add(0x18), seed % 5);
                for i in 0..4u32 {
                    let entry = if (seed / 4) % 2 == 0 { 0 } else { list };
                    c.bus.write32(self_.wrapping_add(RANK_TABLE + 4 * i), entry);
                }
                c.g[A0][0] = sx(self_);
                c.pc = 0x0025_f338;
            },
        );
    }

    // Reuses the merged unit's `lobby_object` (live/down by seed, user list
    // at +0x60), with the stat fields of each room entry filled in.
    #[test]
    fn lobby_man_get_room_user_stats_matches() {
        check(
            ee::s06::f_0025f288__generated,
            lobby_man_get_room_user_stats,
            120,
            &|c, seed| {
                lobby_object(c, seed, 0x25f288, &[LOBBY_USER_LIST]);
                let scratch = DATA + 0x1000;
                for i in 0..4u32 {
                    let room = scratch + 0x2000 + 0x100 * i;
                    put_str(c, room.wrapping_add(0x28), b"bob");
                    c.bus.write32(room.wrapping_add(0x34), 0x100 + i);
                }
                c.g[A2][0] = sx(DATA + 0x8000);
                c.pc = 0x0025_f288;
            },
        );
    }

    // States down / idle (1) / quick-play (0x19) / other (2), by seed.
    #[test]
    fn lobby_man_cancel_quick_play_matches() {
        check(
            ee::s06::f_0025eed8__generated,
            lobby_man_cancel_quick_play,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x25eed8, 3);
                allocator(c);
                let self_ = DATA + 0x100;
                let state = [0u32, 1, 0x19, 2][(seed % 4) as usize];
                c.bus.write32(self_, state);
                c.bus.write32(self_.wrapping_add(LOBBY_API), DATA + 0x400);
                msg_list(c, self_);
                c.g[A0][0] = sx(self_);
                c.pc = 0x0025_eed8;
            },
        );
    }

    // Reply types: record (0), ignored (`uusr`), error (other), by seed.
    #[test]
    fn lobby_man_on_user_stats_reply_matches() {
        check(
            ee::s06::f_0025eb00__generated,
            lobby_man_on_user_stats_reply,
            90,
            &|c, seed| {
                let _r = machine(c, seed, 0x25eb00, 6);
                allocator(c);
                let self_ = DATA + 0x100;
                msg_list(c, self_);
                let msg = DATA + 0x800;
                let ty = [0u32, 0x7575_7372, 7][(seed % 3) as usize];
                c.bus.write32(msg.wrapping_add(8), ty);
                c.bus.write32(msg.wrapping_add(0xc), DATA + 0x400);
                c.g[A0][0] = sx(self_);
                c.g[A1][0] = sx(msg);
                c.pc = 0x0025_eb00;
            },
        );
    }

    #[test]
    fn lobby_man_query_user_stats_matches() {
        check(
            ee::s06::f_0025eaa0__generated,
            lobby_man_query_user_stats,
            60,
            &|c, seed| {
                let _r = machine(c, seed, 0x25eaa0, 2);
                let self_ = DATA + 0x100;
                c.bus.write32(
                    self_.wrapping_add(LOBBY_API),
                    if seed % 2 == 0 { 0 } else { DATA + 0x400 },
                );
                put_str(c, DATA + 0x500, b"bob");
                c.g[A0][0] = sx(self_);
                c.g[A1][0] = sx(DATA + 0x500);
                c.pc = 0x0025_eaa0;
            },
        );
    }

    // Reply error codes: success (0) and failure (3), by seed.
    #[test]
    fn lobby_man_on_lookup_user_reply_matches() {
        check(
            ee::s06::f_0025e7f0__generated,
            lobby_man_on_lookup_user_reply,
            90,
            &|c, seed| {
                let _r = machine(c, seed, 0x25e7f0, 6);
                allocator(c);
                let self_ = DATA + 0x100;
                msg_list(c, self_);
                let msg = DATA + 0x800;
                c.bus
                    .write32(msg.wrapping_add(8), if seed % 2 == 0 { 0 } else { 3 });
                c.bus.write32(msg.wrapping_add(0xc), DATA + 0x400);
                c.g[A0][0] = sx(self_);
                c.g[A1][0] = sx(msg);
                c.pc = 0x0025_e7f0;
            },
        );
    }

    // Down (seed % 3 == 0); otherwise the peer-set entry holds the queried
    // name (seed % 3 == 1, the already-known arm if 0x41aa88 compares
    // strings) or a different one (the request arm).
    #[test]
    fn lobby_man_lookup_user_matches() {
        check(
            ee::s06::f_0025e6d0__generated,
            lobby_man_lookup_user,
            90,
            &|c, seed| {
                let _r = machine(c, seed, 0x25e6d0, 3);
                allocator(c);
                let self_ = DATA + 0x100;
                c.bus.write32(self_, if seed % 3 == 0 { 0 } else { 1 });
                c.bus
                    .write32(self_.wrapping_add(LOBBY_PEER_SET), DATA + 0x600);
                c.bus.write32(self_.wrapping_add(LOBBY_API), DATA + 0x400);
                put_str(c, DATA + 0x600, if seed % 3 == 1 { b"bob" } else { b"amy" });
                put_str(c, DATA + 0x500, b"bob");
                msg_list(c, self_);
                c.g[A0][0] = sx(self_);
                c.g[A1][0] = sx(DATA + 0x500);
                c.pc = 0x0025_e6d0;
            },
        );
    }
}
