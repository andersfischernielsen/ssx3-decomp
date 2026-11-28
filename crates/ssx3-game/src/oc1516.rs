//! Staging module oc1516 (wave wave276): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1516`.

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
        0x0036_0bf8,
        transform_modifier_get_type_00360bf8,
        "TransformModifier_GetType",
    ),
    (
        0x0036_0c00,
        transform_modifier_save_00360c00,
        "TransformModifier_Save",
    ),
    (
        0x0036_0c58,
        boost_modifier_base_v02_00360c58,
        "BoostModifier_BaseV02",
    ),
    (
        0x0036_0c60,
        boost_modifier_base_handle_event_00360c60,
        "BoostModifier_BaseHandleEvent",
    ),
    (
        0x0036_0c68,
        boost_modifier_base_v04_00360c68,
        "BoostModifier_BaseV04",
    ),
    (
        0x0036_0c70,
        boost_modifier_base_get_type_00360c70,
        "BoostModifier_BaseGetType",
    ),
    (
        0x0036_0ce8,
        modifier_list_node_get_type_00360ce8,
        "ModifierListNode_GetType",
    ),
    (
        0x0036_0cf0,
        modifier_list_node_on_transfer_00360cf0,
        "ModifierListNode_OnTransfer",
    ),
    (
        0x0036_0cf8,
        modifier_list_node_save_00360cf8,
        "ModifierListNode_Save",
    ),
    (
        0x0036_0d00,
        particle_modifier_base_update_00360d00,
        "ParticleModifier_BaseUpdate",
    ),
    (
        0x0036_0d08,
        particle_modifier_base_render_00360d08,
        "ParticleModifier_BaseRender",
    ),
    (
        0x0036_0d10,
        particle_modifier_base_handle_event_00360d10,
        "ParticleModifier_BaseHandleEvent",
    ),
    (
        0x0036_0d20,
        particle_modifier_get_type_00360d20,
        "ParticleModifier_GetType",
    ),
    (
        0x0036_0d28,
        dynamic_particle_modifier_get_type_00360d28,
        "DynamicParticleModifier_GetType",
    ),
    (
        0x0036_0d38,
        halo_modifier_get_type_00360d38,
        "HaloModifier_GetType",
    ),
    (
        0x0036_0d90,
        rail_modifier_list_node_v02_00360d90,
        "RailModifierListNode_v02",
    ),
    (0x0036_0dc8, mod_obj_v47_00360dc8, "ModObj_v47"),
    (
        0x0036_0dd0,
        mod_obj_accepts_modifiers_00360dd0,
        "ModObj_AcceptsModifiers",
    ),
    (
        0x0036_1060,
        anim_object_base_v15_00361060,
        "AnimObject_BaseV15",
    ),
    (
        0x0036_1080,
        anim_object_base_v53_00361080,
        "AnimObject_BaseV53",
    ),
    (0x0036_1090, anim_object_v26_00361090, "AnimObject_v26"),
    (0x0036_1150, anim_object_v19_00361150, "AnimObject_v19"),
    (0x0036_1198, anim_object_v17_00361198, "AnimObject_v17"),
    (0x0036_1458, anim_teeter_v19_00361458, "AnimTeeter_v19"),
    (
        0x0036_14b8,
        boost_modifier_get_type_003614b8,
        "BoostModifier_GetType",
    ),
    (
        0x0036_14c0,
        boost_modifier_v09_003614c0,
        "BoostModifier_v09",
    ),
    (
        0x0036_14c8,
        boost_modifier_v10_003614c8,
        "BoostModifier_v10",
    ),
    (0x0036_14f8, debounce_v40_003614f8, "Debounce_v40"),
    (0x0036_1528, conveyor_v19_00361528, "Conveyor_v19"),
    (0x0036_1558, flex_bridge_v19_00361558, "FlexBridge_v19"),
    (0x0036_1560, flex_bridge_v38_00361560, "FlexBridge_v38"),
    (0x0036_1568, flex_bridge_v44_00361568, "FlexBridge_v44"),
    (0x0036_1570, flex_bridge_v45_00361570, "FlexBridge_v45"),
    (0x0036_1578, flex_rail_v47_00361578, "FlexRail_v47"),
    (0x0036_1580, flex_rail_v48_00361580, "FlexRail_v48"),
    (
        0x0036_1588,
        flex_rail_apply_impulse_00361588,
        "FlexRail_ApplyImpulse",
    ),
    (
        0x0036_1590,
        flex_rail_get_contact_velocity_00361590,
        "FlexRail_GetContactVelocity",
    ),
    (0x0036_1598, flex_rail_v49_00361598, "FlexRail_v49"),
    (0x0036_15a0, flex_rail_v46_003615a0, "FlexRail_v46"),
    (0x0036_1768, floating_v19_00361768, "Floating_v19"),
    (0x0036_1770, floating_v44_00361770, "Floating_v44"),
    (0x0036_17b0, floating_v26_003617b0, "Floating_v26"),
    (
        0x0036_18f8,
        position_modifier_update_003618f8,
        "PositionModifier_Update",
    ),
    (
        0x0036_1900,
        position_modifier_has_transform_changed_00361900,
        "PositionModifier_HasTransformChanged",
    ),
    (
        0x0036_1908,
        position_modifier_get_type_00361908,
        "PositionModifier_GetType",
    ),
];

