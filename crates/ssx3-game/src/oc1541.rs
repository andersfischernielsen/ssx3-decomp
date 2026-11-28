//! Staging module oc1541 (wave wave277): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1541`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame, Unwound};
use crate::events::{deliver_event_here, event_due};
use crate::game::fe::fepopupmisc::ui_state_ctor_store;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x001d_e558,
        cfe_state_background_ctor_2,
        "cFEStateBackground_ctor_2",
    ),
    (
        0x001d_e5f0,
        cfe_state_transition_ctor_2,
        "cFEStateTransition_ctor_2",
    ),
    (
        0x001d_ea80,
        cfe_popup_option_reset_on_widget_menu,
        "cFEPopupOptionReset_onWidgetMenu",
    ),
    (
        0x001e_81b0,
        cov_state_race_finishov_on_widget_menu,
        "cOVState_RACE_FINISHOV_onWidgetMenu",
    ),
    (
        0x001e_88d0,
        cov_state_bassp_qual_finishov_on_widget_menu,
        "cOVState_BASSSP_QUAL_FINISHOV_onWidgetMenu",
    ),
    (
        0x001f_72e0,
        ov_enter_lodge_widget_action,
        "OVScreenEnterLodge_OnWidgetAction",
    ),
];

/// Vtable of `cFEStateBackground` (0x46bc58 = 0x470000 - 0x43a8).
pub(crate) const CFE_STATE_BACKGROUND_VTABLE: u32 = 0x0046_bc58;

/// Vtable of `cFEStateTransition` (0x46bb88 = 0x470000 - 0x4478).
pub(crate) const CFE_STATE_TRANSITION_VTABLE: u32 = 0x0046_bb88;

/// `cFEStateBackground_ctor_2(self)` (0x1de558): `UIState_ctor(self, 0)`, then
/// the background state's vtable at `self+8`. Returns `self`.
pub fn cfe_state_background_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        ui_state_ctor_store(c, CFE_STATE_BACKGROUND_VTABLE as u64, 0x001d_e570)
    });
}

/// `cFEStateTransition_ctor_2(self)` (0x1de5f0): `UIState_ctor(self, 0)`, then
/// the transition state's vtable at `self+8`. Returns `self`.
pub fn cfe_state_transition_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        ui_state_ctor_store(c, CFE_STATE_TRANSITION_VTABLE as u64, 0x001d_e608)
    });
}

/// `cFEPopupOptionReset_onWidgetMenu` (0x1dea80): maps the menu key in `a2`
/// (the third argument) to a result. Keys below 7 and keys of 10 or more give
/// 0x101; keys 7, 8 and 9 give 0x100. The key is compared as an unsigned
/// 64-bit value, so a negative key counts as large. `a0` is left as the
/// "key below 7" flag when the key is below 10.
pub fn cfe_popup_option_reset_on_widget_menu(c: &mut Cpu) {
    c.cycles += 3;
    let below_ten = (c.g[A2][0] < 10) as u64;
    c.g[V1][0] = below_ten;
    c.g[V0][0] = 0x101;
    if below_ten != 0 {
        c.cycles += 4;
        let below_seven = (c.g[A2][0] < 7) as u64;
        c.g[A0][0] = below_seven;
        c.g[V1][0] = 0x101;
        c.g[V0][0] = 0x100;
        if below_seven != 0 {
            c.g[V0][0] = c.g[V1][0];
        }
    }
    c.cycles += 2;
    ret(c);
}

/// Shared body of the two "finish overlay" `onWidgetMenu` handlers
/// (`cOVState_RACE_FINISHOV_onWidgetMenu` 0x1e81b0 and
/// `cOVState_BASSSP_QUAL_FINISHOV_onWidgetMenu` 0x1e88d0). With the message in
/// `a2` equal to 6, the result is 0x101 when the word at `this+0xa0` is
/// non-zero and 0 when it is zero; any other message gives 0.
fn finish_ov_on_widget_menu(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V1][0] = 6;
    c.g[V0][0] = 0;
    if c.g[A2][0] == 6 {
        c.cycles += 3;
        c.g[V1][0] = Addr((c.g[A0][0] as u32).wrapping_add(0xa0)).lw(c);
        c.g[V0][0] = 0x101;
        if c.g[V1][0] == 0 {
            c.g[V0][0] = 0;
        }
    }
    c.cycles += 2;
    ret(c);
}

