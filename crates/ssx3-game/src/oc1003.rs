//! Staging module oc1003 (wave wave260): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1003`.

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
        0x0022_0e90,
        cfe_state_career_stats_dtor,
        "cFEStateCareerStats_dtor",
    ),
    (
        0x0022_0eb8,
        ov_screen_enter_lodge_dtor,
        "OVScreenEnterLodge_dtor",
    ),
    (
        0x0022_0ee0,
        ov_screen_enter_lodge_ctor,
        "OVScreenEnterLodge_ctor",
    ),
    (0x0022_0f48, ov_screen_bc_start_ctor, "OVScreenBCStart_ctor"),
    (
        0x0022_0fa8,
        ov_screen_bc_start_ctor_2,
        "OVScreenBCStart_ctor_2",
    ),
    (0x0022_1018, ov_screen_bc_start_dtor, "OVScreenBCStart_dtor"),
    (0x0022_1040, ov_screen_bc_fail_ctor, "OVScreenBCFail_ctor"),
    (
        0x0022_10a0,
        ov_screen_bc_fail_ctor_2,
        "OVScreenBCFail_ctor_2",
    ),
];

/// `cFEStateCareerStats::~cFEStateCareerStats` (0x220e90): stores the career
/// stats vtable (the `0x470000` page less `0x2f30`) at `this+8`, then calls the
/// base destructor `UIState_dtor` (0x39e390) on `this`. Returns to the caller.
pub fn cfe_state_career_stats_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V1][0] = c.g[A0][0];
        c.g[V0][0] = sx(0x0047_0000u32.wrapping_sub(0x2f30));
        c.sw((c.g[V1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
        call_ptr(c, 0x0039_e390, 0x0022_0eac)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cOVState_ENTERLODGE::~cOVState_ENTERLODGE` (0x220eb8): stores the
/// ENTERLODGE vtable (`0x470000 + 0x4e08`) at `this+8`, then calls
/// `UIState_dtor` (0x39e390) on `this`.
pub fn ov_screen_enter_lodge_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V1][0] = c.g[A0][0];
        c.g[V0][0] = sx(0x0047_0000u32.wrapping_add(0x4e08));
        c.sw((c.g[V1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
        call_ptr(c, 0x0039_e390, 0x0022_0ed4)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cOVState_ENTERLODGE::cOVState_ENTERLODGE(this, engine, overlayId)`
/// (0x220ee0): the base constructor `UIState_ctor` (0x39e2a0) with the engine
/// argument, then the overlay id at `+0x98`, the vtable at `+8` (stored twice:
/// the second value, `0x473760`, stays), zeroed words at `+0x48/+0x4c/+0x54`,
/// and the 16-bit word at `+0x58` read from `0x4a2088`.
pub fn ov_screen_enter_lodge_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 26;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A2][0];
        call_ptr(c, 0x0039_e2a0, 0x0022_0efc)?;
        let this = c.g[S0][0] as u32;
        c.g[V1][0] = 0x0047_0000;
        c.sw(this.wrapping_add(0x98), c.g[S1][0]);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4e08));
        c.sw(this.wrapping_add(0x48), 0);
        c.sw(this.wrapping_add(0x8), c.g[V1][0]);
        c.g[A1][0] = 0x004a_0000;
        c.sw(this.wrapping_add(0x4c), 0);
        c.g[V1][0] = 0x0047_0000;
        c.sw(this.wrapping_add(0x54), 0);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x3760));
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x10);
        c.g[A0][0] = c.lhu((c.g[A1][0] as u32).wrapping_add(0x2088));
        c.sw(this.wrapping_add(0x8), c.g[V1][0]);
        c.sh(this.wrapping_add(0x58), c.g[A0][0]);
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cOVState_BIGCHALLENGE_START::cOVState_BIGCHALLENGE_START(this, engine,
/// overlayId)` (0x220f48, `func_00220F48` in the reference): `UIState_ctor`
/// with a zero engine argument, then the zeroed words, the BIGCHALLENGE_START
/// vtable (`0x473688`) at `+8`, and the 16-bit word at `+0x58`.
pub fn ov_screen_bc_start_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0039_e2a0, 0x0022_0f60)?;
        let this = c.g[S0][0] as u32;
        c.g[V1][0] = 0x0047_0000;
        c.sw(this.wrapping_add(0x98), 0);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4e08));
        c.sw(this.wrapping_add(0x48), 0);
        c.sw(this.wrapping_add(0x8), c.g[V1][0]);
        c.g[A1][0] = 0x004a_0000;
        c.sw(this.wrapping_add(0x4c), 0);
        c.g[V1][0] = 0x0047_0000;
        c.sw(this.wrapping_add(0x54), 0);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x3688));
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        c.g[A0][0] = c.lhu((c.g[A1][0] as u32).wrapping_add(0x2088));
        c.sw(this.wrapping_add(0x8), c.g[V1][0]);
        c.sh(this.wrapping_add(0x58), c.g[A0][0]);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `cOVState_BIGCHALLENGE_START` constructor, the variant with the overlay id
