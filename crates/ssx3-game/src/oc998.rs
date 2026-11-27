//! Staging module oc998 (wave wave260): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc998`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x0021_c708,
        ov_screen_overlay_menu_dtor,
        "OVScreenOverlayMenu_dtor",
    ),
    (
        0x0021_c730,
        ov_screen_overlay_menu_ctor,
        "OVScreenOverlayMenu_ctor",
    ),
    (
        0x0021_c798,
        ov_screen_yn_dialog_dtor,
        "OVScreenYNDialog_dtor",
    ),
    (
        0x0021_c7c0,
        ov_screen_yn_dialog_ctor,
        "OVScreenYNDialog_ctor",
    ),
    (
        0x0021_c828,
        ov_screen_yn_dialog_update,
        "OVScreenYNDialog_Update",
    ),
    (0x0021_c848, ov_screen_id0e_dtor, "OVScreenId0E_dtor"),
    (
        0x0021_c870,
        ov_screen_replay_menu_dtor,
        "OVScreenReplayMenu_dtor",
    ),
    (
        0x0021_c898,
        ov_screen_replay_menu_ctor,
        "OVScreenReplayMenu_ctor",
    ),
];

/// Vtable word of the `cOVState` family: `0x470000 + 0x4e08` (`0x474e08`), the
/// value the destructors and the overlay-menu constructors store at `+0x08`.
const OV_STATE_VTBL_LO: u32 = 0x0047_0000;
const OV_STATE_VTBL_BASE_OFF: u32 = 0x4e08;

/// Shared body of the four `cOVState` destructor wrappers (0x21c708, 0x21c798,
/// 0x21c848, 0x21c870): stores the base vtable at `this+0x08` and tail-calls the
/// UI state destructor (`UIState_dtor`, 0x39e390) returning to `ret`. The
/// callee's unwind returns without restoring the frame, as the generated code.
fn oc998_ov_state_dtor_body(c: &mut Cpu, ret: u32) -> crate::call::Flow {
    c.cycles += 10;
    let frame = Frame::open(c, 0x10);
    c.g[V0][0] = OV_STATE_VTBL_LO as u64;
    frame.save_ra(c, 0);
    c.g[V1][0] = c.g[A0][0];
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(OV_STATE_VTBL_BASE_OFF));
    c.sw((c.g[V1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
    call_ptr(c, 0x0039_e390, ret)?;
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// Shared body of the three overlay-state constructors (0x21c730, 0x21c7c0,
/// 0x21c898). Runs `UIState_ctor` (0x39e2a0) on `this` (a0) returning to
/// `ui_ret`, then sets the overlay's owner word (`+0x98` = a2, the overlay id),
/// clears the `+0x48`, `+0x4c` and `+0x54` words, stores the vtable word at
/// `+0x08` (`vtbl_off` past `0x470000`, the last store wins) and the chat
/// buffer's first halfword (`str_sp_b[0]` at `0x4a2088`) at `+0x58`.
fn oc998_ov_state_ctor_body(c: &mut Cpu, ui_ret: u32, vtbl_off: u32) -> crate::call::Flow {
    c.cycles += 26;
    let frame = Frame::open(c, 0x30);
    frame.save(c, S0, 0x20);
    c.g[S0][0] = c.g[A0][0];
    frame.save(c, S1, 0x10);
    frame.save_ra(c, 0);
    c.g[S1][0] = c.g[A2][0];
    call_ptr(c, 0x0039_e2a0, ui_ret)?;
    c.g[V1][0] = OV_STATE_VTBL_LO as u64;
    c.sw((c.g[S0][0] as u32).wrapping_add(0x98), c.g[S1][0]);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(OV_STATE_VTBL_BASE_OFF));
    c.sw((c.g[S0][0] as u32).wrapping_add(0x48), 0u64);
    c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V1][0]);
    c.g[A1][0] = 0x004a_0000;
    c.sw((c.g[S0][0] as u32).wrapping_add(0x4c), 0u64);
    c.g[V1][0] = OV_STATE_VTBL_LO as u64;
    c.sw((c.g[S0][0] as u32).wrapping_add(0x54), 0u64);
    c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(vtbl_off));
    c.g[V0][0] = c.g[S0][0];
    frame.restore(c, S1, 0x10);
    c.g[A0][0] = c.lhu((c.g[A1][0] as u32).wrapping_add(0x2088));
    c.sw((c.g[S0][0] as u32).wrapping_add(0x8), c.g[V1][0]);
    c.sh((c.g[S0][0] as u32).wrapping_add(0x58), c.g[A0][0]);
    frame.restore(c, S0, 0x20);
    frame.restore_ra(c, 0);
    frame.close(c);
    Ok(())
}