/// `cOVState_RACE_FINISHOV_onWidgetMenu` (0x1e81b0). See `finish_ov_on_widget_menu`.
pub fn cov_state_race_finishov_on_widget_menu(c: &mut Cpu) {
    finish_ov_on_widget_menu(c);
}

/// `cOVState_BASSSP_QUAL_FINISHOV_onWidgetMenu` (0x1e88d0). See `finish_ov_on_widget_menu`.
pub fn cov_state_bassp_qual_finishov_on_widget_menu(c: &mut Cpu) {
    finish_ov_on_widget_menu(c);
}

/// The game-object cell is `gp - 0x848`; this returns the address `gp - off`.
fn enter_lodge_gp_cell(c: &Cpu, off: u32) -> u32 {
    (c.g[GP][0] as u32).wrapping_sub(off)
}

/// `OVScreenEnterLodge_OnWidgetAction(this, widget, msg)` (0x1f72e0): the
/// "enter lodge" prompt. A null widget only returns. Message 5 (next) on
/// widget misc 0 quits to the load screen (`GameModule_QuitToLoadScreen`);
/// misc 1 closes the top states and sets `this+0x54` (mRemovePDA). Message 6
/// (prev) sets `this+0x54`, marks the game's overlay done and closes the top
/// states. Other messages return.
pub fn ov_enter_lodge_widget_action(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 4;
        let frame = Frame::open(c, 0x10);
        c.g[A3][0] = c.g[A0][0];
        let no_widget = c.g[A1][0] == 0;
        frame.save_ra(c, 0);
        // `true` when the path leaves through the shared exit (one more cycle
        // and the `ra` reload); the other paths reload `ra` themselves.
        let shared_exit = if no_widget {
            true
        } else {
            enter_lodge_dispatch(c, frame)?
        };
        if shared_exit {
            c.cycles += 1;
            frame.restore_ra(c, 0);
        }
        c.cycles += 2;
        frame.close(c);
        Ok(())
    });
}

/// The message dispatch of [`ov_enter_lodge_widget_action`] for a non-null
/// widget. Returns whether the shared exit block runs.
fn enter_lodge_dispatch(c: &mut Cpu, frame: Frame) -> Result<bool, Unwound> {
    c.cycles += 3;
    c.g[V0][0] = 5;
    let is_next = c.g[A2][0] == c.g[V0][0];
    c.g[V0][0] = 6;
    if !is_next {
        c.cycles += 2;
        let is_prev = c.g[A2][0] == c.g[V0][0];
        frame.restore_ra(c, 0);
        if is_prev {
            enter_lodge_prev(c, frame)?;
            return Ok(true);
        }
        c.cycles += 2;
        return Ok(false);
    }
    c.cycles += 8;
    c.g[A0][0] = Addr(enter_lodge_gp_cell(c, 0x848)).lw(c);
    c.g[A2][0] = 1;
    c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x84)).lw(c);
    c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x200)).lw(c);
    let a2 = c.g[A2][0];
    Addr((c.g[V1][0] as u32).wrapping_add(0x8)).sw(c, a2);
    c.g[V0][0] = Addr((c.g[A1][0] as u32).wrapping_add(0x18)).lw(c);
    if c.g[V0][0] == 0 {
        c.g[A0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x84)).lw(c);
        enter_lodge_quit(c, frame)?;
        return Ok(false);
    }
    c.cycles += 2;
    let is_one = c.g[V0][0] == c.g[A2][0];
    frame.restore_ra(c, 0);
    if is_one {
        enter_lodge_close_states(c, frame)?;
    } else {
        c.cycles += 2;
    }
    Ok(false)
}

/// The "quit" arm (message 5, misc 0): mark the state, then
/// `GameModule_QuitToLoadScreen(state, 0)` (0x230488) with the start-state
/// word set to 39.
fn enter_lodge_quit(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 6;
    c.g[V0][0] = 0x27;
    let v0 = c.g[V0][0];
    Addr(enter_lodge_gp_cell(c, 0x172c)).sw(c, v0);
    c.g[A1][0] = 0;
    call_ptr(c, 0x0023_0488, 0x001f_734c)?;
    frame.restore_ra(c, 0);
    Ok(())
}

