//! Staging module oc1383 (wave memcard-ui-1): the remaining card-state
//! vtable slots and the card base's tiny slots. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1383`.

#![allow(unused_imports)]

use crate::abi::ret;
use crate::call::{call_ptr, guest, Callee, Flow, Frame};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (0x001d_dde0, mem_card_state_base_v13, "MemCardStateBase_v13"),
    (
        0x001d_dde8,
        mem_card_state_base_on_widget_action,
        "MemCardStateBase_OnWidgetAction",
    ),
    (0x001d_ddf0, mem_card_state_base_v26, "MemCardStateBase_v26"),
    (0x001d_ddf8, mem_card_state_base_v33, "MemCardStateBase_v33"),
    (0x001d_de00, mem_card_state_base_v37, "MemCardStateBase_v37"),
    (0x001d_de08, mem_card_state_base_v38, "MemCardStateBase_v38"),
    (0x001d_de10, mem_card_state_base_v39, "MemCardStateBase_v39"),
    (0x002d_37b8, mem_card_base_v26, "MemCardBase_v26"),
    (0x002d_37c0, mem_card_base_v27, "MemCardBase_v27"),
    (0x002d_37c8, mem_card_base_v28, "MemCardBase_v28"),
    (0x002d_37d0, mem_card_base_v74, "MemCardBase_v74"),
];

pub(crate) mod callee {
    use super::ee;
    use super::Callee;
}

/// `void MemCardStateBase_v13(void)` (0x1ddde0): unimplemented state slot.
pub fn mem_card_state_base_v13(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `void MemCardStateBase_OnWidgetAction(void)` (0x1ddde8): unimplemented
/// state slot.
pub fn mem_card_state_base_on_widget_action(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `void MemCardStateBase_v26(void)` (0x1dddf0): unimplemented state slot.
pub fn mem_card_state_base_v26(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `void MemCardStateBase_v33(void)` (0x1dddf8): unimplemented state slot.
pub fn mem_card_state_base_v33(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `void MemCardStateBase_v37(void)` (0x1dde00): unimplemented state slot.
pub fn mem_card_state_base_v37(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `void MemCardStateBase_v38(void)` (0x1dde08): unimplemented state slot.
pub fn mem_card_state_base_v38(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `void MemCardStateBase_v39(void)` (0x1dde10): unimplemented state slot.
pub fn mem_card_state_base_v39(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `int MemCardBase_v26(void)` (0x2d37b8): the card base's always-true slot.
pub fn mem_card_base_v26(c: &mut Cpu) {
    c.cycles += 2;
    let ret = c.g[RA][0] as u32;
    c.g[V0][0] = 0x1;
    c.pc = ret;
}

/// `void MemCardBase_v27(void)` (0x2d37c0): unimplemented card-base slot.
pub fn mem_card_base_v27(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `void MemCardBase_v28(void)` (0x2d37c8): unimplemented card-base slot.
pub fn mem_card_base_v28(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `int MemCardBase_v74(void)` (0x2d37d0): the card base's always-false slot.
pub fn mem_card_base_v74(c: &mut Cpu) {
    c.cycles += 2;
    let ret = c.g[RA][0] as u32;
    c.g[V0][0] = 0;
    c.pc = ret;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::check;
    use crate::difftest::fakes::machine;

    #[test]
    fn mem_card_state_base_v13_matches() {
        check(
            ee::s04::f_001ddde0__generated,
            mem_card_state_base_v13,
            16,
            &|c, seed| {
                let _ = machine(c, seed, 0x1d_dde0, 0);
            },
        );
    }

    #[test]
    fn mem_card_state_base_on_widget_action_matches() {
        check(
            ee::s04::f_001ddde8__generated,
            mem_card_state_base_on_widget_action,
            16,
            &|c, seed| {
                let _ = machine(c, seed, 0x1d_dde8, 0);
            },
        );
    }

    #[test]
    fn mem_card_state_base_v26_matches() {
        check(
            ee::s04::f_001dddf0__generated,
            mem_card_state_base_v26,
            16,
            &|c, seed| {
                let _ = machine(c, seed, 0x1d_ddf0, 0);
            },
        );
    }

    #[test]
    fn mem_card_state_base_v33_matches() {
        check(
            ee::s04::f_001dddf8__generated,
            mem_card_state_base_v33,
            16,
            &|c, seed| {
                let _ = machine(c, seed, 0x1d_ddf8, 0);
            },
        );
    }

    #[test]
    fn mem_card_state_base_v37_matches() {
        check(
            ee::s04::f_001dde00__generated,
            mem_card_state_base_v37,
            16,
            &|c, seed| {
                let _ = machine(c, seed, 0x1d_de00, 0);
            },
        );
    }

    #[test]
    fn mem_card_state_base_v38_matches() {
        check(
            ee::s04::f_001dde08__generated,
            mem_card_state_base_v38,
            16,
            &|c, seed| {
                let _ = machine(c, seed, 0x1d_de08, 0);
            },
        );
    }

    #[test]
    fn mem_card_state_base_v39_matches() {
        check(
            ee::s04::f_001dde10__generated,
            mem_card_state_base_v39,
            16,
            &|c, seed| {
                let _ = machine(c, seed, 0x1d_de10, 0);
            },
        );
    }

    #[test]
    fn mem_card_base_v26_matches() {
        check(ee::s09::f_002d37b8__generated, mem_card_base_v26, 16, &|c, seed| {
            let _ = machine(c, seed, 0x2d_37b8, 0);
        });
    }

    #[test]
    fn mem_card_base_v27_matches() {
        check(ee::s09::f_002d37c0__generated, mem_card_base_v27, 16, &|c, seed| {
            let _ = machine(c, seed, 0x2d_37c0, 0);
        });
    }

    #[test]
    fn mem_card_base_v28_matches() {
        check(ee::s09::f_002d37c8__generated, mem_card_base_v28, 16, &|c, seed| {
            let _ = machine(c, seed, 0x2d_37c8, 0);
        });
    }

    #[test]
    fn mem_card_base_v74_matches() {
        check(ee::s09::f_002d37d0__generated, mem_card_base_v74, 16, &|c, seed| {
            let _ = machine(c, seed, 0x2d_37d0, 0);
        });
    }
}
