//! Staging module oc921 (wave wave254): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc921`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame, Unwound};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x001b_ea38,
        c_fe_state_net_create_room_popup_on_widget_action,
        "cFEStateNetCreateRoomPopup_OnWidgetAction",
    ),
    (
        0x001b_ef60,
        c_fe_state_login_on_widget_created,
        "cFEStateLogin_OnWidgetCreated",
    ),
    (
        0x001b_f758,
        c_fe_state_login_on_widget_action,
        "cFEStateLogin_OnWidgetAction",
    ),
    (
        0x001c_0540,
        c_fe_state_online_agreement_load_terms_text,
        "cFEStateOnlineAgreement_LoadTermsText",
    ),
    (
        0x001c_1080,
        c_fe_state_net_quick_play_popup_on_widget_created,
        "cFEStateNetQuickPlayPopup_OnWidgetCreated",
    ),
];

// ---------------------------------------------------------------------------
// cFEStateNetCreateRoomPopup_OnWidgetAction.

pub(crate) const NET_CREATE_ROOM_KEYBOARD: u32 = 0x69c;
pub(crate) const NET_CREATE_ROOM_PARENT: u32 = 0x20;

/// Epilogue shared by every path of `cFEStateNetCreateRoomPopup_OnWidgetAction`:
/// restore `s0` (the load sits in the delay slot of the last branch), then
/// `s1`, `s2`, `ra` and pop the 0x40 frame.
fn net_create_room_exit(c: &mut Cpu, frame: Frame) -> Flow {
    frame.restore(c, S0, 0x30);
    c.cycles += 5;
    frame.restore(c, S1, 0x20);
    frame.restore(c, S2, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `parent->onPopupEvent(this, event)`: `a3` is the parent state, the method is
/// vtable slot +0x88/+0x8c of the object behind `*(parent + 8)`.
fn net_create_room_notify_parent(c: &mut Cpu) -> Flow {
    c.cycles += 7;
    let parent = r32(c, A3);
    c.g[V0][0] = c.lw(parent.wrapping_add(0x8));
    c.g[A0][0] = c.lh(r32(c, V0).wrapping_add(0x88));
    c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(0x8c));
    let method = r32(c, V1);
    c.g[A0][0] = sx(parent.wrapping_add(r32(c, A0)));
    call_ptr(c, method, 0x001b_eb7c)
}

/// The text-entry tail (`NetFEState_OpenKeyboardTextId`, then restrict the
/// keyboard's keys): `a0..a3`, `t0`, `t1` are the arguments for the first call.
fn net_create_room_open_keyboard(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 9;
    c.g[T2][0] = 0x8;
    Callee {
        addr: 0x001a_9150,
        f: ee::s03::NetFEState_OpenKeyboardTextId,
    }
    .call(c, 0x001b_eb24)?;
    c.g[A0][0] = c.lw(r32(c, S0).wrapping_add(NET_CREATE_ROOM_KEYBOARD));
    c.g[A1][0] = 0x35;
    c.g[A2][0] = 0;
    c.g[A3][0] = 0x2;
    call_ptr(c, 0x001c_e3c8, 0x001b_eb38)?;
    net_create_room_exit(c, frame)
}

/// Fall through to the base state's `NetFEState_OnWidgetAction`
/// (`a0 = this`, `a1 = widget` are set by the caller).
fn net_create_room_forward(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 2;
    c.g[A2][0] = c.g[S1][0];
    Callee {
        addr: 0x001a_8918,
        f: ee::s03::NetFEState_OnWidgetAction,
    }
    .call(c, 0x001b_ec60)?;
    c.cycles += 1;
    net_create_room_exit(c, frame)
}

/// `void cFEStateNetCreateRoomPopup_OnWidgetAction(this, widget, msg)` (0x1bea38):
/// the create-room popup's widget handler. msg 5 (button): misc data 1/2 open
/// the keyboard for the room name / password, 3 validates the name and tells
/// the parent state "created" (0xf); msg 6 tells it "cancelled" (0x14); msg 9
/// (channel list changed) copies the picked channel name; msg 13 (keyboard
/// done) copies the edited string into the room name / password and the text
/// widget. Anything else goes to the base state.
pub fn c_fe_state_net_create_room_popup_on_widget_action(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x40);
        c.g[V0][0] = 0x6;
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S2, 0x10);
        c.g[S1][0] = c.g[A2][0];
        frame.save_ra(c, 0);
        c.g[S2][0] = c.g[A1][0];
        let msg = c.g[S1][0];
        if msg == 6 {
            c.cycles += 3;
            c.g[A3][0] = c.lw(r32(c, S0).wrapping_add(NET_CREATE_ROOM_PARENT));
            c.g[A1][0] = c.g[S0][0];
            c.g[A2][0] = 0x14;
            net_create_room_notify_parent(c)?;
            return net_create_room_exit(c, frame);
        }
        c.cycles += 3;
        c.g[V0][0] = (msg < 7) as u64;
        let below_7 = c.g[V0][0] != 0;
        c.g[V0][0] = 0x5;
        if below_7 {
            c.cycles += 2;
            c.g[A0][0] = c.g[S0][0];
            if msg == 5 {
                return net_create_room_button(c, frame);
            }
            c.cycles += 2;
            c.g[A1][0] = c.g[S2][0];
            return net_create_room_forward(c, frame);
        }
        c.cycles += 3;
        c.g[V0][0] = 0x9;
        let is_list = msg == c.g[V0][0];
        c.g[V0][0] = 0xd;
        if is_list {
            return net_create_room_channel_picked(c, frame);
        }
        c.cycles += 2;
        c.g[A0][0] = c.g[S0][0];
        if msg == 0xd {
            return net_create_room_keyboard_done(c, frame);
        }
        c.cycles += 2;
        c.g[A1][0] = c.g[S2][0];
        net_create_room_forward(c, frame)
    });
}

/// msg 5: dispatch on the button's misc data (`widget + 0x18`).
fn net_create_room_button(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 4;
    c.g[A0][0] = c.lw(r32(c, S2).wrapping_add(0x18));
    c.g[V0][0] = 0x2;
    let misc = c.g[A0][0];
    c.g[V0][0] = ((misc as i64) < 3) as u64;
    if misc == 2 {
        // Password: keyboard with the password text widget.
        c.cycles += 9;
        c.g[A0][0] = 0x460000;
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x5260));
        call_ptr(c, 0x0031_7670, 0x001b_eb04)?;
        c.g[A1][0] = c.lw(r32(c, S0).wrapping_add(0x6d4));
        c.g[A2][0] = c.g[V0][0];
        c.g[A3][0] = c.lw(r32(c, S0).wrapping_add(0x6e4));
        c.g[A0][0] = c.g[S0][0];
        c.g[T0][0] = 0;
        c.g[T1][0] = 0x2;
        return net_create_room_open_keyboard(c, frame);
    }
    c.cycles += 2;
    let misc_ge_3 = c.g[V0][0] == 0;
    c.g[V0][0] = 0x1;
    if misc_ge_3 {
        c.cycles += 3;
        c.g[V0][0] = 0x3;
        if misc == 3 {
            // Create: validate the room name, then tell the parent.
            c.cycles += 4;
            c.g[A0][0] = c.g[S0][0];
            Callee {
                addr: 0x001b_ec78,
                f: ee::s03::cFEStateNetCreateRoomPopup_ValidateRoomName,
            }
            .call(c, 0x001b_eb48)?;
            let invalid = c.g[V0][0] == 0;
            c.g[A1][0] = c.g[S0][0];
            if invalid {
                c.cycles += 1;
                return net_create_room_exit(c, frame);
            }
            c.cycles += 3;
            c.g[A3][0] = c.lw(r32(c, S0).wrapping_add(NET_CREATE_ROOM_PARENT));
            c.g[A2][0] = 0xf;
            net_create_room_notify_parent(c)?;
            return net_create_room_exit(c, frame);
        }
        c.cycles += 2;
        return net_create_room_exit(c, frame);
    }
    c.cycles += 2;
    let is_name = misc == c.g[V0][0];
    c.g[A0][0] = 0x460000;
    if is_name {
        // Room name: keyboard with the room name text widget.
        c.cycles += 9;
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x5248));
        call_ptr(c, 0x0031_7670, 0x001b_eadc)?;
        c.g[A1][0] = c.lw(r32(c, S0).wrapping_add(0x6d0));
        c.g[A2][0] = c.g[V0][0];
        c.g[A3][0] = c.lw(r32(c, S0).wrapping_add(0x6dc));
        c.g[A0][0] = c.g[S0][0];
        c.g[T0][0] = 0;
        c.g[T1][0] = 0x1;
        return net_create_room_open_keyboard(c, frame);
    }
    c.cycles += 2;
    net_create_room_exit(c, frame)
}

/// msg 9: the channel list widget (`this + 0x6d8`) changed; copy the picked
/// entry's name string (globals `gp - 0x1690/-0x168c/-0x1684/-0x1688`) into
/// the channel name (`this + 0x6e0`).
fn net_create_room_channel_picked(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 3;
    c.g[V0][0] = c.lw(r32(c, S0).wrapping_add(0x6d8));
    if c.g[S2][0] != c.g[V0][0] {
        return net_create_room_exit(c, frame);
    }
    c.cycles += 6;
    c.g[A0][0] = c.g[S2][0];
    call_ptr(c, 0x0039_a738, 0x001b_eb98)?;
    c.g[V1][0] = c.g[V0][0];
    c.g[V0][0] = 0x1;
    let entry = c.g[V1][0];
    let is_1 = entry == c.g[V0][0];
    c.g[V0][0] = ((entry as i64) < 2) as u64;
    let gp = r32(c, GP);
    if is_1 {
        c.cycles += 2;
        c.g[A1][0] = c.lw(gp.wrapping_sub(0x168c));
    } else {
        c.cycles += 2;
        if c.g[V0][0] == 0 {
            c.g[V0][0] = 0x2;
            c.cycles += 2;
            let is_2 = entry == c.g[V0][0];
            c.g[V0][0] = 0x3;
            if is_2 {
                c.cycles += 1;
                c.g[A1][0] = c.lw(gp.wrapping_sub(0x1684));
            } else {
                c.cycles += 2;
                let is_3 = entry == c.g[V0][0];
                c.g[A1][0] = c.lw(gp.wrapping_sub(0x1688));
                if !is_3 {
                    c.cycles += 2;
                    return net_create_room_exit(c, frame);
                }
            }
        } else {
            c.cycles += 2;
            let is_0 = entry == 0;
            c.g[A1][0] = c.lw(gp.wrapping_sub(0x1690));
            if !is_0 {
                c.cycles += 2;
                return net_create_room_exit(c, frame);
            }
        }
    }
    c.cycles += 4;
    c.g[A0][0] = sx(r32(c, S0).wrapping_add(0x6e0));
    call_ptr(c, 0x0031_84e8, 0x001b_ebec)?;
    net_create_room_exit(c, frame)
}

/// msg 13: the keyboard popup (`this + 0x69c`) finished; copy its edit string
/// (`keyboard + 0x74`) into the room name (`+0x6dc`) or password (`+0x6e4`)
/// and set the matching text widget (`+0x6d0` / `+0x6d4`).
fn net_create_room_keyboard_done(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 5;
    c.g[A1][0] = c.lw(r32(c, S0).wrapping_add(NET_CREATE_ROOM_KEYBOARD));
    c.g[V0][0] = 0x1;
    c.g[V1][0] = c.lw(r32(c, A1).wrapping_add(0x18));
    let misc = c.g[V1][0];
    let is_name = misc == c.g[V0][0];
    c.g[V0][0] = 0x2;
    if is_name {
        c.cycles += 8;
        c.g[A1][0] = sx(r32(c, A1).wrapping_add(0x74));
        c.g[A0][0] = sx(r32(c, S0).wrapping_add(0x6dc));
        call_ptr(c, 0x0031_84e8, 0x001b_ec24)?;
        c.g[A0][0] = c.lw(r32(c, S0).wrapping_add(0x6d0));
        c.g[A1][0] = c.lw(r32(c, S0).wrapping_add(0x6dc));
        call_ptr(c, 0x003a_0c50, 0x001b_ec30)?;
        c.g[A0][0] = c.g[S0][0];
    } else {
        c.cycles += 2;
        let is_password = misc == c.g[V0][0];
        c.g[A0][0] = c.g[S0][0];
        if !is_password {
            c.cycles += 2;
            c.g[A1][0] = c.g[S2][0];
            return net_create_room_forward(c, frame);
        }
        c.cycles += 7;
        c.g[A1][0] = sx(r32(c, A1).wrapping_add(0x74));
        c.g[A0][0] = sx(r32(c, S0).wrapping_add(0x6e4));
        call_ptr(c, 0x0031_84e8, 0x001b_ec44)?;
        c.g[A0][0] = c.lw(r32(c, S0).wrapping_add(0x6d4));
        c.g[A1][0] = c.lw(r32(c, S0).wrapping_add(0x6e4));
        call_ptr(c, 0x003a_0c50, 0x001b_ec50)?;
        c.g[A0][0] = c.g[S0][0];
    }
    // Both copies fall into the base handler.
    c.cycles += 1;
    c.g[A1][0] = c.g[S2][0];
    net_create_room_forward(c, frame)
}

