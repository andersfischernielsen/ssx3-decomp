//! Staging module oc867 (wave wave251): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc867`.

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
    (0x001a_7970, rmp_init, "cFEStateReadMessagePopup_Init"),
    (0x001a_7b20, rmp_update, "cFEStateReadMessagePopup_Update"),
    (0x001a_7b40, rmp_v32, "cFEStateReadMessagePopup_v32"),
    (
        0x001a_7bb8,
        rmp_on_widget_action,
        "cFEStateReadMessagePopup_OnWidgetAction",
    ),
    (
        0x001a_7d10,
        rmp_on_widget_notify,
        "cFEStateReadMessagePopup_OnWidgetNotify",
    ),
    (0x001a_7d40, rmp_v31, "cFEStateReadMessagePopup_v31"),
    (
        0x001a_7d90,
        rmp_transition_in,
        "cFEStateReadMessagePopup_TransitionIn",
    ),
    (
        0x001a_7dd8,
        rmp_add_message,
        "cFEStateReadMessagePopup_AddMessage",
    ),
];

/// Fields of `cFEStateReadMessagePopup` (offsets as the generated code uses them).
const RMP_ENGINE: u32 = 0x10; // mpEngine
const RMP_PARENT: u32 = 0x20; // mpParentState (a cFEObj with a vtable at +8)
const RMP_SCREEN: u32 = 0x40; // mpScreen
const RMP_TEXT_COLOUR: u32 = 0x6ec; // mTextColour (copied to the scroller)
const RMP_SCROLL_INDEX: u32 = 0x6d4; // mScrollIndex
const RMP_SENDER: u32 = 0x6d0; // mszSender
const RMP_KEYBOARD: u32 = 0x69c; // mpKeyboard
const RMP_LIST: u32 = 0x6d8; // the message list header (S2 in AddMessage)
const RMP_LIST_BEGIN: u32 = 0x6dc; // first slot of the list
const RMP_LIST_END: u32 = 0x6e0; // one past the last used slot
const RMP_LIST_LIMIT: u32 = 0x6e4; // the message the list is limited by

/// Callees (generated addresses; each one goes through `call_ptr`).
const RMP_STR_HASH: u32 = 0x0031_7670; // GetHashValue32 (StrHash)
const RMP_SCREEN_CREATE: u32 = 0x0039_7cc0; // cUIEngine_addScreenByHashName
const RMP_SCREEN_PLAY: u32 = 0x0039_c870; // cUIScreen_playFrame
const RMP_SCREEN_FIND: u32 = 0x0039_d860; // cUIScreen_getObjectByHashName
const RMP_TEXTSCROLL_NAV: u32 = 0x003a_1310; // cUITextScroll_setInputMap
const RMP_REFRESH: u32 = 0x001a_81f0; // cFEStateReadMessagePopup_RefreshText
const RMP_LOAD: u32 = 0x001a_7f78; // cFEStateReadMessagePopup_LoadMessages
const RMP_ADD_SENT: u32 = 0x001a_7ed8; // cFEStateReadMessagePopup_AddSentMessage
const RMP_NET_UPDATE: u32 = 0x001a_8770; // NetFEState_Update
const RMP_NET_EVENT: u32 = 0x001a_8e40; // NetFEState_HandleNetEvent
const RMP_NET_WIDGET: u32 = 0x001a_8918; // NetFEState_OnWidgetAction
const RMP_NET_OPEN_KB: u32 = 0x001a_9090; // NetFEState_OpenKeyboardStr
const RMP_KB_KEY: u32 = 0x001c_e3c8; // cKeyboardPopup_SetKeyEnabled
const RMP_KB_QUICK: u32 = 0x001c_e468; // cKeyboardPopup_SetFunctionKeysEnabled
const RMP_BUDDY_CREATE: u32 = 0x0026_0f80; // BuddyMan_Create
const RMP_BUDDY_CONNECTED: u32 = 0x0026_1460; // BuddyMan_IsConnected
const RMP_BUDDY_RECONNECT: u32 = 0x0026_13c8; // BuddyMan_Reconnect
const RMP_BUDDY_PRESENCE: u32 = 0x0026_2768; // BuddyMan_SetPresence
const RMP_BUDDY_SEND: u32 = 0x0026_2690; // BuddyMan_SendMessage
const RMP_LOBBY_DISCONNECT: u32 = 0x0025_b800; // LobbyMan_Disconnect
const RMP_MEMMOVE: u32 = 0x0041_610c; // memmove
const RMP_LIST_TAIL: u32 = 0x001d_b918; // the list's insert-or-notify helper of AddMessage

