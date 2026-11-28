//! Staging module oc1506 (wave wave276): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1506`.

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
    (0x001e_1080, ui_sound_base_v03_001e1080, "UISoundBase_v03"),
    (0x001e_1088, ui_sound_base_v04_001e1088, "UISoundBase_v04"),
    (0x001e_10e8, ui_custom_base_v02_001e10e8, "UICustomBase_v02"),
    (0x001e_10f0, ui_custom_base_v03_001e10f0, "UICustomBase_v03"),
    (
        0x001e_10f8,
        ui_custom_base_create_state_001e10f8,
        "UICustomBase_CreateState",
    ),
    (0x001e_1100, ui_custom_base_v05_001e1100, "UICustomBase_v05"),
    (0x001e_1138, ui_input_base_v02_001e1138, "UIInputBase_v02"),
    (
        0x001e_1140,
        ui_input_base_is_start_001e1140,
        "UIInputBase_IsStart",
    ),
    (
        0x001e_1148,
        ui_input_base_is_select_001e1148,
        "UIInputBase_IsSelect",
    ),
    (
        0x001e_1150,
        ui_input_base_is_next_001e1150,
        "UIInputBase_IsNext",
    ),
    (
        0x001e_1158,
        ui_input_base_is_previous_001e1158,
        "UIInputBase_IsPrevious",
    ),
    (
        0x001e_1160,
        ui_input_base_is_option_001e1160,
        "UIInputBase_IsOption",
    ),
    (
        0x001e_1168,
        ui_input_base_is_misc_001e1168,
        "UIInputBase_IsMisc",
    ),
    (
        0x001e_1170,
        ui_input_base_is_next_held_001e1170,
        "UIInputBase_IsNextHeld",
    ),
    (
        0x001e_1178,
        ui_input_base_is_previous_held_001e1178,
        "UIInputBase_IsPreviousHeld",
    ),
    (
        0x001e_1180,
        ui_input_base_is_option_held_001e1180,
        "UIInputBase_IsOptionHeld",
    ),
    (
        0x001e_1188,
        ui_input_base_is_misc_held_001e1188,
        "UIInputBase_IsMiscHeld",
    ),
    (
        0x001e_1190,
        ui_input_base_is_next_released_001e1190,
        "UIInputBase_IsNextReleased",
    ),
    (
        0x001e_1198,
        ui_input_base_is_previous_released_001e1198,
        "UIInputBase_IsPreviousReleased",
    ),
    (
        0x001e_11a0,
        ui_input_base_is_option_released_001e11a0,
        "UIInputBase_IsOptionReleased",
    ),
    (
        0x001e_11a8,
        ui_input_base_is_misc_released_001e11a8,
        "UIInputBase_IsMiscReleased",
    ),
    (
        0x001e_11b0,
        ui_input_base_is_up_001e11b0,
        "UIInputBase_IsUp",
    ),
    (
        0x001e_11b8,
        ui_input_base_is_down_001e11b8,
        "UIInputBase_IsDown",
    ),
    (
        0x001e_11c0,
        ui_input_base_is_left_001e11c0,
        "UIInputBase_IsLeft",
    ),
    (
        0x001e_11c8,
        ui_input_base_is_right_001e11c8,
        "UIInputBase_IsRight",
    ),
    (
        0x001e_11d0,
        ui_input_base_is_up_held_001e11d0,
        "UIInputBase_IsUpHeld",
    ),
    (
        0x001e_11d8,
        ui_input_base_is_down_held_001e11d8,
        "UIInputBase_IsDownHeld",
    ),
    (
        0x001e_11e0,
        ui_input_base_is_left_held_001e11e0,
        "UIInputBase_IsLeftHeld",
    ),
    (
        0x001e_11e8,
        ui_input_base_is_right_held_001e11e8,
        "UIInputBase_IsRightHeld",
    ),
    (
        0x001e_11f0,
        ui_input_base_is_r_shift1_001e11f0,
        "UIInputBase_IsRShift1",
    ),
    (
        0x001e_11f8,
        ui_input_base_is_r_shift2_001e11f8,
        "UIInputBase_IsRShift2",
    ),
    (
        0x001e_1200,
        ui_input_base_is_l_shift1_001e1200,
        "UIInputBase_IsLShift1",
    ),
    (
        0x001e_1208,
        ui_input_base_is_l_shift2_001e1208,
        "UIInputBase_IsLShift2",
    ),
    (
        0x001e_1210,
        ui_input_base_is_r_shift1_held_001e1210,
        "UIInputBase_IsRShift1Held",
    ),
    (
        0x001e_1218,
        ui_input_base_is_r_shift2_held_001e1218,
        "UIInputBase_IsRShift2Held",
    ),
    (
        0x001e_1220,
        ui_input_base_is_l_shift1_held_001e1220,
        "UIInputBase_IsLShift1Held",
    ),
    (
        0x001e_1228,
        ui_input_base_is_l_shift2_held_001e1228,
        "UIInputBase_IsLShift2Held",
    ),
    (
        0x001e_1230,
        ui_input_base_is_cam_zoom_001e1230,
        "UIInputBase_IsCamZoom",
    ),
    (
        0x001e_1238,
        ui_input_base_is_cam_pan_001e1238,
        "UIInputBase_IsCamPan",
    ),
    (
        0x001e_1240,
        ui_input_base_is_cam_rotate_001e1240,
        "UIInputBase_IsCamRotate",
    ),
    (
        0x001e_1248,
        ui_input_base_get_cam_zoom_001e1248,
        "UIInputBase_GetCamZoom",
    ),
    (
        0x001e_1258,
        ui_input_base_get_cam_pan_001e1258,
        "UIInputBase_GetCamPan",
    ),
    (
        0x001e_1268,
        ui_input_base_get_cam_rotate_001e1268,
        "UIInputBase_GetCamRotate",
    ),
    (
        0x001e_1278,
        ui_input_base_is_any_button_001e1278,
        "UIInputBase_IsAnyButton",
    ),
    (0x001e_1280, ui_input_base_v41_001e1280, "UIInputBase_v41"),
];

