//! Staging module oc1511 (wave wave276): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1511`.

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
        0x002b_c6d0,
        t_wpigd_base_get_weather88_002bc6d0,
        "tWPIGD_Base_GetWeather88",
    ),
    (
        0x002b_c6d8,
        t_wpigd_base_get_weather90_002bc6d8,
        "tWPIGD_Base_GetWeather90",
    ),
    (
        0x002b_c6e0,
        t_wpigd_base_get_weather98_002bc6e0,
        "tWPIGD_Base_GetWeather98",
    ),
    (
        0x002b_c6e8,
        t_wpigd_base_get_danger08_002bc6e8,
        "tWPIGD_Base_GetDanger08",
    ),
    (
        0x002b_c6f0,
        t_wpigd_base_get_danger10_002bc6f0,
        "tWPIGD_Base_GetDanger10",
    ),
    (
        0x002b_da28,
        t_wpigd_music_trigger_v67_002bda28,
        "tWPIGD_MusicTrigger_v67",
    ),
    (
        0x002b_da30,
        t_wpigd_ambience_v67_002bda30,
        "tWPIGD_Ambience_v67",
    ),
    (
        0x002b_da38,
        t_wpigd_camera_v67_002bda38,
        "tWPIGD_Camera_v67",
    ),
    (
        0x002b_da40,
        t_wpigd_danger_v67_002bda40,
        "tWPIGD_Danger_v67",
    ),
    (0x002b_da48, t_wpigd_fog_v67_002bda48, "tWPIGD_Fog_v67"),
    (
        0x002b_da50,
        t_wpigd_light_glow_v67_002bda50,
        "tWPIGD_LightGlow_v67",
    ),
    (
        0x002b_da58,
        t_wpigd_lighting_v67_002bda58,
        "tWPIGD_Lighting_v67",
    ),
    (0x002b_da60, t_wpigd_mix_v67_002bda60, "tWPIGD_Mix_v67"),
    (
        0x002b_da68,
        t_wpigd_screen_tint_v67_002bda68,
        "tWPIGD_ScreenTint_v67",
    ),
    (
        0x002b_da70,
        t_wpigd_sky_box_v67_002bda70,
        "tWPIGD_SkyBox_v67",
    ),
    (
        0x002b_da78,
        t_wpigd_speech_v67_002bda78,
        "tWPIGD_Speech_v67",
    ),
    (0x002b_da80, t_wpigd_sun_v67_002bda80, "tWPIGD_Sun_v67"),
    (
        0x002b_da88,
        t_wpigd_surface_v67_002bda88,
        "tWPIGD_Surface_v67",
    ),
    (
        0x002b_da90,
        t_wpigd_weather_v67_002bda90,
        "tWPIGD_Weather_v67",
    ),
    (
        0x002b_dac8,
        t_wpigd_mix_is_equal_002bdac8,
        "tWPIGD_Mix_IsEqual",
    ),
    (
        0x002b_dae0,
        t_wpigd_ambience_is_equal_002bdae0,
        "tWPIGD_Ambience_IsEqual",
    ),
    (
        0x002b_daf8,
        t_wpigd_speech_is_equal_002bdaf8,
        "tWPIGD_Speech_IsEqual",
    ),
    (
        0x002b_e0b0,
        t_wpigd_music_trigger_set_defaults_002be0b0,
        "tWPIGD_MusicTrigger_SetDefaults",
    ),
    (
        0x002b_e0c8,
        t_wpigd_mix_set_defaults_002be0c8,
        "tWPIGD_Mix_SetDefaults",
    ),
    (
        0x002b_e0d8,
        t_wpigd_ambience_set_defaults_002be0d8,
        "tWPIGD_Ambience_SetDefaults",
    ),
    (
        0x002b_e0e8,
        t_wpigd_speech_set_defaults_002be0e8,
        "tWPIGD_Speech_SetDefaults",
    ),
    (
        0x002b_e0f8,
        t_wpigd_camera_set_defaults_002be0f8,
        "tWPIGD_Camera_SetDefaults",
    ),
    (
        0x002b_e108,
        t_wpigd_fog_set_defaults_002be108,
        "tWPIGD_Fog_SetDefaults",
    ),
    (
        0x002b_e140,
        t_wpigd_light_glow_set_defaults_002be140,
        "tWPIGD_LightGlow_SetDefaults",
    ),
    (
        0x002b_e170,
        t_wpigd_screen_tint_set_defaults_002be170,
        "tWPIGD_ScreenTint_SetDefaults",
    ),
    (
        0x002b_e198,
        t_wpigd_sky_box_set_defaults_002be198,
        "tWPIGD_SkyBox_SetDefaults",
    ),
    (
        0x002b_e1a8,
        t_wpigd_sun_set_defaults_002be1a8,
        "tWPIGD_Sun_SetDefaults",
    ),
    (
        0x002b_e1e0,
        t_wpigd_surface_set_defaults_002be1e0,
        "tWPIGD_Surface_SetDefaults",
    ),
    (
        0x002b_e2c8,
        t_wpigd_danger_set_defaults_002be2c8,
        "tWPIGD_Danger_SetDefaults",
    ),
    (
        0x002c_0b30,
        world_painter_query_debug_printf,
        "WorldPainterQuery_DebugPrintf",
    ),
    (0x002c_1458, t_wpigd_base_v67_002c1458, "tWPIGD_Base_v67"),
    (
        0x002c_1490,
        t_wpigd_music_trigger_get08_002c1490,
        "tWPIGD_MusicTrigger_Get08",
    ),
    (
        0x002c_1498,
        t_wpigd_music_trigger_get10_002c1498,
        "tWPIGD_MusicTrigger_Get10",
    ),
    (0x002c_14a0, t_wpigd_mix_get08_002c14a0, "tWPIGD_Mix_Get08"),
    (
        0x002c_14a8,
        t_wpigd_ambience_get08_002c14a8,
        "tWPIGD_Ambience_Get08",
    ),
    (
        0x002c_14b0,
        t_wpigd_speech_get08_002c14b0,
        "tWPIGD_Speech_Get08",
    ),
    (
        0x002c_14b8,
        t_wpigd_camera_get08_002c14b8,
        "tWPIGD_Camera_Get08",
    ),
    (0x002c_14c0, t_wpigd_fog_get08_002c14c0, "tWPIGD_Fog_Get08"),
    (0x002c_14c8, t_wpigd_fog_get10_002c14c8, "tWPIGD_Fog_Get10"),
    (0x002c_14d0, t_wpigd_fog_get18_002c14d0, "tWPIGD_Fog_Get18"),
];

