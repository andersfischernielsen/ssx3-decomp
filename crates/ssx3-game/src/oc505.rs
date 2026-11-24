//! Staging module oc505 (wave wave228): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc505`.

#![allow(unused_imports)]

use crate::call::{call_ptr, guest, sp, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use crate::ps2f::FpuF32;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x001d_b528,
        lobby_menu_fill_room_list,
        "LobbyMenu_FillRoomList",
    ),
    (
        0x001d_b620,
        lobby_menu_fill_occupant_list,
        "LobbyMenu_FillOccupantList",
    ),
    (
        0x001d_bab8,
        lobby_menu_on_abort_page_action,
        "LobbyMenu_OnAbortPageAction",
    ),
    (
        0x001d_bad8,
        lobby_menu_on_challenge_page_action,
        "LobbyMenu_OnChallengePageAction",
    ),
    (
        0x001d_bb20,
        lobby_menu_on_chat_page_action,
        "LobbyMenu_OnChatPageAction",
    ),
    (
        0x001d_bb48,
        lobby_menu_on_occupants_page_action,
        "LobbyMenu_OnOccupantsPageAction",
    ),
    (
        0x001d_bb90,
        lobby_menu_on_select_room_page_action,
        "LobbyMenu_OnSelectRoomPageAction",
    ),
    (
        0x001d_bbc0,
        lobby_menu_on_select_persona_page_action,
        "LobbyMenu_OnSelectPersonaPageAction",
    ),
    (
        0x001d_bbe8,
        lobby_menu_on_create_persona_page_action,
        "LobbyMenu_OnCreatePersonaPageAction",
    ),
    (
        0x001d_bc10,
        lobby_menu_on_delete_persona_page_action,
        "LobbyMenu_OnDeletePersonaPageAction",
    ),
    (
        0x001d_bc38,
        lobby_menu_on_create_account_page_action,
        "LobbyMenu_OnCreateAccountPageAction",
    ),
    (
        0x001d_bc68,
        lobby_menu_on_login_page_action,
        "LobbyMenu_OnLoginPageAction",
    ),
    (0x001d_c900, module_base_dtor, "ModuleBase_dtor"),
    (0x001d_c940, lobby_menu_base_dtor, "LobbyMenuBase_dtor"),
    (0x001d_c990, font_draw_text_varargs, "Font_DrawTextVarargs"),
    (
        0x001d_c9d0,
        font_draw_text_scaled_varargs,
        "Font_DrawTextScaledVarargs",
    ),
    (
        0x001d_ca40,
        font_draw_text_centered_varargs,
        "Font_DrawTextCenteredVarargs",
    ),
    (
        0x001d_ca80,
        font_draw_text_right_varargs,
        "Font_DrawTextRightVarargs",
    ),
    (0x001d_cac0, fe_module_camera_dtor, "FEModuleCamera_dtor"),
    (
        0x001d_caf0,
        c_private_message_dtor_2,
        "cPrivateMessage_dtor_2",
    ),
];

/// `cMenuItem_activate(item, on)`: the row's activation (0x2ca388).
const CMENU_ITEM_ACTIVATE: u32 = 0x002c_a388;
/// The `delete` of a gcc dtor (0x317e50).
const DELETE_THIS: u32 = 0x0031_7e50;

/// The row array of one of the two lobby lists (rooms, occupants). Both
/// fillers have the same shape; only the callees, the return addresses and
/// the sizes differ.
struct ItemList {
    /// Offset of row 0 from the menu (rows are 0x24 apart).
    items: u32,
    /// Rows the menu holds.
    rows: u32,
    /// Count callee and its return address.
    get_count: (u32, u32),
    /// Name callee and its return address.
    get_name: (u32, u32),
    /// Third argument of the name callee (occupants pass 0).
    name_a2: Option<u64>,
    /// Return address after activating a filled row.
    act_ret: u32,
    /// Return address after activating row 0 for the empty placeholder.
    empty_ret: u32,
    /// Return address after deactivating the rows past the list.
    clear_ret: u32,
    /// Event heads of the fill loop and of the clear loop.
    head_fill: u32,
    head_clear: u32,
    /// Low half of the placeholder string address (`0x460000 + x`).
    empty_text_lo: u32,
    /// Cycles of one fill iteration and of the clear setup.
    fill_cycles: u64,
    mult_cycles: u64,
}

const ROOM_LIST: ItemList = ItemList {
    items: 0x168,
    rows: 12,
    get_count: (0x0025_f120, 0x001d_b558),
    get_name: (0x0025_f150, 0x001d_b574),
    name_a2: None,
    act_ret: 0x001d_b584,
    empty_ret: 0x001d_b5bc,
    clear_ret: 0x001d_b5ec,
    head_fill: 0x001d_b568,
    head_clear: 0x001d_b5e0,
    empty_text_lo: 0x7de8,
    fill_cycles: 12,
    mult_cycles: 4,
};

