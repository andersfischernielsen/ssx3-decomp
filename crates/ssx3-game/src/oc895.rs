//! Staging module oc895 (wave wave253): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc895`.

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
    (
        0x001b_e8b0,
        oc895_room_popup_widget_created,
        "cFEStateNetCreateRoomPopup_OnWidgetCreated",
    ),
    (
        0x001b_ed58,
        oc895_room_popup_v25,
        "cFEStateNetCreateRoomPopup_v25",
    ),
    (0x001b_ef40, oc895_login_update, "cFEStateLogin_Update"),
    (0x001b_f228, oc895_login_popup_v31, "cFEStateLogin_v31"),
    (
        0x001b_f438,
        oc895_account_popup_event,
        "cFEStateLogin_OnAccountMemPopupEvent",
    ),
];

/// `base + off` as a guest address (`addiu`), from a 64-bit register value.
#[inline(always)]
fn oc895_at(base: u64, off: u32) -> Addr {
    Addr((base as u32).wrapping_add(off))
}

/// `GetHashValue32` (0x317670) of the string at `0x460000 + off`, returning to
/// `ret` with the hash in v0. `off` may wrap (negative offsets).
fn oc895_hash(c: &mut Cpu, off: u32, ret: u32) -> Flow {
    c.g[A0][0] = sx(0x0046_0000u32.wrapping_add(off));
    call_ptr(c, 0x0031_7670, ret)
}

/// The epilogue of a frame that saved `ra` at 0 and closed with `sp += size`.
#[inline]
fn oc895_leave(c: &mut Cpu, frame: Frame) {
    frame.close(c);
}

// ---------------------------------------------------------------------------
// cFEStateNetCreateRoomPopup_OnWidgetCreated (0x1be8b0)
// ---------------------------------------------------------------------------

/// `cFEStateNetCreateRoomPopup_OnWidgetCreated(self, widget)` (0x1be8b0): when
/// the widget is one of the create-room popup's fields (by name hash) it fills
/// it: the channel list gets its four entries and the default channel name, and
/// the room-name, password and pair-tag widgets are stored on `self`.
pub fn oc895_room_popup_widget_created(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x40);
        frame.save(c, 18, 0x10);
        frame.save(c, 17, 0x20);
        c.g[S2][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, 16, 0x30);
        c.g[S0][0] = oc895_at(c.g[S1][0], 0x38).lw(c);
        oc895_hash(c, 0x51f8, 0x001b_e8dc)?;
        if c.g[S0][0] == c.g[V0][0] {
            // The channel list: four entries, then the default channel name.
            c.cycles += 33;
            oc895_hash(c, 0x3ac0, 0x001b_e8f0)?;
            oc895_list_add(c, 2, 0x001b_e900)?;
            oc895_hash(c, 0x3a90, 0x001b_e90c)?;
            oc895_list_add(c, 1, 0x001b_e91c)?;
            oc895_hash(c, 0x3a78, 0x001b_e928)?;
            oc895_list_add(c, 0, 0x001b_e938)?;
            oc895_hash(c, 0x3aa8, 0x001b_e944)?;
            oc895_list_add(c, 3, 0x001b_e954)?;
            c.g[A1][0] = oc895_at(c.g[GP][0], 0u32.wrapping_sub(0x1684)).lw(c);
            c.g[A0][0] = sx((c.g[S2][0] as u32).wrapping_add(0x6e0));
            call_ptr(c, 0x0031_84e8, 0x001b_e960)?;
            let v = c.g[S1][0];
            oc895_at(c.g[S2][0], 0x6d8).sw(c, v);
            oc895_text_end(c, frame);
        } else {
            c.g[A0][0] = 0x0046_0000;
            oc895_text_chain(c, frame)?;
        }
        c.cycles += 5;
        frame.restore(c, 17, 0x20);
        frame.restore(c, 18, 0x10);
        frame.restore_ra(c, 0);
        oc895_leave(c, frame);
        Ok(())
    });
}

