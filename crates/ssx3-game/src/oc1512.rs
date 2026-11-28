//! Staging module oc1512 (wave wave276): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1512`.

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
    (0x002c_14d8, t_wpigd_fog_get20_002c14d8, "tWPIGD_Fog_Get20"),
    (0x002c_14e0, t_wpigd_fog_get28_002c14e0, "tWPIGD_Fog_Get28"),
    (0x002c_14e8, t_wpigd_fog_get30_002c14e8, "tWPIGD_Fog_Get30"),
    (
        0x002c_14f0,
        t_wpigd_light_glow_get08_002c14f0,
        "tWPIGD_LightGlow_Get08",
    ),
    (
        0x002c_14f8,
        t_wpigd_light_glow_get10_002c14f8,
        "tWPIGD_LightGlow_Get10",
    ),
    (
        0x002c_1500,
        t_wpigd_light_glow_get18_002c1500,
        "tWPIGD_LightGlow_Get18",
    ),
    (
        0x002c_1508,
        t_wpigd_light_glow_get20_002c1508,
        "tWPIGD_LightGlow_Get20",
    ),
    (
        0x002c_1510,
        t_wpigd_light_glow_get28_002c1510,
        "tWPIGD_LightGlow_Get28",
    ),
    (
        0x002c_1518,
        t_wpigd_light_glow_get30_002c1518,
        "tWPIGD_LightGlow_Get30",
    ),
    (
        0x002c_1520,
        t_wpigd_light_glow_get38_002c1520,
        "tWPIGD_LightGlow_Get38",
    ),
    (
        0x002c_1528,
        t_wpigd_screen_tint_get_tint_red_002c1528,
        "tWPIGD_ScreenTint_GetTintRed",
    ),
    (
        0x002c_1530,
        t_wpigd_screen_tint_get_tint_green_002c1530,
        "tWPIGD_ScreenTint_GetTintGreen",
    ),
    (
        0x002c_1538,
        t_wpigd_screen_tint_get_tint_blue_002c1538,
        "tWPIGD_ScreenTint_GetTintBlue",
    ),
    (
        0x002c_1540,
        t_wpigd_screen_tint_get_fill_red_002c1540,
        "tWPIGD_ScreenTint_GetFillRed",
    ),
    (
        0x002c_1548,
        t_wpigd_screen_tint_get_fill_green_002c1548,
        "tWPIGD_ScreenTint_GetFillGreen",
    ),
    (
        0x002c_1550,
        t_wpigd_screen_tint_get_fill_blue_002c1550,
        "tWPIGD_ScreenTint_GetFillBlue",
    ),
    (
        0x002c_1558,
        t_wpigd_sky_box_get08_002c1558,
        "tWPIGD_SkyBox_Get08",
    ),
    (
        0x002c_1570,
        t_wpigd_sun_get_red_002c1570,
        "tWPIGD_Sun_GetRed",
    ),
    (
        0x002c_1578,
        t_wpigd_sun_get_green_002c1578,
        "tWPIGD_Sun_GetGreen",
    ),
    (
        0x002c_1580,
        t_wpigd_sun_get_blue_002c1580,
        "tWPIGD_Sun_GetBlue",
    ),
    (
        0x002c_1588,
        t_wpigd_sun_get_intensity_002c1588,
        "tWPIGD_Sun_GetIntensity",
    ),
    (
        0x002c_1590,
        t_wpigd_sun_get_texture_002c1590,
        "tWPIGD_Sun_GetTexture",
    ),
    (
        0x002c_1598,
        t_wpigd_sun_get_flare_intensity_002c1598,
        "tWPIGD_Sun_GetFlareIntensity",
    ),
    (
        0x002c_15a0,
        t_wpigd_sun_get_size_002c15a0,
        "tWPIGD_Sun_GetSize",
    ),
    (
        0x002c_15a8,
        t_wpigd_surface_get08_002c15a8,
        "tWPIGD_Surface_Get08",
    ),
    (
        0x002c_15b0,
        t_wpigd_lighting_get_irradiance_name0_002c15b0,
        "tWPIGD_Lighting_GetIrradianceName0",
    ),
    (
        0x002c_15b8,
        t_wpigd_lighting_get_irradiance_name1_002c15b8,
        "tWPIGD_Lighting_GetIrradianceName1",
    ),
    (
        0x002c_15c0,
        t_wpigd_lighting_get28_002c15c0,
        "tWPIGD_Lighting_Get28",
    ),
    (
        0x002c_15c8,
        t_wpigd_lighting_get38_002c15c8,
        "tWPIGD_Lighting_Get38",
    ),
    (
        0x002c_15d0,
        t_wpigd_lighting_get48_002c15d0,
        "tWPIGD_Lighting_Get48",
    ),
    (
        0x002c_15d8,
        t_wpigd_lighting_get50_002c15d8,
        "tWPIGD_Lighting_Get50",
    ),
    (
        0x002c_15e0,
        t_wpigd_weather_get08_002c15e0,
        "tWPIGD_Weather_Get08",
    ),
    (
        0x002c_15e8,
        t_wpigd_weather_get10_002c15e8,
        "tWPIGD_Weather_Get10",
    ),
    (
        0x002c_15f0,
        t_wpigd_weather_get18_002c15f0,
        "tWPIGD_Weather_Get18",
    ),
    (
        0x002c_15f8,
        t_wpigd_weather_get20_002c15f8,
        "tWPIGD_Weather_Get20",
    ),
    (
        0x002c_1600,
        t_wpigd_weather_get28_002c1600,
        "tWPIGD_Weather_Get28",
    ),
    (
        0x002c_1608,
        t_wpigd_weather_get30_002c1608,
        "tWPIGD_Weather_Get30",
    ),
    (
        0x002c_1610,
        t_wpigd_weather_get38_002c1610,
        "tWPIGD_Weather_Get38",
    ),
    (
        0x002c_1618,
        t_wpigd_weather_get40_002c1618,
        "tWPIGD_Weather_Get40",
    ),
    (
        0x002c_1620,
        t_wpigd_weather_get48_002c1620,
        "tWPIGD_Weather_Get48",
    ),
    (
        0x002c_1628,
        t_wpigd_weather_get50_002c1628,
        "tWPIGD_Weather_Get50",
    ),
    (
        0x002c_1630,
        t_wpigd_weather_get58_002c1630,
        "tWPIGD_Weather_Get58",
    ),
    (
        0x002c_1638,
        t_wpigd_weather_get60_002c1638,
        "tWPIGD_Weather_Get60",
    ),
    (
        0x002c_1640,
        t_wpigd_weather_get68_002c1640,
        "tWPIGD_Weather_Get68",
    ),
    (
        0x002c_1648,
        t_wpigd_weather_get70_002c1648,
        "tWPIGD_Weather_Get70",
    ),
];