const OCCUPANT_LIST: ItemList = ItemList {
    items: 0x16c,
    rows: 24,
    get_count: (0x0025_f1b0, 0x001d_b650),
    get_name: (0x0025_f1e0, 0x001d_b670),
    name_a2: Some(0),
    act_ret: 0x001d_b680,
    empty_ret: 0x001d_b6b8,
    clear_ret: 0x001d_b6e4,
    head_fill: 0x001d_b660,
    head_clear: 0x001d_b6d8,
    empty_text_lo: 0x7e00,
    fill_cycles: 13,
    mult_cycles: 3,
};

/// Fills a lobby list from `self` (S4) and `src` (S3): one row per entry of
/// the count callee (name and activation), row 0 shows the placeholder when
/// the list is empty (item ID -1), and the rows past the list are deactivated.
fn fill_list(c: &mut Cpu, l: &ItemList) -> Flow {
    c.cycles += 15;
    let frame = Frame::open(c, 0x60);
    frame.save(c, 19, 0x20);
    frame.save(c, 20, 0x10);
    c.g[S3][0] = c.g[A1][0];
    c.g[S4][0] = c.g[A0][0];
    frame.save(c, 17, 0x40);
    frame.save(c, 18, 0x30);
    c.g[A0][0] = c.g[S3][0];
    frame.save(c, 16, 0x50);
    frame.save_ra(c, 0);
    c.g[S1][0] = 0;
    call_ptr(c, l.get_count.0, l.get_count.1)?;
    c.g[S2][0] = c.g[V0][0];
    if (c.g[S2][0] as i64) > 0 {
        c.cycles += 1;
        c.g[S0][0] = sx((c.g[S4][0] as u32).wrapping_add(l.items));
        loop {
            c.cycles += l.fill_cycles;
            c.g[A1][0] = c.g[S1][0];
            c.g[A0][0] = c.g[S3][0];
            if let Some(a2) = l.name_a2 {
                c.g[A2][0] = a2;
            }
            call_ptr(c, l.get_name.0, l.get_name.1)?;
            c.sw((c.g[S0][0] as u32).wrapping_add(0x14), c.g[V0][0]);
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 1;
            call_ptr(c, CMENU_ITEM_ACTIVATE, l.act_ret)?;
            c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[S1][0]);
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
            c.g[V0][0] = ((c.g[S1][0] as i64) < (c.g[S2][0] as i64)) as u64;
            let more = c.g[V0][0] != 0;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x24));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, l.head_fill);
            }
        }
    }
    // Empty list: row 0 shows the placeholder, ID -1, and counts as one row.
    c.cycles += 2;
    if c.g[S1][0] == 0 {
        c.cycles += 10;
        c.g[V0][0] = 0x0046_0000;
        c.g[A0][0] = sx((c.g[S4][0] as u32).wrapping_add(l.items));
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(l.empty_text_lo));
        c.g[A1][0] = 1;
        c.sw((c.g[S4][0] as u32).wrapping_add(l.items + 0x14), c.g[V0][0]);
        c.g[S1][0] = 1;
        call_ptr(c, CMENU_ITEM_ACTIVATE, l.empty_ret)?;
        c.g[V0][0] = u64::MAX;
        c.sw((c.g[S4][0] as u32).wrapping_add(l.items + 0x8), c.g[V0][0]);
    }
    c.g[V0][0] = ((c.g[S1][0] as i64) < l.rows as i64) as u64;
    // Deactivate the rows past the list (v0 keeps 0x24 when there are none).
    c.cycles += 2;
    let none_left = c.g[V0][0] == 0;
    c.g[V0][0] = 0x24;
    if !none_left {
        c.cycles += l.mult_cycles;
        c.mult(0, 2, 17, 2); // v0 = s1 * v0: the row offset
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(l.items));
        c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S4][0] as u32));
        loop {
            c.cycles += 7;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = 0;
            call_ptr(c, CMENU_ITEM_ACTIVATE, l.clear_ret)?;
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(1));
            c.g[V0][0] = ((c.g[S1][0] as i64) < l.rows as i64) as u64;
            let more = c.g[V0][0] != 0;
            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x24));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, l.head_clear);
            }
        }
    }
    c.cycles += 8;
    frame.restore(c, 16, 0x50);
    frame.restore(c, 17, 0x40);
    frame.restore(c, 18, 0x30);
    frame.restore(c, 19, 0x20);
    frame.restore(c, 20, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `LobbyMenu_FillRoomList` (0x1db528), the generated `LobbyMenu_updaterooms`:
/// the room rows (12) from `LobbyMan_getnumrooms` and `LobbyMan_GetRoomName`.
pub fn lobby_menu_fill_room_list(c: &mut Cpu) {
    guest(c, |c| fill_list(c, &ROOM_LIST));
}

/// `LobbyMenu_FillOccupantList` (0x1db620): the occupant rows (24) of the room
/// from `func_0025F1B0` (count) and `func_0025F1E0` (name, third argument 0).
pub fn lobby_menu_fill_occupant_list(c: &mut Cpu) {
    guest(c, |c| fill_list(c, &OCCUPANT_LIST));
}

/// `LobbyMenu_OnAbortPageAction` (0x1dbab8): forwards to `0x17e558` with the
/// menu's screen at +0x130; returns 0.
pub fn lobby_menu_on_abort_page_action(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x130));
        call_ptr(c, 0x0017_e558, 0x001d_bac8)?;
        frame.restore_ra(c, 0);
        c.g[V0][0] = 0;
        frame.close(c);
        Ok(())
    });
}