/// `cOVTemplate_Simple` destructor for the overlay menu (0x21c708): base
/// `cOVState` vtable, then `UIState_dtor`.
pub fn ov_screen_overlay_menu_dtor(c: &mut Cpu) {
    guest(c, |c| oc998_ov_state_dtor_body(c, 0x0021_c724));
}

/// `cOVTemplate_Simple` constructor for the overlay menu (0x21c730): runs
/// `UIState_ctor`, then sets the overlay state (see `oc998_ov_state_ctor_body`) with
/// the overlay-menu vtable (`0x474d30`).
pub fn ov_screen_overlay_menu_ctor(c: &mut Cpu) {
    guest(c, |c| oc998_ov_state_ctor_body(c, 0x0021_c74c, 0x4d30));
}

/// `cOVTemplate_Dialog` destructor for the yes/no dialog (0x21c798).
pub fn ov_screen_yn_dialog_dtor(c: &mut Cpu) {
    guest(c, |c| oc998_ov_state_dtor_body(c, 0x0021_c7b4));
}

/// `cOVTemplate_Dialog` constructor for the yes/no dialog (0x21c7c0): the
/// shared overlay-state constructor with the dialog vtable (`0x474c58`).
pub fn ov_screen_yn_dialog_ctor(c: &mut Cpu) {
    guest(c, |c| oc998_ov_state_ctor_body(c, 0x0021_c7dc, 0x4c58));
}