/// `tWPIGD_Fog_Get20` (0x002c_14d8): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_fog_get20_002c14d8(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
    ret(c);
}

/// `tWPIGD_Fog_Get28` (0x002c_14e0): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_fog_get28_002c14e0(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x28));
    ret(c);
}

/// `tWPIGD_Fog_Get30` (0x002c_14e8): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_fog_get30_002c14e8(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x30));
    ret(c);
}

/// `tWPIGD_LightGlow_Get08` (0x002c_14f0): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_light_glow_get08_002c14f0(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x8));
    ret(c);
}

/// `tWPIGD_LightGlow_Get10` (0x002c_14f8): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_light_glow_get10_002c14f8(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x10));
    ret(c);
}

/// `tWPIGD_LightGlow_Get18` (0x002c_1500): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_light_glow_get18_002c1500(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
    ret(c);
}

/// `tWPIGD_LightGlow_Get20` (0x002c_1508): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_light_glow_get20_002c1508(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
    ret(c);
}

/// `tWPIGD_LightGlow_Get28` (0x002c_1510): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_light_glow_get28_002c1510(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x28));
    ret(c);
}

/// `tWPIGD_LightGlow_Get30` (0x002c_1518): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_light_glow_get30_002c1518(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x30));
    ret(c);
}

/// `tWPIGD_LightGlow_Get38` (0x002c_1520): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_light_glow_get38_002c1520(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x38));
    ret(c);
}

/// `tWPIGD_ScreenTint_GetTintRed` (0x002c_1528): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_screen_tint_get_tint_red_002c1528(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x8));
    ret(c);
}

/// `tWPIGD_ScreenTint_GetTintGreen` (0x002c_1530): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_screen_tint_get_tint_green_002c1530(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x10));
    ret(c);
}