/// `LobbyMenu_OnChallengePageAction` (0x1dbad8): the message's field +8 picks
/// the screen action: 0 runs `0x17e490` (menu at +0x130), 1 runs `0x17e4f0`
/// (menu at +0x134), anything else does nothing. Returns 0.
pub fn lobby_menu_on_challenge_page_action(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8));
        c.g[V0][0] = 1;
        if c.g[V1][0] == 0 {
            c.cycles += 4;
            c.g[A0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x130));
            call_ptr(c, 0x0017_e490, 0x001d_baf4)?;
            frame.restore_ra(c, 0);
        } else {
            c.cycles += 2;
            frame.restore_ra(c, 0);
            if c.g[V1][0] == c.g[V0][0] {
                c.cycles += 3;
                c.g[A0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x134));
                call_ptr(c, 0x0017_e4f0, 0x001d_bb0c)?;
                frame.restore_ra(c, 0);
            }
        }
        c.cycles += 3;
        c.g[V0][0] = 0;
        frame.close(c);
        Ok(())
    });
}

/// `LobbyMenu_OnChatPageAction` (0x1dbb20): `0x17e418(menu +0x130, msg +0x14)`.
pub fn lobby_menu_on_chat_page_action(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x130));
        c.g[A1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x14));
        call_ptr(c, 0x0017_e418, 0x001d_bb34)?;
        frame.restore_ra(c, 0);
        c.g[V0][0] = 0;
        frame.close(c);
        Ok(())
    });
}

/// `LobbyMenu_OnOccupantsPageAction` (0x1dbb48): the message's field +8 is
/// -2 (`0x17e3b8` with the menu at +0x134), -1 (nothing) or anything else
/// (`0x17e368` with the menu at +0x130). Returns 0.
pub fn lobby_menu_on_occupants_page_action(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = u64::MAX - 1;
        frame.save_ra(c, 0);
        c.g[A1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8));
        let not_minus_two = c.g[A1][0] != c.g[V0][0];
        c.g[V0][0] = u64::MAX;
        if !not_minus_two {
            c.cycles += 4;
            c.g[A0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x134));
            call_ptr(c, 0x0017_e3b8, 0x001d_bb68)?;
            frame.restore_ra(c, 0);
        } else {
            c.cycles += 2;
            frame.restore_ra(c, 0);
            if c.g[A1][0] != c.g[V0][0] {
                c.cycles += 3;
                c.g[A0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x130));
                call_ptr(c, 0x0017_e368, 0x001d_bb80)?;
                frame.restore_ra(c, 0);
            }
        }
        c.cycles += 3;
        c.g[V0][0] = 0;
        frame.close(c);
        Ok(())
    });
}

/// `LobbyMenu_OnSelectRoomPageAction` (0x1dbb90): a message whose field +8 is
/// negative does nothing; otherwise `0x17e2f8(menu +0x130, ...)`. Returns 0.
pub fn lobby_menu_on_select_room_page_action(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8));
        let negative = (c.g[A1][0] as i64) < 0;
        frame.restore_ra(c, 0);
        if !negative {
            c.cycles += 3;
            c.g[A0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x130));
            call_ptr(c, 0x0017_e2f8, 0x001d_bbac)?;
            frame.restore_ra(c, 0);
        }
        c.cycles += 3;
        c.g[V0][0] = 0;
        frame.close(c);
        Ok(())
    });
}

/// `LobbyMenu_OnSelectPersonaPageAction` (0x1dbbc0):
/// `0x17e290(menu +0x4e0, msg +8)`. Returns 0.
pub fn lobby_menu_on_select_persona_page_action(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4e0));
        c.g[A1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8));
        call_ptr(c, 0x0017_e290, 0x001d_bbd4)?;
        frame.restore_ra(c, 0);
        c.g[V0][0] = 0;
        frame.close(c);
        Ok(())
    });
}

/// `LobbyMenu_OnCreatePersonaPageAction` (0x1dbbe8):
/// `0x17e1c0(menu +0x130, menu +0x180)`. Returns 0.
pub fn lobby_menu_on_create_persona_page_action(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        let menu = c.g[A0][0] as u32;
        c.g[A1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = c.lw(menu.wrapping_add(0x130));
        c.g[A1][0] = sx(menu.wrapping_add(0x180));
        call_ptr(c, 0x0017_e1c0, 0x001d_bc00)?;
        frame.restore_ra(c, 0);
        c.g[V0][0] = 0;
        frame.close(c);
        Ok(())
    });
}

/// `LobbyMenu_OnDeletePersonaPageAction` (0x1dbc10):
/// `0x17e228(menu +0x130, msg +8)`. Returns 0.
pub fn lobby_menu_on_delete_persona_page_action(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[A0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x130));
        c.g[A1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x8));
        call_ptr(c, 0x0017_e228, 0x001d_bc24)?;
        frame.restore_ra(c, 0);
        c.g[V0][0] = 0;
        frame.close(c);
        Ok(())
    });
}

