//! Staging module oc1008 (wave wave): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1008`.

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
    (0x0025_a1d0, lobby_man_on_game_start, "LobbyMan_OnGameStart"),
    (
        0x0025_9a60,
        lobby_man_on_rank_list_changed,
        "LobbyMan_OnRankListChanged",
    ),
    (
        0x0025_99b0,
        lobby_man_on_room_list_changed,
        "LobbyMan_OnRoomListChanged",
    ),
    (0x0025_9628, lobby_man_dtor, "LobbyMan_dtor"),
    (0x0025_94d0, lobby_man_ctor, "LobbyMan_ctor"),
    (
        0x0025_9468,
        lobby_man_compare_rank_asc,
        "LobbyMan_CompareRankAsc",
    ),
    (
        0x0025_9400,
        lobby_man_compare_rank_desc,
        "LobbyMan_CompareRankDesc",
    ),
];

// ---------------------------------------------------------------------------
// Callees.

pub(crate) mod callee {
    use super::ee;
    use super::Callee;

    /// `cDirtysock_tag_TagFieldFind` (0x3eb6d8), via `call_indirect`.
    pub const TAG_FIELD_FIND: u32 = 0x003e_b6d8;
    /// `TagFieldGetString4` (0x3ed378), via `call_indirect`.
    pub const TAG_FIELD_GET_STRING4: u32 = 0x003e_d378;
    /// `TagFieldGetTime` (0x3edb08), via `call_indirect`.
    pub const TAG_FIELD_GET_TIME: u32 = 0x003e_db08;
    /// `TagFieldGetNumber` (0x3ecf30), via `call_indirect`.
    pub const TAG_FIELD_GET_NUMBER: u32 = 0x003e_cf30;
    /// `TagFieldGetAddr` (0x3ecff8), via `call_indirect`.
    pub const TAG_FIELD_GET_ADDR: u32 = 0x003e_cff8;
    /// `TagFieldGetStructure`/`TagFieldGetBinary` (0x3ed678), via `call_indirect`.
    pub const TAG_FIELD_GET_STRUCTURE: u32 = 0x003e_d678;
    /// `cMemMan_alloc` (0x317d70), via `call_indirect`.
    pub const MEM_ALLOC_NAMED: u32 = 0x0031_7d70;
    /// `cBXString_cBXString4` (0x3184e8), via `call_indirect`.
    pub const BXSTRING_CTOR4: u32 = 0x0031_84e8;
    /// `cBXString_cBXString2` (0x3182d8), via `call_indirect`.
    pub const BXSTRING_CTOR2: u32 = 0x0031_82d8;
    /// `cBXString_cBXString` plain ctor thunk (0x268890), via `call_indirect`.
    pub const BXSTRING_CTOR_PLAIN: u32 = 0x0026_8890;
    /// `cBXString__cBXString` (0x318260), via `call_indirect`.
    pub const BXSTRING_DTOR: u32 = 0x0031_8260;
    /// `strcmp`-like compare (0x4165a8), via `call_indirect`.
    pub const STR_CMP_4165A8: u32 = 0x0041_65a8;
    /// `strlen` (0x416810), via `call_indirect`.
    pub const STRLEN_416810: u32 = 0x0041_6810;
    /// `cBE_getBE` (0x14dc80), via `call_indirect`.
    pub const BE_GET_BE: u32 = 0x0014_dc80;
    /// `cBE_getInterface` (0x14dd58), via `call_indirect`.
    pub const BE_GET_INTERFACE: u32 = 0x0014_dd58;
    /// `cBENewRaceInterface_setGameMode` (0x144df0), via `call_indirect`.
    pub const BE_SET_GAME_MODE: u32 = 0x0014_4df0;
    /// `cBENewRaceInterface_setGameEvent` (0x1451e8), via `call_indirect`.
    pub const BE_SET_GAME_EVENT: u32 = 0x0014_51e8;
    /// `cBENewRaceInterface_setNumberAI` (0x144d98), via `call_indirect`.
    pub const BE_SET_NUMBER_AI: u32 = 0x0014_4d98;
    /// `func_00263BA8` (0x263ba8), via `call_indirect`.
    pub const FUNC_263BA8: u32 = 0x0026_3ba8;
    /// `operator delete` (0x317e50), via `call_indirect`.
    pub const OPERATOR_DELETE: u32 = 0x0031_7e50;
    /// List destructor (0x1db718), via `call_indirect`.
    pub const LIST_DTOR_1DB718: u32 = 0x001d_b718;

    pub const LOOKUP_PEER_ADDR: Callee = Callee {
        addr: 0x0025_a730,
        f: ee::s06::LobbyMan_LookupPeerAddr,
    };
    pub const NET_GET_GAME_SETUP: Callee = Callee {
        addr: 0x0025_91d0,
        f: ee::s06::Net_GetGameSetup,
    };
    pub const SETUP_NET_RACE: Callee = Callee {
        addr: 0x0025_9098,
        f: ee::s06::LobbyMan_SetupNetRace,
    };
    pub const RESET_ACCOUNT_INFO: Callee = Callee {
        addr: 0x0026_0f10,
        f: ee::s06::LobbyMan_ResetAccountInfo,
    };
    pub const LOBBY_DISCONNECT: Callee = Callee {
        addr: 0x0025_b800,
        f: ee::s06::LobbyMan_Disconnect,
    };
}

/// `D_00480488`: allocation tag for event objects.
pub(crate) const ALLOC_TAG: u32 = 0x0048_0488;
/// `D_004804A8`: allocation tag for the 12-byte list node.
pub(crate) const ALLOC_TAG_NODE: u32 = 0x0048_04a8;
/// `D_004812B0`: vtable word stored in 8-byte event objects.
pub(crate) const EVENT_VT: u32 = 0x0048_12b0;
/// Tag-format string at `D_004A2F30` used by the rank comparators.
pub(crate) const RANK_TAG_FMT: u32 = 0x004a_2f30;

/// Splice the 8-byte event in `s_event` into the list headed by the word
/// saved at `frame+0x10` (the generated epilogue shared by the two
/// list-changed handlers and the tail of `LobbyMan_OnGameStart`).
/// 14 cycles in the list handlers; the caller charges them.
fn lobby_event_splice(
    c: &mut Cpu,
    frame: Frame,
    saved_s0: usize,
    saved_s1: usize,
    s0_off: u32,
    s1_off: u32,
) {
    c.g[V0][0] = Addr(frame.at(c, 0x10)).lw(c);
    frame.restore(c, saved_s0, s0_off);
    Addr(r32(c, A1)).sw(c, c.g[V0][0]);
    frame.restore(c, saved_s1, s1_off);
    c.g[V1][0] = Addr(r32(c, V0).wrapping_add(4)).lw(c);
    frame.restore_ra(c, 0x20);
    Addr(r32(c, A1).wrapping_add(4)).sw(c, c.g[V1][0]);
    c.g[A0][0] = Addr(r32(c, V0).wrapping_add(4)).lw(c);
    Addr(r32(c, A0)).sw(c, c.g[A1][0]);
    c.g[V0][0] = Addr(frame.at(c, 0x10)).lw(c);
    Addr(r32(c, V0).wrapping_add(4)).sw(c, c.g[A1][0]);
    Addr(frame.at(c, 0)).sw(c, c.g[A1][0]);
    frame.close(c);
}

/// Shared body of the room/rank list-changed handlers: allocate the 8-byte
/// event (`kind`, vtable `D_004812B0`), wrap it in a 12-byte node and splice
/// it at the lobby list head at self+0xF0. Only `kind` and the two return
/// addresses differ between the siblings.
fn lobby_list_changed(c: &mut Cpu, kind: u64, ret_alloc8: u32, ret_alloc12: u32) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x50);
        c.cycles += 29;
        c.g[A1][0] = sx(0x0048_0000);
        frame.save(c, S0, 0x40);
        c.g[A1][0] = sx(ALLOC_TAG);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x30);
        c.g[A2][0] = 0x2000_0000;
        frame.save_ra(c, 0x20);
        c.g[A0][0] = 8;
        c.g[A3][0] = 0;
        call_ptr(c, callee::MEM_ALLOC_NAMED, ret_alloc8)?;
        c.g[S1][0] = c.g[V0][0];
        c.g[A2][0] = Addr(r32(c, S0).wrapping_add(0xf0)).lw(c);
        c.g[V0][0] = sx(0x0048_0000);
        c.g[V1][0] = kind;
        c.g[V0][0] = sx(EVENT_VT);
        Addr(r32(c, S1)).sw(c, c.g[V1][0]);
        Addr(r32(c, S1).wrapping_add(4)).sw(c, c.g[V0][0]);
        c.g[A1][0] = sx(0x0048_0000);
        Addr(frame.at(c, 0x10)).sw(c, c.g[A2][0]);
        c.g[A1][0] = sx(ALLOC_TAG_NODE);
        c.g[A0][0] = 0xc;
        c.g[A2][0] = 0x2000_0000;
        c.g[A3][0] = 0;
        call_ptr(c, callee::MEM_ALLOC_NAMED, ret_alloc12)?;
        c.g[A1][0] = c.g[V0][0];
        c.g[V0][0] = sx(r32(c, A1).wrapping_add(8));
        if c.g[V0][0] != 0 {
            Addr(r32(c, A1).wrapping_add(8)).sw(c, c.g[S1][0]);
        }
        c.cycles += 14;
        lobby_event_splice(c, frame, S0, S1, 0x40, 0x30);
        Ok(())
    })
}

