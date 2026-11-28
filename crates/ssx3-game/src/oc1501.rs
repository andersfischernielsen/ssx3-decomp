//! Staging module oc1501 (wave wave276): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1501`.

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
        0x0011_3cb8,
        race_state_game_init_enter_00113cb8,
        "RaceStateGameInit_Enter",
    ),
    (
        0x0011_3ce0,
        race_state_game_init_exit_00113ce0,
        "RaceStateGameInit_Exit",
    ),
    (
        0x0011_3ce8,
        race_state_freeride_enter_00113ce8,
        "RaceStateFreeride_Enter",
    ),
    (
        0x0011_3d10,
        race_state_freeride_exit_00113d10,
        "RaceStateFreeride_Exit",
    ),
    (
        0x0011_3d18,
        race_state_pre_race_enter_00113d18,
        "RaceStatePreRace_Enter",
    ),
    (
        0x0011_3d28,
        race_state_pre_race_update_00113d28,
        "RaceStatePreRace_Update",
    ),
    (
        0x0011_3d30,
        race_state_pre_race_exit_00113d30,
        "RaceStatePreRace_Exit",
    ),
    (
        0x0011_3d38,
        race_state_countdown_enter_00113d38,
        "RaceStateCountdown_Enter",
    ),
    (
        0x0011_3d80,
        race_state_countdown_exit_00113d80,
        "RaceStateCountdown_Exit",
    ),
    (
        0x0011_3e40,
        race_state_end_race_update_00113e40,
        "RaceStateEndRace_Update",
    ),
    (
        0x0011_3e48,
        race_state_end_race_exit_00113e48,
        "RaceStateEndRace_Exit",
    ),
    (
        0x0011_3e70,
        race_state_shutdown_update_00113e70,
        "RaceStateShutdown_Update",
    ),
    (
        0x0011_3e78,
        race_state_shutdown_exit_00113e78,
        "RaceStateShutdown_Exit",
    ),
    (
        0x0012_78c0,
        player_reset_rumble_001278c0,
        "Player_ResetRumble",
    ),
    (
        0x0012_78e0,
        player_set_rumble_level_001278e0,
        "Player_SetRumbleLevel",
    ),
    (0x0013_f8b8, debug_printf_stub_f8b8, "DebugPrintf_Stub_f8b8"),
    (
        0x0014_06b0,
        rider_iface_get_velocity_001406b0,
        "RiderIface_GetVelocity",
    ),
    (
        0x0014_06c0,
        rider_iface_has_visited_001406c0,
        "RiderIface_HasVisited",
    ),
    (
        0x0014_06c8,
        rider_iface_add_velocity_001406c8,
        "RiderIface_AddVelocity",
    ),
    (
        0x0014_06d0,
        rider_iface_get_position_001406d0,
        "RiderIface_GetPosition",
    ),
    (
        0x0014_06e0,
        rider_get_default_vector_001406e0,
        "Rider_GetDefaultVector",
    ),
    (
        0x0014_06f0,
        rider_iface_get_index_001406f0,
        "RiderIface_GetIndex",
    ),
    (
        0x0014_06f8,
        rider_iface_is_human_001406f8,
        "RiderIface_IsHuman",
    ),
    (
        0x0014_0700,
        rider_iface_is_computer_00140700,
        "RiderIface_IsComputer",
    ),
    (
        0x0014_0708,
        rider_iface_reset_at_matrix_00140708,
        "RiderIface_ResetAtMatrix",
    ),
    (0x0014_0710, rider_v11_empty_00140710, "Rider_v11_Empty"),
    (
        0x0014_0718,
        rider_iface_place_at_matrix_00140718,
        "RiderIface_PlaceAtMatrix",
    ),
    (
        0x0014_0720,
        rider_iface_add_power_up_boost_00140720,
        "RiderIface_AddPowerUpBoost",
    ),
    (
        0x0014_0728,
        rider_iface_add_power_up_trick_00140728,
        "RiderIface_AddPowerUpTrick",
    ),
    (
        0x0014_0730,
        rider_iface_add_trick_multiplier_00140730,
        "RiderIface_AddTrickMultiplier",
    ),
    (
        0x0014_08f0,
        rider_get_position_001408f0,
        "Rider_GetPosition",
    ),
    (
        0x0014_0910,
        rider_get_velocity_00140910,
        "Rider_GetVelocity",
    ),
    (0x0014_0b80, rider_get_index_00140b80, "Rider_GetIndex"),
    (0x0014_0bc0, rider_is_human_00140bc0, "Rider_IsHuman"),
    (
        0x0014_0c68,
        rider_reset_rumble_00140c68,
        "Rider_ResetRumble",
    ),
    (
        0x0014_0c70,
        rider_pulse_rumble_00140c70,
        "Rider_PulseRumble",
    ),
    (
        0x0014_0c78,
        rider_set_rumble_level_00140c78,
        "Rider_SetRumbleLevel",
    ),
    (
        0x0014_0c80,
        rider_update_rumble_00140c80,
        "Rider_UpdateRumble",
    ),
    (
        0x0014_0ce0,
        rider_script_obj_get_self_00140ce0,
        "RiderScriptObj_GetSelf",
    ),
    (
        0x0014_4068,
        race_state_race_exit_00144068,
        "RaceStateRace_Exit",
    ),
    (
        0x0014_46e8,
        player_score_path_branch_001446e8,
        "Player_ScorePathBranch",
    ),
    (0x0014_583c, leaf_0014583c_0014583c, "leaf_0014583c"),
    (0x0014_5844, leaf_00145844_00145844, "leaf_00145844"),
    (0x0014_584c, leaf_0014584c_0014584c, "leaf_0014584c"),
    (0x0014_5854, leaf_00145854_00145854, "leaf_00145854"),
];