/// Store the low word-or-double of register `reg` at `addr` (`sw`).
#[inline(always)]
fn rmp_sw(c: &mut Cpu, addr: u32, reg: usize) {
    let v = c.g[reg][0];
    Addr(addr).sw(c, v);
}

/// Register `r` as a guest address.
#[inline(always)]
fn rmp_addr(c: &Cpu, r: usize) -> u32 {
    c.g[r][0] as u32
}

/// The BuddyMan singleton (`gp+0x238`).
#[inline(always)]
fn rmp_buddy(c: &mut Cpu) -> u64 {
    Addr(rmp_addr(c, GP).wrapping_add(0x238)).lw(c)
}

/// The LobbyMan singleton (`gp-0xc8`).
#[inline(always)]
fn rmp_lobby(c: &mut Cpu) -> u64 {
    Addr(rmp_addr(c, GP).wrapping_sub(0xc8)).lw(c)
}

/// `GetHashValue32` of the string at `name`: returns to `ret` with the hash in v0.
#[inline]
fn rmp_hash(c: &mut Cpu, name: u32, ret: u32) -> Flow {
    c.g[A0][0] = sx(name);
    call_ptr(c, RMP_STR_HASH, ret)
}

/// `cUIScreen_getObjectByHashName(screen, hash)`: the screen is `this+0x40`,
/// the hash is in v0 (as the generated code leaves it).
#[inline]
fn rmp_find(c: &mut Cpu, this: u32, ret: u32) -> Flow {
    c.g[A0][0] = Addr(this.wrapping_add(RMP_SCREEN)).lw(c);
    c.g[A1][0] = c.g[V0][0];
    call_ptr(c, RMP_SCREEN_FIND, ret)
}