/// `void LobbyMan_OnRoomListChanged(self)` (0x2599b0): post a kind-0xE0
/// room-list-changed event to the lobby message list at self+0xF0.
pub fn lobby_man_on_room_list_changed(c: &mut Cpu) {
    lobby_list_changed(c, 0xe0, 0x0025_99dc, 0x0025_9a14);
}

/// `void LobbyMan_OnRankListChanged(self)` (0x259a60): post a kind-0xE1
/// rank-list-changed event to the lobby message list at self+0xF0.
pub fn lobby_man_on_rank_list_changed(c: &mut Cpu) {
    lobby_list_changed(c, 0xe1, 0x0025_9a8c, 0x0025_9ac4);
}

/// Shared body of the rank comparators: decode the 8-byte rank structure at
/// entry+0x28 of each operand (`a2`, `a3`) with the format at `D_004A2F30`
/// into stack buffers at sp+0 / sp+0x10, then subtract the first words.
/// `desc` swaps which buffer is the minuend.
fn lobby_compare_rank(c: &mut Cpu, ret1: u32, ret2: u32, desc: bool) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x50);
        c.cycles += 25;
        c.g[A0][0] = sx(r32(c, A2).wrapping_add(0x28));
        frame.save(c, S0, 0x40);
        c.g[A1][0] = c.g[SP][0];
        frame.save(c, S1, 0x30);
        c.g[S0][0] = sx(RANK_TAG_FMT);
        c.g[S1][0] = c.g[A3][0];
        frame.save_ra(c, 0x20);
        c.g[A3][0] = c.g[S0][0];
        c.g[A2][0] = 8;
        call_ptr(c, callee::TAG_FIELD_GET_STRUCTURE, ret1)?;
        c.g[A0][0] = sx(r32(c, S1).wrapping_add(0x28));
        c.g[A3][0] = c.g[S0][0];
        c.g[A1][0] = sx(frame.at(c, 0x10));
        c.g[A2][0] = 8;
        call_ptr(c, callee::TAG_FIELD_GET_STRUCTURE, ret2)?;
        let (first_word, second_word) = if desc {
            (Addr(frame.at(c, 0x10)).lw(c), Addr(frame.at(c, 0)).lw(c))
        } else {
            (Addr(frame.at(c, 0)).lw(c), Addr(frame.at(c, 0x10)).lw(c))
        };
        // Generated order: desc loads v1=second-buffer, v0=first-buffer;
        // asc loads v1=first, v0=second; both compute v0 = v1 - v0.
        if desc {
            c.g[V1][0] = first_word;
            c.g[V0][0] = second_word;
        } else {
            c.g[V1][0] = first_word;
            c.g[V0][0] = second_word;
        }
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        c.g[V0][0] = sx((c.g[V1][0] as u32).wrapping_sub(c.g[V0][0] as u32));
        frame.restore_ra(c, 0x20);
        frame.close(c);
        Ok(())
    })
}

/// `int LobbyMan_CompareRankDesc(ctx, a, b)` (0x259400): rank of `b` minus
/// rank of `a` (descending order), from the structures at a+0x28 / b+0x28.
pub fn lobby_man_compare_rank_desc(c: &mut Cpu) {
    lobby_compare_rank(c, 0x0025_9430, 0x0025_9444, true);
}

/// `int LobbyMan_CompareRankAsc(ctx, a, b)` (0x259468): rank of `a` minus
/// rank of `b` (ascending order), from the structures at a+0x28 / b+0x28.
pub fn lobby_man_compare_rank_asc(c: &mut Cpu) {
    lobby_compare_rank(c, 0x0025_9498, 0x0025_94ac, false);
}

// ---------------------------------------------------------------------------
// LobbyMan_ctor (0x2594d0), with resume heads 0x259538 / 0x2595e8.

/// First ctor loop (head 0x259538): store the empty-string pointer at
/// v1 = self+0x70, +0x74, +0x78, +0x7c while a0 counts 3 down to -1.
/// Returns Ok(true) when an event unwound at the back edge.
fn lobby_ctor_loop1(c: &mut Cpu) -> Result<bool, crate::call::Unwound> {
    loop {
        c.cycles += 7;
        Addr(r32(c, V1)).sw(c, c.g[A1][0]);
        c.g[A0][0] = sx((r32(c, A0)).wrapping_sub(1));
        let more = c.g[A0][0] != c.g[A2][0];
        c.g[V1][0] = sx(r32(c, V1).wrapping_add(4));
        if more {
            if back_edge_unwinds(c, 0x0025_9538) {
                return Ok(true);
            }
            continue;
        }
        return Ok(false);
    }
}

/// Second ctor loop (head 0x2595e8): zero 29 words descending from
/// v0 = self+0x25c while v1 counts 0x1c down to -1.
fn lobby_ctor_loop2(c: &mut Cpu) -> Result<bool, crate::call::Unwound> {
    loop {
        c.cycles += 7;
        Addr(r32(c, V0)).sw(c, 0);
        c.g[V1][0] = sx((r32(c, V1)).wrapping_sub(1));
        let more = (c.g[V1][0] as i64) >= 0;
        c.g[V0][0] = sx(r32(c, V0).wrapping_sub(4));
        if more {
            if back_edge_unwinds(c, 0x0025_95e8) {
                return Ok(true);
            }
            continue;
        }
        return Ok(false);
    }
}

/// Ctor epilogue: reset account info, return self in v0, restore s0/s1/ra.
fn lobby_ctor_finish(c: &mut Cpu, frame: Frame) -> crate::call::Flow {
    c.cycles += 8;
    c.g[A0][0] = c.g[S1][0];
    callee::RESET_ACCOUNT_INFO.call(c, 0x0025_960c)?;
    c.g[V0][0] = c.g[S1][0];
    frame.restore(c, S0, 0x30);
    frame.restore(c, S1, 0x20);
    frame.restore_ra(c, 0x10);
    frame.close(c);
    Ok(())
}

/// Ctor from the second loop on (shared by the entry and loop1-resume paths).
fn lobby_ctor_after_loop1(c: &mut Cpu, frame: Frame) -> crate::call::Flow {
    c.cycles += 37;
    c.g[V0][0] = Addr(r32(c, GP).wrapping_add(0xda0)).lw(c);
    c.g[A1][0] = sx(0x0048_07d8);
    c.g[A0][0] = sx(r32(c, S1).wrapping_add(0xe8));
    Addr(r32(c, S1).wrapping_add(0x9c)).sw(c, c.g[V0][0]);
    c.g[S0][0] = sx(r32(c, S1).wrapping_add(0xec));
    Addr(r32(c, S1).wrapping_add(0x80)).sw(c, c.g[V0][0]);
    Addr(r32(c, S1).wrapping_add(0x84)).sw(c, c.g[V0][0]);
    Addr(r32(c, S1).wrapping_add(0x88)).sw(c, c.g[V0][0]);
    Addr(r32(c, S1).wrapping_add(0x8c)).sw(c, c.g[V0][0]);
    Addr(r32(c, S1).wrapping_add(0x90)).sw(c, c.g[V0][0]);
    Addr(r32(c, S1).wrapping_add(0x94)).sw(c, c.g[V0][0]);
    Addr(r32(c, S1).wrapping_add(0x98)).sw(c, c.g[V0][0]);
    call_ptr(c, callee::BXSTRING_CTOR2, 0x0025_958c)?;
    c.g[A1][0] = sx(ALLOC_TAG_NODE);
    c.g[A2][0] = 0x2000_0000;
    c.g[A3][0] = 0;
    c.g[A0][0] = 0xc;
    call_ptr(c, callee::MEM_ALLOC_NAMED, 0x0025_95a4)?;
    Addr(r32(c, S1).wrapping_add(0xf0)).sw(c, c.g[V0][0]);
    c.g[A0][0] = sx(r32(c, S1).wrapping_add(0x260));
    c.g[V1][0] = Addr(r32(c, S0).wrapping_add(4)).lw(c);
    Addr(r32(c, V1)).sw(c, c.g[V0][0]);
    c.g[A1][0] = Addr(r32(c, S0).wrapping_add(4)).lw(c);
    c.g[V0][0] = Addr(r32(c, S1).wrapping_add(0xf0)).lw(c);
    Addr(r32(c, A1).wrapping_add(4)).sw(c, c.g[V0][0]);
    Addr(r32(c, S1).wrapping_add(0xf4)).sw(c, 0);
    Addr(r32(c, S1).wrapping_add(0xf8)).sw(c, 0);
    Addr(r32(c, S1).wrapping_add(0xfc)).sw(c, 0);
    Addr(r32(c, S1).wrapping_add(0x100)).sw(c, 0);
    call_ptr(c, callee::BXSTRING_CTOR_PLAIN, 0x0025_95d4)?;
    c.g[A0][0] = sx(r32(c, S1).wrapping_add(0x2b0));
    call_ptr(c, callee::BXSTRING_CTOR_PLAIN, 0x0025_95dc)?;
    c.g[V1][0] = 0x1c;
    c.g[V0][0] = sx(r32(c, S1).wrapping_add(0x25c));
    if lobby_ctor_loop2(c)? {
        return Ok(());
    }
    lobby_ctor_finish(c, frame)
}

