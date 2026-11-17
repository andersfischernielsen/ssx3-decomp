//! Staging module oc1000 (wave wave): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1000`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{back_edge_unwinds, deliver_event_here, event_due};
use crate::game::dirtysock::tagsunk::{BUDDY_ROSTER, LOBBY_INSTANCE_GP};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// BuddyMan message vector begin (`self + 0xac`).
pub(crate) const BUDDY_MESSAGES_BEGIN: u32 = 0xac;
/// BuddyMan message vector end (`self + 0xb0`).
pub(crate) const BUDDY_MESSAGES_END: u32 = 0xb0;
/// BuddyMan challenger vector object (`self + 0xb8`).
pub(crate) const BUDDY_CHALLENGERS_VEC: u32 = 0xb8;
/// BuddyMan challenger vector begin (`self + 0xbc`).
pub(crate) const BUDDY_CHALLENGERS_BEGIN: u32 = 0xbc;
/// BuddyMan challenger vector end (`self + 0xc0`).
pub(crate) const BUDDY_CHALLENGERS_END: u32 = 0xc0;
/// BuddyMan session/api object (`self + 0xc8`).
pub(crate) const BUDDY_SESSION: u32 = 0xc8;
/// BuddyMan local name/id pointer (`self + 0xec`).
pub(crate) const BUDDY_NAME_PTR: u32 = 0xec;
/// BuddyMan/session list head pointer (`self + 0xf0`).
pub(crate) const BUDDY_LIST_HEAD: u32 = 0xf0;
/// BuddyMan scalar field read by `BuddyMan_SendMessage` (`self + 0xa4`).
pub(crate) const BUDDY_FIELD_A4: u32 = 0xa4;
/// Roster/message entry flags word (entry `+ 0xa8`).
pub(crate) const BUDDY_ENTRY_FLAGS: u32 = 0xa8;

/// Tag key strings for `BuddyMan_SendMessage` / `BuddyMan_SetPresence`.
pub(crate) const TAG_SEND_A: u32 = 0x004a_3358;
/// Tag value string for `BuddyMan_SendMessage`.
pub(crate) const TAG_SEND_B: u32 = 0x004a_3390;
/// Tag key string for `BuddyMan_SendMessage`.
pub(crate) const TAG_SEND_C: u32 = 0x004a_3368;
/// Tag value string for `BuddyMan_SendMessage`.
pub(crate) const TAG_SEND_D: u32 = 0x004a_2f40;
/// Tag key string for `BuddyMan_SendMessage`.
pub(crate) const TAG_SEND_E: u32 = 0x004a_3360;
/// Tag key string for `BuddyMan_SendMessage`.
pub(crate) const TAG_SEND_F: u32 = 0x004a_3370;
/// Tag key string for `BuddyMan_SendMessage`.
pub(crate) const TAG_SEND_G: u32 = 0x004a_3398;
/// Tag key string for `BuddyMan_SetPresence`.
pub(crate) const TAG_PRES_A: u32 = 0x004a_31f8;
/// Tag key string for `BuddyMan_SetPresence`.
pub(crate) const TAG_PRES_B: u32 = 0x004a_33a8;
/// Tag key string for `BuddyMan_SetPresence`.
pub(crate) const TAG_PRES_C: u32 = 0x004a_33b0;
/// Tag key string for `BuddyMan_SetPresence`.
pub(crate) const TAG_PRES_D: u32 = 0x004a_33b8;
/// Tag key string for `BuddyMan_SetPresence`.
pub(crate) const TAG_PRES_E: u32 = 0x004a_33c0;
/// Tag key string for `BuddyMan_SetPresence`.
pub(crate) const TAG_PRES_F: u32 = 0x004a_33c8;
/// Global string `D_00480EA8` for `BuddyMan_SetPresence`.
pub(crate) const TAG_PRES_GLOBAL: u32 = 0x0048_0ea8;
/// Global string `D_004A2F18` for `BuddyMan_SetPresence`.
pub(crate) const TAG_PRES_STR: u32 = 0x004a_2f18;

/// Named callees for this module (direct `Callee`s and `call_indirect`
/// targets), following `game::dirtysock::tagsunk::callee`.
pub(crate) mod callee {
    use super::ee;
    use super::Callee;

    pub const BUDDY_MAN_IS_TEMP_AT: Callee = Callee {
        addr: 0x0026_2400,
        f: ee::s07::f_00262400,
    };
    pub const BUDDY_MAN_GET_ROSTER_ENTRY: Callee = Callee {
        addr: 0x0026_25c8,
        f: ee::s07::f_002625c8,
    };
    pub const BUDDY_MAN_GET_ROSTER_COUNT: Callee = Callee {
        addr: 0x0026_2598,
        f: ee::s07::f_00262598,
    };
    pub const BUDDY_MAN_FIND_ROSTER_ENTRY: Callee = Callee {
        addr: 0x0026_2468,
        f: ee::s07::f_00262468,
    };
    pub const BUDDY_MAN_IS_BLOCKED_AT: Callee = Callee {
        addr: 0x0026_23e0,
        f: ee::s07::f_002623e0,
    };
    pub const BUDDY_MAN_HAS_MESSAGE_FROM: Callee = Callee {
        addr: 0x0026_2a38,
        f: ee::s07::f_00262a38,
    };
    pub const BUDDY_MAN_GET_MESSAGE_COUNT: Callee = Callee {
        addr: 0x0026_2938,
        f: ee::s07::f_00262938,
    };
    pub const BUDDY_MAN_GET_MESSAGE: Callee = Callee {
        addr: 0x0026_3630,
        f: ee::s07::f_00263630,
    };
    pub const BUDDY_MAN_DELETE_MESSAGE_AT: Callee = Callee {
        addr: 0x0026_35a8,
        f: ee::s07::f_002635a8,
    };
    pub const BUDDY_MAN_HAS_CHALLENGER: Callee = Callee {
        addr: 0x0026_3128,
        f: ee::s07::f_00263128,
    };
    pub const BUDDY_MAN_FIND_CHALLENGER_INDEX: Callee = Callee {
        addr: 0x0026_2e20,
        f: ee::s07::f_00262e20,
    };
    pub const BUDDY_MAN_MAP_PRESENCE_STATE: Callee = Callee {
        addr: 0x0026_28c0,
        f: ee::s07::f_002628c0,
    };
    pub const BUDDY_MAN_ADD_ROSTER_ENTRY: Callee = Callee {
        addr: 0x0026_20d8,
        f: ee::s07::f_002620d8,
    };
    pub const BUDDY_MAN_REMOVE_CHALLENGER: Callee = Callee {
        addr: 0x0026_3018,
        f: ee::s07::f_00263018,
    };
    pub const BUDDY_MAN_REMOVE_TEMP_ENTRY_AT: Callee = Callee {
        addr: 0x0026_2ec0,
        f: ee::s07::f_00262ec0,
    };
    pub const BUDDY_MAN_COUNT_PERMANENT_ENTRIES: Callee = Callee {
        addr: 0x0026_2280,
        f: ee::s07::f_00262280,
    };
    pub const BUDDY_MAN_COUNT_TEMP_ENTRIES: Callee = Callee {
        addr: 0x0026_22f0,
        f: ee::s07::f_002622f0,
    };
    pub const BUDDY_MAN_CHALLENGER_VECTOR_INSERT: Callee = Callee {
        addr: 0x0026_72c8,
        f: ee::s07::f_002672c8,
    };

    /// `strcmp` (exact) through the function table.
    pub const STR_EQ: u32 = 0x0041_65a8;
    /// `strcasecmp`-style compare through the function table.
    pub const STR_CASE_EQ: u32 = 0x0041_aa88;
    /// `strcpy` through the function table.
    pub const STRCPY: u32 = 0x0041_66f4;
    /// `memmove` through the function table.
    pub const MEMMOVE: u32 = 0x0041_610c;
    /// `cBXString` copy constructor through the function table.
    pub const BXSTRING_CTOR2: u32 = 0x0031_82d8;
    /// `cBXString` destructor through the function table.
    pub const BXSTRING_DTOR: u32 = 0x0031_8260;
    /// `cMemMan_alloc` through the function table.
    pub const MEM_ALLOC: u32 = 0x0031_7d70;
    /// `cDirtysock_tag_TagFieldSetString` through the function table.
    pub const TAG_FIELD_SET_STRING: u32 = 0x003e_c180;
    /// `cDirtysock_tag_TagFieldSetNumber` through the function table.
    pub const TAG_FIELD_SET_NUMBER: u32 = 0x003e_be40;
    /// `cDirtysock_tag_TagFieldSetUnk` through the function table.
    pub const TAG_FIELD_SET_UNK: u32 = 0x0025_9288;
    /// Dirtysock session call `func_003F1538` through the function table.
    pub const SESSION_3F1538: u32 = 0x003f_1538;
    /// Dirtysock session call `func_003F1548` through the function table.
    pub const SESSION_3F1548: u32 = 0x003f_1548;
    /// Dirtysock session call `func_003F1578` through the function table.
    pub const SESSION_3F1578: u32 = 0x003f_1578;
    /// Dirtysock session call `func_003F15C8` through the function table.
    pub const SESSION_3F15C8: u32 = 0x003f_15c8;
    /// Dirtysock session call `func_003F16D8` through the function table.
    pub const SESSION_3F16D8: u32 = 0x003f_16d8;
    /// Dirtysock session call `func_003F11F8` through the function table.
    pub const SESSION_3F11F8: u32 = 0x003f_11f8;
    /// Dirtysock session call `func_003F0E38` through the function table.
    pub const SESSION_3F0E38: u32 = 0x003f_0e38;
    /// Dirtysock session call `func_003F14D8` through the function table.
    pub const SESSION_3F14D8: u32 = 0x003f_14d8;
    /// Lobby decline sender `func_0025FC28` through the function table.
    pub const LOBBY_SEND_DECLINE: u32 = 0x0025_fc28;
    /// `BuddyMan_IsBuddy` through the function table.
    pub const IS_BUDDY: u32 = 0x0026_2360;
    /// `BuddyMan_IsBlocked` through the function table.
    pub const IS_BLOCKED: u32 = 0x0026_23b0;
    /// `BuddyMan_IsTemp` through the function table.
    pub const IS_TEMP: u32 = 0x0026_2438;
    /// `BuddyMan_IsTempAt` through the function table.
    pub const IS_TEMP_AT_ADDR: u32 = 0x0026_2400;
    /// `BuddyMan_MessageVectorInsert` through the function table.
    pub const MESSAGE_VECTOR_INSERT: u32 = 0x0026_7128;
}

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0026_2ec0,
        buddy_man_remove_temp_entry_at,
        "BuddyMan_RemoveTempEntryAt",
    ),
    (
        0x0026_2af0,
        buddy_man_on_challenge_received,
        "BuddyMan_OnChallengeReceived",
    ),
    (
        0x0026_2ab0,
        buddy_man_has_pending_challenge_from,
        "BuddyMan_HasPendingChallengeFrom",
    ),
    (
        0x0026_2950,
        buddy_man_count_messages_from_at,
        "BuddyMan_CountMessagesFromAt",
    ),
    (0x0026_2768, buddy_man_set_presence, "BuddyMan_SetPresence"),
    (0x0026_2690, buddy_man_send_message, "BuddyMan_SendMessage"),
    (
        0x0026_24f8,
        buddy_man_find_roster_entry_no_case,
        "BuddyMan_FindRosterEntryNoCase",
    ),
    (
        0x0026_20d8,
        buddy_man_add_roster_entry,
        "BuddyMan_AddRosterEntry",
    ),
    (
        0x0026_1ef8,
        buddy_man_change_roster_entry,
        "BuddyMan_ChangeRosterEntry",
    ),
    (0x0026_1cd0, buddy_man_add_message, "BuddyMan_AddMessage"),
];

/// The lobby singleton from `gp` (`gp - 0xc8`), as left in a register.
fn lobby_instance(c: &mut Cpu) -> u64 {
    Addr((r32(c, GP)).wrapping_add(LOBBY_INSTANCE_GP as u32)).lw(c)
}