/// `tWPIGD_Base_GetWeather88` (0x002b_c6d0): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_base_get_weather88_002bc6d0(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `tWPIGD_Base_GetWeather90` (0x002b_c6d8): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_base_get_weather90_002bc6d8(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `tWPIGD_Base_GetWeather98` (0x002b_c6e0): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_base_get_weather98_002bc6e0(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `tWPIGD_Base_GetDanger08` (0x002b_c6e8): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_base_get_danger08_002bc6e8(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `tWPIGD_Base_GetDanger10` (0x002b_c6f0): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_base_get_danger10_002bc6f0(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `tWPIGD_MusicTrigger_v67` (0x002b_da28): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_music_trigger_v67_002bda28(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `tWPIGD_Ambience_v67` (0x002b_da30): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_ambience_v67_002bda30(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `tWPIGD_Camera_v67` (0x002b_da38): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_camera_v67_002bda38(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `tWPIGD_Danger_v67` (0x002b_da40): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_danger_v67_002bda40(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `tWPIGD_Fog_v67` (0x002b_da48): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_fog_v67_002bda48(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `tWPIGD_LightGlow_v67` (0x002b_da50): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_light_glow_v67_002bda50(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `tWPIGD_Lighting_v67` (0x002b_da58): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_lighting_v67_002bda58(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `tWPIGD_Mix_v67` (0x002b_da60): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_mix_v67_002bda60(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `tWPIGD_ScreenTint_v67` (0x002b_da68): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_screen_tint_v67_002bda68(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `tWPIGD_SkyBox_v67` (0x002b_da70): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_sky_box_v67_002bda70(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `tWPIGD_Speech_v67` (0x002b_da78): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_speech_v67_002bda78(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `tWPIGD_Sun_v67` (0x002b_da80): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_sun_v67_002bda80(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `tWPIGD_Surface_v67` (0x002b_da88): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_surface_v67_002bda88(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `tWPIGD_Weather_v67` (0x002b_da90): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_weather_v67_002bda90(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `tWPIGD_Mix_IsEqual` (0x002b_dac8): straight-line leaf: 2 load(s). Mechanical port of the generated body.
pub fn t_wpigd_mix_is_equal_002bdac8(c: &mut Cpu) {
    c.cycles += 5;
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4));
    c.g[V0][0] = c.g[V0][0] ^ c.g[V1][0];
    c.g[V0][0] = (c.g[V0][0] < 0x1u64) as u64;
    ret(c);
}