/// `LobbyMan* LobbyMan_ctor(self)` (0x2594d0): zero the lobby manager, point
/// every embedded string at the empty-string global (`gp+0xda0`), construct
/// the strings at +0xE8/+0x260/+0x2B0, link the message list at +0xEC, zero
/// the account block at +0x1EC..+0x25C and reset the account info.
pub fn lobby_man_ctor(c: &mut Cpu) {
    match c.pc {
        0x0025_9538 => guest(c, |c| {
            let frame = Frame { size: 0x40 };
            if lobby_ctor_loop1(c)? {
                return Ok(());
            }
            lobby_ctor_after_loop1(c, frame)
        }),
        0x0025_95e8 => guest(c, |c| {
            let frame = Frame { size: 0x40 };
            if lobby_ctor_loop2(c)? {
                return Ok(());
            }
            lobby_ctor_finish(c, frame)
        }),
        0x0025_94d0 => guest(c, |c| {
            let frame = Frame::open(c, 0x40);
            c.cycles += 26;
            c.g[V0][0] = Addr(r32(c, GP).wrapping_add(0xda0)).lw(c);
            frame.save(c, S1, 0x20);
            c.g[A2][0] = 0xffff_ffff_ffff_ffff;
            c.g[S1][0] = c.g[A0][0];
            frame.save(c, S0, 0x30);
            frame.save_ra(c, 0x10);
            c.g[V1][0] = sx(r32(c, S1).wrapping_add(0x70));
            Addr(r32(c, S1)).sw(c, 0);
            c.g[A1][0] = c.g[V0][0];
            Addr(r32(c, S1).wrapping_add(4)).sw(c, 0);
            c.g[A0][0] = 3;
            Addr(r32(c, S1).wrapping_add(8)).sw(c, 0);
            Addr(r32(c, S1).wrapping_add(0xc)).sw(c, 0);
            Addr(r32(c, S1).wrapping_add(0x10)).sw(c, 0);
            Addr(r32(c, S1).wrapping_add(0x14)).sw(c, 0);
            Addr(r32(c, S1).wrapping_add(0x18)).sw(c, 0);
            Addr(r32(c, S1).wrapping_add(0x44)).sw(c, 0);
            Addr(r32(c, S1).wrapping_add(0x48)).sw(c, c.g[V0][0]);
            Addr(r32(c, S1).wrapping_add(0x4c)).sw(c, c.g[V0][0]);
            Addr(r32(c, S1).wrapping_add(0x54)).sw(c, 0);
            Addr(r32(c, S1).wrapping_add(0x58)).sw(c, 0);
            Addr(r32(c, S1).wrapping_add(0x5c)).sw(c, 0);
            Addr(r32(c, S1).wrapping_add(0x60)).sw(c, 0);
            Addr(r32(c, S1).wrapping_add(0x64)).sw(c, 0);
            Addr(r32(c, S1).wrapping_add(0x6c)).sw(c, 0);
            if lobby_ctor_loop1(c)? {
                return Ok(());
            }
            lobby_ctor_after_loop1(c, frame)
        }),
        _ => c.jump_out(c.pc, 0x0025_94d0, 0x0025_9628),
    }
}

// ---------------------------------------------------------------------------
// LobbyMan_dtor (0x259628), resume heads 0x259688 / 0x259758 / 0x259838.

/// One backward BXString-dtor loop call step shared by the three dtor
/// loops is not shared: loops 1/2 keep s0 on exit, loop 3 decrements it.
/// Each loop below is therefore written out.

/// Dtor loop 1 (head 0x259688): destroy strings at s0 = self+0x2d0 down to
/// s2 = self+0x2c4. On normal exit charges 1 and sets a0 = s3+0x10.
fn lobby_dtor_loop1(c: &mut Cpu) -> Result<bool, crate::call::Unwound> {
    loop {
        c.cycles += 5;
        c.g[A1][0] = 0;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, callee::BXSTRING_DTOR, 0x0025_9694)?;
        if c.g[S2][0] != c.g[S0][0] {
            c.g[S0][0] = sx(r32(c, S0).wrapping_sub(4));
            if back_edge_unwinds(c, 0x0025_9688) {
                return Ok(true);
            }
            continue;
        }
        c.cycles += 1;
        c.g[A0][0] = sx(r32(c, S3).wrapping_add(0x10));
        return Ok(false);
    }
}

/// Dtor loop 2 (head 0x259758): destroy strings at s0 = self+0x280 down to
/// s2 = self+0x274. The caller charges 1 and sets a0 = s4+0x10 on exit.
fn lobby_dtor_loop2(c: &mut Cpu) -> Result<bool, crate::call::Unwound> {
    loop {
        c.cycles += 5;
        c.g[A1][0] = 0;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, callee::BXSTRING_DTOR, 0x0025_9764)?;
        if c.g[S2][0] != c.g[S0][0] {
            c.g[S0][0] = sx(r32(c, S0).wrapping_sub(4));
            if back_edge_unwinds(c, 0x0025_9758) {
                return Ok(true);
            }
            continue;
        }
        return Ok(false);
    }
}

/// Dtor loop 3 (head 0x259838): destroy strings at s0 = self+0x7c down to
/// s3 = self+0x70; s0 is decremented after every call, including the last.
fn lobby_dtor_loop3(c: &mut Cpu) -> Result<bool, crate::call::Unwound> {
    loop {
        c.cycles += 5;
        c.g[A1][0] = 0;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, callee::BXSTRING_DTOR, 0x0025_9844)?;
        let more = c.g[S3][0] != c.g[S0][0];
        c.g[S0][0] = sx(r32(c, S0).wrapping_sub(4));
        if more {
            if back_edge_unwinds(c, 0x0025_9838) {
                return Ok(true);
            }
            continue;
        }
        return Ok(false);
    }
}

/// Dtor final block (0x25987c): destroy the strings at sp+0x1c / sp+0x10,
/// delete self when the saved flags word (sp+0x18) is non-zero, restore all.
fn lobby_dtor_finish(c: &mut Cpu, frame: Frame) -> crate::call::Flow {
    c.cycles += 8;
    c.g[A1][0] = 2;
    call_ptr(c, callee::BXSTRING_DTOR, 0x0025_9858)?;
    c.g[A0][0] = Addr(frame.at(c, 0x10)).lw(c);
    c.g[A1][0] = 2;
    call_ptr(c, callee::BXSTRING_DTOR, 0x0025_9864)?;
    c.g[V0][0] = Addr(frame.at(c, 0x18)).lw(c);
    let delete_self = c.g[V0][0] != 0;
    frame.restore(c, S0, 0xc0);
    if delete_self {
        c.cycles += 3;
        c.g[A0][0] = c.g[S1][0];
        call_ptr(c, callee::OPERATOR_DELETE, 0x0025_9878)?;
        frame.restore(c, S0, 0xc0);
    }
    c.cycles += 11;
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
    Ok(())
}

