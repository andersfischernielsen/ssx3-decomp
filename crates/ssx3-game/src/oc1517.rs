//! Staging module oc1517 (wave wave276): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1517`.

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
        0x0036_1910,
        position_modifier_get_transform_bounds_00361910,
        "PositionModifier_GetTransformBounds",
    ),
    (
        0x0036_1930,
        position_modifier_get_transform_radius_00361930,
        "PositionModifier_GetTransformRadius",
    ),
    (
        0x0036_1938,
        position_modifier_set_transform_radius_00361938,
        "PositionModifier_SetTransformRadius",
    ),
    (
        0x0036_19b8,
        parent_modifier_update_003619b8,
        "ParentModifier_Update",
    ),
    (
        0x0036_19c8,
        parent_modifier_v16_003619c8,
        "ParentModifier_v16",
    ),
    (
        0x0036_19d0,
        parent_modifier_get_type_003619d0,
        "ParentModifier_GetType",
    ),
    (
        0x0036_1a10,
        magnet_modifier_has_transform_changed_00361a10,
        "MagnetModifier_HasTransformChanged",
    ),
    (
        0x0036_1a20,
        magnet_modifier_v20_00361a20,
        "MagnetModifier_v20",
    ),
    (
        0x0036_1a28,
        magnet_modifier_get_transform_bounds_00361a28,
        "MagnetModifier_GetTransformBounds",
    ),
    (
        0x0036_1a48,
        magnet_modifier_get_transform_radius_00361a48,
        "MagnetModifier_GetTransformRadius",
    ),
    (
        0x0036_1a50,
        magnet_modifier_set_transform_radius_00361a50,
        "MagnetModifier_SetTransformRadius",
    ),
    (
        0x0036_1a58,
        magnet_modifier_get_type_00361a58,
        "MagnetModifier_GetType",
    ),
    (
        0x0036_1ad8,
        ava_spline_modifier_get_transform_bounds_00361ad8,
        "AvaSplineModifier_GetTransformBounds",
    ),
    (
        0x0036_1af8,
        ava_spline_modifier_get_transform_radius_00361af8,
        "AvaSplineModifier_GetTransformRadius",
    ),
    (
        0x0036_1b00,
        ava_spline_modifier_set_transform_radius_00361b00,
        "AvaSplineModifier_SetTransformRadius",
    ),
    (
        0x0036_1b08,
        ava_spline_modifier_get_type_00361b08,
        "AvaSplineModifier_GetType",
    ),
    (
        0x0036_1b10,
        ava_spline_modifier_get_matrix_00361b10,
        "AvaSplineModifier_GetMatrix",
    ),
    (
        0x0036_1b48,
        spline_modifier_is_transform_finished_00361b48,
        "SplineModifier_IsTransformFinished",
    ),
    (
        0x0036_1b50,
        spline_modifier_clear_transform_finished_00361b50,
        "SplineModifier_ClearTransformFinished",
    ),
    (
        0x0036_1b58,
        spline_modifier_get_transform_bounds_00361b58,
        "SplineModifier_GetTransformBounds",
    ),
    (
        0x0036_1b78,
        spline_modifier_get_transform_radius_00361b78,
        "SplineModifier_GetTransformRadius",
    ),
    (
        0x0036_1b80,
        spline_modifier_set_transform_radius_00361b80,
        "SplineModifier_SetTransformRadius",
    ),
    (
        0x0036_1b88,
        spline_modifier_has_transform_changed_00361b88,
        "SplineModifier_HasTransformChanged",
    ),
    (
        0x0036_1bd8,
        spline_modifier_get_type_00361bd8,
        "SplineModifier_GetType",
    ),
    (
        0x0036_1be0,
        multi_spline_modifier_get_transform_bounds_00361be0,
        "MultiSplineModifier_GetTransformBounds",
    ),
    (
        0x0036_1c10,
        multi_spline_modifier_get_transform_radius_00361c10,
        "MultiSplineModifier_GetTransformRadius",
    ),
    (
        0x0036_1c18,
        multi_spline_modifier_set_transform_radius_00361c18,
        "MultiSplineModifier_SetTransformRadius",
    ),
    (
        0x0036_1c70,
        multi_spline_modifier_get_type_00361c70,
        "MultiSplineModifier_GetType",
    ),
    (
        0x0036_1ca8,
        roller_modifier_v08_00361ca8,
        "RollerModifier_v08",
    ),
    (
        0x0036_1cf8,
        roller_modifier_v09_00361cf8,
        "RollerModifier_v09",
    ),
    (
        0x0036_1d18,
        roller_modifier_get_transform_bounds_00361d18,
        "RollerModifier_GetTransformBounds",
    ),
    (
        0x0036_1d20,
        roller_modifier_get_transform_radius_00361d20,
        "RollerModifier_GetTransformRadius",
    ),
    (
        0x0036_1d28,
        roller_modifier_set_transform_radius_00361d28,
        "RollerModifier_SetTransformRadius",
    ),
    (
        0x0036_1d30,
        roller_modifier_get_matrix_00361d30,
        "RollerModifier_GetMatrix",
    ),
    (
        0x0036_1d38,
        roller_modifier_v19_00361d38,
        "RollerModifier_v19",
    ),
    (
        0x0036_1d40,
        roller_modifier_get_type_00361d40,
        "RollerModifier_GetType",
    ),
    (0x0036_1d88, anim_combo_v19_00361d88, "AnimCombo_v19"),
    (0x0036_1e08, anim_delta_v19_00361e08, "AnimDelta_v19"),
    (
        0x0036_a1b0,
        graphics_man_acquire_texture_ref_0036a1b0,
        "GraphicsMan_AcquireTextureRef",
    ),
    (
        0x0037_5a00,
        detail_system_update_00375a00,
        "DetailSystem_Update",
    ),
    (
        0x0038_b178,
        ps2_bezier_man_begin_pass_0038b178,
        "PS2BezierMan_BeginPass",
    ),
    (
        0x0038_f7f8,
        vfx_visual_effects_test_update_0038f7f8,
        "VfxVisualEffectsTest_Update",
    ),
    (
        0x0039_3fa8,
        graphics_man_set_default_display_mode_00393fa8,
        "GraphicsMan_SetDefaultDisplayMode",
    ),
    (
        0x0039_3fb0,
        graphics_man_is_progressive_00393fb0,
        "GraphicsMan_IsProgressive",
    ),
    (0x0039_4008, graphics_man_v16_00394008, "GraphicsMan_v16"),
];

