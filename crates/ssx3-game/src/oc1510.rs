//! Staging module oc1510 (wave wave276): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1510`.

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
        0x002b_c568,
        t_wpigd_base_get_light_glow10_002bc568,
        "tWPIGD_Base_GetLightGlow10",
    ),
    (
        0x002b_c570,
        t_wpigd_base_get_light_glow18_002bc570,
        "tWPIGD_Base_GetLightGlow18",
    ),
    (
        0x002b_c578,
        t_wpigd_base_get_light_glow20_002bc578,
        "tWPIGD_Base_GetLightGlow20",
    ),
    (
        0x002b_c580,
        t_wpigd_base_get_light_glow28_002bc580,
        "tWPIGD_Base_GetLightGlow28",
    ),
    (
        0x002b_c588,
        t_wpigd_base_get_light_glow30_002bc588,
        "tWPIGD_Base_GetLightGlow30",
    ),
    (
        0x002b_c590,
        t_wpigd_base_get_light_glow38_002bc590,
        "tWPIGD_Base_GetLightGlow38",
    ),
    (
        0x002b_c598,
        t_wpigd_base_get_screen_tint_red_002bc598,
        "tWPIGD_Base_GetScreenTintRed",
    ),
    (
        0x002b_c5a0,
        t_wpigd_base_get_screen_tint_green_002bc5a0,
        "tWPIGD_Base_GetScreenTintGreen",
    ),
    (
        0x002b_c5a8,
        t_wpigd_base_get_screen_tint_blue_002bc5a8,
        "tWPIGD_Base_GetScreenTintBlue",
    ),
    (
        0x002b_c5b0,
        t_wpigd_base_get_screen_fill_red_002bc5b0,
        "tWPIGD_Base_GetScreenFillRed",
    ),
    (
        0x002b_c5b8,
        t_wpigd_base_get_screen_fill_green_002bc5b8,
        "tWPIGD_Base_GetScreenFillGreen",
    ),
    (
        0x002b_c5c0,
        t_wpigd_base_get_screen_fill_blue_002bc5c0,
        "tWPIGD_Base_GetScreenFillBlue",
    ),
    (
        0x002b_c5c8,
        t_wpigd_base_get_sky_box08_002bc5c8,
        "tWPIGD_Base_GetSkyBox08",
    ),
    (
        0x002b_c5d0,
        t_wpigd_base_get_sun_elevation_002bc5d0,
        "tWPIGD_Base_GetSunElevation",
    ),
    (
        0x002b_c5d8,
        t_wpigd_base_get_sun_azimuth_002bc5d8,
        "tWPIGD_Base_GetSunAzimuth",
    ),
    (
        0x002b_c5e0,
        t_wpigd_base_get_sun_red_002bc5e0,
        "tWPIGD_Base_GetSunRed",
    ),
    (
        0x002b_c5e8,
        t_wpigd_base_get_sun_green_002bc5e8,
        "tWPIGD_Base_GetSunGreen",
    ),
    (
        0x002b_c5f0,
        t_wpigd_base_get_sun_blue_002bc5f0,
        "tWPIGD_Base_GetSunBlue",
    ),
    (
        0x002b_c5f8,
        t_wpigd_base_get_sun_intensity_002bc5f8,
        "tWPIGD_Base_GetSunIntensity",
    ),
    (
        0x002b_c600,
        t_wpigd_base_get_sun_texture_002bc600,
        "tWPIGD_Base_GetSunTexture",
    ),
    (
        0x002b_c608,
        t_wpigd_base_get_sun_flare_intensity_002bc608,
        "tWPIGD_Base_GetSunFlareIntensity",
    ),
    (
        0x002b_c610,
        t_wpigd_base_get_sun_size_002bc610,
        "tWPIGD_Base_GetSunSize",
    ),
    (
        0x002b_c618,
        t_wpigd_base_get_surface08_002bc618,
        "tWPIGD_Base_GetSurface08",
    ),
    (
        0x002b_c620,
        t_wpigd_base_get_irradiance_name0_002bc620,
        "tWPIGD_Base_GetIrradianceName0",
    ),
    (
        0x002b_c628,
        t_wpigd_base_get_irradiance_name1_002bc628,
        "tWPIGD_Base_GetIrradianceName1",
    ),
    (
        0x002b_c630,
        t_wpigd_base_get_irradiance_name2_002bc630,
        "tWPIGD_Base_GetIrradianceName2",
    ),
    (
        0x002b_c638,
        t_wpigd_base_get_irradiance_name3_002bc638,
        "tWPIGD_Base_GetIrradianceName3",
    ),
    (
        0x002b_c640,
        t_wpigd_base_get_lighting48_002bc640,
        "tWPIGD_Base_GetLighting48",
    ),
    (
        0x002b_c648,
        t_wpigd_base_get_lighting50_002bc648,
        "tWPIGD_Base_GetLighting50",
    ),
    (
        0x002b_c650,
        t_wpigd_base_get_weather08_002bc650,
        "tWPIGD_Base_GetWeather08",
    ),
    (
        0x002b_c658,
        t_wpigd_base_get_weather10_002bc658,
        "tWPIGD_Base_GetWeather10",
    ),
    (
        0x002b_c660,
        t_wpigd_base_get_weather18_002bc660,
        "tWPIGD_Base_GetWeather18",
    ),
    (
        0x002b_c668,
        t_wpigd_base_get_weather20_002bc668,
        "tWPIGD_Base_GetWeather20",
    ),
    (
        0x002b_c670,
        t_wpigd_base_get_weather28_002bc670,
        "tWPIGD_Base_GetWeather28",
    ),
    (
        0x002b_c678,
        t_wpigd_base_get_weather30_002bc678,
        "tWPIGD_Base_GetWeather30",
    ),
    (
        0x002b_c680,
        t_wpigd_base_get_weather38_002bc680,
        "tWPIGD_Base_GetWeather38",
    ),
    (
        0x002b_c688,
        t_wpigd_base_get_weather40_002bc688,
        "tWPIGD_Base_GetWeather40",
    ),
    (
        0x002b_c690,
        t_wpigd_base_get_weather48_002bc690,
        "tWPIGD_Base_GetWeather48",
    ),
    (
        0x002b_c698,
        t_wpigd_base_get_weather50_002bc698,
        "tWPIGD_Base_GetWeather50",
    ),
    (
        0x002b_c6a0,
        t_wpigd_base_get_weather58_002bc6a0,
        "tWPIGD_Base_GetWeather58",
    ),
    (
        0x002b_c6a8,
        t_wpigd_base_get_weather60_002bc6a8,
        "tWPIGD_Base_GetWeather60",
    ),
    (
        0x002b_c6b0,
        t_wpigd_base_get_weather68_002bc6b0,
        "tWPIGD_Base_GetWeather68",
    ),
    (
        0x002b_c6b8,
        t_wpigd_base_get_weather70_002bc6b8,
        "tWPIGD_Base_GetWeather70",
    ),
    (
        0x002b_c6c0,
        t_wpigd_base_get_weather78_002bc6c0,
        "tWPIGD_Base_GetWeather78",
    ),
    (
        0x002b_c6c8,
        t_wpigd_base_get_weather80_002bc6c8,
        "tWPIGD_Base_GetWeather80",
    ),
];

