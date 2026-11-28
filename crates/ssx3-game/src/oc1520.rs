//! Staging module oc1520 (wave wave276): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1520`.

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
        0x003c_9358,
        snd_dec_utk_get_pos_mem_003c9358,
        "SND_DecUtk_GetPosMem",
    ),
    (
        0x003c_9518,
        snd_dec_utk_get_pos_loop_003c9518,
        "SND_DecUtk_GetPosLoop",
    ),
    (
        0x003c_9ac8,
        snd_dec_pcm16_init_mem_003c9ac8,
        "SND_DecPcm16_InitMem",
    ),
    (
        0x003c_9bc8,
        snd_dec_pcm16_init_loop_003c9bc8,
        "SND_DecPcm16_InitLoop",
    ),
    (
        0x003c_9ff8,
        snd_dec_eaxa_get_pos_mem_003c9ff8,
        "SND_DecEaxa_GetPosMem",
    ),
    (
        0x003c_a358,
        snd_dec_eaxa_get_pos_loop_003ca358,
        "SND_DecEaxa_GetPosLoop",
    ),
    (
        0x003c_f798,
        snd_dsp_buffer_source_init,
        "SND_DspBufferSource_Init",
    ),
    (0x003d_16e0, path_return_arg_003d16e0, "PATH_ReturnArg"),
    (0x003d_16e8, path_nop_stub2_003d16e8, "PATH_NopStub2"),
    (0x003d_3010, path_nop_stub3_003d3010, "PATH_NopStub3"),
    (
        0x003d_3018,
        path_set_user_callback_003d3018,
        "PATH_SetUserCallback",
    ),
    (
        0x003d_32a8,
        path_get_version_string_003d32a8,
        "PATH_GetVersionString",
    ),
    (
        0x003d_bac8,
        loc_string_table_compare_id_003dbac8,
        "LocStringTable_CompareId",
    ),
    (
        0x003f_61c8,
        usb_kb_null_callback_003f61c8,
        "UsbKb_NullCallback",
    ),
    (
        0x003f_b480,
        crc32_is_initialized_003fb480,
        "Crc32_IsInitialized",
    ),
    (0x003f_d908, leaf_003fd908, "leaf_003fd908"),
    (
        0x0040_cf48,
        libusbkb_set_led_status_callback_0040cf48,
        "libusbkb_SetLEDStatusCallback",
    ),
    (
        0x0040_cf68,
        libusbkb_set_led_mode_callback_0040cf68,
        "libusbkb_SetLEDModeCallback",
    ),
];

/// `SND_DecUtk_GetPosMem` (0x003c_9358): straight-line leaf: 1 load(s). Mechanical port of the generated body.
pub fn snd_dec_utk_get_pos_mem_003c9358(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x20));
    ret(c);
}

/// `SND_DecUtk_GetPosLoop` (0x003c_9518): straight-line leaf: 1 load(s). Mechanical port of the generated body.
pub fn snd_dec_utk_get_pos_loop_003c9518(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x20));
    ret(c);
}

/// `SND_DecPcm16_InitMem` (0x003c_9ac8): straight-line leaf: 3 load(s), 7 store(s). Mechanical port of the generated body.
pub fn snd_dec_pcm16_init_mem_003c9ac8(c: &mut Cpu) {
    c.cycles += 15;
    c.g[V0][0] = 0x3d0000;
    c.g[A2][0] = 0x3d0000;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x6600));
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x6608));
    c.sw((c.g[A0][0] as u32), c.g[V0][0]);
    c.g[V1][0] = c.lw((c.g[A1][0] as u32));
    c.sw((c.g[A0][0] as u32).wrapping_add(0x20), 0u64);
    c.sw((c.g[A0][0] as u32).wrapping_add(0x1c), c.g[V1][0]);
    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0xc));
    c.sw((c.g[A0][0] as u32).wrapping_add(0x24), c.g[V0][0]);
    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x18));
    c.sw((c.g[A0][0] as u32).wrapping_add(0x28), c.g[V1][0]);
    c.sw((c.g[A1][0] as u32).wrapping_add(0x28), c.g[A2][0]);
    c.sw((c.g[A0][0] as u32).wrapping_add(0x4), 0u64);
    ret(c);
}

