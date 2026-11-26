//! Staging module oc823 (wave wave248): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc823`.

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
    (0x00180080, oc823_feui_is_left, "FEUIInput_IsLeft"),
    (0x001800a0, oc823_feui_is_right, "FEUIInput_IsRight"),
    (0x001800c0, oc823_feui_is_up_held, "FEUIInput_IsUpHeld"),
    (0x001800e0, oc823_feui_is_down_held, "FEUIInput_IsDownHeld"),
    (0x00180100, oc823_feui_is_left_held, "FEUIInput_IsLeftHeld"),
    (
        0x00180120,
        oc823_feui_is_right_held,
        "FEUIInput_IsRightHeld",
    ),
];

/// Shared body of the `FEUIInput_Is*` d-pad queries: `InputBinding_EvalBool(
/// *(input + 0x18), button)` (0x320c48); `ret` is the call's return address.
pub(crate) fn oc823_feui_eval(c: &mut Cpu, button: u64, ret: u32) {
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

/// `FEUIInput_IsLeft(input)` (0x0180080): asks the pad object at `*(input + 0x18)` whether
/// button 0x74 (left) is pressed and returns its answer.
pub fn oc823_feui_is_left(c: &mut Cpu) {
    oc823_feui_eval(c, 0x74, 0x00180094);
}

/// `FEUIInput_IsRight(input)` (0x01800a0): asks the pad object at `*(input + 0x18)` whether
/// button 0x75 (right) is pressed and returns its answer.
pub fn oc823_feui_is_right(c: &mut Cpu) {
    oc823_feui_eval(c, 0x75, 0x001800b4);
}

/// `FEUIInput_IsUpHeld(input)` (0x01800c0): asks the pad object at `*(input + 0x18)` whether
/// button 0x93 (up (held)) is pressed and returns its answer.
pub fn oc823_feui_is_up_held(c: &mut Cpu) {
    oc823_feui_eval(c, 0x93, 0x001800d4);
}

/// `FEUIInput_IsDownHeld(input)` (0x01800e0): asks the pad object at `*(input + 0x18)` whether
/// button 0x94 (down (held)) is pressed and returns its answer.
pub fn oc823_feui_is_down_held(c: &mut Cpu) {
    oc823_feui_eval(c, 0x94, 0x001800f4);
}

/// `FEUIInput_IsLeftHeld(input)` (0x0180100): asks the pad object at `*(input + 0x18)` whether
/// button 0x95 (left (held)) is pressed and returns its answer.
pub fn oc823_feui_is_left_held(c: &mut Cpu) {
    oc823_feui_eval(c, 0x95, 0x00180114);
}

/// `FEUIInput_IsRightHeld(input)` (0x0180120): asks the pad object at `*(input + 0x18)` whether
/// button 0x96 (right (held)) is pressed and returns its answer.
pub fn oc823_feui_is_right_held(c: &mut Cpu) {
    oc823_feui_eval(c, 0x96, 0x00180134);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes;
    use crate::difftest::fakes::machine;
    use crate::difftest::DATA;

    /// `FEUIInput_IsLeft`: the button query and its unwinds.
    #[test]
    fn oc823_feui_is_left_matches() {
        fakes::covered(
            ee::s02::f_00180080__generated,
            oc823_feui_is_left,
            60,
            &|c, seed| {
                let _r = machine(c, seed, 0x180080, 1);
                c.pc = 0x00180080;
                c.g[A0][0] = sx(DATA + 0x1000);
                c.bus.write32(DATA + 0x1000 + 0x18, DATA + 0x2000);
            },
        );
    }

    /// `FEUIInput_IsRight`: the button query and its unwinds.
    #[test]
    fn oc823_feui_is_right_matches() {
        fakes::covered(
            ee::s02::f_001800a0__generated,
            oc823_feui_is_right,
            60,
            &|c, seed| {
                let _r = machine(c, seed, 0x1800a0, 1);
                c.pc = 0x001800a0;
                c.g[A0][0] = sx(DATA + 0x1000);
                c.bus.write32(DATA + 0x1000 + 0x18, DATA + 0x2000);
            },
        );
    }

    /// `FEUIInput_IsUpHeld`: the button query and its unwinds.
    #[test]
    fn oc823_feui_is_up_held_matches() {
        fakes::covered(
            ee::s02::f_001800c0__generated,
            oc823_feui_is_up_held,
            60,
            &|c, seed| {
                let _r = machine(c, seed, 0x1800c0, 1);
                c.pc = 0x001800c0;
                c.g[A0][0] = sx(DATA + 0x1000);
                c.bus.write32(DATA + 0x1000 + 0x18, DATA + 0x2000);
            },
        );
    }

    /// `FEUIInput_IsDownHeld`: the button query and its unwinds.
    #[test]
    fn oc823_feui_is_down_held_matches() {
        fakes::covered(
            ee::s02::f_001800e0__generated,
            oc823_feui_is_down_held,
            60,
            &|c, seed| {
                let _r = machine(c, seed, 0x1800e0, 1);
                c.pc = 0x001800e0;
                c.g[A0][0] = sx(DATA + 0x1000);
                c.bus.write32(DATA + 0x1000 + 0x18, DATA + 0x2000);
            },
        );
    }

    /// `FEUIInput_IsLeftHeld`: the button query and its unwinds.
    #[test]
    fn oc823_feui_is_left_held_matches() {
        fakes::covered(
            ee::s02::f_00180100__generated,
            oc823_feui_is_left_held,
            60,
            &|c, seed| {
                let _r = machine(c, seed, 0x180100, 1);
                c.pc = 0x00180100;
                c.g[A0][0] = sx(DATA + 0x1000);
                c.bus.write32(DATA + 0x1000 + 0x18, DATA + 0x2000);
            },
        );
    }

    /// `FEUIInput_IsRightHeld`: the button query and its unwinds.
    #[test]
    fn oc823_feui_is_right_held_matches() {
        fakes::covered(
            ee::s02::f_00180120__generated,
            oc823_feui_is_right_held,
            60,
            &|c, seed| {
                let _r = machine(c, seed, 0x180120, 1);
                c.pc = 0x00180120;
                c.g[A0][0] = sx(DATA + 0x1000);
                c.bus.write32(DATA + 0x1000 + 0x18, DATA + 0x2000);
            },
        );
    }
}
