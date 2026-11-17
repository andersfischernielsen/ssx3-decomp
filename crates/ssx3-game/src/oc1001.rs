//! Staging module oc1001 (wave wave): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1001`.

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
    (
        0x0026_1a28,
        buddy_man_on_message_received,
        "BuddyMan_OnMessageReceived",
    ),
    (
        0x0026_1970,
        buddy_man_on_roster_remove_reply,
        "BuddyMan_OnRosterRemoveReply",
    ),
    (
        0x0026_18c0,
        buddy_man_on_roster_add_reply,
        "BuddyMan_OnRosterAddReply",
    ),
    (
        0x0026_1770,
        buddy_man_on_roster_event,
        "BuddyMan_OnRosterEvent",
    ),
    (
        0x0026_15c8,
        buddy_man_on_connect_event,
        "BuddyMan_OnConnectEvent",
    ),
    (0x0026_1248, buddy_man_connect, "BuddyMan_Connect"),
    (
        0x0026_10d0,
        buddy_man_add_message_senders_to_roster,
        "BuddyMan_AddMessageSendersToRoster",
    ),
    (0x0026_0c68, lobby_man_on_rank_reply, "LobbyMan_OnRankReply"),
    (0x0025_f8a0, lobby_man_on_chal_reply, "LobbyMan_OnChalReply"),
    (
        0x0025_f740,
        lobby_man_clear_challenge_lock,
        "LobbyMan_ClearChallengeLock",
    ),
];

// ---------------------------------------------------------------------------
// Callees.

pub(crate) mod callee {
    use super::ee;
    use super::Callee;

    /// `cDirtysock_tag_TagFieldFind` (0x3eb6d8), via `call_indirect`.
    pub const TAG_FIELD_FIND: u32 = 0x003e_b6d8;
    /// `TagFieldGetString4` / `TagFieldGetUnk4` (0x3ed378), via `call_indirect`.
    pub const TAG_FIELD_GET_STRING4: u32 = 0x003e_d378;
    /// `func_003EDD70` (tag date fields, 0x3edd70), via `call_indirect`.
    pub const TAG_FIELD_GET_DATE: u32 = 0x003e_dd70;
    /// `sprintf` (0x417828), via `call_indirect`.
    pub const SPRINTF: u32 = 0x0041_7828;
    /// `func_004162D0` (0x4162d0), via `call_indirect`.
    pub const FUNC_4162D0: u32 = 0x0041_62d0;
    /// `func_002623B0` (0x2623b0, buddy-name lookup), via `call_indirect`.
    pub const BUDDY_LOOKUP_2623B0: u32 = 0x0026_23b0;
    /// `func_00262A08` (0x262a08, roster index), via `call_indirect`.
    pub const ROSTER_INDEX_262A08: u32 = 0x0026_2a08;
    /// `func_00263630` (0x263630, roster entry), via `call_indirect`.
    pub const ROSTER_ENTRY_263630: u32 = 0x0026_3630;
    /// `func_002635A8` (0x2635a8, roster remove), via `call_indirect`.
    pub const ROSTER_REMOVE_2635A8: u32 = 0x0026_35a8;
    /// `strcmp`-like (0x4165a8), via `call_indirect`.
    pub const STR_CMP_4165A8: u32 = 0x0041_65a8;
    /// `cMemMan_alloc` (0x317d70), via `call_indirect`.
    pub const MEM_ALLOC_NAMED: u32 = 0x0031_7d70;
    /// `func_003F0C90` (0x3f0c90), via `call_indirect`.
    pub const FUNC_3F0C90: u32 = 0x003f_0c90;
    /// `func_003EE5A0` (0x3ee5a0), via `call_indirect`.
    pub const FUNC_3EE5A0: u32 = 0x003e_e5a0;
    /// `func_00262768` (0x262768), via `call_indirect`.
    pub const FUNC_262768: u32 = 0x0026_2768;
    /// `func_003F0818` (0x3f0818), via `call_indirect`.
    pub const FUNC_3F0818: u32 = 0x003f_0818;
    /// `strcpy` (0x4166f4), via `call_indirect`.
    pub const STRCPY: u32 = 0x0041_66f4;
    /// `cBXString_cBXString4` (0x3184e8), via `call_indirect`.
    pub const BXSTRING_CTOR: u32 = 0x0031_84e8;
    /// `cBXString_Concat` (0x318910), via `call_indirect`.
    pub const BXSTRING_CONCAT: u32 = 0x0031_8910;
    /// `cBXString__cBXString` (0x318260), via `call_indirect`.
    pub const BXSTRING_DTOR: u32 = 0x0031_8260;
    /// `func_003F08E8` (0x3f08e8), via `call_indirect`.
    pub const FUNC_3F08E8: u32 = 0x003f_08e8;
    /// `func_00262938` (0x262938, roster count), via `call_indirect`.
    pub const ROSTER_COUNT_262938: u32 = 0x0026_2938;
    /// `func_00262468` (0x262468), via `call_indirect`.
    pub const FUNC_262468: u32 = 0x0026_2468;
    /// `func_00261EF8` (0x261ef8), via `call_indirect`.
    pub const FUNC_261EF8: u32 = 0x0026_1ef8;
    /// `cBE_getInterface` (0x14dd58), via `call_indirect`.
    pub const BE_GET_INTERFACE: u32 = 0x0014_dd58;
    /// `cDirtysock_tag_TagFieldSetString` (0x3ec180), via `call_indirect`.
    pub const TAG_FIELD_SET_STRING: u32 = 0x003e_c180;
    /// `func_003E8B10` (0x3e8b10), via `call_indirect`.
    pub const FUNC_3E8B10: u32 = 0x003e_8b10;

    pub const BUDDY_MAN_ADD_MESSAGE: Callee = Callee {
        addr: 0x0026_1cd0,
        f: ee::s06::BuddyMan_AddMessage,
    };
    pub const TAG_FIELD_GET_STRING_UNESCAPED: Callee = Callee {
        addr: 0x0025_9310,
        f: ee::s06::TagFieldGetStringUnescaped,
    };
    pub const BUDDY_MAN_ADD_SENDERS: Callee = Callee {
        addr: 0x0026_10d0,
        f: ee::s06::BuddyMan_AddMessageSendersToRoster,
    };
    pub const BUDDY_MAN_CREATE_API: Callee = Callee {
        addr: 0x0026_14e8,
        f: ee::s06::BuddyMan_CreateApi,
    };
    pub const LOBBY_MAN_GET_LANGUAGE: Callee = Callee {
        addr: 0x0025_9198,
        f: ee::s06::LobbyMan_GetLanguage,
    };
    pub const LOBBY_MAN_SEND_ABORT_CHAL: Callee = Callee {
        addr: 0x0025_fc48,
        f: ee::s06::LobbyMan_SendAbortChal,
    };
    pub const LOBBY_MAN_SEND_LOCK_CHAL: Callee = Callee {
        addr: 0x0025_fc70,
        f: ee::s06::LobbyMan_SendLockChal,
    };
    pub const BUDDY_MAN_CREATE: Callee = Callee {
        addr: 0x0026_0f80,
        f: ee::s06::BuddyMan_Create,
    };
    pub const BUDDY_MAN_IS_CONNECTED: Callee = Callee {
        addr: 0x0026_1460,
        f: ee::s06::BuddyMan_IsConnected,
    };
    pub const BUDDY_MAN_RECONNECT: Callee = Callee {
        addr: 0x0026_13c8,
        f: ee::s06::BuddyMan_Reconnect,
    };
    pub const LOBBY_MAN_DISCONNECT: Callee = Callee {
        addr: 0x0025_b800,
        f: ee::s06::LobbyMan_Disconnect,
    };
}

/// `D_00480488`: allocation tag passed to every event-object allocation.
pub(crate) const ALLOC_TAG_480488: u32 = 0x0048_0488;
/// `D_004812B0`: vtable word stored in 8-byte event objects.
pub(crate) const EVENT_VT_4812B0: u32 = 0x0048_12b0;
/// The lobby instance global (`gp-0xc8`, `D_004A3028`).
pub(crate) const LOBBY_INSTANCE_GP: i32 = -0xc8;

/// A `gp`-relative global address (the caller in `tagsunk.rs` calls this
/// helper `gp_8`; kept local so this staging module is self-contained).
#[inline(always)]
pub(crate) fn gp_addr(c: &Cpu, off: i32) -> u32 {
    r32(c, GP).wrapping_add(off as u32)
}

/// `void BuddyMan_OnRosterRemoveReply()` (0x261970): post a kind-0x10B
/// roster-remove event: allocate the 8-byte event (kind 0x10B, vtable
/// `D_004812B0`), wrap it in a 12-byte list node and splice the node into
/// the message list headed at lobby+0xF0. 29 + 14 cycles plus the two
/// allocations.
pub fn buddy_man_on_roster_remove_reply(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 29;
        let frame = Frame::open(c, 0x50);
        c.g[A1][0] = sx(ALLOC_TAG_480488);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[A0][0] = 8;
        frame.save_ra(c, 0x20);
        c.g[A2][0] = 0x2000_0000;
        c.g[S0][0] = Addr(gp_addr(c, LOBBY_INSTANCE_GP)).lw(c);
        c.g[A3][0] = 0;
        call_ptr(c, callee::MEM_ALLOC_NAMED, 0x0026_199c)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[V1][0] = 0x10b;
        c.g[V0][0] = 0x480000;
        Addr(r32(c, S1)).sw(c, c.g[V1][0]);
        c.g[V0][0] = sx(EVENT_VT_4812B0);
        c.g[A1][0] = 0x480000;
        Addr(r32(c, S1).wrapping_add(4)).sw(c, c.g[V0][0]);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4a8));
        c.g[A0][0] = 0xc;
        c.g[A2][0] = 0x2000_0000;
        c.g[V0][0] = Addr(r32(c, S0).wrapping_add(0xf0)).lw(c);
        c.g[A3][0] = 0;
        Addr(frame.at(c, 0x10)).sw(c, c.g[V0][0]);
        call_ptr(c, callee::MEM_ALLOC_NAMED, 0x0026_19d4)?;
        c.g[A1][0] = c.g[V0][0];
        c.g[V0][0] = sx(r32(c, A1).wrapping_add(8));
        if c.g[V0][0] != 0 {
            Addr(r32(c, A1).wrapping_add(8)).sw(c, c.g[S1][0]);
        }
        c.cycles += 14;
        c.g[V0][0] = Addr(frame.at(c, 0x10)).lw(c);
        frame.restore(c, S0, 0x40);
        Addr(r32(c, A1)).sw(c, c.g[V0][0]);
        frame.restore(c, S1, 0x30);
        c.g[V1][0] = Addr(r32(c, V0).wrapping_add(4)).lw(c);
        frame.restore_ra(c, 0x20);
        Addr(r32(c, A1).wrapping_add(4)).sw(c, c.g[V1][0]);
        c.g[A0][0] = Addr(r32(c, V0).wrapping_add(4)).lw(c);
        Addr(r32(c, A0)).sw(c, c.g[A1][0]);
        c.g[V0][0] = Addr(frame.at(c, 0x10)).lw(c);
        Addr(r32(c, V0).wrapping_add(4)).sw(c, c.g[A1][0]);
        Addr(frame.at(c, 0)).sw(c, c.g[A1][0]);
        frame.close(c);
        Ok(())
    })
}

