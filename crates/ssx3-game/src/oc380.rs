//! Staging module oc380 (wave wave): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc380`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::game::fe::fememcardcallbacks::callee;
use crate::game::fe::ovstatepause::callee::UI_STATE_MGR_CLOSE_TOP_STATES;

/// `MemCardUI_ShowCorruptSave()` (0x1d7190), through its override slot.
pub const MEM_CARD_UI_SHOW_CORRUPT_SAVE: Callee = Callee {
    addr: 0x001d_7190,
    f: ee::s04::f_001d7190,
};
use crate::game::fe::fememcardcallbacks::{
    memcard_gp, MEMCARD_APP_GP, MEMCARD_FE_BUSY, MEMCARD_FE_ENGINE, MEMCARD_FE_GP,
    MEMCARD_FE_LOAD_FROM_PAUSE, MEMCARD_FE_MESSAGE, MEMCARD_FE_MODE, MEMCARD_FE_POPUP,
    MEMCARD_FE_STATE, MEMCARD_FE_VTABLE, MEMCARD_FE_VT_CLOSE, MEMCARD_FE_VT_SET_MODE,
    MEMCARD_MC_CARD, MEMCARD_MC_MANAGER_GET, MEMCARD_MSG_STATUS, MEMCARD_TEMP_STRING_FLAGS,
    MEMCARD_TEMP_STRING_SIZE, MEMCARD_TEMP_STRING_TAG,
};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x001d_8700,
    mem_card_state_base_on_child_event,
    "MemCardStateBase_OnChildEvent",
)];

/// State vtable slot 41 (`+0x148`): the reply/confirm step most message
/// replies call. `?` (target not named from the state's ctor in the inputs).
pub(crate) const MEMCARD_FE_VT_SLOT41: u32 = 0x148;

/// State vtable slot 45 (`+0x168`): called for choice 0 of the confirm
/// overwrite message. `?` (same reason as slot 41).
pub(crate) const MEMCARD_FE_VT_SLOT45: u32 = 0x168;

/// Set while the message popup is open; cleared after a reply (and by the
/// dismiss path when the message is the status message).
pub(crate) const MEMCARD_FE_POPUP_OPEN: u32 = 0x210;

/// A format is pending (`?`: set by the confirm-format reply, cleared when
/// the format finishes or fails).
pub(crate) const MEMCARD_FE_FORMAT_PENDING: u32 = 0x1e8;

/// A delete is pending (`?`: set by the confirm-delete reply, cleared by
/// the delete-done reply).
pub(crate) const MEMCARD_FE_DELETE_PENDING: u32 = 0x1e0;

/// Ask for a retry (`?`: set by the not-enough-space, not-detected, no-card
/// and game-corrupt replies that do not go on).
pub(crate) const MEMCARD_FE_RETRY: u32 = 0x1ec;

/// Continue from a load (`?`: set by the create/edit and corrupt-save
/// replies, cleared by the not-enough-space and save-failed replies).
pub(crate) const MEMCARD_FE_FROM_LOAD: u32 = 0x1f0;

/// The pending-reply pointer/word the dismiss path clears, and its flag
/// (`?`: `+0x1a4` is the word `MemCardStateBase_Update` tests).
pub(crate) const MEMCARD_FE_REPLY_PENDING: u32 = 0x1a0;

pub(crate) const MEMCARD_FE_REPLY_FLAG: u32 = 0x1a4;

/// The operation-in-progress word (`?`: cleared by the confirm-delete and
/// confirm-format replies; `MemCardStateBase_Update` tests it for busy).
pub(crate) const MEMCARD_FE_OP_PENDING: u32 = 0x1b0;

/// Whether the selected slot has a save (`?`: the delete-done and
/// format-done replies turn the busy word back on when it is set).
pub(crate) const MEMCARD_FE_HAS_SAVE: u32 = 0x1dc;

