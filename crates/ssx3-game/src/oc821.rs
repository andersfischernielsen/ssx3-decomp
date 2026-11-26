//! Staging module oc821 (wave wave248): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc821`.

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
    (0x0017fea0, feui_input_is_select, "FEUIInput_IsSelect"),
    (0x0017fee0, feui_input_is_previous, "FEUIInput_IsPrevious"),
    (0x0017ff00, feui_input_is_option, "FEUIInput_IsOption"),
    (0x0017ff20, feui_input_is_misc, "FEUIInput_IsMisc"),
    (0x0017ff40, feui_input_is_next_held, "FEUIInput_IsNextHeld"),
    (
        0x0017ff60,
        feui_input_is_previous_held,
        "FEUIInput_IsPreviousHeld",
    ),
];

/// Shared body of the `FEUIInput_Is*` button queries: `InputBinding_EvalBool(
/// *(input + 0x18), button)` (0x320c48); `ret` is the call's return address.
pub(crate) fn oc821_feui_eval(c: &mut Cpu, button: u64, ret: u32) {
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

/// `FEUIInput_IsSelect(input)` (0x017fea0): asks the pad object at `*(input + 0x18)` whether
/// button 0x71 (select) is pressed and returns its answer.
pub fn feui_input_is_select(c: &mut Cpu) {
    oc821_feui_eval(c, 0x71, 0x0017feb4);
}

/// `FEUIInput_IsPrevious(input)` (0x017fee0): asks the pad object at `*(input + 0x18)` whether
/// button 0x7b (previous) is pressed and returns its answer.
pub fn feui_input_is_previous(c: &mut Cpu) {
    oc821_feui_eval(c, 0x7b, 0x0017fef4);
}

/// `FEUIInput_IsOption(input)` (0x017ff00): asks the pad object at `*(input + 0x18)` whether
/// button 0x7c (option) is pressed and returns its answer.
pub fn feui_input_is_option(c: &mut Cpu) {
    oc821_feui_eval(c, 0x7c, 0x0017ff14);
}

/// `FEUIInput_IsMisc(input)` (0x017ff20): asks the pad object at `*(input + 0x18)` whether
/// button 0x7d (misc) is pressed and returns its answer.
pub fn feui_input_is_misc(c: &mut Cpu) {
    oc821_feui_eval(c, 0x7d, 0x0017ff34);
}

/// `FEUIInput_IsNextHeld(input)` (0x017ff40): asks the pad object at `*(input + 0x18)` whether
/// button 0x97 (next (held)) is pressed and returns its answer.
pub fn feui_input_is_next_held(c: &mut Cpu) {
    oc821_feui_eval(c, 0x97, 0x0017ff54);
}

/// `FEUIInput_IsPreviousHeld(input)` (0x017ff60): asks the pad object at `*(input + 0x18)` whether
/// button 0x98 (previous (held)) is pressed and returns its answer.
pub fn feui_input_is_previous_held(c: &mut Cpu) {
    oc821_feui_eval(c, 0x98, 0x0017ff74);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes;
    use crate::difftest::fakes::machine;
    use crate::difftest::DATA;

    /// `FEUIInput_IsSelect`: the button query and its unwinds.
    #[test]
    fn feui_input_is_select_matches() {
        fakes::covered(
            ee::s02::f_0017fea0__generated,
            feui_input_is_select,
            60,
            &|c, seed| {
                let _r = machine(c, seed, 0x17fea0, 1);
                c.pc = 0x0017fea0;
                c.g[A0][0] = sx(DATA + 0x1000);
                c.bus.write32(DATA + 0x1000 + 0x18, DATA + 0x2000);
            },
        );
    }

    /// `FEUIInput_IsPrevious`: the button query and its unwinds.
    #[test]
    fn feui_input_is_previous_matches() {
        fakes::covered(
            ee::s02::f_0017fee0__generated,
            feui_input_is_previous,
            60,
            &|c, seed| {
                let _r = machine(c, seed, 0x17fee0, 1);
                c.pc = 0x0017fee0;
                c.g[A0][0] = sx(DATA + 0x1000);
                c.bus.write32(DATA + 0x1000 + 0x18, DATA + 0x2000);
            },
        );
    }

    /// `FEUIInput_IsOption`: the button query and its unwinds.
    #[test]
    fn feui_input_is_option_matches() {
        fakes::covered(
            ee::s02::f_0017ff00__generated,
            feui_input_is_option,
            60,
            &|c, seed| {
                let _r = machine(c, seed, 0x17ff00, 1);
                c.pc = 0x0017ff00;
                c.g[A0][0] = sx(DATA + 0x1000);
                c.bus.write32(DATA + 0x1000 + 0x18, DATA + 0x2000);
            },
        );
    }

    /// `FEUIInput_IsMisc`: the button query and its unwinds.
    #[test]
    fn feui_input_is_misc_matches() {
        fakes::covered(
            ee::s02::f_0017ff20__generated,
            feui_input_is_misc,
            60,
            &|c, seed| {
                let _r = machine(c, seed, 0x17ff20, 1);
                c.pc = 0x0017ff20;
                c.g[A0][0] = sx(DATA + 0x1000);
                c.bus.write32(DATA + 0x1000 + 0x18, DATA + 0x2000);
            },
        );
    }

    /// `FEUIInput_IsNextHeld`: the button query and its unwinds.
    #[test]
    fn feui_input_is_next_held_matches() {
        fakes::covered(
            ee::s02::f_0017ff40__generated,
            feui_input_is_next_held,
            60,
            &|c, seed| {
                let _r = machine(c, seed, 0x17ff40, 1);
                c.pc = 0x0017ff40;
                c.g[A0][0] = sx(DATA + 0x1000);
                c.bus.write32(DATA + 0x1000 + 0x18, DATA + 0x2000);
            },
        );
    }

    /// `FEUIInput_IsPreviousHeld`: the button query and its unwinds.
    #[test]
    fn feui_input_is_previous_held_matches() {
        fakes::covered(
            ee::s02::f_0017ff60__generated,
            feui_input_is_previous_held,
            60,
            &|c, seed| {
                let _r = machine(c, seed, 0x17ff60, 1);
                c.pc = 0x0017ff60;
                c.g[A0][0] = sx(DATA + 0x1000);
                c.bus.write32(DATA + 0x1000 + 0x18, DATA + 0x2000);
            },
        );
    }
}
