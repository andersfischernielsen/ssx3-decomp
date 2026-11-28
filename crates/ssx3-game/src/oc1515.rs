//! Staging module oc1515 (wave wave276): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1515`.

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
    (0x0036_09e0, world_obj_v38_003609e0, "WorldObj_v38"),
    (0x0036_09e8, world_obj_v39_003609e8, "WorldObj_v39"),
    (0x0036_09f0, world_obj_v40_003609f0, "WorldObj_v40"),
    (0x0036_09f8, world_obj_v41_003609f8, "WorldObj_v41"),
    (
        0x0036_0a00,
        world_obj_get_contact_velocity_00360a00,
        "WorldObj_GetContactVelocity",
    ),
    (
        0x0036_0a08,
        world_obj_apply_impulse_00360a08,
        "WorldObj_ApplyImpulse",
    ),
    (0x0036_0a10, world_obj_v44_00360a10, "WorldObj_v44"),
    (0x0036_0a18, world_obj_v45_00360a18, "WorldObj_v45"),
    (0x0036_0a20, world_obj_v46_00360a20, "WorldObj_v46"),
    (
        0x0036_0a50,
        dead_node_kill_restorable_00360a50,
        "DeadNode_KillRestorable",
    ),
    (0x0036_0a58, dead_node_kill_00360a58, "DeadNode_Kill"),
    (
        0x0036_0a60,
        dead_node_handle_event_00360a60,
        "DeadNode_HandleEvent",
    ),
    (
        0x0036_0a68,
        dead_fade_handle_event_00360a68,
        "DeadFade_HandleEvent",
    ),
    (
        0x0036_0a98,
        hide_kill_restorable_00360a98,
        "Hide_KillRestorable",
    ),
    (0x0036_0aa0, hide_kill_00360aa0, "Hide_Kill"),
    (0x0036_0aa8, hide_handle_event_00360aa8, "Hide_HandleEvent"),
    (0x0036_0ab0, hide_v40_00360ab0, "Hide_v40"),
    (
        0x0036_0ae0,
        restore_node_kill_restorable_00360ae0,
        "RestoreNode_KillRestorable",
    ),
    (
        0x0036_0ae8,
        restore_node_handle_event_00360ae8,
        "RestoreNode_HandleEvent",
    ),
    (0x0036_0af0, boost_v49_00360af0, "Boost_v49"),
    (0x0036_0af8, boost_v50_00360af8, "Boost_v50"),
    (
        0x0036_0b30,
        transform_modifier_update_00360b30,
        "TransformModifier_Update",
    ),
    (
        0x0036_0b38,
        transform_modifier_update_transform_00360b38,
        "TransformModifier_UpdateTransform",
    ),
    (
        0x0036_0b40,
        transform_modifier_post_update_00360b40,
        "TransformModifier_PostUpdate",
    ),
    (
        0x0036_0b48,
        transform_modifier_handle_event_00360b48,
        "TransformModifier_HandleEvent",
    ),
    (
        0x0036_0b50,
        transform_modifier_is_transform_finished_00360b50,
        "TransformModifier_IsTransformFinished",
    ),
    (
        0x0036_0b58,
        transform_modifier_clear_transform_finished_00360b58,
        "TransformModifier_ClearTransformFinished",
    ),
    (
        0x0036_0b60,
        transform_modifier_v08_00360b60,
        "TransformModifier_v08",
    ),
    (
        0x0036_0b68,
        transform_modifier_v09_00360b68,
        "TransformModifier_v09",
    ),
    (
        0x0036_0b70,
        transform_modifier_v10_00360b70,
        "TransformModifier_v10",
    ),
    (
        0x0036_0b78,
        transform_modifier_render_00360b78,
        "TransformModifier_Render",
    ),
    (
        0x0036_0b80,
        transform_modifier_get_transform_bounds_00360b80,
        "TransformModifier_GetTransformBounds",
    ),
    (
        0x0036_0b88,
        transform_modifier_set_transform_bounds_00360b88,
        "TransformModifier_SetTransformBounds",
    ),
    (
        0x0036_0b90,
        transform_modifier_get_transform_radius_00360b90,
        "TransformModifier_GetTransformRadius",
    ),
    (
        0x0036_0ba0,
        transform_modifier_set_transform_radius_00360ba0,
        "TransformModifier_SetTransformRadius",
    ),
    (
        0x0036_0ba8,
        transform_modifier_v16_00360ba8,
        "TransformModifier_v16",
    ),
    (
        0x0036_0bb0,
        transform_modifier_has_transform_changed_00360bb0,
        "TransformModifier_HasTransformChanged",
    ),
    (
        0x0036_0bb8,
        transform_modifier_get_matrix_00360bb8,
        "TransformModifier_GetMatrix",
    ),
    (
        0x0036_0bc0,
        transform_modifier_v19_00360bc0,
        "TransformModifier_v19",
    ),
    (
        0x0036_0bc8,
        transform_modifier_v20_00360bc8,
        "TransformModifier_v20",
    ),
    (
        0x0036_0bd0,
        transform_modifier_v21_00360bd0,
        "TransformModifier_v21",
    ),
    (
        0x0036_0bd8,
        transform_modifier_v22_00360bd8,
        "TransformModifier_v22",
    ),
    (
        0x0036_0be0,
        transform_modifier_v23_00360be0,
        "TransformModifier_v23",
    ),
    (
        0x0036_0be8,
        transform_modifier_v24_00360be8,
        "TransformModifier_v24",
    ),
    (
        0x0036_0bf0,
        transform_modifier_v25_00360bf0,
        "TransformModifier_v25",
    ),
];

