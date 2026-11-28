//! Staging module oc1502 (wave wave276): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1502`.

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
    (0x0014_585c, leaf_0014585c_0014585c, "leaf_0014585c"),
    (0x0014_58ec, leaf_001458ec, "leaf_001458ec"),
    (0x0014_58f4, leaf_001458f4, "leaf_001458f4"),
    (0x0014_58fc, leaf_001458fc, "leaf_001458fc"),
    (0x0014_5904, leaf_00145904, "leaf_00145904"),
    (0x0014_590c, leaf_0014590c, "leaf_0014590c"),
    (0x0014_5914, leaf_00145914, "leaf_00145914"),
    (0x0014_591c, leaf_0014591c, "leaf_0014591c"),
    (0x0014_5924, leaf_00145924, "leaf_00145924"),
    (
        0x0015_49e0,
        be_score_int_commit_001549e0,
        "BEScoreInt_Commit",
    ),
    (
        0x0015_5380,
        be_score_int_save_state_00155380,
        "BEScoreInt_SaveState",
    ),
    (
        0x0015_5388,
        be_score_int_load_state_00155388,
        "BEScoreInt_LoadState",
    ),
    (
        0x0015_5aa0,
        be_aggression_int_commit_00155aa0,
        "BEAggressionInt_Commit",
    ),
    (
        0x0015_5aa8,
        be_aggression_int_reset_00155aa8,
        "BEAggressionInt_Reset",
    ),
    (0x0015_afe8, debug_printf_stub_afe8, "DebugPrintf_Stub_afe8"),
    (
        0x0015_bb00,
        be_interface_commit_0015bb00,
        "BEInterface_Commit",
    ),
    (
        0x0015_bb08,
        be_interface_reset_0015bb08,
        "BEInterface_Reset",
    ),
    (
        0x0015_bb10,
        be_interface_save_state_0015bb10,
        "BEInterface_SaveState",
    ),
    (
        0x0015_bb18,
        be_interface_load_state_0015bb18,
        "BEInterface_LoadState",
    ),
    (
        0x0015_bcc0,
        be_mission_int_commit_0015bcc0,
        "BEMissionInt_Commit",
    ),
    (
        0x0015_bcc8,
        be_mission_int_reset_0015bcc8,
        "BEMissionInt_Reset",
    ),
    (
        0x0015_c6c8,
        be_char_db_int_commit_0015c6c8,
        "BECharDBInt_Commit",
    ),
    (
        0x0015_c6d0,
        be_char_db_int_reset_0015c6d0,
        "BECharDBInt_Reset",
    ),
    (0x0015_c6d8, be_save_int_commit_0015c6d8, "BESaveInt_Commit"),
    (0x0015_c6e0, be_save_int_reset_0015c6e0, "BESaveInt_Reset"),
    (0x0015_c7d0, be_econ_int_commit_0015c7d0, "BEEconInt_Commit"),
    (0x0015_c7d8, be_econ_int_reset_0015c7d8, "BEEconInt_Reset"),
    (
        0x0015_f640,
        rider_camera_target_get_steer_0015f640,
        "RiderCameraTarget_GetSteer",
    ),
    (
        0x0015_f680,
        rider_camera_target_get_handplant_data0_0015f680,
        "RiderCameraTarget_GetHandplantData0",
    ),
    (
        0x0015_f690,
        rider_camera_target_get_handplant_data1_0015f690,
        "RiderCameraTarget_GetHandplantData1",
    ),
    (
        0x0015_f6a0,
        rider_camera_target_get_air_time_0015f6a0,
        "RiderCameraTarget_GetAirTime",
    ),
    (
        0x0015_f6e0,
        rider_camera_target_get_jump_origin_0015f6e0,
        "RiderCameraTarget_GetJumpOrigin",
    ),
    (
        0x0015_f6f0,
        rider_camera_target_get_jump_dir_0015f6f0,
        "RiderCameraTarget_GetJumpDir",
    ),
    (
        0x0015_f700,
        rider_camera_target_get_boost_0015f700,
        "RiderCameraTarget_GetBoost",
    ),
    (
        0x0015_f710,
        rider_camera_target_get_slope_term_0015f710,
        "RiderCameraTarget_GetSlopeTerm",
    ),
    (
        0x0015_f750,
        rider_camera_target_get_surface_type_0015f750,
        "RiderCameraTarget_GetSurfaceType",
    ),
    (
        0x0015_f760,
        rider_camera_target_get_takeoff_pop_0015f760,
        "RiderCameraTarget_GetTakeoffPop",
    ),
    (
        0x0015_f770,
        rider_camera_target_is_ollie_0015f770,
        "RiderCameraTarget_IsOllie",
    ),
    (
        0x0015_f908,
        rider_camera_target_get_rail_flag_0015f908,
        "RiderCameraTarget_GetRailFlag",
    ),
    (
        0x0015_f918,
        rider_camera_target_get_landing_normal_0015f918,
        "RiderCameraTarget_GetLandingNormal",
    ),
    (
        0x0015_f928,
        rider_camera_target_get_landing_velocity_0015f928,
        "RiderCameraTarget_GetLandingVelocity",
    ),
    (
        0x0015_fd48,
        manual_cam_algo_reset_for_rider_0015fd48,
        "ManualCamAlgo_ResetForRider",
    ),
    (
        0x0016_04a0,
        manual_chase_camera_algorithm_get_eye_drop_001604a0,
        "ManualChaseCameraAlgorithm_GetEyeDrop",
    ),
    (
        0x0016_04b0,
        manual_chase_camera_algorithm_get_tuning_001604b0,
        "ManualChaseCameraAlgorithm_GetTuning",
    ),
    (
        0x0016_07c8,
        matrix_chase_camera_algorithm_get_eye_drop_001607c8,
        "MatrixChaseCameraAlgorithm_GetEyeDrop",
    ),
];