// ---------------------------------------------------------------------------
// cFEStateLogin_OnWidgetCreated.

pub(crate) const LOGIN_WIDGET_HASH: u32 = 0x38;
pub(crate) const LOGIN_WIDGET_MISC: u32 = 0x18;
#[cfg(test)]
pub(crate) const LOGIN_ACCOUNT_STRING: u32 = 0x0053_6760;

/// Epilogue of `cFEStateLogin_OnWidgetCreated` (`s0` is restored by the caller
/// at the point the original does).
fn login_created_exit(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 5;
    frame.restore(c, S1, 0xa0);
    frame.restore(c, S2, 0x90);
    frame.restore_ra(c, 0x80);
    frame.close(c);
    Ok(())
}

/// `s0 = widget->hash; v0 = StrHash(name)`: whether the widget's name hash
/// differs from the hash of the string at `0x460000 + name_off`.
fn login_name_differs(c: &mut Cpu, name_off: u32, ret: u32) -> Result<bool, Unwound> {
    c.g[S0][0] = c.lw(r32(c, S2).wrapping_add(LOGIN_WIDGET_HASH));
    c.g[A0][0] = sx(0x0046_0000u32.wrapping_add(name_off));
    call_ptr(c, 0x0031_7670, ret)?;
    Ok(c.g[S0][0] != c.g[V0][0])
}

/// Call the widget's virtual method at vtable offset `slot` (`+4` is the
/// function, `+0` the `this` adjustment) with `a1 = arg`; `v0` is loaded from
/// `*(widget + 8)` first when `reload` is set.
fn login_widget_vcall(c: &mut Cpu, reload: bool, slot: u32, arg: u64, ret: u32) -> Flow {
    let widget = r32(c, S2);
    if reload {
        c.g[V0][0] = c.lw(widget.wrapping_add(0x8));
    }
    c.g[A1][0] = arg;
    c.g[A0][0] = c.lh(r32(c, V0).wrapping_add(slot));
    c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(slot + 4));
    let method = r32(c, V1);
    c.g[A0][0] = sx(widget.wrapping_add(r32(c, A0)));
    call_ptr(c, method, ret)
}

/// The "use account" / "delete account" label: `sprintf("%s%S", localised
/// button text, account name)` into the stack buffer, `UIText_SetWideString`,
/// show the widget, and hide it again when the account name is empty. The two
/// labels differ in the misc data (0 / 3), the text id, `v0`/`v1` use and
/// return addresses (`rets`).
fn login_account_label(c: &mut Cpu, frame: Frame, delete: bool, rets: [u32; 6]) -> Flow {
    let gp = r32(c, GP);
    let belib;
    if delete {
        c.cycles += 35;
        c.g[V1][0] = c.lw(gp.wrapping_sub(0x848));
        c.g[V0][0] = 0x3;
        let misc = c.g[V0][0];
        Addr(r32(c, S2).wrapping_add(LOGIN_WIDGET_MISC)).sw(c, misc);
        c.g[A0][0] = 0x460000;
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x2f08));
        belib = r32(c, V1);
    } else {
        c.cycles += 34;
        Addr(r32(c, S2).wrapping_add(LOGIN_WIDGET_MISC)).sw(c, 0);
        c.g[A0][0] = 0x460000;
        c.g[V0][0] = c.lw(gp.wrapping_sub(0x848));
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x52d0));
        belib = r32(c, V0);
    }
    c.g[S0][0] = c.lw(belib.wrapping_add(0x8c));
    c.g[S1][0] = c.lw(r32(c, S0).wrapping_add(0x4));
    c.g[V0][0] = c.lh(r32(c, S1).wrapping_add(0x20));
    c.g[S0][0] = sx(r32(c, S0).wrapping_add(r32(c, V0)));
    call_ptr(c, 0x0031_7670, rets[0])?;
    c.g[V1][0] = c.lw(r32(c, S1).wrapping_add(0x24));
    c.g[A0][0] = c.g[S0][0];
    let localise = r32(c, V1);
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, localise, rets[1])?;
    c.g[S0][0] = 0x530000;
    c.g[A1][0] = 0x4a0000;
    c.g[S0][0] = sx(r32(c, S0).wrapping_add(0x6760));
    c.g[A2][0] = c.g[V0][0];
    c.g[A0][0] = c.g[SP][0];
    c.g[A3][0] = c.g[S0][0];
    c.g[A1][0] = sx(r32(c, A1).wrapping_add(0x1bf8));
    call_ptr(c, 0x002c_2718, rets[2])?;
    c.g[A1][0] = c.g[SP][0];
    c.g[A0][0] = c.g[S2][0];
    call_ptr(c, 0x003a_0e90, rets[3])?;
    login_widget_vcall(c, true, 0x48, 1, rets[4])?;
    c.g[A0][0] = c.g[S0][0];
    call_ptr(c, 0x0041_6810, rets[5])?;
    let has_account = c.g[V0][0] != 0;
    frame.restore(c, S0, 0xb0);
    if has_account {
        return login_created_exit(c, frame);
    }
    if delete {
        c.cycles += 19;
        login_widget_vcall(c, true, 0xc8, 1, 0x001b_f1e0)?;
        login_widget_vcall(c, true, 0x40, 1, 0x001b_f1f8)?;
        login_widget_vcall(c, true, 0x48, 0, 0x001b_f210)?;
        frame.restore(c, S0, 0xb0);
        return login_created_exit(c, frame);
    }
    c.cycles += 15;
    login_widget_vcall(c, true, 0xc8, 1, 0x001b_f030)?;
    login_widget_vcall(c, true, 0x40, 1, 0x001b_f048)?;
    login_hide_widget(c, frame)
}

/// Load `*(widget + 8)`, then `setVisible(a1)` through slot +0x48 and leave:
/// the join of the "rectangle" and "use account" paths (`a1` is set by the
/// caller, `v0` is the widget's vtable owner).
fn login_set_visible_exit(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 6;
    let arg = c.g[A1][0];
    login_widget_vcall(c, false, 0x48, arg, 0x001b_f0c8)?;
    frame.restore(c, S0, 0xb0);
    login_created_exit(c, frame)
}

/// `setVisible(0)`, then leave.
fn login_hide_widget(c: &mut Cpu, frame: Frame) -> Flow {
    c.g[V0][0] = c.lw(r32(c, S2).wrapping_add(0x8));
    c.g[A1][0] = 0;
    login_set_visible_exit(c, frame)
}

/// The "1rectangle" / "4rectangle" decorations: visible only when an account
/// name is stored.
fn login_rectangle(c: &mut Cpu, frame: Frame, ret: u32) -> Flow {
    c.cycles += 5;
    c.g[A0][0] = 0x530000;
    c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x6760));
    call_ptr(c, 0x0041_6810, ret)?;
    let has_account = c.g[V0][0] != 0;
    c.g[V0][0] = c.lw(r32(c, S2).wrapping_add(0x8));
    if has_account {
        c.cycles += 1;
        c.g[A1][0] = 0x1;
    } else {
        c.cycles += 2;
        c.g[A1][0] = 0;
    }
    login_set_visible_exit(c, frame)
}

/// Create / existing-account buttons: store the misc data (2 / 1) and show.
fn login_account_button(c: &mut Cpu, frame: Frame) -> Flow {
    // `v0` holds the misc data on entry.
    c.cycles += 9;
    c.g[V1][0] = c.lw(r32(c, S2).wrapping_add(0x8));
    let misc = c.g[V0][0];
    Addr(r32(c, S2).wrapping_add(LOGIN_WIDGET_MISC)).sw(c, misc);
    c.g[A1][0] = 0x1;
    c.g[A0][0] = c.lh(r32(c, V1).wrapping_add(0x48));
    c.g[V0][0] = c.lw(r32(c, V1).wrapping_add(0x4c));
    let method = r32(c, V0);
    c.g[A0][0] = sx(r32(c, S2).wrapping_add(r32(c, A0)));
    call_ptr(c, method, 0x001b_f120)?;
    frame.restore(c, S0, 0xb0);
    login_created_exit(c, frame)
}

/// `void cFEStateLogin_OnWidgetCreated(this, widget)` (0x1bef60): set up a
/// login screen widget by its name hash: the "use account" and "delete
/// account" texts show the stored account name (hidden and static when none),
/// the two rectangles follow whether an account exists, the create / existing
/// buttons carry misc data 2 / 1.
pub fn c_fe_state_login_on_widget_created(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 12;
        let frame = Frame::open(c, 0xc0);
        c.g[A0][0] = 0x460000;
        frame.save(c, S2, 0x90);
        frame.save(c, S1, 0xa0);
        c.g[S2][0] = c.g[A1][0];
        frame.save_ra(c, 0x80);
        frame.save(c, S0, 0xb0);
        // "TextUseAccount"
        if !login_name_differs(c, 0x52c0, 0x001b_ef88)? {
            return login_account_label(
                c,
                frame,
                false,
                [
                    0x001b_efb4,
                    0x001b_efc4,
                    0x001b_efe4,
                    0x001b_eff0,
                    0x001b_f008,
                    0x001b_f010,
                ],
            );
        }
        // "1rectangle"
        c.cycles += 5;
        if !login_name_differs(c, 0x52e0, 0x001b_f060)? {
            return login_rectangle(c, frame, 0x001b_f074);
        }
        // "4rectangle"
        c.cycles += 5;
        if !login_name_differs(c, 0x52f0, 0x001b_f090)? {
            return login_rectangle(c, frame, 0x001b_f0a4);
        }
        // "TextCreateAccount"
        c.cycles += 5;
        if !login_name_differs(c, 0x5300, 0x001b_f0dc)? {
            c.cycles += 2;
            c.g[V0][0] = 0x2;
            return login_account_button(c, frame);
        }
        // "TextExistingAccount"
        c.cycles += 5;
        if !login_name_differs(c, 0x5318, 0x001b_f0f8)? {
            c.cycles += 1;
            c.g[V0][0] = 0x1;
            return login_account_button(c, frame);
        }
        // "TextDelectAccount"
        c.cycles += 5;
        if login_name_differs(c, 0x5330, 0x001b_f134)? {
            frame.restore(c, S0, 0xb0);
            return login_created_exit(c, frame);
        }
        login_account_label(
            c,
            frame,
            true,
            [
                0x001b_f164,
                0x001b_f174,
                0x001b_f194,
                0x001b_f1a0,
                0x001b_f1b8,
                0x001b_f1c0,
            ],
        )
    });
}

// ---------------------------------------------------------------------------
// cFEStateLogin_OnWidgetAction.

