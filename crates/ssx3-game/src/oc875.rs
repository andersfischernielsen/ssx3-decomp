//! Staging module oc875 (wave wave251): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc875`.

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
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x001a_e128,
    cfe_state_challenge_popup_on_widget_created,
    "cFEStateChallengePopup_OnWidgetCreated",
)];

/// The string table the generated body addresses as `0x460000 + offset`.
const POPUP_STR_BASE: u64 = 0x0046_0000;

// The widget's name hash is compared with the hash of each name in turn.
const POPUP_NAME_TEXT_TITLE: u32 = 0x1520;
const POPUP_NAME_TEXT_PLAYER_NAME: u32 = 0x3108;
const POPUP_NAME_TEXT_SELECT_PEAK_EVENT: u32 = 0x3118;
const POPUP_NAME_TEXT_SELECT_EVENT: u32 = 0x3130;
const POPUP_NAME_PEAK_LABEL: u32 = 0x3140;
const POPUP_NAME_COURSE_LABEL: u32 = 0x3168;
const POPUP_NAME_EVENT_LABEL: u32 = 0x3190;
const POPUP_NAME_TEXT_RANKED_GAME: u32 = 0x31b8;
const POPUP_NAME_TEXT_SUBMIT: u32 = 0x31c8;
const POPUP_NAME_TEXT_CANCEL: u32 = 0x31e8;
const POPUP_NAME_LISTBOX_RANKED_GAME: u32 = 0x31f8;
const POPUP_NAME_LISTBOX_CPU_RIDERS: u32 = 0x3210;
const POPUP_NAME_LISTBOX_POWER_UPS: u32 = 0x3238;
const POPUP_NAME_LISTBOX_MULTIPLIERS: u32 = 0x3248;
const POPUP_NAME_LISTBOX_POINTICONS: u32 = 0x3260;
const POPUP_NAME_MENU_CHALLENGE: u32 = 0x30e0;

// Text ids shown by the labels and buttons (string table offsets; the
// `-` ones are below the table base).
const POPUP_TEXT_PEAK_COLON: u32 = 0x3150;
const POPUP_TEXT_COURSE_COLON: u32 = 0x3178;
const POPUP_TEXT_EVENT_COLON: u32 = 0x31a0;
const POPUP_TEXT_SUBMIT: u32 = 0x31d8;
const POPUP_TEXT_CANCEL: u32 = 0x2c88;
const POPUP_TEXT_HUD_NONE: u32 = 0x3228;
const POPUP_TEXT_YES: u32 = 0xffff_dcc8; // -0x2338
const POPUP_TEXT_NO: u32 = 0xffff_fcd8; // -0x328

/// The CPU rider names: `0x4a0000 + offset`, the list entries 1..4.
const POPUP_RIDER_NAME_BASE: u64 = 0x004a_0000;
const POPUP_RIDER_NAME_0: u32 = 0x14f8;
const POPUP_RIDER_NAME_1: u32 = 0x1500;
const POPUP_RIDER_NAME_2: u32 = 0x1508;
const POPUP_RIDER_NAME_3: u32 = 0x1510;
/// The selected CPU rider index byte (gp-relative).
const POPUP_RIDER_SELECTED_GP: u32 = 0x1d9c;

/// Widget (`item`, S1) fields.
const POPUP_ITEM_NAME: u32 = 0x38;
const POPUP_ITEM_OBJECT: u32 = 0x8;
const POPUP_ITEM_ATTR: u32 = 0x14;
const POPUP_ITEM_MISC: u32 = 0x18;

/// Popup (`self`, S2) fields: the list boxes it remembers.
const POPUP_SELF_RANKED: u32 = 0x6dc;
const POPUP_SELF_CPU_RIDERS: u32 = 0x6e0;
const POPUP_SELF_POWER_UPS: u32 = 0x6e4;
const POPUP_SELF_MULTIPLIERS: u32 = 0x6e8;
const POPUP_SELF_POINTICONS: u32 = 0x6ec;