/// `cFEStateReadMessagePopup_onPopupEvent`-style callback of the parent state:
/// `parent->vt[slot 17](parent, this, 20)` (case 6 of the widget handler).
#[inline]
fn rmp_popup_event(c: &mut Cpu) -> Flow {
    c.cycles += 10;
    c.g[A3][0] = Addr(rmp_addr(c, S0).wrapping_add(RMP_PARENT)).lw(c);
    c.g[A1][0] = c.g[S0][0];
    c.g[A2][0] = 0x14;
    c.g[V0][0] = Addr((c.g[A3][0] as u32).wrapping_add(8)).lw(c);
    c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x88)).lh(c);
    c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x8c)).lw(c);
    let method = c.g[V1][0] as u32;
    c.g[A0][0] = sx((c.g[A3][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, method, 0x001a_7c28)
}

/// Opens the keyboard for a reply (widget event 5): the keyboard is titled with
/// the sender (`A2`, loaded by the caller), empty, and includes key 0x4B.
#[inline]
fn rmp_open_keyboard(c: &mut Cpu) -> Flow {
    c.cycles += 22;
    c.g[A3][0] = sx(0x004a_1408); // str_empty_d
    c.g[T0][0] = 0;
    c.g[T1][0] = 0;
    c.g[T2][0] = 0x3f;
    c.g[A0][0] = c.g[S0][0];
    c.g[A1][0] = 0;
    call_ptr(c, RMP_NET_OPEN_KB, 0x001a_7c50)?;
    let kb = Addr(rmp_addr(c, S0).wrapping_add(RMP_KEYBOARD)).lw(c);
    c.g[V0][0] = kb;
    c.g[A1][0] = 0x4b;
    c.g[A2][0] = 1;
    c.g[A3][0] = 2;
    Addr((kb as u32).wrapping_add(0x43c)).sw(c, 0);
    c.g[A0][0] = Addr(rmp_addr(c, S0).wrapping_add(RMP_KEYBOARD)).lw(c);
    call_ptr(c, RMP_KB_KEY, 0x001a_7c6c)?;
    c.g[A0][0] = Addr(rmp_addr(c, S0).wrapping_add(RMP_KEYBOARD)).lw(c);
    c.g[A1][0] = 1;
    call_ptr(c, RMP_KB_QUICK, 0x001a_7c78)?;
    let kb2 = Addr(rmp_addr(c, S0).wrapping_add(RMP_KEYBOARD)).lw(c);
    c.g[V1][0] = kb2;
    c.g[V0][0] = 1;
    rmp_sw(c, (kb2 as u32).wrapping_add(0x438), V0);
    Ok(())
}

/// The default widget event: forwarded to the cFEStateNet handler.
#[inline]
fn rmp_default(c: &mut Cpu) -> Flow {
    c.cycles += 2;
    c.g[A0][0] = c.g[S0][0];
    call_ptr(c, RMP_NET_WIDGET, 0x001a_7d00)
}

/// The keyboard finished (widget event 13): adds the typed text to the
/// conversation, makes sure BuddyMan exists and is connected, sends the text to
/// the sender and refreshes the list. A1 holds the keyboard on entry.
#[inline]
fn rmp_add_sent(c: &mut Cpu) -> Flow {
    c.cycles += 6;
    c.g[A0][0] = c.g[S0][0];
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x74));
    call_ptr(c, RMP_ADD_SENT, 0x001a_7c94)?;
    c.g[V0][0] = rmp_buddy(c);
    if c.g[V0][0] == 0 {
        c.cycles += 2;
        call_ptr(c, RMP_BUDDY_CREATE, 0x001a_7ca8)?;
    }
    c.cycles += 4;
    c.g[A0][0] = rmp_buddy(c);
    call_ptr(c, RMP_BUDDY_CONNECTED, 0x001a_7cb0)?;
    let connected = c.g[V0][0] != 0;
    c.g[V0][0] = rmp_buddy(c);
    if !connected {
        c.cycles += 3;
        c.g[A0][0] = rmp_buddy(c);
        call_ptr(c, RMP_BUDDY_RECONNECT, 0x001a_7cc0)?;
        c.g[V0][0] = rmp_buddy(c);
    }
    c.cycles += 2;
    if c.g[V0][0] != 0 {
        c.g[A2][0] = Addr(rmp_addr(c, S0).wrapping_add(RMP_KEYBOARD)).lw(c);
    } else {
        c.cycles += 3;
        c.g[A0][0] = rmp_lobby(c);
        call_ptr(c, RMP_LOBBY_DISCONNECT, 0x001a_7cd4)?;
        c.g[A2][0] = Addr(rmp_addr(c, S0).wrapping_add(RMP_KEYBOARD)).lw(c);
    }
    c.cycles += 8;
    c.g[A0][0] = rmp_buddy(c);
    c.g[A1][0] = Addr(rmp_addr(c, S0).wrapping_add(RMP_SENDER)).lw(c);
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_add(0x74));
    call_ptr(c, RMP_BUDDY_SEND, 0x001a_7ce8)?;
    c.g[A0][0] = c.g[S0][0];
    call_ptr(c, RMP_REFRESH, 0x001a_7cf0)
}