/// `TransformModifier_GetType` (0x0036_0bf8): straight-line leaf. Mechanical port of the generated body.
pub fn transform_modifier_get_type_00360bf8(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `TransformModifier_Save` (0x0036_0c00): straight-line leaf. Mechanical port of the generated body.
pub fn transform_modifier_save_00360c00(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `BoostModifier_BaseV02` (0x0036_0c58): straight-line leaf. Mechanical port of the generated body.
pub fn boost_modifier_base_v02_00360c58(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `BoostModifier_BaseHandleEvent` (0x0036_0c60): straight-line leaf. Mechanical port of the generated body.
pub fn boost_modifier_base_handle_event_00360c60(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `BoostModifier_BaseV04` (0x0036_0c68): straight-line leaf: 1 load(s). Mechanical port of the generated body.
pub fn boost_modifier_base_v04_00360c68(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = c.lw((c.g[A0][0] as u32));
    ret(c);
}

/// `BoostModifier_BaseGetType` (0x0036_0c70): straight-line leaf. Mechanical port of the generated body.
pub fn boost_modifier_base_get_type_00360c70(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `ModifierListNode_GetType` (0x0036_0ce8): straight-line leaf. Mechanical port of the generated body.
pub fn modifier_list_node_get_type_00360ce8(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0x1;
    ret(c);
}

/// `ModifierListNode_OnTransfer` (0x0036_0cf0): straight-line leaf. Mechanical port of the generated body.
pub fn modifier_list_node_on_transfer_00360cf0(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `ModifierListNode_Save` (0x0036_0cf8): straight-line leaf. Mechanical port of the generated body.
pub fn modifier_list_node_save_00360cf8(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `ParticleModifier_BaseUpdate` (0x0036_0d00): straight-line leaf. Mechanical port of the generated body.
pub fn particle_modifier_base_update_00360d00(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `ParticleModifier_BaseRender` (0x0036_0d08): straight-line leaf. Mechanical port of the generated body.
pub fn particle_modifier_base_render_00360d08(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `ParticleModifier_BaseHandleEvent` (0x0036_0d10): straight-line leaf. Mechanical port of the generated body.
pub fn particle_modifier_base_handle_event_00360d10(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `ParticleModifier_GetType` (0x0036_0d20): straight-line leaf. Mechanical port of the generated body.
pub fn particle_modifier_get_type_00360d20(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0x1;
    ret(c);
}

/// `DynamicParticleModifier_GetType` (0x0036_0d28): straight-line leaf. Mechanical port of the generated body.
pub fn dynamic_particle_modifier_get_type_00360d28(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0x2;
    ret(c);
}

/// `HaloModifier_GetType` (0x0036_0d38): straight-line leaf. Mechanical port of the generated body.
pub fn halo_modifier_get_type_00360d38(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0x3;
    ret(c);
}

/// `RailModifierListNode_v02` (0x0036_0d90): straight-line leaf. Mechanical port of the generated body.
pub fn rail_modifier_list_node_v02_00360d90(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `ModObj_v47` (0x0036_0dc8): straight-line leaf. Mechanical port of the generated body.
pub fn mod_obj_v47_00360dc8(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `ModObj_AcceptsModifiers` (0x0036_0dd0): straight-line leaf. Mechanical port of the generated body.
pub fn mod_obj_accepts_modifiers_00360dd0(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0x1;
    ret(c);
}

/// `AnimObject_BaseV15` (0x0036_1060): straight-line leaf. Mechanical port of the generated body.
pub fn anim_object_base_v15_00361060(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0x1;
    ret(c);
}

/// `AnimObject_BaseV53` (0x0036_1080): straight-line leaf. Mechanical port of the generated body.
pub fn anim_object_base_v53_00361080(c: &mut Cpu) {
    c.cycles += 3;
    c.mtc1(0, 0);
    ret(c);
}

/// `AnimObject_v26` (0x0036_1090): straight-line leaf. Mechanical port of the generated body.
pub fn anim_object_v26_00361090(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0x1;
    ret(c);
}

/// `AnimObject_v19` (0x0036_1150): straight-line leaf. Mechanical port of the generated body.
pub fn anim_object_v19_00361150(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = c.g[A0][0];
    ret(c);
}

/// `AnimObject_v17` (0x0036_1198): straight-line leaf: 1 load(s). Mechanical port of the generated body.
pub fn anim_object_v17_00361198(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
    ret(c);
}

/// `AnimTeeter_v19` (0x0036_1458): straight-line leaf. Mechanical port of the generated body.
pub fn anim_teeter_v19_00361458(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = c.g[A0][0];
    ret(c);
}

/// `BoostModifier_GetType` (0x0036_14b8): straight-line leaf. Mechanical port of the generated body.
pub fn boost_modifier_get_type_003614b8(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0x1;
    ret(c);
}

/// `BoostModifier_v09` (0x0036_14c0): straight-line leaf. Mechanical port of the generated body.
pub fn boost_modifier_v09_003614c0(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `BoostModifier_v10` (0x0036_14c8): straight-line leaf. Mechanical port of the generated body.
pub fn boost_modifier_v10_003614c8(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `Debounce_v40` (0x0036_14f8): straight-line leaf. Mechanical port of the generated body.
pub fn debounce_v40_003614f8(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `Conveyor_v19` (0x0036_1528): straight-line leaf. Mechanical port of the generated body.
pub fn conveyor_v19_00361528(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1c));
    ret(c);
}

/// `FlexBridge_v19` (0x0036_1558): straight-line leaf. Mechanical port of the generated body.
pub fn flex_bridge_v19_00361558(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = c.g[A0][0];
    ret(c);
}

/// `FlexBridge_v38` (0x0036_1560): straight-line leaf. Mechanical port of the generated body.
pub fn flex_bridge_v38_00361560(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0x1;
    ret(c);
}

/// `FlexBridge_v44` (0x0036_1568): straight-line leaf. Mechanical port of the generated body.
pub fn flex_bridge_v44_00361568(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0x1;
    ret(c);
}

/// `FlexBridge_v45` (0x0036_1570): straight-line leaf. Mechanical port of the generated body.
pub fn flex_bridge_v45_00361570(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x90));
    ret(c);
}

/// `FlexRail_v47` (0x0036_1578): straight-line leaf. Mechanical port of the generated body.
pub fn flex_rail_v47_00361578(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `FlexRail_v48` (0x0036_1580): straight-line leaf. Mechanical port of the generated body.
pub fn flex_rail_v48_00361580(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `FlexRail_ApplyImpulse` (0x0036_1588): straight-line leaf. Mechanical port of the generated body.
pub fn flex_rail_apply_impulse_00361588(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `FlexRail_GetContactVelocity` (0x0036_1590): straight-line leaf. Mechanical port of the generated body.
pub fn flex_rail_get_contact_velocity_00361590(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `FlexRail_v49` (0x0036_1598): straight-line leaf. Mechanical port of the generated body.
pub fn flex_rail_v49_00361598(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `FlexRail_v46` (0x0036_15a0): straight-line leaf. Mechanical port of the generated body.
pub fn flex_rail_v46_003615a0(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0x1;
    ret(c);
}

/// `Floating_v19` (0x0036_1768): straight-line leaf. Mechanical port of the generated body.
pub fn floating_v19_00361768(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x2c));
    ret(c);
}

/// `Floating_v44` (0x0036_1770): straight-line leaf. Mechanical port of the generated body.
pub fn floating_v44_00361770(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0x1;
    ret(c);
}

/// `Floating_v26` (0x0036_17b0): straight-line leaf. Mechanical port of the generated body.
pub fn floating_v26_003617b0(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0x1;
    ret(c);
}

/// `PositionModifier_Update` (0x0036_18f8): straight-line leaf. Mechanical port of the generated body.
pub fn position_modifier_update_003618f8(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `PositionModifier_HasTransformChanged` (0x0036_1900): straight-line leaf: 1 load(s). Mechanical port of the generated body.
pub fn position_modifier_has_transform_changed_00361900(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x44));
    ret(c);
}

/// `PositionModifier_GetType` (0x0036_1908): straight-line leaf. Mechanical port of the generated body.
pub fn position_modifier_get_type_00361908(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0x3;
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

    /// `TransformModifier_GetType` (0x0036_0bf8) against the generated body.
    #[test]
    fn transform_modifier_get_type_00360bf8_matches() {
        check(
            ee::s12::f_00360bf8__generated,
            transform_modifier_get_type_00360bf8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x360bf8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `TransformModifier_Save` (0x0036_0c00) against the generated body.
    #[test]
    fn transform_modifier_save_00360c00_matches() {
        check(
            ee::s12::f_00360c00__generated,
            transform_modifier_save_00360c00,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x360c00, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `BoostModifier_BaseV02` (0x0036_0c58) against the generated body.
    #[test]
    fn boost_modifier_base_v02_00360c58_matches() {
        check(
            ee::s12::f_00360c58__generated,
            boost_modifier_base_v02_00360c58,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x360c58, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `BoostModifier_BaseHandleEvent` (0x0036_0c60) against the generated body.
    #[test]
    fn boost_modifier_base_handle_event_00360c60_matches() {
        check(
            ee::s12::f_00360c60__generated,
            boost_modifier_base_handle_event_00360c60,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x360c60, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `BoostModifier_BaseV04` (0x0036_0c68) against the generated body.
    #[test]
    fn boost_modifier_base_v04_00360c68_matches() {
        check(
            ee::s12::f_00360c68__generated,
            boost_modifier_base_v04_00360c68,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x360c68, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `BoostModifier_BaseGetType` (0x0036_0c70) against the generated body.
    #[test]
    fn boost_modifier_base_get_type_00360c70_matches() {
        check(
            ee::s12::f_00360c70__generated,
            boost_modifier_base_get_type_00360c70,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x360c70, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `ModifierListNode_GetType` (0x0036_0ce8) against the generated body.
    #[test]
    fn modifier_list_node_get_type_00360ce8_matches() {
        check(
            ee::s12::f_00360ce8__generated,
            modifier_list_node_get_type_00360ce8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x360ce8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `ModifierListNode_OnTransfer` (0x0036_0cf0) against the generated body.
    #[test]
    fn modifier_list_node_on_transfer_00360cf0_matches() {
        check(
            ee::s12::f_00360cf0__generated,
            modifier_list_node_on_transfer_00360cf0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x360cf0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `ModifierListNode_Save` (0x0036_0cf8) against the generated body.
    #[test]
    fn modifier_list_node_save_00360cf8_matches() {
        check(
            ee::s12::f_00360cf8__generated,
            modifier_list_node_save_00360cf8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x360cf8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `ParticleModifier_BaseUpdate` (0x0036_0d00) against the generated body.
    #[test]
    fn particle_modifier_base_update_00360d00_matches() {
        check(
            ee::s12::f_00360d00__generated,
            particle_modifier_base_update_00360d00,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x360d00, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `ParticleModifier_BaseRender` (0x0036_0d08) against the generated body.
    #[test]
    fn particle_modifier_base_render_00360d08_matches() {
        check(
            ee::s12::f_00360d08__generated,
            particle_modifier_base_render_00360d08,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x360d08, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `ParticleModifier_BaseHandleEvent` (0x0036_0d10) against the generated body.
    #[test]
    fn particle_modifier_base_handle_event_00360d10_matches() {
        check(
            ee::s12::f_00360d10__generated,
            particle_modifier_base_handle_event_00360d10,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x360d10, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `ParticleModifier_GetType` (0x0036_0d20) against the generated body.
    #[test]
    fn particle_modifier_get_type_00360d20_matches() {
        check(
            ee::s12::f_00360d20__generated,
            particle_modifier_get_type_00360d20,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x360d20, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `DynamicParticleModifier_GetType` (0x0036_0d28) against the generated body.
    #[test]
    fn dynamic_particle_modifier_get_type_00360d28_matches() {
        check(
            ee::s12::f_00360d28__generated,
            dynamic_particle_modifier_get_type_00360d28,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x360d28, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `HaloModifier_GetType` (0x0036_0d38) against the generated body.
    #[test]
    fn halo_modifier_get_type_00360d38_matches() {
        check(
            ee::s12::f_00360d38__generated,
            halo_modifier_get_type_00360d38,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x360d38, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `RailModifierListNode_v02` (0x0036_0d90) against the generated body.
    #[test]
    fn rail_modifier_list_node_v02_00360d90_matches() {
        check(
            ee::s12::f_00360d90__generated,
            rail_modifier_list_node_v02_00360d90,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x360d90, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `ModObj_v47` (0x0036_0dc8) against the generated body.
    #[test]
    fn mod_obj_v47_00360dc8_matches() {
        check(
            ee::s12::f_00360dc8__generated,
            mod_obj_v47_00360dc8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x360dc8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `ModObj_AcceptsModifiers` (0x0036_0dd0) against the generated body.
    #[test]
    fn mod_obj_accepts_modifiers_00360dd0_matches() {
        check(
            ee::s12::f_00360dd0__generated,
            mod_obj_accepts_modifiers_00360dd0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x360dd0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `AnimObject_BaseV15` (0x0036_1060) against the generated body.
    #[test]
    fn anim_object_base_v15_00361060_matches() {
        check(
            ee::s12::f_00361060__generated,
            anim_object_base_v15_00361060,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x361060, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `AnimObject_BaseV53` (0x0036_1080) against the generated body.
    #[test]
    fn anim_object_base_v53_00361080_matches() {
        check(
            ee::s12::f_00361080__generated,
            anim_object_base_v53_00361080,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x361080, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `AnimObject_v26` (0x0036_1090) against the generated body.
    #[test]
    fn anim_object_v26_00361090_matches() {
        check(
            ee::s12::f_00361090__generated,
            anim_object_v26_00361090,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x361090, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `AnimObject_v19` (0x0036_1150) against the generated body.
    #[test]
    fn anim_object_v19_00361150_matches() {
        check(
            ee::s12::f_00361150__generated,
            anim_object_v19_00361150,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x361150, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `AnimObject_v17` (0x0036_1198) against the generated body.
    #[test]
    fn anim_object_v17_00361198_matches() {
        check(
            ee::s12::f_00361198__generated,
            anim_object_v17_00361198,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x361198, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `AnimTeeter_v19` (0x0036_1458) against the generated body.
    #[test]
    fn anim_teeter_v19_00361458_matches() {
        check(
            ee::s12::f_00361458__generated,
            anim_teeter_v19_00361458,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x361458, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `BoostModifier_GetType` (0x0036_14b8) against the generated body.
    #[test]
    fn boost_modifier_get_type_003614b8_matches() {
        check(
            ee::s12::f_003614b8__generated,
            boost_modifier_get_type_003614b8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x3614b8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `BoostModifier_v09` (0x0036_14c0) against the generated body.
    #[test]
    fn boost_modifier_v09_003614c0_matches() {
        check(
            ee::s12::f_003614c0__generated,
            boost_modifier_v09_003614c0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x3614c0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `BoostModifier_v10` (0x0036_14c8) against the generated body.
    #[test]
    fn boost_modifier_v10_003614c8_matches() {
        check(
            ee::s12::f_003614c8__generated,
            boost_modifier_v10_003614c8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x3614c8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `Debounce_v40` (0x0036_14f8) against the generated body.
    #[test]
    fn debounce_v40_003614f8_matches() {
        check(
            ee::s12::f_003614f8__generated,
            debounce_v40_003614f8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x3614f8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `Conveyor_v19` (0x0036_1528) against the generated body.
    #[test]
    fn conveyor_v19_00361528_matches() {
        check(
            ee::s12::f_00361528__generated,
            conveyor_v19_00361528,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x361528, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `FlexBridge_v19` (0x0036_1558) against the generated body.
    #[test]
    fn flex_bridge_v19_00361558_matches() {
        check(
            ee::s12::f_00361558__generated,
            flex_bridge_v19_00361558,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x361558, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `FlexBridge_v38` (0x0036_1560) against the generated body.
    #[test]
    fn flex_bridge_v38_00361560_matches() {
        check(
            ee::s12::f_00361560__generated,
            flex_bridge_v38_00361560,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x361560, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `FlexBridge_v44` (0x0036_1568) against the generated body.
    #[test]
    fn flex_bridge_v44_00361568_matches() {
        check(
            ee::s12::f_00361568__generated,
            flex_bridge_v44_00361568,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x361568, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `FlexBridge_v45` (0x0036_1570) against the generated body.
    #[test]
    fn flex_bridge_v45_00361570_matches() {
        check(
            ee::s12::f_00361570__generated,
            flex_bridge_v45_00361570,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x361570, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `FlexRail_v47` (0x0036_1578) against the generated body.
    #[test]
    fn flex_rail_v47_00361578_matches() {
        check(
            ee::s12::f_00361578__generated,
            flex_rail_v47_00361578,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x361578, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `FlexRail_v48` (0x0036_1580) against the generated body.
    #[test]
    fn flex_rail_v48_00361580_matches() {
        check(
            ee::s12::f_00361580__generated,
            flex_rail_v48_00361580,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x361580, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `FlexRail_ApplyImpulse` (0x0036_1588) against the generated body.
    #[test]
    fn flex_rail_apply_impulse_00361588_matches() {
        check(
            ee::s12::f_00361588__generated,
            flex_rail_apply_impulse_00361588,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x361588, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `FlexRail_GetContactVelocity` (0x0036_1590) against the generated body.
    #[test]
    fn flex_rail_get_contact_velocity_00361590_matches() {
        check(
            ee::s12::f_00361590__generated,
            flex_rail_get_contact_velocity_00361590,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x361590, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `FlexRail_v49` (0x0036_1598) against the generated body.
    #[test]
    fn flex_rail_v49_00361598_matches() {
        check(
            ee::s12::f_00361598__generated,
            flex_rail_v49_00361598,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x361598, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `FlexRail_v46` (0x0036_15a0) against the generated body.
    #[test]
    fn flex_rail_v46_003615a0_matches() {
        check(
            ee::s12::f_003615a0__generated,
            flex_rail_v46_003615a0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x3615a0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `Floating_v19` (0x0036_1768) against the generated body.
    #[test]
    fn floating_v19_00361768_matches() {
        check(
            ee::s12::f_00361768__generated,
            floating_v19_00361768,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x361768, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `Floating_v44` (0x0036_1770) against the generated body.
    #[test]
    fn floating_v44_00361770_matches() {
        check(
            ee::s12::f_00361770__generated,
            floating_v44_00361770,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x361770, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `Floating_v26` (0x0036_17b0) against the generated body.
    #[test]
    fn floating_v26_003617b0_matches() {
        check(
            ee::s12::f_003617b0__generated,
            floating_v26_003617b0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x3617b0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `PositionModifier_Update` (0x0036_18f8) against the generated body.
    #[test]
    fn position_modifier_update_003618f8_matches() {
        check(
            ee::s12::f_003618f8__generated,
            position_modifier_update_003618f8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x3618f8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `PositionModifier_HasTransformChanged` (0x0036_1900) against the generated body.
    #[test]
    fn position_modifier_has_transform_changed_00361900_matches() {
        check(
            ee::s12::f_00361900__generated,
            position_modifier_has_transform_changed_00361900,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x361900, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `PositionModifier_GetType` (0x0036_1908) against the generated body.
    #[test]
    fn position_modifier_get_type_00361908_matches() {
        check(
            ee::s12::f_00361908__generated,
            position_modifier_get_type_00361908,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x361908, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }
}