/// `leaf_0014585c` (0x0014_585c): straight-line leaf. Mechanical port of the generated body.
pub fn leaf_0014585c_0014585c(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0x3;
    ret(c);
}

/// `leaf_001458ec` (0x0014_58ec): straight-line leaf. Mechanical port of the generated body.
pub fn leaf_001458ec(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `leaf_001458f4` (0x0014_58f4): straight-line leaf. Mechanical port of the generated body.
pub fn leaf_001458f4(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0x1;
    ret(c);
}

/// `leaf_001458fc` (0x0014_58fc): straight-line leaf. Mechanical port of the generated body.
pub fn leaf_001458fc(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0x3;
    ret(c);
}

/// `leaf_00145904` (0x0014_5904): straight-line leaf. Mechanical port of the generated body.
pub fn leaf_00145904(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0x2;
    ret(c);
}

/// `leaf_0014590c` (0x0014_590c): straight-line leaf. Mechanical port of the generated body.
pub fn leaf_0014590c(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0x5;
    ret(c);
}

/// `leaf_00145914` (0x0014_5914): straight-line leaf. Mechanical port of the generated body.
pub fn leaf_00145914(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0x6;
    ret(c);
}

/// `leaf_0014591c` (0x0014_591c): straight-line leaf. Mechanical port of the generated body.
pub fn leaf_0014591c(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0x4;
    ret(c);
}

/// `leaf_00145924` (0x0014_5924): straight-line leaf. Mechanical port of the generated body.
pub fn leaf_00145924(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `BEScoreInt_Commit` (0x0015_49e0): straight-line leaf. Mechanical port of the generated body.
pub fn be_score_int_commit_001549e0(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `BEScoreInt_SaveState` (0x0015_5380): straight-line leaf. Mechanical port of the generated body.
pub fn be_score_int_save_state_00155380(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `BEScoreInt_LoadState` (0x0015_5388): straight-line leaf. Mechanical port of the generated body.
pub fn be_score_int_load_state_00155388(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `BEAggressionInt_Commit` (0x0015_5aa0): straight-line leaf. Mechanical port of the generated body.
pub fn be_aggression_int_commit_00155aa0(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `BEAggressionInt_Reset` (0x0015_5aa8): straight-line leaf. Mechanical port of the generated body.
pub fn be_aggression_int_reset_00155aa8(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `DebugPrintf_Stub_afe8` (0x0015_afe8): straight-line leaf: 11 store(s). Mechanical port of the generated body.
pub fn debug_printf_stub_afe8(c: &mut Cpu) {
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

/// `BEInterface_Commit` (0x0015_bb00): straight-line leaf. Mechanical port of the generated body.
pub fn be_interface_commit_0015bb00(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `BEInterface_Reset` (0x0015_bb08): straight-line leaf. Mechanical port of the generated body.
pub fn be_interface_reset_0015bb08(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `BEInterface_SaveState` (0x0015_bb10): straight-line leaf. Mechanical port of the generated body.
pub fn be_interface_save_state_0015bb10(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `BEInterface_LoadState` (0x0015_bb18): straight-line leaf. Mechanical port of the generated body.
pub fn be_interface_load_state_0015bb18(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `BEMissionInt_Commit` (0x0015_bcc0): straight-line leaf. Mechanical port of the generated body.
pub fn be_mission_int_commit_0015bcc0(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `BEMissionInt_Reset` (0x0015_bcc8): straight-line leaf. Mechanical port of the generated body.
pub fn be_mission_int_reset_0015bcc8(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `BECharDBInt_Commit` (0x0015_c6c8): straight-line leaf. Mechanical port of the generated body.
pub fn be_char_db_int_commit_0015c6c8(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `BECharDBInt_Reset` (0x0015_c6d0): straight-line leaf. Mechanical port of the generated body.
pub fn be_char_db_int_reset_0015c6d0(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `BESaveInt_Commit` (0x0015_c6d8): straight-line leaf. Mechanical port of the generated body.
pub fn be_save_int_commit_0015c6d8(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `BESaveInt_Reset` (0x0015_c6e0): straight-line leaf. Mechanical port of the generated body.
pub fn be_save_int_reset_0015c6e0(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `BEEconInt_Commit` (0x0015_c7d0): straight-line leaf. Mechanical port of the generated body.
pub fn be_econ_int_commit_0015c7d0(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `BEEconInt_Reset` (0x0015_c7d8): straight-line leaf. Mechanical port of the generated body.
pub fn be_econ_int_reset_0015c7d8(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `RiderCameraTarget_GetSteer` (0x0015_f640): straight-line leaf: 2 load(s). Mechanical port of the generated body.
pub fn rider_camera_target_get_steer_0015f640(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
    c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x1f0));
    ret(c);
}

/// `RiderCameraTarget_GetHandplantData0` (0x0015_f680): straight-line leaf: 2 load(s). Mechanical port of the generated body.
pub fn rider_camera_target_get_handplant_data0_0015f680(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x77c));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x130));
    ret(c);
}

/// `RiderCameraTarget_GetHandplantData1` (0x0015_f690): straight-line leaf: 2 load(s). Mechanical port of the generated body.
pub fn rider_camera_target_get_handplant_data1_0015f690(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x77c));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x140));
    ret(c);
}

/// `RiderCameraTarget_GetAirTime` (0x0015_f6a0): straight-line leaf: 3 load(s). Mechanical port of the generated body.
pub fn rider_camera_target_get_air_time_0015f6a0(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
    c.g[V1][0] = c.lw((c.g[V0][0] as u32).wrapping_add(0x788));
    c.lwc1(0, (c.g[V1][0] as u32).wrapping_add(0x98));
    ret(c);
}

/// `RiderCameraTarget_GetJumpOrigin` (0x0015_f6e0): straight-line leaf: 2 load(s). Mechanical port of the generated body.
pub fn rider_camera_target_get_jump_origin_0015f6e0(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x788));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x40));
    ret(c);
}

/// `RiderCameraTarget_GetJumpDir` (0x0015_f6f0): straight-line leaf: 1 load(s). Mechanical port of the generated body.
pub fn rider_camera_target_get_jump_dir_0015f6f0(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x3c0));
    ret(c);
}

/// `RiderCameraTarget_GetBoost` (0x0015_f700): straight-line leaf: 2 load(s). Mechanical port of the generated body.
pub fn rider_camera_target_get_boost_0015f700(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
    c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x2fc));
    ret(c);
}

/// `RiderCameraTarget_GetSlopeTerm` (0x0015_f710): straight-line leaf: 2 load(s). Mechanical port of the generated body.
pub fn rider_camera_target_get_slope_term_0015f710(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
    c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x220));
    ret(c);
}

