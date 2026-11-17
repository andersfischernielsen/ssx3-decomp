//! Staging module oc1005 (wave wave): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1005`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] =
    &[(0x0025_ac50, lobby_man_update, "LobbyMan_Update")];

pub(crate) mod callee {
    use super::*;
    /// `LobbyApi_Update(api)` (through the function table).
    pub const LOBBY_API_UPDATE: u32 = 0x003e_95c8;
    /// `LobbyApi_GetEvent(api)` (through the function table).
    pub const LOBBY_API_GET_EVENT: u32 = 0x003e_8fb8;
    /// `LobbyApi_Connect(mgr, addr, port, callback, self)` (function table).
    pub const LOBBY_API_CONNECT: u32 = 0x003e_8260;
    /// `LobbyApi_GetRecord(api, tag)` (through the function table).
    pub const LOBBY_API_GET_RECORD: u32 = 0x003e_8a70;
    /// `cBXString::cBXString(char*)` (through the function table).
    pub const BX_STRING_CTOR_CSTR: u32 = 0x0031_82d8;
    /// `EA_MemAllocNamed(size, name, flags, a3)` (through the function table).
    pub const EA_MEM_ALLOC_NAMED: u32 = 0x0031_7d70;
    /// `__builtin_delete(ptr)` (through the function table).
    pub const BUILTIN_DELETE: u32 = 0x0031_7e50;
    /// `BuddyMan_HasChallenger(mgr, name)` (through the function table).
    pub const BUDDY_MAN_HAS_CHALLENGER: u32 = 0x0026_3128;
    /// `BuddyMan_RemoveChallenger(mgr, name)` (function table).
    pub const BUDDY_MAN_REMOVE_CHALLENGER: u32 = 0x0026_3018;
    /// `Net_AbortGameSetup(setup, a1)` (through the function table).
    pub const NET_ABORT_GAME_SETUP: u32 = 0x0026_3ba8;
    /// `Lobby_ListSort(list)` (through the function table).
    pub const LOBBY_LIST_SORT: u32 = 0x003e_e620;
    /// `LobbyMenu_LobbyEventHook(self, msg)` (through the function table).
    pub const LOBBY_MENU_LOBBY_EVENT_HOOK: u32 = 0x0017_d260;
    pub const LOBBY_MAN_CREATE_PEER_TABLE: Callee = Callee {
        addr: 0x0025_a650,
        f: ee::s06::f_0025a650,
    };
    pub const LOBBY_MAN_DISCONNECT: Callee = Callee {
        addr: 0x0025_b800,
        f: ee::s06::f_0025b800,
    };
    pub const LOBBY_MAN_CANCEL_CHALLENGE: Callee = Callee {
        addr: 0x0025_f628,
        f: ee::s06::f_0025f628,
    };
    pub const LOBBY_MAN_CANCEL_QUICK_PLAY: Callee = Callee {
        addr: 0x0025_eed8,
        f: ee::s06::f_0025eed8,
    };
    pub const BUDDY_MAN_DISCONNECT: Callee = Callee {
        addr: 0x0026_1408,
        f: ee::s06::f_00261408,
    };
    pub const BUDDY_MAN_IS_CONNECTED: Callee = Callee {
        addr: 0x0026_1460,
        f: ee::s06::f_00261460,
    };
    pub const BUDDY_MAN_CREATE: Callee = Callee {
        addr: 0x0026_0f80,
        f: ee::s06::f_00260f80,
    };
    pub const BUDDY_MAN_RECONNECT: Callee = Callee {
        addr: 0x0026_13c8,
        f: ee::s06::f_002613c8,
    };
    pub const BUDDY_MAN_UPDATE: Callee = Callee {
        addr: 0x0026_1490,
        f: ee::s06::f_00261490,
    };
    pub const NET_GET_GAME_SETUP: Callee = Callee {
        addr: 0x0025_91d0,
        f: ee::s06::f_002591d0,
    };
    pub const C_COMM_SYSTEM_HAS_CHANNEL: Callee = Callee {
        addr: 0x0025_5d38,
        f: ee::s06::f_00255d38,
    };
    pub const C_COMM_SYSTEM_CLOSE_CHANNEL: Callee = Callee {
        addr: 0x0025_5cc8,
        f: ee::s06::f_00255cc8,
    };
}

// LobbyMan field offsets (from the generated body / reference dossier).
pub(crate) const LOBBY_MAN_UPDATE_STATE: u32 = 0x00;
pub(crate) const LOBBY_MAN_UPDATE_ADDR: u32 = 0x04;
pub(crate) const LOBBY_MAN_UPDATE_FLAG_C: u32 = 0x0c;
pub(crate) const LOBBY_MAN_UPDATE_FLAG_10: u32 = 0x10;
pub(crate) const LOBBY_MAN_UPDATE_ACTIVE_44: u32 = 0x44;
pub(crate) const LOBBY_MAN_UPDATE_API: u32 = 0x54;
pub(crate) const LOBBY_MAN_UPDATE_CONN: u32 = 0x58;
pub(crate) const LOBBY_MAN_UPDATE_LIST_5C: u32 = 0x5c;
pub(crate) const LOBBY_MAN_UPDATE_LIST_60: u32 = 0x60;
pub(crate) const LOBBY_MAN_UPDATE_NAME_98: u32 = 0x98;
pub(crate) const LOBBY_MAN_UPDATE_TIMER: u32 = 0xe0;
pub(crate) const LOBBY_MAN_UPDATE_TIMER2: u32 = 0xe4;
pub(crate) const LOBBY_MAN_UPDATE_LIST: u32 = 0xec;
pub(crate) const LOBBY_MAN_UPDATE_SENTINEL: u32 = 0xf0;

/// Shared tail of the two message posts that jump to `0x25b034`: write the
/// `{tag, D_004812B0}` message fields from `v1`/`s0`, allocate the 12-byte
/// list node and link it into the list whose sentinel is in `a2` (saved at
/// `sp+0x10`). Entry registers are exactly the generated ones at `0x25b034`.
fn lobby_man_update_post_tail(c: &mut Cpu) -> crate::call::Flow {
    c.cycles += 6;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x12b0));
    c.sw(r32(c, S0), c.g[V1][0]);
    c.sw(r32(c, S0).wrapping_add(4), c.g[V0][0]);
    c.g[A1][0] = 0x480000;
    c.sw(sp(c).wrapping_add(0x10), c.g[A2][0]);
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4a8));
    lobby_man_update_post_node(c)
}

/// Allocate the 12-byte list node (`0x25b06c`) and link the message in
/// `s0` into the list whose sentinel was saved at `sp+0x10`.
fn lobby_man_update_post_node(c: &mut Cpu) -> crate::call::Flow {
    c.cycles += 8;
    c.g[A0][0] = 0xc;
    c.g[A2][0] = 0x20000000;
    c.g[A3][0] = 0u64;
    call_ptr(c, callee::EA_MEM_ALLOC_NAMED, 0x0025_b05c)?;
    c.g[A1][0] = c.g[V0][0];
    c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(8));
    if c.g[V0][0] != 0u64 {
        c.sw((c.g[A1][0] as u32).wrapping_add(8), c.g[S0][0]);
    }
    c.cycles += 10;
    c.g[V0][0] = c.lw(sp(c).wrapping_add(0x10));
    c.sw(c.g[A1][0] as u32, c.g[V0][0]);
    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(4));
    c.sw((c.g[A1][0] as u32).wrapping_add(4), c.g[V1][0]);
    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(4));
    c.sw(c.g[A0][0] as u32, c.g[A1][0]);
    c.g[V0][0] = c.lw(sp(c).wrapping_add(0x10));
    c.sw((c.g[V0][0] as u32).wrapping_add(4), c.g[A1][0]);
    c.sw(sp(c), c.g[A1][0]);
    Ok(())
}