/// `SND_DecPcm16_InitLoop` (0x003c_9bc8): straight-line leaf: 4 load(s), 7 store(s). Mechanical port of the generated body.
pub fn snd_dec_pcm16_init_loop_003c9bc8(c: &mut Cpu) {
    c.cycles += 16;
    c.g[V0][0] = 0x3d0000;
    c.g[A2][0] = 0x3d0000;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x64f0));
    c.g[A2][0] = sx((c.g[A2][0] as u32).wrapping_sub(0x64f8));
    c.sw((c.g[A0][0] as u32), c.g[V0][0]);
    c.g[V1][0] = c.lw((c.g[A1][0] as u32));
    c.sw((c.g[A0][0] as u32).wrapping_add(0x20), 0u64);
    c.sw((c.g[A0][0] as u32).wrapping_add(0x1c), c.g[V1][0]);
    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x10));
    c.sw((c.g[A0][0] as u32).wrapping_add(0x24), c.g[V0][0]);
    c.g[V1][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x14));
    c.sw((c.g[A0][0] as u32).wrapping_add(0x28), c.g[V1][0]);
    c.g[V0][0] = c.lw((c.g[A1][0] as u32).wrapping_add(0x18));
    c.sw((c.g[A0][0] as u32).wrapping_add(0x2c), c.g[V0][0]);
    c.sw((c.g[A1][0] as u32).wrapping_add(0x28), c.g[A2][0]);
    ret(c);
}

/// `SND_DecEaxa_GetPosMem` (0x003c_9ff8): straight-line leaf: 1 load(s). Mechanical port of the generated body.
pub fn snd_dec_eaxa_get_pos_mem_003c9ff8(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x28));
    ret(c);
}

/// `SND_DecEaxa_GetPosLoop` (0x003c_a358): straight-line leaf: 1 load(s). Mechanical port of the generated body.
pub fn snd_dec_eaxa_get_pos_loop_003ca358(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = c.lw((c.g[A0][0] as u32).wrapping_add(0x20));
    ret(c);
}

/// `SND_DspBufferSource_Init` (0x003c_f798): straight-line leaf: 3 store(s). Mechanical port of the generated body.
pub fn snd_dsp_buffer_source_init(c: &mut Cpu) {
    c.cycles += 6;
    c.g[V0][0] = 0x3d0000;
    c.sw((c.g[A0][0] as u32).wrapping_add(0x1c), c.g[A1][0]);
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x8c0));
    c.sw((c.g[A0][0] as u32).wrapping_add(0x4), 0u64);
    c.sw((c.g[A0][0] as u32), c.g[V0][0]);
    ret(c);
}

/// `PATH_ReturnArg` (0x003d_16e0): straight-line leaf. Mechanical port of the generated body.
pub fn path_return_arg_003d16e0(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = c.g[A0][0];
    ret(c);
}