/// `void BuddyMan_OnRosterEvent(BuddyMan* self, char* name)` (0x261770,
/// asm-only in the reference): if `name` is not `"endr"`, or the roster at
/// self+0xE8 is already built, post a kind-0x106 event to the lobby list;
/// on the first `"endr"` event, add the message senders to the roster,
/// mark self+0xE8 and post a kind-0x107 event instead. 10 (+3+28 / +1 /
/// +24) + 1 + 14 cycles plus the calls.
pub fn buddy_man_on_roster_event(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x60);
        let mut direct_to_epilogue = false;
        'epilogue: {
            'post106: {
                'first_endr: {
                    c.cycles += 10;
                    c.g[V0][0] = 0x656e_6472;
                    frame.save(c, S0, 0x50);
                    frame.save(c, S1, 0x40);
                    frame.save(c, S2, 0x30);
                    frame.save_ra(c, 0x20);
                    c.g[V1][0] = Addr(r32(c, A1)).lw(c);
                    let not_endr = c.g[V1][0] != c.g[V0][0];
                    c.g[S0][0] = c.g[A0][0];
                    if not_endr {
                        break 'first_endr;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = Addr(r32(c, S0).wrapping_add(0xe8)).lw(c);
                    let roster_built = c.g[V0][0] != 0;
                    c.g[A1][0] = 0x480000;
                    if roster_built {
                        break 'post106;
                    }
                    c.cycles += 28;
                    c.g[S1][0] = 0x107;
                    callee::BUDDY_MAN_ADD_SENDERS.call(c, 0x0026_17ac)?;
                    c.g[V0][0] = 1;
                    c.g[A1][0] = 0x480000;
                    Addr(r32(c, S0).wrapping_add(0xe8)).sw(c, c.g[V0][0]);
                    c.g[A1][0] = sx(ALLOC_TAG_480488);
                    c.g[S0][0] = Addr(gp_addr(c, LOBBY_INSTANCE_GP)).lw(c);
                    c.g[A0][0] = 8;
                    c.g[A2][0] = 0x2000_0000;
                    c.g[A3][0] = 0;
                    call_ptr(c, callee::MEM_ALLOC_NAMED, 0x0026_17d0)?;
                    c.g[S2][0] = c.g[V0][0];
                    c.g[A1][0] = 0x480000;
                    c.g[V0][0] = 0x480000;
                    Addr(r32(c, S2)).sw(c, c.g[S1][0]);
                    c.g[V0][0] = sx(EVENT_VT_4812B0);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4a8));
                    Addr(r32(c, S2).wrapping_add(4)).sw(c, c.g[V0][0]);
                    c.g[A0][0] = 0xc;
                    c.g[A2][0] = 0x2000_0000;
                    c.g[A3][0] = 0;
                    c.g[V0][0] = Addr(r32(c, S0).wrapping_add(0xf0)).lw(c);
                    Addr(frame.at(c, 0x10)).sw(c, c.g[V0][0]);
                    call_ptr(c, callee::MEM_ALLOC_NAMED, 0x0026_1804)?;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[V0][0] = sx(r32(c, A1).wrapping_add(8));
                    if c.g[V0][0] != 0 {
                        Addr(r32(c, A1).wrapping_add(8)).sw(c, c.g[S2][0]);
                        break 'post106;
                    }
                    c.cycles += 2;
                    c.g[V0][0] = Addr(frame.at(c, 0x10)).lw(c);
                    direct_to_epilogue = true;
                    break 'epilogue;
                }
                c.cycles += 1;
                c.g[A1][0] = 0x480000;
            }
            c.cycles += 24;
            c.g[S0][0] = Addr(gp_addr(c, LOBBY_INSTANCE_GP)).lw(c);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x488));
            c.g[A0][0] = 8;
            c.g[A2][0] = 0x2000_0000;
            c.g[A3][0] = 0;
            call_ptr(c, callee::MEM_ALLOC_NAMED, 0x0026_1838)?;
            c.g[S1][0] = c.g[V0][0];
            c.g[V1][0] = 0x106;
            c.g[V0][0] = 0x480000;
            Addr(r32(c, S1)).sw(c, c.g[V1][0]);
            c.g[V0][0] = sx(EVENT_VT_4812B0);
            c.g[A1][0] = 0x480000;
            Addr(r32(c, S1).wrapping_add(4)).sw(c, c.g[V0][0]);
            c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4a8));
            c.g[A0][0] = 0xc;
            c.g[A2][0] = 0x2000_0000;
            c.g[V0][0] = Addr(r32(c, S0).wrapping_add(0xf0)).lw(c);
            c.g[A3][0] = 0;
            Addr(frame.at(c, 0x10)).sw(c, c.g[V0][0]);
            call_ptr(c, callee::MEM_ALLOC_NAMED, 0x0026_1870)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[V0][0] = sx(r32(c, A1).wrapping_add(8));
            if c.g[V0][0] != 0 {
                Addr(r32(c, A1).wrapping_add(8)).sw(c, c.g[S1][0]);
            }
        }
        if !direct_to_epilogue {
            c.cycles += 1;
            c.g[V0][0] = Addr(frame.at(c, 0x10)).lw(c);
        }
        buddy_man_on_roster_event_splice(c, frame);
        Ok(())
    })
}

/// The shared list splice + epilogue of `BuddyMan_OnRosterEvent`
/// (0x261770): splice the node in `a1` after the head saved at `sp+0x10`,
/// restore s0-s2/ra and return. 14 cycles.
fn buddy_man_on_roster_event_splice(c: &mut Cpu, frame: Frame) {
    c.cycles += 14;
    Addr(r32(c, A1)).sw(c, c.g[V0][0]);
    c.g[V1][0] = Addr(r32(c, V0).wrapping_add(4)).lw(c);
    Addr(r32(c, A1).wrapping_add(4)).sw(c, c.g[V1][0]);
    c.g[A0][0] = Addr(r32(c, V0).wrapping_add(4)).lw(c);
    Addr(r32(c, A0)).sw(c, c.g[A1][0]);
    c.g[V0][0] = Addr(frame.at(c, 0x10)).lw(c);
    Addr(r32(c, V0).wrapping_add(4)).sw(c, c.g[A1][0]);
    Addr(frame.at(c, 0)).sw(c, c.g[A1][0]);
    frame.restore(c, S0, 0x50);
    frame.restore(c, S1, 0x40);
    frame.restore(c, S2, 0x30);
    frame.restore_ra(c, 0x20);
    frame.close(c);
}

