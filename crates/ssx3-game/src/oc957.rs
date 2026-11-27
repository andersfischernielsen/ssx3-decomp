//! Staging module oc957 (wave wave257): hand-written replacements, each with its
//! differential test against the generated body. Moved into the source-tree
//! units by `tools/reorg_modules.py apply oc957`.

#![allow(unused_imports)]

use crate::abi::{r32, ret};
use crate::call::{call_ptr, guest, sp, Callee, Flow, Frame};
use crate::events::{deliver_event_here, event_due};
use crate::guest::Addr;
use ssx3_ee as ee;
use ssx3_ee_rt::cpu::{reg::*, sx};
use ssx3_ee_rt::funcs::GuestFn;
use ssx3_ee_rt::Cpu;

/// Replacements in this module: (address, function, name).
pub const OVERRIDES: &[(u32, GuestFn, &str)] = &[(
    0x001e_8920,
    ov_screen_peak_challenge_results_init,
    "OVScreenPeakChallengeResults_Init",
)];

/// `GetHashValue32(name)` with `a0 = 0x470000 - sub` (the name's address), returning to `ret`.
fn ovpcr_hash(c: &mut Cpu, sub: u32, ret_to: u32) -> Flow {
    c.g[A0][0] = sx(0x0047_0000u32.wrapping_sub(sub));
    call_ptr(c, 0x0031_7670, ret_to)
}

/// `UIText_SetTextId(a0 = obj, a1 = GetHashValue32(...))`-style step: hash `sub`, then the setter.
fn ovpcr_set_text(c: &mut Cpu, sub: u32, hash_ra: u32, obj: u64, set_ra: u32) -> Flow {
    ovpcr_hash(c, sub, hash_ra)?;
    c.g[A1][0] = c.g[V0][0];
    c.g[A0][0] = obj;
    call_ptr(c, 0x003a_1000, set_ra)
}

/// The "DNF" result text: `UIText_SetAsciiString(timescore, str_DNF)` (`a1` holds `0x4a0000`).
fn ovpcr_dnf(c: &mut Cpu) -> Flow {
    c.cycles += 5;
    c.g[A0][0] = c.g[S2][0];
    c.g[A1][0] = sx((c.g[A1][0] as u32).wrapping_add(0x2160));
    call_ptr(c, 0x003a_0c50, 0x001e_8c70)?;
    c.g[A0][0] = 0x0047_0000;
    Ok(())
}

/// The help text: hash of the string at `a0`, set on the object in `s0`.
fn ovpcr_help_text(c: &mut Cpu) -> Flow {
    c.cycles += 7;
    call_ptr(c, 0x0031_7670, 0x001e_8d0c)?;
    c.g[A1][0] = c.g[V0][0];
    c.g[A0][0] = c.g[S0][0];
    call_ptr(c, 0x003a_1000, 0x001e_8d18)?;
    c.g[V0][0] = 0x0053_0000;
    Ok(())
}

/// Convert the unsigned word in `v0` to a float in f0 (gcc's `(float)(unsigned)x`),
/// charging the generated branch cycles: `cycles_pos` for the non-negative path.
fn ovpcr_u32_to_f0(c: &mut Cpu, cycles_pos: u64) {
    let neg = (c.g[V0][0] as i64) < 0;
    c.g[V1][0] = sx((c.g[V0][0] as u32) >> 1);
    if !neg {
        c.cycles += cycles_pos;
        c.mtc1(2, 0);
        c.cvt_s_w(0, 0);
    } else {
        c.cycles += 5;
        c.g[V0][0] &= 1;
        c.g[V0][0] |= c.g[V1][0];
        c.mtc1(2, 0);
        c.cvt_s_w(0, 0);
        c.add_s(0, 0, 0);
    }
}

