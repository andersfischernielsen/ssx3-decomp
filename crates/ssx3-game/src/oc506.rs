//! Staging module oc506 (wave wave228): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc506`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x001d_cb70, fe_module_get_rider_preview_2_thunk, "FEModule_GetRiderPreview_2_Thunk"),
    (0x001d_cb90, fe_module_get_rider_preview_thunk, "FEModule_GetRiderPreview_Thunk"),
    (0x001d_cbd8, felocale_dtor, "FELocale_dtor"),
    (0x001d_cc08, felocale_v02, "FELocale_v02"),
    (0x001d_cd00, debug_menu_v01, "DebugMenu_v01"),
    (0x001d_cd58, lobby_menu_message_page_dtor, "LobbyMenuMessagePage_dtor"),
    (0x001d_cda0, lobby_menu_destroy_login_page, "LobbyMenu_DestroyLoginPage"),
    (0x001d_ce20, lobby_menu_destroy_create_account_page, "LobbyMenu_DestroyCreateAccountPage"),
    (0x001d_cea0, lobby_menu_destroy_login_method_page, "LobbyMenu_DestroyLoginMethodPage"),
    (0x001d_cfc8, lobby_menu_destroy_delete_persona_page, "LobbyMenu_DestroyDeletePersonaPage"),
    (0x001d_d060, lobby_menu_destroy_create_persona_page, "LobbyMenu_DestroyCreatePersonaPage"),
    (0x001d_d0c8, lobby_menu_destroy_select_persona_page, "LobbyMenu_DestroySelectPersonaPage"),
    (0x001d_d258, lobby_menu_destroy_select_room_page, "LobbyMenu_DestroySelectRoomPage"),
    (0x001d_d2f0, lobby_menu_destroy_room_occupants_page, "LobbyMenu_DestroyRoomOccupantsPage"),
    (0x001d_d398, lobby_menu_destroy_chat_page, "LobbyMenu_DestroyChatPage"),
];

/// `sx((base as u32) + off)`: the 32-bit address arithmetic of the generated code.
fn menu_sx_add(base: u64, off: u32) -> u64 {
    sx((base as u32).wrapping_add(off))
}

/// `MenuSysItem_dtor(a0, 2)` (0x2ca280, through the function table), returning to `ret`.
fn menu_item_dtor(c: &mut Cpu, a0: u64, ret: u32) -> Flow {
    c.g[A0][0] = a0;
    c.g[A1][0] = 0x2;
    call_ptr(c, 0x002c_a280, ret)
}

/// `MenuSysPage_dtor(a0, a1)` (0x2caa80, through the function table), returning to `ret`.
fn menu_page_dtor(c: &mut Cpu, a0: u64, a1: u64, ret: u32) -> Flow {
    c.g[A0][0] = a0;
    c.g[A1][0] = a1;
    call_ptr(c, 0x002c_aa80, ret)
}

/// The `'l: loop` that destroys the 0x24-byte entries of a page's list: from
/// `S0` down to `S2`, each entry's destructor found through its vtable
/// (`+0x10` holds the vtable, `+0x8` a halfword offset for `a0`, `+0xc` the
/// function). `head` is the loop's first instruction (the event check's pc),
/// `ret` the return address of the destructor call.
fn menu_list_walk(c: &mut Cpu, head: u32, ret: u32) -> Flow {
    loop {
        c.cycles += 8;
        c.g[A1][0] = 0;
        c.g[V0][0] = Addr(c.g[S0][0] as u32).off(0x10).lw(c);
        c.g[A0][0] = Addr(c.g[V0][0] as u32).off(0x8).lh(c);
        c.g[V1][0] = Addr(c.g[V0][0] as u32).off(0xc).lw(c);
        let target = c.g[V1][0] as u32;
        c.g[RA][0] = sx(ret);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, target, ret)?;
        let again = c.g[S2][0] != c.g[S0][0];
        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x24));
        if !again {
            return Ok(());
        }
        if event_due(c) {
            deliver_event_here(c, head);
        }
    }
}