/// The lobby pointer (a gp global, `gp - 0xc8`) and its opponent name field.
const POPUP_LOBBY_GP: u32 = 0xc8;
const POPUP_LOBBY_OPPONENT: u32 = 0x98;

/// Object method slots: `{ short delta; short; fn }` at these offsets (the
/// method is called with `item + delta`).
const POPUP_SLOT_SET_VISIBLE: u32 = 0x48;
const POPUP_SLOT_V24: u32 = 0xc0;
const POPUP_SLOT_V25: u32 = 0xc8;

/// The generated callees: `GetHashValue32`, the list box adders and
/// selection, `UIText` setters.
const POPUP_GET_HASH: u32 = 0x0031_7670;
const POPUP_LIST_ADD_STR_ID: u32 = 0x0039_a568;
const POPUP_LIST_ADD_ASCII: u32 = 0x0039_a3f8;
const POPUP_LIST_SET_SELECTION: u32 = 0x0039_a7a8;
const POPUP_TEXT_SET_STRING: u32 = 0x003a_0c50;
const POPUP_TEXT_SET_TEXT_ID: u32 = 0x003a_1000;

/// Where the chain leaves: the shared tails of the generated body.
#[derive(Clone, Copy)]
enum PopupExit {
    /// `'b_1ae288`: `setVisible(1)` on the object in V0, then the epilogue.
    Visible,
    /// `'b_1ae360`: the method in V0 (4 cycles), then the epilogue.
    CallV0,
    /// `'b_1ae668`: one more cycle, then the epilogue.
    Store,
    /// Straight to the epilogue.
    Plain,
}