/// `void cFEStateLogin_OnWidgetAction(this, popup, msg)` (0x1bf758): the login
/// state's widget handler. msg 5 (button) dispatches on the button's misc data:
/// 0 stores the user name / password in the lobby manager and connects, 2
/// opens the create-account step, 1 the existing-account screen, 3 the account
/// memory popup; msg 6 builds the "disconnecting" yes/no popup; msg 8 opens the
/// online agreement; anything else goes to the base state. The generated
/// block structure is kept (every register and cycle of its paths).
pub fn c_fe_state_login_on_widget_action(c: &mut Cpu) {
    guest(c, |c| {
        'b_1bfabc: {
            'b_1bfab8: {
                'b_1bfab4: {
                    c.cycles += 11;
                    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x80));
                    c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x50));
                    c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
                    c.g[S2][0] = c.g[A0][0];
                    c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
                    c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x40));
                    c.sq(20, (c.g[SP][0] as u32).wrapping_add(0x30));
                    c.sq(21, (c.g[SP][0] as u32).wrapping_add(0x20));
                    c.sq(22, (c.g[SP][0] as u32).wrapping_add(0x10));
                    let t = c.g[A1][0] == 0u64;
                    c.sd((c.g[SP][0] as u32), c.g[RA][0]);
                    if t {
                        break 'b_1bfab4;
                    } else {
                        'b_1bfaac: {
                            'b_1bfa9c: {
                                'b_1bfa70: {
                                    'b_1bf8ec: {
                                        c.cycles += 3;
                                        c.g[V0][0] = 0x6;
                                        let t = c.g[A2][0] == c.g[V0][0];
                                        c.g[V0][0] = (c.g[A2][0] < 0x7u64) as u64;
                                        if t {
                                            break 'b_1bf8ec;
                                        } else {
                                            'b_1bf7bc: {
                                                'b_1bf7a8: {
                                                    c.cycles += 2;
                                                    let t = c.g[V0][0] == 0u64;
                                                    c.g[V0][0] = 0x5;
                                                    if t {
                                                        break 'b_1bf7a8;
                                                    } else {
                                                        c.cycles += 2;
                                                        if c.g[A2][0] == c.g[V0][0] {
                                                            c.g[A1][0] = c.lw((c.g[A1][0] as u32)
                                                                .wrapping_add(0x18));
                                                            break 'b_1bf7bc;
                                                        } else {
                                                            c.cycles += 2;
                                                            let t = true;

                                                            if t {
                                                                break 'b_1bfaac;
                                                            } else {
                                                                break 'b_1bf7a8;
                                                            }
                                                        }
                                                    }
                                                }
                                                c.cycles += 3;
                                                c.g[V0][0] = 0x8;
                                                if c.g[A2][0] == c.g[V0][0] {
                                                    c.g[A1][0] = 0x460000;
                                                    break 'b_1bfa70;
                                                } else {
                                                    c.cycles += 2;
                                                    let t = true;

                                                    if t {
                                                        break 'b_1bfaac;
                                                    } else {
                                                        break 'b_1bf7bc;
                                                    }
                                                }
                                            }
                                            'b_1bf890: {
                                                'b_1bf864: {
                                                    c.cycles += 3;
                                                    c.g[V0][0] = 0x1;
                                                    let t = c.g[A1][0] == c.g[V0][0];
                                                    c.g[V0][0] =
                                                        ((c.g[A1][0] as i64) < 2i64) as u64;
                                                    if t {
                                                        break 'b_1bf864;
                                                    } else {
                                                        'b_1bf834: {
                                                            'b_1bf7f8: {
                                                                'b_1bf7e0: {
                                                                    c.cycles += 2;
                                                                    if c.g[V0][0] == 0u64 {
                                                                        c.g[V0][0] = 0x2;
                                                                        break 'b_1bf7e0;
                                                                    } else {
                                                                        c.cycles += 2;
                                                                        let t = c.g[A1][0] == 0u64;
                                                                        c.lq(
                                                                            16,
                                                                            (c.g[SP][0] as u32)
                                                                                .wrapping_add(0x70),
                                                                        );
                                                                        if t {
                                                                            break 'b_1bf7f8;
                                                                        } else {
                                                                            c.cycles += 2;
                                                                            let t = true;
                                                                            c.lq(
                                                                                17,
                                                                                (c.g[SP][0] as u32)
                                                                                    .wrapping_add(
                                                                                        0x60,
                                                                                    ),
                                                                            );
                                                                            if t {
                                                                                break 'b_1bfabc;
                                                                            } else {
                                                                                break 'b_1bf7e0;
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                                c.cycles += 2;
                                                                let t = c.g[A1][0] == c.g[V0][0];
                                                                c.g[V0][0] = 0x3;
                                                                if t {
                                                                    break 'b_1bf834;
                                                                } else {
                                                                    c.cycles += 2;
                                                                    let t =
                                                                        c.g[A1][0] == c.g[V0][0];
                                                                    c.lq(
                                                                        16,
                                                                        (c.g[SP][0] as u32)
                                                                            .wrapping_add(0x70),
                                                                    );
                                                                    if t {
                                                                        break 'b_1bf890;
                                                                    } else {
                                                                        c.cycles += 2;
                                                                        let t = true;
                                                                        c.lq(
                                                                            17,
                                                                            (c.g[SP][0] as u32)
                                                                                .wrapping_add(0x60),
                                                                        );
                                                                        if t {
                                                                            break 'b_1bfabc;
                                                                        } else {
                                                                            break 'b_1bf7f8;
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                            c.cycles += 15;
                                                            c.g[V0][0] = c.lw((c.g[GP][0] as u32)
                                                                .wrapping_sub(0x848));
                                                            c.g[A0][0] = c.lw((c.g[GP][0] as u32)
                                                                .wrapping_sub(0xc8));
                                                            c.g[A1][0] = c.lw((c.g[V0][0] as u32)
                                                                .wrapping_add(0xc4));
                                                            c.g[RA][0] = sx(0x001bf80c);
                                                            c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                                .wrapping_add(0x48));
                                                            call_ptr(c, 0x003184e8, 0x001bf80c)?;
                                                            c.g[V1][0] = c.lw((c.g[GP][0] as u32)
                                                                .wrapping_sub(0x848));
                                                            c.g[A0][0] = c.lw((c.g[GP][0] as u32)
                                                                .wrapping_sub(0xc8));
                                                            c.g[A1][0] = c.lw((c.g[V1][0] as u32)
                                                                .wrapping_add(0xc8));
                                                            c.g[RA][0] = sx(0x001bf820);
                                                            c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                                .wrapping_add(0x4c));
                                                            call_ptr(c, 0x003184e8, 0x001bf820)?;
                                                            c.g[A0][0] = c.lw((c.g[GP][0] as u32)
                                                                .wrapping_sub(0xc8));
                                                            c.g[RA][0] = sx(0x001bf82c);
                                                            c.g[A1][0] = 0u64;
                                                            call_ptr(c, 0x0025b6d8, 0x001bf82c)?;
                                                            let t = true;
                                                            c.lq(
                                                                16,
                                                                (c.g[SP][0] as u32)
                                                                    .wrapping_add(0x70),
                                                            );
                                                            if t {
                                                                break 'b_1bfab8;
                                                            } else {
                                                                break 'b_1bf834;
                                                            }
                                                        }
                                                        c.cycles += 12;
                                                        c.g[A1][0] = 0x460000;
                                                        c.g[A0][0] = 0x6f4;
                                                        c.g[A1][0] = sx((c.g[A1][0] as u32)
                                                            .wrapping_add(0x2ed0));
                                                        c.g[A2][0] = 0u64;
                                                        c.g[RA][0] = sx(0x001bf84c);
                                                        c.g[A3][0] = 0u64;
                                                        call_ptr(c, 0x00317d70, 0x001bf84c)?;
                                                        c.g[A1][0] = c
                                                            .lw((c.g[S2][0] as u32)
                                                                .wrapping_add(0x10));
                                                        c.g[A0][0] = c.g[V0][0];
                                                        c.g[RA][0] = sx(0x001bf85c);
                                                        c.g[A2][0] = 0x1;
                                                        Callee {
                                                            addr: 0x001b88f0,
                                                            f: ee::s03::cFEStateCreateStep1_ctor,
                                                        }
                                                        .call(c, 0x001bf85c)?;
                                                        let t = true;
                                                        c.g[A0][0] = c.g[S2][0];
                                                        if t {
                                                            break 'b_1bfa9c;
                                                        } else {
                                                            break 'b_1bf864;
                                                        }
                                                    }
                                                }
                                                c.cycles += 11;
                                                c.g[A1][0] = 0x460000;
                                                c.g[A0][0] = 0x6e4;
                                                c.g[A1][0] =
                                                    sx((c.g[A1][0] as u32).wrapping_add(0x5348));
                                                c.g[A2][0] = 0u64;
                                                c.g[RA][0] = sx(0x001bf87c);
                                                c.g[A3][0] = 0u64;
                                                call_ptr(c, 0x00317d70, 0x001bf87c)?;
                                                c.g[A1][0] =
                                                    c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                                                c.g[RA][0] = sx(0x001bf888);
                                                c.g[A0][0] = c.g[V0][0];
                                                Callee {
                                                    addr: 0x001ac010,
                                                    f: ee::s03::cFEUseExistingAccount_ctor,
                                                }
                                                .call(c, 0x001bf888)?;
                                                let t = true;
                                                c.g[A0][0] = c.g[S2][0];
                                                if t {
                                                    break 'b_1bfa9c;
                                                } else {
                                                    break 'b_1bf890;
                                                }
                                            }
                                            c.cycles += 23;
                                            c.g[A1][0] = 0x460000;
                                            c.g[A0][0] = 0xb10;
                                            c.g[A1][0] =
                                                sx((c.g[A1][0] as u32).wrapping_add(0x2d10));
                                            c.g[A2][0] = 0u64;
                                            c.g[RA][0] = sx(0x001bf8a8);
                                            c.g[A3][0] = 0u64;
                                            call_ptr(c, 0x00317d70, 0x001bf8a8)?;
                                            c.g[A1][0] =
                                                c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                                            c.g[A0][0] = c.g[V0][0];
                                            c.g[T0][0] = 0u64;
                                            c.g[T1][0] = 0u64;
                                            c.g[A2][0] = c.g[S2][0];
                                            c.g[RA][0] = sx(0x001bf8c4);
                                            c.g[A3][0] = 0x3;
                                            Callee {
                                                addr: 0x001b6ed0,
                                                f: ee::s03::cFEStateNetAccountMem_ctor,
                                            }
                                            .call(c, 0x001bf8c4)?;
                                            c.g[V1][0] = 0x82c;
                                            c.g[A1][0] = c.g[V0][0];
                                            c.sw(
                                                (c.g[V0][0] as u32).wrapping_add(0x18),
                                                c.g[V1][0],
                                            );
                                            c.g[V0][0] =
                                                c.lw((c.g[S2][0] as u32).wrapping_add(0x40));
                                            c.g[V1][0] =
                                                c.lw((c.g[V0][0] as u32).wrapping_add(0xd0));
                                            c.g[A0][0] =
                                                c.lw((c.g[V1][0] as u32).wrapping_add(0x10));
                                            c.g[RA][0] = sx(0x001bf8e4);
                                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
                                            call_ptr(c, 0x0039f290, 0x001bf8e4)?;
                                            let t = true;
                                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
                                            if t {
                                                break 'b_1bfab8;
                                            } else {
                                                break 'b_1bf8ec;
                                            }
                                        }
                                    }
                                    'b_1bf908: {
                                        c.cycles += 3;
                                        c.g[A0][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x6d0));
                                        let t = c.g[A0][0] == 0u64;
                                        c.g[V0][0] = c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                        if t {
                                            break 'b_1bf908;
                                        } else {
                                            c.cycles += 4;
                                            c.g[RA][0] = sx(0x001bf900);
                                            call_ptr(c, 0x00317e98, 0x001bf900)?;
                                            c.sw((c.g[S2][0] as u32).wrapping_add(0x6d0), 0u64);
                                            c.g[V0][0] =
                                                c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                            break 'b_1bf908;
                                        }
                                    }
                                    'b_1bfa0c: {
                                        'b_1bf9f8: {
                                            c.cycles += 51;
                                            c.g[A0][0] = 0x460000;
                                            c.g[A0][0] =
                                                sx((c.g[A0][0] as u32).wrapping_add(0x5360));
                                            c.g[S0][0] =
                                                c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                                            c.g[S1][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                                            c.g[V0][0] =
                                                c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                                            c.g[RA][0] = sx(0x001bf924);
                                            c.g[S0][0] = sx((c.g[S0][0] as u32)
                                                .wrapping_add((c.g[V0][0] as u32)));
                                            call_ptr(c, 0x00317670, 0x001bf924)?;
                                            c.g[V1][0] =
                                                c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                                            c.g[A1][0] = c.g[V0][0];
                                            let t = c.g[V1][0] as u32;
                                            c.g[RA][0] = sx(0x001bf934);
                                            c.g[A0][0] = c.g[S0][0];
                                            call_ptr(c, t, 0x001bf934)?;
                                            c.g[S4][0] = c.g[V0][0];
                                            c.g[V1][0] = 0x460000;
                                            c.g[V0][0] =
                                                c.lw((c.g[GP][0] as u32).wrapping_sub(0x848));
                                            c.g[S6][0] =
                                                sx((c.g[V1][0] as u32).wrapping_add(0x5380));
                                            c.g[A0][0] = c.g[S6][0];
                                            c.g[S0][0] =
                                                c.lw((c.g[V0][0] as u32).wrapping_add(0x8c));
                                            c.g[S1][0] =
                                                c.lw((c.g[S0][0] as u32).wrapping_add(0x4));
                                            c.g[V0][0] =
                                                c.lh((c.g[S1][0] as u32).wrapping_add(0x20));
                                            c.g[RA][0] = sx(0x001bf95c);
                                            c.g[S0][0] = sx((c.g[S0][0] as u32)
                                                .wrapping_add((c.g[V0][0] as u32)));
                                            call_ptr(c, 0x00317670, 0x001bf95c)?;
                                            c.g[V1][0] =
                                                c.lw((c.g[S1][0] as u32).wrapping_add(0x24));
                                            c.g[A1][0] = c.g[V0][0];
                                            let t = c.g[V1][0] as u32;
                                            c.g[RA][0] = sx(0x001bf96c);
                                            c.g[A0][0] = c.g[S0][0];
                                            call_ptr(c, t, 0x001bf96c)?;
                                            c.g[S3][0] = c.g[V0][0];
                                            c.g[A0][0] = c.g[S4][0];
                                            c.g[V0][0] = 0x460000;
                                            c.g[RA][0] = sx(0x001bf980);
                                            c.g[S5][0] =
                                                sx((c.g[V0][0] as u32).wrapping_add(0x5398));
                                            call_ptr(c, 0x003dbca0, 0x001bf980)?;
                                            c.g[S0][0] = c.g[V0][0];
                                            c.g[RA][0] = sx(0x001bf98c);
                                            c.g[A0][0] = c.g[S3][0];
                                            call_ptr(c, 0x003dbca0, 0x001bf98c)?;
                                            c.g[S0][0] = sx((c.g[S0][0] as u32)
                                                .wrapping_add((c.g[V0][0] as u32)));
                                            c.g[RA][0] = sx(0x001bf998);
                                            c.g[A0][0] = c.g[S5][0];
                                            call_ptr(c, 0x00416810, 0x001bf998)?;
                                            c.g[S0][0] = sx((c.g[S0][0] as u32)
                                                .wrapping_add((c.g[V0][0] as u32)));
                                            c.g[A1][0] = 0x460000;
                                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                                            c.g[A1][0] =
                                                sx((c.g[A1][0] as u32).wrapping_add(0x1cd8));
                                            c.g[A0][0] = sx((c.g[S0][0] as u32) << 1);
                                            c.g[A2][0] = 0x100;
                                            c.g[RA][0] = sx(0x001bf9b8);
                                            c.g[A3][0] = 0u64;
                                            call_ptr(c, 0x00317e30, 0x001bf9b8)?;
                                            c.g[A0][0] = c.g[S2][0];
                                            c.g[RA][0] = sx(0x001bf9c4);
                                            c.sw(
                                                (c.g[S2][0] as u32).wrapping_add(0x6d0),
                                                c.g[V0][0],
                                            );
                                            Callee {
                                                addr: 0x001a9c50,
                                                f: ee::s03::NetFEState_GetPopup,
                                            }
                                            .call(c, 0x001bf9c4)?;
                                            c.g[S0][0] = c.g[V0][0];
                                            c.g[A0][0] =
                                                c.lw((c.g[S2][0] as u32).wrapping_add(0x6d0));
                                            let t = c.g[A0][0] == 0u64;
                                            c.g[S1][0] = sx((c.g[S0][0] as u32).wrapping_add(0xbc));
                                            if t {
                                                break 'b_1bf9f8;
                                            } else {
                                                c.cycles += 9;
                                                c.g[A1][0] = c.g[S5][0];
                                                c.g[A2][0] = c.g[S4][0];
                                                c.g[RA][0] = sx(0x001bf9e4);
                                                c.g[A3][0] = c.g[S3][0];
                                                call_ptr(c, 0x002c27c0, 0x001bf9e4)?;
                                                c.g[V1][0] =
                                                    c.lw((c.g[S2][0] as u32).wrapping_add(0x6d0));
                                                c.g[V0][0] = 0x1;
                                                c.sw(
                                                    (c.g[S0][0] as u32).wrapping_add(0xc0),
                                                    c.g[V0][0],
                                                );
                                                let t = true;
                                                c.sw(
                                                    (c.g[S0][0] as u32).wrapping_add(0xc4),
                                                    c.g[V1][0],
                                                );
                                                if t {
                                                    break 'b_1bfa0c;
                                                } else {
                                                    break 'b_1bf9f8;
                                                }
                                            }
                                        }
                                        c.cycles += 5;
                                        c.g[RA][0] = sx(0x001bfa00);
                                        c.g[A0][0] = c.g[S6][0];
                                        call_ptr(c, 0x00317670, 0x001bfa00)?;
                                        c.g[V1][0] = 0x3;
                                        c.sw((c.g[S0][0] as u32).wrapping_add(0xc4), c.g[V0][0]);
                                        c.sw((c.g[S0][0] as u32).wrapping_add(0xc0), c.g[V1][0]);
                                        break 'b_1bfa0c;
                                    }
                                    c.cycles += 25;
                                    c.g[V0][0] = 0x2;
                                    c.g[A0][0] = 0x460000;
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x2338));
                                    c.g[RA][0] = sx(0x001bfa20);
                                    c.sw((c.g[S1][0] as u32), c.g[V0][0]);
                                    call_ptr(c, 0x00317670, 0x001bfa20)?;
                                    c.g[A2][0] = c.g[V0][0];
                                    c.g[A1][0] = 0u64;
                                    c.g[RA][0] = sx(0x001bfa30);
                                    c.g[A0][0] = c.g[S1][0];
                                    Callee {
                                        addr: 0x001c57c0,
                                        f: ee::s03::cFEPopupConfirm_CfgSetOptionTextId,
                                    }
                                    .call(c, 0x001bfa30)?;
                                    c.g[A0][0] = 0x460000;
                                    c.g[RA][0] = sx(0x001bfa3c);
                                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x328));
                                    call_ptr(c, 0x00317670, 0x001bfa3c)?;
                                    c.g[A2][0] = c.g[V0][0];
                                    c.g[A0][0] = c.g[S1][0];
                                    c.g[RA][0] = sx(0x001bfa4c);
                                    c.g[A1][0] = 0x1;
                                    Callee {
                                        addr: 0x001c57c0,
                                        f: ee::s03::cFEPopupConfirm_CfgSetOptionTextId,
                                    }
                                    .call(c, 0x001bfa4c)?;
                                    c.g[V1][0] = 0x1;
                                    c.g[V0][0] = 0x848;
                                    c.sw((c.g[S1][0] as u32).wrapping_add(0x94), c.g[V0][0]);
                                    c.g[A0][0] = c.g[S2][0];
                                    c.sw((c.g[S1][0] as u32).wrapping_add(0xac), c.g[V1][0]);
                                    c.g[RA][0] = sx(0x001bfa68);
                                    c.g[A1][0] = c.g[S0][0];
                                    Callee {
                                        addr: 0x001a9bc0,
                                        f: ee::s03::NetFEState_ShowPopup,
                                    }
                                    .call(c, 0x001bfa68)?;
                                    let t = true;
                                    c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
                                    if t {
                                        break 'b_1bfab8;
                                    } else {
                                        break 'b_1bfa70;
                                    }
                                }
                                c.cycles += 11;
                                c.g[A0][0] = 0x6dc;
                                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4860));
                                c.g[A2][0] = 0x100;
                                c.g[RA][0] = sx(0x001bfa84);
                                c.g[A3][0] = 0u64;
                                call_ptr(c, 0x00317d70, 0x001bfa84)?;
                                c.g[A1][0] = c.lw((c.g[S2][0] as u32).wrapping_add(0x10));
                                c.g[RA][0] = sx(0x001bfa90);
                                c.g[A0][0] = c.g[V0][0];
                                Callee {
                                    addr: 0x001c0418,
                                    f: ee::s03::cFEStateOnlineAgreement_ctor,
                                }
                                .call(c, 0x001bfa90)?;
                                c.g[V1][0] = 0x12;
                                c.g[A0][0] = c.g[S2][0];
                                c.sw((c.g[V0][0] as u32).wrapping_add(0x18), c.g[V1][0]);
                                break 'b_1bfa9c;
                            }
                            c.cycles += 4;
                            c.g[RA][0] = sx(0x001bfaa4);
                            c.g[A1][0] = c.g[V0][0];
                            Callee {
                                addr: 0x001a8a40,
                                f: ee::s03::NetFEState_GotoNextState,
                            }
                            .call(c, 0x001bfaa4)?;
                            let t = true;
                            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
                            if t {
                                break 'b_1bfab8;
                            } else {
                                break 'b_1bfaac;
                            }
                        }
                        c.cycles += 2;
                        c.g[RA][0] = sx(0x001bfab4);
                        c.g[A0][0] = c.g[S2][0];
                        Callee {
                            addr: 0x001a8918,
                            f: ee::s03::NetFEState_OnWidgetAction,
                        }
                        .call(c, 0x001bfab4)?;
                        break 'b_1bfab4;
                    }
                }
                c.cycles += 1;
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x70));
                break 'b_1bfab8;
            }
            c.cycles += 1;
            c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x60));
            break 'b_1bfabc;
        }
        c.cycles += 8;
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(20, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.lq(21, (c.g[SP][0] as u32).wrapping_add(0x20));
        c.lq(22, (c.g[SP][0] as u32).wrapping_add(0x10));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
        c.pc = t;
        return Ok(());
    });
}