/// Dtor middle block 2 (0x259850) through the finish: destroy the +0x260
/// sub-object strings, the +0xEC list and its head, the +0x80..+0x9C and
/// +0xE8 strings, then the +0x70 array (loop 3, head 0x259838).
fn lobby_dtor_mid2(c: &mut Cpu, frame: Frame) -> crate::call::Flow {
    c.cycles += 47;
    c.g[A1][0] = 2;
    call_ptr(c, callee::BXSTRING_DTOR, 0x0025_9778)?;
    c.g[A0][0] = sx(r32(c, S4).wrapping_add(0xc));
    c.g[A1][0] = 2;
    call_ptr(c, callee::BXSTRING_DTOR, 0x0025_9784)?;
    c.g[A0][0] = sx(r32(c, S4).wrapping_add(8));
    c.g[A1][0] = 2;
    call_ptr(c, callee::BXSTRING_DTOR, 0x0025_9790)?;
    c.g[A0][0] = sx(r32(c, S4).wrapping_add(4));
    c.g[A1][0] = 2;
    call_ptr(c, callee::BXSTRING_DTOR, 0x0025_979c)?;
    c.g[A0][0] = c.g[S4][0];
    c.g[A1][0] = 2;
    call_ptr(c, callee::BXSTRING_DTOR, 0x0025_97a8)?;
    c.g[A0][0] = c.g[S5][0];
    call_ptr(c, callee::LIST_DTOR_1DB718, 0x0025_97b0)?;
    c.g[A0][0] = Addr(r32(c, S5).wrapping_add(4)).lw(c);
    call_ptr(c, callee::OPERATOR_DELETE, 0x0025_97b8)?;
    c.g[A0][0] = Addr(frame.at(c, 4)).lw(c);
    c.g[A1][0] = 2;
    call_ptr(c, callee::BXSTRING_DTOR, 0x0025_97c4)?;
    c.g[A0][0] = c.g[S7][0];
    c.g[A1][0] = 2;
    call_ptr(c, callee::BXSTRING_DTOR, 0x0025_97d0)?;
    c.g[A0][0] = c.g[FP][0];
    c.g[A1][0] = 2;
    call_ptr(c, callee::BXSTRING_DTOR, 0x0025_97dc)?;
    c.g[A0][0] = Addr(frame.at(c, 0)).lw(c);
    c.g[A1][0] = 2;
    call_ptr(c, callee::BXSTRING_DTOR, 0x0025_97e8)?;
    c.g[A0][0] = Addr(frame.at(c, 0x20)).lw(c);
    c.g[A1][0] = 2;
    call_ptr(c, callee::BXSTRING_DTOR, 0x0025_97f4)?;
    c.g[A0][0] = Addr(frame.at(c, 0x14)).lw(c);
    c.g[A1][0] = 2;
    call_ptr(c, callee::BXSTRING_DTOR, 0x0025_9800)?;
    c.g[A0][0] = Addr(frame.at(c, 0xc)).lw(c);
    c.g[A1][0] = 2;
    call_ptr(c, callee::BXSTRING_DTOR, 0x0025_980c)?;
    c.g[A0][0] = Addr(frame.at(c, 8)).lw(c);
    c.g[A1][0] = 2;
    call_ptr(c, callee::BXSTRING_DTOR, 0x0025_9818)?;
    c.g[A0][0] = c.g[S6][0];
    c.g[A1][0] = 2;
    call_ptr(c, callee::BXSTRING_DTOR, 0x0025_9824)?;
    let array_is_null = c.g[S3][0] == 0;
    c.g[S0][0] = c.g[S6][0];
    if array_is_null {
        c.cycles += 1;
        c.g[A0][0] = Addr(frame.at(c, 0x1c)).lw(c);
        return lobby_dtor_finish(c, frame);
    }
    c.cycles += 2;
    let at_base = c.g[S3][0] == c.g[S0][0];
    c.g[A0][0] = Addr(frame.at(c, 0x1c)).lw(c);
    if at_base {
        return lobby_dtor_finish(c, frame);
    }
    c.cycles += 1;
    c.g[S0][0] = sx(r32(c, S0).wrapping_sub(4));
    if lobby_dtor_loop3(c)? {
        return Ok(());
    }
    c.cycles += 1;
    c.g[A0][0] = Addr(frame.at(c, 0x1c)).lw(c);
    lobby_dtor_finish(c, frame)
}

/// Dtor middle block 1 (0x259770) through the rest: destroy the +0x2B0
/// sub-object strings, save the +0x48..+0x94 string addresses on the stack,
/// destroy the +0x260 array (loop 2, head 0x259758), then block 2.
fn lobby_dtor_mid1(c: &mut Cpu, frame: Frame) -> crate::call::Flow {
    c.cycles += 40;
    c.g[A1][0] = 2;
    call_ptr(c, callee::BXSTRING_DTOR, 0x0025_96a8)?;
    c.g[S4][0] = c.g[S4][0] & 1;
    Addr(frame.at(c, 0x18)).sw(c, c.g[S4][0]);
    c.g[A0][0] = sx(r32(c, S3).wrapping_add(0xc));
    c.g[A1][0] = 2;
    c.g[S4][0] = sx(r32(c, S1).wrapping_add(0x260));
    call_ptr(c, callee::BXSTRING_DTOR, 0x0025_96c0)?;
    c.g[S5][0] = sx(r32(c, S1).wrapping_add(0xec));
    c.g[A0][0] = sx(r32(c, S3).wrapping_add(8));
    c.g[A1][0] = 2;
    call_ptr(c, callee::BXSTRING_DTOR, 0x0025_96d0)?;
    c.g[S7][0] = sx(r32(c, S1).wrapping_add(0x9c));
    c.g[A0][0] = sx(r32(c, S3).wrapping_add(4));
    c.g[A1][0] = 2;
    call_ptr(c, callee::BXSTRING_DTOR, 0x0025_96e0)?;
    c.g[FP][0] = sx(r32(c, S1).wrapping_add(0x98));
    c.g[V0][0] = sx(r32(c, S1).wrapping_add(0xe8));
    c.g[A0][0] = c.g[S3][0];
    Addr(frame.at(c, 4)).sw(c, c.g[V0][0]);
    c.g[A1][0] = 2;
    call_ptr(c, callee::BXSTRING_DTOR, 0x0025_96f8)?;
    c.g[S6][0] = sx(r32(c, S1).wrapping_add(0x80));
    c.g[V1][0] = sx(r32(c, S1).wrapping_add(0x88));
    c.g[V0][0] = sx(r32(c, S1).wrapping_add(0x274));
    Addr(frame.at(c, 0xc)).sw(c, c.g[V1][0]);
    c.g[S3][0] = sx(r32(c, S1).wrapping_add(0x70));
    c.g[V1][0] = sx(r32(c, S1).wrapping_add(0x94));
    Addr(frame.at(c, 0)).sw(c, c.g[V1][0]);
    c.g[V1][0] = sx(r32(c, S1).wrapping_add(0x90));
    Addr(frame.at(c, 0x20)).sw(c, c.g[V1][0]);
    c.g[V1][0] = sx(r32(c, S1).wrapping_add(0x8c));
    Addr(frame.at(c, 0x14)).sw(c, c.g[V1][0]);
    c.g[V1][0] = sx(r32(c, S1).wrapping_add(0x84));
    Addr(frame.at(c, 8)).sw(c, c.g[V1][0]);
    c.g[V1][0] = sx(r32(c, S1).wrapping_add(0x4c));
    Addr(frame.at(c, 0x1c)).sw(c, c.g[V1][0]);
    c.g[V1][0] = sx(r32(c, S1).wrapping_add(0x48));
    let array_is_null = c.g[V0][0] == 0;
    Addr(frame.at(c, 0x10)).sw(c, c.g[V1][0]);
    if array_is_null {
        c.cycles += 1;
        c.g[A0][0] = sx(r32(c, S4).wrapping_add(0x10));
        return lobby_dtor_mid2(c, frame);
    }
    c.cycles += 3;
    c.g[S0][0] = sx(r32(c, S1).wrapping_add(0x284));
    let at_end = c.g[V0][0] == c.g[S0][0];
    c.g[A0][0] = sx(r32(c, S4).wrapping_add(0x10));
    if at_end {
        return lobby_dtor_mid2(c, frame);
    }
    c.cycles += 3;
    c.g[S2][0] = c.g[V0][0];
    c.g[S0][0] = sx(r32(c, S0).wrapping_sub(4));
    if lobby_dtor_loop2(c)? {
        return Ok(());
    }
    c.cycles += 1;
    c.g[A0][0] = sx(r32(c, S4).wrapping_add(0x10));
    lobby_dtor_mid2(c, frame)
}

/// Tag field names (`D_004A2Fxx`) read by `LobbyMan_OnGameStart`.
pub(crate) const TAG_NAME: u32 = 0x004a_2f58;
pub(crate) const TAG_B1: u32 = 0x004a_2f60;
pub(crate) const TAG_B2: u32 = 0x004a_2f68;
pub(crate) const TAG_B3: u32 = 0x004a_2f70;
pub(crate) const TAG_B4: u32 = 0x004a_2f78;
pub(crate) const TAG_TIME: u32 = 0x004a_2f80;
pub(crate) const TAG_EV: u32 = 0x004a_2f88;
pub(crate) const TAG_KIND: u32 = 0x004a_2f90;
pub(crate) const TAG_ADDR1: u32 = 0x004a_2f98;
pub(crate) const TAG_ADDR2: u32 = 0x004a_2fa0;
pub(crate) const TAG_SCORE: u32 = 0x004a_2fa8;
/// Default/empty string (`D_004A2F40`) passed to every GetString4.
pub(crate) const TAG_DEFAULT: u32 = 0x004a_2f40;

