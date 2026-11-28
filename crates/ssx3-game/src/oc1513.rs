//! Staging module oc1513 (wave wave276): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1513`.

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
        0x002c_1650,
        t_wpigd_weather_get78_002c1650,
        "tWPIGD_Weather_Get78",
    ),
    (
        0x002c_1658,
        t_wpigd_weather_get80_002c1658,
        "tWPIGD_Weather_Get80",
    ),
    (
        0x002c_1660,
        t_wpigd_weather_get88_002c1660,
        "tWPIGD_Weather_Get88",
    ),
    (
        0x002c_1668,
        t_wpigd_weather_get90_002c1668,
        "tWPIGD_Weather_Get90",
    ),
    (
        0x002c_1670,
        t_wpigd_weather_get98_002c1670,
        "tWPIGD_Weather_Get98",
    ),
    (
        0x002c_1678,
        t_wpigd_danger_get08_002c1678,
        "tWPIGD_Danger_Get08",
    ),
    (
        0x002c_1680,
        t_wpigd_danger_get10_002c1680,
        "tWPIGD_Danger_Get10",
    ),
    (0x002c_a368, menu_sys_item_on_focus, "MenuSysItem_OnFocus"),
    (0x002c_a370, menu_sys_item_on_blur, "MenuSysItem_OnBlur"),
    (
        0x002c_ad48,
        menu_sys_page_on_action_002cad48,
        "MenuSysPage_OnAction",
    ),
    (0x002c_ad50, menu_sys_page_v04_002cad50, "MenuSysPage_v04"),
    (0x002c_c0b8, menu_sys_set_active, "MenuSys_SetActive"),
    (0x002c_c248, menu_sys_get_input_code, "MenuSys_GetInputCode"),
    (0x002c_c250, menu_sys_get_key_code, "MenuSys_GetKeyCode"),
    (0x002c_c258, menu_sys_get_key_char, "MenuSys_GetKeyChar"),
    (
        0x002c_d2e8,
        menu_sys_label_item_is_selectable_002cd2e8,
        "MenuSysLabelItem_IsSelectable",
    ),
    (
        0x002c_d398,
        menu_sys_spacer_item_is_selectable_002cd398,
        "MenuSysSpacerItem_IsSelectable",
    ),
    (
        0x002c_d3f0,
        menu_sys_spacer_item_draw_002cd3f0,
        "MenuSysSpacerItem_Draw",
    ),
    (
        0x002c_ff80,
        menu_sys_color_swatch_item_is_selectable_002cff80,
        "MenuSysColorSwatchItem_IsSelectable",
    ),
    (
        0x002d_0448,
        menu_sys_vector_label_item_is_selectable_002d0448,
        "MenuSysVectorLabelItem_IsSelectable",
    ),
    (
        0x002d_2948,
        ssx_stats_man_debug_printf,
        "SSXStatsMan_DebugPrintf",
    ),
    (
        0x002d_39f8,
        menu_sys_item_reset_value_002d39f8,
        "MenuSysItem_ResetValue",
    ),
    (
        0x002d_4080,
        menu_sys_debug_printf_4080,
        "MenuSys_DebugPrintf_4080",
    ),
    (
        0x002f_1148,
        rider_base_reset_attack_fx,
        "RiderBase_ResetAttackFX",
    ),
    (
        0x002f_1a00,
        attached_emitter_fx_reset,
        "AttachedEmitterFX_Reset",
    ),
    (
        0x002f_6b48,
        world_light_man_debug_printf,
        "WorldLightMan_DebugPrintf",
    ),
    (
        0x002f_7c20,
        vfx_component_update_002f7c20,
        "VfxComponent_Update",
    ),
    (
        0x002f_7c28,
        vfx_component_render_002f7c28,
        "VfxComponent_Render",
    ),
    (0x002f_7c30, vfx_component_v04_002f7c30, "VfxComponent_v04"),
    (
        0x002f_7ff0,
        c_transition_man_script_v09_002f7ff0,
        "cTransitionMan_Script_v09",
    ),
    (0x002f_8168, snow_fall_save_002f8168, "SnowFall_Save"),
    (
        0x002f_9000,
        attached_emitter_fx_debug_printf,
        "AttachedEmitterFX_DebugPrintf",
    ),
    (
        0x002f_a600,
        attached_emitter_fx_debug_printf_2,
        "AttachedEmitterFX_DebugPrintf_2",
    ),
    (
        0x0030_b4b8,
        w_script_man_render_0030b4b8,
        "WScriptMan_Render",
    ),
    (
        0x0030_d3d8,
        w_script_process_is_mission_0030d3d8,
        "WScriptProcess_IsMission",
    ),
    (
        0x0030_d480,
        w_script_mission_is_mission_0030d480,
        "WScriptMission_IsMission",
    ),
    (
        0x0031_1860,
        anim_table_get_anim_name_stub,
        "AnimTable_GetAnimNameStub",
    ),
    (0x0031_50b8, anim_data_identity, "AnimData_Identity"),
    (0x0031_abc0, ps2_exec_man_v07_0031abc0, "PS2ExecMan_v07"),
    (
        0x0031_abc8,
        ps2_exec_man_get_frame_rate_0031abc8,
        "PS2ExecMan_GetFrameRate",
    ),
    (0x0032_0950, exec_man_base_v11_00320950, "ExecManBase_v11"),
    (0x0032_0958, exec_man_base_v12_00320958, "ExecManBase_v12"),
    (
        0x0032_7738,
        ps2_pad_get_num_actuators_00327738,
        "PS2Pad_GetNumActuators",
    ),
    (
        0x0032_e688,
        coll_segment_collide_sphere_tree_0032e688,
        "CollSegment_CollideSphereTree",
    ),
    (
        0x0032_f8b0,
        coll_hull_reset_transform_0032f8b0,
        "CollHull_ResetTransform",
    ),
];