// ---------------------------------------------------------------------------
// cFEStateOnlineAgreement_LoadTermsText.

/// `void cFEStateOnlineAgreement_LoadTermsText(this)` (0x1c0540): free the old
/// terms-of-service line table (`this+0x6d0`), allocate one for 178 entries
/// (`this+0x6d4`) and fill it with the string hashes of `kT_TOS%03d` for each
/// line (with the lettered sub-lines `kT_TOS115a`, ... in between). The
/// generated block structure is kept.
pub fn c_fe_state_online_agreement_load_terms_text(c: &mut Cpu) {
    guest(c, |c| {
        'b_1c0580: {
            c.cycles += 12;
            c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x70));
            c.g[V0][0] = 0xb2;
            c.sq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
            c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
            c.g[S1][0] = c.g[A0][0];
            c.sq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
            c.sq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
            c.sd((c.g[SP][0] as u32).wrapping_add(0x20), c.g[RA][0]);
            c.sw((c.g[S1][0] as u32).wrapping_add(0x6d4), c.g[V0][0]);
            c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x6d0));
            if c.g[A0][0] == 0u64 {
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x6d4));
                break 'b_1c0580;
            } else {
                c.cycles += 4;
                c.g[RA][0] = sx(0x001c0578);
                call_ptr(c, 0x00317e98, 0x001c0578)?;
                c.sw((c.g[S1][0] as u32).wrapping_add(0x6d0), 0u64);
                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x6d4));
                break 'b_1c0580;
            }
        }
        'b_1c0a08: {
            c.cycles += 2;
            let t = c.g[A0][0] == 0u64;
            c.g[A1][0] = 0x460000;
            if t {
                break 'b_1c0a08;
            } else {
                c.cycles += 10;
                c.g[A0][0] = sx((c.g[A0][0] as u32) << 2);
                c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x4860));
                c.g[A2][0] = 0x100;
                c.g[A3][0] = 0u64;
                c.g[RA][0] = sx(0x001c05a0);
                c.g[S0][0] = 0u64;
                call_ptr(c, 0x00317e30, 0x001c05a0)?;
                c.g[S2][0] = 0u64;
                c.sw((c.g[S1][0] as u32).wrapping_add(0x6d0), c.g[V0][0]);
                c.g[S3][0] = 0x460000;
                'l_1c05b0: loop {
                    c.cycles += 14;
                    c.g[A0][0] = c.g[SP][0];
                    c.g[A2][0] = c.g[S2][0];
                    c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_add(0x5590));
                    c.g[RA][0] = sx(0x001c05c4);
                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                    call_ptr(c, 0x00417828, 0x001c05c4)?;
                    c.g[RA][0] = sx(0x001c05cc);
                    c.g[A0][0] = c.g[SP][0];
                    call_ptr(c, 0x00317670, 0x001c05cc)?;
                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x6d0));
                    c.g[A0][0] = sx((c.g[S0][0] as u32) << 2);
                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                    c.g[A1][0] = ((c.g[S2][0] as i64) < 116i64) as u64;
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                    let t = c.g[A1][0] != 0u64;
                    c.sw((c.g[A0][0] as u32), c.g[V0][0]);
                    if t {
                        if event_due(c) {
                            deliver_event_here(c, 0x001c05b0);
                        }
                        continue 'l_1c05b0;
                    } else {
                        c.cycles += 10;
                        c.g[A0][0] = 0x460000;
                        c.g[S2][0] = 0x74;
                        c.g[RA][0] = sx(0x001c05f8);
                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x55a0));
                        call_ptr(c, 0x00317670, 0x001c05f8)?;
                        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x6d0));
                        c.g[V1][0] = sx((c.g[S0][0] as u32) << 2);
                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                        c.sw((c.g[V1][0] as u32), c.g[V0][0]);
                        'l_1c0610: loop {
                            c.cycles += 14;
                            c.g[A0][0] = c.g[SP][0];
                            c.g[A2][0] = c.g[S2][0];
                            c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_add(0x5590));
                            c.g[RA][0] = sx(0x001c0624);
                            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                            call_ptr(c, 0x00417828, 0x001c0624)?;
                            c.g[RA][0] = sx(0x001c062c);
                            c.g[A0][0] = c.g[SP][0];
                            call_ptr(c, 0x00317670, 0x001c062c)?;
                            c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x6d0));
                            c.g[A0][0] = sx((c.g[S0][0] as u32) << 2);
                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                            c.g[A1][0] = ((c.g[S2][0] as i64) < 118i64) as u64;
                            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                            let t = c.g[A1][0] != 0u64;
                            c.sw((c.g[A0][0] as u32), c.g[V0][0]);
                            if t {
                                if event_due(c) {
                                    deliver_event_here(c, 0x001c0610);
                                }
                                continue 'l_1c0610;
                            } else {
                                c.cycles += 42;
                                c.g[A0][0] = 0x460000;
                                c.g[S2][0] = 0x7a;
                                c.g[RA][0] = sx(0x001c0658);
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x55b0));
                                call_ptr(c, 0x00317670, 0x001c0658)?;
                                c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x6d0));
                                c.g[V1][0] = sx((c.g[S0][0] as u32) << 2);
                                c.g[A0][0] = 0x460000;
                                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                                c.g[V1][0] =
                                    sx((c.g[V1][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x55c0));
                                c.g[RA][0] = sx(0x001c0678);
                                c.sw((c.g[V1][0] as u32), c.g[V0][0]);
                                call_ptr(c, 0x00317670, 0x001c0678)?;
                                c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x6d0));
                                c.g[V1][0] = sx((c.g[S0][0] as u32) << 2);
                                c.g[A0][0] = 0x460000;
                                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                                c.g[V1][0] =
                                    sx((c.g[V1][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x55d0));
                                c.g[RA][0] = sx(0x001c0698);
                                c.sw((c.g[V1][0] as u32), c.g[V0][0]);
                                call_ptr(c, 0x00317670, 0x001c0698)?;
                                c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x6d0));
                                c.g[V1][0] = sx((c.g[S0][0] as u32) << 2);
                                c.g[A0][0] = 0x460000;
                                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                                c.g[V1][0] =
                                    sx((c.g[V1][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x55e0));
                                c.g[RA][0] = sx(0x001c06b8);
                                c.sw((c.g[V1][0] as u32), c.g[V0][0]);
                                call_ptr(c, 0x00317670, 0x001c06b8)?;
                                c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x6d0));
                                c.g[V1][0] = sx((c.g[S0][0] as u32) << 2);
                                c.g[A0][0] = 0x460000;
                                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                                c.g[V1][0] =
                                    sx((c.g[V1][0] as u32).wrapping_add((c.g[A1][0] as u32)));
                                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x55f0));
                                c.g[RA][0] = sx(0x001c06d8);
                                c.sw((c.g[V1][0] as u32), c.g[V0][0]);
                                call_ptr(c, 0x00317670, 0x001c06d8)?;
                                c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x6d0));
                                c.g[V1][0] = sx((c.g[S0][0] as u32) << 2);
                                c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                                c.g[V1][0] =
                                    sx((c.g[V1][0] as u32).wrapping_add((c.g[A0][0] as u32)));
                                c.sw((c.g[V1][0] as u32), c.g[V0][0]);
                                'l_1c06f0: loop {
                                    c.cycles += 14;
                                    c.g[A0][0] = c.g[SP][0];
                                    c.g[A2][0] = c.g[S2][0];
                                    c.g[A1][0] = sx((c.g[S3][0] as u32).wrapping_add(0x5590));
                                    c.g[RA][0] = sx(0x001c0704);
                                    c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                                    call_ptr(c, 0x00417828, 0x001c0704)?;
                                    c.g[RA][0] = sx(0x001c070c);
                                    c.g[A0][0] = c.g[SP][0];
                                    call_ptr(c, 0x00317670, 0x001c070c)?;
                                    c.g[V1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x6d0));
                                    c.g[A0][0] = sx((c.g[S0][0] as u32) << 2);
                                    c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                                    c.g[A1][0] = ((c.g[S2][0] as i64) < 125i64) as u64;
                                    c.g[A0][0] =
                                        sx((c.g[A0][0] as u32).wrapping_add((c.g[V1][0] as u32)));
                                    let t = c.g[A1][0] != 0u64;
                                    c.sw((c.g[A0][0] as u32), c.g[V0][0]);
                                    if t {
                                        if event_due(c) {
                                            deliver_event_here(c, 0x001c06f0);
                                        }
                                        continue 'l_1c06f0;
                                    } else {
                                        c.cycles += 34;
                                        c.g[A0][0] = 0x460000;
                                        c.g[S2][0] = 0x7e;
                                        c.g[RA][0] = sx(0x001c0738);
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5600));
                                        call_ptr(c, 0x00317670, 0x001c0738)?;
                                        c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x6d0));
                                        c.g[V1][0] = sx((c.g[S0][0] as u32) << 2);
                                        c.g[A0][0] = 0x460000;
                                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                                        c.g[V1][0] =
                                            sx((c.g[V1][0] as u32)
                                                .wrapping_add((c.g[A1][0] as u32)));
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5610));
                                        c.g[RA][0] = sx(0x001c0758);
                                        c.sw((c.g[V1][0] as u32), c.g[V0][0]);
                                        call_ptr(c, 0x00317670, 0x001c0758)?;
                                        c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x6d0));
                                        c.g[V1][0] = sx((c.g[S0][0] as u32) << 2);
                                        c.g[A0][0] = 0x460000;
                                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                                        c.g[V1][0] =
                                            sx((c.g[V1][0] as u32)
                                                .wrapping_add((c.g[A1][0] as u32)));
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5620));
                                        c.g[RA][0] = sx(0x001c0778);
                                        c.sw((c.g[V1][0] as u32), c.g[V0][0]);
                                        call_ptr(c, 0x00317670, 0x001c0778)?;
                                        c.g[A1][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x6d0));
                                        c.g[V1][0] = sx((c.g[S0][0] as u32) << 2);
                                        c.g[A0][0] = 0x460000;
                                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                                        c.g[V1][0] =
                                            sx((c.g[V1][0] as u32)
                                                .wrapping_add((c.g[A1][0] as u32)));
                                        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x5630));
                                        c.g[RA][0] = sx(0x001c0798);
                                        c.sw((c.g[V1][0] as u32), c.g[V0][0]);
                                        call_ptr(c, 0x00317670, 0x001c0798)?;
                                        c.g[A0][0] = c.lw((c.g[S1][0] as u32).wrapping_add(0x6d0));
                                        c.g[V1][0] = sx((c.g[S0][0] as u32) << 2);
                                        c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                                        c.g[V1][0] =
                                            sx((c.g[V1][0] as u32)
                                                .wrapping_add((c.g[A0][0] as u32)));
                                        c.sw((c.g[V1][0] as u32), c.g[V0][0]);
                                        'l_1c07b0: loop {
                                            c.cycles += 14;
                                            c.g[A0][0] = c.g[SP][0];
                                            c.g[A2][0] = c.g[S2][0];
                                            c.g[A1][0] =
                                                sx((c.g[S3][0] as u32).wrapping_add(0x5590));
                                            c.g[RA][0] = sx(0x001c07c4);
                                            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(0x1));
                                            call_ptr(c, 0x00417828, 0x001c07c4)?;
                                            c.g[RA][0] = sx(0x001c07cc);
                                            c.g[A0][0] = c.g[SP][0];
                                            call_ptr(c, 0x00317670, 0x001c07cc)?;
                                            c.g[V1][0] =
                                                c.lw((c.g[S1][0] as u32).wrapping_add(0x6d0));
                                            c.g[A0][0] = sx((c.g[S0][0] as u32) << 2);
                                            c.g[S0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x1));
                                            c.g[A1][0] = ((c.g[S2][0] as i64) < 129i64) as u64;
                                            c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                .wrapping_add((c.g[V1][0] as u32)));
                                            let t = c.g[A1][0] != 0u64;
                                            c.sw((c.g[A0][0] as u32), c.g[V0][0]);
                                            if t {
                                                if event_due(c) {
                                                    deliver_event_here(c, 0x001c07b0);
                                                }
                                                continue 'l_1c07b0;
                                            } else {
                                                c.cycles += 90;
                                                c.g[A0][0] = 0x460000;
                                                c.g[S2][0] = 0x83;
                                                c.g[RA][0] = sx(0x001c07f8);
                                                c.g[A0][0] =
                                                    sx((c.g[A0][0] as u32).wrapping_add(0x5640));
                                                call_ptr(c, 0x00317670, 0x001c07f8)?;
                                                c.g[A1][0] =
                                                    c.lw((c.g[S1][0] as u32).wrapping_add(0x6d0));
                                                c.g[V1][0] = sx((c.g[S0][0] as u32) << 2);
                                                c.g[A0][0] = 0x460000;
                                                c.g[S0][0] =
                                                    sx((c.g[S0][0] as u32).wrapping_add(0x1));
                                                c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                    .wrapping_add((c.g[A1][0] as u32)));
                                                c.g[A0][0] =
                                                    sx((c.g[A0][0] as u32).wrapping_add(0x5650));
                                                c.g[RA][0] = sx(0x001c0818);
                                                c.sw((c.g[V1][0] as u32), c.g[V0][0]);
                                                call_ptr(c, 0x00317670, 0x001c0818)?;
                                                c.g[A1][0] =
                                                    c.lw((c.g[S1][0] as u32).wrapping_add(0x6d0));
                                                c.g[V1][0] = sx((c.g[S0][0] as u32) << 2);
                                                c.g[A0][0] = 0x460000;
                                                c.g[S0][0] =
                                                    sx((c.g[S0][0] as u32).wrapping_add(0x1));
                                                c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                    .wrapping_add((c.g[A1][0] as u32)));
                                                c.g[A0][0] =
                                                    sx((c.g[A0][0] as u32).wrapping_add(0x5660));
                                                c.g[RA][0] = sx(0x001c0838);
                                                c.sw((c.g[V1][0] as u32), c.g[V0][0]);
                                                call_ptr(c, 0x00317670, 0x001c0838)?;
                                                c.g[A1][0] =
                                                    c.lw((c.g[S1][0] as u32).wrapping_add(0x6d0));
                                                c.g[V1][0] = sx((c.g[S0][0] as u32) << 2);
                                                c.g[A0][0] = 0x460000;
                                                c.g[S0][0] =
                                                    sx((c.g[S0][0] as u32).wrapping_add(0x1));
                                                c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                    .wrapping_add((c.g[A1][0] as u32)));
                                                c.g[A0][0] =
                                                    sx((c.g[A0][0] as u32).wrapping_add(0x5670));
                                                c.g[RA][0] = sx(0x001c0858);
                                                c.sw((c.g[V1][0] as u32), c.g[V0][0]);
                                                call_ptr(c, 0x00317670, 0x001c0858)?;
                                                c.g[A1][0] =
                                                    c.lw((c.g[S1][0] as u32).wrapping_add(0x6d0));
                                                c.g[V1][0] = sx((c.g[S0][0] as u32) << 2);
                                                c.g[A0][0] = 0x460000;
                                                c.g[S0][0] =
                                                    sx((c.g[S0][0] as u32).wrapping_add(0x1));
                                                c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                    .wrapping_add((c.g[A1][0] as u32)));
                                                c.g[A0][0] =
                                                    sx((c.g[A0][0] as u32).wrapping_add(0x5680));
                                                c.g[RA][0] = sx(0x001c0878);
                                                c.sw((c.g[V1][0] as u32), c.g[V0][0]);
                                                call_ptr(c, 0x00317670, 0x001c0878)?;
                                                c.g[A1][0] =
                                                    c.lw((c.g[S1][0] as u32).wrapping_add(0x6d0));
                                                c.g[V1][0] = sx((c.g[S0][0] as u32) << 2);
                                                c.g[A0][0] = 0x460000;
                                                c.g[S0][0] =
                                                    sx((c.g[S0][0] as u32).wrapping_add(0x1));
                                                c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                    .wrapping_add((c.g[A1][0] as u32)));
                                                c.g[A0][0] =
                                                    sx((c.g[A0][0] as u32).wrapping_add(0x5690));
                                                c.g[RA][0] = sx(0x001c0898);
                                                c.sw((c.g[V1][0] as u32), c.g[V0][0]);
                                                call_ptr(c, 0x00317670, 0x001c0898)?;
                                                c.g[A1][0] =
                                                    c.lw((c.g[S1][0] as u32).wrapping_add(0x6d0));
                                                c.g[V1][0] = sx((c.g[S0][0] as u32) << 2);
                                                c.g[A0][0] = 0x460000;
                                                c.g[S0][0] =
                                                    sx((c.g[S0][0] as u32).wrapping_add(0x1));
                                                c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                    .wrapping_add((c.g[A1][0] as u32)));
                                                c.g[A0][0] =
                                                    sx((c.g[A0][0] as u32).wrapping_add(0x56a0));
                                                c.g[RA][0] = sx(0x001c08b8);
                                                c.sw((c.g[V1][0] as u32), c.g[V0][0]);
                                                call_ptr(c, 0x00317670, 0x001c08b8)?;
                                                c.g[A1][0] =
                                                    c.lw((c.g[S1][0] as u32).wrapping_add(0x6d0));
                                                c.g[V1][0] = sx((c.g[S0][0] as u32) << 2);
                                                c.g[A0][0] = 0x460000;
                                                c.g[S0][0] =
                                                    sx((c.g[S0][0] as u32).wrapping_add(0x1));
                                                c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                    .wrapping_add((c.g[A1][0] as u32)));
                                                c.g[A0][0] =
                                                    sx((c.g[A0][0] as u32).wrapping_add(0x56b0));
                                                c.g[RA][0] = sx(0x001c08d8);
                                                c.sw((c.g[V1][0] as u32), c.g[V0][0]);
                                                call_ptr(c, 0x00317670, 0x001c08d8)?;
                                                c.g[A1][0] =
                                                    c.lw((c.g[S1][0] as u32).wrapping_add(0x6d0));
                                                c.g[V1][0] = sx((c.g[S0][0] as u32) << 2);
                                                c.g[A0][0] = 0x460000;
                                                c.g[S0][0] =
                                                    sx((c.g[S0][0] as u32).wrapping_add(0x1));
                                                c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                    .wrapping_add((c.g[A1][0] as u32)));
                                                c.g[A0][0] =
                                                    sx((c.g[A0][0] as u32).wrapping_add(0x56c0));
                                                c.g[RA][0] = sx(0x001c08f8);
                                                c.sw((c.g[V1][0] as u32), c.g[V0][0]);
                                                call_ptr(c, 0x00317670, 0x001c08f8)?;
                                                c.g[A1][0] =
                                                    c.lw((c.g[S1][0] as u32).wrapping_add(0x6d0));
                                                c.g[V1][0] = sx((c.g[S0][0] as u32) << 2);
                                                c.g[A0][0] = 0x460000;
                                                c.g[S0][0] =
                                                    sx((c.g[S0][0] as u32).wrapping_add(0x1));
                                                c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                    .wrapping_add((c.g[A1][0] as u32)));
                                                c.g[A0][0] =
                                                    sx((c.g[A0][0] as u32).wrapping_add(0x56d0));
                                                c.g[RA][0] = sx(0x001c0918);
                                                c.sw((c.g[V1][0] as u32), c.g[V0][0]);
                                                call_ptr(c, 0x00317670, 0x001c0918)?;
                                                c.g[A1][0] =
                                                    c.lw((c.g[S1][0] as u32).wrapping_add(0x6d0));
                                                c.g[V1][0] = sx((c.g[S0][0] as u32) << 2);
                                                c.g[A0][0] = 0x460000;
                                                c.g[S0][0] =
                                                    sx((c.g[S0][0] as u32).wrapping_add(0x1));
                                                c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                    .wrapping_add((c.g[A1][0] as u32)));
                                                c.g[A0][0] =
                                                    sx((c.g[A0][0] as u32).wrapping_add(0x56e0));
                                                c.g[RA][0] = sx(0x001c0938);
                                                c.sw((c.g[V1][0] as u32), c.g[V0][0]);
                                                call_ptr(c, 0x00317670, 0x001c0938)?;
                                                c.g[A0][0] =
                                                    c.lw((c.g[S1][0] as u32).wrapping_add(0x6d0));
                                                c.g[V1][0] = sx((c.g[S0][0] as u32) << 2);
                                                c.g[S0][0] =
                                                    sx((c.g[S0][0] as u32).wrapping_add(0x1));
                                                c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                    .wrapping_add((c.g[A0][0] as u32)));
                                                c.sw((c.g[V1][0] as u32), c.g[V0][0]);
                                                'l_1c0950: loop {
                                                    c.cycles += 14;
                                                    c.g[A0][0] = c.g[SP][0];
                                                    c.g[A2][0] = c.g[S2][0];
                                                    c.g[A1][0] =
                                                        sx((c.g[S3][0] as u32)
                                                            .wrapping_add(0x5590));
                                                    c.g[RA][0] = sx(0x001c0964);
                                                    c.g[S2][0] =
                                                        sx((c.g[S2][0] as u32).wrapping_add(0x1));
                                                    call_ptr(c, 0x00417828, 0x001c0964)?;
                                                    c.g[RA][0] = sx(0x001c096c);
                                                    c.g[A0][0] = c.g[SP][0];
                                                    call_ptr(c, 0x00317670, 0x001c096c)?;
                                                    c.g[V1][0] =
                                                        c.lw(
                                                            (c.g[S1][0] as u32).wrapping_add(0x6d0)
                                                        );
                                                    c.g[A0][0] = sx((c.g[S0][0] as u32) << 2);
                                                    c.g[S0][0] =
                                                        sx((c.g[S0][0] as u32).wrapping_add(0x1));
                                                    c.g[A1][0] =
                                                        ((c.g[S2][0] as i64) < 154i64) as u64;
                                                    c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                        .wrapping_add((c.g[V1][0] as u32)));
                                                    let t = c.g[A1][0] != 0u64;
                                                    c.sw((c.g[A0][0] as u32), c.g[V0][0]);
                                                    if t {
                                                        if event_due(c) {
                                                            deliver_event_here(c, 0x001c0950);
                                                        }
                                                        continue 'l_1c0950;
                                                    } else {
                                                        c.cycles += 18;
                                                        c.g[A0][0] = 0x460000;
                                                        c.g[S2][0] = 0x9e;
                                                        c.g[RA][0] = sx(0x001c0998);
                                                        c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                            .wrapping_add(0x56f0));
                                                        call_ptr(c, 0x00317670, 0x001c0998)?;
                                                        c.g[A1][0] = c
                                                            .lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0x6d0));
                                                        c.g[V1][0] = sx((c.g[S0][0] as u32) << 2);
                                                        c.g[A0][0] = 0x460000;
                                                        c.g[S0][0] =
                                                            sx((c.g[S0][0] as u32)
                                                                .wrapping_add(0x1));
                                                        c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                            .wrapping_add((c.g[A1][0] as u32)));
                                                        c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                            .wrapping_add(0x5700));
                                                        c.g[RA][0] = sx(0x001c09b8);
                                                        c.sw((c.g[V1][0] as u32), c.g[V0][0]);
                                                        call_ptr(c, 0x00317670, 0x001c09b8)?;
                                                        c.g[A0][0] = c
                                                            .lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0x6d0));
                                                        c.g[V1][0] = sx((c.g[S0][0] as u32) << 2);
                                                        c.g[S0][0] =
                                                            sx((c.g[S0][0] as u32)
                                                                .wrapping_add(0x1));
                                                        c.g[V1][0] = sx((c.g[V1][0] as u32)
                                                            .wrapping_add((c.g[A0][0] as u32)));
                                                        c.sw((c.g[V1][0] as u32), c.g[V0][0]);
                                                        'l_1c09d0: loop {
                                                            c.cycles += 14;
                                                            c.g[A0][0] = c.g[SP][0];
                                                            c.g[A2][0] = c.g[S2][0];
                                                            c.g[A1][0] = sx((c.g[S3][0] as u32)
                                                                .wrapping_add(0x5590));
                                                            c.g[RA][0] = sx(0x001c09e4);
                                                            c.g[S2][0] = sx((c.g[S2][0] as u32)
                                                                .wrapping_add(0x1));
                                                            call_ptr(c, 0x00417828, 0x001c09e4)?;
                                                            c.g[RA][0] = sx(0x001c09ec);
                                                            c.g[A0][0] = c.g[SP][0];
                                                            call_ptr(c, 0x00317670, 0x001c09ec)?;
                                                            c.g[V1][0] = c.lw((c.g[S1][0] as u32)
                                                                .wrapping_add(0x6d0));
                                                            c.g[A0][0] =
                                                                sx((c.g[S0][0] as u32) << 2);
                                                            c.g[S0][0] = sx((c.g[S0][0] as u32)
                                                                .wrapping_add(0x1));
                                                            c.g[A1][0] = ((c.g[S2][0] as i64)
                                                                < 166i64)
                                                                as u64;
                                                            c.g[A0][0] = sx((c.g[A0][0] as u32)
                                                                .wrapping_add((c.g[V1][0] as u32)));
                                                            let t = c.g[A1][0] != 0u64;
                                                            c.sw((c.g[A0][0] as u32), c.g[V0][0]);
                                                            if t {
                                                                if event_due(c) {
                                                                    deliver_event_here(
                                                                        c, 0x001c09d0,
                                                                    );
                                                                }
                                                                continue 'l_1c09d0;
                                                            } else {
                                                                break 'b_1c0a08;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        c.cycles += 7;
        c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x60));
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0x50));
        c.lq(18, (c.g[SP][0] as u32).wrapping_add(0x40));
        c.lq(19, (c.g[SP][0] as u32).wrapping_add(0x30));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0x20));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x70));
        c.pc = t;
        return Ok(());
    });
}