/// `bool BuddyMan_HasPendingChallengeFrom(BuddyMan* self, void** nameRef)`
/// (0x262ab0): true when the message list is non-empty and its first entry's
/// name compares equal (exact `strcmp`-style call) to `*nameRef`; the
/// generated body returns 1 for any non-empty list whose compare call
/// returns, and 0 for an empty list.
pub fn buddy_man_has_pending_challenge_from(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        let this = r32(c, A0);
        c.g[V1][0] = Addr(this).off(BUDDY_MESSAGES_BEGIN).lw(c);
        c.g[V0][0] = Addr(this).off(BUDDY_MESSAGES_END).lw(c);
        if c.g[V1][0] != c.g[V0][0] {
            c.g[V0][0] = Addr(r32(c, V1)).lw(c);
            c.cycles += 4;
            c.g[A1][0] = Addr(r32(c, A1)).lw(c);
            c.g[A0][0] = Addr(r32(c, V0)).lw(c);
            call_ptr(c, callee::STR_EQ, 0x0026_2adc)?;
            c.g[V0][0] = 1;
        } else {
            c.cycles += 2;
            c.g[V0][0] = 0;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int BuddyMan_FindRosterEntryNoCase(BuddyMan* self, void* name)`
/// (0x2624f8): index of the first roster entry whose name matches `*name`
/// case-insensitively, or -1 (null name, empty roster, or no match).
/// Loop head (event check) is 0x262540.
pub fn buddy_man_find_roster_entry_no_case(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S2, 0x20);
        frame.save(c, S3, 0x10);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S0, 0x40);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S1, 0x30);
        frame.save_ra(c, 0);
        let mut done = false;
        if c.g[S2][0] == 0 {
            c.cycles += 2;
            c.g[V0][0] = u64::MAX;
            done = true;
        }
        if !done {
            c.cycles += 6;
            c.g[A0][0] = c.g[S3][0];
            c.g[S0][0] = 0;
            callee::BUDDY_MAN_GET_ROSTER_COUNT.call(c, 0x0026_2530)?;
            c.g[S1][0] = c.g[V0][0];
            let empty = (c.g[S1][0] as i64) <= 0;
            c.g[V0][0] = u64::MAX;
            if !empty {
                c.cycles += 1;
                c.g[A0][0] = c.g[S3][0];
                loop {
                    c.cycles += 4;
                    c.g[A1][0] = c.g[S0][0];
                    callee::BUDDY_MAN_GET_ROSTER_ENTRY.call(c, 0x0026_2548)?;
                    let mut found = false;
                    if c.g[V0][0] == 0 {
                        c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
                    } else {
                        c.cycles += 5;
                        c.g[A0][0] = Addr(r32(c, S2)).lw(c);
                        let entry = c.g[V0][0];
                        c.g[A1][0] = entry;
                        call_ptr(c, callee::STR_CASE_EQ, 0x0026_255c)?;
                        let eq = c.g[V0][0] == 0;
                        c.g[V0][0] = c.g[S0][0];
                        if eq {
                            found = true;
                        } else {
                            c.cycles += 1;
                            c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
                        }
                    }
                    if found {
                        break;
                    }
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[S1][0] as i64)) as u64;
                    let more = c.g[V0][0] != 0;
                    c.g[A0][0] = c.g[S3][0];
                    if !more {
                        c.cycles += 1;
                        c.g[V0][0] = u64::MAX;
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0026_2540);
                    }
                }
            }
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `int BuddyMan_CountMessagesFromAt(BuddyMan* self, int idx)` (0x262950):
/// count stored messages whose sender name compares equal (exact) to the
/// name of roster entry `idx`; -1 when that entry does not exist.
/// Loop head (event check) is 0x2629a0.
pub fn buddy_man_count_messages_from_at(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 11;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S1, 0x30);
        frame.save(c, S3, 0x10);
        frame.save(c, S0, 0x40);
        frame.save(c, S2, 0x20);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A0][0];
        callee::BUDDY_MAN_GET_ROSTER_ENTRY.call(c, 0x0026_2970)?;
        c.g[S3][0] = c.g[V0][0];
        let missing = c.g[S3][0] == 0;
        c.g[V0][0] = u64::MAX;
        if !missing {
            c.cycles += 7;
            let this = r32(c, S1);
            c.g[V0][0] = Addr(this).off(BUDDY_MESSAGES_END).lw(c);
            c.g[S2][0] = 0;
            c.g[V1][0] = Addr(this).off(BUDDY_MESSAGES_BEGIN).lw(c);
            c.g[V0][0] = sx(((r32(c, V0).wrapping_sub(r32(c, V1)) as i32) >> 2) as u32);
            let empty = c.g[V0][0] == 0;
            c.g[S0][0] = 0;
            if !empty {
                c.cycles += 2;
                c.g[A0][0] = c.g[S1][0];
                loop {
                    c.cycles += 4;
                    c.g[A1][0] = c.g[S2][0];
                    callee::BUDDY_MAN_GET_MESSAGE.call(c, 0x0026_29a8)?;
                    if c.g[V0][0] == 0 {
                        c.g[V0][0] = Addr(r32(c, S1)).off(BUDDY_MESSAGES_END).lw(c);
                    } else {
                        c.cycles += 6;
                        c.g[A0][0] = Addr(r32(c, V0)).lw(c);
                        c.g[A1][0] = c.g[S3][0];
                        call_ptr(c, callee::STR_EQ, 0x0026_29bc)?;
                        c.g[V1][0] = sx(r32(c, S0).wrapping_add(1));
                        if c.g[V0][0] == 0 {
                            c.g[S0][0] = c.g[V1][0];
                        }
                        c.g[V0][0] = Addr(r32(c, S1)).off(BUDDY_MESSAGES_END).lw(c);
                    }
                    c.cycles += 7;
                    c.g[S2][0] = sx(r32(c, S2).wrapping_add(1));
                    c.g[V1][0] = Addr(r32(c, S1)).off(BUDDY_MESSAGES_BEGIN).lw(c);
                    c.g[V0][0] = sx(((r32(c, V0).wrapping_sub(r32(c, V1)) as i32) >> 2) as u32);
                    c.g[V0][0] = (c.g[S2][0] < c.g[V0][0]) as u64;
                    let more = c.g[V0][0] != 0;
                    c.g[A0][0] = c.g[S1][0];
                    if !more {
                        break;
                    }
                    if event_due(c) {
                        deliver_event_here(c, 0x0026_29a0);
                    }
                }
            }
            c.cycles += 1;
            c.g[V0][0] = c.g[S0][0];
        }
        c.cycles += 7;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void BuddyMan_SendMessage(BuddyMan* self, char* name, void* text)`
/// (0x262690): build a 0x1000-byte dirtysock tag buffer on the stack (sender
/// name, message text, and the `self + 0xa4` number) and hand it to the
/// session send call.
pub fn buddy_man_send_message(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 54;
        let frame = Frame::open(c, 0x1040);
        frame.save(c, S2, 0x1010);
        frame.save(c, S0, 0x1030);
        frame.save(c, S1, 0x1020);
        frame.save_ra(c, 0x1000);
        c.g[S2][0] = c.g[A2][0];
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        let buf = sp(c);
        // Tag 1: key TAG_SEND_A, value TAG_SEND_B.
        c.g[A0][0] = sx(buf);
        Addr(buf).sb(c, 0);
        c.g[A2][0] = sx(TAG_SEND_A);
        c.g[A1][0] = 0x1000;
        let val = TAG_SEND_B;
        c.g[A3][0] = sx(val);
        call_ptr(c, callee::TAG_FIELD_SET_STRING, 0x0026_26d0)?;
        // Tag 2: key TAG_SEND_C, value TAG_SEND_D.
        c.g[A0][0] = sx(buf);
        c.g[A2][0] = sx(TAG_SEND_C);
        c.g[A3][0] = sx(TAG_SEND_D);
        c.g[A1][0] = 0x1000;
        call_ptr(c, callee::TAG_FIELD_SET_STRING, 0x0026_26ec)?;
        // Tag 3: key TAG_SEND_E, value = name (a1).
        c.g[A0][0] = sx(buf);
        c.g[A3][0] = c.g[S1][0];
        c.g[A2][0] = sx(TAG_SEND_E);
        c.g[A1][0] = 0x1000;
        call_ptr(c, callee::TAG_FIELD_SET_STRING, 0x0026_2704)?;
        // Tag 4: key TAG_SEND_F, value = text (a2), via the unk setter.
        c.g[A0][0] = sx(buf);
        c.g[A3][0] = c.g[S2][0];
        c.g[A2][0] = sx(TAG_SEND_F);
        c.g[A1][0] = 0x1000;
        call_ptr(c, callee::TAG_FIELD_SET_UNK, 0x0026_271c)?;
        // Tag 5: key TAG_SEND_G, number = *(self + 0xa4).
        let this = r32(c, S0);
        c.g[A3][0] = Addr(this).off(BUDDY_FIELD_A4).lw(c);
        c.g[A0][0] = sx(buf);
        c.g[A2][0] = sx(TAG_SEND_G);
        c.g[A1][0] = 0x1000;
        call_ptr(c, callee::TAG_FIELD_SET_NUMBER, 0x0026_2734)?;
        // Send through the session object.
        c.g[A3][0] = c.g[S0][0];
        c.g[A0][0] = Addr(this).off(BUDDY_SESSION).lw(c);
        c.g[T0][0] = Addr(r32(c, A3)).off(BUDDY_FIELD_A4).lw(c);
        c.g[A2][0] = sx(0x0026_8fe0);
        c.g[A1][0] = sx(buf);
        call_ptr(c, callee::SESSION_3F16D8, 0x0026_2750)?;
        frame.restore(c, S0, 0x1030);
        frame.restore(c, S1, 0x1020);
        frame.restore(c, S2, 0x1010);
        frame.restore_ra(c, 0x1000);
        frame.close(c);
        Ok(())
    })
}