/// `void BuddyMan_OnConnectEvent(BuddyMan* self, int* msg)` (0x2615c8):
/// dispatch on the message tag: `"conn"` updates the connection state
/// fields (+0xDC/+0xD0, clearing +0xE8 for state 6), `"auth"` either
/// creates the session (+0xCC/+0xD8/+0xE0) when msg[2] is 0 or posts a
/// kind-0x104 event and copies msg[2] into +0xD0. 12 (+4…) + 6 cycles
/// plus the calls.
pub fn buddy_man_on_connect_event(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x70);
        'out: {
            'mid: {
                'auth: {
                    c.cycles += 12;
                    c.g[V0][0] = 0x6175_7468;
                    frame.save(c, S1, 0x50);
                    frame.save(c, S3, 0x30);
                    frame.save(c, S0, 0x60);
                    c.g[S3][0] = c.g[A1][0];
                    frame.save(c, S2, 0x40);
                    frame.save_ra(c, 0x20);
                    c.g[V1][0] = Addr(r32(c, S3)).lw(c);
                    let is_auth = c.g[V1][0] == c.g[V0][0];
                    c.g[S1][0] = c.g[A0][0];
                    if is_auth {
                        break 'auth;
                    }
                    c.cycles += 4;
                    c.g[V0][0] = 0x636f_6e6e;
                    let not_conn = c.g[V1][0] != c.g[V0][0];
                    if not_conn {
                        break 'out;
                    }
                    'conn_done: {
                        'conn_state: {
                            c.cycles += 4;
                            c.g[A1][0] = Addr(r32(c, S3).wrapping_add(4)).lw(c);
                            c.g[V0][0] = 2;
                            let state_is_2 = c.g[A1][0] == c.g[V0][0];
                            c.g[V0][0] = 6;
                            if state_is_2 {
                                break 'conn_state;
                            }
                            'state6: {
                                c.cycles += 2;
                                let state_not_6 = c.g[A1][0] != c.g[V0][0];
                                c.g[V0][0] = 4;
                                if state_not_6 {
                                    break 'state6;
                                }
                                c.cycles += 4;
                                c.g[V1][0] = 1;
                                c.g[V0][0] = Addr(r32(c, S1).wrapping_add(0xd0)).lw(c);
                                Addr(r32(c, S1).wrapping_add(0xe8)).sw(c, 0);
                                break 'conn_done;
                            }
                            c.cycles += 2;
                            let state_is_4 = c.g[A1][0] == c.g[V0][0];
                            c.g[V0][0] = 3;
                            if state_is_4 {
                                break 'conn_state;
                            }
                            c.cycles += 2;
                            let state_not_3 = c.g[A1][0] != c.g[V0][0];
                            if state_not_3 {
                                break 'out;
                            }
                            break 'conn_state;
                        }
                        c.cycles += 2;
                        c.g[V1][0] = 1;
                        c.g[V0][0] = Addr(r32(c, S1).wrapping_add(0xd0)).lw(c);
                    }
                    c.cycles += 2;
                    let was_connected = c.g[V0][0] != 0;
                    Addr(r32(c, S1).wrapping_add(0xdc)).sw(c, c.g[V1][0]);
                    if was_connected {
                        break 'mid;
                    }
                    c.cycles += 2;
                    Addr(r32(c, S1).wrapping_add(0xd0)).sw(c, c.g[V1][0]);
                    break 'mid;
                }
                'auth_body: {
                    c.cycles += 3;
                    c.g[V0][0] = Addr(r32(c, S3).wrapping_add(8)).lw(c);
                    let msg2_nonzero = c.g[V0][0] != 0;
                    c.g[A1][0] = 0x480000;
                    if msg2_nonzero {
                        break 'auth_body;
                    }
                    c.cycles += 23;
                    c.g[A0][0] = Addr(r32(c, S1).wrapping_add(0xc8)).lw(c);
                    c.g[A1][0] = sx(0x0026_8f68);
                    c.g[A2][0] = c.g[S1][0];
                    call_ptr(c, callee::FUNC_3F0C90, 0x0026_1678)?;
                    c.g[A3][0] = sx(0x0026_34c0);
                    c.g[A0][0] = c.g[V0][0];
                    Addr(r32(c, S1).wrapping_add(0xcc)).sw(c, c.g[V0][0]);
                    c.g[A1][0] = 0;
                    c.g[A2][0] = 0;
                    call_ptr(c, callee::FUNC_3EE5A0, 0x0026_1694)?;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[A1][0] = 1;
                    c.g[A2][0] = 0;
                    c.g[A3][0] = 0;
                    c.g[T0][0] = 0;
                    c.g[T1][0] = 0;
                    call_ptr(c, callee::FUNC_262768, 0x0026_16b0)?;
                    c.g[V0][0] = 1;
                    Addr(r32(c, S1).wrapping_add(0xd8)).sw(c, 0);
                    Addr(r32(c, S1).wrapping_add(0xe0)).sw(c, c.g[V0][0]);
                    break 'mid;
                }
                {
                    c.cycles += 24;
                    c.g[S0][0] = Addr(gp_addr(c, LOBBY_INSTANCE_GP)).lw(c);
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x488));
                    c.g[A0][0] = 8;
                    c.g[A2][0] = 0x2000_0000;
                    c.g[A3][0] = 0;
                    call_ptr(c, callee::MEM_ALLOC_NAMED, 0x0026_16d8)?;
                    c.g[S2][0] = c.g[V0][0];
                    c.g[V1][0] = 0x104;
                    c.g[V0][0] = 0x480000;
                    Addr(r32(c, S2)).sw(c, c.g[V1][0]);
                    c.g[V0][0] = sx(EVENT_VT_4812B0);
                    c.g[A1][0] = 0x480000;
                    Addr(r32(c, S2).wrapping_add(4)).sw(c, c.g[V0][0]);
                    c.g[A2][0] = 0x2000_0000;
                    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4a8));
                    c.g[A0][0] = 0xc;
                    c.g[V0][0] = Addr(r32(c, S0).wrapping_add(0xf0)).lw(c);
                    c.g[A3][0] = 0;
                    Addr(frame.at(c, 0x10)).sw(c, c.g[V0][0]);
                    call_ptr(c, callee::MEM_ALLOC_NAMED, 0x0026_1710)?;
                    c.g[A2][0] = c.g[V0][0];
                    c.g[V0][0] = sx(r32(c, A2).wrapping_add(8));
                    if c.g[V0][0] != 0 {
                        Addr(r32(c, A2).wrapping_add(8)).sw(c, c.g[S2][0]);
                    }
                }
                c.cycles += 13;
                c.g[A0][0] = Addr(frame.at(c, 0x10)).lw(c);
                c.g[A1][0] = 1;
                Addr(r32(c, A2)).sw(c, c.g[A0][0]);
                c.g[V0][0] = Addr(r32(c, A0).wrapping_add(4)).lw(c);
                Addr(r32(c, A2).wrapping_add(4)).sw(c, c.g[V0][0]);
                c.g[V1][0] = Addr(r32(c, A0).wrapping_add(4)).lw(c);
                Addr(r32(c, V1)).sw(c, c.g[A2][0]);
                c.g[V0][0] = Addr(frame.at(c, 0x10)).lw(c);
                Addr(r32(c, V0).wrapping_add(4)).sw(c, c.g[A2][0]);
                Addr(frame.at(c, 0)).sw(c, c.g[A2][0]);
                c.g[V0][0] = Addr(r32(c, S3).wrapping_add(8)).lw(c);
                Addr(r32(c, S1).wrapping_add(0xdc)).sw(c, c.g[A1][0]);
                Addr(r32(c, S1).wrapping_add(0xd0)).sw(c, c.g[V0][0]);
                break 'mid;
            }
            c.cycles += 1;
        }
        c.cycles += 6;
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `void BuddyMan_Connect(BuddyMan* self, char* name, int idx, char* a3,
/// char* a4, char* a5)` (0x261248): create the buddy API if self+0xC8 is
/// empty, copy name/a3/a4 into the object, build the two BXStrings and
/// register with the session at self+0xC8. 16 (+11) + 68 cycles plus the
/// calls.
pub fn buddy_man_connect(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x90);
        'have_api: {
            c.cycles += 16;
            frame.save(c, S0, 0x80);
            frame.save(c, S1, 0x70);
            c.g[S0][0] = c.g[A3][0];
            frame.save(c, S2, 0x60);
            c.g[S1][0] = c.g[A0][0];
            frame.save(c, S3, 0x50);
            c.g[S2][0] = c.g[T1][0];
            frame.save(c, S4, 0x40);
            c.g[S3][0] = c.g[T0][0];
            frame.save(c, S5, 0x30);
            c.g[S4][0] = c.g[A2][0];
            frame.save_ra(c, 0x20);
            c.g[V0][0] = Addr(r32(c, S1).wrapping_add(0xc8)).lw(c);
            let has_api = c.g[V0][0] != 0;
            c.g[S5][0] = c.g[A1][0];
            if has_api {
                break 'have_api;
            }
            c.cycles += 11;
            c.g[A1][0] = c.g[S2][0];
            callee::BUDDY_MAN_CREATE_API.call(c, 0x0026_1290)?;
            callee::LOBBY_MAN_GET_LANGUAGE.call(c, 0x0026_1298)?;
            c.g[A2][0] = 0x7373_7833;
            c.g[A0][0] = Addr(r32(c, S1).wrapping_add(0xc8)).lw(c);
            c.g[A3][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S2][0];
            c.g[T0][0] = 0;
            call_ptr(c, callee::FUNC_3F0818, 0x0026_12b4)?;
        }
        c.cycles += 68;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = c.g[S5][0];
        call_ptr(c, callee::STRCPY, 0x0026_12c0)?;
        Addr(r32(c, S1).wrapping_add(0x40)).sw(c, c.g[S4][0]);
        c.g[A0][0] = sx(r32(c, S1).wrapping_add(0x44));
        c.g[A1][0] = c.g[S0][0];
        call_ptr(c, callee::STRCPY, 0x0026_12d0)?;
        c.g[A0][0] = sx(r32(c, S1).wrapping_add(0x64));
        c.g[A1][0] = c.g[S3][0];
        call_ptr(c, callee::STRCPY, 0x0026_12dc)?;
        c.g[V1][0] = Addr(r32(c, GP).wrapping_add(0xda0)).lw(c);
        c.g[V0][0] = 1;
        c.g[AT][0] = 0x41f0_0000;
        c.mtc1(1, 0);
        c.g[A0][0] = c.g[SP][0];
        Addr(r32(c, S1).wrapping_add(0xd0)).sw(c, 0);
        c.g[A1][0] = c.g[S0][0];
        Addr(r32(c, S1).wrapping_add(0xdc)).sw(c, 0);
        Addr(r32(c, S1).wrapping_add(0xe0)).sw(c, 0);
        Addr(frame.at(c, 0)).sw(c, c.g[V1][0]);
        c.swc1(0, r32(c, S1).wrapping_add(0xd4));
        Addr(r32(c, S1).wrapping_add(0xd8)).sw(c, c.g[V0][0]);
        call_ptr(c, callee::BXSTRING_CTOR, 0x0026_1310)?;
        c.g[A1][0] = sx(0x004a_3330);
        c.g[A0][0] = c.g[SP][0];
        call_ptr(c, callee::BXSTRING_CONCAT, 0x0026_1320)?;
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] = c.g[S3][0];
        call_ptr(c, callee::BXSTRING_CONCAT, 0x0026_132c)?;
        c.g[V1][0] = Addr(r32(c, GP).wrapping_add(0xda0)).lw(c);
        c.g[S0][0] = sx(frame.at(c, 0x10));
        c.g[A1][0] = sx(0x004a_3338);
        c.g[A0][0] = c.g[S0][0];
        Addr(frame.at(c, 0x10)).sw(c, c.g[V1][0]);
        call_ptr(c, callee::BXSTRING_CTOR, 0x0026_1348)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[S2][0];
        call_ptr(c, callee::BXSTRING_CONCAT, 0x0026_1354)?;
        c.g[A1][0] = c.g[S2][0];
        c.g[A0][0] = sx(r32(c, S1).wrapping_add(0xec));
        call_ptr(c, callee::BXSTRING_CTOR, 0x0026_1360)?;
        c.g[T1][0] = Addr(frame.at(c, 0x10)).lw(c);
        c.g[T2][0] = sx(0x0026_8f18);
        c.g[A0][0] = Addr(r32(c, S1).wrapping_add(0xc8)).lw(c);
        c.g[T3][0] = c.g[S1][0];
        c.g[T0][0] = Addr(frame.at(c, 0)).lw(c);
        c.g[A2][0] = c.g[S4][0];
        c.g[A1][0] = c.g[S5][0];
        c.g[A3][0] = 0x1e;
        call_ptr(c, callee::FUNC_3F08E8, 0x0026_1388)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = 2;
        call_ptr(c, callee::BXSTRING_DTOR, 0x0026_1394)?;
        c.g[A0][0] = c.g[SP][0];
        c.g[A1][0] = 2;
        call_ptr(c, callee::BXSTRING_DTOR, 0x0026_13a0)?;
        frame.restore(c, S0, 0x80);
        frame.restore(c, S1, 0x70);
        frame.restore(c, S2, 0x60);
        frame.restore(c, S3, 0x50);
        frame.restore(c, S4, 0x40);
        frame.restore(c, S5, 0x30);
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `void LobbyMan_ClearChallengeLock(LobbyMan* self, int type)` (0x25f740):
/// if the state word is 0x19, reset the challenge timer to 20.0f, clear
/// the `"chal"` tag string and set the state to 1; then, when `type` is
/// non-zero, post an 8-byte event of that kind to the list at self+0xF0.
/// 9 (+3+19 / +2+22+9) + 6 cycles plus the calls.
pub fn lobby_man_clear_challenge_lock(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x140);
        'done: {
            c.cycles += 9;
            frame.save(c, S1, 0x120);
            frame.save(c, S2, 0x110);
            c.g[S1][0] = c.g[A0][0];
            frame.save(c, S0, 0x130);
            frame.save_ra(c, 0x100);
            c.g[V1][0] = Addr(r32(c, S1)).lw(c);
            let state_zero = c.g[V1][0] == 0;
            c.g[S2][0] = c.g[A1][0];
            if state_zero {
                break 'done;
            }
            'cleared: {
                c.cycles += 3;
                c.g[V0][0] = 0x19;
                let not_locked = c.g[V1][0] != c.g[V0][0];
                c.g[A2][0] = 0x4a0000;
                if not_locked {
                    break 'cleared;
                }
                c.cycles += 19;
                c.g[AT][0] = 0x41a0_0000;
                c.mtc1(1, 0);
                c.g[A3][0] = sx(0x004a_3218);
                c.g[A0][0] = c.g[SP][0];
                c.swc1(0, r32(c, S1).wrapping_add(0xe0));
                c.g[A2][0] = sx(0x004a_3108);
                Addr(frame.at(c, 0)).sb(c, 0);
                c.g[A1][0] = 0x100;
                call_ptr(c, callee::TAG_FIELD_SET_STRING, 0x0025_f798)?;
                c.g[A0][0] = Addr(r32(c, S1).wrapping_add(0x54)).lw(c);
                c.g[A1][0] = 0x6368_616c;
                c.g[A2][0] = c.g[SP][0];
                c.g[A3][0] = 0;
                c.g[T0][0] = 0;
                call_ptr(c, callee::FUNC_3E8B10, 0x0025_f7b4)?;
                c.g[V1][0] = 1;
                Addr(r32(c, S1)).sw(c, c.g[V1][0]);
            }
            c.cycles += 2;
            let no_type = c.g[S2][0] == 0;
            c.g[A1][0] = 0x480000;
            if no_type {
                break 'done;
            }
            {
                c.cycles += 22;
                c.g[A0][0] = 8;
                c.g[A1][0] = sx(ALLOC_TAG_480488);
                c.g[A2][0] = 0x2000_0000;
                c.g[A3][0] = 0;
                call_ptr(c, callee::MEM_ALLOC_NAMED, 0x0025_f7d8)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[V1][0] = Addr(r32(c, S1).wrapping_add(0xf0)).lw(c);
                c.g[V0][0] = 0x480000;
                Addr(r32(c, S0)).sw(c, c.g[S2][0]);
                c.g[V0][0] = sx(EVENT_VT_4812B0);
                c.g[A1][0] = 0x480000;
                Addr(r32(c, S0).wrapping_add(4)).sw(c, c.g[V0][0]);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4a8));
                Addr(frame.at(c, 0x10)).sw(c, c.g[V1][0]);
                c.g[A0][0] = 0xc;
                c.g[A2][0] = 0x2000_0000;
                c.g[A3][0] = 0;
                call_ptr(c, callee::MEM_ALLOC_NAMED, 0x0025_f80c)?;
                c.g[A1][0] = c.g[V0][0];
                c.g[V0][0] = sx(r32(c, A1).wrapping_add(8));
                if c.g[V0][0] != 0 {
                    Addr(r32(c, A1).wrapping_add(8)).sw(c, c.g[S0][0]);
                }
            }
            c.cycles += 9;
            c.g[V0][0] = Addr(frame.at(c, 0x10)).lw(c);
            Addr(r32(c, A1)).sw(c, c.g[V0][0]);
            c.g[V1][0] = Addr(r32(c, V0).wrapping_add(4)).lw(c);
            Addr(r32(c, A1).wrapping_add(4)).sw(c, c.g[V1][0]);
            c.g[A0][0] = Addr(r32(c, V0).wrapping_add(4)).lw(c);
            Addr(r32(c, A0)).sw(c, c.g[A1][0]);
            c.g[V0][0] = Addr(frame.at(c, 0x10)).lw(c);
            Addr(r32(c, V0).wrapping_add(4)).sw(c, c.g[A1][0]);
            Addr(frame.at(c, 0)).sw(c, c.g[A1][0]);
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