// ---------------------------------------------------------------------------
// cFEStateNetQuickPlayPopup_OnWidgetCreated.

/// Epilogue of `cFEStateNetQuickPlayPopup_OnWidgetCreated` (the caller has
/// restored `s0`).
fn quick_play_exit(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 5;
    frame.restore(c, S1, 0x20);
    frame.restore(c, S2, 0x10);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// Call the widget's virtual method at vtable offset `slot` with `a1 = arg`.
fn quick_play_vcall(c: &mut Cpu, slot: u32, arg: u64, ret: u32) -> Flow {
    let widget = r32(c, S0);
    c.g[V0][0] = c.lw(widget.wrapping_add(0x8));
    c.g[A1][0] = arg;
    c.g[A0][0] = c.lh(r32(c, V0).wrapping_add(slot));
    c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(slot + 4));
    let method = r32(c, V1);
    c.g[A0][0] = sx(widget.wrapping_add(r32(c, A0)));
    call_ptr(c, method, ret)
}

/// What the matched widget needs.
#[derive(Clone, Copy)]
enum QuickPlayKind {
    /// A list box: fill it as an on/off list, `v24(1)`, store it.
    List { fill_ret: u32, call_ret: u32 },
    /// A static label: `v25(1)`, store it.
    Static { call_ret: u32 },
    /// Only store it.
    Pair,
}