/// `void LobbyMan_OnGameStart(self, msg)` (0x25a1d0): read the game-start
/// tag record at msg+0xC (name and four option strings into stack buffers,
/// start time to self+0xA0, event/kind numbers, peer addresses resolved to
/// self+0xA8/+0xAC), build the strings at self+0x88..+0x9C, and when the
/// lobby is in state 0x18/0x14 with a non-empty event string, configure the
/// race interface (game mode/event from kind and event, AI count when the
/// 0x534B34 flag is set). Finally post a kind-0xEF event to self+0xF0.
/// Other states just set state 0x19 and notify the game setup.
pub fn lobby_man_on_game_start(c: &mut Cpu) {
    guest(c, |c| {
        let frame = Frame::open(c, 0x6d0);
        let mut early_exit = false;
        'game_start: {
            'read_tags: {
                c.cycles += 156;
                frame.save(c, S0, 0x6c0);
                frame.save(c, S1, 0x6b0);
                frame.save(c, S2, 0x6a0);
                c.g[S2][0] = c.g[A1][0];
                frame.save(c, S3, 0x690);
                frame.save(c, S4, 0x680);
                frame.save(c, S5, 0x670);
                frame.save(c, S6, 0x660);
                frame.save(c, S7, 0x650);
                c.g[S7][0] = c.g[A0][0];
                frame.save(c, FP, 0x640);
                frame.save_ra(c, 0x630);
                // name -> sp+0
                c.g[A0][0] = Addr(r32(c, S2).wrapping_add(0xc)).lw(c);
                c.g[A1][0] = sx(TAG_NAME);
                call_ptr(c, callee::TAG_FIELD_FIND, 0x0025_a214)?;
                c.g[S1][0] = sx(TAG_DEFAULT);
                c.g[A1][0] = c.g[SP][0];
                c.g[A2][0] = 0x100;
                c.g[A3][0] = c.g[S1][0];
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, callee::TAG_FIELD_GET_STRING4, 0x0025_a230)?;
                // b1 -> sp+0x100 (also saved at sp+0x620)
                c.g[A0][0] = Addr(r32(c, S2).wrapping_add(0xc)).lw(c);
                c.g[A1][0] = sx(TAG_B1);
                call_ptr(c, callee::TAG_FIELD_FIND, 0x0025_a240)?;
                c.g[V1][0] = sx(frame.at(c, 0x100));
                c.g[A2][0] = 0x100;
                c.g[A1][0] = c.g[V1][0];
                Addr(frame.at(c, 0x620)).sw(c, c.g[V1][0]);
                c.g[A3][0] = c.g[S1][0];
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, callee::TAG_FIELD_GET_STRING4, 0x0025_a25c)?;
                // b2 -> s3 = sp+0x200
                c.g[A0][0] = Addr(r32(c, S2).wrapping_add(0xc)).lw(c);
                c.g[A1][0] = sx(TAG_B2);
                call_ptr(c, callee::TAG_FIELD_FIND, 0x0025_a26c)?;
                c.g[S3][0] = sx(frame.at(c, 0x200));
                c.g[A2][0] = 0x100;
                c.g[A3][0] = c.g[S1][0];
                c.g[A1][0] = c.g[S3][0];
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, callee::TAG_FIELD_GET_STRING4, 0x0025_a284)?;
                // b3 -> s4 = sp+0x300
                c.g[A0][0] = Addr(r32(c, S2).wrapping_add(0xc)).lw(c);
                c.g[A1][0] = sx(TAG_B3);
                call_ptr(c, callee::TAG_FIELD_FIND, 0x0025_a294)?;
                c.g[S4][0] = sx(frame.at(c, 0x300));
                c.g[A2][0] = 0x100;
                c.g[A3][0] = c.g[S1][0];
                c.g[A1][0] = c.g[S4][0];
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, callee::TAG_FIELD_GET_STRING4, 0x0025_a2ac)?;
                // b4 -> s5 = sp+0x400
                c.g[A0][0] = Addr(r32(c, S2).wrapping_add(0xc)).lw(c);
                c.g[A1][0] = sx(TAG_B4);
                call_ptr(c, callee::TAG_FIELD_FIND, 0x0025_a2bc)?;
                c.g[S5][0] = sx(frame.at(c, 0x400));
                c.g[A2][0] = 0x100;
                c.g[A3][0] = c.g[S1][0];
                c.g[A1][0] = c.g[S5][0];
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, callee::TAG_FIELD_GET_STRING4, 0x0025_a2d4)?;
                // start time -> self+0xA0
                c.g[A0][0] = Addr(r32(c, S2).wrapping_add(0xc)).lw(c);
                c.g[A1][0] = sx(TAG_TIME);
                call_ptr(c, callee::TAG_FIELD_FIND, 0x0025_a2e4)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0;
                call_ptr(c, callee::TAG_FIELD_GET_TIME, 0x0025_a2f0)?;
                Addr(r32(c, S7).wrapping_add(0xa0)).sw(c, c.g[V0][0]);
                // event number -> s6
                c.g[S0][0] = sx(TAG_EV);
                c.g[A0][0] = Addr(r32(c, S2).wrapping_add(0xc)).lw(c);
                c.g[A1][0] = c.g[S0][0];
                call_ptr(c, callee::TAG_FIELD_FIND, 0x0025_a308)?;
                c.g[A1][0] = 0xffff_ffff_ffff_ffff;
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, callee::TAG_FIELD_GET_NUMBER, 0x0025_a314)?;
                // kind -> fp (s6 = event first)
                c.g[A0][0] = Addr(r32(c, S2).wrapping_add(0xc)).lw(c);
                c.g[A1][0] = sx(TAG_KIND);
                c.g[S6][0] = c.g[V0][0];
                call_ptr(c, callee::TAG_FIELD_FIND, 0x0025_a328)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0xffff_ffff_ffff_ffff;
                call_ptr(c, callee::TAG_FIELD_GET_NUMBER, 0x0025_a334)?;
                // event string b5 -> s0 = sp+0x500
                c.g[A0][0] = Addr(r32(c, S2).wrapping_add(0xc)).lw(c);
                c.g[A1][0] = c.g[S0][0];
                c.g[FP][0] = c.g[V0][0];
                call_ptr(c, callee::TAG_FIELD_FIND, 0x0025_a344)?;
                c.g[S0][0] = sx(frame.at(c, 0x500));
                c.g[A3][0] = c.g[S1][0];
                c.g[A2][0] = 0x100;
                c.g[A1][0] = c.g[S0][0];
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, callee::TAG_FIELD_GET_STRING4, 0x0025_a35c)?;
                // peer addr 1 -> self+0xA8 (via b2)
                c.g[A0][0] = Addr(r32(c, S2).wrapping_add(0xc)).lw(c);
                c.g[A1][0] = sx(TAG_ADDR1);
                call_ptr(c, callee::TAG_FIELD_FIND, 0x0025_a36c)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0;
                call_ptr(c, callee::TAG_FIELD_GET_ADDR, 0x0025_a378)?;
                c.g[A2][0] = c.g[V0][0];
                c.g[A1][0] = c.g[S3][0];
                c.g[A0][0] = c.g[S7][0];
                callee::LOOKUP_PEER_ADDR.call(c, 0x0025_a388)?;
                Addr(r32(c, S7).wrapping_add(0xa8)).sw(c, c.g[V0][0]);
                // peer addr 2 -> self+0xAC (via b3)
                c.g[A1][0] = sx(TAG_ADDR2);
                c.g[A0][0] = Addr(r32(c, S2).wrapping_add(0xc)).lw(c);
                call_ptr(c, callee::TAG_FIELD_FIND, 0x0025_a39c)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0;
                call_ptr(c, callee::TAG_FIELD_GET_ADDR, 0x0025_a3a8)?;
                c.g[A2][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S7][0];
                c.g[A1][0] = c.g[S4][0];
                callee::LOOKUP_PEER_ADDR.call(c, 0x0025_a3b8)?;
                c.g[A1][0] = c.g[SP][0];
                c.g[A0][0] = sx(r32(c, S7).wrapping_add(0x88));
                Addr(r32(c, S7).wrapping_add(0xac)).sw(c, c.g[V0][0]);
                call_ptr(c, callee::BXSTRING_CTOR4, 0x0025_a3c8)?;
                c.g[A0][0] = sx(r32(c, S7).wrapping_add(0x90));
                c.g[A1][0] = c.g[S3][0];
                call_ptr(c, callee::BXSTRING_CTOR4, 0x0025_a3d4)?;
                c.g[A0][0] = sx(r32(c, S7).wrapping_add(0x94));
                c.g[A1][0] = c.g[S4][0];
                call_ptr(c, callee::BXSTRING_CTOR4, 0x0025_a3e0)?;
                c.g[A1][0] = c.g[S5][0];
                c.g[A0][0] = sx(r32(c, S7).wrapping_add(0x9c));
                call_ptr(c, callee::BXSTRING_CTOR4, 0x0025_a3ec)?;
                // same = (b1 == b2); self+0x98 gets b3 if same else b2
                c.g[A0][0] = Addr(frame.at(c, 0x620)).lw(c);
                c.g[A1][0] = c.g[S3][0];
                call_ptr(c, callee::STR_CMP_4165A8, 0x0025_a3f8)?;
                c.g[V0][0] = (c.g[V0][0] < 1u64) as u64;
                c.g[A1][0] = c.g[S4][0];
                if c.g[V0][0] == 0 {
                    c.g[A1][0] = c.g[S3][0];
                }
                c.g[A0][0] = sx(r32(c, S7).wrapping_add(0x98));
                Addr(r32(c, S7).wrapping_add(0xb4)).sw(c, c.g[V0][0]);
                call_ptr(c, callee::BXSTRING_CTOR4, 0x0025_a410)?;
                // score -> self+0xB0
                c.g[A0][0] = Addr(r32(c, S2).wrapping_add(0xc)).lw(c);
                c.g[A1][0] = sx(TAG_SCORE);
                call_ptr(c, callee::TAG_FIELD_FIND, 0x0025_a420)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0;
                call_ptr(c, callee::TAG_FIELD_GET_NUMBER, 0x0025_a42c)?;
                Addr(r32(c, S7).wrapping_add(0xb0)).sw(c, c.g[V0][0]);
                c.g[V1][0] = Addr(r32(c, S7)).lw(c);
                let state_is_18 = c.g[V1][0] == 0x18;
                c.g[A0][0] = c.g[S0][0];
                if state_is_18 {
                    break 'read_tags;
                }
                c.cycles += 3;
                let state_is_14 = c.g[V1][0] == 0x14;
                c.g[V0][0] = 0x19;
                if state_is_14 {
                    break 'read_tags;
                }
                c.cycles += 7;
                Addr(r32(c, S7)).sw(c, c.g[V0][0]);
                callee::NET_GET_GAME_SETUP.call(c, 0x0025_a454)?;
                c.g[A0][0] = c.g[V0][0];
                c.g[A1][0] = 0;
                call_ptr(c, callee::FUNC_263BA8, 0x0025_a460)?;
                frame.restore(c, S0, 0x6c0);
                early_exit = true;
                break 'game_start;
            }
            // ---- state 0x18/0x14: strlen(b5), maybe configure the race ----
            'configure: {
                c.cycles += 5;
                c.g[V0][0] = 0x19;
                Addr(r32(c, S7)).sw(c, c.g[V0][0]);
                call_ptr(c, callee::STRLEN_416810, 0x0025_a474)?;
                if c.g[V0][0] == 0 {
                    break 'configure;
                }
                // Mode/event selection (generated blocks b4f0/b4bc/b4e8/
                // b508): on entry a1=mode=0, fp=kind, s6=event.
                c.cycles += 9;
                call_ptr(c, callee::BE_GET_BE, 0x0025_a484)?;
                c.g[A1][0] = 0;
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, callee::BE_GET_INTERFACE, 0x0025_a490)?;
                c.g[S0][0] = c.g[V0][0];
                c.g[V0][0] = 1;
                let kind_is_1 = c.g[FP][0] == c.g[V0][0];
                c.g[A1][0] = 0;
                if kind_is_1 {
                    // b4e8 with v0=1: (1<3)!=0, so mode 5, kind 4.
                    c.cycles += 3;
                    c.g[V0][0] = (c.g[V0][0] < 3u64) as u64;
                    c.g[A1][0] = 5;
                    c.cycles += 2;
                    c.g[FP][0] = 4;
                } else if c.g[FP][0] == 0 {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[FP][0] as i64) < 2) as u64;
                    c.g[V0][0] = 2;
                    c.cycles += 2;
                    c.g[V0][0] = sx(r32(c, S6).wrapping_sub(0xe));
                    c.cycles += 3;
                    c.g[V0][0] = (c.g[V0][0] < 3u64) as u64;
                    c.g[A1][0] = 5;
                    c.cycles += 2;
                    if c.g[V0][0] == 0 {
                        c.g[A1][0] = 0;
                    } else {
                        c.g[FP][0] = 4;
                    }
                } else if (c.g[FP][0] as i64) >= 2 {
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[FP][0] as i64) < 2) as u64;
                    c.g[V0][0] = 2;
                    c.cycles += 2;
                    let kind_is_2 = c.g[FP][0] == c.g[V0][0];
                    c.g[V0][0] = 3;
                    if kind_is_2 {
                        // b508: event in [14,16] -> mode 6, kind 5.
                        c.cycles += 4;
                        c.g[V0][0] = sx(r32(c, S6).wrapping_sub(0xe));
                        c.g[V0][0] = (c.g[V0][0] < 3u64) as u64;
                        c.g[A1][0] = 6;
                        c.cycles += 2;
                        if c.g[V0][0] == 0 {
                            c.g[A1][0] = 1;
                        } else {
                            c.g[FP][0] = 5;
                        }
                    } else if c.g[FP][0] == c.g[V0][0] {
                        c.cycles += 2;
                        c.g[A1][0] = 2;
                    } else {
                        c.cycles += 2;
                        c.cycles += 2;
                    }
                } else {
                    // Negative kind: mode 0, kind unchanged.
                    c.cycles += 3;
                    c.g[V0][0] = ((c.g[FP][0] as i64) < 2) as u64;
                    c.g[V0][0] = 2;
                    c.cycles += 2;
                    c.g[V0][0] = sx(r32(c, S6).wrapping_sub(0xe));
                    c.cycles += 2;
                }
                c.cycles += 17;
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, callee::BE_SET_GAME_MODE, 0x0025_a51c)?;
                c.g[A1][0] = c.g[FP][0];
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, callee::BE_SET_GAME_EVENT, 0x0025_a528)?;
                c.g[V1][0] = 0x0053_0000;
                c.g[A2][0] = Addr(r32(c, GP).wrapping_sub(0x848)).lw(c);
                Addr(0x0053_5c08).sw(c, c.g[S6][0]);
                c.g[V0][0] = 1;
                Addr(r32(c, GP).wrapping_sub(0x1f38)).sw(c, c.g[V0][0]);
                c.g[A1][0] = 7;
                c.g[A0][0] = Addr(r32(c, A2).wrapping_add(0x78)).lw(c);
                call_ptr(c, callee::BE_GET_INTERFACE, 0x0025_a548)?;
                c.g[V1][0] = 0x0053_0000;
                c.g[V0][0] = Addr(0x0053_4b34).lw(c);
                let no_ai_block = c.g[V0][0] == 0;
                c.g[V1][0] = 0x0053_0000;
                if no_ai_block {
                    break 'configure;
                }
                c.cycles += 12;
                c.g[A0][0] = c.g[S0][0];
                c.g[V1][0] = sx(0x0053_08b8);
                c.g[A1][0] = 0;
                c.g[V0][0] = Addr(0x0053_08b8 + 0x18).lw(c);
                c.g[V0][0] = c.g[V0][0] | 0x1c0;
                Addr(0x0053_08b8 + 0x18).sw(c, c.g[V0][0]);
                call_ptr(c, callee::BE_SET_NUMBER_AI, 0x0025_a574)?;
                c.g[V1][0] = Addr(r32(c, S0).wrapping_add(0xc)).lw(c);
                c.g[A0][0] = Addr(r32(c, V1).wrapping_add(8)).lh(c);
                c.g[V0][0] = Addr(r32(c, V1).wrapping_add(0xc)).lw(c);
                let target = c.g[V0][0] as u32;
                c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, A0)));
                call_ptr(c, target, 0x0025_a588)?;
                break 'configure;
            }
            // ---- post the kind-0xEF game-start event ----
            c.cycles += 29;
            c.g[S0][0] = 0xef;
            callee::NET_GET_GAME_SETUP.call(c, 0x0025_a590)?;
            c.g[A1][0] = Addr(r32(c, S7).wrapping_add(0xa4)).lw(c);
            c.g[A0][0] = c.g[V0][0];
            c.g[A2][0] = 0;
            callee::SETUP_NET_RACE.call(c, 0x0025_a5a0)?;
            c.g[A1][0] = sx(ALLOC_TAG);
            c.g[A0][0] = 8;
            c.g[A2][0] = 0x2000_0000;
            c.g[A3][0] = 0;
            call_ptr(c, callee::MEM_ALLOC_NAMED, 0x0025_a5b8)?;
            c.g[S1][0] = c.g[V0][0];
            c.g[V1][0] = Addr(r32(c, S7).wrapping_add(0xf0)).lw(c);
            Addr(r32(c, S1)).sw(c, c.g[S0][0]);
            c.g[V0][0] = sx(EVENT_VT);
            c.g[A1][0] = sx(ALLOC_TAG_NODE);
            Addr(r32(c, S1).wrapping_add(4)).sw(c, c.g[V0][0]);
            Addr(frame.at(c, 0x610)).sw(c, c.g[V1][0]);
            c.g[A0][0] = 0xc;
            c.g[A2][0] = 0x2000_0000;
            c.g[A3][0] = 0;
            call_ptr(c, callee::MEM_ALLOC_NAMED, 0x0025_a5ec)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[V0][0] = sx(r32(c, A1).wrapping_add(8));
            if c.g[V0][0] != 0 {
                Addr(r32(c, A1).wrapping_add(8)).sw(c, c.g[S1][0]);
            }
            c.cycles += 10;
            c.g[V0][0] = Addr(frame.at(c, 0x610)).lw(c);
            Addr(r32(c, A1)).sw(c, c.g[V0][0]);
            c.g[V1][0] = Addr(r32(c, V0).wrapping_add(4)).lw(c);
            Addr(r32(c, A1).wrapping_add(4)).sw(c, c.g[V1][0]);
            c.g[A0][0] = Addr(r32(c, V0).wrapping_add(4)).lw(c);
            Addr(r32(c, A0)).sw(c, c.g[A1][0]);
            c.g[V0][0] = Addr(frame.at(c, 0x610)).lw(c);
            Addr(r32(c, V0).wrapping_add(4)).sw(c, c.g[A1][0]);
            Addr(frame.at(c, 0x600)).sw(c, c.g[A1][0]);
            frame.restore(c, S0, 0x6c0);
        }
        let _ = early_exit;
        c.cycles += 11;
        frame.restore(c, S1, 0x6b0);
        frame.restore(c, S2, 0x6a0);
        frame.restore(c, S3, 0x690);
        frame.restore(c, S4, 0x680);
        frame.restore(c, S5, 0x670);
        frame.restore(c, S6, 0x660);
        frame.restore(c, S7, 0x650);
        frame.restore(c, FP, 0x640);
        frame.restore_ra(c, 0x630);
        frame.close(c);
        Ok(())
    })
}