/// `tWPIGD_Ambience_IsEqual` (0x002b_dae0): straight-line leaf: 2 load(s). Mechanical port of the generated body.
pub fn t_wpigd_ambience_is_equal_002bdae0(c: &mut Cpu) {
    c.cycles += 5;
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4));
    c.g[V0][0] = c.g[V0][0] ^ c.g[V1][0];
    c.g[V0][0] = (c.g[V0][0] < 0x1u64) as u64;
    ret(c);
}

/// `tWPIGD_Speech_IsEqual` (0x002b_daf8): straight-line leaf: 2 load(s). Mechanical port of the generated body.
pub fn t_wpigd_speech_is_equal_002bdaf8(c: &mut Cpu) {
    c.cycles += 5;
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x8));
    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x4));
    c.g[V0][0] = c.g[V0][0] ^ c.g[V1][0];
    c.g[V0][0] = (c.g[V0][0] < 0x1u64) as u64;
    ret(c);
}

/// `tWPIGD_MusicTrigger_SetDefaults` (0x002b_e0b0): straight-line leaf: 3 store(s). Mechanical port of the generated body.
pub fn t_wpigd_music_trigger_set_defaults_002be0b0(c: &mut Cpu) {
    c.cycles += 5;
    c.g[V0][0] = 0xffffffffffffffff;
    c.sw((c.g[A0][0] as u32), 0u64);
    c.sw((c.g[A0][0] as u32).wrapping_add(0x10), c.g[V0][0]);
    c.sw((c.g[A0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
    ret(c);
}

/// `tWPIGD_Mix_SetDefaults` (0x002b_e0c8): straight-line leaf: 2 store(s). Mechanical port of the generated body.
pub fn t_wpigd_mix_set_defaults_002be0c8(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V0][0] = 0xffffffffffffffff;
    c.sw((c.g[A0][0] as u32), 0u64);
    c.sw((c.g[A0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
    ret(c);
}

/// `tWPIGD_Ambience_SetDefaults` (0x002b_e0d8): straight-line leaf: 2 store(s). Mechanical port of the generated body.
pub fn t_wpigd_ambience_set_defaults_002be0d8(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V0][0] = 0xffffffffffffffff;
    c.sw((c.g[A0][0] as u32), 0u64);
    c.sw((c.g[A0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
    ret(c);
}

/// `tWPIGD_Speech_SetDefaults` (0x002b_e0e8): straight-line leaf: 2 store(s). Mechanical port of the generated body.
pub fn t_wpigd_speech_set_defaults_002be0e8(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V0][0] = 0xffffffffffffffff;
    c.sw((c.g[A0][0] as u32), 0u64);
    c.sw((c.g[A0][0] as u32).wrapping_add(0x8), c.g[V0][0]);
    ret(c);
}

/// `tWPIGD_Camera_SetDefaults` (0x002b_e0f8): straight-line leaf: 1 load(s), 2 store(s). Mechanical port of the generated body.
pub fn t_wpigd_camera_set_defaults_002be0f8(c: &mut Cpu) {
    c.cycles += 4;
    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x42a8));
    c.sw((c.g[A0][0] as u32), 0u64);
    c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x8));
    ret(c);
}

/// `tWPIGD_Fog_SetDefaults` (0x002b_e108): straight-line leaf: 5 load(s), 7 store(s). Mechanical port of the generated body.
pub fn t_wpigd_fog_set_defaults_002be108(c: &mut Cpu) {
    c.cycles += 13;
    c.lwc1(4, (c.g[GP][0] as u32).wrapping_sub(0x42a4));
    c.lwc1(0, (c.g[GP][0] as u32).wrapping_sub(0x42a0));
    c.lwc1(1, (c.g[GP][0] as u32).wrapping_sub(0x429c));
    c.lwc1(2, (c.g[GP][0] as u32).wrapping_sub(0x4298));
    c.lwc1(3, (c.g[GP][0] as u32).wrapping_sub(0x4294));
    c.swc1(4, (c.g[A0][0] as u32).wrapping_add(0x10));
    c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x18));
    c.swc1(1, (c.g[A0][0] as u32).wrapping_add(0x20));
    c.swc1(2, (c.g[A0][0] as u32).wrapping_add(0x28));
    c.swc1(3, (c.g[A0][0] as u32).wrapping_add(0x30));
    c.sw((c.g[A0][0] as u32), 0u64);
    c.sw((c.g[A0][0] as u32).wrapping_add(0x8), 0u64);
    ret(c);
}