/// `PATH_NopStub2` (0x003d_16e8): straight-line leaf. Mechanical port of the generated body.
pub fn path_nop_stub2_003d16e8(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `PATH_NopStub3` (0x003d_3010): straight-line leaf. Mechanical port of the generated body.
pub fn path_nop_stub3_003d3010(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `PATH_SetUserCallback` (0x003d_3018): straight-line leaf: 2 store(s). Mechanical port of the generated body.
pub fn path_set_user_callback_003d3018(c: &mut Cpu) {
    c.cycles += 5;
    c.g[V0][0] = 0x450000;
    c.g[V1][0] = 0x450000;
    c.sw((c.g[V0][0] as u32).wrapping_sub(0xbdc), c.g[A0][0]);
    c.sw((c.g[V1][0] as u32).wrapping_sub(0xbd8), c.g[A1][0]);
    ret(c);
}

/// `PATH_GetVersionString` (0x003d_32a8): straight-line leaf. Mechanical port of the generated body.
pub fn path_get_version_string_003d32a8(c: &mut Cpu) {
    c.cycles += 3;
    c.g[V0][0] = 0x450000;
    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_sub(0x12b8));
    ret(c);
}

/// `LocStringTable_CompareId` (0x003d_bac8): straight-line leaf: 3 load(s). Mechanical port of the generated body.
pub fn loc_string_table_compare_id_003dbac8(c: &mut Cpu) {
    c.cycles += 7;
    c.g[V0][0] = c.lbu((c.g[A1][0] as u32).wrapping_add(0x1));
    c.g[V1][0] = c.lbu((c.g[A1][0] as u32));
    c.g[A2][0] = c.lhu((c.g[A0][0] as u32));
    c.g[V0][0] = sx((c.g[V0][0] as u32) << 8);
    c.g[V0][0] = c.g[V0][0] | c.g[V1][0];
    c.g[V0][0] = sx((c.g[A2][0] as u32).wrapping_sub((c.g[V0][0] as u32)));
    ret(c);
}

/// `UsbKb_NullCallback` (0x003f_61c8): straight-line leaf. Mechanical port of the generated body.
pub fn usb_kb_null_callback_003f61c8(c: &mut Cpu) {
    c.cycles += 2;
    c.g[V0][0] = 0u64;
    ret(c);
}

/// `Crc32_IsInitialized` (0x003f_b480): straight-line leaf: 1 load(s). Mechanical port of the generated body.
pub fn crc32_is_initialized_003fb480(c: &mut Cpu) {
    c.cycles += 4;
    c.g[V1][0] = 0x450000;
    c.g[V0][0] = c.lw((c.g[V1][0] as u32).wrapping_add(0x1518));
    c.g[V0][0] = (0u64 < c.g[V0][0]) as u64;
    ret(c);
}

/// `leaf_003fd908` (0x003f_d908): straight-line leaf. Mechanical port of the generated body.
pub fn leaf_003fd908(c: &mut Cpu) {
    c.cycles += 2;
    ret(c);
}

/// `libusbkb_SetLEDStatusCallback` (0x0040_cf48): straight-line leaf: 2 load(s). Mechanical port of the generated body.
pub fn libusbkb_set_led_status_callback_0040cf48(c: &mut Cpu) {
    c.cycles += 8;
    c.g[V0][0] = 0x530000;
    c.g[A1][0] = 0x530000;
    c.g[V1][0] = c.lbu((c.g[V0][0] as u32).wrapping_sub(0x6ebf));
    c.g[A0][0] = c.lbu((c.g[A1][0] as u32).wrapping_sub(0x6eff));
    c.g[V0][0] = 0xffffffffffffffff;
    c.g[V1][0] = c.g[V1][0] ^ c.g[A0][0];
    if c.g[V1][0] == 0 {
        c.g[V0][0] = 0u64;
    }
    ret(c);
}

/// `libusbkb_SetLEDModeCallback` (0x0040_cf68): straight-line leaf: 2 load(s). Mechanical port of the generated body.
pub fn libusbkb_set_led_mode_callback_0040cf68(c: &mut Cpu) {
    c.cycles += 8;
    c.g[V0][0] = 0x530000;
    c.g[A1][0] = 0x530000;
    c.g[V1][0] = c.lbu((c.g[V0][0] as u32).wrapping_sub(0x6ebf));
    c.g[A0][0] = c.lbu((c.g[A1][0] as u32).wrapping_sub(0x6eff));
    c.g[V0][0] = 0xffffffffffffffff;
    c.g[V1][0] = c.g[V1][0] ^ c.g[A0][0];
    if c.g[V1][0] == 0 {
        c.g[V0][0] = 0u64;
    }
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

    /// `SND_DecUtk_GetPosMem` (0x003c_9358) against the generated body.
    #[test]
    fn snd_dec_utk_get_pos_mem_003c9358_matches() {
        check(
            ee::s14::f_003c9358__generated,
            snd_dec_utk_get_pos_mem_003c9358,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3c9358, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `SND_DecUtk_GetPosLoop` (0x003c_9518) against the generated body.
    #[test]
    fn snd_dec_utk_get_pos_loop_003c9518_matches() {
        check(
            ee::s14::f_003c9518__generated,
            snd_dec_utk_get_pos_loop_003c9518,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3c9518, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `SND_DecPcm16_InitMem` (0x003c_9ac8) against the generated body.
    #[test]
    fn snd_dec_pcm16_init_mem_003c9ac8_matches() {
        check(
            ee::s14::f_003c9ac8__generated,
            snd_dec_pcm16_init_mem_003c9ac8,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3c9ac8, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                leaf_fill(c, &mut r, 0x0080_5000, 0x2000, false, 0x0080_5000);
                c.g[A1][0] = sx(0x0080_5000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `SND_DecPcm16_InitLoop` (0x003c_9bc8) against the generated body.
    #[test]
    fn snd_dec_pcm16_init_loop_003c9bc8_matches() {
        check(
            ee::s14::f_003c9bc8__generated,
            snd_dec_pcm16_init_loop_003c9bc8,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3c9bc8, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                leaf_fill(c, &mut r, 0x0080_5000, 0x2000, false, 0x0080_5000);
                c.g[A1][0] = sx(0x0080_5000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `SND_DecEaxa_GetPosMem` (0x003c_9ff8) against the generated body.
    #[test]
    fn snd_dec_eaxa_get_pos_mem_003c9ff8_matches() {
        check(
            ee::s14::f_003c9ff8__generated,
            snd_dec_eaxa_get_pos_mem_003c9ff8,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3c9ff8, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `SND_DecEaxa_GetPosLoop` (0x003c_a358) against the generated body.
    #[test]
    fn snd_dec_eaxa_get_pos_loop_003ca358_matches() {
        check(
            ee::s14::f_003ca358__generated,
            snd_dec_eaxa_get_pos_loop_003ca358,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3ca358, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `SND_DspBufferSource_Init` (0x003c_f798) against the generated body.
    #[test]
    fn snd_dsp_buffer_source_init_matches() {
        check(
            ee::s14::f_003cf798__generated,
            snd_dsp_buffer_source_init,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3cf798, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                c.g[A1][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `PATH_ReturnArg` (0x003d_16e0) against the generated body.
    #[test]
    fn path_return_arg_003d16e0_matches() {
        check(
            ee::s14::f_003d16e0__generated,
            path_return_arg_003d16e0,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3d16e0, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `PATH_NopStub2` (0x003d_16e8) against the generated body.
    #[test]
    fn path_nop_stub2_003d16e8_matches() {
        check(
            ee::s14::f_003d16e8__generated,
            path_nop_stub2_003d16e8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x3d16e8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `PATH_NopStub3` (0x003d_3010) against the generated body.
    #[test]
    fn path_nop_stub3_003d3010_matches() {
        check(
            ee::s14::f_003d3010__generated,
            path_nop_stub3_003d3010,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x3d3010, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `PATH_SetUserCallback` (0x003d_3018) against the generated body.
    #[test]
    fn path_set_user_callback_003d3018_matches() {
        check(
            ee::s14::f_003d3018__generated,
            path_set_user_callback_003d3018,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3d3018, 0);
                c.g[A0][0] = leaf_val(&mut r);
                c.g[A1][0] = leaf_val(&mut r);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `PATH_GetVersionString` (0x003d_32a8) against the generated body.
    #[test]
    fn path_get_version_string_003d32a8_matches() {
        check(
            ee::s14::f_003d32a8__generated,
            path_get_version_string_003d32a8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x3d32a8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `LocStringTable_CompareId` (0x003d_bac8) against the generated body.
    #[test]
    fn loc_string_table_compare_id_003dbac8_matches() {
        check(
            ee::s14::f_003dbac8__generated,
            loc_string_table_compare_id_003dbac8,
            120,
            &|c, seed| {
                let mut r = machine(c, seed, 0x3dbac8, 0);
                leaf_fill(c, &mut r, 0x0080_2000, 0x2000, false, 0x0080_2000);
                c.g[A0][0] = sx(0x0080_2000 + 16 * (r.next_u32() % 32));
                leaf_fill(c, &mut r, 0x0080_5000, 0x2000, false, 0x0080_5000);
                c.g[A1][0] = sx(0x0080_5000 + 16 * (r.next_u32() % 32));
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `UsbKb_NullCallback` (0x003f_61c8) against the generated body.
    #[test]
    fn usb_kb_null_callback_003f61c8_matches() {
        check(
            ee::s14::f_003f61c8__generated,
            usb_kb_null_callback_003f61c8,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x3f61c8, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `Crc32_IsInitialized` (0x003f_b480) against the generated body.
    #[test]
    fn crc32_is_initialized_003fb480_matches() {
        check(
            ee::s15::f_003fb480__generated,
            crc32_is_initialized_003fb480,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x3fb480, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `leaf_003fd908` (0x003f_d908) against the generated body.
    #[test]
    fn leaf_003fd908_matches() {
        check(
            ee::s15::f_003fd908__generated,
            leaf_003fd908,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x3fd908, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `libusbkb_SetLEDStatusCallback` (0x0040_cf48) against the generated body.
    #[test]
    fn libusbkb_set_led_status_callback_0040cf48_matches() {
        check(
            ee::s15::f_0040cf48__generated,
            libusbkb_set_led_status_callback_0040cf48,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x40cf48, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }

    /// `libusbkb_SetLEDModeCallback` (0x0040_cf68) against the generated body.
    #[test]
    fn libusbkb_set_led_mode_callback_0040cf68_matches() {
        check(
            ee::s15::f_0040cf68__generated,
            libusbkb_set_led_mode_callback_0040cf68,
            120,
            &|c, seed| {
                let _r = machine(c, seed, 0x40cf68, 0);
                c.g[RA][0] = sx(0x0040_0000 + 8 * seed);
            },
        );
    }
}