/// `int BuddyMan_AddRosterEntry(BuddyMan* self, char* name, int kind,
/// int* err, char* other)` (0x2620d8): validate `kind` against the lobby and
/// the permanent/temp roster limits, build a stack roster entry (name,
/// account name, kind flags), and register it through the session call;
/// `*err` receives the outcome code. The generated nesting is kept because
/// its exits skip different flag/count phases.
pub fn buddy_man_add_roster_entry(c: &mut Cpu) {
    guest(c, |c| {
        'epilogue: {
            'lobby_check: {
                c.cycles += 19;
                let frame = Frame::open(c, 0x140);
                frame.save(c, S0, 0x130);
                frame.save(c, S1, 0x120);
                frame.save(c, S2, 0x110);
                frame.save(c, S3, 0x100);
                frame.save(c, S6, 0xd0);
                frame.save(c, S4, 0xf0);
                frame.save(c, S5, 0xe0);
                frame.save_ra(c, 0xc0);
                c.g[S0][0] = c.g[T0][0];
                c.g[S1][0] = c.g[A2][0];
                c.g[S2][0] = c.g[A0][0];
                c.g[S3][0] = c.g[A3][0];
                c.g[S6][0] = c.g[A1][0];
                c.g[V0][0] = Addr(r32(c, GP).wrapping_add(LOBBY_INSTANCE_GP as u32)).lw(c);
                c.g[A0][0] = Addr(r32(c, V0).wrapping_add(0x80)).lw(c);
                call_ptr(c, callee::STR_CASE_EQ, 0x0026_211c)?;
                c.g[S4][0] = 0;
                if c.g[V0][0] != 0 {
                    break 'lobby_check;
                }
                c.cycles += 4;
                c.g[V1][0] = 3;
                c.g[V0][0] = 0;
                Addr(r32(c, S3)).sw(c, c.g[V1][0]);
                break 'epilogue;
            }
            // Temporary-entry classification of `other` (s0): s4 counts the
            // entry as permanent, s5 as temporary.
            {
                c.cycles += 2;
                let has_other = c.g[S0][0] != 0;
                c.g[S5][0] = 0;
                if !has_other {
                    c.cycles += 2;
                    c.g[S4][0] = 1;
                    c.cycles += 1;
                    c.g[S5][0] = 1;
                } else {
                    c.cycles += 5;
                    c.g[V0][0] = Addr(r32(c, S0).wrapping_add(BUDDY_ENTRY_FLAGS)).lw(c);
                    c.g[V1][0] = 0x10_0000;
                    c.g[V0][0] &= c.g[V1][0];
                    if c.g[V0][0] == 0 {
                        c.g[S5][0] = 1;
                    } else {
                        c.cycles += 2;
                        c.g[S4][0] = 1;
                    }
                }
            }
            // Kind dispatch and registration (generated block nesting kept:
            // `flags_join` is the flag-store join, `temp_exit` the temp-limit
            // join, `temp_over` the error exit writing `*err = kind`).
            'register_done: {
                'flags_join: {
                    'temp_exit: {
                        'temp_over: {
                            'kind_two: {
                                'entry: {
                                    c.cycles += 9;
                                    let buf = sp(c);
                                    c.g[A0][0] = sx(buf);
                                    c.g[A1][0] = c.g[S6][0];
                                    call_ptr(c, callee::STRCPY, 0x0026_2170)?;
                                    c.g[S0][0] = 1;
                                    c.g[A1][0] =
                                        Addr(r32(c, S2).wrapping_add(BUDDY_NAME_PTR)).lw(c);
                                    c.g[A0][0] = sx(buf.wrapping_add(0x40));
                                    call_ptr(c, callee::STRCPY, 0x0026_2180)?;
                                    let kind_is_one = c.g[S1][0] == c.g[S0][0];
                                    Addr(buf.wrapping_add(0x20)).sb(c, 0);
                                    if kind_is_one {
                                        break 'kind_two;
                                    }
                                    'kind_check: {
                                        c.cycles += 3;
                                        c.g[V0][0] = ((c.g[S1][0] as i64) < 2) as u64;
                                        let at_least_two = c.g[V0][0] == 0;
                                        c.g[V0][0] = 2;
                                        if at_least_two {
                                            break 'kind_check;
                                        }
                                        c.cycles += 2;
                                        if c.g[S1][0] == 0 {
                                            break 'entry;
                                        }
                                        c.cycles += 2;
                                        Addr(r32(c, S3)).sw(c, 0);
                                        break 'register_done;
                                    }
                                    c.cycles += 2;
                                    if c.g[S1][0] == c.g[V0][0] {
                                        break 'temp_over;
                                    }
                                    c.cycles += 2;
                                    Addr(r32(c, S3)).sw(c, 0);
                                    break 'register_done;
                                }
                                // kind 0: permanent-limit check.
                                c.cycles += 6;
                                c.g[A0][0] = c.g[S2][0];
                                callee::BUDDY_MAN_COUNT_PERMANENT_ENTRIES.call(c, 0x0026_21bc)?;
                                c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, S4)));
                                c.g[V0][0] = ((c.g[V0][0] as i64) < 41) as u64;
                                let under = c.g[V0][0] != 0;
                                c.g[V0][0] = Addr(sp(c).wrapping_add(0xa8)).lw(c);
                                if !under {
                                    c.cycles += 3;
                                    Addr(r32(c, S3)).sw(c, c.g[S0][0]);
                                    c.g[V0][0] = 0;
                                    break 'epilogue;
                                }
                                c.cycles += 2;
                                c.g[V0][0] |= 4;
                                break 'flags_join;
                            }
                            // kind 1: permanent-limit check, then the 0x200 flag.
                            c.cycles += 6;
                            c.g[A0][0] = c.g[S2][0];
                            callee::BUDDY_MAN_COUNT_PERMANENT_ENTRIES.call(c, 0x0026_21e8)?;
                            c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, S4)));
                            c.g[V0][0] = ((c.g[V0][0] as i64) < 41) as u64;
                            let full = c.g[V0][0] == 0;
                            c.g[V0][0] = Addr(sp(c).wrapping_add(0xa8)).lw(c);
                            if full {
                                c.cycles += 3;
                                Addr(r32(c, S3)).sw(c, c.g[S1][0]);
                                c.g[V0][0] = 0;
                                break 'epilogue;
                            }
                            c.cycles += 2;
                            c.g[V0][0] |= 0x200;
                            break 'flags_join;
                        }
                        // (reached only via `break 'temp_over` from the temp-limit
                        // check below)
                        'temp_check: {
                            c.cycles += 6;
                            c.g[A0][0] = c.g[S2][0];
                            callee::BUDDY_MAN_COUNT_TEMP_ENTRIES.call(c, 0x0026_2208)?;
                            c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, S5)));
                            c.g[V0][0] = ((c.g[V0][0] as i64) < 11) as u64;
                            let under = c.g[V0][0] != 0;
                            c.g[V0][0] = Addr(sp(c).wrapping_add(0xa8)).lw(c);
                            if under {
                                break 'temp_exit;
                            }
                            break 'temp_check;
                        }
                        c.cycles += 3;
                        Addr(r32(c, S3)).sw(c, c.g[S1][0]);
                        c.g[V0][0] = 0;
                        break 'epilogue;
                    }
                    c.cycles += 4;
                    c.g[V1][0] = 0x10_0000;
                    c.g[A0][0] = 0x80_0000;
                    c.g[V0][0] |= c.g[V1][0];
                    c.g[V0][0] |= c.g[A0][0];
                }
                c.cycles += 2;
                Addr(sp(c).wrapping_add(0xa8)).sw(c, c.g[V0][0]);
                Addr(r32(c, S3)).sw(c, 0);
            }
            c.cycles += 7;
            c.g[A3][0] = c.g[S2][0];
            c.g[A0][0] = Addr(r32(c, S2).wrapping_add(BUDDY_SESSION)).lw(c);
            c.g[A1][0] = sx(sp(c));
            c.g[A2][0] = sx(0x0026_8fb8);
            c.g[T0][0] = 0x258;
            call_ptr(c, callee::SESSION_3F0E38, 0x0026_2258)?;
        }
        c.cycles += 10;
        // Restore through the current stack pointer (frame still open).
        let frame = Frame { size: 0x140 };
        frame.restore(c, S0, 0x130);
        frame.restore(c, S1, 0x120);
        frame.restore(c, S2, 0x110);
        frame.restore(c, S3, 0x100);
        frame.restore(c, S4, 0xf0);
        frame.restore(c, S5, 0xe0);
        frame.restore(c, S6, 0xd0);
        frame.restore_ra(c, 0xc0);
        frame.close(c);
        Ok(())
    })
}

