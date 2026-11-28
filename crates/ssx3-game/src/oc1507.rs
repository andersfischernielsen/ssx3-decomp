//! Staging module oc1507 (wave wave276): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1507`.

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
    (0x001e_1288, ui_input_base_v42_001e1288, "UIInputBase_v42"),
    (0x001e_1290, ui_input_base_v43_001e1290, "UIInputBase_v43"),
    (
        0x001e_1298,
        ui_input_base_set_port_001e1298,
        "UIInputBase_SetPort",
    ),
    (0x001e_12a0, ui_input_base_v45_001e12a0, "UIInputBase_v45"),
    (
        0x001e_12a8,
        ui_input_base_poll_usb_keyboard_001e12a8,
        "UIInputBase_PollUsbKeyboard",
    ),
    (
        0x001e_7920,
        ov_screen_final_standings_on_deactivate_001e7920,
        "OVScreenFinalStandings_OnDeactivate",
    ),
    (
        0x001e_8e90,
        ov_screen_peak_challenge_results_handle_input_001e8e90,
        "OVScreenPeakChallengeResults_HandleInput",
    ),
    (
        0x001f_8720,
        ov_screen_pause_freeride_on_deactivate_001f8720,
        "OVScreenPauseFreeride_OnDeactivate",
    ),
    (
        0x001f_8df0,
        ov_screen_pause_freeride_handle_input_001f8df0,
        "OVScreenPauseFreeride_HandleInput",
    ),
    (
        0x001f_af08,
        ov_screen_in_game_error_transition_in_001faf08,
        "OVScreenInGameError_TransitionIn",
    ),
    (
        0x0020_87e8,
        ov_screen_session_on_widget_created_002087e8,
        "OVScreenSession_OnWidgetCreated",
    ),
    (
        0x0021_22e0,
        ov_screen_autosave_handle_input_002122e0,
        "OVScreenAutosave_HandleInput",
    ),
    (
        0x0021_c700,
        c_ov_state_overlay_v25_0021c700,
        "cOVStateOverlay_v25",
    ),
    (
        0x0021_cb68,
        c_ov_state_view_popup_transition_out_0021cb68,
        "cOVState_ViewPopup_TransitionOut",
    ),
    (0x0022_12c0, ov_screen_map_v13_002212c0, "OVScreenMap_v13"),
    (
        0x0022_1400,
        ov_screen_session_v13_00221400,
        "OVScreenSession_v13",
    ),
    (
        0x0022_1d80,
        ov_screen_in_game_error_v13_00221d80,
        "OVScreenInGameError_v13",
    ),
    (
        0x0022_22a8,
        ov_screen_autosave_on_widget_action_002222a8,
        "OVScreenAutosave_OnWidgetAction",
    ),
    (
        0x0022_22b0,
        ov_screen_autosave_v37_002222b0,
        "OVScreenAutosave_v37",
    ),
    (0x0022_24a0, luno_vm_return_zero, "LunoVM_ReturnZero"),
    (
        0x0023_2710,
        c_pre_game_load_screen_load_00232710,
        "cPreGameLoadScreen_Load",
    ),
    (
        0x0023_2718,
        c_pre_fe_load_screen_load_00232718,
        "cPreFELoadScreen_Load",
    ),
    (
        0x0023_3b88,
        game_state_quit_exit_00233b88,
        "GameStateQuit_Exit",
    ),
    (
        0x0023_6af0,
        game_state_event_switch_load_exit_00236af0,
        "GameStateEventSwitchLoad_Exit",
    ),
    (
        0x0023_7030,
        game_state_replay_playback_update_00237030,
        "GameStateReplayPlayback_Update",
    ),
    (0x0023_ac10, gm_handler1_v07_0023ac10, "GMHandler1_v07"),
    (
        0x0024_3ab0,
        pre_start_screen_load_00243ab0,
        "PreStartScreen_Load",
    ),
    (
        0x0024_3cb0,
        game_state_overlay_enter_00243cb0,
        "GameState_OverlayEnter",
    ),
    (
        0x0024_3cb8,
        game_state_overlay_update_00243cb8,
        "GameState_OverlayUpdate",
    ),
    (
        0x0024_3cc0,
        game_state_overlay_exit_00243cc0,
        "GameState_OverlayExit",
    ),
    (0x0024_3cc8, game_state_enter_00243cc8, "GameState_Enter"),
    (0x0024_3cd0, game_state_update_00243cd0, "GameState_Update"),
    (0x0024_3cd8, game_state_exit_00243cd8, "GameState_Exit"),
    (0x0024_3ce0, game_state_v08_00243ce0, "GameState_v08"),
    (0x0024_3ce8, game_state_v09_00243ce8, "GameState_v09"),
    (
        0x0024_43f8,
        generic_stream_pause_v03_002443f8,
        "GenericStreamPause_v03",
    ),
    (0x0024_4528, gm_handler_reset_00244528, "GMHandler_Reset"),
    (0x0024_4530, gm_handler_v06_00244530, "GMHandler_v06"),
    (0x0024_4538, gm_handler_v07_00244538, "GMHandler_v07"),
    (
        0x0024_4540,
        gm_handler_on_rider_finished_00244540,
        "GMHandler_OnRiderFinished",
    ),
    (0x0024_4548, gm_handler_v09_00244548, "GMHandler_v09"),
    (
        0x0024_4550,
        gm_handler_save_state_00244550,
        "GMHandler_SaveState",
    ),
    (
        0x0024_4558,
        gm_handler_load_state_00244558,
        "GMHandler_LoadState",
    ),
    (
        0x0024_4620,
        gm_handler9_save_state_00244620,
        "GMHandler9_SaveState",
    ),
    (
        0x0024_4628,
        gm_handler9_load_state_00244628,
        "GMHandler9_LoadState",
    ),
];

