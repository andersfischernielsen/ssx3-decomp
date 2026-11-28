//! Staging module oc1514 (wave wave276): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1514`.

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
        0x0033_0778,
        coll_sphere_tree_reset_transform_00330778,
        "CollSphereTree_ResetTransform",
    ),
    (0x0033_88e0, nash_return_self, "Nash_ReturnSelf"),
    (
        0x0034_0a08,
        coll_volume_refine_triangle_hit_00340a08,
        "CollVolume_RefineTriangleHit",
    ),
    (
        0x0034_0a10,
        coll_segment_get_diameter_00340a10,
        "CollSegment_GetDiameter",
    ),
    (
        0x0034_6060,
        dynamic_particle_modifier_on_transfer_00346060,
        "DynamicParticleModifier_OnTransfer",
    ),
    (0x0034_af18, flag_render_0034af18, "Flag_Render"),
    (0x0035_cfe0, rail_modifier_v04_0035cfe0, "RailModifier_v04"),
    (
        0x0035_ed88,
        roller_modifier_update_transform_0035ed88,
        "RollerModifier_UpdateTransform",
    ),
    (
        0x0035_fdd0,
        sync_process_return_self,
        "SyncProcess_ReturnSelf",
    ),
    (
        0x0036_0720,
        game_process_update_00360720,
        "GameProcess_Update",
    ),
    (
        0x0036_0728,
        game_process_post_update_00360728,
        "GameProcess_PostUpdate",
    ),
    (
        0x0036_0730,
        game_process_render_00360730,
        "GameProcess_Render",
    ),
    (0x0036_0738, game_process_v05_00360738, "GameProcess_v05"),
    (
        0x0036_0740,
        game_process_id_less_equal_00360740,
        "GameProcess_IdLessEqual",
    ),
    (
        0x0036_0748,
        game_process_id_equals_00360748,
        "GameProcess_IdEquals",
    ),
    (
        0x0036_0750,
        game_process_get_id_00360750,
        "GameProcess_GetId",
    ),
    (0x0036_0758, game_process_v09_00360758, "GameProcess_v09"),
    (
        0x0036_0760,
        game_process_is_type_00360760,
        "GameProcess_IsType",
    ),
    (0x0036_0778, game_process_save_00360778, "GameProcess_Save"),
    (0x0036_0780, game_process_v13_00360780, "GameProcess_v13"),
    (
        0x0036_0788,
        typed_process_update_00360788,
        "TypedProcess_Update",
    ),
    (
        0x0036_0790,
        typed_process_render_00360790,
        "TypedProcess_Render",
    ),
    (0x0036_0798, typed_process_v14_00360798, "TypedProcess_v14"),
    (
        0x0036_0880,
        sync_process_get_id_00360880,
        "SyncProcess_GetId",
    ),
    (0x0036_0910, world_obj_v15_00360910, "WorldObj_v15"),
    (0x0036_0918, world_obj_v16_00360918, "WorldObj_v16"),
    (0x0036_0920, world_obj_v11_00360920, "WorldObj_v11"),
    (0x0036_0930, world_obj_v17_00360930, "WorldObj_v17"),
    (0x0036_0938, world_obj_v18_00360938, "WorldObj_v18"),
    (0x0036_0940, world_obj_v19_00360940, "WorldObj_v19"),
    (0x0036_0948, world_obj_v20_00360948, "WorldObj_v20"),
    (0x0036_0950, world_obj_v21_00360950, "WorldObj_v21"),
    (0x0036_0958, world_obj_v22_00360958, "WorldObj_v22"),
    (0x0036_0960, world_obj_v23_00360960, "WorldObj_v23"),
    (0x0036_0970, world_obj_v24_00360970, "WorldObj_v24"),
    (0x0036_0980, world_obj_v25_00360980, "WorldObj_v25"),
    (0x0036_0990, world_obj_v26_00360990, "WorldObj_v26"),
    (
        0x0036_0998,
        world_obj_get_matrices_00360998,
        "WorldObj_GetMatrices",
    ),
    (0x0036_09a0, world_obj_v28_003609a0, "WorldObj_v28"),
    (
        0x0036_09a8,
        world_obj_get_matrix_at_003609a8,
        "WorldObj_GetMatrixAt",
    ),
    (0x0036_09b0, world_obj_v30_003609b0, "WorldObj_v30"),
    (0x0036_09c0, world_obj_v31_003609c0, "WorldObj_v31"),
    (0x0036_09c8, world_obj_v32_003609c8, "WorldObj_v32"),
    (
        0x0036_09d0,
        world_obj_handle_event_003609d0,
        "WorldObj_HandleEvent",
    ),
    (0x0036_09d8, world_obj_v37_003609d8, "WorldObj_v37"),
];