/// `void BuddyMan_ChangeRosterEntry(BuddyMan* self, char* name, int kind,
/// int* err)` (0x261ef8): look the name up in the session roster; when absent
/// add it directly, otherwise re-add it under the kind/flags rules, removing
/// any pending challenger for it when required. Generated nesting kept.
pub fn buddy_man_change_roster_entry(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x90);
        'epilogue: {
            'found: {
                c.cycles += 18;
                frame.save(c, S0, 0x80);
                frame.save(c, S1, 0x70);
                frame.save(c, S2, 0x60);
                frame.save(c, S3, 0x50);
                frame.save(c, S5, 0x30);
                frame.save(c, S4, 0x40);
                frame.save(c, S6, 0x20);
                frame.save_ra(c, 0x10);
                c.g[S1][0] = c.g[A0][0];
                c.g[S2][0] = c.g[A2][0];
                c.g[S3][0] = c.g[A3][0];
                c.g[S5][0] = c.g[A1][0];
                c.g[A0][0] = Addr(r32(c, S1).wrapping_add(BUDDY_SESSION)).lw(c);
                call_ptr(c, callee::SESSION_3F14D8, 0x0026_1f34)?;
                c.g[S0][0] = c.g[V0][0];
                if c.g[S0][0] != 0 {
                    c.g[A0][0] = Addr(r32(c, S1).wrapping_add(BUDDY_SESSION)).lw(c);
                    break 'found;
                }
                c.cycles += 9;
                c.g[A0][0] = c.g[S1][0];
                c.g[A1][0] = c.g[S5][0];
                c.g[A2][0] = c.g[S2][0];
                c.g[A3][0] = c.g[S3][0];
                c.g[T0][0] = 0;
                callee::BUDDY_MAN_ADD_ROSTER_ENTRY.call(c, 0x0026_1f58)?;
                c.g[V1][0] = Addr(r32(c, S3)).lw(c);
                let failed = c.g[V1][0] != 0;
                frame.restore(c, S0, 0x80);
                if !failed {
                    c.cycles += 2;
                    Addr(r32(c, S3)).sw(c, 0);
                }
                break 'epilogue;
            }
            'ac: {
                'join98: {
                    'join78: {
                        'join6c: {
                            'main: {
                                c.cycles += 14;
                                c.g[A2][0] = sx(0x0026_8f90);
                                c.g[A1][0] = c.g[S0][0];
                                c.g[A3][0] = c.g[S1][0];
                                c.g[T0][0] = 0x258;
                                c.g[S4][0] = Addr(r32(c, S0).wrapping_add(BUDDY_ENTRY_FLAGS)).lw(c);
                                call_ptr(c, callee::SESSION_3F11F8, 0x0026_1f88)?;
                                c.g[A0][0] = c.g[S1][0];
                                c.g[A1][0] = c.g[S5][0];
                                callee::BUDDY_MAN_HAS_CHALLENGER.call(c, 0x0026_1f94)?;
                                c.g[S6][0] = c.g[V0][0];
                                c.g[V0][0] = 1;
                                let kind_is_one = c.g[S2][0] == c.g[V0][0];
                                c.g[V0][0] = ((c.g[S2][0] as i64) < 2) as u64;
                                if kind_is_one {
                                    break 'main;
                                }
                                'cc: {
                                    'cb: {
                                        c.cycles += 2;
                                        if c.g[V0][0] == 0 {
                                            c.g[V0][0] = 2;
                                            break 'cb;
                                        }
                                        c.cycles += 2;
                                        let kind_is_zero = c.g[S2][0] == 0;
                                        c.g[V0][0] = c.g[S4][0] & 4;
                                        if kind_is_zero {
                                            break 'cc;
                                        }
                                        c.cycles += 2;
                                        break 'join98;
                                    }
                                    c.cycles += 2;
                                    let kind_is_two = c.g[S2][0] == c.g[V0][0];
                                    c.g[V0][0] = 0x10_0000;
                                    if kind_is_two {
                                        break 'join6c;
                                    }
                                    c.cycles += 2;
                                    break 'join98;
                                }
                                c.cycles += 2;
                                let flagged = c.g[V0][0] != 0;
                                c.g[A0][0] = c.g[S1][0];
                                if flagged {
                                    break 'join98;
                                }
                                'add0: {
                                    c.cycles += 8;
                                    c.g[A1][0] = c.g[S5][0];
                                    c.g[A2][0] = 0;
                                    c.g[A3][0] = c.g[S3][0];
                                    c.g[T0][0] = c.g[S0][0];
                                    callee::BUDDY_MAN_ADD_ROSTER_ENTRY.call(c, 0x0026_1fe8)?;
                                    c.g[V1][0] = Addr(r32(c, S3)).lw(c);
                                    let no_err = c.g[V1][0] == 0;
                                    c.g[V0][0] = 0x10_0000;
                                    if no_err {
                                        break 'add0;
                                    }
                                    c.cycles += 3;
                                    c.g[V0][0] = c.g[S4][0] & c.g[V0][0];
                                    let clear = c.g[V0][0] == 0;
                                    c.g[A0][0] = c.g[S1][0];
                                    if clear {
                                        break 'ac;
                                    }
                                    c.cycles += 7;
                                    c.g[A1][0] = c.g[S5][0];
                                    c.g[T0][0] = c.g[S0][0];
                                    c.g[A2][0] = 2;
                                    c.g[A3][0] = sx(sp(c));
                                    callee::BUDDY_MAN_ADD_ROSTER_ENTRY.call(c, 0x0026_2014)?;
                                    frame.restore(c, S0, 0x80);
                                    break 'epilogue;
                                }
                                c.cycles += 3;
                                c.g[V0][0] = c.g[S4][0] & c.g[V0][0];
                                if c.g[V0][0] != 0 {
                                    c.g[S6][0] = 0;
                                }
                                break 'join98;
                            }
                            // kind == 1 landing.
                            c.cycles += 3;
                            c.g[V0][0] = c.g[S4][0] & 0x200;
                            let temp_flag = c.g[V0][0] != 0;
                            c.g[T0][0] = c.g[S0][0];
                            if temp_flag {
                                break 'join78;
                            }
                            c.cycles += 8;
                            c.g[A0][0] = c.g[S1][0];
                            c.g[A1][0] = c.g[S5][0];
                            c.g[A2][0] = 1;
                            c.g[A3][0] = c.g[S3][0];
                            callee::BUDDY_MAN_ADD_ROSTER_ENTRY.call(c, 0x0026_2048)?;
                            c.g[V1][0] = Addr(r32(c, S3)).lw(c);
                            if c.g[V1][0] == 0 {
                                break 'join98;
                            }
                            c.cycles += 2;
                            let no_chal = c.g[S6][0] == 0;
                            c.g[A0][0] = c.g[S1][0];
                            if no_chal {
                                break 'ac;
                            }
                            c.cycles += 4;
                            c.g[A1][0] = c.g[S5][0];
                            callee::BUDDY_MAN_REMOVE_CHALLENGER.call(c, 0x0026_2064)?;
                            frame.restore(c, S0, 0x80);
                            break 'epilogue;
                        }
                        c.cycles += 3;
                        c.g[V0][0] = c.g[S4][0] & c.g[V0][0];
                        let flagged = c.g[V0][0] != 0;
                        c.g[T0][0] = c.g[S0][0];
                        if flagged {
                            break 'join98;
                        }
                    }
                    c.cycles += 8;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[A1][0] = c.g[S5][0];
                    c.g[A2][0] = 2;
                    c.g[A3][0] = c.g[S3][0];
                    callee::BUDDY_MAN_ADD_ROSTER_ENTRY.call(c, 0x0026_208c)?;
                    c.g[V1][0] = Addr(r32(c, S3)).lw(c);
                    let failed = c.g[V1][0] != 0;
                    if failed {
                        frame.restore(c, S0, 0x80);
                        break 'epilogue;
                    }
                    break 'join98;
                }
                // Pending-challenger cleanup (generated block 0x2620a8).
                c.cycles += 2;
                let has_chal = c.g[S6][0] != 0;
                c.g[A0][0] = c.g[S1][0];
                if has_chal {
                    c.cycles += 2;
                    c.g[A1][0] = c.g[S5][0];
                    callee::BUDDY_MAN_REMOVE_CHALLENGER.call(c, 0x0026_20a8)?;
                }
                c.cycles += 1;
                Addr(r32(c, S3)).sw(c, 0);
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x80);
        }
        c.cycles += 9;
        frame.restore(c, S1, 0x70);
        frame.restore(c, S2, 0x60);
        frame.restore(c, S3, 0x50);
        frame.restore(c, S4, 0x40);
        frame.restore(c, S5, 0x30);
        frame.restore(c, S6, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void BuddyMan_RemoveTempEntryAt(BuddyMan* self, int idx)` (0x262ec0):
/// when roster entry `idx` is temporary, delete its stored messages and any
/// pending challenger for its name, notifying the lobby. Loop head (event
/// check) is 0x262f50. Generated nesting kept where exits differ.
pub fn buddy_man_remove_temp_entry_at(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x40);
        'epilogue: {
            c.cycles += 9;
            frame.save(c, S0, 0x30);
            frame.save(c, S1, 0x20);
            frame.save_ra(c, 0x10);
            c.g[S0][0] = c.g[A1][0];
            c.g[S1][0] = c.g[A0][0];
            callee::BUDDY_MAN_IS_TEMP_AT.call(c, 0x0026_2edc)?;
            if c.g[V0][0] == 0 {
                frame.restore(c, S0, 0x30);
                break 'epilogue;
            }
            'entry: {
                c.cycles += 6;
                c.g[A1][0] = c.g[S0][0];
                c.g[A0][0] = c.g[S1][0];
                callee::BUDDY_MAN_GET_ROSTER_ENTRY.call(c, 0x0026_2ef0)?;
                c.g[S0][0] = c.g[V0][0];
                let missing = c.g[S0][0] == 0;
                c.g[A0][0] = sx(sp(c));
                if missing {
                    break 'entry;
                }
                {
                    c.cycles += 14;
                    c.g[A1][0] = c.g[S0][0];
                    call_ptr(c, callee::BXSTRING_CTOR2, 0x0026_2f04)?;
                    c.g[A0][0] = Addr(r32(c, S1).wrapping_add(BUDDY_SESSION)).lw(c);
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A3][0] = c.g[S1][0];
                    c.g[T0][0] = 0x258;
                    c.g[A2][0] = sx(0x0026_8f90);
                    call_ptr(c, callee::SESSION_3F11F8, 0x0026_2f20)?;
                    c.g[A1][0] = Addr(sp(c)).lw(c);
                    c.g[A0][0] = c.g[S1][0];
                    callee::BUDDY_MAN_HAS_MESSAGE_FROM.call(c, 0x0026_2f2c)?;
                    let has_msg = c.g[V0][0] != 0;
                    c.g[A1][0] = Addr(sp(c)).lw(c);
                    if has_msg {
                        c.cycles += 5;
                        c.g[A0][0] = c.g[S1][0];
                        callee::BUDDY_MAN_GET_MESSAGE_COUNT.call(c, 0x0026_2f3c)?;
                        c.g[S0][0] = sx(r32(c, V0).wrapping_sub(1));
                        let negative = (c.g[S0][0] as i64) < 0;
                        c.g[A1][0] = Addr(sp(c)).lw(c);
                        if !negative {
                            c.cycles += 2;
                            c.g[A0][0] = c.g[S1][0];
                            loop {
                                c.cycles += 4;
                                c.g[A1][0] = c.g[S0][0];
                                callee::BUDDY_MAN_GET_MESSAGE.call(c, 0x0026_2f58)?;
                                if c.g[V0][0] == 0 {
                                    c.g[S0][0] = sx(r32(c, S0).wrapping_sub(1));
                                } else {
                                    c.cycles += 5;
                                    c.g[A0][0] = Addr(r32(c, V0)).lw(c);
                                    c.g[A1][0] = Addr(sp(c)).lw(c);
                                    call_ptr(c, callee::STR_CASE_EQ, 0x0026_2f6c)?;
                                    if c.g[V0][0] != 0 {
                                        c.g[S0][0] = sx(r32(c, S0).wrapping_sub(1));
                                    } else {
                                        c.cycles += 4;
                                        c.g[A0][0] = c.g[S1][0];
                                        c.g[A1][0] = c.g[S0][0];
                                        callee::BUDDY_MAN_DELETE_MESSAGE_AT.call(c, 0x0026_2f80)?;
                                        c.g[S0][0] = sx(r32(c, S0).wrapping_sub(1));
                                    }
                                }
                                c.cycles += 2;
                                let more = (c.g[S0][0] as i64) >= 0;
                                c.g[A0][0] = c.g[S1][0];
                                if !more {
                                    c.cycles += 1;
                                    c.g[A1][0] = Addr(sp(c)).lw(c);
                                    break;
                                }
                                if event_due(c) {
                                    deliver_event_here(c, 0x0026_2f50);
                                }
                            }
                        }
                    }
                }
                {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S1][0];
                    callee::BUDDY_MAN_HAS_CHALLENGER.call(c, 0x0026_2f98)?;
                    let has_chal = c.g[V0][0] != 0;
                    c.g[A1][0] = Addr(sp(c)).lw(c);
                    if has_chal {
                        c.cycles += 12;
                        c.g[A0][0] = lobby_instance(c);
                        call_ptr(c, callee::LOBBY_SEND_DECLINE, 0x0026_2fa8)?;
                        c.g[A1][0] = Addr(sp(c)).lw(c);
                        c.g[A0][0] = c.g[S1][0];
                        callee::BUDDY_MAN_FIND_CHALLENGER_INDEX.call(c, 0x0026_2fb4)?;
                        let this = r32(c, S1);
                        c.g[V1][0] = Addr(this).off(BUDDY_CHALLENGERS_BEGIN).lw(c);
                        c.g[V0][0] = sx(r32(c, V0) << 2);
                        c.g[A0][0] = Addr(this).off(BUDDY_CHALLENGERS_END).lw(c);
                        c.g[V0][0] = sx(r32(c, V0).wrapping_add(r32(c, V1)));
                        c.g[V1][0] = sx(r32(c, V0).wrapping_add(4));
                        let at_end = c.g[V1][0] == c.g[A0][0];
                        c.g[A1][0] = sx(this.wrapping_add(BUDDY_CHALLENGERS_VEC));
                        if !at_end {
                            c.cycles += 7;
                            c.g[A2][0] = Addr(r32(c, A1).wrapping_add(8)).lw(c);
                            c.g[A0][0] = c.g[V0][0];
                            c.g[A1][0] = c.g[V1][0];
                            c.g[A2][0] = sx(r32(c, A2).wrapping_sub(r32(c, V1)));
                            c.g[A2][0] = sx(((r32(c, A2) as i32) >> 2) as u32);
                            c.g[A2][0] = sx(r32(c, A2) << 2);
                            call_ptr(c, callee::MEMMOVE, 0x0026_2fec)?;
                        }
                        c.cycles += 3;
                        c.g[V0][0] = Addr(this).off(BUDDY_CHALLENGERS_END).lw(c);
                        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(4));
                        Addr(this).off(BUDDY_CHALLENGERS_END).sw(c, c.g[V0][0]);
                    }
                }
                c.cycles += 3;
                c.g[A0][0] = sx(sp(c));
                c.g[A1][0] = 2;
                call_ptr(c, callee::BXSTRING_DTOR, 0x0026_3004)?;
            }
            c.cycles += 1;
            frame.restore(c, S0, 0x30);
        }
        c.cycles += 4;
        frame.restore(c, S1, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `void BuddyMan_SetPresence(BuddyMan* self, int state, int a2, char* a3,
/// char* a4, char* a5)` (0x262768): when a session exists, build a 0x50-byte
/// presence tag buffer (name, fixed number 3, `a2`, then the optional
/// `a3`/`a4`/`a5` strings) and push it through the session presence calls.
pub fn buddy_man_set_presence(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0xc0);
        frame.save(c, S0, 0xb0);
        frame.save(c, S1, 0xa0);
        frame.save(c, S2, 0x90);
        frame.save(c, S3, 0x80);
        frame.save(c, S4, 0x70);
        frame.save(c, S5, 0x60);
        frame.save_ra(c, 0x50);
        c.g[S0][0] = c.g[A3][0];
        c.g[S1][0] = c.g[A0][0];
        c.g[S2][0] = c.g[A2][0];
        c.g[S3][0] = c.g[T0][0];
        c.g[S4][0] = c.g[T1][0];
        let this = r32(c, S1);
        c.g[V0][0] = Addr(this).off(BUDDY_SESSION).lw(c);
        let has_session = c.g[V0][0] != 0;
        c.g[S5][0] = c.g[A1][0];
        if has_session {
            c.cycles += 21;
            let buf = sp(c);
            c.g[A3][0] = Addr(this).off(BUDDY_NAME_PTR).lw(c);
            c.g[A0][0] = sx(buf);
            Addr(buf).sb(c, 0);
            c.g[A2][0] = sx(TAG_PRES_A);
            c.g[A1][0] = 0x50;
            call_ptr(c, callee::TAG_FIELD_SET_STRING, 0x0026_27c4)?;
            c.g[A0][0] = sx(buf);
            c.g[A2][0] = sx(TAG_PRES_B);
            c.g[A1][0] = 0x50;
            c.g[A3][0] = 3;
            call_ptr(c, callee::TAG_FIELD_SET_NUMBER, 0x0026_27dc)?;
            c.g[A3][0] = c.g[S2][0];
            c.g[A2][0] = sx(TAG_PRES_C);
            c.g[A0][0] = sx(buf);
            c.g[A1][0] = 0x50;
            call_ptr(c, callee::TAG_FIELD_SET_NUMBER, 0x0026_27f4)?;
            c.g[A3][0] = c.g[S0][0];
            if c.g[S0][0] != 0 {
                c.cycles += 7;
                c.g[A2][0] = sx(TAG_PRES_D);
                c.g[A0][0] = sx(buf);
                c.g[A1][0] = 0x50;
                call_ptr(c, callee::TAG_FIELD_SET_STRING, 0x0026_2810)?;
                c.g[A3][0] = c.g[S3][0];
                if c.g[S3][0] != 0 {
                    c.cycles += 7;
                    c.g[A2][0] = sx(TAG_PRES_E);
                    c.g[A0][0] = sx(buf);
                    c.g[A1][0] = 0x50;
                    call_ptr(c, callee::TAG_FIELD_SET_STRING, 0x0026_282c)?;
                    c.g[A3][0] = c.g[S4][0];
                    if c.g[S4][0] != 0 {
                        c.cycles += 5;
                        c.g[A2][0] = sx(TAG_PRES_F);
                        c.g[A0][0] = sx(buf);
                        c.g[A1][0] = 0x50;
                        call_ptr(c, callee::TAG_FIELD_SET_STRING, 0x0026_2848)?;
                    }
                }
            }
            c.cycles += 20;
            c.g[A0][0] = Addr(this).off(BUDDY_SESSION).lw(c);
            call_ptr(c, callee::SESSION_3F1538, 0x0026_2850)?;
            c.g[A0][0] = Addr(this).off(BUDDY_SESSION).lw(c);
            c.g[A1][0] = sx(buf);
            call_ptr(c, callee::SESSION_3F1548, 0x0026_285c)?;
            c.g[A0][0] = Addr(this).off(BUDDY_SESSION).lw(c);
            c.g[A1][0] = sx(TAG_PRES_GLOBAL);
            c.g[A2][0] = sx(TAG_PRES_STR);
            call_ptr(c, callee::SESSION_3F1578, 0x0026_2874)?;
            c.g[A1][0] = c.g[S5][0];
            c.g[S0][0] = Addr(this).off(BUDDY_NAME_PTR).lw(c);
            c.g[A0][0] = c.g[S1][0];
            callee::BUDDY_MAN_MAP_PRESENCE_STATE.call(c, 0x0026_2884)?;
            c.g[A0][0] = Addr(this).off(BUDDY_SESSION).lw(c);
            c.g[A1][0] = c.g[S0][0];
            c.g[A2][0] = c.g[V0][0];
            c.g[A3][0] = 0x7d0;
            call_ptr(c, callee::SESSION_3F15C8, 0x0026_2898)?;
        }
        c.cycles += 9;
        frame.restore(c, S0, 0xb0);
        frame.restore(c, S1, 0xa0);
        frame.restore(c, S2, 0x90);
        frame.restore(c, S3, 0x80);
        frame.restore(c, S4, 0x70);
        frame.restore(c, S5, 0x60);
        frame.restore_ra(c, 0x50);
        frame.close(c);
        Ok(())
    })
}

