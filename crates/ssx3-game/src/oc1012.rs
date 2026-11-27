//! Staging module oc1012 (wave wave261): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1012`.

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
        0x0022_1ac8,
        ov_screen_pause_mode5_dtor,
        "OVScreenPauseMode5_dtor",
    ),
    (
        0x0022_1af0,
        ov_screen_pause_mode6_ctor,
        "OVScreenPauseMode6_ctor",
    ),
    (
        0x0022_1b58,
        ov_screen_pause_mode6_ctor_2,
        "OVScreenPauseMode6_ctor_2",
    ),
    (
        0x0022_1bc8,
        ov_screen_pause_mode6_dtor,
        "OVScreenPauseMode6_dtor",
    ),
    (
        0x0022_1bf0,
        ov_screen_be_options_dtor,
        "OVScreenBEOptions_dtor",
    ),
    (
        0x0022_1c18,
        ov_screen_be_options_ctor,
        "OVScreenBEOptions_ctor",
    ),
    (
        0x0022_1c80,
        ov_screen_in_game_error_ctor,
        "OVScreenInGameError_ctor",
    ),
    (
        0x0022_1ce0,
        ov_screen_in_game_error_ctor_2,
        "OVScreenInGameError_ctor_2",
    ),
];

// The `cOVState` screens below derive from the UI state base: its constructor
// at 0x39e2a0 and destructor at 0x39e390, both reached through the function
// table (`call_ptr`). Their vtables sit in the game image at 0x47xxxx (the
// `+8` word of each object). Offsets of the state fields: `+0x48` done
// transition, `+0x4c` allow pause menu, `+0x54` remove PDA, `+0x58` chat
// buffer (u16), `+0x98` overlay id, `+0x9c` ping start tick, `+0xa0` display
// aux info flag, `+0xc4` thread.

/// Base `cOVState` vtable (game image 0x474e08): the first vptr of every
/// pause, options and in-game error state built here.
const OV_STATE_VTBL_BASE: u32 = 0x0047_4e08;
/// The `cOVState_PAUSE_*_vtbl` each constructor switches to.
const OV_PAUSE_VTBL_ONLINE: u32 = 0x0047_2c60;
const OV_PAUSE_VTBL_OPTIONS: u32 = 0x0047_2b88;
const OV_PAUSE_VTBL_ONLINE_ERROR: u32 = 0x0047_2ab0;
/// The high half loaded into the constructors' argument register before the
/// `str_sp_b[0]` read (`lhu` at +0x2088): the chat buffer's initial word.
const OV_STR_SP_B_HI: u32 = 0x004a_0000;