/// `PositionModifier_GetTransformBounds` (0x0036_1910): straight-line leaf. Mechanical port of the generated body.
pub fn position_modifier_get_transform_bounds_00361910(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x10));
    ret(c);
}

/// `PositionModifier_GetTransformRadius` (0x0036_1930): straight-line leaf: 1 load(s). Mechanical port of the generated body.
pub fn position_modifier_get_transform_radius_00361930(c: &mut Cpu) {
    c.cycles += 2;
    c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(0x40));
    ret(c);
}

/// `PositionModifier_SetTransformRadius` (0x0036_1938): straight-line leaf: 1 store(s). Mechanical port of the generated body.
pub fn position_modifier_set_transform_radius_00361938(c: &mut Cpu) {
    c.cycles += 2;
    c.swc1(12, (c.g[A0][0] as u32).wrapping_add(0x40));
    ret(c);
}

/// `ParentModifier_Update` (0x0036_19b8): straight-line leaf: 1 store(s). Mechanical port of the generated body.
pub fn parent_modifier_update_003619b8(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = 0x1;
    c.sw((c.g[A0][0] as u32).wrapping_add(0x44), c.g[V0][0]);
    ret(c);
}

/// `ParentModifier_v16` (0x0036_19c8): straight-line leaf: 1 load(s). Mechanical port of the generated body.
pub fn parent_modifier_v16_003619c8(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x90));
    ret(c);
}