/// One entry of the channel list: `cUIListBox_addEntryByStringID(text, hash, n)`.
fn oc895_list_add(c: &mut Cpu, n: u64, ret: u32) -> Flow {
    c.g[A1][0] = c.g[V0][0];
    c.g[A2][0] = n;
    c.g[A0][0] = c.g[S1][0];
    call_ptr(c, 0x0039_a568, ret)
}

/// The chain of name tests after the channel list: the room-name and password
/// fields (`0x5208`, `0x2ba0`), the three pair tags, and the done button.
fn oc895_text_chain(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 4;
    oc895_hash(c, 0x5208, 0x001b_e970)?;
    c.g[A0][0] = 0x0046_0000;
    if c.g[S0][0] == c.g[V0][0] {
        return oc895_clear_field(c, frame, 0x6d0, 0x001b_e988);
    }
    c.cycles += 4;
    oc895_hash(c, 0x2ba0, 0x001b_e998)?;
    c.g[A0][0] = 0x0046_0000;
    if c.g[S0][0] == c.g[V0][0] {
        return oc895_clear_field(c, frame, 0x6d4, 0x001b_e9b0);
    }
    // Pair-channel tag: mMiscData = 0.
    c.cycles += 4;
    oc895_hash(c, 0x5218, 0x001b_e9c0)?;
    if c.g[S0][0] == c.g[V0][0] {
        c.cycles += 2;
        oc895_at(c.g[S1][0], 0x18).sw(c, 0);
        oc895_text_end(c, frame);
        return Ok(());
    }
    c.g[A0][0] = 0x0046_0000;
    // Pair-room tag: mMiscData = 1.
    c.cycles += 4;
    oc895_hash(c, 0x5228, 0x001b_e9d8)?;
    if c.g[S0][0] == c.g[V0][0] {
        c.cycles += 2;
        c.g[V0][0] = 1;
        return oc895_store_tag(c, frame);
    }
    c.g[A0][0] = 0x0046_0000;
    // Pair-password tag: mMiscData = 2.
    c.cycles += 4;
    oc895_hash(c, 0x2b48, 0x001b_e9f0)?;
    if c.g[S0][0] == c.g[V0][0] {
        c.cycles += 2;
        c.g[V0][0] = 2;
        return oc895_store_tag(c, frame);
    }
    c.g[A0][0] = 0x0046_0000;
    // Done button: mMiscData = 3. The restore comes before the test, and a
    // mismatch leaves at once (no further cycles).
    c.cycles += 4;
    oc895_hash(c, 0x5238, 0x001b_ea08)?;
    let mismatch = c.g[S0][0] != c.g[V0][0];
    frame.restore(c, 16, 0x30);
    if mismatch {
        return Ok(());
    }
    c.cycles += 1;
    c.g[V0][0] = 3;
    oc895_store_tag(c, frame)
}

/// The room-name and password fields: `cUIText_setAsciiString(text, "")` (the
/// empty string at `0x4a1408`), then `self->field = text`.
fn oc895_clear_field(c: &mut Cpu, frame: Frame, field: u32, ret: u32) -> Flow {
    c.cycles += 6;
    c.g[A1][0] = sx(0x004a_0000u32.wrapping_add(0x1408));
    c.g[A0][0] = c.g[S1][0];
    call_ptr(c, 0x003a_0c50, ret)?;
    let v = c.g[S1][0];
    oc895_at(c.g[S2][0], field).sw(c, v);
    oc895_text_end(c, frame);
    Ok(())
}

/// `text->mMiscData = v0`, then the common exit.
fn oc895_store_tag(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 1;
    let v = c.g[V0][0];
    oc895_at(c.g[S1][0], 0x18).sw(c, v);
    oc895_text_end(c, frame);
    Ok(())
}

/// The common exit of a branch: one cycle, then the saved `s0` comes back.
fn oc895_text_end(c: &mut Cpu, frame: Frame) {
    c.cycles += 1;
    frame.restore(c, 16, 0x30);
}

// ---------------------------------------------------------------------------
// cFEStateNetCreateRoomPopup_v25 (0x1bed58)
// ---------------------------------------------------------------------------