/// `void OVScreenPauseMode5_dtor(OVScreenPauseMode5 *self)` (0x221ac8): sets
/// the base vtable at `+8` and runs the UIState destructor.
pub fn ov_screen_pause_mode5_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V1][0] = c.g[A0][0];
        c.g[V0][0] = sx(OV_STATE_VTBL_BASE);
        c.sw((c.g[V1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
        call_ptr(c, 0x0039_e390, 0x0022_1ae4)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `OVScreenPauseMode6 *OVScreenPauseMode6_ctor(OVScreenPauseMode6 *self)`
/// (0x221af0): runs the UIState constructor (with a null second argument),
/// clears the state fields, sets the online-pause vtable and the chat buffer's
/// initial word. Returns `self` in `v0`.
pub fn ov_screen_pause_mode6_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 25;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0;
        frame.save(c, 16, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0039_e2a0, 0x0022_1b08)?;
        let this = c.g[S0][0] as u32;
        c.sw(this.wrapping_add(0x98), 0);
        c.sw(this.wrapping_add(0x48), 0);
        c.sw(this.wrapping_add(0x8), sx(OV_STATE_VTBL_BASE));
        c.g[A1][0] = OV_STR_SP_B_HI as u64;
        c.sw(this.wrapping_add(0x4c), 0);
        c.sw(this.wrapping_add(0x54), 0);
        c.g[V1][0] = sx(OV_PAUSE_VTBL_ONLINE);
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        c.g[A0][0] = c.lhu((c.g[A1][0] as u32).wrapping_add(0x2088));
        c.sw(this.wrapping_add(0x8), c.g[V1][0]);
        c.sh(this.wrapping_add(0x58), c.g[A0][0]);
        c.sw(this.wrapping_add(0xc4), 0);
        frame.restore(c, 16, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `OVScreenPauseMode6 *OVScreenPauseMode6_ctor_2(OVScreenPauseMode6 *self,
/// int a1, int overlayId)` (0x221b58): as `OVScreenPauseMode6_ctor`, but the
/// UIState constructor gets the caller's `a1` and the overlay id is stored at
/// `+0x98`. Returns `self` in `v0`.
pub fn ov_screen_pause_mode6_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 27;
        let frame = Frame::open(c, 0x30);
        frame.save(c, 16, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, 17, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A2][0];
        call_ptr(c, 0x0039_e2a0, 0x0022_1b74)?;
        let this = c.g[S0][0] as u32;
        c.sw(this.wrapping_add(0x98), c.g[S1][0]);
        c.sw(this.wrapping_add(0x48), 0);
        c.sw(this.wrapping_add(0x8), sx(OV_STATE_VTBL_BASE));
        c.g[A1][0] = OV_STR_SP_B_HI as u64;
        c.sw(this.wrapping_add(0x4c), 0);
        c.sw(this.wrapping_add(0x54), 0);
        c.g[V1][0] = sx(OV_PAUSE_VTBL_ONLINE);
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, 17, 0x10);
        c.g[A0][0] = c.lhu((c.g[A1][0] as u32).wrapping_add(0x2088));
        c.sw(this.wrapping_add(0x8), c.g[V1][0]);
        c.sh(this.wrapping_add(0x58), c.g[A0][0]);
        c.sw(this.wrapping_add(0xc4), 0);
        frame.restore(c, 16, 0x20);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void OVScreenPauseMode6_dtor(OVScreenPauseMode6 *self)` (0x221bc8): sets
/// the base vtable at `+8` and runs the UIState destructor.
pub fn ov_screen_pause_mode6_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V1][0] = c.g[A0][0];
        c.g[V0][0] = sx(OV_STATE_VTBL_BASE);
        c.sw((c.g[V1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
        call_ptr(c, 0x0039_e390, 0x0022_1be4)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `void OVScreenBEOptions_dtor(OVScreenBEOptions *self)` (0x221bf0): sets
/// the base vtable at `+8` and runs the UIState destructor.
pub fn ov_screen_be_options_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V1][0] = c.g[A0][0];
        c.g[V0][0] = sx(OV_STATE_VTBL_BASE);
        c.sw((c.g[V1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
        call_ptr(c, 0x0039_e390, 0x0022_1c0c)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `OVScreenBEOptions *OVScreenBEOptions_ctor(OVScreenBEOptions *self, int a1,
/// int overlayId)` (0x221c18): runs the UIState constructor with the caller's
/// `a1`, stores the overlay id at `+0x98`, sets the options vtable and the
/// chat buffer's initial word. Returns `self` in `v0`.
pub fn ov_screen_be_options_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x30);
        frame.save(c, 16, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, 17, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A2][0];
        call_ptr(c, 0x0039_e2a0, 0x0022_1c34)?;
        let this = c.g[S0][0] as u32;
        c.sw(this.wrapping_add(0x98), c.g[S1][0]);
        c.sw(this.wrapping_add(0x48), 0);
        c.sw(this.wrapping_add(0x8), sx(OV_STATE_VTBL_BASE));
        c.g[A1][0] = OV_STR_SP_B_HI as u64;
        c.sw(this.wrapping_add(0x4c), 0);
        c.sw(this.wrapping_add(0x54), 0);
        c.g[V1][0] = sx(OV_PAUSE_VTBL_OPTIONS);
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, 17, 0x10);
        c.g[A0][0] = c.lhu((c.g[A1][0] as u32).wrapping_add(0x2088));
        c.sw(this.wrapping_add(0x8), c.g[V1][0]);
        c.sh(this.wrapping_add(0x58), c.g[A0][0]);
        frame.restore(c, 16, 0x20);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `OVScreenInGameError *OVScreenInGameError_ctor(OVScreenInGameError *self)`
/// (0x221c80): runs the UIState constructor with a null second argument,
/// clears the state fields, sets the online-error vtable and the chat
/// buffer's initial word. Returns `self` in `v0`.
pub fn ov_screen_in_game_error_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0;
        frame.save(c, 16, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0039_e2a0, 0x0022_1c98)?;
        let this = c.g[S0][0] as u32;
        c.sw(this.wrapping_add(0x98), 0);
        c.sw(this.wrapping_add(0x48), 0);
        c.sw(this.wrapping_add(0x8), sx(OV_STATE_VTBL_BASE));
        c.g[A1][0] = OV_STR_SP_B_HI as u64;
        c.sw(this.wrapping_add(0x4c), 0);
        c.sw(this.wrapping_add(0x54), 0);
        c.g[V1][0] = sx(OV_PAUSE_VTBL_ONLINE_ERROR);
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        c.g[A0][0] = c.lhu((c.g[A1][0] as u32).wrapping_add(0x2088));
        c.sw(this.wrapping_add(0x8), c.g[V1][0]);
        c.sh(this.wrapping_add(0x58), c.g[A0][0]);
        frame.restore(c, 16, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `OVScreenInGameError *OVScreenInGameError_ctor_2(OVScreenInGameError *self,
/// int a1, int overlayId)` (0x221ce0): as `OVScreenInGameError_ctor`, but the
/// UIState constructor gets the caller's `a1`, the overlay id is stored at
/// `+0x98`, and the aux info flag (`+0xa0`) is set to 1 with the ping start
/// tick (`+0x9c`) cleared. Returns `self` in `v0`.
pub fn ov_screen_in_game_error_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 29;
        let frame = Frame::open(c, 0x30);
        frame.save(c, 16, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, 17, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A2][0];
        call_ptr(c, 0x0039_e2a0, 0x0022_1cfc)?;
        let this = c.g[S0][0] as u32;
        c.sw(this.wrapping_add(0x98), c.g[S1][0]);
        c.sw(this.wrapping_add(0x48), 0);
        c.sw(this.wrapping_add(0x8), sx(OV_STATE_VTBL_BASE));
        c.g[A2][0] = OV_STR_SP_B_HI as u64;
        c.sw(this.wrapping_add(0x4c), 0);
        c.sw(this.wrapping_add(0x54), 0);
        c.g[V1][0] = sx(OV_PAUSE_VTBL_ONLINE_ERROR);
        c.g[A1][0] = 0x1;
        c.g[V0][0] = c.g[S0][0];
        c.g[A0][0] = c.lhu((c.g[A2][0] as u32).wrapping_add(0x2088));
        c.sw(this.wrapping_add(0x8), c.g[V1][0]);
        c.sh(this.wrapping_add(0x58), c.g[A0][0]);
        c.sw(this.wrapping_add(0xa0), c.g[A1][0]);
        c.sw(this.wrapping_add(0x9c), 0);
        frame.restore(c, 16, 0x20);
        frame.restore(c, 17, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, covered, machine};
    use crate::difftest::DATA;
    use std::sync::atomic::AtomicUsize;

    /// Gate word of the oc1012 fakes (a fixture address of this machine only).
    const GATE: u32 = DATA + 0x0020_1200;
    const MAGIC: u32 = 0x0c10_1200;
    const SEEDS: u32 = 200;

    static OC1012_PREV_39E2A0: AtomicUsize = AtomicUsize::new(0);
    static OC1012_PREV_39E390: AtomicUsize = AtomicUsize::new(0);

    /// The gated fake for the UIState base: logs the call like `fake_call`
    /// while the gate word is set, else runs the previous body.
    fn oc1012_gated(c: &mut Cpu, target: u32, prev: &AtomicUsize, generated: fn(&mut Cpu)) {
        if c.bus.read32(GATE) != MAGIC {
            return fakes::fall_back(c, prev, generated);
        }
        fakes::fake_call(c, target);
    }

    fn oc1012_fake_39e2a0(c: &mut Cpu) {
        oc1012_gated(
            c,
            0x0039_e2a0,
            &OC1012_PREV_39E2A0,
            ee::s13::f_0039e2a0__generated,
        )
    }

    fn oc1012_fake_39e390(c: &mut Cpu) {
        oc1012_gated(
            c,
            0x0039_e390,
            &OC1012_PREV_39E390,
            ee::s13::f_0039e390__generated,
        )
    }

    fn oc1012_install_fakes() {
        use std::ptr::addr_of_mut;
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_0039E2A0),
            oc1012_fake_39e2a0,
            &OC1012_PREV_39E2A0,
        );
        fakes::hook(
            addr_of_mut!(ee::s13::OVERRIDE_0039E390),
            oc1012_fake_39e390,
            &OC1012_PREV_39E390,
        );
    }

    /// Scramble the machine (the `calls` fake unwinds at a random call) and
    /// arm the UIState fakes for this machine.
    fn oc1012_object(c: &mut Cpu, seed: u32, salt: u64, calls: u32) {
        machine(c, seed, salt, calls);
        c.bus.write32(GATE, MAGIC);
    }

    #[test]
    fn ov_screen_pause_mode5_dtor_matches() {
        oc1012_install_fakes();
        covered(
            ee::s05::f_00221ac8__generated,
            ov_screen_pause_mode5_dtor,
            SEEDS,
            &|c, seed| {
                oc1012_object(c, seed, 0x221ac8, 1);
                c.g[A0][0] = sx(DATA + 0x100 + (seed % 4) * 0x10);
            },
        );
    }

    #[test]
    fn ov_screen_pause_mode6_ctor_matches() {
        oc1012_install_fakes();
        covered(
            ee::s05::f_00221af0__generated,
            ov_screen_pause_mode6_ctor,
            SEEDS,
            &|c, seed| {
                oc1012_object(c, seed, 0x221af0, 1);
                c.g[A0][0] = sx(DATA + 0x100 + (seed % 4) * 0x10);
            },
        );
    }

    #[test]
    fn ov_screen_pause_mode6_ctor_2_matches() {
        oc1012_install_fakes();
        covered(
            ee::s05::f_00221b58__generated,
            ov_screen_pause_mode6_ctor_2,
            SEEDS,
            &|c, seed| {
                oc1012_object(c, seed, 0x221b58, 1);
                c.g[A0][0] = sx(DATA + 0x100 + (seed % 4) * 0x10);
                // `a1` is passed through to the UIState constructor.
                c.g[A1][0] = sx(seed.wrapping_mul(0x9e37_79b9));
                c.g[A2][0] = sx(0x30 + seed % 9);
            },
        );
    }

    #[test]
    fn ov_screen_pause_mode6_dtor_matches() {
        oc1012_install_fakes();
        covered(
            ee::s05::f_00221bc8__generated,
            ov_screen_pause_mode6_dtor,
            SEEDS,
            &|c, seed| {
                oc1012_object(c, seed, 0x221bc8, 1);
                c.g[A0][0] = sx(DATA + 0x100 + (seed % 4) * 0x10);
            },
        );
    }

    #[test]
    fn ov_screen_be_options_dtor_matches() {
        oc1012_install_fakes();
        covered(
            ee::s05::f_00221bf0__generated,
            ov_screen_be_options_dtor,
            SEEDS,
            &|c, seed| {
                oc1012_object(c, seed, 0x221bf0, 1);
                c.g[A0][0] = sx(DATA + 0x100 + (seed % 4) * 0x10);
            },
        );
    }

    #[test]
    fn ov_screen_be_options_ctor_matches() {
        oc1012_install_fakes();
        covered(
            ee::s05::f_00221c18__generated,
            ov_screen_be_options_ctor,
            SEEDS,
            &|c, seed| {
                oc1012_object(c, seed, 0x221c18, 1);
                c.g[A0][0] = sx(DATA + 0x100 + (seed % 4) * 0x10);
                c.g[A1][0] = sx(seed.wrapping_mul(0x85eb_ca6b));
                c.g[A2][0] = sx(0x40 + seed % 5);
            },
        );
    }

    #[test]
    fn ov_screen_in_game_error_ctor_matches() {
        oc1012_install_fakes();
        covered(
            ee::s05::f_00221c80__generated,
            ov_screen_in_game_error_ctor,
            SEEDS,
            &|c, seed| {
                oc1012_object(c, seed, 0x221c80, 1);
                c.g[A0][0] = sx(DATA + 0x100 + (seed % 4) * 0x10);
            },
        );
    }

    #[test]
    fn ov_screen_in_game_error_ctor_2_matches() {
        oc1012_install_fakes();
        covered(
            ee::s05::f_00221ce0__generated,
            ov_screen_in_game_error_ctor_2,
            SEEDS,
            &|c, seed| {
                oc1012_object(c, seed, 0x221ce0, 1);
                c.g[A0][0] = sx(DATA + 0x100 + (seed % 4) * 0x10);
                c.g[A1][0] = sx(seed.wrapping_mul(0xc2b2_ae35));
                c.g[A2][0] = sx(0x50 + seed % 6);
            },
        );
    }
}