/// `void LobbyMan_OnRankReply(LobbyMan* self, int* msg)` (0x260c68): ask
/// the BE interface (id 7) about the session, then post a kind-0x101
/// event when msg[2] is 0 or a `"rank"` reply with the global flag clear;
/// otherwise post a kind-0x102 event carrying msg[2] followed by a
/// kind-0x101 event, both to the list at self+0xEC. 17 (+3+4 / +20 /
/// +30+24+6) + 18 cycles plus the calls.
pub fn lobby_man_on_rank_reply(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x90);
        'out: {
            {
                'simple: {
                    c.cycles += 17;
                    c.g[V0][0] = Addr(gp_addr(c, -0x848)).lw(c);
                    frame.save(c, S0, 0x80);
                    frame.save(c, S2, 0x60);
                    c.g[S0][0] = c.g[A1][0];
                    frame.save(c, S1, 0x70);
                    c.g[S2][0] = c.g[A0][0];
                    frame.save(c, S3, 0x50);
                    c.g[A1][0] = 7;
                    frame.save(c, S4, 0x40);
                    frame.save(c, S5, 0x30);
                    frame.save_ra(c, 0x20);
                    c.g[A0][0] = Addr(r32(c, V0).wrapping_add(0x78)).lw(c);
                    call_ptr(c, callee::BE_GET_INTERFACE, 0x0026_0ca0)?;
                    c.g[V1][0] = Addr(r32(c, S0).wrapping_add(8)).lw(c);
                    let code_zero = c.g[V1][0] == 0;
                    c.g[V0][0] = 0x7261_0000;
                    if code_zero {
                        break 'simple;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = 0x7261_6e6b;
                    let not_rank = c.g[V1][0] != c.g[V0][0];
                    c.g[V0][0] = 0x480000;
                    if not_rank {
                        lobby_man_on_rank_reply_err_splice(c, frame);
                        return Ok(());
                    }
                    c.cycles += 4;
                    c.g[V1][0] = Addr(0x0053_4b34).lw(c);
                    let flag_set = c.g[V1][0] != 0;
                    c.g[V0][0] = 0x480000;
                    if flag_set {
                        lobby_man_on_rank_reply_err_splice(c, frame);
                        return Ok(());
                    }
                    break 'simple;
                }
                c.cycles += 20;
                c.g[A1][0] = sx(ALLOC_TAG_480488);
                c.g[A0][0] = 8;
                c.g[A2][0] = 0x2000_0000;
                c.g[A3][0] = 0;
                c.g[S0][0] = 0x101;
                call_ptr(c, callee::MEM_ALLOC_NAMED, 0x0026_0ce4)?;
                c.g[S1][0] = c.g[V0][0];
                c.g[V1][0] = Addr(r32(c, S2).wrapping_add(0xf0)).lw(c);
                c.g[V0][0] = 0x480000;
                Addr(r32(c, S1)).sw(c, c.g[S0][0]);
                c.g[V0][0] = sx(EVENT_VT_4812B0);
                c.g[A1][0] = 0x480000;
                Addr(r32(c, S1).wrapping_add(4)).sw(c, c.g[V0][0]);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4a8));
                Addr(frame.at(c, 0x10)).sw(c, c.g[V1][0]);
                c.g[A0][0] = 0xc;
                c.g[A2][0] = 0x2000_0000;
                c.g[A3][0] = 0;
                break 'out;
            }
        }
        lobby_man_on_rank_reply_node_splice(c, frame);
        Ok(())
    })
}

/// The error post + node splice of `LobbyMan_OnRankReply` (0x260c68):
/// post the kind-0x102 event carrying the reply code and a kind-0x101
/// event to the list at self+0xEC, then the shared node splice.
/// 30 + 24 cycles plus the allocations.
fn lobby_man_on_rank_reply_err_splice(c: &mut Cpu, frame: Frame) {
    guest(c, |c| {
        c.cycles += 30;
        c.g[A0][0] = 0xc;
        c.g[S3][0] = sx((c.g[V0][0] as u32).wrapping_add(0x488));
        c.g[S0][0] = Addr(r32(c, S0).wrapping_add(8)).lw(c);
        c.g[A1][0] = c.g[S3][0];
        c.g[A2][0] = 0x2000_0000;
        c.g[A3][0] = 0;
        c.g[S2][0] = sx(r32(c, S2).wrapping_add(0xec));
        call_ptr(c, callee::MEM_ALLOC_NAMED, 0x0026_0d38)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[V1][0] = 0x480000;
        c.g[V0][0] = 0x102;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x1290));
        Addr(r32(c, S1)).sw(c, c.g[V0][0]);
        c.g[A0][0] = 0x480000;
        Addr(r32(c, S1).wrapping_add(4)).sw(c, c.g[V1][0]);
        c.g[V0][0] = 0x480000;
        Addr(r32(c, S1).wrapping_add(8)).sw(c, c.g[S0][0]);
        c.g[S5][0] = sx(EVENT_VT_4812B0);
        c.g[S4][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4a8));
        c.g[A2][0] = 0x2000_0000;
        c.g[V0][0] = Addr(r32(c, S2).wrapping_add(4)).lw(c);
        c.g[A0][0] = 0xc;
        c.g[A1][0] = c.g[S4][0];
        c.g[A3][0] = 0;
        Addr(frame.at(c, 0x10)).sw(c, c.g[V0][0]);
        call_ptr(c, callee::MEM_ALLOC_NAMED, 0x0026_0d80)?;
        c.g[T1][0] = c.g[V0][0];
        c.g[V0][0] = sx(r32(c, T1).wrapping_add(8));
        if c.g[V0][0] != 0 {
            Addr(r32(c, T1).wrapping_add(8)).sw(c, c.g[S1][0]);
        }
        c.cycles += 24;
        c.g[T0][0] = Addr(frame.at(c, 0x10)).lw(c);
        c.g[A1][0] = c.g[S3][0];
        c.g[A0][0] = 8;
        c.g[A2][0] = 0x2000_0000;
        Addr(r32(c, T1)).sw(c, c.g[T0][0]);
        c.g[A3][0] = 0;
        c.g[S0][0] = 0x101;
        c.g[V0][0] = Addr(r32(c, T0).wrapping_add(4)).lw(c);
        Addr(r32(c, T1).wrapping_add(4)).sw(c, c.g[V0][0]);
        c.g[V1][0] = Addr(r32(c, T0).wrapping_add(4)).lw(c);
        Addr(r32(c, V1)).sw(c, c.g[T1][0]);
        c.g[V0][0] = Addr(frame.at(c, 0x10)).lw(c);
        Addr(r32(c, V0).wrapping_add(4)).sw(c, c.g[T1][0]);
        Addr(frame.at(c, 0)).sw(c, c.g[T1][0]);
        call_ptr(c, callee::MEM_ALLOC_NAMED, 0x0026_0dcc)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S4][0];
        Addr(r32(c, S1).wrapping_add(4)).sw(c, c.g[S5][0]);
        c.g[A0][0] = 0xc;
        Addr(r32(c, S1)).sw(c, c.g[S0][0]);
        c.g[A2][0] = 0x2000_0000;
        c.g[A3][0] = 0;
        c.g[V0][0] = Addr(r32(c, S2).wrapping_add(4)).lw(c);
        Addr(frame.at(c, 0x10)).sw(c, c.g[V0][0]);
        lobby_man_on_rank_reply_node_splice(c, frame);
        Ok(())
    })
}

/// The node allocation + splice reached from `LobbyMan_OnRankReply`
/// (0x260c68) when the reply is not a rank result: allocate the 12-byte
/// node with whatever argument registers the head left, then splice.
/// 6 cycles plus the allocation, then the shared splice.
fn lobby_man_on_rank_reply_node_splice(c: &mut Cpu, frame: Frame) {
    guest(c, |c| {
        c.cycles += 6;
        call_ptr(c, callee::MEM_ALLOC_NAMED, 0x0026_0df8)?;
        c.g[A1][0] = c.g[V0][0];
        c.g[V0][0] = sx(r32(c, A1).wrapping_add(8));
        if c.g[V0][0] != 0 {
            Addr(r32(c, A1).wrapping_add(8)).sw(c, c.g[S1][0]);
        }
        lobby_man_on_rank_reply_splice(c, frame);
        Ok(())
    })
}

/// The shared splice + epilogue of `LobbyMan_OnRankReply` (0x260c68):
/// splice the node in `a1` after the head saved at `sp+0x10`, restore
/// s0-s5/ra and return. 18 cycles.
fn lobby_man_on_rank_reply_splice(c: &mut Cpu, frame: Frame) {
    c.cycles += 18;
    c.g[V0][0] = Addr(frame.at(c, 0x10)).lw(c);
    Addr(r32(c, A1)).sw(c, c.g[V0][0]);
    c.g[V1][0] = Addr(r32(c, V0).wrapping_add(4)).lw(c);
    Addr(r32(c, A1).wrapping_add(4)).sw(c, c.g[V1][0]);
    c.g[A0][0] = Addr(r32(c, V0).wrapping_add(4)).lw(c);
    Addr(r32(c, A0)).sw(c, c.g[A1][0]);
    c.g[V0][0] = Addr(frame.at(c, 0x10)).lw(c);
    Addr(r32(c, V0).wrapping_add(4)).sw(c, c.g[A1][0]);
    Addr(frame.at(c, 0)).sw(c, c.g[A1][0]);
    frame.restore(c, S0, 0x80);
    frame.restore(c, S1, 0x70);
    frame.restore(c, S2, 0x60);
    frame.restore(c, S3, 0x50);
    frame.restore(c, S4, 0x40);
    frame.restore(c, S5, 0x30);
    frame.restore_ra(c, 0x20);
    frame.close(c);
}

