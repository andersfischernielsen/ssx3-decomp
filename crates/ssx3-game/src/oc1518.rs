//! Staging module oc1518 (wave wave276): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1518`.

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
        0x0039_4010,
        graphics_man_try_acquire_frame_00394010,
        "GraphicsMan_TryAcquireFrame",
    ),
    (
        0x0039_4018,
        graphics_man_is_idle_00394018,
        "GraphicsMan_IsIdle",
    ),
    (
        0x0039_42c0,
        graphics_man_release_texture_ref_003942c0,
        "GraphicsMan_ReleaseTextureRef",
    ),
    (
        0x0039_42c8,
        graphics_man_draw_transition_003942c8,
        "GraphicsMan_DrawTransition",
    ),
    (0x0039_42d0, graphics_man_v62_003942d0, "GraphicsMan_v62"),
    (
        0x0039_42d8,
        graphics_man_get_color_mask_003942d8,
        "GraphicsMan_GetColorMask",
    ),
    (0x0039_42e0, graphics_man_v65_003942e0, "GraphicsMan_v65"),
    (
        0x0039_45b8,
        graphics_man_clear_lights_003945b8,
        "GraphicsMan_ClearLights",
    ),
    (
        0x0039_45c0,
        graphics_man_add_light_003945c0,
        "GraphicsMan_AddLight",
    ),
    (0x0039_45c8, graphics_man_v74_003945c8, "GraphicsMan_v74"),
    (0x0039_45d0, graphics_man_v88_003945d0, "GraphicsMan_v88"),
    (
        0x0039_45d8,
        graphics_man_draw_shadow_volume_003945d8,
        "GraphicsMan_DrawShadowVolume",
    ),
    (
        0x0039_45e0,
        graphics_man_draw_board_wake_003945e0,
        "GraphicsMan_DrawBoardWake",
    ),
    (0x0039_45e8, graphics_man_v98_003945e8, "GraphicsMan_v98"),
    (
        0x0039_4610,
        graphics_man_draw_skinned_model_shadow_00394610,
        "GraphicsMan_DrawSkinnedModelShadow",
    ),
    (0x0039_4618, graphics_man_v102_00394618, "GraphicsMan_v102"),
    (0x0039_4620, graphics_man_v103_00394620, "GraphicsMan_v103"),
    (0x0039_4628, graphics_man_v104_00394628, "GraphicsMan_v104"),
    (0x0039_4630, graphics_man_v105_00394630, "GraphicsMan_v105"),
    (0x0039_4638, graphics_man_v106_00394638, "GraphicsMan_v106"),
    (0x0039_4640, graphics_man_v107_00394640, "GraphicsMan_v107"),
    (0x0039_4648, graphics_man_v108_00394648, "GraphicsMan_v108"),
    (0x0039_4650, graphics_man_v109_00394650, "GraphicsMan_v109"),
    (0x0039_4658, graphics_man_v110_00394658, "GraphicsMan_v110"),
    (0x0039_4660, graphics_man_v111_00394660, "GraphicsMan_v111"),
    (0x0039_4668, graphics_man_v112_00394668, "GraphicsMan_v112"),
    (0x0039_4670, graphics_man_v113_00394670, "GraphicsMan_v113"),
    (
        0x0039_4678,
        graphics_man_get_flip_count_00394678,
        "GraphicsMan_GetFlipCount",
    ),
    (
        0x0039_4680,
        graphics_man_update_lens_effects_00394680,
        "GraphicsMan_UpdateLensEffects",
    ),
    (
        0x0039_4688,
        graphics_man_add_view_00394688,
        "GraphicsMan_AddView",
    ),
    (
        0x0039_46e8,
        graphics_man_remove_interrupt_handlers_003946e8,
        "GraphicsMan_RemoveInterruptHandlers",
    ),
    (0x0039_4a44, power_curve_one_00394a44, "PowerCurve_One"),
    (
        0x0039_4a54,
        power_curve_linear_00394a54,
        "PowerCurve_Linear",
    ),
    (0x0039_4a5c, power_curve_pow2_00394a5c, "PowerCurve_Pow2"),
    (0x0039_4a64, power_curve_pow3_00394a64, "PowerCurve_Pow3"),
    (0x0039_4a70, power_curve_pow4_00394a70, "PowerCurve_Pow4"),
    (
        0x0039_4b68,
        grid_mesh_base_set_scale_00394b68,
        "GridMeshBase_SetScale",
    ),
    (
        0x0039_4b78,
        bezier_man_base_v04_00394b78,
        "BezierManBase_v04",
    ),
    (
        0x0039_4b80,
        bezier_man_base_v05_00394b80,
        "BezierManBase_v05",
    ),
    (
        0x0039_4b88,
        bezier_man_base_v19_00394b88,
        "BezierManBase_v19",
    ),
    (
        0x0039_4b90,
        bezier_man_base_v20_00394b90,
        "BezierManBase_v20",
    ),
    (
        0x0039_4bc0,
        ps2_bezier_man_set_lod_near_dist_00394bc0,
        "PS2BezierMan_SetLodNearDist",
    ),
    (
        0x0039_4bc8,
        ps2_bezier_man_set_lod_far_dist_00394bc8,
        "PS2BezierMan_SetLodFarDist",
    ),
    (
        0x0039_4bd0,
        ps2_bezier_man_get_lod_near_dist_00394bd0,
        "PS2BezierMan_GetLodNearDist",
    ),
    (
        0x0039_4bd8,
        ps2_bezier_man_get_lod_far_dist_00394bd8,
        "PS2BezierMan_GetLodFarDist",
    ),
];

