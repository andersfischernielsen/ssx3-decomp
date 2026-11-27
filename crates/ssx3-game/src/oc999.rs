//! Staging module oc999 (wave wave260): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc999`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::game::fe::fepopup::oc508_vtable_dtor;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0021_c900,
        ov_screen_yn_dialog_restart_event_dtor,
        "OVScreenYNDialogRestartEvent_dtor",
    ),
    (
        0x0021_c928,
        ov_screen_yn_dialog_quit_game_dtor,
        "OVScreenYNDialogQuitGame_dtor",
    ),
    (
        0x0021_c950,
        ov_screen_yn_dialog_quit_game_ctor,
        "OVScreenYNDialogQuitGame_ctor",
    ),
    (
        0x0021_c9c0,
        ov_screen_yn_dialog_restart_chal_dtor,
        "OVScreenYNDialogRestartChal_dtor",
    ),
    (
        0x0021_c9e8,
        ov_screen_yn_dialog_quit_chal_dtor,
        "OVScreenYNDialogQuitChal_dtor",
    ),
    (
        0x0021_ca10,
        ov_screen_yn_dialog_quit_event_dtor,
        "OVScreenYNDialogQuitEvent_dtor",
    ),
    (
        0x0021_ca38,
        ov_screen_yn_dialog_give_up_dtor,
        "OVScreenYNDialogGiveUp_dtor",
    ),
    (
        0x0021_ca60,
        ov_screen_yn_dialog_save_before_quit_dtor,
        "OVScreenYNDialogSaveBeforeQuit_dtor",
    ),
];

/// Vtable of the overlay state base class (`cOVState_vtbl`, 0x474e08).
const OC999_OV_STATE_VTBL: u32 = 0x0047_4e08;

/// Destructor of an overlay YN dialog (`cOVState` subclass): installs
/// `cOVState_vtbl` at +8 and runs the base `UIState` destructor (0x39e390).
/// The generated body, through the shared vtable-dtor helper.
fn oc999_yn_dtor(c: &mut Cpu, ret_addr: u32) -> crate::call::Flow {
    oc508_vtable_dtor(c, sx(OC999_OV_STATE_VTBL), 0x0039_e390, ret_addr)
}

/// `OVScreenYNDialogRestartEvent_dtor` (0x21c900).
pub fn ov_screen_yn_dialog_restart_event_dtor(c: &mut Cpu) {
    guest(c, |c| oc999_yn_dtor(c, 0x0021_c91c));
}

/// `OVScreenYNDialogQuitGame_dtor` (0x21c928).
pub fn ov_screen_yn_dialog_quit_game_dtor(c: &mut Cpu) {
    guest(c, |c| oc999_yn_dtor(c, 0x0021_c944));
}

/// `OVScreenYNDialogRestartChal_dtor` (0x21c9c0).
pub fn ov_screen_yn_dialog_restart_chal_dtor(c: &mut Cpu) {
    guest(c, |c| oc999_yn_dtor(c, 0x0021_c9dc));
}

/// `OVScreenYNDialogQuitChal_dtor` (0x21c9e8).
pub fn ov_screen_yn_dialog_quit_chal_dtor(c: &mut Cpu) {
    guest(c, |c| oc999_yn_dtor(c, 0x0021_ca04));
}

/// `OVScreenYNDialogQuitEvent_dtor` (0x21ca10).
pub fn ov_screen_yn_dialog_quit_event_dtor(c: &mut Cpu) {
    guest(c, |c| oc999_yn_dtor(c, 0x0021_ca2c));
}

/// `OVScreenYNDialogGiveUp_dtor` (0x21ca38).
pub fn ov_screen_yn_dialog_give_up_dtor(c: &mut Cpu) {
    guest(c, |c| oc999_yn_dtor(c, 0x0021_ca54));
}

/// `OVScreenYNDialogSaveBeforeQuit_dtor` (0x21ca60).
pub fn ov_screen_yn_dialog_save_before_quit_dtor(c: &mut Cpu) {
    guest(c, |c| oc999_yn_dtor(c, 0x0021_ca7c));
}