/// `void LobbyMan_OnChalReply(LobbyMan* self, int* msg)` (0x25f8a0): read
/// the reply string tag; if msg[2] is non-zero and the state is 0x15,
/// abort the challenge and post a kind-0x102 event carrying msg[2];
/// otherwise compare the string against the three known replies and, for
/// the third, lock the challenge (state 0x18, timer 20.0f). 20 (+3+30 /
/// +6+4+6 / +5+5+4+6) + 5 cycles plus the calls.
pub fn lobby_man_on_chal_reply(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x70);
        'out: {
            'mid: {
                'have_code: {
                    c.cycles += 20;
                    frame.save(c, S0, 0x60);
                    frame.save(c, S2, 0x40);
                    c.g[S0][0] = c.g[A0][0];
                    frame.save(c, S1, 0x50);
                    c.g[S2][0] = c.g[A1][0];
                    frame.save_ra(c, 0x30);
                    c.g[A1][0] = sx(0x004a_3220);
                    c.g[A0][0] = Addr(r32(c, S2).wrapping_add(0xc)).lw(c);
                    call_ptr(c, callee::TAG_FIELD_FIND, 0x0025_f8cc)?;
                    c.g[A3][0] = sx(0x004a_2f40);
                    c.g[A0][0] = c.g[V0][0];
                    c.g[A1][0] = c.g[SP][0];
                    c.g[A2][0] = 0x10;
                    call_ptr(c, callee::TAG_FIELD_GET_STRING4, 0x0025_f8e4)?;
                    c.g[V1][0] = Addr(r32(c, S2).wrapping_add(8)).lw(c);
                    let code_zero = c.g[V1][0] == 0;
                    c.g[V0][0] = 0x15;
                    if code_zero {
                        break 'have_code;
                    }
                    c.cycles += 3;
                    c.g[V1][0] = Addr(r32(c, S0)).lw(c);
                    if c.g[V1][0] != c.g[V0][0] {
                        break 'out;
                    }
                    {
                        c.cycles += 30;
                        c.g[V0][0] = 1;
                        c.g[A0][0] = c.g[S0][0];
                        Addr(r32(c, S0)).sw(c, c.g[V0][0]);
                        callee::LOBBY_MAN_SEND_ABORT_CHAL.call(c, 0x0025_f90c)?;
                        c.g[A1][0] = sx(ALLOC_TAG_480488);
                        c.g[A0][0] = 0xc;
                        c.g[A2][0] = 0x2000_0000;
                        c.g[A3][0] = 0;
                        call_ptr(c, callee::MEM_ALLOC_NAMED, 0x0025_f924)?;
                        c.g[S1][0] = c.g[V0][0];
                        c.g[A0][0] = Addr(r32(c, S2).wrapping_add(8)).lw(c);
                        c.g[V0][0] = 0x480000;
                        c.g[A2][0] = Addr(r32(c, S0).wrapping_add(0xf0)).lw(c);
                        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1290));
                        c.g[V1][0] = 0x102;
                        Addr(r32(c, S1).wrapping_add(8)).sw(c, c.g[A0][0]);
                        c.g[A1][0] = 0x480000;
                        Addr(r32(c, S1)).sw(c, c.g[V1][0]);
                        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4a8));
                        Addr(r32(c, S1).wrapping_add(4)).sw(c, c.g[V0][0]);
                        c.g[A0][0] = 0xc;
                        Addr(frame.at(c, 0x20)).sw(c, c.g[A2][0]);
                        c.g[A3][0] = 0;
                        c.g[A2][0] = 0x2000_0000;
                        call_ptr(c, callee::MEM_ALLOC_NAMED, 0x0025_f964)?;
                        c.g[A1][0] = c.g[V0][0];
                        c.g[V0][0] = sx(r32(c, A1).wrapping_add(8));
                        if c.g[V0][0] != 0 {
                            Addr(r32(c, A1).wrapping_add(8)).sw(c, c.g[S1][0]);
                        }
                    }
                    c.cycles += 10;
                    c.g[V0][0] = Addr(frame.at(c, 0x20)).lw(c);
                    Addr(r32(c, A1)).sw(c, c.g[V0][0]);
                    c.g[V1][0] = Addr(r32(c, V0).wrapping_add(4)).lw(c);
                    Addr(r32(c, A1).wrapping_add(4)).sw(c, c.g[V1][0]);
                    c.g[A0][0] = Addr(r32(c, V0).wrapping_add(4)).lw(c);
                    Addr(r32(c, A0)).sw(c, c.g[A1][0]);
                    c.g[V0][0] = Addr(frame.at(c, 0x20)).lw(c);
                    Addr(r32(c, V0).wrapping_add(4)).sw(c, c.g[A1][0]);
                    Addr(frame.at(c, 0x10)).sw(c, c.g[A1][0]);
                    break 'mid;
                }
                'cmp1: {
                    c.cycles += 6;
                    c.g[A1][0] = sx(0x004a_3228);
                    c.g[A0][0] = c.g[SP][0];
                    call_ptr(c, callee::STR_CMP_4165A8, 0x0025_f9ac)?;
                    if c.g[V0][0] != 0 {
                        c.g[A1][0] = 0x4a0000;
                        break 'cmp1;
                    }
                    c.cycles += 4;
                    c.g[V1][0] = Addr(r32(c, S0)).lw(c);
                    c.g[V0][0] = 0x15;
                    if c.g[V1][0] != c.g[V0][0] {
                        break 'out;
                    }
                    c.cycles += 6;
                    c.g[A0][0] = c.g[S0][0];
                    c.g[A2][0] = 0x18;
                    c.g[A1][0] = Addr(r32(c, A0).wrapping_add(0x98)).lw(c);
                    callee::LOBBY_MAN_SEND_LOCK_CHAL.call(c, 0x0025_f9d4)?;
                    break 'out;
                }
                c.cycles += 5;
                c.g[A0][0] = c.g[SP][0];
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x3230));
                call_ptr(c, callee::STR_CMP_4165A8, 0x0025_f9e8)?;
                let second_equal = c.g[V0][0] == 0;
                c.g[A0][0] = c.g[SP][0];
                if second_equal {
                    break 'mid;
                }
                c.cycles += 5;
                c.g[A1][0] = sx(0x004a_3238);
                call_ptr(c, callee::STR_CMP_4165A8, 0x0025_f9fc)?;
                if c.g[V0][0] != 0 {
                    break 'out;
                }
                c.cycles += 4;
                c.g[V1][0] = Addr(r32(c, S0)).lw(c);
                c.g[V0][0] = 0x15;
                if c.g[V1][0] != c.g[V0][0] {
                    break 'out;
                }
                c.cycles += 6;
                c.g[AT][0] = 0x41a0_0000;
                c.mtc1(1, 0);
                c.g[V0][0] = 0x18;
                Addr(r32(c, S0)).sw(c, c.g[V0][0]);
                c.swc1(0, r32(c, S0).wrapping_add(0xe0));
                c.swc1(0, r32(c, GP).wrapping_add(0x2064));
            }
            c.cycles += 1;
        }
        c.cycles += 5;
        frame.restore(c, S0, 0x60);
        frame.restore(c, S1, 0x50);
        frame.restore(c, S2, 0x40);
        frame.restore_ra(c, 0x30);
        frame.close(c);
        Ok(())
    })
}