/// `void BuddyMan_OnChallengeReceived(BuddyMan* self, char* name, ...)`
/// (0x262af0): register the challenger (adding a temporary roster entry when
/// needed), allocate the challenger record and the reply node, and insert
/// them into the challenger/message vectors. Dispatcher: every arm is a
/// resume head; the event back-edges are at 0x262b58/0x262b64 (challenger
/// drain) and 0x262bb8/0x262bd0 (temporary-entry scan).
pub fn buddy_man_on_challenge_received(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x0026_2af0 => {
                    c.cycles += 24;
                    let frame = Frame::open(c, 0xd0);
                    frame.save(c, S2, 0xa0);
                    frame.save(c, S4, 0x80);
                    frame.save(c, S5, 0x70);
                    frame.save(c, S6, 0x60);
                    frame.save(c, S7, 0x50);
                    frame.save(c, FP, 0x40);
                    frame.save(c, S0, 0xc0);
                    frame.save(c, S1, 0xb0);
                    frame.save(c, S3, 0x90);
                    frame.save_ra(c, 0x30);
                    c.g[S2][0] = c.g[A0][0];
                    c.g[S4][0] = c.g[A1][0];
                    c.g[S5][0] = c.g[A3][0];
                    c.g[S6][0] = c.g[T0][0];
                    c.g[S7][0] = c.g[T1][0];
                    c.g[FP][0] = c.g[T2][0];
                    Addr(sp(c).wrapping_add(0x28)).sw(c, c.g[A2][0]);
                    c.g[V0][0] = lobby_instance(c);
                    c.g[A0][0] = Addr(r32(c, V0).wrapping_add(0x80)).lw(c);
                    Addr(sp(c).wrapping_add(0x2c)).sw(c, c.g[T3][0]);
                    call_ptr(c, callee::STR_CASE_EQ, 0x0026_2b48)?;
                    let no_lobby = c.g[V0][0] == 0;
                    c.g[A0][0] = c.g[S2][0];
                    pc = if no_lobby { 0x0026_2df0 } else { 0x0026_2b50 };
                }
                0x0026_2b50 => {
                    c.cycles += 2;
                    pc = 0x0026_2b64;
                }
                0x0026_2b58 => {
                    c.cycles += 3;
                    c.g[A1][0] = c.g[S4][0];
                    callee::BUDDY_MAN_REMOVE_CHALLENGER.call(c, 0x0026_2b60)?;
                    c.g[A0][0] = c.g[S2][0];
                    pc = 0x0026_2b64;
                }
                0x0026_2b64 => {
                    c.cycles += 4;
                    c.g[A1][0] = c.g[S4][0];
                    callee::BUDDY_MAN_HAS_CHALLENGER.call(c, 0x0026_2b6c)?;
                    let has = c.g[V0][0] != 0;
                    c.g[A0][0] = c.g[S2][0];
                    if has {
                        if back_edge_unwinds(c, 0x0026_2b58) {
                            return Ok(());
                        }
                        pc = 0x0026_2b58;
                    } else {
                        pc = 0x0026_2b74;
                    }
                }
                0x0026_2b74 => {
                    c.cycles += 4;
                    c.g[A1][0] = c.g[S4][0];
                    callee::BUDDY_MAN_FIND_ROSTER_ENTRY.call(c, 0x0026_2b7c)?;
                    let found = (c.g[V0][0] as i64) >= 0;
                    c.g[A1][0] = c.g[V0][0];
                    pc = if found { 0x0026_2c8c } else { 0x0026_2b84 };
                }
                0x0026_2b84 => {
                    c.cycles += 11;
                    c.g[A3][0] = sx(sp(c).wrapping_add(0x20));
                    c.g[A0][0] = c.g[S2][0];
                    c.g[S1][0] = c.g[A3][0];
                    c.g[A1][0] = c.g[S4][0];
                    c.g[A2][0] = 2;
                    c.g[T0][0] = 0;
                    callee::BUDDY_MAN_ADD_ROSTER_ENTRY.call(c, 0x0026_2ba0)?;
                    c.g[V1][0] = Addr(sp(c).wrapping_add(0x20)).lw(c);
                    let failed = c.g[V1][0] != 2;
                    c.g[V0][0] = 2;
                    c.g[A1][0] = 0x480000;
                    pc = if failed { 0x0026_2ca0 } else { 0x0026_2bb0 };
                }
                0x0026_2bb0 => {
                    c.cycles += 2;
                    c.g[S0][0] = 0;
                    pc = 0x0026_2bbc;
                }
                0x0026_2bb8 => {
                    c.cycles += 1;
                    c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
                    pc = 0x0026_2bbc;
                }
                0x0026_2bbc => {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S2][0];
                    callee::BUDDY_MAN_GET_ROSTER_COUNT.call(c, 0x0026_2bc4)?;
                    c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let done = c.g[V0][0] == 0;
                    c.g[A0][0] = c.g[S2][0];
                    pc = if done { 0x0026_2be8 } else { 0x0026_2bd0 };
                }
                0x0026_2bd0 => {
                    c.cycles += 4;
                    c.g[A1][0] = c.g[S0][0];
                    callee::BUDDY_MAN_IS_TEMP_AT.call(c, 0x0026_2bd8)?;
                    let not_temp = c.g[V0][0] == 0;
                    c.g[A1][0] = c.g[S0][0];
                    if not_temp {
                        if back_edge_unwinds(c, 0x0026_2bb8) {
                            return Ok(());
                        }
                        pc = 0x0026_2bb8;
                    } else {
                        pc = 0x0026_2be0;
                    }
                }
                0x0026_2be0 => {
                    c.cycles += 2;
                    c.g[A0][0] = c.g[S2][0];
                    callee::BUDDY_MAN_REMOVE_TEMP_ENTRY_AT.call(c, 0x0026_2be8)?;
                    pc = 0x0026_2be8;
                }
                0x0026_2be8 => {
                    c.cycles += 31;
                    c.g[A3][0] = c.g[S1][0];
                    c.g[T0][0] = 0;
                    c.g[A1][0] = c.g[S4][0];
                    c.g[A0][0] = c.g[S2][0];
                    c.g[A2][0] = 2;
                    c.g[S0][0] = 0x108;
                    callee::BUDDY_MAN_ADD_ROSTER_ENTRY.call(c, 0x0026_2c04)?;
                    c.g[S1][0] = lobby_instance(c);
                    c.g[A1][0] = sx(0x0048_0488);
                    c.g[A0][0] = 8;
                    c.g[A2][0] = 0x2000_0000;
                    c.g[A3][0] = 0;
                    call_ptr(c, callee::MEM_ALLOC, 0x0026_2c20)?;
                    c.g[S3][0] = c.g[V0][0];
                    Addr(r32(c, S3)).sw(c, c.g[S0][0]);
                    Addr(r32(c, S3).wrapping_add(4)).sw(c, sx(0x0048_12b0));
                    c.g[A1][0] = sx(0x0048_04a8);
                    c.g[A0][0] = 0xc;
                    c.g[A2][0] = 0x2000_0000;
                    c.g[A3][0] = 0;
                    c.g[V0][0] = Addr(r32(c, S1).wrapping_add(BUDDY_LIST_HEAD)).lw(c);
                    Addr(sp(c).wrapping_add(0x10)).sw(c, c.g[V0][0]);
                    call_ptr(c, callee::MEM_ALLOC, 0x0026_2c54)?;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[V0][0] = sx(r32(c, A1).wrapping_add(8));
                    if c.g[V0][0] != 0 {
                        Addr(r32(c, A1).wrapping_add(8)).sw(c, c.g[S3][0]);
                    }
                    pc = 0x0026_2c64;
                }
                0x0026_2c64 => {
                    c.cycles += 10;
                    c.g[V0][0] = Addr(sp(c).wrapping_add(0x10)).lw(c);
                    Addr(r32(c, A1)).sw(c, c.g[V0][0]);
                    c.g[V1][0] = Addr(r32(c, V0).wrapping_add(4)).lw(c);
                    Addr(r32(c, A1).wrapping_add(4)).sw(c, c.g[V1][0]);
                    c.g[A0][0] = Addr(r32(c, V0).wrapping_add(4)).lw(c);
                    Addr(r32(c, A0)).sw(c, c.g[A1][0]);
                    c.g[V0][0] = Addr(sp(c).wrapping_add(0x10)).lw(c);
                    Addr(r32(c, V0).wrapping_add(4)).sw(c, c.g[A1][0]);
                    Addr(sp(c)).sw(c, c.g[A1][0]);
                    pc = 0x0026_2c9c;
                }
                0x0026_2c8c => {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S2][0];
                    callee::BUDDY_MAN_IS_BLOCKED_AT.call(c, 0x0026_2c94)?;
                    let blocked = c.g[V0][0] != 0;
                    let frame = Frame { size: 0xd0 };
                    if blocked {
                        frame.restore(c, S0, 0xc0);
                        pc = 0x0026_2df4;
                    } else {
                        pc = 0x0026_2c9c;
                    }
                }
                0x0026_2c9c => {
                    c.cycles += 1;
                    c.g[A1][0] = 0x480000;
                    pc = 0x0026_2ca0;
                }
                0x0026_2ca0 => {
                    c.cycles += 41;
                    c.g[A1][0] = sx(r32(c, A1).wrapping_add(0xec0));
                    c.g[A2][0] = 0x2000_0000;
                    c.g[A3][0] = 0;
                    c.g[A0][0] = 0x40;
                    call_ptr(c, callee::MEM_ALLOC, 0x0026_2cb4)?;
                    c.g[S0][0] = c.g[V0][0];
                    c.g[A1][0] = c.g[S4][0];
                    c.g[A0][0] = c.g[S0][0];
                    Addr(r32(c, S0).wrapping_add(0x3c)).sw(c, sx(0x0048_10e0));
                    call_ptr(c, callee::BXSTRING_CTOR2, 0x0026_2cd0)?;
                    c.g[A1][0] = Addr(sp(c).wrapping_add(0xe0)).lw(c);
                    c.g[A0][0] = sx(r32(c, S0).wrapping_add(4));
                    call_ptr(c, callee::BXSTRING_CTOR2, 0x0026_2cdc)?;
                    c.g[A1][0] = Addr(r32(c, S2).wrapping_add(BUDDY_CHALLENGERS_END)).lw(c);
                    c.g[A3][0] = sx(r32(c, S2).wrapping_add(BUDDY_CHALLENGERS_VEC));
                    c.g[A0][0] = Addr(r32(c, S2).wrapping_add(0xc4)).lw(c);
                    c.g[A2][0] = sx(sp(c).wrapping_add(0x24));
                    c.g[V0][0] = Addr(sp(c).wrapping_add(0x28)).lw(c);
                    Addr(r32(c, S0).wrapping_add(0xc)).sw(c, c.g[S5][0]);
                    Addr(r32(c, S0).wrapping_add(0x10)).sw(c, c.g[S6][0]);
                    Addr(r32(c, S0).wrapping_add(0x14)).sw(c, c.g[S7][0]);
                    Addr(r32(c, S0).wrapping_add(0x18)).sw(c, c.g[FP][0]);
                    Addr(r32(c, S0).wrapping_add(8)).sw(c, c.g[V0][0]);
                    c.g[V0][0] = Addr(sp(c).wrapping_add(0x2c)).lw(c);
                    Addr(r32(c, S0).wrapping_add(0x1c)).sw(c, c.g[V0][0]);
                    c.g[V0][0] = Addr(sp(c).wrapping_add(0xd0)).lw(c);
                    Addr(r32(c, S0).wrapping_add(0x20)).sw(c, c.g[V0][0]);
                    c.g[V1][0] = Addr(sp(c).wrapping_add(0xd8)).lw(c);
                    Addr(r32(c, S0).wrapping_add(0x24)).sw(c, c.g[V1][0]);
                    c.g[V0][0] = Addr(sp(c).wrapping_add(0xe8)).lw(c);
                    Addr(r32(c, S0).wrapping_add(0x2c)).sw(c, c.g[V0][0]);
                    c.g[V1][0] = Addr(sp(c).wrapping_add(0xf0)).lw(c);
                    Addr(r32(c, S0).wrapping_add(0x30)).sw(c, c.g[V1][0]);
                    c.g[V0][0] = Addr(sp(c).wrapping_add(0xf8)).lw(c);
                    Addr(r32(c, S0).wrapping_add(0x34)).sw(c, c.g[V0][0]);
                    c.g[V1][0] = Addr(sp(c).wrapping_add(0x100)).lw(c);
                    Addr(r32(c, S0).wrapping_add(0x28)).sw(c, c.g[V1][0]);
                    let at_cap = c.g[A1][0] == c.g[A0][0];
                    Addr(sp(c).wrapping_add(0x24)).sw(c, c.g[S0][0]);
                    pc = if at_cap { 0x0026_2d60 } else { 0x0026_2d44 };
                }
                0x0026_2d44 => {
                    c.cycles += 3;
                    c.g[A0][0] = Addr(r32(c, A3).wrapping_add(8)).lw(c);
                    if c.g[A0][0] != 0 {
                        Addr(r32(c, A0)).sw(c, c.g[S0][0]);
                    }
                    pc = 0x0026_2d50;
                }
                0x0026_2d50 => {
                    c.cycles += 4;
                    c.g[V0][0] = Addr(r32(c, S2).wrapping_add(BUDDY_CHALLENGERS_END)).lw(c);
                    c.g[V0][0] = sx(r32(c, V0).wrapping_add(4));
                    Addr(r32(c, S2).wrapping_add(BUDDY_CHALLENGERS_END)).sw(c, c.g[V0][0]);
                    pc = 0x0026_2d68;
                }
                0x0026_2d60 => {
                    c.cycles += 2;
                    c.g[A0][0] = c.g[A3][0];
                    callee::BUDDY_MAN_CHALLENGER_VECTOR_INSERT.call(c, 0x0026_2d68)?;
                    pc = 0x0026_2d68;
                }
                0x0026_2d68 => {
                    c.cycles += 25;
                    c.g[S0][0] = lobby_instance(c);
                    c.g[A1][0] = sx(0x0048_0488);
                    c.g[A0][0] = 8;
                    c.g[A2][0] = 0x2000_0000;
                    c.g[A3][0] = 0;
                    call_ptr(c, callee::MEM_ALLOC, 0x0026_2d84)?;
                    c.g[S1][0] = c.g[V0][0];
                    Addr(r32(c, S1)).sw(c, 0xec);
                    Addr(r32(c, S1).wrapping_add(4)).sw(c, sx(0x0048_12b0));
                    c.g[A1][0] = sx(0x0048_04a8);
                    c.g[A0][0] = 0xc;
                    c.g[A2][0] = 0x2000_0000;
                    c.g[A3][0] = 0;
                    c.g[V0][0] = Addr(r32(c, S0).wrapping_add(BUDDY_LIST_HEAD)).lw(c);
                    Addr(sp(c).wrapping_add(0x10)).sw(c, c.g[V0][0]);
                    call_ptr(c, callee::MEM_ALLOC, 0x0026_2dbc)?;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[V0][0] = sx(r32(c, A1).wrapping_add(8));
                    if c.g[V0][0] != 0 {
                        Addr(r32(c, A1).wrapping_add(8)).sw(c, c.g[S1][0]);
                    }
                    pc = 0x0026_2dcc;
                }
                0x0026_2dcc => {
                    c.cycles += 9;
                    c.g[V0][0] = Addr(sp(c).wrapping_add(0x10)).lw(c);
                    Addr(r32(c, A1)).sw(c, c.g[V0][0]);
                    c.g[V1][0] = Addr(r32(c, V0).wrapping_add(4)).lw(c);
                    Addr(r32(c, A1).wrapping_add(4)).sw(c, c.g[V1][0]);
                    c.g[A0][0] = Addr(r32(c, V0).wrapping_add(4)).lw(c);
                    Addr(r32(c, A0)).sw(c, c.g[A1][0]);
                    c.g[V0][0] = Addr(sp(c).wrapping_add(0x10)).lw(c);
                    Addr(r32(c, V0).wrapping_add(4)).sw(c, c.g[A1][0]);
                    Addr(sp(c)).sw(c, c.g[A1][0]);
                    pc = 0x0026_2df0;
                }
                0x0026_2df0 => {
                    c.cycles += 1;
                    let frame = Frame { size: 0xd0 };
                    frame.restore(c, S0, 0xc0);
                    pc = 0x0026_2df4;
                }
                0x0026_2df4 => {
                    c.cycles += 11;
                    let frame = Frame { size: 0xd0 };
                    frame.restore(c, S1, 0xb0);
                    frame.restore(c, S2, 0xa0);
                    frame.restore(c, S3, 0x90);
                    frame.restore(c, S4, 0x80);
                    frame.restore(c, S5, 0x70);
                    frame.restore(c, S6, 0x60);
                    frame.restore(c, S7, 0x50);
                    frame.restore(c, FP, 0x40);
                    frame.restore_ra(c, 0x30);
                    frame.close(c);
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x0026_2af0, 0x0026_2e20);
                    return Ok(());
                }
            }
        }
    })
}

