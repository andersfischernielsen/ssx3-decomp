//! Staging module oc824 (wave wave248): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc824`.

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
    (0x00180140, feui_input_is_r_shift1, "FEUIInput_IsRShift1"),
    (0x00180160, feui_input_is_r_shift2, "FEUIInput_IsRShift2"),
    (0x00180180, feui_input_is_l_shift1, "FEUIInput_IsLShift1"),
    (0x001801a0, feui_input_is_l_shift2, "FEUIInput_IsLShift2"),
    (
        0x001801c0,
        feui_input_is_r_shift1_held,
        "FEUIInput_IsRShift1Held",
    ),
    (
        0x001801e0,
        feui_input_is_r_shift2_held,
        "FEUIInput_IsRShift2Held",
    ),
];

/// Shared body of the `FEUIInput_Is*` shoulder queries: `InputBinding_EvalBool(
/// *(input + 0x18), button)` (0x320c48); `ret` is the call's return address.
pub(crate) fn oc824_feui_eval(c: &mut Cpu, button: u64, ret: u32) {
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

/// `FEUIInput_IsRShift1(input)` (0x0180140): asks the pad object at `*(input + 0x18)` whether
/// button 0x7e (RShift1) is pressed and returns its answer.
pub fn feui_input_is_r_shift1(c: &mut Cpu) {
    oc824_feui_eval(c, 0x7e, 0x00180154);
}

/// `FEUIInput_IsRShift2(input)` (0x0180160): asks the pad object at `*(input + 0x18)` whether
/// button 0x7f (RShift2) is pressed and returns its answer.
pub fn feui_input_is_r_shift2(c: &mut Cpu) {
    oc824_feui_eval(c, 0x7f, 0x00180174);
}

/// `FEUIInput_IsLShift1(input)` (0x0180180): asks the pad object at `*(input + 0x18)` whether
/// button 0x80 (LShift1) is pressed and returns its answer.
pub fn feui_input_is_l_shift1(c: &mut Cpu) {
    oc824_feui_eval(c, 0x80, 0x00180194);
}

/// `FEUIInput_IsLShift2(input)` (0x01801a0): asks the pad object at `*(input + 0x18)` whether
/// button 0x81 (LShift2) is pressed and returns its answer.
pub fn feui_input_is_l_shift2(c: &mut Cpu) {
    oc824_feui_eval(c, 0x81, 0x001801b4);
}

/// `FEUIInput_IsRShift1Held(input)` (0x01801c0): asks the pad object at `*(input + 0x18)` whether
/// button 0x82 (RShift1 (held)) is pressed and returns its answer.
pub fn feui_input_is_r_shift1_held(c: &mut Cpu) {
    oc824_feui_eval(c, 0x82, 0x001801d4);
}

/// `FEUIInput_IsRShift2Held(input)` (0x01801e0): asks the pad object at `*(input + 0x18)` whether
/// button 0x83 (RShift2 (held)) is pressed and returns its answer.
pub fn feui_input_is_r_shift2_held(c: &mut Cpu) {
    oc824_feui_eval(c, 0x83, 0x001801f4);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes;
    use crate::difftest::fakes::machine;
    use crate::difftest::DATA;

    /// `FEUIInput_IsRShift1`: the button query and its unwinds.
    #[test]
    fn feui_input_is_r_shift1_matches() {
        fakes::covered(
            ee::s02::f_00180140__generated,
            feui_input_is_r_shift1,
            60,
            &|c, seed| {
                let _r = machine(c, seed, 0x180140, 1);
                c.pc = 0x00180140;
                c.g[A0][0] = sx(DATA + 0x1000);
                c.bus.write32(DATA + 0x1000 + 0x18, DATA + 0x2000);
            },
        );
    }

    /// `FEUIInput_IsRShift2`: the button query and its unwinds.
    #[test]
    fn feui_input_is_r_shift2_matches() {
        fakes::covered(
            ee::s02::f_00180160__generated,
            feui_input_is_r_shift2,
            60,
            &|c, seed| {
                let _r = machine(c, seed, 0x180160, 1);
                c.pc = 0x00180160;
                c.g[A0][0] = sx(DATA + 0x1000);
                c.bus.write32(DATA + 0x1000 + 0x18, DATA + 0x2000);
            },
        );
    }

    /// `FEUIInput_IsLShift1`: the button query and its unwinds.
    #[test]
    fn feui_input_is_l_shift1_matches() {
        fakes::covered(
            ee::s02::f_00180180__generated,
            feui_input_is_l_shift1,
            60,
            &|c, seed| {
                let _r = machine(c, seed, 0x180180, 1);
                c.pc = 0x00180180;
                c.g[A0][0] = sx(DATA + 0x1000);
                c.bus.write32(DATA + 0x1000 + 0x18, DATA + 0x2000);
            },
        );
    }

    /// `FEUIInput_IsLShift2`: the button query and its unwinds.
    #[test]
    fn feui_input_is_l_shift2_matches() {
        fakes::covered(
            ee::s02::f_001801a0__generated,
            feui_input_is_l_shift2,
            60,
            &|c, seed| {
                let _r = machine(c, seed, 0x1801a0, 1);
                c.pc = 0x001801a0;
                c.g[A0][0] = sx(DATA + 0x1000);
                c.bus.write32(DATA + 0x1000 + 0x18, DATA + 0x2000);
            },
        );
    }

    /// `FEUIInput_IsRShift1Held`: the button query and its unwinds.
    #[test]
    fn feui_input_is_r_shift1_held_matches() {
        fakes::covered(
            ee::s02::f_001801c0__generated,
            feui_input_is_r_shift1_held,
            60,
            &|c, seed| {
                let _r = machine(c, seed, 0x1801c0, 1);
                c.pc = 0x001801c0;
                c.g[A0][0] = sx(DATA + 0x1000);
                c.bus.write32(DATA + 0x1000 + 0x18, DATA + 0x2000);
            },
        );
    }

    /// `FEUIInput_IsRShift2Held`: the button query and its unwinds.
    #[test]
    fn feui_input_is_r_shift2_held_matches() {
        fakes::covered(
            ee::s02::f_001801e0__generated,
            feui_input_is_r_shift2_held,
            60,
            &|c, seed| {
                let _r = machine(c, seed, 0x1801e0, 1);
                c.pc = 0x001801e0;
                c.g[A0][0] = sx(DATA + 0x1000);
                c.bus.write32(DATA + 0x1000 + 0x18, DATA + 0x2000);
            },
        );
    }
}