/// One widget name of the quick-play popup: the string whose hash is compared
/// (`base + off`; `base` is what `a0` held after the previous miss), where
/// the widget is stored in the state, and what to do with it.
struct QuickPlayWidget {
    base: u32,
    off: u32,
    hash_ret: u32,
    field: u32,
    kind: QuickPlayKind,
}

const QUICK_PLAY_WIDGETS: [QuickPlayWidget; 15] = {
    use QuickPlayKind::*;
    const fn w(
        base: u32,
        off: u32,
        hash_ret: u32,
        field: u32,
        kind: QuickPlayKind,
    ) -> QuickPlayWidget {
        QuickPlayWidget {
            base,
            off,
            hash_ret,
            field,
            kind,
        }
    }
    [
        // LBRanked, LBRace, LBBigAir, LBSuperpipe, LBSlopeStyle
        w(
            0x46_0000,
            0x57f8,
            0x001c_10f8,
            0x70c,
            List {
                fill_ret: 0x001c_1110,
                call_ret: 0x001c_1128,
            },
        ),
        w(
            0x4a_0000,
            0x1c00,
            0x001c_1138,
            0x710,
            List {
                fill_ret: 0x001c_1150,
                call_ret: 0x001c_1168,
            },
        ),
        w(
            0x46_0000,
            0x5808,
            0x001c_1178,
            0x714,
            List {
                fill_ret: 0x001c_1190,
                call_ret: 0x001c_11a8,
            },
        ),
        w(
            0x46_0000,
            0x5818,
            0x001c_11b8,
            0x718,
            List {
                fill_ret: 0x001c_11d0,
                call_ret: 0x001c_11e8,
            },
        ),
        w(
            0x46_0000,
            0x5828,
            0x001c_11f8,
            0x71c,
            List {
                fill_ret: 0x001c_1210,
                call_ret: 0x001c_1228,
            },
        ),
        // StaticRanked, StaticRace, StaticBigAir, StaticSuperpipe, StaticSlopeStyle
        w(
            0x46_0000,
            0x5838,
            0x001c_1238,
            0x6f8,
            Static {
                call_ret: 0x001c_1258,
            },
        ),
        w(
            0x46_0000,
            0x5848,
            0x001c_1268,
            0x6fc,
            Static {
                call_ret: 0x001c_1288,
            },
        ),
        w(
            0x46_0000,
            0x5858,
            0x001c_1298,
            0x700,
            Static {
                call_ret: 0x001c_12b8,
            },
        ),
        w(
            0x46_0000,
            0x5868,
            0x001c_12c8,
            0x704,
            Static {
                call_ret: 0x001c_12e8,
            },
        ),
        w(
            0x46_0000,
            0x5878,
            0x001c_12f8,
            0x708,
            Static {
                call_ret: 0x001c_1318,
            },
        ),
        // RankedPair, RacePair, BigAirPair, SuperpipePair, SlopestylePair
        w(0x46_0000, 0x5890, 0x001c_1328, 0x720, Pair),
        w(0x46_0000, 0x58a0, 0x001c_1340, 0x724, Pair),
        w(0x46_0000, 0x58b0, 0x001c_1358, 0x728, Pair),
        w(0x46_0000, 0x58c0, 0x001c_1370, 0x72c, Pair),
        w(0x46_0000, 0x58d0, 0x001c_1388, 0x730, Pair),
    ]
};