/// `cOVTemplate_Dialog_onUpdate` (0x21c828): runs the overlay state's update
/// (`cOVStateOverlay_Update`, 0x20e900) and returns.
pub fn ov_screen_yn_dialog_update(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 7;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        Callee {
            addr: 0x0020_e900,
            f: ee::s05::f_0020e900,
        }
        .call(c, 0x0021_c838)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// Destructor of the id-0x0e overlay (0x21c848): the shared `cOVState`
/// destructor body.
pub fn ov_screen_id0e_dtor(c: &mut Cpu) {
    guest(c, |c| oc998_ov_state_dtor_body(c, 0x0021_c864));
}

/// `cOVState_REPLAY` destructor (0x21c870): the shared `cOVState` destructor
/// body, as the generated code has it.
pub fn ov_screen_replay_menu_dtor(c: &mut Cpu) {
    guest(c, |c| oc998_ov_state_dtor_body(c, 0x0021_c88c));
}

/// `cOVState_REPLAY` constructor (0x21c898): the shared overlay-state
/// constructor with the replay vtable (`0x474aa8`).
pub fn ov_screen_replay_menu_ctor(c: &mut Cpu) {
    guest(c, |c| oc998_ov_state_ctor_body(c, 0x0021_c8b4, 0x4aa8));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine};
    use crate::difftest::{check, DATA};
    use std::ptr::addr_of_mut;
    use std::sync::atomic::AtomicUsize;

    /// Gate word of this module's fakes (its own address and magic in the
    /// machine's RAM, so other tests' fakes stay off).
    const OC998_CTRL: u32 = DATA + 0x0020_9980;
    const OC998_MAGIC: u32 = 0x0998_0c0d;

    static OC998_PREV_UI_STATE_CTOR: AtomicUsize = AtomicUsize::new(0);
    static OC998_PREV_UI_STATE_DTOR: AtomicUsize = AtomicUsize::new(0);
    static OC998_PREV_OV_STATE_UPDATE: AtomicUsize = AtomicUsize::new(0);

    /// `UIState_ctor` with its gate on: logs the call and returns to the
    /// caller, so only the wrapper's own work is compared.
    fn oc998_fake_ui_state_ctor(c: &mut Cpu) {
        if c.bus.read32(OC998_CTRL) != OC998_MAGIC {
            return fakes::fall_back(c, &OC998_PREV_UI_STATE_CTOR, ee::s13::f_0039e2a0__generated);
        }
        fakes::fake_call(c, 0x0039_e2a0);
    }

    /// `UIState_dtor` with its gate on (see `oc998_fake_ui_state_ctor`).
    fn oc998_fake_ui_state_dtor(c: &mut Cpu) {
        if c.bus.read32(OC998_CTRL) != OC998_MAGIC {
            return fakes::fall_back(c, &OC998_PREV_UI_STATE_DTOR, ee::s13::f_0039e390__generated);
        }
        fakes::fake_call(c, 0x0039_e390);
    }

    /// `cOVStateOverlay_Update` with its gate on (see `oc998_fake_ui_state_ctor`).
    fn oc998_fake_ov_state_update(c: &mut Cpu) {
        if c.bus.read32(OC998_CTRL) != OC998_MAGIC {
            return fakes::fall_back(
                c,
                &OC998_PREV_OV_STATE_UPDATE,
                ee::s05::f_0020e900__generated,
            );
        }
        fakes::fake_call(c, 0x0020_e900);
    }

    /// Puts this module's three gated fakes in their override slots (once per
    /// process; with the gate off they run the body they replaced).
    fn oc998_hook() {
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_0039E2A0),
            oc998_fake_ui_state_ctor,
            &OC998_PREV_UI_STATE_CTOR,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_0039E390),
            oc998_fake_ui_state_dtor,
            &OC998_PREV_UI_STATE_DTOR,
        );
        fakes::hook(
            addr_of_mut!(ee::s05::OVERRIDE_0020E900),
            oc998_fake_ov_state_update,
            &OC998_PREV_OV_STATE_UPDATE,
        );
    }

    /// Scrambled registers and the gate on; the object at `DATA + 0x2000`
    /// (varied by seed) is the `this` argument. `calls` sets how early the
    /// fakes may unwind.
    fn oc998_setup(c: &mut Cpu, seed: u32, salt: u64, calls: u32) {
        machine(c, seed, salt, calls);
        c.bus.write32(OC998_CTRL, OC998_MAGIC);
        c.g[A0][0] = (DATA + 0x2000 + (seed % 4) * 0x100) as u64;
    }

    #[test]
    fn ov_screen_overlay_menu_dtor_matches() {
        oc998_hook();
        check(
            ee::s05::f_0021c708__generated,
            ov_screen_overlay_menu_dtor,
            200,
            &|c, seed| oc998_setup(c, seed, 0x0021_c708, 2),
        );
    }

    #[test]
    fn ov_screen_overlay_menu_ctor_matches() {
        oc998_hook();
        check(
            ee::s05::f_0021c730__generated,
            ov_screen_overlay_menu_ctor,
            200,
            &|c, seed| oc998_setup(c, seed, 0x0021_c730, 2),
        );
    }

    #[test]
    fn ov_screen_yn_dialog_dtor_matches() {
        oc998_hook();
        check(
            ee::s05::f_0021c798__generated,
            ov_screen_yn_dialog_dtor,
            200,
            &|c, seed| oc998_setup(c, seed, 0x0021_c798, 2),
        );
    }

    #[test]
    fn ov_screen_yn_dialog_ctor_matches() {
        oc998_hook();
        check(
            ee::s05::f_0021c7c0__generated,
            ov_screen_yn_dialog_ctor,
            200,
            &|c, seed| oc998_setup(c, seed, 0x0021_c7c0, 2),
        );
    }

    #[test]
    fn ov_screen_yn_dialog_update_matches() {
        oc998_hook();
        check(
            ee::s05::f_0021c828__generated,
            ov_screen_yn_dialog_update,
            200,
            &|c, seed| oc998_setup(c, seed, 0x0021_c828, 1),
        );
    }

    #[test]
    fn ov_screen_id0e_dtor_matches() {
        oc998_hook();
        check(
            ee::s05::f_0021c848__generated,
            ov_screen_id0e_dtor,
            200,
            &|c, seed| oc998_setup(c, seed, 0x0021_c848, 2),
        );
    }

    #[test]
    fn ov_screen_replay_menu_dtor_matches() {
        oc998_hook();
        check(
            ee::s05::f_0021c870__generated,
            ov_screen_replay_menu_dtor,
            200,
            &|c, seed| oc998_setup(c, seed, 0x0021_c870, 2),
        );
    }

    #[test]
    fn ov_screen_replay_menu_ctor_matches() {
        oc998_hook();
        check(
            ee::s05::f_0021c898__generated,
            ov_screen_replay_menu_ctor,
            200,
            &|c, seed| oc998_setup(c, seed, 0x0021_c898, 2),
        );
    }
}