/// `GraphicsMan_TryAcquireFrame` (0x0039_4010): straight-line leaf. Mechanical port of the generated body.
pub fn graphics_man_try_acquire_frame_00394010(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0x1;
    ret(c);
}

/// `GraphicsMan_IsIdle` (0x0039_4018): straight-line leaf. Mechanical port of the generated body.
pub fn graphics_man_is_idle_00394018(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0x1;
    ret(c);
}

/// `GraphicsMan_ReleaseTextureRef` (0x0039_42c0): straight-line leaf. Mechanical port of the generated body.
pub fn graphics_man_release_texture_ref_003942c0(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `GraphicsMan_DrawTransition` (0x0039_42c8): straight-line leaf. Mechanical port of the generated body.
pub fn graphics_man_draw_transition_003942c8(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `GraphicsMan_v62` (0x0039_42d0): straight-line leaf. Mechanical port of the generated body.
pub fn graphics_man_v62_003942d0(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `GraphicsMan_GetColorMask` (0x0039_42d8): straight-line leaf. Mechanical port of the generated body.
pub fn graphics_man_get_color_mask_003942d8(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0xed8));
    ret(c);
}

/// `GraphicsMan_v65` (0x0039_42e0): straight-line leaf. Mechanical port of the generated body.
pub fn graphics_man_v65_003942e0(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `GraphicsMan_ClearLights` (0x0039_45b8): straight-line leaf. Mechanical port of the generated body.
pub fn graphics_man_clear_lights_003945b8(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `GraphicsMan_AddLight` (0x0039_45c0): straight-line leaf. Mechanical port of the generated body.
pub fn graphics_man_add_light_003945c0(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `GraphicsMan_v74` (0x0039_45c8): straight-line leaf. Mechanical port of the generated body.
pub fn graphics_man_v74_003945c8(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `GraphicsMan_v88` (0x0039_45d0): straight-line leaf. Mechanical port of the generated body.
pub fn graphics_man_v88_003945d0(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `GraphicsMan_DrawShadowVolume` (0x0039_45d8): straight-line leaf. Mechanical port of the generated body.
pub fn graphics_man_draw_shadow_volume_003945d8(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `GraphicsMan_DrawBoardWake` (0x0039_45e0): straight-line leaf. Mechanical port of the generated body.
pub fn graphics_man_draw_board_wake_003945e0(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `GraphicsMan_v98` (0x0039_45e8): straight-line leaf. Mechanical port of the generated body.
pub fn graphics_man_v98_003945e8(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `GraphicsMan_DrawSkinnedModelShadow` (0x0039_4610): straight-line leaf. Mechanical port of the generated body.
pub fn graphics_man_draw_skinned_model_shadow_00394610(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `GraphicsMan_v102` (0x0039_4618): straight-line leaf. Mechanical port of the generated body.
pub fn graphics_man_v102_00394618(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `GraphicsMan_v103` (0x0039_4620): straight-line leaf. Mechanical port of the generated body.
pub fn graphics_man_v103_00394620(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `GraphicsMan_v104` (0x0039_4628): straight-line leaf. Mechanical port of the generated body.
pub fn graphics_man_v104_00394628(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `GraphicsMan_v105` (0x0039_4630): straight-line leaf. Mechanical port of the generated body.
pub fn graphics_man_v105_00394630(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `GraphicsMan_v106` (0x0039_4638): straight-line leaf. Mechanical port of the generated body.
pub fn graphics_man_v106_00394638(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `GraphicsMan_v107` (0x0039_4640): straight-line leaf. Mechanical port of the generated body.
pub fn graphics_man_v107_00394640(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `GraphicsMan_v108` (0x0039_4648): straight-line leaf. Mechanical port of the generated body.
pub fn graphics_man_v108_00394648(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `GraphicsMan_v109` (0x0039_4650): straight-line leaf. Mechanical port of the generated body.
pub fn graphics_man_v109_00394650(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `GraphicsMan_v110` (0x0039_4658): straight-line leaf. Mechanical port of the generated body.
pub fn graphics_man_v110_00394658(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `GraphicsMan_v111` (0x0039_4660): straight-line leaf. Mechanical port of the generated body.
pub fn graphics_man_v111_00394660(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `GraphicsMan_v112` (0x0039_4668): straight-line leaf. Mechanical port of the generated body.
pub fn graphics_man_v112_00394668(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `GraphicsMan_v113` (0x0039_4670): straight-line leaf. Mechanical port of the generated body.
pub fn graphics_man_v113_00394670(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `GraphicsMan_GetFlipCount` (0x0039_4678): straight-line leaf. Mechanical port of the generated body.
pub fn graphics_man_get_flip_count_00394678(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `GraphicsMan_UpdateLensEffects` (0x0039_4680): straight-line leaf. Mechanical port of the generated body.
pub fn graphics_man_update_lens_effects_00394680(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `GraphicsMan_AddView` (0x0039_4688): straight-line leaf. Mechanical port of the generated body.
pub fn graphics_man_add_view_00394688(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `GraphicsMan_RemoveInterruptHandlers` (0x0039_46e8): straight-line leaf. Mechanical port of the generated body.
pub fn graphics_man_remove_interrupt_handlers_003946e8(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `PowerCurve_One` (0x0039_4a44): straight-line leaf. Mechanical port of the generated body.
pub fn power_curve_one_00394a44(c: &mut Cpu) {
    c.cycles += 4;
    c.g[AT][0] = 0x3f800000;
    c.mtc1(1, 0);
    ret(c);
}

/// `PowerCurve_Linear` (0x0039_4a54): straight-line leaf. Mechanical port of the generated body.
pub fn power_curve_linear_00394a54(c: &mut Cpu) {
    c.cycles += 2;
    c.mov_s(0, 12);
    ret(c);
}

/// `PowerCurve_Pow2` (0x0039_4a5c): straight-line leaf. Mechanical port of the generated body.
pub fn power_curve_pow2_00394a5c(c: &mut Cpu) {
    c.cycles += 2;
    c.mul_s(0, 12, 12);
    ret(c);
}

/// `PowerCurve_Pow3` (0x0039_4a64): straight-line leaf. Mechanical port of the generated body.
pub fn power_curve_pow3_00394a64(c: &mut Cpu) {
    c.cycles += 3;
    c.mul_s(0, 12, 12);
    c.mul_s(0, 0, 12);
    ret(c);
}

/// `PowerCurve_Pow4` (0x0039_4a70): straight-line leaf. Mechanical port of the generated body.
pub fn power_curve_pow4_00394a70(c: &mut Cpu) {
    c.cycles += 4;
    c.mul_s(0, 12, 12);
    c.mul_s(0, 0, 12);
    c.mul_s(0, 0, 12);
    ret(c);
}

/// `GridMeshBase_SetScale` (0x0039_4b68): straight-line leaf. Mechanical port of the generated body.
pub fn grid_mesh_base_set_scale_00394b68(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `BezierManBase_v04` (0x0039_4b78): straight-line leaf. Mechanical port of the generated body.
pub fn bezier_man_base_v04_00394b78(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `BezierManBase_v05` (0x0039_4b80): straight-line leaf. Mechanical port of the generated body.
pub fn bezier_man_base_v05_00394b80(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `BezierManBase_v19` (0x0039_4b88): straight-line leaf: 1 store(s). Mechanical port of the generated body.
pub fn bezier_man_base_v19_00394b88(c: &mut Cpu) {
    c.cycles += 2;
    c.swc1(12, (c.g[A0][0] as u32));
    ret(c);
}

/// `BezierManBase_v20` (0x0039_4b90): straight-line leaf: 1 load(s). Mechanical port of the generated body.
pub fn bezier_man_base_v20_00394b90(c: &mut Cpu) {
    c.cycles += 2;
    c.lwc1(0, (c.g[A0][0] as u32));
    ret(c);
}

/// `PS2BezierMan_SetLodNearDist` (0x0039_4bc0): straight-line leaf: 1 store(s). Mechanical port of the generated body.
pub fn ps2_bezier_man_set_lod_near_dist_00394bc0(c: &mut Cpu) {
    c.cycles += 2;
    c.swc1(12, (c.g[A0][0] as u32).wrapping_add(0x33c));
    ret(c);
}

/// `PS2BezierMan_SetLodFarDist` (0x0039_4bc8): straight-line leaf: 1 store(s). Mechanical port of the generated body.
pub fn ps2_bezier_man_set_lod_far_dist_00394bc8(c: &mut Cpu) {
    c.cycles += 2;
    c.swc1(12, (c.g[A0][0] as u32).wrapping_add(0x340));
    ret(c);
}

/// `PS2BezierMan_GetLodNearDist` (0x0039_4bd0): straight-line leaf: 1 load(s). Mechanical port of the generated body.
pub fn ps2_bezier_man_get_lod_near_dist_00394bd0(c: &mut Cpu) {
    c.cycles += 2;
    c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(0x33c));
    ret(c);
}

/// `PS2BezierMan_GetLodFarDist` (0x0039_4bd8): straight-line leaf: 1 load(s). Mechanical port of the generated body.
pub fn ps2_bezier_man_get_lod_far_dist_00394bd8(c: &mut Cpu) {
    c.cycles += 2;
    c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(0x340));
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

    /// `GraphicsMan_TryAcquireFrame` (0x0039_4010) against the generated body.
    #[test]
    fn graphics_man_try_acquire_frame_00394010_matches() {
        check(
            ee::s13::f_00394010__generated,
            graphics_man_try_acquire_frame_00394010,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x394010, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `GraphicsMan_IsIdle` (0x0039_4018) against the generated body.
    #[test]
    fn graphics_man_is_idle_00394018_matches() {
        check(
            ee::s13::f_00394018__generated,
            graphics_man_is_idle_00394018,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x394018, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `GraphicsMan_ReleaseTextureRef` (0x0039_42c0) against the generated body.
    #[test]
    fn graphics_man_release_texture_ref_003942c0_matches() {
        check(
            ee::s13::f_003942c0__generated,
            graphics_man_release_texture_ref_003942c0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x3942c0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `GraphicsMan_DrawTransition` (0x0039_42c8) against the generated body.
    #[test]
    fn graphics_man_draw_transition_003942c8_matches() {
        check(
            ee::s13::f_003942c8__generated,
            graphics_man_draw_transition_003942c8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x3942c8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `GraphicsMan_v62` (0x0039_42d0) against the generated body.
    #[test]
    fn graphics_man_v62_003942d0_matches() {
        check(
            ee::s13::f_003942d0__generated,
            graphics_man_v62_003942d0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x3942d0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `GraphicsMan_GetColorMask` (0x0039_42d8) against the generated body.
    #[test]
    fn graphics_man_get_color_mask_003942d8_matches() {
        check(
            ee::s13::f_003942d8__generated,
            graphics_man_get_color_mask_003942d8,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3942d8, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `GraphicsMan_v65` (0x0039_42e0) against the generated body.
    #[test]
    fn graphics_man_v65_003942e0_matches() {
        check(
            ee::s13::f_003942e0__generated,
            graphics_man_v65_003942e0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x3942e0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `GraphicsMan_ClearLights` (0x0039_45b8) against the generated body.
    #[test]
    fn graphics_man_clear_lights_003945b8_matches() {
        check(
            ee::s13::f_003945b8__generated,
            graphics_man_clear_lights_003945b8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x3945b8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `GraphicsMan_AddLight` (0x0039_45c0) against the generated body.
    #[test]
    fn graphics_man_add_light_003945c0_matches() {
        check(
            ee::s13::f_003945c0__generated,
            graphics_man_add_light_003945c0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x3945c0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `GraphicsMan_v74` (0x0039_45c8) against the generated body.
    #[test]
    fn graphics_man_v74_003945c8_matches() {
        check(
            ee::s13::f_003945c8__generated,
            graphics_man_v74_003945c8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x3945c8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `GraphicsMan_v88` (0x0039_45d0) against the generated body.
    #[test]
    fn graphics_man_v88_003945d0_matches() {
        check(
            ee::s13::f_003945d0__generated,
            graphics_man_v88_003945d0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x3945d0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `GraphicsMan_DrawShadowVolume` (0x0039_45d8) against the generated body.
    #[test]
    fn graphics_man_draw_shadow_volume_003945d8_matches() {
        check(
            ee::s13::f_003945d8__generated,
            graphics_man_draw_shadow_volume_003945d8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x3945d8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `GraphicsMan_DrawBoardWake` (0x0039_45e0) against the generated body.
    #[test]
    fn graphics_man_draw_board_wake_003945e0_matches() {
        check(
            ee::s13::f_003945e0__generated,
            graphics_man_draw_board_wake_003945e0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x3945e0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `GraphicsMan_v98` (0x0039_45e8) against the generated body.
    #[test]
    fn graphics_man_v98_003945e8_matches() {
        check(
            ee::s13::f_003945e8__generated,
            graphics_man_v98_003945e8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x3945e8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `GraphicsMan_DrawSkinnedModelShadow` (0x0039_4610) against the generated body.
    #[test]
    fn graphics_man_draw_skinned_model_shadow_00394610_matches() {
        check(
            ee::s13::f_00394610__generated,
            graphics_man_draw_skinned_model_shadow_00394610,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x394610, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `GraphicsMan_v102` (0x0039_4618) against the generated body.
    #[test]
    fn graphics_man_v102_00394618_matches() {
        check(
            ee::s13::f_00394618__generated,
            graphics_man_v102_00394618,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x394618, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `GraphicsMan_v103` (0x0039_4620) against the generated body.
    #[test]
    fn graphics_man_v103_00394620_matches() {
        check(
            ee::s13::f_00394620__generated,
            graphics_man_v103_00394620,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x394620, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `GraphicsMan_v104` (0x0039_4628) against the generated body.
    #[test]
    fn graphics_man_v104_00394628_matches() {
        check(
            ee::s13::f_00394628__generated,
            graphics_man_v104_00394628,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x394628, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `GraphicsMan_v105` (0x0039_4630) against the generated body.
    #[test]
    fn graphics_man_v105_00394630_matches() {
        check(
            ee::s13::f_00394630__generated,
            graphics_man_v105_00394630,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x394630, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `GraphicsMan_v106` (0x0039_4638) against the generated body.
    #[test]
    fn graphics_man_v106_00394638_matches() {
        check(
            ee::s13::f_00394638__generated,
            graphics_man_v106_00394638,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x394638, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `GraphicsMan_v107` (0x0039_4640) against the generated body.
    #[test]
    fn graphics_man_v107_00394640_matches() {
        check(
            ee::s13::f_00394640__generated,
            graphics_man_v107_00394640,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x394640, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `GraphicsMan_v108` (0x0039_4648) against the generated body.
    #[test]
    fn graphics_man_v108_00394648_matches() {
        check(
            ee::s13::f_00394648__generated,
            graphics_man_v108_00394648,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x394648, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `GraphicsMan_v109` (0x0039_4650) against the generated body.
    #[test]
    fn graphics_man_v109_00394650_matches() {
        check(
            ee::s13::f_00394650__generated,
            graphics_man_v109_00394650,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x394650, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `GraphicsMan_v110` (0x0039_4658) against the generated body.
    #[test]
    fn graphics_man_v110_00394658_matches() {
        check(
            ee::s13::f_00394658__generated,
            graphics_man_v110_00394658,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x394658, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `GraphicsMan_v111` (0x0039_4660) against the generated body.
    #[test]
    fn graphics_man_v111_00394660_matches() {
        check(
            ee::s13::f_00394660__generated,
            graphics_man_v111_00394660,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x394660, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `GraphicsMan_v112` (0x0039_4668) against the generated body.
    #[test]
    fn graphics_man_v112_00394668_matches() {
        check(
            ee::s13::f_00394668__generated,
            graphics_man_v112_00394668,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x394668, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `GraphicsMan_v113` (0x0039_4670) against the generated body.
    #[test]
    fn graphics_man_v113_00394670_matches() {
        check(
            ee::s13::f_00394670__generated,
            graphics_man_v113_00394670,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x394670, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `GraphicsMan_GetFlipCount` (0x0039_4678) against the generated body.
    #[test]
    fn graphics_man_get_flip_count_00394678_matches() {
        check(
            ee::s13::f_00394678__generated,
            graphics_man_get_flip_count_00394678,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x394678, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `GraphicsMan_UpdateLensEffects` (0x0039_4680) against the generated body.
    #[test]
    fn graphics_man_update_lens_effects_00394680_matches() {
        check(
            ee::s13::f_00394680__generated,
            graphics_man_update_lens_effects_00394680,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x394680, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `GraphicsMan_AddView` (0x0039_4688) against the generated body.
    #[test]
    fn graphics_man_add_view_00394688_matches() {
        check(
            ee::s13::f_00394688__generated,
            graphics_man_add_view_00394688,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x394688, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `GraphicsMan_RemoveInterruptHandlers` (0x0039_46e8) against the generated body.
    #[test]
    fn graphics_man_remove_interrupt_handlers_003946e8_matches() {
        check(
            ee::s13::f_003946e8__generated,
            graphics_man_remove_interrupt_handlers_003946e8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x3946e8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `PowerCurve_One` (0x0039_4a44) against the generated body.
    #[test]
    fn power_curve_one_00394a44_matches() {
        check(
            ee::s13::f_00394a44__generated,
            power_curve_one_00394a44,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x394a44, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `PowerCurve_Linear` (0x0039_4a54) against the generated body.
    #[test]
    fn power_curve_linear_00394a54_matches() {
        check(
            ee::s13::f_00394a54__generated,
            power_curve_linear_00394a54,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x394a54, 0);
                c.f[12] = r.edge_f32();
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `PowerCurve_Pow2` (0x0039_4a5c) against the generated body.
    #[test]
    fn power_curve_pow2_00394a5c_matches() {
        check(
            ee::s13::f_00394a5c__generated,
            power_curve_pow2_00394a5c,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x394a5c, 0);
                c.f[12] = r.edge_f32();
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `PowerCurve_Pow3` (0x0039_4a64) against the generated body.
    #[test]
    fn power_curve_pow3_00394a64_matches() {
        check(
            ee::s13::f_00394a64__generated,
            power_curve_pow3_00394a64,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x394a64, 0);
                c.f[12] = r.edge_f32();
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `PowerCurve_Pow4` (0x0039_4a70) against the generated body.
    #[test]
    fn power_curve_pow4_00394a70_matches() {
        check(
            ee::s13::f_00394a70__generated,
            power_curve_pow4_00394a70,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x394a70, 0);
                c.f[12] = r.edge_f32();
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `GridMeshBase_SetScale` (0x0039_4b68) against the generated body.
    #[test]
    fn grid_mesh_base_set_scale_00394b68_matches() {
        check(
            ee::s13::f_00394b68__generated,
            grid_mesh_base_set_scale_00394b68,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x394b68, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `BezierManBase_v04` (0x0039_4b78) against the generated body.
    #[test]
    fn bezier_man_base_v04_00394b78_matches() {
        check(
            ee::s13::f_00394b78__generated,
            bezier_man_base_v04_00394b78,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x394b78, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `BezierManBase_v05` (0x0039_4b80) against the generated body.
    #[test]
    fn bezier_man_base_v05_00394b80_matches() {
        check(
            ee::s13::f_00394b80__generated,
            bezier_man_base_v05_00394b80,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x394b80, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `BezierManBase_v19` (0x0039_4b88) against the generated body.
    #[test]
    fn bezier_man_base_v19_00394b88_matches() {
        check(
            ee::s13::f_00394b88__generated,
            bezier_man_base_v19_00394b88,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x394b88, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.f[12] = r.edge_f32();
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `BezierManBase_v20` (0x0039_4b90) against the generated body.
    #[test]
    fn bezier_man_base_v20_00394b90_matches() {
        check(
            ee::s13::f_00394b90__generated,
            bezier_man_base_v20_00394b90,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x394b90, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `PS2BezierMan_SetLodNearDist` (0x0039_4bc0) against the generated body.
    #[test]
    fn ps2_bezier_man_set_lod_near_dist_00394bc0_matches() {
        check(
            ee::s13::f_00394bc0__generated,
            ps2_bezier_man_set_lod_near_dist_00394bc0,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x394bc0, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.f[12] = r.edge_f32();
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `PS2BezierMan_SetLodFarDist` (0x0039_4bc8) against the generated body.
    #[test]
    fn ps2_bezier_man_set_lod_far_dist_00394bc8_matches() {
        check(
            ee::s13::f_00394bc8__generated,
            ps2_bezier_man_set_lod_far_dist_00394bc8,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x394bc8, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.f[12] = r.edge_f32();
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `PS2BezierMan_GetLodNearDist` (0x0039_4bd0) against the generated body.
    #[test]
    fn ps2_bezier_man_get_lod_near_dist_00394bd0_matches() {
        check(
            ee::s13::f_00394bd0__generated,
            ps2_bezier_man_get_lod_near_dist_00394bd0,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x394bd0, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `PS2BezierMan_GetLodFarDist` (0x0039_4bd8) against the generated body.
    #[test]
    fn ps2_bezier_man_get_lod_far_dist_00394bd8_matches() {
        check(
            ee::s13::f_00394bd8__generated,
            ps2_bezier_man_get_lod_far_dist_00394bd8,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x394bd8, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }
}