/// `OVScreenPeakChallengeResults_Init(self)` (0x1e8920): builds the Peak
/// Challenge results overlay. Adds the "70peakchal_results" and "OV_darkblue"
/// screens to the UI engine (`self+0x10`, screen kept at `self+0x40`), sets
/// the titles, the target/result time (events 6..8, mm:ss) or score (sprintf)
/// labels, the help text (won or sorry, by the human rider's ranking), hides
/// it on give-up, and fills the three "overlay_opt%d" text slots from the id
/// triple at 0x46e9b0 (the other two are hidden and made static).
pub fn ov_screen_peak_challenge_results_init(c: &mut Cpu) {
    guest(c, |c| {
        c.cycles += 23;
        let frame = Frame::open(c, 0xe0);
        frame.save(c, 20, 0x90);
        c.g[S4][0] = c.g[A0][0];
        frame.save(c, 17, 0xc0);
        frame.save(c, 18, 0xb0);
        c.g[A0][0] = 0x0047_0000;
        frame.save(c, 19, 0xa0);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1750));
        frame.save(c, 21, 0x80);
        frame.save(c, 22, 0x70);
        frame.save(c, 23, 0x60);
        frame.save(c, 30, 0x50);
        frame.save_ra(c, 0x40);
        frame.save(c, 16, 0xd0);
        c.g[S0][0] = Addr((c.g[S4][0] as u32).wrapping_add(0x10)).lw(c);
        call_ptr(c, 0x0031_7670, 0x001e_8960)?;
        // screen = cUIEngine_addScreenByHashName(engine, self, hash, 0)
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S4][0];
        c.g[A3][0] = 0;
        call_ptr(c, 0x0039_7cc0, 0x001e_8974)?;
        let screen = c.g[V0][0];
        Addr((c.g[S4][0] as u32).wrapping_add(0x40)).sw(c, screen as u64);
        if screen != 0 {
            c.cycles += 4;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            call_ptr(c, 0x0039_c870, 0x001e_898c)?;
        }

        // The dark-blue overlay screen.
        c.cycles += 11;
        c.g[A0][0] = 0x0047_0000;
        c.g[S0][0] = Addr((c.g[S4][0] as u32).wrapping_add(0x10)).lw(c);
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1c88));
        call_ptr(c, 0x0031_7670, 0x001e_899c)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A2][0] = c.g[V0][0];
        c.g[A1][0] = c.g[S4][0];
        c.g[A3][0] = 0;
        call_ptr(c, 0x0039_7cc0, 0x001e_89b0)?;
        if c.g[V0][0] != 0 {
            c.cycles += 4;
            c.g[A0][0] = c.g[V0][0];
            c.g[A1][0] = 0;
            c.g[A2][0] = 0;
            call_ptr(c, 0x0039_c870, 0x001e_89c8)?;
        }

        c.cycles += 43;
        c.g[A0][0] = c.g[S4][0];
        call_ptr(c, 0x0020_a380, 0x001e_89d0)?;
        call_ptr(c, 0x0014_dc80, 0x001e_89d8)?;
        c.g[A0][0] = c.g[V0][0];
        c.g[A1][0] = 0;
        call_ptr(c, 0x0014_dd58, 0x001e_89e4)?;
        call_ptr(c, 0x0014_dc80, 0x001e_89ec)?;
        c.g[A1][0] = 1;
        c.g[A0][0] = c.g[V0][0];
        call_ptr(c, 0x0014_dd58, 0x001e_89f8)?;
        c.g[A1][0] = Addr((c.g[GP][0] as u32).wrapping_sub(0x848)).lw(c);
        c.g[V1][0] = 0x0053_0000;
        c.g[S1][0] = c.lb((c.g[V1][0] as u32).wrapping_add(0x5c12));
        c.g[A0][0] = c.g[V0][0];
        c.g[S6][0] = Addr((c.g[A1][0] as u32).wrapping_add(0xc0)).lw(c);
        c.g[A1][0] = 0;
        call_ptr(c, 0x0014_6e98, 0x001e_8a14)?;
        c.g[FP][0] = c.g[V0][0];
        // title1 = kT_TITLERivalChallenge
        c.g[A0][0] = 0x004a_0000;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x21b0));
        call_ptr(c, 0x0031_7670, 0x001e_8a24)?;
        c.g[A0][0] = Addr((c.g[S4][0] as u32).wrapping_add(0x40)).lw(c);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_d860, 0x001e_8a30)?;
        c.g[S0][0] = c.g[V0][0];
        ovpcr_hash(c, 0x1a10, 0x001e_8a40)?;
        c.g[A0][0] = c.g[S0][0];
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x003a_1000, 0x001e_8a4c)?;
        // title2 = event name (events 6..11)
        c.g[A0][0] = 0x004a_0000;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(0x21b8));
        call_ptr(c, 0x0031_7670, 0x001e_8a58)?;
        c.g[A0][0] = Addr((c.g[S4][0] as u32).wrapping_add(0x40)).lw(c);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_d860, 0x001e_8a64)?;
        c.g[S0][0] = c.g[V0][0];
        let s1 = c.g[S1][0] as i64;
        c.g[V0][0] = (s1 < 12) as u64;
        let ge12 = c.g[V0][0] == 0;
        c.g[V0][0] = (s1 < 6) as u64;
        if ge12 {
            c.cycles += 1;
        } else {
            c.cycles += 2;
            if c.g[V0][0] == 0 {
                c.cycles += 5;
                c.g[A0][0] = c.g[S1][0];
                call_ptr(c, 0x001f_d190, 0x001e_8a84)?;
                c.g[A0][0] = c.g[S0][0];
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x003a_0e90, 0x001e_8a90)?;
                c.cycles += 1;
            }
        }
        c.g[A0][0] = 0x0047_0000;

        // The four labels.
        c.cycles += 32;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1738));
        call_ptr(c, 0x0031_7670, 0x001e_8a9c)?;
        c.g[A0][0] = Addr((c.g[S4][0] as u32).wrapping_add(0x40)).lw(c);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_d860, 0x001e_8aa8)?;
        c.g[A0][0] = 0x0047_0000;
        c.g[S3][0] = c.g[V0][0]; // label_target_timescore
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1720));
        call_ptr(c, 0x0031_7670, 0x001e_8ab8)?;
        c.g[A0][0] = Addr((c.g[S4][0] as u32).wrapping_add(0x40)).lw(c);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_d860, 0x001e_8ac4)?;
        c.g[A0][0] = 0x0047_0000;
        c.g[S5][0] = c.g[V0][0]; // target_timescore
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1708));
        call_ptr(c, 0x0031_7670, 0x001e_8ad4)?;
        c.g[A0][0] = Addr((c.g[S4][0] as u32).wrapping_add(0x40)).lw(c);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_d860, 0x001e_8ae0)?;
        c.g[A0][0] = 0x0047_0000;
        c.g[S0][0] = c.g[V0][0]; // label_timescore
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x16f8));
        call_ptr(c, 0x0031_7670, 0x001e_8af0)?;
        c.g[A0][0] = Addr((c.g[S4][0] as u32).wrapping_add(0x40)).lw(c);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_d860, 0x001e_8afc)?;
        c.g[S2][0] = c.g[V0][0]; // timescore
        c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_sub(6));
        c.g[S7][0] = c.g[V0][0];
        c.g[V0][0] = (c.g[V0][0] < 3) as u64;
        let timed = c.g[V0][0] != 0;
        if timed {
            c.cycles += 15;
            let (s3, s0) = (c.g[S3][0], c.g[S0][0]);
            ovpcr_set_text(c, 0x16e8, 0x001e_8b20, s3, 0x001e_8b2c)?;
            ovpcr_set_text(c, 0x16c8, 0x001e_8b38, s0, 0x001e_8b44)?;
            // target time = time limit / 60 s, formatted
            c.g[V0][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x78)).lw(c);
            ovpcr_u32_to_f0(c, 4);
            c.cycles += 12;
            c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(0x5784));
            c.mul_s(12, 0, 12);
            call_ptr(c, 0x0041_39f8, 0x001e_8b80)?;
            c.g[A0][0] = c.g[V0][0];
            call_ptr(c, 0x0020_a1a8, 0x001e_8b88)?;
            c.g[A1][0] = c.g[V0][0];
            c.g[A0][0] = c.g[S5][0];
            call_ptr(c, 0x003a_0c50, 0x001e_8b94)?;
            c.g[V0][0] = 0x0053_0000;
            c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x66a8)).lw(c);
            c.g[A1][0] = 0x004a_0000;
            if c.g[V1][0] != 0 {
                ovpcr_dnf(c)?;
            } else {
                c.cycles += 4;
                c.g[V0][0] = 0x0053_0000;
                c.g[V0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x6640)).lw(c);
                ovpcr_u32_to_f0(c, 4);
                c.cycles += 10;
                c.lwc1(12, (c.g[GP][0] as u32).wrapping_sub(0x5780));
                c.mul_s(12, 0, 12);
                call_ptr(c, 0x0041_39f8, 0x001e_8be4)?;
                c.g[A0][0] = c.g[V0][0];
                call_ptr(c, 0x0020_a1a8, 0x001e_8bec)?;
                c.g[A0][0] = c.g[S2][0];
                c.g[A1][0] = c.g[V0][0];
                call_ptr(c, 0x003a_0c50, 0x001e_8bf8)?;
                c.g[A0][0] = 0x0047_0000;
            }
        } else {
            c.cycles += 24;
            let (s3, s0) = (c.g[S3][0], c.g[S0][0]);
            ovpcr_set_text(c, 0x16b0, 0x001e_8c08, s3, 0x001e_8c14)?;
            ovpcr_set_text(c, 0x1690, 0x001e_8c20, s0, 0x001e_8c2c)?;
            c.g[V0][0] = 0x004a_0000;
            c.g[A2][0] = Addr((c.g[S6][0] as u32).wrapping_add(0x7c)).lw(c);
            c.g[S0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x2138));
            c.g[A0][0] = c.g[SP][0];
            c.g[A1][0] = c.g[S0][0];
            call_ptr(c, 0x0041_7828, 0x001e_8c44)?;
            c.g[A0][0] = c.g[S5][0];
            c.g[A1][0] = c.g[SP][0];
            call_ptr(c, 0x003a_0c50, 0x001e_8c50)?;
            c.g[V0][0] = 0x0053_0000;
            c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x66a8)).lw(c);
            let dnf = c.g[V1][0] != 0;
            c.g[V0][0] = 0x0053_0000;
            if dnf {
                c.cycles += 1;
                c.g[A1][0] = 0x004a_0000;
                ovpcr_dnf(c)?;
            } else {
                c.cycles += 8;
                c.g[A1][0] = c.g[S0][0];
                c.g[A2][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x6640)).lw(c);
                c.g[A0][0] = c.g[SP][0];
                call_ptr(c, 0x0041_7828, 0x001e_8c88)?;
                c.g[A0][0] = c.g[S2][0];
                c.g[A1][0] = c.g[SP][0];
                call_ptr(c, 0x003a_0c50, 0x001e_8c94)?;
                c.g[A0][0] = 0x0047_0000;
            }
        }

        // The help text, by the human rider's ranking.
        c.cycles += 13;
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x19a0));
        call_ptr(c, 0x0031_7670, 0x001e_8ca0)?;
        c.g[A0][0] = Addr((c.g[S4][0] as u32).wrapping_add(0x40)).lw(c);
        c.g[A1][0] = c.g[V0][0];
        call_ptr(c, 0x0039_d860, 0x001e_8cac)?;
        c.g[A0][0] = sx((c.g[FP][0] as u32) << 2);
        c.g[V1][0] = 0x0053_0000;
        c.g[V1][0] = sx((c.g[V1][0] as u32).wrapping_add(0x6730));
        c.g[S1][0] = c.g[A0][0];
        c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_add(c.g[V1][0] as u32));
        c.g[A0][0] = Addr(c.g[A0][0] as u32).lw(c);
        c.g[S0][0] = c.g[V0][0];
        let place = c.g[A0][0];
        // 0 | 1 | 2 | 3: won the challenge.
        let won = if place == 0 {
            true
        } else {
            c.cycles += 3;
            // The `li v0, n` after each compare sits in the branch delay slot.
            c.g[V0][0] = 1;
            let is1 = place == c.g[V0][0];
            c.g[V0][0] = 2;
            if is1 {
                true
            } else {
                c.cycles += 2;
                let is2 = place == c.g[V0][0];
                c.g[V0][0] = 3;
                if is2 {
                    true
                } else {
                    c.cycles += 2;
                    let lost = place != c.g[V0][0];
                    c.g[V0][0] = (c.g[S7][0] < 3) as u64;
                    !lost
                }
            }
        };
        if won {
            c.cycles += 3;
            c.g[A0][0] = 0x0047_0000;
            c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1990));
            ovpcr_help_text(c)?;
        } else {
            c.cycles += 2;
            if c.g[V0][0] == 0 {
                // Score challenge: "sorry, beat the score".
                c.g[A0][0] = 0x0047_0000;
                c.cycles += 6;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1668));
                call_ptr(c, 0x0031_7670, 0x001e_8d28)?;
                c.g[A1][0] = c.g[V0][0];
                c.g[A0][0] = c.g[S0][0];
                call_ptr(c, 0x003a_1000, 0x001e_8d34)?;
                c.g[V0][0] = 0x0053_0000;
            } else {
                // Time challenge: "sorry, beat the time".
                c.cycles += 2;
                c.g[A0][0] = 0x0047_0000;
                c.g[A0][0] = sx((c.g[A0][0] as u32).wrapping_sub(0x1680));
                ovpcr_help_text(c)?;
            }
        }

        // Hide the help text when the rider gave up (vtable slot 0x48: setVisible(0)).
        c.cycles += 5;
        c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(0x66d0));
        c.g[V0][0] = sx((c.g[S1][0] as u32).wrapping_add(c.g[V0][0] as u32));
        c.g[V1][0] = Addr(c.g[V0][0] as u32).lw(c);
        c.g[V0][0] = 0x0044_0000;
        if c.g[V1][0] != 0 {
            c.cycles += 7;
            c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(8)).lw(c);
            c.g[A1][0] = 0;
            c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x48)).lh(c);
            c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x4c)).lw(c);
            let target = c.g[V1][0] as u32;
            c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
            call_ptr(c, target, 0x001e_8d64)?;
            c.g[V0][0] = 0x0044_0000;
        }

        // The five "overlay_opt%d" slots.
        c.cycles += 14;
        c.g[V1][0] = 0x0047_0000;
        c.g[S6][0] = sx((c.g[V0][0] as u32).wrapping_add(0x1d10));
        c.g[S2][0] = 0;
        c.g[A2][0] = sx((c.g[V1][0] as u32).wrapping_sub(0x1650));
        c.ldl(2, (c.g[A2][0] as u32).wrapping_add(7));
        c.ldr(2, c.g[A2][0] as u32);
        c.g[A0][0] = Addr((c.g[A2][0] as u32).wrapping_add(8)).lw(c);
        c.sdl(2, (c.g[SP][0] as u32).wrapping_add(7));
        c.sdr(2, c.g[SP][0] as u32);
        let ids_hi = c.g[A0][0] as u32;
        Addr((c.g[SP][0] as u32).wrapping_add(8)).sw(c, ids_hi as u64);
        c.g[S5][0] = 0x0047_0000;
        c.g[S3][0] = sx((c.g[SP][0] as u32).wrapping_add(0x20));
        c.g[S1][0] = c.g[SP][0];
        loop {
            c.cycles += 13;
            c.g[A2][0] = c.g[S2][0];
            c.g[A1][0] = sx((c.g[S5][0] as u32).wrapping_sub(0x1b40));
            c.g[A0][0] = c.g[S3][0];
            call_ptr(c, 0x0041_7828, 0x001e_8db0)?;
            c.g[A0][0] = c.g[S3][0];
            call_ptr(c, 0x0031_7670, 0x001e_8db8)?;
            c.g[A0][0] = Addr((c.g[S4][0] as u32).wrapping_add(0x40)).lw(c);
            c.g[A1][0] = c.g[V0][0];
            call_ptr(c, 0x0039_d860, 0x001e_8dc4)?;
            c.g[S0][0] = c.g[V0][0];
            let first_three = (c.g[S2][0] as i64) < 3;
            c.g[V0][0] = first_three as u64;
            let missing = c.g[S0][0] == 0;
            c.g[S2][0] = sx((c.g[S2][0] as u32).wrapping_add(1));
            if !missing {
                c.cycles += 2;
                c.g[A1][0] = 0;
                if first_three {
                    c.cycles += 11;
                    c.g[V0][0] = Addr(c.g[S1][0] as u32).lw(c);
                    c.g[V0][0] = sx((c.g[V0][0] as u32) << 2);
                    c.g[V0][0] = sx((c.g[V0][0] as u32).wrapping_add(c.g[S6][0] as u32));
                    c.g[A0][0] = Addr(c.g[V0][0] as u32).lw(c);
                    call_ptr(c, 0x0031_7670, 0x001e_8df0)?;
                    c.g[A1][0] = c.g[V0][0];
                    c.g[A0][0] = c.g[S0][0];
                    call_ptr(c, 0x003a_1000, 0x001e_8dfc)?;
                    c.g[V0][0] = Addr(c.g[S1][0] as u32).lw(c);
                    let id = c.g[V0][0] as u32;
                    Addr((c.g[S0][0] as u32).wrapping_add(0x18)).sw(c, id as u64);
                } else {
                    // setVisible(0), then setStatic(1) through the object's vtable.
                    c.cycles += 11;
                    c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(8)).lw(c);
                    c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x48)).lh(c);
                    c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x4c)).lw(c);
                    let target = c.g[V1][0] as u32;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, target, 0x001e_8e1c)?;
                    c.g[V0][0] = Addr((c.g[S0][0] as u32).wrapping_add(8)).lw(c);
                    c.g[A1][0] = 1;
                    c.g[A0][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x40)).lh(c);
                    c.g[V1][0] = Addr((c.g[V0][0] as u32).wrapping_add(0x44)).lw(c);
                    let target = c.g[V1][0] as u32;
                    c.g[A0][0] = sx((c.g[S0][0] as u32).wrapping_add(c.g[A0][0] as u32));
                    call_ptr(c, target, 0x001e_8e34)?;
                }
            }
            c.cycles += 3;
            c.g[V0][0] = ((c.g[S2][0] as i64) < 5) as u64;
            let more = c.g[V0][0] != 0;
            c.g[S1][0] = sx((c.g[S1][0] as u32).wrapping_add(4));
            if more {
                if event_due(c) {
                    deliver_event_here(c, 0x001e_8da0);
                }
                continue;
            }
            break;
        }
        c.cycles += 12;
        frame.restore(c, 16, 0xd0);
        frame.restore(c, 17, 0xc0);
        frame.restore(c, 18, 0xb0);
        frame.restore(c, 19, 0xa0);
        frame.restore(c, 20, 0x90);
        frame.restore(c, 21, 0x80);
        frame.restore(c, 22, 0x70);
        frame.restore(c, 23, 0x60);
        frame.restore(c, 30, 0x50);
        frame.restore_ra(c, 0x40);
        frame.close(c);
        Ok(())
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::difftest::fakes::{self, machine};
    use crate::difftest::{DATA, GAME_GP};
    use std::ptr::addr_of_mut;

    const OVPCR_CTRL: u32 = DATA + 0x20_0000;
    const OVPCR_MAGIC: u32 = 0x0957_0957;
    /// Bit n set: the n-th `addScreenByHashName` call returns null.
    const OVPCR_NULL_SCREENS: u32 = OVPCR_CTRL + 4;
    const OVPCR_SCREEN_CALLS: u32 = OVPCR_CTRL + 8;
    /// The human rider index `getHumanRider` returns.
    const OVPCR_RIDER: u32 = OVPCR_CTRL + 0xc;
    const OVPCR_OBJ_CALLS: u32 = OVPCR_CTRL + 0x10;
    /// Objects `getObjectByHashName` returns, in call order (0: not found).
    const OVPCR_OBJ_TABLE: u32 = OVPCR_CTRL + 0x100;
    const OVPCR_SELF: u32 = DATA + 0x1000;
    const OVPCR_VT: u32 = DATA + 0x4000;
    const OVPCR_OBJS: u32 = DATA + 0x5000;
    const OVPCR_WORLD: u32 = DATA + 0x2000;
    const OVPCR_MODE: u32 = DATA + 0x2800;

    fn ovpcr_nop(_c: &mut Cpu) {}

    fn ovpcr_post_add_screen(c: &mut Cpu) {
        let n = c.bus.read32(OVPCR_SCREEN_CALLS);
        c.bus.write32(OVPCR_SCREEN_CALLS, n + 1);
        if c.bus.read32(OVPCR_NULL_SCREENS) >> n & 1 != 0 {
            c.g[V0][0] = 0;
        }
    }

    fn ovpcr_post_rider(c: &mut Cpu) {
        c.g[V0][0] = c.bus.read32(OVPCR_RIDER) as u64;
    }

    fn ovpcr_post_object(c: &mut Cpu) {
        let n = c.bus.read32(OVPCR_OBJ_CALLS);
        c.bus.write32(OVPCR_OBJ_CALLS, n + 1);
        c.g[V0][0] = c.bus.read32(OVPCR_OBJ_TABLE + 4 * (n % 16)) as u64;
    }

    /// Gated fakes in the override slots of the callees: each logs its call
    /// like `fakes::fake`, then `$post` adjusts the result; with the gate off
    /// the slot runs the body it held.
    macro_rules! ovpcr_gated {
        ($($fake:ident => $addr:literal, $post:path, $slot:path, $generated:path;)*) => {
            $(
                fn $fake(c: &mut Cpu) {
                    if c.bus.read32(OVPCR_CTRL) != OVPCR_MAGIC {
                        return fakes::fall_back(c, fakes::prev_cell($fake), $generated);
                    }
                    let ra = c.g[RA][0] as u32;
                    fakes::fake(c, $addr, ra);
                    if c.pc == ra {
                        $post(c);
                    }
                }
            )*

            fn ovpcr_install_fakes() {
                $( fakes::hook(addr_of_mut!($slot), $fake, fakes::prev_cell($fake)); )*
            }
        };
    }

    ovpcr_gated! {
        ovpcr_fake_00317670 => 0x0031_7670, ovpcr_nop, ee::s10::OVERRIDE_00317670, ee::s10::f_00317670__generated;
        ovpcr_fake_00397cc0 => 0x0039_7cc0, ovpcr_post_add_screen, ee::s13::OVERRIDE_00397CC0, ee::s13::f_00397cc0__generated;
        ovpcr_fake_0039c870 => 0x0039_c870, ovpcr_nop, ee::s13::OVERRIDE_0039C870, ee::s13::f_0039c870__generated;
        ovpcr_fake_0020a380 => 0x0020_a380, ovpcr_nop, ee::s05::OVERRIDE_0020A380, ee::s05::f_0020a380__generated;
        ovpcr_fake_0014dc80 => 0x0014_dc80, ovpcr_nop, ee::s01::OVERRIDE_0014DC80, ee::s01::f_0014dc80__generated;
        ovpcr_fake_0014dd58 => 0x0014_dd58, ovpcr_nop, ee::s01::OVERRIDE_0014DD58, ee::s01::f_0014dd58__generated;
        ovpcr_fake_00146e98 => 0x0014_6e98, ovpcr_post_rider, ee::s01::OVERRIDE_00146E98, ee::s01::f_00146e98__generated;
        ovpcr_fake_0039d860 => 0x0039_d860, ovpcr_post_object, ee::s13::OVERRIDE_0039D860, ee::s13::f_0039d860__generated;
        ovpcr_fake_003a1000 => 0x003a_1000, ovpcr_nop, ee::s13::OVERRIDE_003A1000, ee::s13::f_003a1000__generated;
        ovpcr_fake_001fd190 => 0x001f_d190, ovpcr_nop, ee::s04::OVERRIDE_001FD190, ee::s04::f_001fd190__generated;
        ovpcr_fake_003a0e90 => 0x003a_0e90, ovpcr_nop, ee::s13::OVERRIDE_003A0E90, ee::s13::f_003a0e90__generated;
        ovpcr_fake_0020a1a8 => 0x0020_a1a8, ovpcr_nop, ee::s05::OVERRIDE_0020A1A8, ee::s05::f_0020a1a8__generated;
        ovpcr_fake_004139f8 => 0x0041_39f8, ovpcr_nop, ee::s15::OVERRIDE_004139F8, ee::s15::f_004139f8__generated;
        ovpcr_fake_003a0c50 => 0x003a_0c50, ovpcr_nop, ee::s13::OVERRIDE_003A0C50, ee::s13::f_003a0c50__generated;
        ovpcr_fake_00417828 => 0x0041_7828, ovpcr_nop, ee::s15::OVERRIDE_00417828, ee::s15::f_00417828__generated;
    }

    /// `OVScreenPeakChallengeResults_Init`: every event kind (below 6, time
    /// events 6..8, score events 9..11, 12 and up), DNF and finished results
    /// (time words with the top bit set too), all rankings, give-up on and
    /// off, missing screens and missing objects, unwinds at any call.
    #[test]
    fn ov_screen_peak_challenge_results_init_matches() {
        const EVENTS: [u8; 10] = [0, 5, 6, 7, 8, 9, 11, 12, 13, 0xff];
        fakes::covered(
            ee::s04::f_001e8920__generated,
            ov_screen_peak_challenge_results_init,
            400,
            &|c, seed| {
                let mut r = machine(c, seed, 0x1e8920, 48);
                ovpcr_install_fakes();
                c.bus.write32(OVPCR_CTRL, OVPCR_MAGIC);
                c.bus.write32(
                    OVPCR_NULL_SCREENS,
                    if seed % 7 == 0 { r.next_u32() & 3 } else { 0 },
                );
                c.bus.write32(OVPCR_SCREEN_CALLS, 0);
                c.bus.write32(OVPCR_OBJ_CALLS, 0);
                c.bus.write32(OVPCR_RIDER, r.next_u32() % 5);
                // Objects with a vtable whose slots 0x48 / 0x40 are fake methods.
                for slot in [0x48u32, 0x40] {
                    let delta = (r.next_u32() as i16 as i32 & !3) as u32;
                    c.bus.write16(OVPCR_VT + slot, delta as u16);
                    c.bus.write32(OVPCR_VT + slot + 4, fakes::FAKE + slot);
                }
                for i in 0..16u32 {
                    let obj = if r.next_u32() % 6 == 0 {
                        0
                    } else {
                        OVPCR_OBJS + 0x100 * i
                    };
                    c.bus.write32(OVPCR_OBJ_TABLE + 4 * i, obj);
                    if obj != 0 {
                        c.bus.write32(obj + 8, OVPCR_VT);
                    }
                }
                // self: engine at +0x10.
                c.bus.write32(OVPCR_SELF + 0x10, r.next_u32());
                c.g[A0][0] = sx(OVPCR_SELF);
                // World -> game mode data: time limit (+0x78), point target (+0x7c).
                c.bus.write32(GAME_GP.wrapping_sub(0x848), OVPCR_WORLD);
                c.bus.write32(OVPCR_WORLD + 0xc0, OVPCR_MODE);
                let time = if seed % 3 == 0 {
                    r.next_u32() | 0x8000_0000
                } else {
                    r.next_u32() >> 8
                };
                c.bus.write32(OVPCR_MODE + 0x78, time);
                c.bus.write32(OVPCR_MODE + 0x7c, r.next_u32());
                // Event, results, flags (game data words).
                c.bus.write8(0x0053_5c12, EVENTS[(seed % 10) as usize]);
                c.bus.write32(0x0053_66a8, (seed / 3) % 2);
                let res = if seed % 5 == 0 {
                    r.next_u32() | 0x8000_0000
                } else {
                    r.next_u32() >> 6
                };
                c.bus.write32(0x0053_6640, res);
                for i in 0..5u32 {
                    c.bus.write32(0x0053_6730 + 4 * i, (r.next_u32() >> 3) % 6);
                    c.bus.write32(0x0053_66d0 + 4 * i, (r.next_u32() >> 5) % 2);
                }
                // The three option ids and the id -> string table.
                for i in 0..3u32 {
                    c.bus.write32(0x0046_e9b0 + 4 * i, r.next_u32() % 8);
                }
                for i in 0..8u32 {
                    c.bus.write32(0x0044_1d10 + 4 * i, r.next_u32());
                }
                // Seconds-per-tick style constants in the literal pool.
                c.bus.write32(GAME_GP.wrapping_sub(0x5784), 0x3c88_8889);
                c.bus.write32(GAME_GP.wrapping_sub(0x5780), 0x3c88_8889);
                c.pc = 0x001e_8920;
            },
        );
    }
}
