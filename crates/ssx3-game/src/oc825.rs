//! Staging module oc825 (wave wave248): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc825`.

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
        0x00180200,
        feui_input_is_l_shift1_held,
        "FEUIInput_IsLShift1Held",
    ),
    (
        0x00180220,
        feui_input_is_l_shift2_held,
        "FEUIInput_IsLShift2Held",
    ),
    (0x001802a0, feui_input_get_cam_zoom, "FEUIInput_GetCamZoom"),
    (0x001802c0, feui_input_get_cam_pan, "FEUIInput_GetCamPan"),
    (
        0x001802e0,
        feui_input_get_cam_rotate,
        "FEUIInput_GetCamRotate",
    ),
    (
        0x001807d8,
        keyboard_screen_build_masked_text,
        "KeyboardScreen_BuildMaskedText",
    ),
];

/// Shared body of the `FEUIInput_*` queries: `callee(*(input + 0x18), button)`;
/// `ret` is the call's return address.
pub(crate) fn oc825_feui_query(c: &mut Cpu, callee: u32, button: u64, ret: u32) {
    guest(c, |c| {
        c.cycles += 8;
        let frame = Frame::open(c, 0x10);
        c.g[A1][0] = button;
        frame.save_ra(c, 0);
        c.g[A0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x18));
        call_ptr(c, callee, ret)?;
        frame.restore_ra(c, 0);
        frame.close(c);
        Ok(())
    });
}

/// `FEUIInput_IsLShift1Held(input)`: is button 0x84 (LShift1, held) down.
pub fn feui_input_is_l_shift1_held(c: &mut Cpu) {
    oc825_feui_query(c, 0x0032_0c48, 0x84, 0x0018_0214);
}

/// `FEUIInput_IsLShift2Held(input)`: is button 0x85 (LShift2, held) down.
pub fn feui_input_is_l_shift2_held(c: &mut Cpu) {
    oc825_feui_query(c, 0x0032_0c48, 0x85, 0x0018_0234);
}

/// `FEUIInput_GetCamZoom(input)`: analog value of button 0x90 (camera zoom).
pub fn feui_input_get_cam_zoom(c: &mut Cpu) {
    oc825_feui_query(c, 0x0032_0bf0, 0x90, 0x0018_02b4);
}

/// `FEUIInput_GetCamPan(input)`: analog value of button 0x92 (camera pan).
pub fn feui_input_get_cam_pan(c: &mut Cpu) {
    oc825_feui_query(c, 0x0032_0bf0, 0x92, 0x0018_02d4);
}

/// `FEUIInput_GetCamRotate(input)`: analog value of button 0x91 (camera rotate).
pub fn feui_input_get_cam_rotate(c: &mut Cpu) {
    oc825_feui_query(c, 0x0032_0bf0, 0x91, 0x0018_02f4);
}

