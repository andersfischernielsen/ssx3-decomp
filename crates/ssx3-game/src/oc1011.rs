//! Staging module oc1011 (wave wave261): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1011`.

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
        0x0022_17f0,
        ov_screen_pause_mode3_ctor,
        "OVScreenPauseMode3_ctor",
    ),
    (
        0x0022_1858,
        ov_screen_pause_mode3_ctor_2,
        "OVScreenPauseMode3_ctor_2",
    ),
    (
        0x0022_18c8,
        ov_screen_pause_mode3_dtor,
        "OVScreenPauseMode3_dtor",
    ),
    (
        0x0022_18f0,
        ov_screen_pause_mode4_ctor,
        "OVScreenPauseMode4_ctor",
    ),
    (
        0x0022_1958,
        ov_screen_pause_mode4_ctor_2,
        "OVScreenPauseMode4_ctor_2",
    ),
    (
        0x0022_19c8,
        ov_screen_pause_mode4_dtor,
        "OVScreenPauseMode4_dtor",
    ),
    (
        0x0022_19f0,
        ov_screen_pause_mode5_ctor,
        "OVScreenPauseMode5_ctor",
    ),
    (
        0x0022_1a58,
        ov_screen_pause_mode5_ctor_2,
        "OVScreenPauseMode5_ctor_2",
    ),
];

/// The game's global block (`0x470000`): the pause overlay vtables sit at fixed
/// offsets from it.
const OVPAUSE_GLOBALS: u32 = 0x0047_0000;
/// The base `cOVState` vtable (`OVPAUSE_GLOBALS + 0x4e08`), stored at `+8` first.
const OVPAUSE_VTABLE_BASE: u32 = OVPAUSE_GLOBALS + 0x4e08;
/// The config block (`0x4a0000`): its halfword at `+0x2088` goes to `+0x58`.
const OVPAUSE_CONFIG_BLOCK: u32 = 0x004a_0000;
/// The `cUIState` constructor (0x39e2a0) and destructor (0x39e390), called
/// through the function table.
const OVPAUSE_UISTATE_CTOR: u32 = 0x0039_e2a0;
const OVPAUSE_UISTATE_DTOR: u32 = 0x0039_e390;

/// The destructor body of the pause overlays (`this` in `a0`): stores the base
/// `cOVState` vtable at `+8` of `this`, runs the `cUIState` destructor
/// returning to `ret_addr`, and returns to the caller.
fn ov_pause_dtor_body(c: &mut Cpu, ret_addr: u32) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V1][0] = c.g[A0][0];
        c.g[V0][0] = u64::from(OVPAUSE_VTABLE_BASE);
        let vtable = c.g[V0][0];
        Addr((c.g[V1][0] as u32).wrapping_add(8)).sw(c, vtable);
        call_ptr(c, OVPAUSE_UISTATE_DTOR, ret_addr)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// The constructor body of the plain pause overlays (`this` in `a0`, no overlay
/// id): runs the `cUIState` constructor returning to `base_ret` with `a1` = 0,
/// zeroes the overlay id at `+0x98`, `+0x48`, `+0x4c`, `+0x54` and `+0xc4`,
/// stores the base vtable at `+8` and then the pause vtable
/// (`OVPAUSE_GLOBALS + final_offset`) at `+8`, and the config halfword at `+0x58`.
fn ov_pause_ctor_plain_body(c: &mut Cpu, base_ret: u32, final_offset: u32) {
    guest(c, |c| {
        c.cycles += 25;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, OVPAUSE_UISTATE_CTOR, base_ret)?;
        let this = c.g[S0][0] as u32;
        Addr(this).off(0x98).sw(c, 0);
        Addr(this).off(0x48).sw(c, 0);
        Addr(this).off(8).sw(c, u64::from(OVPAUSE_VTABLE_BASE));
        c.g[A1][0] = u64::from(OVPAUSE_CONFIG_BLOCK);
        Addr(this).off(0x4c).sw(c, 0);
        Addr(this).off(0x54).sw(c, 0);
        c.g[V1][0] = u64::from(OVPAUSE_GLOBALS + final_offset);
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        let config = Addr((c.g[A1][0] as u32).wrapping_add(0x2088)).lhu(c);
        c.g[A0][0] = config;
        let final_vtable = c.g[V1][0];
        Addr(this).off(8).sw(c, final_vtable);
        Addr(this).off(0x58).sh(c, config);
        Addr(this).off(0xc4).sw(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    });
}