/// `CollSphereTree_ResetTransform` (0x0033_0778): straight-line leaf: 1 load(s), 2 store(s). Mechanical port of the generated body.
pub fn coll_sphere_tree_reset_transform_00330778(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x64));
    c.sw((c.g[A0][0] as u32).wrapping_add(0x4), 0u64);
    c.sw((c.g[A0][0] as u32).wrapping_add(0x60), c.g[V0][0]);
    ret(c);
}

/// `Nash_ReturnSelf` (0x0033_88e0): straight-line leaf: 11 store(s). Mechanical port of the generated body.
pub fn nash_return_self(c: &mut Cpu) {
    c.cycles += 15;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x80));
    c.g[V0][0] = c.g[A0][0];
    c.sd((c.g[SP][0] as u32).wrapping_add(0x48), c.g[A1][0]);
    c.sd((c.g[SP][0] as u32).wrapping_add(0x50), c.g[A2][0]);
    c.sd((c.g[SP][0] as u32).wrapping_add(0x58), c.g[A3][0]);
    c.sd((c.g[SP][0] as u32).wrapping_add(0x60), c.g[T0][0]);
    c.sd((c.g[SP][0] as u32).wrapping_add(0x68), c.g[T1][0]);
    c.sd((c.g[SP][0] as u32).wrapping_add(0x70), c.g[T2][0]);
    c.sd((c.g[SP][0] as u32).wrapping_add(0x78), c.g[T3][0]);
    c.swc1(12, (c.g[SP][0] as u32).wrapping_add(0x38));
    c.swc1(14, (c.g[SP][0] as u32).wrapping_add(0x3c));
    c.swc1(16, (c.g[SP][0] as u32).wrapping_add(0x40));
    c.swc1(18, (c.g[SP][0] as u32).wrapping_add(0x44));
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
    ret(c);
}

