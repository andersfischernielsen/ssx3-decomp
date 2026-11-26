//! Staging module oc815 (wave wave248): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc815`.

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
/// `SPCH_ResetBankTable`: Clears the bank table pointer and size globals (gp+0x1734, gp+0x1738).
pub fn oc815_sp_ch_reset_bank_table(c: &mut Cpu) {
    c.cycles += 3;
    c.bus.write32((c.g[GP][0] as u32).wrapping_add(0x1734), 0);
    c.bus.write32((c.g[GP][0] as u32).wrapping_add(0x1738), 0);
    ret(c);
}

/// `SPCH_GetConfig`: Returns the constant config address 0x44ffa8 in v0.
pub fn oc815_sp_ch_get_config(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = sx(0x44_ffa8);
    ret(c);
}

/// `SPCH_SetTimeOffset`: Stores a0 in the time offset global (gp+0x174c).
pub fn oc815_sp_ch_set_time_offset(c: &mut Cpu) {
    c.cycles += 2;
    let a0 = c.g[A0][0];
    c.bus
        .write32((c.g[GP][0] as u32).wrapping_add(0x174c), a0 as u32);
    ret(c);
}

/// `SPCH_CalcBankTableSize`: Returns the byte size of a bank table of a0 entries (a0 * 4) in v0.
pub fn oc815_sp_ch_calc_bank_table_size(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32) << 2);
    ret(c);
}

/// `UIScreen_ThreadNopC`: Empty thread command: returns at once.
pub fn oc815_ui_screen_thread_nop_c(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `AttachedEmitterTypeData_FreePart2`: Empty free hook: returns at once.
pub fn oc815_attached_emitter_type_data_free_part2(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[
    (
        0x003d6d40,
        oc815_sp_ch_reset_bank_table,
        "SPCH_ResetBankTable",
    ),
    (0x003d6768, oc815_sp_ch_get_config, "SPCH_GetConfig"),
    (
        0x003da3c0,
        oc815_sp_ch_set_time_offset,
        "SPCH_SetTimeOffset",
    ),
    (
        0x003d6c58,
        oc815_sp_ch_calc_bank_table_size,
        "SPCH_CalcBankTableSize",
    ),
    (
        0x0039ce90,
        oc815_ui_screen_thread_nop_c,
        "UIScreen_ThreadNopC",
    ),
    (
        0x002fa5f8,
        oc815_attached_emitter_type_data_free_part2,
        "AttachedEmitterTypeData_FreePart2",
    ),
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::machine;

    /// `SPCH_ResetBankTable` against the generated body.
    #[test]
    fn oc815_sp_ch_reset_bank_table_matches() {
        crate::difftest::check(
            ee::s14::f_003d6d40__generated,
            oc815_sp_ch_reset_bank_table,
            40,
            &|c, seed| {
                let _ = machine(c, seed, 0x815, 0);
                c.pc = 0x003d6d40;
            },
        );
    }

    /// `SPCH_GetConfig` against the generated body.
    #[test]
    fn oc815_sp_ch_get_config_matches() {
        crate::difftest::check(
            ee::s14::f_003d6768__generated,
            oc815_sp_ch_get_config,
            40,
            &|c, seed| {
                let _ = machine(c, seed, 0x815, 0);
                c.pc = 0x003d6768;
            },
        );
    }

    /// `SPCH_SetTimeOffset` against the generated body.
    #[test]
    fn oc815_sp_ch_set_time_offset_matches() {
        crate::difftest::check(
            ee::s14::f_003da3c0__generated,
            oc815_sp_ch_set_time_offset,
            40,
            &|c, seed| {
                let _ = machine(c, seed, 0x815, 0);
                c.pc = 0x003da3c0;
            },
        );
    }

    /// `SPCH_CalcBankTableSize` against the generated body.
    #[test]
    fn oc815_sp_ch_calc_bank_table_size_matches() {
        crate::difftest::check(
            ee::s14::f_003d6c58__generated,
            oc815_sp_ch_calc_bank_table_size,
            40,
            &|c, seed| {
                let _ = machine(c, seed, 0x815, 0);
                c.pc = 0x003d6c58;
            },
        );
    }

    /// `UIScreen_ThreadNopC` against the generated body.
    #[test]
    fn oc815_ui_screen_thread_nop_c_matches() {
        crate::difftest::check(
            ee::s13::f_0039ce90__generated,
            oc815_ui_screen_thread_nop_c,
            40,
            &|c, seed| {
                let _ = machine(c, seed, 0x815, 0);
                c.pc = 0x0039ce90;
            },
        );
    }

    /// `AttachedEmitterTypeData_FreePart2` against the generated body.
    #[test]
    fn oc815_attached_emitter_type_data_free_part2_matches() {
        crate::difftest::check(
            ee::s10::f_002fa5f8__generated,
            oc815_attached_emitter_type_data_free_part2,
            40,
            &|c, seed| {
                let _ = machine(c, seed, 0x815, 0);
                c.pc = 0x002fa5f8;
            },
        );
    }
}
