//! Staging module oc1001 (wave wave260): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1001`.

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
        0x0021_cd70,
        ov_screen_final_standings_dtor,
        "OVScreenFinalStandings_dtor",
    ),
    (
        0x0021_cd98,
        ov_screen_final_standings_ctor,
        "OVScreenFinalStandings_ctor",
    ),
    (
        0x0021_ce00,
        ov_screen_peak_challenge_results_dtor,
        "OVScreenPeakChallengeResults_dtor",
    ),
    (
        0x0021_ce28,
        ov_screen_peak_challenge_results_ctor,
        "OVScreenPeakChallengeResults_ctor",
    ),
    (
        0x0021_ce90,
        ov_screen_finish_ov_dtor,
        "OVScreenFinishOv_dtor",
    ),
    (
        0x0021_ceb8,
        ov_screen_finish_ov_ctor,
        "OVScreenFinishOv_ctor",
    ),
    (
        0x0021_cf38,
        ov_screen_finish_ov2_dtor,
        "OVScreenFinishOv2_dtor",
    ),
    (
        0x0021_cf60,
        ov_screen_finish_ov2_ctor,
        "OVScreenFinishOv2_ctor",
    ),
];

/// The game's global block (`0x470000`): the overlay vtables sit at fixed
/// offsets from it.
const OVST_GLOBALS: u32 = 0x0047_0000;
/// The base `cOVState` vtable (`OVST_GLOBALS + 0x4e08`), stored at `+8` first.
const OVST_VTABLE_BASE: u32 = OVST_GLOBALS + 0x4e08;
/// The config block (`0x4a0000`): its halfword at `+0x2088` goes to `+0x58`.
const OVST_CONFIG_BLOCK: u32 = 0x004a_0000;
/// The name string the finish overlays hash (`OVST_GLOBALS - 0x1cc0`).
const OVST_NAME: u32 = OVST_GLOBALS - 0x1cc0;
/// `cUIState` constructor (0x39e2a0), `cUIState` destructor (0x39e390), and
/// the string hash (0x317670), all called through the function table.
const OVST_UISTATE_CTOR: u32 = 0x0039_e2a0;
const OVST_UISTATE_DTOR: u32 = 0x0039_e390;
const OVST_STR_HASH: u32 = 0x0031_7670;

/// The destructor body shared by the four overlay `_dtor`s: stores the base
/// vtable at `+8` of `this` (`a0`), runs the `cUIState` destructor returning to
/// `ret_addr`, and returns to the caller.
fn ov_state_dtor_body(c: &mut Cpu, ret_addr: u32) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V1][0] = c.g[A0][0];
        c.g[V0][0] = u64::from(OVST_VTABLE_BASE);
        let vtable = c.g[V0][0];
        Addr((c.g[V1][0] as u32).wrapping_add(8)).sw(c, vtable);
        call_ptr(c, OVST_UISTATE_DTOR, ret_addr)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// The constructor body of the two standings overlays (`this` in `a0`, overlay