/// `tWPIGD_Weather_Get78` (0x002c_1650): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_weather_get78_002c1650(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x78));
    ret(c);
}

/// `tWPIGD_Weather_Get80` (0x002c_1658): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_weather_get80_002c1658(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x80));
    ret(c);
}

/// `tWPIGD_Weather_Get88` (0x002c_1660): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_weather_get88_002c1660(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x88));
    ret(c);
}

/// `tWPIGD_Weather_Get90` (0x002c_1668): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_weather_get90_002c1668(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x90));
    ret(c);
}

/// `tWPIGD_Weather_Get98` (0x002c_1670): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_weather_get98_002c1670(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x98));
    ret(c);
}

/// `tWPIGD_Danger_Get08` (0x002c_1678): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_danger_get08_002c1678(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x8));
    ret(c);
}

/// `tWPIGD_Danger_Get10` (0x002c_1680): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_danger_get10_002c1680(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x10));
    ret(c);
}

/// `MenuSysItem_OnFocus` (0x002c_a368): straight-line leaf. Mechanical port of the generated body.
pub fn menu_sys_item_on_focus(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `MenuSysItem_OnBlur` (0x002c_a370): straight-line leaf. Mechanical port of the generated body.
pub fn menu_sys_item_on_blur(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `MenuSysPage_OnAction` (0x002c_ad48): straight-line leaf. Mechanical port of the generated body.
pub fn menu_sys_page_on_action_002cad48(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `MenuSysPage_v04` (0x002c_ad50): straight-line leaf. Mechanical port of the generated body.
pub fn menu_sys_page_v04_002cad50(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `MenuSys_SetActive` (0x002c_c0b8): straight-line leaf: 1 store(s). Mechanical port of the generated body.
pub fn menu_sys_set_active(c: &mut Cpu) {
    c.cycles += 2;
    c.sw((c.g[A0][0] as u32).wrapping_add(0x54), c.g[A1][0]);
    ret(c);
}

/// `MenuSys_GetInputCode` (0x002c_c248): straight-line leaf: 1 load(s). Mechanical port of the generated body.
pub fn menu_sys_get_input_code(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x48));
    ret(c);
}

/// `MenuSys_GetKeyCode` (0x002c_c250): straight-line leaf: 1 load(s). Mechanical port of the generated body.
pub fn menu_sys_get_key_code(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x4c));
    ret(c);
}