/// `tWPIGD_LightGlow_SetDefaults` (0x002b_e140): straight-line leaf: 8 store(s). Mechanical port of the generated body.
pub fn t_wpigd_light_glow_set_defaults_002be140(c: &mut Cpu) {
    c.cycles += 11;
    c.g[AT][0] = 0x3f800000;
    c.mtc1(1, 0);
    c.sw((c.g[A0][0] as u32), 0u64);
    c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x28));
    c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x8));
    c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x10));
    c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x18));
    c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x20));
    c.sw((c.g[A0][0] as u32).wrapping_add(0x30), 0u64);
    c.sw((c.g[A0][0] as u32).wrapping_add(0x38), 0u64);
    ret(c);
}

/// `tWPIGD_ScreenTint_SetDefaults` (0x002b_e170): straight-line leaf: 7 store(s). Mechanical port of the generated body.
pub fn t_wpigd_screen_tint_set_defaults_002be170(c: &mut Cpu) {
    c.cycles += 10;
    c.g[AT][0] = 0x3f800000;
    c.mtc1(1, 0);
    c.sw((c.g[A0][0] as u32), 0u64);
    c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x18));
    c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x8));
    c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x10));
    c.sw((c.g[A0][0] as u32).wrapping_add(0x20), 0u64);
    c.sw((c.g[A0][0] as u32).wrapping_add(0x28), 0u64);
    c.sw((c.g[A0][0] as u32).wrapping_add(0x30), 0u64);
    ret(c);
}

/// `tWPIGD_SkyBox_SetDefaults` (0x002b_e198): straight-line leaf: 2 store(s). Mechanical port of the generated body.
pub fn t_wpigd_sky_box_set_defaults_002be198(c: &mut Cpu) {
    c.cycles += 3;
    c.sw((c.g[A0][0] as u32), 0u64);
    c.sw((c.g[A0][0] as u32).wrapping_add(0x8), 0u64);
    ret(c);
}

/// `tWPIGD_Sun_SetDefaults` (0x002b_e1a8): straight-line leaf: 10 store(s). Mechanical port of the generated body.
pub fn t_wpigd_sun_set_defaults_002be1a8(c: &mut Cpu) {
    c.cycles += 13;
    c.g[AT][0] = 0x3f800000;
    c.mtc1(1, 0);
    c.sw((c.g[A0][0] as u32), 0u64);
    c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x48));
    c.sw((c.g[A0][0] as u32).wrapping_add(0x8), 0u64);
    c.sw((c.g[A0][0] as u32).wrapping_add(0x10), 0u64);
    c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x18));
    c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x20));
    c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x28));
    c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x30));
    c.sw((c.g[A0][0] as u32).wrapping_add(0x38), 0u64);
    c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x40));
    ret(c);
}