/// `cFEStateNetCreateRoomPopup_v25(self, popup)` (0x1bed58): fills the help
/// popup of the create-room screen: the screen name, the description, and the
/// two highlight options (through the help popup's setters at 0x1da510).
pub fn oc895_room_popup_v25(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 54;
        let frame = Frame::open(c, 0x20);
        frame.save(c, 16, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        c.g[A1][0] = 4;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, 0x001d_a528, 0x001b_ed74)?;
        oc895_hash(c, 0x5278, 0x001b_ed80)?;
        let v = c.g[V0][0];
        oc895_at(c.g[S0][0], 0x54).sw(c, v);
        oc895_hash(c, 0x5290, 0x001b_ed90)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x001d_a530, 0x001b_eda0)?;
        oc895_hash(c, 0u32.wrapping_sub(0x1a0), 0x001b_edac)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        c.g[A3][0] = 3;
        call_ptr(c, 0x001d_a510, 0x001b_edc0)?;
        oc895_hash(c, 0u32.wrapping_sub(0x1c0), 0x001b_edcc)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = 1;
        c.g[A3][0] = 4;
        call_ptr(c, 0x001d_a510, 0x001b_ede0)?;
        oc895_hash(c, 0x2c50, 0x001b_edec)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = 2;
        c.g[A3][0] = 0;
        call_ptr(c, 0x001d_a510, 0x001b_ee00)?;
        oc895_hash(c, 0x14e8, 0x001b_ee0c)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = 3;
        c.g[A3][0] = 1;
        call_ptr(c, 0x001d_a510, 0x001b_ee20)?;
        frame.restore(c, 16, 0x10);
        frame.restore_ra(c, 0);
        oc895_leave(c, frame);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// cFEStateLogin_Update (0x1bef40)
// ---------------------------------------------------------------------------

/// `cFEStateLogin_Update(self)` (0x1bef40): the base `NetFEState_Update`.
pub fn oc895_login_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        call_ptr(c, 0x001a_8770, 0x001b_ef50)?;
        frame.restore_ra(c, 0);
        oc895_leave(c, frame);
        Ok(())
    });
}

// ---------------------------------------------------------------------------
// cFEStateLogin_v31 (0x1bf228)
// ---------------------------------------------------------------------------

/// `cFEStateLogin_v31(self, id, data)` (0x1bf228): the popup-event switch of the
/// login screen. 201 hands the account name and password to the lobby; 200
/// opens the verify popup; 212 creates the persona screen; anything else goes
/// to the base `NetFEState_HandleNetEvent`.
pub fn oc895_login_popup_v31(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = 0xc9;
        frame.save(c, 18, 0x10);
        frame.save(c, 16, 0x30);
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, 17, 0x20);
        let is_201 = c.g[A1][0] == c.g[V0][0];
        frame.save_ra(c, 0);
        oc895_v31_switch(c, frame, is_201)?;
        c.cycles += 5;
        frame.restore(c, 17, 0x20);
        frame.restore(c, 18, 0x10);
        frame.restore_ra(c, 0);
        oc895_leave(c, frame);
        Ok(())
    });
}

/// The switch of `cFEStateLogin_v31`, leaving the way the generated code does:
/// the cases 200, 201 and 212 leave through the one-cycle exit that restores
/// `s0`; the default leaves after its own restore.
fn oc895_v31_switch(c: &mut Cpu, frame: Frame, is_201: bool) -> Flow {
    if is_201 {
        oc895_v31_case_201(c)?;
        return oc895_v31_exit(c, frame);
    }
    c.cycles += 3;
    c.g[V0][0] = ((c.g[A1][0] as i64) < 202) as u64;
    let below = c.g[V0][0] != 0;
    c.g[V0][0] = 0xc8;
    if below {
        c.cycles += 2;
        if c.g[A1][0] == c.g[V0][0] {
            oc895_v31_case_200(c)?;
            return oc895_v31_exit(c, frame);
        }
        c.cycles += 2;
    } else {
        c.cycles += 3;
        c.g[V0][0] = 0xd4;
        if c.g[A1][0] == c.g[V0][0] {
            c.g[A1][0] = 0x0046_0000;
            oc895_v31_case_212(c)?;
            return oc895_v31_exit(c, frame);
        }
        c.cycles += 2;
    }
    // Default: the base NetFEState_HandleNetEvent, then the restore of s0.
    c.cycles += 4;
    c.g[RA][0] = sx(0x001b_f354);
    c.g[A0][0] = c.g[S2][0];
    call_ptr(c, 0x001a_8e40, 0x001b_f354)?;
    frame.restore(c, 16, 0x30);
    Ok(())
}