/// `void cFEStateChallengePopup_OnWidgetCreated(cFEStatePopup* self, cFEVObj* item)`
/// (0x1ae128). The popup's widget setup: the widget's name hash picks one of
/// sixteen cases. Static labels get their text and become visible, the list
/// boxes get their entries and are remembered on `self`, and the challenge
/// menu entry gets flag `0x80`. Calls and the frame follow the compiled body.
pub fn cfe_state_challenge_popup_on_widget_created(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 13;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S2, 0x10);
        frame.save(c, S1, 0x20);
        c.g[S2][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A1][0];
        frame.save(c, S0, 0x30);
        popup_chain(c, frame)?;
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// The `if (name == ...) ... else if` chain of the generated body, with the
/// shared exits. Every test that fails falls through to the next one, as the
/// generated `break 'b_...` does.
fn popup_chain(c: &mut Cpu, frame: Frame) -> Flow {
    let exit = 'chain: {
        // TextTitle: setVisible(1).
        c.g[A0][0] = POPUP_STR_BASE;
        if popup_name_is(c, POPUP_NAME_TEXT_TITLE, 0x001a_e154)? {
            c.g[V0][0] = popup_item(c, POPUP_ITEM_OBJECT);
            break 'chain PopupExit::Visible;
        }

        // TextPlayerName: setString(item, LobbyMan->mOpponent), setVisible(1).
        c.cycles += 6;
        c.g[A0][0] = POPUP_STR_BASE;
        if popup_name_is(c, POPUP_NAME_TEXT_PLAYER_NAME, 0x001a_e16c)? {
            c.cycles += 6;
            c.g[V0][0] = Addr((c.g[GP][0] as u32).wrapping_sub(POPUP_LOBBY_GP)).lw(c);
            c.g[A0][0] = c.g[S1][0];
            c.g[A1][0] = Addr((c.g[V0][0] as u32).wrapping_add(POPUP_LOBBY_OPPONENT)).lw(c);
            call_ptr(c, POPUP_TEXT_SET_STRING, 0x001a_e184)?;
            c.g[V0][0] = popup_item(c, POPUP_ITEM_OBJECT);
            break 'chain PopupExit::Visible;
        }
        c.g[A0][0] = POPUP_STR_BASE;

        // TextSelectPeakEvent: setVisible(1).
        c.cycles += 5;
        if popup_name_is(c, POPUP_NAME_TEXT_SELECT_PEAK_EVENT, 0x001a_e198)? {
            c.g[V0][0] = popup_item(c, POPUP_ITEM_OBJECT);
            break 'chain PopupExit::Visible;
        }

        // Textselectevent: mMiscData = 7, v25(1).
        c.cycles += 6;
        c.g[A0][0] = POPUP_STR_BASE;
        if popup_name_is(c, POPUP_NAME_TEXT_SELECT_EVENT, 0x001a_e1b0)? {
            c.cycles += 7;
            c.g[V0][0] = 7;
            c.g[V1][0] = popup_item(c, POPUP_ITEM_OBJECT);
            let seven = c.g[V0][0];
            popup_put_item(c, POPUP_ITEM_MISC, seven);
            c.g[A1][0] = 0x1;
            c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(POPUP_SLOT_V25)).lh(c);
            c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(POPUP_SLOT_V25 + 4)).lw(c);
            break 'chain PopupExit::CallV0;
        }
        c.g[A0][0] = POPUP_STR_BASE;

        // PeakLabel: setText(kT_OVRCMNPeakColon), setVisible(1).
        c.cycles += 5;
        if popup_name_is(c, POPUP_NAME_PEAK_LABEL, 0x001a_e1e0)? {
            c.cycles += 8;
            popup_set_text(c, POPUP_TEXT_PEAK_COLON, 0x001a_e1f4, 0x001a_e200)?;
            c.g[V0][0] = popup_item(c, POPUP_ITEM_OBJECT);
            break 'chain PopupExit::Visible;
        }
        c.g[A0][0] = POPUP_STR_BASE;

        // CourseLabel: setText(kT_OVRCMNCourseColon), setVisible(1).
        c.cycles += 5;
        if popup_name_is(c, POPUP_NAME_COURSE_LABEL, 0x001a_e214)? {
            c.cycles += 8;
            popup_set_text(c, POPUP_TEXT_COURSE_COLON, 0x001a_e228, 0x001a_e234)?;
            c.g[V0][0] = popup_item(c, POPUP_ITEM_OBJECT);
            break 'chain PopupExit::Visible;
        }
        c.g[A0][0] = POPUP_STR_BASE;

        // EventLabel: setText(kT_OVRCMNEventColon), setVisible(1).
        c.cycles += 5;
        if popup_name_is(c, POPUP_NAME_EVENT_LABEL, 0x001a_e248)? {
            c.cycles += 8;
            popup_set_text(c, POPUP_TEXT_EVENT_COLON, 0x001a_e25c, 0x001a_e268)?;
            c.g[V0][0] = popup_item(c, POPUP_ITEM_OBJECT);
            break 'chain PopupExit::Visible;
        }
        c.g[A0][0] = POPUP_STR_BASE;

        // TextRankedGame: setVisible(1).
        c.cycles += 5;
        if popup_name_is(c, POPUP_NAME_TEXT_RANKED_GAME, 0x001a_e27c)? {
            c.cycles += 1;
            c.g[V0][0] = popup_item(c, POPUP_ITEM_OBJECT);
            break 'chain PopupExit::Visible;
        }
        c.g[A0][0] = POPUP_STR_BASE;

        // TextSubmit: setText(kT_CMNSubmit), misc = 9, v25(1), setVisible(1), v25(1).
        c.cycles += 5;
        if popup_name_is(c, POPUP_NAME_TEXT_SUBMIT, 0x001a_e2b0)? {
            c.cycles += 25;
            popup_set_text(c, POPUP_TEXT_SUBMIT, 0x001a_e2c4, 0x001a_e2d0)?;
            c.g[V0][0] = 9;
            let nine = c.g[V0][0];
            popup_put_item(c, POPUP_ITEM_MISC, nine);
            popup_load_slot(c, POPUP_SLOT_V25, 1);
            popup_call_v0(c, 0x001a_e2f0)?;
            popup_load_slot(c, POPUP_SLOT_SET_VISIBLE, 1);
            popup_call_v0(c, 0x001a_e308)?;
            popup_load_slot(c, POPUP_SLOT_V25, 1);
            break 'chain PopupExit::CallV0;
        }
        c.g[A0][0] = POPUP_STR_BASE;

        // TextCancel: setText(kT_BTNCancel), misc = 10, setVisible(1).
        c.cycles += 5;
        if popup_name_is(c, POPUP_NAME_TEXT_CANCEL, 0x001a_e328)? {
            c.cycles += 12;
            popup_set_text(c, POPUP_TEXT_CANCEL, 0x001a_e33c, 0x001a_e348)?;
            c.g[V0][0] = 0xa;
            let ten = c.g[V0][0];
            popup_put_item(c, POPUP_ITEM_MISC, ten);
            popup_load_slot(c, POPUP_SLOT_SET_VISIBLE, 1);
            break 'chain PopupExit::CallV0;
        }
        c.g[A0][0] = POPUP_STR_BASE;

        // ListBoxRankedGame: two entries (Yes 1, No 0), setVisible, v24, remember.
        c.cycles += 5;
        if popup_name_is(c, POPUP_NAME_LISTBOX_RANKED_GAME, 0x001a_e37c)? {
            c.cycles += 28;
            c.g[A0][0] = POPUP_STR_BASE;
            popup_hash(c, POPUP_TEXT_YES, 0x001a_e390)?;
            popup_add_str(c, 1, 0x001a_e3a0)?;
            c.g[A0][0] = POPUP_STR_BASE;
            popup_hash(c, POPUP_TEXT_NO, 0x001a_e3ac)?;
            popup_add_str(c, 0, 0x001a_e3bc)?;
            popup_load_slot(c, POPUP_SLOT_SET_VISIBLE, 1);
            popup_call_v0(c, 0x001a_e3d4)?;
            popup_load_slot(c, POPUP_SLOT_V24, 1);
            popup_call_v0(c, 0x001a_e3ec)?;
            popup_set_self(c, POPUP_SELF_RANKED);
            break 'chain PopupExit::Store;
        }
        c.g[A0][0] = POPUP_STR_BASE;

        // ListBoxCPURiders: the "none" entry and four riders, setVisible, v24,
        // remember, then the selected rider.
        c.cycles += 5;
        if popup_name_is(c, POPUP_NAME_LISTBOX_CPU_RIDERS, 0x001a_e400)? {
            c.cycles += 45;
            c.g[A0][0] = POPUP_STR_BASE;
            popup_hash(c, POPUP_TEXT_HUD_NONE, 0x001a_e414)?;
            popup_add_str(c, 0, 0x001a_e424)?;
            popup_add_ascii(c, POPUP_RIDER_NAME_0, 1, 0x001a_e438)?;
            popup_add_ascii(c, POPUP_RIDER_NAME_1, 2, 0x001a_e44c)?;
            popup_add_ascii(c, POPUP_RIDER_NAME_2, 3, 0x001a_e460)?;
            popup_add_ascii(c, POPUP_RIDER_NAME_3, 4, 0x001a_e474)?;
            popup_load_slot(c, POPUP_SLOT_SET_VISIBLE, 1);
            popup_call_v0(c, 0x001a_e48c)?;
            popup_load_slot(c, POPUP_SLOT_V24, 1);
            popup_call_v0(c, 0x001a_e4a4)?;
            popup_set_self(c, POPUP_SELF_CPU_RIDERS);
            c.g[A1][0] = Addr((c.g[GP][0] as u32).wrapping_add(POPUP_RIDER_SELECTED_GP)).lbu(c);
            c.g[A0][0] = c.g[S1][0];
            call_ptr(c, POPUP_LIST_SET_SELECTION, 0x001a_e4b4)?;
            break 'chain PopupExit::Plain;
        }
        c.g[A0][0] = POPUP_STR_BASE;

        // ListBoxPowerUps: two entries (Yes 0, No 1), setVisible, v24, remember.
        c.cycles += 5;
        if popup_name_is(c, POPUP_NAME_LISTBOX_POWER_UPS, 0x001a_e4c8)? {
            c.cycles += 28;
            c.g[A0][0] = POPUP_STR_BASE;
            popup_hash(c, POPUP_TEXT_YES, 0x001a_e4dc)?;
            popup_add_str(c, 0, 0x001a_e4ec)?;
            c.g[A0][0] = POPUP_STR_BASE;
            popup_hash(c, POPUP_TEXT_NO, 0x001a_e4f8)?;
            popup_add_str(c, 1, 0x001a_e508)?;
            popup_load_slot(c, POPUP_SLOT_SET_VISIBLE, 1);
            popup_call_v0(c, 0x001a_e520)?;
            popup_load_slot(c, POPUP_SLOT_V24, 1);
            popup_call_v0(c, 0x001a_e538)?;
            popup_set_self(c, POPUP_SELF_POWER_UPS);
            break 'chain PopupExit::Store;
        }
        c.g[A0][0] = POPUP_STR_BASE;

        // ListBoxMultipliers: as PowerUps.
        c.cycles += 5;
        if popup_name_is(c, POPUP_NAME_LISTBOX_MULTIPLIERS, 0x001a_e54c)? {
            c.cycles += 28;
            c.g[A0][0] = POPUP_STR_BASE;
            popup_hash(c, POPUP_TEXT_YES, 0x001a_e560)?;
            popup_add_str(c, 0, 0x001a_e570)?;
            c.g[A0][0] = POPUP_STR_BASE;
            popup_hash(c, POPUP_TEXT_NO, 0x001a_e57c)?;
            popup_add_str(c, 1, 0x001a_e58c)?;
            popup_load_slot(c, POPUP_SLOT_SET_VISIBLE, 1);
            popup_call_v0(c, 0x001a_e5a4)?;
            popup_load_slot(c, POPUP_SLOT_V24, 1);
            popup_call_v0(c, 0x001a_e5bc)?;
            popup_set_self(c, POPUP_SELF_MULTIPLIERS);
            break 'chain PopupExit::Store;
        }
        c.g[A0][0] = POPUP_STR_BASE;

        // ListBoxPointicons: as PowerUps.
        c.cycles += 5;
        if popup_name_is(c, POPUP_NAME_LISTBOX_POINTICONS, 0x001a_e5d0)? {
            c.cycles += 28;
            c.g[A0][0] = POPUP_STR_BASE;
            popup_hash(c, POPUP_TEXT_YES, 0x001a_e5e4)?;
            popup_add_str(c, 0, 0x001a_e5f4)?;
            c.g[A0][0] = POPUP_STR_BASE;
            popup_hash(c, POPUP_TEXT_NO, 0x001a_e600)?;
            popup_add_str(c, 1, 0x001a_e610)?;
            popup_load_slot(c, POPUP_SLOT_SET_VISIBLE, 1);
            popup_call_v0(c, 0x001a_e628)?;
            popup_load_slot(c, POPUP_SLOT_V24, 1);
            popup_call_v0(c, 0x001a_e640)?;
            popup_set_self(c, POPUP_SELF_POINTICONS);
            break 'chain PopupExit::Store;
        }
        c.g[A0][0] = POPUP_STR_BASE;

        // MenuChallenge: flag 0x80 on the widget.
        c.cycles += 5;
        if popup_name_is(c, POPUP_NAME_MENU_CHALLENGE, 0x001a_e654)? {
            c.cycles += 3;
            c.g[V0][0] = popup_item(c, POPUP_ITEM_ATTR) | 0x80;
            let attr = c.g[V0][0];
            popup_put_item(c, POPUP_ITEM_ATTR, attr);
            break 'chain PopupExit::Store;
        }
        PopupExit::Plain
    };

    match exit {
        PopupExit::Visible => {
            // `'b_1ae288`: setVisible(1) on the object in V0.
            c.cycles += 7;
            c.g[A1][0] = 0x1;
            c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(POPUP_SLOT_SET_VISIBLE)).lh(c);
            c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(POPUP_SLOT_SET_VISIBLE + 4)).lw(c);
            let t = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, t, 0x001a_e29c)?;
            frame.restore(c, S0, 0x30);
        }
        PopupExit::CallV0 => {
            c.cycles += 4;
            popup_call_v0(c, 0x001a_e368)?;
            frame.restore(c, S0, 0x30);
        }
        PopupExit::Store => {
            c.cycles += 1;
            frame.restore(c, S0, 0x30);
        }
        PopupExit::Plain => frame.restore(c, S0, 0x30),
    }
    Ok(())
}