/// `tWPIGD_Surface_SetDefaults` (0x002b_e1e0): straight-line leaf: 2 store(s). Mechanical port of the generated body.
pub fn t_wpigd_surface_set_defaults_002be1e0(c: &mut Cpu) {
    c.cycles += 5;
    c.g[AT][0] = 0x3f800000;
    c.mtc1(1, 0);
    c.sw((c.g[A0][0] as u32), 0u64);
    c.swc1(0, (c.g[A0][0] as u32).wrapping_add(0x8));
    ret(c);
}

/// `tWPIGD_Danger_SetDefaults` (0x002b_e2c8): straight-line leaf: 3 store(s). Mechanical port of the generated body.
pub fn t_wpigd_danger_set_defaults_002be2c8(c: &mut Cpu) {
    c.cycles += 4;
    c.sw((c.g[A0][0] as u32), 0u64);
    c.sw((c.g[A0][0] as u32).wrapping_add(0x8), 0u64);
    c.sw((c.g[A0][0] as u32).wrapping_add(0x10), 0u64);
    ret(c);
}

/// `WorldPainterQuery_DebugPrintf` (0x002c_0b30): straight-line leaf: 11 store(s). Mechanical port of the generated body.
pub fn world_painter_query_debug_printf(c: &mut Cpu) {
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

/// `tWPIGD_Base_v67` (0x002c_1458): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_base_v67_002c1458(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `tWPIGD_MusicTrigger_Get08` (0x002c_1490): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_music_trigger_get08_002c1490(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x8));
    ret(c);
}

/// `tWPIGD_MusicTrigger_Get10` (0x002c_1498): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_music_trigger_get10_002c1498(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x10));
    ret(c);
}

/// `tWPIGD_Mix_Get08` (0x002c_14a0): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_mix_get08_002c14a0(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x8));
    ret(c);
}

/// `tWPIGD_Ambience_Get08` (0x002c_14a8): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_ambience_get08_002c14a8(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x8));
    ret(c);
}

/// `tWPIGD_Speech_Get08` (0x002c_14b0): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_speech_get08_002c14b0(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x8));
    ret(c);
}

/// `tWPIGD_Camera_Get08` (0x002c_14b8): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_camera_get08_002c14b8(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x8));
    ret(c);
}

/// `tWPIGD_Fog_Get08` (0x002c_14c0): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_fog_get08_002c14c0(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x8));
    ret(c);
}

/// `tWPIGD_Fog_Get10` (0x002c_14c8): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_fog_get10_002c14c8(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x10));
    ret(c);
}