/// (0x220fa8): `UIState_ctor` with the engine argument, the overlay id at
/// `+0x98`, the vtable at `+8`, zeroed words, the 16-bit word at `+0x58`, then
/// `OVScreenBCStart_PopEventArg` (0x1f7548) on `this` (returns to 0x221000).
pub fn ov_screen_bc_start_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 28;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A2][0];
        call_ptr(c, 0x0039_e2a0, 0x0022_0fc4)?;
        let this = c.g[S0][0] as u32;
        c.g[V1][0] = 0x0047_0000;
        c.sw(this.wrapping_add(0x98), c.g[S1][0]);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4e08));
        c.sw(this.wrapping_add(0x48), 0);
        c.sw(this.wrapping_add(0x8), c.g[V1][0]);
        c.g[A1][0] = 0x004a_0000;
        c.sw(this.wrapping_add(0x4c), 0);
        c.g[V0][0] = 0x0047_0000;
        c.sw(this.wrapping_add(0x54), 0);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3688));
        c.g[A0][0] = c.g[S0][0];
        c.g[V1][0] = c.lhu((c.g[A1][0] as u32).wrapping_add(0x2088));
        c.sw(this.wrapping_add(0x8), c.g[V0][0]);
        c.sh(this.wrapping_add(0x58), c.g[V1][0]);
        call_ptr(c, 0x001f_7548, 0x0022_1000)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x10);
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cOVState_BIGCHALLENGE_START::~cOVState_BIGCHALLENGE_START` (0x221018):
/// the BIGCHALLENGE_START vtable (`0x470000 + 0x4e08`) at `this+8`, then
/// `UIState_dtor` (0x39e390) on `this`.
pub fn ov_screen_bc_start_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        frame.save_ra(c, 0);
        c.g[V1][0] = c.g[A0][0];
        c.g[V0][0] = sx(0x0047_0000u32.wrapping_add(0x4e08));
        c.sw((c.g[V1][0] as u32).wrapping_add(0x8), c.g[V0][0]);
        call_ptr(c, 0x0039_e390, 0x0022_1034)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cOVState_BIGCHALLENGE_FAIL::cOVState_BIGCHALLENGE_FAIL(this)` (0x221040):
/// `UIState_ctor` with a zero engine argument, then the zeroed words, the
/// BIGCHALLENGE_FAIL vtable (`0x4735b0`) at `+8`, and the 16-bit word at `+0x58`.
pub fn ov_screen_bc_fail_ctor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 24;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0;
        frame.save(c, S0, 0x10);
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x0039_e2a0, 0x0022_1058)?;
        let this = c.g[S0][0] as u32;
        c.g[V1][0] = 0x0047_0000;
        c.sw(this.wrapping_add(0x98), 0);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4e08));
        c.sw(this.wrapping_add(0x48), 0);
        c.sw(this.wrapping_add(0x8), c.g[V1][0]);
        c.g[A1][0] = 0x004a_0000;
        c.sw(this.wrapping_add(0x4c), 0);
        c.g[V1][0] = 0x0047_0000;
        c.sw(this.wrapping_add(0x54), 0);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x35b0));
        c.g[V0][0] = c.g[S0][0];
        frame.restore_ra(c, 0);
        c.g[A0][0] = c.lhu((c.g[A1][0] as u32).wrapping_add(0x2088));
        c.sw(this.wrapping_add(0x8), c.g[V1][0]);
        c.sh(this.wrapping_add(0x58), c.g[A0][0]);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    })
}