/// `void cFEStateNetQuickPlayPopup_OnWidgetCreated(this, widget)` (0x1c1080):
/// show the new widget, then store it in the state field that goes with its
/// name hash: the OK button (`this+0x6f4`), the five ranked/race/big air/
/// superpipe/slope style list boxes, static labels and "pair" boxes, and the
/// hidden "searching" text (`this+0x6e4`).
pub fn c_fe_state_net_quick_play_popup_on_widget_created(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S2, 0x10);
        c.g[S0][0] = c.g[A1][0];
        frame.save(c, S1, 0x20);
        c.g[S2][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        let widget = r32(c, S0);
        c.g[V0][0] = c.lw(widget.wrapping_add(0x8));
        c.g[S1][0] = c.lw(widget.wrapping_add(0x38));
        // setVisible(1)
        c.g[A1][0] = 0x1;
        c.g[A0][0] = c.lh(r32(c, V0).wrapping_add(0x48));
        c.g[V1][0] = c.lw(r32(c, V0).wrapping_add(0x4c));
        let method = r32(c, V1);
        c.g[A0][0] = sx(widget.wrapping_add(r32(c, A0)));
        call_ptr(c, method, 0x001c_10b8)?;
        // "TextOK"
        c.g[A0][0] = 0x4a0000;
        c.g[A0][0] = sx(r32(c, A0).wrapping_add(0x1a80));
        call_ptr(c, 0x0031_7670, 0x001c_10c4)?;
        if c.g[S1][0] == c.g[V0][0] {
            c.cycles += 9;
            let this = r32(c, S2);
            let widget_ptr = c.g[S0][0];
            Addr(this.wrapping_add(0x6f4)).sw(c, widget_ptr);
            quick_play_vcall(c, 0xc8, 1, 0x001c_10e8)?;
            frame.restore(c, S0, 0x30);
            return quick_play_exit(c, frame);
        }
        for w in &QUICK_PLAY_WIDGETS {
            c.cycles += 4;
            c.g[A0][0] = sx(w.base.wrapping_add(w.off));
            call_ptr(c, 0x0031_7670, w.hash_ret)?;
            if c.g[S1][0] != c.g[V0][0] {
                continue;
            }
            match w.kind {
                QuickPlayKind::List { fill_ret, call_ret } => {
                    c.cycles += 12;
                    c.g[A0][0] = c.g[S2][0];
                    c.g[A1][0] = c.g[S0][0];
                    c.g[A2][0] = 0x1;
                    Callee {
                        addr: 0x001a_9ab8,
                        f: ee::s03::NetFEState_FillOnOffList,
                    }
                    .call(c, fill_ret)?;
                    quick_play_vcall(c, 0xc0, 1, call_ret)?;
                }
                QuickPlayKind::Static { call_ret } => {
                    c.cycles += 8;
                    quick_play_vcall(c, 0xc8, 1, call_ret)?;
                }
                QuickPlayKind::Pair => c.cycles += 2,
            }
            let this = r32(c, S2);
            let widget_ptr = c.g[S0][0];
            Addr(this.wrapping_add(w.field)).sw(c, widget_ptr);
            return quick_play_stored(c, frame);
        }
        // "TextSearching"
        c.cycles += 4;
        c.g[A0][0] = sx(0x0046_0000 + 0x58e0);
        call_ptr(c, 0x0031_7670, 0x001c_13a0)?;
        if c.g[S1][0] != c.g[V0][0] {
            frame.restore(c, S0, 0x30);
            return quick_play_exit(c, frame);
        }
        c.cycles += 7;
        let this = r32(c, S2);
        let widget_ptr = c.g[S0][0];
        Addr(this.wrapping_add(0x6e4)).sw(c, widget_ptr);
        quick_play_vcall(c, 0x48, 0, 0x001c_13c4)?;
        quick_play_stored(c, frame)
    });
}

