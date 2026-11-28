//! Staging module oc1503 (wave wave276): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1503`.

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
        0x0016_07d8,
        matrix_chase_camera_algorithm_get_tuning_001607d8,
        "MatrixChaseCameraAlgorithm_GetTuning",
    ),
    (
        0x0016_0be0,
        relative_chase_camera_algorithm_get_eye_drop_00160be0,
        "RelativeChaseCameraAlgorithm_GetEyeDrop",
    ),
    (
        0x0016_0bf0,
        relative_chase_camera_algorithm_get_tuning_00160bf0,
        "RelativeChaseCameraAlgorithm_GetTuning",
    ),
    (
        0x0016_13d8,
        spline_camera_algorithm_get_eye_drop_001613d8,
        "SplineCameraAlgorithm_GetEyeDrop",
    ),
    (
        0x0016_13e8,
        spline_camera_algorithm_get_tuning_001613e8,
        "SplineCameraAlgorithm_GetTuning",
    ),
    (
        0x0016_1710,
        idle_chase_camera_algorithm_get_eye_drop_00161710,
        "IdleChaseCameraAlgorithm_GetEyeDrop",
    ),
    (
        0x0016_1720,
        idle_chase_camera_algorithm_get_tuning_00161720,
        "IdleChaseCameraAlgorithm_GetTuning",
    ),
    (0x0016_73b8, debug_printf_stub_73b8, "DebugPrintf_Stub_73b8"),
    (
        0x0016_7e18,
        camera_algorithm_reset_for_rider_00167e18,
        "CameraAlgorithm_ResetForRider",
    ),
    (
        0x0016_7e20,
        camera_algorithm_reset_00167e20,
        "CameraAlgorithm_Reset",
    ),
    (
        0x0016_7e28,
        camera_algorithm_update_00167e28,
        "CameraAlgorithm_Update",
    ),
    (
        0x0016_7e58,
        camera_algorithm_get_tuning_00167e58,
        "CameraAlgorithm_GetTuning",
    ),
    (
        0x0016_7e60,
        camera_algorithm_post_update_00167e60,
        "CameraAlgorithm_PostUpdate",
    ),
    (
        0x0016_7e90,
        camera_algorithm_v10_empty_00167e90,
        "CameraAlgorithm_v10_Empty",
    ),
    (
        0x0016_7e98,
        camera_algorithm_get_reset_flag_00167e98,
        "CameraAlgorithm_GetResetFlag",
    ),
    (
        0x0016_7fb0,
        chase_camera_algorithm_get_reset_flag_00167fb0,
        "ChaseCameraAlgorithm_GetResetFlag",
    ),
    (
        0x0016_7fb8,
        cam_controller_update_00167fb8,
        "CamController_Update",
    ),
    (
        0x0016_8320,
        lazy_chase_camera_algorithm_get_eye_drop_00168320,
        "LazyChaseCameraAlgorithm_GetEyeDrop",
    ),
    (
        0x0016_8330,
        lazy_chase_camera_algorithm_get_tuning_00168330,
        "LazyChaseCameraAlgorithm_GetTuning",
    ),
    (
        0x0016_86b8,
        look_back_chase_camera_algorithm_get_eye_drop_001686b8,
        "LookBackChaseCameraAlgorithm_GetEyeDrop",
    ),
    (
        0x0016_86c8,
        look_back_chase_camera_algorithm_get_tuning_001686c8,
        "LookBackChaseCameraAlgorithm_GetTuning",
    ),
    (0x0017_4808, debug_printf_stub_4808, "DebugPrintf_Stub_4808"),
    (
        0x0017_65f8,
        camera_action_v01_empty_001765f8,
        "CameraAction_v01_Empty",
    ),
    (
        0x0017_7548,
        direction_ne_chase_camera_algorithm_get_eye_drop_00177548,
        "DirectionNeChaseCameraAlgorithm_GetEyeDrop",
    ),
    (0x0017_7610, debug_printf_stub_7610, "DebugPrintf_Stub_7610"),
    (
        0x0017_8b98,
        post_race_1_chase_camera_algorithm_get_eye_drop_00178b98,
        "Post_Race_1ChaseCameraAlgorithm_GetEyeDrop",
    ),
    (
        0x0017_8ba8,
        post_race_1_chase_camera_algorithm_get_tuning_00178ba8,
        "Post_Race_1ChaseCameraAlgorithm_GetTuning",
    ),
    (0x0017_8f18, debug_printf_stub_8f18, "DebugPrintf_Stub_8f18"),
    (0x0017_9758, debug_printf_stub_9758, "DebugPrintf_Stub_9758"),
    (
        0x0018_6b48,
        c_fe_state_mem_card_on_widget_notify_00186b48,
        "cFEStateMemCard_OnWidgetNotify",
    ),
    (
        0x0018_7c10,
        c_fe_state_mem_card_handle_input_00187c10,
        "cFEStateMemCard_HandleInput",
    ),
    (
        0x0019_3310,
        c_fe_state_bragging_rights_transition_in_00193310,
        "cFEStateBraggingRights_TransitionIn",
    ),
    (
        0x0019_4980,
        c_fe_state_title_on_child_event_00194980,
        "cFEStateTitle_OnChildEvent",
    ),
    (
        0x001a_3300,
        c_fe_state_mp_circuit_on_widget_created_001a3300,
        "cFEStateMPCircuit_OnWidgetCreated",
    ),
    (
        0x001a_7bb0,
        c_fe_state_read_message_popup_on_widget_created_001a7bb0,
        "cFEStateReadMessagePopup_OnWidgetCreated",
    ),
    (
        0x001b_6078,
        c_fe_state_net_config_v21_001b6078,
        "cFEStateNetConfig_v21",
    ),
    (
        0x001b_ea30,
        c_fe_state_net_create_room_popup_v21_001bea30,
        "cFEStateNetCreateRoomPopup_v21",
    ),
    (
        0x001c_17d8,
        c_fe_state_net_quick_play_popup_v21_001c17d8,
        "cFEStateNetQuickPlayPopup_v21",
    ),
    (
        0x001c_5dd0,
        c_fe_popup_confirm_transition_out_001c5dd0,
        "cFEPopupConfirm_TransitionOut",
    ),
    (
        0x001d_1198,
        c_fe_state_reward_gallery_posters_get_icon_path_001d1198,
        "cFEStateRewardGalleryPosters_GetIconPath",
    ),
    (
        0x001d_1330,
        c_fe_state_reward_gallery_trading_cards_get_icon_path_001d1330,
        "cFEStateRewardGalleryTradingCards_GetIconPath",
    ),
    (
        0x001d_14c8,
        c_fe_state_reward_gallery_concept_arts_get_icon_path_001d14c8,
        "cFEStateRewardGalleryConceptArts_GetIconPath",
    ),
    (
        0x001d_1660,
        c_fe_state_reward_gallery_videos_get_icon_path_001d1660,
        "cFEStateRewardGalleryVideos_GetIconPath",
    ),
    (
        0x001d_17f8,
        c_fe_state_reward_gallery_toys_get_icon_path_001d17f8,
        "cFEStateRewardGalleryToys_GetIconPath",
    ),
    (
        0x001d_9f30,
        c_fe_state_net_help_popup_handle_input_001d9f30,
        "cFEStateNetHelpPopup_HandleInput",
    ),
];