/// `cFEStateReadMessagePopup_Init` (0x1a7970): creates the "282ReadPrivateMessage"
/// screen on the popup's engine and wires its chat scroller. The agreement slider
/// gets bit 0 of its attributes and the up and down arrows and its outline;
/// "textScrollChat", when it exists, gets its input map, visibility, the 0x80
/// attribute, a red colour, a copy of the popup's text colour and the slider.
pub fn rmp_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x60);
        frame.save(c, 19, 0x20);
        c.g[S3][0] = c.g[A0][0];
        frame.save(c, 17, 0x40);
        frame.save(c, 18, 0x30);
        frame.save_ra(c, 0x10);
        frame.save(c, 16, 0x50);
        let this = rmp_addr(c, S3);
        c.g[S0][0] = Addr(this.wrapping_add(RMP_ENGINE)).lw(c);
        rmp_hash(c, 0x0046_19a0, 0x001a_799c)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S3][0];
        c.g[A3][0] = 0;
        call_ptr(c, RMP_SCREEN_CREATE, 0x001a_79b0)?;
        let screen = c.g[V0][0];
        Addr(this.wrapping_add(RMP_SCREEN)).sw(c, screen);
        if screen != 0 {
            c.cycles += 4;
            c.g[A0][0] = screen;
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            call_ptr(c, RMP_SCREEN_PLAY, 0x001a_79c8)?;
        }

        c.cycles += 44;
        rmp_hash(c, 0x0046_19b8, 0x001a_79d4)?;
        rmp_find(c, this, 0x001a_79e0)?;
        c.g[S2][0] = c.g[V0][0];
        let bar = rmp_addr(c, S2);
        c.g[V0][0] = Addr(bar.wrapping_add(0x14)).lw(c) | 1;
        rmp_sw(c, bar.wrapping_add(0x14), V0);
        rmp_hash(c, 0x0046_19c8, 0x001a_79fc)?;
        rmp_find(c, this, 0x001a_7a08)?;
        c.g[S1][0] = c.g[V0][0];
        rmp_hash(c, 0x0046_19d8, 0x001a_7a18)?;
        rmp_find(c, this, 0x001a_7a24)?;
        c.g[S0][0] = c.g[V0][0];
        rmp_hash(c, 0x0046_19e8, 0x001a_7a34)?;
        rmp_find(c, this, 0x001a_7a40)?;
        c.g[V1][0] = 2;
        let bar = rmp_addr(c, S2);
        rmp_sw(c, bar.wrapping_add(0x80), S0);
        rmp_sw(c, bar.wrapping_add(0x7c), S1);
        rmp_sw(c, bar.wrapping_add(0x88), V1);
        rmp_sw(c, bar.wrapping_add(0x84), V0);
        rmp_hash(c, 0x0046_19f8, 0x001a_7a60)?;
        rmp_find(c, this, 0x001a_7a6c)?;
        c.g[S0][0] = c.g[V0][0];
        c.g[A0][0] = c.g[S0][0];
        if c.g[S0][0] != 0 {
            c.cycles += 35;
            let text = rmp_addr(c, S0);
            c.g[V0][0] = Addr(text.wrapping_add(0x74)).lw(c) | 1;
            c.g[A1][0] = 1;
            rmp_sw(c, text.wrapping_add(0x74), V0);
            call_ptr(c, RMP_TEXTSCROLL_NAV, 0x001a_7a8c)?;

            // setVisible(1): vtable slot 8 of the scroller.
            let text = rmp_addr(c, S0);
            c.g[V0][0] = Addr(text.wrapping_add(8)).lw(c);
            c.g[A1][0] = 1;
            c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x48)).lh(c);
            c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x4c)).lw(c);
            let set_visible = c.g[V1][0] as u32;
            c.g[A0][0] = sx(text.wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, set_visible, 0x001a_7aa4)?;

            // setColour(1, 0, 0, 0) on a stack copy: attribute 0x80 first.
            let text = rmp_addr(c, S0);
            c.g[V0][0] = Addr(text.wrapping_add(0x14)).lw(c) | 0x80;
            c.g[A1][0] = c.g[SP][0];
            c.g[V1][0] = Addr(text.wrapping_add(8)).lw(c);
            c.g[AT][0] = 0x3f80_0000;
            c.mtc1(1, 0);
            rmp_sw(c, text.wrapping_add(0x14), V0);
            let vt = rmp_addr(c, V1);
            c.g[A0][0] = Addr(vt.wrapping_add(0x58)).lh(c);
            let colour = sp(c);
            c.swc1(0, colour);
            Addr(colour.wrapping_add(4)).sw(c, 0);
            Addr(colour.wrapping_add(8)).sw(c, 0);
            Addr(colour.wrapping_add(0xc)).sw(c, 0);
            c.g[V0][0] = Addr(vt.wrapping_add(0x5c)).lw(c);
            let set_colour = c.g[V0][0] as u32;
            c.g[A0][0] = sx(text.wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, set_colour, 0x001a_7ae0)?;

            // The popup's text colour (16 bytes at 0x6ec) becomes the scroller's
            // colour at +0xa8; the slider goes to +0xb8.
            let src_a = rmp_addr(c, S3);
            let src_b = rmp_addr(c, S3);
            let text = rmp_addr(c, S0);
            c.ldl(2, src_a.wrapping_add(0x6f3));
            c.ldr(2, src_b.wrapping_add(RMP_TEXT_COLOUR));
            c.ldl(3, src_b.wrapping_add(0x6fb));
            c.ldr(3, src_b.wrapping_add(0x6f4));
            c.sdl(2, text.wrapping_add(0xaf));
            c.sdr(2, text.wrapping_add(0xa8));
            c.sdl(3, text.wrapping_add(0xb7));
            c.sdr(3, text.wrapping_add(0xb0));
            rmp_sw(c, text.wrapping_add(0xb8), S2);
        }

        c.cycles += 7;
        frame.restore(c, 16, 0x50);
        frame.restore(c, 17, 0x40);
        frame.restore(c, 18, 0x30);
        frame.restore(c, 19, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    });
}