/// `item->name_hash`, then `GetHashValue32(0x460000 + name)`: the chain's
/// `if (S0 == V0)` test, with S0 left holding the widget's hash.
fn popup_name_is(c: &mut Cpu, name: u32, back: u32) -> Result<bool, Unwound> {
    c.g[S0][0] = popup_item(c, POPUP_ITEM_NAME);
    popup_hash(c, name, back)?;
    Ok(c.g[S0][0] == c.g[V0][0])
}

/// `A0 += delta` (the string table entry), then `GetHashValue32(A0)` returning
/// to `back`.
fn popup_hash(c: &mut Cpu, delta: u32, back: u32) -> Flow {
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(delta));
    call_ptr(c, POPUP_GET_HASH, back)
}

/// `SetTextId(item, GetHashValue32(0x460000 + text))`.
fn popup_set_text(c: &mut Cpu, text: u32, back_hash: u32, back_set: u32) -> Flow {
    c.g[A0][0] = POPUP_STR_BASE;
    popup_hash(c, text, back_hash)?;
    c.g[A1][0] = c.g[V0][0];
    c.g[A0][0] = c.g[S1][0];
    call_ptr(c, POPUP_TEXT_SET_TEXT_ID, back_set)
}

/// `AddItemStrId(item, GetHashValue32(...), a2)`: the hash is in V0.
fn popup_add_str(c: &mut Cpu, a2: u64, back: u32) -> Flow {
    c.g[A1][0] = c.g[V0][0];
    c.g[A2][0] = a2;
    c.g[A0][0] = c.g[S1][0];
    call_ptr(c, POPUP_LIST_ADD_STR_ID, back)
}