/// The create-account and login page actions: `callee(menu +0x130,
/// menu +0x180, menu +0x1ac)`. Returns 0.
fn persona_form_action(c: &mut Cpu, callee: u32, ret: u32) -> Flow {
    c.cycles += 11;
    let frame = Frame::open(c, 0x10);
    let menu = c.g[A0][0] as u32;
    c.g[A1][0] = c.g[A0][0];
    frame.save_ra(c, 0);
    c.g[A2][0] = sx(menu.wrapping_add(0x1ac));
    c.g[A0][0] = c.lw(menu.wrapping_add(0x130));
    c.g[A1][0] = sx(menu.wrapping_add(0x180));
    call_ptr(c, callee, ret)?;
    frame.restore_ra(c, 0);
    c.g[V0][0] = 0;
    frame.close(c);
    Ok(())
}

/// `LobbyMenu_OnCreateAccountPageAction` (0x1dbc38).
pub fn lobby_menu_on_create_account_page_action(c: &mut Cpu) {
    guest(c, |c| persona_form_action(c, 0x0017_e030, 0x001d_bc54));
}

/// `LobbyMenu_OnLoginPageAction` (0x1dbc68).
pub fn lobby_menu_on_login_page_action(c: &mut Cpu) {
    guest(c, |c| persona_form_action(c, 0x0017_e118, 0x001d_bc84));
}