/// `void BuddyMan_OnMessageReceived(BuddyMan* self, int* msg)` (0x261a28):
/// extract the flag/name/text/date tag fields from the message, add the
/// message unless the sender is already known, drop the sender from the
/// roster vector when its roster index is under 31, then post a
/// kind-0x109 event to the lobby list. 38 (+60+6+2+…) + 21 + 10 + 10
/// cycles plus the calls.
pub fn buddy_man_on_message_received(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x1230);
        'epilogue: {
            c.cycles += 38;
            frame.save(c, S0, 0x1220);
            frame.save(c, S2, 0x1200);
            c.g[S0][0] = c.g[A1][0];
            frame.save(c, S3, 0x11f0);
            c.g[S2][0] = c.g[A0][0];
            frame.save(c, S4, 0x11e0);
            c.g[A1][0] = sx(0x004a_3358);
            frame.save(c, S1, 0x1210);
            frame.save(c, S5, 0x11d0);
            frame.save(c, S6, 0x11c0);
            frame.save(c, S7, 0x11b0);
            frame.save_ra(c, 0x11a0);
            c.g[A0][0] = Addr(r32(c, S0).wrapping_add(0xc)).lw(c);
            call_ptr(c, callee::TAG_FIELD_FIND, 0x0026_1a68)?;
            c.g[V1][0] = 0x480000;
            c.g[A1][0] = sx(frame.at(c, 0x1150));
            c.g[S4][0] = sx(0x0048_0e68);
            c.g[A2][0] = 2;
            c.g[A3][0] = c.g[S4][0];
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, callee::TAG_FIELD_GET_STRING4, 0x0026_1a84)?;
            c.g[A0][0] = Addr(r32(c, S0).wrapping_add(0xc)).lw(c);
            c.g[A1][0] = sx(0x004a_3360);
            call_ptr(c, callee::TAG_FIELD_FIND, 0x0026_1a94)?;
            c.g[S3][0] = sx(frame.at(c, 0x10d0));
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = c.g[S3][0];
            c.g[A2][0] = 0x40;
            c.g[A3][0] = c.g[S4][0];
            call_ptr(c, callee::TAG_FIELD_GET_STRING4, 0x0026_1aac)?;
            c.g[A0][0] = c.g[S2][0];
            c.g[A1][0] = c.g[S3][0];
            call_ptr(c, callee::BUDDY_LOOKUP_2623B0, 0x0026_1ab8)?;
            if c.g[V0][0] != 0 {
                break 'epilogue;
            }
            'inner: {
                c.cycles += 60;
                c.g[A0][0] = Addr(r32(c, S0).wrapping_add(0xc)).lw(c);
                c.g[A1][0] = sx(0x004a_3368);
                c.g[S6][0] = 0x480000;
                call_ptr(c, callee::TAG_FIELD_FIND, 0x0026_1ad4)?;
                c.g[S7][0] = 0x480000;
                c.g[S1][0] = sx(frame.at(c, 0x800));
                c.g[A3][0] = sx(0x004a_2ed0);
                c.g[A2][0] = 0x8cf;
                c.g[A1][0] = c.g[S1][0];
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, callee::TAG_FIELD_GET_STRING4, 0x0026_1af4)?;
                c.g[S5][0] = 0x480000;
                c.g[A0][0] = Addr(r32(c, S0).wrapping_add(0xc)).lw(c);
                c.g[A1][0] = sx(0x004a_3370);
                call_ptr(c, callee::TAG_FIELD_FIND, 0x0026_1b08)?;
                c.g[A1][0] = c.g[SP][0];
                c.g[A3][0] = c.g[S4][0];
                c.g[A2][0] = 0x800;
                c.g[A0][0] = c.g[V0][0];
                callee::TAG_FIELD_GET_STRING_UNESCAPED.call(c, 0x0026_1b1c)?;
                c.g[A1][0] = c.g[SP][0];
                c.g[A0][0] = c.g[S1][0];
                call_ptr(c, callee::FUNC_4162D0, 0x0026_1b28)?;
                c.g[A0][0] = Addr(r32(c, S0).wrapping_add(0xc)).lw(c);
                c.g[A1][0] = sx(0x004a_3378);
                call_ptr(c, callee::TAG_FIELD_FIND, 0x0026_1b38)?;
                c.g[A1][0] = sx(frame.at(c, 0x1180));
                c.g[A2][0] = sx(frame.at(c, 0x1184));
                c.g[A3][0] = sx(frame.at(c, 0x1188));
                c.g[T0][0] = sx(frame.at(c, 0x118c));
                c.g[T1][0] = sx(frame.at(c, 0x1190));
                c.g[T2][0] = sx(frame.at(c, 0x1194));
                c.g[T3][0] = 0;
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, callee::TAG_FIELD_GET_DATE, 0x0026_1b5c)?;
                c.g[S0][0] = sx(frame.at(c, 0x1110));
                c.g[A2][0] = Addr(frame.at(c, 0x1184)).lw(c);
                c.g[A3][0] = Addr(frame.at(c, 0x1188)).lw(c);
                c.g[A1][0] = sx(0x0048_0e78);
                c.g[T0][0] = Addr(frame.at(c, 0x1180)).lw(c);
                c.g[T1][0] = Addr(frame.at(c, 0x118c)).lw(c);
                c.g[A0][0] = c.g[S0][0];
                c.g[T2][0] = Addr(frame.at(c, 0x1190)).lw(c);
                call_ptr(c, callee::SPRINTF, 0x0026_1b84)?;
                c.g[A2][0] = c.g[S1][0];
                c.g[A3][0] = c.g[S0][0];
                c.g[A0][0] = c.g[S2][0];
                c.g[A1][0] = c.g[S3][0];
                callee::BUDDY_MAN_ADD_MESSAGE.call(c, 0x0026_1b98)?;
                c.g[A1][0] = c.g[S3][0];
                c.g[A0][0] = c.g[S2][0];
                call_ptr(c, callee::ROSTER_INDEX_262A08, 0x0026_1ba4)?;
                c.g[V0][0] = ((c.g[V0][0] as i64) < 31) as u64;
                let index_small = c.g[V0][0] != 0;
                c.g[A1][0] = sx((c.g[S6][0] as u32).wrapping_add(0x488));
                if index_small {
                    break 'inner;
                }
                c.cycles += 6;
                c.g[V0][0] = Addr(r32(c, S2).wrapping_add(0xb0)).lw(c);
                c.g[V1][0] = Addr(r32(c, S2).wrapping_add(0xac)).lw(c);
                c.g[V0][0] = sx(r32(c, V0).wrapping_sub(r32(c, V1)));
                c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 2) as u32);
                let vec_empty = c.g[V0][0] == 0;
                c.g[S0][0] = 0;
                if vec_empty {
                    break 'inner;
                }
                c.cycles += 2;
                c.g[A0][0] = c.g[S2][0];
                'scan: loop {
                    'iter: {
                        c.cycles += 4;
                        c.g[A1][0] = c.g[S0][0];
                        call_ptr(c, callee::ROSTER_ENTRY_263630, 0x0026_1bd8)?;
                        if c.g[V0][0] == 0 {
                            c.g[V0][0] = Addr(r32(c, S2).wrapping_add(0xb0)).lw(c);
                            break 'iter;
                        }
                        c.cycles += 5;
                        c.g[A0][0] = Addr(r32(c, V0)).lw(c);
                        c.g[A1][0] = sx(frame.at(c, 0x10d0));
                        call_ptr(c, callee::STR_CMP_4165A8, 0x0026_1bec)?;
                        if c.g[V0][0] != 0 {
                            c.g[V0][0] = Addr(r32(c, S2).wrapping_add(0xb0)).lw(c);
                            break 'iter;
                        }
                        c.cycles += 5;
                        c.g[A0][0] = c.g[S2][0];
                        c.g[A1][0] = c.g[S0][0];
                        call_ptr(c, callee::ROSTER_REMOVE_2635A8, 0x0026_1c00)?;
                        c.g[A1][0] = sx((c.g[S6][0] as u32).wrapping_add(0x488));
                        break 'inner;
                    }
                    c.cycles += 7;
                    c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
                    c.g[V1][0] = Addr(r32(c, S2).wrapping_add(0xac)).lw(c);
                    c.g[V0][0] = sx(r32(c, V0).wrapping_sub(r32(c, V1)));
                    c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 2) as u32);
                    c.g[V0][0] = (c.g[S0][0] < c.g[V0][0]) as u64;
                    let more = c.g[V0][0] != 0;
                    c.g[A0][0] = c.g[S2][0];
                    if more {
                        if event_due(c) {
                            deliver_event_here(c, 0x0026_1bd0);
                        }
                        continue 'scan;
                    }
                    c.cycles += 1;
                    c.g[A1][0] = sx((c.g[S6][0] as u32).wrapping_add(0x488));
                    break 'inner;
                }
            }
            {
                c.cycles += 21;
                c.g[S1][0] = Addr(gp_addr(c, LOBBY_INSTANCE_GP)).lw(c);
                c.g[A0][0] = 8;
                c.g[A2][0] = 0x2000_0000;
                c.g[A3][0] = 0;
                c.g[S0][0] = 0x109;
                call_ptr(c, callee::MEM_ALLOC_NAMED, 0x0026_1c40)?;
                c.g[S2][0] = c.g[V0][0];
                c.g[A1][0] = sx((c.g[S5][0] as u32).wrapping_add(0x4a8));
                c.g[V0][0] = sx((c.g[S7][0] as u32).wrapping_add(0x12b0));
                Addr(r32(c, S2)).sw(c, c.g[S0][0]);
                Addr(r32(c, S2).wrapping_add(4)).sw(c, c.g[V0][0]);
                c.g[A0][0] = 0xc;
                c.g[A2][0] = 0x2000_0000;
                c.g[A3][0] = 0;
                c.g[V0][0] = Addr(r32(c, S1).wrapping_add(0xf0)).lw(c);
                Addr(frame.at(c, 0x1170)).sw(c, c.g[V0][0]);
                call_ptr(c, callee::MEM_ALLOC_NAMED, 0x0026_1c6c)?;
                c.g[A1][0] = c.g[V0][0];
                c.g[V0][0] = sx(r32(c, A1).wrapping_add(8));
                if c.g[V0][0] != 0 {
                    Addr(r32(c, A1).wrapping_add(8)).sw(c, c.g[S2][0]);
                }
            }
            c.cycles += 10;
            c.g[V0][0] = Addr(frame.at(c, 0x1170)).lw(c);
            Addr(r32(c, A1)).sw(c, c.g[V0][0]);
            c.g[V1][0] = Addr(r32(c, V0).wrapping_add(4)).lw(c);
            Addr(r32(c, A1).wrapping_add(4)).sw(c, c.g[V1][0]);
            c.g[A0][0] = Addr(r32(c, V0).wrapping_add(4)).lw(c);
            Addr(r32(c, A0)).sw(c, c.g[A1][0]);
            c.g[V0][0] = Addr(frame.at(c, 0x1170)).lw(c);
            Addr(r32(c, V0).wrapping_add(4)).sw(c, c.g[A1][0]);
            Addr(frame.at(c, 0x1160)).sw(c, c.g[A1][0]);
        }
        c.cycles += 10;
        frame.restore(c, S0, 0x1220);
        frame.restore(c, S1, 0x1210);
        frame.restore(c, S2, 0x1200);
        frame.restore(c, S3, 0x11f0);
        frame.restore(c, S4, 0x11e0);
        frame.restore(c, S5, 0x11d0);
        frame.restore(c, S6, 0x11c0);
        frame.restore(c, S7, 0x11b0);
        frame.restore_ra(c, 0x11a0);
        frame.close(c);
        Ok(())
    })
}