/// `WorldObj_v38` (0x0036_09e0): straight-line leaf. Mechanical port of the generated body.
pub fn world_obj_v38_003609e0(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `WorldObj_v39` (0x0036_09e8): straight-line leaf. Mechanical port of the generated body.
pub fn world_obj_v39_003609e8(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `WorldObj_v40` (0x0036_09f0): straight-line leaf. Mechanical port of the generated body.
pub fn world_obj_v40_003609f0(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `WorldObj_v41` (0x0036_09f8): straight-line leaf. Mechanical port of the generated body.
pub fn world_obj_v41_003609f8(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `WorldObj_GetContactVelocity` (0x0036_0a00): straight-line leaf. Mechanical port of the generated body.
pub fn world_obj_get_contact_velocity_00360a00(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `WorldObj_ApplyImpulse` (0x0036_0a08): straight-line leaf. Mechanical port of the generated body.
pub fn world_obj_apply_impulse_00360a08(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `WorldObj_v44` (0x0036_0a10): straight-line leaf. Mechanical port of the generated body.
pub fn world_obj_v44_00360a10(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `WorldObj_v45` (0x0036_0a18): straight-line leaf. Mechanical port of the generated body.
pub fn world_obj_v45_00360a18(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `WorldObj_v46` (0x0036_0a20): straight-line leaf. Mechanical port of the generated body.
pub fn world_obj_v46_00360a20(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `DeadNode_KillRestorable` (0x0036_0a50): straight-line leaf. Mechanical port of the generated body.
pub fn dead_node_kill_restorable_00360a50(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `DeadNode_Kill` (0x0036_0a58): straight-line leaf. Mechanical port of the generated body.
pub fn dead_node_kill_00360a58(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `DeadNode_HandleEvent` (0x0036_0a60): straight-line leaf. Mechanical port of the generated body.
pub fn dead_node_handle_event_00360a60(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `DeadFade_HandleEvent` (0x0036_0a68): straight-line leaf. Mechanical port of the generated body.
pub fn dead_fade_handle_event_00360a68(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `Hide_KillRestorable` (0x0036_0a98): straight-line leaf. Mechanical port of the generated body.
pub fn hide_kill_restorable_00360a98(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `Hide_Kill` (0x0036_0aa0): straight-line leaf. Mechanical port of the generated body.
pub fn hide_kill_00360aa0(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `Hide_HandleEvent` (0x0036_0aa8): straight-line leaf. Mechanical port of the generated body.
pub fn hide_handle_event_00360aa8(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `Hide_v40` (0x0036_0ab0): straight-line leaf. Mechanical port of the generated body.
pub fn hide_v40_00360ab0(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `RestoreNode_KillRestorable` (0x0036_0ae0): straight-line leaf. Mechanical port of the generated body.
pub fn restore_node_kill_restorable_00360ae0(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `RestoreNode_HandleEvent` (0x0036_0ae8): straight-line leaf. Mechanical port of the generated body.
pub fn restore_node_handle_event_00360ae8(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `Boost_v49` (0x0036_0af0): straight-line leaf. Mechanical port of the generated body.
pub fn boost_v49_00360af0(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `Boost_v50` (0x0036_0af8): straight-line leaf. Mechanical port of the generated body.
pub fn boost_v50_00360af8(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `TransformModifier_Update` (0x0036_0b30): straight-line leaf. Mechanical port of the generated body.
pub fn transform_modifier_update_00360b30(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `TransformModifier_UpdateTransform` (0x0036_0b38): straight-line leaf. Mechanical port of the generated body.
pub fn transform_modifier_update_transform_00360b38(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `TransformModifier_PostUpdate` (0x0036_0b40): straight-line leaf. Mechanical port of the generated body.
pub fn transform_modifier_post_update_00360b40(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `TransformModifier_HandleEvent` (0x0036_0b48): straight-line leaf. Mechanical port of the generated body.
pub fn transform_modifier_handle_event_00360b48(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `TransformModifier_IsTransformFinished` (0x0036_0b50): straight-line leaf. Mechanical port of the generated body.
pub fn transform_modifier_is_transform_finished_00360b50(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `TransformModifier_ClearTransformFinished` (0x0036_0b58): straight-line leaf. Mechanical port of the generated body.
pub fn transform_modifier_clear_transform_finished_00360b58(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `TransformModifier_v08` (0x0036_0b60): straight-line leaf. Mechanical port of the generated body.
pub fn transform_modifier_v08_00360b60(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0x1;
    ret(c);
}

/// `TransformModifier_v09` (0x0036_0b68): straight-line leaf. Mechanical port of the generated body.
pub fn transform_modifier_v09_00360b68(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0x1;
    ret(c);
}

/// `TransformModifier_v10` (0x0036_0b70): straight-line leaf. Mechanical port of the generated body.
pub fn transform_modifier_v10_00360b70(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `TransformModifier_Render` (0x0036_0b78): straight-line leaf. Mechanical port of the generated body.
pub fn transform_modifier_render_00360b78(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `TransformModifier_GetTransformBounds` (0x0036_0b80): straight-line leaf. Mechanical port of the generated body.
pub fn transform_modifier_get_transform_bounds_00360b80(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `TransformModifier_SetTransformBounds` (0x0036_0b88): straight-line leaf. Mechanical port of the generated body.
pub fn transform_modifier_set_transform_bounds_00360b88(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `TransformModifier_GetTransformRadius` (0x0036_0b90): straight-line leaf. Mechanical port of the generated body.
pub fn transform_modifier_get_transform_radius_00360b90(c: &mut Cpu) {
    c.cycles += 3;
    c.mtc1(0, 0);
    ret(c);
}

/// `TransformModifier_SetTransformRadius` (0x0036_0ba0): straight-line leaf. Mechanical port of the generated body.
pub fn transform_modifier_set_transform_radius_00360ba0(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `TransformModifier_v16` (0x0036_0ba8): straight-line leaf. Mechanical port of the generated body.
pub fn transform_modifier_v16_00360ba8(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `TransformModifier_HasTransformChanged` (0x0036_0bb0): straight-line leaf. Mechanical port of the generated body.
pub fn transform_modifier_has_transform_changed_00360bb0(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `TransformModifier_GetMatrix` (0x0036_0bb8): straight-line leaf. Mechanical port of the generated body.
pub fn transform_modifier_get_matrix_00360bb8(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `TransformModifier_v19` (0x0036_0bc0): straight-line leaf. Mechanical port of the generated body.
pub fn transform_modifier_v19_00360bc0(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `TransformModifier_v20` (0x0036_0bc8): straight-line leaf. Mechanical port of the generated body.
pub fn transform_modifier_v20_00360bc8(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `TransformModifier_v21` (0x0036_0bd0): straight-line leaf. Mechanical port of the generated body.
pub fn transform_modifier_v21_00360bd0(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `TransformModifier_v22` (0x0036_0bd8): straight-line leaf. Mechanical port of the generated body.
pub fn transform_modifier_v22_00360bd8(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `TransformModifier_v23` (0x0036_0be0): straight-line leaf. Mechanical port of the generated body.
pub fn transform_modifier_v23_00360be0(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `TransformModifier_v24` (0x0036_0be8): straight-line leaf. Mechanical port of the generated body.
pub fn transform_modifier_v24_00360be8(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `TransformModifier_v25` (0x0036_0bf0): straight-line leaf. Mechanical port of the generated body.
pub fn transform_modifier_v25_00360bf0(c: &mut Cpu) {
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

    /// `WorldObj_v38` (0x0036_09e0) against the generated body.
    #[test]
    fn world_obj_v38_003609e0_matches() {
        check(
            ee::s12::f_003609e0__generated,
            world_obj_v38_003609e0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x3609e0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `WorldObj_v39` (0x0036_09e8) against the generated body.
    #[test]
    fn world_obj_v39_003609e8_matches() {
        check(
            ee::s12::f_003609e8__generated,
            world_obj_v39_003609e8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x3609e8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `WorldObj_v40` (0x0036_09f0) against the generated body.
    #[test]
    fn world_obj_v40_003609f0_matches() {
        check(
            ee::s12::f_003609f0__generated,
            world_obj_v40_003609f0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x3609f0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `WorldObj_v41` (0x0036_09f8) against the generated body.
    #[test]
    fn world_obj_v41_003609f8_matches() {
        check(
            ee::s12::f_003609f8__generated,
            world_obj_v41_003609f8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x3609f8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `WorldObj_GetContactVelocity` (0x0036_0a00) against the generated body.
    #[test]
    fn world_obj_get_contact_velocity_00360a00_matches() {
        check(
            ee::s12::f_00360a00__generated,
            world_obj_get_contact_velocity_00360a00,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x360a00, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `WorldObj_ApplyImpulse` (0x0036_0a08) against the generated body.
    #[test]
    fn world_obj_apply_impulse_00360a08_matches() {
        check(
            ee::s12::f_00360a08__generated,
            world_obj_apply_impulse_00360a08,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x360a08, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `WorldObj_v44` (0x0036_0a10) against the generated body.
    #[test]
    fn world_obj_v44_00360a10_matches() {
        check(
            ee::s12::f_00360a10__generated,
            world_obj_v44_00360a10,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x360a10, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `WorldObj_v45` (0x0036_0a18) against the generated body.
    #[test]
    fn world_obj_v45_00360a18_matches() {
        check(
            ee::s12::f_00360a18__generated,
            world_obj_v45_00360a18,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x360a18, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `WorldObj_v46` (0x0036_0a20) against the generated body.
    #[test]
    fn world_obj_v46_00360a20_matches() {
        check(
            ee::s12::f_00360a20__generated,
            world_obj_v46_00360a20,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x360a20, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `DeadNode_KillRestorable` (0x0036_0a50) against the generated body.
    #[test]
    fn dead_node_kill_restorable_00360a50_matches() {
        check(
            ee::s12::f_00360a50__generated,
            dead_node_kill_restorable_00360a50,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x360a50, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `DeadNode_Kill` (0x0036_0a58) against the generated body.
    #[test]
    fn dead_node_kill_00360a58_matches() {
        check(
            ee::s12::f_00360a58__generated,
            dead_node_kill_00360a58,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x360a58, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `DeadNode_HandleEvent` (0x0036_0a60) against the generated body.
    #[test]
    fn dead_node_handle_event_00360a60_matches() {
        check(
            ee::s12::f_00360a60__generated,
            dead_node_handle_event_00360a60,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x360a60, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `DeadFade_HandleEvent` (0x0036_0a68) against the generated body.
    #[test]
    fn dead_fade_handle_event_00360a68_matches() {
        check(
            ee::s12::f_00360a68__generated,
            dead_fade_handle_event_00360a68,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x360a68, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `Hide_KillRestorable` (0x0036_0a98) against the generated body.
    #[test]
    fn hide_kill_restorable_00360a98_matches() {
        check(
            ee::s12::f_00360a98__generated,
            hide_kill_restorable_00360a98,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x360a98, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `Hide_Kill` (0x0036_0aa0) against the generated body.
    #[test]
    fn hide_kill_00360aa0_matches() {
        check(
            ee::s12::f_00360aa0__generated,
            hide_kill_00360aa0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x360aa0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `Hide_HandleEvent` (0x0036_0aa8) against the generated body.
    #[test]
    fn hide_handle_event_00360aa8_matches() {
        check(
            ee::s12::f_00360aa8__generated,
            hide_handle_event_00360aa8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x360aa8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `Hide_v40` (0x0036_0ab0) against the generated body.
    #[test]
    fn hide_v40_00360ab0_matches() {
        check(
            ee::s12::f_00360ab0__generated,
            hide_v40_00360ab0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x360ab0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `RestoreNode_KillRestorable` (0x0036_0ae0) against the generated body.
    #[test]
    fn restore_node_kill_restorable_00360ae0_matches() {
        check(
            ee::s12::f_00360ae0__generated,
            restore_node_kill_restorable_00360ae0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x360ae0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `RestoreNode_HandleEvent` (0x0036_0ae8) against the generated body.
    #[test]
    fn restore_node_handle_event_00360ae8_matches() {
        check(
            ee::s12::f_00360ae8__generated,
            restore_node_handle_event_00360ae8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x360ae8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `Boost_v49` (0x0036_0af0) against the generated body.
    #[test]
    fn boost_v49_00360af0_matches() {
        check(
            ee::s12::f_00360af0__generated,
            boost_v49_00360af0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x360af0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `Boost_v50` (0x0036_0af8) against the generated body.
    #[test]
    fn boost_v50_00360af8_matches() {
        check(
            ee::s12::f_00360af8__generated,
            boost_v50_00360af8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x360af8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `TransformModifier_Update` (0x0036_0b30) against the generated body.
    #[test]
    fn transform_modifier_update_00360b30_matches() {
        check(
            ee::s12::f_00360b30__generated,
            transform_modifier_update_00360b30,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x360b30, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `TransformModifier_UpdateTransform` (0x0036_0b38) against the generated body.
    #[test]
    fn transform_modifier_update_transform_00360b38_matches() {
        check(
            ee::s12::f_00360b38__generated,
            transform_modifier_update_transform_00360b38,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x360b38, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `TransformModifier_PostUpdate` (0x0036_0b40) against the generated body.
    #[test]
    fn transform_modifier_post_update_00360b40_matches() {
        check(
            ee::s12::f_00360b40__generated,
            transform_modifier_post_update_00360b40,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x360b40, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `TransformModifier_HandleEvent` (0x0036_0b48) against the generated body.
    #[test]
    fn transform_modifier_handle_event_00360b48_matches() {
        check(
            ee::s12::f_00360b48__generated,
            transform_modifier_handle_event_00360b48,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x360b48, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `TransformModifier_IsTransformFinished` (0x0036_0b50) against the generated body.
    #[test]
    fn transform_modifier_is_transform_finished_00360b50_matches() {
        check(
            ee::s12::f_00360b50__generated,
            transform_modifier_is_transform_finished_00360b50,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x360b50, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `TransformModifier_ClearTransformFinished` (0x0036_0b58) against the generated body.
    #[test]
    fn transform_modifier_clear_transform_finished_00360b58_matches() {
        check(
            ee::s12::f_00360b58__generated,
            transform_modifier_clear_transform_finished_00360b58,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x360b58, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `TransformModifier_v08` (0x0036_0b60) against the generated body.
    #[test]
    fn transform_modifier_v08_00360b60_matches() {
        check(
            ee::s12::f_00360b60__generated,
            transform_modifier_v08_00360b60,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x360b60, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `TransformModifier_v09` (0x0036_0b68) against the generated body.
    #[test]
    fn transform_modifier_v09_00360b68_matches() {
        check(
            ee::s12::f_00360b68__generated,
            transform_modifier_v09_00360b68,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x360b68, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `TransformModifier_v10` (0x0036_0b70) against the generated body.
    #[test]
    fn transform_modifier_v10_00360b70_matches() {
        check(
            ee::s12::f_00360b70__generated,
            transform_modifier_v10_00360b70,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x360b70, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `TransformModifier_Render` (0x0036_0b78) against the generated body.
    #[test]
    fn transform_modifier_render_00360b78_matches() {
        check(
            ee::s12::f_00360b78__generated,
            transform_modifier_render_00360b78,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x360b78, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `TransformModifier_GetTransformBounds` (0x0036_0b80) against the generated body.
    #[test]
    fn transform_modifier_get_transform_bounds_00360b80_matches() {
        check(
            ee::s12::f_00360b80__generated,
            transform_modifier_get_transform_bounds_00360b80,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x360b80, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `TransformModifier_SetTransformBounds` (0x0036_0b88) against the generated body.
    #[test]
    fn transform_modifier_set_transform_bounds_00360b88_matches() {
        check(
            ee::s12::f_00360b88__generated,
            transform_modifier_set_transform_bounds_00360b88,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x360b88, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `TransformModifier_GetTransformRadius` (0x0036_0b90) against the generated body.
    #[test]
    fn transform_modifier_get_transform_radius_00360b90_matches() {
        check(
            ee::s12::f_00360b90__generated,
            transform_modifier_get_transform_radius_00360b90,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x360b90, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `TransformModifier_SetTransformRadius` (0x0036_0ba0) against the generated body.
    #[test]
    fn transform_modifier_set_transform_radius_00360ba0_matches() {
        check(
            ee::s12::f_00360ba0__generated,
            transform_modifier_set_transform_radius_00360ba0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x360ba0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `TransformModifier_v16` (0x0036_0ba8) against the generated body.
    #[test]
    fn transform_modifier_v16_00360ba8_matches() {
        check(
            ee::s12::f_00360ba8__generated,
            transform_modifier_v16_00360ba8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x360ba8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `TransformModifier_HasTransformChanged` (0x0036_0bb0) against the generated body.
    #[test]
    fn transform_modifier_has_transform_changed_00360bb0_matches() {
        check(
            ee::s12::f_00360bb0__generated,
            transform_modifier_has_transform_changed_00360bb0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x360bb0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `TransformModifier_GetMatrix` (0x0036_0bb8) against the generated body.
    #[test]
    fn transform_modifier_get_matrix_00360bb8_matches() {
        check(
            ee::s12::f_00360bb8__generated,
            transform_modifier_get_matrix_00360bb8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x360bb8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `TransformModifier_v19` (0x0036_0bc0) against the generated body.
    #[test]
    fn transform_modifier_v19_00360bc0_matches() {
        check(
            ee::s12::f_00360bc0__generated,
            transform_modifier_v19_00360bc0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x360bc0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `TransformModifier_v20` (0x0036_0bc8) against the generated body.
    #[test]
    fn transform_modifier_v20_00360bc8_matches() {
        check(
            ee::s12::f_00360bc8__generated,
            transform_modifier_v20_00360bc8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x360bc8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `TransformModifier_v21` (0x0036_0bd0) against the generated body.
    #[test]
    fn transform_modifier_v21_00360bd0_matches() {
        check(
            ee::s12::f_00360bd0__generated,
            transform_modifier_v21_00360bd0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x360bd0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `TransformModifier_v22` (0x0036_0bd8) against the generated body.
    #[test]
    fn transform_modifier_v22_00360bd8_matches() {
        check(
            ee::s12::f_00360bd8__generated,
            transform_modifier_v22_00360bd8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x360bd8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `TransformModifier_v23` (0x0036_0be0) against the generated body.
    #[test]
    fn transform_modifier_v23_00360be0_matches() {
        check(
            ee::s12::f_00360be0__generated,
            transform_modifier_v23_00360be0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x360be0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `TransformModifier_v24` (0x0036_0be8) against the generated body.
    #[test]
    fn transform_modifier_v24_00360be8_matches() {
        check(
            ee::s12::f_00360be8__generated,
            transform_modifier_v24_00360be8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x360be8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `TransformModifier_v25` (0x0036_0bf0) against the generated body.
    #[test]
    fn transform_modifier_v25_00360bf0_matches() {
        check(
            ee::s12::f_00360bf0__generated,
            transform_modifier_v25_00360bf0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x360bf0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }
}