/// `tWPIGD_Fog_Get18` (0x002c_14d0): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_fog_get18_002c14d0(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
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

    /// `tWPIGD_Base_GetWeather88` (0x002b_c6d0) against the generated body.
    #[test]
    fn t_wpigd_base_get_weather88_002bc6d0_matches() {
        check(
            ee::s08::f_002bc6d0__generated,
            t_wpigd_base_get_weather88_002bc6d0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bc6d0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Base_GetWeather90` (0x002b_c6d8) against the generated body.
    #[test]
    fn t_wpigd_base_get_weather90_002bc6d8_matches() {
        check(
            ee::s08::f_002bc6d8__generated,
            t_wpigd_base_get_weather90_002bc6d8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bc6d8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Base_GetWeather98` (0x002b_c6e0) against the generated body.
    #[test]
    fn t_wpigd_base_get_weather98_002bc6e0_matches() {
        check(
            ee::s08::f_002bc6e0__generated,
            t_wpigd_base_get_weather98_002bc6e0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bc6e0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Base_GetDanger08` (0x002b_c6e8) against the generated body.
    #[test]
    fn t_wpigd_base_get_danger08_002bc6e8_matches() {
        check(
            ee::s08::f_002bc6e8__generated,
            t_wpigd_base_get_danger08_002bc6e8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bc6e8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Base_GetDanger10` (0x002b_c6f0) against the generated body.
    #[test]
    fn t_wpigd_base_get_danger10_002bc6f0_matches() {
        check(
            ee::s08::f_002bc6f0__generated,
            t_wpigd_base_get_danger10_002bc6f0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bc6f0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_MusicTrigger_v67` (0x002b_da28) against the generated body.
    #[test]
    fn t_wpigd_music_trigger_v67_002bda28_matches() {
        check(
            ee::s08::f_002bda28__generated,
            t_wpigd_music_trigger_v67_002bda28,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bda28, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Ambience_v67` (0x002b_da30) against the generated body.
    #[test]
    fn t_wpigd_ambience_v67_002bda30_matches() {
        check(
            ee::s08::f_002bda30__generated,
            t_wpigd_ambience_v67_002bda30,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bda30, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Camera_v67` (0x002b_da38) against the generated body.
    #[test]
    fn t_wpigd_camera_v67_002bda38_matches() {
        check(
            ee::s08::f_002bda38__generated,
            t_wpigd_camera_v67_002bda38,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bda38, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Danger_v67` (0x002b_da40) against the generated body.
    #[test]
    fn t_wpigd_danger_v67_002bda40_matches() {
        check(
            ee::s08::f_002bda40__generated,
            t_wpigd_danger_v67_002bda40,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bda40, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Fog_v67` (0x002b_da48) against the generated body.
    #[test]
    fn t_wpigd_fog_v67_002bda48_matches() {
        check(
            ee::s08::f_002bda48__generated,
            t_wpigd_fog_v67_002bda48,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bda48, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_LightGlow_v67` (0x002b_da50) against the generated body.
    #[test]
    fn t_wpigd_light_glow_v67_002bda50_matches() {
        check(
            ee::s08::f_002bda50__generated,
            t_wpigd_light_glow_v67_002bda50,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bda50, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Lighting_v67` (0x002b_da58) against the generated body.
    #[test]
    fn t_wpigd_lighting_v67_002bda58_matches() {
        check(
            ee::s08::f_002bda58__generated,
            t_wpigd_lighting_v67_002bda58,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bda58, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Mix_v67` (0x002b_da60) against the generated body.
    #[test]
    fn t_wpigd_mix_v67_002bda60_matches() {
        check(
            ee::s08::f_002bda60__generated,
            t_wpigd_mix_v67_002bda60,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bda60, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_ScreenTint_v67` (0x002b_da68) against the generated body.
    #[test]
    fn t_wpigd_screen_tint_v67_002bda68_matches() {
        check(
            ee::s08::f_002bda68__generated,
            t_wpigd_screen_tint_v67_002bda68,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bda68, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_SkyBox_v67` (0x002b_da70) against the generated body.
    #[test]
    fn t_wpigd_sky_box_v67_002bda70_matches() {
        check(
            ee::s08::f_002bda70__generated,
            t_wpigd_sky_box_v67_002bda70,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bda70, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Speech_v67` (0x002b_da78) against the generated body.
    #[test]
    fn t_wpigd_speech_v67_002bda78_matches() {
        check(
            ee::s08::f_002bda78__generated,
            t_wpigd_speech_v67_002bda78,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bda78, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Sun_v67` (0x002b_da80) against the generated body.
    #[test]
    fn t_wpigd_sun_v67_002bda80_matches() {
        check(
            ee::s08::f_002bda80__generated,
            t_wpigd_sun_v67_002bda80,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bda80, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Surface_v67` (0x002b_da88) against the generated body.
    #[test]
    fn t_wpigd_surface_v67_002bda88_matches() {
        check(
            ee::s08::f_002bda88__generated,
            t_wpigd_surface_v67_002bda88,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bda88, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Weather_v67` (0x002b_da90) against the generated body.
    #[test]
    fn t_wpigd_weather_v67_002bda90_matches() {
        check(
            ee::s08::f_002bda90__generated,
            t_wpigd_weather_v67_002bda90,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bda90, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Mix_IsEqual` (0x002b_dac8) against the generated body.
    #[test]
    fn t_wpigd_mix_is_equal_002bdac8_matches() {
        check(
            ee::s08::f_002bdac8__generated,
            t_wpigd_mix_is_equal_002bdac8,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2bdac8, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                leaf_fill(c, &mut r, 0x0080_5000, 0x2000, false, 0x0080_5000);
                c.g[A1][0] = sx(0x0080_5000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Ambience_IsEqual` (0x002b_dae0) against the generated body.
    #[test]
    fn t_wpigd_ambience_is_equal_002bdae0_matches() {
        check(
            ee::s08::f_002bdae0__generated,
            t_wpigd_ambience_is_equal_002bdae0,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2bdae0, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                leaf_fill(c, &mut r, 0x0080_5000, 0x2000, false, 0x0080_5000);
                c.g[A1][0] = sx(0x0080_5000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Speech_IsEqual` (0x002b_daf8) against the generated body.
    #[test]
    fn t_wpigd_speech_is_equal_002bdaf8_matches() {
        check(
            ee::s08::f_002bdaf8__generated,
            t_wpigd_speech_is_equal_002bdaf8,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2bdaf8, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                leaf_fill(c, &mut r, 0x0080_5000, 0x2000, false, 0x0080_5000);
                c.g[A1][0] = sx(0x0080_5000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_MusicTrigger_SetDefaults` (0x002b_e0b0) against the generated body.
    #[test]
    fn t_wpigd_music_trigger_set_defaults_002be0b0_matches() {
        check(
            ee::s08::f_002be0b0__generated,
            t_wpigd_music_trigger_set_defaults_002be0b0,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2be0b0, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Mix_SetDefaults` (0x002b_e0c8) against the generated body.
    #[test]
    fn t_wpigd_mix_set_defaults_002be0c8_matches() {
        check(
            ee::s08::f_002be0c8__generated,
            t_wpigd_mix_set_defaults_002be0c8,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2be0c8, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Ambience_SetDefaults` (0x002b_e0d8) against the generated body.
    #[test]
    fn t_wpigd_ambience_set_defaults_002be0d8_matches() {
        check(
            ee::s08::f_002be0d8__generated,
            t_wpigd_ambience_set_defaults_002be0d8,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2be0d8, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Speech_SetDefaults` (0x002b_e0e8) against the generated body.
    #[test]
    fn t_wpigd_speech_set_defaults_002be0e8_matches() {
        check(
            ee::s08::f_002be0e8__generated,
            t_wpigd_speech_set_defaults_002be0e8,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2be0e8, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Camera_SetDefaults` (0x002b_e0f8) against the generated body.
    #[test]
    fn t_wpigd_camera_set_defaults_002be0f8_matches() {
        check(
            ee::s08::f_002be0f8__generated,
            t_wpigd_camera_set_defaults_002be0f8,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2be0f8, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x6000, false, 0x0080_62b0);
                c.g[A0][0] = sx(0x0080_62b0 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Fog_SetDefaults` (0x002b_e108) against the generated body.
    #[test]
    fn t_wpigd_fog_set_defaults_002be108_matches() {
        check(
            ee::s08::f_002be108__generated,
            t_wpigd_fog_set_defaults_002be108,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2be108, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x6000, false, 0x0080_62b0);
                c.g[A0][0] = sx(0x0080_62b0 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_LightGlow_SetDefaults` (0x002b_e140) against the generated body.
    #[test]
    fn t_wpigd_light_glow_set_defaults_002be140_matches() {
        check(
            ee::s08::f_002be140__generated,
            t_wpigd_light_glow_set_defaults_002be140,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2be140, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_ScreenTint_SetDefaults` (0x002b_e170) against the generated body.
    #[test]
    fn t_wpigd_screen_tint_set_defaults_002be170_matches() {
        check(
            ee::s08::f_002be170__generated,
            t_wpigd_screen_tint_set_defaults_002be170,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2be170, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_SkyBox_SetDefaults` (0x002b_e198) against the generated body.
    #[test]
    fn t_wpigd_sky_box_set_defaults_002be198_matches() {
        check(
            ee::s08::f_002be198__generated,
            t_wpigd_sky_box_set_defaults_002be198,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2be198, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Sun_SetDefaults` (0x002b_e1a8) against the generated body.
    #[test]
    fn t_wpigd_sun_set_defaults_002be1a8_matches() {
        check(
            ee::s08::f_002be1a8__generated,
            t_wpigd_sun_set_defaults_002be1a8,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2be1a8, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Surface_SetDefaults` (0x002b_e1e0) against the generated body.
    #[test]
    fn t_wpigd_surface_set_defaults_002be1e0_matches() {
        check(
            ee::s08::f_002be1e0__generated,
            t_wpigd_surface_set_defaults_002be1e0,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2be1e0, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Danger_SetDefaults` (0x002b_e2c8) against the generated body.
    #[test]
    fn t_wpigd_danger_set_defaults_002be2c8_matches() {
        check(
            ee::s08::f_002be2c8__generated,
            t_wpigd_danger_set_defaults_002be2c8,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2be2c8, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `WorldPainterQuery_DebugPrintf` (0x002c_0b30) against the generated body.
    #[test]
    fn world_painter_query_debug_printf_matches() {
        check(
            ee::s08::f_002c0b30__generated,
            world_painter_query_debug_printf,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c0b30, 0);
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

    /// `tWPIGD_Base_v67` (0x002c_1458) against the generated body.
    #[test]
    fn t_wpigd_base_v67_002c1458_matches() {
        check(
            ee::s08::f_002c1458__generated,
            t_wpigd_base_v67_002c1458,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2c1458, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_MusicTrigger_Get08` (0x002c_1490) against the generated body.
    #[test]
    fn t_wpigd_music_trigger_get08_002c1490_matches() {
        check(
            ee::s08::f_002c1490__generated,
            t_wpigd_music_trigger_get08_002c1490,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c1490, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_MusicTrigger_Get10` (0x002c_1498) against the generated body.
    #[test]
    fn t_wpigd_music_trigger_get10_002c1498_matches() {
        check(
            ee::s08::f_002c1498__generated,
            t_wpigd_music_trigger_get10_002c1498,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c1498, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Mix_Get08` (0x002c_14a0) against the generated body.
    #[test]
    fn t_wpigd_mix_get08_002c14a0_matches() {
        check(
            ee::s08::f_002c14a0__generated,
            t_wpigd_mix_get08_002c14a0,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c14a0, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Ambience_Get08` (0x002c_14a8) against the generated body.
    #[test]
    fn t_wpigd_ambience_get08_002c14a8_matches() {
        check(
            ee::s08::f_002c14a8__generated,
            t_wpigd_ambience_get08_002c14a8,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c14a8, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Speech_Get08` (0x002c_14b0) against the generated body.
    #[test]
    fn t_wpigd_speech_get08_002c14b0_matches() {
        check(
            ee::s08::f_002c14b0__generated,
            t_wpigd_speech_get08_002c14b0,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c14b0, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Camera_Get08` (0x002c_14b8) against the generated body.
    #[test]
    fn t_wpigd_camera_get08_002c14b8_matches() {
        check(
            ee::s08::f_002c14b8__generated,
            t_wpigd_camera_get08_002c14b8,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c14b8, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Fog_Get08` (0x002c_14c0) against the generated body.
    #[test]
    fn t_wpigd_fog_get08_002c14c0_matches() {
        check(
            ee::s08::f_002c14c0__generated,
            t_wpigd_fog_get08_002c14c0,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c14c0, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Fog_Get10` (0x002c_14c8) against the generated body.
    #[test]
    fn t_wpigd_fog_get10_002c14c8_matches() {
        check(
            ee::s08::f_002c14c8__generated,
            t_wpigd_fog_get10_002c14c8,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c14c8, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Fog_Get18` (0x002c_14d0) against the generated body.
    #[test]
    fn t_wpigd_fog_get18_002c14d0_matches() {
        check(
            ee::s08::f_002c14d0__generated,
            t_wpigd_fog_get18_002c14d0,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c14d0, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }
}