/// `cFEStateReadMessagePopup_Update` (0x1a7b20): just the cFEStateNet update.
pub fn rmp_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        call_ptr(c, RMP_NET_UPDATE, 0x001a_7b30)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `func_001A7B40` (0x1a7b40), the buddy presence vtable entry 32: makes sure the
/// BuddyMan exists and is connected (LobbyMan is dropped when it is missing),
/// then publishes presence "in the messenger" (SetPresence 1, 10, 0, 0, 0).
pub fn rmp_v32(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V0][0] = rmp_buddy(c);
        if c.g[V0][0] == 0 {
            c.cycles += 2;
            call_ptr(c, RMP_BUDDY_CREATE, 0x001a_7b58)?;
        }
        c.cycles += 4;
        c.g[A0][0] = rmp_buddy(c);
        call_ptr(c, RMP_BUDDY_CONNECTED, 0x001a_7b60)?;
        let connected = c.g[V0][0] != 0;
        c.g[V0][0] = rmp_buddy(c);
        if !connected {
            c.cycles += 3;
            c.g[A0][0] = rmp_buddy(c);
            call_ptr(c, RMP_BUDDY_RECONNECT, 0x001a_7b70)?;
            c.g[V0][0] = rmp_buddy(c);
        }
        c.cycles += 2;
        c.g[A0][0] = rmp_buddy(c);
        if c.g[V0][0] == 0 {
            c.cycles += 3;
            c.g[A0][0] = rmp_lobby(c);
            call_ptr(c, RMP_LOBBY_DISCONNECT, 0x001a_7b84)?;
            c.g[A0][0] = rmp_buddy(c);
        }
        c.cycles += 9;
        c.g[A1][0] = 1;
        c.g[A2][0] = 0xa;
        c.g[A3][0] = 0;
        c.g[T0][0] = 0;
        c.g[T1][0] = 0;
        call_ptr(c, RMP_BUDDY_PRESENCE, 0x001a_7ba0)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `cFEStateReadMessagePopup_OnWidgetAction` (0x1a7bb8): the popup's widget