/// `void BuddyMan_AddMessageSendersToRoster()` (0x2610d0): walk the buddy
/// roster (`func_00262938` count, `func_00263630` entry); for each entry
/// whose `func_00262468` lookup fails, add it with `func_00261EF8`.
/// Dispatcher-shaped: re-enters at the loop head 0x2610E8 after an unwind,
/// with s0 = current entry and s1 = index in the registers.
pub fn buddy_man_add_message_senders_to_roster(c: &mut Cpu) {
    let mut pc: u32 = c.pc;
    loop {
        match pc {
            0x0026_10d0 => {
                c.cycles += 6;
                let frame = Frame::open(c, 0x40);
                frame.save(c, S1, 0x20);
                frame.save(c, S0, 0x30);
                c.g[S1][0] = 0;
                frame.save_ra(c, 0x10);
                pc = 0x0026_11e0;
            }
            0x0026_10e8 => {
                c.cycles += 2;
                if c.g[V0][0] != 0 {
                    pc = 0x0026_10f8;
                } else {
                    pc = 0x0026_10f0;
                }
            }
            0x0026_10f0 => {
                c.cycles += 2;
                if callee::BUDDY_MAN_CREATE.call(c, 0x0026_10f8).is_err() {
                    return;
                }
                pc = 0x0026_10f8;
            }
            0x0026_10f8 => {
                c.cycles += 4;
                c.g[A0][0] = Addr(gp_addr(c, 0x238)).lw(c);
                if callee::BUDDY_MAN_IS_CONNECTED.call(c, 0x0026_1100).is_err() {
                    return;
                }
                let connected = c.g[V0][0] != 0;
                c.g[V0][0] = Addr(gp_addr(c, 0x238)).lw(c);
                if connected {
                    pc = 0x0026_1114;
                } else {
                    pc = 0x0026_1108;
                }
            }
            0x0026_1108 => {
                c.cycles += 3;
                c.g[A0][0] = Addr(gp_addr(c, 0x238)).lw(c);
                if callee::BUDDY_MAN_RECONNECT.call(c, 0x0026_1110).is_err() {
                    return;
                }
                c.g[V0][0] = Addr(gp_addr(c, 0x238)).lw(c);
                pc = 0x0026_1114;
            }
            0x0026_1114 => {
                c.cycles += 2;
                let mgr = c.g[V0][0] != 0;
                c.g[A0][0] = Addr(gp_addr(c, 0x238)).lw(c);
                if mgr {
                    pc = 0x0026_1128;
                } else {
                    pc = 0x0026_111c;
                }
            }
            0x0026_111c => {
                c.cycles += 3;
                c.g[A0][0] = Addr(gp_addr(c, LOBBY_INSTANCE_GP)).lw(c);
                if callee::LOBBY_MAN_DISCONNECT.call(c, 0x0026_1124).is_err() {
                    return;
                }
                c.g[A0][0] = Addr(gp_addr(c, 0x238)).lw(c);
                pc = 0x0026_1128;
            }
            0x0026_1128 => {
                c.cycles += 6;
                c.g[A1][0] = c.g[S1][0];
                if call_ptr(c, callee::ROSTER_ENTRY_263630, 0x0026_1130).is_err() {
                    return;
                }
                c.g[S0][0] = c.g[V0][0];
                c.g[V0][0] = Addr(gp_addr(c, 0x238)).lw(c);
                if c.g[V0][0] != 0 {
                    pc = 0x0026_1148;
                } else {
                    pc = 0x0026_1140;
                }
            }
            0x0026_1140 => {
                c.cycles += 2;
                if callee::BUDDY_MAN_CREATE.call(c, 0x0026_1148).is_err() {
                    return;
                }
                pc = 0x0026_1148;
            }
            0x0026_1148 => {
                c.cycles += 4;
                c.g[A0][0] = Addr(gp_addr(c, 0x238)).lw(c);
                if callee::BUDDY_MAN_IS_CONNECTED.call(c, 0x0026_1150).is_err() {
                    return;
                }
                let connected = c.g[V0][0] != 0;
                c.g[V0][0] = Addr(gp_addr(c, 0x238)).lw(c);
                if connected {
                    pc = 0x0026_1164;
                } else {
                    pc = 0x0026_1158;
                }
            }
            0x0026_1158 => {
                c.cycles += 3;
                c.g[A0][0] = Addr(gp_addr(c, 0x238)).lw(c);
                if callee::BUDDY_MAN_RECONNECT.call(c, 0x0026_1160).is_err() {
                    return;
                }
                c.g[V0][0] = Addr(gp_addr(c, 0x238)).lw(c);
                pc = 0x0026_1164;
            }
            0x0026_1164 => {
                c.cycles += 2;
                let mgr = c.g[V0][0] != 0;
                c.g[A0][0] = Addr(gp_addr(c, 0x238)).lw(c);
                if mgr {
                    pc = 0x0026_1178;
                } else {
                    pc = 0x0026_116c;
                }
            }
            0x0026_116c => {
                c.cycles += 3;
                c.g[A0][0] = Addr(gp_addr(c, LOBBY_INSTANCE_GP)).lw(c);
                if callee::LOBBY_MAN_DISCONNECT.call(c, 0x0026_1174).is_err() {
                    return;
                }
                c.g[A0][0] = Addr(gp_addr(c, 0x238)).lw(c);
                pc = 0x0026_1178;
            }
            0x0026_1178 => {
                c.cycles += 4;
                c.g[A1][0] = Addr(r32(c, S0)).lw(c);
                if call_ptr(c, callee::FUNC_262468, 0x0026_1180).is_err() {
                    return;
                }
                if (c.g[V0][0] as i64) >= 0 {
                    c.g[S1][0] = sx(r32(c, S1).wrapping_add(1));
                    pc = 0x0026_11e0;
                } else {
                    pc = 0x0026_1188;
                }
            }
            0x0026_1188 => {
                c.cycles += 3;
                c.g[V0][0] = Addr(gp_addr(c, 0x238)).lw(c);
                if c.g[V0][0] != 0 {
                    pc = 0x0026_119c;
                } else {
                    pc = 0x0026_1194;
                }
            }
            0x0026_1194 => {
                c.cycles += 2;
                if callee::BUDDY_MAN_CREATE.call(c, 0x0026_119c).is_err() {
                    return;
                }
                pc = 0x0026_119c;
            }
            0x0026_119c => {
                c.cycles += 4;
                c.g[A0][0] = Addr(gp_addr(c, 0x238)).lw(c);
                if callee::BUDDY_MAN_IS_CONNECTED.call(c, 0x0026_11a4).is_err() {
                    return;
                }
                let connected = c.g[V0][0] != 0;
                c.g[V0][0] = Addr(gp_addr(c, 0x238)).lw(c);
                if connected {
                    pc = 0x0026_11b8;
                } else {
                    pc = 0x0026_11ac;
                }
            }
            0x0026_11ac => {
                c.cycles += 3;
                c.g[A0][0] = Addr(gp_addr(c, 0x238)).lw(c);
                if callee::BUDDY_MAN_RECONNECT.call(c, 0x0026_11b4).is_err() {
                    return;
                }
                c.g[V0][0] = Addr(gp_addr(c, 0x238)).lw(c);
                pc = 0x0026_11b8;
            }
            0x0026_11b8 => {
                c.cycles += 2;
                if c.g[V0][0] != 0 {
                    c.g[A1][0] = Addr(r32(c, S0)).lw(c);
                    pc = 0x0026_11cc;
                } else {
                    pc = 0x0026_11c0;
                }
            }
            0x0026_11c0 => {
                c.cycles += 3;
                c.g[A0][0] = Addr(gp_addr(c, LOBBY_INSTANCE_GP)).lw(c);
                if callee::LOBBY_MAN_DISCONNECT.call(c, 0x0026_11c8).is_err() {
                    return;
                }
                c.g[A1][0] = Addr(r32(c, S0)).lw(c);
                pc = 0x0026_11cc;
            }
            0x0026_11cc => {
                c.cycles += 5;
                c.g[A2][0] = 2;
                c.g[A0][0] = Addr(gp_addr(c, 0x238)).lw(c);
                c.g[A3][0] = c.g[SP][0];
                if call_ptr(c, callee::FUNC_261EF8, 0x0026_11dc).is_err() {
                    return;
                }
                c.g[S1][0] = sx(r32(c, S1).wrapping_add(1));
                pc = 0x0026_11e0;
            }
            0x0026_11e0 => {
                c.cycles += 3;
                c.g[V0][0] = Addr(gp_addr(c, 0x238)).lw(c);
                if c.g[V0][0] != 0 {
                    pc = 0x0026_11f4;
                } else {
                    pc = 0x0026_11ec;
                }
            }
            0x0026_11ec => {
                c.cycles += 2;
                if callee::BUDDY_MAN_CREATE.call(c, 0x0026_11f4).is_err() {
                    return;
                }
                pc = 0x0026_11f4;
            }
            0x0026_11f4 => {
                c.cycles += 4;
                c.g[A0][0] = Addr(gp_addr(c, 0x238)).lw(c);
                if callee::BUDDY_MAN_IS_CONNECTED.call(c, 0x0026_11fc).is_err() {
                    return;
                }
                let connected = c.g[V0][0] != 0;
                c.g[V0][0] = Addr(gp_addr(c, 0x238)).lw(c);
                if connected {
                    pc = 0x0026_1210;
                } else {
                    pc = 0x0026_1204;
                }
            }
            0x0026_1204 => {
                c.cycles += 3;
                c.g[A0][0] = Addr(gp_addr(c, 0x238)).lw(c);
                if callee::BUDDY_MAN_RECONNECT.call(c, 0x0026_120c).is_err() {
                    return;
                }
                c.g[V0][0] = Addr(gp_addr(c, 0x238)).lw(c);
                pc = 0x0026_1210;
            }
            0x0026_1210 => {
                c.cycles += 2;
                let mgr = c.g[V0][0] != 0;
                if mgr {
                    pc = 0x0026_1220;
                } else {
                    pc = 0x0026_1218;
                }
            }
            0x0026_1218 => {
                c.cycles += 2;
                c.g[A0][0] = Addr(gp_addr(c, LOBBY_INSTANCE_GP)).lw(c);
                if callee::LOBBY_MAN_DISCONNECT.call(c, 0x0026_1220).is_err() {
                    return;
                }
                pc = 0x0026_1220;
            }
            0x0026_1220 => {
                c.cycles += 5;
                c.g[A0][0] = Addr(gp_addr(c, 0x238)).lw(c);
                if call_ptr(c, callee::ROSTER_COUNT_262938, 0x0026_1228).is_err() {
                    return;
                }
                c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[V0][0] as i64)) as u64;
                if c.g[V0][0] != 0 {
                    c.g[V0][0] = Addr(gp_addr(c, 0x238)).lw(c);
                    if back_edge_unwinds(c, 0x0026_10e8) {
                        return;
                    }
                    pc = 0x0026_10e8;
                } else {
                    pc = 0x0026_1234;
                }
            }
            0x0026_1234 => {
                c.cycles += 5;
                let frame = Frame { size: 0x40 };
                frame.restore(c, S0, 0x30);
                frame.restore(c, S1, 0x20);
                frame.restore_ra(c, 0x10);
                frame.close(c);
                return;
            }
            _ => {
                c.jump_out(pc, 0x0026_10d0, 0x0026_1248);
                return;
            }
        }
    }
}