/// `MatrixChaseCameraAlgorithm_GetTuning` (0x0016_07d8): straight-line leaf: 1 store(s). Mechanical port of the generated body.
pub fn matrix_chase_camera_algorithm_get_tuning_001607d8(c: &mut Cpu) {
    c.cycles += 2;
    c.sw((c.g[A1][0] as u32).wrapping_add(0x10), 0u64);
    ret(c);
}

/// `RelativeChaseCameraAlgorithm_GetEyeDrop` (0x0016_0be0): straight-line leaf. Mechanical port of the generated body.
pub fn relative_chase_camera_algorithm_get_eye_drop_00160be0(c: &mut Cpu) {
    c.cycles += 3;
    c.mtc1(0, 0);
    ret(c);
}

/// `RelativeChaseCameraAlgorithm_GetTuning` (0x0016_0bf0): straight-line leaf: 1 store(s). Mechanical port of the generated body.
pub fn relative_chase_camera_algorithm_get_tuning_00160bf0(c: &mut Cpu) {
    c.cycles += 2;
    c.sw((c.g[A1][0] as u32).wrapping_add(0x10), 0u64);
    ret(c);
}

/// `SplineCameraAlgorithm_GetEyeDrop` (0x0016_13d8): straight-line leaf. Mechanical port of the generated body.
pub fn spline_camera_algorithm_get_eye_drop_001613d8(c: &mut Cpu) {
    c.cycles += 3;
    c.mtc1(0, 0);
    ret(c);
}