/// The use-save-title flag `MemCardStateBase_SetUseSaveTitle` stores; the
/// delete-done reply calls slot 41 when it is set.
pub(crate) const MEMCARD_FE_USE_SAVE_TITLE: u32 = 0x1f4;

/// The card manager word (`+0x440`) a reply that came from a load writes
/// its `+0x204` flag into before closing (`?`).
pub(crate) const MEMCARD_MC_NOTIFY: u32 = 0x440;

/// The UI state manager list inside the state's UI engine (`+0x10`); the
/// engine is `MEMCARD_FE_ENGINE`, this is the `+0x18` passed to
/// `UIStateMgr_CloseTopStates`.
pub(crate) const MEMCARD_UI_STATE_MGR: u32 = 0x18;

/// Popup buttons: options 0, 1 and 2, and dismiss.
pub(crate) const MEMCARD_BUTTON_OPTION0: u32 = 0xf;

pub(crate) const MEMCARD_BUTTON_OPTION1: u32 = 0x10;

pub(crate) const MEMCARD_BUTTON_OPTION2: u32 = 0x11;

pub(crate) const MEMCARD_BUTTON_DISMISS: u32 = 0x16;

/// The message jump table at 0x4678e0 (24 words, one per message id minus
/// one); only the test reads it, to give the generated body its table.
#[cfg(test)]
pub(crate) const MEMCARD_MSG_TABLE: u32 = 0x0046_78e0;

/// Load state vtable slot `slot` of the state in `s0` into `v0`/`a0`/`v1`
/// (vtable, delta, function). Charges no cycles: the caller charges them.
pub(crate) fn memcard_on_child_load_vcall(c: &mut Cpu, slot: u32) {
    c.g[V0][0] = Addr(r32(c, S0).wrapping_add(MEMCARD_FE_VTABLE)).lw(c);
    c.g[A0][0] = Addr(r32(c, V0).wrapping_add(slot)).lh(c);
    c.g[V1][0] = Addr(r32(c, V0).wrapping_add(slot + 4)).lw(c);
}

/// Call the vtable slot already loaded by
/// [`memcard_on_child_load_vcall`] (the shared tail at 0x1d8b44: 4 cycles,
/// return 0x1d8b4c), then reload the message into `v1` for the close-out.
pub(crate) fn memcard_on_child_close_tail(c: &mut Cpu) -> Flow {
    c.cycles += 4;
    c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, A0)));
    call_ptr(c, r32(c, V1), 0x001d_8b4c)?;
    c.g[V1][0] = Addr(r32(c, S0).wrapping_add(MEMCARD_FE_MESSAGE)).lw(c);
    Ok(())
}

/// Call state vtable slot `slot` of the state in `s0`, returning to `ret`.
/// Charges no cycles: the caller charges them where the original does.
pub(crate) fn memcard_on_child_vcall(c: &mut Cpu, slot: u32, ret: u32) -> Flow {
    memcard_on_child_load_vcall(c, slot);
    c.g[A0][0] = sx(r32(c, S0).wrapping_add(r32(c, A0)));
    call_ptr(c, r32(c, V1), ret)
}

/// The notify tail with `v1` already holding `self+0x204` and `v0` holding
/// 1: if the reply did not come from a load, reload the message into `v1`;
/// otherwise store the flag in the manager and close through slot 40.
pub(crate) fn memcard_on_child_notify_loaded(c: &mut Cpu) -> Flow {
    if c.g[V1][0] != c.g[V0][0] {
        c.g[V1][0] = Addr(r32(c, S0).wrapping_add(MEMCARD_FE_MESSAGE)).lw(c);
        return Ok(());
    }
    c.cycles += 5;
    Addr(r32(c, S2).wrapping_add(MEMCARD_MC_NOTIFY)).sw(c, c.g[V1][0]);
    memcard_on_child_load_vcall(c, MEMCARD_FE_VT_CLOSE);
    memcard_on_child_close_tail(c)
}

