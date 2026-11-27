//! Staging module oc948 (wave wave256): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc948`.

#![allow(unused_imports)]

use crate::abi::r32;
use crate::call::{call_ptr, guest, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x001d_eb90,
        cfe_popup_video_calibration_init,
        "cFEPopupVideoCalibration_Init",
    ),
    (
        0x001d_ec68,
        cfe_popup_video_calibration_handle_input,
        "cFEPopupVideoCalibration_HandleInput",
    ),
    (
        0x001d_ede8,
        cfe_popup_screen_pos_dtor,
        "cFEPopupScreenPos_dtor",
    ),
    (
        0x001d_f380,
        cfe_state_rider_bio_ctor_2,
        "cFEStateRiderBio_ctor_2",
    ),
    (
        0x001d_f3b8,
        cfe_state_rider_bio_dtor,
        "cFEStateRiderBio_dtor",
    ),
    (
        0x001d_f3e0,
        cfe_state_rule_select_ctor_2,
        "cFEStateRuleSelect_ctor_2",
    ),
    (
        0x001d_f470,
        cfe_state_bragging_rights_ctor_2,
        "cFEStateBraggingRights_ctor_2",
    ),
    (
        0x001d_f580,
        cfe_state_chat_room_ctor_2,
        "cFEStateChatRoom_ctor_2",
    ),
];

/// Callees of this module's bodies (all reached through `call_indirect`).
const STR_HASH: u32 = 0x0031_7670; // GetHashValue32(char*)
const ADD_SCREEN: u32 = 0x0039_7cc0; // cUIEngine_addScreenByHashName
const PLAY_FRAME: u32 = 0x0039_c870; // cUIScreen_playFrame
const FIND_OBJECT: u32 = 0x0039_d860; // cUIScreen_getObjectByHashName
const SET_TEXT_ID: u32 = 0x003a_1000; // cUIText_setUnicodeStringByID
const UI_STATE_CTOR: u32 = 0x0039_e2a0; // UIState_ctor
const UI_STATE_DTOR: u32 = 0x0039_e390; // UIState_dtor
const NET_FE_STATE_CTOR: u32 = 0x001a_8500; // NetFEState_ctor
const INIT_PLAYER_ROWS: u32 = 0x0019_26f0; // cFEStateBraggingRights_InitPlayerRows