/// `tWPIGD_ScreenTint_GetTintBlue` (0x002c_1538): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_screen_tint_get_tint_blue_002c1538(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
    ret(c);
}

/// `tWPIGD_ScreenTint_GetFillRed` (0x002c_1540): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_screen_tint_get_fill_red_002c1540(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
    ret(c);
}

/// `tWPIGD_ScreenTint_GetFillGreen` (0x002c_1548): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_screen_tint_get_fill_green_002c1548(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x28));
    ret(c);
}

/// `tWPIGD_ScreenTint_GetFillBlue` (0x002c_1550): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_screen_tint_get_fill_blue_002c1550(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x30));
    ret(c);
}

/// `tWPIGD_SkyBox_Get08` (0x002c_1558): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_sky_box_get08_002c1558(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x8));
    ret(c);
}

/// `tWPIGD_Sun_GetRed` (0x002c_1570): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_sun_get_red_002c1570(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
    ret(c);
}

/// `tWPIGD_Sun_GetGreen` (0x002c_1578): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_sun_get_green_002c1578(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
    ret(c);
}

/// `tWPIGD_Sun_GetBlue` (0x002c_1580): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_sun_get_blue_002c1580(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x28));
    ret(c);
}

/// `tWPIGD_Sun_GetIntensity` (0x002c_1588): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_sun_get_intensity_002c1588(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x30));
    ret(c);
}

/// `tWPIGD_Sun_GetTexture` (0x002c_1590): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_sun_get_texture_002c1590(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x38));
    ret(c);
}

/// `tWPIGD_Sun_GetFlareIntensity` (0x002c_1598): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_sun_get_flare_intensity_002c1598(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x40));
    ret(c);
}

/// `tWPIGD_Sun_GetSize` (0x002c_15a0): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_sun_get_size_002c15a0(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x48));
    ret(c);
}

/// `tWPIGD_Surface_Get08` (0x002c_15a8): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_surface_get08_002c15a8(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x8));
    ret(c);
}

/// `tWPIGD_Lighting_GetIrradianceName0` (0x002c_15b0): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_lighting_get_irradiance_name0_002c15b0(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x8));
    ret(c);
}

/// `tWPIGD_Lighting_GetIrradianceName1` (0x002c_15b8): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_lighting_get_irradiance_name1_002c15b8(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
    ret(c);
}

/// `tWPIGD_Lighting_Get28` (0x002c_15c0): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_lighting_get28_002c15c0(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x28));
    ret(c);
}

/// `tWPIGD_Lighting_Get38` (0x002c_15c8): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_lighting_get38_002c15c8(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x38));
    ret(c);
}

/// `tWPIGD_Lighting_Get48` (0x002c_15d0): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_lighting_get48_002c15d0(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x48));
    ret(c);
}

/// `tWPIGD_Lighting_Get50` (0x002c_15d8): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_lighting_get50_002c15d8(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x50));
    ret(c);
}

/// `tWPIGD_Weather_Get08` (0x002c_15e0): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_weather_get08_002c15e0(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x8));
    ret(c);
}

/// `tWPIGD_Weather_Get10` (0x002c_15e8): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_weather_get10_002c15e8(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x10));
    ret(c);
}

/// `tWPIGD_Weather_Get18` (0x002c_15f0): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_weather_get18_002c15f0(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x18));
    ret(c);
}

/// `tWPIGD_Weather_Get20` (0x002c_15f8): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_weather_get20_002c15f8(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x20));
    ret(c);
}

/// `tWPIGD_Weather_Get28` (0x002c_1600): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_weather_get28_002c1600(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x28));
    ret(c);
}

/// `tWPIGD_Weather_Get30` (0x002c_1608): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_weather_get30_002c1608(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x30));
    ret(c);
}

/// `tWPIGD_Weather_Get38` (0x002c_1610): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_weather_get38_002c1610(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x38));
    ret(c);
}

/// `tWPIGD_Weather_Get40` (0x002c_1618): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_weather_get40_002c1618(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x40));
    ret(c);
}

/// `tWPIGD_Weather_Get48` (0x002c_1620): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_weather_get48_002c1620(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x48));
    ret(c);
}

/// `tWPIGD_Weather_Get50` (0x002c_1628): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_weather_get50_002c1628(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x50));
    ret(c);
}

/// `tWPIGD_Weather_Get58` (0x002c_1630): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_weather_get58_002c1630(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x58));
    ret(c);
}