/// The constructor body of the pause overlays that take an overlay id (`this`
/// in `a0`, overlay id in `a2`): as the plain constructor, but the id is stored
/// at `+0x98` and kept in `s1` across the `cUIState` constructor.
fn ov_pause_ctor_overlay_body(c: &mut Cpu, base_ret: u32, final_offset: u32) {
    guest(c, |c| {
        c.cycles += 27;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A2][0];
        call_ptr(c, OVPAUSE_UISTATE_CTOR, base_ret)?;
        let this = c.g[S0][0] as u32;
        let overlay = c.g[S1][0];
        Addr(this).off(0x98).sw(c, overlay);
        Addr(this).off(0x48).sw(c, 0);
        Addr(this).off(8).sw(c, u64::from(OVPAUSE_VTABLE_BASE));
        c.g[A1][0] = u64::from(OVPAUSE_CONFIG_BLOCK);
        Addr(this).off(0x4c).sw(c, 0);
        Addr(this).off(0x54).sw(c, 0);
        c.g[V1][0] = u64::from(OVPAUSE_GLOBALS + final_offset);
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x10);
        let config = Addr((c.g[A1][0] as u32).wrapping_add(0x2088)).lhu(c);
        c.g[A0][0] = config;
        let final_vtable = c.g[V1][0];
        Addr(this).off(8).sw(c, final_vtable);
        Addr(this).off(0x58).sh(c, config);
        Addr(this).off(0xc4).sw(c, 0);
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `OVScreenPauseMode3::~OVScreenPauseMode3` (0x2218c8).
pub fn ov_screen_pause_mode3_dtor(c: &mut Cpu) {
    ov_pause_dtor_body(c, 0x0022_18e4);
}

/// `OVScreenPauseMode3::OVScreenPauseMode3` (0x2217f0): the plain constructor
/// with the `PAUSE_FREERIDE` vtable.
pub fn ov_screen_pause_mode3_ctor(c: &mut Cpu) {
    ov_pause_ctor_plain_body(c, 0x0022_1808, 0x2ee8);
}

/// `OVScreenPauseMode3` constructor taking the overlay id (0x221858).
pub fn ov_screen_pause_mode3_ctor_2(c: &mut Cpu) {
    ov_pause_ctor_overlay_body(c, 0x0022_1874, 0x2ee8);
}

/// `OVScreenPauseMode4::~OVScreenPauseMode4` (0x2219c8).
pub fn ov_screen_pause_mode4_dtor(c: &mut Cpu) {
    ov_pause_dtor_body(c, 0x0022_19e4);
}

/// `OVScreenPauseMode4::OVScreenPauseMode4` (0x2218f0): the plain constructor
/// with the `PAUSE_SPOKE` vtable.
pub fn ov_screen_pause_mode4_ctor(c: &mut Cpu) {
    ov_pause_ctor_plain_body(c, 0x0022_1908, 0x2e10);
}

/// `OVScreenPauseMode4` constructor taking the overlay id (0x221958).
pub fn ov_screen_pause_mode4_ctor_2(c: &mut Cpu) {
    ov_pause_ctor_overlay_body(c, 0x0022_1974, 0x2e10);
}

/// `OVScreenPauseMode5::OVScreenPauseMode5` (0x2219f0): the plain constructor
/// with the `PAUSE_QUICKPLAY` vtable.
pub fn ov_screen_pause_mode5_ctor(c: &mut Cpu) {
    ov_pause_ctor_plain_body(c, 0x0022_1a08, 0x2d38);
}

/// `OVScreenPauseMode5` constructor taking the overlay id (0x221a58).
pub fn ov_screen_pause_mode5_ctor_2(c: &mut Cpu) {
    ov_pause_ctor_overlay_body(c, 0x0022_1a74, 0x2d38);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::machine_rt;
    use crate::difftest::{check, DATA};

    /// A zeroed overlay object at `DATA+0x1000` (the base destructor's lists
    /// take their short path), `a1` = 0 so no `operator_delete` runs.
    fn pause_dtor_fixture(c: &mut Cpu, seed: u32, salt: u64) {
        machine_rt(c, seed, salt, 0);
        c.g[A0][0] = sx(DATA + 0x1000);
        c.g[A1][0] = 0;
    }

    /// A zeroed overlay object, the engine pointer in `a1` (stored, not read),
    /// a random overlay id in `a2`, and the config halfword the constructor
    /// copies into `+0x58`.
    fn pause_ctor_fixture(c: &mut Cpu, seed: u32, salt: u64) {
        let mut r = machine_rt(c, seed, salt, 0);
        c.g[A0][0] = sx(DATA + 0x1000);
        c.g[A1][0] = sx(DATA + 0x3000);
        c.g[A2][0] = sx(r.next_u32());
        c.bus.write16(
            OVPAUSE_CONFIG_BLOCK + 0x2088,
            (r.next_u32() & 0xffff) as u16,
        );
    }

    #[test]
    fn ov_screen_pause_mode3_dtor_matches() {
        check(
            ee::s05::f_002218c8__generated,
            ov_screen_pause_mode3_dtor,
            200,
            &|c, seed| pause_dtor_fixture(c, seed, 0x0022_18c8),
        );
    }

    #[test]
    fn ov_screen_pause_mode3_ctor_matches() {
        check(
            ee::s05::f_002217f0__generated,
            ov_screen_pause_mode3_ctor,
            200,
            &|c, seed| pause_ctor_fixture(c, seed, 0x0022_17f0),
        );
    }

    #[test]
    fn ov_screen_pause_mode3_ctor_2_matches() {
        check(
            ee::s05::f_00221858__generated,
            ov_screen_pause_mode3_ctor_2,
            200,
            &|c, seed| pause_ctor_fixture(c, seed, 0x0022_1858),
        );
    }

    #[test]
    fn ov_screen_pause_mode4_dtor_matches() {
        check(
            ee::s05::f_002219c8__generated,
            ov_screen_pause_mode4_dtor,
            200,
            &|c, seed| pause_dtor_fixture(c, seed, 0x0022_19c8),
        );
    }

    #[test]
    fn ov_screen_pause_mode4_ctor_matches() {
        check(
            ee::s05::f_002218f0__generated,
            ov_screen_pause_mode4_ctor,
            200,
            &|c, seed| pause_ctor_fixture(c, seed, 0x0022_18f0),
        );
    }

    #[test]
    fn ov_screen_pause_mode4_ctor_2_matches() {
        check(
            ee::s05::f_00221958__generated,
            ov_screen_pause_mode4_ctor_2,
            200,
            &|c, seed| pause_ctor_fixture(c, seed, 0x0022_1958),
        );
    }

    #[test]
    fn ov_screen_pause_mode5_ctor_matches() {
        check(
            ee::s05::f_002219f0__generated,
            ov_screen_pause_mode5_ctor,
            200,
            &|c, seed| pause_ctor_fixture(c, seed, 0x0022_19f0),
        );
    }

    #[test]
    fn ov_screen_pause_mode5_ctor_2_matches() {
        check(
            ee::s05::f_00221a58__generated,
            ov_screen_pause_mode5_ctor_2,
            200,
            &|c, seed| pause_ctor_fixture(c, seed, 0x0022_1a58),
        );
    }
}