/// `CollVolume_RefineTriangleHit` (0x0034_0a08): straight-line leaf. Mechanical port of the generated body.
pub fn coll_volume_refine_triangle_hit_00340a08(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `CollSegment_GetDiameter` (0x0034_0a10): straight-line leaf: 1 load(s). Mechanical port of the generated body.
pub fn coll_segment_get_diameter_00340a10(c: &mut Cpu) {
    c.cycles += 2;
    c.lwc1(0, (c.g[A0][0] as u32).wrapping_add(0xa0));
    ret(c);
}

/// `DynamicParticleModifier_OnTransfer` (0x0034_6060): straight-line leaf: 1 store(s). Mechanical port of the generated body.
pub fn dynamic_particle_modifier_on_transfer_00346060(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = 0x1;
    c.sw((c.g[A0][0] as u32).wrapping_add(0x14), c.g[V0][0]);
    ret(c);
}

/// `Flag_Render` (0x0034_af18): straight-line leaf. Mechanical port of the generated body.
pub fn flag_render_0034af18(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `RailModifier_v04` (0x0035_cfe0): straight-line leaf. Mechanical port of the generated body.
pub fn rail_modifier_v04_0035cfe0(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `RollerModifier_UpdateTransform` (0x0035_ed88): straight-line leaf. Mechanical port of the generated body.
pub fn roller_modifier_update_transform_0035ed88(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `SyncProcess_ReturnSelf` (0x0035_fdd0): straight-line leaf: 11 store(s). Mechanical port of the generated body.
pub fn sync_process_return_self(c: &mut Cpu) {
    c.cycles += 15;
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x80));
    c.g[V0][0] = c.g[A0][0];
    c.sd((c.g[SP][0] as u32).wrapping_add(0x48), c.g[A1][0]);
    c.sd((c.g[SP][0] as u32).wrapping_add(0x50), c.g[A2][0]);
    c.sd((c.g[SP][0] as u32).wrapping_add(0x58), c.g[A3][0]);
    c.sd((c.g[SP][0] as u32).wrapping_add(0x60), c.g[T0][0]);
    c.sd((c.g[SP][0] as u32).wrapping_add(0x68), c.g[T1][0]);
    c.sd((c.g[SP][0] as u32).wrapping_add(0x70), c.g[T2][0]);
    c.sd((c.g[SP][0] as u32).wrapping_add(0x78), c.g[T3][0]);
    c.swc1(12, (c.g[SP][0] as u32).wrapping_add(0x38));
    c.swc1(14, (c.g[SP][0] as u32).wrapping_add(0x3c));
    c.swc1(16, (c.g[SP][0] as u32).wrapping_add(0x40));
    c.swc1(18, (c.g[SP][0] as u32).wrapping_add(0x44));
    c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x80));
    ret(c);
}