/// `tWPIGD_Base_GetLightGlow10` (0x002b_c568): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_base_get_light_glow10_002bc568(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `tWPIGD_Base_GetLightGlow18` (0x002b_c570): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_base_get_light_glow18_002bc570(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `tWPIGD_Base_GetLightGlow20` (0x002b_c578): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_base_get_light_glow20_002bc578(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `tWPIGD_Base_GetLightGlow28` (0x002b_c580): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_base_get_light_glow28_002bc580(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `tWPIGD_Base_GetLightGlow30` (0x002b_c588): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_base_get_light_glow30_002bc588(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `tWPIGD_Base_GetLightGlow38` (0x002b_c590): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_base_get_light_glow38_002bc590(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `tWPIGD_Base_GetScreenTintRed` (0x002b_c598): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_base_get_screen_tint_red_002bc598(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `tWPIGD_Base_GetScreenTintGreen` (0x002b_c5a0): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_base_get_screen_tint_green_002bc5a0(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `tWPIGD_Base_GetScreenTintBlue` (0x002b_c5a8): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_base_get_screen_tint_blue_002bc5a8(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `tWPIGD_Base_GetScreenFillRed` (0x002b_c5b0): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_base_get_screen_fill_red_002bc5b0(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `tWPIGD_Base_GetScreenFillGreen` (0x002b_c5b8): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_base_get_screen_fill_green_002bc5b8(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `tWPIGD_Base_GetScreenFillBlue` (0x002b_c5c0): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_base_get_screen_fill_blue_002bc5c0(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `tWPIGD_Base_GetSkyBox08` (0x002b_c5c8): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_base_get_sky_box08_002bc5c8(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `tWPIGD_Base_GetSunElevation` (0x002b_c5d0): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_base_get_sun_elevation_002bc5d0(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `tWPIGD_Base_GetSunAzimuth` (0x002b_c5d8): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_base_get_sun_azimuth_002bc5d8(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `tWPIGD_Base_GetSunRed` (0x002b_c5e0): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_base_get_sun_red_002bc5e0(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `tWPIGD_Base_GetSunGreen` (0x002b_c5e8): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_base_get_sun_green_002bc5e8(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `tWPIGD_Base_GetSunBlue` (0x002b_c5f0): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_base_get_sun_blue_002bc5f0(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `tWPIGD_Base_GetSunIntensity` (0x002b_c5f8): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_base_get_sun_intensity_002bc5f8(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `tWPIGD_Base_GetSunTexture` (0x002b_c600): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_base_get_sun_texture_002bc600(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `tWPIGD_Base_GetSunFlareIntensity` (0x002b_c608): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_base_get_sun_flare_intensity_002bc608(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `tWPIGD_Base_GetSunSize` (0x002b_c610): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_base_get_sun_size_002bc610(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `tWPIGD_Base_GetSurface08` (0x002b_c618): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_base_get_surface08_002bc618(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `tWPIGD_Base_GetIrradianceName0` (0x002b_c620): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_base_get_irradiance_name0_002bc620(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `tWPIGD_Base_GetIrradianceName1` (0x002b_c628): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_base_get_irradiance_name1_002bc628(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `tWPIGD_Base_GetIrradianceName2` (0x002b_c630): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_base_get_irradiance_name2_002bc630(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `tWPIGD_Base_GetIrradianceName3` (0x002b_c638): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_base_get_irradiance_name3_002bc638(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `tWPIGD_Base_GetLighting48` (0x002b_c640): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_base_get_lighting48_002bc640(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `tWPIGD_Base_GetLighting50` (0x002b_c648): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_base_get_lighting50_002bc648(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `tWPIGD_Base_GetWeather08` (0x002b_c650): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_base_get_weather08_002bc650(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `tWPIGD_Base_GetWeather10` (0x002b_c658): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_base_get_weather10_002bc658(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `tWPIGD_Base_GetWeather18` (0x002b_c660): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_base_get_weather18_002bc660(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `tWPIGD_Base_GetWeather20` (0x002b_c668): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_base_get_weather20_002bc668(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `tWPIGD_Base_GetWeather28` (0x002b_c670): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_base_get_weather28_002bc670(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `tWPIGD_Base_GetWeather30` (0x002b_c678): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_base_get_weather30_002bc678(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `tWPIGD_Base_GetWeather38` (0x002b_c680): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_base_get_weather38_002bc680(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `tWPIGD_Base_GetWeather40` (0x002b_c688): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_base_get_weather40_002bc688(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `tWPIGD_Base_GetWeather48` (0x002b_c690): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_base_get_weather48_002bc690(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `tWPIGD_Base_GetWeather50` (0x002b_c698): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_base_get_weather50_002bc698(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `tWPIGD_Base_GetWeather58` (0x002b_c6a0): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_base_get_weather58_002bc6a0(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `tWPIGD_Base_GetWeather60` (0x002b_c6a8): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_base_get_weather60_002bc6a8(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `tWPIGD_Base_GetWeather68` (0x002b_c6b0): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_base_get_weather68_002bc6b0(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `tWPIGD_Base_GetWeather70` (0x002b_c6b8): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_base_get_weather70_002bc6b8(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `tWPIGD_Base_GetWeather78` (0x002b_c6c0): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_base_get_weather78_002bc6c0(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `tWPIGD_Base_GetWeather80` (0x002b_c6c8): straight-line leaf. Mechanical port of the generated body.
pub fn t_wpigd_base_get_weather80_002bc6c8(c: &mut Cpu) {
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

    /// `tWPIGD_Base_GetLightGlow10` (0x002b_c568) against the generated body.
    #[test]
    fn t_wpigd_base_get_light_glow10_002bc568_matches() {
        check(
            ee::s08::f_002bc568__generated,
            t_wpigd_base_get_light_glow10_002bc568,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bc568, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Base_GetLightGlow18` (0x002b_c570) against the generated body.
    #[test]
    fn t_wpigd_base_get_light_glow18_002bc570_matches() {
        check(
            ee::s08::f_002bc570__generated,
            t_wpigd_base_get_light_glow18_002bc570,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bc570, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Base_GetLightGlow20` (0x002b_c578) against the generated body.
    #[test]
    fn t_wpigd_base_get_light_glow20_002bc578_matches() {
        check(
            ee::s08::f_002bc578__generated,
            t_wpigd_base_get_light_glow20_002bc578,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bc578, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Base_GetLightGlow28` (0x002b_c580) against the generated body.
    #[test]
    fn t_wpigd_base_get_light_glow28_002bc580_matches() {
        check(
            ee::s08::f_002bc580__generated,
            t_wpigd_base_get_light_glow28_002bc580,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bc580, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Base_GetLightGlow30` (0x002b_c588) against the generated body.
    #[test]
    fn t_wpigd_base_get_light_glow30_002bc588_matches() {
        check(
            ee::s08::f_002bc588__generated,
            t_wpigd_base_get_light_glow30_002bc588,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bc588, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Base_GetLightGlow38` (0x002b_c590) against the generated body.
    #[test]
    fn t_wpigd_base_get_light_glow38_002bc590_matches() {
        check(
            ee::s08::f_002bc590__generated,
            t_wpigd_base_get_light_glow38_002bc590,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bc590, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Base_GetScreenTintRed` (0x002b_c598) against the generated body.
    #[test]
    fn t_wpigd_base_get_screen_tint_red_002bc598_matches() {
        check(
            ee::s08::f_002bc598__generated,
            t_wpigd_base_get_screen_tint_red_002bc598,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bc598, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Base_GetScreenTintGreen` (0x002b_c5a0) against the generated body.
    #[test]
    fn t_wpigd_base_get_screen_tint_green_002bc5a0_matches() {
        check(
            ee::s08::f_002bc5a0__generated,
            t_wpigd_base_get_screen_tint_green_002bc5a0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bc5a0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Base_GetScreenTintBlue` (0x002b_c5a8) against the generated body.
    #[test]
    fn t_wpigd_base_get_screen_tint_blue_002bc5a8_matches() {
        check(
            ee::s08::f_002bc5a8__generated,
            t_wpigd_base_get_screen_tint_blue_002bc5a8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bc5a8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Base_GetScreenFillRed` (0x002b_c5b0) against the generated body.
    #[test]
    fn t_wpigd_base_get_screen_fill_red_002bc5b0_matches() {
        check(
            ee::s08::f_002bc5b0__generated,
            t_wpigd_base_get_screen_fill_red_002bc5b0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bc5b0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Base_GetScreenFillGreen` (0x002b_c5b8) against the generated body.
    #[test]
    fn t_wpigd_base_get_screen_fill_green_002bc5b8_matches() {
        check(
            ee::s08::f_002bc5b8__generated,
            t_wpigd_base_get_screen_fill_green_002bc5b8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bc5b8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Base_GetScreenFillBlue` (0x002b_c5c0) against the generated body.
    #[test]
    fn t_wpigd_base_get_screen_fill_blue_002bc5c0_matches() {
        check(
            ee::s08::f_002bc5c0__generated,
            t_wpigd_base_get_screen_fill_blue_002bc5c0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bc5c0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Base_GetSkyBox08` (0x002b_c5c8) against the generated body.
    #[test]
    fn t_wpigd_base_get_sky_box08_002bc5c8_matches() {
        check(
            ee::s08::f_002bc5c8__generated,
            t_wpigd_base_get_sky_box08_002bc5c8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bc5c8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Base_GetSunElevation` (0x002b_c5d0) against the generated body.
    #[test]
    fn t_wpigd_base_get_sun_elevation_002bc5d0_matches() {
        check(
            ee::s08::f_002bc5d0__generated,
            t_wpigd_base_get_sun_elevation_002bc5d0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bc5d0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Base_GetSunAzimuth` (0x002b_c5d8) against the generated body.
    #[test]
    fn t_wpigd_base_get_sun_azimuth_002bc5d8_matches() {
        check(
            ee::s08::f_002bc5d8__generated,
            t_wpigd_base_get_sun_azimuth_002bc5d8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bc5d8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Base_GetSunRed` (0x002b_c5e0) against the generated body.
    #[test]
    fn t_wpigd_base_get_sun_red_002bc5e0_matches() {
        check(
            ee::s08::f_002bc5e0__generated,
            t_wpigd_base_get_sun_red_002bc5e0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bc5e0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Base_GetSunGreen` (0x002b_c5e8) against the generated body.
    #[test]
    fn t_wpigd_base_get_sun_green_002bc5e8_matches() {
        check(
            ee::s08::f_002bc5e8__generated,
            t_wpigd_base_get_sun_green_002bc5e8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bc5e8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Base_GetSunBlue` (0x002b_c5f0) against the generated body.
    #[test]
    fn t_wpigd_base_get_sun_blue_002bc5f0_matches() {
        check(
            ee::s08::f_002bc5f0__generated,
            t_wpigd_base_get_sun_blue_002bc5f0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bc5f0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Base_GetSunIntensity` (0x002b_c5f8) against the generated body.
    #[test]
    fn t_wpigd_base_get_sun_intensity_002bc5f8_matches() {
        check(
            ee::s08::f_002bc5f8__generated,
            t_wpigd_base_get_sun_intensity_002bc5f8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bc5f8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Base_GetSunTexture` (0x002b_c600) against the generated body.
    #[test]
    fn t_wpigd_base_get_sun_texture_002bc600_matches() {
        check(
            ee::s08::f_002bc600__generated,
            t_wpigd_base_get_sun_texture_002bc600,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bc600, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Base_GetSunFlareIntensity` (0x002b_c608) against the generated body.
    #[test]
    fn t_wpigd_base_get_sun_flare_intensity_002bc608_matches() {
        check(
            ee::s08::f_002bc608__generated,
            t_wpigd_base_get_sun_flare_intensity_002bc608,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bc608, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Base_GetSunSize` (0x002b_c610) against the generated body.
    #[test]
    fn t_wpigd_base_get_sun_size_002bc610_matches() {
        check(
            ee::s08::f_002bc610__generated,
            t_wpigd_base_get_sun_size_002bc610,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bc610, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Base_GetSurface08` (0x002b_c618) against the generated body.
    #[test]
    fn t_wpigd_base_get_surface08_002bc618_matches() {
        check(
            ee::s08::f_002bc618__generated,
            t_wpigd_base_get_surface08_002bc618,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bc618, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Base_GetIrradianceName0` (0x002b_c620) against the generated body.
    #[test]
    fn t_wpigd_base_get_irradiance_name0_002bc620_matches() {
        check(
            ee::s08::f_002bc620__generated,
            t_wpigd_base_get_irradiance_name0_002bc620,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bc620, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Base_GetIrradianceName1` (0x002b_c628) against the generated body.
    #[test]
    fn t_wpigd_base_get_irradiance_name1_002bc628_matches() {
        check(
            ee::s08::f_002bc628__generated,
            t_wpigd_base_get_irradiance_name1_002bc628,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bc628, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Base_GetIrradianceName2` (0x002b_c630) against the generated body.
    #[test]
    fn t_wpigd_base_get_irradiance_name2_002bc630_matches() {
        check(
            ee::s08::f_002bc630__generated,
            t_wpigd_base_get_irradiance_name2_002bc630,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bc630, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Base_GetIrradianceName3` (0x002b_c638) against the generated body.
    #[test]
    fn t_wpigd_base_get_irradiance_name3_002bc638_matches() {
        check(
            ee::s08::f_002bc638__generated,
            t_wpigd_base_get_irradiance_name3_002bc638,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bc638, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Base_GetLighting48` (0x002b_c640) against the generated body.
    #[test]
    fn t_wpigd_base_get_lighting48_002bc640_matches() {
        check(
            ee::s08::f_002bc640__generated,
            t_wpigd_base_get_lighting48_002bc640,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bc640, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Base_GetLighting50` (0x002b_c648) against the generated body.
    #[test]
    fn t_wpigd_base_get_lighting50_002bc648_matches() {
        check(
            ee::s08::f_002bc648__generated,
            t_wpigd_base_get_lighting50_002bc648,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bc648, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Base_GetWeather08` (0x002b_c650) against the generated body.
    #[test]
    fn t_wpigd_base_get_weather08_002bc650_matches() {
        check(
            ee::s08::f_002bc650__generated,
            t_wpigd_base_get_weather08_002bc650,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bc650, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Base_GetWeather10` (0x002b_c658) against the generated body.
    #[test]
    fn t_wpigd_base_get_weather10_002bc658_matches() {
        check(
            ee::s08::f_002bc658__generated,
            t_wpigd_base_get_weather10_002bc658,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bc658, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Base_GetWeather18` (0x002b_c660) against the generated body.
    #[test]
    fn t_wpigd_base_get_weather18_002bc660_matches() {
        check(
            ee::s08::f_002bc660__generated,
            t_wpigd_base_get_weather18_002bc660,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bc660, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Base_GetWeather20` (0x002b_c668) against the generated body.
    #[test]
    fn t_wpigd_base_get_weather20_002bc668_matches() {
        check(
            ee::s08::f_002bc668__generated,
            t_wpigd_base_get_weather20_002bc668,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bc668, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Base_GetWeather28` (0x002b_c670) against the generated body.
    #[test]
    fn t_wpigd_base_get_weather28_002bc670_matches() {
        check(
            ee::s08::f_002bc670__generated,
            t_wpigd_base_get_weather28_002bc670,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bc670, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Base_GetWeather30` (0x002b_c678) against the generated body.
    #[test]
    fn t_wpigd_base_get_weather30_002bc678_matches() {
        check(
            ee::s08::f_002bc678__generated,
            t_wpigd_base_get_weather30_002bc678,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bc678, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Base_GetWeather38` (0x002b_c680) against the generated body.
    #[test]
    fn t_wpigd_base_get_weather38_002bc680_matches() {
        check(
            ee::s08::f_002bc680__generated,
            t_wpigd_base_get_weather38_002bc680,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bc680, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Base_GetWeather40` (0x002b_c688) against the generated body.
    #[test]
    fn t_wpigd_base_get_weather40_002bc688_matches() {
        check(
            ee::s08::f_002bc688__generated,
            t_wpigd_base_get_weather40_002bc688,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bc688, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Base_GetWeather48` (0x002b_c690) against the generated body.
    #[test]
    fn t_wpigd_base_get_weather48_002bc690_matches() {
        check(
            ee::s08::f_002bc690__generated,
            t_wpigd_base_get_weather48_002bc690,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bc690, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Base_GetWeather50` (0x002b_c698) against the generated body.
    #[test]
    fn t_wpigd_base_get_weather50_002bc698_matches() {
        check(
            ee::s08::f_002bc698__generated,
            t_wpigd_base_get_weather50_002bc698,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bc698, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Base_GetWeather58` (0x002b_c6a0) against the generated body.
    #[test]
    fn t_wpigd_base_get_weather58_002bc6a0_matches() {
        check(
            ee::s08::f_002bc6a0__generated,
            t_wpigd_base_get_weather58_002bc6a0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bc6a0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Base_GetWeather60` (0x002b_c6a8) against the generated body.
    #[test]
    fn t_wpigd_base_get_weather60_002bc6a8_matches() {
        check(
            ee::s08::f_002bc6a8__generated,
            t_wpigd_base_get_weather60_002bc6a8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bc6a8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Base_GetWeather68` (0x002b_c6b0) against the generated body.
    #[test]
    fn t_wpigd_base_get_weather68_002bc6b0_matches() {
        check(
            ee::s08::f_002bc6b0__generated,
            t_wpigd_base_get_weather68_002bc6b0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bc6b0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Base_GetWeather70` (0x002b_c6b8) against the generated body.
    #[test]
    fn t_wpigd_base_get_weather70_002bc6b8_matches() {
        check(
            ee::s08::f_002bc6b8__generated,
            t_wpigd_base_get_weather70_002bc6b8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bc6b8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Base_GetWeather78` (0x002b_c6c0) against the generated body.
    #[test]
    fn t_wpigd_base_get_weather78_002bc6c0_matches() {
        check(
            ee::s08::f_002bc6c0__generated,
            t_wpigd_base_get_weather78_002bc6c0,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bc6c0, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `tWPIGD_Base_GetWeather80` (0x002b_c6c8) against the generated body.
    #[test]
    fn t_wpigd_base_get_weather80_002bc6c8_matches() {
        check(
            ee::s08::f_002bc6c8__generated,
            t_wpigd_base_get_weather80_002bc6c8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x2bc6c8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }
}