/// `KeyboardScreen_BuildMaskedText(screen)`: fills `screen + 0x12c` with one `*` per
/// character of the high-score name at `screen + 0xec`, terminates it, returns the buffer end.
pub fn keyboard_screen_build_masked_text(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 9;
        let frame = Frame::open(c, 0x20);
        c.sq(16, frame.at(c, 0x10));
        c.g[S0][0] = c.g[A0][0];
        frame.save_ra(c, 0);
        c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(0xec));
        call_ptr(c, 0x0041_6810, 0x0018_07f0)?;
        c.g[A1][0] = c.g[V0][0];
        let le = (c.g[A1][0] as i64) <= 0;
        c.g[A0][0] = 0;
        if !le {
            c.cycles += 3;
            c.g[A2][0] = sx((c.g[S0][0] as u32).wrapping_add(0x12c));
            c.g[A3][0] = 0x2a;
            loop {
                c.cycles += 7;
                c.g[V1][0] = sx((c.g[A2][0] as u32).wrapping_add(c.g[A0][0] as u32));
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(1));
                c.g[V0][0] = ((c.g[A0][0] as i64) < (c.g[A1][0] as i64)) as u64;
                let more = c.g[V0][0] != 0;
                c.sb(c.g[V1][0] as u32, c.g[A3][0]);
                if !more {
                    break;
                }
                if event_due(c) {
                    deliver_event_here(c, 0x0018_0808);
                }
            }
        }
        c.cycles += 7;
        c.g[V0][0] = sx((c.g[S0][0] as u32).wrapping_add(0x12c));
        frame.restore_ra(c, 0);
        c.g[V1][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[A1][0] as u32));
        c.lq(16, frame.at(c, 0x10));
        c.sb(c.g[V1][0] as u32, 0);
        frame.close(c);
        Ok(())
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::difftest::fakes;
    use crate::difftest::fakes::machine;
    use crate::difftest::DATA;

    fn oc825_query_fixture(c: &mut Cpu, seed: u32, entry: u32) {
        let _r = machine(c, seed, entry as u64, 1);
        c.pc = entry;
        c.g[A0][0] = sx(DATA + 0x1000);
        c.bus.write32(DATA + 0x1000 + 0x18, DATA + 0x2000);
    }

    /// `FEUIInput_IsLShift1Held`: the button query and its unwinds.
    #[test]
    fn feui_input_is_l_shift1_held_matches() {
        fakes::covered(
            ee::s02::f_00180200__generated,
            feui_input_is_l_shift1_held,
            60,
            &|c, seed| oc825_query_fixture(c, seed, 0x0018_0200),
        );
    }

    /// `FEUIInput_IsLShift2Held`: the button query and its unwinds.
    #[test]
    fn feui_input_is_l_shift2_held_matches() {
        fakes::covered(
            ee::s02::f_00180220__generated,
            feui_input_is_l_shift2_held,
            60,
            &|c, seed| oc825_query_fixture(c, seed, 0x0018_0220),
        );
    }

    /// `FEUIInput_GetCamZoom`: the analog query and its unwinds.
    #[test]
    fn feui_input_get_cam_zoom_matches() {
        fakes::covered(
            ee::s02::f_001802a0__generated,
            feui_input_get_cam_zoom,
            60,
            &|c, seed| oc825_query_fixture(c, seed, 0x0018_02a0),
        );
    }

    /// `FEUIInput_GetCamPan`: the analog query and its unwinds.
    #[test]
    fn feui_input_get_cam_pan_matches() {
        fakes::covered(
            ee::s02::f_001802c0__generated,
            feui_input_get_cam_pan,
            60,
            &|c, seed| oc825_query_fixture(c, seed, 0x0018_02c0),
        );
    }

    /// `FEUIInput_GetCamRotate`: the analog query and its unwinds.
    #[test]
    fn feui_input_get_cam_rotate_matches() {
        fakes::covered(
            ee::s02::f_001802e0__generated,
            feui_input_get_cam_rotate,
            60,
            &|c, seed| oc825_query_fixture(c, seed, 0x0018_02e0),
        );
    }

    /// `KeyboardScreen_BuildMaskedText`: the name length drives the star loop.
    #[test]
    fn keyboard_screen_build_masked_text_matches() {
        crate::difftest::check(
            ee::s02::f_001807d8__generated,
            keyboard_screen_build_masked_text,
            60,
            &|c, seed| {
                let _r = machine(c, seed, 0x1807d8, 1);
                c.pc = 0x0018_07d8;
                c.g[A0][0] = sx(DATA + 0x1000);
                c.bus.write32(DATA + 0x1000 + 0xec, 0x6968_6766);
                c.bus.write32(DATA + 0x1000 + 0xf0, 0x006b_6a00);
                // strlen is faked: force 1..=8 (the fake cannot return 0 or a sane random length).
                c.bus.write32(fakes::LOG_RESULTS, 1 + seed % 8);
            },
        );
    }
}
