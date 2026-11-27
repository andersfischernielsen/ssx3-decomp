//! Staging module oc947 (wave wave256): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc947`.

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
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x001d_a7e8,
        lobby_menu_init_login_page,
        "LobbyMenu_InitLoginPage",
    ),
    (
        0x001d_a988,
        lobby_menu_init_login_method_page,
        "LobbyMenu_InitLoginMethodPage",
    ),
    (
        0x001d_aac8,
        lobby_menu_init_create_persona_page,
        "LobbyMenu_InitCreatePersonaPage",
    ),
    (
        0x001d_abd0,
        lobby_menu_init_delete_persona_page,
        "LobbyMenu_InitDeletePersonaPage",
    ),
    (
        0x001d_acb8,
        lobby_menu_init_select_persona_page,
        "LobbyMenu_InitSelectPersonaPage",
    ),
    (
        0x001d_ae20,
        lobby_menu_init_select_room_page,
        "LobbyMenu_InitSelectRoomPage",
    ),
    (
        0x001d_af08,
        lobby_menu_init_room_occupants_page,
        "LobbyMenu_InitRoomOccupantsPage",
    ),
    (
        0x001d_b028,
        lobby_menu_init_chat_page,
        "LobbyMenu_InitChatPage",
    ),
];

/// `reg + off` as the 32-bit sum the generated code computes.
#[inline(always)]
fn init_pages_at(c: &Cpu, r: usize, off: u32) -> u32 {
    (c.g[r][0] as u32).wrapping_add(off)
}

/// `addiu reg, reg, off`: the sign-extended 32-bit sum, as a register value.
#[inline(always)]
fn init_pages_reg(c: &Cpu, r: usize, off: u32) -> u64 {
    sx(init_pages_at(c, r, off))
}