/// `RiderCameraTarget_GetSurfaceType` (0x0015_f750): straight-line leaf: 2 load(s). Mechanical port of the generated body.
pub fn rider_camera_target_get_surface_type_0015f750(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x438));
    ret(c);
}

/// `RiderCameraTarget_GetTakeoffPop` (0x0015_f760): straight-line leaf: 2 load(s). Mechanical port of the generated body.
pub fn rider_camera_target_get_takeoff_pop_0015f760(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
    c.lwc1(0, (c.g[V0][0] as u32).wrapping_add(0x5a4));
    ret(c);
}

/// `RiderCameraTarget_IsOllie` (0x0015_f770): straight-line leaf: 2 load(s). Mechanical port of the generated body.
pub fn rider_camera_target_is_ollie_0015f770(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x5a8));
    ret(c);
}

/// `RiderCameraTarget_GetRailFlag` (0x0015_f908): straight-line leaf: 2 load(s). Mechanical port of the generated body.
pub fn rider_camera_target_get_rail_flag_0015f908(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x5ac));
    ret(c);
}

/// `RiderCameraTarget_GetLandingNormal` (0x0015_f918): straight-line leaf: 2 load(s). Mechanical port of the generated body.
pub fn rider_camera_target_get_landing_normal_0015f918(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x788));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x20));
    ret(c);
}