/// The shape of a gcc `dtor(this, flags)` that stores its vtable at
/// `this + field` and deletes `this` (0x317e50) when `flags & 1`.
fn vtable_dtor(c: &mut Cpu, vtable: u32, field: u32, ret: u32) -> Flow {
    c.cycles += 7;
    let frame = Frame::open(c, 0x10);
    c.g[V0][0] = sx(vtable);
    frame.save_ra(c, 0);
    c.g[A1][0] &= 1;
    let delete = c.g[A1][0] != 0;
    let this = c.g[A0][0] as u32;
    c.sw(this.wrapping_add(field), c.g[V0][0]);
    if delete {
        c.cycles += 2;
        call_ptr(c, DELETE_THIS, ret)?;
    }
    c.cycles += 3;
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `ModuleBase_dtor` (0x1dc900).
pub fn module_base_dtor(c: &mut Cpu) {
    guest(c, |c| vtable_dtor(c, 0x0046_d970, 0, 0x001d_c924));
}

/// `LobbyMenuBase_dtor` (0x1dc940).
pub fn lobby_menu_base_dtor(c: &mut Cpu) {
    guest(c, |c| vtable_dtor(c, 0x0046_d948, 0, 0x001d_c964));
}

/// `FEModuleCamera_dtor` (0x1dcac0): the vtable is stored at +0x90.
pub fn fe_module_camera_dtor(c: &mut Cpu) {
    guest(c, |c| vtable_dtor(c, 0x0045_b8b0, 0x90, 0x001d_cae4));
}

/// `cPrivateMessage_dtor_2` (0x1dcaf0): stores the vtable at +0xc, the
/// three members at +8, +4 and +0 are destroyed (0x318260 with 2), and the
/// object is deleted (0x317e50) when `flags & 1`.
pub fn c_private_message_dtor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 21;
        let frame = Frame::open(c, 0x30);
        c.g[V0][0] = sx(0x0047_0000u32.wrapping_sub(0x2740));
        frame.save(c, 16, 0x20);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, 17, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        let this = c.g[S1][0] as u32;
        c.sw(this.wrapping_add(0xc), c.g[V0][0]);
        c.g[A0][0] = sx(this.wrapping_add(0x8));
        c.g[A1][0] = 2;
        call_ptr(c, 0x0031_8260, 0x001d_cb20)?;
        c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(0x4));
        c.g[A1][0] = 2;
        call_ptr(c, 0x0031_8260, 0x001d_cb2c)?;
        c.g[A0][0] = c.g[S1][0];
        c.g[A1][0] = 2;
        call_ptr(c, 0x0031_8260, 0x001d_cb38)?;
        c.g[S0][0] &= 1;
        let delete = c.g[S0][0] != 0;
        frame.restore(c, 16, 0x20);
        if delete {
            c.cycles += 3;
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, DELETE_THIS, 0x001d_cb4c)?;
            frame.restore(c, 16, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// The gcc varargs forwarder of the font draw calls: spills `a2..t3` and
/// `f14, f16, f18` to the caller's varargs area (the format string now at
/// `sp + 0x40`) and tail-calls `draw`, returning to `ret`.
fn draw_varargs(c: &mut Cpu, draw: u32, ret: u32) -> Flow {
    c.cycles += 16;
    let frame = Frame::open(c, 0x70);
    let s = sp(c);
    let a2 = c.g[A2][0];
    c.sd(s.wrapping_add(0x40), a2);
    frame.save_ra(c, 0);
    c.g[A2][0] = sx(s.wrapping_add(0x40));
    let spill = [c.g[A3][0], c.g[T0][0], c.g[T1][0], c.g[T2][0], c.g[T3][0]];
    for (i, v) in spill.into_iter().enumerate() {
        c.sd(s.wrapping_add(0x48 + 8 * i as u32), v);
    }
    c.swc1(14, s.wrapping_add(0x34));
    c.swc1(16, s.wrapping_add(0x38));
    c.swc1(18, s.wrapping_add(0x3c));
    call_ptr(c, draw, ret)?;
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `Font_DrawTextVarargs` (0x1dc990): forwards to `0x391c48`.
pub fn font_draw_text_varargs(c: &mut Cpu) {
    guest(c, |c| draw_varargs(c, 0x0039_1c48, 0x001d_c9c4));
}

/// `Font_DrawTextCenteredVarargs` (0x1dca40): forwards to `0x3929f8`.
pub fn font_draw_text_centered_varargs(c: &mut Cpu) {
    guest(c, |c| draw_varargs(c, 0x0039_29f8, 0x001d_ca74));
}

/// `Font_DrawTextRightVarargs` (0x1dca80): forwards to `0x392b40`.
pub fn font_draw_text_right_varargs(c: &mut Cpu) {
    guest(c, |c| draw_varargs(c, 0x0039_2b40, 0x001d_cab4));
}

/// `Font_DrawTextScaledVarargs` (0x1dc9d0): the object at `a0` holds the
/// scale (f0 = +0x34 times f15, f1 = +0x30 times f14); both are stored back
/// at +0x38/+0x3c, `0x391c48` draws, and the 8 bytes at +0x30 are copied to
/// +0x38 through v0.
pub fn font_draw_text_scaled_varargs(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 28;
        let frame = Frame::open(c, 0x70);
        frame.save(c, 16, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        let s = sp(c);
        let a2 = c.g[A2][0];
        c.sd(s.wrapping_add(0x40), a2);
        c.g[A2][0] = sx(s.wrapping_add(0x40));
        let obj = c.g[S0][0] as u32;
        c.lwc1(1, obj.wrapping_add(0x30));
        c.lwc1(0, obj.wrapping_add(0x34));
        c.f[1] = (FpuF32(c.f[1]) * FpuF32(c.f[14])).0;
        let a3 = c.g[A3][0];
        c.sd(s.wrapping_add(0x48), a3);
        c.f[0] = (FpuF32(c.f[0]) * FpuF32(c.f[15])).0;
        let spill = [c.g[T0][0], c.g[T1][0], c.g[T2][0], c.g[T3][0]];
        for (i, v) in spill.into_iter().enumerate() {
            c.sd(s.wrapping_add(0x50 + 8 * i as u32), v);
        }
        c.swc1(16, s.wrapping_add(0x38));
        c.swc1(18, s.wrapping_add(0x3c));
        c.swc1(1, obj.wrapping_add(0x38));
        c.swc1(0, obj.wrapping_add(0x3c));
        call_ptr(c, 0x0039_1c48, 0x001d_ca20)?;
        let obj = c.g[S0][0] as u32;
        c.ldl(2, obj.wrapping_add(0x37));
        c.ldr(2, obj.wrapping_add(0x30));
        c.sdl(2, obj.wrapping_add(0x3f));
        c.sdr(2, obj.wrapping_add(0x38));
        frame.restore(c, 16, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes;
    use crate::difftest::fakes::covered;
    use crate::difftest::fakes::machine;
    use crate::difftest::Rng;
    use crate::difftest::DATA;
    use std::ptr::addr_of_mut;

    /// Gate of the callee fakes below: they log the call and run the real
    /// body only when this magic word is not in the machine's RAM.
    pub(crate) const OC505_GATE: u32 = DATA + 0xf_e000;
    pub(crate) const OC505_MAGIC: u32 = 0x0505_6a7e;

    /// A callee fake that logs the call (and unwinds when the machine says
    /// so); the gate off runs the callee body it replaced.
    macro_rules! gated {
        ($fake:ident, $gen:path, $addr:expr) => {
            pub(crate) fn $fake(c: &mut Cpu) {
                if c.bus.read32(OC505_GATE) != OC505_MAGIC {
                    return fakes::fall_back(c, fakes::prev_cell($fake), $gen);
                }
                let ra = c.g[RA][0] as u32;
                fakes::fake(c, $addr, ra);
            }
        };
    }

    /// Installs a fake in a callee's override slot, once per process.
    macro_rules! install {
        ($slot:path, $fake:ident) => {
            fakes::hook(addr_of_mut!($slot), $fake, fakes::prev_cell($fake));
        };
    }

    gated!(
        oc505_fake_0025f150,
        ee::s06::f_0025f150__generated,
        0x0025_f150
    );
    gated!(
        oc505_fake_0025f1e0,
        ee::s06::f_0025f1e0__generated,
        0x0025_f1e0
    );
    gated!(
        oc505_fake_002ca388,
        ee::s09::f_002ca388__generated,
        0x002c_a388
    );
    gated!(
        oc505_fake_0017e558,
        ee::s02::f_0017e558__generated,
        0x0017_e558
    );
    gated!(
        oc505_fake_0017e490,
        ee::s02::f_0017e490__generated,
        0x0017_e490
    );
    gated!(
        oc505_fake_0017e4f0,
        ee::s02::f_0017e4f0__generated,
        0x0017_e4f0
    );
    gated!(
        oc505_fake_0017e418,
        ee::s02::f_0017e418__generated,
        0x0017_e418
    );
    gated!(
        oc505_fake_0017e3b8,
        ee::s02::f_0017e3b8__generated,
        0x0017_e3b8
    );
    gated!(
        oc505_fake_0017e368,
        ee::s02::f_0017e368__generated,
        0x0017_e368
    );
    gated!(
        oc505_fake_0017e2f8,
        ee::s02::f_0017e2f8__generated,
        0x0017_e2f8
    );
    gated!(
        oc505_fake_0017e290,
        ee::s02::f_0017e290__generated,
        0x0017_e290
    );
    gated!(
        oc505_fake_0017e228,
        ee::s02::f_0017e228__generated,
        0x0017_e228
    );
    gated!(
        oc505_fake_0017e1c0,
        ee::s02::f_0017e1c0__generated,
        0x0017_e1c0
    );
    gated!(
        oc505_fake_0017e030,
        ee::s02::f_0017e030__generated,
        0x0017_e030
    );
    gated!(
        oc505_fake_0017e118,
        ee::s02::f_0017e118__generated,
        0x0017_e118
    );
    gated!(
        oc505_fake_00317e50,
        ee::s10::f_00317e50__generated,
        0x0031_7e50
    );
    gated!(
        oc505_fake_00318260,
        ee::s10::f_00318260__generated,
        0x0031_8260
    );
    gated!(
        oc505_fake_00391c48,
        ee::s12::f_00391c48__generated,
        0x0039_1c48
    );
    gated!(
        oc505_fake_003929f8,
        ee::s13::f_003929f8__generated,
        0x0039_29f8
    );
    gated!(
        oc505_fake_00392b40,
        ee::s13::f_00392b40__generated,
        0x0039_2b40
    );

    /// The count callees return the test's row count (the fake logs the call
    /// as usual; the result is the count the seed picked, -1 included).
    pub(crate) fn oc505_fake_count_room(c: &mut Cpu) {
        if c.bus.read32(OC505_GATE) != OC505_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc505_fake_count_room),
                ee::s06::f_0025f120__generated,
            );
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0x0025_f120, ra);
        c.g[V0][0] = sx(c.bus.read32(OC505_GATE + 4));
    }

    /// The occupant count callee (see [`oc505_fake_count_room`]).
    pub(crate) fn oc505_fake_count_occupants(c: &mut Cpu) {
        if c.bus.read32(OC505_GATE) != OC505_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc505_fake_count_occupants),
                ee::s06::f_0025f1b0__generated,
            );
        }
        let ra = c.g[RA][0] as u32;
        fakes::fake(c, 0x0025_f1b0, ra);
        c.g[V0][0] = sx(c.bus.read32(OC505_GATE + 4));
    }

    /// Installs every fake in its callee's override slot.
    pub(crate) fn oc505_install_fakes() {
        install!(ee::s06::OVERRIDE_0025F120, oc505_fake_count_room);
        install!(ee::s06::OVERRIDE_0025F1B0, oc505_fake_count_occupants);
        install!(ee::s06::OVERRIDE_0025F150, oc505_fake_0025f150);
        install!(ee::s06::OVERRIDE_0025F1E0, oc505_fake_0025f1e0);
        install!(ee::s09::OVERRIDE_002CA388, oc505_fake_002ca388);
        install!(ee::s02::OVERRIDE_0017E558, oc505_fake_0017e558);
        install!(ee::s02::OVERRIDE_0017E490, oc505_fake_0017e490);
        install!(ee::s02::OVERRIDE_0017E4F0, oc505_fake_0017e4f0);
        install!(ee::s02::OVERRIDE_0017E418, oc505_fake_0017e418);
        install!(ee::s02::OVERRIDE_0017E3B8, oc505_fake_0017e3b8);
        install!(ee::s02::OVERRIDE_0017E368, oc505_fake_0017e368);
        install!(ee::s02::OVERRIDE_0017E2F8, oc505_fake_0017e2f8);
        install!(ee::s02::OVERRIDE_0017E290, oc505_fake_0017e290);
        install!(ee::s02::OVERRIDE_0017E228, oc505_fake_0017e228);
        install!(ee::s02::OVERRIDE_0017E1C0, oc505_fake_0017e1c0);
        install!(ee::s02::OVERRIDE_0017E030, oc505_fake_0017e030);
        install!(ee::s02::OVERRIDE_0017E118, oc505_fake_0017e118);
        install!(ee::s10::OVERRIDE_00317E50, oc505_fake_00317e50);
        install!(ee::s10::OVERRIDE_00318260, oc505_fake_00318260);
        install!(ee::s12::OVERRIDE_00391C48, oc505_fake_00391c48);
        install!(ee::s13::OVERRIDE_003929F8, oc505_fake_003929f8);
        install!(ee::s13::OVERRIDE_00392B40, oc505_fake_00392b40);
    }

    /// A scrambled machine with the callee gate open and an unwind picked
    /// among `calls` fake calls.
    fn oc505_machine(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        let r = machine(c, seed, salt, calls);
        c.bus.write32(OC505_GATE, OC505_MAGIC);
        r
    }

    /// A row count in -1..m-1 (negative counts take the empty path).
    fn rows(seed: u32, m: u32) -> u32 {
        ((seed % m) as i32 - 1) as u32
    }

    /// A message whose +8 field is `v`; returns its address.
    fn msg(c: &mut Cpu, v: u32) -> u64 {
        let at = DATA + 0x300;
        c.bus.write32(at + 8, v);
        at as u64
    }

    #[test]
    fn lobby_menu_fill_room_list_matches() {
        oc505_install_fakes();
        covered(
            ee::s04::f_001db528__generated,
            lobby_menu_fill_room_list,
            80,
            &|c, seed| {
                let _ = oc505_machine(c, seed, 0x1db5_28, 8);
                c.pc = 0x001d_b528;
                c.g[A0][0] = (DATA + 0x200) as u64;
                c.g[A1][0] = (DATA + 0x6000) as u64;
                c.bus.write32(OC505_GATE + 4, rows(seed, 15));
            },
        );
    }

    #[test]
    fn lobby_menu_fill_occupant_list_matches() {
        oc505_install_fakes();
        covered(
            ee::s04::f_001db620__generated,
            lobby_menu_fill_occupant_list,
            80,
            &|c, seed| {
                let _ = oc505_machine(c, seed, 0x1db6_20, 8);
                c.pc = 0x001d_b620;
                c.g[A0][0] = (DATA + 0x200) as u64;
                c.g[A1][0] = (DATA + 0x6000) as u64;
                c.bus.write32(OC505_GATE + 4, rows(seed, 27));
            },
        );
    }

    #[test]
    fn lobby_menu_on_abort_page_action_matches() {
        oc505_install_fakes();
        covered(
            ee::s04::f_001dbab8__generated,
            lobby_menu_on_abort_page_action,
            60,
            &|c, seed| {
                let _ = oc505_machine(c, seed, 0x1dba_b8, 1);
                c.pc = 0x001d_bab8;
                c.g[A0][0] = (DATA + 0x100) as u64;
            },
        );
    }

    #[test]
    fn lobby_menu_on_challenge_page_action_matches() {
        oc505_install_fakes();
        covered(
            ee::s04::f_001dbad8__generated,
            lobby_menu_on_challenge_page_action,
            80,
            &|c, seed| {
                let _ = oc505_machine(c, seed, 0x1dba_d8, 1);
                c.pc = 0x001d_bad8;
                c.g[A0][0] = (DATA + 0x100) as u64;
                c.g[A1][0] = msg(c, [0u32, 1, 2, 9][(seed % 4) as usize]);
            },
        );
    }

    #[test]
    fn lobby_menu_on_chat_page_action_matches() {
        oc505_install_fakes();
        covered(
            ee::s04::f_001dbb20__generated,
            lobby_menu_on_chat_page_action,
            60,
            &|c, seed| {
                let _ = oc505_machine(c, seed, 0x1dbb_20, 1);
                c.pc = 0x001d_bb20;
                c.g[A0][0] = (DATA + 0x100) as u64;
                c.g[A1][0] = (DATA + 0x300) as u64;
            },
        );
    }

    #[test]
    fn lobby_menu_on_occupants_page_action_matches() {
        oc505_install_fakes();
        covered(
            ee::s04::f_001dbb48__generated,
            lobby_menu_on_occupants_page_action,
            80,
            &|c, seed| {
                let _ = oc505_machine(c, seed, 0x1dbb_48, 1);
                c.pc = 0x001d_bb48;
                c.g[A0][0] = (DATA + 0x100) as u64;
                c.g[A1][0] = msg(c, [0u32, u32::MAX - 1, u32::MAX, 3][(seed % 4) as usize]);
            },
        );
    }

    #[test]
    fn lobby_menu_on_select_room_page_action_matches() {
        oc505_install_fakes();
        covered(
            ee::s04::f_001dbb90__generated,
            lobby_menu_on_select_room_page_action,
            80,
            &|c, seed| {
                let _ = oc505_machine(c, seed, 0x1dbb_90, 1);
                c.pc = 0x001d_bb90;
                c.g[A0][0] = (DATA + 0x100) as u64;
                c.g[A1][0] = msg(c, [0u32, u32::MAX, 0x7fff_ffff, 5][(seed % 4) as usize]);
            },
        );
    }

    #[test]
    fn lobby_menu_on_select_persona_page_action_matches() {
        oc505_install_fakes();
        covered(
            ee::s04::f_001dbbc0__generated,
            lobby_menu_on_select_persona_page_action,
            60,
            &|c, seed| {
                let _ = oc505_machine(c, seed, 0x1dbb_c0, 1);
                c.pc = 0x001d_bbc0;
                c.g[A0][0] = (DATA + 0x100) as u64;
                c.g[A1][0] = (DATA + 0x300) as u64;
            },
        );
    }

    #[test]
    fn lobby_menu_on_create_persona_page_action_matches() {
        oc505_install_fakes();
        covered(
            ee::s04::f_001dbbe8__generated,
            lobby_menu_on_create_persona_page_action,
            60,
            &|c, seed| {
                let _ = oc505_machine(c, seed, 0x1dbb_e8, 1);
                c.pc = 0x001d_bbe8;
                c.g[A0][0] = (DATA + 0x100) as u64;
            },
        );
    }

    #[test]
    fn lobby_menu_on_delete_persona_page_action_matches() {
        oc505_install_fakes();
        covered(
            ee::s04::f_001dbc10__generated,
            lobby_menu_on_delete_persona_page_action,
            60,
            &|c, seed| {
                let _ = oc505_machine(c, seed, 0x1dbc_10, 1);
                c.pc = 0x001d_bc10;
                c.g[A0][0] = (DATA + 0x100) as u64;
                c.g[A1][0] = (DATA + 0x300) as u64;
            },
        );
    }

    #[test]
    fn lobby_menu_on_create_account_page_action_matches() {
        oc505_install_fakes();
        covered(
            ee::s04::f_001dbc38__generated,
            lobby_menu_on_create_account_page_action,
            60,
            &|c, seed| {
                let _ = oc505_machine(c, seed, 0x1dbc_38, 1);
                c.pc = 0x001d_bc38;
                c.g[A0][0] = (DATA + 0x100) as u64;
            },
        );
    }

    #[test]
    fn lobby_menu_on_login_page_action_matches() {
        oc505_install_fakes();
        covered(
            ee::s04::f_001dbc68__generated,
            lobby_menu_on_login_page_action,
            60,
            &|c, seed| {
                let _ = oc505_machine(c, seed, 0x1dbc_68, 1);
                c.pc = 0x001d_bc68;
                c.g[A0][0] = (DATA + 0x100) as u64;
            },
        );
    }

    #[test]
    fn module_base_dtor_matches() {
        oc505_install_fakes();
        covered(ee::s04::f_001dc900__generated, module_base_dtor, 60, &|c, seed| {
            let _ = oc505_machine(c, seed, 0x1dc9_00, 1);
            c.pc = 0x001d_c900;
            c.g[A0][0] = (DATA + 0x400) as u64;
            c.g[A1][0] = seed as u64;
        });
    }

    #[test]
    fn lobby_menu_base_dtor_matches() {
        oc505_install_fakes();
        covered(ee::s04::f_001dc940__generated, lobby_menu_base_dtor, 60, &|c, seed| {
            let _ = oc505_machine(c, seed, 0x1dc9_40, 1);
            c.pc = 0x001d_c940;
            c.g[A0][0] = (DATA + 0x400) as u64;
            c.g[A1][0] = seed as u64;
        });
    }

    #[test]
    fn fe_module_camera_dtor_matches() {
        oc505_install_fakes();
        covered(
            ee::s04::f_001dcac0__generated,
            fe_module_camera_dtor,
            60,
            &|c, seed| {
                let _ = oc505_machine(c, seed, 0x1dca_c0, 1);
                c.pc = 0x001d_cac0;
                c.g[A0][0] = (DATA + 0x400) as u64;
                c.g[A1][0] = seed as u64;
            },
        );
    }

    #[test]
    fn c_private_message_dtor_2_matches() {
        oc505_install_fakes();
        covered(
            ee::s04::f_001dcaf0__generated,
            c_private_message_dtor_2,
            80,
            &|c, seed| {
                let _ = oc505_machine(c, seed, 0x1dca_f0, 4);
                c.pc = 0x001d_caf0;
                c.g[A0][0] = (DATA + 0x500) as u64;
                c.g[A1][0] = seed as u64;
            },
        );
    }

    #[test]
    fn font_draw_text_varargs_matches() {
        oc505_install_fakes();
        covered(
            ee::s04::f_001dc990__generated,
            font_draw_text_varargs,
            60,
            &|c, seed| {
                let _ = oc505_machine(c, seed, 0x1dc9_90, 1);
                c.pc = 0x001d_c990;
            },
        );
    }

    #[test]
    fn font_draw_text_centered_varargs_matches() {
        oc505_install_fakes();
        covered(
            ee::s04::f_001dca40__generated,
            font_draw_text_centered_varargs,
            60,
            &|c, seed| {
                let _ = oc505_machine(c, seed, 0x1dca_40, 1);
                c.pc = 0x001d_ca40;
            },
        );
    }

    #[test]
    fn font_draw_text_right_varargs_matches() {
        oc505_install_fakes();
        covered(
            ee::s04::f_001dca80__generated,
            font_draw_text_right_varargs,
            60,
            &|c, seed| {
                let _ = oc505_machine(c, seed, 0x1dca_80, 1);
                c.pc = 0x001d_ca80;
            },
        );
    }

    #[test]
    fn font_draw_text_scaled_varargs_matches() {
        oc505_install_fakes();
        covered(
            ee::s04::f_001dc9d0__generated,
            font_draw_text_scaled_varargs,
            60,
            &|c, seed| {
                let mut r = oc505_machine(c, seed, 0x1dc9_d0, 1);
                c.pc = 0x001d_c9d0;
                c.g[A0][0] = (DATA + 0x100) as u64;
                c.bus.write32(DATA + 0x130, r.f32(100.0));
                c.bus.write32(DATA + 0x134, r.edge_f32());
            },
        );
    }
}