/// `RaceStateGameInit_Enter` (0x0011_3cb8): straight-line leaf. Mechanical port of the generated body.
pub fn race_state_game_init_enter_00113cb8(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `RaceStateGameInit_Exit` (0x0011_3ce0): straight-line leaf. Mechanical port of the generated body.
pub fn race_state_game_init_exit_00113ce0(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `RaceStateFreeride_Enter` (0x0011_3ce8): straight-line leaf: 1 store(s). Mechanical port of the generated body.
pub fn race_state_freeride_enter_00113ce8(c: &mut Cpu) {
    c.cycles += 2;
    c.sw((c.g[A1][0] as u32).wrapping_add(0x14), 0u64);
    ret(c);
}

/// `RaceStateFreeride_Exit` (0x0011_3d10): straight-line leaf. Mechanical port of the generated body.
pub fn race_state_freeride_exit_00113d10(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `RaceStatePreRace_Enter` (0x0011_3d18): straight-line leaf: 2 store(s). Mechanical port of the generated body.
pub fn race_state_pre_race_enter_00113d18(c: &mut Cpu) {
    c.cycles += 3;
    c.sw((c.g[A1][0] as u32).wrapping_add(0xc), 0u64);
    c.sw((c.g[A0][0] as u32).wrapping_add(0x4), 0u64);
    ret(c);
}

/// `RaceStatePreRace_Update` (0x0011_3d28): straight-line leaf. Mechanical port of the generated body.
pub fn race_state_pre_race_update_00113d28(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `RaceStatePreRace_Exit` (0x0011_3d30): straight-line leaf. Mechanical port of the generated body.
pub fn race_state_pre_race_exit_00113d30(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `RaceStateCountdown_Enter` (0x0011_3d38): straight-line leaf: 1 store(s). Mechanical port of the generated body.
pub fn race_state_countdown_enter_00113d38(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = 0xb4;
    c.sw((c.g[A1][0] as u32).wrapping_add(0x1c), c.g[V0][0]);
    ret(c);
}

/// `RaceStateCountdown_Exit` (0x0011_3d80): straight-line leaf: 1 store(s). Mechanical port of the generated body.
pub fn race_state_countdown_exit_00113d80(c: &mut Cpu) {
    c.cycles += 2;
    c.sw((c.g[A1][0] as u32).wrapping_add(0x1c), 0u64);
    ret(c);
}

/// `RaceStateEndRace_Update` (0x0011_3e40): straight-line leaf. Mechanical port of the generated body.
pub fn race_state_end_race_update_00113e40(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `RaceStateEndRace_Exit` (0x0011_3e48): straight-line leaf. Mechanical port of the generated body.
pub fn race_state_end_race_exit_00113e48(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `RaceStateShutdown_Update` (0x0011_3e70): straight-line leaf. Mechanical port of the generated body.
pub fn race_state_shutdown_update_00113e70(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `RaceStateShutdown_Exit` (0x0011_3e78): straight-line leaf. Mechanical port of the generated body.
pub fn race_state_shutdown_exit_00113e78(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `Player_ResetRumble` (0x0012_78c0): straight-line leaf: 2 store(s). Mechanical port of the generated body.
pub fn player_reset_rumble_001278c0(c: &mut Cpu) {
    c.cycles += 3;
    c.sw((c.g[A0][0] as u32).wrapping_add(0xdfc), 0u64);
    c.sw((c.g[A0][0] as u32).wrapping_add(0xe00), 0u64);
    ret(c);
}

/// `Player_SetRumbleLevel` (0x0012_78e0): straight-line leaf: 1 store(s). Mechanical port of the generated body.
pub fn player_set_rumble_level_001278e0(c: &mut Cpu) {
    c.cycles += 2;
    c.swc1(12, (c.g[A0][0] as u32).wrapping_add(0xe00));
    ret(c);
}

/// `DebugPrintf_Stub_f8b8` (0x0013_f8b8): straight-line leaf: 11 store(s). Mechanical port of the generated body.
pub fn debug_printf_stub_f8b8(c: &mut Cpu) {
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

/// `RiderIface_GetVelocity` (0x0014_06b0): straight-line leaf. Mechanical port of the generated body.
pub fn rider_iface_get_velocity_001406b0(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = 0x500000;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xee0));
    ret(c);
}

/// `RiderIface_HasVisited` (0x0014_06c0): straight-line leaf. Mechanical port of the generated body.
pub fn rider_iface_has_visited_001406c0(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `RiderIface_AddVelocity` (0x0014_06c8): straight-line leaf. Mechanical port of the generated body.
pub fn rider_iface_add_velocity_001406c8(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `RiderIface_GetPosition` (0x0014_06d0): straight-line leaf. Mechanical port of the generated body.
pub fn rider_iface_get_position_001406d0(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = 0x500000;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xee0));
    ret(c);
}

/// `Rider_GetDefaultVector` (0x0014_06e0): straight-line leaf. Mechanical port of the generated body.
pub fn rider_get_default_vector_001406e0(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = 0x500000;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0xe60));
    ret(c);
}

/// `RiderIface_GetIndex` (0x0014_06f0): straight-line leaf. Mechanical port of the generated body.
pub fn rider_iface_get_index_001406f0(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `RiderIface_IsHuman` (0x0014_06f8): straight-line leaf. Mechanical port of the generated body.
pub fn rider_iface_is_human_001406f8(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `RiderIface_IsComputer` (0x0014_0700): straight-line leaf. Mechanical port of the generated body.
pub fn rider_iface_is_computer_00140700(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `RiderIface_ResetAtMatrix` (0x0014_0708): straight-line leaf. Mechanical port of the generated body.
pub fn rider_iface_reset_at_matrix_00140708(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `Rider_v11_Empty` (0x0014_0710): straight-line leaf. Mechanical port of the generated body.
pub fn rider_v11_empty_00140710(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `RiderIface_PlaceAtMatrix` (0x0014_0718): straight-line leaf. Mechanical port of the generated body.
pub fn rider_iface_place_at_matrix_00140718(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `RiderIface_AddPowerUpBoost` (0x0014_0720): straight-line leaf. Mechanical port of the generated body.
pub fn rider_iface_add_power_up_boost_00140720(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `RiderIface_AddPowerUpTrick` (0x0014_0728): straight-line leaf. Mechanical port of the generated body.
pub fn rider_iface_add_power_up_trick_00140728(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `RiderIface_AddTrickMultiplier` (0x0014_0730): straight-line leaf. Mechanical port of the generated body.
pub fn rider_iface_add_trick_multiplier_00140730(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `Rider_GetPosition` (0x0014_08f0): straight-line leaf. Mechanical port of the generated body.
pub fn rider_get_position_001408f0(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x110));
    ret(c);
}

/// `Rider_GetVelocity` (0x0014_0910): straight-line leaf. Mechanical port of the generated body.
pub fn rider_get_velocity_00140910(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x1e0));
    ret(c);
}

/// `Rider_GetIndex` (0x0014_0b80): straight-line leaf: 1 load(s). Mechanical port of the generated body.
pub fn rider_get_index_00140b80(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x86c));
    ret(c);
}

/// `Rider_IsHuman` (0x0014_0bc0): straight-line leaf: 1 load(s). Mechanical port of the generated body.
pub fn rider_is_human_00140bc0(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x874));
    ret(c);
}

/// `Rider_ResetRumble` (0x0014_0c68): straight-line leaf. Mechanical port of the generated body.
pub fn rider_reset_rumble_00140c68(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `Rider_PulseRumble` (0x0014_0c70): straight-line leaf. Mechanical port of the generated body.
pub fn rider_pulse_rumble_00140c70(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `Rider_SetRumbleLevel` (0x0014_0c78): straight-line leaf. Mechanical port of the generated body.
pub fn rider_set_rumble_level_00140c78(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `Rider_UpdateRumble` (0x0014_0c80): straight-line leaf. Mechanical port of the generated body.
pub fn rider_update_rumble_00140c80(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `RiderScriptObj_GetSelf` (0x0014_0ce0): straight-line leaf. Mechanical port of the generated body.
pub fn rider_script_obj_get_self_00140ce0(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = c.g[A0][0];
    ret(c);
}

/// `RaceStateRace_Exit` (0x0014_4068): straight-line leaf. Mechanical port of the generated body.
pub fn race_state_race_exit_00144068(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `Player_ScorePathBranch` (0x0014_46e8): straight-line leaf. Mechanical port of the generated body.
pub fn player_score_path_branch_001446e8(c: &mut Cpu) {
    c.cycles += 3;
    c.mtc1(0, 0);
    ret(c);
}

/// `leaf_0014583c` (0x0014_583c): straight-line leaf. Mechanical port of the generated body.
pub fn leaf_0014583c_0014583c(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0x4;
    ret(c);
}

/// `leaf_00145844` (0x0014_5844): straight-line leaf. Mechanical port of the generated body.
pub fn leaf_00145844_00145844(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `leaf_0014584c` (0x0014_584c): straight-line leaf. Mechanical port of the generated body.
pub fn leaf_0014584c_0014584c(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0x1;
    ret(c);
}

/// `leaf_00145854` (0x0014_5854): straight-line leaf. Mechanical port of the generated body.
pub fn leaf_00145854_00145854(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0x2;
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

    /// `RaceStateGameInit_Enter` (0x0011_3cb8) against the generated body.
    #[test]
    fn race_state_game_init_enter_00113cb8_matches() {
        check(
            ee::s00::f_00113cb8__generated,
            race_state_game_init_enter_00113cb8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x113cb8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `RaceStateGameInit_Exit` (0x0011_3ce0) against the generated body.
    #[test]
    fn race_state_game_init_exit_00113ce0_matches() {
        check(
            ee::s00::f_00113ce0__generated,
            race_state_game_init_exit_00113ce0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x113ce0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `RaceStateFreeride_Enter` (0x0011_3ce8) against the generated body.
    #[test]
    fn race_state_freeride_enter_00113ce8_matches() {
        check(
            ee::s00::f_00113ce8__generated,
            race_state_freeride_enter_00113ce8,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x113ce8, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A1][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `RaceStateFreeride_Exit` (0x0011_3d10) against the generated body.
    #[test]
    fn race_state_freeride_exit_00113d10_matches() {
        check(
            ee::s00::f_00113d10__generated,
            race_state_freeride_exit_00113d10,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x113d10, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `RaceStatePreRace_Enter` (0x0011_3d18) against the generated body.
    #[test]
    fn race_state_pre_race_enter_00113d18_matches() {
        check(
            ee::s00::f_00113d18__generated,
            race_state_pre_race_enter_00113d18,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x113d18, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                leaf_fill(c, &mut r, 0x0080_5000, 0x2000, false, 0x0080_5000);
                c.g[A1][0] = sx(0x0080_5000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `RaceStatePreRace_Update` (0x0011_3d28) against the generated body.
    #[test]
    fn race_state_pre_race_update_00113d28_matches() {
        check(
            ee::s00::f_00113d28__generated,
            race_state_pre_race_update_00113d28,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x113d28, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `RaceStatePreRace_Exit` (0x0011_3d30) against the generated body.
    #[test]
    fn race_state_pre_race_exit_00113d30_matches() {
        check(
            ee::s00::f_00113d30__generated,
            race_state_pre_race_exit_00113d30,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x113d30, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `RaceStateCountdown_Enter` (0x0011_3d38) against the generated body.
    #[test]
    fn race_state_countdown_enter_00113d38_matches() {
        check(
            ee::s00::f_00113d38__generated,
            race_state_countdown_enter_00113d38,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x113d38, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A1][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `RaceStateCountdown_Exit` (0x0011_3d80) against the generated body.
    #[test]
    fn race_state_countdown_exit_00113d80_matches() {
        check(
            ee::s00::f_00113d80__generated,
            race_state_countdown_exit_00113d80,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x113d80, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A1][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `RaceStateEndRace_Update` (0x0011_3e40) against the generated body.
    #[test]
    fn race_state_end_race_update_00113e40_matches() {
        check(
            ee::s00::f_00113e40__generated,
            race_state_end_race_update_00113e40,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x113e40, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `RaceStateEndRace_Exit` (0x0011_3e48) against the generated body.
    #[test]
    fn race_state_end_race_exit_00113e48_matches() {
        check(
            ee::s00::f_00113e48__generated,
            race_state_end_race_exit_00113e48,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x113e48, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `RaceStateShutdown_Update` (0x0011_3e70) against the generated body.
    #[test]
    fn race_state_shutdown_update_00113e70_matches() {
        check(
            ee::s00::f_00113e70__generated,
            race_state_shutdown_update_00113e70,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x113e70, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `RaceStateShutdown_Exit` (0x0011_3e78) against the generated body.
    #[test]
    fn race_state_shutdown_exit_00113e78_matches() {
        check(
            ee::s00::f_00113e78__generated,
            race_state_shutdown_exit_00113e78,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x113e78, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `Player_ResetRumble` (0x0012_78c0) against the generated body.
    #[test]
    fn player_reset_rumble_001278c0_matches() {
        check(
            ee::s00::f_001278c0__generated,
            player_reset_rumble_001278c0,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1278c0, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `Player_SetRumbleLevel` (0x0012_78e0) against the generated body.
    #[test]
    fn player_set_rumble_level_001278e0_matches() {
        check(
            ee::s00::f_001278e0__generated,
            player_set_rumble_level_001278e0,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1278e0, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.f[12] = r.edge_f32();
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `DebugPrintf_Stub_f8b8` (0x0013_f8b8) against the generated body.
    #[test]
    fn debug_printf_stub_f8b8_matches() {
        check(
            ee::s01::f_0013f8b8__generated,
            debug_printf_stub_f8b8,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x13f8b8, 0);
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

    /// `RiderIface_GetVelocity` (0x0014_06b0) against the generated body.
    #[test]
    fn rider_iface_get_velocity_001406b0_matches() {
        check(
            ee::s01::f_001406b0__generated,
            rider_iface_get_velocity_001406b0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1406b0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `RiderIface_HasVisited` (0x0014_06c0) against the generated body.
    #[test]
    fn rider_iface_has_visited_001406c0_matches() {
        check(
            ee::s01::f_001406c0__generated,
            rider_iface_has_visited_001406c0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1406c0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `RiderIface_AddVelocity` (0x0014_06c8) against the generated body.
    #[test]
    fn rider_iface_add_velocity_001406c8_matches() {
        check(
            ee::s01::f_001406c8__generated,
            rider_iface_add_velocity_001406c8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1406c8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `RiderIface_GetPosition` (0x0014_06d0) against the generated body.
    #[test]
    fn rider_iface_get_position_001406d0_matches() {
        check(
            ee::s01::f_001406d0__generated,
            rider_iface_get_position_001406d0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1406d0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `Rider_GetDefaultVector` (0x0014_06e0) against the generated body.
    #[test]
    fn rider_get_default_vector_001406e0_matches() {
        check(
            ee::s01::f_001406e0__generated,
            rider_get_default_vector_001406e0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1406e0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `RiderIface_GetIndex` (0x0014_06f0) against the generated body.
    #[test]
    fn rider_iface_get_index_001406f0_matches() {
        check(
            ee::s01::f_001406f0__generated,
            rider_iface_get_index_001406f0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1406f0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `RiderIface_IsHuman` (0x0014_06f8) against the generated body.
    #[test]
    fn rider_iface_is_human_001406f8_matches() {
        check(
            ee::s01::f_001406f8__generated,
            rider_iface_is_human_001406f8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1406f8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `RiderIface_IsComputer` (0x0014_0700) against the generated body.
    #[test]
    fn rider_iface_is_computer_00140700_matches() {
        check(
            ee::s01::f_00140700__generated,
            rider_iface_is_computer_00140700,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x140700, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `RiderIface_ResetAtMatrix` (0x0014_0708) against the generated body.
    #[test]
    fn rider_iface_reset_at_matrix_00140708_matches() {
        check(
            ee::s01::f_00140708__generated,
            rider_iface_reset_at_matrix_00140708,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x140708, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `Rider_v11_Empty` (0x0014_0710) against the generated body.
    #[test]
    fn rider_v11_empty_00140710_matches() {
        check(
            ee::s01::f_00140710__generated,
            rider_v11_empty_00140710,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x140710, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `RiderIface_PlaceAtMatrix` (0x0014_0718) against the generated body.
    #[test]
    fn rider_iface_place_at_matrix_00140718_matches() {
        check(
            ee::s01::f_00140718__generated,
            rider_iface_place_at_matrix_00140718,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x140718, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `RiderIface_AddPowerUpBoost` (0x0014_0720) against the generated body.
    #[test]
    fn rider_iface_add_power_up_boost_00140720_matches() {
        check(
            ee::s01::f_00140720__generated,
            rider_iface_add_power_up_boost_00140720,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x140720, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `RiderIface_AddPowerUpTrick` (0x0014_0728) against the generated body.
    #[test]
    fn rider_iface_add_power_up_trick_00140728_matches() {
        check(
            ee::s01::f_00140728__generated,
            rider_iface_add_power_up_trick_00140728,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x140728, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `RiderIface_AddTrickMultiplier` (0x0014_0730) against the generated body.
    #[test]
    fn rider_iface_add_trick_multiplier_00140730_matches() {
        check(
            ee::s01::f_00140730__generated,
            rider_iface_add_trick_multiplier_00140730,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x140730, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `Rider_GetPosition` (0x0014_08f0) against the generated body.
    #[test]
    fn rider_get_position_001408f0_matches() {
        check(
            ee::s01::f_001408f0__generated,
            rider_get_position_001408f0,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1408f0, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `Rider_GetVelocity` (0x0014_0910) against the generated body.
    #[test]
    fn rider_get_velocity_00140910_matches() {
        check(
            ee::s01::f_00140910__generated,
            rider_get_velocity_00140910,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x140910, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `Rider_GetIndex` (0x0014_0b80) against the generated body.
    #[test]
    fn rider_get_index_00140b80_matches() {
        check(
            ee::s01::f_00140b80__generated,
            rider_get_index_00140b80,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x140b80, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `Rider_IsHuman` (0x0014_0bc0) against the generated body.
    #[test]
    fn rider_is_human_00140bc0_matches() {
        check(
            ee::s01::f_00140bc0__generated,
            rider_is_human_00140bc0,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x140bc0, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `Rider_ResetRumble` (0x0014_0c68) against the generated body.
    #[test]
    fn rider_reset_rumble_00140c68_matches() {
        check(
            ee::s01::f_00140c68__generated,
            rider_reset_rumble_00140c68,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x140c68, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `Rider_PulseRumble` (0x0014_0c70) against the generated body.
    #[test]
    fn rider_pulse_rumble_00140c70_matches() {
        check(
            ee::s01::f_00140c70__generated,
            rider_pulse_rumble_00140c70,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x140c70, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `Rider_SetRumbleLevel` (0x0014_0c78) against the generated body.
    #[test]
    fn rider_set_rumble_level_00140c78_matches() {
        check(
            ee::s01::f_00140c78__generated,
            rider_set_rumble_level_00140c78,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x140c78, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `Rider_UpdateRumble` (0x0014_0c80) against the generated body.
    #[test]
    fn rider_update_rumble_00140c80_matches() {
        check(
            ee::s01::f_00140c80__generated,
            rider_update_rumble_00140c80,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x140c80, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `RiderScriptObj_GetSelf` (0x0014_0ce0) against the generated body.
    #[test]
    fn rider_script_obj_get_self_00140ce0_matches() {
        check(
            ee::s01::f_00140ce0__generated,
            rider_script_obj_get_self_00140ce0,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x140ce0, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `RaceStateRace_Exit` (0x0014_4068) against the generated body.
    #[test]
    fn race_state_race_exit_00144068_matches() {
        check(
            ee::s01::f_00144068__generated,
            race_state_race_exit_00144068,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x144068, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `Player_ScorePathBranch` (0x0014_46e8) against the generated body.
    #[test]
    fn player_score_path_branch_001446e8_matches() {
        check(
            ee::s01::f_001446e8__generated,
            player_score_path_branch_001446e8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1446e8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `leaf_0014583c` (0x0014_583c) against the generated body.
    #[test]
    fn leaf_0014583c_0014583c_matches() {
        check(
            ee::s01::f_0014583c__generated,
            leaf_0014583c_0014583c,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x14583c, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `leaf_00145844` (0x0014_5844) against the generated body.
    #[test]
    fn leaf_00145844_00145844_matches() {
        check(
            ee::s01::f_00145844__generated,
            leaf_00145844_00145844,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x145844, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `leaf_0014584c` (0x0014_584c) against the generated body.
    #[test]
    fn leaf_0014584c_0014584c_matches() {
        check(
            ee::s01::f_0014584c__generated,
            leaf_0014584c_0014584c,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x14584c, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `leaf_00145854` (0x0014_5854) against the generated body.
    #[test]
    fn leaf_00145854_00145854_matches() {
        check(
            ee::s01::f_00145854__generated,
            leaf_00145854_00145854,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x145854, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }
}
