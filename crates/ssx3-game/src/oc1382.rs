//! Staging module oc1382 (wave memcard-ui-1): the card state's finish and
//! restart setters plus its unimplemented vtable slots. Moved into the
//! source-tree units by `tools/reorg_modules.py apply oc1382`.

#![allow(unused_imports)]

use crate::abi::ret;
use crate::call::{call_ptr, guest, Callee, Flow, Frame};
use crate::game::fe::fememcardcallbacks::MEMCARD_FE_STATE;
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x001d_86a0,
        mem_card_state_base_finish,
        "MemCardStateBase_Finish",
    ),
    (
        0x001d_93b0,
        mem_card_state_base_restart,
        "MemCardStateBase_Restart",
    ),
    (0x001d_93a8, mem_card_state_base_v27, "MemCardStateBase_v27"),
    (0x001d_93c0, mem_card_state_base_v28, "MemCardStateBase_v28"),
    (0x001d_93c8, mem_card_state_base_v30, "MemCardStateBase_v30"),
    (0x001d_93d0, mem_card_state_base_v29, "MemCardStateBase_v29"),
    (
        0x001d_9878,
        mem_card_state_base_on_deactivate,
        "MemCardStateBase_OnDeactivate",
    ),
    (
        0x001d_ddd8,
        mem_card_state_base_init,
        "MemCardStateBase_Init",
    ),
];

pub(crate) mod callee {
    use super::ee;
    use super::Callee;
}

/// `void MemCardStateBase_Finish(void *self)` (0x1d86a0): mark the card
/// state finished (state 6).
pub fn mem_card_state_base_finish(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = 0x6;
    Addr(c.g[A0][0] as u32)
        .off(MEMCARD_FE_STATE)
        .sw(c, c.g[V0][0]);
    ret(c);
}

/// `void MemCardStateBase_Restart(void *self)` (0x1d93b0): restart the card
/// state (state 1).
pub fn mem_card_state_base_restart(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = 0x1;
    Addr(c.g[A0][0] as u32)
        .off(MEMCARD_FE_STATE)
        .sw(c, c.g[V0][0]);
    ret(c);
}

/// `void MemCardStateBase_v27(void)` (0x1d93a8): unimplemented state slot.
pub fn mem_card_state_base_v27(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `void MemCardStateBase_v28(void)` (0x1d93c0): unimplemented state slot.
pub fn mem_card_state_base_v28(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `void MemCardStateBase_v30(void)` (0x1d93c8): unimplemented state slot.
pub fn mem_card_state_base_v30(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `void MemCardStateBase_v29(void)` (0x1d93d0): unimplemented state slot.
pub fn mem_card_state_base_v29(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `void MemCardStateBase_OnDeactivate(void)` (0x1d9878): unimplemented
/// state slot.
pub fn mem_card_state_base_on_deactivate(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `void MemCardStateBase_Init(void)` (0x1dddd8): unimplemented state slot.
pub fn mem_card_state_base_init(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::machine;
    use crate::difftest::{check, fill, Rng, DATA};

    /// The card state the setters store into, with nonzero words.
    pub(crate) const OC1382_THIS: u32 = DATA + 0x100;

    fn oc1382_setup(c: &mut Cpu, seed: u32) -> Rng {
        let mut r = machine(c, seed, 0x1d_86a0, 0);
        fill(c, &mut r, OC1382_THIS, 0x100);
        r
    }

    #[test]
    fn mem_card_state_base_finish_matches() {
        check(
            ee::s04::f_001d86a0__generated,
            mem_card_state_base_finish,
            16,
            &|c, seed| {
                let _ = oc1382_setup(c, seed);
                c.g[A0][0] = OC1382_THIS as u64;
            },
        );
    }

    #[test]
    fn mem_card_state_base_restart_matches() {
        check(
            ee::s04::f_001d93b0__generated,
            mem_card_state_base_restart,
            16,
            &|c, seed| {
                let _ = oc1382_setup(c, seed);
                c.g[A0][0] = OC1382_THIS as u64;
            },
        );
    }

    #[test]
    fn mem_card_state_base_v27_matches() {
        check(
            ee::s04::f_001d93a8__generated,
            mem_card_state_base_v27,
            16,
            &|c, seed| {
                let _ = machine(c, seed, 0x1d_93a8, 0);
            },
        );
    }

    #[test]
    fn mem_card_state_base_v28_matches() {
        check(
            ee::s04::f_001d93c0__generated,
            mem_card_state_base_v28,
            16,
            &|c, seed| {
                let _ = machine(c, seed, 0x1d_93c0, 0);
            },
        );
    }

    #[test]
    fn mem_card_state_base_v30_matches() {
        check(
            ee::s04::f_001d93c8__generated,
            mem_card_state_base_v30,
            16,
            &|c, seed| {
                let _ = machine(c, seed, 0x1d_93c8, 0);
            },
        );
    }

    #[test]
    fn mem_card_state_base_v29_matches() {
        check(
            ee::s04::f_001d93d0__generated,
            mem_card_state_base_v29,
            16,
            &|c, seed| {
                let _ = machine(c, seed, 0x1d_93d0, 0);
            },
        );
    }

    #[test]
    fn mem_card_state_base_on_deactivate_matches() {
        check(
            ee::s04::f_001d9878__generated,
            mem_card_state_base_on_deactivate,
            16,
            &|c, seed| {
                let _ = machine(c, seed, 0x1d_9878, 0);
            },
        );
    }

    #[test]
    fn mem_card_state_base_init_matches() {
        check(
            ee::s04::f_001dddd8__generated,
            mem_card_state_base_init,
            16,
            &|c, seed| {
                let _ = machine(c, seed, 0x1d_ddd8, 0);
            },
        );
    }
}