/// The "close" arm (message 5, misc 1): `this+0x54 = misc`, then
/// `UIStateMgr_CloseTopStates(&engine->stack, 1)` (0x39f190).
fn enter_lodge_close_states(c: &mut Cpu, frame: Frame) -> Flow {
    c.cycles += 7;
    c.g[A0][0] = Addr((c.g[A3][0] as u32).wrapping_add(0x10)).lw(c);
    c.g[A1][0] = 1;
    let misc = c.g[V0][0];
    Addr((c.g[A3][0] as u32).wrapping_add(0x54)).sw(c, misc);
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
    call_ptr(c, 0x0039_f190, 0x001f_7368)?;
    frame.restore_ra(c, 0);
    Ok(())
}

/// The message-6 arm: `this+0x54 = 1`, the game's overlay-done flag = 1, then
/// `UIStateMgr_CloseTopStates(&engine->stack, 1)` (0x39f190). Returns to the
/// shared exit.
fn enter_lodge_prev(c: &mut Cpu, _frame: Frame) -> Flow {
    c.cycles += 10;
    c.g[A0][0] = Addr(enter_lodge_gp_cell(c, 0x848)).lw(c);
    c.g[A2][0] = 1;
    let one = c.g[A2][0];
    Addr((c.g[A3][0] as u32).wrapping_add(0x54)).sw(c, one);
    c.g[A1][0] = 1;
    c.g[V0][0] = Addr((c.g[A0][0] as u32).wrapping_add(0x84)).lw(c);
    c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x200)).lw(c);
    Addr((c.g[V1][0] as u32).wrapping_add(0x8)).sw(c, one);
    c.g[A0][0] = Addr((c.g[A3][0] as u32).wrapping_add(0x10)).lw(c);
    c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
    call_ptr(c, 0x0039_f190, 0x001f_7398)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, fakes, fakes::machine, DATA, GAME_GP};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate word of the enter-lodge fakes (in the machine's RAM).
    const ENTER_LODGE_CTRL: u32 = DATA + 0x0020_2300;
    const ENTER_LODGE_MAGIC: u32 = 0x1541_0e1e;
    const ENTER_LODGE_THIS: u32 = DATA + 0x1000;
    const ENTER_LODGE_ENGINE: u32 = DATA + 0x2000;
    const ENTER_LODGE_WIDGET: u32 = DATA + 0x3000;
    const ENTER_LODGE_GAME: u32 = DATA + 0x4000;
    const ENTER_LODGE_STATE: u32 = DATA + 0x5000;
    const ENTER_LODGE_OVERLAY: u32 = DATA + 0x6000;

    static ENTER_LODGE_PREV_QUIT: AtomicUsize = AtomicUsize::new(0);
    static ENTER_LODGE_PREV_CLOSE: AtomicUsize = AtomicUsize::new(0);

    /// `GameModule_QuitToLoadScreen` (0x230488): gated, else the body it replaced.
    fn enter_lodge_fake_quit(c: &mut Cpu) {
        if c.bus.read32(ENTER_LODGE_CTRL) != ENTER_LODGE_MAGIC {
            return fakes::fall_back(c, &ENTER_LODGE_PREV_QUIT, ee::s06::f_00230488__generated);
        }
        fakes::fake_call(c, 0x0023_0488);
    }

    /// `UIStateMgr_CloseTopStates` (0x39f190): gated, else the body it replaced.
    fn enter_lodge_fake_close(c: &mut Cpu) {
        if c.bus.read32(ENTER_LODGE_CTRL) != ENTER_LODGE_MAGIC {
            return fakes::fall_back(c, &ENTER_LODGE_PREV_CLOSE, ee::s13::f_0039f190__generated);
        }
        fakes::fake_call(c, 0x0039_f190);
    }

    fn enter_lodge_install() {
        fakes::hook(
            addr_of_mut!(ee::s06::OVERRIDE_00230488),
            enter_lodge_fake_quit,
            &ENTER_LODGE_PREV_QUIT,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_0039F190),
            enter_lodge_fake_close,
            &ENTER_LODGE_PREV_CLOSE,
        );
    }

    #[test]
    fn ov_enter_lodge_widget_action_matches() {
        enter_lodge_install();
        check(
            ee::s04::f_001f72e0__generated,
            ov_enter_lodge_widget_action,
            300,
            &|c, seed| {
                let _ = machine(c, seed, 0x001f_72e0, 0);
                c.bus.write32(ENTER_LODGE_CTRL, ENTER_LODGE_MAGIC);
                c.bus.write32(GAME_GP.wrapping_sub(0x848), ENTER_LODGE_GAME);
                c.bus.write32(ENTER_LODGE_GAME + 0x84, ENTER_LODGE_STATE);
                c.bus.write32(ENTER_LODGE_STATE + 0x200, ENTER_LODGE_OVERLAY);
                c.bus.write32(ENTER_LODGE_THIS + 0x10, ENTER_LODGE_ENGINE);
                c.g[A0][0] = sx(ENTER_LODGE_THIS);
                // Null widget on every 11th seed; otherwise a widget whose misc
                // data is 0, 1 or 2, and messages 5 (next), 6 (prev), 7.
                c.g[A1][0] = if seed % 11 == 0 {
                    0
                } else {
                    sx(ENTER_LODGE_WIDGET)
                };
                c.bus.write32(
                    ENTER_LODGE_WIDGET + 0x18,
                    match seed % 3 {
                        0 => 0,
                        1 => 1,
                        _ => 2,
                    },
                );
                c.g[A2][0] = match seed % 5 {
                    0 | 2 => 5,
                    1 => 6,
                    _ => 7,
                };
            },
        );
    }

    /// The object the constructors build (`this`), inside the data window.
    const CTOR_SELF: u32 = DATA + 0x1000;

    /// The state the finish handlers read (`this+0xa0`).
    const FINISH_SELF: u32 = DATA + 0x1000;

    fn ctor_fixture(c: &mut Cpu, seed: u32, salt: u64) {
        let _ = machine(c, seed, salt, 0);
        c.g[A0][0] = sx(CTOR_SELF);
    }

    #[test]
    fn cfe_state_background_ctor_2_matches() {
        check(
            ee::s04::f_001de558__generated,
            cfe_state_background_ctor_2,
            200,
            &|c, seed| ctor_fixture(c, seed, 0x001d_e558),
        );
    }

    #[test]
    fn cfe_state_transition_ctor_2_matches() {
        check(
            ee::s04::f_001de5f0__generated,
            cfe_state_transition_ctor_2,
            200,
            &|c, seed| ctor_fixture(c, seed, 0x001d_e5f0),
        );
    }

    #[test]
    fn cfe_popup_option_reset_on_widget_menu_matches() {
        check(
            ee::s04::f_001dea80__generated,
            cfe_popup_option_reset_on_widget_menu,
            300,
            &|c, seed| {
                let _ = machine(c, seed, 0x001d_ea80, 0);
                // Keys below 7, 7..9, 10 and up, negative, and a high-word key.
                c.g[A2][0] = match seed % 7 {
                    0 => 0,
                    1 => 6,
                    2 => 7,
                    3 => 9,
                    4 => 10,
                    5 => sx(0xffff_ffff),
                    _ => (1u64 << 40) | 3,
                };
            },
        );
    }

    fn finish_fixture(c: &mut Cpu, seed: u32, salt: u64) {
        let _ = machine(c, seed, salt, 0);
        c.g[A0][0] = sx(FINISH_SELF);
        // The word at +0xa0: zero, non-zero, and a value with high bits set.
        let word = match seed % 3 {
            0 => 0,
            1 => 1,
            _ => 0x1234_5678,
        };
        c.bus.write32(FINISH_SELF + 0xa0, word);
        // The message: 6 (the handled case), other 32-bit values, and a
        // 64-bit value whose low word is 6 (must not match).
        c.g[A2][0] = match seed % 4 {
            0 => 6,
            1 => 5,
            2 => 7,
            _ => (1u64 << 32) | 6,
        };
    }

    #[test]
    fn cov_state_race_finishov_on_widget_menu_matches() {
        check(
            ee::s04::f_001e81b0__generated,
            cov_state_race_finishov_on_widget_menu,
            200,
            &|c, seed| finish_fixture(c, seed, 0x001e_81b0),
        );
    }

    #[test]
    fn cov_state_bassp_qual_finishov_on_widget_menu_matches() {
        check(
            ee::s04::f_001e88d0__generated,
            cov_state_bassp_qual_finishov_on_widget_menu,
            200,
            &|c, seed| finish_fixture(c, seed, 0x001e_88d0),
        );
    }
}