/// `ParentModifier_GetType` (0x0036_19d0): straight-line leaf. Mechanical port of the generated body.
pub fn parent_modifier_get_type_003619d0(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0x4;
    ret(c);
}

/// `MagnetModifier_HasTransformChanged` (0x0036_1a10): straight-line leaf: 1 load(s). Mechanical port of the generated body.
pub fn magnet_modifier_has_transform_changed_00361a10(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x50));
    c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
    ret(c);
}

/// `MagnetModifier_v20` (0x0036_1a20): straight-line leaf. Mechanical port of the generated body.
pub fn magnet_modifier_v20_00361a20(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0x1;
    ret(c);
}

/// `MagnetModifier_GetTransformBounds` (0x0036_1a28): straight-line leaf. Mechanical port of the generated body.
pub fn magnet_modifier_get_transform_bounds_00361a28(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
    ret(c);
}

/// `MagnetModifier_GetTransformRadius` (0x0036_1a48): straight-line leaf: 1 load(s). Mechanical port of the generated body.
pub fn magnet_modifier_get_transform_radius_00361a48(c: &mut Cpu) {
    c.cycles += 2;
    c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(0x40));
    ret(c);
}

/// `MagnetModifier_SetTransformRadius` (0x0036_1a50): straight-line leaf. Mechanical port of the generated body.
pub fn magnet_modifier_set_transform_radius_00361a50(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `MagnetModifier_GetType` (0x0036_1a58): straight-line leaf. Mechanical port of the generated body.
pub fn magnet_modifier_get_type_00361a58(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0x6;
    ret(c);
}

/// `AvaSplineModifier_GetTransformBounds` (0x0036_1ad8): straight-line leaf. Mechanical port of the generated body.
pub fn ava_spline_modifier_get_transform_bounds_00361ad8(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x10));
    ret(c);
}

/// `AvaSplineModifier_GetTransformRadius` (0x0036_1af8): straight-line leaf: 1 load(s). Mechanical port of the generated body.
pub fn ava_spline_modifier_get_transform_radius_00361af8(c: &mut Cpu) {
    c.cycles += 2;
    c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(0x30));
    ret(c);
}

/// `AvaSplineModifier_SetTransformRadius` (0x0036_1b00): straight-line leaf: 1 store(s). Mechanical port of the generated body.
pub fn ava_spline_modifier_set_transform_radius_00361b00(c: &mut Cpu) {
    c.cycles += 2;
    c.swc1(12, (c.g[A0][0] as u32).wrapping_add(0x30));
    ret(c);
}

/// `AvaSplineModifier_GetType` (0x0036_1b08): straight-line leaf. Mechanical port of the generated body.
pub fn ava_spline_modifier_get_type_00361b08(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0x7;
    ret(c);
}

/// `AvaSplineModifier_GetMatrix` (0x0036_1b10): straight-line leaf. Mechanical port of the generated body.
pub fn ava_spline_modifier_get_matrix_00361b10(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x40));
    ret(c);
}

/// `SplineModifier_IsTransformFinished` (0x0036_1b48): straight-line leaf: 1 load(s). Mechanical port of the generated body.
pub fn spline_modifier_is_transform_finished_00361b48(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x50));
    ret(c);
}

/// `SplineModifier_ClearTransformFinished` (0x0036_1b50): straight-line leaf: 1 store(s). Mechanical port of the generated body.
pub fn spline_modifier_clear_transform_finished_00361b50(c: &mut Cpu) {
    c.cycles += 2;
    c.sw((c.g[A0][0] as u32).wrapping_add(0x50), 0u64);
    ret(c);
}

/// `SplineModifier_GetTransformBounds` (0x0036_1b58): straight-line leaf. Mechanical port of the generated body.
pub fn spline_modifier_get_transform_bounds_00361b58(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x10));
    ret(c);
}