/// `void LobbyMenu_InitLoginPage(LobbyMenu* menu, void* state, u32 arg)`
/// (0x1da7e8): lay out the login page. Fills the page's widget slots at
/// `menu+0x12c..+0x1e4` through the menu-item and text calls, and clears the
/// first byte of the two text buffers at `menu+0x180` and `menu+0x1ac`.
pub fn lobby_menu_init_login_page(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 103;
        let frame = Frame::open(c, 0xa0);
        frame.save(c, S1, 0x80);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x90);
        frame.save(c, FP, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S2, 0x70);
        c.g[FP][0] = c.g[A2][0];
        frame.save(c, S3, 0x60);
        frame.save(c, S4, 0x50);
        c.g[S3][0] = init_pages_reg(c, S1, 0x134);
        frame.save(c, S5, 0x40);
        c.g[S2][0] = init_pages_reg(c, S1, 0x150);
        frame.save(c, S6, 0x30);
        c.g[S4][0] = init_pages_reg(c, S1, 0x168);
        frame.save(c, S7, 0x20);
        c.g[S5][0] = init_pages_reg(c, S1, 0x194);
        frame.save_ra(c, 0);
        c.g[S6][0] = init_pages_reg(c, S1, 0x1c0);
        call_ptr(c, 0x002c_aa58, 0x001d_a838)?;
        c.g[S7][0] = init_pages_reg(c, S1, 0x1e4);
        // Globals: 0x470000 - 0x2918 = D_0046D6E8, 0x460000 + 0x7a20 = D_00467A20.
        c.g[V0][0] = 0x47_0000 - 0x2918;
        c.g[A0][0] = c.g[S3][0];
        let label = c.g[V0][0];
        Addr(init_pages_at(c, S1, 0x12c)).sw(c, label);
        c.g[A1][0] = 0x46_0000 + 0x7a20;
        let text = Addr(c.g[S0][0] as u32).lw(c);
        c.g[V0][0] = text;
        Addr(init_pages_at(c, S1, 0x130)).sw(c, text);
        call_ptr(c, 0x002c_d2a0, 0x001d_a860)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = 0xa;
        call_ptr(c, 0x002c_d350, 0x001d_a86c)?;
        // D_004A1408 and the four globals below it are the widget data.
        c.g[S0][0] = 0x4a_0000 + 0x1408;
        c.g[A1][0] = 0x46_0000 + 0x7a30;
        c.g[A0][0] = c.g[S4][0];
        c.g[A2][0] = c.g[S0][0];
        call_ptr(c, 0x002c_e418, 0x001d_a888)?;
        c.g[A1][0] = 0x46_0000 + 0x7a40;
        c.g[A2][0] = c.g[S0][0];
        c.g[A0][0] = c.g[S5][0];
        call_ptr(c, 0x002c_e418, 0x001d_a89c)?;
        c.g[A3][0] = 0;
        c.g[A2][0] = c.g[FP][0];
        c.g[A0][0] = c.g[S6][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x002c_cf98, 0x001d_a8b0)?;
        c.g[A1][0] = 0x46_0000 + 0x7a50;
        c.g[A0][0] = c.g[S7][0];
        call_ptr(c, 0x002c_cdf0, 0x001d_a8c0)?;
        c.g[A1][0] = c.g[S3][0];
        c.g[A0][0] = c.g[S1][0];
        c.g[A2][0] = u64::MAX;
        call_ptr(c, 0x002c_bcc8, 0x001d_a8d0)?;
        c.g[A1][0] = c.g[S2][0];
        c.g[A0][0] = c.g[S1][0];
        c.g[A2][0] = u64::MAX;
        call_ptr(c, 0x002c_bcc8, 0x001d_a8e0)?;
        c.g[A1][0] = c.g[S4][0];
        c.g[A0][0] = c.g[S1][0];
        c.g[A2][0] = u64::MAX;
        call_ptr(c, 0x002c_bcc8, 0x001d_a8f0)?;
        c.g[A1][0] = c.g[S5][0];
        c.g[A0][0] = c.g[S1][0];
        c.g[A2][0] = u64::MAX;
        call_ptr(c, 0x002c_bcc8, 0x001d_a900)?;
        c.g[A1][0] = c.g[S6][0];
        c.g[A0][0] = c.g[S1][0];
        c.g[A2][0] = u64::MAX;
        call_ptr(c, 0x002c_bcc8, 0x001d_a910)?;
        c.g[A1][0] = c.g[S7][0];
        c.g[A0][0] = c.g[S1][0];
        c.g[A2][0] = u64::MAX;
        call_ptr(c, 0x002c_bcc8, 0x001d_a920)?;
        // The world pointer at gp-0x848: its +0xc4 and +0xc8 are the two texts.
        let world = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
        c.g[V0][0] = world;
        c.g[A0][0] = init_pages_reg(c, S1, 0x180);
        c.g[A2][0] = 0xf;
        let text0 = Addr((world as u32).wrapping_add(0xc4)).lw(c);
        c.g[A1][0] = text0;
        Addr(init_pages_at(c, S1, 0x180)).sb(c, 0);
        call_ptr(c, 0x0041_6958, 0x001d_a938)?;
        let world = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
        c.g[V1][0] = world;
        c.g[A0][0] = init_pages_reg(c, S1, 0x1ac);
        c.g[A2][0] = 0xf;
        let text1 = Addr((world as u32).wrapping_add(0xc8)).lw(c);
        c.g[A1][0] = text1;
        Addr(init_pages_at(c, S1, 0x1ac)).sb(c, 0);
        call_ptr(c, 0x0041_6958, 0x001d_a950)?;
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, S0, 0x90);
        frame.restore(c, S1, 0x80);
        frame.restore(c, S2, 0x70);
        frame.restore(c, S3, 0x60);
        frame.restore(c, S4, 0x50);
        frame.restore(c, S5, 0x40);
        frame.restore(c, S6, 0x30);
        frame.restore(c, S7, 0x20);
        frame.restore(c, FP, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void LobbyMenu_InitLoginMethodPage(LobbyMenu* menu, void* state, u32 arg)`
/// (0x1da988): lay out the login-method page. Builds the create-account
/// and login pages it hands over to, then fills the method list's slots.
pub fn lobby_menu_init_login_method_page(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 80;
        let frame = Frame::open(c, 0xa0);
        frame.save(c, S0, 0x90);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x80);
        frame.save(c, S3, 0x60);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S2, 0x70);
        c.g[S3][0] = c.g[A2][0];
        frame.save(c, S4, 0x50);
        frame.save(c, S5, 0x40);
        c.g[S4][0] = init_pages_reg(c, S0, 0x32c);
        frame.save(c, S6, 0x30);
        c.g[S5][0] = init_pages_reg(c, S0, 0x544);
        frame.save(c, S7, 0x20);
        c.g[S6][0] = init_pages_reg(c, S0, 0x55c);
        frame.save(c, FP, 0x10);
        c.g[S7][0] = init_pages_reg(c, S0, 0x578);
        frame.save_ra(c, 0);
        c.g[FP][0] = init_pages_reg(c, S0, 0x130);
        call_ptr(c, 0x002c_aa58, 0x001d_a9d8)?;
        // Globals: 0x470000 - 0x29c8 = D_0046D638, 0x460000 + 0x7a20 = D_00467A20.
        c.g[V0][0] = 0x47_0000 - 0x29c8;
        c.g[S2][0] = 0x46_0000 + 0x7a20;
        c.g[A1][0] = c.g[S1][0];
        c.g[A0][0] = c.g[FP][0];
        c.g[A2][0] = c.g[S2][0];
        let label = c.g[V0][0];
        Addr(init_pages_at(c, S0, 0x12c)).sw(c, label);
        Callee {
            addr: 0x001d_a648,
            f: ee::s04::LobbyMenu_InitCreateAccountPage,
        }
        .call(c, 0x001d_a9fc)?;
        c.g[S1][0] = 0x46_0000 + 0x7a58;
        c.g[A1][0] = c.g[S3][0];
        c.g[A0][0] = c.g[S4][0];
        c.g[A2][0] = c.g[S1][0];
        c.g[S3][0] = init_pages_reg(c, S0, 0x528);
        Callee {
            addr: 0x001d_a7e8,
            f: ee::s04::LobbyMenu_InitLoginPage,
        }
        .call(c, 0x001d_aa18)?;
        c.g[A1][0] = 0x46_0000 + 0x7a60;
        c.g[A0][0] = c.g[S3][0];
        call_ptr(c, 0x002c_d2a0, 0x001d_aa28)?;
        c.g[A0][0] = c.g[S5][0];
        c.g[A1][0] = 0xa;
        call_ptr(c, 0x002c_d350, 0x001d_aa34)?;
        c.g[A1][0] = c.g[FP][0];
        c.g[A2][0] = c.g[S2][0];
        c.g[A0][0] = c.g[S6][0];
        call_ptr(c, 0x002c_ccc8, 0x001d_aa44)?;
        c.g[A1][0] = c.g[S4][0];
        c.g[A2][0] = c.g[S1][0];
        c.g[A0][0] = c.g[S7][0];
        call_ptr(c, 0x002c_ccc8, 0x001d_aa54)?;
        c.g[A1][0] = c.g[S3][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = u64::MAX;
        call_ptr(c, 0x002c_bcc8, 0x001d_aa64)?;
        c.g[A1][0] = c.g[S5][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = u64::MAX;
        call_ptr(c, 0x002c_bcc8, 0x001d_aa74)?;
        c.g[A1][0] = c.g[S6][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = u64::MAX;
        call_ptr(c, 0x002c_bcc8, 0x001d_aa84)?;
        c.g[A1][0] = c.g[S7][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = u64::MAX;
        call_ptr(c, 0x002c_bcc8, 0x001d_aa94)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x80);
        frame.restore(c, S0, 0x90);
        frame.restore(c, S2, 0x70);
        frame.restore(c, S3, 0x60);
        frame.restore(c, S4, 0x50);
        frame.restore(c, S5, 0x40);
        frame.restore(c, S6, 0x30);
        frame.restore(c, S7, 0x20);
        frame.restore(c, FP, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void LobbyMenu_InitCreatePersonaPage(LobbyMenu* menu, void* state)`
/// (0x1daac8): lay out the create-persona page: its widget slots at
/// `menu+0x134..`, the two text labels, and the persona list rows.
pub fn lobby_menu_init_create_persona_page(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 65;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S0, 0x60);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x50);
        frame.save(c, S2, 0x40);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S3, 0x30);
        frame.save(c, S4, 0x20);
        c.g[S3][0] = init_pages_reg(c, S0, 0x134);
        frame.save(c, S5, 0x10);
        c.g[S4][0] = init_pages_reg(c, S0, 0x150);
        frame.save_ra(c, 0);
        c.g[S2][0] = init_pages_reg(c, S0, 0x168);
        call_ptr(c, 0x002c_aa58, 0x001d_ab00)?;
        c.g[S5][0] = init_pages_reg(c, S0, 0x194);
        // Globals: 0x470000 - 0x2a78 = D_0046D588, 0x460000 + 0x7a70 = D_00467A70.
        c.g[V0][0] = 0x47_0000 - 0x2a78;
        c.g[A1][0] = 0x46_0000 + 0x7a70;
        c.g[A0][0] = c.g[S3][0];
        let label = c.g[V0][0];
        Addr(init_pages_at(c, S0, 0x12c)).sw(c, label);
        let text = Addr(c.g[S1][0] as u32).lw(c);
        c.g[V0][0] = text;
        Addr(init_pages_at(c, S0, 0x130)).sw(c, text);
        call_ptr(c, 0x002c_d2a0, 0x001d_ab28)?;
        c.g[A0][0] = c.g[S4][0];
        c.g[A1][0] = 0xe;
        call_ptr(c, 0x002c_d350, 0x001d_ab34)?;
        c.g[A1][0] = 0x46_0000;
        c.g[A2][0] = 0x4a_0000 + 0x1408;
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = 0x46_0000 + 0x7a80;
        call_ptr(c, 0x002c_e418, 0x001d_ab4c)?;
        c.g[A2][0] = 0x46_0000 + 0x7a90;
        c.g[A3][0] = 0;
        c.g[A0][0] = c.g[S5][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x002c_cf98, 0x001d_ab64)?;
        c.g[A1][0] = c.g[S3][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = u64::MAX;
        call_ptr(c, 0x002c_bcc8, 0x001d_ab74)?;
        c.g[A1][0] = c.g[S4][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = u64::MAX;
        call_ptr(c, 0x002c_bcc8, 0x001d_ab84)?;
        c.g[A1][0] = c.g[S2][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = u64::MAX;
        call_ptr(c, 0x002c_bcc8, 0x001d_ab94)?;
        c.g[A1][0] = c.g[S5][0];
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = u64::MAX;
        call_ptr(c, 0x002c_bcc8, 0x001d_aba4)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x50);
        frame.restore(c, S0, 0x60);
        frame.restore(c, S2, 0x40);
        frame.restore(c, S3, 0x30);
        frame.restore(c, S4, 0x20);
        frame.restore(c, S5, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void LobbyMenu_InitDeletePersonaPage(LobbyMenu* menu, void* state)`
/// (0x1dabd0): lay out the delete-persona page. Runs the row-count loop
/// over the persona list (four passes) and then the label loop.
pub fn lobby_menu_init_delete_persona_page(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S1, 0x40);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x50);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S2, 0x30);
        frame.save(c, S3, 0x20);
        frame.save(c, S4, 0x10);
        c.g[S2][0] = init_pages_reg(c, S1, 0x168);
        frame.save_ra(c, 0);
        c.g[S3][0] = 3;
        call_ptr(c, 0x002c_aa58, 0x001d_ac00)?;
        c.g[S4][0] = u64::MAX;
        // Globals: 0x470000 - 0x2a20 = D_0046D5E0, 0x460000 + 0x7a98 = D_00467A98.
        c.g[V0][0] = 0x47_0000 - 0x2a20;
        c.g[A1][0] = 0x46_0000 + 0x7a98;
        let label = c.g[V0][0];
        Addr(init_pages_at(c, S1, 0x12c)).sw(c, label);
        c.g[A0][0] = init_pages_reg(c, S1, 0x134);
        let text = Addr(c.g[S0][0] as u32).lw(c);
        c.g[V0][0] = text;
        Addr(init_pages_at(c, S1, 0x130)).sw(c, text);
        call_ptr(c, 0x002c_d2a0, 0x001d_ac28)?;
        c.g[A0][0] = init_pages_reg(c, S1, 0x150);
        c.g[A1][0] = 0xe;
        call_ptr(c, 0x002c_d350, 0x001d_ac34)?;
        loop {
            c.cycles += 5;
            c.g[A0][0] = c.g[S2][0];
            c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_sub(1));
            call_ptr(c, 0x002c_cf08, 0x001d_ac44)?;
            let more = c.g[S3][0] != c.g[S4][0];
            c.g[S2][0] = init_pages_reg(c, S2, 0x24);
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x001d_ac38);
                }
                continue;
            }
            break;
        }
        c.cycles += 11;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = init_pages_reg(c, S1, 0x134);
        c.g[A2][0] = u64::MAX;
        c.g[S0][0] = init_pages_reg(c, S1, 0x168);
        call_ptr(c, 0x002c_bcc8, 0x001d_ac60)?;
        c.g[S2][0] = 3;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = init_pages_reg(c, S1, 0x150);
        c.g[A2][0] = u64::MAX;
        call_ptr(c, 0x002c_bcc8, 0x001d_ac74)?;
        c.g[A1][0] = c.g[S0][0];
        loop {
            c.cycles += 7;
            c.g[A0][0] = c.g[S1][0];
            c.g[A2][0] = u64::MAX;
            c.g[S0][0] = init_pages_reg(c, S0, 0x24);
            call_ptr(c, 0x002c_bcc8, 0x001d_ac88)?;
            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(1));
            let more = (c.g[S2][0] as i64) >= 0;
            c.g[A1][0] = c.g[S0][0];
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x001d_ac78);
                }
                continue;
            }
            break;
        }
        c.cycles += 9;
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void LobbyMenu_InitSelectPersonaPage(LobbyMenu* menu, void* a1, void* state,
/// void* a3)` (0x1dacb8): lay out the select-persona page. Builds the
/// create-persona and delete-persona pages, the persona list rows and their
/// label cells.
pub fn lobby_menu_init_select_persona_page(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 34;
        let frame = Frame::open(c, 0x70);
        frame.save(c, S3, 0x30);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, S0, 0x60);
        frame.save(c, S1, 0x50);
        c.g[S0][0] = c.g[A2][0];
        frame.save(c, S2, 0x40);
        c.g[S1][0] = c.g[A3][0];
        frame.save(c, S4, 0x20);
        c.g[S2][0] = c.g[A1][0];
        frame.save(c, S5, 0x10);
        frame.save_ra(c, 0);
        c.g[S4][0] = init_pages_reg(c, S3, 0x518);
        call_ptr(c, 0x002c_aa58, 0x001d_acf0)?;
        c.g[S5][0] = 3;
        // Globals: 0x470000 - 0x2ad0 = D_0046D530, 0x460000 + 0x7aa8 = D_00467AA8.
        c.g[V0][0] = 0x47_0000 - 0x2ad0;
        c.g[A1][0] = c.g[S0][0];
        c.g[A0][0] = init_pages_reg(c, S3, 0x130);
        let label = c.g[V0][0];
        Addr(init_pages_at(c, S3, 0x12c)).sw(c, label);
        c.g[S0][0] = u64::MAX;
        Callee {
            addr: 0x001d_aac8,
            f: ee::s04::LobbyMenu_InitCreatePersonaPage,
        }
        .call(c, 0x001d_ad10)?;
        c.g[A1][0] = c.g[S1][0];
        c.g[A0][0] = init_pages_reg(c, S3, 0x2e8);
        Callee {
            addr: 0x001d_abd0,
            f: ee::s04::LobbyMenu_InitDeletePersonaPage,
        }
        .call(c, 0x001d_ad1c)?;
        let text = Addr(c.g[S2][0] as u32).lw(c);
        c.g[V0][0] = text;
        c.g[A1][0] = 0x46_0000 + 0x7aa8;
        c.g[A0][0] = init_pages_reg(c, S3, 0x4e4);
        Addr(init_pages_at(c, S3, 0x4e0)).sw(c, text);
        call_ptr(c, 0x002c_d2a0, 0x001d_ad34)?;
        c.g[A0][0] = init_pages_reg(c, S3, 0x500);
        c.g[A1][0] = 0xe;
        call_ptr(c, 0x002c_d350, 0x001d_ad40)?;
        loop {
            c.cycles += 5;
            c.g[A0][0] = c.g[S4][0];
            c.g[S5][0] = sx((c.g[S5][0] as u32).wrapping_sub(1));
            call_ptr(c, 0x002c_cf08, 0x001d_ad4c)?;
            let more = c.g[S5][0] != c.g[S0][0];
            c.g[S4][0] = init_pages_reg(c, S4, 0x24);
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x001d_ad40);
                }
                continue;
            }
            break;
        }
        c.cycles += 25;
        c.g[A0][0] = init_pages_reg(c, S3, 0x5a8);
        c.g[A2][0] = 0x46_0000 + 0x7a70;
        c.g[S4][0] = c.g[A0][0];
        c.g[A1][0] = init_pages_reg(c, S3, 0x130);
        c.g[S0][0] = init_pages_reg(c, S3, 0x5c4);
        call_ptr(c, 0x002c_ccc8, 0x001d_ad70)?;
        c.g[S2][0] = c.g[S0][0];
        c.g[A2][0] = 0x46_0000 + 0x7a98;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = init_pages_reg(c, S3, 0x2e8);
        call_ptr(c, 0x002c_ccc8, 0x001d_ad88)?;
        c.g[S0][0] = init_pages_reg(c, S3, 0x518);
        c.g[A0][0] = c.g[S3][0];
        c.g[A1][0] = init_pages_reg(c, S3, 0x4e4);
        c.g[A2][0] = u64::MAX;
        c.g[S1][0] = 3;
        call_ptr(c, 0x002c_bcc8, 0x001d_ada0)?;
        c.g[A0][0] = c.g[S3][0];
        c.g[A1][0] = init_pages_reg(c, S3, 0x500);
        c.g[A2][0] = u64::MAX;
        call_ptr(c, 0x002c_bcc8, 0x001d_adb0)?;
        c.g[A1][0] = c.g[S0][0];
        loop {
            c.cycles += 7;
            c.g[A0][0] = c.g[S3][0];
            c.g[A2][0] = u64::MAX;
            c.g[S0][0] = init_pages_reg(c, S0, 0x24);
            call_ptr(c, 0x002c_bcc8, 0x001d_adc8)?;
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(1));
            let more = (c.g[S1][0] as i64) >= 0;
            c.g[A1][0] = c.g[S0][0];
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x001d_adb8);
                }
                continue;
            }
            break;
        }
        c.cycles += 18;
        c.g[A1][0] = c.g[S4][0];
        c.g[A0][0] = c.g[S3][0];
        c.g[A2][0] = u64::MAX;
        call_ptr(c, 0x002c_bcc8, 0x001d_ade4)?;
        c.g[A1][0] = c.g[S2][0];
        c.g[A0][0] = c.g[S3][0];
        c.g[A2][0] = u64::MAX;
        call_ptr(c, 0x002c_bcc8, 0x001d_adf4)?;
        c.g[V0][0] = c.g[S3][0];
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