/// `void BuddyMan_AddMessage(BuddyMan* self, char* name, char* text,
/// char* date)` (0x261cd0): ensure the sender is on the roster (adding a
/// temporary entry when needed), allocate the message record, and append it
/// to the message vector. Dispatcher: every arm is a resume head; the event
/// back-edge is at 0x261d80/0x261d98 (temporary-entry scan).
pub fn buddy_man_add_message(c: &mut Cpu) {
    guest(c, |c| {
        let mut pc: u32 = c.pc;
        loop {
            match pc {
                0x0026_1cd0 => {
                    c.cycles += 18;
                    let frame = Frame::open(c, 0xd0);
                    frame.save(c, S2, 0xa0);
                    frame.save(c, S5, 0x70);
                    frame.save(c, FP, 0x40);
                    frame.save(c, S4, 0x80);
                    frame.save(c, S0, 0xc0);
                    frame.save(c, S1, 0xb0);
                    frame.save(c, S3, 0x90);
                    frame.save(c, S6, 0x60);
                    frame.save(c, S7, 0x50);
                    frame.save_ra(c, 0x30);
                    c.g[S2][0] = c.g[A0][0];
                    c.g[S5][0] = c.g[A1][0];
                    c.g[FP][0] = c.g[A2][0];
                    Addr(sp(c).wrapping_add(0x28)).sw(c, c.g[A3][0]);
                    call_ptr(c, callee::IS_BUDDY, 0x0026_1d10)?;
                    let is_buddy = c.g[V0][0] != 0;
                    c.g[S4][0] = sx(sp(c).wrapping_add(0x24));
                    pc = if is_buddy { 0x0026_1e44 } else { 0x0026_1d18 };
                }
                0x0026_1d18 => {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[A1][0] = c.g[S5][0];
                    call_ptr(c, callee::IS_BLOCKED, 0x0026_1d24)?;
                    let blocked = c.g[V0][0] != 0;
                    c.g[V0][0] = 0x480000;
                    pc = if blocked { 0x0026_1e48 } else { 0x0026_1d2c };
                }
                0x0026_1d2c => {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[A1][0] = c.g[S5][0];
                    call_ptr(c, callee::IS_TEMP, 0x0026_1d38)?;
                    let temp = c.g[V0][0] != 0;
                    c.g[V0][0] = 0x480000;
                    pc = if temp { 0x0026_1e48 } else { 0x0026_1d40 };
                }
                0x0026_1d40 => {
                    c.cycles += 11;
                    c.g[A3][0] = sx(sp(c).wrapping_add(0x20));
                    c.g[A0][0] = c.g[S2][0];
                    c.g[S3][0] = c.g[A3][0];
                    c.g[A1][0] = c.g[S5][0];
                    c.g[A2][0] = 2;
                    c.g[T0][0] = 0;
                    callee::BUDDY_MAN_ADD_ROSTER_ENTRY.call(c, 0x0026_1d5c)?;
                    c.g[V1][0] = Addr(sp(c).wrapping_add(0x20)).lw(c);
                    let failed = c.g[V1][0] != 2;
                    c.g[V0][0] = 0x480000;
                    pc = if failed { 0x0026_1e48 } else { 0x0026_1d6c };
                }
                0x0026_1d6c => {
                    c.cycles += 5;
                    c.g[S0][0] = 0;
                    c.g[S1][0] = 0x480000;
                    c.g[S6][0] = 0x480000;
                    c.g[S7][0] = 0x480000;
                    pc = 0x0026_1d84;
                }
                0x0026_1d80 => {
                    c.cycles += 1;
                    c.g[S0][0] = sx(r32(c, S0).wrapping_add(1));
                    pc = 0x0026_1d84;
                }
                0x0026_1d84 => {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S2][0];
                    callee::BUDDY_MAN_GET_ROSTER_COUNT.call(c, 0x0026_1d8c)?;
                    c.g[V0][0] = ((c.g[S0][0] as i64) < (c.g[V0][0] as i64)) as u64;
                    let done = c.g[V0][0] == 0;
                    c.g[A0][0] = c.g[S2][0];
                    pc = if done { 0x0026_1db0 } else { 0x0026_1d98 };
                }
                0x0026_1d98 => {
                    c.cycles += 4;
                    c.g[A1][0] = c.g[S0][0];
                    call_ptr(c, callee::IS_TEMP_AT_ADDR, 0x0026_1da0)?;
                    let not_temp = c.g[V0][0] == 0;
                    c.g[A1][0] = c.g[S0][0];
                    if not_temp {
                        if back_edge_unwinds(c, 0x0026_1d80) {
                            return Ok(());
                        }
                        pc = 0x0026_1d80;
                    } else {
                        pc = 0x0026_1da8;
                    }
                }
                0x0026_1da8 => {
                    c.cycles += 2;
                    c.g[A0][0] = c.g[S2][0];
                    callee::BUDDY_MAN_REMOVE_TEMP_ENTRY_AT.call(c, 0x0026_1db0)?;
                    pc = 0x0026_1db0;
                }
                0x0026_1db0 => {
                    c.cycles += 28;
                    c.g[A3][0] = c.g[S3][0];
                    c.g[T0][0] = 0;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[A1][0] = c.g[S5][0];
                    c.g[A2][0] = 2;
                    c.g[S0][0] = 0x108;
                    callee::BUDDY_MAN_ADD_ROSTER_ENTRY.call(c, 0x0026_1dcc)?;
                    c.g[S1][0] = lobby_instance(c);
                    c.g[A1][0] = sx(0x0048_0488);
                    c.g[A0][0] = 8;
                    c.g[A2][0] = 0x2000_0000;
                    c.g[A3][0] = 0;
                    call_ptr(c, callee::MEM_ALLOC, 0x0026_1de4)?;
                    c.g[S3][0] = c.g[V0][0];
                    Addr(r32(c, S3)).sw(c, c.g[S0][0]);
                    Addr(r32(c, S3).wrapping_add(4)).sw(c, sx(0x0048_12b0));
                    c.g[A1][0] = sx(0x0048_04a8);
                    c.g[A0][0] = 0xc;
                    c.g[A2][0] = 0x2000_0000;
                    c.g[A3][0] = 0;
                    c.g[V0][0] = Addr(r32(c, S1).wrapping_add(BUDDY_LIST_HEAD)).lw(c);
                    Addr(sp(c).wrapping_add(0x10)).sw(c, c.g[V0][0]);
                    call_ptr(c, callee::MEM_ALLOC, 0x0026_1e10)?;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[V0][0] = sx(r32(c, A1).wrapping_add(8));
                    if c.g[V0][0] != 0 {
                        Addr(r32(c, A1).wrapping_add(8)).sw(c, c.g[S3][0]);
                    }
                    pc = 0x0026_1e20;
                }
                0x0026_1e20 => {
                    c.cycles += 9;
                    c.g[V0][0] = Addr(sp(c).wrapping_add(0x10)).lw(c);
                    Addr(r32(c, A1)).sw(c, c.g[V0][0]);
                    c.g[V1][0] = Addr(r32(c, V0).wrapping_add(4)).lw(c);
                    Addr(r32(c, A1).wrapping_add(4)).sw(c, c.g[V1][0]);
                    c.g[A0][0] = Addr(r32(c, V0).wrapping_add(4)).lw(c);
                    Addr(r32(c, A0)).sw(c, c.g[A1][0]);
                    c.g[V0][0] = Addr(sp(c).wrapping_add(0x10)).lw(c);
                    Addr(r32(c, V0).wrapping_add(4)).sw(c, c.g[A1][0]);
                    Addr(sp(c)).sw(c, c.g[A1][0]);
                    pc = 0x0026_1e44;
                }
                0x0026_1e44 => {
                    c.cycles += 1;
                    c.g[V0][0] = 0x480000;
                    pc = 0x0026_1e48;
                }
                0x0026_1e48 => {
                    c.cycles += 23;
                    c.g[A1][0] = sx(r32(c, V0).wrapping_add(0xe98));
                    c.g[A2][0] = 0x2000_0000;
                    c.g[A3][0] = 0;
                    c.g[A0][0] = 0x10;
                    call_ptr(c, callee::MEM_ALLOC, 0x0026_1e5c)?;
                    c.g[S0][0] = c.g[V0][0];
                    c.g[A1][0] = c.g[S5][0];
                    c.g[A0][0] = c.g[S0][0];
                    Addr(r32(c, S0).wrapping_add(0xc)).sw(c, sx(0x0046_d8c0));
                    call_ptr(c, callee::BXSTRING_CTOR2, 0x0026_1e78)?;
                    c.g[A1][0] = c.g[FP][0];
                    c.g[A0][0] = sx(r32(c, S0).wrapping_add(4));
                    call_ptr(c, callee::BXSTRING_CTOR2, 0x0026_1e84)?;
                    c.g[A1][0] = Addr(sp(c).wrapping_add(0x28)).lw(c);
                    c.g[A0][0] = sx(r32(c, S0).wrapping_add(8));
                    call_ptr(c, callee::BXSTRING_CTOR2, 0x0026_1e90)?;
                    c.g[A1][0] = Addr(r32(c, S2).wrapping_add(BUDDY_MESSAGES_END)).lw(c);
                    c.g[A0][0] = sx(r32(c, S2).wrapping_add(0xa8));
                    c.g[V0][0] = Addr(r32(c, S2).wrapping_add(0xb4)).lw(c);
                    let at_cap = c.g[A1][0] == c.g[V0][0];
                    Addr(sp(c).wrapping_add(0x24)).sw(c, c.g[S0][0]);
                    pc = if at_cap { 0x0026_1ec0 } else { 0x0026_1ea4 };
                }
                0x0026_1ea4 => {
                    c.cycles += 3;
                    c.g[A0][0] = Addr(r32(c, A0).wrapping_add(8)).lw(c);
                    if c.g[A0][0] != 0 {
                        Addr(r32(c, A0)).sw(c, c.g[S0][0]);
                    }
                    pc = 0x0026_1eb0;
                }
                0x0026_1eb0 => {
                    c.cycles += 4;
                    c.g[V0][0] = Addr(r32(c, S2).wrapping_add(BUDDY_MESSAGES_END)).lw(c);
                    c.g[V0][0] = sx(r32(c, V0).wrapping_add(4));
                    Addr(r32(c, S2).wrapping_add(BUDDY_MESSAGES_END)).sw(c, c.g[V0][0]);
                    pc = 0x0026_1ec8;
                }
                0x0026_1ec0 => {
                    c.cycles += 2;
                    c.g[A2][0] = c.g[S4][0];
                    call_ptr(c, callee::MESSAGE_VECTOR_INSERT, 0x0026_1ec8)?;
                    pc = 0x0026_1ec8;
                }
                0x0026_1ec8 => {
                    c.cycles += 12;
                    let frame = Frame { size: 0xd0 };
                    frame.restore(c, S0, 0xc0);
                    frame.restore(c, S1, 0xb0);
                    frame.restore(c, S2, 0xa0);
                    frame.restore(c, S3, 0x90);
                    frame.restore(c, S4, 0x80);
                    frame.restore(c, S5, 0x70);
                    frame.restore(c, S6, 0x60);
                    frame.restore(c, S7, 0x50);
                    frame.restore(c, FP, 0x40);
                    frame.restore_ra(c, 0x30);
                    frame.close(c);
                    return Ok(());
                }
                _ => {
                    c.jump_out(pc, 0x0026_1cd0, 0x0026_1ef8);
                    return Ok(());
                }
            }
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::events::{check_events, driven, schedule, Events};
    use crate::difftest::fakes::machine;
    use crate::difftest::{check, DATA, GAME_GP};
    use crate::game::dirtysock::tagsunk::tests::{oc405_gate, OC405_R_ALLOC, OC405_R_FIND};

    driven!(
        gen_buddy_man_find_roster_entry_no_case,
        ee::s07::f_002624f8__generated
    );
    driven!(
        hand_buddy_man_find_roster_entry_no_case,
        buddy_man_find_roster_entry_no_case
    );
    driven!(
        gen_buddy_man_count_messages_from_at,
        ee::s07::f_00262950__generated
    );
    driven!(
        hand_buddy_man_count_messages_from_at,
        buddy_man_count_messages_from_at
    );

    /// Gate for the `strcmp` fake used by the count-messages events test:
    /// a word in the machine's RAM. With it off, the hooked slot runs the
    /// body it held before.
    pub(crate) const STRCMP_CTRL: u32 = DATA + 0xf_fe00;

    pub(crate) const STRCMP_MAGIC: u32 = 0x7374_7263;

    /// Native byte compare standing in for `strcmp` (0x4165a8) inside
    /// `drive`: the generated body has an event check in its word loop, so
    /// a due event can unwind mid-call, and `drive` cannot re-enter a
    /// structured port from a nested unwind (it would restart the prologue
    /// and return to a stale `RA`). The fake has no event checks, so
    /// unwinds surface only at the loop head under test; both sides call
    /// through the hooked slot, so the call sequence is identical. 0 when
    /// equal, 1 otherwise (the port only tests `== 0`).
    fn strcmp_fake(c: &mut Cpu) {
        use crate::difftest::fakes;
        if c.bus.read32(STRCMP_CTRL) != STRCMP_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(strcmp_fake),
                ee::s15::f_004165a8__generated,
            );
        }
        let (mut a, mut b) = (c.g[A0][0] as u32, c.g[A1][0] as u32);
        c.g[V0][0] = loop {
            let (x, y) = (c.bus.read8(a), c.bus.read8(b));
            if x != y || x == 0 {
                break (x != y) as u64;
            }
            a += 1;
            b += 1;
        };
        c.pc = r32(c, RA);
    }

    /// Install the `strcmp` fake once per process (the hook is global; the
    /// gate word keeps it off for every other test's machines).
    fn strcmp_install() {
        use crate::difftest::fakes;
        use std::ptr::addr_of_mut;
        fakes::hook(
            addr_of_mut!(ee::s15::OVERRIDE_004165A8),
            strcmp_fake,
            fakes::prev_cell(strcmp_fake),
        );
    }
    driven!(
        gen_buddy_man_remove_temp_entry_at,
        ee::s07::f_00262ec0__generated
    );
    driven!(
        hand_buddy_man_remove_temp_entry_at,
        buddy_man_remove_temp_entry_at
    );
    driven!(
        gen_buddy_man_on_challenge_received,
        ee::s07::f_00262af0__generated
    );
    driven!(
        hand_buddy_man_on_challenge_received,
        buddy_man_on_challenge_received
    );
    driven!(gen_buddy_man_add_message, ee::s06::f_00261cd0__generated);
    driven!(hand_buddy_man_add_message, buddy_man_add_message);
    fn put_str(c: &mut Cpu, at: u32, text: &[u8]) {
        for (i, b) in text.iter().enumerate() {
            c.bus.write8(at + i as u32, *b);
        }
        c.bus.write8(at + text.len() as u32, 0);
    }

    /// A BuddyMan at DATA: roster list (2 entries alice/BOB), message vector,
    /// challenger vector, session and lobby objects, BELibrary dummy.
    /// Branch steering is by seed residues at each call site.
    fn buddy_fixture(c: &mut Cpu, seed: u32) {
        // Roster list at +0xcc: count at +0x18, entry array at +0x40.
        let list = DATA + 0x3000;
        let array = DATA + 0x3100;
        c.bus.write32(DATA + BUDDY_ROSTER, list);
        c.bus.write32(list + 0x40, array);
        let names: [&[u8]; 2] = [b"alice", b"BOB"];
        for i in 0..2u32 {
            let entry = DATA + 0x3200 + 0x40 * i;
            let name = DATA + 0x3400 + 0x20 * i;
            put_str(c, name, names[i as usize]);
            c.bus.write32(array + 4 * i, entry);
            c.bus.write32(entry, name);
            // Entry 0 permanent, entry 1 temporary (bit 20).
            c.bus.write32(
                entry + BUDDY_ENTRY_FLAGS,
                if i == 1 { 0x10_0000 } else { 0 },
            );
        }
        c.bus.write32(list + 0x18, 2);
        // Messages: three records, senders alice / carol / alice.
        let msgs = DATA + 0x2000;
        c.bus.write32(DATA + BUDDY_MESSAGES_BEGIN, msgs);
        c.bus.write32(DATA + BUDDY_MESSAGES_END, msgs + 12);
        c.bus.write32(DATA + 0xb4, msgs + 12);
        let senders: [&[u8]; 3] = [b"alice", b"carol", b"alice"];
        for i in 0..3u32 {
            let msg = DATA + 0x2200 + 0x40 * i;
            let name = DATA + 0x2400 + 0x20 * i;
            put_str(c, name, senders[i as usize]);
            c.bus.write32(msgs + 4 * i, msg);
            c.bus.write32(msg, name);
        }
        // Challengers: one record named alice.
        let chals = DATA + 0x2600;
        let chal = DATA + 0x2800;
        let chal_name = DATA + 0x2900;
        put_str(c, chal_name, b"alice");
        c.bus.write32(DATA + BUDDY_CHALLENGERS_BEGIN, chals);
        c.bus.write32(DATA + BUDDY_CHALLENGERS_END, chals + 4);
        c.bus.write32(DATA + 0xc4, chals + 4);
        c.bus.write32(chals, chal);
        c.bus.write32(chal, chal_name);
        // Name/id pointer and scalar field.
        let own_name = DATA + 0x3500;
        put_str(c, own_name, b"me");
        c.bus.write32(DATA + BUDDY_NAME_PTR, own_name);
        c.bus.write32(DATA + BUDDY_FIELD_A4, seed % 7);
        // Session object (hash table at +0x4a4 left null).
        let session = DATA + 0x4000;
        c.bus.write32(DATA + BUDDY_SESSION, session);
        c.bus.write32(session + BUDDY_LIST_HEAD, DATA + 0x4100);
        c.bus.write32(DATA + 0x4100 + 4, DATA + 0x4200);
        // Lobby singleton and BELibrary dummy.
        let lobby = DATA + 0x1000;
        c.bus
            .write32(GAME_GP.wrapping_add(LOBBY_INSTANCE_GP as u32), lobby);
        c.bus.write32(lobby + BUDDY_LIST_HEAD, DATA + 0x1200);
        c.bus.write32(DATA + 0x1200 + 4, DATA + 0x1300);
        let lobby_name = DATA + 0x1100;
        put_str(c, lobby_name, b"lobby");
        c.bus.write32(lobby + 0x80, lobby_name);
        c.bus.write32(GAME_GP.wrapping_sub(0x1f04), DATA + 0x6000);
    }

    #[test]
    fn buddy_man_has_pending_challenge_from_matches() {
        check(
            ee::s07::f_00262ab0__generated,
            buddy_man_has_pending_challenge_from,
            40,
            &|c, seed| {
                let _r = machine(c, seed, 0x262ab0, 0);
                buddy_fixture(c, seed);
                c.pc = 0x0026_2ab0;
                // seed % 2: empty vs non-empty message list.
                if seed % 2 == 0 {
                    c.bus.write32(DATA + BUDDY_MESSAGES_END, DATA + 0x2000);
                }
                let name_ref = DATA + 0x900;
                c.bus.write32(name_ref, DATA + 0x3400);
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = name_ref as u64;
            },
        );
    }

    #[test]
    fn buddy_man_find_roster_entry_no_case_matches() {
        check(
            ee::s07::f_002624f8__generated,
            buddy_man_find_roster_entry_no_case,
            60,
            &|c, seed| {
                let _r = machine(c, seed, 0x2624f8, 0);
                buddy_fixture(c, seed);
                c.pc = 0x0026_24f8;
                // seed % 5: null name / empty roster / match 0 / match 1 / no match.
                match seed % 5 {
                    0 => c.g[A1][0] = 0,
                    1 => c.bus.write32(DATA + 0x3000 + 0x18, 0),
                    _ => {}
                }
                if seed % 5 != 0 {
                    let query = DATA + 0x900;
                    let text: &[u8] = match seed % 5 {
                        2 => b"ALICE",
                        3 => b"bob",
                        _ => b"dave",
                    };
                    put_str(c, DATA + 0x910, text);
                    c.bus.write32(query, DATA + 0x910);
                    c.g[A1][0] = query as u64;
                }
                c.g[A0][0] = DATA as u64;
            },
        );
    }

    #[test]
    fn buddy_man_find_roster_entry_no_case_events() {
        let setup = |c: &mut Cpu, seed: u32| -> Events {
            let mut r = machine(c, seed, 0x2624f8, 0);
            buddy_fixture(c, seed);
            let query = DATA + 0x900;
            put_str(c, DATA + 0x910, b"dave");
            c.bus.write32(query, DATA + 0x910);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = query as u64;
            schedule(c, &mut r, seed, 0x0026_24f8)
        };
        check_events(
            gen_buddy_man_find_roster_entry_no_case,
            hand_buddy_man_find_roster_entry_no_case,
            30,
            &setup,
        );
    }

    #[test]
    fn buddy_man_count_messages_from_at_matches() {
        check(
            ee::s07::f_00262950__generated,
            buddy_man_count_messages_from_at,
            60,
            &|c, seed| {
                let _r = machine(c, seed, 0x262950, 0);
                buddy_fixture(c, seed);
                c.pc = 0x0026_2950;
                c.g[A0][0] = DATA as u64;
                // seed % 4: entry 0 / entry 1 / out of range / no messages.
                c.g[A1][0] = match seed % 4 {
                    0 => 0,
                    1 => 1,
                    2 => 5,
                    _ => 0,
                } as u64;
                if seed % 4 == 3 {
                    c.bus.write32(DATA + BUDDY_MESSAGES_END, DATA + 0x2000);
                }
            },
        );
    }

    #[test]
    fn buddy_man_count_messages_from_at_events() {
        let setup = |c: &mut Cpu, seed: u32| -> Events {
            let mut r = machine(c, seed, 0x262950, 0);
            buddy_fixture(c, seed);
            // Route the in-loop `strcmp` through the gated fake (no event
            // checks inside, so no nested unwind can break `drive`); the
            // loop-head check under test stays real.
            strcmp_install();
            c.bus.write32(STRCMP_CTRL, STRCMP_MAGIC);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (seed % 2) as u64;
            schedule(c, &mut r, seed, 0x0026_2950)
        };
        check_events(
            gen_buddy_man_count_messages_from_at,
            hand_buddy_man_count_messages_from_at,
            30,
            &setup,
        );
    }

    #[test]
    fn buddy_man_send_message_matches() {
        check(
            ee::s07::f_00262690__generated,
            buddy_man_send_message,
            30,
            &|c, seed| {
                let _r = machine(c, seed, 0x262690, 0);
                buddy_fixture(c, seed);
                c.pc = 0x0026_2690;
                put_str(c, DATA + 0x910, b"alice");
                put_str(c, DATA + 0x920, b"hello there");
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x910) as u64;
                c.g[A2][0] = (DATA + 0x920) as u64;
            },
        );
    }

    #[test]
    fn buddy_man_set_presence_matches() {
        check(
            ee::s07::f_00262768__generated,
            buddy_man_set_presence,
            60,
            &|c, seed| {
                let _r = machine(c, seed, 0x262768, 0);
                buddy_fixture(c, seed);
                c.pc = 0x0026_2768;
                // seed % 2: no session vs session.
                if seed % 2 == 0 {
                    c.bus.write32(DATA + BUDDY_SESSION, 0);
                }
                put_str(c, DATA + 0x910, b"away");
                put_str(c, DATA + 0x920, b"in game");
                put_str(c, DATA + 0x930, b"peak");
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (seed % 3) as u64;
                c.g[A2][0] = (seed % 5) as u64;
                // Different modulus per optional string.
                c.g[A3][0] = if seed % 3 == 0 {
                    0
                } else {
                    (DATA + 0x910) as u64
                };
                c.g[T0][0] = if seed % 5 == 0 {
                    0
                } else {
                    (DATA + 0x920) as u64
                };
                c.g[T1][0] = if seed % 7 == 0 {
                    0
                } else {
                    (DATA + 0x930) as u64
                };
            },
        );
    }

    #[test]
    fn buddy_man_add_roster_entry_matches() {
        check(
            ee::s07::f_002620d8__generated,
            buddy_man_add_roster_entry,
            72,
            &|c, seed| {
                let _r = machine(c, seed, 0x2620d8, 0);
                buddy_fixture(c, seed);
                c.pc = 0x0026_20d8;
                // seed % 6 == 0: lobby name equals the name (early error 3).
                if seed % 6 == 0 {
                    c.bus.write32(DATA + 0x1000 + 0x80, DATA + 0x910);
                }
                put_str(c, DATA + 0x910, b"alice");
                let err = DATA + 0x900;
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x910) as u64;
                // seed % 4: kind 0 / 1 / 2 / invalid 3.
                c.g[A2][0] = (seed % 4) as u64;
                c.g[A3][0] = err as u64;
                // seed % 3: no other / permanent other / temporary other.
                c.g[T0][0] = match seed % 3 {
                    0 => 0,
                    1 => (DATA + 0x3200) as u64,
                    _ => (DATA + 0x3240) as u64,
                };
            },
        );
    }

    #[test]
    fn buddy_man_change_roster_entry_matches() {
        check(
            ee::s07::f_00261ef8__generated,
            buddy_man_change_roster_entry,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x261ef8, 0);
                buddy_fixture(c, seed);
                c.pc = 0x0026_1ef8;
                put_str(c, DATA + 0x910, b"alice");
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x910) as u64;
                // seed % 4: kind 0 / 1 / 2 / invalid 3 (session lookup misses:
                // the session hash table at +0x4a4 is null in this fixture).
                c.g[A2][0] = (seed % 4) as u64;
                c.g[A3][0] = (DATA + 0x900) as u64;
            },
        );
    }

    #[test]
    fn buddy_man_remove_temp_entry_at_matches() {
        check(
            ee::s07::f_00262ec0__generated,
            buddy_man_remove_temp_entry_at,
            60,
            &|c, seed| {
                let _r = machine(c, seed, 0x262ec0, 0);
                buddy_fixture(c, seed);
                c.pc = 0x0026_2ec0;
                c.g[A0][0] = DATA as u64;
                // seed % 3: permanent entry / temporary entry / out of range.
                c.g[A1][0] = (seed % 3) as u64;
            },
        );
    }

    #[test]
    fn buddy_man_remove_temp_entry_at_events() {
        let setup = |c: &mut Cpu, seed: u32| -> Events {
            let mut r = machine(c, seed, 0x262ec0, 0);
            buddy_fixture(c, seed);
            // The temporary entry (index 1) must carry messages or the
            // delete loop at 0x262f50 is never entered. GetRosterEntry
            // returns the entry's name pointer, and IsTempAt reads the
            // flags word at name+0xa8: point entry 1 at the "alice"
            // string (two fixture messages are from "alice") and set
            // the temporary bit (bit 20) at alice+0xa8.
            c.bus.write32(DATA + 0x3240, DATA + 0x3400);
            c.bus.write32(DATA + 0x3400 + 0xa8, 0x10_0000);
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = 1;
            schedule(c, &mut r, seed, 0x0026_2ec0)
        };
        check_events(
            gen_buddy_man_remove_temp_entry_at,
            hand_buddy_man_remove_temp_entry_at,
            30,
            &setup,
        );
    }

    #[test]
    fn buddy_man_on_challenge_received_matches() {
        check(
            ee::s07::f_00262af0__generated,
            buddy_man_on_challenge_received,
            48,
            &|c, seed| {
                let _r = machine(c, seed, 0x262af0, 0);
                buddy_fixture(c, seed);
                oc405_gate(c);
                // Allocator fake returns one scratch record.
                c.bus.write32(OC405_R_ALLOC, DATA + 0x5000);
                // seed % 4 == 0: roster lookup misses (add path); else found.
                c.bus
                    .write32(OC405_R_FIND, if seed % 4 == 0 { u32::MAX } else { 0 });
                c.pc = 0x0026_2af0;
                put_str(c, DATA + 0x910, b"alice");
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x910) as u64;
                c.g[A2][0] = (seed % 3) as u64;
                c.g[A3][0] = (DATA + 0x920) as u64;
            },
        );
    }

    #[test]
    fn buddy_man_on_challenge_received_events() {
        let setup = |c: &mut Cpu, seed: u32| -> Events {
            let mut r = machine(c, seed, 0x262af0, 0);
            buddy_fixture(c, seed);
            oc405_gate(c);
            c.bus.write32(OC405_R_ALLOC, DATA + 0x5000);
            c.bus.write32(OC405_R_FIND, 0);
            put_str(c, DATA + 0x910, b"alice");
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA + 0x910) as u64;
            schedule(c, &mut r, seed, 0x0026_2af0)
        };
        check_events(
            gen_buddy_man_on_challenge_received,
            hand_buddy_man_on_challenge_received,
            30,
            &setup,
        );
    }

    #[test]
    fn buddy_man_add_message_matches() {
        check(
            ee::s06::f_00261cd0__generated,
            buddy_man_add_message,
            50,
            &|c, seed| {
                let _r = machine(c, seed, 0x261cd0, 0);
                buddy_fixture(c, seed);
                oc405_gate(c);
                c.bus.write32(OC405_R_ALLOC, DATA + 0x5000);
                c.pc = 0x0026_1cd0;
                put_str(c, DATA + 0x910, b"alice");
                put_str(c, DATA + 0x920, b"hello");
                put_str(c, DATA + 0x930, b"today");
                // seed % 5: sender alice (roster) vs dave (not on roster).
                if seed % 5 == 4 {
                    put_str(c, DATA + 0x910, b"dave");
                }
                // seed % 2: message vector at capacity vs with room.
                if seed % 2 == 0 {
                    c.bus.write32(DATA + 0xb4, DATA + 0x2000 + 12);
                } else {
                    c.bus.write32(DATA + 0xb4, DATA + 0x2000 + 4);
                }
                c.g[A0][0] = DATA as u64;
                c.g[A1][0] = (DATA + 0x910) as u64;
                c.g[A2][0] = (DATA + 0x920) as u64;
                c.g[A3][0] = (DATA + 0x930) as u64;
            },
        );
    }

    #[test]
    fn buddy_man_add_message_events() {
        let setup = |c: &mut Cpu, seed: u32| -> Events {
            let mut r = machine(c, seed, 0x261cd0, 0);
            buddy_fixture(c, seed);
            oc405_gate(c);
            c.bus.write32(OC405_R_ALLOC, DATA + 0x5000);
            put_str(c, DATA + 0x910, b"dave");
            put_str(c, DATA + 0x920, b"hello");
            put_str(c, DATA + 0x930, b"today");
            c.g[A0][0] = DATA as u64;
            c.g[A1][0] = (DATA + 0x910) as u64;
            c.g[A2][0] = (DATA + 0x920) as u64;
            c.g[A3][0] = (DATA + 0x930) as u64;
            schedule(c, &mut r, seed, 0x0026_1cd0)
        };
        check_events(
            gen_buddy_man_add_message,
            hand_buddy_man_add_message,
            30,
            &setup,
        );
    }
}