/// `SplineModifier_GetTransformRadius` (0x0036_1b78): straight-line leaf: 1 load(s). Mechanical port of the generated body.
pub fn spline_modifier_get_transform_radius_00361b78(c: &mut Cpu) {
    c.cycles += 2;
    c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(0x4c));
    ret(c);
}

/// `SplineModifier_SetTransformRadius` (0x0036_1b80): straight-line leaf: 1 store(s). Mechanical port of the generated body.
pub fn spline_modifier_set_transform_radius_00361b80(c: &mut Cpu) {
    c.cycles += 2;
    c.swc1(12, (c.g[A0][0] as u32).wrapping_add(0x4c));
    ret(c);
}

/// `SplineModifier_HasTransformChanged` (0x0036_1b88): straight-line leaf: 1 load(s). Mechanical port of the generated body.
pub fn spline_modifier_has_transform_changed_00361b88(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x58));
    ret(c);
}

/// `SplineModifier_GetType` (0x0036_1bd8): straight-line leaf. Mechanical port of the generated body.
pub fn spline_modifier_get_type_00361bd8(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0x1;
    ret(c);
}

/// `MultiSplineModifier_GetTransformBounds` (0x0036_1be0): straight-line leaf: 1 load(s). Mechanical port of the generated body.
pub fn multi_spline_modifier_get_transform_bounds_00361be0(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x3c));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x30));
    ret(c);
}

/// `MultiSplineModifier_GetTransformRadius` (0x0036_1c10): straight-line leaf: 1 load(s). Mechanical port of the generated body.
pub fn multi_spline_modifier_get_transform_radius_00361c10(c: &mut Cpu) {
    c.cycles += 2;
    c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(0x18));
    ret(c);
}

/// `MultiSplineModifier_SetTransformRadius` (0x0036_1c18): straight-line leaf: 1 store(s). Mechanical port of the generated body.
pub fn multi_spline_modifier_set_transform_radius_00361c18(c: &mut Cpu) {
    c.cycles += 2;
    c.swc1(12, (c.g[A0][0] as u32).wrapping_add(0x18));
    ret(c);
}

/// `MultiSplineModifier_GetType` (0x0036_1c70): straight-line leaf. Mechanical port of the generated body.
pub fn multi_spline_modifier_get_type_00361c70(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0x2;
    ret(c);
}

/// `RollerModifier_v08` (0x0036_1ca8): straight-line leaf. Mechanical port of the generated body.
pub fn roller_modifier_v08_00361ca8(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `RollerModifier_v09` (0x0036_1cf8): straight-line leaf. Mechanical port of the generated body.
pub fn roller_modifier_v09_00361cf8(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `RollerModifier_GetTransformBounds` (0x0036_1d18): straight-line leaf. Mechanical port of the generated body.
pub fn roller_modifier_get_transform_bounds_00361d18(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x220));
    ret(c);
}

/// `RollerModifier_GetTransformRadius` (0x0036_1d20): straight-line leaf: 1 load(s). Mechanical port of the generated body.
pub fn roller_modifier_get_transform_radius_00361d20(c: &mut Cpu) {
    c.cycles += 2;
    c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(0x24));
    ret(c);
}

/// `RollerModifier_SetTransformRadius` (0x0036_1d28): straight-line leaf: 1 store(s). Mechanical port of the generated body.
pub fn roller_modifier_set_transform_radius_00361d28(c: &mut Cpu) {
    c.cycles += 2;
    c.swc1(12, (c.g[A0][0] as u32).wrapping_add(0x24));
    ret(c);
}

/// `RollerModifier_GetMatrix` (0x0036_1d30): straight-line leaf. Mechanical port of the generated body.
pub fn roller_modifier_get_matrix_00361d30(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x240));
    ret(c);
}

/// `RollerModifier_v19` (0x0036_1d38): straight-line leaf. Mechanical port of the generated body.
pub fn roller_modifier_v19_00361d38(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x240));
    ret(c);
}