/// events. Event 6 tells the parent state the popup was backed out of (its
/// vtable slot, with 20); event 5 opens the keyboard; event 13 (the keyboard
/// finished) adds the reply and sends it; event 7 is ignored; the rest go to the
/// cFEStateNet handler. `A1` is the widget, `A2` the event.
pub fn rmp_on_widget_action(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 6;
        frame.save(c, 16, 0x10);
        frame.save_ra(c, 0);
        let msg = c.g[A2][0];
        c.g[S0][0] = c.g[A0][0];
        // Whether the event falls through the block that ends in the default call
        // (its tail adds one cycle); event 6 and event 13 leave it early.
        let mut through_default_tail = true;
        if msg == 6 {
            rmp_popup_event(c)?;
            through_default_tail = false;
        } else if msg < 7 {
            c.cycles += 3;
            c.g[V0][0] = 5;
            c.cycles += 2;
            if msg == 5 {
                c.g[A2][0] = Addr(rmp_addr(c, S0).wrapping_add(RMP_SENDER)).lw(c);
                rmp_open_keyboard(c)?;
            } else {
                c.cycles += 2;
                rmp_default(c)?;
            }
        } else {
            c.cycles += 3;
            c.cycles += 3;
            c.g[V0][0] = 0xd;
            if msg != 7 {
                c.cycles += 2;
                if msg == 13 {
                    c.g[A1][0] = Addr(rmp_addr(c, S0).wrapping_add(RMP_KEYBOARD)).lw(c);
                    rmp_add_sent(c)?;
                    through_default_tail = false;
                } else {
                    c.cycles += 2;
                    rmp_default(c)?;
                }
            }
        }
        if through_default_tail {
            c.cycles += 1;
        }
        frame.restore(c, 16, 0x10);
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `cFEStateReadMessagePopup_OnWidgetNotify` (0x1a7d10): the notify event 4
/// stores the scroll index and refreshes the conversation; the result is 0 for
/// event 4 and 0x101 otherwise (the notify hands the rest back).
pub fn rmp_on_widget_notify(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[V0][0] = 4;
        frame.save_ra(c, 0);
        if c.g[A2][0] == c.g[V0][0] {
            c.cycles += 3;
            {
                let slot = rmp_addr(c, A0).wrapping_add(RMP_SCROLL_INDEX);
                rmp_sw(c, slot, A3);
            };
            call_ptr(c, RMP_REFRESH, 0x001a_7d30)?;
            c.g[V0][0] = 0;
        } else {
            c.cycles += 2;
            c.g[V0][0] = 0x101;
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `func_001A7D40` (0x1a7d40), the online-event vtable entry 31: event 265 (new
/// private messages) loads the sender's messages and redraws the conversation;
/// other events go to the cFEStateNet handler. Returns 0 for 265, else the
/// handler's result.
pub fn rmp_v31(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 6;
        let frame = Frame::open(c, 0x20);
        c.g[V0][0] = 0x109;
        frame.save(c, 16, 0x10);
        frame.save_ra(c, 0);
        let is_new_messages = c.g[A1][0] == c.g[V0][0];
        c.g[S0][0] = c.g[A0][0];
        if !is_new_messages {
            c.cycles += 4;
            call_ptr(c, RMP_NET_EVENT, 0x001a_7d60)?;
            frame.restore(c, 16, 0x10);
        } else {
            c.cycles += 6;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, RMP_LOAD, 0x001a_7d70)?;
            c.g[A0][0] = c.g[S0][0];
            call_ptr(c, RMP_REFRESH, 0x001a_7d78)?;
            c.g[V0][0] = 0;
            frame.restore(c, 16, 0x10);
        }
        c.cycles += 3;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `cFEStateReadMessagePopup_TransitionIn` (0x1a7d90): the popup's transition
/// hook: the base's vtable entry (+0x100), then loads and redraws the messages.
/// Returns 1.
pub fn rmp_transition_in(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 18;
        let frame = Frame::open(c, 0x20);
        frame.save(c, 16, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = Addr(rmp_addr(c, S0).wrapping_add(8)).lw(c);
        c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x100)).lh(c);
        c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x104)).lw(c);
        let method = c.g[V1][0] as u32;
        c.g[A0][0] = sx(rmp_addr(c, S0).wrapping_add(c.g[A0][0] as u32));
        call_ptr(c, method, 0x001a_7db4)?;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, RMP_LOAD, 0x001a_7dbc)?;
        c.g[A0][0] = c.g[S0][0];
        call_ptr(c, RMP_REFRESH, 0x001a_7dc4)?;
        frame.restore(c, 16, 0x10);
        c.g[V0][0] = 1;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `cFEStateReadMessagePopup_AddMessage` (0x1a7dd8): inserts the message in A1
/// into the conversation list. While the list holds at least 0x1f entries, the
/// oldest entries are dropped (each one that exists gets its virtual call
/// with 3); then the message goes to the first free slot or is announced to the
/// slot's owner (0x1db918) when it equals the list's limit.
pub fn rmp_add_message(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 14;
        let frame = Frame::open(c, 0x50);
        frame.save(c, 16, 0x40);
        frame.save(c, 17, 0x30);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, 18, 0x20);
        frame.save_ra(c, 0x10);
        {
            let slot = frame.at(c, 0);
            rmp_sw(c, slot, A1);
        };
        let this = rmp_addr(c, S0);
        c.g[V0][0] = Addr(this.wrapping_add(RMP_LIST_END)).lw(c);
        c.g[V1][0] = Addr(this.wrapping_add(RMP_LIST_BEGIN)).lw(c);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
        c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 2) as u32);
        c.g[V0][0] = (c.g[V0][0] < 0x1f) as u64;
        if c.g[V0][0] != 0 {
            c.g[A1][0] = Addr(this.wrapping_add(RMP_LIST_END)).lw(c);
        } else {
            c.cycles += 2;
            c.g[S2][0] = sx(this.wrapping_add(RMP_LIST));
            c.g[A0][0] = Addr(this.wrapping_add(RMP_LIST_BEGIN)).lw(c);
            loop {
                c.cycles += 3;
                c.g[S1][0] = Addr(rmp_addr(c, A0)).lw(c);
                c.g[V1][0] = sx(rmp_addr(c, A0).wrapping_add(4));
                if c.g[S1][0] != 0 {
                    c.cycles += 3;
                    c.g[V0][0] = Addr(this.wrapping_add(RMP_LIST_END)).lw(c);
                    let at_end = c.g[V1][0] == c.g[V0][0];
                    c.g[A1][0] = c.g[V1][0];
                    if !at_end {
                        c.cycles += 6;
                        c.g[A2][0] = Addr(rmp_addr(c, S2).wrapping_add(8)).lw(c);
                        c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(c.g[V1][0] as u32));
                        c.g[A2][0] = sx(((c.g[A2][0] as u32) as i32 >> 2) as u32);
                        c.g[A2][0] = sx((c.g[A2][0] as u32) << 2);
                        call_ptr(c, RMP_MEMMOVE, 0x001a_7e44)?;
                        c.g[V0][0] = Addr(this.wrapping_add(RMP_LIST_END)).lw(c);
                    }
                    c.cycles += 8;
                    c.g[A1][0] = 3;
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(4));
                    rmp_sw(c, this.wrapping_add(RMP_LIST_END), V0);
                    let entry = rmp_addr(c, S1);
                    c.g[V1][0] = Addr(entry.wrapping_add(0xc)).lw(c);
                    c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(8)).lh(c);
                    c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(0xc)).lw(c);
                    let method = c.g[V0][0] as u32;
                    c.g[A0][0] = sx(entry.wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, method, 0x001a_7e68)?;
                }
                c.cycles += 7;
                c.g[V0][0] = Addr(this.wrapping_add(RMP_LIST_END)).lw(c);
                c.g[V1][0] = Addr(this.wrapping_add(RMP_LIST_BEGIN)).lw(c);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(c.g[V1][0] as u32));
                c.g[V0][0] = sx(((c.g[V0][0] as u32) as i32 >> 2) as u32);
                c.g[V0][0] = (c.g[V0][0] < 0x1f) as u64;
                if c.g[V0][0] == 0 {
                    c.g[A0][0] = Addr(this.wrapping_add(RMP_LIST_BEGIN)).lw(c);
                    if event_due(c) {
                        deliver_event_here(c, 0x001a_7e18);
                    }
                } else {
                    c.cycles += 1;
                    c.g[A1][0] = Addr(this.wrapping_add(RMP_LIST_END)).lw(c);
                    break;
                }
            }
        }

        // Store the message in the first free slot, or announce it when the list
        // is at its limit (A1 equals the limit word).
        c.cycles += 3;
        c.g[V0][0] = Addr(this.wrapping_add(RMP_LIST_LIMIT)).lw(c);
        let at_limit = c.g[A1][0] == c.g[V0][0];
        c.g[A0][0] = sx(this.wrapping_add(RMP_LIST));
        if at_limit {
            c.cycles += 2;
            c.g[A2][0] = c.g[SP][0];
            call_ptr(c, RMP_LIST_TAIL, 0x001a_7ebc)?;
        } else {
            c.cycles += 3;
            c.g[A0][0] = Addr(rmp_addr(c, A0).wrapping_add(8)).lw(c);
            let no_slot = c.g[A0][0] == 0;
            c.g[V0][0] = Addr(frame.at(c, 0)).lw(c);
            if !no_slot {
                c.cycles += 1;
                {
                    let slot = rmp_addr(c, A0);
                    rmp_sw(c, slot, V0);
                };
            }
            c.cycles += 4;
            c.g[V0][0] = Addr(this.wrapping_add(RMP_LIST_END)).lw(c);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(4));
            rmp_sw(c, this.wrapping_add(RMP_LIST_END), V0);
        }

        c.cycles += 6;
        frame.restore(c, 16, 0x40);
        frame.restore(c, 17, 0x30);
        frame.restore(c, 18, 0x20);
        frame.restore_ra(c, 0x10);
        frame.close(c);
        Ok(())
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{machine, vtable};
    use crate::difftest::{check, DATA, GAME_GP};

    /// Points the popup's BuddyMan and LobbyMan globals at test objects (or at
    /// null BuddyMan for `buddy_null`).
    fn globals(c: &mut Cpu, buddy_null: bool) {
        c.bus
            .write32(GAME_GP + 0x238, if buddy_null { 0 } else { DATA + 0x700 });
        c.bus.write32(GAME_GP - 0xc8, DATA + 0x780);
    }

    #[test]
    fn rmp_init_matches() {
        check(ee::s03::f_001a7970__generated, rmp_init, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x001a_7970, 16);
            let this = DATA;
            c.g[A0][0] = this as u64;
            c.bus.write32(this + RMP_ENGINE, DATA + 0x100);
            c.bus.write32(this + RMP_SCREEN, 0);
            for i in 0..4 {
                c.bus.write32(this + RMP_TEXT_COLOUR + 4 * i, r.next_u32());
            }
            c.bus.write32(this + RMP_SENDER, r.next_u32());
        });
    }

    #[test]
    fn rmp_update_matches() {
        check(
            ee::s03::f_001a7b20__generated,
            rmp_update,
            100,
            &|c, seed| {
                machine(c, seed, 0x001a_7b20, 4);
            },
        );
    }

    #[test]
    fn rmp_v32_matches() {
        check(ee::s03::f_001a7b40__generated, rmp_v32, 200, &|c, seed| {
            machine(c, seed, 0x001a_7b40, 8);
            globals(c, seed % 4 == 0);
        });
    }

    #[test]
    fn rmp_on_widget_action_matches() {
        check(
            ee::s03::f_001a7bb8__generated,
            rmp_on_widget_action,
            300,
            &|c, seed| {
                let mut r = machine(c, seed, 0x001a_7bb8, 12);
                const MSGS: [u64; 9] = [6, 5, 7, 13, 0, 1, 4, 8, 12];
                let this = DATA + 0x800;
                c.g[A0][0] = this as u64;
                c.g[A1][0] = r.next_u32() as u64;
                c.g[A2][0] = MSGS[seed as usize % MSGS.len()];
                c.bus.write32(this + RMP_PARENT, DATA + 0x400);
                let vt = vtable(c, &mut r, &[0x88]);
                c.bus.write32(DATA + 0x408, vt);
                c.bus.write32(this + RMP_KEYBOARD, DATA + 0x600);
                c.bus.write32(this + RMP_SENDER, r.next_u32());
                globals(c, seed % 4 == 0);
            },
        );
    }

    #[test]
    fn rmp_on_widget_notify_matches() {
        check(
            ee::s03::f_001a7d10__generated,
            rmp_on_widget_notify,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x001a_7d10, 4);
                c.g[A0][0] = (DATA + 0x800) as u64;
                c.g[A2][0] = if seed % 2 == 0 {
                    4
                } else {
                    r.next_u32() as u64
                };
                c.g[A3][0] = r.next_u32() as u64;
            },
        );
    }

    #[test]
    fn rmp_v31_matches() {
        check(ee::s03::f_001a7d40__generated, rmp_v31, 200, &|c, seed| {
            let mut r = machine(c, seed, 0x001a_7d40, 4);
            c.g[A0][0] = (DATA + 0x800) as u64;
            c.g[A1][0] = if seed % 2 == 0 {
                0x109
            } else {
                r.next_u32() as u64
            };
        });
    }

    #[test]
    fn rmp_transition_in_matches() {
        check(
            ee::s03::f_001a7d90__generated,
            rmp_transition_in,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x001a_7d90, 4);
                let this = DATA + 0x800;
                c.g[A0][0] = this as u64;
                let vt = vtable(c, &mut r, &[0x100]);
                c.bus.write32(this + 8, vt);
            },
        );
    }

    #[test]
    fn rmp_add_message_matches() {
        check(
            ee::s03::f_001a7dd8__generated,
            rmp_add_message,
            200,
            &|c, seed| {
                let mut r = machine(c, seed, 0x001a_7dd8, 6);
                let this = DATA + 0x800;
                let msg = r.next_u32();
                c.g[A0][0] = this as u64;
                c.g[A1][0] = msg as u64;
                let start = DATA + 0x1000;
                let n = if seed % 4 == 0 {
                    seed % 0x1f
                } else {
                    0x1f + seed % 8
                };
                let end = start + 4 * n;
                let vt = vtable(c, &mut r, &[8]);
                for i in 0..n.max(1) {
                    let obj = DATA + 0x2000 + 0x10 * i;
                    c.bus.write32(obj + 0xc, vt);
                    c.bus.write32(start + 4 * i, obj); // never 0: a zero entry skips the shift and the loop never ends
                }
                c.bus.write32(this + RMP_LIST_BEGIN, start);
                c.bus.write32(this + RMP_LIST_END, end);
                c.bus.write32(
                    this + RMP_LIST_LIMIT,
                    if seed % 3 == 0 { msg } else { msg ^ 0x10 },
                );
            },
        );
    }
}
