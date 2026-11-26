//! Staging module oc851 (wave wave250): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc851`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame, Target};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0019_4138,
        bragging_rights_v24,
        "cFEStateBraggingRights_v24",
    ),
    (
        0x0019_4168,
        bragging_rights_on_child_event,
        "cFEStateBraggingRights_OnChildEvent",
    ),
    (
        0x0019_41b0,
        bragging_rights_v31,
        "cFEStateBraggingRights_v31",
    ),
    (
        0x0019_5328,
        main_menu_on_child_event,
        "cFEStateMainMenu_OnChildEvent",
    ),
    (
        0x0019_55e0,
        bonus_material_on_activate,
        "cFEStateBonusMaterial_OnActivate",
    ),
    (
        0x0019_5600,
        bonus_material_on_widget_action,
        "cFEStateBonusMaterial_OnWidgetAction",
    ),
];

/// Gp-relative word that is 1 while the online front end is up
/// (`isOn00194138` in the reference).
const OC851_ONLINE_FLAG: u32 = 0x1680;

/// `void* cFEStateBraggingRights_handleEvent(self, int msgId, int data)`
/// (0x194138): while the online flag at `gp-0x1680` is 1, forward to
/// `cFEStateNet_handleEvent` (0x1a8b88); otherwise return 0.
pub fn bragging_rights_v24(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let flag = Addr(r32(c, GP).wrapping_sub(OC851_ONLINE_FLAG)).lw(c);
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = flag ^ 1;
        frame.save_ra(c, 0);
        c.cycles += 2;
        if c.g[V0][0] == 0 {
            call_ptr(c, 0x001a_8b88, 0x0019_415c)?;
        } else {
            c.g[V0][0] = 0;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateBraggingRights_onPopupEvent(self, cUIState* popup, int event)`
/// (0x194168): while online, popup id 0x838 (field +0x18) goes to
/// `OnChildEventQuit` (0x194440), any other to `cFEStateNet_onPopupEvent`
/// (0x1a97b8).
pub fn bragging_rights_on_child_event(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 5;
        let flag = Addr(r32(c, GP).wrapping_sub(OC851_ONLINE_FLAG)).lw(c);
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = flag ^ 1;
        frame.save_ra(c, 0);
        if c.g[V0][0] != 0 {
            c.cycles += 1;
            frame.restore_ra(c, 0);
        } else {
            c.cycles += 4;
            c.g[V1][0] = Addr(r32(c, A1).wrapping_add(0x18)).lw(c);
            c.g[V0][0] = 0x838;
            if c.g[V1][0] == c.g[V0][0] {
                c.cycles += 4;
                Callee {
                    addr: 0x0019_4440,
                    f: ee::s02::f_00194440,
                }
                .call(c, 0x0019_4194)?;
                frame.restore_ra(c, 0);
            } else {
                c.cycles += 2;
                call_ptr(c, 0x001a_97b8, 0x0019_41a4)?;
                c.cycles += 1;
                frame.restore_ra(c, 0);
            }
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    })
}

/// Stride of one `cBraggingRightsMenu` entry (the row tables use `* 0x34`).
const OC851_MENU_STRIDE: u32 = 0x34;

/// `int cFEStateBraggingRights_handleMessage(self, int msg, data)` (0x1941b0):
/// the message switch of the Bragging Rights screen. Messages 0xee..0x113
/// index a jump table at 0x45ff40; the cases are row selection (251), list
/// box selection (252), confirm (253), unconfirm and prompt (254), the
/// network back-outs (238, 241, 248, 250, 274, 275) and the chat/online main
/// menu switch (268). Everything else goes to `NetFEState` (0x1a8e40).
///
/// The generated body has a resume twin; this entry keeps the guard.
pub fn bragging_rights_v31(c: &mut Cpu) {
    if c.pc != 0x0019_41b0 {
        return ee::s02::f_001941b0__entries(c);
    }
    guest(c, |c| {
        loop {
            c.cycles += 12;
            let frame = Frame::open(c, 0x70);
            c.g[V1][0] = sx(r32(c, A1).wrapping_sub(0xee));
            frame.save(c, 17, 0x50);
            c.g[V0][0] = (c.g[V1][0] < 0x26) as u64;
            frame.save(c, 18, 0x40);
            c.g[S1][0] = c.g[A0][0];
            frame.save(c, 16, 0x60);
            c.g[S2][0] = c.g[A2][0];
            frame.save(c, 19, 0x30);
            frame.save(c, 20, 0x20);
            let in_range = c.g[V0][0] != 0;
            frame.save_ra(c, 0x10);
            // Tail entry: 2 = 0x194418 (v0 = 0, then 1), 1 = 0x19441c, 0 = 0x194420.
            let tail: u8;
            let mut target = 0x0019_4404u32;
            if in_range {
                c.cycles += 7;
                c.g[V0][0] = 0x0046_0000;
                c.g[V1][0] = sx(r32(c, V1) << 2);
                c.g[V0][0] = sx(r32(c, V0).wrapping_sub(0xc0));
                c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
                c.g[A0][0] = Addr(r32(c, V1)).lw(c);
                target = r32(c, A0);
            }
            match target {
                0x0019_41b0 => {
                    if event_due(c) {
                        deliver_event_here(c, 0x0019_41b0);
                    }
                    continue;
                }
                // Message 251: select an entry of the player's menu.
                0x0019_41fc => {
                    c.cycles += 9;
                    c.g[V0][0] = Addr(r32(c, S2).wrapping_add(8)).lw(c);
                    c.g[V1][0] = OC851_MENU_STRIDE as u64;
                    c.g[A1][0] = Addr(r32(c, S2).wrapping_add(0xc)).lbu(c);
                    c.mult(0, 4, 2, 3);
                    c.g[V0][0] = sx(r32(c, A0).wrapping_add(r32(c, S1)));
                    c.g[A0][0] = Addr(r32(c, V0).wrapping_add(0x6d0)).lw(c);
                    call_ptr(c, 0x0039_b960, 0x0019_4218)?;
                    c.g[V0][0] = 0;
                    tail = 1;
                }
                // Message 252: choose a list box entry by key.
                0x0019_4220 => {
                    c.cycles += 14;
                    c.g[V1][0] = Addr(r32(c, S2).wrapping_add(8)).lw(c);
                    c.g[S0][0] = OC851_MENU_STRIDE as u64;
                    c.g[V0][0] = Addr(r32(c, S2).wrapping_add(0xc)).lw(c);
                    c.g[A2][0] = c.g[SP][0];
                    c.mult(0, 4, 3, 16);
                    c.g[A1][0] = Addr(r32(c, S2).wrapping_add(0x10)).lw(c);
                    c.g[V0][0] = sx(r32(c, V0) << 2);
                    c.g[V1][0] = sx(r32(c, A0).wrapping_add(r32(c, S1)));
                    c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
                    c.g[S3][0] = Addr(r32(c, V1).wrapping_add(0x6d8)).lw(c);
                    c.g[A0][0] = c.g[S3][0];
                    call_ptr(c, 0x0039_a768, 0x0019_4250)?;
                    let found = c.g[V0][0] != 0;
                    c.g[A1][0] = Addr(sp(c)).lbu(c);
                    if !found {
                        tail = 2;
                    } else {
                        c.cycles += 14;
                        c.g[A0][0] = c.g[S3][0];
                        call_ptr(c, 0x0039_a7a8, 0x0019_4260)?;
                        c.g[A0][0] = Addr(r32(c, S2).wrapping_add(8)).lw(c);
                        c.g[A1][0] = c.g[S3][0];
                        c.mult(0, 4, 4, 16);
                        c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x6d0));
                        c.g[A0][0] = sx(r32(c, S1).wrapping_add(r32(c, A0)));
                        Callee {
                            addr: 0x0019_2918,
                            f: ee::s02::f_00192918,
                        }
                        .call(c, 0x0019_4278)?;
                        c.g[V1][0] = Addr(r32(c, S1).wrapping_add(0x738)).lw(c);
                        c.g[A0][0] = 0xffff_ffff_ffff_ffff;
                        c.g[V1][0] = sx(r32(c, V1).wrapping_add(r32(c, V0)));
                        c.g[S4][0] = Addr(r32(c, V1)).lb(c);
                        let none = c.g[S4][0] == c.g[A0][0];
                        c.g[V0][0] = 0;
                        if none {
                            tail = 1;
                        } else {
                            c.cycles += 10;
                            c.g[S0][0] = Addr(r32(c, S2).wrapping_add(8)).lw(c);
                            c.g[A0][0] = c.g[S3][0];
                            call_ptr(c, 0x0039_a738, 0x0019_429c)?;
                            c.g[A0][0] = c.g[S1][0];
                            c.g[A1][0] = c.g[S0][0];
                            c.g[A2][0] = c.g[S4][0];
                            c.g[A3][0] = c.g[V0][0];
                            Callee {
                                addr: 0x0019_3de0,
                                f: ee::s02::f_00193de0,
                            }
                            .call(c, 0x0019_42b0)?;
                            c.g[V0][0] = 0;
                            tail = 1;
                        }
                    }
                }
                // Message 253: mark the player's menu confirmed.
                0x0019_42b8 => {
                    c.cycles += 11;
                    c.g[A0][0] = Addr(r32(c, S2).wrapping_add(8)).lw(c);
                    c.g[V0][0] = OC851_MENU_STRIDE as u64;
                    c.g[A1][0] = 1;
                    c.mult(0, 4, 4, 2);
                    c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x6d0));
                    c.g[A0][0] = sx(r32(c, S1).wrapping_add(r32(c, A0)));
                    Callee {
                        addr: 0x0019_2740,
                        f: ee::s02::f_00192740,
                    }
                    .call(c, 0x0019_42d4)?;
                    c.g[A0][0] = c.g[S1][0];
                    Callee {
                        addr: 0x0019_4080,
                        f: ee::s02::f_00194080,
                    }
                    .call(c, 0x0019_42dc)?;
                    c.g[V0][0] = 0;
                    tail = 1;
                }
                // Messages 238, 241, 248, 250, 274, 275: network back-outs.
                0x0019_42e4 => {
                    c.cycles += 4;
                    c.g[A0][0] = c.g[S1][0];
                    call_ptr(c, 0x001a_b288, 0x0019_42ec)?;
                    c.g[V0][0] = 0;
                    tail = 1;
                }
                // Message 268: switch to the chat room or the online main menu.
                0x0019_42f4 => {
                    c.cycles += 4;
                    c.g[A0][0] = Addr(r32(c, GP).wrapping_sub(0xc8)).lw(c);
                    call_ptr(c, 0x0025_da90, 0x0019_42fc)?;
                    let lobby = c.g[V0][0] != 0;
                    c.g[A1][0] = 0x0046_0000;
                    if lobby {
                        c.cycles += 10;
                        c.g[A0][0] = 0x73c;
                        c.g[A1][0] = sx(r32(c, A1).wrapping_sub(0xf0));
                        c.g[A2][0] = 0;
                        c.g[A3][0] = 0;
                        call_ptr(c, 0x0031_7d70, 0x0019_4318)?;
                        c.g[A1][0] = Addr(r32(c, S1).wrapping_add(0x10)).lw(c);
                        c.g[A0][0] = c.g[V0][0];
                        call_ptr(c, 0x001b_1f88, 0x0019_4324)?;
                        c.g[A0][0] = c.g[S1][0];
                    } else {
                        c.cycles += 10;
                        c.g[A1][0] = 0x0046_0000;
                        c.g[A0][0] = 0x6d0;
                        c.g[A1][0] = sx(r32(c, A1).wrapping_sub(0xd8));
                        c.g[A2][0] = 0;
                        c.g[A3][0] = 0;
                        call_ptr(c, 0x0031_7d70, 0x0019_4344)?;
                        c.g[A1][0] = Addr(r32(c, S1).wrapping_add(0x10)).lw(c);
                        c.g[A0][0] = c.g[V0][0];
                        call_ptr(c, 0x001b_fbb8, 0x0019_4350)?;
                        c.g[A0][0] = c.g[S1][0];
                    }
                    c.cycles += 4;
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x001a_8a40, 0x0019_435c)?;
                    c.g[V0][0] = 0;
                    tail = 1;
                }
                // Message 254: unconfirm, enable the items, play the frame.
                0x0019_4364 => {
                    c.cycles += 22;
                    c.g[A0][0] = Addr(r32(c, S2).wrapping_add(8)).lw(c);
                    c.g[S0][0] = OC851_MENU_STRIDE as u64;
                    c.g[A1][0] = 0;
                    c.mult(0, 4, 4, 16);
                    c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x6d0));
                    c.g[A0][0] = sx(r32(c, S1).wrapping_add(r32(c, A0)));
                    Callee {
                        addr: 0x0019_2740,
                        f: ee::s02::f_00192740,
                    }
                    .call(c, 0x0019_4380)?;
                    c.g[A0][0] = Addr(r32(c, S2).wrapping_add(8)).lw(c);
                    c.g[A1][0] = Addr(r32(c, S1).wrapping_add(0x40)).lw(c);
                    c.mult(0, 4, 4, 16);
                    c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x6d0));
                    c.g[A0][0] = sx(r32(c, S1).wrapping_add(r32(c, A0)));
                    Callee {
                        addr: 0x0019_2970,
                        f: ee::s02::f_00192970,
                    }
                    .call(c, 0x0019_4398)?;
                    c.g[V0][0] = Addr(r32(c, S2).wrapping_add(8)).lw(c);
                    c.g[A1][0] = 0;
                    c.mult(0, 3, 2, 16);
                    c.g[V0][0] = sx(r32(c, V1).wrapping_add(r32(c, S1)));
                    c.g[A0][0] = Addr(r32(c, V0).wrapping_add(0x6d0)).lw(c);
                    call_ptr(c, 0x0039_b960, 0x0019_43b0)?;
                    c.g[V0][0] = Addr(r32(c, S2).wrapping_add(8)).lw(c);
                    c.g[A0][0] = 0x004a_0000;
                    if c.g[V0][0] != 0 {
                        c.cycles += 1;
                        c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x17b0));
                    } else {
                        c.cycles += 3;
                        c.g[A0][0] = 0x004a_0000;
                        c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x1788));
                    }
                    c.cycles += 9;
                    call_ptr(c, 0x0031_7670, 0x0019_43d4)?;
                    c.g[A0][0] = Addr(r32(c, S1).wrapping_add(0x40)).lw(c);
                    c.g[A1][0] = c.g[V0][0];
                    call_ptr(c, 0x0039_c778, 0x0019_43e0)?;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[V0][0] = 0xffff;
                    let missing = c.g[A1][0] == c.g[V0][0];
                    c.g[A1][0] &= 0xffff;
                    if missing {
                        tail = 2;
                    } else {
                        c.cycles += 5;
                        c.g[A0][0] = Addr(r32(c, S1).wrapping_add(0x40)).lw(c);
                        c.g[A2][0] = 1;
                        call_ptr(c, 0x0039_c870, 0x0019_43fc)?;
                        c.g[V0][0] = 0;
                        tail = 1;
                    }
                }
                // Anything else (the default of the jump table): the network state.
                _ if target == 0x0019_4404 => {
                    c.cycles += 5;
                    c.g[A0][0] = c.g[S1][0];
                    c.g[A2][0] = c.g[S2][0];
                    call_ptr(c, 0x001a_8e40, 0x0019_4410)?;
                    frame.restore(c, 16, 0x60);
                    tail = 0;
                }
                _ => {
                    c.jump_out(target, 0x0019_41b0, 0x0019_443c);
                    return Err(crate::call::Unwound);
                }
            }
            if tail >= 2 {
                c.cycles += 1;
                c.g[V0][0] = 0;
            }
            if tail >= 1 {
                c.cycles += 1;
                frame.restore(c, 16, 0x60);
            }
            c.cycles += 7;
            frame.restore(c, 17, 0x50);
            frame.restore(c, 18, 0x40);
            frame.restore(c, 19, 0x30);
            frame.restore(c, 20, 0x20);
            frame.restore_ra(c, 0x10);
            frame.close(c);
            return Ok(());
        }
    })
}

/// One "show widget" step of `cFEStateMainMenu_OnChildEvent`: look up the
/// string at `hi + lo` (`0x317670`), find the child widget of the state's
/// screen (`0x39d860`), and when found call its vtable slot at +0x4c with
/// `a1 = 1` (show).
fn oc851_show_widget(c: &mut Cpu, hi: u32, lo: u32, ras: [u32; 3]) -> Flow {
    c.cycles += 9;
    c.g[A0][0] = hi as u64;
    c.g[A0][0] = sx(r32(c, A0).wrapping_add(lo));
    call_ptr(c, 0x0031_7670, ras[0])?;
    c.g[A0][0] = Addr(r32(c, S0).wrapping_add(0x40)).lw(c);
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, 0x0039_d860, ras[1])?;
    c.g[A2][0] = c.g[V0][0];
    let found = c.g[A2][0] != 0;
    c.g[A1][0] = 1;
    if found {
        c.cycles += 5;
        c.g[V0][0] = Addr(r32(c, A2).wrapping_add(8)).lw(c);
        c.g[A0][0] = Addr(r32(c, V0).wrapping_add(0x48)).lh(c);
        c.g[V1][0] = Addr(r32(c, V0).wrapping_add(0x4c)).lw(c);
        let t = r32(c, V1);
        c.g[A0][0] = sx(r32(c, A2).wrapping_add(r32(c, A0)));
        call_ptr(c, t, ras[2])?;
    }
    Ok(())
}

/// `void cFEStateMainMenu_onWidgetEvent(self, obj, int msg)` (0x195328):
/// message 0x15 does nothing; message 0x16 asks the widget (vtable +0xb8)
/// and, if it answers, copies the saved control (+0x50 into +0x14), sets
/// `mGoToNextState` (+0x48) and tells the multi-mode widget (+0x4c) to play
/// keyframe 2 and gets event 5; then it shows four widgets.
pub fn main_menu_on_child_event(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x20);
        c.g[A3][0] = c.g[A1][0];
        frame.save(c, 16, 0x10);
        c.g[V0][0] = 0x15;
        frame.save_ra(c, 0);
        let is_15 = c.g[A2][0] == c.g[V0][0];
        c.g[S0][0] = c.g[A0][0];
        if is_15 {
            c.cycles += 1;
            frame.restore(c, 16, 0x10);
        } else {
            c.cycles += 3;
            c.g[V0][0] = 0x16;
            if c.g[A2][0] != c.g[V0][0] {
                frame.restore(c, 16, 0x10);
            } else {
                c.cycles += 8;
                c.g[V1][0] = Addr(r32(c, A3).wrapping_add(8)).lw(c);
                c.g[A1][0] = 0;
                c.g[A0][0] = Addr(r32(c, V1).wrapping_add(0xb8)).lh(c);
                c.g[V0][0] = Addr(r32(c, V1).wrapping_add(0xbc)).lw(c);
                let t = r32(c, V0);
                c.g[A0][0] = sx(r32(c, A3).wrapping_add(r32(c, A0)));
                call_ptr(c, t, 0x0019_5368)?;
                let none = c.g[V0][0] == 0;
                c.g[V0][0] = 1;
                if !none {
                    c.cycles += 13;
                    c.g[V1][0] = Addr(r32(c, S0).wrapping_add(0x50)).lb(c);
                    c.g[A0][0] = Addr(r32(c, S0).wrapping_add(0x4c)).lw(c);
                    c.g[A1][0] = 2;
                    let saved = c.g[V1][0];
                    Addr(r32(c, S0).wrapping_add(0x14)).sb(c, saved);
                    let one = c.g[V0][0];
                    Addr(r32(c, S0).wrapping_add(0x48)).sw(c, one);
                    call_ptr(c, 0x0039_ffa0, 0x0019_5388)?;
                    c.g[V1][0] = Addr(r32(c, S0).wrapping_add(8)).lw(c);
                    c.g[A2][0] = 5;
                    c.g[A1][0] = Addr(r32(c, S0).wrapping_add(0x4c)).lw(c);
                    c.g[A0][0] = Addr(r32(c, V1).wrapping_add(0x98)).lh(c);
                    c.g[V0][0] = Addr(r32(c, V1).wrapping_add(0x9c)).lw(c);
                    let t = r32(c, V0);
                    c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, A0)));
                    call_ptr(c, t, 0x0019_53a4)?;
                }
                oc851_show_widget(
                    c,
                    0x4a_0000,
                    0x17e0,
                    [0x0019_53b0, 0x0019_53bc, 0x0019_53dc],
                )?;
                oc851_show_widget(c, 0x46_0000, 0xa0, [0x0019_53e8, 0x0019_53f4, 0x0019_5414])?;
                oc851_show_widget(
                    c,
                    0x4a_0000,
                    0x1458,
                    [0x0019_5420, 0x0019_542c, 0x0019_544c],
                )?;
                oc851_show_widget(
                    c,
                    0x4a_0000,
                    0x1460,
                    [0x0019_5458, 0x0019_5464, 0x0019_5484],
                )?;
                c.cycles += 1;
                frame.restore(c, 16, 0x10);
            }
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void cFEStateBonusMaterial_onGainFocus(self)` (0x1955e0): forwards to
/// `UIState_OnActivate` (0x39e4c0).
pub fn bonus_material_on_activate(c: &mut Cpu) {
    crate::call::tail_wrapper(c, None, Target::Table(0x0039_e4c0), 0x0019_55f0);
}

/// `void cFEStateBonusMaterial_onWidgetAction(self, item, int event)`
/// (0x195600): with an item, event 5 opens the video player popup for the
/// item's clip (`cFEStateVideoPlayer` ctor 0x1d22f0, source files 0x1d2598,
/// push popup 0x39f290); event 6 asks the engine's custom interface for the
/// next state and pushes it (0x39f400).
pub fn bonus_material_on_widget_action(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x30);
        frame.save(c, 16, 0x20);
        frame.save(c, 17, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save_ra(c, 0);
        let no_item = c.g[S0][0] == 0;
        c.g[S1][0] = c.g[A0][0];
        // 0: leave through the epilogue after `lq s0`, 1: with the extra cycle.
        let mut extra_cycle = false;
        if no_item {
            extra_cycle = true;
        } else {
            c.cycles += 3;
            c.g[V0][0] = 5;
            let is_5 = c.g[A2][0] == c.g[V0][0];
            c.g[V0][0] = (c.g[A2][0] < 6) as u64;
            if is_5 {
                oc851_open_video_popup(c)?;
            } else {
                c.cycles += 2;
                if c.g[V0][0] != 0 {
                    frame.restore(c, 16, 0x20);
                } else {
                    c.cycles += 3;
                    c.g[V0][0] = 6;
                    if c.g[A2][0] == c.g[V0][0] {
                        c.g[V0][0] = Addr(r32(c, S1).wrapping_add(0x10)).lw(c);
                        c.cycles += 10;
                        c.g[A1][0] = c.g[S1][0];
                        c.g[A2][0] = Addr(r32(c, S0).wrapping_add(0x18)).lw(c);
                        c.g[A3][0] = Addr(r32(c, V0)).lw(c);
                        c.g[V1][0] = Addr(r32(c, A3).wrapping_add(4)).lw(c);
                        c.g[A0][0] = Addr(r32(c, V1).wrapping_add(0x28)).lh(c);
                        c.g[V0][0] = Addr(r32(c, V1).wrapping_add(0x2c)).lw(c);
                        let t = r32(c, V0);
                        c.g[A0][0] = sx(r32(c, A3).wrapping_add(r32(c, A0)));
                        call_ptr(c, t, 0x0019_56d0)?;
                        let none = c.g[V0][0] == 0;
                        c.g[A1][0] = c.g[V0][0];
                        if !none {
                            c.cycles += 3;
                            c.g[A0][0] = Addr(r32(c, S1).wrapping_add(0x10)).lw(c);
                            c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x18));
                            call_ptr(c, 0x0039_f400, 0x0019_56e4)?;
                        }
                        extra_cycle = true;
                    } else {
                        c.cycles += 2;
                        frame.restore(c, 16, 0x20);
                    }
                }
            }
        }
        if extra_cycle {
            c.cycles += 1;
            frame.restore(c, 16, 0x20);
        }
        c.cycles += 4;
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// Event 5 of `bonus_material_on_widget_action`: build a `cFEStateVideoPlayer`
/// popup for the clip table entry of the item (`0x440000 + 0x1128`, indexed by
/// the item's +0x18) and push it. Ends with `lq s0` (the caller's epilogue).
fn oc851_open_video_popup(c: &mut Cpu) -> Flow {
    c.cycles += 27;
    c.g[A1][0] = 0x0046_0000;
    c.g[A0][0] = 0x280;
    c.g[A1][0] = sx(r32(c, A1).wrapping_add(0x1d8));
    c.g[A2][0] = 0;
    c.g[A3][0] = 0;
    call_ptr(c, 0x0031_7d70, 0x0019_565c)?;
    c.g[A3][0] = Addr(r32(c, S1).wrapping_add(0x44)).lb(c);
    c.g[A0][0] = c.g[V0][0];
    c.g[A1][0] = Addr(r32(c, S1).wrapping_add(0x10)).lw(c);
    c.g[A2][0] = c.g[S1][0];
    call_ptr(c, 0x001d_22f0, 0x0019_5670)?;
    c.g[A3][0] = Addr(r32(c, S0).wrapping_add(0x18)).lw(c);
    c.g[V1][0] = 0x0044_0000;
    c.g[V1][0] = sx(r32(c, V1).wrapping_add(0x1128));
    c.g[S0][0] = c.g[V0][0];
    c.g[A3][0] = sx(r32(c, A3) << 2);
    c.g[A0][0] = c.g[S0][0];
    c.g[A3][0] = sx(r32(c, A3).wrapping_add(r32(c, V1)));
    c.g[A2][0] = 0;
    c.g[A1][0] = Addr(r32(c, A3)).lw(c);
    call_ptr(c, 0x001d_2598, 0x0019_5698)?;
    c.g[A0][0] = Addr(r32(c, S1).wrapping_add(0x10)).lw(c);
    c.g[A1][0] = c.g[S0][0];
    c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x18));
    call_ptr(c, 0x0039_f290, 0x0019_56a8)?;
    let frame = Frame { size: 0x30 };
    frame.restore(c, 16, 0x20);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine, FAKE};
    use crate::difftest::{check, check_lohi, Rng, DATA, GAME_GP};
    use std::sync::atomic::AtomicUsize;

    /// Gate of the oc851 fakes (a magic word in the machine's RAM) and the
    /// mailbox of forced results, one word per callee at `OC851_MAIL + (addr & 0xffc)`.
    pub(crate) const OC851_CTRL: u32 = DATA + 0x20_0000;

    pub(crate) const OC851_MAIL: u32 = OC851_CTRL + 0x100;

    pub(crate) const OC851_MAGIC: u32 = 0x4f43_3835;

    /// Fixture objects (clear of the fakes' result pointers at `DATA..DATA+0x40000`).
    pub(crate) const OC851_SELF: u32 = DATA + 0x8_1000;

    pub(crate) const OC851_VT_SELF: u32 = DATA + 0x8_2000;

    pub(crate) const OC851_OBJ2: u32 = DATA + 0x8_3000;

    pub(crate) const OC851_VT_OBJ2: u32 = DATA + 0x8_3800;

    pub(crate) const OC851_MAP: u32 = DATA + 0x8_4000;

    pub(crate) const OC851_LISTBOX: u32 = DATA + 0x8_5000;

    pub(crate) const OC851_IFACE: u32 = DATA + 0x8_6000;

    pub(crate) const OC851_IFACE_VT: u32 = DATA + 0x8_6800;

    /// `leaf` returning 0 (a real function with no callees).
    const OC851_ZERO_LEAF: u32 = 0x0014_06c0;

    /// A forced result for a callee: 0 default pointer, 1 return 0, 2 return
    /// 0xffff, 3 return 1.
    pub(crate) fn oc851_mail(c: &mut Cpu, addr: u32, v: u32) {
        c.bus.write32(OC851_MAIL + (addr & 0xffc), v);
    }

    /// The fake callee: logs like `fakes::fake_call`, then applies the mailbox.
    pub(crate) fn oc851_fake_call(c: &mut Cpu, addr: u32) {
        let ra = c.g[RA][0] as u32;
        let mail = c.bus.read32(OC851_MAIL + (addr & 0xffc));
        fakes::fake_call(c, addr);
        if c.pc == ra {
            match mail {
                1 => c.g[V0][0] = 0,
                2 => c.g[V0][0] = 0xffff,
                3 => c.g[V0][0] = 1,
                _ => {}
            }
        }
    }

    macro_rules! oc851_fakes {
        ($(($fake:ident, $prev:ident, $slot:expr, $gen:expr, $addr:expr)),* $(,)?) => {
            $(
                static $prev: AtomicUsize = AtomicUsize::new(0);
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC851_CTRL) != OC851_MAGIC {
                        return fakes::fall_back(c, &$prev, $gen);
                    }
                    oc851_fake_call(c, $addr);
                }
            )*
            pub(crate) fn oc851_install_fakes() {
                use std::ptr::addr_of_mut;
                $( fakes::hook(addr_of_mut!($slot), $fake, &$prev); )*
            }
        };
    }

    oc851_fakes!(
        (
            oc851_fake_194440,
            OC851_PREV_194440,
            ee::s02::OVERRIDE_00194440,
            ee::s02::f_00194440__generated,
            0x0019_4440
        ),
        (
            oc851_fake_192740,
            OC851_PREV_192740,
            ee::s02::OVERRIDE_00192740,
            ee::s02::f_00192740__generated,
            0x0019_2740
        ),
        (
            oc851_fake_194080,
            OC851_PREV_194080,
            ee::s02::OVERRIDE_00194080,
            ee::s02::f_00194080__generated,
            0x0019_4080
        ),
        (
            oc851_fake_39b960,
            OC851_PREV_39B960,
            ee::s13::OVERRIDE_0039B960,
            ee::s13::f_0039b960__generated,
            0x0039_b960
        ),
        (
            oc851_fake_39a768,
            OC851_PREV_39A768,
            ee::s13::OVERRIDE_0039A768,
            ee::s13::f_0039a768__generated,
            0x0039_a768
        ),
        (
            oc851_fake_39a7a8,
            OC851_PREV_39A7A8,
            ee::s13::OVERRIDE_0039A7A8,
            ee::s13::f_0039a7a8__generated,
            0x0039_a7a8
        ),
        (
            oc851_fake_39a738,
            OC851_PREV_39A738,
            ee::s13::OVERRIDE_0039A738,
            ee::s13::f_0039a738__generated,
            0x0039_a738
        ),
        (
            oc851_fake_39c778,
            OC851_PREV_39C778,
            ee::s13::OVERRIDE_0039C778,
            ee::s13::f_0039c778__generated,
            0x0039_c778
        ),
        (
            oc851_fake_39c870,
            OC851_PREV_39C870,
            ee::s13::OVERRIDE_0039C870,
            ee::s13::f_0039c870__generated,
            0x0039_c870
        ),
        (
            oc851_fake_39d860,
            OC851_PREV_39D860,
            ee::s13::OVERRIDE_0039D860,
            ee::s13::f_0039d860__generated,
            0x0039_d860
        ),
        (
            oc851_fake_39e4c0,
            OC851_PREV_39E4C0,
            ee::s13::OVERRIDE_0039E4C0,
            ee::s13::f_0039e4c0__generated,
            0x0039_e4c0
        ),
        (
            oc851_fake_39f400,
            OC851_PREV_39F400,
            ee::s13::OVERRIDE_0039F400,
            ee::s13::f_0039f400__generated,
            0x0039_f400
        ),
        (
            oc851_fake_25da90,
            OC851_PREV_25DA90,
            ee::s06::OVERRIDE_0025DA90,
            ee::s06::f_0025da90__generated,
            0x0025_da90
        ),
        (
            oc851_fake_317670,
            OC851_PREV_317670,
            ee::s10::OVERRIDE_00317670,
            ee::s10::f_00317670__generated,
            0x0031_7670
        ),
        (
            oc851_fake_317d70,
            OC851_PREV_317D70,
            ee::s10::OVERRIDE_00317D70,
            ee::s10::f_00317d70__generated,
            0x0031_7d70
        ),
        (
            oc851_fake_1d22f0,
            OC851_PREV_1D22F0,
            ee::s04::OVERRIDE_001D22F0,
            ee::s04::f_001d22f0__generated,
            0x001d_22f0
        ),
        (
            oc851_fake_1d2598,
            OC851_PREV_1D2598,
            ee::s04::OVERRIDE_001D2598,
            ee::s04::f_001d2598__generated,
            0x001d_2598
        ),
        (
            oc851_fake_1bfbb8,
            OC851_PREV_1BFBB8,
            ee::s03::OVERRIDE_001BFBB8,
            ee::s03::f_001bfbb8__generated,
            0x001b_fbb8
        ),
    );

    /// The machine with the fakes switched on and random mailbox answers for
    /// the callees whose results the ports branch on.
    pub(crate) fn oc851_machine(c: &mut Cpu, seed: u32, salt: u64, calls: u32) -> Rng {
        oc851_install_fakes();
        let mut r = machine(c, seed, salt, calls);
        c.bus.write32(OC851_CTRL, OC851_MAGIC);
        for addr in [
            0x0039_a768u32,
            0x0025_da90,
            0x0039_c778,
            0x0039_d860,
            0x0031_7670,
        ] {
            let v = r.next_u32() % 4;
            oc851_mail(c, addr, v);
        }
        r
    }

    /// A vtable at `vt` whose slot pairs `off`/`off+4` hold `{delta, fake i}`.
    pub(crate) fn oc851_vtable(c: &mut Cpu, r: &mut Rng, vt: u32, slots: &[u32]) {
        for (i, &off) in slots.iter().enumerate() {
            let delta = (r.next_u32() as i16 as i32 & !3) as u32;
            c.bus.write16(vt + off, delta as u16);
            c.bus.write32(vt + off + 4, FAKE + 0x10 * i as u32);
        }
    }

    /// `cFEStateBraggingRights_v24`: flag off returns 0, flag on forwards to
    /// the net state's handler (real; out-of-range message ends in a vtable call).
    #[test]
    fn bragging_rights_v24_matches() {
        fakes::covered(
            ee::s02::f_00194138__generated,
            bragging_rights_v24,
            120,
            &|c, seed| {
                let mut r = oc851_machine(c, seed, 0x194138, 1);
                oc851_vtable(c, &mut r, OC851_VT_SELF, &[0xf8]);
                c.bus.write32(OC851_SELF + 8, OC851_VT_SELF);
                c.bus.write32(
                    GAME_GP - OC851_ONLINE_FLAG,
                    [1, 1, 0, 5, 1][seed as usize % 5],
                );
                c.g[A0][0] = sx(OC851_SELF);
                c.g[A1][0] = [0u64, 0x10, 0x1000][seed as usize % 3];
                c.pc = 0x0019_4138;
            },
        );
    }

    /// `cFEStateBraggingRights_OnChildEvent`: flag off does nothing, popup
    /// 0x838 goes to OnChildEventQuit (fake), any other to the net state's popup handler.
    #[test]
    fn bragging_rights_on_child_event_matches() {
        check(
            ee::s02::f_00194168__generated,
            bragging_rights_on_child_event,
            120,
            &|c, seed| {
                let _ = oc851_machine(c, seed, 0x194168, 2);
                c.bus
                    .write32(GAME_GP - OC851_ONLINE_FLAG, [1, 1, 0, 1][seed as usize % 4]);
                c.bus.write32(OC851_SELF + 0x69c, 0);
                c.bus
                    .write32(OC851_OBJ2 + 0x18, [0x838, 0x500, 0x838][seed as usize % 3]);
                c.g[A0][0] = sx(OC851_SELF);
                c.g[A1][0] = sx(OC851_OBJ2);
                c.g[A2][0] = (seed % 17) as u64;
                c.pc = 0x0019_4168;
            },
        );
    }

    /// The message-switch jump table at 0x45ff40 (index = message - 0xee).
    fn oc851_v31_table(c: &mut Cpu, msg: u32) {
        let mut t = [0x0019_4404u32; 0x26];
        for i in [0usize, 3, 10, 12, 36, 37] {
            t[i] = 0x0019_42e4;
        }
        t[0xfb - 0xee] = 0x0019_41fc;
        t[0xfc - 0xee] = 0x0019_4220;
        t[0xfd - 0xee] = 0x0019_42b8;
        t[0xfe - 0xee] = 0x0019_4364;
        t[0x10c - 0xee] = 0x0019_42f4;
        // A table entry outside the function: the `jump_out` exit.
        t[0x105 - 0xee] = 0x0040_0000;
        let _ = msg;
        for (i, v) in t.iter().enumerate() {
            c.bus.write32(0x0045_ff40 + 4 * i as u32, *v);
        }
    }

    /// `cFEStateBraggingRights_v31`: every message of the switch, the default
    /// (net state handler), out-of-range messages and a table entry outside
    /// the function. Not covered: a table entry equal to the function start
    /// (a restart that never ends in the test machine).
    #[test]
    fn bragging_rights_v31_matches() {
        const MSGS: [u32; 20] = [
            251, 252, 252, 252, 253, 254, 254, 268, 268, 238, 241, 248, 250, 274, 275, 0x100, 0x50,
            0x2000, 0x105, 252,
        ];
        check_lohi(
            ee::s02::f_001941b0__generated,
            bragging_rights_v31,
            400,
            &|c, seed| {
                let mut r = oc851_machine(c, seed, 0x1941b0, 3);
                let msg = MSGS[seed as usize % MSGS.len()];
                oc851_v31_table(c, msg);
                c.bus.write32(GAME_GP - 0xc8, DATA + 0x1_0000);
                c.bus.write32(OC851_SELF + 0x10, OC851_IFACE);
                c.bus.write32(OC851_SELF + 0x40, DATA + 0x2_0000);
                c.bus.write32(OC851_SELF + 0x738, OC851_MAP);
                for p in 0..2u32 {
                    let menu = OC851_SELF + 0x6d0 + p * OC851_MENU_STRIDE;
                    c.bus.write32(menu, DATA + 0x3_0000 + p * 0x100);
                    for k in 0..8u32 {
                        c.bus
                            .write32(menu + 8 + 4 * k, OC851_LISTBOX + (p * 8 + k) * 0x100);
                    }
                }
                for k in 0..16u32 {
                    let b = if r.next_u32() % 3 == 0 {
                        0xff
                    } else {
                        (k % 8) as u8
                    };
                    c.bus.write8(OC851_MAP + k, b);
                }
                let panel = r.next_u32() % 2;
                c.bus.write32(OC851_OBJ2 + 8, panel);
                c.bus.write32(OC851_OBJ2 + 0xc, r.next_u32() % 9);
                c.bus.write32(OC851_OBJ2 + 0x10, r.next_u32());
                c.g[A0][0] = sx(OC851_SELF);
                c.g[A1][0] = sx(msg);
                c.g[A2][0] = sx(OC851_OBJ2);
                c.pc = 0x0019_41b0;
            },
        );
    }

    /// `cFEStateMainMenu_OnChildEvent`: messages 0x15 and 0x16 (with and
    /// without a widget answer), anything else; the four show-widget steps.
    #[test]
    fn main_menu_on_child_event_matches() {
        check(
            ee::s02::f_00195328__generated,
            main_menu_on_child_event,
            200,
            &|c, seed| {
                let mut r = oc851_machine(c, seed, 0x195328, 4);
                oc851_vtable(c, &mut r, OC851_VT_SELF, &[0x98]);
                c.bus.write32(OC851_SELF + 8, OC851_VT_SELF);
                c.bus.write32(OC851_SELF + 0x4c, DATA + 0x4_0000);
                c.bus.write8(OC851_SELF + 0x50, r.next_u32() as u8);
                // The widget object: slot 0xb8 is a fake, or a real leaf returning 0.
                oc851_vtable(c, &mut r, OC851_VT_OBJ2, &[0xb8]);
                if seed % 3 == 0 {
                    c.bus.write32(OC851_VT_OBJ2 + 0xbc, OC851_ZERO_LEAF);
                }
                c.bus.write32(OC851_OBJ2 + 8, OC851_VT_OBJ2);
                c.g[A0][0] = sx(OC851_SELF);
                c.g[A1][0] = sx(OC851_OBJ2);
                c.g[A2][0] = [0x15u64, 0x16, 0x16, 0x16, 0x17, 0][seed as usize % 6];
                c.pc = 0x0019_5328;
            },
        );
    }

    /// `cFEStateBonusMaterial_OnActivate`: forwards to `UIState_OnActivate`.
    #[test]
    fn bonus_material_on_activate_matches() {
        check(
            ee::s02::f_001955e0__generated,
            bonus_material_on_activate,
            40,
            &|c, seed| {
                let _ = oc851_machine(c, seed, 0x1955e0, 1);
                c.g[A0][0] = sx(OC851_SELF);
                c.pc = 0x0019_55e0;
            },
        );
    }

    /// `cFEStateBonusMaterial_OnWidgetAction`: no item, events 4 and 7 (nothing),
    /// 5 (video popup), 6 (next state, with and without an answer).
    #[test]
    fn bonus_material_on_widget_action_matches() {
        check(
            ee::s02::f_00195600__generated,
            bonus_material_on_widget_action,
            240,
            &|c, seed| {
                let mut r = oc851_machine(c, seed, 0x195600, 4);
                c.bus.write32(OC851_SELF + 0x10, OC851_IFACE);
                c.bus.write8(OC851_SELF + 0x44, r.next_u32() as u8);
                // The engine's custom interface: slot 0x28/0x2c of the table at +4.
                c.bus.write32(OC851_IFACE, OC851_IFACE);
                c.bus.write32(OC851_IFACE + 4, OC851_IFACE_VT);
                c.bus.write16(OC851_IFACE_VT + 0x28, 0);
                let target = if seed % 3 == 0 { OC851_ZERO_LEAF } else { FAKE };
                c.bus.write32(OC851_IFACE_VT + 0x2c, target);
                c.bus.write32(OC851_OBJ2 + 0x18, r.next_u32() % 4);
                c.g[A0][0] = sx(OC851_SELF);
                c.g[A1][0] = if seed % 7 == 0 { 0 } else { OC851_OBJ2 as u64 };
                c.g[A2][0] = [5u64, 6, 6, 4, 7, 0x10, 5][seed as usize % 7];
                c.pc = 0x0019_5600;
            },
        );
    }
}