/// `RiderCameraTarget_GetLandingVelocity` (0x0015_f928): straight-line leaf: 2 load(s). Mechanical port of the generated body.
pub fn rider_camera_target_get_landing_velocity_0015f928(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V1][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4));
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x788));
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x10));
    ret(c);
}

/// `ManualCamAlgo_ResetForRider` (0x0015_fd48): straight-line leaf. Mechanical port of the generated body.
pub fn manual_cam_algo_reset_for_rider_0015fd48(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `ManualChaseCameraAlgorithm_GetEyeDrop` (0x0016_04a0): straight-line leaf. Mechanical port of the generated body.
pub fn manual_chase_camera_algorithm_get_eye_drop_001604a0(c: &mut Cpu) {
    c.cycles += 3;
    c.mtc1(0, 0);
    ret(c);
}

/// `ManualChaseCameraAlgorithm_GetTuning` (0x0016_04b0): straight-line leaf: 1 store(s). Mechanical port of the generated body.
pub fn manual_chase_camera_algorithm_get_tuning_001604b0(c: &mut Cpu) {
    c.cycles += 2;
    c.sw((c.g[A1][0] as u32).wrapping_add(0x10), 0u64);
    ret(c);
}

/// `MatrixChaseCameraAlgorithm_GetEyeDrop` (0x0016_07c8): straight-line leaf. Mechanical port of the generated body.
pub fn matrix_chase_camera_algorithm_get_eye_drop_001607c8(c: &mut Cpu) {
    c.cycles += 3;
    c.mtc1(0, 0);
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

    /// `leaf_0014585c` (0x0014_585c) against the generated body.
    #[test]
    fn leaf_0014585c_0014585c_matches() {
        check(
            ee::s01::f_0014585c__generated,
            leaf_0014585c_0014585c,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x14585c, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `leaf_001458ec` (0x0014_58ec) against the generated body.
    #[test]
    fn leaf_001458ec_matches() {
        check(
            ee::s01::f_001458ec__generated,
            leaf_001458ec,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1458ec, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `leaf_001458f4` (0x0014_58f4) against the generated body.
    #[test]
    fn leaf_001458f4_matches() {
        check(
            ee::s01::f_001458f4__generated,
            leaf_001458f4,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1458f4, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `leaf_001458fc` (0x0014_58fc) against the generated body.
    #[test]
    fn leaf_001458fc_matches() {
        check(
            ee::s01::f_001458fc__generated,
            leaf_001458fc,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1458fc, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `leaf_00145904` (0x0014_5904) against the generated body.
    #[test]
    fn leaf_00145904_matches() {
        check(
            ee::s01::f_00145904__generated,
            leaf_00145904,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x145904, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `leaf_0014590c` (0x0014_590c) against the generated body.
    #[test]
    fn leaf_0014590c_matches() {
        check(
            ee::s01::f_0014590c__generated,
            leaf_0014590c,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x14590c, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `leaf_00145914` (0x0014_5914) against the generated body.
    #[test]
    fn leaf_00145914_matches() {
        check(
            ee::s01::f_00145914__generated,
            leaf_00145914,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x145914, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `leaf_0014591c` (0x0014_591c) against the generated body.
    #[test]
    fn leaf_0014591c_matches() {
        check(
            ee::s01::f_0014591c__generated,
            leaf_0014591c,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x14591c, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `leaf_00145924` (0x0014_5924) against the generated body.
    #[test]
    fn leaf_00145924_matches() {
        check(
            ee::s01::f_00145924__generated,
            leaf_00145924,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x145924, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `BEScoreInt_Commit` (0x0015_49e0) against the generated body.
    #[test]
    fn be_score_int_commit_001549e0_matches() {
        check(
            ee::s01::f_001549e0__generated,
            be_score_int_commit_001549e0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1549e0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `BEScoreInt_SaveState` (0x0015_5380) against the generated body.
    #[test]
    fn be_score_int_save_state_00155380_matches() {
        check(
            ee::s01::f_00155380__generated,
            be_score_int_save_state_00155380,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x155380, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `BEScoreInt_LoadState` (0x0015_5388) against the generated body.
    #[test]
    fn be_score_int_load_state_00155388_matches() {
        check(
            ee::s01::f_00155388__generated,
            be_score_int_load_state_00155388,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x155388, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `BEAggressionInt_Commit` (0x0015_5aa0) against the generated body.
    #[test]
    fn be_aggression_int_commit_00155aa0_matches() {
        check(
            ee::s01::f_00155aa0__generated,
            be_aggression_int_commit_00155aa0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x155aa0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `BEAggressionInt_Reset` (0x0015_5aa8) against the generated body.
    #[test]
    fn be_aggression_int_reset_00155aa8_matches() {
        check(
            ee::s01::f_00155aa8__generated,
            be_aggression_int_reset_00155aa8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x155aa8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `DebugPrintf_Stub_afe8` (0x0015_afe8) against the generated body.
    #[test]
    fn debug_printf_stub_afe8_matches() {
        check(
            ee::s01::f_0015afe8__generated,
            debug_printf_stub_afe8,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x15afe8, 0);
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

    /// `BEInterface_Commit` (0x0015_bb00) against the generated body.
    #[test]
    fn be_interface_commit_0015bb00_matches() {
        check(
            ee::s01::f_0015bb00__generated,
            be_interface_commit_0015bb00,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x15bb00, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `BEInterface_Reset` (0x0015_bb08) against the generated body.
    #[test]
    fn be_interface_reset_0015bb08_matches() {
        check(
            ee::s01::f_0015bb08__generated,
            be_interface_reset_0015bb08,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x15bb08, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `BEInterface_SaveState` (0x0015_bb10) against the generated body.
    #[test]
    fn be_interface_save_state_0015bb10_matches() {
        check(
            ee::s01::f_0015bb10__generated,
            be_interface_save_state_0015bb10,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x15bb10, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `BEInterface_LoadState` (0x0015_bb18) against the generated body.
    #[test]
    fn be_interface_load_state_0015bb18_matches() {
        check(
            ee::s01::f_0015bb18__generated,
            be_interface_load_state_0015bb18,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x15bb18, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `BEMissionInt_Commit` (0x0015_bcc0) against the generated body.
    #[test]
    fn be_mission_int_commit_0015bcc0_matches() {
        check(
            ee::s01::f_0015bcc0__generated,
            be_mission_int_commit_0015bcc0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x15bcc0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `BEMissionInt_Reset` (0x0015_bcc8) against the generated body.
    #[test]
    fn be_mission_int_reset_0015bcc8_matches() {
        check(
            ee::s01::f_0015bcc8__generated,
            be_mission_int_reset_0015bcc8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x15bcc8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `BECharDBInt_Commit` (0x0015_c6c8) against the generated body.
    #[test]
    fn be_char_db_int_commit_0015c6c8_matches() {
        check(
            ee::s01::f_0015c6c8__generated,
            be_char_db_int_commit_0015c6c8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x15c6c8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `BECharDBInt_Reset` (0x0015_c6d0) against the generated body.
    #[test]
    fn be_char_db_int_reset_0015c6d0_matches() {
        check(
            ee::s01::f_0015c6d0__generated,
            be_char_db_int_reset_0015c6d0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x15c6d0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `BESaveInt_Commit` (0x0015_c6d8) against the generated body.
    #[test]
    fn be_save_int_commit_0015c6d8_matches() {
        check(
            ee::s01::f_0015c6d8__generated,
            be_save_int_commit_0015c6d8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x15c6d8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `BESaveInt_Reset` (0x0015_c6e0) against the generated body.
    #[test]
    fn be_save_int_reset_0015c6e0_matches() {
        check(
            ee::s01::f_0015c6e0__generated,
            be_save_int_reset_0015c6e0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x15c6e0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `BEEconInt_Commit` (0x0015_c7d0) against the generated body.
    #[test]
    fn be_econ_int_commit_0015c7d0_matches() {
        check(
            ee::s01::f_0015c7d0__generated,
            be_econ_int_commit_0015c7d0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x15c7d0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `BEEconInt_Reset` (0x0015_c7d8) against the generated body.
    #[test]
    fn be_econ_int_reset_0015c7d8_matches() {
        check(
            ee::s01::f_0015c7d8__generated,
            be_econ_int_reset_0015c7d8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x15c7d8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `RiderCameraTarget_GetSteer` (0x0015_f640) against the generated body.
    #[test]
    fn rider_camera_target_get_steer_0015f640_matches() {
        check(
            ee::s01::f_0015f640__generated,
            rider_camera_target_get_steer_0015f640,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x15f640, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, true, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `RiderCameraTarget_GetHandplantData0` (0x0015_f680) against the generated body.
    #[test]
    fn rider_camera_target_get_handplant_data0_0015f680_matches() {
        check(
            ee::s01::f_0015f680__generated,
            rider_camera_target_get_handplant_data0_0015f680,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x15f680, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, true, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `RiderCameraTarget_GetHandplantData1` (0x0015_f690) against the generated body.
    #[test]
    fn rider_camera_target_get_handplant_data1_0015f690_matches() {
        check(
            ee::s01::f_0015f690__generated,
            rider_camera_target_get_handplant_data1_0015f690,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x15f690, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, true, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `RiderCameraTarget_GetAirTime` (0x0015_f6a0) against the generated body.
    #[test]
    fn rider_camera_target_get_air_time_0015f6a0_matches() {
        check(
            ee::s01::f_0015f6a0__generated,
            rider_camera_target_get_air_time_0015f6a0,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x15f6a0, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, true, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `RiderCameraTarget_GetJumpOrigin` (0x0015_f6e0) against the generated body.
    #[test]
    fn rider_camera_target_get_jump_origin_0015f6e0_matches() {
        check(
            ee::s01::f_0015f6e0__generated,
            rider_camera_target_get_jump_origin_0015f6e0,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x15f6e0, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, true, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `RiderCameraTarget_GetJumpDir` (0x0015_f6f0) against the generated body.
    #[test]
    fn rider_camera_target_get_jump_dir_0015f6f0_matches() {
        check(
            ee::s01::f_0015f6f0__generated,
            rider_camera_target_get_jump_dir_0015f6f0,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x15f6f0, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `RiderCameraTarget_GetBoost` (0x0015_f700) against the generated body.
    #[test]
    fn rider_camera_target_get_boost_0015f700_matches() {
        check(
            ee::s01::f_0015f700__generated,
            rider_camera_target_get_boost_0015f700,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x15f700, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, true, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `RiderCameraTarget_GetSlopeTerm` (0x0015_f710) against the generated body.
    #[test]
    fn rider_camera_target_get_slope_term_0015f710_matches() {
        check(
            ee::s01::f_0015f710__generated,
            rider_camera_target_get_slope_term_0015f710,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x15f710, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, true, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `RiderCameraTarget_GetSurfaceType` (0x0015_f750) against the generated body.
    #[test]
    fn rider_camera_target_get_surface_type_0015f750_matches() {
        check(
            ee::s01::f_0015f750__generated,
            rider_camera_target_get_surface_type_0015f750,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x15f750, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, true, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `RiderCameraTarget_GetTakeoffPop` (0x0015_f760) against the generated body.
    #[test]
    fn rider_camera_target_get_takeoff_pop_0015f760_matches() {
        check(
            ee::s01::f_0015f760__generated,
            rider_camera_target_get_takeoff_pop_0015f760,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x15f760, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, true, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `RiderCameraTarget_IsOllie` (0x0015_f770) against the generated body.
    #[test]
    fn rider_camera_target_is_ollie_0015f770_matches() {
        check(
            ee::s01::f_0015f770__generated,
            rider_camera_target_is_ollie_0015f770,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x15f770, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, true, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `RiderCameraTarget_GetRailFlag` (0x0015_f908) against the generated body.
    #[test]
    fn rider_camera_target_get_rail_flag_0015f908_matches() {
        check(
            ee::s01::f_0015f908__generated,
            rider_camera_target_get_rail_flag_0015f908,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x15f908, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, true, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `RiderCameraTarget_GetLandingNormal` (0x0015_f918) against the generated body.
    #[test]
    fn rider_camera_target_get_landing_normal_0015f918_matches() {
        check(
            ee::s01::f_0015f918__generated,
            rider_camera_target_get_landing_normal_0015f918,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x15f918, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, true, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `RiderCameraTarget_GetLandingVelocity` (0x0015_f928) against the generated body.
    #[test]
    fn rider_camera_target_get_landing_velocity_0015f928_matches() {
        check(
            ee::s01::f_0015f928__generated,
            rider_camera_target_get_landing_velocity_0015f928,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x15f928, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, true, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `ManualCamAlgo_ResetForRider` (0x0015_fd48) against the generated body.
    #[test]
    fn manual_cam_algo_reset_for_rider_0015fd48_matches() {
        check(
            ee::s01::f_0015fd48__generated,
            manual_cam_algo_reset_for_rider_0015fd48,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x15fd48, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `ManualChaseCameraAlgorithm_GetEyeDrop` (0x0016_04a0) against the generated body.
    #[test]
    fn manual_chase_camera_algorithm_get_eye_drop_001604a0_matches() {
        check(
            ee::s01::f_001604a0__generated,
            manual_chase_camera_algorithm_get_eye_drop_001604a0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1604a0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `ManualChaseCameraAlgorithm_GetTuning` (0x0016_04b0) against the generated body.
    #[test]
    fn manual_chase_camera_algorithm_get_tuning_001604b0_matches() {
        check(
            ee::s01::f_001604b0__generated,
            manual_chase_camera_algorithm_get_tuning_001604b0,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1604b0, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A1][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `MatrixChaseCameraAlgorithm_GetEyeDrop` (0x0016_07c8) against the generated body.
    #[test]
    fn matrix_chase_camera_algorithm_get_eye_drop_001607c8_matches() {
        check(
            ee::s01::f_001607c8__generated,
            matrix_chase_camera_algorithm_get_eye_drop_001607c8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1607c8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }
}