/// `void cFEPopupVideoCalibration_Init(self)` (0x1deb90; the reference name is
/// `cFEPopupVideoCalibration_onCreateScreen`): open the calibration screen
/// (`self+0x40`) by its hash name, play one frame of it, show its title and
/// clear the "shown" flag at `self+0x48`.
pub fn cfe_popup_video_calibration_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        frame.save(c, S0, 0x20);
        // The screen: GetHashValue32(D_004A1548), then addScreen(engine, self, hash, 0).
        c.g[A0][0] = sx(0x004a_1548);
        c.g[S0][0] = Addr(r32(c, S1).wrapping_add(0x10)).lw(c);
        call_ptr(c, STR_HASH, 0x001d_ebb4)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S1][0];
        c.g[A3][0] = 0;
        call_ptr(c, ADD_SCREEN, 0x001d_ebc8)?;
        let no_screen = c.g[V0][0] == 0;
        let value = c.g[V0][0];
        Addr(r32(c, S1).wrapping_add(0x40)).sw(c, value);
        if !no_screen {
            c.cycles += 4;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            call_ptr(c, PLAY_FRAME, 0x001d_ebe0)?;
        }
        // The title widget: setVisible(1).
        c.cycles += 8;
        c.g[A0][0] = Addr(r32(c, GP).wrapping_sub(0x1bb0)).lw(c);
        call_ptr(c, STR_HASH, 0x001d_ebe8)?;
        calibration_find_screen_widget(c, 0x001d_ebf4)?;
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = 1;
        if c.g[A2][0] != 0 {
            popup_call_method48(c, 0x001d_ec14)?;
        }
        // The title text.
        c.cycles += 9;
        c.g[A0][0] = sx(0x0045_dd90);
        call_ptr(c, STR_HASH, 0x001d_ec20)?;
        calibration_find_screen_widget(c, 0x001d_ec2c)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A0][0] = 0x0046_0000;
        if c.g[S0][0] != 0 {
            c.cycles += 5;
            c.g[A0][0] = sx(0x0045_dda0);
            call_ptr(c, STR_HASH, 0x001d_ec40)?;
            c.g[A0][0] = c.g[S0][0];
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, SET_TEXT_ID, 0x001d_ec4c)?;
        }
        c.cycles += 6;
        Addr(r32(c, S1).wrapping_add(0x48)).sb(c, 0);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `int cFEPopupVideoCalibration_HandleInput(self, w)` (0x1dec68; the
/// reference name is `func_001DEC68`): when the widget's `vt[5]` method
/// answers, show the calibration popup the first time (or refresh its
/// position on later calls), then play its sound. Returns 1.
pub fn cfe_popup_video_calibration_handle_input(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S1, 0x10);
        frame.save(c, S0, 0x20);
        c.g[S1][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        // w->vt[5].fn(w + delta)
        c.g[V1][0] = Addr(r32(c, A1).wrapping_add(8)).lw(c);
        c.g[A0][0] = Addr(r32(c, V1).wrapping_add(0x28)).lh(c);
        c.g[V0][0] = Addr(r32(c, V1).wrapping_add(0x2c)).lw(c);
        let widget_method = r32(c, V0);
        c.g[A0][0] = sx(r32(c, A1).wrapping_add(r32(c, A0)));
        call_ptr(c, widget_method, 0x001d_ec90)?;
        frame.restore(c, S0, 0x20);
        if c.g[V0][0] != 0 {
            calibration_popup_body(c, frame)?;
        }
        c.cycles += 5;
        c.g[V0][0] = 1;
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// The part of `HandleInput` that runs when the widget answered: the
/// "shown" flag at `self+0x48` decides between a refresh of the popup flags
/// and the first show, then the popup's sound plays (`self+0x10 -> +0x14`,
/// or the default sound object at `gp+0x2968`).
fn calibration_popup_body(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 3;
    c.g[V0][0] = Addr(r32(c, S1).wrapping_add(0x48)).lb(c);
    if c.g[V0][0] != 0 {
        c.g[V0][0] = Addr(r32(c, S1).wrapping_add(0x1c)).lw(c);
        c.cycles += 5;
        c.g[V1][0] = 0xffff_ffff_ffff_c0ff;
        c.g[V0][0] |= 0x80;
        c.g[V0][0] &= c.g[V1][0];
        c.g[V0][0] |= 0x700;
        let value = c.g[V0][0];
        Addr(r32(c, S1).wrapping_add(0x1c)).sw(c, value);
    } else {
        calibration_show_1bb0(c)?;
        calibration_show_1bac(c)?;
        calibration_title_text(c)?;
        calibration_show_45ddc8(c)?;
        c.cycles += 3;
        c.g[V0][0] = 1;
        let value = c.g[V0][0];
        Addr(r32(c, S1).wrapping_add(0x48)).sb(c, value);
    }
    // The popup's sound: sound->vt[2].fn(sound + delta, 6).
    c.cycles += 6;
    c.g[V0][0] = Addr(r32(c, S1).wrapping_add(0x10)).lw(c);
    c.g[V1][0] = sx(r32(c, GP).wrapping_add(0x2968));
    c.g[A2][0] = Addr(r32(c, V0).wrapping_add(0x14)).lw(c);
    if c.g[A2][0] == 0 {
        c.g[A2][0] = c.g[V1][0];
    }
    c.g[A1][0] = 6;
    if c.g[A2][0] != 0 {
        c.cycles += 5;
        c.g[V0][0] = Addr(r32(c, A2)).lw(c);
        c.g[A0][0] = Addr(r32(c, V0).wrapping_add(0x10)).lh(c);
        c.g[V1][0] = Addr(r32(c, V0).wrapping_add(0x14)).lw(c);
        let sound_method = r32(c, V1);
        c.g[A0][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
        call_ptr(c, sound_method, 0x001d_edd0)?;
    }
    c.cycles += 1;
    frame.restore(c, S0, 0x20);
    Ok(())
}

/// `GetHashValue32` of the global at `gp-0x1bb0`, the screen's widget of that
/// name, and when found its method with a1 = 0 (`show_EC68(self, name, 0)`).
fn calibration_show_1bb0(c: &mut Cpu) -> Flow {
    c.cycles += 8;
    c.g[A0][0] = Addr(r32(c, GP).wrapping_sub(0x1bb0)).lw(c);
    call_ptr(c, STR_HASH, 0x001d_ecac)?;
    calibration_find_screen_widget(c, 0x001d_ecb8)?;
    c.g[A2][0] = c.g[V0][0];
    c.g[A1][0] = 0;
    if c.g[A2][0] != 0 {
        popup_call_method48(c, 0x001d_ecd8)?;
    }
    Ok(())
}

/// `show_EC68(self, gp-0x1bac name, 1)`: the same lookup with a1 = 1.
fn calibration_show_1bac(c: &mut Cpu) -> Flow {
    c.cycles += 9;
    c.g[A0][0] = Addr(r32(c, GP).wrapping_sub(0x1bac)).lw(c);
    call_ptr(c, STR_HASH, 0x001d_ece4)?;
    calibration_find_screen_widget(c, 0x001d_ecf0)?;
    c.g[A2][0] = c.g[V0][0];
    c.g[A1][0] = 1;
    if c.g[A2][0] != 0 {
        popup_call_method48(c, 0x001d_ed10)?;
    }
    Ok(())
}

/// The title text: `cUIText_setUnicodeStringByID(text, hash(D_0045DDB8))`
/// when the widget `D_0045DD90` exists.
fn calibration_title_text(c: &mut Cpu) -> Flow {
    c.cycles += 9;
    c.g[A0][0] = sx(0x0045_dd90);
    call_ptr(c, STR_HASH, 0x001d_ed1c)?;
    calibration_find_screen_widget(c, 0x001d_ed28)?;
    c.g[S0][0] = c.g[V0][0];
    if c.g[S0][0] == 0 {
        c.g[A0][0] = 0x0046_0000;
    } else {
        c.cycles += 7;
        c.g[A0][0] = 0x0046_0000;
        c.g[A0][0] = sx(0x0045_ddb8);
        call_ptr(c, STR_HASH, 0x001d_ed40)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, SET_TEXT_ID, 0x001d_ed4c)?;
        c.g[A0][0] = 0x0046_0000;
    }
    Ok(())
}

/// `show_EC68(self, D_0045DDC8, 0)`: the last of the four popup steps.
fn calibration_show_45ddc8(c: &mut Cpu) -> Flow {
    c.cycles += 8;
    c.g[A0][0] = sx(0x0045_ddc8);
    call_ptr(c, STR_HASH, 0x001d_ed58)?;
    calibration_find_screen_widget(c, 0x001d_ed64)?;
    c.g[A2][0] = c.g[V0][0];
    c.g[A1][0] = 0;
    if c.g[A2][0] != 0 {
        popup_call_method48(c, 0x001d_ed84)?;
    }
    Ok(())
}

/// `cUIScreen_getObjectByHashName(self->screen (+0x40), a1 = the hash in v0)`,
/// returning to `ret`. The caller sets a2 from v0 where the code does.
fn calibration_find_screen_widget(c: &mut Cpu, ret: u32) -> Flow {
    c.g[A0][0] = Addr(r32(c, S1).wrapping_add(0x40)).lw(c);
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, FIND_OBJECT, ret)
}

/// The method at vtable slot 0x48 of the object in a2 (`cycles 5`):
/// `obj->vt[9].fn(obj + delta, a1)`, returning to `ret`.
fn popup_call_method48(c: &mut Cpu, ret: u32) -> Flow {
    c.cycles += 5;
    c.g[V0][0] = Addr(r32(c, A2).wrapping_add(8)).lw(c);
    c.g[A0][0] = Addr(r32(c, V0).wrapping_add(0x48)).lh(c);
    c.g[V1][0] = Addr(r32(c, V0).wrapping_add(0x4c)).lw(c);
    let method = r32(c, V1);
    c.g[A0][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
    call_ptr(c, method, ret)
}

/// A UIState-derived destructor that stores the vtable word `vtable` at
/// `self+8` and then runs `UIState_dtor(self)`, returning to `ret`.
fn ui_state_dtor_store(c: &mut Cpu, vtable: u64, ret: u32) -> Flow {
    c.cycles += 10;
    let frame = Frame::open(c, 0x10);
    frame.save_ra(c, 0);
    c.g[V1][0] = c.g[A0][0];
    c.g[V0][0] = vtable;
    let value = c.g[V0][0];
    Addr(r32(c, V1).wrapping_add(8)).sw(c, value);
    call_ptr(c, UI_STATE_DTOR, ret)?;
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// A UIState-derived constructor shell: `UIState_ctor(self, 0)`, then the
/// vtable word `vtable` at `self+8`. Returns to `ret` (the caller's).
fn ui_state_ctor_store(c: &mut Cpu, vtable: u64, ret: u32) -> Flow {
    c.cycles += 14;
    let frame = Frame::open(c, 0x20);
    c.g[A1][0] = 0;
    frame.save(c, S0, 0x10);
    frame.save_ra(c, 0);
    c.g[S0][0] = c.g[A0][0];
    call_ptr(c, UI_STATE_CTOR, ret)?;
    c.g[V1][0] = vtable;
    c.g[V0][0] = c.g[S0][0];
    frame.restore_ra(c, 0);
    let value = c.g[V1][0];
    Addr(r32(c, S0).wrapping_add(8)).sw(c, value);
    frame.restore(c, S0, 0x10);
    frame.close(c);
    Ok(())
}

/// `void cFEPopupScreenPos_dtor(self)` (0x1dede8).
pub fn cfe_popup_screen_pos_dtor(c: &mut Cpu) {
    guest(c, |c| ui_state_dtor_store(c, 0x0046_b230, 0x001d_ee04));
}

/// `cFEStateRiderBio_ctor_2(self)` (0x1df380): `UIState_ctor`, then the
/// rider-bio vtable at `self+8`.
pub fn cfe_state_rider_bio_ctor_2(c: &mut Cpu) {
    guest(c, |c| ui_state_ctor_store(c, 0x0046_b160, 0x001d_f398));
}

/// `cFEStateRiderBio_dtor(self)` (0x1df3b8).
pub fn cfe_state_rider_bio_dtor(c: &mut Cpu) {
    guest(c, |c| ui_state_dtor_store(c, 0x0046_d0d0, 0x001d_f3d4));
}

/// `cFEStateRuleSelect_ctor_2(self)` (0x1df3e0).
pub fn cfe_state_rule_select_ctor_2(c: &mut Cpu) {
    guest(c, |c| ui_state_ctor_store(c, 0x0046_b090, 0x001d_f3f8));
}

/// `cFEStateBraggingRights_ctor_2(self)` (0x1df470): `NetFEState_ctor`, the
/// bragging-rights vtable at `self+8`, then `InitPlayerRows` on the two
/// player rows at `self+0x6d0` (0x34 bytes apart). Returns `self`.
pub fn cfe_state_bragging_rights_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x50);
        c.g[A1][0] = 0;
        frame.save(c, S2, 0x20);
        c.g[A2][0] = 0;
        c.g[S2][0] = c.g[A0][0];
        frame.save(c, S0, 0x40);
        frame.save(c, S1, 0x30);
        frame.save(c, S3, 0x10);
        c.g[S0][0] = sx(r32(c, S2).wrapping_add(0x6d0));
        frame.save_ra(c, 0);
        c.g[S1][0] = 1;
        call_ptr(c, NET_FE_STATE_CTOR, 0x001d_f4a0)?;
        c.g[S3][0] = u64::MAX;
        c.g[V0][0] = 0x0046_af68;
        let value = c.g[V0][0];
        Addr(r32(c, S2).wrapping_add(8)).sw(c, value);
        loop {
            c.cycles += 5;
            c.g[A0][0] = c.g[S0][0];
            c.g[S1][0] = sx(r32(c, S1).wrapping_sub(1));
            call_ptr(c, INIT_PLAYER_ROWS, 0x001d_f4bc)?;
            let more = c.g[S1][0] != c.g[S3][0];
            c.g[S0][0] = sx(r32(c, S0).wrapping_add(0x34));
            if !more {
                break;
            }
            if event_due(c) {
                deliver_event_here(c, 0x001d_f4b0);
            }
        }
        c.cycles += 8;
        c.g[V0][0] = c.g[S2][0];
        frame.restore(c, S0, 0x40);
        frame.restore(c, S1, 0x30);
        frame.restore(c, S2, 0x20);
        frame.restore(c, S3, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `cFEStateChatRoom_ctor_2(self)` (0x1df580): `NetFEState_ctor`, clear the
/// word at `self+0x6f8`, then the chat-room vtable at `self+8`.
pub fn cfe_state_chat_room_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 16;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0;
        frame.save(c, S0, 0x10);
        c.g[A2][0] = 0;
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, NET_FE_STATE_CTOR, 0x001d_f59c)?;
        c.g[V1][0] = 0x0046_ae58;
        c.g[V0][0] = c.g[S0][0];
        Addr(r32(c, S0).wrapping_add(0x6f8)).sw(c, 0);
        let value = c.g[V1][0];
        Addr(r32(c, S0).wrapping_add(8)).sw(c, value);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::fakes;
    use crate::difftest::fakes::machine;
    use crate::difftest::{DATA, GAME_GP};
    use std::sync::atomic::AtomicUsize;

    const SEEDS: u32 = 240;

    /// Gate word of this module's gated fakes (its own word in the machine).
    const CTRL: u32 = DATA + 0x7e_0000;
    const MAGIC: u32 = 0x4f43_3948;
    /// Forced-result mailboxes: one word per fake address at `MAIL + (addr & 0xffc)`.
    const MAIL: u32 = DATA + 0x7f_0000;

    /// Fixture objects, clear of the fakes' pointers (`DATA..DATA+0x40000`)
    /// and of the call log (`DATA+0x100000`).
    const SELF: u32 = DATA + 0x50_0000;
    const WIDGET: u32 = DATA + 0x50_1000;
    const WIDGET_VT: u32 = DATA + 0x50_1100;
    const ENG: u32 = DATA + 0x50_2000;
    const SNDO: u32 = DATA + 0x50_3000;
    const SND_VT: u32 = DATA + 0x50_3100;
    const DEF_VT: u32 = DATA + 0x50_3200;
    const OBJ: u32 = DATA + 0x50_4000;
    const OBJ_VT: u32 = DATA + 0x50_4100;
    const SCREEN: u32 = DATA + 0x50_5000;
    /// A zeroed buffer: the empty hash name the gp globals point at.
    const NAME: u32 = DATA + 0x51_0000;

    /// Method targets outside the function table (the miss hook answers).
    const F_WIDGET: u32 = fakes::FAKE + 0x60;
    const F_OBJ: u32 = fakes::FAKE + 0x30;
    const F_SND: u32 = fakes::FAKE + 0x50;
    const F_DEF: u32 = fakes::FAKE + 0x70;

    fn oc948_mail(addr: u32) -> u32 {
        MAIL + (addr & 0xffc)
    }

    /// Make the fake for `addr` return `v` (0 is a real zero result).
    fn oc948_force(c: &mut Cpu, addr: u32, v: u32) {
        c.bus.write32(oc948_mail(addr), v.wrapping_add(3));
    }

    /// The fake callee: logs the call, then returns the forced result of
    /// its mailbox unless it unwound.
    fn oc948_fake_call(c: &mut Cpu, addr: u32) {
        let ra = c.g[RA][0] as u32;
        let mail = c.bus.read32(oc948_mail(addr));
        fakes::fake_call(c, addr);
        if c.pc == ra && mail != 0 {
            c.g[V0][0] = (mail - 3) as u64;
        }
    }

    /// The miss hook of this module's machines (targets outside the table).
    fn oc948_miss(c: &mut Cpu, target: u32) {
        oc948_fake_call(c, target)
    }

    /// Gated fakes for the callees with override slots: with the gate off they
    /// run the body they replaced.
    macro_rules! oc948_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(CTRL) != MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc948_fake_call(c, $addr);
                }
            )*
            fn oc948_install() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc948_fakes!(
        (
            oc948_fake_397cc0,
            OC948_PREV_397CC0,
            ee::s13::OVERRIDE_00397CC0,
            ee::s13::f_00397cc0__generated,
            0x0039_7cc0
        ),
        (
            oc948_fake_39c870,
            OC948_PREV_39C870,
            ee::s13::OVERRIDE_0039C870,
            ee::s13::f_0039c870__generated,
            0x0039_c870
        ),
        (
            oc948_fake_39d860,
            OC948_PREV_39D860,
            ee::s13::OVERRIDE_0039D860,
            ee::s13::f_0039d860__generated,
            0x0039_d860
        ),
        (
            oc948_fake_3a1000,
            OC948_PREV_3A1000,
            ee::s13::OVERRIDE_003A1000,
            ee::s13::f_003a1000__generated,
            0x003a_1000
        ),
        (
            oc948_fake_39e2a0,
            OC948_PREV_39E2A0,
            ee::s13::OVERRIDE_0039E2A0,
            ee::s13::f_0039e2a0__generated,
            0x0039_e2a0
        ),
        (
            oc948_fake_39e390,
            OC948_PREV_39E390,
            ee::s13::OVERRIDE_0039E390,
            ee::s13::f_0039e390__generated,
            0x0039_e390
        ),
        (
            oc948_fake_1a8500,
            OC948_PREV_1A8500,
            ee::s03::OVERRIDE_001A8500,
            ee::s03::f_001a8500__generated,
            0x001a_8500
        ),
        (
            oc948_fake_1926f0,
            OC948_PREV_1926F0,
            ee::s02::OVERRIDE_001926F0,
            ee::s02::f_001926f0__generated,
            0x0019_26f0
        ),
    );

    /// Scrambled machine with this module's fakes hooked and its gate open.
    fn oc948_machine(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> crate::difftest::Rng {
        oc948_install();
        let r = machine(c, seed, salt, calls);
        c.hooks.miss = oc948_miss;
        c.bus.write32(CTRL, MAGIC);
        // The data the callees read through gp: the hash names and the
        // default sound object (`gp+0x2968`, a vtable pointer).
        c.bus.write32(GAME_GP.wrapping_sub(0x1bb0), NAME);
        c.bus.write32(GAME_GP.wrapping_sub(0x1bac), NAME);
        c.bus.write32(GAME_GP + 0x2968, DEF_VT);
        r
    }

    /// A vtable at `vt` whose slot `off` is `{delta, func}`.
    fn oc948_slot(c: &mut Cpu, vt: u32, off: u32, delta: i16, func: u32) {
        c.bus.write16(vt + off, delta as u16);
        c.bus.write32(vt + off + 4, func);
    }

    #[test]
    fn cfe_popup_video_calibration_init_matches() {
        check(
            ee::s04::f_001deb90__generated,
            cfe_popup_video_calibration_init,
            SEEDS,
            &|c, seed| {
                let mut r = oc948_machine(c, seed, 0x001d_eb90, 6);
                c.g[A0][0] = SELF as u64;
                c.bus.write32(SELF + 0x10, ENG);
                c.bus.write8(SELF + 0x48, r.next_u32() as u8);
                if seed % 3 == 0 {
                    oc948_force(c, ADD_SCREEN, 0);
                }
                oc948_force(c, FIND_OBJECT, if seed % 4 == 1 { 0 } else { OBJ });
                c.bus.write32(OBJ + 8, OBJ_VT);
                oc948_slot(c, OBJ_VT, 0x48, (r.next_u32() as i16) & !3, F_OBJ);
            },
        );
    }

    #[test]
    fn cfe_popup_video_calibration_handle_input_matches() {
        check(
            ee::s04::f_001dec68__generated,
            cfe_popup_video_calibration_handle_input,
            SEEDS,
            &|c, seed| {
                let mut r = oc948_machine(c, seed, 0x001d_ec68, 8);
                c.g[A0][0] = SELF as u64;
                c.g[A1][0] = WIDGET as u64;
                c.bus.write32(WIDGET + 8, WIDGET_VT);
                oc948_slot(c, WIDGET_VT, 0x28, (r.next_u32() as i16) & !3, F_WIDGET);
                // The widget's answer: zero for a quarter of the seeds.
                if seed % 4 == 0 {
                    oc948_force(c, F_WIDGET, 0);
                }
                c.bus.write32(SELF + 0x10, ENG);
                c.bus.write32(SELF + 0x40, SCREEN);
                c.bus.write8(SELF + 0x48, (seed % 2) as u8);
                c.bus.write32(SELF + 0x1c, r.next_u32());
                // The popup sound: its own object, or the default at gp+0x2968.
                c.bus
                    .write32(ENG + 0x14, if seed % 3 == 0 { 0 } else { SNDO });
                c.bus.write32(SNDO, SND_VT);
                oc948_slot(c, SND_VT, 0x10, (r.next_u32() as i16) & !3, F_SND);
                oc948_slot(c, DEF_VT, 0x10, (r.next_u32() as i16) & !3, F_DEF);
                // The widgets found on the screen: none for a fifth of the seeds.
                oc948_force(c, FIND_OBJECT, if seed % 5 == 1 { 0 } else { OBJ });
                c.bus.write32(OBJ + 8, OBJ_VT);
                oc948_slot(c, OBJ_VT, 0x48, (r.next_u32() as i16) & !3, F_OBJ);
            },
        );
    }

    #[test]
    fn cfe_popup_screen_pos_dtor_matches() {
        check(
            ee::s04::f_001dede8__generated,
            cfe_popup_screen_pos_dtor,
            SEEDS,
            &|c, seed| {
                let _ = oc948_machine(c, seed, 0x001d_ede8, 1);
                c.g[A0][0] = SELF as u64;
            },
        );
    }

    #[test]
    fn cfe_state_rider_bio_ctor_2_matches() {
        check(
            ee::s04::f_001df380__generated,
            cfe_state_rider_bio_ctor_2,
            SEEDS,
            &|c, seed| {
                let _ = oc948_machine(c, seed, 0x001d_f380, 1);
                c.g[A0][0] = SELF as u64;
            },
        );
    }

    #[test]
    fn cfe_state_rider_bio_dtor_matches() {
        check(
            ee::s04::f_001df3b8__generated,
            cfe_state_rider_bio_dtor,
            SEEDS,
            &|c, seed| {
                let _ = oc948_machine(c, seed, 0x001d_f3b8, 1);
                c.g[A0][0] = SELF as u64;
            },
        );
    }

    #[test]
    fn cfe_state_rule_select_ctor_2_matches() {
        check(
            ee::s04::f_001df3e0__generated,
            cfe_state_rule_select_ctor_2,
            SEEDS,
            &|c, seed| {
                let _ = oc948_machine(c, seed, 0x001d_f3e0, 1);
                c.g[A0][0] = SELF as u64;
            },
        );
    }

    #[test]
    fn cfe_state_bragging_rights_ctor_2_matches() {
        check(
            ee::s04::f_001df470__generated,
            cfe_state_bragging_rights_ctor_2,
            SEEDS,
            &|c, seed| {
                let _ = oc948_machine(c, seed, 0x001d_f470, 4);
                c.g[A0][0] = SELF as u64;
            },
        );
    }

    #[test]
    fn cfe_state_chat_room_ctor_2_matches() {
        check(
            ee::s04::f_001df580__generated,
            cfe_state_chat_room_ctor_2,
            SEEDS,
            &|c, seed| {
                let _ = oc948_machine(c, seed, 0x001d_f580, 1);
                c.g[A0][0] = SELF as u64;
            },
        );
    }
}
