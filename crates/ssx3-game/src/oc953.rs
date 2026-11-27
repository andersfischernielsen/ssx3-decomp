//! Staging module oc953 (wave wave256): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc953`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame, Target};
use crate::events::{deliver_event_here, event_due};
use crate::game::fe::feasyncfile::{fe_ctor_with_vtable, fe_dtor_with_vtable};
use crate::game::fe::festatebonusmaterial::{FE_UI_STATE_CTOR, FE_UI_STATE_DTOR};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x001e_0578,
        cfe_state_request_line_ctor_2,
        "cFEStateRequestLine_ctor_2",
    ),
    (
        0x001e_05b0,
        cfe_state_request_line_dtor,
        "cFEStateRequestLine_dtor",
    ),
    (
        0x001e_07c0,
        fe_screen_sel_multi_ctor_2,
        "FEScreenSelMulti_ctor_2",
    ),
    (
        0x001e_07f8,
        fe_screen_sel_multi_dtor,
        "FEScreenSelMulti_dtor",
    ),
    (
        0x001e_0820,
        cfe_state_mp_circuit_ctor_2,
        "cFEStateMPCircuit_ctor_2",
    ),
    (
        0x001e_0858,
        cfe_state_mp_circuit_dtor,
        "cFEStateMPCircuit_dtor",
    ),
    (
        0x001e_0880,
        cfe_use_existing_account_ctor_2,
        "cFEUseExistingAccount_ctor_2",
    ),
    (
        0x001e_08d8,
        cfe_state_select_persona_ctor_2,
        "cFEStateSelectPersona_ctor_2",
    ),
];

/// `cFEStateRequestLine` constructor (0x1e0578): `UIState_ctor(this, 0)`, then
/// the vtable `0x470000 - 0x6c88` at `+8`.
pub fn cfe_state_request_line_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        fe_ctor_with_vtable(
            c,
            14,
            true,
            false,
            Target::Table(FE_UI_STATE_CTOR),
            0x001e_0590,
            0x6c88,
        )
    });
}

/// `cFEStateRequestLine` destructor (0x1e05b0): the vtable `0x470000 - 0x2f30`
/// at `+8`, then `UIState_dtor`.
pub fn cfe_state_request_line_dtor(c: &mut Cpu) {
    guest(c, |c| {
        fe_dtor_with_vtable(c, 10, Target::Table(FE_UI_STATE_DTOR), 0x2f30, 0x001e_05cc)
    });
}

/// `FEScreenSelMulti` constructor (0x1e07c0): `UIState_ctor(this, 0)`, then the
/// vtable `0x470000 - 0x6db8` at `+8`.
pub fn fe_screen_sel_multi_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        fe_ctor_with_vtable(
            c,
            14,
            true,
            false,
            Target::Table(FE_UI_STATE_CTOR),
            0x001e_07d8,
            0x6db8,
        )
    });
}

/// `FEScreenSelMulti` destructor (0x1e07f8): the vtable `0x470000 - 0x2f30` at
/// `+8`, then `UIState_dtor`.
pub fn fe_screen_sel_multi_dtor(c: &mut Cpu) {
    guest(c, |c| {
        fe_dtor_with_vtable(c, 10, Target::Table(FE_UI_STATE_DTOR), 0x2f30, 0x001e_0814)
    });
}

/// `cFEStateMPCircuit` constructor (0x1e0820): `UIState_ctor(this, 0)`, then
/// the vtable `0x470000 - 0x6e88` at `+8`.
pub fn cfe_state_mp_circuit_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        fe_ctor_with_vtable(
            c,
            14,
            true,
            false,
            Target::Table(FE_UI_STATE_CTOR),
            0x001e_0838,
            0x6e88,
        )
    });
}

/// `cFEStateMPCircuit` destructor (0x1e0858): the vtable `0x470000 - 0x2f30`
/// at `+8`, then `UIState_dtor`.
pub fn cfe_state_mp_circuit_dtor(c: &mut Cpu) {
    guest(c, |c| {
        fe_dtor_with_vtable(c, 10, Target::Table(FE_UI_STATE_DTOR), 0x2f30, 0x001e_0874)
    });
}