/// Notify the card manager (`mp` in `s2`) that a load-started reply is
/// done, closing the screen when `self+0x204` is 1.
pub(crate) fn memcard_on_child_notify(c: &mut Cpu) -> Flow {
    c.g[V1][0] = Addr(r32(c, S0).wrapping_add(MEMCARD_FE_LOAD_FROM_PAUSE)).lw(c);
    c.g[V0][0] = 1;
    memcard_on_child_notify_loaded(c)
}

/// `void MemCardStateBase_OnChildEvent(MemCardStateBase* self, int a1, int button)`
/// (0x1d8700): the popup-button callback of the memory-card state. Buttons
/// 0xf/0x10/0x11 pick popup option 0/1/2 and answer the message the state
/// is showing (`self+0x19c`): close, cancel, or confirm a create,
/// overwrite, delete or format, then pop the top UI state and clear the
/// popup-open flag. Button 0x16 dismisses: it clears a pending reply, or
/// the status popup. Handles the 0x1d8700 entry only (the `e` twin is the
/// message jump table in the data section, not a resume entry).
pub fn mem_card_state_base_on_child_event(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x40);
        frame.save(c, S0, 0x30);
        frame.save(c, S1, 0x20);
        frame.save(c, S2, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_sub(MEMCARD_BUTTON_OPTION0));
        c.g[V0][0] = (c.g[V0][0] < 0x3u64) as u64;
        let is_option = c.g[V0][0] != 0;
        // Set by message 0xc choice 2, which returns after showing the
        // corrupt-save screen, skipping the close-out below.
        let mut skip_post = false;
        if is_option {
            // The movn choice, as generated: 1 only for 0x10; 2, then 0
            // unless the button is 0x11.
            c.cycles += 3;
            c.g[V0][0] = MEMCARD_BUTTON_OPTION1 as u64;
            let not_option1 = c.g[A2][0] != c.g[V0][0];
            c.g[V0][0] = c.g[A2][0] ^ MEMCARD_BUTTON_OPTION2 as u64;
            if not_option1 {
                c.cycles += 2;
                c.g[S1][0] = 0x2;
                if c.g[V0][0] != 0 {
                    c.g[S1][0] = 0;
                }
            } else {
                c.cycles += 2;
                c.g[S1][0] = 0x1;
            }
            c.cycles += 8;
            c.g[A0][0] = Addr(memcard_gp(c, MEMCARD_APP_GP)).lw(c);
            call_ptr(c, MEMCARD_MC_MANAGER_GET, 0x001d_8748)?;
            c.g[S2][0] = c.g[V0][0];
            c.g[V0][0] = Addr(r32(c, S0).wrapping_add(MEMCARD_FE_MESSAGE)).lw(c);
            let msg = r32(c, V0);
            c.g[A0][0] = sx(msg.wrapping_sub(0x1));
            c.g[V1][0] = (c.g[A0][0] < 0x18u64) as u64;
            let in_range = c.g[V1][0] != 0;
            c.g[V0][0] = 0x460000;
            // Whether the close-out must still load the message into `v1`
            // (the generated `+1 cycle` load at 0x1d8b6c). Paths that
            // already leave it there clear this.
            let mut post_load = true;
            if in_range {
                c.cycles += 6;
                c.g[V1][0] = sx((c.g[A0][0] as u32) << 2);
                c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x78e0));
                c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
                c.g[A0][0] = Addr(r32(c, V1)).lw(c);
                match msg {
                    // 2: confirm overwrite.
                    2 => {
                        c.cycles += 2;
                        let choice_ne = c.g[S1][0] != 0;
                        c.g[V0][0] = 1;
                        if choice_ne {
                            c.cycles += 2;
                            Addr(r32(c, S0).wrapping_add(MEMCARD_FE_STATE)).sw(c, c.g[V0][0]);
                        } else {
                            c.cycles += 4;
                            memcard_on_child_load_vcall(c, MEMCARD_FE_VT_SLOT45);
                            memcard_on_child_close_tail(c)?;
                            post_load = false;
                        }
                    }
                    // 0x13: not enough space.
                    0x13 => {
                        c.cycles += 2;
                        let choice_ne = c.g[S1][0] != 0;
                        c.g[V0][0] = 1;
                        if choice_ne {
                            c.cycles += 2;
                            Addr(r32(c, S0).wrapping_add(MEMCARD_FE_RETRY)).sw(c, c.g[V0][0]);
                        } else {
                            c.cycles += 10;
                            Addr(r32(c, S0).wrapping_add(MEMCARD_FE_FROM_LOAD)).sw(c, 0);
                            memcard_on_child_vcall(c, MEMCARD_FE_VT_SLOT41, 0x001d_87b8)?;
                            memcard_on_child_notify(c)?;
                            post_load = false;
                        }
                    }
                    // 1: create/edit.
                    1 => {
                        c.cycles += 2;
                        let choice_eq = c.g[S1][0] == 0;
                        c.g[V0][0] = 0x3;
                        if choice_eq {
                            c.cycles += 4;
                            c.g[V1][0] = 1;
                            Addr(r32(c, S0).wrapping_add(MEMCARD_FE_STATE)).sw(c, c.g[V0][0]);
                            Addr(r32(c, S0).wrapping_add(MEMCARD_FE_FROM_LOAD)).sw(c, c.g[V1][0]);
                        } else {
                            c.cycles += 9;
                            memcard_on_child_vcall(c, MEMCARD_FE_VT_SLOT41, 0x001d_87f8)?;
                            memcard_on_child_notify(c)?;
                            post_load = false;
                        }
                    }
                    // 9: not detected.
                    9 => {
                        c.cycles += 2;
                        let choice_ne = c.g[S1][0] != 0;
                        c.g[V0][0] = 1;
                        if choice_ne {
                            c.cycles += 2;
                            Addr(r32(c, S0).wrapping_add(MEMCARD_FE_RETRY)).sw(c, c.g[V0][0]);
                        } else {
                            c.cycles += 9;
                            memcard_on_child_vcall(c, MEMCARD_FE_VT_SLOT41, 0x001d_8838)?;
                            memcard_on_child_notify(c)?;
                            post_load = false;
                        }
                    }
                    // 0x12: no card.
                    0x12 => {
                        c.cycles += 2;
                        if c.g[S1][0] != 0 {
                            c.cycles += 8;
                            memcard_on_child_vcall(c, MEMCARD_FE_VT_SLOT41, 0x001d_8880)?;
                            memcard_on_child_notify(c)?;
                            post_load = false;
                        } else {
                            c.cycles += 1;
                            c.g[V0][0] = 1;
                            c.cycles += 2;
                            Addr(r32(c, S0).wrapping_add(MEMCARD_FE_RETRY)).sw(c, c.g[V0][0]);
                        }
                    }
                    // 0xa: load failed.
                    0xa => {
                        c.cycles += 2;
                        if c.g[S1][0] != 0 {
                            c.g[V1][0] = Addr(r32(c, S0).wrapping_add(MEMCARD_FE_MESSAGE)).lw(c);
                            post_load = false;
                        } else {
                            c.cycles += 9;
                            memcard_on_child_vcall(c, MEMCARD_FE_VT_SLOT41, 0x001d_88c0)?;
                            memcard_on_child_notify(c)?;
                            post_load = false;
                        }
                    }
                    // 0xb: save failed.
                    0xb => {
                        c.cycles += 10;
                        Addr(r32(c, S0).wrapping_add(MEMCARD_FE_FROM_LOAD)).sw(c, 0);
                        memcard_on_child_vcall(c, MEMCARD_FE_VT_SLOT41, 0x001d_88fc)?;
                        memcard_on_child_notify(c)?;
                        post_load = false;
                    }
                    // 4, 5, 6: the done messages close the screen.
                    4 | 5 | 6 => {
                        c.cycles += 4;
                        memcard_on_child_load_vcall(c, MEMCARD_FE_VT_CLOSE);
                        memcard_on_child_close_tail(c)?;
                        post_load = false;
                    }
                    // 3: confirm delete.
                    3 => {
                        c.cycles += 2;
                        let choice_ne = c.g[S1][0] != 0;
                        Addr(r32(c, S0).wrapping_add(MEMCARD_FE_OP_PENDING)).sw(c, 0);
                        if choice_ne {
                            c.cycles += 9;
                            memcard_on_child_vcall(c, MEMCARD_FE_VT_SLOT41, 0x001d_895c)?;
                            memcard_on_child_notify(c)?;
                            post_load = false;
                        } else {
                            c.cycles += 4;
                            c.g[V0][0] = 1;
                            Addr(r32(c, S0).wrapping_add(MEMCARD_FE_DELETE_PENDING))
                                .sw(c, c.g[V0][0]);
                            Addr(r32(c, S0).wrapping_add(MEMCARD_FE_BUSY)).sw(c, c.g[V0][0]);
                        }
                    }
                    // 0xc: game corrupt.
                    0xc => {
                        c.cycles += 2;
                        let choice_ne = c.g[S1][0] != 0;
                        c.g[V0][0] = 1;
                        if !choice_ne {
                            c.cycles += 9;
                            memcard_on_child_vcall(c, MEMCARD_FE_VT_SLOT41, 0x001d_899c)?;
                            c.g[V0][0] = 1;
                            c.g[V1][0] =
                                Addr(r32(c, S0).wrapping_add(MEMCARD_FE_LOAD_FROM_PAUSE)).lw(c);
                            Addr(r32(c, S0).wrapping_add(MEMCARD_FE_STATE)).sw(c, c.g[V0][0]);
                            if c.g[V1][0] != c.g[V0][0] {
                                c.cycles += 1;
                            }
                            memcard_on_child_notify_loaded(c)?;
                            post_load = false;
                        } else {
                            c.cycles += 2;
                            if c.g[S1][0] != c.g[V0][0] {
                                c.cycles += 4;
                                MEM_CARD_UI_SHOW_CORRUPT_SAVE.call(c, 0x001d_89d8)?;
                                frame.restore(c, S0, 0x30);
                                skip_post = true;
                            } else {
                                c.cycles += 2;
                                Addr(r32(c, S0).wrapping_add(MEMCARD_FE_RETRY)).sw(c, c.g[S1][0]);
                            }
                        }
                    }
                    // 0xd: corrupt save.
                    0xd => {
                        c.cycles += 2;
                        if c.g[S1][0] != 0 {
                            c.cycles += 8;
                            memcard_on_child_vcall(c, MEMCARD_FE_VT_SLOT41, 0x001d_8a0c)?;
                            memcard_on_child_notify(c)?;
                            post_load = false;
                        } else {
                            c.cycles += 1;
                            c.g[V0][0] = 0x3;
                            c.cycles += 4;
                            c.g[V1][0] = 1;
                            Addr(r32(c, S0).wrapping_add(MEMCARD_FE_STATE)).sw(c, c.g[V0][0]);
                            Addr(r32(c, S0).wrapping_add(MEMCARD_FE_FROM_LOAD)).sw(c, c.g[V1][0]);
                        }
                    }
                    // 7: delete done.
                    7 => {
                        c.cycles += 4;
                        c.g[V0][0] = Addr(r32(c, S0).wrapping_add(MEMCARD_FE_USE_SAVE_TITLE)).lw(c);
                        Addr(r32(c, S0).wrapping_add(MEMCARD_FE_BUSY)).sw(c, 0);
                        let no_title = c.g[V0][0] == 0;
                        Addr(r32(c, S0).wrapping_add(MEMCARD_FE_DELETE_PENDING)).sw(c, 0);
                        if !no_title {
                            c.cycles += 5;
                            memcard_on_child_vcall(c, MEMCARD_FE_VT_SLOT41, 0x001d_8a54)?;
                        }
                        c.cycles += 3;
                        c.g[V0][0] = Addr(r32(c, S0).wrapping_add(MEMCARD_FE_HAS_SAVE)).lw(c);
                        let no_save = c.g[V0][0] == 0;
                        c.g[V0][0] = 1;
                        if !no_save {
                            c.cycles += 1;
                            Addr(r32(c, S0).wrapping_add(MEMCARD_FE_BUSY)).sw(c, c.g[V0][0]);
                        }
                        c.cycles += 4;
                        memcard_on_child_notify(c)?;
                        post_load = false;
                    }
                    // 0x10: format done.
                    0x10 => {
                        c.cycles += 7;
                        Addr(r32(c, S0).wrapping_add(MEMCARD_FE_FORMAT_PENDING)).sw(c, 0);
                        c.g[A0][0] = c.g[S0][0];
                        c.g[A1][0] = 0;
                        callee::MEM_CARD_STATE_BASE_SET_USE_SAVE_TITLE.call(c, 0x001d_8a98)?;
                        c.g[V0][0] = Addr(r32(c, S0).wrapping_add(MEMCARD_FE_HAS_SAVE)).lw(c);
                        let no_save = c.g[V0][0] == 0;
                        c.g[V0][0] = 1;
                        if no_save {
                            c.cycles += 1;
                            Addr(r32(c, S0).wrapping_add(MEMCARD_FE_BUSY)).sw(c, 0);
                        } else {
                            c.cycles += 2;
                            Addr(r32(c, S0).wrapping_add(MEMCARD_FE_BUSY)).sw(c, c.g[V0][0]);
                        }
                        c.cycles += 4;
                        memcard_on_child_notify(c)?;
                        post_load = false;
                    }
                    // 0x11: format failed.
                    0x11 => {
                        c.cycles += 2;
                        Addr(r32(c, S0).wrapping_add(MEMCARD_FE_FORMAT_PENDING)).sw(c, 0);
                        c.cycles += 3;
                        memcard_on_child_load_vcall(c, MEMCARD_FE_VT_SLOT41);
                        memcard_on_child_close_tail(c)?;
                        post_load = false;
                    }
                    // 0xe, 0xf: confirm format.
                    0xe | 0xf => {
                        c.cycles += 2;
                        let choice_ne = c.g[S1][0] != 0;
                        Addr(r32(c, S0).wrapping_add(MEMCARD_FE_OP_PENDING)).sw(c, 0);
                        if !choice_ne {
                            c.cycles += 4;
                            c.g[V0][0] = 1;
                            Addr(r32(c, S0).wrapping_add(MEMCARD_FE_FORMAT_PENDING))
                                .sw(c, c.g[V0][0]);
                            Addr(r32(c, S0).wrapping_add(MEMCARD_FE_BUSY)).sw(c, c.g[V0][0]);
                        } else {
                            c.cycles += 3;
                            c.g[V0][0] = 1;
                            if c.g[S1][0] != c.g[V0][0] {
                                c.g[V1][0] =
                                    Addr(r32(c, S0).wrapping_add(MEMCARD_FE_MESSAGE)).lw(c);
                                post_load = false;
                            } else {
                                c.cycles += 9;
                                Addr(r32(c, S0).wrapping_add(MEMCARD_FE_FORMAT_PENDING)).sw(c, 0);
                                memcard_on_child_vcall(c, MEMCARD_FE_VT_SLOT41, 0x001d_8b18)?;
                                c.g[V0][0] =
                                    Addr(r32(c, S0).wrapping_add(MEMCARD_FE_LOAD_FROM_PAUSE)).lw(c);
                                if c.g[V0][0] != c.g[S1][0] {
                                    c.g[V1][0] =
                                        Addr(r32(c, S0).wrapping_add(MEMCARD_FE_MESSAGE)).lw(c);
                                } else {
                                    c.cycles += 5;
                                    Addr(r32(c, S2).wrapping_add(MEMCARD_MC_NOTIFY))
                                        .sw(c, c.g[V0][0]);
                                    memcard_on_child_load_vcall(c, MEMCARD_FE_VT_CLOSE);
                                    memcard_on_child_close_tail(c)?;
                                }
                                post_load = false;
                            }
                        }
                    }
                    // 8, 0x17: slot 41 only.
                    8 | 0x17 => {
                        c.cycles += 3;
                        memcard_on_child_load_vcall(c, MEMCARD_FE_VT_SLOT41);
                        memcard_on_child_close_tail(c)?;
                        post_load = false;
                    }
                    // 0x18: slot 41 only, through the shared tail whose
                    // return is the close-out itself.
                    0x18 => {
                        c.cycles += 5;
                        memcard_on_child_vcall(c, MEMCARD_FE_VT_SLOT41, 0x001d_8b68)?;
                    }
                    // 0x14, 0x15, 0x16 and anything else in range: no
                    // reply body, straight to the close-out.
                    _ => {}
                }
            }
            if !skip_post {
                if post_load {
                    c.cycles += 1;
                    c.g[V1][0] = Addr(r32(c, S0).wrapping_add(MEMCARD_FE_MESSAGE)).lw(c);
                }
                c.cycles += 3;
                c.g[V0][0] = MEMCARD_MSG_STATUS;
                let is_status = c.g[V1][0] == c.g[V0][0];
                c.g[V0][0] = Addr(memcard_gp(c, MEMCARD_FE_GP)).lw(c);
                if !is_status {
                    c.cycles += 4;
                    c.g[A1][0] = 1;
                    c.g[A0][0] = Addr(r32(c, V0).wrapping_add(MEMCARD_FE_ENGINE)).lw(c);
                    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(MEMCARD_UI_STATE_MGR));
                    UI_STATE_MGR_CLOSE_TOP_STATES.call(c, 0x001d_8b88)?;
                }
                c.cycles += 2;
                Addr(r32(c, S0).wrapping_add(MEMCARD_FE_POPUP_OPEN)).sw(c, 0);
                c.cycles += 1;
                frame.restore(c, S0, 0x30);
            }
        } else {
            c.cycles += 3;
            c.g[V0][0] = MEMCARD_BUTTON_DISMISS as u64;
            if c.g[A2][0] != c.g[V0][0] {
                frame.restore(c, S0, 0x30);
            } else {
                c.cycles += 4;
                c.g[V1][0] = Addr(memcard_gp(c, MEMCARD_FE_GP)).lw(c);
                c.g[V0][0] = Addr(r32(c, V1).wrapping_add(MEMCARD_FE_REPLY_PENDING)).lw(c);
                if c.g[V0][0] != 0 {
                    c.cycles += 3;
                    Addr(r32(c, V1).wrapping_add(MEMCARD_FE_REPLY_PENDING)).sw(c, 0);
                    Addr(r32(c, V1).wrapping_add(MEMCARD_FE_REPLY_FLAG)).sw(c, 0);
                } else {
                    c.g[V0][0] = Addr(r32(c, S0).wrapping_add(MEMCARD_FE_MESSAGE)).lw(c);
                    c.cycles += 2;
                    if c.g[V0][0] == c.g[A2][0] {
                        Addr(r32(c, S0).wrapping_add(MEMCARD_FE_POPUP_OPEN)).sw(c, 0);
                    }
                    c.cycles += 2;
                    c.g[V0][0] = Addr(memcard_gp(c, MEMCARD_FE_GP)).lw(c);
                    Addr(r32(c, V0).wrapping_add(MEMCARD_FE_POPUP)).sw(c, 0);
                }
                c.cycles += 1;
                frame.restore(c, S0, 0x30);
            }
        }
        c.cycles += 5;
        frame.restore(c, S1, 0x20);
        frame.restore(c, S2, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::DATA;
    use crate::difftest::GAME_GP;
    use crate::game::fe::fememcardcallbacks::tests::{
        hash_screen, memcard_t_setup, memcard_t_vtable, MEMCARD_T_CARD_VT, MEMCARD_T_ENGINE,
        MEMCARD_T_FE, MEMCARD_T_FE_VT, MEMCARD_T_MC, MEMCARD_T_POPUP_VT,
    };
    use crate::game::movie::movieplayer::tests::RET_ZERO;

    #[test]
    fn mem_card_state_base_on_child_event_matches() {
        check(
            ee::s04::f_001d8700__generated,
            mem_card_state_base_on_child_event,
            160,
            &|c, seed| {
                let mut r = memcard_t_setup(c, seed, 0x1d_8700, 8);
                // The fixture's vtable has slots 0x140/0x148; add slot 0x168.
                memcard_t_vtable(c, &mut r, MEMCARD_T_FE_VT, &[MEMCARD_FE_VT_SLOT45]);
                // The generated body dispatches through this jump table.
                for (i, target) in [
                    0x001d_87dcu32,
                    0x001d_8778,
                    0x001d_8930,
                    0x001d_8920,
                    0x001d_8920,
                    0x001d_8920,
                    0x001d_8a30,
                    0x001d_8b38,
                    0x001d_881c,
                    0x001d_88a4,
                    0x001d_88e4,
                    0x001d_8980,
                    0x001d_89e0,
                    0x001d_8adc,
                    0x001d_8adc,
                    0x001d_8a88,
                    0x001d_8ad4,
                    0x001d_885c,
                    0x001d_8798,
                    0x001d_8b68,
                    0x001d_8b68,
                    0x001d_8b68,
                    0x001d_8b38,
                    0x001d_8b54,
                ]
                .iter()
                .enumerate()
                {
                    c.bus.write32(MEMCARD_MSG_TABLE + 4 * i as u32, *target);
                }
                // The UI state manager the close-out pops: an engine whose
                // `+0x18` list is empty, so `UIStateMgr_CloseTopStates` (the
                // real leaf) takes its empty path on both sides.
                c.bus
                    .write32(MEMCARD_T_FE + MEMCARD_FE_ENGINE, MEMCARD_T_ENGINE);
                c.bus.write32(MEMCARD_T_ENGINE + MEMCARD_UI_STATE_MGR, 0);
                c.g[A0][0] = sx(MEMCARD_T_FE);
                c.g[A1][0] = r.next_u32() as u64;
                let button = [0xf, 0x10, 0x11, 0x16, 0x9][(seed % 5) as usize];
                c.g[A2][0] = button as u64;
                // Messages 1..=0x18, plus 0 and 0x19 out of range.
                let msg = (seed / 5) % 26;
                c.bus.write32(MEMCARD_T_FE + MEMCARD_FE_MESSAGE, msg);
                c.bus
                    .write32(MEMCARD_T_FE + MEMCARD_FE_LOAD_FROM_PAUSE, r.next_u32() % 2);
                c.bus
                    .write32(MEMCARD_T_FE + MEMCARD_FE_USE_SAVE_TITLE, r.next_u32() % 2);
                c.bus
                    .write32(MEMCARD_T_FE + MEMCARD_FE_HAS_SAVE, r.next_u32() % 2);
                c.bus
                    .write32(MEMCARD_T_FE + MEMCARD_FE_REPLY_PENDING, r.next_u32() % 2);
                c.pc = 0x001d_8700;
            },
        );
    }
}