/// `AddItemAscii(item, 0x4a0000 + text, a2)`.
fn popup_add_ascii(c: &mut Cpu, text: u32, a2: u64, back: u32) -> Flow {
    c.g[A1][0] = sx(POPUP_RIDER_NAME_BASE.wrapping_add(text as u64) as u32);
    c.g[A0][0] = c.g[S1][0];
    c.g[A2][0] = a2;
    call_ptr(c, POPUP_LIST_ADD_ASCII, back)
}

/// `V1 = item->object; A1 = arg; A0 = slot.delta; V0 = slot.fn`: the method
/// slot read before a call.
fn popup_load_slot(c: &mut Cpu, slot: u32, arg: u64) {
    c.g[V1][0] = popup_item(c, POPUP_ITEM_OBJECT);
    c.g[A1][0] = arg;
    c.g[A0][0] = Addr((c.g[V1][0] as u32).wrapping_add(slot)).lh(c);
    c.g[V0][0] = Addr((c.g[V1][0] as u32).wrapping_add(slot + 4)).lw(c);
}

/// Calls the method in V0 with `this = item + A0`, returning to `back`.
fn popup_call_v0(c: &mut Cpu, back: u32) -> Flow {
    let t = c.g[V0][0] as u32;
    c.g[A0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[A0][0] as u32));
    call_ptr(c, t, back)
}