/// `OVScreenYNDialogQuitGame` constructor (0x21c950): `this` (a0), overlay id
/// (a2). Runs the base `UIState` constructor (0x39e2a0) with the arguments
/// unchanged, then stores the overlay id at +0x98, clears the transition and
/// pause-menu flags (+0x48, +0x4c) and the PDA flag (+0x54), sets the vtable to
/// the quit-game variant (0x4748f8), copies the first halfword of the chat
/// string (0x4a2088) to +0x58 and the dialog type 1 to +0x9c. Returns `this`.
pub fn ov_screen_yn_dialog_quit_game_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 28;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A2][0];
        call_ptr(c, 0x0039_e2a0, 0x0021_c96c)?;
        let this = c.g[S0][0] as u32;
        c.sw(this.wrapping_add(0x98), c.g[S1][0]);
        c.g[V1][0] = 0x0047_0000;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4e08));
        c.sw(this.wrapping_add(0x48), 0u64);
        c.sw(this.wrapping_add(0x8), c.g[V1][0]);
        c.g[A2][0] = 0x004a_0000;
        c.sw(this.wrapping_add(0x4c), 0u64);
        c.g[V1][0] = 0x0047_0000;
        c.sw(this.wrapping_add(0x54), 0u64);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x48f8));
        c.g[A1][0] = 0x1;
        c.g[V0][0] = c.g[S0][0];
        c.g[A0][0] = c.lhu((c.g[A2][0] as u32).wrapping_add(0x2088));
        c.sw(this.wrapping_add(0x8), c.g[V1][0]);
        c.sh(this.wrapping_add(0x58), c.g[A0][0]);
        c.sw(this.wrapping_add(0x9c), c.g[A1][0]);
        frame.restore(c, S0, 0x20);
        frame.restore(c, S1, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::fakes::machine;
    use crate::difftest::DATA;

    /// Fixture for the overlay dialogs: a machine with the object at DATA+0x2000.
    pub(crate) fn oc999_yn_setup(c: &mut Cpu, seed: u32, entry: u32) {
        machine(c, seed, entry as u64, 2);
        c.g[A0][0] = (DATA + 0x2000) as u64;
        c.g[A2][0] = (seed % 5) as u64;
        c.pc = entry;
    }

    #[test]
    fn ov_screen_yn_dialog_restart_event_dtor_matches() {
        check(
            ee::s05::f_0021c900__generated,
            ov_screen_yn_dialog_restart_event_dtor,
            200,
            &|c, seed| oc999_yn_setup(c, seed, 0x0021_c900),
        );
    }

    #[test]
    fn ov_screen_yn_dialog_quit_game_dtor_matches() {
        check(
            ee::s05::f_0021c928__generated,
            ov_screen_yn_dialog_quit_game_dtor,
            200,
            &|c, seed| oc999_yn_setup(c, seed, 0x0021_c928),
        );
    }

    #[test]
    fn ov_screen_yn_dialog_quit_game_ctor_matches() {
        check(
            ee::s05::f_0021c950__generated,
            ov_screen_yn_dialog_quit_game_ctor,
            200,
            &|c, seed| oc999_yn_setup(c, seed, 0x0021_c950),
        );
    }

    #[test]
    fn ov_screen_yn_dialog_restart_chal_dtor_matches() {
        check(
            ee::s05::f_0021c9c0__generated,
            ov_screen_yn_dialog_restart_chal_dtor,
            200,
            &|c, seed| oc999_yn_setup(c, seed, 0x0021_c9c0),
        );
    }

    #[test]
    fn ov_screen_yn_dialog_quit_chal_dtor_matches() {
        check(
            ee::s05::f_0021c9e8__generated,
            ov_screen_yn_dialog_quit_chal_dtor,
            200,
            &|c, seed| oc999_yn_setup(c, seed, 0x0021_c9e8),
        );
    }

    #[test]
    fn ov_screen_yn_dialog_quit_event_dtor_matches() {
        check(
            ee::s05::f_0021ca10__generated,
            ov_screen_yn_dialog_quit_event_dtor,
            200,
            &|c, seed| oc999_yn_setup(c, seed, 0x0021_ca10),
        );
    }

    #[test]
    fn ov_screen_yn_dialog_give_up_dtor_matches() {
        check(
            ee::s05::f_0021ca38__generated,
            ov_screen_yn_dialog_give_up_dtor,
            200,
            &|c, seed| oc999_yn_setup(c, seed, 0x0021_ca38),
        );
    }

    #[test]
    fn ov_screen_yn_dialog_save_before_quit_dtor_matches() {
        check(
            ee::s05::f_0021ca60__generated,
            ov_screen_yn_dialog_save_before_quit_dtor,
            200,
            &|c, seed| oc999_yn_setup(c, seed, 0x0021_ca60),
        );
    }
}