/// `cOVState_BIGCHALLENGE_FAIL` constructor, the variant with the overlay id
/// (0x2210a0): as the BIGCHALLENGE_START variant, with the FAIL vtable
/// (`0x4735b0`), then `OVScreenBCFail_PopEventArg` (0x1f7738) on `this`
/// (returns to 0x2210f8).
pub fn ov_screen_bc_fail_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 28;
        let frame = Frame::open(c, 0x30);
        frame.save(c, S0, 0x20);
        c.g[S0][0] = c.g[A0][0];
        frame.save(c, S1, 0x10);
        frame.save_ra(c, 0);
        c.g[S1][0] = c.g[A2][0];
        call_ptr(c, 0x0039_e2a0, 0x0022_10bc)?;
        let this = c.g[S0][0] as u32;
        c.g[V1][0] = 0x0047_0000;
        c.sw(this.wrapping_add(0x98), c.g[S1][0]);
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x4e08));
        c.sw(this.wrapping_add(0x48), 0);
        c.sw(this.wrapping_add(0x8), c.g[V1][0]);
        c.g[A1][0] = 0x004a_0000;
        c.sw(this.wrapping_add(0x4c), 0);
        c.g[V0][0] = 0x0047_0000;
        c.sw(this.wrapping_add(0x54), 0);
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x35b0));
        c.g[A0][0] = c.g[S0][0];
        c.g[V1][0] = c.lhu((c.g[A1][0] as u32).wrapping_add(0x2088));
        c.sw(this.wrapping_add(0x8), c.g[V0][0]);
        c.sh(this.wrapping_add(0x58), c.g[V1][0]);
        call_ptr(c, 0x001f_7738, 0x0022_10f8)?;
        c.g[V0][0] = c.g[S0][0];
        frame.restore(c, S1, 0x10);
        frame.restore(c, S0, 0x20);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::fakes::machine;
    use crate::difftest::{DATA, GAME_GP};

    /// The fixture shared by the eight functions: scrambled registers, `this`
    /// at `DATA + 0x100`, the engine (a1) and overlay id (a2) varied by seed.
    fn object_fixture(c: &mut Cpu, seed: u32, salt: u64) -> u32 {
        let mut r = machine(c, seed, salt, 0);
        let this = DATA + 0x100;
        c.g[A0][0] = sx(this);
        c.g[A1][0] = (seed % 4) as u64;
        c.g[A2][0] = if seed % 3 == 0 {
            0
        } else {
            r.next_u32() as u64
        };
        this
    }

    /// The constructors' `_2` variants call a `PopEventArg` helper that pops
    /// the global queue at `gp-0xce8`: an empty queue takes its fallback.
    fn empty_pop_queue(c: &mut Cpu, queue: u32) {
        c.bus.write32(GAME_GP + 0xce8, queue);
        c.bus.write32(queue + 0x1c4, 0);
    }

    #[test]
    fn cfe_state_career_stats_dtor_matches() {
        check(
            ee::s05::f_00220e90__generated,
            cfe_state_career_stats_dtor,
            200,
            &|c, seed| {
                object_fixture(c, seed, 0x0022_0e90);
            },
        );
    }

    #[test]
    fn ov_screen_enter_lodge_dtor_matches() {
        check(
            ee::s05::f_00220eb8__generated,
            ov_screen_enter_lodge_dtor,
            200,
            &|c, seed| {
                object_fixture(c, seed, 0x0022_0eb8);
            },
        );
    }

    #[test]
    fn ov_screen_enter_lodge_ctor_matches() {
        check(
            ee::s05::f_00220ee0__generated,
            ov_screen_enter_lodge_ctor,
            200,
            &|c, seed| {
                object_fixture(c, seed, 0x0022_0ee0);
            },
        );
    }

    #[test]
    fn ov_screen_bc_start_ctor_matches() {
        check(
            ee::s05::f_00220f48__generated,
            ov_screen_bc_start_ctor,
            200,
            &|c, seed| {
                object_fixture(c, seed, 0x0022_0f48);
            },
        );
    }

    #[test]
    fn ov_screen_bc_start_ctor_2_matches() {
        check(
            ee::s05::f_00220fa8__generated,
            ov_screen_bc_start_ctor_2,
            200,
            &|c, seed| {
                object_fixture(c, seed, 0x0022_0fa8);
                empty_pop_queue(c, DATA + 0x1000);
            },
        );
    }

    #[test]
    fn ov_screen_bc_start_dtor_matches() {
        check(
            ee::s05::f_00221018__generated,
            ov_screen_bc_start_dtor,
            200,
            &|c, seed| {
                object_fixture(c, seed, 0x0022_1018);
            },
        );
    }

    #[test]
    fn ov_screen_bc_fail_ctor_matches() {
        check(
            ee::s05::f_00221040__generated,
            ov_screen_bc_fail_ctor,
            200,
            &|c, seed| {
                object_fixture(c, seed, 0x0022_1040);
            },
        );
    }

    #[test]
    fn ov_screen_bc_fail_ctor_2_matches() {
        check(
            ee::s05::f_002210a0__generated,
            ov_screen_bc_fail_ctor_2,
            200,
            &|c, seed| {
                object_fixture(c, seed, 0x0022_10a0);
                empty_pop_queue(c, DATA + 0x200);
            },
        );
    }
}