/// `void BuddyMan_OnRosterAddReply()` (0x2618c0): post a kind-0x10A
/// roster-add event: allocate the 8-byte event (kind 0x10A, vtable
/// `D_004812B0`), wrap it in a 12-byte list node and splice the node into
/// the message list headed at lobby+0xF0. 29 + 14 cycles plus the two
/// allocations. (Sibling of the remove reply: kind and return addresses
/// differ, nothing else.)
pub fn buddy_man_on_roster_add_reply(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 29;
        let frame = Frame::open(c, 0x50);
        c.g[A1][0] = sx(ALLOC_TAG_480488);
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        c.g[A0][0] = 8;
        frame.save_ra(c, 0x20);
        c.g[A2][0] = 0x2000_0000;
        c.g[S0][0] = Addr(gp_addr(c, LOBBY_INSTANCE_GP)).lw(c);
        c.g[A3][0] = 0;
        call_ptr(c, callee::MEM_ALLOC_NAMED, 0x0026_18ec)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[V1][0] = 0x10a;
        c.g[V0][0] = 0x480000;
        Addr(r32(c, S1)).sw(c, c.g[V1][0]);
        c.g[V0][0] = sx(EVENT_VT_4812B0);
        c.g[A1][0] = 0x480000;
        Addr(r32(c, S1).wrapping_add(4)).sw(c, c.g[V0][0]);
        c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4a8));
        c.g[A0][0] = 0xc;
        c.g[A2][0] = 0x2000_0000;
        c.g[V0][0] = Addr(r32(c, S0).wrapping_add(0xf0)).lw(c);
        c.g[A3][0] = 0;
        Addr(frame.at(c, 0x10)).sw(c, c.g[V0][0]);
        call_ptr(c, callee::MEM_ALLOC_NAMED, 0x0026_1924)?;
        c.g[A1][0] = c.g[V0][0];
        c.g[V0][0] = sx(r32(c, A1).wrapping_add(8));
        if c.g[V0][0] != 0 {
            Addr(r32(c, A1).wrapping_add(8)).sw(c, c.g[S1][0]);
        }
        c.cycles += 14;
        c.g[V0][0] = Addr(frame.at(c, 0x10)).lw(c);
        frame.restore(c, S0, 0x40);
        Addr(r32(c, A1)).sw(c, c.g[V0][0]);
        frame.restore(c, S1, 0x30);
        c.g[V1][0] = Addr(r32(c, V0).wrapping_add(4)).lw(c);
        frame.restore_ra(c, 0x20);
        Addr(r32(c, A1).wrapping_add(4)).sw(c, c.g[V1][0]);
        c.g[A0][0] = Addr(r32(c, V0).wrapping_add(4)).lw(c);
        Addr(r32(c, A0)).sw(c, c.g[A1][0]);
        c.g[V0][0] = Addr(frame.at(c, 0x10)).lw(c);
        Addr(r32(c, V0).wrapping_add(4)).sw(c, c.g[A1][0]);
        Addr(frame.at(c, 0)).sw(c, c.g[A1][0]);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::events::{check_events, driven, schedule};
    use crate::difftest::{check, scramble, Rng, DATA, GAME_GP};
    use crate::game::be::beintstat::tests::allocator;
    use crate::game::dirtysock::tags::tests::put_str;

    driven!(
        gen_buddy_man_on_message_received,
        ee::s06::f_00261a28__generated
    );
    driven!(
        hand_buddy_man_on_message_received,
        buddy_man_on_message_received
    );
    driven!(
        gen_buddy_man_add_message_senders_to_roster,
        ee::s06::f_002610d0__generated
    );
    driven!(
        hand_buddy_man_add_message_senders_to_roster,
        buddy_man_add_message_senders_to_roster
    );
    const LOBBY: u32 = DATA + 0x1000;
    const HEAD: u32 = DATA + 0x1800;
    const SELF: u32 = DATA + 0x2000;
    const MSG: u32 = DATA + 0x2800;
    const STR: u32 = DATA + 0x3000;

    /// Common machine state: game `gp`, BELibrary dummy at `gp-0x1f04`,
    /// world dummy at `gp-0x848`, lobby instance at `gp-0xc8` with a
    /// self-linked message-list head at +0xF0, and the pool/DLHeap
    /// allocator the event posters allocate from.
    fn base(c: &mut Cpu, seed: u32, salt: u64, entry: u32) -> Rng {
        let mut r = Rng(salt ^ seed as u64);
        scramble(c, &mut r);
        c.g[GP][0] = GAME_GP as u64;
        c.pc = entry;
        c.bus.write32(GAME_GP - 0x1f04, DATA + 0x9000);
        c.bus.write32(GAME_GP - 0x848, DATA + 0x9800);
        c.bus.write32(DATA + 0x9800 + 0x78, DATA + 0xa000);
        c.bus.write32(GAME_GP - 0xc8, LOBBY);
        c.bus.write32(LOBBY + 0xf0, HEAD);
        c.bus.write32(HEAD, HEAD);
        c.bus.write32(HEAD + 4, HEAD);
        c.bus.write32(LOBBY + 0xec, HEAD);
        allocator(c);
        c.g[GP][0] = GAME_GP as u64;
        c.pc = entry;
        c.g[RA][0] = crate::difftest::RA as u64;
        r
    }

    #[test]
    fn buddy_man_on_message_received_matches() {
        check(
            ee::s06::f_00261a28__generated,
            buddy_man_on_message_received,
            90,
            &|c, seed| {
                let _r = base(c, seed, 0x261a_2800, 0x0026_1a28);
                let tags = DATA + 0x4000;
                c.g[A0][0] = SELF as u64;
                c.g[A1][0] = MSG as u64;
                c.bus.write32(MSG + 0xc, tags);
                for i in 0..16u32 {
                    c.bus.write32(tags + i * 4, 0);
                }
                let begin = DATA + 0x5000;
                let n = (seed % 5) as u32;
                c.bus.write32(SELF + 0xac, begin);
                c.bus.write32(SELF + 0xb0, begin + 4 * n);
                for i in 0..n {
                    let entry = DATA + 0x6000 + 0x100 * i;
                    c.bus.write32(begin + 4 * i, entry);
                    c.bus.write32(entry, STR + 0x100 * i);
                    put_str(c, STR + 0x100 * i, b"buddy\0");
                }
            },
        );
    }

    #[test]
    fn buddy_man_on_message_received_events() {
        check_events(
            gen_buddy_man_on_message_received,
            hand_buddy_man_on_message_received,
            60,
            &|c, seed| {
                let mut r = base(c, seed, 0x261a_2900, 0x0026_1a28);
                let tags = DATA + 0x4000;
                c.g[A0][0] = SELF as u64;
                c.g[A1][0] = MSG as u64;
                c.bus.write32(MSG + 0xc, tags);
                for i in 0..16u32 {
                    c.bus.write32(tags + i * 4, 0);
                }
                let begin = DATA + 0x5000;
                let n = 1 + (seed % 3) as u32;
                c.bus.write32(SELF + 0xac, begin);
                c.bus.write32(SELF + 0xb0, begin + 4 * n);
                for i in 0..n {
                    let entry = DATA + 0x6000 + 0x100 * i;
                    c.bus.write32(begin + 4 * i, entry);
                    c.bus.write32(entry, STR + 0x100 * i);
                    put_str(c, STR + 0x100 * i, b"buddy\0");
                }
                let a0 = c.g[A0][0];
                let a1 = c.g[A1][0];
                let mode = schedule(c, &mut r, seed, 0x0026_1a28);
                c.g[A0][0] = a0;
                c.g[A1][0] = a1;
                mode
            },
        );
    }

    #[test]
    fn buddy_man_on_roster_remove_reply_matches() {
        check(
            ee::s06::f_00261970__generated,
            buddy_man_on_roster_remove_reply,
            100,
            &|c, seed| {
                let _r = base(c, seed, 0x2619_7000, 0x0026_1970);
                if seed % 5 == 0 {
                    let other = DATA + 0x1c00;
                    c.bus.write32(HEAD + 4, other);
                    c.bus.write32(other, HEAD);
                    c.bus.write32(other + 4, HEAD);
                }
            },
        );
    }

    #[test]
    fn buddy_man_on_roster_add_reply_matches() {
        check(
            ee::s06::f_002618c0__generated,
            buddy_man_on_roster_add_reply,
            100,
            &|c, seed| {
                let _r = base(c, seed, 0x2618_c000, 0x0026_18c0);
                if seed % 7 == 0 {
                    let other = DATA + 0x1c00;
                    c.bus.write32(HEAD + 4, other);
                    c.bus.write32(other, HEAD);
                    c.bus.write32(other + 4, HEAD);
                }
            },
        );
    }

    #[test]
    fn buddy_man_on_roster_event_matches() {
        check(
            ee::s06::f_00261770__generated,
            buddy_man_on_roster_event,
            210,
            &|c, seed| {
                let _r = base(c, seed, 0x2617_7000, 0x0026_1770);
                c.g[A0][0] = SELF as u64;
                c.g[A1][0] = STR as u64;
                match seed % 7 {
                    0 => {
                        put_str(c, STR, b"endr\0");
                        c.bus.write32(SELF + 0xe8, 0);
                    }
                    1 => {
                        put_str(c, STR, b"endr\0");
                        c.bus.write32(SELF + 0xe8, 1);
                    }
                    _ => {
                        put_str(c, STR, b"join\0");
                        c.bus.write32(SELF + 0xe8, (seed % 2) as u32);
                    }
                }
                let mgr = DATA + 0x7000;
                c.bus.write32(GAME_GP + 0x238, mgr);
                c.bus.write32(mgr + 0xac, DATA + 0x7800);
                c.bus.write32(mgr + 0xb0, DATA + 0x7800);
                c.bus.write32(mgr + 0xc8, 1);
                c.bus.write32(mgr + 0xd8, 1);
                c.bus.write32(mgr + 0xe0, 1);
            },
        );
    }

    #[test]
    fn buddy_man_on_connect_event_matches() {
        check(
            ee::s06::f_002615c8__generated,
            buddy_man_on_connect_event,
            220,
            &|c, seed| {
                let _r = base(c, seed, 0x2615_c800, 0x0026_15c8);
                c.g[A0][0] = SELF as u64;
                c.g[A1][0] = MSG as u64;
                match seed % 11 {
                    0 => put_str(c, MSG, b"auth\0"),
                    1..=5 => put_str(c, MSG, b"conn\0"),
                    _ => put_str(c, MSG, b"xxxx\0"),
                }
                c.bus.write32(MSG + 4, (seed % 11) as u32);
                c.bus.write32(MSG + 8, if seed % 3 == 0 { 0 } else { 7 });
                c.bus.write32(SELF + 0xc8, DATA + 0x7000);
                c.bus
                    .write32(SELF + 0xd0, if seed % 5 == 0 { 0 } else { 3 });
                c.bus.write32(SELF + 0xd8, 9);
                c.bus.write32(SELF + 0xdc, 9);
                c.bus.write32(SELF + 0xe0, 9);
                c.bus.write32(SELF + 0xe8, 9);
            },
        );
    }

    #[test]
    fn buddy_man_connect_matches() {
        check(
            ee::s06::f_00261248__generated,
            buddy_man_connect,
            130,
            &|c, seed| {
                let _r = base(c, seed, 0x2612_4800, 0x0026_1248);
                c.g[A0][0] = SELF as u64;
                put_str(c, STR, b"name\0");
                put_str(c, STR + 0x100, b"a3\0");
                put_str(c, STR + 0x200, b"a4\0");
                put_str(c, STR + 0x300, b"a5\0");
                c.g[A1][0] = STR as u64;
                c.g[A2][0] = (seed % 13) as u64;
                c.g[A3][0] = (STR + 0x100) as u64;
                c.g[T0][0] = (STR + 0x200) as u64;
                c.g[T1][0] = (STR + 0x300) as u64;
                c.bus
                    .write32(SELF + 0xc8, if seed % 13 == 0 { 0 } else { DATA + 0x7000 });
                c.bus.write32(GAME_GP + 0xda0, DATA + 0x7400);
            },
        );
    }

    #[test]
    fn buddy_man_add_message_senders_to_roster_matches() {
        check(
            ee::s06::f_002610d0__generated,
            buddy_man_add_message_senders_to_roster,
            90,
            &|c, seed| {
                let _r = base(c, seed, 0x2610_d000, 0x0026_10d0);
                roster_fixture(c, seed);
            },
        );
    }

    #[test]
    fn buddy_man_add_message_senders_to_roster_events() {
        check_events(
            gen_buddy_man_add_message_senders_to_roster,
            hand_buddy_man_add_message_senders_to_roster,
            90,
            &|c, seed| {
                let mut r = base(c, seed, 0x2610_d100, 0x0026_10d0);
                roster_fixture(c, seed);
                schedule(c, &mut r, seed, 0x0026_10d0)
            },
        );
    }

    /// Buddy-manager fixture for the roster walk: `seed%4` entries in the
    /// vector at mgr+0xAC..0xB0, each pointing at a name object.
    fn roster_fixture(c: &mut Cpu, seed: u32) {
        let mgr = DATA + 0x7000;
        let begin = DATA + 0x7800;
        let n = (seed % 4) as u32;
        c.bus.write32(GAME_GP + 0x238, mgr);
        c.bus.write32(mgr + 0xac, begin);
        c.bus.write32(mgr + 0xb0, begin + 4 * n);
        c.bus.write32(mgr + 0xc8, 1);
        c.bus.write32(mgr + 0xd8, 2);
        c.bus.write32(mgr + 0xe0, 1);
        for i in 0..n {
            let entry = DATA + 0x6000 + 0x100 * i;
            c.bus.write32(begin + 4 * i, entry);
            c.bus.write32(entry, STR + 0x100 * i);
            put_str(c, STR + 0x100 * i, b"buddy\0");
        }
        c.bus.write32(LOBBY, 1);
        c.bus.write32(LOBBY + 0x54, 0);
    }

    #[test]
    fn lobby_man_on_rank_reply_matches() {
        check(
            ee::s06::f_00260c68__generated,
            lobby_man_on_rank_reply,
            170,
            &|c, seed| {
                let _r = base(c, seed, 0x260c_6800, 0x0026_0c68);
                c.g[A0][0] = SELF as u64;
                c.g[A1][0] = MSG as u64;
                let code = match seed % 17 {
                    0..=5 => 0,
                    6..=11 => 0x7261_6e6b,
                    _ => 0x1234_5678,
                };
                c.bus.write32(MSG + 8, code);
                c.bus.write32(0x0053_4b34, (seed % 2) as u32);
                c.bus.write32(SELF + 0xf0, HEAD);
                c.bus.write32(SELF + 0xec, HEAD);
            },
        );
    }

    #[test]
    fn lobby_man_on_chal_reply_matches() {
        check(
            ee::s06::f_0025f8a0__generated,
            lobby_man_on_chal_reply,
            190,
            &|c, seed| {
                let _r = base(c, seed, 0x25f8_a000, 0x0025_f8a0);
                c.g[A0][0] = SELF as u64;
                c.g[A1][0] = MSG as u64;
                let tags = DATA + 0x4000;
                c.bus.write32(MSG + 0xc, tags);
                for i in 0..16u32 {
                    c.bus.write32(tags + i * 4, 0);
                }
                c.bus.write32(MSG + 8, if seed % 19 < 10 { 5 } else { 0 });
                c.bus.write32(SELF, if seed % 19 < 5 { 0x15 } else { 3 });
                c.bus.write32(SELF + 0x98, 0x18);
                c.bus.write32(SELF + 0xf0, HEAD);
            },
        );
    }

    #[test]
    fn lobby_man_clear_challenge_lock_matches() {
        check(
            ee::s06::f_0025f740__generated,
            lobby_man_clear_challenge_lock,
            230,
            &|c, seed| {
                let _r = base(c, seed, 0x25f7_4000, 0x0025_f740);
                c.g[A0][0] = SELF as u64;
                c.bus.write32(
                    SELF,
                    match seed % 23 {
                        0..=7 => 0,
                        8..=15 => 0x19,
                        _ => 7,
                    },
                );
                c.g[A1][0] = if seed % 23 < 12 { 0 } else { 0x77 };
                c.bus.write32(SELF + 0x54, DATA + 0x7000);
                c.bus.write32(SELF + 0xf0, HEAD);
                put_str(c, DATA + 0x7000, b"chal\0");
            },
        );
    }
}