/// id in `a2`): runs the `cUIState` constructor returning to `base_ret`, then
/// stores the overlay id at `+0x98`, the vtables (base, then `final_vtable`)
/// at `+8`, zeroes at `+0x48`, `+0x4c`, `+0x54`, and the config halfword at
/// `+0x58`.
fn ov_state_ctor_standings_body(c: &mut Cpu, base_ret: u32, final_vtable: u32) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A2][0];
        call_ptr(c, OVST_UISTATE_CTOR, base_ret)?;
        let this = c.g[S0][0] as u32;
        let overlay = c.g[S1][0];
        Addr(this).off(0x98).sw(c, overlay);
        Addr(this).off(0x48).sw(c, 0);
        Addr(this).off(8).sw(c, u64::from(OVST_VTABLE_BASE));
        c.g[A1][0] = u64::from(OVST_CONFIG_BLOCK);
        Addr(this).off(0x4c).sw(c, 0);
        Addr(this).off(0x54).sw(c, 0);
        c.g[V1][0] = u64::from(final_vtable);
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x10);
        let config = Addr((c.g[A1][0] as u32).wrapping_add(0x2088)).lhu(c);
        c.g[A0][0] = config;
        let vtable = c.g[V1][0];
        Addr(this).off(8).sw(c, vtable);
        Addr(this).off(0x58).sh(c, config);
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// The constructor body of the two finish overlays (`this` in `a0`, overlay id
/// in `a2`): runs the `cUIState` constructor returning to `base_ret`, stores
/// the overlay id at `+0x98`, the base vtable at `+8`, zeroes at `+0x48` and
/// `+0x4c`, `+0x54` (then `+0xa0` = 1 and the config halfword at `+0x58`), then
/// sets the vtable to `vtable` and stores `StrHash(OVST_NAME, 1)` at `+0xc`
/// (the hash call returns to `name_ret`).
fn ov_state_ctor_finish_body(c: &mut Cpu, base_ret: u32, name_ret: u32, vtable: u32) {
    guest(c, |c| {
        c.cycles += 32;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A2][0];
        call_ptr(c, OVST_UISTATE_CTOR, base_ret)?;
        let this = c.g[S0][0] as u32;
        let overlay = c.g[S1][0];
        Addr(this).off(0x98).sw(c, overlay);
        Addr(this).off(0x48).sw(c, 0);
        Addr(this).off(8).sw(c, u64::from(OVST_VTABLE_BASE));
        c.g[A2][0] = u64::from(OVST_CONFIG_BLOCK);
        Addr(this).off(0x4c).sw(c, 0);
        Addr(this).off(0x54).sw(c, 0);
        c.g[V0][0] = u64::from(vtable);
        c.g[A1][0] = 1;
        c.g[A0][0] = u64::from(OVST_NAME);
        let config = Addr((c.g[A2][0] as u32).wrapping_add(0x2088)).lhu(c);
        c.g[V1][0] = config;
        let one = c.g[A1][0];
        Addr(this).off(0xa0).sw(c, one);
        Addr(this).off(0x58).sh(c, config);
        let vt = c.g[V0][0];
        Addr(this).off(8).sw(c, vt);
        call_ptr(c, OVST_STR_HASH, name_ret)?;
        let hash = c.g[V0][0];
        let this_after = c.g[S0][0] as u32;
        Addr(this_after).off(0xc).sw(c, hash);
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x10);
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `OVScreenFinalStandings::~OVScreenFinalStandings` (0x21cd70).
pub fn ov_screen_final_standings_dtor(c: &mut Cpu) {
    ov_state_dtor_body(c, 0x0021_cd8c);
}

/// `OVScreenFinalStandings::OVScreenFinalStandings` (0x21cd98): the standings
/// overlay constructor with the `0x473fb0` vtable.
pub fn ov_screen_final_standings_ctor(c: &mut Cpu) {
    ov_state_ctor_standings_body(c, 0x0021_cdb4, OVST_GLOBALS + 0x3fb0);
}

/// `OVScreenPeakChallengeResults::~OVScreenPeakChallengeResults` (0x21ce00).
pub fn ov_screen_peak_challenge_results_dtor(c: &mut Cpu) {
    ov_state_dtor_body(c, 0x0021_ce1c);
}

/// `OVScreenPeakChallengeResults::OVScreenPeakChallengeResults` (0x21ce28): the
/// standings constructor with the `0x473ed8` vtable.
pub fn ov_screen_peak_challenge_results_ctor(c: &mut Cpu) {
    ov_state_ctor_standings_body(c, 0x0021_ce44, OVST_GLOBALS + 0x3ed8);
}

/// `OVScreenFinishOv::~OVScreenFinishOv` (0x21ce90).
pub fn ov_screen_finish_ov_dtor(c: &mut Cpu) {
    ov_state_dtor_body(c, 0x0021_ceac);
}

/// `OVScreenFinishOv::OVScreenFinishOv` (0x21ceb8): the finish constructor
/// with the `0x473e00` vtable.
pub fn ov_screen_finish_ov_ctor(c: &mut Cpu) {
    ov_state_ctor_finish_body(c, 0x0021_ced4, 0x0021_cf1c, OVST_GLOBALS + 0x3e00);
}

/// `OVScreenFinishOv2::~OVScreenFinishOv2` (0x21cf38).
pub fn ov_screen_finish_ov2_dtor(c: &mut Cpu) {
    ov_state_dtor_body(c, 0x0021_cf54);
}

/// `OVScreenFinishOv2::OVScreenFinishOv2` (0x21cf60): the finish constructor
/// with the `0x473d28` vtable.
pub fn ov_screen_finish_ov2_ctor(c: &mut Cpu) {
    ov_state_ctor_finish_body(c, 0x0021_cf7c, 0x0021_cfc4, OVST_GLOBALS + 0x3d28);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::machine_rt;
    use crate::difftest::{check, DATA};

    /// A zeroed overlay object at `DATA+0x1000` (the base destructor's lists
    /// take their short path), `a1` = 0 so no `operator_delete` runs.
    fn ov_state_dtor_fixture(c: &mut Cpu, seed: u32, salt: u64) {
        machine_rt(c, seed, salt, 0);
        c.g[A0][0] = sx(DATA + 0x1000);
        c.g[A1][0] = 0;
    }

    /// A zeroed overlay object, the engine pointer in `a1` (stored, not read),
    /// a random overlay id in `a2`, and the config halfword the constructor
    /// copies into `+0x58`.
    fn ov_state_ctor_fixture(c: &mut Cpu, seed: u32, salt: u64) {
        let mut r = machine_rt(c, seed, salt, 0);
        c.g[A0][0] = sx(DATA + 0x1000);
        c.g[A1][0] = sx(DATA + 0x3000);
        c.g[A2][0] = sx(r.next_u32());
        c.bus
            .write16(OVST_CONFIG_BLOCK + 0x2088, (r.next_u32() & 0xffff) as u16);
    }

    #[test]
    fn ov_screen_final_standings_dtor_matches() {
        check(
            ee::s05::f_0021cd70__generated,
            ov_screen_final_standings_dtor,
            200,
            &|c, seed| ov_state_dtor_fixture(c, seed, 0x0021_cd70),
        );
    }

    #[test]
    fn ov_screen_final_standings_ctor_matches() {
        check(
            ee::s05::f_0021cd98__generated,
            ov_screen_final_standings_ctor,
            200,
            &|c, seed| ov_state_ctor_fixture(c, seed, 0x0021_cd98),
        );
    }

    #[test]
    fn ov_screen_peak_challenge_results_dtor_matches() {
        check(
            ee::s05::f_0021ce00__generated,
            ov_screen_peak_challenge_results_dtor,
            200,
            &|c, seed| ov_state_dtor_fixture(c, seed, 0x0021_ce00),
        );
    }

    #[test]
    fn ov_screen_peak_challenge_results_ctor_matches() {
        check(
            ee::s05::f_0021ce28__generated,
            ov_screen_peak_challenge_results_ctor,
            200,
            &|c, seed| ov_state_ctor_fixture(c, seed, 0x0021_ce28),
        );
    }

    #[test]
    fn ov_screen_finish_ov_dtor_matches() {
        check(
            ee::s05::f_0021ce90__generated,
            ov_screen_finish_ov_dtor,
            200,
            &|c, seed| ov_state_dtor_fixture(c, seed, 0x0021_ce90),
        );
    }

    #[test]
    fn ov_screen_finish_ov_ctor_matches() {
        check(
            ee::s05::f_0021ceb8__generated,
            ov_screen_finish_ov_ctor,
            200,
            &|c, seed| ov_state_ctor_fixture(c, seed, 0x0021_ceb8),
        );
    }

    #[test]
    fn ov_screen_finish_ov2_dtor_matches() {
        check(
            ee::s05::f_0021cf38__generated,
            ov_screen_finish_ov2_dtor,
            200,
            &|c, seed| ov_state_dtor_fixture(c, seed, 0x0021_cf38),
        );
    }

    #[test]
    fn ov_screen_finish_ov2_ctor_matches() {
        check(
            ee::s05::f_0021cf60__generated,
            ov_screen_finish_ov2_ctor,
            200,
            &|c, seed| ov_state_ctor_fixture(c, seed, 0x0021_cf60),
        );
    }
}