/// The common exit of cases 200, 201 and 212: `return 0` after one cycle and
/// the restore of `s0`.
fn oc895_v31_exit(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 1;
    c.g[V0][0] = 0;
    frame.restore(c, 16, 0x30);
    Ok(())
}

/// Case 201: the account name and password go to the lobby.
fn oc895_v31_case_201(c: &mut Cpu) -> Flow {
    c.cycles += 14;
    c.g[S0][0] = sx(0x0053_0000u32.wrapping_add(0x6760));
    c.g[A0][0] = oc895_at(c.g[GP][0], 0u32.wrapping_sub(0xc8)).lw(c);
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x48));
    c.g[A1][0] = c.g[S0][0];
    call_ptr(c, 0x0031_84e8, 0x001b_f2f8)?;
    c.g[A0][0] = oc895_at(c.g[GP][0], 0u32.wrapping_sub(0xc8)).lw(c);
    c.g[A1][0] = sx((c.g[S0][0] as u32).wrapping_add(0x11));
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x4c));
    call_ptr(c, 0x0031_84e8, 0x001b_f308)?;
    c.g[A0][0] = oc895_at(c.g[GP][0], 0u32.wrapping_sub(0xc8)).lw(c);
    call_ptr(c, 0x0025_c610, 0x001b_f310)?;
    Ok(())
}

/// Case 200: the verify popup (its texts, its confirm options, then the show call).
fn oc895_v31_case_200(c: &mut Cpu) -> Flow {
    c.cycles += 26;
    c.g[A0][0] = c.g[S2][0];
    call_ptr(c, 0x001a_9c50, 0x001b_f280)?;
    c.g[S0][0] = c.g[V0][0];
    c.g[V0][0] = 1;
    let v = c.g[V0][0];
    oc895_at(c.g[S0][0], 0xbc).sw(c, v);
    c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0xbc));
    oc895_hash(c, 0x2c70, 0x001b_f29c)?;
    c.g[V1][0] = 3;
    let v = c.g[V1][0];
    oc895_at(c.g[S0][0], 0xc0).sw(c, v);
    let v = c.g[V0][0];
    oc895_at(c.g[S0][0], 0xc4).sw(c, v);
    oc895_hash(c, 0x2c88, 0x001b_f2b4)?;
    c.g[A0][0] = c.g[S1][0];
    c.g[A2][0] = c.g[V0][0];
    c.g[A1][0] = 0;
    call_ptr(c, 0x001c_57c0, 0x001b_f2c4)?;
    c.g[V0][0] = 0x80e;
    c.g[A0][0] = c.g[S2][0];
    let v = c.g[V0][0];
    oc895_at(c.g[S0][0], 0x150).sw(c, v);
    c.g[A1][0] = c.g[S0][0];
    call_ptr(c, 0x001a_9bc0, 0x001b_f2d8)?;
    Ok(())
}

/// Case 212: the persona screen (allocated, constructed, then the state change).
fn oc895_v31_case_212(c: &mut Cpu) -> Flow {
    c.cycles += 13;
    c.g[A0][0] = 0x6d8;
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2d68));
    c.g[A2][0] = 0;
    c.g[A3][0] = 0;
    call_ptr(c, 0x0031_7d70, 0x001b_f32c)?;
    c.g[A1][0] = oc895_at(c.g[S2][0], 0x10).lw(c);
    c.g[A0][0] = c.g[V0][0];
    call_ptr(c, 0x001a_d130, 0x001b_f338)?;
    c.g[A0][0] = c.g[S2][0];
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, 0x001a_8a40, 0x001b_f344)?;
    Ok(())
}

// ---------------------------------------------------------------------------
// cFEStateLogin_OnAccountMemPopupEvent (0x1bf438)
// ---------------------------------------------------------------------------