/// `UIInputBase_v42` (0x001e_1288): straight-line leaf. Mechanical port of the generated body.
pub fn ui_input_base_v42_001e1288(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `UIInputBase_v43` (0x001e_1290): straight-line leaf. Mechanical port of the generated body.
pub fn ui_input_base_v43_001e1290(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `UIInputBase_SetPort` (0x001e_1298): straight-line leaf: 1 store(s). Mechanical port of the generated body.
pub fn ui_input_base_set_port_001e1298(c: &mut Cpu) {
    c.cycles += 2;
    c.sb((c.g[A0][0] as u32).wrapping_add(0x4), c.g[A1][0]);
    ret(c);
}

/// `UIInputBase_v45` (0x001e_12a0): straight-line leaf. Mechanical port of the generated body.
pub fn ui_input_base_v45_001e12a0(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `UIInputBase_PollUsbKeyboard` (0x001e_12a8): straight-line leaf. Mechanical port of the generated body.
pub fn ui_input_base_poll_usb_keyboard_001e12a8(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `OVScreenFinalStandings_OnDeactivate` (0x001e_7920): straight-line leaf. Mechanical port of the generated body.
pub fn ov_screen_final_standings_on_deactivate_001e7920(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `OVScreenPeakChallengeResults_HandleInput` (0x001e_8e90): straight-line leaf. Mechanical port of the generated body.
pub fn ov_screen_peak_challenge_results_handle_input_001e8e90(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `OVScreenPauseFreeride_OnDeactivate` (0x001f_8720): straight-line leaf. Mechanical port of the generated body.
pub fn ov_screen_pause_freeride_on_deactivate_001f8720(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `OVScreenPauseFreeride_HandleInput` (0x001f_8df0): straight-line leaf. Mechanical port of the generated body.
pub fn ov_screen_pause_freeride_handle_input_001f8df0(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `OVScreenInGameError_TransitionIn` (0x001f_af08): straight-line leaf. Mechanical port of the generated body.
pub fn ov_screen_in_game_error_transition_in_001faf08(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0x1;
    ret(c);
}

/// `OVScreenSession_OnWidgetCreated` (0x0020_87e8): straight-line leaf. Mechanical port of the generated body.
pub fn ov_screen_session_on_widget_created_002087e8(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `OVScreenAutosave_HandleInput` (0x0021_22e0): straight-line leaf. Mechanical port of the generated body.
pub fn ov_screen_autosave_handle_input_002122e0(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `cOVStateOverlay_v25` (0x0021_c700): straight-line leaf: 1 load(s). Mechanical port of the generated body.
pub fn c_ov_state_overlay_v25_0021c700(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4c));
    ret(c);
}

/// `cOVState_ViewPopup_TransitionOut` (0x0021_cb68): straight-line leaf. Mechanical port of the generated body.
pub fn c_ov_state_view_popup_transition_out_0021cb68(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0x1;
    ret(c);
}

/// `OVScreenMap_v13` (0x0022_12c0): straight-line leaf. Mechanical port of the generated body.
pub fn ov_screen_map_v13_002212c0(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `OVScreenSession_v13` (0x0022_1400): straight-line leaf. Mechanical port of the generated body.
pub fn ov_screen_session_v13_00221400(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `OVScreenInGameError_v13` (0x0022_1d80): straight-line leaf. Mechanical port of the generated body.
pub fn ov_screen_in_game_error_v13_00221d80(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `OVScreenAutosave_OnWidgetAction` (0x0022_22a8): straight-line leaf. Mechanical port of the generated body.
pub fn ov_screen_autosave_on_widget_action_002222a8(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `OVScreenAutosave_v37` (0x0022_22b0): straight-line leaf. Mechanical port of the generated body.
pub fn ov_screen_autosave_v37_002222b0(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `LunoVM_ReturnZero` (0x0022_24a0): straight-line leaf. Mechanical port of the generated body.
pub fn luno_vm_return_zero(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `cPreGameLoadScreen_Load` (0x0023_2710): straight-line leaf. Mechanical port of the generated body.
pub fn c_pre_game_load_screen_load_00232710(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `cPreFELoadScreen_Load` (0x0023_2718): straight-line leaf. Mechanical port of the generated body.
pub fn c_pre_fe_load_screen_load_00232718(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `GameStateQuit_Exit` (0x0023_3b88): straight-line leaf. Mechanical port of the generated body.
pub fn game_state_quit_exit_00233b88(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `GameStateEventSwitchLoad_Exit` (0x0023_6af0): straight-line leaf. Mechanical port of the generated body.
pub fn game_state_event_switch_load_exit_00236af0(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `GameStateReplayPlayback_Update` (0x0023_7030): straight-line leaf. Mechanical port of the generated body.
pub fn game_state_replay_playback_update_00237030(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `GMHandler1_v07` (0x0023_ac10): straight-line leaf. Mechanical port of the generated body.
pub fn gm_handler1_v07_0023ac10(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `PreStartScreen_Load` (0x0024_3ab0): straight-line leaf. Mechanical port of the generated body.
pub fn pre_start_screen_load_00243ab0(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `GameState_OverlayEnter` (0x0024_3cb0): straight-line leaf. Mechanical port of the generated body.
pub fn game_state_overlay_enter_00243cb0(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `GameState_OverlayUpdate` (0x0024_3cb8): straight-line leaf. Mechanical port of the generated body.
pub fn game_state_overlay_update_00243cb8(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `GameState_OverlayExit` (0x0024_3cc0): straight-line leaf. Mechanical port of the generated body.
pub fn game_state_overlay_exit_00243cc0(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `GameState_Enter` (0x0024_3cc8): straight-line leaf. Mechanical port of the generated body.
pub fn game_state_enter_00243cc8(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `GameState_Update` (0x0024_3cd0): straight-line leaf. Mechanical port of the generated body.
pub fn game_state_update_00243cd0(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `GameState_Exit` (0x0024_3cd8): straight-line leaf. Mechanical port of the generated body.
pub fn game_state_exit_00243cd8(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `GameState_v08` (0x0024_3ce0): straight-line leaf. Mechanical port of the generated body.
pub fn game_state_v08_00243ce0(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0x1;
    ret(c);
}

/// `GameState_v09` (0x0024_3ce8): straight-line leaf. Mechanical port of the generated body.
pub fn game_state_v09_00243ce8(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `GenericStreamPause_v03` (0x0024_43f8): straight-line leaf. Mechanical port of the generated body.
pub fn generic_stream_pause_v03_002443f8(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0x1;
    ret(c);
}

/// `GMHandler_Reset` (0x0024_4528): straight-line leaf. Mechanical port of the generated body.
pub fn gm_handler_reset_00244528(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `GMHandler_v06` (0x0024_4530): straight-line leaf. Mechanical port of the generated body.
pub fn gm_handler_v06_00244530(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0x1;
    ret(c);
}

/// `GMHandler_v07` (0x0024_4538): straight-line leaf. Mechanical port of the generated body.
pub fn gm_handler_v07_00244538(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `GMHandler_OnRiderFinished` (0x0024_4540): straight-line leaf. Mechanical port of the generated body.
pub fn gm_handler_on_rider_finished_00244540(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `GMHandler_v09` (0x0024_4548): straight-line leaf. Mechanical port of the generated body.
pub fn gm_handler_v09_00244548(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `GMHandler_SaveState` (0x0024_4550): straight-line leaf. Mechanical port of the generated body.
pub fn gm_handler_save_state_00244550(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `GMHandler_LoadState` (0x0024_4558): straight-line leaf. Mechanical port of the generated body.
pub fn gm_handler_load_state_00244558(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `GMHandler9_SaveState` (0x0024_4620): straight-line leaf. Mechanical port of the generated body.
pub fn gm_handler9_save_state_00244620(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `GMHandler9_LoadState` (0x0024_4628): straight-line leaf. Mechanical port of the generated body.
pub fn gm_handler9_load_state_00244628(c: &mut Cpu) {
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

    /// `UIInputBase_v42` (0x001e_1288) against the generated body.
    #[test]
    fn ui_input_base_v42_001e1288_matches() {
        check(
            ee::s04::f_001e1288__generated,
            ui_input_base_v42_001e1288,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1e1288, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `UIInputBase_v43` (0x001e_1290) against the generated body.
    #[test]
    fn ui_input_base_v43_001e1290_matches() {
        check(
            ee::s04::f_001e1290__generated,
            ui_input_base_v43_001e1290,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1e1290, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `UIInputBase_SetPort` (0x001e_1298) against the generated body.
    #[test]
    fn ui_input_base_set_port_001e1298_matches() {
        check(
            ee::s04::f_001e1298__generated,
            ui_input_base_set_port_001e1298,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1e1298, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[A1][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `UIInputBase_v45` (0x001e_12a0) against the generated body.
    #[test]
    fn ui_input_base_v45_001e12a0_matches() {
        check(
            ee::s04::f_001e12a0__generated,
            ui_input_base_v45_001e12a0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1e12a0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `UIInputBase_PollUsbKeyboard` (0x001e_12a8) against the generated body.
    #[test]
    fn ui_input_base_poll_usb_keyboard_001e12a8_matches() {
        check(
            ee::s04::f_001e12a8__generated,
            ui_input_base_poll_usb_keyboard_001e12a8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1e12a8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `OVScreenFinalStandings_OnDeactivate` (0x001e_7920) against the generated body.
    #[test]
    fn ov_screen_final_standings_on_deactivate_001e7920_matches() {
        check(
            ee::s04::f_001e7920__generated,
            ov_screen_final_standings_on_deactivate_001e7920,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1e7920, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `OVScreenPeakChallengeResults_HandleInput` (0x001e_8e90) against the generated body.
    #[test]
    fn ov_screen_peak_challenge_results_handle_input_001e8e90_matches() {
        check(
            ee::s04::f_001e8e90__generated,
            ov_screen_peak_challenge_results_handle_input_001e8e90,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1e8e90, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `OVScreenPauseFreeride_OnDeactivate` (0x001f_8720) against the generated body.
    #[test]
    fn ov_screen_pause_freeride_on_deactivate_001f8720_matches() {
        check(
            ee::s04::f_001f8720__generated,
            ov_screen_pause_freeride_on_deactivate_001f8720,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1f8720, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `OVScreenPauseFreeride_HandleInput` (0x001f_8df0) against the generated body.
    #[test]
    fn ov_screen_pause_freeride_handle_input_001f8df0_matches() {
        check(
            ee::s04::f_001f8df0__generated,
            ov_screen_pause_freeride_handle_input_001f8df0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1f8df0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `OVScreenInGameError_TransitionIn` (0x001f_af08) against the generated body.
    #[test]
    fn ov_screen_in_game_error_transition_in_001faf08_matches() {
        check(
            ee::s04::f_001faf08__generated,
            ov_screen_in_game_error_transition_in_001faf08,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1faf08, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `OVScreenSession_OnWidgetCreated` (0x0020_87e8) against the generated body.
    #[test]
    fn ov_screen_session_on_widget_created_002087e8_matches() {
        check(
            ee::s05::f_002087e8__generated,
            ov_screen_session_on_widget_created_002087e8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2087e8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `OVScreenAutosave_HandleInput` (0x0021_22e0) against the generated body.
    #[test]
    fn ov_screen_autosave_handle_input_002122e0_matches() {
        check(
            ee::s05::f_002122e0__generated,
            ov_screen_autosave_handle_input_002122e0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2122e0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `cOVStateOverlay_v25` (0x0021_c700) against the generated body.
    #[test]
    fn c_ov_state_overlay_v25_0021c700_matches() {
        check(
            ee::s05::f_0021c700__generated,
            c_ov_state_overlay_v25_0021c700,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x21c700, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `cOVState_ViewPopup_TransitionOut` (0x0021_cb68) against the generated body.
    #[test]
    fn c_ov_state_view_popup_transition_out_0021cb68_matches() {
        check(
            ee::s05::f_0021cb68__generated,
            c_ov_state_view_popup_transition_out_0021cb68,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x21cb68, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `OVScreenMap_v13` (0x0022_12c0) against the generated body.
    #[test]
    fn ov_screen_map_v13_002212c0_matches() {
        check(
            ee::s05::f_002212c0__generated,
            ov_screen_map_v13_002212c0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2212c0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `OVScreenSession_v13` (0x0022_1400) against the generated body.
    #[test]
    fn ov_screen_session_v13_00221400_matches() {
        check(
            ee::s05::f_00221400__generated,
            ov_screen_session_v13_00221400,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x221400, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `OVScreenInGameError_v13` (0x0022_1d80) against the generated body.
    #[test]
    fn ov_screen_in_game_error_v13_00221d80_matches() {
        check(
            ee::s05::f_00221d80__generated,
            ov_screen_in_game_error_v13_00221d80,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x221d80, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `OVScreenAutosave_OnWidgetAction` (0x0022_22a8) against the generated body.
    #[test]
    fn ov_screen_autosave_on_widget_action_002222a8_matches() {
        check(
            ee::s05::f_002222a8__generated,
            ov_screen_autosave_on_widget_action_002222a8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2222a8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `OVScreenAutosave_v37` (0x0022_22b0) against the generated body.
    #[test]
    fn ov_screen_autosave_v37_002222b0_matches() {
        check(
            ee::s05::f_002222b0__generated,
            ov_screen_autosave_v37_002222b0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2222b0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `LunoVM_ReturnZero` (0x0022_24a0) against the generated body.
    #[test]
    fn luno_vm_return_zero_matches() {
        check(
            ee::s05::f_002224a0__generated,
            luno_vm_return_zero,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2224a0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `cPreGameLoadScreen_Load` (0x0023_2710) against the generated body.
    #[test]
    fn c_pre_game_load_screen_load_00232710_matches() {
        check(
            ee::s06::f_00232710__generated,
            c_pre_game_load_screen_load_00232710,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x232710, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `cPreFELoadScreen_Load` (0x0023_2718) against the generated body.
    #[test]
    fn c_pre_fe_load_screen_load_00232718_matches() {
        check(
            ee::s06::f_00232718__generated,
            c_pre_fe_load_screen_load_00232718,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x232718, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `GameStateQuit_Exit` (0x0023_3b88) against the generated body.
    #[test]
    fn game_state_quit_exit_00233b88_matches() {
        check(
            ee::s06::f_00233b88__generated,
            game_state_quit_exit_00233b88,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x233b88, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `GameStateEventSwitchLoad_Exit` (0x0023_6af0) against the generated body.
    #[test]
    fn game_state_event_switch_load_exit_00236af0_matches() {
        check(
            ee::s06::f_00236af0__generated,
            game_state_event_switch_load_exit_00236af0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x236af0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `GameStateReplayPlayback_Update` (0x0023_7030) against the generated body.
    #[test]
    fn game_state_replay_playback_update_00237030_matches() {
        check(
            ee::s06::f_00237030__generated,
            game_state_replay_playback_update_00237030,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x237030, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `GMHandler1_v07` (0x0023_ac10) against the generated body.
    #[test]
    fn gm_handler1_v07_0023ac10_matches() {
        check(
            ee::s06::f_0023ac10__generated,
            gm_handler1_v07_0023ac10,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x23ac10, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `PreStartScreen_Load` (0x0024_3ab0) against the generated body.
    #[test]
    fn pre_start_screen_load_00243ab0_matches() {
        check(
            ee::s06::f_00243ab0__generated,
            pre_start_screen_load_00243ab0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x243ab0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `GameState_OverlayEnter` (0x0024_3cb0) against the generated body.
    #[test]
    fn game_state_overlay_enter_00243cb0_matches() {
        check(
            ee::s06::f_00243cb0__generated,
            game_state_overlay_enter_00243cb0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x243cb0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `GameState_OverlayUpdate` (0x0024_3cb8) against the generated body.
    #[test]
    fn game_state_overlay_update_00243cb8_matches() {
        check(
            ee::s06::f_00243cb8__generated,
            game_state_overlay_update_00243cb8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x243cb8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `GameState_OverlayExit` (0x0024_3cc0) against the generated body.
    #[test]
    fn game_state_overlay_exit_00243cc0_matches() {
        check(
            ee::s06::f_00243cc0__generated,
            game_state_overlay_exit_00243cc0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x243cc0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `GameState_Enter` (0x0024_3cc8) against the generated body.
    #[test]
    fn game_state_enter_00243cc8_matches() {
        check(
            ee::s06::f_00243cc8__generated,
            game_state_enter_00243cc8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x243cc8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `GameState_Update` (0x0024_3cd0) against the generated body.
    #[test]
    fn game_state_update_00243cd0_matches() {
        check(
            ee::s06::f_00243cd0__generated,
            game_state_update_00243cd0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x243cd0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `GameState_Exit` (0x0024_3cd8) against the generated body.
    #[test]
    fn game_state_exit_00243cd8_matches() {
        check(
            ee::s06::f_00243cd8__generated,
            game_state_exit_00243cd8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x243cd8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `GameState_v08` (0x0024_3ce0) against the generated body.
    #[test]
    fn game_state_v08_00243ce0_matches() {
        check(
            ee::s06::f_00243ce0__generated,
            game_state_v08_00243ce0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x243ce0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `GameState_v09` (0x0024_3ce8) against the generated body.
    #[test]
    fn game_state_v09_00243ce8_matches() {
        check(
            ee::s06::f_00243ce8__generated,
            game_state_v09_00243ce8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x243ce8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `GenericStreamPause_v03` (0x0024_43f8) against the generated body.
    #[test]
    fn generic_stream_pause_v03_002443f8_matches() {
        check(
            ee::s06::f_002443f8__generated,
            generic_stream_pause_v03_002443f8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2443f8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `GMHandler_Reset` (0x0024_4528) against the generated body.
    #[test]
    fn gm_handler_reset_00244528_matches() {
        check(
            ee::s06::f_00244528__generated,
            gm_handler_reset_00244528,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x244528, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `GMHandler_v06` (0x0024_4530) against the generated body.
    #[test]
    fn gm_handler_v06_00244530_matches() {
        check(
            ee::s06::f_00244530__generated,
            gm_handler_v06_00244530,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x244530, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `GMHandler_v07` (0x0024_4538) against the generated body.
    #[test]
    fn gm_handler_v07_00244538_matches() {
        check(
            ee::s06::f_00244538__generated,
            gm_handler_v07_00244538,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x244538, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `GMHandler_OnRiderFinished` (0x0024_4540) against the generated body.
    #[test]
    fn gm_handler_on_rider_finished_00244540_matches() {
        check(
            ee::s06::f_00244540__generated,
            gm_handler_on_rider_finished_00244540,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x244540, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `GMHandler_v09` (0x0024_4548) against the generated body.
    #[test]
    fn gm_handler_v09_00244548_matches() {
        check(
            ee::s06::f_00244548__generated,
            gm_handler_v09_00244548,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x244548, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `GMHandler_SaveState` (0x0024_4550) against the generated body.
    #[test]
    fn gm_handler_save_state_00244550_matches() {
        check(
            ee::s06::f_00244550__generated,
            gm_handler_save_state_00244550,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x244550, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `GMHandler_LoadState` (0x0024_4558) against the generated body.
    #[test]
    fn gm_handler_load_state_00244558_matches() {
        check(
            ee::s06::f_00244558__generated,
            gm_handler_load_state_00244558,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x244558, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `GMHandler9_SaveState` (0x0024_4620) against the generated body.
    #[test]
    fn gm_handler9_save_state_00244620_matches() {
        check(
            ee::s06::f_00244620__generated,
            gm_handler9_save_state_00244620,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x244620, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `GMHandler9_LoadState` (0x0024_4628) against the generated body.
    #[test]
    fn gm_handler9_load_state_00244628_matches() {
        check(
            ee::s06::f_00244628__generated,
            gm_handler9_load_state_00244628,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x244628, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }
}