/// `void LobbyMan_Update(LobbyMan* self)` (0x25ac50): per-frame lobby
/// driver. When the lobby API exists it pumps it, takes its pending event,
/// refreshes the peer table, then runs the connection state machine in
/// `self[0]` (connect poll, session timeouts, buddy/challenge expiry),
/// posting reply messages onto the list at `self+0xec`; finally it updates
/// the buddy manager and drains that message list to the UI hook.
pub fn lobby_man_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x80);
        frame.save(c, S2, 0x50);
        frame.save(c, S0, 0x70);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S1, 0x60);
        frame.save(c, S3, 0x40);
        frame.save(c, S4, 0x30);
        frame.save_ra(c, 0x20);
        c.g[A0][0] = c.lw(r32(c, S2).wrapping_add(LOBBY_MAN_UPDATE_API));
        if c.g[A0][0] != 0 {
            'b_25b4b8: {
                'b_25b4b4: {
                    'b_25b49c: {
                        'b_25b498: {
                            c.cycles += 10;
                            call_ptr(c, callee::LOBBY_API_UPDATE, 0x0025_ac84)?;
                            c.g[A0][0] = c.lw(r32(c, S2).wrapping_add(LOBBY_MAN_UPDATE_API));
                            call_ptr(c, callee::LOBBY_API_GET_EVENT, 0x0025_ac8c)?;
                            c.g[S0][0] = c.g[V0][0];
                            c.g[A0][0] = c.g[S2][0];
                            callee::LOBBY_MAN_CREATE_PEER_TABLE.call(c, 0x0025_ac98)?;
                            c.g[V0][0] = c.lw(r32(c, S2).wrapping_add(LOBBY_MAN_UPDATE_FLAG_10));
                            if c.g[V0][0] != 0 {
                                c.cycles += 14;
                                c.sw(r32(c, S2).wrapping_add(LOBBY_MAN_UPDATE_FLAG_10), 0u64);
                                c.g[A0][0] = c.g[S2][0];
                                callee::LOBBY_MAN_DISCONNECT.call(c, 0x0025_acb0)?;
                                c.g[A1][0] = 0x480000;
                                c.g[A2][0] = 0x20000000;
                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x488));
                                c.g[A0][0] = 0x8;
                                c.g[A3][0] = 0u64;
                                call_ptr(c, callee::EA_MEM_ALLOC_NAMED, 0x0025_acc8)?;
                                c.g[S0][0] = c.g[V0][0];
                                c.g[A2][0] =
                                    c.lw(r32(c, S2).wrapping_add(LOBBY_MAN_UPDATE_SENTINEL));
                                c.g[V0][0] = 0x480000;
                                c.g[V1][0] = 0x115;
                                lobby_man_update_post_tail(c)?;
                                break 'b_25b4b4;
                            }
                            c.g[V1][0] = c.lw(r32(c, S2).wrapping_add(LOBBY_MAN_UPDATE_STATE));
                            // State dispatch: each failed check charges as generated.
                            c.cycles += 3;
                            c.g[V0][0] = 0x2;
                            let lobby_man_update_not2 = c.g[V1][0] != c.g[V0][0];
                            c.g[V0][0] = 0x3;
                            if !lobby_man_update_not2 {
                                // State 2: poll the connection callback.
                                c.cycles += 7;
                                c.g[A0][0] = c.lw(r32(c, S2).wrapping_add(LOBBY_MAN_UPDATE_CONN));
                                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
                                let lobby_man_update_poll = c.g[V0][0] as u32;
                                call_ptr(c, lobby_man_update_poll, 0x0025_acf8)?;
                                c.g[S1][0] = c.g[V0][0];
                                let lobby_man_update_le0 = (c.g[S1][0] as i64) <= 0;
                                c.g[T1][0] = 0x3;
                                if !lobby_man_update_le0 {
                                    c.cycles += 17;
                                    c.g[V1][0] =
                                        c.lw(r32(c, S2).wrapping_add(LOBBY_MAN_UPDATE_CONN));
                                    c.g[AT][0] = 0x41f00000;
                                    c.mtc1(1, 0);
                                    c.g[A3][0] = 0x270000;
                                    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                                    c.g[A3][0] = sx((c.g[A3][0] as u32).wrapping_sub(0x7458));
                                    c.g[A0][0] =
                                        c.lw(r32(c, S2).wrapping_add(LOBBY_MAN_UPDATE_API));
                                    c.g[A2][0] = 0x2af8;
                                    c.sw(
                                        r32(c, S2).wrapping_add(LOBBY_MAN_UPDATE_ADDR),
                                        c.g[V0][0],
                                    );
                                    c.g[A1][0] = c.g[V0][0];
                                    c.sw((c.g[GP][0] as u32).wrapping_sub(0xd8), c.g[V0][0]);
                                    c.g[T0][0] = c.g[S2][0];
                                    c.sw(
                                        r32(c, S2).wrapping_add(LOBBY_MAN_UPDATE_STATE),
                                        c.g[T1][0],
                                    );
                                    c.swc1(0, r32(c, S2).wrapping_add(LOBBY_MAN_UPDATE_TIMER));
                                    call_ptr(c, callee::LOBBY_API_CONNECT, 0x0025_ad40)?;
                                } else {
                                    c.cycles += 2;
                                    let lobby_man_update_ge0 = (c.g[S1][0] as i64) >= 0;
                                    if !lobby_man_update_ge0 {
                                        c.cycles += 29;
                                        c.g[A0][0] = c.g[S2][0];
                                        callee::LOBBY_MAN_DISCONNECT.call(c, 0x0025_ad58)?;
                                        c.g[A1][0] = 0x480000;
                                        c.g[A2][0] = 0x20000000;
                                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x488));
                                        c.g[A3][0] = 0u64;
                                        c.g[A0][0] = 0xc;
                                        call_ptr(c, callee::EA_MEM_ALLOC_NAMED, 0x0025_ad70)?;
                                        c.g[S0][0] = c.g[V0][0];
                                        c.g[A2][0] = 0x646e0000;
                                        c.g[V0][0] = 0x480000;
                                        c.g[A3][0] = c
                                            .lw(r32(c, S2).wrapping_add(LOBBY_MAN_UPDATE_SENTINEL));
                                        c.g[A2][0] = c.g[A2][0] | 0x7366;
                                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1290));
                                        c.g[V1][0] = 0x102;
                                        c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[A2][0]);
                                        c.sw(r32(c, S0), c.g[V1][0]);
                                        c.g[A1][0] = 0x480000;
                                        c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4a8));
                                        c.sw(sp(c).wrapping_add(0x10), c.g[A3][0]);
                                        c.g[A0][0] = 0xc;
                                        c.g[A2][0] = 0x20000000;
                                        c.g[A3][0] = 0u64;
                                        call_ptr(c, callee::EA_MEM_ALLOC_NAMED, 0x0025_adb4)?;
                                        c.g[A1][0] = c.g[V0][0];
                                        c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x8));
                                        if c.g[V0][0] != 0u64 {
                                            c.sw((c.g[A1][0] as u32).wrapping_add(0x8), c.g[S0][0]);
                                        }
                                        c.cycles += 9;
                                        c.g[V0][0] = c.lw(sp(c).wrapping_add(0x10));
                                        c.sw(c.g[A1][0] as u32, c.g[V0][0]);
                                        c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
                                        c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[V1][0]);
                                        c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
                                        c.sw(c.g[A0][0] as u32, c.g[A1][0]);
                                        c.g[V0][0] = c.lw(sp(c).wrapping_add(0x10));
                                        c.sw((c.g[V0][0] as u32).wrapping_add(0x4), c.g[A1][0]);
                                        c.sw(sp(c), c.g[A1][0]);
                                    }
                                }
                                c.cycles += 2;
                                if c.g[S1][0] == 0u64 {
                                    c.g[V0][0] =
                                        c.lw(r32(c, S2).wrapping_add(LOBBY_MAN_UPDATE_STATE));
                                    break 'b_25b4b8;
                                }
                                c.cycles += 2;
                                c.g[A0][0] = c.lw(r32(c, S2).wrapping_add(LOBBY_MAN_UPDATE_CONN));
                                break 'b_25b49c;
                            }
                            c.cycles += 2;
                            let lobby_man_update_not3 = c.g[V1][0] != c.g[V0][0];
                            c.g[V0][0] = 0x1a;
                            if !lobby_man_update_not3 {
                                // State 3: session established.
                                c.cycles += 3;
                                c.g[V0][0] = 0x400;
                                if c.g[S0][0] != c.g[V0][0] {
                                    c.lwc1(1, r32(c, S2).wrapping_add(LOBBY_MAN_UPDATE_TIMER));
                                    c.cycles += 5;
                                    c.mtc1(0, 0);
                                    c.c_lt_s(1, 0);
                                    let lobby_man_update_expired3 = c.fcc();
                                    c.g[V0][0] = 0x1;
                                    if !lobby_man_update_expired3 {
                                        c.cycles += 1;
                                        c.g[V0][0] = 0u64;
                                    }
                                    c.cycles += 2;
                                    if c.g[V0][0] == 0u64 {
                                        c.g[V0][0] =
                                            c.lw(r32(c, S2).wrapping_add(LOBBY_MAN_UPDATE_STATE));
                                        break 'b_25b4b8;
                                    }
                                    // Expired: disconnect, post "time", close.
                                    c.cycles += 29;
                                    c.g[A0][0] = c.g[S2][0];
                                    callee::LOBBY_MAN_DISCONNECT.call(c, 0x0025_aef4)?;
                                    c.g[A1][0] = 0x480000;
                                    c.g[A2][0] = 0x20000000;
                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x488));
                                    c.g[A3][0] = 0u64;
                                    c.g[A0][0] = 0xc;
                                    call_ptr(c, callee::EA_MEM_ALLOC_NAMED, 0x0025_af0c)?;
                                    c.g[S0][0] = c.g[V0][0];
                                    c.g[A2][0] = 0x74690000;
                                    c.g[V0][0] = 0x480000;
                                    c.g[A3][0] =
                                        c.lw(r32(c, S2).wrapping_add(LOBBY_MAN_UPDATE_SENTINEL));
                                    c.g[A2][0] = c.g[A2][0] | 0x6d65;
                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1290));
                                    c.g[V1][0] = 0x102;
                                    c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[A2][0]);
                                    c.sw(r32(c, S0), c.g[V1][0]);
                                    c.g[A1][0] = 0x480000;
                                    c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4a8));
                                    c.sw(sp(c).wrapping_add(0x10), c.g[A3][0]);
                                    c.g[A0][0] = 0xc;
                                    c.g[A2][0] = 0x20000000;
                                    c.g[A3][0] = 0u64;
                                    call_ptr(c, callee::EA_MEM_ALLOC_NAMED, 0x0025_af50)?;
                                    c.g[A1][0] = c.g[V0][0];
                                    c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x8));
                                    if c.g[V0][0] != 0u64 {
                                        c.sw((c.g[A1][0] as u32).wrapping_add(0x8), c.g[S0][0]);
                                    }
                                    c.cycles += 10;
                                    c.g[V0][0] = c.lw(sp(c).wrapping_add(0x10));
                                    c.sw(c.g[A1][0] as u32, c.g[V0][0]);
                                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
                                    c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[V1][0]);
                                    c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
                                    c.sw(c.g[A0][0] as u32, c.g[A1][0]);
                                    c.g[V0][0] = c.lw(sp(c).wrapping_add(0x10));
                                    c.sw((c.g[V0][0] as u32).wrapping_add(0x4), c.g[A1][0]);
                                    c.sw(sp(c), c.g[A1][0]);
                                    break 'b_25b498;
                                }
                                // Event 0x400: post the "down" record, disconnect.
                                c.cycles += 35;
                                c.g[A0][0] = c.lw(r32(c, S2).wrapping_add(LOBBY_MAN_UPDATE_API));
                                c.g[A1][0] = 0x646f0000;
                                c.g[A1][0] = c.g[A1][0] | 0x776e;
                                call_ptr(c, callee::LOBBY_API_GET_RECORD, 0x0025_ae1c)?;
                                c.g[A1][0] = 0x480000;
                                c.g[A2][0] = 0x20000000;
                                c.g[A3][0] = 0u64;
                                c.g[A0][0] = 0x10;
                                c.g[S0][0] = c.g[V0][0];
                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x488));
                                call_ptr(c, callee::EA_MEM_ALLOC_NAMED, 0x0025_ae38)?;
                                c.g[S1][0] = c.g[V0][0];
                                c.g[A0][0] = 0x102;
                                c.g[V0][0] = 0x646f0000;
                                c.g[V1][0] = 0x480000;
                                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x11b0));
                                c.g[V0][0] = c.g[V0][0] | 0x776e;
                                c.sw(r32(c, S1), c.g[A0][0]);
                                c.g[A1][0] = c.g[S0][0];
                                c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0xc));
                                c.sw((c.g[S1][0] as u32).wrapping_add(0x4), c.g[V1][0]);
                                c.sw((c.g[S1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
                                call_ptr(c, callee::BX_STRING_CTOR_CSTR, 0x0025_ae68)?;
                                c.g[V1][0] =
                                    c.lw(r32(c, S2).wrapping_add(LOBBY_MAN_UPDATE_SENTINEL));
                                c.g[A1][0] = 0x480000;
                                c.g[A2][0] = 0x20000000;
                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4a8));
                                c.g[A0][0] = 0xc;
                                c.sw(sp(c).wrapping_add(0x10), c.g[V1][0]);
                                c.g[A3][0] = 0u64;
                                call_ptr(c, callee::EA_MEM_ALLOC_NAMED, 0x0025_ae88)?;
                                c.g[A2][0] = c.g[V0][0];
                                c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(0x8));
                                if c.g[V0][0] != 0u64 {
                                    c.sw((c.g[A2][0] as u32).wrapping_add(0x8), c.g[S1][0]);
                                }
                                c.cycles += 13;
                                c.g[A1][0] = c.lw(sp(c).wrapping_add(0x10));
                                c.g[A0][0] = c.g[S2][0];
                                c.sw(c.g[A2][0] as u32, c.g[A1][0]);
                                c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4));
                                c.sw((c.g[A2][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                                c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4));
                                c.sw(c.g[V1][0] as u32, c.g[A2][0]);
                                c.g[V0][0] = c.lw(sp(c).wrapping_add(0x10));
                                c.sw((c.g[V0][0] as u32).wrapping_add(0x4), c.g[A2][0]);
                                c.sw(sp(c), c.g[A2][0]);
                                callee::LOBBY_MAN_DISCONNECT.call(c, 0x0025_aec4)?;
                                c.g[V0][0] = c.lw(r32(c, S2).wrapping_add(LOBBY_MAN_UPDATE_STATE));
                                break 'b_25b4b8;
                            }
                            c.cycles += 2;
                            let lobby_man_update_not1a = c.g[V1][0] != c.g[V0][0];
                            c.g[V0][0] = 0x16;
                            if !lobby_man_update_not1a {
                                // State 0x1a: buddy session ended / reconnect.
                                c.cycles += 4;
                                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xd0));
                                if c.g[V0][0] != 0u64 {
                                    c.cycles += 22;
                                    callee::BUDDY_MAN_DISCONNECT.call(c, 0x0025_afa8)?;
                                    c.g[A1][0] = 0x480000;
                                    c.g[A2][0] = 0x20000000;
                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x488));
                                    c.g[A3][0] = 0u64;
                                    c.g[A0][0] = 0xc;
                                    call_ptr(c, callee::EA_MEM_ALLOC_NAMED, 0x0025_afc0)?;
                                    c.g[V1][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                                    c.g[S0][0] = c.g[V0][0];
                                    c.g[V0][0] = 0x480000;
                                    c.g[A3][0] =
                                        c.lw(r32(c, S2).wrapping_add(LOBBY_MAN_UPDATE_SENTINEL));
                                    c.g[A2][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0xd0));
                                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1270));
                                    c.g[V1][0] = 0x103;
                                    c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                                    c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[A2][0]);
                                    c.g[A1][0] = 0x480000;
                                    c.sw(r32(c, S0), c.g[V1][0]);
                                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4a8));
                                    c.sw(sp(c).wrapping_add(0x10), c.g[A3][0]);
                                    lobby_man_update_post_node(c)?;
                                    break 'b_25b4b4;
                                }
                                c.cycles += 4;
                                callee::BUDDY_MAN_IS_CONNECTED.call(c, 0x0025_b000)?;
                                let lobby_man_update_buddy_down = c.g[V0][0] == 0u64;
                                c.g[V0][0] = 0x1;
                                if lobby_man_update_buddy_down {
                                    break 'b_25b4b4;
                                }
                                c.cycles += 11;
                                c.g[A1][0] = 0x480000;
                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x488));
                                c.g[A2][0] = 0x20000000;
                                c.g[A0][0] = 0x8;
                                c.g[A3][0] = 0u64;
                                c.sw(r32(c, S2).wrapping_add(LOBBY_MAN_UPDATE_STATE), c.g[V0][0]);
                                call_ptr(c, callee::EA_MEM_ALLOC_NAMED, 0x0025_b024)?;
                                c.g[S0][0] = c.g[V0][0];
                                c.g[A2][0] =
                                    c.lw(r32(c, S2).wrapping_add(LOBBY_MAN_UPDATE_SENTINEL));
                                c.g[V0][0] = 0x480000;
                                c.g[V1][0] = 0xda;
                                lobby_man_update_post_tail(c)?;
                                break 'b_25b4b4;
                            }
                            c.cycles += 2;
                            let lobby_man_update_is16 = c.g[V1][0] == c.g[V0][0];
                            c.g[V0][0] = 0x17;
                            if lobby_man_update_is16 {
                                break 'b_25b4b4;
                            }
                            c.cycles += 2;
                            let lobby_man_update_not17 = c.g[V1][0] != c.g[V0][0];
                            if !lobby_man_update_not17 {
                                // State 0x17: challenge expiry.
                                c.cycles += 9;
                                c.lwc1(0, (c.g[GP][0] as u32).wrapping_add(0x2064));
                                c.g[AT][0] = 0x3f800000;
                                c.mtc1(1, 1);
                                c.lwc1(2, r32(c, S2).wrapping_add(LOBBY_MAN_UPDATE_TIMER));
                                c.sub_s(0, 0, 1);
                                c.c_lt_s(2, 0);
                                if c.fcc() {
                                    c.swc1(2, (c.g[GP][0] as u32).wrapping_add(0x2064));
                                }
                                c.cycles += 5;
                                c.mtc1(0, 0);
                                c.c_lt_s(2, 0);
                                let lobby_man_update_neg17 = c.fcc();
                                c.g[V0][0] = 0x1;
                                if !lobby_man_update_neg17 {
                                    c.cycles += 1;
                                    c.g[V0][0] = 0u64;
                                }
                                c.cycles += 2;
                                let lobby_man_update_notneg17 = c.g[V0][0] == 0u64;
                                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                                if lobby_man_update_notneg17 {
                                    break 'b_25b4b4;
                                }
                                // Mgr_AC50(): create/reconnect the buddy manager.
                                c.cycles += 2;
                                if c.g[V0][0] == 0u64 {
                                    c.cycles += 2;
                                    callee::BUDDY_MAN_CREATE.call(c, 0x0025_b0f8)?;
                                }
                                c.cycles += 4;
                                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                                callee::BUDDY_MAN_IS_CONNECTED.call(c, 0x0025_b100)?;
                                let lobby_man_update_mgr_down = c.g[V0][0] == 0u64;
                                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                                if lobby_man_update_mgr_down {
                                    c.cycles += 3;
                                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                                    callee::BUDDY_MAN_RECONNECT.call(c, 0x0025_b110)?;
                                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                                }
                                c.cycles += 2;
                                let lobby_man_update_mgr_live = c.g[V0][0] != 0u64;
                                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                                if !lobby_man_update_mgr_live {
                                    c.cycles += 3;
                                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                                    callee::LOBBY_MAN_DISCONNECT.call(c, 0x0025_b124)?;
                                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                                }
                                c.cycles += 4;
                                c.g[A1][0] =
                                    c.lw(r32(c, S2).wrapping_add(LOBBY_MAN_UPDATE_NAME_98));
                                call_ptr(c, callee::BUDDY_MAN_HAS_CHALLENGER, 0x0025_b130)?;
                                let lobby_man_update_no_chal = c.g[V0][0] == 0u64;
                                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                                if !lobby_man_update_no_chal {
                                    c.cycles += 2;
                                    if c.g[V0][0] == 0u64 {
                                        c.cycles += 2;
                                        callee::BUDDY_MAN_CREATE.call(c, 0x0025_b148)?;
                                    }
                                    c.cycles += 4;
                                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                                    callee::BUDDY_MAN_IS_CONNECTED.call(c, 0x0025_b150)?;
                                    let lobby_man_update_mgr2_down = c.g[V0][0] == 0u64;
                                    c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                                    if lobby_man_update_mgr2_down {
                                        c.cycles += 3;
                                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                                        callee::BUDDY_MAN_RECONNECT.call(c, 0x0025_b160)?;
                                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                                    }
                                    c.cycles += 2;
                                    let lobby_man_update_mgr2_live = c.g[V0][0] != 0u64;
                                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                                    if !lobby_man_update_mgr2_live {
                                        c.cycles += 3;
                                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0xc8));
                                        callee::LOBBY_MAN_DISCONNECT.call(c, 0x0025_b174)?;
                                        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
                                    }
                                    c.cycles += 2;
                                    c.g[A1][0] =
                                        c.lw(r32(c, S2).wrapping_add(LOBBY_MAN_UPDATE_NAME_98));
                                    call_ptr(c, callee::BUDDY_MAN_REMOVE_CHALLENGER, 0x0025_b180)?;
                                }
                                // Post 0x112 and go to state 1.
                                c.cycles += 24;
                                c.g[A1][0] = 0x480000;
                                c.g[A0][0] = 0x8;
                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x488));
                                c.g[A2][0] = 0x20000000;
                                c.g[A3][0] = 0u64;
                                c.g[S0][0] = 0x112;
                                call_ptr(c, callee::EA_MEM_ALLOC_NAMED, 0x0025_b19c)?;
                                c.g[S1][0] = c.g[V0][0];
                                c.g[V1][0] =
                                    c.lw(r32(c, S2).wrapping_add(LOBBY_MAN_UPDATE_SENTINEL));
                                c.g[V0][0] = 0x480000;
                                c.sw(r32(c, S1), c.g[S0][0]);
                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x12b0));
                                c.g[A1][0] = 0x480000;
                                c.sw((c.g[S1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                                c.g[A2][0] = 0x20000000;
                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4a8));
                                c.sw(sp(c).wrapping_add(0x10), c.g[V1][0]);
                                c.g[A0][0] = 0xc;
                                c.g[A3][0] = 0u64;
                                call_ptr(c, callee::EA_MEM_ALLOC_NAMED, 0x0025_b1d0)?;
                                c.g[A2][0] = c.g[V0][0];
                                c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(0x8));
                                if c.g[V0][0] != 0u64 {
                                    c.sw((c.g[A2][0] as u32).wrapping_add(0x8), c.g[S1][0]);
                                }
                                c.cycles += 12;
                                c.g[A0][0] = c.lw(sp(c).wrapping_add(0x10));
                                c.g[A1][0] = 0x1;
                                c.sw(c.g[A2][0] as u32, c.g[A0][0]);
                                c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
                                c.sw((c.g[A2][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                                c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
                                c.sw(c.g[V1][0] as u32, c.g[A2][0]);
                                c.g[V0][0] = c.lw(sp(c).wrapping_add(0x10));
                                c.sw((c.g[V0][0] as u32).wrapping_add(0x4), c.g[A2][0]);
                                c.sw(sp(c), c.g[A2][0]);
                                c.sw(r32(c, S2).wrapping_add(LOBBY_MAN_UPDATE_STATE), c.g[A1][0]);
                                break 'b_25b4b4;
                            }
                            c.g[V0][0] = 0x18;
                            c.cycles += 2;
                            let lobby_man_update_not18 = c.g[V1][0] != c.g[V0][0];
                            if !lobby_man_update_not18 {
                                // State 0x18: challenge expiry, cancel on timeout.
                                c.cycles += 9;
                                c.lwc1(0, (c.g[GP][0] as u32).wrapping_add(0x2064));
                                c.g[AT][0] = 0x3f800000;
                                c.mtc1(1, 1);
                                c.lwc1(2, r32(c, S2).wrapping_add(LOBBY_MAN_UPDATE_TIMER));
                                c.sub_s(0, 0, 1);
                                c.c_lt_s(2, 0);
                                if c.fcc() {
                                    c.swc1(2, (c.g[GP][0] as u32).wrapping_add(0x2064));
                                }
                                c.cycles += 5;
                                c.mtc1(0, 0);
                                c.c_lt_s(2, 0);
                                let lobby_man_update_neg18 = c.fcc();
                                c.g[V0][0] = 0x1;
                                if !lobby_man_update_neg18 {
                                    c.cycles += 1;
                                    c.g[V0][0] = 0u64;
                                }
                                c.cycles += 2;
                                let lobby_man_update_notneg18 = c.g[V0][0] == 0u64;
                                c.g[A1][0] = 0x480000;
                                if lobby_man_update_notneg18 {
                                    break 'b_25b4b4;
                                }
                                c.cycles += 23;
                                c.g[A0][0] = 0x8;
                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x488));
                                c.g[A2][0] = 0x20000000;
                                c.g[A3][0] = 0u64;
                                c.g[S0][0] = 0x112;
                                call_ptr(c, callee::EA_MEM_ALLOC_NAMED, 0x0025_b274)?;
                                c.g[S1][0] = c.g[V0][0];
                                c.g[V1][0] =
                                    c.lw(r32(c, S2).wrapping_add(LOBBY_MAN_UPDATE_SENTINEL));
                                c.g[V0][0] = 0x480000;
                                c.sw(r32(c, S1), c.g[S0][0]);
                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x12b0));
                                c.g[A1][0] = 0x480000;
                                c.sw((c.g[S1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4a8));
                                c.sw(sp(c).wrapping_add(0x10), c.g[V1][0]);
                                c.g[A0][0] = 0xc;
                                c.g[A2][0] = 0x20000000;
                                c.g[A3][0] = 0u64;
                                call_ptr(c, callee::EA_MEM_ALLOC_NAMED, 0x0025_b2a8)?;
                                c.g[T0][0] = c.g[V0][0];
                                c.g[V0][0] = sx((c.g[T0][0] as u32).wrapping_add(0x8));
                                if c.g[V0][0] != 0u64 {
                                    c.sw((c.g[T0][0] as u32).wrapping_add(0x8), c.g[S1][0]);
                                }
                                c.cycles += 15;
                                c.g[A3][0] = c.lw(sp(c).wrapping_add(0x10));
                                c.g[A0][0] = c.g[S2][0];
                                c.g[A1][0] = 0u64;
                                c.g[A2][0] = 0x1;
                                c.sw(c.g[T0][0] as u32, c.g[A3][0]);
                                c.g[V0][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
                                c.sw((c.g[T0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                                c.g[V1][0] = c.lw((c.g[A3][0] as u32).wrapping_add(0x4));
                                c.sw(c.g[V1][0] as u32, c.g[T0][0]);
                                c.g[V0][0] = c.lw(sp(c).wrapping_add(0x10));
                                c.sw((c.g[V0][0] as u32).wrapping_add(0x4), c.g[T0][0]);
                                c.sw(sp(c), c.g[T0][0]);
                                callee::LOBBY_MAN_CANCEL_CHALLENGE.call(c, 0x0025_b2ec)?;
                                c.g[V0][0] = c.lw(r32(c, S2).wrapping_add(LOBBY_MAN_UPDATE_STATE));
                                break 'b_25b4b8;
                            }
                            c.g[V0][0] = 0x14;
                            c.cycles += 2;
                            let lobby_man_update_not14 = c.g[V1][0] != c.g[V0][0];
                            c.g[V0][0] = 0x19;
                            if !lobby_man_update_not14 {
                                // State 0x14: quick-play expiry.
                                c.cycles += 6;
                                c.lwc1(1, r32(c, S2).wrapping_add(LOBBY_MAN_UPDATE_TIMER));
                                c.mtc1(0, 0);
                                c.c_lt_s(1, 0);
                                let lobby_man_update_neg14 = c.fcc();
                                c.g[V0][0] = 0x1;
                                if !lobby_man_update_neg14 {
                                    c.cycles += 1;
                                    c.g[V0][0] = 0u64;
                                }
                                c.cycles += 2;
                                let lobby_man_update_notneg14 = c.g[V0][0] == 0u64;
                                c.g[A1][0] = 0x480000;
                                if lobby_man_update_notneg14 {
                                    break 'b_25b4b4;
                                }
                                c.cycles += 23;
                                c.g[A2][0] = 0x20000000;
                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x488));
                                c.g[A0][0] = 0x8;
                                c.g[A3][0] = 0u64;
                                call_ptr(c, callee::EA_MEM_ALLOC_NAMED, 0x0025_b334)?;
                                c.g[S0][0] = c.g[V0][0];
                                c.g[A2][0] =
                                    c.lw(r32(c, S2).wrapping_add(LOBBY_MAN_UPDATE_SENTINEL));
                                c.g[V0][0] = 0x480000;
                                c.g[V1][0] = 0xe6;
                                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x12b0));
                                c.sw(r32(c, S0), c.g[V1][0]);
                                c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                                c.g[A1][0] = 0x480000;
                                c.sw(sp(c).wrapping_add(0x10), c.g[A2][0]);
                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4a8));
                                c.g[A2][0] = 0x20000000;
                                c.g[A0][0] = 0xc;
                                c.g[A3][0] = 0u64;
                                call_ptr(c, callee::EA_MEM_ALLOC_NAMED, 0x0025_b36c)?;
                                c.g[A2][0] = c.g[V0][0];
                                c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_add(0x8));
                                if c.g[V0][0] != 0u64 {
                                    c.sw((c.g[A2][0] as u32).wrapping_add(0x8), c.g[S0][0]);
                                }
                                c.cycles += 13;
                                c.g[A1][0] = c.lw(sp(c).wrapping_add(0x10));
                                c.g[A0][0] = c.g[S2][0];
                                c.sw(c.g[A2][0] as u32, c.g[A1][0]);
                                c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4));
                                c.sw((c.g[A2][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                                c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4));
                                c.sw(c.g[V1][0] as u32, c.g[A2][0]);
                                c.g[V0][0] = c.lw(sp(c).wrapping_add(0x10));
                                c.sw((c.g[V0][0] as u32).wrapping_add(0x4), c.g[A2][0]);
                                c.sw(sp(c), c.g[A2][0]);
                                callee::LOBBY_MAN_CANCEL_QUICK_PLAY.call(c, 0x0025_b3a8)?;
                                c.g[V0][0] = c.lw(r32(c, S2).wrapping_add(LOBBY_MAN_UPDATE_STATE));
                                break 'b_25b4b8;
                            }
                            c.cycles += 2;
                            let lobby_man_update_is19 = c.g[V1][0] == c.g[V0][0];
                            c.g[V0][0] = c.g[V1][0] ^ 0x1;
                            if lobby_man_update_is19 {
                                break 'b_25b4b4;
                            }
                            // Default: idle/activity timeout.
                            c.cycles += 2;
                            let lobby_man_update_notidle = c.g[V0][0] != 0u64;
                            c.g[V1][0] = 0u64;
                            if !lobby_man_update_notidle {
                                c.cycles += 3;
                                c.g[V0][0] =
                                    c.lw(r32(c, S2).wrapping_add(LOBBY_MAN_UPDATE_ACTIVE_44));
                                if c.g[V0][0] != 0u64 {
                                    c.cycles += 1;
                                    c.g[V1][0] = 0x1;
                                }
                            } else {
                                c.cycles += 1;
                                c.g[V1][0] = 0x1;
                            }
                            c.cycles += 2;
                            if c.g[V1][0] == 0u64 {
                                c.g[V0][0] = c.lw(r32(c, S2).wrapping_add(LOBBY_MAN_UPDATE_STATE));
                                break 'b_25b4b8;
                            }
                            c.cycles += 6;
                            c.lwc1(1, r32(c, S2).wrapping_add(LOBBY_MAN_UPDATE_TIMER));
                            c.mtc1(0, 0);
                            c.c_lt_s(1, 0);
                            let lobby_man_update_negd = c.fcc();
                            c.g[V0][0] = 0x1;
                            if !lobby_man_update_negd {
                                c.cycles += 1;
                                c.g[V0][0] = 0u64;
                            }
                            c.cycles += 2;
                            if c.g[V0][0] == 0u64 {
                                c.g[V0][0] = c.lw(r32(c, S2).wrapping_add(LOBBY_MAN_UPDATE_STATE));
                                break 'b_25b4b8;
                            }
                            c.cycles += 29;
                            c.g[A0][0] = c.g[S2][0];
                            callee::LOBBY_MAN_DISCONNECT.call(c, 0x0025_b404)?;
                            c.g[A1][0] = 0x480000;
                            c.g[A2][0] = 0x20000000;
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x488));
                            c.g[A3][0] = 0u64;
                            c.g[A0][0] = 0xc;
                            call_ptr(c, callee::EA_MEM_ALLOC_NAMED, 0x0025_b41c)?;
                            c.g[S0][0] = c.g[V0][0];
                            c.g[A2][0] = 0x74690000;
                            c.g[V0][0] = 0x480000;
                            c.g[A3][0] = c.lw(r32(c, S2).wrapping_add(LOBBY_MAN_UPDATE_SENTINEL));
                            c.g[A2][0] = c.g[A2][0] | 0x6d65;
                            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1290));
                            c.g[V1][0] = 0x102;
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[A2][0]);
                            c.sw(r32(c, S0), c.g[V1][0]);
                            c.g[A1][0] = 0x480000;
                            c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4a8));
                            c.sw(sp(c).wrapping_add(0x10), c.g[A3][0]);
                            c.g[A0][0] = 0xc;
                            c.g[A2][0] = 0x20000000;
                            c.g[A3][0] = 0u64;
                            call_ptr(c, callee::EA_MEM_ALLOC_NAMED, 0x0025_b460)?;
                            c.g[A1][0] = c.g[V0][0];
                            c.g[V0][0] = sx((c.g[A1][0] as u32).wrapping_add(0x8));
                            if c.g[V0][0] != 0u64 {
                                c.sw((c.g[A1][0] as u32).wrapping_add(0x8), c.g[S0][0]);
                            }
                            c.cycles += 10;
                            c.g[V0][0] = c.lw(sp(c).wrapping_add(0x10));
                            c.sw(c.g[A1][0] as u32, c.g[V0][0]);
                            c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
                            c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[V1][0]);
                            c.g[A0][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x4));
                            c.sw(c.g[A0][0] as u32, c.g[A1][0]);
                            c.g[V0][0] = c.lw(sp(c).wrapping_add(0x10));
                            c.sw((c.g[V0][0] as u32).wrapping_add(0x4), c.g[A1][0]);
                            c.sw(sp(c), c.g[A1][0]);
                            c.sw(r32(c, S2).wrapping_add(LOBBY_MAN_UPDATE_ACTIVE_44), 0u64);
                            break 'b_25b498;
                        }
                        // 'b_25b498 target: shared closeConn entry (a0 set below).
                        c.cycles += 1;
                        c.g[A0][0] = c.lw(r32(c, S2).wrapping_add(LOBBY_MAN_UPDATE_CONN));
                    }
                    // 'b_25b49c target: close the connection callback if present.
                    c.cycles += 2;
                    if c.g[A0][0] == 0u64 {
                        c.g[V0][0] = c.lw(r32(c, S2).wrapping_add(LOBBY_MAN_UPDATE_STATE));
                        break 'b_25b4b8;
                    }
                    c.cycles += 4;
                    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0xc));
                    let lobby_man_update_close = c.g[V0][0] as u32;
                    call_ptr(c, lobby_man_update_close, 0x0025_b4b0)?;
                    c.sw(r32(c, S2).wrapping_add(LOBBY_MAN_UPDATE_CONN), 0u64);
                }
                // 'b_25b4b4 target.
                c.cycles += 1;
                c.g[V0][0] = c.lw(r32(c, S2).wrapping_add(LOBBY_MAN_UPDATE_STATE));
            }
            // 'b_25b4b8 target: not-joined (state != 0x19) housekeeping.
            c.cycles += 3;
            c.g[V0][0] = c.g[V0][0] ^ 0x19;
            if c.g[V0][0] != 0u64 {
                c.cycles += 5;
                callee::NET_GET_GAME_SETUP.call(c, 0x0025_b4cc)?;
                c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                let lobby_man_update_setup_live = c.g[V1][0] != 0u64;
                c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x238));
                if lobby_man_update_setup_live {
                    c.cycles += 7;
                    callee::NET_GET_GAME_SETUP.call(c, 0x0025_b4e0)?;
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A1][0] = 0u64;
                    call_ptr(c, callee::NET_ABORT_GAME_SETUP, 0x0025_b4ec)?;
                    c.g[V0][0] = c.lw(r32(c, S2).wrapping_add(LOBBY_MAN_UPDATE_FLAG_C));
                } else {
                    c.cycles += 4;
                    c.g[A1][0] = 0u64;
                    callee::C_COMM_SYSTEM_HAS_CHANNEL.call(c, 0x0025_b4fc)?;
                    let lobby_man_update_has = c.g[V0][0] != 0u64;
                    c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x238));
                    if lobby_man_update_has {
                        c.cycles += 2;
                        c.g[A1][0] = 0u64;
                        callee::C_COMM_SYSTEM_CLOSE_CHANNEL.call(c, 0x0025_b50c)?;
                    }
                    c.cycles += 1;
                    c.g[V0][0] = c.lw(r32(c, S2).wrapping_add(LOBBY_MAN_UPDATE_FLAG_C));
                }
            } else {
                c.g[V0][0] = c.lw(r32(c, S2).wrapping_add(LOBBY_MAN_UPDATE_FLAG_C));
            }
            c.cycles += 2;
            let lobby_man_update_no_sort = c.g[V0][0] == 0u64;
            c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            if !lobby_man_update_no_sort {
                c.cycles += 5;
                c.g[A0][0] = c.lw(r32(c, S2).wrapping_add(LOBBY_MAN_UPDATE_LIST_5C));
                call_ptr(c, callee::LOBBY_LIST_SORT, 0x0025_b520)?;
                c.g[A0][0] = c.lw(r32(c, S2).wrapping_add(LOBBY_MAN_UPDATE_LIST_60));
                call_ptr(c, callee::LOBBY_LIST_SORT, 0x0025_b528)?;
                c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
            }
            c.cycles += 8;
            c.lwc1(0, r32(c, S2).wrapping_add(LOBBY_MAN_UPDATE_TIMER));
            c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x14));
            c.lwc1(2, r32(c, S2).wrapping_add(LOBBY_MAN_UPDATE_TIMER2));
            c.sub_s(0, 0, 1);
            c.swc1(0, r32(c, S2).wrapping_add(LOBBY_MAN_UPDATE_TIMER));
            c.lwc1(1, (c.g[V0][0] as u32).wrapping_add(0x14));
            c.sub_s(2, 2, 1);
            c.swc1(2, r32(c, S2).wrapping_add(LOBBY_MAN_UPDATE_TIMER2));
        }
        // BuddyMan_Update and the message drain (also on the no-API path).
        c.cycles += 8;
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0x238));
        callee::BUDDY_MAN_UPDATE.call(c, 0x0025_b554)?;
        c.g[A0][0] = sx(r32(c, S2).wrapping_add(LOBBY_MAN_UPDATE_LIST));
        c.g[A1][0] = c.lw(r32(c, S2).wrapping_add(LOBBY_MAN_UPDATE_SENTINEL));
        c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
        c.g[V1][0] = c.lw(c.g[V0][0] as u32);
        let lobby_man_update_list_empty = c.g[V1][0] == c.g[A1][0];
        c.g[S4][0] = c.g[A0][0];
        if !lobby_man_update_list_empty {
            c.cycles += 3;
            c.g[S3][0] = c.g[S4][0];
            c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x4));
            'l_25b578: loop {
                c.cycles += 18;
                c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                c.sw(sp(c), c.g[V1][0]);
                c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x4));
                c.g[S1][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x8));
                c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                c.sw(sp(c).wrapping_add(0x10), c.g[V1][0]);
                c.g[S0][0] = c.lw(c.g[V1][0] as u32);
                c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x4));
                c.sw(c.g[V0][0] as u32, c.g[S0][0]);
                c.sw((c.g[S0][0] as u32).wrapping_add(0x4), c.g[V0][0]);
                c.g[A0][0] = c.lw(sp(c).wrapping_add(0x10));
                call_ptr(c, callee::BUILTIN_DELETE, 0x0025_b5a8)?;
                c.sw(sp(c), c.g[S0][0]);
                c.g[A0][0] = c.g[S2][0];
                c.g[A1][0] = c.g[S1][0];
                call_ptr(c, callee::LOBBY_MENU_LOBBY_EVENT_HOOK, 0x0025_b5b8)?;
                let lobby_man_update_msg_null = c.g[S1][0] == 0u64;
                c.g[A1][0] = 0x3;
                if !lobby_man_update_msg_null {
                    c.cycles += 5;
                    c.g[V0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x4));
                    c.g[A0][0] = c.lh((c.g[V0][0] as u32).wrapping_add(0x8));
                    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0xc));
                    let lobby_man_update_vfn = c.g[V1][0] as u32;
                    c.g[A0][0] = sx((r32(c, S1)).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, lobby_man_update_vfn, 0x0025_b5d4)?;
                }
                c.cycles += 5;
                c.g[V0][0] = c.lw((c.g[S4][0] as u32).wrapping_add(0x4));
                c.g[A0][0] = c.lw(r32(c, S2).wrapping_add(LOBBY_MAN_UPDATE_SENTINEL));
                c.g[V1][0] = c.lw(c.g[V0][0] as u32);
                if c.g[V1][0] == c.g[A0][0] {
                    break 'l_25b578;
                }
                c.g[V0][0] = c.lw((c.g[S3][0] as u32).wrapping_add(0x4));
                if event_due(c) {
                    deliver_event_here(c, 0x0025_b578);
                }
            }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::events::{check_events, driven, schedule};
    use crate::difftest::fakes::machine;
    use crate::difftest::{check, Rng, DATA, GAME_GP};
    use crate::game::fe::feasyncfile::tests::{allocator, be_library, heap_init, pool_allocator_3};

    driven!(gen_lobby_man_update, ee::s06::f_0025ac50__generated);
    driven!(hand_lobby_man_update, lobby_man_update);
    const LOBBY_MAN_UPDATE_SELF: u32 = DATA + 0x1000;
    const LOBBY_MAN_UPDATE_API_OBJ: u32 = DATA + 0x1800;
    const LOBBY_MAN_UPDATE_HEAD: u32 = DATA + 0x2000;
    const LOBBY_MAN_UPDATE_NODE1: u32 = DATA + 0x2100;
    const LOBBY_MAN_UPDATE_NODE2: u32 = DATA + 0x2180;
    const LOBBY_MAN_UPDATE_MSG1: u32 = DATA + 0x2200;
    const LOBBY_MAN_UPDATE_MSGVT: u32 = DATA + 0x2300;
    const LOBBY_MAN_UPDATE_CONN_OBJ: u32 = DATA + 0x2400;
    const LOBBY_MAN_UPDATE_BUDDY_OBJ: u32 = DATA + 0x2800;
    const LOBBY_MAN_UPDATE_COMM_OBJ: u32 = DATA + 0x2c00;
    const LOBBY_MAN_UPDATE_CHAN_OBJ: u32 = DATA + 0x2d00;
    const LOBBY_MAN_UPDATE_CHAN_VT: u32 = DATA + 0x2d80;
    const LOBBY_MAN_UPDATE_APP_OBJ: u32 = DATA + 0x3000;
    const LOBBY_MAN_UPDATE_CHAL: u32 = DATA + 0x3600;
    const LOBBY_MAN_UPDATE_NAME_OBJ: u32 = DATA + 0x3900;
    const LOBBY_MAN_UPDATE_SORT1: u32 = DATA + 0x3a00;
    const LOBBY_MAN_UPDATE_SORT2: u32 = DATA + 0x3b00;
    /// Real leaves: return 0 / return -1 (poll and vtable slots).
    const LOBBY_MAN_UPDATE_ZERO_FN: u32 = 0x0014_06c0;
    const LOBBY_MAN_UPDATE_NEG_FN: u32 = 0x0027_5300;
    /// `LobbyApi_GetEvent` as a poll fn: returns 8 from the field at +8.
    const LOBBY_MAN_UPDATE_POLL_FN: u32 = 0x003e_8fb8;
    /// Address `Net_GetGameSetup` computes (`[app+0x7c] + 0xb5abc`, with
    /// `[app+0x7c] = 0` in this fixture).
    const LOBBY_MAN_UPDATE_SETUP: u32 = 0x000b_5abc;

    fn lobby_man_update_put_str(c: &mut Cpu, at: u32, s: &[u8]) {
        for (i, b) in s.iter().enumerate() {
            c.bus.write8(at + i as u32, *b);
        }
    }

    fn lobby_man_update_set_drain(c: &mut Cpu, count: u32) {
        let head = LOBBY_MAN_UPDATE_HEAD;
        c.bus.write32(LOBBY_MAN_UPDATE_SELF + 0xf0, head);
        if count == 0 {
            c.bus.write32(head, head);
            c.bus.write32(head + 4, head);
            return;
        }
        let n1 = LOBBY_MAN_UPDATE_NODE1;
        c.bus.write32(head, n1);
        c.bus.write32(n1 + 4, head);
        c.bus.write32(n1 + 8, LOBBY_MAN_UPDATE_MSG1);
        c.bus
            .write32(LOBBY_MAN_UPDATE_MSG1 + 4, LOBBY_MAN_UPDATE_MSGVT);
        c.bus.write16(LOBBY_MAN_UPDATE_MSGVT + 8, 0);
        c.bus
            .write32(LOBBY_MAN_UPDATE_MSGVT + 12, LOBBY_MAN_UPDATE_ZERO_FN);
        if count == 1 {
            c.bus.write32(head + 4, n1);
            c.bus.write32(n1, head);
            return;
        }
        let n2 = LOBBY_MAN_UPDATE_NODE2;
        c.bus.write32(head + 4, n2);
        c.bus.write32(n1, n2);
        c.bus.write32(n2, head);
        c.bus.write32(n2 + 4, n1);
        c.bus.write32(n2 + 8, 0);
    }

    /// Branch steering is by `seed % 24` / `seed / 24` arithmetic only;
    /// the rng from `machine` only scrambles registers.
    fn lobby_man_update_fixture(c: &mut Cpu, seed: u32) -> Rng {
        let r = machine(c, seed, 0x25ac50, 0);
        be_library(c);
        pool_allocator_3(c);
        heap_init(c);
        allocator(c);
        c.pc = 0x0025_ac50;
        let scenario = seed % 24;
        let sub = (seed / 24) % 6;
        let self_ = LOBBY_MAN_UPDATE_SELF;
        c.g[A0][0] = sx(self_);
        // Globals: buddy manager, comm system, app, lobby singleton.
        c.bus.write32(GAME_GP + 0x238, LOBBY_MAN_UPDATE_BUDDY_OBJ);
        c.bus
            .write32(GAME_GP.wrapping_sub(0x238), LOBBY_MAN_UPDATE_COMM_OBJ);
        c.bus
            .write32(GAME_GP.wrapping_sub(0x848), LOBBY_MAN_UPDATE_APP_OBJ);
        c.bus.write32(GAME_GP.wrapping_sub(0xc8), self_);
        c.bus.write32(GAME_GP + 0x1da0, 0);
        c.bus.write32(GAME_GP + 0x2064, 0x40a0_0000);
        c.bus.write32(LOBBY_MAN_UPDATE_APP_OBJ + 0x14, 0x3f80_0000);
        c.bus.write32(LOBBY_MAN_UPDATE_APP_OBJ + 0x7c, 0);
        // Not-joined tail sub-branches by sub % 3.
        c.bus
            .write32(LOBBY_MAN_UPDATE_SETUP, if sub % 3 == 0 { 1 } else { 0 });
        c.bus.write32(LOBBY_MAN_UPDATE_SETUP + 4, 0);
        c.bus.write32(
            LOBBY_MAN_UPDATE_COMM_OBJ,
            if sub % 3 == 2 {
                LOBBY_MAN_UPDATE_CHAN_OBJ
            } else {
                0
            },
        );
        c.bus.write32(LOBBY_MAN_UPDATE_CHAN_OBJ, 0);
        c.bus
            .write32(LOBBY_MAN_UPDATE_CHAN_OBJ + 8, LOBBY_MAN_UPDATE_CHAN_VT);
        c.bus.write16(LOBBY_MAN_UPDATE_CHAN_VT + 8, 0);
        c.bus
            .write32(LOBBY_MAN_UPDATE_CHAN_VT + 12, LOBBY_MAN_UPDATE_ZERO_FN);
        // Buddy manager: Reconnect is a no-op (+0x40 < 0); challengers alice/bob.
        let buddy = LOBBY_MAN_UPDATE_BUDDY_OBJ;
        c.bus.write32(buddy + 0x40, 0xffff_ffff);
        c.bus.write32(buddy + 0xbc, LOBBY_MAN_UPDATE_CHAL);
        c.bus.write32(buddy + 0xc0, LOBBY_MAN_UPDATE_CHAL + 8);
        c.bus.write32(LOBBY_MAN_UPDATE_CHAL, DATA + 0x3700);
        c.bus.write32(LOBBY_MAN_UPDATE_CHAL + 4, DATA + 0x3780);
        lobby_man_update_put_str(c, DATA + 0x3700, b"alice\0");
        lobby_man_update_put_str(c, DATA + 0x3780, b"bob\0");
        lobby_man_update_put_str(
            c,
            LOBBY_MAN_UPDATE_NAME_OBJ,
            if scenario == 12 {
                b"alice\0"
            } else {
                b"carol\0"
            },
        );
        c.bus.write32(self_ + 0x98, LOBBY_MAN_UPDATE_NAME_OBJ);
        if scenario == 8 {
            c.bus.write32(buddy + 0xd0, 7);
        }
        if scenario == 9 {
            c.bus.write32(buddy + 0xc8, DATA + 0x3c00);
            c.bus.write32(buddy + 0xe0, 1);
        }
        // Lobby object.
        c.bus.write32(
            self_ + 0x54,
            if scenario == 0 {
                0
            } else {
                LOBBY_MAN_UPDATE_API_OBJ
            },
        );
        c.bus.write32(LOBBY_MAN_UPDATE_API_OBJ, 0);
        c.bus.write32(
            LOBBY_MAN_UPDATE_API_OBJ + 8,
            if scenario == 5 { 0x400 } else { 0 },
        );
        lobby_man_update_put_str(c, LOBBY_MAN_UPDATE_API_OBJ + 0x3b8, b"hi\0");
        c.bus.write32(self_ + 0x58, LOBBY_MAN_UPDATE_CONN_OBJ);
        c.bus.write32(LOBBY_MAN_UPDATE_CONN_OBJ + 4, 0x1000);
        c.bus.write32(
            LOBBY_MAN_UPDATE_CONN_OBJ + 8,
            match scenario {
                2 => LOBBY_MAN_UPDATE_POLL_FN,
                4 => LOBBY_MAN_UPDATE_NEG_FN,
                _ => LOBBY_MAN_UPDATE_ZERO_FN,
            },
        );
        c.bus
            .write32(LOBBY_MAN_UPDATE_CONN_OBJ + 12, LOBBY_MAN_UPDATE_ZERO_FN);
        c.bus
            .write32(self_ + 0x10, if scenario == 1 { 1 } else { 0 });
        c.bus
            .write32(self_ + 0x44, if scenario == 21 { 1 } else { 0 });
        c.bus.write32(self_ + 0xc, if sub >= 3 { 1 } else { 0 });
        c.bus.write32(self_ + 0x5c, LOBBY_MAN_UPDATE_SORT1);
        c.bus.write32(self_ + 0x60, LOBBY_MAN_UPDATE_SORT2);
        let state: u32 = match scenario {
            0 | 1 => 0,
            2 | 3 | 4 => 2,
            5 | 6 | 7 => 3,
            8 | 9 | 10 => 0x1a,
            11 => 0x16,
            12 | 13 | 14 => 0x17,
            15 | 16 => 0x18,
            17 | 18 => 0x14,
            19 => 0x19,
            20 | 21 => 1,
            _ => 5,
        };
        c.bus.write32(self_, state);
        let neg_timer = matches!(scenario, 6 | 12 | 13 | 15 | 17 | 20 | 21 | 23);
        c.bus.write32(
            self_ + 0xe0,
            if neg_timer { 0xbf80_0000 } else { 0x3f80_0000 },
        );
        c.bus.write32(self_ + 0xe4, 0x4000_0000);
        lobby_man_update_set_drain(c, sub % 3);
        r
    }

    #[test]
    fn lobby_man_update_matches() {
        check(
            ee::s06::f_0025ac50__generated,
            lobby_man_update,
            144,
            &|c, seed| {
                let _r = lobby_man_update_fixture(c, seed);
            },
        );
    }

    #[test]
    fn lobby_man_update_events() {
        check_events(
            gen_lobby_man_update,
            hand_lobby_man_update,
            60,
            &|c, seed| {
                let mut r = lobby_man_update_fixture(c, seed);
                // No-API path with a 2-message drain: the only back-edge
                // event check is the drain head at 0x25b578.
                c.bus.write32(LOBBY_MAN_UPDATE_SELF + 0x54, 0);
                c.bus.write32(LOBBY_MAN_UPDATE_BUDDY_OBJ + 0xc8, 0);
                lobby_man_update_set_drain(c, 2);
                schedule(c, &mut r, seed, 0x0025_ac50)
            },
        );
    }
}