/// `cFEUseExistingAccount` constructor (0x1e0880): `UIState_ctor`-style base
/// construction at `0x1a8500` with `a1 = a2 = 0`, the vtable
/// `0x470000 - 0x6f98` at `+8`, then the pointer `gp[0xda0]` at `+0x6dc` and
/// zeros at `+0x6d0` and `+0x6e0`.
pub fn cfe_use_existing_account_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0;
        frame.save(c, S0, 0x10);
        c.g[A2][0] = 0;
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x001a_8500, 0x001e_089c)?;
        c.g[A0][0] = c.lw((c.g[GP][0] as u32).wrapping_add(0xda0));
        c.g[V1][0] = sx(0x0047_0000u32.wrapping_sub(0x6f98));
        c.g[V0][0] = c.g[S0][0];
        let this = c.g[S0][0] as u32;
        c.sw(this.wrapping_add(0x8), c.g[V1][0]);
        c.sw(this.wrapping_add(0x6dc), c.g[A0][0]);
        c.sw(this.wrapping_add(0x6d0), 0);
        c.sw(this.wrapping_add(0x6e0), 0);
        frame.restore_ra(c, 0);
        frame.restore(c, S0, 0x10);
        frame.close(c);
        Ok(())
    });
}

/// `cFEStateSelectPersona` constructor (0x1e08d8): base construction at
/// `0x1a8500` with `a1 = a2 = 0`, then the vtable `0x470000 - 0x70b0` at `+8`.
pub fn cfe_state_select_persona_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        fe_ctor_with_vtable(
            c,
            15,
            true,
            true,
            Target::Table(0x001a_8500),
            0x001e_08f4,
            0x70b0,
        )
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::fakes::machine_rt;
    use crate::difftest::DATA;

    /// Runs the generated body and the port from the same machine state, with
    /// `this` in the data region (the allocator and BELibrary are real).
    fn oc953_case(generated: GuestFn, hand: GuestFn, entry: u32) {
        check(generated, hand, 40, &|c, seed| {
            machine_rt(c, seed, entry as u64, 0);
            c.g[A0][0] = (DATA + 0x2000) as u64;
            c.pc = entry;
        });
    }

    #[test]
    fn cfe_state_request_line_ctor_2_matches() {
        oc953_case(
            ee::s04::f_001e0578__generated,
            cfe_state_request_line_ctor_2,
            0x001e_0578,
        );
    }

    #[test]
    fn cfe_state_request_line_dtor_matches() {
        oc953_case(
            ee::s04::f_001e05b0__generated,
            cfe_state_request_line_dtor,
            0x001e_05b0,
        );
    }

    #[test]
    fn fe_screen_sel_multi_ctor_2_matches() {
        oc953_case(
            ee::s04::f_001e07c0__generated,
            fe_screen_sel_multi_ctor_2,
            0x001e_07c0,
        );
    }

    #[test]
    fn fe_screen_sel_multi_dtor_matches() {
        oc953_case(
            ee::s04::f_001e07f8__generated,
            fe_screen_sel_multi_dtor,
            0x001e_07f8,
        );
    }

    #[test]
    fn cfe_state_mp_circuit_ctor_2_matches() {
        oc953_case(
            ee::s04::f_001e0820__generated,
            cfe_state_mp_circuit_ctor_2,
            0x001e_0820,
        );
    }

    #[test]
    fn cfe_state_mp_circuit_dtor_matches() {
        oc953_case(
            ee::s04::f_001e0858__generated,
            cfe_state_mp_circuit_dtor,
            0x001e_0858,
        );
    }

    #[test]
    fn cfe_use_existing_account_ctor_2_matches() {
        oc953_case(
            ee::s04::f_001e0880__generated,
            cfe_use_existing_account_ctor_2,
            0x001e_0880,
        );
    }

    #[test]
    fn cfe_state_select_persona_ctor_2_matches() {
        oc953_case(
            ee::s04::f_001e08d8__generated,
            cfe_state_select_persona_ctor_2,
            0x001e_08d8,
        );
    }
}