/// `lw off(item)`, sign-extended.
fn popup_item(c: &mut Cpu, off: u32) -> u64 {
    Addr((c.g[S1][0] as u32).wrapping_add(off)).lw(c)
}

/// `sw v, off(item)`.
fn popup_put_item(c: &mut Cpu, off: u32, v: u64) {
    Addr((c.g[S1][0] as u32).wrapping_add(off)).sw(c, v);
}

/// `sw item, off(self)`.
fn popup_set_self(c: &mut Cpu, off: u32) {
    let item = c.g[S1][0];
    Addr((c.g[S2][0] as u32).wrapping_add(off)).sw(c, item);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, fakes, DATA, GAME_GP};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// The gate word: the fakes below answer only while the machine holds it.
    const OC875_CTRL: u32 = DATA + 0x2800;
    const OC875_MAGIC: u32 = 0x0875_c0de;
    static OC875_PREV_HASH: AtomicUsize = AtomicUsize::new(0);
    static OC875_PREV_ADD_STR: AtomicUsize = AtomicUsize::new(0);
    static OC875_PREV_ADD_ASCII: AtomicUsize = AtomicUsize::new(0);
    static OC875_PREV_SELECTION: AtomicUsize = AtomicUsize::new(0);
    static OC875_PREV_SET_STRING: AtomicUsize = AtomicUsize::new(0);
    static OC875_PREV_SET_TEXT_ID: AtomicUsize = AtomicUsize::new(0);

    /// The name offsets the test gives the widget, in chain order; index 16
    /// matches no name.
    const OC875_NAMES: [u32; 16] = [
        POPUP_NAME_TEXT_TITLE,
        POPUP_NAME_TEXT_PLAYER_NAME,
        POPUP_NAME_TEXT_SELECT_PEAK_EVENT,
        POPUP_NAME_TEXT_SELECT_EVENT,
        POPUP_NAME_PEAK_LABEL,
        POPUP_NAME_COURSE_LABEL,
        POPUP_NAME_EVENT_LABEL,
        POPUP_NAME_TEXT_RANKED_GAME,
        POPUP_NAME_TEXT_SUBMIT,
        POPUP_NAME_TEXT_CANCEL,
        POPUP_NAME_LISTBOX_RANKED_GAME,
        POPUP_NAME_LISTBOX_CPU_RIDERS,
        POPUP_NAME_LISTBOX_POWER_UPS,
        POPUP_NAME_LISTBOX_MULTIPLIERS,
        POPUP_NAME_LISTBOX_POINTICONS,
        POPUP_NAME_MENU_CHALLENGE,
    ];

    /// `GetHashValue32` fake: the hash is the string's address, so the test
    /// picks the case by the widget's name. The gated body is the real one.
    fn oc875_str_hash(c: &mut Cpu) {
        if c.bus.read32(OC875_CTRL) != OC875_MAGIC {
            return fakes::fall_back(c, &OC875_PREV_HASH, ee::s10::f_00317670__generated);
        }
        let s = c.g[A0][0];
        fakes::fake_call(c, POPUP_GET_HASH);
        c.g[V0][0] = s;
    }

    fn oc875_add_str(c: &mut Cpu) {
        if c.bus.read32(OC875_CTRL) != OC875_MAGIC {
            return fakes::fall_back(c, &OC875_PREV_ADD_STR, ee::s13::f_0039a568__generated);
        }
        fakes::fake_call(c, POPUP_LIST_ADD_STR_ID);
    }

    fn oc875_add_ascii(c: &mut Cpu) {
        if c.bus.read32(OC875_CTRL) != OC875_MAGIC {
            return fakes::fall_back(c, &OC875_PREV_ADD_ASCII, ee::s13::f_0039a3f8__generated);
        }
        fakes::fake_call(c, POPUP_LIST_ADD_ASCII);
    }

    fn oc875_selection(c: &mut Cpu) {
        if c.bus.read32(OC875_CTRL) != OC875_MAGIC {
            return fakes::fall_back(c, &OC875_PREV_SELECTION, ee::s13::f_0039a7a8__generated);
        }
        fakes::fake_call(c, POPUP_LIST_SET_SELECTION);
    }

    fn oc875_set_string(c: &mut Cpu) {
        if c.bus.read32(OC875_CTRL) != OC875_MAGIC {
            return fakes::fall_back(c, &OC875_PREV_SET_STRING, ee::s13::f_003a0c50__generated);
        }
        fakes::fake_call(c, POPUP_TEXT_SET_STRING);
    }

    fn oc875_set_text_id(c: &mut Cpu) {
        if c.bus.read32(OC875_CTRL) != OC875_MAGIC {
            return fakes::fall_back(c, &OC875_PREV_SET_TEXT_ID, ee::s13::f_003a1000__generated);
        }
        fakes::fake_call(c, POPUP_TEXT_SET_TEXT_ID);
    }

    /// Puts the gated fakes in the callees' override slots (once per process).
    fn oc875_hook() {
        fakes::hook(
            addr_of_mut!(ee::s10::OVERRIDE_00317670),
            oc875_str_hash,
            &OC875_PREV_HASH,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_0039A568),
            oc875_add_str,
            &OC875_PREV_ADD_STR,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_0039A3F8),
            oc875_add_ascii,
            &OC875_PREV_ADD_ASCII,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_0039A7A8),
            oc875_selection,
            &OC875_PREV_SELECTION,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003A0C50),
            oc875_set_string,
            &OC875_PREV_SET_STRING,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_003A1000),
            oc875_set_text_id,
            &OC875_PREV_SET_TEXT_ID,
        );
    }

    /// Every case of the chain (and the no-match path), the object's method
    /// slots pointing at the miss-hook fakes, and the lobby's opponent name.
    #[test]
    fn cfe_state_challenge_popup_on_widget_created_matches() {
        oc875_hook();
        check(
            ee::s03::f_001ae128__generated,
            cfe_state_challenge_popup_on_widget_created,
            200,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 875, 0);
                c.bus.write32(OC875_CTRL, OC875_MAGIC);
                let me = DATA + 0x1400;
                let item = DATA + 0x1000;
                let vt = DATA + 0x1100;
                let lobby = DATA + 0x2000;
                let target = (seed % 17) as usize;
                let name_hash = if target < 16 {
                    POPUP_STR_BASE as u32 + OC875_NAMES[target]
                } else {
                    0x1234
                };
                c.bus.write32(item + POPUP_ITEM_NAME, name_hash);
                c.bus.write32(item + POPUP_ITEM_OBJECT, vt);
                c.bus.write32(item + POPUP_ITEM_ATTR, r.next_u32());
                c.bus.write32(item + POPUP_ITEM_MISC, r.next_u32());
                for (i, off) in [POPUP_SLOT_SET_VISIBLE, POPUP_SLOT_V24, POPUP_SLOT_V25]
                    .into_iter()
                    .enumerate()
                {
                    c.bus.write16(vt + off, (r.next_u32() as i16 & !3) as u16);
                    c.bus.write32(vt + off + 4, fakes::FAKE + 0x10 * i as u32);
                }
                c.bus.write32(GAME_GP - POPUP_LOBBY_GP, lobby);
                c.bus.write32(lobby + POPUP_LOBBY_OPPONENT, DATA + 0x2400);
                c.bus
                    .write8(GAME_GP + POPUP_RIDER_SELECTED_GP, r.next_u32() as u8);
                c.g[A0][0] = sx(me);
                c.g[A1][0] = sx(item);
                c.pc = 0x001a_e128;
            },
        );
    }
}
