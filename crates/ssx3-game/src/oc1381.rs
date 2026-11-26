//! Staging module oc1381 (wave memcard-ui-1): the card state's flag takers
//! and field getters, plus the save-start hook. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1381`.

#![allow(unused_imports)]

use crate::abi::ret;
use crate::call::{call_ptr, guest, Callee, Flow, Frame};
use crate::game::fe::fememcardcallbacks::{memcard_fe, MEMCARD_FE_RETRY};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::reg::*;
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x001d_dd58,
        mem_card_ui_take_flag_1e4,
        "MemCardUI_TakeFlag1e4",
    ),
    (
        0x001d_dd88,
        mem_card_ui_take_flag_1f8,
        "MemCardUI_TakeFlag1f8",
    ),
    (
        0x001d_dd98,
        mem_card_ui_take_flag_1fc,
        "MemCardUI_TakeFlag1fc",
    ),
    (
        0x001d_ddb8,
        mem_card_ui_take_flag_1ec,
        "MemCardUI_TakeFlag1ec",
    ),
    (
        0x001d_dd28,
        mem_card_ui_get_field_1dc,
        "MemCardUI_GetField1dc",
    ),
    (
        0x001d_dd38,
        mem_card_ui_get_field_1f4,
        "MemCardUI_GetField1f4",
    ),
    (
        0x001d_dd48,
        mem_card_ui_get_field_1e0,
        "MemCardUI_GetField1e0",
    ),
    (
        0x001d_dd68,
        mem_card_ui_get_field_1f0,
        "MemCardUI_GetField1f0",
    ),
    (
        0x001d_dd78,
        mem_card_ui_get_field_1a8,
        "MemCardUI_GetField1a8",
    ),
    (
        0x001d_dda8,
        mem_card_ui_get_field_1e8,
        "MemCardUI_GetField1e8",
    ),
    (
        0x001d_5f38,
        mem_card_ui_on_save_start,
        "MemCardUI_OnSaveStart",
    ),
];

pub(crate) mod callee {
    use super::ee;
    use super::Callee;
}

/// The one-shot flags `MemCardUI_TakeFlag*` take (return, then clear) on the
/// card state.
pub(crate) const MEMCARD_UI_FLAG_1E4: u32 = 0x1e4;
pub(crate) const MEMCARD_UI_FLAG_1F8: u32 = 0x1f8;
pub(crate) const MEMCARD_UI_FLAG_1FC: u32 = 0x1fc;

/// `int MemCardUI_TakeFlag1e4(void)` (0x1ddd58): return the card state's
/// `+0x1e4` flag, clearing it.
pub fn mem_card_ui_take_flag_1e4(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V1][0] = memcard_fe(c) as u64;
    c.g[V0][0] = Addr(c.g[V1][0] as u32).off(MEMCARD_UI_FLAG_1E4).lw(c);
    Addr(c.g[V1][0] as u32).off(MEMCARD_UI_FLAG_1E4).sw(c, 0);
    ret(c);
}

/// `int MemCardUI_TakeFlag1f8(void)` (0x1ddd88): return the card state's
/// `+0x1f8` flag, clearing it.
pub fn mem_card_ui_take_flag_1f8(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V1][0] = memcard_fe(c) as u64;
    c.g[V0][0] = Addr(c.g[V1][0] as u32).off(MEMCARD_UI_FLAG_1F8).lw(c);
    Addr(c.g[V1][0] as u32).off(MEMCARD_UI_FLAG_1F8).sw(c, 0);
    ret(c);
}

/// `int MemCardUI_TakeFlag1fc(void)` (0x1ddd98): return the card state's
/// `+0x1fc` flag, clearing it.
pub fn mem_card_ui_take_flag_1fc(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V1][0] = memcard_fe(c) as u64;
    c.g[V0][0] = Addr(c.g[V1][0] as u32).off(MEMCARD_UI_FLAG_1FC).lw(c);
    Addr(c.g[V1][0] as u32).off(MEMCARD_UI_FLAG_1FC).sw(c, 0);
    ret(c);
}

/// `int MemCardUI_TakeFlag1ec(void)` (0x1dddb8): return the card state's
/// retry flag (`+0x1ec`), clearing it.
pub fn mem_card_ui_take_flag_1ec(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V1][0] = memcard_fe(c) as u64;
    c.g[V0][0] = Addr(c.g[V1][0] as u32).off(MEMCARD_FE_RETRY).lw(c);
    Addr(c.g[V1][0] as u32).off(MEMCARD_FE_RETRY).sw(c, 0);
    ret(c);
}

/// `int MemCardUI_GetField1dc(void)` (0x1ddd28): the card state's has-save
/// word (`+0x1dc`).
pub fn mem_card_ui_get_field_1dc(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V1][0] = memcard_fe(c) as u64;
    c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x1dc).lw(c);
    ret(c);
}

/// `int MemCardUI_GetField1f4(void)` (0x1ddd38): the card state's
/// use-save-title flag (`+0x1f4`).
pub fn mem_card_ui_get_field_1f4(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V1][0] = memcard_fe(c) as u64;
    c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x1f4).lw(c);
    ret(c);
}