/// `SplineCameraAlgorithm_GetTuning` (0x0016_13e8): straight-line leaf: 1 store(s). Mechanical port of the generated body.
pub fn spline_camera_algorithm_get_tuning_001613e8(c: &mut Cpu) {
    c.cycles += 2;
    c.sw((c.g[A1][0] as u32).wrapping_add(0x10), 0u64);
    ret(c);
}

/// `IdleChaseCameraAlgorithm_GetEyeDrop` (0x0016_1710): straight-line leaf. Mechanical port of the generated body.
pub fn idle_chase_camera_algorithm_get_eye_drop_00161710(c: &mut Cpu) {
    c.cycles += 3;
    c.mtc1(0, 0);
    ret(c);
}

/// `IdleChaseCameraAlgorithm_GetTuning` (0x0016_1720): straight-line leaf: 1 store(s). Mechanical port of the generated body.
pub fn idle_chase_camera_algorithm_get_tuning_00161720(c: &mut Cpu) {
    c.cycles += 2;
    c.sw((c.g[A1][0] as u32).wrapping_add(0x10), 0u64);
    ret(c);
}

/// `DebugPrintf_Stub_73b8` (0x0016_73b8): straight-line leaf: 11 store(s). Mechanical port of the generated body.
pub fn debug_printf_stub_73b8(c: &mut Cpu) {
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

/// `CameraAlgorithm_ResetForRider` (0x0016_7e18): straight-line leaf. Mechanical port of the generated body.
pub fn camera_algorithm_reset_for_rider_00167e18(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `CameraAlgorithm_Reset` (0x0016_7e20): straight-line leaf. Mechanical port of the generated body.
pub fn camera_algorithm_reset_00167e20(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `CameraAlgorithm_Update` (0x0016_7e28): straight-line leaf. Mechanical port of the generated body.
pub fn camera_algorithm_update_00167e28(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `CameraAlgorithm_GetTuning` (0x0016_7e58): straight-line leaf. Mechanical port of the generated body.
pub fn camera_algorithm_get_tuning_00167e58(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `CameraAlgorithm_PostUpdate` (0x0016_7e60): straight-line leaf. Mechanical port of the generated body.
pub fn camera_algorithm_post_update_00167e60(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `CameraAlgorithm_v10_Empty` (0x0016_7e90): straight-line leaf. Mechanical port of the generated body.
pub fn camera_algorithm_v10_empty_00167e90(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `CameraAlgorithm_GetResetFlag` (0x0016_7e98): straight-line leaf. Mechanical port of the generated body.
pub fn camera_algorithm_get_reset_flag_00167e98(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `ChaseCameraAlgorithm_GetResetFlag` (0x0016_7fb0): straight-line leaf: 1 load(s). Mechanical port of the generated body.
pub fn chase_camera_algorithm_get_reset_flag_00167fb0(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x2f0));
    ret(c);
}

/// `CamController_Update` (0x0016_7fb8): straight-line leaf. Mechanical port of the generated body.
pub fn cam_controller_update_00167fb8(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `LazyChaseCameraAlgorithm_GetEyeDrop` (0x0016_8320): straight-line leaf. Mechanical port of the generated body.
pub fn lazy_chase_camera_algorithm_get_eye_drop_00168320(c: &mut Cpu) {
    c.cycles += 3;
    c.mtc1(0, 0);
    ret(c);
}

/// `LazyChaseCameraAlgorithm_GetTuning` (0x0016_8330): straight-line leaf: 1 store(s). Mechanical port of the generated body.
pub fn lazy_chase_camera_algorithm_get_tuning_00168330(c: &mut Cpu) {
    c.cycles += 2;
    c.sw((c.g[A1][0] as u32).wrapping_add(0x10), 0u64);
    ret(c);
}

/// `LookBackChaseCameraAlgorithm_GetEyeDrop` (0x0016_86b8): straight-line leaf. Mechanical port of the generated body.
pub fn look_back_chase_camera_algorithm_get_eye_drop_001686b8(c: &mut Cpu) {
    c.cycles += 3;
    c.mtc1(0, 0);
    ret(c);
}

/// `LookBackChaseCameraAlgorithm_GetTuning` (0x0016_86c8): straight-line leaf: 1 store(s). Mechanical port of the generated body.
pub fn look_back_chase_camera_algorithm_get_tuning_001686c8(c: &mut Cpu) {
    c.cycles += 2;
    c.sw((c.g[A1][0] as u32).wrapping_add(0x10), 0u64);
    ret(c);
}

/// `DebugPrintf_Stub_4808` (0x0017_4808): straight-line leaf: 11 store(s). Mechanical port of the generated body.
pub fn debug_printf_stub_4808(c: &mut Cpu) {
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

/// `CameraAction_v01_Empty` (0x0017_65f8): straight-line leaf. Mechanical port of the generated body.
pub fn camera_action_v01_empty_001765f8(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `DirectionNeChaseCameraAlgorithm_GetEyeDrop` (0x0017_7548): straight-line leaf. Mechanical port of the generated body.
pub fn direction_ne_chase_camera_algorithm_get_eye_drop_00177548(c: &mut Cpu) {
    c.cycles += 3;
    c.mtc1(0, 0);
    ret(c);
}

/// `DebugPrintf_Stub_7610` (0x0017_7610): straight-line leaf: 11 store(s). Mechanical port of the generated body.
pub fn debug_printf_stub_7610(c: &mut Cpu) {
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

/// `Post_Race_1ChaseCameraAlgorithm_GetEyeDrop` (0x0017_8b98): straight-line leaf. Mechanical port of the generated body.
pub fn post_race_1_chase_camera_algorithm_get_eye_drop_00178b98(c: &mut Cpu) {
    c.cycles += 3;
    c.mtc1(0, 0);
    ret(c);
}

/// `Post_Race_1ChaseCameraAlgorithm_GetTuning` (0x0017_8ba8): straight-line leaf: 1 store(s). Mechanical port of the generated body.
pub fn post_race_1_chase_camera_algorithm_get_tuning_00178ba8(c: &mut Cpu) {
    c.cycles += 2;
    c.sw((c.g[A1][0] as u32).wrapping_add(0x10), 0u64);
    ret(c);
}

/// `DebugPrintf_Stub_8f18` (0x0017_8f18): straight-line leaf: 11 store(s). Mechanical port of the generated body.
pub fn debug_printf_stub_8f18(c: &mut Cpu) {
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

/// `DebugPrintf_Stub_9758` (0x0017_9758): straight-line leaf: 11 store(s). Mechanical port of the generated body.
pub fn debug_printf_stub_9758(c: &mut Cpu) {
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

/// `cFEStateMemCard_OnWidgetNotify` (0x0018_6b48): straight-line leaf. Mechanical port of the generated body.
pub fn c_fe_state_mem_card_on_widget_notify_00186b48(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0x100;
    ret(c);
}

/// `cFEStateMemCard_HandleInput` (0x0018_7c10): straight-line leaf. Mechanical port of the generated body.
pub fn c_fe_state_mem_card_handle_input_00187c10(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `cFEStateBraggingRights_TransitionIn` (0x0019_3310): straight-line leaf. Mechanical port of the generated body.
pub fn c_fe_state_bragging_rights_transition_in_00193310(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `cFEStateTitle_OnChildEvent` (0x0019_4980): straight-line leaf. Mechanical port of the generated body.
pub fn c_fe_state_title_on_child_event_00194980(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `cFEStateMPCircuit_OnWidgetCreated` (0x001a_3300): straight-line leaf. Mechanical port of the generated body.
pub fn c_fe_state_mp_circuit_on_widget_created_001a3300(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `cFEStateReadMessagePopup_OnWidgetCreated` (0x001a_7bb0): straight-line leaf. Mechanical port of the generated body.
pub fn c_fe_state_read_message_popup_on_widget_created_001a7bb0(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `cFEStateNetConfig_v21` (0x001b_6078): straight-line leaf. Mechanical port of the generated body.
pub fn c_fe_state_net_config_v21_001b6078(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0x101;
    ret(c);
}

/// `cFEStateNetCreateRoomPopup_v21` (0x001b_ea30): straight-line leaf. Mechanical port of the generated body.
pub fn c_fe_state_net_create_room_popup_v21_001bea30(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0x101;
    ret(c);
}

/// `cFEStateNetQuickPlayPopup_v21` (0x001c_17d8): straight-line leaf. Mechanical port of the generated body.
pub fn c_fe_state_net_quick_play_popup_v21_001c17d8(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0x101;
    ret(c);
}

/// `cFEPopupConfirm_TransitionOut` (0x001c_5dd0): straight-line leaf. Mechanical port of the generated body.
pub fn c_fe_popup_confirm_transition_out_001c5dd0(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0x1;
    ret(c);
}

/// `cFEStateRewardGalleryPosters_GetIconPath` (0x001d_1198): straight-line leaf. Mechanical port of the generated body.
pub fn c_fe_state_reward_gallery_posters_get_icon_path_001d1198(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = 0x460000;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x7070));
    ret(c);
}

/// `cFEStateRewardGalleryTradingCards_GetIconPath` (0x001d_1330): straight-line leaf. Mechanical port of the generated body.
pub fn c_fe_state_reward_gallery_trading_cards_get_icon_path_001d1330(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = 0x460000;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x7098));
    ret(c);
}

/// `cFEStateRewardGalleryConceptArts_GetIconPath` (0x001d_14c8): straight-line leaf. Mechanical port of the generated body.
pub fn c_fe_state_reward_gallery_concept_arts_get_icon_path_001d14c8(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = 0x460000;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x70c8));
    ret(c);
}

/// `cFEStateRewardGalleryVideos_GetIconPath` (0x001d_1660): straight-line leaf. Mechanical port of the generated body.
pub fn c_fe_state_reward_gallery_videos_get_icon_path_001d1660(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = 0x460000;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x70f0));
    ret(c);
}

/// `cFEStateRewardGalleryToys_GetIconPath` (0x001d_17f8): straight-line leaf. Mechanical port of the generated body.
pub fn c_fe_state_reward_gallery_toys_get_icon_path_001d17f8(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = 0x460000;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x7118));
    ret(c);
}

/// `cFEStateNetHelpPopup_HandleInput` (0x001d_9f30): straight-line leaf. Mechanical port of the generated body.
pub fn c_fe_state_net_help_popup_handle_input_001d9f30(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0x1;
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

    /// `MatrixChaseCameraAlgorithm_GetTuning` (0x0016_07d8) against the generated body.
    #[test]
    fn matrix_chase_camera_algorithm_get_tuning_001607d8_matches() {
        check(
            ee::s01::f_001607d8__generated,
            matrix_chase_camera_algorithm_get_tuning_001607d8,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1607d8, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A1][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `RelativeChaseCameraAlgorithm_GetEyeDrop` (0x0016_0be0) against the generated body.
    #[test]
    fn relative_chase_camera_algorithm_get_eye_drop_00160be0_matches() {
        check(
            ee::s01::f_00160be0__generated,
            relative_chase_camera_algorithm_get_eye_drop_00160be0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x160be0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `RelativeChaseCameraAlgorithm_GetTuning` (0x0016_0bf0) against the generated body.
    #[test]
    fn relative_chase_camera_algorithm_get_tuning_00160bf0_matches() {
        check(
            ee::s01::f_00160bf0__generated,
            relative_chase_camera_algorithm_get_tuning_00160bf0,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x160bf0, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A1][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `SplineCameraAlgorithm_GetEyeDrop` (0x0016_13d8) against the generated body.
    #[test]
    fn spline_camera_algorithm_get_eye_drop_001613d8_matches() {
        check(
            ee::s01::f_001613d8__generated,
            spline_camera_algorithm_get_eye_drop_001613d8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1613d8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `SplineCameraAlgorithm_GetTuning` (0x0016_13e8) against the generated body.
    #[test]
    fn spline_camera_algorithm_get_tuning_001613e8_matches() {
        check(
            ee::s01::f_001613e8__generated,
            spline_camera_algorithm_get_tuning_001613e8,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1613e8, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A1][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `IdleChaseCameraAlgorithm_GetEyeDrop` (0x0016_1710) against the generated body.
    #[test]
    fn idle_chase_camera_algorithm_get_eye_drop_00161710_matches() {
        check(
            ee::s01::f_00161710__generated,
            idle_chase_camera_algorithm_get_eye_drop_00161710,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x161710, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `IdleChaseCameraAlgorithm_GetTuning` (0x0016_1720) against the generated body.
    #[test]
    fn idle_chase_camera_algorithm_get_tuning_00161720_matches() {
        check(
            ee::s01::f_00161720__generated,
            idle_chase_camera_algorithm_get_tuning_00161720,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x161720, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A1][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `DebugPrintf_Stub_73b8` (0x0016_73b8) against the generated body.
    #[test]
    fn debug_printf_stub_73b8_matches() {
        check(
            ee::s02::f_001673b8__generated,
            debug_printf_stub_73b8,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1673b8, 0);
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

    /// `CameraAlgorithm_ResetForRider` (0x0016_7e18) against the generated body.
    #[test]
    fn camera_algorithm_reset_for_rider_00167e18_matches() {
        check(
            ee::s02::f_00167e18__generated,
            camera_algorithm_reset_for_rider_00167e18,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x167e18, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `CameraAlgorithm_Reset` (0x0016_7e20) against the generated body.
    #[test]
    fn camera_algorithm_reset_00167e20_matches() {
        check(
            ee::s02::f_00167e20__generated,
            camera_algorithm_reset_00167e20,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x167e20, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `CameraAlgorithm_Update` (0x0016_7e28) against the generated body.
    #[test]
    fn camera_algorithm_update_00167e28_matches() {
        check(
            ee::s02::f_00167e28__generated,
            camera_algorithm_update_00167e28,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x167e28, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `CameraAlgorithm_GetTuning` (0x0016_7e58) against the generated body.
    #[test]
    fn camera_algorithm_get_tuning_00167e58_matches() {
        check(
            ee::s02::f_00167e58__generated,
            camera_algorithm_get_tuning_00167e58,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x167e58, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `CameraAlgorithm_PostUpdate` (0x0016_7e60) against the generated body.
    #[test]
    fn camera_algorithm_post_update_00167e60_matches() {
        check(
            ee::s02::f_00167e60__generated,
            camera_algorithm_post_update_00167e60,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x167e60, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `CameraAlgorithm_v10_Empty` (0x0016_7e90) against the generated body.
    #[test]
    fn camera_algorithm_v10_empty_00167e90_matches() {
        check(
            ee::s02::f_00167e90__generated,
            camera_algorithm_v10_empty_00167e90,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x167e90, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `CameraAlgorithm_GetResetFlag` (0x0016_7e98) against the generated body.
    #[test]
    fn camera_algorithm_get_reset_flag_00167e98_matches() {
        check(
            ee::s02::f_00167e98__generated,
            camera_algorithm_get_reset_flag_00167e98,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x167e98, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `ChaseCameraAlgorithm_GetResetFlag` (0x0016_7fb0) against the generated body.
    #[test]
    fn chase_camera_algorithm_get_reset_flag_00167fb0_matches() {
        check(
            ee::s02::f_00167fb0__generated,
            chase_camera_algorithm_get_reset_flag_00167fb0,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x167fb0, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `CamController_Update` (0x0016_7fb8) against the generated body.
    #[test]
    fn cam_controller_update_00167fb8_matches() {
        check(
            ee::s02::f_00167fb8__generated,
            cam_controller_update_00167fb8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x167fb8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `LazyChaseCameraAlgorithm_GetEyeDrop` (0x0016_8320) against the generated body.
    #[test]
    fn lazy_chase_camera_algorithm_get_eye_drop_00168320_matches() {
        check(
            ee::s02::f_00168320__generated,
            lazy_chase_camera_algorithm_get_eye_drop_00168320,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x168320, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `LazyChaseCameraAlgorithm_GetTuning` (0x0016_8330) against the generated body.
    #[test]
    fn lazy_chase_camera_algorithm_get_tuning_00168330_matches() {
        check(
            ee::s02::f_00168330__generated,
            lazy_chase_camera_algorithm_get_tuning_00168330,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x168330, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A1][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `LookBackChaseCameraAlgorithm_GetEyeDrop` (0x0016_86b8) against the generated body.
    #[test]
    fn look_back_chase_camera_algorithm_get_eye_drop_001686b8_matches() {
        check(
            ee::s02::f_001686b8__generated,
            look_back_chase_camera_algorithm_get_eye_drop_001686b8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1686b8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `LookBackChaseCameraAlgorithm_GetTuning` (0x0016_86c8) against the generated body.
    #[test]
    fn look_back_chase_camera_algorithm_get_tuning_001686c8_matches() {
        check(
            ee::s02::f_001686c8__generated,
            look_back_chase_camera_algorithm_get_tuning_001686c8,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1686c8, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A1][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `DebugPrintf_Stub_4808` (0x0017_4808) against the generated body.
    #[test]
    fn debug_printf_stub_4808_matches() {
        check(
            ee::s02::f_00174808__generated,
            debug_printf_stub_4808,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x174808, 0);
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

    /// `CameraAction_v01_Empty` (0x0017_65f8) against the generated body.
    #[test]
    fn camera_action_v01_empty_001765f8_matches() {
        check(
            ee::s02::f_001765f8__generated,
            camera_action_v01_empty_001765f8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1765f8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `DirectionNeChaseCameraAlgorithm_GetEyeDrop` (0x0017_7548) against the generated body.
    #[test]
    fn direction_ne_chase_camera_algorithm_get_eye_drop_00177548_matches() {
        check(
            ee::s02::f_00177548__generated,
            direction_ne_chase_camera_algorithm_get_eye_drop_00177548,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x177548, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `DebugPrintf_Stub_7610` (0x0017_7610) against the generated body.
    #[test]
    fn debug_printf_stub_7610_matches() {
        check(
            ee::s02::f_00177610__generated,
            debug_printf_stub_7610,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x177610, 0);
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

    /// `Post_Race_1ChaseCameraAlgorithm_GetEyeDrop` (0x0017_8b98) against the generated body.
    #[test]
    fn post_race_1_chase_camera_algorithm_get_eye_drop_00178b98_matches() {
        check(
            ee::s02::f_00178b98__generated,
            post_race_1_chase_camera_algorithm_get_eye_drop_00178b98,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x178b98, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `Post_Race_1ChaseCameraAlgorithm_GetTuning` (0x0017_8ba8) against the generated body.
    #[test]
    fn post_race_1_chase_camera_algorithm_get_tuning_00178ba8_matches() {
        check(
            ee::s02::f_00178ba8__generated,
            post_race_1_chase_camera_algorithm_get_tuning_00178ba8,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x178ba8, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A1][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `DebugPrintf_Stub_8f18` (0x0017_8f18) against the generated body.
    #[test]
    fn debug_printf_stub_8f18_matches() {
        check(
            ee::s02::f_00178f18__generated,
            debug_printf_stub_8f18,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x178f18, 0);
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

    /// `DebugPrintf_Stub_9758` (0x0017_9758) against the generated body.
    #[test]
    fn debug_printf_stub_9758_matches() {
        check(
            ee::s02::f_00179758__generated,
            debug_printf_stub_9758,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x179758, 0);
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

    /// `cFEStateMemCard_OnWidgetNotify` (0x0018_6b48) against the generated body.
    #[test]
    fn c_fe_state_mem_card_on_widget_notify_00186b48_matches() {
        check(
            ee::s02::f_00186b48__generated,
            c_fe_state_mem_card_on_widget_notify_00186b48,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x186b48, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `cFEStateMemCard_HandleInput` (0x0018_7c10) against the generated body.
    #[test]
    fn c_fe_state_mem_card_handle_input_00187c10_matches() {
        check(
            ee::s02::f_00187c10__generated,
            c_fe_state_mem_card_handle_input_00187c10,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x187c10, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `cFEStateBraggingRights_TransitionIn` (0x0019_3310) against the generated body.
    #[test]
    fn c_fe_state_bragging_rights_transition_in_00193310_matches() {
        check(
            ee::s02::f_00193310__generated,
            c_fe_state_bragging_rights_transition_in_00193310,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x193310, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `cFEStateTitle_OnChildEvent` (0x0019_4980) against the generated body.
    #[test]
    fn c_fe_state_title_on_child_event_00194980_matches() {
        check(
            ee::s02::f_00194980__generated,
            c_fe_state_title_on_child_event_00194980,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x194980, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `cFEStateMPCircuit_OnWidgetCreated` (0x001a_3300) against the generated body.
    #[test]
    fn c_fe_state_mp_circuit_on_widget_created_001a3300_matches() {
        check(
            ee::s03::f_001a3300__generated,
            c_fe_state_mp_circuit_on_widget_created_001a3300,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1a3300, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `cFEStateReadMessagePopup_OnWidgetCreated` (0x001a_7bb0) against the generated body.
    #[test]
    fn c_fe_state_read_message_popup_on_widget_created_001a7bb0_matches() {
        check(
            ee::s03::f_001a7bb0__generated,
            c_fe_state_read_message_popup_on_widget_created_001a7bb0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1a7bb0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `cFEStateNetConfig_v21` (0x001b_6078) against the generated body.
    #[test]
    fn c_fe_state_net_config_v21_001b6078_matches() {
        check(
            ee::s03::f_001b6078__generated,
            c_fe_state_net_config_v21_001b6078,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1b6078, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `cFEStateNetCreateRoomPopup_v21` (0x001b_ea30) against the generated body.
    #[test]
    fn c_fe_state_net_create_room_popup_v21_001bea30_matches() {
        check(
            ee::s03::f_001bea30__generated,
            c_fe_state_net_create_room_popup_v21_001bea30,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1bea30, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `cFEStateNetQuickPlayPopup_v21` (0x001c_17d8) against the generated body.
    #[test]
    fn c_fe_state_net_quick_play_popup_v21_001c17d8_matches() {
        check(
            ee::s03::f_001c17d8__generated,
            c_fe_state_net_quick_play_popup_v21_001c17d8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1c17d8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `cFEPopupConfirm_TransitionOut` (0x001c_5dd0) against the generated body.
    #[test]
    fn c_fe_popup_confirm_transition_out_001c5dd0_matches() {
        check(
            ee::s03::f_001c5dd0__generated,
            c_fe_popup_confirm_transition_out_001c5dd0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1c5dd0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `cFEStateRewardGalleryPosters_GetIconPath` (0x001d_1198) against the generated body.
    #[test]
    fn c_fe_state_reward_gallery_posters_get_icon_path_001d1198_matches() {
        check(
            ee::s04::f_001d1198__generated,
            c_fe_state_reward_gallery_posters_get_icon_path_001d1198,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1d1198, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `cFEStateRewardGalleryTradingCards_GetIconPath` (0x001d_1330) against the generated body.
    #[test]
    fn c_fe_state_reward_gallery_trading_cards_get_icon_path_001d1330_matches() {
        check(
            ee::s04::f_001d1330__generated,
            c_fe_state_reward_gallery_trading_cards_get_icon_path_001d1330,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1d1330, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `cFEStateRewardGalleryConceptArts_GetIconPath` (0x001d_14c8) against the generated body.
    #[test]
    fn c_fe_state_reward_gallery_concept_arts_get_icon_path_001d14c8_matches() {
        check(
            ee::s04::f_001d14c8__generated,
            c_fe_state_reward_gallery_concept_arts_get_icon_path_001d14c8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1d14c8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `cFEStateRewardGalleryVideos_GetIconPath` (0x001d_1660) against the generated body.
    #[test]
    fn c_fe_state_reward_gallery_videos_get_icon_path_001d1660_matches() {
        check(
            ee::s04::f_001d1660__generated,
            c_fe_state_reward_gallery_videos_get_icon_path_001d1660,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1d1660, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `cFEStateRewardGalleryToys_GetIconPath` (0x001d_17f8) against the generated body.
    #[test]
    fn c_fe_state_reward_gallery_toys_get_icon_path_001d17f8_matches() {
        check(
            ee::s04::f_001d17f8__generated,
            c_fe_state_reward_gallery_toys_get_icon_path_001d17f8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1d17f8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `cFEStateNetHelpPopup_HandleInput` (0x001d_9f30) against the generated body.
    #[test]
    fn c_fe_state_net_help_popup_handle_input_001d9f30_matches() {
        check(
            ee::s04::f_001d9f30__generated,
            c_fe_state_net_help_popup_handle_input_001d9f30,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x1d9f30, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }
}
