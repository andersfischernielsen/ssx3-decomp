//! Staging module oc822 (wave wave248): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc822`.

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
        0x0017ff80,
        feui_input_is_option_held,
        "FEUIInput_IsOptionHeld",
    ),
    (0x0017ffa0, feui_input_is_misc_held, "FEUIInput_IsMiscHeld"),
    (
        0x0017ffc0,
        feui_input_is_next_released,
        "FEUIInput_IsNextReleased",
    ),
    (
        0x0017ffe0,
        feui_input_is_previous_released,
        "FEUIInput_IsPreviousReleased",
    ),
    (
        0x00180000,
        feui_input_is_option_released,
        "FEUIInput_IsOptionReleased",
    ),
    (
        0x00180020,
        feui_input_is_misc_released,
        "FEUIInput_IsMiscReleased",
    ),
];

/// Shared body of the `FEUIInput_Is*` button queries: `InputBinding_EvalBool(
/// *(input + 0x18), button)` (0x320c48); `ret` is the call's return address.
pub(crate) fn oc822_feui_eval(c: &mut Cpu, button: u64, ret: u32) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = button;
        frame.save_ra(c, 0);
        c.g[A0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x18));
        call_ptr(c, 0x0032_0c48, ret)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `FEUIInput_IsOptionHeld(input)` (0x017ff80): asks the pad object at `*(input + 0x18)` whether
/// button 0x99 (option (held)) is pressed and returns its answer.
pub fn feui_input_is_option_held(c: &mut Cpu) {
    oc822_feui_eval(c, 0x99, 0x0017ff94);
}

/// `FEUIInput_IsMiscHeld(input)` (0x017ffa0): asks the pad object at `*(input + 0x18)` whether
/// button 0x9a (misc (held)) is pressed and returns its answer.
pub fn feui_input_is_misc_held(c: &mut Cpu) {
    oc822_feui_eval(c, 0x9a, 0x0017ffb4);
}

/// `FEUIInput_IsNextReleased(input)` (0x017ffc0): asks the pad object at `*(input + 0x18)` whether
/// button 0x9b (next (released)) is pressed and returns its answer.
pub fn feui_input_is_next_released(c: &mut Cpu) {
    oc822_feui_eval(c, 0x9b, 0x0017ffd4);
}

/// `FEUIInput_IsPreviousReleased(input)` (0x017ffe0): asks the pad object at `*(input + 0x18)` whether
/// button 0x9c (previous (released)) is pressed and returns its answer.
pub fn feui_input_is_previous_released(c: &mut Cpu) {
    oc822_feui_eval(c, 0x9c, 0x0017fff4);
}

/// `FEUIInput_IsOptionReleased(input)` (0x0180000): asks the pad object at `*(input + 0x18)` whether
/// button 0x9d (option (released)) is pressed and returns its answer.
pub fn feui_input_is_option_released(c: &mut Cpu) {
    oc822_feui_eval(c, 0x9d, 0x00180014);
}

/// `FEUIInput_IsMiscReleased(input)` (0x0180020): asks the pad object at `*(input + 0x18)` whether
/// button 0x9e (misc (released)) is pressed and returns its answer.
pub fn feui_input_is_misc_released(c: &mut Cpu) {
    oc822_feui_eval(c, 0x9e, 0x00180034);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes;
    use crate::difftest::fakes::machine;
    use crate::difftest::DATA;

    /// `FEUIInput_IsOptionHeld`: the button query and its unwinds.
    #[test]
    fn feui_input_is_option_held_matches() {
        fakes::covered(
            ee::s02::f_0017ff80__generated,
            feui_input_is_option_held,
            60,
            &|c, seed| {
                let _r = machine(c, seed, 0x17ff80, 1);
                c.pc = 0x0017ff80;
                c.g[A0][0] = sx(DATA + 0x1000);
                c.bus.write32(DATA + 0x1000 + 0x18, DATA + 0x2000);
            },
        );
    }

    /// `FEUIInput_IsMiscHeld`: the button query and its unwinds.
    #[test]
    fn feui_input_is_misc_held_matches() {
        fakes::covered(
            ee::s02::f_0017ffa0__generated,
            feui_input_is_misc_held,
            60,
            &|c, seed| {
                let _r = machine(c, seed, 0x17ffa0, 1);
                c.pc = 0x0017ffa0;
                c.g[A0][0] = sx(DATA + 0x1000);
                c.bus.write32(DATA + 0x1000 + 0x18, DATA + 0x2000);
            },
        );
    }

    /// `FEUIInput_IsNextReleased`: the button query and its unwinds.
    #[test]
    fn feui_input_is_next_released_matches() {
        fakes::covered(
            ee::s02::f_0017ffc0__generated,
            feui_input_is_next_released,
            60,
            &|c, seed| {
                let _r = machine(c, seed, 0x17ffc0, 1);
                c.pc = 0x0017ffc0;
                c.g[A0][0] = sx(DATA + 0x1000);
                c.bus.write32(DATA + 0x1000 + 0x18, DATA + 0x2000);
            },
        );
    }

    /// `FEUIInput_IsPreviousReleased`: the button query and its unwinds.
    #[test]
    fn feui_input_is_previous_released_matches() {
        fakes::covered(
            ee::s02::f_0017ffe0__generated,
            feui_input_is_previous_released,
            60,
            &|c, seed| {
                let _r = machine(c, seed, 0x17ffe0, 1);
                c.pc = 0x0017ffe0;
                c.g[A0][0] = sx(DATA + 0x1000);
                c.bus.write32(DATA + 0x1000 + 0x18, DATA + 0x2000);
            },
        );
    }

    /// `FEUIInput_IsOptionReleased`: the button query and its unwinds.
    #[test]
    fn feui_input_is_option_released_matches() {
        fakes::covered(
            ee::s02::f_00180000__generated,
            feui_input_is_option_released,
            60,
            &|c, seed| {
                let _r = machine(c, seed, 0x180000, 1);
                c.pc = 0x00180000;
                c.g[A0][0] = sx(DATA + 0x1000);
                c.bus.write32(DATA + 0x1000 + 0x18, DATA + 0x2000);
            },
        );
    }

    /// `FEUIInput_IsMiscReleased`: the button query and its unwinds.
    #[test]
    fn feui_input_is_misc_released_matches() {
        fakes::covered(
            ee::s02::f_00180020__generated,
            feui_input_is_misc_released,
            60,
            &|c, seed| {
                let _r = machine(c, seed, 0x180020, 1);
                c.pc = 0x00180020;
                c.g[A0][0] = sx(DATA + 0x1000);
                c.bus.write32(DATA + 0x1000 + 0x18, DATA + 0x2000);
            },
        );
    }
}