/// `void LobbyMenu_InitSelectRoomPage(LobbyMenu* menu, void* state)`
/// (0x1dae20): lay out the select-room page. Same shape as the
/// delete-persona page, with twelve row passes instead of four.
pub fn lobby_menu_init_select_room_page(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S1, 0x40);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x50);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S2, 0x30);
        frame.save(c, S3, 0x20);
        frame.save(c, S4, 0x10);
        c.g[S2][0] = init_pages_reg(c, S1, 0x168);
        frame.save_ra(c, 0);
        c.g[S3][0] = 0xb;
        call_ptr(c, 0x002c_aa58, 0x001d_ae50)?;
        c.g[S4][0] = u64::MAX;
        // Globals: 0x470000 - 0x2b28 = D_0046D4D8, 0x460000 + 0x7ab8 = D_00467AB8.
        c.g[V0][0] = 0x47_0000 - 0x2b28;
        c.g[A1][0] = 0x46_0000 + 0x7ab8;
        let label = c.g[V0][0];
        Addr(init_pages_at(c, S1, 0x12c)).sw(c, label);
        c.g[A0][0] = init_pages_reg(c, S1, 0x134);
        let text = Addr(c.g[S0][0] as u32).lw(c);
        c.g[V0][0] = text;
        Addr(init_pages_at(c, S1, 0x130)).sw(c, text);
        call_ptr(c, 0x002c_d2a0, 0x001d_ae78)?;
        c.g[A0][0] = init_pages_reg(c, S1, 0x150);
        c.g[A1][0] = 0xe;
        call_ptr(c, 0x002c_d350, 0x001d_ae84)?;
        loop {
            c.cycles += 5;
            c.g[A0][0] = c.g[S2][0];
            c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_sub(1));
            call_ptr(c, 0x002c_cf08, 0x001d_ae94)?;
            let more = c.g[S3][0] != c.g[S4][0];
            c.g[S2][0] = init_pages_reg(c, S2, 0x24);
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x001d_ae88);
                }
                continue;
            }
            break;
        }
        c.cycles += 11;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = init_pages_reg(c, S1, 0x134);
        c.g[A2][0] = u64::MAX;
        c.g[S0][0] = init_pages_reg(c, S1, 0x168);
        call_ptr(c, 0x002c_bcc8, 0x001d_aeb0)?;
        c.g[S2][0] = 0xb;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = init_pages_reg(c, S1, 0x150);
        c.g[A2][0] = u64::MAX;
        call_ptr(c, 0x002c_bcc8, 0x001d_aec4)?;
        c.g[A1][0] = c.g[S0][0];
        loop {
            c.cycles += 7;
            c.g[A0][0] = c.g[S1][0];
            c.g[A2][0] = u64::MAX;
            c.g[S0][0] = init_pages_reg(c, S0, 0x24);
            call_ptr(c, 0x002c_bcc8, 0x001d_aed8)?;
            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_sub(1));
            let more = (c.g[S2][0] as i64) >= 0;
            c.g[A1][0] = c.g[S0][0];
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x001d_aec8);
                }
                continue;
            }
            break;
        }
        c.cycles += 9;
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void LobbyMenu_InitRoomOccupantsPage(LobbyMenu* menu, void* a1, void* a2)`
/// (0x1daf08): lay out the room-occupants page: the occupant rows at
/// `menu+0x16c` and `menu+0x138..`, and the two label cells.
pub fn lobby_menu_init_room_occupants_page(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 28;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S2, 0x30);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0x50);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S1, 0x40);
        c.g[S1][0] = c.g[A2][0];
        frame.save(c, S3, 0x20);
        frame.save(c, S4, 0x10);
        frame.save_ra(c, 0);
        c.g[S3][0] = init_pages_reg(c, S2, 0x16c);
        call_ptr(c, 0x002c_aa58, 0x001d_af38)?;
        c.g[S4][0] = 0x17;
        // Globals: 0x470000 - 0x2b80 = D_0046D480, 0x460000 + 0x7ac8 = D_00467AC8.
        c.g[V0][0] = 0x47_0000 - 0x2b80;
        c.g[A1][0] = 0x46_0000 + 0x7ac8;
        let label = c.g[V0][0];
        Addr(init_pages_at(c, S2, 0x12c)).sw(c, label);
        c.g[A0][0] = init_pages_reg(c, S2, 0x138);
        let first = Addr(c.g[S0][0] as u32).lw(c);
        c.g[V1][0] = first;
        c.g[S0][0] = u64::MAX;
        Addr(init_pages_at(c, S2, 0x130)).sw(c, first);
        let second = Addr(c.g[S1][0] as u32).lw(c);
        c.g[V0][0] = second;
        Addr(init_pages_at(c, S2, 0x134)).sw(c, second);
        call_ptr(c, 0x002c_d2a0, 0x001d_af6c)?;
        c.g[A0][0] = init_pages_reg(c, S2, 0x154);
        c.g[A1][0] = 0xe;
        call_ptr(c, 0x002c_d350, 0x001d_af78)?;
        loop {
            c.cycles += 5;
            c.g[A0][0] = c.g[S3][0];
            c.g[S4][0] = sx((c.g[S4][0] as u32).wrapping_sub(1));
            call_ptr(c, 0x002c_cf08, 0x001d_af84)?;
            let more = c.g[S4][0] != c.g[S0][0];
            c.g[S3][0] = init_pages_reg(c, S3, 0x24);
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x001d_af78);
                }
                continue;
            }
            break;
        }
        c.cycles += 19;
        c.g[A0][0] = init_pages_reg(c, S2, 0x4cc);
        c.g[A2][0] = 0x46_0000 + 0x7ad8;
        c.g[S3][0] = c.g[A0][0];
        c.g[A3][0] = 0;
        c.g[A1][0] = u64::MAX - 1;
        call_ptr(c, 0x002c_cf98, 0x001d_afa8)?;
        c.g[S0][0] = init_pages_reg(c, S2, 0x16c);
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = init_pages_reg(c, S2, 0x138);
        c.g[A2][0] = u64::MAX;
        c.g[S1][0] = 0x17;
        call_ptr(c, 0x002c_bcc8, 0x001d_afc0)?;
        c.g[A0][0] = c.g[S2][0];
        c.g[A1][0] = init_pages_reg(c, S2, 0x154);
        c.g[A2][0] = u64::MAX;
        call_ptr(c, 0x002c_bcc8, 0x001d_afd0)?;
        c.g[A1][0] = c.g[S0][0];
        loop {
            c.cycles += 7;
            c.g[A0][0] = c.g[S2][0];
            c.g[A2][0] = u64::MAX;
            c.g[S0][0] = init_pages_reg(c, S0, 0x24);
            call_ptr(c, 0x002c_bcc8, 0x001d_afe8)?;
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_sub(1));
            let more = (c.g[S1][0] as i64) >= 0;
            c.g[A1][0] = c.g[S0][0];
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x001d_afd8);
                }
                continue;
            }
            break;
        }
        c.cycles += 13;
        c.g[A1][0] = c.g[S3][0];
        c.g[A0][0] = c.g[S2][0];
        c.g[A2][0] = u64::MAX;
        call_ptr(c, 0x002c_bcc8, 0x001d_b004)?;
        c.g[V0][0] = c.g[S2][0];
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void LobbyMenu_InitChatPage(LobbyMenu* menu, void* state)` (0x1db028):
/// lay out the chat page. Fills the 14 chat-line slots from the table at
/// `gp+0x7d70` and the two label cells.
pub fn lobby_menu_init_chat_page(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 32;
        let frame = Frame::open(c, 0x60);
        frame.save(c, S1, 0x40);
        c.g[S1][0] = c.g[A0][0];
        frame.save(c, S0, 0x50);
        frame.save(c, S2, 0x30);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S3, 0x20);
        frame.save(c, S4, 0x10);
        c.g[S2][0] = init_pages_reg(c, S1, 0x194);
        frame.save_ra(c, 0);
        c.g[S3][0] = 0xd;
        call_ptr(c, 0x002c_aa58, 0x001d_b058)?;
        c.g[S4][0] = u64::MAX;
        // Globals: 0x470000 - 0x2bd8 = D_0046D428, 0x460000 + 0x7ae0 = D_00467AE0.
        c.g[V0][0] = 0x47_0000 - 0x2bd8;
        c.g[A1][0] = 0x46_0000 + 0x7ae0;
        let label = c.g[V0][0];
        Addr(init_pages_at(c, S1, 0x12c)).sw(c, label);
        c.g[A0][0] = init_pages_reg(c, S1, 0x134);
        let text = Addr(c.g[S0][0] as u32).lw(c);
        c.g[V0][0] = text;
        Addr(init_pages_at(c, S1, 0x130)).sw(c, text);
        call_ptr(c, 0x002c_d2a0, 0x001d_b080)?;
        c.g[A1][0] = 0xe;
        c.g[A0][0] = init_pages_reg(c, S1, 0x150);
        call_ptr(c, 0x002c_d350, 0x001d_b08c)?;
        c.g[A1][0] = 0x46_0000 + 0x7af0;
        c.g[A2][0] = 0x4a_0000 + 0x1408;
        c.g[A0][0] = init_pages_reg(c, S1, 0x168);
        call_ptr(c, 0x002c_e418, 0x001d_b0a4)?;
        loop {
            c.cycles += 5;
            c.g[A0][0] = c.g[S2][0];
            c.g[S3][0] = sx((c.g[S3][0] as u32).wrapping_sub(1));
            call_ptr(c, 0x002c_cf08, 0x001d_b0b4)?;
            let more = c.g[S3][0] != c.g[S4][0];
            c.g[S2][0] = init_pages_reg(c, S2, 0x24);
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x001d_b0a8);
                }
                continue;
            }
            break;
        }
        c.cycles += 13;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = init_pages_reg(c, S1, 0x134);
        c.g[A2][0] = u64::MAX;
        call_ptr(c, 0x002c_bcc8, 0x001d_b0d0)?;
        c.g[S2][0] = 0;
        c.g[S0][0] = init_pages_reg(c, S1, 0x194);
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = init_pages_reg(c, S1, 0x150);
        c.g[A2][0] = u64::MAX;
        call_ptr(c, 0x002c_bcc8, 0x001d_b0e4)?;
        c.g[V0][0] = 0x46_0000;
        c.g[S3][0] = init_pages_reg(c, V0, 0x7d70);
        loop {
            c.cycles += 12;
            let index = c.g[S2][0];
            Addr(init_pages_at(c, S0, 0x8)).sw(c, index);
            c.g[A1][0] = c.g[S0][0];
            c.g[A0][0] = c.g[S1][0];
            c.g[A2][0] = u64::MAX;
            let row = Addr(c.g[S3][0] as u32).lw(c);
            c.g[V0][0] = row;
            c.g[S2][0] = init_pages_reg(c, S2, 0x1);
            c.g[S3][0] = init_pages_reg(c, S3, 0x4);
            Addr(init_pages_at(c, S0, 0x14)).sw(c, row);
            call_ptr(c, 0x002c_bcc8, 0x001d_b114)?;
            c.g[V0][0] = ((c.g[S2][0] as i64) < 14) as u64;
            let more = c.g[V0][0] != 0;
            c.g[S0][0] = init_pages_reg(c, S0, 0x24);
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x001d_b0f0);
                }
                continue;
            }
            break;
        }
        c.cycles += 9;
        c.g[V0][0] = c.g[S1][0];
        frame.restore(c, S0, 0x50);
        frame.restore(c, S1, 0x40);
        frame.restore(c, S2, 0x30);
        frame.restore(c, S3, 0x20);
        frame.restore(c, S4, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, fakes, DATA, GAME_GP};

    /// The lobby menu object the `LobbyMenu_Init*Page` bodies fill.
    const OBJ: u32 = DATA + 0x9_0000;
    /// The world pointer at `gp - 0x848`; its `+0xc4` and `+0xc8` are text pointers.
    const WORLD: u32 = DATA + 0x9_8000;
    /// A zeroed text buffer (the world's text pointers point here).
    const TEXT: u32 = DATA + 0x9_8100;
    /// An argument word the bodies read through `a1`.
    const ARG: u32 = DATA + 0x9_8200;

    /// A scrambled machine with the allocator, the world and text fixture,
    /// and the four argument registers `args`.
    fn init_pages_setup(c: &mut Cpu, seed: u32, salt: u64, args: [u64; 4]) {
        fakes::machine_rt(c, seed, salt, 0);
        c.bus.write32(GAME_GP.wrapping_sub(0x848), WORLD);
        c.bus.write32(WORLD + 0xc4, TEXT);
        c.bus.write32(WORLD + 0xc8, TEXT + 0x40);
        c.bus.write32(ARG, seed);
        c.bus.write32(ARG + 4, seed.wrapping_mul(0x9e37));
        c.g[A0][0] = args[0];
        c.g[A1][0] = args[1];
        c.g[A2][0] = args[2];
        c.g[A3][0] = args[3];
    }

    /// `a2` as a pointer (even seeds) or a plain word (odd seeds).
    fn init_pages_arg2(seed: u32) -> u64 {
        if seed % 2 == 0 {
            (ARG + 0x10) as u64
        } else {
            seed as u64
        }
    }

    #[test]
    fn lobby_menu_init_login_page_matches() {
        check(
            ssx3_ee::s04::f_001da7e8__generated,
            lobby_menu_init_login_page,
            20,
            &|c, seed| {
                init_pages_setup(
                    c,
                    seed,
                    0x1da7e8,
                    [OBJ as u64, ARG as u64, init_pages_arg2(seed), 0],
                );
            },
        );
    }

    #[test]
    fn lobby_menu_init_login_method_page_matches() {
        check(
            ssx3_ee::s04::f_001da988__generated,
            lobby_menu_init_login_method_page,
            20,
            &|c, seed| {
                init_pages_setup(
                    c,
                    seed,
                    0x1da988,
                    [OBJ as u64, ARG as u64, init_pages_arg2(seed), 0],
                );
            },
        );
    }

    #[test]
    fn lobby_menu_init_create_persona_page_matches() {
        check(
            ssx3_ee::s04::f_001daac8__generated,
            lobby_menu_init_create_persona_page,
            20,
            &|c, seed| {
                init_pages_setup(c, seed, 0x1daac8, [OBJ as u64, ARG as u64, 0, 0]);
            },
        );
    }

    #[test]
    fn lobby_menu_init_delete_persona_page_matches() {
        check(
            ssx3_ee::s04::f_001dabd0__generated,
            lobby_menu_init_delete_persona_page,
            20,
            &|c, seed| {
                init_pages_setup(c, seed, 0x1dabd0, [OBJ as u64, ARG as u64, 0, 0]);
            },
        );
    }

    #[test]
    fn lobby_menu_init_select_persona_page_matches() {
        check(
            ssx3_ee::s04::f_001dacb8__generated,
            lobby_menu_init_select_persona_page,
            20,
            &|c, seed| {
                init_pages_setup(
                    c,
                    seed,
                    0x1dacb8,
                    [
                        OBJ as u64,
                        ARG as u64,
                        init_pages_arg2(seed),
                        (ARG + 0x20) as u64,
                    ],
                );
            },
        );
    }

    #[test]
    fn lobby_menu_init_select_room_page_matches() {
        check(
            ssx3_ee::s04::f_001dae20__generated,
            lobby_menu_init_select_room_page,
            20,
            &|c, seed| {
                init_pages_setup(c, seed, 0x1dae20, [OBJ as u64, ARG as u64, 0, 0]);
            },
        );
    }

    #[test]
    fn lobby_menu_init_room_occupants_page_matches() {
        check(
            ssx3_ee::s04::f_001daf08__generated,
            lobby_menu_init_room_occupants_page,
            20,
            &|c, seed| {
                init_pages_setup(
                    c,
                    seed,
                    0x1daf08,
                    [OBJ as u64, ARG as u64, init_pages_arg2(seed), 0],
                );
            },
        );
    }

    #[test]
    fn lobby_menu_init_chat_page_matches() {
        check(
            ssx3_ee::s04::f_001db028__generated,
            lobby_menu_init_chat_page,
            20,
            &|c, seed| {
                init_pages_setup(c, seed, 0x1db028, [OBJ as u64, ARG as u64, 0, 0]);
            },
        );
    }
}