/// `GameProcess_Update` (0x0036_0720): straight-line leaf. Mechanical port of the generated body.
pub fn game_process_update_00360720(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `GameProcess_PostUpdate` (0x0036_0728): straight-line leaf. Mechanical port of the generated body.
pub fn game_process_post_update_00360728(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `GameProcess_Render` (0x0036_0730): straight-line leaf. Mechanical port of the generated body.
pub fn game_process_render_00360730(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `GameProcess_v05` (0x0036_0738): straight-line leaf. Mechanical port of the generated body.
pub fn game_process_v05_00360738(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `GameProcess_IdLessEqual` (0x0036_0740): straight-line leaf. Mechanical port of the generated body.
pub fn game_process_id_less_equal_00360740(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `GameProcess_IdEquals` (0x0036_0748): straight-line leaf. Mechanical port of the generated body.
pub fn game_process_id_equals_00360748(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `GameProcess_GetId` (0x0036_0750): straight-line leaf. Mechanical port of the generated body.
pub fn game_process_get_id_00360750(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `GameProcess_v09` (0x0036_0758): straight-line leaf. Mechanical port of the generated body.
pub fn game_process_v09_00360758(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `GameProcess_IsType` (0x0036_0760): straight-line leaf. Mechanical port of the generated body.
pub fn game_process_is_type_00360760(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `GameProcess_Save` (0x0036_0778): straight-line leaf. Mechanical port of the generated body.
pub fn game_process_save_00360778(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `GameProcess_v13` (0x0036_0780): straight-line leaf. Mechanical port of the generated body.
pub fn game_process_v13_00360780(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `TypedProcess_Update` (0x0036_0788): straight-line leaf. Mechanical port of the generated body.
pub fn typed_process_update_00360788(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `TypedProcess_Render` (0x0036_0790): straight-line leaf. Mechanical port of the generated body.
pub fn typed_process_render_00360790(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `TypedProcess_v14` (0x0036_0798): straight-line leaf. Mechanical port of the generated body.
pub fn typed_process_v14_00360798(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0x1;
    ret(c);
}

/// `SyncProcess_GetId` (0x0036_0880): straight-line leaf: 1 load(s). Mechanical port of the generated body.
pub fn sync_process_get_id_00360880(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x14));
    ret(c);
}

/// `WorldObj_v15` (0x0036_0910): straight-line leaf. Mechanical port of the generated body.
pub fn world_obj_v15_00360910(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0x1;
    ret(c);
}

/// `WorldObj_v16` (0x0036_0918): straight-line leaf. Mechanical port of the generated body.
pub fn world_obj_v16_00360918(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `WorldObj_v11` (0x0036_0920): straight-line leaf: 2 load(s). Mechanical port of the generated body.
pub fn world_obj_v11_00360920(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x18));
    c.g[V0][0] = c.lbu((c.g[V1][0] as u32).wrapping_add(0x78));
    ret(c);
}

/// `WorldObj_v17` (0x0036_0930): straight-line leaf. Mechanical port of the generated body.
pub fn world_obj_v17_00360930(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0x1;
    ret(c);
}

/// `WorldObj_v18` (0x0036_0938): straight-line leaf. Mechanical port of the generated body.
pub fn world_obj_v18_00360938(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `WorldObj_v19` (0x0036_0940): straight-line leaf. Mechanical port of the generated body.
pub fn world_obj_v19_00360940(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `WorldObj_v20` (0x0036_0948): straight-line leaf. Mechanical port of the generated body.
pub fn world_obj_v20_00360948(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `WorldObj_v21` (0x0036_0950): straight-line leaf. Mechanical port of the generated body.
pub fn world_obj_v21_00360950(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `WorldObj_v22` (0x0036_0958): straight-line leaf. Mechanical port of the generated body.
pub fn world_obj_v22_00360958(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `WorldObj_v23` (0x0036_0960): straight-line leaf. Mechanical port of the generated body.
pub fn world_obj_v23_00360960(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `WorldObj_v24` (0x0036_0970): straight-line leaf: 1 load(s). Mechanical port of the generated body.
pub fn world_obj_v24_00360970(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x18));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10));
    ret(c);
}

/// `WorldObj_v25` (0x0036_0980): straight-line leaf: 1 load(s). Mechanical port of the generated body.
pub fn world_obj_v25_00360980(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x18));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10));
    ret(c);
}

/// `WorldObj_v26` (0x0036_0990): straight-line leaf. Mechanical port of the generated body.
pub fn world_obj_v26_00360990(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `WorldObj_GetMatrices` (0x0036_0998): straight-line leaf. Mechanical port of the generated body.
pub fn world_obj_get_matrices_00360998(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `WorldObj_v28` (0x0036_09a0): straight-line leaf. Mechanical port of the generated body.
pub fn world_obj_v28_003609a0(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `WorldObj_GetMatrixAt` (0x0036_09a8): straight-line leaf. Mechanical port of the generated body.
pub fn world_obj_get_matrix_at_003609a8(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `WorldObj_v30` (0x0036_09b0): straight-line leaf. Mechanical port of the generated body.
pub fn world_obj_v30_003609b0(c: &mut Cpu) {
    c.cycles += 3;
    c.mtc1(0, 0);
    ret(c);
}

/// `WorldObj_v31` (0x0036_09c0): straight-line leaf. Mechanical port of the generated body.
pub fn world_obj_v31_003609c0(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `WorldObj_v32` (0x0036_09c8): straight-line leaf. Mechanical port of the generated body.
pub fn world_obj_v32_003609c8(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `WorldObj_HandleEvent` (0x0036_09d0): straight-line leaf. Mechanical port of the generated body.
pub fn world_obj_handle_event_003609d0(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `WorldObj_v37` (0x0036_09d8): straight-line leaf. Mechanical port of the generated body.
pub fn world_obj_v37_003609d8(c: &mut Cpu) {
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

    /// `CollSphereTree_ResetTransform` (0x0033_0778) against the generated body.
    #[test]
    fn coll_sphere_tree_reset_transform_00330778_matches() {
        check(
            ee::s11::f_00330778__generated,
            coll_sphere_tree_reset_transform_00330778,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x330778, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `Nash_ReturnSelf` (0x0033_88e0) against the generated body.
    #[test]
    fn nash_return_self_matches() {
        check(
            ee::s11::f_003388e0__generated,
            nash_return_self,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3388e0, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[A1][0] = leaf_val(&mut r);
                c.g[A2][0] = leaf_val(&mut r);
                c.g[A3][0] = leaf_val(&mut r);
                c.g[T0][0] = leaf_val(&mut r);
                c.g[T1][0] = leaf_val(&mut r);
                c.g[T2][0] = leaf_val(&mut r);
                c.g[T3][0] = leaf_val(&mut r);
                c.f[12] = r.edge_f32();
                c.f[14] = r.edge_f32();
                c.f[16] = r.edge_f32();
                c.f[18] = r.edge_f32();
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `CollVolume_RefineTriangleHit` (0x0034_0a08) against the generated body.
    #[test]
    fn coll_volume_refine_triangle_hit_00340a08_matches() {
        check(
            ee::s11::f_00340a08__generated,
            coll_volume_refine_triangle_hit_00340a08,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x340a08, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `CollSegment_GetDiameter` (0x0034_0a10) against the generated body.
    #[test]
    fn coll_segment_get_diameter_00340a10_matches() {
        check(
            ee::s11::f_00340a10__generated,
            coll_segment_get_diameter_00340a10,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x340a10, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `DynamicParticleModifier_OnTransfer` (0x0034_6060) against the generated body.
    #[test]
    fn dynamic_particle_modifier_on_transfer_00346060_matches() {
        check(
            ee::s11::f_00346060__generated,
            dynamic_particle_modifier_on_transfer_00346060,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x346060, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `Flag_Render` (0x0034_af18) against the generated body.
    #[test]
    fn flag_render_0034af18_matches() {
        check(
            ee::s11::f_0034af18__generated,
            flag_render_0034af18,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x34af18, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `RailModifier_v04` (0x0035_cfe0) against the generated body.
    #[test]
    fn rail_modifier_v04_0035cfe0_matches() {
        check(
            ee::s11::f_0035cfe0__generated,
            rail_modifier_v04_0035cfe0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x35cfe0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `RollerModifier_UpdateTransform` (0x0035_ed88) against the generated body.
    #[test]
    fn roller_modifier_update_transform_0035ed88_matches() {
        check(
            ee::s11::f_0035ed88__generated,
            roller_modifier_update_transform_0035ed88,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x35ed88, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `SyncProcess_ReturnSelf` (0x0035_fdd0) against the generated body.
    #[test]
    fn sync_process_return_self_matches() {
        check(
            ee::s12::f_0035fdd0__generated,
            sync_process_return_self,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x35fdd0, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[A1][0] = leaf_val(&mut r);
                c.g[A2][0] = leaf_val(&mut r);
                c.g[A3][0] = leaf_val(&mut r);
                c.g[T0][0] = leaf_val(&mut r);
                c.g[T1][0] = leaf_val(&mut r);
                c.g[T2][0] = leaf_val(&mut r);
                c.g[T3][0] = leaf_val(&mut r);
                c.f[12] = r.edge_f32();
                c.f[14] = r.edge_f32();
                c.f[16] = r.edge_f32();
                c.f[18] = r.edge_f32();
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `GameProcess_Update` (0x0036_0720) against the generated body.
    #[test]
    fn game_process_update_00360720_matches() {
        check(
            ee::s12::f_00360720__generated,
            game_process_update_00360720,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x360720, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `GameProcess_PostUpdate` (0x0036_0728) against the generated body.
    #[test]
    fn game_process_post_update_00360728_matches() {
        check(
            ee::s12::f_00360728__generated,
            game_process_post_update_00360728,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x360728, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `GameProcess_Render` (0x0036_0730) against the generated body.
    #[test]
    fn game_process_render_00360730_matches() {
        check(
            ee::s12::f_00360730__generated,
            game_process_render_00360730,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x360730, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `GameProcess_v05` (0x0036_0738) against the generated body.
    #[test]
    fn game_process_v05_00360738_matches() {
        check(
            ee::s12::f_00360738__generated,
            game_process_v05_00360738,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x360738, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `GameProcess_IdLessEqual` (0x0036_0740) against the generated body.
    #[test]
    fn game_process_id_less_equal_00360740_matches() {
        check(
            ee::s12::f_00360740__generated,
            game_process_id_less_equal_00360740,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x360740, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `GameProcess_IdEquals` (0x0036_0748) against the generated body.
    #[test]
    fn game_process_id_equals_00360748_matches() {
        check(
            ee::s12::f_00360748__generated,
            game_process_id_equals_00360748,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x360748, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `GameProcess_GetId` (0x0036_0750) against the generated body.
    #[test]
    fn game_process_get_id_00360750_matches() {
        check(
            ee::s12::f_00360750__generated,
            game_process_get_id_00360750,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x360750, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `GameProcess_v09` (0x0036_0758) against the generated body.
    #[test]
    fn game_process_v09_00360758_matches() {
        check(
            ee::s12::f_00360758__generated,
            game_process_v09_00360758,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x360758, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `GameProcess_IsType` (0x0036_0760) against the generated body.
    #[test]
    fn game_process_is_type_00360760_matches() {
        check(
            ee::s12::f_00360760__generated,
            game_process_is_type_00360760,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x360760, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `GameProcess_Save` (0x0036_0778) against the generated body.
    #[test]
    fn game_process_save_00360778_matches() {
        check(
            ee::s12::f_00360778__generated,
            game_process_save_00360778,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x360778, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `GameProcess_v13` (0x0036_0780) against the generated body.
    #[test]
    fn game_process_v13_00360780_matches() {
        check(
            ee::s12::f_00360780__generated,
            game_process_v13_00360780,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x360780, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `TypedProcess_Update` (0x0036_0788) against the generated body.
    #[test]
    fn typed_process_update_00360788_matches() {
        check(
            ee::s12::f_00360788__generated,
            typed_process_update_00360788,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x360788, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `TypedProcess_Render` (0x0036_0790) against the generated body.
    #[test]
    fn typed_process_render_00360790_matches() {
        check(
            ee::s12::f_00360790__generated,
            typed_process_render_00360790,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x360790, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `TypedProcess_v14` (0x0036_0798) against the generated body.
    #[test]
    fn typed_process_v14_00360798_matches() {
        check(
            ee::s12::f_00360798__generated,
            typed_process_v14_00360798,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x360798, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `SyncProcess_GetId` (0x0036_0880) against the generated body.
    #[test]
    fn sync_process_get_id_00360880_matches() {
        check(
            ee::s12::f_00360880__generated,
            sync_process_get_id_00360880,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x360880, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `WorldObj_v15` (0x0036_0910) against the generated body.
    #[test]
    fn world_obj_v15_00360910_matches() {
        check(
            ee::s12::f_00360910__generated,
            world_obj_v15_00360910,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x360910, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `WorldObj_v16` (0x0036_0918) against the generated body.
    #[test]
    fn world_obj_v16_00360918_matches() {
        check(
            ee::s12::f_00360918__generated,
            world_obj_v16_00360918,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x360918, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `WorldObj_v11` (0x0036_0920) against the generated body.
    #[test]
    fn world_obj_v11_00360920_matches() {
        check(
            ee::s12::f_00360920__generated,
            world_obj_v11_00360920,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x360920, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, true, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `WorldObj_v17` (0x0036_0930) against the generated body.
    #[test]
    fn world_obj_v17_00360930_matches() {
        check(
            ee::s12::f_00360930__generated,
            world_obj_v17_00360930,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x360930, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `WorldObj_v18` (0x0036_0938) against the generated body.
    #[test]
    fn world_obj_v18_00360938_matches() {
        check(
            ee::s12::f_00360938__generated,
            world_obj_v18_00360938,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x360938, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `WorldObj_v19` (0x0036_0940) against the generated body.
    #[test]
    fn world_obj_v19_00360940_matches() {
        check(
            ee::s12::f_00360940__generated,
            world_obj_v19_00360940,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x360940, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `WorldObj_v20` (0x0036_0948) against the generated body.
    #[test]
    fn world_obj_v20_00360948_matches() {
        check(
            ee::s12::f_00360948__generated,
            world_obj_v20_00360948,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x360948, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `WorldObj_v21` (0x0036_0950) against the generated body.
    #[test]
    fn world_obj_v21_00360950_matches() {
        check(
            ee::s12::f_00360950__generated,
            world_obj_v21_00360950,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x360950, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `WorldObj_v22` (0x0036_0958) against the generated body.
    #[test]
    fn world_obj_v22_00360958_matches() {
        check(
            ee::s12::f_00360958__generated,
            world_obj_v22_00360958,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x360958, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `WorldObj_v23` (0x0036_0960) against the generated body.
    #[test]
    fn world_obj_v23_00360960_matches() {
        check(
            ee::s12::f_00360960__generated,
            world_obj_v23_00360960,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x360960, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `WorldObj_v24` (0x0036_0970) against the generated body.
    #[test]
    fn world_obj_v24_00360970_matches() {
        check(
            ee::s12::f_00360970__generated,
            world_obj_v24_00360970,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x360970, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `WorldObj_v25` (0x0036_0980) against the generated body.
    #[test]
    fn world_obj_v25_00360980_matches() {
        check(
            ee::s12::f_00360980__generated,
            world_obj_v25_00360980,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x360980, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `WorldObj_v26` (0x0036_0990) against the generated body.
    #[test]
    fn world_obj_v26_00360990_matches() {
        check(
            ee::s12::f_00360990__generated,
            world_obj_v26_00360990,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x360990, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `WorldObj_GetMatrices` (0x0036_0998) against the generated body.
    #[test]
    fn world_obj_get_matrices_00360998_matches() {
        check(
            ee::s12::f_00360998__generated,
            world_obj_get_matrices_00360998,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x360998, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `WorldObj_v28` (0x0036_09a0) against the generated body.
    #[test]
    fn world_obj_v28_003609a0_matches() {
        check(
            ee::s12::f_003609a0__generated,
            world_obj_v28_003609a0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x3609a0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `WorldObj_GetMatrixAt` (0x0036_09a8) against the generated body.
    #[test]
    fn world_obj_get_matrix_at_003609a8_matches() {
        check(
            ee::s12::f_003609a8__generated,
            world_obj_get_matrix_at_003609a8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x3609a8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `WorldObj_v30` (0x0036_09b0) against the generated body.
    #[test]
    fn world_obj_v30_003609b0_matches() {
        check(
            ee::s12::f_003609b0__generated,
            world_obj_v30_003609b0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x3609b0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `WorldObj_v31` (0x0036_09c0) against the generated body.
    #[test]
    fn world_obj_v31_003609c0_matches() {
        check(
            ee::s12::f_003609c0__generated,
            world_obj_v31_003609c0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x3609c0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `WorldObj_v32` (0x0036_09c8) against the generated body.
    #[test]
    fn world_obj_v32_003609c8_matches() {
        check(
            ee::s12::f_003609c8__generated,
            world_obj_v32_003609c8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x3609c8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `WorldObj_HandleEvent` (0x0036_09d0) against the generated body.
    #[test]
    fn world_obj_handle_event_003609d0_matches() {
        check(
            ee::s12::f_003609d0__generated,
            world_obj_handle_event_003609d0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x3609d0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `WorldObj_v37` (0x0036_09d8) against the generated body.
    #[test]
    fn world_obj_v37_003609d8_matches() {
        check(
            ee::s12::f_003609d8__generated,
            world_obj_v37_003609d8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x3609d8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }
}