/// `MenuSys_GetKeyChar` (0x002c_c258): straight-line leaf: 1 load(s). Mechanical port of the generated body.
pub fn menu_sys_get_key_char(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x50));
    ret(c);
}

/// `MenuSysLabelItem_IsSelectable` (0x002c_d2e8): straight-line leaf. Mechanical port of the generated body.
pub fn menu_sys_label_item_is_selectable_002cd2e8(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `MenuSysSpacerItem_IsSelectable` (0x002c_d398): straight-line leaf. Mechanical port of the generated body.
pub fn menu_sys_spacer_item_is_selectable_002cd398(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `MenuSysSpacerItem_Draw` (0x002c_d3f0): straight-line leaf. Mechanical port of the generated body.
pub fn menu_sys_spacer_item_draw_002cd3f0(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `MenuSysColorSwatchItem_IsSelectable` (0x002c_ff80): straight-line leaf. Mechanical port of the generated body.
pub fn menu_sys_color_swatch_item_is_selectable_002cff80(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `MenuSysVectorLabelItem_IsSelectable` (0x002d_0448): straight-line leaf. Mechanical port of the generated body.
pub fn menu_sys_vector_label_item_is_selectable_002d0448(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `SSXStatsMan_DebugPrintf` (0x002d_2948): straight-line leaf: 11 store(s). Mechanical port of the generated body.
pub fn ssx_stats_man_debug_printf(c: &mut Cpu) {
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

/// `MenuSysItem_ResetValue` (0x002d_39f8): straight-line leaf. Mechanical port of the generated body.
pub fn menu_sys_item_reset_value_002d39f8(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `MenuSys_DebugPrintf_4080` (0x002d_4080): straight-line leaf: 11 store(s). Mechanical port of the generated body.
pub fn menu_sys_debug_printf_4080(c: &mut Cpu) {
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

/// `RiderBase_ResetAttackFX` (0x002f_1148): straight-line leaf: 1 store(s). Mechanical port of the generated body.
pub fn rider_base_reset_attack_fx(c: &mut Cpu) {
    c.cycles += 2;
    c.sw((c.g[A0][0] as u32).wrapping_add(0xa0), 0u64);
    ret(c);
}

/// `AttachedEmitterFX_Reset` (0x002f_1a00): straight-line leaf: 1 store(s). Mechanical port of the generated body.
pub fn attached_emitter_fx_reset(c: &mut Cpu) {
    c.cycles += 2;
    c.sw((c.g[A0][0] as u32).wrapping_add(0xa0), 0u64);
    ret(c);
}

/// `WorldLightMan_DebugPrintf` (0x002f_6b48): straight-line leaf: 11 store(s). Mechanical port of the generated body.
pub fn world_light_man_debug_printf(c: &mut Cpu) {
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

/// `VfxComponent_Update` (0x002f_7c20): straight-line leaf. Mechanical port of the generated body.
pub fn vfx_component_update_002f7c20(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `VfxComponent_Render` (0x002f_7c28): straight-line leaf. Mechanical port of the generated body.
pub fn vfx_component_render_002f7c28(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `VfxComponent_v04` (0x002f_7c30): straight-line leaf. Mechanical port of the generated body.
pub fn vfx_component_v04_002f7c30(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `cTransitionMan_Script_v09` (0x002f_7ff0): straight-line leaf: 1 load(s). Mechanical port of the generated body.
pub fn c_transition_man_script_v09_002f7ff0(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x74));
    ret(c);
}

/// `SnowFall_Save` (0x002f_8168): straight-line leaf. Mechanical port of the generated body.
pub fn snow_fall_save_002f8168(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `AttachedEmitterFX_DebugPrintf` (0x002f_9000): straight-line leaf: 11 store(s). Mechanical port of the generated body.
pub fn attached_emitter_fx_debug_printf(c: &mut Cpu) {
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

/// `AttachedEmitterFX_DebugPrintf_2` (0x002f_a600): straight-line leaf: 11 store(s). Mechanical port of the generated body.
pub fn attached_emitter_fx_debug_printf_2(c: &mut Cpu) {
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

/// `WScriptMan_Render` (0x0030_b4b8): straight-line leaf. Mechanical port of the generated body.
pub fn w_script_man_render_0030b4b8(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `WScriptProcess_IsMission` (0x0030_d3d8): straight-line leaf. Mechanical port of the generated body.
pub fn w_script_process_is_mission_0030d3d8(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `WScriptMission_IsMission` (0x0030_d480): straight-line leaf. Mechanical port of the generated body.
pub fn w_script_mission_is_mission_0030d480(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0x1;
    ret(c);
}

/// `AnimTable_GetAnimNameStub` (0x0031_1860): straight-line leaf. Mechanical port of the generated body.
pub fn anim_table_get_anim_name_stub(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = 0x490000;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x5b40));
    ret(c);
}

/// `AnimData_Identity` (0x0031_50b8): straight-line leaf: 11 store(s). Mechanical port of the generated body.
pub fn anim_data_identity(c: &mut Cpu) {
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

/// `PS2ExecMan_v07` (0x0031_abc0): straight-line leaf. Mechanical port of the generated body.
pub fn ps2_exec_man_v07_0031abc0(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `PS2ExecMan_GetFrameRate` (0x0031_abc8): straight-line leaf. Mechanical port of the generated body.
pub fn ps2_exec_man_get_frame_rate_0031abc8(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0x3c;
    ret(c);
}

/// `ExecManBase_v11` (0x0032_0950): straight-line leaf. Mechanical port of the generated body.
pub fn exec_man_base_v11_00320950(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `ExecManBase_v12` (0x0032_0958): straight-line leaf. Mechanical port of the generated body.
pub fn exec_man_base_v12_00320958(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `PS2Pad_GetNumActuators` (0x0032_7738): straight-line leaf. Mechanical port of the generated body.
pub fn ps2_pad_get_num_actuators_00327738(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0x2;
    ret(c);
}

/// `CollSegment_CollideSphereTree` (0x0032_e688): straight-line leaf. Mechanical port of the generated body.
pub fn coll_segment_collide_sphere_tree_0032e688(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `CollHull_ResetTransform` (0x0032_f8b0): straight-line leaf: 1 load(s), 2 store(s). Mechanical port of the generated body.
pub fn coll_hull_reset_transform_0032f8b0(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x64));
    c.sw((c.g[A0][0] as u32).wrapping_add(0x4), 0u64);
    c.sw((c.g[A0][0] as u32).wrapping_add(0x60), c.g[V0][0]);
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

    /// `tWPIGD_Weather_Get78` (0x002c_1650) against the generated body.
    #[test]
    fn t_wpigd_weather_get78_002c1650_matches() {
        check(
            ee::s08::f_002c1650__generated,
            t_wpigd_weather_get78_002c1650,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c1650, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Weather_Get80` (0x002c_1658) against the generated body.
    #[test]
    fn t_wpigd_weather_get80_002c1658_matches() {
        check(
            ee::s08::f_002c1658__generated,
            t_wpigd_weather_get80_002c1658,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c1658, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Weather_Get88` (0x002c_1660) against the generated body.
    #[test]
    fn t_wpigd_weather_get88_002c1660_matches() {
        check(
            ee::s08::f_002c1660__generated,
            t_wpigd_weather_get88_002c1660,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c1660, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Weather_Get90` (0x002c_1668) against the generated body.
    #[test]
    fn t_wpigd_weather_get90_002c1668_matches() {
        check(
            ee::s08::f_002c1668__generated,
            t_wpigd_weather_get90_002c1668,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c1668, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Weather_Get98` (0x002c_1670) against the generated body.
    #[test]
    fn t_wpigd_weather_get98_002c1670_matches() {
        check(
            ee::s08::f_002c1670__generated,
            t_wpigd_weather_get98_002c1670,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c1670, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Danger_Get08` (0x002c_1678) against the generated body.
    #[test]
    fn t_wpigd_danger_get08_002c1678_matches() {
        check(
            ee::s08::f_002c1678__generated,
            t_wpigd_danger_get08_002c1678,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c1678, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Danger_Get10` (0x002c_1680) against the generated body.
    #[test]
    fn t_wpigd_danger_get10_002c1680_matches() {
        check(
            ee::s08::f_002c1680__generated,
            t_wpigd_danger_get10_002c1680,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c1680, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `MenuSysItem_OnFocus` (0x002c_a368) against the generated body.
    #[test]
    fn menu_sys_item_on_focus_matches() {
        check(
            ee::s09::f_002ca368__generated,
            menu_sys_item_on_focus,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2ca368, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `MenuSysItem_OnBlur` (0x002c_a370) against the generated body.
    #[test]
    fn menu_sys_item_on_blur_matches() {
        check(
            ee::s09::f_002ca370__generated,
            menu_sys_item_on_blur,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2ca370, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `MenuSysPage_OnAction` (0x002c_ad48) against the generated body.
    #[test]
    fn menu_sys_page_on_action_002cad48_matches() {
        check(
            ee::s09::f_002cad48__generated,
            menu_sys_page_on_action_002cad48,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2cad48, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `MenuSysPage_v04` (0x002c_ad50) against the generated body.
    #[test]
    fn menu_sys_page_v04_002cad50_matches() {
        check(
            ee::s09::f_002cad50__generated,
            menu_sys_page_v04_002cad50,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2cad50, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `MenuSys_SetActive` (0x002c_c0b8) against the generated body.
    #[test]
    fn menu_sys_set_active_matches() {
        check(
            ee::s09::f_002cc0b8__generated,
            menu_sys_set_active,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2cc0b8, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[A1][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `MenuSys_GetInputCode` (0x002c_c248) against the generated body.
    #[test]
    fn menu_sys_get_input_code_matches() {
        check(
            ee::s09::f_002cc248__generated,
            menu_sys_get_input_code,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2cc248, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `MenuSys_GetKeyCode` (0x002c_c250) against the generated body.
    #[test]
    fn menu_sys_get_key_code_matches() {
        check(
            ee::s09::f_002cc250__generated,
            menu_sys_get_key_code,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2cc250, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `MenuSys_GetKeyChar` (0x002c_c258) against the generated body.
    #[test]
    fn menu_sys_get_key_char_matches() {
        check(
            ee::s09::f_002cc258__generated,
            menu_sys_get_key_char,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2cc258, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `MenuSysLabelItem_IsSelectable` (0x002c_d2e8) against the generated body.
    #[test]
    fn menu_sys_label_item_is_selectable_002cd2e8_matches() {
        check(
            ee::s09::f_002cd2e8__generated,
            menu_sys_label_item_is_selectable_002cd2e8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2cd2e8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `MenuSysSpacerItem_IsSelectable` (0x002c_d398) against the generated body.
    #[test]
    fn menu_sys_spacer_item_is_selectable_002cd398_matches() {
        check(
            ee::s09::f_002cd398__generated,
            menu_sys_spacer_item_is_selectable_002cd398,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2cd398, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `MenuSysSpacerItem_Draw` (0x002c_d3f0) against the generated body.
    #[test]
    fn menu_sys_spacer_item_draw_002cd3f0_matches() {
        check(
            ee::s09::f_002cd3f0__generated,
            menu_sys_spacer_item_draw_002cd3f0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2cd3f0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `MenuSysColorSwatchItem_IsSelectable` (0x002c_ff80) against the generated body.
    #[test]
    fn menu_sys_color_swatch_item_is_selectable_002cff80_matches() {
        check(
            ee::s09::f_002cff80__generated,
            menu_sys_color_swatch_item_is_selectable_002cff80,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2cff80, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `MenuSysVectorLabelItem_IsSelectable` (0x002d_0448) against the generated body.
    #[test]
    fn menu_sys_vector_label_item_is_selectable_002d0448_matches() {
        check(
            ee::s09::f_002d0448__generated,
            menu_sys_vector_label_item_is_selectable_002d0448,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2d0448, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `SSXStatsMan_DebugPrintf` (0x002d_2948) against the generated body.
    #[test]
    fn ssx_stats_man_debug_printf_matches() {
        check(
            ee::s09::f_002d2948__generated,
            ssx_stats_man_debug_printf,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2d2948, 0);
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

    /// `MenuSysItem_ResetValue` (0x002d_39f8) against the generated body.
    #[test]
    fn menu_sys_item_reset_value_002d39f8_matches() {
        check(
            ee::s09::f_002d39f8__generated,
            menu_sys_item_reset_value_002d39f8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2d39f8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `MenuSys_DebugPrintf_4080` (0x002d_4080) against the generated body.
    #[test]
    fn menu_sys_debug_printf_4080_matches() {
        check(
            ee::s09::f_002d4080__generated,
            menu_sys_debug_printf_4080,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2d4080, 0);
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

    /// `RiderBase_ResetAttackFX` (0x002f_1148) against the generated body.
    #[test]
    fn rider_base_reset_attack_fx_matches() {
        check(
            ee::s09::f_002f1148__generated,
            rider_base_reset_attack_fx,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2f1148, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `AttachedEmitterFX_Reset` (0x002f_1a00) against the generated body.
    #[test]
    fn attached_emitter_fx_reset_matches() {
        check(
            ee::s09::f_002f1a00__generated,
            attached_emitter_fx_reset,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2f1a00, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `WorldLightMan_DebugPrintf` (0x002f_6b48) against the generated body.
    #[test]
    fn world_light_man_debug_printf_matches() {
        check(
            ee::s09::f_002f6b48__generated,
            world_light_man_debug_printf,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2f6b48, 0);
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

    /// `VfxComponent_Update` (0x002f_7c20) against the generated body.
    #[test]
    fn vfx_component_update_002f7c20_matches() {
        check(
            ee::s09::f_002f7c20__generated,
            vfx_component_update_002f7c20,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2f7c20, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `VfxComponent_Render` (0x002f_7c28) against the generated body.
    #[test]
    fn vfx_component_render_002f7c28_matches() {
        check(
            ee::s09::f_002f7c28__generated,
            vfx_component_render_002f7c28,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2f7c28, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `VfxComponent_v04` (0x002f_7c30) against the generated body.
    #[test]
    fn vfx_component_v04_002f7c30_matches() {
        check(
            ee::s09::f_002f7c30__generated,
            vfx_component_v04_002f7c30,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2f7c30, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `cTransitionMan_Script_v09` (0x002f_7ff0) against the generated body.
    #[test]
    fn c_transition_man_script_v09_002f7ff0_matches() {
        check(
            ee::s09::f_002f7ff0__generated,
            c_transition_man_script_v09_002f7ff0,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2f7ff0, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `SnowFall_Save` (0x002f_8168) against the generated body.
    #[test]
    fn snow_fall_save_002f8168_matches() {
        check(
            ee::s09::f_002f8168__generated,
            snow_fall_save_002f8168,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2f8168, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `AttachedEmitterFX_DebugPrintf` (0x002f_9000) against the generated body.
    #[test]
    fn attached_emitter_fx_debug_printf_matches() {
        check(
            ee::s09::f_002f9000__generated,
            attached_emitter_fx_debug_printf,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2f9000, 0);
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

    /// `AttachedEmitterFX_DebugPrintf_2` (0x002f_a600) against the generated body.
    #[test]
    fn attached_emitter_fx_debug_printf_2_matches() {
        check(
            ee::s10::f_002fa600__generated,
            attached_emitter_fx_debug_printf_2,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2fa600, 0);
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

    /// `WScriptMan_Render` (0x0030_b4b8) against the generated body.
    #[test]
    fn w_script_man_render_0030b4b8_matches() {
        check(
            ee::s10::f_0030b4b8__generated,
            w_script_man_render_0030b4b8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x30b4b8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `WScriptProcess_IsMission` (0x0030_d3d8) against the generated body.
    #[test]
    fn w_script_process_is_mission_0030d3d8_matches() {
        check(
            ee::s10::f_0030d3d8__generated,
            w_script_process_is_mission_0030d3d8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x30d3d8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `WScriptMission_IsMission` (0x0030_d480) against the generated body.
    #[test]
    fn w_script_mission_is_mission_0030d480_matches() {
        check(
            ee::s10::f_0030d480__generated,
            w_script_mission_is_mission_0030d480,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x30d480, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `AnimTable_GetAnimNameStub` (0x0031_1860) against the generated body.
    #[test]
    fn anim_table_get_anim_name_stub_matches() {
        check(
            ee::s10::f_00311860__generated,
            anim_table_get_anim_name_stub,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x311860, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `AnimData_Identity` (0x0031_50b8) against the generated body.
    #[test]
    fn anim_data_identity_matches() {
        check(
            ee::s10::f_003150b8__generated,
            anim_data_identity,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3150b8, 0);
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

    /// `PS2ExecMan_v07` (0x0031_abc0) against the generated body.
    #[test]
    fn ps2_exec_man_v07_0031abc0_matches() {
        check(
            ee::s10::f_0031abc0__generated,
            ps2_exec_man_v07_0031abc0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x31abc0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `PS2ExecMan_GetFrameRate` (0x0031_abc8) against the generated body.
    #[test]
    fn ps2_exec_man_get_frame_rate_0031abc8_matches() {
        check(
            ee::s10::f_0031abc8__generated,
            ps2_exec_man_get_frame_rate_0031abc8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x31abc8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `ExecManBase_v11` (0x0032_0950) against the generated body.
    #[test]
    fn exec_man_base_v11_00320950_matches() {
        check(
            ee::s10::f_00320950__generated,
            exec_man_base_v11_00320950,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x320950, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `ExecManBase_v12` (0x0032_0958) against the generated body.
    #[test]
    fn exec_man_base_v12_00320958_matches() {
        check(
            ee::s10::f_00320958__generated,
            exec_man_base_v12_00320958,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x320958, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `PS2Pad_GetNumActuators` (0x0032_7738) against the generated body.
    #[test]
    fn ps2_pad_get_num_actuators_00327738_matches() {
        check(
            ee::s10::f_00327738__generated,
            ps2_pad_get_num_actuators_00327738,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x327738, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `CollSegment_CollideSphereTree` (0x0032_e688) against the generated body.
    #[test]
    fn coll_segment_collide_sphere_tree_0032e688_matches() {
        check(
            ee::s11::f_0032e688__generated,
            coll_segment_collide_sphere_tree_0032e688,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x32e688, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `CollHull_ResetTransform` (0x0032_f8b0) against the generated body.
    #[test]
    fn coll_hull_reset_transform_0032f8b0_matches() {
        check(
            ee::s11::f_0032f8b0__generated,
            coll_hull_reset_transform_0032f8b0,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x32f8b0, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }
}