/// `int MemCardUI_GetField1e0(void)` (0x1ddd48): the card state's
/// delete-pending word (`+0x1e0`).
pub fn mem_card_ui_get_field_1e0(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V1][0] = memcard_fe(c) as u64;
    c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x1e0).lw(c);
    ret(c);
}

/// `int MemCardUI_GetField1f0(void)` (0x1ddd68): the card state's
/// from-load word (`+0x1f0`).
pub fn mem_card_ui_get_field_1f0(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V1][0] = memcard_fe(c) as u64;
    c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x1f0).lw(c);
    ret(c);
}

/// `int MemCardUI_GetField1a8(void)` (0x1ddd78): the card state's busy word
/// (`+0x1a8`).
pub fn mem_card_ui_get_field_1a8(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V1][0] = memcard_fe(c) as u64;
    c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x1a8).lw(c);
    ret(c);
}

/// `int MemCardUI_GetField1e8(void)` (0x1ddda8): the card state's
/// format-pending word (`+0x1e8`).
pub fn mem_card_ui_get_field_1e8(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V1][0] = memcard_fe(c) as u64;
    c.g[V0][0] = Addr(c.g[V1][0] as u32).off(0x1e8).lw(c);
    ret(c);
}

/// `void MemCardUI_OnSaveStart(void)` (0x1d5f38): the save-start hook (a
/// no-op in this build).
pub fn mem_card_ui_on_save_start(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::machine;
    use crate::difftest::{check, fill, Rng, DATA, GAME_GP};

    /// The card state the takers and getters read, with nonzero words.
    pub(crate) const OC1381_FE: u32 = DATA + 0x100;

    fn oc1381_setup(c: &mut Cpu, seed: u32) -> Rng {
        let mut r = machine(c, seed, 0x1d_dd58, 0);
        fill(c, &mut r, OC1381_FE, 0x100);
        c.bus.write32(GAME_GP.wrapping_sub(0x10c8), OC1381_FE);
        r
    }

    #[test]
    fn mem_card_ui_take_flag_1e4_matches() {
        check(
            ee::s04::f_001ddd58__generated,
            mem_card_ui_take_flag_1e4,
            16,
            &|c, seed| {
                let _ = oc1381_setup(c, seed);
                // The returned flag and its clearing are both compared.
            },
        );
    }

    #[test]
    fn mem_card_ui_take_flag_1f8_matches() {
        check(
            ee::s04::f_001ddd88__generated,
            mem_card_ui_take_flag_1f8,
            16,
            &|c, seed| {
                let _ = oc1381_setup(c, seed);
            },
        );
    }

    #[test]
    fn mem_card_ui_take_flag_1fc_matches() {
        check(
            ee::s04::f_001ddd98__generated,
            mem_card_ui_take_flag_1fc,
            16,
            &|c, seed| {
                let _ = oc1381_setup(c, seed);
            },
        );
    }

    #[test]
    fn mem_card_ui_take_flag_1ec_matches() {
        check(
            ee::s04::f_001dddb8__generated,
            mem_card_ui_take_flag_1ec,
            16,
            &|c, seed| {
                let _ = oc1381_setup(c, seed);
            },
        );
    }

    #[test]
    fn mem_card_ui_get_field_1dc_matches() {
        check(
            ee::s04::f_001ddd28__generated,
            mem_card_ui_get_field_1dc,
            16,
            &|c, seed| {
                let _ = oc1381_setup(c, seed);
            },
        );
    }

    #[test]
    fn mem_card_ui_get_field_1f4_matches() {
        check(
            ee::s04::f_001ddd38__generated,
            mem_card_ui_get_field_1f4,
            16,
            &|c, seed| {
                let _ = oc1381_setup(c, seed);
            },
        );
    }

    #[test]
    fn mem_card_ui_get_field_1e0_matches() {
        check(
            ee::s04::f_001ddd48__generated,
            mem_card_ui_get_field_1e0,
            16,
            &|c, seed| {
                let _ = oc1381_setup(c, seed);
            },
        );
    }

    #[test]
    fn mem_card_ui_get_field_1f0_matches() {
        check(
            ee::s04::f_001ddd68__generated,
            mem_card_ui_get_field_1f0,
            16,
            &|c, seed| {
                let _ = oc1381_setup(c, seed);
            },
        );
    }

    #[test]
    fn mem_card_ui_get_field_1a8_matches() {
        check(
            ee::s04::f_001ddd78__generated,
            mem_card_ui_get_field_1a8,
            16,
            &|c, seed| {
                let _ = oc1381_setup(c, seed);
            },
        );
    }

    #[test]
    fn mem_card_ui_get_field_1e8_matches() {
        check(
            ee::s04::f_001ddda8__generated,
            mem_card_ui_get_field_1e8,
            16,
            &|c, seed| {
                let _ = oc1381_setup(c, seed);
            },
        );
    }

    #[test]
    fn mem_card_ui_on_save_start_matches() {
        check(
            ee::s04::f_001d5f38__generated,
            mem_card_ui_on_save_start,
            16,
            &|c, seed| {
                let _ = machine(c, seed, 0x1d_5f38, 0);
            },
        );
    }
}