/// `tWPIGD_Weather_Get60` (0x002c_1638): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_weather_get60_002c1638(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x60));
    ret(c);
}

/// `tWPIGD_Weather_Get68` (0x002c_1640): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_weather_get68_002c1640(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x68));
    ret(c);
}

/// `tWPIGD_Weather_Get70` (0x002c_1648): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_weather_get70_002c1648(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x70));
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

    /// `tWPIGD_Fog_Get20` (0x002c_14d8) against the generated body.
    #[test]
    fn t_wpigd_fog_get20_002c14d8_matches() {
        check(
            ee::s08::f_002c14d8__generated,
            t_wpigd_fog_get20_002c14d8,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c14d8, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Fog_Get28` (0x002c_14e0) against the generated body.
    #[test]
    fn t_wpigd_fog_get28_002c14e0_matches() {
        check(
            ee::s08::f_002c14e0__generated,
            t_wpigd_fog_get28_002c14e0,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c14e0, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Fog_Get30` (0x002c_14e8) against the generated body.
    #[test]
    fn t_wpigd_fog_get30_002c14e8_matches() {
        check(
            ee::s08::f_002c14e8__generated,
            t_wpigd_fog_get30_002c14e8,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c14e8, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_LightGlow_Get08` (0x002c_14f0) against the generated body.
    #[test]
    fn t_wpigd_light_glow_get08_002c14f0_matches() {
        check(
            ee::s08::f_002c14f0__generated,
            t_wpigd_light_glow_get08_002c14f0,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c14f0, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_LightGlow_Get10` (0x002c_14f8) against the generated body.
    #[test]
    fn t_wpigd_light_glow_get10_002c14f8_matches() {
        check(
            ee::s08::f_002c14f8__generated,
            t_wpigd_light_glow_get10_002c14f8,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c14f8, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_LightGlow_Get18` (0x002c_1500) against the generated body.
    #[test]
    fn t_wpigd_light_glow_get18_002c1500_matches() {
        check(
            ee::s08::f_002c1500__generated,
            t_wpigd_light_glow_get18_002c1500,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c1500, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_LightGlow_Get20` (0x002c_1508) against the generated body.
    #[test]
    fn t_wpigd_light_glow_get20_002c1508_matches() {
        check(
            ee::s08::f_002c1508__generated,
            t_wpigd_light_glow_get20_002c1508,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c1508, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_LightGlow_Get28` (0x002c_1510) against the generated body.
    #[test]
    fn t_wpigd_light_glow_get28_002c1510_matches() {
        check(
            ee::s08::f_002c1510__generated,
            t_wpigd_light_glow_get28_002c1510,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c1510, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_LightGlow_Get30` (0x002c_1518) against the generated body.
    #[test]
    fn t_wpigd_light_glow_get30_002c1518_matches() {
        check(
            ee::s08::f_002c1518__generated,
            t_wpigd_light_glow_get30_002c1518,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c1518, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_LightGlow_Get38` (0x002c_1520) against the generated body.
    #[test]
    fn t_wpigd_light_glow_get38_002c1520_matches() {
        check(
            ee::s08::f_002c1520__generated,
            t_wpigd_light_glow_get38_002c1520,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c1520, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_ScreenTint_GetTintRed` (0x002c_1528) against the generated body.
    #[test]
    fn t_wpigd_screen_tint_get_tint_red_002c1528_matches() {
        check(
            ee::s08::f_002c1528__generated,
            t_wpigd_screen_tint_get_tint_red_002c1528,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c1528, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_ScreenTint_GetTintGreen` (0x002c_1530) against the generated body.
    #[test]
    fn t_wpigd_screen_tint_get_tint_green_002c1530_matches() {
        check(
            ee::s08::f_002c1530__generated,
            t_wpigd_screen_tint_get_tint_green_002c1530,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c1530, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_ScreenTint_GetTintBlue` (0x002c_1538) against the generated body.
    #[test]
    fn t_wpigd_screen_tint_get_tint_blue_002c1538_matches() {
        check(
            ee::s08::f_002c1538__generated,
            t_wpigd_screen_tint_get_tint_blue_002c1538,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c1538, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_ScreenTint_GetFillRed` (0x002c_1540) against the generated body.
    #[test]
    fn t_wpigd_screen_tint_get_fill_red_002c1540_matches() {
        check(
            ee::s08::f_002c1540__generated,
            t_wpigd_screen_tint_get_fill_red_002c1540,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c1540, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_ScreenTint_GetFillGreen` (0x002c_1548) against the generated body.
    #[test]
    fn t_wpigd_screen_tint_get_fill_green_002c1548_matches() {
        check(
            ee::s08::f_002c1548__generated,
            t_wpigd_screen_tint_get_fill_green_002c1548,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c1548, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_ScreenTint_GetFillBlue` (0x002c_1550) against the generated body.
    #[test]
    fn t_wpigd_screen_tint_get_fill_blue_002c1550_matches() {
        check(
            ee::s08::f_002c1550__generated,
            t_wpigd_screen_tint_get_fill_blue_002c1550,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c1550, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_SkyBox_Get08` (0x002c_1558) against the generated body.
    #[test]
    fn t_wpigd_sky_box_get08_002c1558_matches() {
        check(
            ee::s08::f_002c1558__generated,
            t_wpigd_sky_box_get08_002c1558,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c1558, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Sun_GetRed` (0x002c_1570) against the generated body.
    #[test]
    fn t_wpigd_sun_get_red_002c1570_matches() {
        check(
            ee::s08::f_002c1570__generated,
            t_wpigd_sun_get_red_002c1570,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c1570, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Sun_GetGreen` (0x002c_1578) against the generated body.
    #[test]
    fn t_wpigd_sun_get_green_002c1578_matches() {
        check(
            ee::s08::f_002c1578__generated,
            t_wpigd_sun_get_green_002c1578,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c1578, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Sun_GetBlue` (0x002c_1580) against the generated body.
    #[test]
    fn t_wpigd_sun_get_blue_002c1580_matches() {
        check(
            ee::s08::f_002c1580__generated,
            t_wpigd_sun_get_blue_002c1580,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c1580, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Sun_GetIntensity` (0x002c_1588) against the generated body.
    #[test]
    fn t_wpigd_sun_get_intensity_002c1588_matches() {
        check(
            ee::s08::f_002c1588__generated,
            t_wpigd_sun_get_intensity_002c1588,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c1588, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Sun_GetTexture` (0x002c_1590) against the generated body.
    #[test]
    fn t_wpigd_sun_get_texture_002c1590_matches() {
        check(
            ee::s08::f_002c1590__generated,
            t_wpigd_sun_get_texture_002c1590,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c1590, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Sun_GetFlareIntensity` (0x002c_1598) against the generated body.
    #[test]
    fn t_wpigd_sun_get_flare_intensity_002c1598_matches() {
        check(
            ee::s08::f_002c1598__generated,
            t_wpigd_sun_get_flare_intensity_002c1598,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c1598, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Sun_GetSize` (0x002c_15a0) against the generated body.
    #[test]
    fn t_wpigd_sun_get_size_002c15a0_matches() {
        check(
            ee::s08::f_002c15a0__generated,
            t_wpigd_sun_get_size_002c15a0,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c15a0, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Surface_Get08` (0x002c_15a8) against the generated body.
    #[test]
    fn t_wpigd_surface_get08_002c15a8_matches() {
        check(
            ee::s08::f_002c15a8__generated,
            t_wpigd_surface_get08_002c15a8,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c15a8, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Lighting_GetIrradianceName0` (0x002c_15b0) against the generated body.
    #[test]
    fn t_wpigd_lighting_get_irradiance_name0_002c15b0_matches() {
        check(
            ee::s08::f_002c15b0__generated,
            t_wpigd_lighting_get_irradiance_name0_002c15b0,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c15b0, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Lighting_GetIrradianceName1` (0x002c_15b8) against the generated body.
    #[test]
    fn t_wpigd_lighting_get_irradiance_name1_002c15b8_matches() {
        check(
            ee::s08::f_002c15b8__generated,
            t_wpigd_lighting_get_irradiance_name1_002c15b8,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c15b8, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Lighting_Get28` (0x002c_15c0) against the generated body.
    #[test]
    fn t_wpigd_lighting_get28_002c15c0_matches() {
        check(
            ee::s08::f_002c15c0__generated,
            t_wpigd_lighting_get28_002c15c0,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c15c0, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Lighting_Get38` (0x002c_15c8) against the generated body.
    #[test]
    fn t_wpigd_lighting_get38_002c15c8_matches() {
        check(
            ee::s08::f_002c15c8__generated,
            t_wpigd_lighting_get38_002c15c8,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c15c8, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Lighting_Get48` (0x002c_15d0) against the generated body.
    #[test]
    fn t_wpigd_lighting_get48_002c15d0_matches() {
        check(
            ee::s08::f_002c15d0__generated,
            t_wpigd_lighting_get48_002c15d0,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c15d0, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Lighting_Get50` (0x002c_15d8) against the generated body.
    #[test]
    fn t_wpigd_lighting_get50_002c15d8_matches() {
        check(
            ee::s08::f_002c15d8__generated,
            t_wpigd_lighting_get50_002c15d8,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c15d8, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Weather_Get08` (0x002c_15e0) against the generated body.
    #[test]
    fn t_wpigd_weather_get08_002c15e0_matches() {
        check(
            ee::s08::f_002c15e0__generated,
            t_wpigd_weather_get08_002c15e0,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c15e0, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Weather_Get10` (0x002c_15e8) against the generated body.
    #[test]
    fn t_wpigd_weather_get10_002c15e8_matches() {
        check(
            ee::s08::f_002c15e8__generated,
            t_wpigd_weather_get10_002c15e8,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c15e8, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Weather_Get18` (0x002c_15f0) against the generated body.
    #[test]
    fn t_wpigd_weather_get18_002c15f0_matches() {
        check(
            ee::s08::f_002c15f0__generated,
            t_wpigd_weather_get18_002c15f0,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c15f0, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Weather_Get20` (0x002c_15f8) against the generated body.
    #[test]
    fn t_wpigd_weather_get20_002c15f8_matches() {
        check(
            ee::s08::f_002c15f8__generated,
            t_wpigd_weather_get20_002c15f8,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c15f8, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Weather_Get28` (0x002c_1600) against the generated body.
    #[test]
    fn t_wpigd_weather_get28_002c1600_matches() {
        check(
            ee::s08::f_002c1600__generated,
            t_wpigd_weather_get28_002c1600,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c1600, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Weather_Get30` (0x002c_1608) against the generated body.
    #[test]
    fn t_wpigd_weather_get30_002c1608_matches() {
        check(
            ee::s08::f_002c1608__generated,
            t_wpigd_weather_get30_002c1608,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c1608, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Weather_Get38` (0x002c_1610) against the generated body.
    #[test]
    fn t_wpigd_weather_get38_002c1610_matches() {
        check(
            ee::s08::f_002c1610__generated,
            t_wpigd_weather_get38_002c1610,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c1610, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Weather_Get40` (0x002c_1618) against the generated body.
    #[test]
    fn t_wpigd_weather_get40_002c1618_matches() {
        check(
            ee::s08::f_002c1618__generated,
            t_wpigd_weather_get40_002c1618,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c1618, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Weather_Get48` (0x002c_1620) against the generated body.
    #[test]
    fn t_wpigd_weather_get48_002c1620_matches() {
        check(
            ee::s08::f_002c1620__generated,
            t_wpigd_weather_get48_002c1620,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c1620, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Weather_Get50` (0x002c_1628) against the generated body.
    #[test]
    fn t_wpigd_weather_get50_002c1628_matches() {
        check(
            ee::s08::f_002c1628__generated,
            t_wpigd_weather_get50_002c1628,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c1628, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Weather_Get58` (0x002c_1630) against the generated body.
    #[test]
    fn t_wpigd_weather_get58_002c1630_matches() {
        check(
            ee::s08::f_002c1630__generated,
            t_wpigd_weather_get58_002c1630,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c1630, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Weather_Get60` (0x002c_1638) against the generated body.
    #[test]
    fn t_wpigd_weather_get60_002c1638_matches() {
        check(
            ee::s08::f_002c1638__generated,
            t_wpigd_weather_get60_002c1638,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c1638, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Weather_Get68` (0x002c_1640) against the generated body.
    #[test]
    fn t_wpigd_weather_get68_002c1640_matches() {
        check(
            ee::s08::f_002c1640__generated,
            t_wpigd_weather_get68_002c1640,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c1640, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Weather_Get70` (0x002c_1648) against the generated body.
    #[test]
    fn t_wpigd_weather_get70_002c1648_matches() {
        check(
            ee::s08::f_002c1648__generated,
            t_wpigd_weather_get70_002c1648,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x2c1648, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }
}
