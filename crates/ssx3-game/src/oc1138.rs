//! Staging module oc1138 (wave wave272): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc1138`.

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
    (0x003c_54c0, snd_is_output_mode_supported, "SND_IsOutputModeSupported"),
];

/// `SND_IsOutputModeSupported(int mode, int* out)` (0x3c54c0): clears `*out`,
/// fetches the sound config (`SND_GetConfig` into a stack copy), then for a
/// mode below 9 jumps through the table at 0x4958a0. Table targets: 0x3c5508
/// (config byte at +5 below 2 means unsupported, else supported), 0x3c5518
/// (supported: `*out = 1`) and 0x3c5520 (unsupported). Modes of 9 and above
/// are unsupported. Always returns 0 in `v0`.
pub fn snd_is_output_mode_supported(c: &mut Cpu) {
    if c.pc != 0x003c_54c0 {
        return ee::s13::f_003c54c0__generated(c);
    }
    guest(c, |c| {
        c.cycles += 12;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_sub(0x110));
        c.sq(16, (c.g[SP][0] as u32).wrapping_add(0x100));
        c.sq(17, (c.g[SP][0] as u32).wrapping_add(0xf0));
        c.g[S0][0] = c.g[A1][0];
        c.sd((c.g[SP][0] as u32).wrapping_add(0xe0), c.g[RA][0]);
        c.g[S1][0] = c.g[A0][0];
        c.sw(c.g[S0][0] as u32, 0u64);
        c.g[A0][0] = c.g[SP][0];
        Callee { addr: 0x003b_4dd8, f: ee::s13::SND_GetConfig }.call(c, 0x003c_54e4)?;
        c.g[V1][0] = (c.g[S1][0] < 0x9u64) as u64;
        c.g[V0][0] = 0x49_0000;
        // 0 = table target 0x3c5508, 1 = 0x3c5518, 2 = 0x3c5520, 3 = 0x3c5524.
        let mut stage = 2;
        if c.g[V1][0] != 0 {
            c.cycles += 6;
            c.g[V1][0] = sx((c.g[S1][0] as u32) << 2);
            c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x58a0));
            c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(c.g[V0][0] as u32));
            c.g[A0][0] = c.lw(c.g[V1][0] as u32);
            let t = c.g[A0][0] as u32;
            stage = match t {
                0x003c_5508 => 0,
                0x003c_5518 => 1,
                0x003c_5520 => 2,
                _ => {
                    c.jump_out(t, 0x003c_54c0, 0x003c_5538);
                    return Ok(());
                }
            };
        }
        if stage == 0 {
            c.cycles += 4;
            c.g[V0][0] = c.lbu((c.g[SP][0] as u32).wrapping_add(0x5));
            c.g[V0][0] = (c.g[V0][0] < 0x2u64) as u64;
            if c.g[V0][0] != 0 {
                c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x100));
                stage = 3;
            } else {
                stage = 1;
            }
        }
        if stage == 1 {
            c.cycles += 2;
            c.g[V0][0] = 1;
            c.sw(c.g[S0][0] as u32, c.g[V0][0]);
            stage = 2;
        }
        if stage == 2 {
            c.cycles += 1;
            c.lq(16, (c.g[SP][0] as u32).wrapping_add(0x100));
        }
        c.cycles += 5;
        c.g[V0][0] = 0;
        c.lq(17, (c.g[SP][0] as u32).wrapping_add(0xf0));
        c.g[RA][0] = c.ld((c.g[SP][0] as u32).wrapping_add(0xe0));
        let t = c.g[RA][0] as u32;
        c.g[SP][0] = sx((c.g[SP][0] as u32).wrapping_add(0x110));
        c.pc = t;
        Ok(())
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::{check, elf_image, fakes, DATA};

    /// Modes 0..12 (table targets and out-of-range), config byte +5 below and
    /// above 2, against the generated body; `SND_GetConfig` takes its cached
    /// path (flag at 0x44c45c set) and copies the state block at 0x50a8e8.
    #[test]
    fn snd_is_output_mode_supported_matches() {
        check(
            ee::s13::f_003c54c0__generated,
            snd_is_output_mode_supported,
            300,
            &|c, seed| {
                let mut r = fakes::machine(c, seed, 0x1138, 0);
                elf_image(c);
                c.bus.write32(0x0044_c45c, 1);
                let byte = if seed % 2 == 0 { r.next_u32() % 2 } else { 2 + r.next_u32() % 100 };
                c.bus.write8(0x0050_a8e8 + 5, byte as u8);
                c.g[A0][0] = (seed % 12) as u64;
                c.g[A1][0] = sx(DATA);
                c.bus.write32(DATA, r.next_u32());
                c.pc = 0x003c_54c0;
            },
        );
    }
}