/// `UISoundBase_v03` (0x001e_1080): straight-line leaf. Mechanical port of the generated body.
pub fn ui_sound_base_v03_001e1080(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `UISoundBase_v04` (0x001e_1088): straight-line leaf. Mechanical port of the generated body.
pub fn ui_sound_base_v04_001e1088(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `UICustomBase_v02` (0x001e_10e8): straight-line leaf. Mechanical port of the generated body.
pub fn ui_custom_base_v02_001e10e8(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `UICustomBase_v03` (0x001e_10f0): straight-line leaf. Mechanical port of the generated body.
pub fn ui_custom_base_v03_001e10f0(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `UICustomBase_CreateState` (0x001e_10f8): straight-line leaf. Mechanical port of the generated body.
pub fn ui_custom_base_create_state_001e10f8(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `UICustomBase_v05` (0x001e_1100): straight-line leaf. Mechanical port of the generated body.
pub fn ui_custom_base_v05_001e1100(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `UIInputBase_v02` (0x001e_1138): straight-line leaf. Mechanical port of the generated body.
pub fn ui_input_base_v02_001e1138(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `UIInputBase_IsStart` (0x001e_1140): straight-line leaf. Mechanical port of the generated body.
pub fn ui_input_base_is_start_001e1140(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `UIInputBase_IsSelect` (0x001e_1148): straight-line leaf. Mechanical port of the generated body.
pub fn ui_input_base_is_select_001e1148(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `UIInputBase_IsNext` (0x001e_1150): straight-line leaf. Mechanical port of the generated body.
pub fn ui_input_base_is_next_001e1150(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `UIInputBase_IsPrevious` (0x001e_1158): straight-line leaf. Mechanical port of the generated body.
pub fn ui_input_base_is_previous_001e1158(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `UIInputBase_IsOption` (0x001e_1160): straight-line leaf. Mechanical port of the generated body.
pub fn ui_input_base_is_option_001e1160(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `UIInputBase_IsMisc` (0x001e_1168): straight-line leaf. Mechanical port of the generated body.
pub fn ui_input_base_is_misc_001e1168(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `UIInputBase_IsNextHeld` (0x001e_1170): straight-line leaf. Mechanical port of the generated body.
pub fn ui_input_base_is_next_held_001e1170(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `UIInputBase_IsPreviousHeld` (0x001e_1178): straight-line leaf. Mechanical port of the generated body.
pub fn ui_input_base_is_previous_held_001e1178(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `UIInputBase_IsOptionHeld` (0x001e_1180): straight-line leaf. Mechanical port of the generated body.
pub fn ui_input_base_is_option_held_001e1180(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `UIInputBase_IsMiscHeld` (0x001e_1188): straight-line leaf. Mechanical port of the generated body.
pub fn ui_input_base_is_misc_held_001e1188(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `UIInputBase_IsNextReleased` (0x001e_1190): straight-line leaf. Mechanical port of the generated body.
pub fn ui_input_base_is_next_released_001e1190(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `UIInputBase_IsPreviousReleased` (0x001e_1198): straight-line leaf. Mechanical port of the generated body.
pub fn ui_input_base_is_previous_released_001e1198(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `UIInputBase_IsOptionReleased` (0x001e_11a0): straight-line leaf. Mechanical port of the generated body.
pub fn ui_input_base_is_option_released_001e11a0(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `UIInputBase_IsMiscReleased` (0x001e_11a8): straight-line leaf. Mechanical port of the generated body.
pub fn ui_input_base_is_misc_released_001e11a8(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `UIInputBase_IsUp` (0x001e_11b0): straight-line leaf. Mechanical port of the generated body.
pub fn ui_input_base_is_up_001e11b0(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `UIInputBase_IsDown` (0x001e_11b8): straight-line leaf. Mechanical port of the generated body.
pub fn ui_input_base_is_down_001e11b8(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `UIInputBase_IsLeft` (0x001e_11c0): straight-line leaf. Mechanical port of the generated body.
pub fn ui_input_base_is_left_001e11c0(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `UIInputBase_IsRight` (0x001e_11c8): straight-line leaf. Mechanical port of the generated body.
pub fn ui_input_base_is_right_001e11c8(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `UIInputBase_IsUpHeld` (0x001e_11d0): straight-line leaf. Mechanical port of the generated body.
pub fn ui_input_base_is_up_held_001e11d0(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `UIInputBase_IsDownHeld` (0x001e_11d8): straight-line leaf. Mechanical port of the generated body.
pub fn ui_input_base_is_down_held_001e11d8(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `UIInputBase_IsLeftHeld` (0x001e_11e0): straight-line leaf. Mechanical port of the generated body.
pub fn ui_input_base_is_left_held_001e11e0(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `UIInputBase_IsRightHeld` (0x001e_11e8): straight-line leaf. Mechanical port of the generated body.
pub fn ui_input_base_is_right_held_001e11e8(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `UIInputBase_IsRShift1` (0x001e_11f0): straight-line leaf. Mechanical port of the generated body.
pub fn ui_input_base_is_r_shift1_001e11f0(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `UIInputBase_IsRShift2` (0x001e_11f8): straight-line leaf. Mechanical port of the generated body.
pub fn ui_input_base_is_r_shift2_001e11f8(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `UIInputBase_IsLShift1` (0x001e_1200): straight-line leaf. Mechanical port of the generated body.
pub fn ui_input_base_is_l_shift1_001e1200(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `UIInputBase_IsLShift2` (0x001e_1208): straight-line leaf. Mechanical port of the generated body.
pub fn ui_input_base_is_l_shift2_001e1208(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `UIInputBase_IsRShift1Held` (0x001e_1210): straight-line leaf. Mechanical port of the generated body.
pub fn ui_input_base_is_r_shift1_held_001e1210(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `UIInputBase_IsRShift2Held` (0x001e_1218): straight-line leaf. Mechanical port of the generated body.
pub fn ui_input_base_is_r_shift2_held_001e1218(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `UIInputBase_IsLShift1Held` (0x001e_1220): straight-line leaf. Mechanical port of the generated body.
pub fn ui_input_base_is_l_shift1_held_001e1220(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `UIInputBase_IsLShift2Held` (0x001e_1228): straight-line leaf. Mechanical port of the generated body.
pub fn ui_input_base_is_l_shift2_held_001e1228(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `UIInputBase_IsCamZoom` (0x001e_1230): straight-line leaf. Mechanical port of the generated body.
pub fn ui_input_base_is_cam_zoom_001e1230(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `UIInputBase_IsCamPan` (0x001e_1238): straight-line leaf. Mechanical port of the generated body.
pub fn ui_input_base_is_cam_pan_001e1238(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `UIInputBase_IsCamRotate` (0x001e_1240): straight-line leaf. Mechanical port of the generated body.
pub fn ui_input_base_is_cam_rotate_001e1240(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `UIInputBase_GetCamZoom` (0x001e_1248): straight-line leaf. Mechanical port of the generated body.
pub fn ui_input_base_get_cam_zoom_001e1248(c: &mut Cpu) {
    c.cycles += 3;
    c.mtc1(0, 0);
    ret(c);
}

/// `UIInputBase_GetCamPan` (0x001e_1258): straight-line leaf. Mechanical port of the generated body.
pub fn ui_input_base_get_cam_pan_001e1258(c: &mut Cpu) {
    c.cycles += 3;
    c.mtc1(0, 0);
    ret(c);
}

/// `UIInputBase_GetCamRotate` (0x001e_1268): straight-line leaf. Mechanical port of the generated body.
pub fn ui_input_base_get_cam_rotate_001e1268(c: &mut Cpu) {
    c.cycles += 3;
    c.mtc1(0, 0);
    ret(c);
}

/// `UIInputBase_IsAnyButton` (0x001e_1278): straight-line leaf. Mechanical port of the generated body.
pub fn ui_input_base_is_any_button_001e1278(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `UIInputBase_v41` (0x001e_1280): straight-line leaf. Mechanical port of the generated body.
pub fn ui_input_base_v41_001e1280(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::machine;
    use crate::difftest::{check, check_lohi, Rng};

    /// An edge-case 64-bit register value (0, 1, -1, small, random).
    #[allow(dead_code)]
    fn leaf_val(r: &mut Rng) -> u64 {
        match r.next_u32() % 8 {
            0 => 0,
            1 => 1,
            2 => u64::MAX,
            3 => (r.next_u32() % 64) as u64,
            4 => sx(r.next_u32()),
            5 => r.next_u32() as u64,
            6 => 0x8000_0000,
            _ => (r.next_u32() as u64) << 32 | r.next_u32() as u64,
        }
    }

    /// Fill `size` bytes at `start` with zero / small / random words, or, when
    /// `chase` is set, with pointers (16-byte aligned) into `ptr_lo..ptr_lo + 0x200`.
    #[allow(dead_code)]
    fn leaf_fill(c: &mut Cpu, r: &mut Rng, start: u32, size: u32, chase: bool, ptr_lo: u32) {
        for i in 0..size / 4 {
            let w = if chase {
                ptr_lo + 16 * (r.next_u32() % 32)
            } else {
                match r.next_u32() % 4 {
                    0 => 0,
                    1 => r.next_u32() % 16,
                    _ => r.next_u32(),
                }
            };
            c.bus.write32(start + 4 * i, w);
        }
    }

    /// `UISoundBase_v03` (0x001e_1080) against the generated body.
    #[test]
    fn ui_sound_base_v03_001e1080_matches() {
        check(
            ee::s04::f_001e1080__generated,
            ui_sound_base_v03_001e1080,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1e1080, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `UISoundBase_v04` (0x001e_1088) against the generated body.
    #[test]
    fn ui_sound_base_v04_001e1088_matches() {
        check(
            ee::s04::f_001e1088__generated,
            ui_sound_base_v04_001e1088,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1e1088, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `UICustomBase_v02` (0x001e_10e8) against the generated body.
    #[test]
    fn ui_custom_base_v02_001e10e8_matches() {
        check(
            ee::s04::f_001e10e8__generated,
            ui_custom_base_v02_001e10e8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1e10e8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `UICustomBase_v03` (0x001e_10f0) against the generated body.
    #[test]
    fn ui_custom_base_v03_001e10f0_matches() {
        check(
            ee::s04::f_001e10f0__generated,
            ui_custom_base_v03_001e10f0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1e10f0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `UICustomBase_CreateState` (0x001e_10f8) against the generated body.
    #[test]
    fn ui_custom_base_create_state_001e10f8_matches() {
        check(
            ee::s04::f_001e10f8__generated,
            ui_custom_base_create_state_001e10f8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1e10f8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `UICustomBase_v05` (0x001e_1100) against the generated body.
    #[test]
    fn ui_custom_base_v05_001e1100_matches() {
        check(
            ee::s04::f_001e1100__generated,
            ui_custom_base_v05_001e1100,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1e1100, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `UIInputBase_v02` (0x001e_1138) against the generated body.
    #[test]
    fn ui_input_base_v02_001e1138_matches() {
        check(
            ee::s04::f_001e1138__generated,
            ui_input_base_v02_001e1138,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1e1138, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `UIInputBase_IsStart` (0x001e_1140) against the generated body.
    #[test]
    fn ui_input_base_is_start_001e1140_matches() {
        check(
            ee::s04::f_001e1140__generated,
            ui_input_base_is_start_001e1140,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1e1140, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `UIInputBase_IsSelect` (0x001e_1148) against the generated body.
    #[test]
    fn ui_input_base_is_select_001e1148_matches() {
        check(
            ee::s04::f_001e1148__generated,
            ui_input_base_is_select_001e1148,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1e1148, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `UIInputBase_IsNext` (0x001e_1150) against the generated body.
    #[test]
    fn ui_input_base_is_next_001e1150_matches() {
        check(
            ee::s04::f_001e1150__generated,
            ui_input_base_is_next_001e1150,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1e1150, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `UIInputBase_IsPrevious` (0x001e_1158) against the generated body.
    #[test]
    fn ui_input_base_is_previous_001e1158_matches() {
        check(
            ee::s04::f_001e1158__generated,
            ui_input_base_is_previous_001e1158,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1e1158, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `UIInputBase_IsOption` (0x001e_1160) against the generated body.
    #[test]
    fn ui_input_base_is_option_001e1160_matches() {
        check(
            ee::s04::f_001e1160__generated,
            ui_input_base_is_option_001e1160,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1e1160, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `UIInputBase_IsMisc` (0x001e_1168) against the generated body.
    #[test]
    fn ui_input_base_is_misc_001e1168_matches() {
        check(
            ee::s04::f_001e1168__generated,
            ui_input_base_is_misc_001e1168,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1e1168, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `UIInputBase_IsNextHeld` (0x001e_1170) against the generated body.
    #[test]
    fn ui_input_base_is_next_held_001e1170_matches() {
        check(
            ee::s04::f_001e1170__generated,
            ui_input_base_is_next_held_001e1170,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1e1170, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `UIInputBase_IsPreviousHeld` (0x001e_1178) against the generated body.
    #[test]
    fn ui_input_base_is_previous_held_001e1178_matches() {
        check(
            ee::s04::f_001e1178__generated,
            ui_input_base_is_previous_held_001e1178,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1e1178, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `UIInputBase_IsOptionHeld` (0x001e_1180) against the generated body.
    #[test]
    fn ui_input_base_is_option_held_001e1180_matches() {
        check(
            ee::s04::f_001e1180__generated,
            ui_input_base_is_option_held_001e1180,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1e1180, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `UIInputBase_IsMiscHeld` (0x001e_1188) against the generated body.
    #[test]
    fn ui_input_base_is_misc_held_001e1188_matches() {
        check(
            ee::s04::f_001e1188__generated,
            ui_input_base_is_misc_held_001e1188,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1e1188, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `UIInputBase_IsNextReleased` (0x001e_1190) against the generated body.
    #[test]
    fn ui_input_base_is_next_released_001e1190_matches() {
        check(
            ee::s04::f_001e1190__generated,
            ui_input_base_is_next_released_001e1190,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1e1190, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `UIInputBase_IsPreviousReleased` (0x001e_1198) against the generated body.
    #[test]
    fn ui_input_base_is_previous_released_001e1198_matches() {
        check(
            ee::s04::f_001e1198__generated,
            ui_input_base_is_previous_released_001e1198,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1e1198, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `UIInputBase_IsOptionReleased` (0x001e_11a0) against the generated body.
    #[test]
    fn ui_input_base_is_option_released_001e11a0_matches() {
        check(
            ee::s04::f_001e11a0__generated,
            ui_input_base_is_option_released_001e11a0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1e11a0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `UIInputBase_IsMiscReleased` (0x001e_11a8) against the generated body.
    #[test]
    fn ui_input_base_is_misc_released_001e11a8_matches() {
        check(
            ee::s04::f_001e11a8__generated,
            ui_input_base_is_misc_released_001e11a8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1e11a8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `UIInputBase_IsUp` (0x001e_11b0) against the generated body.
    #[test]
    fn ui_input_base_is_up_001e11b0_matches() {
        check(
            ee::s04::f_001e11b0__generated,
            ui_input_base_is_up_001e11b0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1e11b0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `UIInputBase_IsDown` (0x001e_11b8) against the generated body.
    #[test]
    fn ui_input_base_is_down_001e11b8_matches() {
        check(
            ee::s04::f_001e11b8__generated,
            ui_input_base_is_down_001e11b8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1e11b8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `UIInputBase_IsLeft` (0x001e_11c0) against the generated body.
    #[test]
    fn ui_input_base_is_left_001e11c0_matches() {
        check(
            ee::s04::f_001e11c0__generated,
            ui_input_base_is_left_001e11c0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1e11c0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `UIInputBase_IsRight` (0x001e_11c8) against the generated body.
    #[test]
    fn ui_input_base_is_right_001e11c8_matches() {
        check(
            ee::s04::f_001e11c8__generated,
            ui_input_base_is_right_001e11c8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1e11c8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `UIInputBase_IsUpHeld` (0x001e_11d0) against the generated body.
    #[test]
    fn ui_input_base_is_up_held_001e11d0_matches() {
        check(
            ee::s04::f_001e11d0__generated,
            ui_input_base_is_up_held_001e11d0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1e11d0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `UIInputBase_IsDownHeld` (0x001e_11d8) against the generated body.
    #[test]
    fn ui_input_base_is_down_held_001e11d8_matches() {
        check(
            ee::s04::f_001e11d8__generated,
            ui_input_base_is_down_held_001e11d8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1e11d8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `UIInputBase_IsLeftHeld` (0x001e_11e0) against the generated body.
    #[test]
    fn ui_input_base_is_left_held_001e11e0_matches() {
        check(
            ee::s04::f_001e11e0__generated,
            ui_input_base_is_left_held_001e11e0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1e11e0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `UIInputBase_IsRightHeld` (0x001e_11e8) against the generated body.
    #[test]
    fn ui_input_base_is_right_held_001e11e8_matches() {
        check(
            ee::s04::f_001e11e8__generated,
            ui_input_base_is_right_held_001e11e8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1e11e8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `UIInputBase_IsRShift1` (0x001e_11f0) against the generated body.
    #[test]
    fn ui_input_base_is_r_shift1_001e11f0_matches() {
        check(
            ee::s04::f_001e11f0__generated,
            ui_input_base_is_r_shift1_001e11f0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1e11f0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `UIInputBase_IsRShift2` (0x001e_11f8) against the generated body.
    #[test]
    fn ui_input_base_is_r_shift2_001e11f8_matches() {
        check(
            ee::s04::f_001e11f8__generated,
            ui_input_base_is_r_shift2_001e11f8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1e11f8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `UIInputBase_IsLShift1` (0x001e_1200) against the generated body.
    #[test]
    fn ui_input_base_is_l_shift1_001e1200_matches() {
        check(
            ee::s04::f_001e1200__generated,
            ui_input_base_is_l_shift1_001e1200,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1e1200, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `UIInputBase_IsLShift2` (0x001e_1208) against the generated body.
    #[test]
    fn ui_input_base_is_l_shift2_001e1208_matches() {
        check(
            ee::s04::f_001e1208__generated,
            ui_input_base_is_l_shift2_001e1208,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1e1208, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `UIInputBase_IsRShift1Held` (0x001e_1210) against the generated body.
    #[test]
    fn ui_input_base_is_r_shift1_held_001e1210_matches() {
        check(
            ee::s04::f_001e1210__generated,
            ui_input_base_is_r_shift1_held_001e1210,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1e1210, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `UIInputBase_IsRShift2Held` (0x001e_1218) against the generated body.
    #[test]
    fn ui_input_base_is_r_shift2_held_001e1218_matches() {
        check(
            ee::s04::f_001e1218__generated,
            ui_input_base_is_r_shift2_held_001e1218,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1e1218, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `UIInputBase_IsLShift1Held` (0x001e_1220) against the generated body.
    #[test]
    fn ui_input_base_is_l_shift1_held_001e1220_matches() {
        check(
            ee::s04::f_001e1220__generated,
            ui_input_base_is_l_shift1_held_001e1220,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1e1220, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `UIInputBase_IsLShift2Held` (0x001e_1228) against the generated body.
    #[test]
    fn ui_input_base_is_l_shift2_held_001e1228_matches() {
        check(
            ee::s04::f_001e1228__generated,
            ui_input_base_is_l_shift2_held_001e1228,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1e1228, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `UIInputBase_IsCamZoom` (0x001e_1230) against the generated body.
    #[test]
    fn ui_input_base_is_cam_zoom_001e1230_matches() {
        check(
            ee::s04::f_001e1230__generated,
            ui_input_base_is_cam_zoom_001e1230,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1e1230, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `UIInputBase_IsCamPan` (0x001e_1238) against the generated body.
    #[test]
    fn ui_input_base_is_cam_pan_001e1238_matches() {
        check(
            ee::s04::f_001e1238__generated,
            ui_input_base_is_cam_pan_001e1238,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1e1238, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `UIInputBase_IsCamRotate` (0x001e_1240) against the generated body.
    #[test]
    fn ui_input_base_is_cam_rotate_001e1240_matches() {
        check(
            ee::s04::f_001e1240__generated,
            ui_input_base_is_cam_rotate_001e1240,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1e1240, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `UIInputBase_GetCamZoom` (0x001e_1248) against the generated body.
    #[test]
    fn ui_input_base_get_cam_zoom_001e1248_matches() {
        check(
            ee::s04::f_001e1248__generated,
            ui_input_base_get_cam_zoom_001e1248,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1e1248, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `UIInputBase_GetCamPan` (0x001e_1258) against the generated body.
    #[test]
    fn ui_input_base_get_cam_pan_001e1258_matches() {
        check(
            ee::s04::f_001e1258__generated,
            ui_input_base_get_cam_pan_001e1258,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1e1258, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `UIInputBase_GetCamRotate` (0x001e_1268) against the generated body.
    #[test]
    fn ui_input_base_get_cam_rotate_001e1268_matches() {
        check(
            ee::s04::f_001e1268__generated,
            ui_input_base_get_cam_rotate_001e1268,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1e1268, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `UIInputBase_IsAnyButton` (0x001e_1278) against the generated body.
    #[test]
    fn ui_input_base_is_any_button_001e1278_matches() {
        check(
            ee::s04::f_001e1278__generated,
            ui_input_base_is_any_button_001e1278,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1e1278, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `UIInputBase_v41` (0x001e_1280) against the generated body.
    #[test]
    fn ui_input_base_v41_001e1280_matches() {
        check(
            ee::s04::f_001e1280__generated,
            ui_input_base_v41_001e1280,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1e1280, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }
}
