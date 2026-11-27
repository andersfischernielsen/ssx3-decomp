//! Staging module oc954 (wave wave256): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc954`.

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
        0x001e_0918,
        cfe_state_select_persona_dtor,
        "cFEStateSelectPersona_dtor",
    ),
    (
        0x001e_0950,
        cfe_state_challenge_popup_ctor_2,
        "cFEStateChallengePopup_ctor_2",
    ),
];

/// `cFEStateSelectPersona::~cFEStateSelectPersona(this)` (0x1e0918): set the
/// object's vtable word at `+0x8` to the select-persona vtable (0x468f50),
/// then chain to the base destructor `NetFEState_dtor` (0x1a85d0).
pub fn cfe_state_select_persona_dtor(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 10;
        let frame = Frame::open(c, 0x10);
        // 0x470000 - 0x70b0 = D_00468F50 (the vtable).
        c.g[V0][0] = 0x47_0000 - 0x70b0;
        frame.save_ra(c, 0);
        c.g[V1][0] = c.g[A0][0];
        let vtable = c.g[V0][0];
        Addr((c.g[V1][0] as u32).wrapping_add(0x8)).sw(c, vtable);
        call_ptr(c, 0x001a_85d0, 0x001e_0934)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

/// `cFEStateChallengePopup::cFEStateChallengePopup(this)`, the second
/// constructor (0x1e0950): build the base with `NetFEState_ctor(this, 0, 0)`
/// (0x1a8500), then set the vtable at `+0x8` (D_00468E38) and the two words at
/// `+0x6d8` (D_00441268, the popup's text) and `+0x6d0` (0).
pub fn cfe_state_challenge_popup_ctor_2(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 19;
        let frame = Frame::open(c, 0x20);
        c.g[A1][0] = 0;
        frame.save(c, S0, 0x10);
        c.g[A2][0] = 0;
        frame.save_ra(c, 0);
        c.g[S0][0] = c.g[A0][0];
        call_ptr(c, 0x001a_8500, 0x001e_096c)?;
        // The base constructor returns with `this` still in s0.
        // 0x470000 - 0x71c8 = D_00468E38, 0x440000 + 0x1268 = D_00441268.
        c.g[V1][0] = 0x47_0000 - 0x71c8;
        c.g[A0][0] = 0x44_0000 + 0x1268;
        let vtable = c.g[V1][0];
        Addr((c.g[S0][0] as u32).wrapping_add(0x8)).sw(c, vtable);
        c.g[V0][0] = c.g[S0][0];
        let text = c.g[A0][0];
        Addr((c.g[S0][0] as u32).wrapping_add(0x6d8)).sw(c, text);
        Addr((c.g[S0][0] as u32).wrapping_add(0x6d0)).sw(c, 0);
        frame.restore(c, S0, 0x10);
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, fakes, DATA, GAME_GP};

    /// The net FE state object both bodies write (`this`).
    const POPUP_THIS: u32 = DATA + 0x1_0000;

    /// The globals `NetFEState` reads through `gp`: the BELibrary dummy and
    /// the world with its vtable-backed fields.
    pub(crate) fn cfe_popup_globals(c: &mut Cpu) {
        c.bus.write32(GAME_GP.wrapping_sub(0x1f04), DATA + 0x6_c000);
        c.bus.write32(GAME_GP.wrapping_sub(0x848), DATA + 0x6_8000);
        c.bus.write32(DATA + 0x6_8000 + 0x8c, DATA + 0x6_9000);
        c.bus.write32(DATA + 0x6_9000 + 0x4, DATA + 0x6_9100);
        c.bus.write16(DATA + 0x6_9100 + 0x20, 0x10);
        c.bus
            .write32(DATA + 0x6_9100 + 0x24, crate::difftest::fakes::FAKE + 0x10);
    }

    /// The message list of the net FE state at `this`: an empty list (even
    /// seeds) or two nodes, each with a sub-object whose method is a fake.
    pub(crate) fn cfe_popup_net_state(c: &mut Cpu, seed: u32, this: u32) {
        let head = DATA + 0x2_0000;
        let n1 = DATA + 0x2_1000;
        let n2 = DATA + 0x2_1100;
        c.bus.write32(this + 0x6cc, head);
        c.bus.write32(this + 0x6c8 + 0x4, head);
        c.bus.write32(head, if seed % 2 == 0 { head } else { n1 });
        c.bus.write32(n1, if seed % 2 == 0 { head } else { n2 });
        c.bus.write32(n1 + 4, head);
        c.bus.write32(n1 + 8, DATA + 0x2_2000);
        c.bus.write32(n2, head);
        c.bus.write32(n2 + 4, n1);
        c.bus.write32(n2 + 8, 0);
        c.bus.write32(DATA + 0x2_2000 + 0x4, DATA + 0x2_2100);
        c.bus.write16(DATA + 0x2_2100 + 0x8, 0x10);
        c.bus
            .write32(DATA + 0x2_2100 + 0xc, crate::difftest::fakes::FAKE + 0x20);
    }

    #[test]
    fn cfe_state_select_persona_dtor_matches() {
        check(
            ssx3_ee::s04::f_001e0918__generated,
            cfe_state_select_persona_dtor,
            40,
            &|c, seed| {
                let _r = fakes::machine(c, seed, 0x0918, 3);
                cfe_popup_globals(c);
                c.pc = 0x001e_0918;
                c.g[A0][0] = sx(POPUP_THIS);
                c.g[A1][0] = (seed % 2) as u64;
                cfe_popup_net_state(c, seed, POPUP_THIS);
            },
        );
    }

    #[test]
    fn cfe_state_challenge_popup_ctor_2_matches() {
        check(
            ssx3_ee::s04::f_001e0950__generated,
            cfe_state_challenge_popup_ctor_2,
            40,
            &|c, seed| {
                let _r = fakes::machine(c, seed, 0x0950, 3);
                cfe_popup_globals(c);
                c.pc = 0x001e_0950;
                c.g[A0][0] = sx(POPUP_THIS);
                c.bus.write32(DATA + 0x2_0000 + 0x4, DATA + 0x3_0000);
                c.bus.write32(POPUP_THIS + 0x6c8 + 0x4, DATA + 0x3_0000);
            },
        );
    }
}