/// `cFEStateLogin_OnAccountMemPopupEvent(self, popup, kind)` (0x1bf438): for
/// kind 15 the base popup handler runs, then the four account widgets and the
/// menu's selected item are refreshed; other kinds run only the base handler.
pub fn oc895_account_popup_event(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 0xf;
        frame.save(c, 16, 0x10);
        frame.save_ra(c, 0);
        let kind_is_15 = c.g[A2][0] == c.g[V0][0];
        c.g[S0][0] = c.g[A0][0];
        if kind_is_15 {
            oc895_account_kind_15(c, frame)?;
        } else {
            c.cycles += 3;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, 0x001a_97b8, 0x001b_f594)?;
            frame.restore(c, 16, 0x10);
            c.cycles += 1;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        oc895_leave(c, frame);
        Ok(())
    });
}

/// The kind 15 part: the base handler, the four account widgets, the menu item.
fn oc895_account_kind_15(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 11;
    c.g[A2][0] = 0xf;
    call_ptr(c, 0x001a_97b8, 0x001b_f458)?;
    oc895_account_widget(c, 0x52c0, 0x001b_f464, 0x001b_f470, 0x001b_f490)?;
    c.cycles += 8;
    oc895_account_widget(c, 0x5330, 0x001b_f49c, 0x001b_f4a8, 0x001b_f4c8)?;
    c.cycles += 8;
    oc895_account_widget(c, 0x52e0, 0x001b_f4d4, 0x001b_f4e0, 0x001b_f500)?;
    c.cycles += 8;
    oc895_account_widget(c, 0x52f0, 0x001b_f50c, 0x001b_f518, 0x001b_f538)?;
    c.cycles += 8;
    oc895_hash(c, 0u32.wrapping_sub(0x2470), 0x001b_f544)?;
    c.g[A0][0] = oc895_at(c.g[S0][0], 0x40).lw(c);
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, 0x0039_d860, 0x001b_f550)?;
    c.g[A0][0] = c.g[V0][0];
    let menu_null = c.g[A0][0] == 0;
    frame.restore(c, 16, 0x10);
    if menu_null {
        c.cycles += 1;
        frame.restore_ra(c, 0);
        return Ok(());
    }
    c.cycles += 3;
    c.g[V0][0] = oc895_at(c.g[A0][0], 0xa0).lw(c);
    let no_item = c.g[V0][0] == 0;
    frame.restore_ra(c, 0);
    if no_item {
        return Ok(());
    }
    c.cycles += 5;
    c.g[V0][0] = oc895_at(c.g[V0][0], 0x14).lw(c);
    c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 5) as u32);
    c.g[V0][0] &= 1;
    if c.g[V0][0] == 0 {
        return Ok(());
    }
    c.cycles += 4;
    c.g[A1][0] = 1;
    call_ptr(c, 0x0039_b960, 0x001b_f584)?;
    frame.restore(c, 16, 0x10);
    c.cycles += 1;
    frame.restore_ra(c, 0);
    Ok(())
}