/// The join after a stored widget: one more cycle, restore `s0`, leave.
fn quick_play_stored(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 1;
    frame.restore(c, S0, 0x30);
    quick_play_exit(c, frame)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, covered, machine, FAKE, LOG_RESULTS};
    use crate::difftest::{Rng, DATA, GAME_GP};
    use std::ptr::addr_of_mut;

    /// Gate of this module's callee fakes (a magic word in the machine's own
    /// RAM, so other tests in the binary run the real callees) and the call
    /// whose result a fake forces to 0 (1-based call number, 0 = none).
    pub(crate) const OC921_CTRL: u32 = DATA + 0x28_0000;
    pub(crate) const OC921_MAGIC: u32 = 0x0921_c0de;
    pub(crate) const OC921_ZERO_AT: u32 = OC921_CTRL + 4;
    /// Fixture objects live above the pointers the fakes hand out
    /// (`DATA..DATA + 0x40000`).
    pub(crate) const OC921_BASE: u32 = DATA + 0x8_0000;

    /// FNV-1a of the C string at `addr`, the fake `StrHash` (never 0).
    pub(crate) fn oc921_str_hash(c: &mut Cpu, addr: u32) -> u32 {
        let mut h: u32 = 0x811c_9dc5;
        for i in 0..64 {
            let b = c.bus.read8(addr.wrapping_add(i));
            if b == 0 {
                break;
            }
            h = (h ^ b as u32).wrapping_mul(0x0100_0193);
        }
        h | 1
    }

    pub(crate) fn oc921_put_str(c: &mut Cpu, addr: u32, s: &str) {
        for (i, b) in s.bytes().enumerate() {
            c.bus.write8(addr + i as u32, b);
        }
        c.bus.write8(addr + s.len() as u32, 0);
    }

    /// `StrHash(a0)` of the string in memory (gated).
    pub(crate) fn oc921_fake_str_hash(c: &mut Cpu) {
        if c.bus.read32(OC921_CTRL) != OC921_MAGIC {
            return fakes::fall_back(
                c,
                fakes::prev_cell(oc921_fake_str_hash),
                ee::s10::f_00317670__generated,
            );
        }
        let ra = c.g[RA][0] as u32;
        let name = c.g[A0][0] as u32;
        let hash = oc921_str_hash(c, name);
        fakes::fake(c, 0x0031_7670, ra);
        c.g[V0][0] = sx(hash);
    }

    macro_rules! oc921_gated {
        ($($fake:ident => $addr:literal, $slot:path, $generated:path;)*) => {
            $(
                pub(crate) fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OC921_CTRL) != OC921_MAGIC {
                        return fakes::fall_back(c, fakes::prev_cell($fake), $generated);
                    }
                    let ra = c.g[RA][0] as u32;
                    let n = c.bus.read32(fakes::LOG_COUNT);
                    fakes::fake(c, $addr, ra);
                    if c.bus.read32(OC921_ZERO_AT) == n + 1 {
                        c.g[V0][0] = 0;
                    }
                }
            )*

            pub(crate) fn oc921_install_fakes() {
                $( fakes::hook(addr_of_mut!($slot), $fake, fakes::prev_cell($fake)); )*
                fakes::hook(
                    addr_of_mut!(ee::s10::OVERRIDE_00317670),
                    oc921_fake_str_hash,
                    fakes::prev_cell(oc921_fake_str_hash),
                );
            }
        };
    }

    oc921_gated! {
        oc921_fake_assign_cstr => 0x003184e8, ee::s10::OVERRIDE_003184E8, ee::s10::f_003184e8__generated;
        oc921_fake_alloc_named => 0x00317d70, ee::s10::OVERRIDE_00317D70, ee::s10::f_00317d70__generated;
        oc921_fake_mem_free => 0x00317e98, ee::s10::OVERRIDE_00317E98, ee::s10::f_00317e98__generated;
        oc921_fake_new_named => 0x00317e30, ee::s10::OVERRIDE_00317E30, ee::s10::f_00317e30__generated;
        oc921_fake_set_string => 0x003a0c50, ee::s13::OVERRIDE_003A0C50, ee::s13::f_003a0c50__generated;
        oc921_fake_set_wide_string => 0x003a0e90, ee::s13::OVERRIDE_003A0E90, ee::s13::f_003a0e90__generated;
        oc921_fake_set_key_enabled => 0x001ce3c8, ee::s04::OVERRIDE_001CE3C8, ee::s04::f_001ce3c8__generated;
        oc921_fake_open_keyboard => 0x001a9150, ee::s03::OVERRIDE_001A9150, ee::s03::f_001a9150__generated;
        oc921_fake_net_widget_action => 0x001a8918, ee::s03::OVERRIDE_001A8918, ee::s03::f_001a8918__generated;
        oc921_fake_validate_room_name => 0x001bec78, ee::s03::OVERRIDE_001BEC78, ee::s03::f_001bec78__generated;
        oc921_fake_create_step1_ctor => 0x001b88f0, ee::s03::OVERRIDE_001B88F0, ee::s03::f_001b88f0__generated;
        oc921_fake_use_existing_ctor => 0x001ac010, ee::s03::OVERRIDE_001AC010, ee::s03::f_001ac010__generated;
        oc921_fake_account_mem_ctor => 0x001b6ed0, ee::s03::OVERRIDE_001B6ED0, ee::s03::f_001b6ed0__generated;
        oc921_fake_get_popup => 0x001a9c50, ee::s03::OVERRIDE_001A9C50, ee::s03::f_001a9c50__generated;
        oc921_fake_cfg_option_text => 0x001c57c0, ee::s03::OVERRIDE_001C57C0, ee::s03::f_001c57c0__generated;
        oc921_fake_show_popup => 0x001a9bc0, ee::s03::OVERRIDE_001A9BC0, ee::s03::f_001a9bc0__generated;
        oc921_fake_goto_next_state => 0x001a8a40, ee::s03::OVERRIDE_001A8A40, ee::s03::f_001a8a40__generated;
        oc921_fake_agreement_ctor => 0x001c0418, ee::s03::OVERRIDE_001C0418, ee::s03::f_001c0418__generated;
        oc921_fake_fill_on_off_list => 0x001a9ab8, ee::s03::OVERRIDE_001A9AB8, ee::s03::f_001a9ab8__generated;
        oc921_fake_sprintf_ascii => 0x002c2718, ee::s08::OVERRIDE_002C2718, ee::s08::f_002c2718__generated;
        oc921_fake_sprintf_ascii_2 => 0x002c27c0, ee::s08::OVERRIDE_002C27C0, ee::s08::f_002c27c0__generated;
        oc921_fake_lobby_connect => 0x0025b6d8, ee::s06::OVERRIDE_0025B6D8, ee::s06::f_0025b6d8__generated;
        oc921_fake_ustr_len => 0x003dbca0, ee::s14::OVERRIDE_003DBCA0, ee::s14::f_003dbca0__generated;
        oc921_fake_sprintf => 0x00417828, ee::s15::OVERRIDE_00417828, ee::s15::f_00417828__generated;
    }

    /// Switch this machine's fakes on.
    pub(crate) fn oc921_gate(c: &mut Cpu) {
        oc921_install_fakes();
        c.bus.write32(OC921_CTRL, OC921_MAGIC);
    }

    /// Vtable slots `{delta: i16 at slot, function at slot + 4}`, the
    /// function pointing at a fake method.
    pub(crate) fn oc921_vslots(c: &mut Cpu, r: &mut Rng, vt: u32, slots: &[u32]) {
        for (i, &slot) in slots.iter().enumerate() {
            c.bus.write16(vt + slot, (r.next_u32() & 0xfc) as u16);
            c.bus.write32(vt + slot + 4, FAKE + 0x10 * i as u32);
        }
    }

    /// The BELibrary-like world at `gp - 0x848`: `world + 0x8c` is the
    /// localisation object whose `+4` vtable has the method at `+0x20/+0x24`.
    pub(crate) fn oc921_world(c: &mut Cpu, r: &mut Rng, world: u32) {
        c.bus.write32(GAME_GP - 0x848, world);
        c.bus.write32(world + 0x8c, world + 0x100);
        c.bus.write32(world + 0x100 + 4, world + 0x200);
        oc921_vslots(c, r, world + 0x200, &[0x20]);
    }

    /// `cFEStateNetCreateRoomPopup_OnWidgetAction`: every message / misc-data
    /// path (rename, password, create valid / invalid, cancel, channel
    /// picked 0..3 and other, keyboard done 1/2/other, base-state forward).
    #[test]
    fn c_fe_state_net_create_room_popup_on_widget_action_matches() {
        covered(
            ee::s03::f_001bea38__generated,
            c_fe_state_net_create_room_popup_on_widget_action,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1b_ea38, 4);
                oc921_gate(c);
                let this = OC921_BASE;
                let (parent, vt, keyboard, widget) =
                    (this + 0x1000, this + 0x2000, this + 0x3000, this + 0x4000);
                c.bus.write32(this + 0x20, parent);
                c.bus.write32(parent + 8, vt);
                oc921_vslots(c, &mut r, vt, &[0x88]);
                c.bus.write32(this + 0x69c, keyboard);
                for off in [0x6d0, 0x6d4, 0x6dc, 0x6e4] {
                    c.bus.write32(this + off, r.next_u32());
                }
                for off in [0x1690, 0x168c, 0x1684, 0x1688] {
                    c.bus.write32(GAME_GP - off, r.next_u32());
                }
                // widget as a list box: the selected entry is `+0xa4`.
                c.bus.write8(widget + 0x318, 1);
                c.bus.write8(widget + 0x319, 0);
                let mut msg = 5u64;
                let mut misc = 0u32;
                let mut entry = 0u32;
                let mut kb_misc = 0u32;
                let mut list = widget;
                match seed % 20 {
                    0 => msg = 6,
                    1 => misc = 1,
                    2 => misc = 2,
                    3 => misc = 3,
                    4 => {
                        misc = 3;
                        c.bus.write32(OC921_ZERO_AT, 1);
                    }
                    5 => misc = 4,
                    6 => misc = 0,
                    7 => misc = u32::MAX,
                    8..=12 => {
                        msg = 9;
                        entry = [0, 1, 2, 3, 7][(seed % 20 - 8) as usize];
                    }
                    13 => {
                        msg = 9;
                        entry = u32::MAX;
                    }
                    14 => {
                        msg = 9;
                        list = widget + 0x100;
                    }
                    15..=17 => {
                        msg = 13;
                        kb_misc = seed % 20 - 14;
                    }
                    18 => msg = 7,
                    _ => msg = 0x20,
                }
                c.bus.write32(widget + 0x18, misc);
                c.bus.write32(widget + 0xa4, entry);
                c.bus.write32(this + 0x6d8, list);
                c.bus.write32(keyboard + 0x18, kb_misc);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(widget);
                c.g[A2][0] = msg;
            },
        );
    }

    /// `cFEStateLogin_OnWidgetCreated`: each widget name (use account, the two
    /// rectangles, create, existing, delete, none of them), with and without a
    /// stored account name.
    #[test]
    fn c_fe_state_login_on_widget_created_matches() {
        const NAMES: [(u32, &str); 6] = [
            (0x52c0, "TextUseAccount"),
            (0x52e0, "1rectangle"),
            (0x52f0, "4rectangle"),
            (0x5300, "TextCreateAccount"),
            (0x5318, "TextExistingAccount"),
            (0x5330, "TextDelectAccount"),
        ];
        covered(
            ee::s03::f_001bef60__generated,
            c_fe_state_login_on_widget_created,
            140,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1b_ef60, 12);
                oc921_gate(c);
                let widget = OC921_BASE + 0x1000;
                let vt = OC921_BASE + 0x2000;
                oc921_world(c, &mut r, OC921_BASE + 0x3000);
                oc921_vslots(c, &mut r, vt, &[0x48, 0xc8, 0x40]);
                for (off, name) in NAMES {
                    oc921_put_str(c, 0x0046_0000 + off, name);
                }
                let k = ((seed / 2) % 7) as usize;
                let hash = match NAMES.get(k) {
                    // `StrHash` of the string the widget is compared with.
                    Some((_, name)) => oc921_str_hash_of(c, name),
                    None => 0x1234_5679,
                };
                c.bus.write32(widget + 0x38, hash);
                c.bus.write32(widget + 8, vt);
                if seed % 2 == 0 {
                    oc921_put_str(c, LOGIN_ACCOUNT_STRING, "ab");
                } else {
                    c.bus.write8(LOGIN_ACCOUNT_STRING, 0);
                }
                c.g[A0][0] = sx(OC921_BASE);
                c.g[A1][0] = sx(widget);
            },
        );
    }

    /// `FNV(name)` as the fake `StrHash` computes it for a string in memory.
    pub(crate) fn oc921_str_hash_of(c: &mut Cpu, s: &str) -> u32 {
        let scratch = OC921_BASE + 0x7_0000;
        oc921_put_str(c, scratch, s);
        oc921_str_hash(c, scratch)
    }

    /// `cFEStateLogin_OnWidgetAction`: no popup, every button misc data
    /// (0 connect, 1 existing, 2 create, 3 account memory, other), the
    /// disconnect popup (with / without an old message, allocation failing),
    /// the online agreement and the base-state forward.
    #[test]
    fn c_fe_state_login_on_widget_action_matches() {
        covered(
            ee::s03::f_001bf758__generated,
            c_fe_state_login_on_widget_action,
            150,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1b_f758, 12);
                oc921_gate(c);
                let this = OC921_BASE;
                let popup = this + 0x1000;
                let lobby = this + 0x3000;
                let (x1, x2, list) = (this + 0x4000, this + 0x4100, this + 0x4200);
                let new_state = this + 0x5000;
                oc921_world(c, &mut r, this + 0x2000);
                c.bus.write32(GAME_GP - 0xc8, lobby);
                c.bus.write32(this + 0x10, r.next_u32());
                c.bus.write32(this + 0x40, x1);
                c.bus.write32(x1 + 0xd0, x2);
                c.bus.write32(x2 + 0x10, list);
                c.bus.write32(list + 0x18 + 4, this + 0x4300);
                c.bus.write32(new_state + 0x20, this + 0x5100);
                c.bus.write32(this + 0x5100 + 0x1c, r.next_u32());
                c.bus.write32(new_state + 0x1c, r.next_u32());
                let mut msg = 5u64;
                let mut misc = 0u32;
                let mut popup_arg = popup;
                c.bus.write32(this + 0x6d0, 0);
                match seed % 15 {
                    0 => popup_arg = 0,
                    1 => misc = 0,
                    2 => misc = 1,
                    3 => misc = 2,
                    4 => {
                        misc = 3;
                        c.bus.write32(LOG_RESULTS + 4, new_state);
                    }
                    5 => misc = 4,
                    6 => misc = u32::MAX,
                    7..=10 => {
                        msg = 6;
                        if seed % 2 == 0 {
                            c.bus.write32(this + 0x6d0, this + 0x6000);
                        }
                        if seed % 4 >= 2 {
                            // `operator_new` returns 0: call 7 (no old
                            // message) or 8 (old message freed first).
                            let at = 8 - (seed % 2);
                            c.bus.write32(OC921_ZERO_AT, at);
                        }
                    }
                    11 => msg = 8,
                    12 => msg = 7,
                    13 => msg = 0,
                    _ => msg = 0x44,
                }
                c.bus.write32(popup + 0x18, misc);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(popup_arg);
                c.g[A2][0] = msg;
            },
        );
    }

    /// `cFEStateOnlineAgreement_LoadTermsText`: allocates the 178-entry line
    /// table (freeing an old one first) and fills it with `StrHash` of the
    /// `kT_TOS%03d` names.
    #[test]
    fn c_fe_state_online_agreement_load_terms_text_matches() {
        covered(
            ee::s03::f_001c0540__generated,
            c_fe_state_online_agreement_load_terms_text,
            60,
            &|c, seed| {
                let _ = machine(c, seed, 0x1c_0540, 340);
                oc921_gate(c);
                let this = OC921_BASE;
                c.bus
                    .write32(this + 0x6d0, if seed % 2 == 0 { 0 } else { this + 0x6000 });
                c.g[A0][0] = sx(this);
            },
        );
    }

    /// `cFEStateNetQuickPlayPopup_OnWidgetCreated`: every widget name (OK
    /// button, five list boxes, five static labels, five pair boxes, the
    /// "searching" text) and an unknown one.
    #[test]
    fn c_fe_state_net_quick_play_popup_on_widget_created_matches() {
        covered(
            ee::s03::f_001c1080__generated,
            c_fe_state_net_quick_play_popup_on_widget_created,
            144,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1c_1080, 20);
                oc921_gate(c);
                let this = OC921_BASE;
                let widget = OC921_BASE + 0x1000;
                let vt = OC921_BASE + 0x2000;
                oc921_vslots(c, &mut r, vt, &[0x48, 0xc0, 0xc8]);
                // The strings the handler hashes, all different.
                let mut names: Vec<u32> = vec![0x004a_1a80];
                names.extend(QUICK_PLAY_WIDGETS.iter().map(|w| w.base + w.off));
                names.push(0x0046_58e0);
                for (i, &addr) in names.iter().enumerate() {
                    oc921_put_str(c, addr, &format!("name{i}"));
                }
                let k = (seed % 18) as usize;
                let hash = match names.get(k) {
                    Some(_) => oc921_str_hash_of(c, &format!("name{k}")),
                    None => 0x1234_5679,
                };
                c.bus.write32(widget + 0x38, hash);
                c.bus.write32(widget + 8, vt);
                c.g[A0][0] = sx(this);
                c.g[A1][0] = sx(widget);
            },
        );
    }
}