/// `void LobbyMan_dtor(self, flags)` (0x259628): disconnect, destroy the
/// string arrays at +0x2B0/+0x260/+0x70 and every embedded string, destroy
/// the +0xEC list, and delete self when `flags & 1`.
pub fn lobby_man_dtor(c: &mut Cpu) {
    match c.pc {
        0x0025_9688 => guest(c, |c| {
            let frame = Frame { size: 0xd0 };
            if lobby_dtor_loop1(c)? {
                return Ok(());
            }
            lobby_dtor_mid1(c, frame)
        }),
        0x0025_9758 => guest(c, |c| {
            let frame = Frame { size: 0xd0 };
            if lobby_dtor_loop2(c)? {
                return Ok(());
            }
            c.cycles += 1;
            c.g[A0][0] = sx(r32(c, S4).wrapping_add(0x10));
            lobby_dtor_mid2(c, frame)
        }),
        0x0025_9838 => guest(c, |c| {
            let frame = Frame { size: 0xd0 };
            if lobby_dtor_loop3(c)? {
                return Ok(());
            }
            c.cycles += 1;
            c.g[A0][0] = Addr(frame.at(c, 0x1c)).lw(c);
            lobby_dtor_finish(c, frame)
        }),
        0x0025_9628 => guest(c, |c| {
            let frame = Frame::open(c, 0xd0);
            c.cycles += 18;
            frame.save(c, S1, 0xb0);
            c.g[S1][0] = c.g[A0][0];
            frame.save(c, S4, 0x80);
            frame.save(c, S3, 0x90);
            c.g[S4][0] = c.g[A1][0];
            frame.save(c, S0, 0xc0);
            frame.save(c, S2, 0xa0);
            frame.save(c, S5, 0x70);
            frame.save(c, S6, 0x60);
            frame.save(c, S7, 0x50);
            frame.save(c, FP, 0x40);
            frame.save_ra(c, 0x30);
            c.g[S3][0] = sx(r32(c, S1).wrapping_add(0x2b0));
            callee::LOBBY_DISCONNECT.call(c, 0x0025_9664)?;
            c.g[V0][0] = sx(r32(c, S1).wrapping_add(0x2c4));
            let array_is_null = c.g[V0][0] == 0;
            c.g[A0][0] = sx(r32(c, S3).wrapping_add(0x10));
            if array_is_null {
                return lobby_dtor_mid1(c, frame);
            }
            c.cycles += 3;
            c.g[S0][0] = sx(r32(c, S1).wrapping_add(0x2d4));
            let at_end = c.g[V0][0] == c.g[S0][0];
            if at_end {
                return lobby_dtor_mid1(c, frame);
            }
            c.cycles += 3;
            c.g[S2][0] = c.g[V0][0];
            c.g[S0][0] = sx(r32(c, S0).wrapping_sub(4));
            if lobby_dtor_loop1(c)? {
                return Ok(());
            }
            lobby_dtor_mid1(c, frame)
        }),
        _ => c.jump_out(c.pc, 0x0025_9628, 0x0025_98a8),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::events::{check_events, driven, schedule};
    use crate::difftest::{check, scramble, Rng, DATA, GAME_GP};
    use crate::game::be::beintstat::tests::allocator;
    use crate::game::dirtysock::tags::tests::put_str;
    use std::sync::atomic::AtomicUsize;

    pub(crate) const SELF: u32 = DATA + 0x1000;
    pub(crate) const HEAD: u32 = DATA + 0x1800;
    pub(crate) const MSG: u32 = DATA + 0x2000;
    pub(crate) const RECORD: u32 = DATA + 0x3000;

    driven!(gen_lobby_man_ctor, ee::s06::f_002594d0__generated);
    driven!(hand_lobby_man_ctor, lobby_man_ctor);
    driven!(gen_lobby_man_dtor, ee::s06::f_00259628__generated);
    driven!(hand_lobby_man_dtor, lobby_man_dtor);
    /// Tag-parser tables (stand-ins copied from the tags unit's fixture):
    /// the class table at 0x4964c0 and the hex tables at 0x4961c0/0x4962c0.
    fn tables(c: &mut Cpu) {
        c.bus.write8(0x004a_4a70, b'\n');
        for i in 0..256u32 {
            let cls = if (b'a'..=b'z').contains(&(i as u8))
                || (b'A'..=b'Z').contains(&(i as u8))
                || (b'0'..=b'9').contains(&(i as u8))
                || i == b'_' as u32
                || i == b'-' as u32
                || i == b'.' as u32
            {
                2
            } else if i == b'=' as u32 || i == b':' as u32 {
                1
            } else {
                0
            };
            c.bus.write8(0x0049_64c0 + i, cls);
            let v = (i as u8 as char).to_digit(16).unwrap_or(0) as u8;
            c.bus.write8(0x0049_61c0 + i, v << 4);
            c.bus.write8(0x0049_62c0 + i, v);
            c.bus
                .write8(0x0049_5fc0 + i, b"0123456789abcdef"[(i >> 4) as usize]);
            c.bus
                .write8(0x0049_60c0 + i, b"0123456789abcdef"[(i & 15) as usize]);
        }
    }

    /// Common machine: game gp, BELibrary dummy at gp-0x1f04, world at
    /// gp-0x848 (interface at +0x78, setup base at +0x7C), lobby instance
    /// at gp-0xC8, self-linked list head, pool/DLHeap allocator.
    fn base(c: &mut Cpu, seed: u32, salt: u64, entry: u32) -> Rng {
        let mut r = Rng(salt ^ seed as u64);
        scramble(c, &mut r);
        c.g[GP][0] = GAME_GP as u64;
        c.pc = entry;
        c.bus.write32(GAME_GP - 0x1f04, DATA + 0x9000);
        c.bus.write32(GAME_GP - 0x848, DATA + 0x9800);
        c.bus.write32(DATA + 0x9800 + 0x78, DATA + 0xa000);
        c.bus.write32(DATA + 0x9800 + 0x7c, DATA + 0xd000);
        c.bus.write32(GAME_GP - 0xc8, SELF);
        c.bus.write32(SELF + 0xf0, HEAD);
        c.bus.write32(HEAD, HEAD);
        c.bus.write32(HEAD + 4, HEAD);
        allocator(c);
        c.g[GP][0] = GAME_GP as u64;
        c.pc = entry;
        tables(c);
        r
    }

    #[test]
    fn lobby_man_on_room_list_changed_matches() {
        check(
            ee::s06::f_002599b0__generated,
            lobby_man_on_room_list_changed,
            60,
            &|c, seed| {
                let _r = base(c, seed, 0x2599_b000, 0x0025_99b0);
                c.g[A0][0] = SELF as u64;
                if seed % 3 == 0 {
                    let other = DATA + 0x1c00;
                    c.bus.write32(HEAD + 4, other);
                    c.bus.write32(other, HEAD);
                    c.bus.write32(other + 4, HEAD);
                }
            },
        );
    }

    #[test]
    fn lobby_man_on_rank_list_changed_matches() {
        check(
            ee::s06::f_00259a60__generated,
            lobby_man_on_rank_list_changed,
            60,
            &|c, seed| {
                let _r = base(c, seed, 0x259a_6000, 0x0025_9a60);
                c.g[A0][0] = SELF as u64;
                if seed % 3 == 0 {
                    let other = DATA + 0x1c00;
                    c.bus.write32(HEAD + 4, other);
                    c.bus.write32(other, HEAD);
                    c.bus.write32(other + 4, HEAD);
                }
            },
        );
    }

    #[test]
    fn lobby_man_compare_rank_desc_matches() {
        check(
            ee::s06::f_00259400__generated,
            lobby_man_compare_rank_desc,
            60,
            &|c, seed| {
                let _r = base(c, seed, 0x2594_0000, 0x0025_9400);
                let a = DATA + 0x5000;
                let b = DATA + 0x5100;
                put_str(c, RANK_TAG_FMT, b"l1");
                put_str(
                    c,
                    a + 0x28,
                    match seed % 3 {
                        0 => b"$00000009",
                        1 => b"$00000002",
                        _ => b"$00000002",
                    },
                );
                put_str(
                    c,
                    b + 0x28,
                    match seed % 3 {
                        0 => b"$00000002",
                        1 => b"$00000009",
                        _ => b"$00000002",
                    },
                );
                c.g[A2][0] = a as u64;
                c.g[A3][0] = b as u64;
            },
        );
    }

    #[test]
    fn lobby_man_compare_rank_asc_matches() {
        check(
            ee::s06::f_00259468__generated,
            lobby_man_compare_rank_asc,
            60,
            &|c, seed| {
                let _r = base(c, seed, 0x2594_6800, 0x0025_9468);
                let a = DATA + 0x5000;
                let b = DATA + 0x5100;
                put_str(c, RANK_TAG_FMT, b"l1");
                put_str(
                    c,
                    a + 0x28,
                    match seed % 3 {
                        0 => b"$00000009",
                        1 => b"$00000002",
                        _ => b"$00000002",
                    },
                );
                put_str(
                    c,
                    b + 0x28,
                    match seed % 3 {
                        0 => b"$00000002",
                        1 => b"$00000009",
                        _ => b"$00000002",
                    },
                );
                c.g[A2][0] = a as u64;
                c.g[A3][0] = b as u64;
            },
        );
    }

    fn ctor_fixture(c: &mut Cpu, _seed: u32) {
        c.bus.write32(GAME_GP + 0xda0, DATA + 0x900);
        c.g[A0][0] = SELF as u64;
    }

    #[test]
    fn lobby_man_ctor_matches() {
        check(
            ee::s06::f_002594d0__generated,
            lobby_man_ctor,
            60,
            &|c, seed| {
                let _r = base(c, seed, 0x2594_d000, 0x0025_94d0);
                ctor_fixture(c, seed);
            },
        );
    }

    #[test]
    fn lobby_man_ctor_events() {
        check_events(gen_lobby_man_ctor, hand_lobby_man_ctor, 60, &|c, seed| {
            let mut r = base(c, seed, 0x2594_d100, 0x0025_94d0);
            ctor_fixture(c, seed);
            let a0 = c.g[A0][0];
            let mode = schedule(c, &mut r, seed, 0x0025_94d0);
            c.g[A0][0] = a0;
            c.g[GP][0] = GAME_GP as u64;
            mode
        });
    }

    fn dtor_fixture(c: &mut Cpu, seed: u32) {
        for i in 0..0xc0u32 {
            c.bus.write32(SELF + i * 4, 0);
        }
        c.bus.write32(SELF + 0xf0, HEAD);
        c.g[A0][0] = SELF as u64;
        c.g[A1][0] = match seed % 3 {
            0 => 0,
            1 => 1,
            _ => 3,
        } as u64;
    }

    #[test]
    fn lobby_man_dtor_matches() {
        check(
            ee::s06::f_00259628__generated,
            lobby_man_dtor,
            60,
            &|c, seed| {
                let _r = base(c, seed, 0x2596_2800, 0x0025_9628);
                dtor_fixture(c, seed);
            },
        );
    }

    #[test]
    fn lobby_man_dtor_events() {
        check_events(gen_lobby_man_dtor, hand_lobby_man_dtor, 60, &|c, seed| {
            let mut r = base(c, seed, 0x2596_2900, 0x0025_9628);
            dtor_fixture(c, seed);
            let a0 = c.g[A0][0];
            let a1 = c.g[A1][0];
            let mode = schedule(c, &mut r, seed, 0x0025_9628);
            c.g[A0][0] = a0;
            c.g[A1][0] = a1;
            c.g[GP][0] = GAME_GP as u64;
            mode
        });
    }

    // ---- gated BE fakes for OnGameStart (gate: CTRL magic) ----
    pub(crate) const CTRL: u32 = DATA + 0xf_f000;
    pub(crate) const MAGIC: u32 = 0x6f63_1008;
    pub(crate) const R_BE: u32 = DATA + 0xf_f010;
    pub(crate) const R_IFACE: u32 = DATA + 0xf_f014;

    macro_rules! gated {
        ($name:ident, $prev:ident, $cell:expr, $gen:path) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $name(c: &mut Cpu) {
                if c.bus.read32(CTRL) == MAGIC {
                    c.cycles += 3;
                    c.g[V0][0] = sx(c.bus.read32($cell));
                    c.pc = r32(c, RA);
                    return;
                }
                crate::difftest::fakes::fall_back(c, &$prev, $gen)
            }
        };
    }

    gated!(
        fake_get_be,
        PREV_GET_BE,
        R_BE,
        ee::s01::f_0014dc80__generated
    );
    gated!(
        fake_get_iface,
        PREV_GET_IFACE,
        R_IFACE,
        ee::s01::f_0014dd58__generated
    );
    gated!(
        fake_set_mode,
        PREV_SET_MODE,
        R_BE,
        ee::s01::f_00144df0__generated
    );
    gated!(
        fake_set_event,
        PREV_SET_EVENT,
        R_BE,
        ee::s01::f_001451e8__generated
    );
    gated!(
        fake_set_ai,
        PREV_SET_AI,
        R_BE,
        ee::s01::f_00144d98__generated
    );

    fn install_fakes() {
        use crate::difftest::fakes::hook;
        hook(
            std::ptr::addr_of_mut!(ee::s01::OVERRIDE_0014DC80),
            fake_get_be,
            &PREV_GET_BE,
        );
        hook(
            std::ptr::addr_of_mut!(ee::s01::OVERRIDE_0014DD58),
            fake_get_iface,
            &PREV_GET_IFACE,
        );
        hook(
            std::ptr::addr_of_mut!(ee::s01::OVERRIDE_00144DF0),
            fake_set_mode,
            &PREV_SET_MODE,
        );
        hook(
            std::ptr::addr_of_mut!(ee::s01::OVERRIDE_001451E8),
            fake_set_event,
            &PREV_SET_EVENT,
        );
        hook(
            std::ptr::addr_of_mut!(ee::s01::OVERRIDE_00144D98),
            fake_set_ai,
            &PREV_SET_AI,
        );
    }

    #[test]
    fn lobby_man_on_game_start_matches() {
        check(
            ee::s06::f_0025a1d0__generated,
            lobby_man_on_game_start,
            150,
            &|c, seed| {
                install_fakes();
                let _r = base(c, seed, 0x25a1_d000, 0x0025_a1d0);
                // Tag field names at their D_004A2Fxx addresses.
                put_str(c, TAG_NAME, b"nm");
                put_str(c, TAG_B1, b"b1");
                put_str(c, TAG_B2, b"b2");
                put_str(c, TAG_B3, b"b3");
                put_str(c, TAG_B4, b"b4");
                put_str(c, TAG_TIME, b"tm");
                put_str(c, TAG_EV, b"ev");
                put_str(c, TAG_KIND, b"kd");
                put_str(c, TAG_ADDR1, b"a1");
                put_str(c, TAG_ADDR2, b"a2");
                put_str(c, TAG_SCORE, b"sc");
                // kind by seed%5, event in-range by (seed/5)%2, state by
                // (seed/10)%3, AI flag by (seed/30)%2 (seed arithmetic only).
                let record: &[u8] = match (seed % 5, (seed / 5) % 2) {
                    (0, 0) => {
                        b"nm=hero\nb1=x\nb2=y\nb3=z\nb4=w\ntm=9\nev=15\nkd=0\na1=0\na2=0\nsc=7\n"
                    }
                    (0, _) => {
                        b"nm=hero\nb1=x\nb2=y\nb3=z\nb4=w\ntm=9\nev=3\nkd=0\na1=0\na2=0\nsc=7\n"
                    }
                    (1, 0) => {
                        b"nm=hero\nb1=x\nb2=y\nb3=z\nb4=w\ntm=9\nev=15\nkd=1\na1=0\na2=0\nsc=7\n"
                    }
                    (1, _) => {
                        b"nm=hero\nb1=x\nb2=y\nb3=z\nb4=w\ntm=9\nev=3\nkd=1\na1=0\na2=0\nsc=7\n"
                    }
                    (2, 0) => {
                        b"nm=hero\nb1=x\nb2=y\nb3=z\nb4=w\ntm=9\nev=15\nkd=2\na1=0\na2=0\nsc=7\n"
                    }
                    (2, _) => {
                        b"nm=hero\nb1=x\nb2=y\nb3=z\nb4=w\ntm=9\nev=3\nkd=2\na1=0\na2=0\nsc=7\n"
                    }
                    (3, 0) => {
                        b"nm=hero\nb1=x\nb2=y\nb3=z\nb4=w\ntm=9\nev=15\nkd=3\na1=0\na2=0\nsc=7\n"
                    }
                    (3, _) => {
                        b"nm=hero\nb1=x\nb2=y\nb3=z\nb4=w\ntm=9\nev=3\nkd=3\na1=0\na2=0\nsc=7\n"
                    }
                    (_, 0) => {
                        b"nm=hero\nb1=x\nb2=y\nb3=z\nb4=w\ntm=9\nev=15\nkd=7\na1=0\na2=0\nsc=7\n"
                    }
                    (_, _) => {
                        b"nm=hero\nb1=x\nb2=y\nb3=z\nb4=w\ntm=9\nev=3\nkd=7\na1=0\na2=0\nsc=7\n"
                    }
                };
                put_str(c, RECORD, record);
                for i in 0..0xc0u32 {
                    c.bus.write32(SELF + i * 4, 0);
                }
                c.bus.write32(
                    SELF,
                    match (seed / 10) % 3 {
                        0 => 0x18,
                        1 => 0x14,
                        _ => 9,
                    },
                );
                c.bus.write32(SELF + 0xf0, HEAD);
                c.bus.write32(SELF + 0xa4, 5);
                c.bus.write32(MSG + 0xc, RECORD);
                // Gated BE fakes: getBE -> DATA+0x9000, getInterface -> RI.
                let ri = DATA + 0xa800;
                let vt = DATA + 0xb000;
                c.bus.write32(ri + 0xc, vt);
                c.bus.write16(vt + 8, 0);
                c.bus.write32(vt + 0xc, 0x0014_06c0);
                c.bus.write32(CTRL, MAGIC);
                c.bus.write32(R_BE, DATA + 0x9000);
                c.bus.write32(R_IFACE, ri);
                c.bus.write32(0x0053_4b34, (seed / 30) % 2);
                c.g[A0][0] = SELF as u64;
                c.g[A1][0] = MSG as u64;
            },
        );
    }
}