/// One account widget of the popup: look it up by name hash in the screen and,
/// when found, call its `onWidgetCreate` virtual (slot 0x90).
fn oc895_account_widget(
    c: &mut Cpu,
    name: u32,
    ret_hash: u32,
    ret_get: u32,
    ret_call: u32,
) -> Flow {
    oc895_hash(c, name, ret_hash)?;
    c.g[A0][0] = oc895_at(c.g[S0][0], 0x40).lw(c);
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, 0x0039_d860, ret_get)?;
    c.g[A1][0] = c.g[V0][0];
    if c.g[A1][0] == 0 {
        c.g[A0][0] = 0x0046_0000;
        return Ok(());
    }
    c.cycles += 6;
    c.g[V0][0] = oc895_at(c.g[S0][0], 8).lw(c);
    c.g[A0][0] = oc895_at(c.g[V0][0], 0x90).lh(c);
    c.g[V1][0] = oc895_at(c.g[V0][0], 0x94).lw(c);
    let target = c.g[V1][0] as u32;
    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, target, ret_call)?;
    c.g[A0][0] = 0x0046_0000;
    Ok(())
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{machine, vtable};
    use crate::difftest::{check, elf_image, fresh, Rng, DATA, GAME_GP};

    /// The hash of the string at `0x460000 + off`, from the generated
    /// `GetHashValue32` over the game's image.
    fn oc895_str_hash(off: u32) -> u64 {
        let mut m = fresh();
        elf_image(&mut m);
        m.g[A0][0] = (0x0046_0000 + off) as u64;
        m.pc = 0x0031_7670;
        ee::s10::f_00317670(&mut m);
        m.g[V0][0]
    }

    /// Hashes of the create-room popup's widget names, cached per process.
    fn oc895_hashes() -> &'static [u32] {
        static H: std::sync::OnceLock<Vec<u32>> = std::sync::OnceLock::new();
        H.get_or_init(|| {
            [0x51f8u32, 0x5208, 0x2ba0, 0x5218, 0x5228, 0x2b48, 0x5238]
                .iter()
                .map(|&off| oc895_str_hash(off) as u32)
                .collect()
        })
    }

    fn oc895_setup(c: &mut Cpu, seed: u32, entry: u32) -> Rng {
        let r = machine(c, seed, 0x0895, 0);
        elf_image(c);
        c.g[GP][0] = GAME_GP as u64;
        c.pc = entry;
        r
    }

    #[test]
    fn room_popup_widget_created_matches() {
        check(
            ee::s03::f_001be8b0__generated,
            oc895_room_popup_widget_created,
            60,
            &|c, seed| {
                oc895_setup(c, seed, 0x001b_e8b0);
                let obj = DATA + 0x1000;
                let widget = DATA + 0x2000;
                c.g[A0][0] = obj as u64;
                c.g[A1][0] = widget as u64;
                let hashes = oc895_hashes();
                let pick = seed as usize % (hashes.len() + 1);
                let h = if pick < hashes.len() {
                    hashes[pick]
                } else {
                    0xdead_beef
                };
                c.bus.write32(widget + 0x38, h);
            },
        );
    }

    #[test]
    fn room_popup_v25_matches() {
        check(
            ee::s03::f_001bed58__generated,
            oc895_room_popup_v25,
            40,
            &|c, seed| {
                oc895_setup(c, seed, 0x001b_ed58);
                c.g[A1][0] = (DATA + 0x1000) as u64;
            },
        );
    }

    #[test]
    fn login_update_matches() {
        check(
            ee::s03::f_001bef40__generated,
            oc895_login_update,
            40,
            &|c, seed| {
                oc895_setup(c, seed, 0x001b_ef40);
                c.g[A0][0] = (DATA + 0x1000) as u64;
            },
        );
    }

    #[test]
    fn login_popup_v31_matches() {
        check(
            ee::s03::f_001bf228__generated,
            oc895_login_popup_v31,
            50,
            &|c, seed| {
                oc895_setup(c, seed, 0x001b_f228);
                // Case 212 allocates the persona screen: give it the engine's
                // pool allocator (the real cMemMan_alloc, 0x317d70).
                crate::game::ai::computer::tests::rbc_test_allocator(c);
                c.g[GP][0] = GAME_GP as u64;
                c.pc = 0x001b_f228;
                let obj = DATA + 0x1000;
                c.g[A0][0] = obj as u64;
                // 200, 201, 212, a default below 202 and a default above it.
                let ids = [200u64, 201, 212, 150, 300];
                c.g[A1][0] = ids[seed as usize % ids.len()];
                c.bus.write32(obj + 0x8, DATA + 0x3000);
                c.bus.write32(obj + 0x10, DATA + 0x4000);
            },
        );
    }

    #[test]
    fn account_popup_event_matches() {
        check(
            ee::s03::f_001bf438__generated,
            oc895_account_popup_event,
            60,
            &|c, seed| {
                let mut r = oc895_setup(c, seed, 0x001b_f438);
                let this = DATA + 0x1000;
                let popup = DATA + 0x2000;
                let screen = DATA + 0x3000;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = popup as u64;
                c.g[A2][0] = if seed % 3 == 0 { 3 } else { 15 };
                let vt = vtable(c, &mut r, &[0x90]);
                c.bus.write32(this + 8, vt);
                c.bus.write32(this + 0x40, screen);
            },
        );
    }
}