/// `RollerModifier_GetType` (0x0036_1d40): straight-line leaf. Mechanical port of the generated body.
pub fn roller_modifier_get_type_00361d40(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0x5;
    ret(c);
}

/// `AnimCombo_v19` (0x0036_1d88): straight-line leaf. Mechanical port of the generated body.
pub fn anim_combo_v19_00361d88(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = c.g[A0][0];
    ret(c);
}

/// `AnimDelta_v19` (0x0036_1e08): straight-line leaf. Mechanical port of the generated body.
pub fn anim_delta_v19_00361e08(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = c.g[A0][0];
    ret(c);
}

/// `GraphicsMan_AcquireTextureRef` (0x0036_a1b0): straight-line leaf: 2 store(s). Mechanical port of the generated body.
pub fn graphics_man_acquire_texture_ref_0036a1b0(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V0][0] = 0xffffffffffffffff;
    c.sw((c.g[A1][0] as u32), 0u64);
    c.sw((c.g[A1][0] as u32).wrapping_add(0x4), c.g[V0][0]);
    ret(c);
}

/// `DetailSystem_Update` (0x0037_5a00): straight-line leaf. Mechanical port of the generated body.
pub fn detail_system_update_00375a00(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `PS2BezierMan_BeginPass` (0x0038_b178): straight-line leaf: 4 store(s). Mechanical port of the generated body.
pub fn ps2_bezier_man_begin_pass_0038b178(c: &mut Cpu) {
    c.cycles += 5;
    c.sw((c.g[A0][0] as u32).wrapping_add(0x4d84), 0u64);
    c.sw((c.g[A0][0] as u32).wrapping_add(0x458), 0u64);
    c.sw((c.g[A0][0] as u32).wrapping_add(0x3c9c), 0u64);
    c.sw((c.g[A0][0] as u32).wrapping_add(0x4a60), 0u64);
    ret(c);
}

/// `VfxVisualEffectsTest_Update` (0x0038_f7f8): straight-line leaf. Mechanical port of the generated body.
pub fn vfx_visual_effects_test_update_0038f7f8(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `GraphicsMan_SetDefaultDisplayMode` (0x0039_3fa8): straight-line leaf. Mechanical port of the generated body.
pub fn graphics_man_set_default_display_mode_00393fa8(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `GraphicsMan_IsProgressive` (0x0039_3fb0): straight-line leaf. Mechanical port of the generated body.
pub fn graphics_man_is_progressive_00393fb0(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `GraphicsMan_v16` (0x0039_4008): straight-line leaf. Mechanical port of the generated body.
pub fn graphics_man_v16_00394008(c: &mut Cpu) {
    c.cycles += 2;
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

    /// `PositionModifier_GetTransformBounds` (0x0036_1910) against the generated body.
    #[test]
    fn position_modifier_get_transform_bounds_00361910_matches() {
        check(
            ee::s12::f_00361910__generated,
            position_modifier_get_transform_bounds_00361910,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x361910, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `PositionModifier_GetTransformRadius` (0x0036_1930) against the generated body.
    #[test]
    fn position_modifier_get_transform_radius_00361930_matches() {
        check(
            ee::s12::f_00361930__generated,
            position_modifier_get_transform_radius_00361930,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x361930, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `PositionModifier_SetTransformRadius` (0x0036_1938) against the generated body.
    #[test]
    fn position_modifier_set_transform_radius_00361938_matches() {
        check(
            ee::s12::f_00361938__generated,
            position_modifier_set_transform_radius_00361938,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x361938, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.f[12] = r.edge_f32();
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `ParentModifier_Update` (0x0036_19b8) against the generated body.
    #[test]
    fn parent_modifier_update_003619b8_matches() {
        check(
            ee::s12::f_003619b8__generated,
            parent_modifier_update_003619b8,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3619b8, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `ParentModifier_v16` (0x0036_19c8) against the generated body.
    #[test]
    fn parent_modifier_v16_003619c8_matches() {
        check(
            ee::s12::f_003619c8__generated,
            parent_modifier_v16_003619c8,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3619c8, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `ParentModifier_GetType` (0x0036_19d0) against the generated body.
    #[test]
    fn parent_modifier_get_type_003619d0_matches() {
        check(
            ee::s12::f_003619d0__generated,
            parent_modifier_get_type_003619d0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x3619d0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `MagnetModifier_HasTransformChanged` (0x0036_1a10) against the generated body.
    #[test]
    fn magnet_modifier_has_transform_changed_00361a10_matches() {
        check(
            ee::s12::f_00361a10__generated,
            magnet_modifier_has_transform_changed_00361a10,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x361a10, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `MagnetModifier_v20` (0x0036_1a20) against the generated body.
    #[test]
    fn magnet_modifier_v20_00361a20_matches() {
        check(
            ee::s12::f_00361a20__generated,
            magnet_modifier_v20_00361a20,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x361a20, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `MagnetModifier_GetTransformBounds` (0x0036_1a28) against the generated body.
    #[test]
    fn magnet_modifier_get_transform_bounds_00361a28_matches() {
        check(
            ee::s12::f_00361a28__generated,
            magnet_modifier_get_transform_bounds_00361a28,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x361a28, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `MagnetModifier_GetTransformRadius` (0x0036_1a48) against the generated body.
    #[test]
    fn magnet_modifier_get_transform_radius_00361a48_matches() {
        check(
            ee::s12::f_00361a48__generated,
            magnet_modifier_get_transform_radius_00361a48,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x361a48, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `MagnetModifier_SetTransformRadius` (0x0036_1a50) against the generated body.
    #[test]
    fn magnet_modifier_set_transform_radius_00361a50_matches() {
        check(
            ee::s12::f_00361a50__generated,
            magnet_modifier_set_transform_radius_00361a50,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x361a50, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `MagnetModifier_GetType` (0x0036_1a58) against the generated body.
    #[test]
    fn magnet_modifier_get_type_00361a58_matches() {
        check(
            ee::s12::f_00361a58__generated,
            magnet_modifier_get_type_00361a58,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x361a58, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `AvaSplineModifier_GetTransformBounds` (0x0036_1ad8) against the generated body.
    #[test]
    fn ava_spline_modifier_get_transform_bounds_00361ad8_matches() {
        check(
            ee::s12::f_00361ad8__generated,
            ava_spline_modifier_get_transform_bounds_00361ad8,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x361ad8, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `AvaSplineModifier_GetTransformRadius` (0x0036_1af8) against the generated body.
    #[test]
    fn ava_spline_modifier_get_transform_radius_00361af8_matches() {
        check(
            ee::s12::f_00361af8__generated,
            ava_spline_modifier_get_transform_radius_00361af8,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x361af8, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `AvaSplineModifier_SetTransformRadius` (0x0036_1b00) against the generated body.
    #[test]
    fn ava_spline_modifier_set_transform_radius_00361b00_matches() {
        check(
            ee::s12::f_00361b00__generated,
            ava_spline_modifier_set_transform_radius_00361b00,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x361b00, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.f[12] = r.edge_f32();
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `AvaSplineModifier_GetType` (0x0036_1b08) against the generated body.
    #[test]
    fn ava_spline_modifier_get_type_00361b08_matches() {
        check(
            ee::s12::f_00361b08__generated,
            ava_spline_modifier_get_type_00361b08,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x361b08, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `AvaSplineModifier_GetMatrix` (0x0036_1b10) against the generated body.
    #[test]
    fn ava_spline_modifier_get_matrix_00361b10_matches() {
        check(
            ee::s12::f_00361b10__generated,
            ava_spline_modifier_get_matrix_00361b10,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x361b10, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `SplineModifier_IsTransformFinished` (0x0036_1b48) against the generated body.
    #[test]
    fn spline_modifier_is_transform_finished_00361b48_matches() {
        check(
            ee::s12::f_00361b48__generated,
            spline_modifier_is_transform_finished_00361b48,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x361b48, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `SplineModifier_ClearTransformFinished` (0x0036_1b50) against the generated body.
    #[test]
    fn spline_modifier_clear_transform_finished_00361b50_matches() {
        check(
            ee::s12::f_00361b50__generated,
            spline_modifier_clear_transform_finished_00361b50,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x361b50, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `SplineModifier_GetTransformBounds` (0x0036_1b58) against the generated body.
    #[test]
    fn spline_modifier_get_transform_bounds_00361b58_matches() {
        check(
            ee::s12::f_00361b58__generated,
            spline_modifier_get_transform_bounds_00361b58,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x361b58, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `SplineModifier_GetTransformRadius` (0x0036_1b78) against the generated body.
    #[test]
    fn spline_modifier_get_transform_radius_00361b78_matches() {
        check(
            ee::s12::f_00361b78__generated,
            spline_modifier_get_transform_radius_00361b78,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x361b78, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `SplineModifier_SetTransformRadius` (0x0036_1b80) against the generated body.
    #[test]
    fn spline_modifier_set_transform_radius_00361b80_matches() {
        check(
            ee::s12::f_00361b80__generated,
            spline_modifier_set_transform_radius_00361b80,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x361b80, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.f[12] = r.edge_f32();
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `SplineModifier_HasTransformChanged` (0x0036_1b88) against the generated body.
    #[test]
    fn spline_modifier_has_transform_changed_00361b88_matches() {
        check(
            ee::s12::f_00361b88__generated,
            spline_modifier_has_transform_changed_00361b88,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x361b88, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `SplineModifier_GetType` (0x0036_1bd8) against the generated body.
    #[test]
    fn spline_modifier_get_type_00361bd8_matches() {
        check(
            ee::s12::f_00361bd8__generated,
            spline_modifier_get_type_00361bd8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x361bd8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `MultiSplineModifier_GetTransformBounds` (0x0036_1be0) against the generated body.
    #[test]
    fn multi_spline_modifier_get_transform_bounds_00361be0_matches() {
        check(
            ee::s12::f_00361be0__generated,
            multi_spline_modifier_get_transform_bounds_00361be0,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x361be0, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `MultiSplineModifier_GetTransformRadius` (0x0036_1c10) against the generated body.
    #[test]
    fn multi_spline_modifier_get_transform_radius_00361c10_matches() {
        check(
            ee::s12::f_00361c10__generated,
            multi_spline_modifier_get_transform_radius_00361c10,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x361c10, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `MultiSplineModifier_SetTransformRadius` (0x0036_1c18) against the generated body.
    #[test]
    fn multi_spline_modifier_set_transform_radius_00361c18_matches() {
        check(
            ee::s12::f_00361c18__generated,
            multi_spline_modifier_set_transform_radius_00361c18,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x361c18, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.f[12] = r.edge_f32();
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `MultiSplineModifier_GetType` (0x0036_1c70) against the generated body.
    #[test]
    fn multi_spline_modifier_get_type_00361c70_matches() {
        check(
            ee::s12::f_00361c70__generated,
            multi_spline_modifier_get_type_00361c70,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x361c70, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `RollerModifier_v08` (0x0036_1ca8) against the generated body.
    #[test]
    fn roller_modifier_v08_00361ca8_matches() {
        check(
            ee::s12::f_00361ca8__generated,
            roller_modifier_v08_00361ca8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x361ca8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `RollerModifier_v09` (0x0036_1cf8) against the generated body.
    #[test]
    fn roller_modifier_v09_00361cf8_matches() {
        check(
            ee::s12::f_00361cf8__generated,
            roller_modifier_v09_00361cf8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x361cf8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `RollerModifier_GetTransformBounds` (0x0036_1d18) against the generated body.
    #[test]
    fn roller_modifier_get_transform_bounds_00361d18_matches() {
        check(
            ee::s12::f_00361d18__generated,
            roller_modifier_get_transform_bounds_00361d18,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x361d18, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `RollerModifier_GetTransformRadius` (0x0036_1d20) against the generated body.
    #[test]
    fn roller_modifier_get_transform_radius_00361d20_matches() {
        check(
            ee::s12::f_00361d20__generated,
            roller_modifier_get_transform_radius_00361d20,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x361d20, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `RollerModifier_SetTransformRadius` (0x0036_1d28) against the generated body.
    #[test]
    fn roller_modifier_set_transform_radius_00361d28_matches() {
        check(
            ee::s12::f_00361d28__generated,
            roller_modifier_set_transform_radius_00361d28,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x361d28, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.f[12] = r.edge_f32();
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `RollerModifier_GetMatrix` (0x0036_1d30) against the generated body.
    #[test]
    fn roller_modifier_get_matrix_00361d30_matches() {
        check(
            ee::s12::f_00361d30__generated,
            roller_modifier_get_matrix_00361d30,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x361d30, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `RollerModifier_v19` (0x0036_1d38) against the generated body.
    #[test]
    fn roller_modifier_v19_00361d38_matches() {
        check(
            ee::s12::f_00361d38__generated,
            roller_modifier_v19_00361d38,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x361d38, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `RollerModifier_GetType` (0x0036_1d40) against the generated body.
    #[test]
    fn roller_modifier_get_type_00361d40_matches() {
        check(
            ee::s12::f_00361d40__generated,
            roller_modifier_get_type_00361d40,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x361d40, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `AnimCombo_v19` (0x0036_1d88) against the generated body.
    #[test]
    fn anim_combo_v19_00361d88_matches() {
        check(
            ee::s12::f_00361d88__generated,
            anim_combo_v19_00361d88,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x361d88, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `AnimDelta_v19` (0x0036_1e08) against the generated body.
    #[test]
    fn anim_delta_v19_00361e08_matches() {
        check(
            ee::s12::f_00361e08__generated,
            anim_delta_v19_00361e08,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x361e08, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `GraphicsMan_AcquireTextureRef` (0x0036_a1b0) against the generated body.
    #[test]
    fn graphics_man_acquire_texture_ref_0036a1b0_matches() {
        check(
            ee::s12::f_0036a1b0__generated,
            graphics_man_acquire_texture_ref_0036a1b0,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x36a1b0, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A1][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `DetailSystem_Update` (0x0037_5a00) against the generated body.
    #[test]
    fn detail_system_update_00375a00_matches() {
        check(
            ee::s12::f_00375a00__generated,
            detail_system_update_00375a00,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x375a00, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `PS2BezierMan_BeginPass` (0x0038_b178) against the generated body.
    #[test]
    fn ps2_bezier_man_begin_pass_0038b178_matches() {
        check(
            ee::s12::f_0038b178__generated,
            ps2_bezier_man_begin_pass_0038b178,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x38b178, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x6000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `VfxVisualEffectsTest_Update` (0x0038_f7f8) against the generated body.
    #[test]
    fn vfx_visual_effects_test_update_0038f7f8_matches() {
        check(
            ee::s12::f_0038f7f8__generated,
            vfx_visual_effects_test_update_0038f7f8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x38f7f8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `GraphicsMan_SetDefaultDisplayMode` (0x0039_3fa8) against the generated body.
    #[test]
    fn graphics_man_set_default_display_mode_00393fa8_matches() {
        check(
            ee::s13::f_00393fa8__generated,
            graphics_man_set_default_display_mode_00393fa8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x393fa8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `GraphicsMan_IsProgressive` (0x0039_3fb0) against the generated body.
    #[test]
    fn graphics_man_is_progressive_00393fb0_matches() {
        check(
            ee::s13::f_00393fb0__generated,
            graphics_man_is_progressive_00393fb0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x393fb0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `GraphicsMan_v16` (0x0039_4008) against the generated body.
    #[test]
    fn graphics_man_v16_00394008_matches() {
        check(
            ee::s13::f_00394008__generated,
            graphics_man_v16_00394008,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x394008, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }
}