/// `FEModule_GetRiderPreview_2_Thunk` (0x1dcb70): `cFERiderManager_getHuman(this + 0xb0)`,
/// tail-returning its result.
pub fn fe_module_get_rider_preview_2_thunk(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = menu_sx_add(c.g[A0][0], 0xb0);
        call_ptr(c, 0x001a_0538, 0x001d_cb80)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `FEModule_GetRiderPreview_Thunk` (0x1dcb90): `cFERiderManager_getHuman1(this + 0xb0)`,
/// tail-returning its result.
pub fn fe_module_get_rider_preview_thunk(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = menu_sx_add(c.g[A0][0], 0xb0);
        call_ptr(c, 0x001a_0548, 0x001d_cba0)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cUILocaleInterface::~cUILocaleInterface(this, delete_flag)` (0x1dcbd8): stores
/// the locale vtable-ish word (0x46d848) at `+4`; with bit 0 of the flag set,
/// frees `this` through `operator delete`.
pub fn felocale_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 0x0047_0000;
        frame.save_ra(c, 0);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x27b8));
        c.g[A1][0] &= 0x1;
        let delete = c.g[A1][0] != 0;
        let this = c.g[A0][0] as u32;
        let value = c.g[V0][0];
        Addr(this).off(0x4).sw(c, value);
        if delete {
            c.cycles += 2;
            call_ptr(c, 0x0031_7e50, 0x001d_cbfc)?;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cUILocaleInterface::setDisplayMode(this, mode)` (0x1dcc08): stores the low byte
/// of `mode` at `this`, then calls `FELocale_SetFlag27(this, mode ^ 2 < 1)`.
pub fn felocale_v02(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = sx((c.g[A1][0] as u32) << 24);
        frame.save_ra(c, 0);
        c.g[V1][0] = c.g[A0][0];
        c.g[A1][0] = sx(((c.g[A1][0] as u32) as i32 >> 24) as u32);
        c.g[V0][0] = c.g[A1][0] ^ 0x2;
        let mode = c.g[A1][0];
        Addr(c.g[V1][0] as u32).sb(c, mode);
        c.g[A1][0] = (c.g[V0][0] < 0x1) as u64;
        call_ptr(c, 0x001e_1090, 0x001d_cc2c)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cDebugMenuScreen::~cDebugMenuScreen(this, delete_flag)` (0x1dcd00): frees the
/// background set at `this + 0x8c10`, stores the screen vtable word (0x46d948)
/// at `+0`, and with bit 0 of the flag set deletes `this`.
pub fn debug_menu_v01(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x20);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = 0x2;
        c.g[A0][0] = menu_sx_add(c.g[S1][0], 0x8c10);
        call_ptr(c, 0x0017_abe0, 0x001d_cd28)?;
        c.g[S0][0] &= 0x1;
        c.g[V0][0] = 0x0047_0000;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x26b8));
        let delete = c.g[S0][0] != 0;
        let this = c.g[S1][0] as u32;
        let value = c.g[V0][0];
        Addr(this).sw(c, value);
        if delete {
            c.cycles += 2;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, 0x0031_7e50, 0x001d_cd44)?;
        }
        c.cycles += 5;
        frame.restore(c, S1, 0x10);
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `LobbyMenuMessagePage::~LobbyMenuMessagePage(this, flags)` (0x1dcd58): destroys
/// the message page's item at `+0x130`, then the base page.
pub fn lobby_menu_message_page_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 17;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        let s0 = c.g[S0][0];
        let s1 = c.g[S1][0];
        menu_item_dtor(c, menu_sx_add(s0, 0x130), 0x001d_cd7c)?;
        menu_page_dtor(c, s0, s1, 0x001d_cd88)?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// Destroys the items at `base + off` (each with a destructor call returning to
/// `ret`), then the page at `base` with `a1`: the common tail of the
/// `LobbyMenu_Destroy*Page` functions.
fn lobby_menu_destroy_items(
    c: &mut Cpu,
    base: u64,
    items: &[(u32, u32)],
    page: (u64, u32),
) -> Flow {
    for &(off, ret) in items {
        menu_item_dtor(c, menu_sx_add(base, off), ret)?;
    }
    menu_page_dtor(c, base, page.0, page.1)
}

/// `LobbyMenu_DestroyLoginPage(this, a1)` (0x1dcda0): six list items, then the page.
pub fn lobby_menu_destroy_login_page(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 32;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        let s0 = c.g[S0][0];
        let s1 = c.g[S1][0];
        lobby_menu_destroy_items(
            c,
            s0,
            &[
                (0x1e4, 0x001d_cdc4),
                (0x1c0, 0x001d_cdd0),
                (0x194, 0x001d_cddc),
                (0x168, 0x001d_cde8),
                (0x150, 0x001d_cdf4),
                (0x134, 0x001d_ce00),
            ],
            (s1, 0x001d_ce0c),
        )?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `LobbyMenu_DestroyCreateAccountPage(this, a1)` (0x1dce20): six list items, then the page.
pub fn lobby_menu_destroy_create_account_page(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 32;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        let s0 = c.g[S0][0];
        let s1 = c.g[S1][0];
        lobby_menu_destroy_items(
            c,
            s0,
            &[
                (0x1e4, 0x001d_ce44),
                (0x1c0, 0x001d_ce50),
                (0x194, 0x001d_ce5c),
                (0x168, 0x001d_ce68),
                (0x150, 0x001d_ce74),
                (0x134, 0x001d_ce80),
            ],
            (s1, 0x001d_ce8c),
        )?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `LobbyMenu_DestroyLoginMethodPage(this, a1)` (0x1dcea0): the largest of the
/// page destructors, a straight run of item and page destructions.
pub fn lobby_menu_destroy_login_method_page(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 74;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S0, 0x40);
        frame.save(c, S3, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[S3][0] = c.g[A1][0];
        frame.save(c, S1, 0x30);
        frame.save(c, S2, 0x20);
        frame.save_ra(c, 0);
        let s0 = c.g[S0][0];
        menu_item_dtor(c, menu_sx_add(s0, 0x578), 0x001d_cecc)?;
        c.g[S1][0] = menu_sx_add(s0, 0x32c);
        menu_item_dtor(c, menu_sx_add(s0, 0x55c), 0x001d_cedc)?;
        c.g[S2][0] = menu_sx_add(s0, 0x130);
        for (off, ret) in [
            (0x544, 0x001d_ceec),
            (0x528, 0x001d_cef8),
            (0x510, 0x001d_cf04),
            (0x4ec, 0x001d_cf10),
            (0x4c0, 0x001d_cf1c),
            (0x494, 0x001d_cf28),
            (0x47c, 0x001d_cf34),
            (0x460, 0x001d_cf40),
        ] {
            menu_item_dtor(c, menu_sx_add(s0, off), ret)?;
        }
        let s1 = c.g[S1][0];
        menu_page_dtor(c, s1, 0x2, 0x001d_cf4c)?;
        for (off, ret) in [
            (0x314, 0x001d_cf58),
            (0x2f0, 0x001d_cf64),
            (0x2c4, 0x001d_cf70),
            (0x298, 0x001d_cf7c),
            (0x280, 0x001d_cf88),
            (0x264, 0x001d_cf94),
        ] {
            menu_item_dtor(c, menu_sx_add(s0, off), ret)?;
        }
        let s2 = c.g[S2][0];
        menu_page_dtor(c, s2, 0x2, 0x001d_cfa0)?;
        let s3 = c.g[S3][0];
        menu_page_dtor(c, s0, s3, 0x001d_cfac)?;
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `LobbyMenu_DestroyDeletePersonaPage(this, a1)` (0x1dcfc8): walks the entries
/// from `this + 0x1f8` down to `this + 0x168` (when they exist), then two items and the page.
pub fn lobby_menu_destroy_delete_persona_page(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S1, 0x30);
        frame.save(c, S3, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x40);
        c.g[S3][0] = c.g[A1][0];
        frame.save(c, S2, 0x20);
        let s1 = c.g[S1][0];
        let v0 = menu_sx_add(s1, 0x168);
        c.g[V0][0] = v0;
        frame.save_ra(c, 0);
        if v0 != 0 {
            c.cycles += 3;
            c.g[S0][0] = menu_sx_add(s1, 0x1f8);
            c.g[S2][0] = v0;
            if v0 != c.g[S0][0] {
                c.cycles += 1;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x24));
                menu_list_walk(c, 0x001d_d000, 0x001d_d018)?;
            }
        }
        c.cycles += 16;
        menu_item_dtor(c, menu_sx_add(s1, 0x150), 0x001d_d02c)?;
        menu_item_dtor(c, menu_sx_add(s1, 0x134), 0x001d_d038)?;
        let s3 = c.g[S3][0];
        menu_page_dtor(c, s1, s3, 0x001d_d044)?;
        frame.restore(c, S1, 0x30);
        frame.restore(c, S3, 0x10);
        frame.restore(c, S0, 0x40);
        frame.restore(c, S2, 0x20);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `LobbyMenu_DestroyCreatePersonaPage(this, a1)` (0x1dd060): four items, then the page.
pub fn lobby_menu_destroy_create_persona_page(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        frame.save(c, S1, 0x10);
        c.g[S0][0] = c.g[A0][0];
        c.g[S1][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        let s0 = c.g[S0][0];
        let s1 = c.g[S1][0];
        lobby_menu_destroy_items(
            c,
            s0,
            &[
                (0x194, 0x001d_d084),
                (0x168, 0x001d_d090),
                (0x150, 0x001d_d09c),
                (0x134, 0x001d_d0a8),
            ],
            (s1, 0x001d_d0b4),
        )?;
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `LobbyMenu_DestroySelectPersonaPage(this, a1)` (0x1dd0c8): the largest page
/// destructor. Three entry lists (`+0x518..0x5a8`, `+0x450..0x4e0`, and the
/// items from `+0x264` to `+0x5c4`), then the page with the stored `a1`.
pub fn lobby_menu_destroy_select_persona_page(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0xb0);
        frame.save(c, S1, 0x90);
        c.g[S1][0] = c.g[A0][0];
        let a1 = c.g[A1][0];
        let a1_slot = frame.at(c, 0);
        Addr(a1_slot).sw(c, a1);
        frame.save(c, S0, 0xa0);
        frame.save(c, S2, 0x80);
        frame.save(c, S3, 0x70);
        frame.save(c, S4, 0x60);
        frame.save(c, S5, 0x50);
        frame.save(c, S6, 0x40);
        frame.save(c, S7, 0x30);
        frame.save(c, FP, 0x20);
        frame.save_ra(c, 0x10);
        let s1 = c.g[S1][0];
        c.g[A1][0] = 0x2;
        c.g[S0][0] = menu_sx_add(s1, 0x5a8);
        menu_item_dtor(c, menu_sx_add(s1, 0x5c4), 0x001d_d10c)?;
        let s0 = c.g[S0][0];
        menu_item_dtor(c, s0, 0x001d_d118)?;

        // Block A: the list at +0x518 (its end is +0x5a8).
        let v0 = menu_sx_add(s1, 0x518);
        c.g[V0][0] = v0;
        c.g[A0][0] = menu_sx_add(s1, 0x500);
        if v0 != 0 {
            c.cycles += 2;
            let same = v0 == c.g[S0][0];
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x24));
            if !same {
                c.cycles += 1;
                c.g[S2][0] = v0;
                menu_list_walk(c, 0x001d_d130, 0x001d_d148)?;
                c.cycles += 1;
                c.g[A0][0] = menu_sx_add(s1, 0x500);
            }
        }

        // Block B: the list at +0x450 (its end is +0x4e0).
        c.cycles += 13;
        menu_item_dtor(c, menu_sx_add(s1, 0x500), 0x001d_d15c)?;
        c.g[S3][0] = menu_sx_add(s1, 0x2e8);
        menu_item_dtor(c, menu_sx_add(s1, 0x4e4), 0x001d_d16c)?;
        c.g[FP][0] = menu_sx_add(s1, 0x130);
        c.g[V0][0] = menu_sx_add(s1, 0x450);
        c.g[S4][0] = menu_sx_add(s1, 0x2c4);
        c.g[S5][0] = menu_sx_add(s1, 0x298);
        c.g[S6][0] = menu_sx_add(s1, 0x280);
        let empty = c.g[V0][0] == 0;
        c.g[S7][0] = menu_sx_add(s1, 0x264);
        if !empty {
            c.cycles += 3;
            c.g[S0][0] = menu_sx_add(s1, 0x4e0);
            let same = c.g[V0][0] == c.g[S0][0];
            c.g[S2][0] = c.g[V0][0];
            if !same {
                c.cycles += 1;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x24));
                menu_list_walk(c, 0x001d_d198, 0x001d_d1b0)?;
            }
        }

        // Block C: the rest, then the page with the stored a1.
        c.cycles += 39;
        let s3 = c.g[S3][0];
        menu_item_dtor(c, menu_sx_add(s3, 0x150), 0x001d_d1c4)?;
        menu_item_dtor(c, menu_sx_add(s3, 0x134), 0x001d_d1d0)?;
        menu_page_dtor(c, s3, 0x2, 0x001d_d1dc)?;
        let s4 = c.g[S4][0];
        menu_item_dtor(c, s4, 0x001d_d1e8)?;
        let s5 = c.g[S5][0];
        menu_item_dtor(c, s5, 0x001d_d1f4)?;
        let s6 = c.g[S6][0];
        menu_item_dtor(c, s6, 0x001d_d200)?;
        let s7 = c.g[S7][0];
        menu_item_dtor(c, s7, 0x001d_d20c)?;
        let fp = c.g[FP][0];
        menu_page_dtor(c, fp, 0x2, 0x001d_d218)?;
        let stored_a1 = Addr(frame.at(c, 0)).lw(c);
        menu_page_dtor(c, s1, stored_a1, 0x001d_d224)?;

        frame.restore(c, S0, 0xa0);
        frame.restore(c, S1, 0x90);
        frame.restore(c, S2, 0x80);
        frame.restore(c, S3, 0x70);
        frame.restore(c, S4, 0x60);
        frame.restore(c, S5, 0x50);
        frame.restore(c, S6, 0x40);
        frame.restore(c, S7, 0x30);
        frame.restore(c, FP, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `LobbyMenu_DestroySelectRoomPage(this, a1)` (0x1dd258): one list from
/// `+0x318` down to `+0x168` (when present), two items and the page.
pub fn lobby_menu_destroy_select_room_page(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S1, 0x30);
        frame.save(c, S3, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x40);
        c.g[S3][0] = c.g[A1][0];
        frame.save(c, S2, 0x20);
        let s1 = c.g[S1][0];
        let v0 = menu_sx_add(s1, 0x168);
        c.g[V0][0] = v0;
        frame.save_ra(c, 0);
        if v0 != 0 {
            c.cycles += 3;
            c.g[S0][0] = menu_sx_add(s1, 0x318);
            let same = v0 == c.g[S0][0];
            c.g[S2][0] = v0;
            if !same {
                c.cycles += 1;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x24));
                menu_list_walk(c, 0x001d_d290, 0x001d_d2a8)?;
            }
        }
        c.cycles += 16;
        menu_item_dtor(c, menu_sx_add(s1, 0x150), 0x001d_d2bc)?;
        menu_item_dtor(c, menu_sx_add(s1, 0x134), 0x001d_d2c8)?;
        let s3 = c.g[S3][0];
        menu_page_dtor(c, s1, s3, 0x001d_d2d4)?;
        frame.restore(c, S1, 0x30);
        frame.restore(c, S3, 0x10);
        frame.restore(c, S0, 0x40);
        frame.restore(c, S2, 0x20);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `LobbyMenu_DestroyRoomOccupantsPage(this, a1)` (0x1dd2f0): the item at
/// `+0x4cc`, a list from `+0x4cc` down to `+0x16c` (when present), then two
/// items (`+0x154` before the list's end, `+0x138` after) and the page.
pub fn lobby_menu_destroy_room_occupants_page(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 15;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S1, 0x30);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x40);
        frame.save(c, S3, 0x10);
        c.g[S0][0] = menu_sx_add(c.g[S1][0], 0x4cc);
        c.g[S3][0] = c.g[A1][0];
        frame.save(c, S2, 0x20);
        frame.save_ra(c, 0);
        let s0 = c.g[S0][0];
        menu_item_dtor(c, s0, 0x001d_d320)?;
        let s1 = c.g[S1][0];
        let v0 = menu_sx_add(s1, 0x16c);
        c.g[V0][0] = v0;
        if v0 != 0 {
            c.cycles += 2;
            let same = v0 == c.g[S0][0];
            c.g[S2][0] = v0;
            if !same {
                c.cycles += 1;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x24));
                menu_list_walk(c, 0x001d_d338, 0x001d_d350)?;
            }
            c.cycles += 1;
        }
        c.cycles += 15;
        menu_item_dtor(c, menu_sx_add(s1, 0x154), 0x001d_d364)?;
        menu_item_dtor(c, menu_sx_add(s1, 0x138), 0x001d_d370)?;
        let s3 = c.g[S3][0];
        menu_page_dtor(c, s1, s3, 0x001d_d37c)?;
        frame.restore(c, S1, 0x30);
        frame.restore(c, S0, 0x40);
        frame.restore(c, S3, 0x10);
        frame.restore(c, S2, 0x20);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `LobbyMenu_DestroyChatPage(this, a1)` (0x1dd398): a list from `+0x38c` down
/// to `+0x194` (when present), three items and the page.
pub fn lobby_menu_destroy_chat_page(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x50);
        frame.save(c, S1, 0x30);
        frame.save(c, S3, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x40);
        c.g[S3][0] = c.g[A1][0];
        frame.save(c, S2, 0x20);
        let s1 = c.g[S1][0];
        let v0 = menu_sx_add(s1, 0x194);
        c.g[V0][0] = v0;
        frame.save_ra(c, 0);
        if v0 != 0 {
            c.cycles += 3;
            c.g[S0][0] = menu_sx_add(s1, 0x38c);
            let same = v0 == c.g[S0][0];
            c.g[S2][0] = v0;
            if !same {
                c.cycles += 1;
                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_sub(0x24));
                menu_list_walk(c, 0x001d_d3d0, 0x001d_d3e8)?;
            }
        }
        c.cycles += 19;
        menu_item_dtor(c, menu_sx_add(s1, 0x168), 0x001d_d3fc)?;
        menu_item_dtor(c, menu_sx_add(s1, 0x150), 0x001d_d408)?;
        menu_item_dtor(c, menu_sx_add(s1, 0x134), 0x001d_d414)?;
        let s3 = c.g[S3][0];
        menu_page_dtor(c, s1, s3, 0x001d_d420)?;
        frame.restore(c, S1, 0x30);
        frame.restore(c, S3, 0x10);
        frame.restore(c, S0, 0x40);
        frame.restore(c, S2, 0x20);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{covered, fake_call, fall_back, hook, machine, FAKE};
    use crate::difftest::{Rng, DATA};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate: the fakes below only log when this word is set in the machine's RAM.
    pub(crate) const OC506_GATE: u32 = DATA + 0x0020_0000;
    pub(crate) const OC506_MAGIC: u32 = 0x0c0c_506a;
    /// The fixture object (a page) and the vtable its list entries point at.
    pub(crate) const OC506_THIS: u32 = DATA + 0x0020_0100;
    pub(crate) const OC506_VT: u32 = DATA + 0x0020_0800;

    /// A gated fake for a callee in the function table: logs the call with
    /// [`fake_call`] when armed, otherwise runs what the slot held before.
    macro_rules! oc506_fake {
        ($fake:ident, $prev:ident, $target:expr, $generated:path) => {
            static $prev: AtomicUsize = AtomicUsize::new(0);
            fn $fake(c: &mut Cpu) {
                if c.bus.read32(OC506_GATE) != OC506_MAGIC {
                    return fall_back(c, &$prev, $generated);
                }
                fake_call(c, $target);
            }
        };
    }

    oc506_fake!(
        oc506_fake_item,
        OC506_PREV_ITEM,
        0x002c_a280,
        ee::s09::f_002ca280__generated
    );
    oc506_fake!(
        oc506_fake_page,
        OC506_PREV_PAGE,
        0x002c_aa80,
        ee::s09::f_002caa80__generated
    );
    oc506_fake!(
        oc506_fake_delete,
        OC506_PREV_DELETE,
        0x0031_7e50,
        ee::s10::f_00317e50__generated
    );
    oc506_fake!(
        oc506_fake_flag27,
        OC506_PREV_FLAG27,
        0x001e_1090,
        ee::s04::f_001e1090__generated
    );
    oc506_fake!(
        oc506_fake_free_backgrounds,
        OC506_PREV_FREE_BG,
        0x0017_abe0,
        ee::s02::f_0017abe0__generated
    );
    oc506_fake!(
        oc506_fake_get_human,
        OC506_PREV_GET_HUMAN,
        0x001a_0538,
        ee::s03::f_001a0538__generated
    );
    oc506_fake!(
        oc506_fake_get_human1,
        OC506_PREV_GET_HUMAN1,
        0x001a_0548,
        ee::s03::f_001a0548__generated
    );

    /// Install the fakes in their override slots (once per process).
    fn oc506_hook_all() {
        hook(
            addr_of_mut!(ee::s09::OVERRIDE_002CA280),
            oc506_fake_item,
            &OC506_PREV_ITEM,
        );
        hook(
            addr_of_mut!(ee::s09::OVERRIDE_002CAA80),
            oc506_fake_page,
            &OC506_PREV_PAGE,
        );
        hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317E50),
            oc506_fake_delete,
            &OC506_PREV_DELETE,
        );
        hook(
            addr_of_mut!(ee::s04::OVERRIDE_001E1090),
            oc506_fake_flag27,
            &OC506_PREV_FLAG27,
        );
        hook(
            addr_of_mut!(ee::s02::OVERRIDE_0017ABE0),
            oc506_fake_free_backgrounds,
            &OC506_PREV_FREE_BG,
        );
        hook(
            addr_of_mut!(ee::s03::OVERRIDE_001A0538),
            oc506_fake_get_human,
            &OC506_PREV_GET_HUMAN,
        );
        hook(
            addr_of_mut!(ee::s03::OVERRIDE_001A0548),
            oc506_fake_get_human1,
            &OC506_PREV_GET_HUMAN1,
        );
    }

    /// Scrambled machine, the gate armed, and a vtable whose function is the
    /// log fake ([`FAKE`]) for the list entries' destructors.
    fn oc506_arm(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        let r = machine(c, seed, salt, calls);
        c.bus.write32(OC506_GATE, OC506_MAGIC);
        c.bus.write32(OC506_VT + 0x8, 0);
        c.bus.write32(OC506_VT + 0xc, FAKE);
        r
    }

    /// Mark the list entries from `stop` up to `end` (absolute addresses, a
    /// multiple of 0x24 apart) so their vtable pointer leads to [`OC506_VT`].
    fn oc506_entries(c: &mut Cpu, stop: u32, end: u32) {
        let mut p = stop;
        while p < end {
            c.bus.write32(p + 0x10, OC506_VT);
            p += 0x24;
        }
    }

    /// `this` for a page: the fixture object, or (for the guard of a list
    /// that must be empty) an address that makes `this + off` zero.
    fn oc506_this(seed: u32, empty_off: u32) -> u64 {
        if seed % 4 == 3 {
            0u64.wrapping_sub(empty_off as u64)
        } else {
            OC506_THIS as u64
        }
    }

    #[test]
    fn fe_module_get_rider_preview_2_thunk_matches() {
        oc506_hook_all();
        covered(
            ee::s04::f_001dcb70__generated,
            fe_module_get_rider_preview_2_thunk,
            200,
            &|c, seed| {
                oc506_arm(c, seed, 0x1dcb70, 2);
                c.g[A0][0] = OC506_THIS as u64;
            },
        );
    }

    #[test]
    fn fe_module_get_rider_preview_thunk_matches() {
        oc506_hook_all();
        covered(
            ee::s04::f_001dcb90__generated,
            fe_module_get_rider_preview_thunk,
            200,
            &|c, seed| {
                oc506_arm(c, seed, 0x1dcb90, 2);
                c.g[A0][0] = OC506_THIS as u64;
            },
        );
    }

    #[test]
    fn felocale_dtor_matches() {
        oc506_hook_all();
        covered(ee::s04::f_001dcbd8__generated, felocale_dtor, 200, &|c, seed| {
            let mut r = oc506_arm(c, seed, 0x1dcbd8, 2);
            c.g[A0][0] = OC506_THIS as u64;
            c.g[A1][0] = r.next_u32() as u64;
        });
    }

    #[test]
    fn felocale_v02_matches() {
        oc506_hook_all();
        covered(ee::s04::f_001dcc08__generated, felocale_v02, 200, &|c, seed| {
            let mut r = oc506_arm(c, seed, 0x1dcc08, 2);
            c.g[A0][0] = OC506_THIS as u64;
            c.g[A1][0] = r.next_u32() as u64;
        });
    }

    #[test]
    fn debug_menu_v01_matches() {
        oc506_hook_all();
        covered(ee::s04::f_001dcd00__generated, debug_menu_v01, 200, &|c, seed| {
            let mut r = oc506_arm(c, seed, 0x1dcd00, 3);
            c.g[A0][0] = OC506_THIS as u64;
            c.g[A1][0] = r.next_u32() as u64;
        });
    }

    #[test]
    fn lobby_menu_message_page_dtor_matches() {
        oc506_hook_all();
        covered(
            ee::s04::f_001dcd58__generated,
            lobby_menu_message_page_dtor,
            200,
            &|c, seed| {
                let mut r = oc506_arm(c, seed, 0x1dcd58, 3);
                c.g[A0][0] = OC506_THIS as u64;
                c.g[A1][0] = r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn lobby_menu_destroy_login_page_matches() {
        oc506_hook_all();
        covered(
            ee::s04::f_001dcda0__generated,
            lobby_menu_destroy_login_page,
            200,
            &|c, seed| {
                let mut r = oc506_arm(c, seed, 0x1dcda0, 8);
                c.g[A0][0] = OC506_THIS as u64;
                c.g[A1][0] = r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn lobby_menu_destroy_create_account_page_matches() {
        oc506_hook_all();
        covered(
            ee::s04::f_001dce20__generated,
            lobby_menu_destroy_create_account_page,
            200,
            &|c, seed| {
                let mut r = oc506_arm(c, seed, 0x1dce20, 8);
                c.g[A0][0] = OC506_THIS as u64;
                c.g[A1][0] = r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn lobby_menu_destroy_login_method_page_matches() {
        oc506_hook_all();
        covered(
            ee::s04::f_001dcea0__generated,
            lobby_menu_destroy_login_method_page,
            200,
            &|c, seed| {
                let mut r = oc506_arm(c, seed, 0x1dcea0, 24);
                c.g[A0][0] = OC506_THIS as u64;
                c.g[A1][0] = r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn lobby_menu_destroy_delete_persona_page_matches() {
        oc506_hook_all();
        covered(
            ee::s04::f_001dcfc8__generated,
            lobby_menu_destroy_delete_persona_page,
            200,
            &|c, seed| {
                let mut r = oc506_arm(c, seed, 0x1dcfc8, 8);
                let this = oc506_this(seed, 0x168);
                c.g[A0][0] = this;
                c.g[A1][0] = r.next_u32() as u64;
                if this == OC506_THIS as u64 {
                    oc506_entries(c, OC506_THIS + 0x168, OC506_THIS + 0x1f8);
                }
            },
        );
    }

    #[test]
    fn lobby_menu_destroy_create_persona_page_matches() {
        oc506_hook_all();
        covered(
            ee::s04::f_001dd060__generated,
            lobby_menu_destroy_create_persona_page,
            200,
            &|c, seed| {
                let mut r = oc506_arm(c, seed, 0x1dd060, 6);
                c.g[A0][0] = OC506_THIS as u64;
                c.g[A1][0] = r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn lobby_menu_destroy_select_persona_page_matches() {
        oc506_hook_all();
        covered(
            ee::s04::f_001dd0c8__generated,
            lobby_menu_destroy_select_persona_page,
            200,
            &|c, seed| {
                let mut r = oc506_arm(c, seed, 0x1dd0c8, 40);
                c.g[A0][0] = OC506_THIS as u64;
                c.g[A1][0] = r.next_u32() as u64;
                oc506_entries(c, OC506_THIS + 0x518, OC506_THIS + 0x5a8);
                oc506_entries(c, OC506_THIS + 0x450, OC506_THIS + 0x4e0);
            },
        );
    }

    #[test]
    fn lobby_menu_destroy_select_room_page_matches() {
        oc506_hook_all();
        covered(
            ee::s04::f_001dd258__generated,
            lobby_menu_destroy_select_room_page,
            200,
            &|c, seed| {
                let mut r = oc506_arm(c, seed, 0x1dd258, 8);
                let this = oc506_this(seed, 0x168);
                c.g[A0][0] = this;
                c.g[A1][0] = r.next_u32() as u64;
                if this == OC506_THIS as u64 {
                    oc506_entries(c, OC506_THIS + 0x168, OC506_THIS + 0x318);
                }
            },
        );
    }

    #[test]
    fn lobby_menu_destroy_room_occupants_page_matches() {
        oc506_hook_all();
        covered(
            ee::s04::f_001dd2f0__generated,
            lobby_menu_destroy_room_occupants_page,
            200,
            &|c, seed| {
                let mut r = oc506_arm(c, seed, 0x1dd2f0, 8);
                let this = oc506_this(seed, 0x16c);
                c.g[A0][0] = this;
                c.g[A1][0] = r.next_u32() as u64;
                if this == OC506_THIS as u64 {
                    oc506_entries(c, OC506_THIS + 0x16c, OC506_THIS + 0x4cc);
                }
            },
        );
    }

    #[test]
    fn lobby_menu_destroy_chat_page_matches() {
        oc506_hook_all();
        covered(
            ee::s04::f_001dd398__generated,
            lobby_menu_destroy_chat_page,
            200,
            &|c, seed| {
                let mut r = oc506_arm(c, seed, 0x1dd398, 8);
                let this = oc506_this(seed, 0x194);
                c.g[A0][0] = this;
                c.g[A1][0] = r.next_u32() as u64;
                if this == OC506_THIS as u64 {
                    oc506_entries(c, OC506_THIS + 0x194, OC506_THIS + 0x38c);
                }
            },
        );
    }
}
